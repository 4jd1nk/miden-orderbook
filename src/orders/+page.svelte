<script>
    import "./styles.css";
    import Tree from "../../components/Tree.svelte";
    import * as orderService from "../../services/orderService";
    import { Side } from "../../services/orderService";

    let price = "";
    let quantity = "";

    let isMarketOrder = false;
    let showVerified = false;

    let verificationSuccess = false;
    let proof = null;

    const getTreeProps = () => {
        const uiTree = orderService.getOrders();
        const rootNode = uiTree.find((node) => node.parent == 1);
        return {
            treeData: uiTree,
            rootIdProp: rootNode?.id,
        };
    };

    let treeProps = getTreeProps();

    const onCreateOrder = async () => {
        if (quantity && (isMarketOrder ? price : true)) {
            const { sProof } = await orderService.createOrder(
                isMarketOrder ? Number(quantity) : null,
                Number(price),
                Side.Buy
            );
            proof = sProof;
            treeProps = getTreeProps();
        }
    };

    const onVerify = () => {
        showVerified = true;

        // TODO call the verify function with the proof and set the verificationSuccess accordingly
        verificationSuccess = true;
    };
</script>

<article class=" flex mt-3">
    <div class=" flex-[1] p-2 flex flex-col gap-2">
        <div class=" p-3 flex flex-col bg-white/5 rounded-md">
            <h3 class=" text-xl mb-6">Create Order</h3>

            <div class=" flex flex-col gap-2">
                <div class="align-middle">
                    <input
                        type="checkbox"
                        class="checkbox checkbox-primary align-text-top mr-1"
                        bind:checked={isMarketOrder}
                    />
                    <span class="">Market Order</span>
                </div>
                {#if isMarketOrder}
                    <input
                        bind:value={price}
                        type="number"
                        placeholder="Price"
                        class=" flex flex-1 bg-transparent border border-white/10 p-3 py-1 rounded-md"
                    />
                {/if}

                <input
                    bind:value={quantity}
                    placeholder="Quantity"
                    type="number"
                    class=" flex flex-1 bg-transparent border border-white/10 p-3 py-1 rounded-md"
                />
                <input
                    placeholder="Order Type: Buy"
                    disabled
                    class=" flex flex-1 bg-transparent border border-white/10 p-3 py-1 rounded-md"
                />
            </div>
            <button
                on:click={onCreateOrder}
                class=" flex p-3 py-2 bg-blue-500 rounded-md justify-center text-white mt-7"
                >Create order</button
            >
        </div>
        <div class=" p-3 flex flex-col bg-white/5 rounded-md">
            <h3 class=" text-xl mb-6">Proof</h3>
            <textarea
                class="flex bg-transparent h-32 text-sm"
                disabled
                bind:value={proof}
            />
            <button
                on:click={onVerify}
                class=" flex p-3 py-2 bg-blue-500 rounded-md justify-center text-white mt-3"
                >Verify</button
            >
            {#if showVerified}
                {#if verificationSuccess}
                    <p class=" flex justify-center pt-4 text-green-400">
                        Verification successful!
                    </p>
                {:else}
                    <p class=" flex justify-center pt-4 text-red-500">
                        Verification failed!
                    </p>
                {/if}
            {/if}
        </div>
    </div>
    <div class=" flex-[3]">
        <p class=" flex justify-center text-2xl font-medium mb-4">Orders</p>
        <Tree {...treeProps} />
    </div>
</article>
