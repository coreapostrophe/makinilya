use makinilya_core::files::FileHandler;

#[test]
fn fetches_files() {
    let story_model = FileHandler::init("./tests/mock");
    assert!(story_model.is_ok());
}
