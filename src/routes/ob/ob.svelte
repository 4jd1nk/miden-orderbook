
<!-- OrderBook.svelte -->
<script>
	// You can replace the sample data with your real data
	let buyOrders = [
		{ price: 1000.0, amount: 0.5 },
		{ price: 999.0, amount: 1.2 },

		{ price: 980.5, amount: 0.8 },
		{ price: 1020.0, amount: 0.3 },
		{ price: 1005.5, amount: 0.7 },
		{ price: 1015.75, amount: 0.6 },
		{ price: 1012.25, amount: 0.5 },

		{ price: 990.5, amount: 0.9 },
		{ price: 1030.25, amount: 0.4 },
		{ price: 1028.5, amount: 0.5 }
	];

	// Sample Sell Orders
	let sellOrders = [
		{ price: 1020.0, amount: 0.3 },

		{ price: 1025.0, amount: 0.8 },
		{ price: 1032.75, amount: 0.6 },
		{ price: 1008.5, amount: 0.4 },
		{ price: 1010.25, amount: 0.7 },
		{ price: 1015.5, amount: 0.9 },
		{ price: 1022.0, amount: 0.5 },
		{ price: 1035.25, amount: 0.2 },
		{ price: 1005.5, amount: 0.5 },

		{ price: 1018.75, amount: 0.3 }
	];

	export function sort(orderBook, direction) {
		if (direction === 'asc') {
			return orderBook.sort((a, b) => a.price - b.price);
		} else {
			return orderBook.sort((a, b) => b.price - a.price);
		}
	}

	export function getSpread(buyOrders, sellOrders) {
		let highestBuy = buyOrders[0].price;
		let lowestSell = sellOrders[0].price;
		return lowestSell - highestBuy;
	}

	function getCellFillPercent(orders, direction) {}
</script>

<div class="max-w-[400px] h-full">
	<div class="flex flex-col space-y-1">
		<!-- Order Book - Sell Side -->
		<div class="bg-base-300 p-4 rounded-lg shadow">
			<h2 class="text-lg font-semibold mb-4">Sell Orders</h2>

			<table class="w-full">
				<thead>
					<tr>     
						<th class="text-left font-mono text-xs">Price</th>

						<th class="text-right font-mono text-xs">Amount</th>
					</tr>
				</thead>
				<tbody>
					{#each sort(sellOrders, 'asc') as order}
						<tr class="my-2">
							<td class="text-red-400 font-mono text-sm">{order.price.toFixed(2)}</td>
							<td class="text-right font-mono font-weight text-sm">{order.amount.toFixed(2)} BTC</td
							>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	<div class="bg-base-300 flex-initial p-4 rounded-lg shadow">
		<div class="flex justify-between">
			<div class="text-sm font-semibold">Spread</div>
			<div class="text-sm font-semibold">{getSpread(buyOrders, sellOrders).toFixed(2)}</div>
		</div>
	</div>

	<!-- Order Book - Buy Side -->
	<div class="bg-base-300 p-4 rounded-lg shadow">
		<h2 class="text-lg font-semibold mb-4">Buy Orders</h2>
		<table class="w-full">
			<thead>
				<tr>     
					<th class="text-left font-mono text-xs">Price</th>
					<th class="text-right font-mono text-xs">Amount</th>
				</tr>
			</thead>
			<tbody>
				{#each sort(buyOrders, 'desc') as order}
					<tr>
						<td class="text-green-400 font-mono text-sm">{order.price.toFixed(2)}</td>
						<td class="text-right font-mono font-weight text-sm"> {order.amount.toFixed(2)} BTC</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>

	</div>
</div>

<style>
	/* You can add additional custom styling here if needed */
</style>

