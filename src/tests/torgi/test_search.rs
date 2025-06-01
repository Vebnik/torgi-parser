
#[tokio::test]
async fn test_search() {
    use crate::torgi::Lot;
    use std::env;

    if env::var("RUST_LOG").is_err() {
        unsafe { env::set_var("RUST_LOG", "info") }
    }
    env_logger::init();
    log::info!("Logger initialized");

    let query = "2-комн. квартира".to_string();

    let lots = Lot::search(query, 5).await.unwrap();

    dbg!(&lots.len());

    assert!(lots.len() == 50)
}
