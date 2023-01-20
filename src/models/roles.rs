use diesel::{Identifiable, Queryable, Insertable, Associations};
use serde::{Serialize, Deserialize};
use crate::database::schema::{users_roles, api_roles};
use crate::models::users::User;

#[derive(Identifiable, Queryable, Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = api_roles)]
pub struct Role {
    pub id: i32,
    pub name: String,
}


#[derive(Identifiable, Queryable, Debug, Associations, Serialize, Deserialize)]
#[belongs_to(User)]
#[diesel(belongs_to(Role) )]
#[diesel(table_name = users_roles)]
pub struct UserRole {
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
}
