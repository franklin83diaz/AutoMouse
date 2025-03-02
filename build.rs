#![windows_subsystem = "windows"]
fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icon_r.ico");
        res.compile().unwrap();
    }
    let config =
    slint_build::CompilerConfiguration::new()
    .with_style("fluent-light".into());
    slint_build::compile_with_config("ui/app-window.slint", config).expect("Slint build failed");
}
