<script>
    import { onMount } from "svelte";

    export let treeData;
    export let rootIdProp;

    let mounted = false;

    $: {
        if (mounted) {
            renderTreeFromArray(treeData, "tc", rootIdProp);
        }
    }

    // Function to convert the array representation to a tree data structure
    function buildTree(nodes, parentId) {
        const tree = [];
        console.log({
            nodes,
            parentId,
        });

        nodes.forEach((node) => {
            if (node.id === parentId) {
                const children = [];
                node.children.forEach((childId) => {
                    if (childId !== 0) {
                        children.push(...buildTree(nodes, childId));
                    }
                });

                tree.push({ ...node, children });
            }
        });

        return tree;
    }

    // Function to render the tree using HTML and CSS
    function renderTree(tree, container) {
        const ul = document.createElement("ul");

        tree.forEach((node) => {
            const li = document.createElement("li");
            const a = document.createElement("a");

            const span = document.createElement("span");

            const p1 = document.createElement("p");
            const p2 = document.createElement("p");

            p1.textContent = `${node.id}`;
            p2.textContent = `${node.id}`;

            span.appendChild(p1);
            span.appendChild(p2);

            a.appendChild(span);

            a.style.backgroundColor = node.color;
            li.appendChild(a);

            if (node.children.length > 0) {
                renderTree(node.children, li);
            }

            ul.appendChild(li);
        });

        container.appendChild(ul);
    }

    // Entry point to build and render the tree
    function renderTreeFromArray(treeArray, containerId, rootId) {
        const container = document.getElementById(containerId);
        container.innerHTML = "";

        const tree = buildTree(treeArray, rootId); // Assuming the root node has id 1

        renderTree(tree, container);
    }

    onMount(() => {
        console.log("calling on mount");
        renderTreeFromArray(treeData, "tc", rootIdProp);
        mounted = true;
    });

    // // Call the function with the container ID
</script>

<article>
    <div class="container">
        <div class="row">
            <div class="tree" id="tc" />
        </div>
    </div>
</article>
