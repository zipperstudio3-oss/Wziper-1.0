fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let mut resource = winres::WindowsResource::new();
        resource.set_icon("assets/wziper.ico");
        resource.compile().expect("failed to embed Wziper application icon");
    }
}
