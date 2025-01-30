import { useEffect, useState } from "react";
import type { Expense, Group } from "../../types";

function expensemap(expense: Expense) {
    return (
        <tr>
            <td>{expense.amount}</td>
            <td>{expense.name}</td>
        </tr>
    )
}

function groupmap(g: Group) {
    return <li>
        {g.name}
    </li>;
}

export default function ExpenseView({ user_id }: { user_id: number }) {
    const [groups, setGroups] = useState<Group[]>([])

    const expenses: Expense[] = [];

    useEffect(() => {
        async function fetchGroups() {
            const res = await fetch('/groups')
            const data = await res.json()
            setGroups(data)
        }
        fetchGroups()
    }, [])


    const exp = expenses.map((e: Expense[]) => expensemap(e));
    const groups_elem = groups.map((g: Group) => groupmap(g));
    return (<>
        <div className="row">
            <ul>
                {groups_elem}
            </ul>
        </div>
        <div className="row">
            <table className="table">
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
            </table>
        </div>
    </>)

}
