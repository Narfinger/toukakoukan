import {
    type Expense,
    type ExpenseAdjusted,
    type Group,
    type PayedType,
    type GroupResponse,
} from "../js/types";

export function createPayed(type: String, who: Number): PayedType {
    return { "t": type, "c": who }
}

export function fromPayed(t: PayedType): String {
    return t.t + " " + t.c;
}

export function adjusted_expense(user_id: Number, number_of_group_members: Number, val: Expense): String {
    console.log(val.payed_type.t);
    if (val.payed_type.t == "OwedTotal") {
        return "NNN";
    } else if (val.payed_type.t == "EvenSplit") {
        if (val.payed_type.c === user_id) {
            return String(val.amount / number_of_group_members);
        } else {
            return String(- val.amount / number_of_group_members);
        }
    } else {
        return "SWR";
    }
}