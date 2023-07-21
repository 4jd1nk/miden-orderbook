<script>
	import Ob from './ob.svelte';

export const options = {
  width: 980,
  height: 610,
  symbol: "BINANCE:BTCUSDT",
  interval: "D",
  timezone: "Etc/UTC",
  theme: "dark",
  style: "1",
  locale: "en",
  toolbar_bg: "#f1f3f6",
  enable_publishing: false,
  allow_symbol_change: true,
  container_id: "tradingview_9b6a4"
};


</script>

<div>
	<div class="bg-base-300 w- full h-[64px] mt-4 rounded-xl mb-4" />

	<div class="flex flex-row gap-4">
		<div class="bg-base-300 p-4 h-full w-1/4 min-w-[400px] rounded-xl" />
		<div class="bg-base-300 p-4 h-full w-1/4 min-w-[400px] rounded-xl">
			<Ob />
		</div>
		<div class="bg-base-300 p-4 h-full w-full rounded-xl">

        </div>
	</div>

</div>
<div class="bg-base-300 p-4 h-full mt-2 h-screen rounded-xl w-full">
</div>
