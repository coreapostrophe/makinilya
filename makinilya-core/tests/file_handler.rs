use makinilya_core::files::FileHandler;

#[test]
fn fetches_files() {
    let mut file_handler = FileHandler::new();
    assert!(file_handler.init().is_ok());

    println!("{:?}", file_handler);
}
