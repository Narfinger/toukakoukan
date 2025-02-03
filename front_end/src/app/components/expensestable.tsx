'use client';
import { Expense, Group } from "@/types";
import useSWR from "swr";

function expensemap(expense: Expense) {
    return (
        <tr>
            <td>{expense.amount}</td>
            <td>{expense.name}</td>
        </tr>
    )
}
const fetcher = (...args) => fetch(...args).then(res => res.json())

export default function ExpensesTableView({ user_id, groups, active }: { user_id: number, groups: Group[], active: number }) {
    const group = groups[active];
    const { data, error, isLoading } = useSWR('/expenses/' + group, fetcher)

    if (error) {
        return <span>{"Error in query" + error}</span>;
    } else if (isLoading) {
        return <span>{"Loading"}</span>;
    } else {

        const exp = data.map((e: Expense[]) => expensemap(e));
        return (
            < table className="table" >
                <thead>
                    <tr>
                        <th>Who Payed</th>
                        <th>Description</th>
                        <th>Amount</th>
                    </tr>
                </thead>
                <tbody>
                    {exp}
                </tbody>
            </table >
        );
    }
}
