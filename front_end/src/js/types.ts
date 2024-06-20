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
    t: string;
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

export type PayedTypeSelect = Array<[number, PayedType]>;