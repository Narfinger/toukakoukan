// eslint-disable-next-line @typescript-eslint/no-explicit-any
export const fetcher = (...args: any[]) => fetch(...args).then(res => res.json())

export type GroupResponse = {
    name: string;
    users: Array<User>;
    querying_user_is: number;
};

export type LoginResult = {
    result: string;
};

// A group
export type Group = {
    id: number;
    name: string;
    users: Array<User>;
    querying_user_is: number;
};

// payed type
export type PayedType = {
    // type
    t: string;
    // id
    c: number;
};

// expense
export type Expense = {
    id: number;
    payed_type: PayedType;
    amount: number;
    name: string;
    time: Date;
    expense_group_id: number;
};

// expense adjusted to the group and who payed what
export type ExpenseAdjusted = {
    id: number;
    payed_type: PayedType;
    amount: number;
    name: string;
    time: Date;
    expense_group_id: number;
    amount_adjusted: string;
};

// a user
export type User = {
    id: number;
    name: string;
};

// type for select who payed
export type PayedTypeSelect = Array<[number, PayedType]>;