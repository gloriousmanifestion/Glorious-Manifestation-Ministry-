use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub name: String,
    pub phone: String,
    pub ministry_role: String,
    pub church_name: String,
    pub calling: String,
    pub selected_course_id: Option<String>,
    pub verified: bool,
    pub payment_complete: bool,
    #[serde(default = "default_role")]
    pub role: String,
    pub created_at: String,
}

fn default_role() -> String {
    "student".to_string()
}

impl User {
    pub fn to_response(&self) -> UserResponse {
        UserResponse {
            id: self.id.clone(),
            email: self.email.clone(),
            name: self.name.clone(),
            phone: self.phone.clone(),
            ministry_role: self.ministry_role.clone(),
            church_name: self.church_name.clone(),
            calling: self.calling.clone(),
            selected_course_id: self.selected_course_id.clone(),
            verified: self.verified,
            payment_complete: self.payment_complete,
            role: self.role.clone(),
            created_at: self.created_at.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub name: String,
    pub phone: String,
    pub ministry_role: String,
    pub church_name: String,
    pub calling: String,
    pub selected_course_id: Option<String>,
    pub verified: bool,
    pub payment_complete: bool,
    pub role: String,
    pub created_at: String,
}
