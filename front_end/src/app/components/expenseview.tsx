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

function groupmap(g: Group, active: number, setActive) {
    if (active == g.id) {
        return <a
            role="tab"
            className="tab tab-active text-primary [--tab-bg:yellow] [--tab-border-color:orange]"
            onClick={setActive(g.id)}
        >
            {g.name}
        </a>
    } else {
        <a role="tab" className="tab text-primary [--tab-bg:yellow] [--tab-border-color:orange]" onClick={setActive(g.id)}>{g.name}</a>
    }
}

export default function ExpenseView({ user_id }: { user_id: number }) {
    const [groups, setGroups] = useState<Group[]>([])
    const [active, setActive] = useState<number>(0);

    const expenses: Expense[] = [];

    useEffect(() => {
        async function fetchGroups() {
            const res = await fetch('/api/groups/')
            console.log(res);
            const data = await res.json();
            console.log(data);
            setGroups(data)
        }
        fetchGroups()
    }, [])


    const exp = expenses.map((e: Expense[]) => expensemap(e));
    const groups_elem = groups.map((g: Group) => groupmap(g, active, () => setActive));
    return (<>
        <div className="row">
            <div>
                {"here is the tab list"}
            </div>

            <div role="tablist" className="tabs tabs-lifted">
                {groups_elem}
            </div>
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
