import {
    type Expense,
    type PayedType,
} from "../js/types";

export function createPayed(type: string, who: number): PayedType {
    return { "t": type, "c": who }
}

export function fromPayed(t: PayedType): string {
    return t.t + " " + t.c;
}

export function adjusted_expense(user_id: number, number_of_group_members: number, val: Expense): string {
    if (val.payed_type.t == "OwedTotal") {
        if (val.payed_type.c === user_id) {
            return String(-val.amount);
        } else {
            return String(val.amount);
        }
    } else if (val.payed_type.t == "EvenSplit") {
        if (val.payed_type.c === user_id) {
            return String(val.amount / number_of_group_members);
        } else {
            return String(-val.amount / number_of_group_members);
        }
    } else {
        return "SWR";
    }
}