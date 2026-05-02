<script lang="ts">
  import Tabs from "$lib/Tabs.svelte";
  import { openUrl } from "@tauri-apps/plugin-opener";
  import Radio from "$lib/Radio.svelte";
  import {
    getCurrentWebviewWindow,
    WebviewWindow,
  } from "@tauri-apps/api/webviewWindow";
  import { get, writable } from "svelte/store";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import {
    register,
    unregister,
    type ShortcutEvent,
  } from "@tauri-apps/plugin-global-shortcut";
  const appWindow = getCurrentWebviewWindow();
  let clicking = $state(false);
  let picking = $state(false);
  let pickerWindow = $state<WebviewWindow>();

  let max_input: HTMLInputElement;
  // svelte-ignore state_referenced_locally
  const clickingStore = writable($state.snapshot(clicking)); // I had no idea how to do it otherwise
  $effect(() => {
    clickingStore.set(clicking);
  });
  clickingStore.subscribe(async (v) => {
    if (v) {
      let timesLeft =
        config.repeat_type == "number" ? config.repeat_x_count : Infinity;
      for (;;) {
        if (config.do_mouse_pos) {
          await invoke("set_mouse_pos", {
            location: [config.mouse_pos.x, config.mouse_pos.y],
          });
        }
        await invoke("mouse_button", {
          double: config.backend__double,
          button: config.backend__button,
        });
        console.log("click");
        timesLeft--;
        if (!get(clickingStore) || timesLeft == 0) {
          break;
        }
        await waitForTimeOrStop();
      }
    }
    clicking = false;
  });

  // @ts-expect-error: config.mode can only be either fixed or random
  const getTime = (): number => {
    if (config.mode === "fixed") {
      const minutes =
        config.options_for_fixed.m + config.options_for_fixed.h * 60;
      const seconds = config.options_for_fixed.s + minutes * 60;
      return config.options_for_fixed.ms + seconds * 1000;
    } else if (config.mode === "random") {
      return (
        Math.floor(
          Math.random() *
            (config.options_for_random.max - config.options_for_random.min + 1),
        ) + config.options_for_random.min
      );
    }
  };
  const waitForTimeOrStop = async () => {
    return new Promise<void>((res, rej) => {
      const timeout = setTimeout(() => {
        res();
        stop();
      }, getTime());
      const unsub = clickingStore.subscribe((v) => {
        if (v == false) {
          stop();
          rej();
        }
      });

      function stop() {
        unsub();
        clearTimeout(timeout);
      }
    });
  };

  const fixMinMax = () => {
    if (
      config.options_for_random.min >= config.options_for_random.max &&
      document.activeElement !== max_input
    ) {
      config.options_for_random.max = config.options_for_random.min + 1;
    }
  };
  $effect(fixMinMax);

  async function openPicker() {
    pickerWindow = new WebviewWindow("picker", {
      url: "/app/picker",
      transparent: true,
      acceptFirstMouse: true,
      decorations: false,
      x: 0,
      y: 0,
      closable: false,
      alwaysOnTop: true,
      minimizable: false,
      resizable: false,
    });
    await pickerWindow.once("tauri://webview-created", async (_) => {
      picking = true;
      await appWindow.minimize();
      pickerWindow?.listen("loaded", async () => {
        await pickerWindow?.emit("setkeybind", config.picker_keybind);
      });
    });
    await pickerWindow.once("tauri://destroyed", async (_) => {
      picking = false;
      await appWindow.unminimize();
      pickerWindow = undefined;
    });
    await pickerWindow.listen<[number, number]>("mouse-val", async (e) => {
      config.mouse_pos = { x: e.payload[0], y: e.payload[1] };
    });
  }

  function fixedAnchor(a: HTMLAnchorElement) {
    a.onclick = async (e) => {
      e.preventDefault();
      await openUrl(a.href);
    };
  }

  function disabledCandidate(value: boolean = false) {
    return picking || clicking || value;
  }

  class SavedState {
    mode = $state<"random" | "fixed">("random");
    options_for_fixed = $state({ h: 0, m: 0, s: 0, ms: 0 });
    options_for_random = $state({ min: 0, max: 0 });

    repeat_type = $state<"forever" | "number">("forever");
    repeat_x_count = $state(10);
    button = $state<"left" | "middle" | "right">("left");
    quantity = $state<"single" | "double">("single");

    do_mouse_pos = $state(false);
    mouse_pos = $state({ x: 0, y: 0 });

    backend__double = $derived(this.quantity === "double");
    backend__button = $derived.by(() => {
      if (this.button === "left") return 0;
      else if (this.button === "middle") return 1;
      else if (this.button === "right") return 2;
    });

    main_keybind = $state("Alt+C");
    picker_keybind = $state("K");

    constructor() {
      let first = true;
      $effect(() => {
        if (first) {
          const loaded_x_count = localStorage.getItem("repeat_count");
          const loaded_fixed_options = localStorage.getItem("fixed");
          const loaded_random_options = localStorage.getItem("random");
          const loaded_mouse_pos = localStorage.getItem("pos");
          const loaded_do_mouse_pos = localStorage.getItem("use_mouse_pos");

          this.mode = localStorage.getItem("mode") || this.mode;
          this.options_for_fixed = loaded_fixed_options
            ? JSON.parse(loaded_fixed_options)
            : this.options_for_fixed;
          this.options_for_random = loaded_random_options
            ? JSON.parse(loaded_random_options)
            : this.options_for_random;

          this.repeat_x_count = loaded_x_count
            ? JSON.parse(loaded_x_count)
            : this.repeat_x_count;

          this.repeat_type =
            localStorage.getItem("repeat_type") || this.repeat_type;
          this.button = localStorage.getItem("mouse_button") || this.button;
          this.quantity = localStorage.getItem("quantity") || this.quantity;

          this.mouse_pos = loaded_mouse_pos
            ? JSON.parse(loaded_mouse_pos)
            : this.mouse_pos;
          this.do_mouse_pos = loaded_do_mouse_pos
            ? JSON.parse(loaded_do_mouse_pos)
            : this.do_mouse_pos;

          first = false;
        }
        localStorage.setItem("mode", this.mode);
        localStorage.setItem("fixed", JSON.stringify(this.options_for_fixed));
        localStorage.setItem("random", JSON.stringify(this.options_for_random));

        localStorage.setItem("repeat_type", this.repeat_type);
        localStorage.setItem(
          "repeat_count",
          JSON.stringify(this.repeat_x_count),
        );
        localStorage.setItem("mouse_button", this.button);
        localStorage.setItem("quantity", this.quantity);
        localStorage.setItem("pos", JSON.stringify(this.mouse_pos));

        localStorage.setItem(
          "use_mouse_pos",
          JSON.stringify(this.do_mouse_pos),
        );
      });
    }
  }
  let config = new SavedState();

  let prev_main_kb = config.main_keybind;
  let prev_picker_kb = config.picker_keybind;
  let lastPKB_press: Date | null = null;
  let lastMKB_press: Date | null = null;

  function toggleClicking() {
    clicking = !clicking;
  }
  function picker_kb(e: ShortcutEvent) {
    if (e.state === "Pressed") lastPKB_press = new Date();
    if (
      e.state === "Released" &&
      new Date().getTime() - lastPKB_press?.getTime()! <= 500 &&
      pickerWindow
    ) {
      pickerWindow.emit("keybind");
    }
  }

  function main_kb(e: ShortcutEvent) {
    if (e.state === "Pressed") lastMKB_press = new Date();
    if (
      e.state === "Released" &&
      new Date().getTime() - lastMKB_press?.getTime()! <= 500
    ) {
      toggleClicking();
    }
  }

  $effect(() => {
    register(config.main_keybind, main_kb).catch((r) => {
      console.log(r);
    });
    prev_main_kb = config.main_keybind;
    return () => {
      unregister(prev_main_kb);
    };
  });
  $effect(() => {
    if (picking) {
      register(config.picker_keybind, picker_kb);
      register("ESC", (e) => {
        if (e.state === "Pressed") {
          pickerWindow?.close();
        }
      });
    } else {
      unregister(prev_picker_kb);
      unregister("ESC");
    }
    prev_picker_kb = config.picker_keybind;
    return () => {
      unregister(prev_picker_kb);
      unregister("ESC");
    };
  });
