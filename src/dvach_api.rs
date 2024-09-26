use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Board {
    #[serde(rename = "id")]
    pub identifier: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Thread {
    #[serde(rename = "num")]
    pub number: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThreadsResponse {
    pub threads: Vec<Thread>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub comment: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThreadWithPosts {
    pub posts: Vec<Post>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThreadsWithPostsResponse {
    pub threads: Vec<ThreadWithPosts>,
}

pub struct DvachApi;

impl DvachApi {
    pub fn new() -> DvachApi {
        DvachApi {}
    }

    async fn request<T: DeserializeOwned>(&self, endpoint: String) -> T {
        let response = reqwest::get(format!("https://2ch.hk/{endpoint}"))
            .await
            .unwrap()
            .json::<T>()
            .await
            .unwrap();

        return response;
    }

    pub async fn get_boards(&self) -> Vec<Board> {
        self.request(String::from("api/mobile/v2/boards")).await
    }

    pub async fn get_threads(&self, board_identifier: String) -> ThreadsResponse {
        self.request(format!("{board_identifier}/threads.json"))
            .await
    }

    pub async fn get_threads_with_posts(
        &self,
        board_identifier: String,
        thread_number: u32,
    ) -> ThreadsWithPostsResponse {
        self.request(format!("{board_identifier}/res/{thread_number}.json"))
            .await
    }
}
