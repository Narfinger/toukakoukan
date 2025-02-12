'use client';
import { Expense, fetcher, Group, User } from "@/types";
import { JSX } from "react";
import useSWR from "swr";


export type PayedType = {
    // type
    t: string;
    // id
    c: number;
};

function expensetypeToString(t: PayedType, users: User[]): string {
    return t.t + " for " + users[t.c].name;
}

function expensemap(expense: Expense, users: User[]): JSX.Element {
    if (users) {
        return (
            <tr>
                <td>{expensetypeToString(expense.payed_type, users)}</td>
                <td>{expense.name}</td>
                <td>{expense.amount}</td>
            </tr>
        )
    } else {
        return <tr></tr>
    }
}

// eslint-disable-next-line @typescript-eslint/no-unused-vars
export default function ExpensesTableView({ user_id, groups, active }: { user_id: number, groups: Group[], active: number }) {
    const group = groups[active];
    const { data: expenses, error, isLoading } = useSWR('/api/expense/' + group.id, fetcher)

    if (error) {
        return <span>{"Error in query" + error}</span>;
    } else if (isLoading) {
        return <span>{"Loading"}</span>;
    } else {
        const exp = expenses.map((e: Expense) => expensemap(e, groups[active].users));
        return (
            < table className="table" >
                <thead>
                    <tr>
                        <th>Who Payed</th>
                        <th>Description</th>
                        <th>Amount</th>
                    </tr>
                </thead>
                {exp &&
                    <tbody>
                        {exp}
                    </tbody>
                }
            </table >
        );
    }
}
