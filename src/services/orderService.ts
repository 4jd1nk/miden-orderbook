import init, {Outputs, prove_program, verify_program} from "miden-vm";
import initData from "./data.json";
import RbTree from "red-black-tree-js";

let treeInitiated = false;
let rbTree:any;
let inputData:typeof initData;
export let uiTree : UiNode[] = new Array();

interface OrderWord {
    quantity : number,
    price : number,
    timestamp : number,
    id: number
}

interface CoordinateWord {
    color: number,
    parentId: number,
    leftChildId: number,
    rightChildId: number
}

interface MemoryWord {
    pad1: number,
    pad2: number,
    pad3: number,
    memoryLocation: number
}

interface Node {
    memory : MemoryWord,
    coordinate : CoordinateWord,
    order : OrderWord
}

interface UiNode {
    id :number,
    price:number,
    children: number[],
    parent: number,
    quantity: number,
    color: string
}

export enum Side {
    Sell,
    Buy
}

function BNtoNumber(n: bigint) {
    return Number(n);
}

function initializeTrees() {
    if (treeInitiated)
        return;

    rbTree = new RbTree();
    inputData = initData;

    const nodeArray = Object.values(inputData.advice_map);
    for (let i = 1; i< nodeArray.length; i++) {
        const nodeEl = nodeArray[i];
        const node : Node = {
            memory : { pad1 : 0, pad2 : 0, pad3 : 0, memoryLocation : nodeEl[3]},
            coordinate: {
                color:nodeEl[4], 
                parentId: nodeEl[5], 
                leftChildId: nodeEl[6],
                rightChildId: nodeEl[7]
            },
            order: {
                quantity: nodeEl[8],
                price: nodeEl[9],
                timestamp: nodeEl[10],
                id: nodeEl[11]
            }
        };
        rbTree.insert(node.memory.memoryLocation, node);

        uiTree.push({
            id: node.memory.memoryLocation,
            price: node.order.price,
            children: [node.coordinate.leftChildId, node.coordinate.rightChildId],
            parent: node.coordinate.parentId,
            quantity: node.order.quantity,
            color: node.coordinate.color ? "red" : "black"
        });
    }
    treeInitiated = true;
}

export function getOrders() {
    initializeTrees();
    return uiTree;
}

function prove_program_mock(input:string) {
    //modification, deletion and addition
    //modification
    const outputStack = new BigUint64Array(45);
    outputStack[0] = 268n;    //memoryLocation
    outputStack[1] = 1n;      //color = red (changed)
    outputStack[2] = 259n;    //(parentId)
    outputStack[3] = 0n;      //leftChildId
    outputStack[4] = 0n;      //rightChildId
    outputStack[5] = 2n;      //quantity (changed)
    outputStack[6] = 49n;     //price
    outputStack[7] = 1688473018n;  //timestamp
    outputStack[8] = 4929032n;     //orderId

    //deleted node
    outputStack[9] = 274n;    //memoryLocation
    outputStack[10] = 0n;      //color = black
    outputStack[11] = 0n;    //(parentId)
    outputStack[12] = 0n;      //leftChildId
    outputStack[13] = 0n;      //rightChildId
    outputStack[14] = 0n;      //quantity
    outputStack[15] = 48n;     //price
    outputStack[16] = 1688473234n;  //timestamp
    outputStack[17] = 3248575n;     //orderId

    //parent of deleted node
    outputStack[18] = 271n;    //memoryLocation
    outputStack[19] = 0n;      //color = black
    outputStack[20] = 277n;    //(parentId)
    outputStack[21] = 280n;      //leftChildId
    outputStack[22] = 0n;      //rightChildId
    outputStack[23] = 40n;      //quantity
    outputStack[24] = 49n;     //price
    outputStack[25] = 1688473214n;  //timestamp
    outputStack[26] = 2697766n;     //orderId   

    //addition
    outputStack[27] = 300n;    //memoryLocation
    outputStack[28] = 1n;      //color = red 
    outputStack[29] = 286n;    //(parentId)
    outputStack[30] = 0n;      //leftChildId
    outputStack[31] = 0n;      //rightChildId
    outputStack[32] = 25n;      //quantity (changed)
    outputStack[33] = 49n;     //price
    outputStack[34] = 1688473018n;  //timestamp
    outputStack[35] = 4929032n;     //orderId

    //parent of added node
    outputStack[36] = 286n;    //memoryLocation
    outputStack[37] = 0n;      //color = black 
    outputStack[38] = 283n;    //(parentId)
    outputStack[39] = 300n;      //leftChildId
    outputStack[40] = 0n;      //rightChildId
    outputStack[41] = 5n;      //quantity (changed)
    outputStack[42] = 49n;     //price
    outputStack[43] = 1688474789n;  //timestamp
    outputStack[44] = 4306835n;     //orderId

    const proof = new Uint8Array(3);
    proof[0] = 200;
    proof[1] = 300;
    proof[2] = 400;

    const output : Outputs = { 
        stack_output : outputStack,
        overflow_addrs : outputStack,
        trace_len : 64,
        proof : proof,
        free : () => {}
    }
    return output;
}

