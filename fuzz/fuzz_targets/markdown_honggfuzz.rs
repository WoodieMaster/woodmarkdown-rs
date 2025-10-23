use honggfuzz::fuzz;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if let Ok(s) = std::str::from_utf8(data) {
                let _ = woodmarkdown::to_html(s);
                let _ = woodmarkdown::to_html_with_options(s, &woodmarkdown::Options::gfm());
                let _ = woodmarkdown::to_mdast(s, &woodmarkdown::ParseOptions::default());
                let _ = woodmarkdown::to_mdast(s, &woodmarkdown::ParseOptions::gfm());
                let _ = woodmarkdown::to_mdast(s, &woodmarkdown::ParseOptions::mdx());
            }
        });
    }
}
