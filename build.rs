#![windows_subsystem = "windows"]
fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        // Asegúrate de tener el archivo "icon.ico" en la raíz del proyecto o especifica la ruta correcta
        res.set_icon("icon_r.ico");
        res.compile().unwrap();
    }
    let config =
    slint_build::CompilerConfiguration::new()
    .with_style("fluent-light".into());
    slint_build::compile_with_config("ui/app-window.slint", config).expect("Slint build failed");
}
