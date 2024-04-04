use self::{
    enums::TaskStatus,
    structs::{done::Done, pending::Pending},
};

pub mod enums;
pub mod structs;

pub enum ItemTypes {
    Pending(Pending),
    Done(Done),
}

pub fn to_do_factory(title: &str, status: TaskStatus) -> ItemTypes {
    match status {
        TaskStatus::DONE => ItemTypes::Done(Done::new(title)),
        TaskStatus::PENDING => ItemTypes::Pending(Pending::new(title)),
    }
}
