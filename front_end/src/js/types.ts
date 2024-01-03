export type GroupResponse = {
    name: String;
    people: Array<String>;
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

export function createPayed(type: String, who: Number): PayedType {
    return { "t": type, "c": who }
}