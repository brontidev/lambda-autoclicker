<script lang="ts">
	import type { Snippet } from 'svelte';

	let { value = $bindable(), disabled = $bindable(false), ...props }: {
        value: string | undefined;
        disabled: boolean;
        [x: `_${string}`]: Snippet<[boolean]>;
	} = $props();
	const options = Object.fromEntries(Object.entries(props).filter((v) => v[0].startsWith('_')).map(([name, snippet]): [string, Snippet<[boolean]>] => [name.replace('_', ''), snippet]));
</script>

{#each Object.entries(options) as [option, snippet]}
<label class="flex items-center gap-x-1 mb-1">
    <input class="radio radio-primary" type="radio" {disabled} checked={value === option} onchange={() => value = option} />
    {@render snippet(value === option)}
</label>
{/each}