use rust_explorer::{App, AppError};

fn main() -> Result<(), AppError> {
    // アプリケーションを初期化して起動
    let app = App::new()?;
    app.run()
}
