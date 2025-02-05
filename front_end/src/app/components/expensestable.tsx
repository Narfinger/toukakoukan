'use client';
import { Expense, fetcher, Group } from "@/types";
import useSWR from "swr";

function expensemap(expense: Expense) {
    return (
        <tr>
            <td>{expense.amount}</td>
            <td>{expense.name}</td>
        </tr>
    )
}

// eslint-disable-next-line @typescript-eslint/no-unused-vars
export default function ExpensesTableView({ user_id, groups, active }: { user_id: number, groups: Group[], active: number }) {
    const group = groups[active];
    const { data, error, isLoading } = useSWR('/api/expense/' + group.id, fetcher)

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
