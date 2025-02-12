"use client"

import { fetcher, Group, PayedType, User } from "@/types";
import { formatISO } from "date-fns";
import { redirect } from "next/navigation";
import { FormEvent, useEffect, useState } from "react";
import useSWR from "swr";

function users_to_payed_type(u: User, id: number) {
    return <>
        <option value={id.toString() + 'E'}>{u.name + ' ' + id + " EVEN"}</option>;
        <option value={id.toString() + 'F'}>{u.name + ' ' + id + " FULL"} </option>
    </>
}

export default function AddExpense({ user_id, group, setShowAdd }: { user_id: number, group: Group, setShowAdd: any }) {
    const [payedType, setPayedType] = useState<PayedType>();
    const [name, setName] = useState<string>("");
    const [amount, setAmount] = useState<number>(0);
    const [payedTypeList, setPayedTypeList] = useState('');

    function selectChangeEvent(event) {
        const v = event.target.value;
        const id = parseInt(v.substring(0, v.length - 1));
        let type = "";
        if (v[v - 1] == "F") {
            type = "OwedFull";
        } else {
            type = "EvenSplit";
        }
        setPayedType({ c: id, t: type });
    }

    function add(event: FormEvent<HTMLFormElement>): void {
        event.preventDefault();
        const date = formatISO(Date.now());
        fetch('/api/expense/' + group.id, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                id: 0,
                payed_type: payedType,
                amount: amount,
                name: name,
                time: date,
                expense_group_id: group.id,
            })
        });

        setShowAdd(false);
    }
    const { data: users, error, isLoading } = useSWR('/api/users', fetcher);
    useEffect(() => {
        if (users) {
            setPayedTypeList(users.map(users_to_payed_type));
        }
    }, [users]);



    return (<>
        <div className="grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20 font-[family-name:var(--font-geist-sans)]">
            <main className="flex flex-col gap-8 row-start-2 items-center sm:items-start">
                <h1>Create a new Expense</h1>
                <form onSubmit={add} >
                    <input type="text" placeholder="Name" className="input input-bordered w-full max-w-xs" value={name} onChange={(e) => setName(e.target.value)} />
                    <input type="number" placeholder="Amount" className="input input-bordered w-full max-w-xs" value={amount} onChange={(e) => setAmount(parseInt(e.target.value))} />
                    {!isLoading &&
                        <select onChange={selectChangeEvent}>
                            {payedTypeList}
                        </select>
                    }

                    <button className="btn btn-primary" type="submit">Add</button>
                    <button className="btn" type="reset" onClick={() => setShowAdd(false)}>Cancel</button>
                </form>
            </main>
        </div>
    </>
    );
}