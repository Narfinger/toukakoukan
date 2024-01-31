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

export function adjusted_expense(val: Expense): String {
    return "NYI";
}