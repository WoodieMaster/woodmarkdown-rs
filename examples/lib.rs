fn main() -> Result<(), woodmarkdown::message::Message> {
    // Turn on debugging.
    // You can show it with `RUST_LOG=debug cargo run --features log --example lib`
    env_logger::init();

    // Safely turn (untrusted?) markdown into HTML.
    println!("{:?}", woodmarkdown::to_html("## Hello, *world*!"));

    // Turn trusted markdown into HTML.
    println!(
        "{:?}",
        woodmarkdown::to_html_with_options(
            "<div style=\"color: goldenrod\">\n\n# Hi, *Saturn*! 🪐\n\n</div>",
            &woodmarkdown::Options {
                compile: woodmarkdown::CompileOptions {
                    allow_dangerous_html: true,
                    allow_dangerous_protocol: true,
                    ..woodmarkdown::CompileOptions::default()
                },
                ..woodmarkdown::Options::default()
            }
        )
    );

    // Support GFM extensions.
    println!(
        "{}",
        woodmarkdown::to_html_with_options(
            "* [x] contact ~Mercury~Venus at hi@venus.com!",
            &woodmarkdown::Options::gfm()
        )?
    );

    // Access syntax tree and support MDX extensions:
    println!(
        "{:?}",
        woodmarkdown::to_mdast(
            "# <HelloMessage />, {username}!",
            &woodmarkdown::ParseOptions::mdx()
        )?
    );

    Ok(())
}
