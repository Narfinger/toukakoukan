import type {Expense} from "../../types";

function expensemap(expense:Expense) {
    return (
        <tr>
            <td>{expense.amount}</td>
            <td>{expense.name}</td>
        </tr>
    )
}


export default function ExpenseView({ login_cookie }: { login_cookie: string }) {
    const expenses: Expense[] = [];

    const exp = expenses.map((e: Expense[]) =>expensemap(e));
    return (<>
        <table className="table">
            <tr>
                <th>Who Payed</th>
                <th>Description</th>
                <th>Amount</th>
            </tr>
            {exp}
        </table>
    </>)

}
