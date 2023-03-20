#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct article {
    pub id: usize,
    pub title: String,
    pub text: String,
}

pub fn GetArticles() -> Vec<article> {
    vec![
        article {
            id: 0,
            title: "Rust".to_string(),
            text: "毎日Rustをやっています".to_string(),
        },
        article {
            id: 1,
            title: "hello".to_string(),
            text: "レコードをたくさんゲットしました".to_string(),
        },
    ]
}
