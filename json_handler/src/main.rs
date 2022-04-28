use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: i32,
    id: Option<i32>,
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let todos: Vec<Todo> = reqwest::Client::new()
        .get("https://jsonplaceholder.typicode.com/todos?userId=1")
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", todos);

    // let new_todo = Todo {
    //     user_id: 1,
    //     id: None,
    //     title: "Subscribe to my non existent channel".to_owned(),
    //     completed: false,
    // };

    let new_todo: serde_json::Value = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/todos")
        .json(&serde_json::json!({
            "userId": 1,
            "title": "Subscribe to my non existent channel".to_owned(),
            "completed": false,
        }))
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", new_todo);
    Ok(())
}
