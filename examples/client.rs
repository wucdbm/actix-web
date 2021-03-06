use actix_http::Error;
use actix_rt::System;

fn main() -> Result<(), Error> {
    std::env::set_var("RUST_LOG", "actix_http=trace");
    env_logger::init();

    System::new("test").block_on(async {
        let client = awc::Client::new();

        // Create request builder, configure request and send
        let mut response = client
            .get("https://www.rust-lang.org/")
            .header("User-Agent", "Actix-web")
            .send()
            .await?;

        // server http response
        println!("Response: {:?}", response);

        // read response body
        let body = response.body().await?;
        println!("Downloaded: {:?} bytes", body.len());

        Ok(())
    })
}
