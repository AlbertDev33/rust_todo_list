use deadpool_postgres::Client;
use std::io::{Error, ErrorKind};
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::models::{TodoItem, TodoList};

pub async fn get_todos(client: &Client) -> Result<Vec<TodoList>, Error> {
    let statement = client
        .prepare("select * from todo_list order by id asc")
        .await
        .unwrap();
    let todos = client
        .query(&statement, &[])
        .await
        .expect("Error getting todo lists")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>();

    return Ok(todos);
}

pub async fn get_items(client: &Client, list_id: i32) -> Result<Vec<TodoItem>, Error> {
    let statement = client
        .prepare("select * from todo_item where list_id = $1 order by id desc")
        .await
        .unwrap();

    let items = client
        .query(&statement, &[&list_id])
        .await
        .expect("Error getting todo items")
        .iter()
        .map(|row| TodoItem::from_row_ref(row).unwrap())
        .collect::<Vec<TodoItem>>();

    return Ok(items);
}

pub async fn create_todo(client: &Client, title: String) -> Result<TodoList, Error> {
    let statement = client
        .prepare("insert into todo_list (title) values ($1) returning id, title")
        .await
        .unwrap();

    let item = client
        .query(&statement, &[&title])
        .await
        .expect("Error creating todo list")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>()
        .pop()
        .ok_or(Error::new(ErrorKind::Other, "Error creating todo list"));

    return item;
}

pub async fn check_item(client: &Client, list_id: i32, item_id: i32) -> Result<(), Error> {
    let statement = client.prepare("update todo_item set checked = true where list_id = $1 and id = $2 and checked = false").await.unwrap();
    let result = client
        .execute(&statement, &[&list_id, &item_id])
        .await
        .expect("Error checking todo item");

    match result {
        ref updated if *updated == 1 => Ok(()),
        _=> Err(Error::new(ErrorKind::Other, "Failed to check list"))
    }
}
