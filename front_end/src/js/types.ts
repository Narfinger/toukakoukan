export type GroupResponse = {
    name: String;
    users: Array<String>;
};

export type Group = {
    id: number;
    name: String;
    users: Array<String>;
};

export type PayedType = {
    t: String;
    c: Number;
};

export type Expense = {
    id: Number;
    payed_type: PayedType;
    amount: Number;
    name: String;
    time: Date;
    expense_group_id: Number;
};

export type ExpenseAdjusted = {
    id: Number;
    payed_type: PayedType;
    amount: Number;
    name: String;
    time: Date;
    expense_group_id: Number;
    amount_adjusted: String;
};

export type User = {
    id: Number;
    name: String;
};
