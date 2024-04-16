fn main() {
    // Compile TailwindCSS .css file
    std::process::Command::new(if_windows("npx.cmd", "npx"))
        .args([
            "tailwindcss",
            "-i",
            "src/input.css",
            "-c",
            "tailwind.config.js",
            "-o",
            "public/tailwind.css",
            "--minify",
        ])
        .env("NODE_ENV", "production")
        .spawn()
        .unwrap();
}

#[allow(unused_variables)]
const fn if_windows(windows: &'static str, unix: &'static str) -> &'static str {
    #[cfg(windows)]
    {
        windows
    }
    #[cfg(unix)]
    {
        unix
    }
}
