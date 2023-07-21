<script>
	export let counter = 1;
	export let data;
	export let loading;
	export async function fetchJson() {
		loading = true;
		const res = await fetch('https://jsonplaceholder.typicode.com/todos/1');
		data = await res.json();
		loading = false;
	}
</script>

<h1 class="text-3xl font-bold underline">Hello world!</h1>
<h3>Counter: <span class="text-red-500">{counter}</span></h3>
<div>
	<button class="btn py-2 px-4 rounded" on:click={() => (counter += 1)}>
		Increment
	</button>
	<button class="btn px-4 rounded" on:click={() => (counter -= 1)}> Decrement </button>
	<button class="btn px-4 rounded" on:click={async () => await fetchJson()}>
		Fetch
	</button>
	<button class="btn" onclick="my_modal_1.showModal()">open modal</button>
</div>
<div class="mockup-code min-h-64 mt-8 w-1/2">
	<pre class="min-h-[220px]">
    {#if loading || !data}
			Loading...
		{:else}
			<code>
    {JSON.stringify(data, null, 4).trim()}
    </code>
		{/if}
    </pre>

	<dialog id="my_modal_1" class="modal">
		<form method="dialog" class="modal-box">
			<h3 class="font-bold text-lg">Hello!</h3>
			<p class="py-4">Press ESC key or click the button below to close</p>
			<div class="modal-action">
				<!-- if there is a button in form, it will close the modal -->
				<button class="btn">Close</button>
			</div>
		</form>
	</dialog>

</div>

<style lang="postcss">
	:global(html) {
		background-color: theme(colors.gray.800);
	}
</style>
