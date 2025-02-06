import { JSX, useState } from "react";
import { fetcher, type Group } from "../../types";
import ExpensesTableView from "./expensestable";
import useSWR from "swr";
import AddExpense from "./addexpenseview";

function groupmap(g: Group, index: number, active: number, setActive: any): JSX.Element {
    if (active == g.id) {
        return <a
            role="tab"
            key={index}
            className="tab tab-active text-primary [--tab-bg:yellow] [--tab-border-color:orange]"
            onClick={setActive(g.id)}
        >
            {g.name}
        </a>
    } else {
        return <a role="tab" key={index} className="tab text-primary [--tab-bg:yellow] [--tab-border-color:orange]" onClick={setActive(g.id)}>{g.name}</a>
    }
}

export default function ExpenseView({ user_id }: { user_id: number }) {
    const [active, setActive] = useState<number>(0);
    const { data, error, isLoading } = useSWR('/api/groups', fetcher);
    const [showAdd, setShowAdd] = useState<boolean>(false);

    if (isLoading) {
        return <span>{"IS LOADING"}</span>
    } else if (error) {
        return <span>{"ERROR" + error}</span>;
    } else {

        const groups = data;


        const groups_elem = groups.map((g: Group, index: number) => groupmap(g, active, index, () => setActive));
        console.log(showAdd);
        if (!showAdd) {
            return (<>
                <div className="row">
                    <button className="btn btn-primary" onClick={() => setShowAdd(true)}>Add Expense</button>
                </div>
                <div className="row">
                    <div>
                        {"here is the tab list"}
                    </div>

                    <div role="tablist" className="tabs tabs-lifted">
                        {groups_elem}
                    </div>
                </div>

                <div className="row">
                    {
                        groups && <ExpensesTableView user_id={user_id} groups={groups} active={active} />
                    }
                </div>
            </>)
        }
        else {
            return <AddExpense user_id={user_id} group={groups[active]} setShowAdd={setShowAdd} />
        }
    }

}
