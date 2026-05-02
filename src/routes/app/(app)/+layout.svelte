<script lang="ts">
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
  import { getVersion } from "@tauri-apps/api/app";
  import { onMount } from "svelte";
  const { children, data } = $props();
  import settings from "$lib/settings.svelte";

  let version = $state();
  const appWindow = getCurrentWebviewWindow();
  
  const settingsPromise = settings.init();

  onMount(async () => {
    version = await getVersion();
    await appWindow.setDecorations(false);
    await appWindow.setMinimizable(true);
  });
</script>

{#await settingsPromise then}
  <div class="h-full! w-full flex flex-col select-none pt-2 bg-base-100/70">
    <nav class="flex items-center mx-2 bg-base-200/70 pl-2 rounded-box grow-0">
      <!-- svelte-ignore a11y_interactive_supports_focus -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div class="grow flex justify-between items-center">
        <div
          class="grow flex justify-between"
          data-tauri-drag-region
          onmousedown={() => appWindow.startDragging()}
        >
          <div class="font-appbartop">
            <abbr title="Lambds">λ</abbr> Autoclicker
            <span class="text-slate-500 font-sans lowercase">v{version}</span>
          </div>
        </div>
        <div class="join">
          <button
            class="join-item btn btn-xs btn-warning btn-square btn-outline no-animation"
            onclick={() => appWindow.minimize()}
          >
            ♢
          </button>
          <button
            class="join-item btn btn-xs btn-error btn-square btn-outline no-animation"
            onclick={() => appWindow.close()}
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              stroke-width="1.5"
              stroke="currentColor"
              class="size-4"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M6 18L18 6M6 6l12 12"
              />
            </svg>
          </button>
        </div>
      </div>
    </nav>
    <main class="mx-2 mt-1 mb-2 h-fit">{@render children()}</main>
  </div>
{/await}
