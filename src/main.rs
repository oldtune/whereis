#[tokio::main]
async fn main() -> std::io::Result<()> {
    whereis_lib::run_app(8080).await?;
    Ok(())
}

// fn find(key_word: String) {}

// fn post() {}

// fn top_keyword() {}

// fn top_location() {}

// fn top_region() {}
