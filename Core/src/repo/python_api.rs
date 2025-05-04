use crate::domain::model::MyObject;
use reqwest;

// use crate::domain::model::MyObject;
// use serde::Deserialize;


// #[derive(Debug, Deserialize)]
// struct MyJsonRes {
//     key: String,
// }

pub async fn repository_get_item_from_rest_api(url: &str) -> Result<MyObject, reqwest::Error> {
    let res = reqwest::get(url).await?.json::<MyObject>().await?;
    Ok(MyObject {
        // key: format!("{}", res.key),
        id: res.id,
        name: format!("{}", res.name),
    })
}

pub async fn repository_get_item() -> MyObject {
    let url = "http://127.0.0.1:8000/api";

    match repository_get_item_from_rest_api(url).await {
        Ok(res) => res,
        Err(_) => MyObject {
            // id: "default_id".to_string(),
            id: 0,
            name: "default_name".to_string(),
        }, // If error occurs, return a default value
    }
}

#[cfg(test)]
mod tests {
    // use core::panic;

    use super::*;


    #[tokio::test]
    async fn test_repository_get_item_from_rest_api() {
        let url = "http://127.0.0.1:8000/api";
        let res = repository_get_item_from_rest_api(url).await;
        let res = res.unwrap();
        assert_eq!(res.id, 0);
        assert_eq!(res.name, "my_name".to_string());
    }

    #[tokio::test]
    async fn test_repository_get_item() {
        let res = repository_get_item().await;
        assert_eq!(res.id, 0);
        assert_eq!(res.name, "my_name".to_string());
    }

}