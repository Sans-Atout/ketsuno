use crate::models::roles::Role;
use crate::models::users::{NewUser, User};
use diesel::{PgConnection, TextExpressionMethods};
use diesel::RunQueryDsl;
use diesel::query_dsl::methods::SelectDsl;

use super::schema::api_roles;
use super::schema::web_user;

pub fn create_user(db: &mut PgConnection, new_user: &NewUser) -> User {
    let user: User = diesel::insert_into(web_user::table)
        .values(new_user)
        .get_result(db)
        .unwrap();
    let user_basic_roles = diesel::query_dsl::methods::FilterDsl::filter(api_roles::table, api_roles::name.like("%_OWN_%")).select(api_roles::id).get_results::<i32>(db);
    //TODO Implement user role implementation and return the user
    return user;
}
