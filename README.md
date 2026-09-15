# SATs Maths Revision Platform

A full-stack web app for primary-school SATs maths revision, built from scratch in Rust — currently in progress.

Pupils pick a topic, work through multiple-choice questions with instant feedback and explanations, and get a final score. A separate "Practice Papers" mode offers timed-paper-style question sets.

## Status

This is an active, in-progress project. Two things to know going in:

- Only **Calculations → Written Adding and Subtracting** currently has real quiz questions wired up; other topics/subtopics on the home page are placeholders for now.
- **Practice Paper A** has sample questions; Paper B and the Mental Maths paper are scaffolded but empty.

## Tech stack

- **Backend:** Rust, [Axum](https://github.com/tokio-rs/axum) (routing/state), [Tokio](https://tokio.rs/) (async runtime)
- **Templating:** [Askama](https://github.com/djc/askama) (compile-time-checked HTML templates)
- **Other:** Serde (form parsing), Tower-http (static file serving), Rand

## Running it locally

```bash
cargo run
```

Then open `http://127.0.0.1:3000`.

## How it works

- `/` — topic selection home page
- `/topics/:topic` — subtopics for a given topic
- `/quiz/:topic/:subtopic/:index` — a single quiz question
- `/answer/:topic/:subtopic/:index` — submits an answer, returns feedback
- `/results/:topic/:subtopic` — final score for a quiz
- `/p-papers` — practice paper selection
- `/p-papers/:paper/:index` — a single practice paper question

Quiz questions live in `question.rs`; practice paper questions live in `p_papers_questions.rs`. Adding a new subtopic means adding a new match arm and question set in the relevant file.

## Roadmap

- [ ] Fill in remaining topics/subtopics with real question banks
- [ ] Complete Practice Paper B and the Mental Maths paper
- [ ] Add a progress-tracking page (linked in the nav bar, not yet built)
- [ ] Persist results to a real database instead of a flat file
