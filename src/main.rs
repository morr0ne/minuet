use anyhow::Result;
slint::include_modules!();

fn main() -> Result<()> {
    let main_window = MainWindow::new()?;

    main_window.run()?;

    Ok(())
}
