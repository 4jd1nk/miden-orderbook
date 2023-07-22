import init, {Outputs, prove_program} from "miden-vm";
import initData from "./data.json";
import fs from "fs";
import RbTree from "red-black-tree-js";

let treeInitiated = false;
let rbTree:any;
let inputData:typeof initData;

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

interface InputFile {
    operand_stack : string[],
    advice_map : Map<string, number[]>
}

export enum Side {
    Buy,
    Sell
}

function BNtoNumber(n: bigint) {
    return Number(n);
}

export async function createOrder(quantity: number, price: number | null, side : Side) {
    /*try {
        fs.writeFile("./data1.json", JSON.stringify("hello"), (err : any) => {
            if (err) console.log('Error writing file:', err);
        });
    }
    catch (Error) {
        console.log(Error);
    }*/

    if(!treeInitiated) {
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
        }
        treeInitiated = true;
    }

    await init();
 

    inputData.operand_stack[0] = side.toString();
    inputData.operand_stack[1] = quantity.toString();
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
    inputData.operand_stack[3] = Math.floor((new Date().getTime()) /1000).toString();
    inputData.operand_stack[4] = "9999999"; //TBI

    const inputs = `{
        "operand_stack": ["0"],
        "advice_stack": ["0"]
    }`;

    const { stack_output, trace_len, overflow_addrs, proof }: Outputs =
        prove_program(JSON.stringify(inputData));

    const sProof = Array.from(proof!)
        .map((b) => b.toString(16).padStart(2, "0"))
        .join("");

        console.log(trace_len);

        const outputArray = Array.from(stack_output);

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

        const iterator = rbTree.createIterator();
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
            i++;
        }

        newAdviceMap[keyPrefix+keySuffix] = [0, 0, 0, i-1];
        inputData.advice_map = newAdviceMap;
       
        /*const newAdviceArray = rbTree.toSortedArray();

        for(let i=0; i<newAdviceArray.length; i++) {

        }
        var result = newAdviceMap.reduce(function(map: any, obj:any) {
            map[obj.key] = obj.value;
            return map;
        }, {});
        console.log(result);*/

       
        
    return sProof;




}