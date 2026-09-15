#[derive(Clone)]
pub struct Question {
    pub text: String,
    pub choices: Option<Vec<String>>, 
    pub correct_answer: String,
    pub explanation: String,
    pub topic: String,
    pub subtopic: String,

}

pub fn get_questions(subtopic: &str) -> Vec<Question> {
    match subtopic {
        "written-add-sub" => calculations_written_add_sub(),
        _ => Vec::new(),

    }
}

pub fn calculations_written_add_sub() -> Vec<Question> {
    vec![
        Question {
            text: "What is 21 508 + 13 942 equal to?".to_string(),
            choices: Some(vec![
                "37 987".to_string(),
                "32 765".to_string(),
                "36 000".to_string(),
                "35 450".to_string(),
            ]),
            correct_answer: "D".to_string(),
            explanation: "21 508 + 13 942 = 35 450".to_string(),
            topic: "calculations".to_string(),
            subtopic: "written-add-sub".to_string(),
        },
        
        Question {
            text: "What is 24 971 - 5433 equal to?".to_string(),
            choices: Some(vec![
                "19 538".to_string(),
                "20 598".to_string(),
                "18 732".to_string(),
                "18 900".to_string(),
            ]),
            correct_answer: "A".to_string(),
            explanation: "24 971 - 5433 = 19 538".to_string(),
            topic: "calculations".to_string(),
            subtopic: "written-add-sub".to_string(),
        },

        Question {
            text: "At a recent football match, 17 930 spectators were supporting Tinbury FC and 11 746 were supporting Milrow United. How many spectators were there altogether?".to_string(),
            choices: Some(vec![
                "31 650".to_string(),
                "29 676".to_string(),
                "30 735".to_string(),
                "25 902".to_string(),
            ]),
            correct_answer: "B".to_string(),
            explanation: "17 930 + 11 746 = 29 676".to_string(),
            topic: "calculations".to_string(),
            subtopic: "written-add-sub".to_string(), 
        },

        Question {
            text: "Celine inherits £22 350. She gives £5365. How much money does she have left?".to_string(),
            choices: Some(vec![
                "19 732".to_string(),
                "18 655".to_string(),
                "16 985".to_string(),
                "20 876".to_string(),
            ]),
            correct_answer: "C".to_string(),
            explanation: "22 350 - 5365 = 16 985".to_string(),
            topic: "calculations".to_string(),
            subtopic: "written-add-sub".to_string() ,
        },
    ]
}
