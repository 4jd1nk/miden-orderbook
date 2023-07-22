<script>

import * as orderService from "../../services/orderService";

	export let termAccepted = false;
	export let started = false;

	export async function clickFn() {
		await orderService.createOrder(3, 5, 0);
	}
</script>

<article class="container py-16 prose prose-lg prose-stone">
	<h1>WASM Miden</h1>
	<p class="p">
		This is a playground for WebAssembly. It is a work in progress. It should test miden-clob crate
		via wasm. miden-clob crate runs `rbt.masm` and bridges output
	</p>
	<div class="py-2 mb-4">
		{#if started}
			Start here
		{:else}
			<div class="align-middle">
				<input
					type="checkbox"
					class="checkbox checkbox-primary align-text-top mr-2"
					bind:checked={termAccepted}
				/>
				<span class="">I understand that this is experimental</span>
			</div>
			<button disabled={!termAccepted} class="btn btn-primary mt-4 btn-md" on:click={await clickFn}>Start</button>
		{/if}
	</div>
</article>
