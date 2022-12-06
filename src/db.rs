use deadpool_postgres::Client;
use std::io::Error;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::models::TodoList;

pub async fn get_todos(client: &Client) -> Result<Vec<TodoList>, Error> {
    let statement = client.prepare("select * from todo_list").await.unwrap();
    let todos = client
        .query(&statement, &[])
        .await
        .expect("Error getting todo lists")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>();
        
    return Ok(todos);
}
