"use client"
import ExpenseView from "./components/expenseview";
import UserView from "./components/userview";

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