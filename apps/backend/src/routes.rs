mod admin;
mod announcements;
mod auth;
mod blog;
mod church;
mod contact;
mod events;
mod newsletter;
mod testimonials;

pub use admin::admin_routes;
pub use announcements::announcements_routes;
pub use auth::auth_routes;
pub use blog::blog_routes;
pub use church::church_routes;
pub use contact::contact_routes;
pub use events::events_routes;
pub use newsletter::newsletter_routes;
pub use testimonials::testimonials_routes;
