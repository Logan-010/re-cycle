use std::io;
#[cfg(target_os = "windows")]
use {std::env, winres::WindowsResource};

fn main() -> io::Result<()> {
    #[cfg(target_os = "windows")]
    {
        if env::var_os("CARGO_CFG_WINDOWS").is_some() {
            WindowsResource::new()
                // This path can be absolute, or relative to your crate root.
                .set_icon("./icon.ico")
                .compile()?;
        }
    }
    Ok(())
}
