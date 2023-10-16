use uuid::Uuid;

#[derive(Debug)]
pub struct Task {
    pub(crate) id: Uuid,
    pub(crate) label: String,
    pub(crate) description: String
}

impl Task {
    pub fn new(_id: Uuid, label: String, description: String) -> Task {
        Task{
            id: Uuid::new_v4(),
            label,
            description
        }
    }
}