function expensemap({ expense}: {Array<string>}) {
    return (
        <tr>
            <td>{expense[0]}</td>
            <td>{expense[1]}</td>
        </tr>
    )
}


export default function ExpenseView({ login_cookie }: { login_cookie: string }) {
    const expenses: Array<Array<string>> = [
        ["You", "test", "400"]
    ];

    expenses.map((e) =>expensemap(e));
    return (<>
        <table className="table">
            <tr>
                <th>Who Payed</th>
                <th>Description</th>
                <th>Amount</th>
            </tr>
            {expenses}
        </table>
    </>)

}
