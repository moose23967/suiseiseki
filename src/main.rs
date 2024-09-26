use std::borrow::Cow;
use std::env;
use std::io::Result;

use actix_web::web::Data;
use actix_web::{get, main, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use dvach_api::DvachApi;
use rand::seq::SliceRandom;
use rand::thread_rng;
use regex::Regex;
use telegram_api::{SendMessageJson, TelegramApi};

pub mod dvach_api;
pub mod telegram_api;

#[get("/")]
async fn index(dvach_api: Data<DvachApi>, telegram_api: Data<TelegramApi>) -> impl Responder {
    let mut rng = thread_rng();

    let boards = dvach_api.get_boards().await;
    let board = boards.choose(&mut rng).unwrap();
    let board_identifier = board.identifier.clone();

    let threads_response = dvach_api.get_threads(board_identifier.clone()).await;
    let threads = threads_response.threads;
    let thread = threads.choose(&mut rng).unwrap();

    let threads_with_posts_response = dvach_api
        .get_threads_with_posts(board_identifier.clone(), thread.number)
        .await;
    let threads_with_posts = threads_with_posts_response.threads;
    let thread_with_posts = threads_with_posts.choose(&mut rng).unwrap();
    let posts = &thread_with_posts.posts;
    let post = posts.choose(&mut rng).unwrap();
    let post_comment = post.comment.clone();

    let first_parse_post_comment: Cow<'_, str>;
    let second_parse_post_comment: Cow<'_, str>;
    let third_parse_post_comment: Cow<'_, str>;

    first_parse_post_comment = Regex::new(r">>[0-9]+")
        .unwrap()
        .replace_all(&post_comment, "");
    second_parse_post_comment = Regex::new(r"\(OP\)")
        .unwrap()
        .replace_all(&first_parse_post_comment, "");
    third_parse_post_comment = Regex::new(r"<.*?>")
        .unwrap()
        .replace_all(&second_parse_post_comment, "");

    telegram_api
        .send_message(SendMessageJson {
            chat_identifier: String::from("-1002433676575"),
            text: third_parse_post_comment.to_string(),
            parse_mode: String::from("HTML"),
        })
        .await;

    HttpResponse::Ok()
}

#[main]
async fn main() -> Result<()> {
    dotenv().unwrap();

    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(DvachApi::new()))
            .app_data(Data::new(TelegramApi::new(env::var("BOT_TOKEN").unwrap())))
            .service(index)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
