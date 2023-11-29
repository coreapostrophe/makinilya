use makinilya_core::files::FileHandler;

#[test]
fn fetches_files() {
    let mut file_handler = FileHandler::new();
    file_handler.set_base_directory("./tests/mock");
    
    assert!(file_handler.init().is_ok());

    println!("{:?}", file_handler);
}
