<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
	import { currentMonitor } from '@tauri-apps/api/window';
	import { onMount } from 'svelte';
	let keybind = $state('<none>');
    const appWindow = getCurrentWebviewWindow();
	appWindow.listen<string>('setkeybind', (e) => {
		keybind = e.payload;
	});
	onMount(async () => {
		const screenSize = (await currentMonitor())?.size!;
		await getCurrentWebviewWindow().setSize(screenSize);
		await appWindow.listen('keybind', set_current_mouse);
		await appWindow.emit('loaded')
	});

	async function keyPress(e: KeyboardEvent) {
		await appWindow.emit('log', e.key);
		if (e.key === 'Escape') {
			await appWindow.close();
		}
	}
	1;
	async function click() {
		await set_current_mouse();
	}

	async function set_current_mouse() {
		const mouse_pos = await invoke('mouse_pos');
		await appWindow.emit('mouse-val', mouse_pos);
		await appWindow.close();
	}
</script>

<div
	class="cursor-move !h-full w-full grid grid-rows-3 select-none justify-center items-center text-base-content bg-base-100/30"
>
	<div></div>
	<span class="hover:opacity-10 transition text-center contents-end">
		Click anywhere, or Press {keybind} to select a position.<br />
		ESC to cancel.
	</span>
	<div></div>
</div>

<svelte:window onclick={click} onkeypress={keyPress} />
