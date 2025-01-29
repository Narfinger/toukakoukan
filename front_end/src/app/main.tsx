"use client"
import ExpenseView from "./expenseview";
import UserView from "./userview";

export default function MainView({ login_cookie }: { login_cookie: string }) {

  return (<>
    <div className="row">
      <UserView login_cookie={login_cookie} />
    </div>
    <div className="row">
      <ExpenseView login_cookie={login_cookie} />
    </div>
  </>
);
}