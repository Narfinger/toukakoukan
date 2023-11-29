/// What type of payment we have
#[derive(Debug)]
pub(crate) enum PayedType {
    /// the split is even and `usize` payed
    EvenSplit(usize),
    /// `usize' owes the total
    OwedTotal(usize),
}

/// An expense
#[derive(Debug, sqlx::FromRow)]
pub(crate) struct Expense {
    /// the id of the expense
    id: usize,
    /// payed type
    payed_type: PayedType,
    /// amount
    amount: u64,
}

/// An Expense Group, a collection of people and expenses
#[derive(Debug, sqlx::FromRow)]
pub(crate) struct ExpenseGroup {
    /// the id of the expense group
    id: usize,
    /// the people that are in the expense group
    people: Vec<String>,
    /// the list of expenses
    expenses: Vec<Expense>,
    /// the name of the expense group
    name: String,
}
