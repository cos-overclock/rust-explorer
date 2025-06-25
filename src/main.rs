use floem::prelude::*;
use floem::text::Weight;
use rust_explorer::AppError;

fn main() -> Result<(), AppError> {
    // 将来的にはプロジェクト構造を使用した適切な初期化を行う
    // 現在は基本的なサンプルUIを表示
    floem::launch(sample_view);
    Ok(())
}

fn sample_view() -> impl IntoView {
    let mut counter = RwSignal::new(0);

    v_stack((
        label(|| "rust-explorer - Development Version")
            .style(|s| s.font_size(24.0).font_weight(Weight::BOLD)),
        h_stack((
            button("Increment").action(move || counter += 1),
            label(move || format!("Value: {counter}")),
            button("Decrement").action(move || counter -= 1),
        ))
        .style(|s| s.gap(10)),
        label(|| "Project structure initialized successfully!")
            .style(|s| s.margin_top(20.0).color(floem::peniko::Color::GREEN)),
    ))
    .style(|s| s.size_full().items_center().justify_center().gap(20))
}
