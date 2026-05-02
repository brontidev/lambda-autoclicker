<script lang="ts">
  import Tabs from "$lib/Tabs.svelte";
  import { openUrl } from "@tauri-apps/plugin-opener";
  import Radio from "$lib/Radio.svelte";
  import {
    register,
    unregister,
    type ShortcutEvent,
  } from "@tauri-apps/plugin-global-shortcut";

  import config from "$lib/settings.svelte";
  import {
    interval_mode_to_string,
    string_to_interval_mode,
    repeat_mode_to_string,
    string_to_repeat_mode,
    click_button_to_string,
    string_to_click_button,
    click_quantity_to_string,
    string_to_click_quantity,
  } from "$lib/types/settings";
  import { invoke } from "@tauri-apps/api/core";
  import clicking from "$lib/clicking.svelte";

  let max_input: HTMLInputElement;

  const fixMinMax = () => {
    if (
      config.current.interval_rand_options.min >= config.current.interval_rand_options.max &&
      document.activeElement !== max_input
    ) {
      config.current.interval_rand_options.max = config.current.interval_rand_options.min + 1;
    }
  };
  $effect(fixMinMax);

  async function openPicker() {
    invoke("open_picker")
  }

  function fixedAnchor(a: HTMLAnchorElement) {
    a.onclick = async (e) => {
      e.preventDefault();
      await openUrl(a.href);
    };
  }

  function disabledCandidate(value: boolean = false) {
    return clicking.is_clicking || value;
  }

  let lastMKB_press: Date | null = null;

  function main_kb(e: ShortcutEvent) {
    if (e.state === "Pressed") lastMKB_press = new Date();
    if (
      e.state === "Released" &&
      new Date().getTime() - lastMKB_press?.getTime()! <= 500
    ) {
      clicking.toggle()
    }
  }

  $effect(() => {
    register("Alt+C", main_kb)
    return () => {
      unregister("Alt+C");
    };
  });
  // $effect(() => {
  //   if (picking) {
  //     register("K", picker_kb);
  //     register("ESC", (e) => {
  //       if (e.state === "Pressed") {
  //         pickerWindow?.close();
  //       }
  //     });
  //   } else {
  //     unregister("K");
  //     unregister("ESC");
  //   }
  //   return () => {
  //     unregister("K");
  //     unregister("ESC");
  //   };
  // });
</script>

<div
  id="section-click-interval"
  class="bg-base-200/50 h-48 rounded-box px-1 flex flex-col pb-1"
>
  <span class="text-neutral-content uppercase font-appbartop ml-1"
    ><span class="border-b">click interval</span></span
  >
  <Tabs disabled={disabledCandidate()} bind:tab={() => interval_mode_to_string(config.current.interval_mode), (v) => config.current.interval_mode = string_to_interval_mode(v)}>
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
              bind:value={config.current.interval_rand_options.min}
            />ms
          </label>
          <label
            class="input input-sm input-bordered items-center gap-2 join-item"
          >
            max: <input
              disabled={disabledCandidate()}
              bind:this={max_input}
              type="number"
              min={config.current.interval_rand_options.min + 1}
              onchange={fixMinMax}
              bind:value={config.current.interval_rand_options.max}
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
                bind:value={config.current.interval_fixed_options.hours}
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
                bind:value={config.current.interval_fixed_options.minutes}
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
                bind:value={config.current.interval_fixed_options.seconds}
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
                bind:value={config.current.interval_fixed_options.milliseconds}
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
  class="bg-base-200/50 flex flex-row pl-1 pr-2 mt-2 gap-x-2 h-10 items-center rounded-box"
>
  <span class="text-neutral-content uppercase font-appbartop ml-1 mb-1"
    ><span class="border-b">mouse position</span></span
  >
  <input
    type="checkbox"
    disabled={disabledCandidate()}
    bind:checked={config.current.set_mouse_position}
    class="checkbox checkbox-primary"
  />
  <div class="join">
    <label
      class:input-disabled={disabledCandidate(!config.current.set_mouse_position)}
      class="input input-xs input-bordered items-center gap-2 join-item"
    >
      x <input
        disabled={disabledCandidate(!config.current.set_mouse_position)}
        type="number"
        min="0"
        bind:value={config.current.mouse_position.x}
      />
    </label>
    <label
      class:input-disabled={disabledCandidate(!config.current.set_mouse_position)}
      class="input input-xs input-bordered items-center gap-2 join-item"
    >
      y <input
        disabled={disabledCandidate(!config.current.set_mouse_position)}
        type="number"
        min="0"
        bind:value={config.current.mouse_position.y}
      />
    </label>
  </div>
  <button
    disabled={disabledCandidate(!config.current.set_mouse_position)}
    onclick={openPicker}
    class="btn btn-xs btn-primary grow uppercase">pick</button
  >
</div>

<div id="click-options" class="grid grid-cols-2 gap-x-2 mt-2 h-28">
  <div
    id="section-click-type"
    class="flex flex-col pl-1 bg-base-200/50 rounded-box"
  >
    <span class="text-neutral-content uppercase font-appbartop ml-1 mb-1"
      ><span class="border-b">click type</span></span
    >
    <Tabs disabled={disabledCandidate()} bind:tab={() => click_button_to_string(config.current.click_button), (v) => config.current.click_button = string_to_click_button(v)}>
      {#snippet _left()}{/snippet}
      {#snippet _middle()}{/snippet}
      {#snippet _right()}{/snippet}
    </Tabs>
    <Tabs disabled={disabledCandidate()} bind:tab={() => click_quantity_to_string(config.current.click_quantity), (v) => config.current.click_quantity = string_to_click_quantity(v)}>
      {#snippet _single()}{/snippet}
      {#snippet _double()}{/snippet}
    </Tabs>
  </div>
  <div id="section-repeat" class="bg-base-200/50 pl-1 flex flex-col rounded-box">
    <span class="text-neutral-content uppercase font-appbartop ml-1 mb-1"
      ><span class="border-b">repeat</span></span
    >
    <Radio disabled={disabledCandidate()} bind:value={() => repeat_mode_to_string(config.current.repeat_mode), (v) => config.current.repeat_mode = string_to_repeat_mode(v)}>
      {#snippet _forever()}
        Until Stopped
      {/snippet}
      {#snippet _count(checked)}
        <input
          class="input input-xs input-bordered"
          type="number"
          min="0"
          disabled={disabledCandidate(!checked)}
          bind:value={config.current.repeat_count}
        /> Times
      {/snippet}
    </Radio>
  </div>
</div>

<div
  id="main-buttons"
  class="grid grid-cols-2 gap-4 h-16 mt-2 p-2 bg-base-200/50 rounded-box"
>
  <button class="btn btn-block btn-neutral" onclick={clicking.toggle}
    >{#if clicking.is_clicking}Stop{:else}Start{/if}
    <kbd class="kbd">Alt/Opt+C</kbd></button
  >
  <button
    title="Not implemented"
    disabled={disabledCandidate(true)}
    class="btn btn-block btn-neutral">Settings</button
  >
</div>

<div
  id="info"
  class="flex flex-row h-8 bg-base-200/50 mt-2 items-center px-2 rounded-box"
>
  <div class="grow flex">
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
  <span class="text-neutral-content font-appbartop ml-2"
    ><span class="border-b">© 2026 bronti.</span></span
  >
</div>
