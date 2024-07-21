use rocket_user::routers::rocket;
use rocket::launch;
#[launch]
fn launch() -> _ {
    rocket()
}