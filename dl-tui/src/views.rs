pub mod actions;
pub mod dash;
pub mod facts;
pub mod prefs;
pub mod records;
pub mod stats;
pub mod topics;
pub mod user;
pub mod edit;
pub mod items;

pub use {
    actions::ActionsView,
    dash::DashView,
    facts::FactsView,
    records::RecordsView,
    stats::StatsView,
    topics::TopicsView,
    user::UserView,
    items::ItemsView,
    edit::EditView,

};
