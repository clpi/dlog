pub mod facts;
pub mod home;
pub mod items;
pub mod records;
pub mod user;

pub use self::{
    user::UserView,
    facts::FactsView,
    records::RecordsView,
    items::ItemView,
    home::HomeView,
};

#[derive(Debug)]
pub enum Views {
    Home(HomeView),
    Facts(FactsView),
    Records(RecordsView),
    Items(ItemView),
    User(UserView),
}
