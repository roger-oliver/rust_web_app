use serde::Serialize;

#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    DONE,
    PENDING
}

impl TaskStatus {
    pub fn stringfy(&self) -> String {
        match &self {
            &Self::DONE => "DONE".to_string(),
            &Self::PENDING => "PENDING".to_string()
        }
    }
    pub fn from_string(input_string: &str) -> Self {
        match input_string.to_uppercase().trim() {
            "DONE" => TaskStatus::DONE,
            "PENDING" => TaskStatus::PENDING,
            _ => panic!("input {} not supported", input_string)
        }
    }
    // from book, he was using this method to createa a new status, but you can use the "from_string" instead
    // and from his github repo, he is using the later.....
    // pub fn new (input_string: &str) -> Self {
    //     match input_string.to_uppercase().trim() {
    //         "DONE" => TaskStatus::DONE,
    //         "PENDING" => TaskStatus::PENDING,
    //         _ => panic!("TaskStatus::new() - input {} not supported", input_string)
    //     }
    // }
}

impl Serialize for TaskStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        Ok(serializer.serialize_str(&self.stringfy().as_str())?)
    }
}
