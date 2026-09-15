use askama::Template;
// send string sent to browser as a html file
// send get request to server to open html file in browser
// router is to map the function to webpage

use axum::{
    extract::{Form, Path, State},
    response::Html,
    routing::{get, post},
    Router,
};

use tower_http::services::ServeDir;

use crate::question::{
    get_questions,
    Question,
};

use crate::p_papers_questions::{
    practise_paper_a,
    practise_paper_b,
    mental_maths_paper,
    Question as PracticeQuestion,
};

use serde::Deserialize;

// update state of variables
use std::sync::{Arc, Mutex};

struct QuizState {
    points: usize,
    num_correct: usize,
    percentage: Option<f64>,
}

// This struct is going to be used in the html template. so it not only stores data like in other
// sruct but it also allows it to be generated to a html.
// Use index.html as the file template.
#[derive(Template)]
#[template(path = "index.html")]
struct QuizTemplate {
    question: Option<Question>,
    question_number: Option<usize>,
    total_questions: usize,
    current_index: Option<usize>,
    feedback: Option<String>,
    correct: Option<bool>,
    num_correct: Option<usize>,
    points: Option<usize>,
    percentage: Option<f64>,
    show_results: bool,
    topic: Option<String>,
    subtopic: Option<String>,
}

#[derive(Template)]
#[template(path = "practice_papers.html")]
struct PracticePapersTemplate {}

#[derive(Template)]
#[template(path = "practice_paper_questions.html")]
struct PracticePaperQuestionsTemplate {
    question: Option<PracticeQuestion>,
    question_number: Option<usize>,
    total_questions: usize,
    current_index: usize,
    feedback: Option<String>,
    correct: Option<bool>,
    num_correct: Option<usize>,
    percentage: Option<f64>,
    show_results: bool,
    paper: Option<String>,
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    selected_topic: Option<String>,
}

#[derive(Deserialize)]
struct AnswerForm {
    answer: String,
}

// --- NEW: a small shared "not found" page instead of letting a bad index panic the handler.
fn not_found_page(message: &str) -> Html<String> {
    Html(format!(
        r#"<!DOCTYPE html>
<html lang="eng">
<head>
    <meta charset="UTF-8">
    <title>Kuromi Maths game</title>
    <link rel="stylesheet" href="/static/style.css?v=2">
</head>
<body>
    <div class="navigation-bar">
        <div class="logo">
            <img src="/static/app-logo.jpg" width="50" height="50">
            <span>Kuromi x BadtzMaru</span>
        </div>
        <a href="/">Home</a>
        <a href="/p-papers">Practise Paper</a>
        <a href="/progress">Progress</a>
    </div>
    <main>
        <h1 class="question-number">Oops!</h1>
        <p class="question-text">{}</p>
        <a href="/">Go back home</a>
    </main>
</body>
</html>"#,
        message
    ))
}

