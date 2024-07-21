use rocket::{Rocket,routes};
use crate::views::{index, login, register,register_post,login_post,home_page};
use rocket_dyn_templates::Template;
pub fn rocket() -> Rocket<rocket::Build> {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index])
        .mount("/login/", routes![login,login_post])
        .mount("/register/", routes![register,register_post])
        .mount("/home/", routes![home_page])
}