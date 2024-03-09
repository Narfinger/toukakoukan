export type GroupResponse = {
    name: string;
    users: Array<User>;
    querying_user_is: number;
};

export type Group = {
    id: number;
    name: string;
    users: Array<User>;
    querying_user_is: number;
};

export type PayedType = {
    t: string;
    c: number;
};

export type Expense = {
    id: number;
    payed_type: PayedType;
    amount: number;
    name: string;
    time: Date;
    expense_group_id: number;
};

export type ExpenseAdjusted = {
    id: number;
    payed_type: PayedType;
    amount: number;
    name: string;
    time: Date;
    expense_group_id: number;
    amount_adjusted: string;
};

export type User = {
    id: number;
    name: string;
};

export type PayedTypeSelect = Array<[number, PayedType]>;