// start website and keep it running
pub async fn run_server() {
    let state = Arc::new(Mutex::new(QuizState {
        points: 0,
        num_correct: 0,
        percentage: None,
    }));

    let app = Router::new()
        .route("/", get(show_home))
        .route("/topics/{topic}", get(show_topic))
        .route("/quiz/{topic}/{subtopic}/{index}", get(show_question))
        .route("/answer/{topic}/{subtopic}/{index}", post(submit_answers))
        .route("/results/{topic}/{subtopic}", get(show_results))
        .route("/p-papers", get(show_practice_papers))
        .route("/p-papers/{paper}/{index}", get(show_paper_question))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Could not start server");

    println!("Open http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .expect("Server crashed unexpectedly")
}

async fn show_home() -> Html<String> {
    let page = HomeTemplate { selected_topic: None };
    let completed_html = page.render().expect("Could not render home.html");
    Html(completed_html)
}

async fn show_topic(Path(topic): Path<String>) -> Html<String> {
    let page = HomeTemplate { selected_topic: Some(topic) };
    let completed_html = page.render().expect("Could not render home.html");
    Html(completed_html)
}

async fn show_question(Path((topic, subtopic, index)): Path<(String, String, usize)>) -> Html<String> {
    let questions = get_questions(&subtopic);

    // FIX: .get(index) returns None instead of panicking when the index
    // (or the whole subtopic) has no questions yet.
    let question = match questions.get(index) {
        Some(q) => q.clone(),
        None => {
            return not_found_page(
                "That question doesn't exist yet — this topic is still being built, or the question number was out of range.",
            );
        }
    };

    let page = QuizTemplate {
        question: Some(question),
        question_number: Some(index + 1),
        total_questions: questions.len(),
        current_index: Some(index),
        topic: Some(topic),
        subtopic: Some(subtopic),
        feedback: None,
        correct: None,
        num_correct: None,
        points: None,
        percentage: None,
        show_results: false,
    };

    let completed_html = page.render().expect("Could not render index.html");
    Html(completed_html)
}

pub fn is_correct(user_answer: &str, question: &Question) -> bool {
    user_answer == question.correct_answer
}

async fn submit_answers(
    Path((topic, subtopic, index)): Path<(String, String, usize)>,
    State(state): State<Arc<Mutex<QuizState>>>,
    Form(form): Form<AnswerForm>,
) -> Html<String> {
    let questions = get_questions(&subtopic);

    // FIX: same bounds check as show_question, on the submit path.
    let question = match questions.get(index) {
        Some(q) => q,
        None => {
            return not_found_page(
                "That question doesn't exist yet — this topic is still being built, or the question number was out of range.",
            );
        }
    };

    let mut quiz_state = state.lock().unwrap();
    let correct_answer = is_correct(&form.answer, question);

    let feedback = if correct_answer {
        "Well done! You got it correct!"
    } else {
        "You were very close - keep going!"
    };

    if correct_answer {
        quiz_state.points += 10;
        quiz_state.num_correct += 1;
    }

    let points = quiz_state.points;
    let num_correct = quiz_state.num_correct;

    quiz_state.percentage = None;

    if index + 1 == questions.len() {
        quiz_state.percentage = Some((num_correct as f64 / questions.len() as f64) * 100.0);
    }

    let percentage = quiz_state.percentage;

    let page = QuizTemplate {
        question: Some(question.clone()),
        question_number: Some(index + 1),
        total_questions: questions.len(),
        current_index: Some(index),
        feedback: Some(feedback.to_string()),
        correct: Some(correct_answer),
        num_correct: Some(num_correct),
        points: Some(points),
        topic: Some(topic),
        subtopic: Some(subtopic),
        percentage,
        show_results: false,
    };

    let completed_html = page.render().expect("Could not render index.html");
    Html(completed_html)
}

async fn show_results(
    Path((topic, subtopic)): Path<(String, String)>,
    State(state): State<Arc<Mutex<QuizState>>>,
) -> Html<String> {
    let quiz_state = state.lock().unwrap();

    let points = quiz_state.points;
    let num_correct = quiz_state.num_correct;
    let percentage = quiz_state.percentage;

    let page = QuizTemplate {
        question: None,
        question_number: None,
        total_questions: get_questions(&subtopic).len(),
        current_index: None,
        feedback: None,
        correct: None,
        num_correct: Some(num_correct),
        points: Some(points),
        percentage,
        show_results: true,
        topic: Some(topic),
        subtopic: Some(subtopic),
    };

    let completed_html = page.render().expect("Could not render index.html");
    Html(completed_html)
}

fn get_practice_papers(paper: &str) -> Vec<PracticeQuestion> {
    match paper {
        "paper-a" => practise_paper_a(),
        "paper-b" => practise_paper_b(),
        "mental-maths" => mental_maths_paper(),
        _ => Vec::new(),
    }
}

async fn show_practice_papers() -> Html<String> {
    let page = PracticePapersTemplate {};
    let completed_html = page.render().expect("Could not render practice_papers.html");
    Html(completed_html)
}

async fn show_paper_question(Path((paper, index)): Path<(String, usize)>) -> Html<String> {
    let questions = get_practice_papers(&paper);

    // FIX: .get(index) instead of questions[index] — paper-b and mental-maths
    // are currently empty, so every request to them was panicking before.
    let question = match questions.get(index) {
        Some(q) => q.clone(),
        None => {
            return not_found_page(
                "That practice paper doesn't have questions yet — it's still being built, or the question number was out of range.",
            );
        }
    };

    let page = PracticePaperQuestionsTemplate {
        question: Some(question),
        question_number: Some(index + 1),
        total_questions: questions.len(),
        current_index: index,
        feedback: None,
        correct: None,
        num_correct: None,
        percentage: None,
        show_results: false,
        paper: Some(paper),
    };

    let completed_html = page
        .render()
        .expect("Could not render practice_paper_questions.html");

    Html(completed_html)
}
