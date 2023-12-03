pub(crate) async fn get_expenses(
    State(state): State<AppState>,
    Path(expense_group_id): Path<u32>,
) -> Result<Json<Vec<types::Expense>>, StatusCode> {
    let rows = sqlx::query_as::<_, types::Expense>(
        "SELECT * FROM expense WHERE expense_group_id = ? LIMIT >",
    )
    .bind(expense_group_id)
    .bind(EXPENSE_LIMIT)
    .fetch_all(&state.pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(rows))
}

pub(crate) async fn post_expense(
    State(state): State<AppState>,
    Path(expense_group_id): Path<u32>,
    Json(payload): extract::Json<types::Expense>,
) -> Result<(), StatusCode> {
    sqlx::query("INSERT INTO expense (payed_type, amount, expense_group_id) VALUES (?, ?, ?);
")
        .bind(payload.payed_type)
        .bind(payload.amount as i64)
        .bind(expense_group_id as i64)
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}