"use client"

import { Group, PayedType } from "@/types";
import { redirect } from "next/navigation";
import { FormEvent, useState } from "react";
import useSWR from "swr";



export default function AddExpense({ user_id, group, setShowAdd }: { user_id: number, group: Group, setShowAdd: any }) {
    const [payedType, setPayedType] = useState<PayedType>();
    const [name, setName] = useState<string>("");
    const [amount, setAmount] = useState<number>(0);


    function add(event: FormEvent<HTMLFormElement>): void {
        throw new Error("Function not implemented.");
        setShowAdd(false);
    }


    return (<>
        <div className="grid grid-rows-[20px_1fr_20px] items-cente
  r justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20 font-[family-name:var(--font-geist-sans)]">
            <main className="flex flex-col gap-8 row-start-2 items-center sm:items-start">
                <h1>Create a new Expense</h1>
                <form onSubmit={add} >
                    <input type="text" placeholder="Name" className="input input-bordered w-full max-w-xs" value={name} onChange={(e) => setName(e.target.value)} />
                    <input type="number" placeholder="Amount" className="input input-bordered w-full max-w-xs" value={amount} onChange={(e) => setAmount(parseInt(e.target.value))} />
                    <select>
                        <option>Payed 1</option>
                        <option>Payed 2</option>
                        <option>Payed 3</option>
                    </select>

                    <button className="btn btn-primary" type="submit">Add</button>
                    <button className="btn" type="reset" onClick={() => setShowAdd(false)}>Cancel</button>
                </form>
            </main>
        </div>
    </>
    );
}