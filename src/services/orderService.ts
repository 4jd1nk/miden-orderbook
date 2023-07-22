import init, {Outputs, prove_program} from "miden-vm";
import inputData from "./data.json";
//const fs = require("browserify-fs");

interface OrderWord {
    quantity : string,
    price : string,
    timestamp : string,
    id: string
}

interface CoordinateWord {
    color: string,
    parentId: string,
    leftChildId: string,
    rightChildId: string
}

interface MemoryWord {
    pad1: number,
    pad2: number,
    pad3: number,
    memoryLocation: number
}

interface InputFile {
    operand_stack : string[],
    advice_map : Map<string, number[]>
}

export enum Side {
    Buy,
    Sell
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
    inputData.operand_stack[4] = (Math.floor(Math.random() * 10000000)).toString(); //TBI

    const inputs = `{
        "operand_stack": ["0"],
        "advice_stack": ["0"]
    }`;

    const { stack_output, trace_len, overflow_addrs, proof }: Outputs =
        prove_program(JSON.stringify(inputData));

    const sProof = Array.from(proof!)
        .map((b) => b.toString(16).padStart(2, "0"))
        .join("");

        console.log(sProof);
        console.log(stack_output)
        
    return sProof;



}