export async function createOrder(quantity: number, price: number | null, side : Side) {
    initializeTrees();
    await init();
 
    //construct the operand stack as a new order
    inputData.operand_stack[4] = side.toString();
    inputData.operand_stack[3] = quantity.toString();
    let sPrice : string;
    if (price === null) {
        if (side == Side.Sell) {
            sPrice = "0";
        }
        else {
            sPrice = "18446744073709551615";
        }
    }
    else {
        sPrice = price.toString();
    }
    inputData.operand_stack[2] = sPrice;
    inputData.operand_stack[1] = Math.floor((new Date().getTime()) /1000).toString();
    inputData.operand_stack[0] = "9999999"; //TBI

    console.log(JSON.stringify(inputData));
    const { stack_output, trace_len, overflow_addrs, proof }: Outputs =
        prove_program(JSON.stringify(inputData));
    console.log("hello");
    const sProof = Array.from(proof!)
        .map((b) => b.toString(16).padStart(2, "0"))
        .join("");

    const outputArray = Array.from(stack_output);

    //update rbTree
    for(let i = 0; i < outputArray.length; i+=9) {
        if (i+8 >= outputArray.length)
            break;

        const newNode: Node = {
            memory : { pad1 : 0, pad2 : 0, pad3 : 0, memoryLocation : BNtoNumber(outputArray[i])},
            coordinate: {
                color:BNtoNumber(outputArray[i+1]), 
                parentId: BNtoNumber(outputArray[i+2]), 
                leftChildId: BNtoNumber(outputArray[i+3]),
                rightChildId: BNtoNumber(outputArray[i+4])
            },
            order: {
                quantity: BNtoNumber(outputArray[i+5]),
                price: BNtoNumber(outputArray[i+6]),
                timestamp: BNtoNumber(outputArray[i+7]),
                id: BNtoNumber(outputArray[i+8])
            }
        };

        let modifiedNode : Node = rbTree.find(newNode.memory.memoryLocation);
        if (modifiedNode === null) { //insert
            rbTree.insert(newNode.memory.memoryLocation, newNode);
        }
        else if(newNode.coordinate.parentId == 0 && newNode.coordinate.leftChildId == 0 && newNode.coordinate.rightChildId == 0) { //delete
            rbTree.remove(newNode.memory.memoryLocation);
        }
        else { //modified
            rbTree.update(newNode.memory.memoryLocation, newNode);
        }
    }

    //update uiTree and adviceMap based on rbTree
    const iterator = rbTree.createIterator();
    uiTree = new Array();

    let newAdviceMap : any= {};
    let i = 1;
    const keyPrefix = "00000000000000000000000000000000000000000000000000";
    const keySuffix = "00000000000000";

    while (iterator.hasNext()) {
        const element = iterator.next().value as Node;
        const ordinalNumber = i.toString(16);
        const key = keyPrefix.substring(0, keyPrefix.length - ordinalNumber.length) + ordinalNumber + keySuffix;
        const value = [0, 0, 0, element.memory.memoryLocation, 
            element.coordinate.color, element.coordinate.parentId, element.coordinate.leftChildId, element.coordinate.rightChildId,
            element.order.quantity, element.order.price, element.order.timestamp, element.order.id
        ];
        newAdviceMap[key] = value;

        uiTree.push({
            id: element.memory.memoryLocation,
            price: element.order.price,
            children: [element.coordinate.leftChildId, element.coordinate.rightChildId],
            parent: element.coordinate.parentId,
            quantity: element.order.quantity,
            color: element.coordinate.color ? "red" : "black"
        });

        i++;
    }

    newAdviceMap[keyPrefix+keySuffix] = [0, 0, 0, i-1];
    inputData.advice_map = newAdviceMap;

    console.log(uiTree);
    console.log(inputData);
    console.log(sProof);

    const inputOperandStack = JSON.stringify({
        "operand_stack" : inputData.operand_stack
    });

    return {sProof, inputOperandStack, stack_output, overflow_addrs, proof};
}

export function verifyProof(input : string, output : BigUint64Array, overflowAddress : BigUint64Array, proof : Uint8Array) {
    try {
        verify_program(input, proof, output, overflowAddress);
        return true;
    }
    catch (error) {
        return false;
    }
}