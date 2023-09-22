use warp::Filter;

struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

struct QuestionId(String);

impl Question {
    fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {id, title, content, tags}
    }
}


async fn get_questions() -> Result<impl warp::Reply, warp::Rejection> {
    let question = Question::new()
}

#[tokio::main]
async fn main() {
    let hello = warp::path("hello")
        .map(||format!("Hello, World!"));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3000))
        .await;
}
