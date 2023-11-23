#[derive(strum_macros::Display, strum_macros::EnumIter, PartialEq, Clone, Copy)]
pub enum Project {
    Project1,
    Project2,
    InvalidProject,
}
