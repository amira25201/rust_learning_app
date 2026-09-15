#[derive(Clone)] 
pub struct Question {
    pub text: String,
    pub correct_answer: String,
    pub explanation: String,
    pub topic: String,
    pub subtopic: String,
}

pub fn practise_paper_a() -> Vec<Question> { 
    vec![
        Question {
            text: "Calculate 546 + 423".to_string(),
            correct_answer: "969".to_string(),
            explanation: "546 + 423 = 969".to_string(),
            topic: "calculations".to_string(),
            subtopic: "written-add-sub".to_string(), 
        },
        Question {
            text: "Here is a list of numbers:\n\n1140 1400 1440 1040 1410\n\nSmallest number is: ".to_string(),
            correct_answer: "1040".to_string(),
            explanation: "The ordered list would like 1040 1140 14400 1440. 1040 is at the beginning of the list, therefore the smallest one.".to_string(),
            topic: "numbers".to_string(),
            subtopic: "order-comparing-numbers".to_string(),
        },
        Question {
            text: "Here is a list of numbers:\n\n1140 1400 1440 1040 1410\n\nLargest number is: ".to_string(),
            correct_answer: "1440".to_string(),
            explanation: "The ordered list would like 1040 1140 14400 1440. 1440 is at the end of the list, therefore the largest one.".to_string(),
            topic: "numbers".to_string(),
            subtopic: "order-comparing-numbers".to_string(),
        },
        Question {
            text: "Emma buys 5 bunches of flowers.\n\nEach bunch of flowers costs £1.20\n\nHow much does Emma pay altogether?".to_string(),
            correct_answer: "£6.00".to_string(),
            explanation: "£1.20 x 5 = £6.00 where 5 flowers are multiplied by the cost of one which is 1.20 to give the total price of 5 flowers".to_string(),
            topic: "calculations".to_string(),
            subtopic: "mutliplying-dividing".to_string(),
        }
    ]

}

pub fn practise_paper_b() -> Vec<Question> {
    Vec::new()
}

pub fn mental_maths_paper() -> Vec<Question> {
    Vec::new()
}
