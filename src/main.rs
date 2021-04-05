use crate::io::Cursor;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{File, create_dir_all};
use std::io;

#[derive(Serialize, Deserialize)]
struct PostResponse {
    posts: Vec<Post>,
}

#[derive(Serialize, Deserialize)]
struct Post {
    id: i32,
    created_at: String,
    updated_at: String,
    file: PostFile,
}

#[derive(Serialize, Deserialize)]
struct PostFile {
    width: i32,
    height: i32,
    ext: String,
    size: i32,
    md5: String,
    url: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let request_url = "https://e621.net/explore/posts/popular.json";
    println!("Fetching data from: {}", request_url);

    let client = Client::new();
    let resp = client
        .get(request_url)
        .header("User-Agent", "Popumurr/0.1.0 (ZachyFoxx)")
        .send()
        .await?;

    if !resp.status().is_success() {
        return Err("There was an error".into());
    }
    
    // deserialize response
    let post_response = resp.json::<PostResponse>().await?;
    
    for i in 0..9 {
        let most_popular = post_response.posts.get(i).expect("no post returned!");
        
        // get the url of the post
        println!("downloading post {}...", most_popular.id);
        let post_url = most_popular.file.url.as_ref().expect("no post url returned!");
        let post_resp = client.get(post_url).header("User-Agent", "Popumurr/0.1.0 (ZachyFoxx)")
            .send()
            .await?;

        // create images directory
        create_dir_all("./images")?;

        // Save the post bytes to memory
        let mut content = Cursor::new(post_resp.bytes().await?);

        // Create file to store post
        let mut dest = File::create(format!("./images/{id}.{ext}", id = most_popular.file.md5, ext = most_popular.file.ext))?;
        
        // Copy bytes to the file we just created
        io::copy(&mut content, &mut dest).expect("failed to copy file!");
    };

    Ok(())
}
