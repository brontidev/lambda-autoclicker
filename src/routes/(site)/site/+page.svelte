<script lang="ts">
	import Tabs from '$lib/Tabs.svelte';
	import { onMount } from 'svelte';

	function detectOS() {
		let userAgent = window.navigator.userAgent,
			platform = window.navigator.platform,
			macosPlatforms = ['Macintosh', 'MacIntel', 'MacPPC', 'Mac68K'],
			windowsPlatforms = ['Win32', 'Win64', 'Windows', 'WinCE'];

		if (macosPlatforms.indexOf(platform) !== -1) {
			return 'mac';
		} else if (windowsPlatforms.indexOf(platform) !== -1) {
			return 'win';
		} else if (/Linux/.test(platform)) {
			return 'lin';
		}

		return null;
	}
	let platform = $state<'win' | 'lin' | 'mac'>('win');
	let macChip = $state<'intel' | 'silicon'>('silicon');
	const platformMap = { win: 'Windows', mac: 'Mac OS', lin: 'Linux' };
	const chipMap = { intel: 'Intel', silicon: 'M1 or M2' };
	const chipArchMap = { intel: 'x86', silicon: 'AArch64' };
	onMount(() => {
		platform = detectOS() || platform;
	});
</script>

<div class="!h-full w-full flex flex-col select-none pt-2 bg-base-100">
	<nav class="flex items-center mx-2 bg-base-200 pl-2 rounded-box grow-0">
		<!-- svelte-ignore a11y_interactive_supports_focus -->
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div class="flex-grow flex justify-between items-center">
			<div class="flex-grow flex justify-between">
				<span class="font-appbartop"> <abbr title="Lambda">λ</abbr> Autoclicker</span>
			</div>
		</div>
	</nav>
	<main class="mx-2 mt-1 mb-2 h-full flex flex-col">
		<!-- depends on if I wanna put more stuff here or not: -->
		<!-- <div class="flex-grow"></div> -->
		<div class="bg-base-200 rounded-box px-1 h-80 grid pb-1 pl-1 mb-2 grid-cols-2">
			<div class="flex flex-col prose !max-w-none">
				<span class="text-neutral uppercase font-appbartop ml-1"
					><span class="border-b">features</span></span
				>
				You can:
				<ul>
					<li>Set a specific point for the cursor</li>
					<li><s title="Not yet implemented">Choose a custom keybind </s> (it works even if the app is not in the foreground)</li>
					<li>Select the mouse button that will be clicked</li>
					<li>Use single or double click</li>
					<li>Repeat a certain number of times, or Until stopped</li>
				</ul>
			</div>
			<div class="flex items-center justify-center">
				<img src="/screenshot.png" alt="" class="h-72">
			</div>
		</div>
		<div class="bg-base-200 rounded-box px-1 h-44 flex flex-col pb-1 pl-1">
			<span class="text-neutral uppercase font-appbartop ml-1"
				><span class="border-b">download</span></span
			>
			<Tabs bind:tab={platform} map={platformMap} class="mb-2">
				{#snippet _win()}{/snippet}
				{#snippet _mac()}
					<Tabs bind:tab={macChip} map={chipMap} class="mb-2">
						{#snippet _intel()}{/snippet}
						{#snippet _silicon()}{/snippet}
					</Tabs>
				{/snippet}
				{#snippet _lin()}{/snippet}
			</Tabs>
			<button class="btn btn-block btn-primary"
				>Download v0.0.1 for {platformMap[platform]}
				{#if platform === 'mac'}({chipArchMap[macChip]}){/if}</button
			>
		</div>
		<div class="flex-grow"></div>
		<div id="info" class="flex flex-row h-8 bg-base-200 mt-2 items-center px-2 rounded-box">
			<div class="flex-grow flex">
				<a target="_blank" href="https://github.com/mavdotjs/lambda-autoclicker">
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
			<span class="text-neutral uppercase font-appbartop ml-2"
				><span class="border-b">© 2024 mavdotjs.</span></span
			>
		</div>
	</main>
</div>
