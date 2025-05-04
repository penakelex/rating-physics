#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("icons_windows/icon.ico");
    res.compile().unwrap();
}
