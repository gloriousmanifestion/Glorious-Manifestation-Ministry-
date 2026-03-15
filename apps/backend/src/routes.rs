mod admin;
mod announcements;
mod auth;
mod church;
mod contact;
mod events;

pub use admin::admin_routes;
pub use announcements::announcements_routes;
pub use auth::auth_routes;
pub use church::church_routes;
pub use contact::contact_routes;
pub use events::events_routes;
