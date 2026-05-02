<script lang="ts">
  import { emit, listen } from "@tauri-apps/api/event";
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
  import { currentMonitor } from "@tauri-apps/api/window";
  import { onMount } from "svelte";
  import {
    register,
    unregister,
    type ShortcutEvent,
  } from "@tauri-apps/plugin-global-shortcut";

  const window = getCurrentWebviewWindow();
  onMount(() => {
    register("K", keybind);
    listen("pick", pick);
    set_window_size();
  });

  async function set_window_size() {
    const screenSize = (await currentMonitor())?.workArea.size!;
    await window.setSize(screenSize);
    setTimeout(() => {
      window.show();
    }, 20);
  }

  let last_keybind_press: Date | null = null;

  function keybind(e: ShortcutEvent) {
    if (e.state === "Pressed") last_keybind_press = new Date();
    if (
      e.state === "Released" &&
      new Date().getTime() - last_keybind_press?.getTime()! <= 500
    ) {
      pick();
    }
  }

  async function keyPress(e: KeyboardEvent) {
    await window.emit("log", e.key);
    if (e.key === "Escape") {
      await window.close();
    }
  }

  async function pick() {
    unregister("K");
    emit("picked");
    window.close();
  }
</script>

<div
  class="cursor-move h-full! w-full grid grid-rows-3 select-none justify-center items-center text-base-content bg-base-100/30"
>
  <div></div>
  <span class="hover:opacity-10 transition text-center contents-end">
    Click anywhere (or Press K) to select a position.<br />
    ESC to cancel.
  </span>
  <div></div>
</div>

<svelte:window onclick={pick} onkeypress={keyPress} />
