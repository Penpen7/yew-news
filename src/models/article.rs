#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct article {
    pub id: usize,
    pub title: String,
    pub text: String,
    pub good_point: usize,
}

pub fn GetArticles() -> Vec<article> {
    vec![
        article {
            id: 0,
            title: "Rust".to_string(),
            text: "毎日Rustをやっています".to_string(),
            good_point: 20,
        },
        article {
            id: 1,
            title: "hello".to_string(),
            text: "レコードをたくさんゲットしました".to_string(),
            good_point: 5,
        },
    ]
}