</script>

<div
  id="section-click-interval"
  class="bg-base-200 h-48 rounded-box px-1 flex flex-col pb-1"
>
  <span class="text-neutral-content uppercase font-appbartop ml-1"
    ><span class="border-b">click interval</span></span
  >
  <Tabs disabled={disabledCandidate()} bind:tab={config.mode}>
    {#snippet _random()}
      <div id="tab-random-click-interval" class="flex flex-col grow">
        <div class="join w-max">
          <label
            class="input input-sm input-bordered items-center gap-2 join-item"
          >
            min: <input
              disabled={disabledCandidate()}
              type="number"
              min="0"
              bind:value={config.options_for_random.min}
            />ms
          </label>
          <label
            class="input input-sm input-bordered items-center gap-2 join-item"
          >
            max: <input
              disabled={disabledCandidate()}
              bind:this={max_input}
              type="number"
              min={config.options_for_random.min + 1}
              onchange={fixMinMax}
              bind:value={config.options_for_random.max}
            />ms
          </label>
        </div>
        <div class="grow"></div>
        <span class="text-xs">
          Time between each click will be random number between the 2 values</span
        >
      </div>
    {/snippet}
    {#snippet _fixed()}
      <div id="tab-random-click-interval" class="flex flex-col grow">
        <div class="flex">
          <div class="grid grid-cols-2 grid-rows-2">
            <label
              class="rounded-r-none rounded-b-none input input-sm input-bordered items-center flex gap-2"
            >
              <input
                disabled={disabledCandidate()}
                class="grow"
                type="number"
                min="0"
                bind:value={config.options_for_fixed.h}
              />h
            </label>
            <label
              class="rounded-l-none rounded-b-none input input-sm input-bordered items-center flex gap-2"
            >
              <input
                disabled={disabledCandidate()}
                class="grow"
                type="number"
                min="0"
                bind:value={config.options_for_fixed.m}
              />m
            </label>
            <label
              class="rounded-r-none rounded-t-none input input-sm input-bordered items-center flex gap-2"
            >
              <input
                disabled={disabledCandidate()}
                class="grow"
                type="number"
                min="0"
                bind:value={config.options_for_fixed.s}
              />s
            </label>
            <label
              class="rounded-l-none rounded-t-none input input-sm input-bordered items-center flex gap-2"
            >
              <input
                disabled={disabledCandidate()}
                class="grow"
                type="number"
                min="0"
                bind:value={config.options_for_fixed.ms}
              />ms
            </label>
          </div>
        </div>
        <div class="grow"></div>
        <span class="text-xs"
          >Time between click will be exactly the above time</span
        >
      </div>
    {/snippet}
  </Tabs>
</div>

<div
  id="section-mouse-position"
  class="bg-base-200 flex flex-row pl-1 pr-2 mt-2 gap-x-2 h-10 items-center rounded-box"
>
  <span class="text-neutral-content uppercase font-appbartop ml-1 mb-1"
    ><span class="border-b">mouse position</span></span
  >
  <input
    type="checkbox"
    disabled={disabledCandidate()}
    bind:checked={config.do_mouse_pos}
    class="checkbox checkbox-primary"
  />
  <div class="join">
    <label
      class:input-disabled={disabledCandidate(!config.do_mouse_pos)}
      class="input input-xs input-bordered items-center gap-2 join-item"
    >
      x <input
        disabled={disabledCandidate(!config.do_mouse_pos)}
        type="number"
        min="0"
        bind:value={config.mouse_pos.x}
      />
    </label>
    <label
      class:input-disabled={disabledCandidate(!config.do_mouse_pos)}
      class="input input-xs input-bordered items-center gap-2 join-item"
    >
      y <input
        disabled={disabledCandidate(!config.do_mouse_pos)}
        type="number"
        min="0"
        bind:value={config.mouse_pos.y}
      />
    </label>
  </div>
  <button
    disabled={disabledCandidate(!config.do_mouse_pos)}
    onclick={openPicker}
    class="btn btn-xs btn-primary grow uppercase">pick</button
  >
</div>

<div id="click-options" class="grid grid-cols-2 gap-x-2 mt-2 h-28">
  <div
    id="section-click-type"
    class="flex flex-col pl-1 bg-base-200 rounded-box"
  >
    <span class="text-neutral-content uppercase font-appbartop ml-1 mb-1"
      ><span class="border-b">click type</span></span
    >
    <Tabs disabled={disabledCandidate()} bind:tab={config.button}>
      {#snippet _left()}{/snippet}
      {#snippet _middle()}{/snippet}
      {#snippet _right()}{/snippet}
    </Tabs>
    <Tabs disabled={disabledCandidate()} bind:tab={config.quantity}>
      {#snippet _single()}{/snippet}
      {#snippet _double()}{/snippet}
    </Tabs>
  </div>
  <div id="section-repeat" class="bg-base-200 pl-1 flex flex-col rounded-box">
    <span class="text-neutral-content uppercase font-appbartop ml-1 mb-1"
      ><span class="border-b">repeat</span></span
    >
    <Radio disabled={disabledCandidate()} bind:value={config.repeat_type}>
      {#snippet _forever()}
        Until Stopped
      {/snippet}
      {#snippet _number(checked)}
        <input
          class="input input-xs input-bordered"
          type="number"
          min="0"
          disabled={disabledCandidate(!checked)}
          bind:value={config.repeat_x_count}
        /> Times
      {/snippet}
    </Radio>
  </div>
</div>

<div
  id="main-buttons"
  class="grid grid-cols-2 gap-4 h-16 mt-2 p-2 bg-base-200 rounded-box"
>
  <button class="btn btn-block btn-neutral" onclick={toggleClicking}
    >{#if clicking}Stop{:else}Start{/if}
    <kbd class="kbd">{config.main_keybind}</kbd></button
  >
  <button
    title="Not implemented"
    disabled={disabledCandidate(true)}
    class="btn btn-block btn-neutral">Settings</button
  >
</div>

<div
  id="info"
  class="flex flex-row h-8 bg-base-200 mt-2 items-center px-2 rounded-box"
>
  <div class="flex-grow flex">
    <a use:fixedAnchor href="https://github.com/mavdotjs/lambda-autoclicker">
      <svg viewBox="0 0 96 96" class="size-6" xmlns="http://www.w3.org/2000/svg"
        ><path
          fill-rule="evenodd"
          clip-rule="evenodd"
          d="M48.854 0C21.839 0 0 22 0 49.217c0 21.756 13.993 40.172 33.405 46.69 2.427.49 3.316-1.059 3.316-2.362 0-1.141-.08-5.052-.08-9.127-13.59 2.934-16.42-5.867-16.42-5.867-2.184-5.704-5.42-7.17-5.42-7.17-4.448-3.015.324-3.015.324-3.015 4.934.326 7.523 5.052 7.523 5.052 4.367 7.496 11.404 5.378 14.235 4.074.404-3.178 1.699-5.378 3.074-6.6-10.839-1.141-22.243-5.378-22.243-24.283 0-5.378 1.94-9.778 5.014-13.2-.485-1.222-2.184-6.275.486-13.038 0 0 4.125-1.304 13.426 5.052a46.97 46.97 0 0 1 12.214-1.63c4.125 0 8.33.571 12.213 1.63 9.302-6.356 13.427-5.052 13.427-5.052 2.67 6.763.97 11.816.485 13.038 3.155 3.422 5.015 7.822 5.015 13.2 0 18.905-11.404 23.06-22.324 24.283 1.78 1.548 3.316 4.481 3.316 9.126 0 6.6-.08 11.897-.08 13.526 0 1.304.89 2.853 3.316 2.364 19.412-6.52 33.405-24.935 33.405-46.691C97.707 22 75.788 0 48.854 0z"
          fill="#fff"
        /></svg
      >
    </a>
  </div>
  <span class="text-neutral-content uppercase font-appbartop ml-2"
    ><span class="border-b">© 2026 bronti.</span></span
  >
</div>
