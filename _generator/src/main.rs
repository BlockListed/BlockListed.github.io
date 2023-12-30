use std::path::PathBuf;

use askama::Template;

#[derive(Template)]
#[template(path = "article.html")]
struct Article<'a> {
    pub title: String,
    pub content: &'a str,
}

fn main() {
    let articles_path: PathBuf = std::env::var_os("ARTICLES").expect("Missing ARTICLES env var").into();

    let output: PathBuf = std::env::var_os("OUTPUT").expect("Missing OUTPUT env var").into();

    for i in std::fs::read_dir(articles_path).unwrap().map(|p| p.unwrap().path()).map(|p| (p.file_name().unwrap().to_str().unwrap().to_owned(), std::fs::read_to_string(p).unwrap())) {
        let (title, content) = i;

        let rendered = Article {
            title: title.clone(),
            content: &content,
        }.render().unwrap();

        let output_file = {
            let mut tmp = output.clone();
            tmp.push(format!("{}.html", title));
            tmp
        };

        std::fs::write(output_file, rendered).unwrap();
    }
}
