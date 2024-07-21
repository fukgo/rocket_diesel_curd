use diesel::ExpressionMethods;
use rocket_dyn_templates::Template;
use rocket::{get, post};
use rocket_dyn_templates::context;
use rocket::form::Form;
use rocket::FromForm;
use rocket::response::Redirect;
use diesel::{QueryDsl,RunQueryDsl};
#[derive(FromForm)]
pub struct RegisterForm{
    username: String,
    password: String,
    confirm_password: String,
    email: String,
}

#[derive(FromForm)]
pub struct LoginForm{
    username: String,
    password: String,
}

impl RegisterForm {
    fn validate(&self) -> Result<(), String> {
        if self.username.is_empty() {
            return Err("用户名不能为空".to_string());
        }
        if self.password.is_empty() {
            return Err("密码不能为空".to_string());
        }
        if self.password != self.confirm_password {
            return Err("两次输入的密码不一致".to_string());
        }
        if self.email.is_empty() {
            return Err("邮箱不能为空".to_string());
        }
        // 当邮箱格式不正确时，返回错误信息
        if !crate::is_email_valid(&self.email) {
            return Err("邮箱地址格式错误".to_string());
        }
            Ok(())
        }
    fn username_is_not_exist(&self,query_username: &str) -> bool {
        // 查询数据库，判断用户名是否存在
        use crate::schema::user::dsl::*;
        let conn = &mut crate::establish_connection();
    
        // 使用 match 表达式处理 Result
        match user.filter(username.eq(query_username)).first::<crate::models::User>(conn) {
            Ok(_) => false, // 如果查询成功，表示用户名存在，返回 false
            Err(diesel::result::Error::NotFound) => true, // 如果没有找到用户，表示用户名不存在，返回 true
            Err(_) => false, // 对于其他错误，这里简化处理为返回 false，实际应用中可能需要更复杂的错误处理
        }
    }
    fn add_user(&self) -> Result<(), String> {
        // 添加用户到数据库
        use crate::schema::user;
        use crate::models::NewUser;
        let conn = &mut crate::establish_connection();
    
        // 使用 bcrypt 哈希密码
        let hashed_password = bcrypt::hash(&self.password, bcrypt::DEFAULT_COST).unwrap();
    
        // 创建一个新用户
        let new_user = NewUser {
            username: &self.username,
            password: &hashed_password,
            email: &self.email,
        };
    
        // 插入新用户到数据库
        diesel::insert_into(user::table)
            .values(&new_user)
            .execute(conn)
            .map_err(|_| "添加用户失败".to_string())?;
    
        Ok(())
    }
}

impl LoginForm{
    fn validate(&self)-> Result<(), String>{
        if self.username.is_empty() {
            return Err("用户名不能为空".to_string());
        }
        if self.password.is_empty() {
            return Err("密码不能为空".to_string());
        }
        Ok(())
    }
    fn username_and_password_is_match(&self,match_username:&str,match_password:&str)->bool{
        // 查询数据库，判断用户名和密码是否匹配
        use crate::schema::user::dsl::*;
        let conn = &mut crate::establish_connection();
    
        // 使用 match 表达式处理 Result
        match user.filter(username.eq(match_username)).first::<crate::models::User>(conn) {
            Ok(User) => {
                // 如果查询成功，表示用户名存在
                // 验证密码是否匹配
                if bcrypt::verify(match_password, &User.password).unwrap() {
                    // 如果密码匹配，返回 true
                    true
                } else {
                    // 如果密码不匹配，返回 false
                    false
                }
            }
            Err(diesel::result::Error::NotFound) => false, // 如果没有找到用户，表示用户名不存在，返回 false
            Err(_) => false, // 对于其他错误，这里简化处理为返回 false，实际应用中可能需要更复杂的错误处理
        }
    }
}
#[get("/")]
pub fn home_page() -> Template {
    Template::render("home", context! {
        title: "Home",
        message: "Welcome to the home page!"
    })
}


#[get("/")]
pub fn index() -> Template {
    Template::render("index", context! {
        title: "Welcome",
        message: "Hello, world!"
    })
}

#[get("/")]
pub fn login() -> Template {
    Template::render("user/login", context! {
        title: "登录",
        message: "Welcome to the login page!"
    })
}

#[get("/")]
pub fn register() -> Template {
    Template::render("user/register", context! {
        title: "注册",
        message: "Welcome to the register page!"
    })
}
#[post("/", data = "<user_form>")]
pub fn register_post(user_form: Form<RegisterForm>) -> Result<Redirect, Template> {
    match user_form.validate(){
        Ok(_)=>{
            if user_form.username_is_not_exist(&user_form.username) {
                match user_form.add_user() {
                    Ok(_) => Ok(Redirect::to("/login")),
                    Err(e) => Err(Template::render("user/register", context! {
                        title: "注册",
                        error: e,
                    })),
                }
            }else {
                Err(Template::render("user/register", context! {
                    title: "注册",
                    error: "用户名已存在",
                }))
            
            }
        }
        Err(e)=>{
            // 验证失败的情况
        Err(Template::render("user/register", context! {
            title: "注册",
            error: e,
        }))
        }
    }

}
#[post("/", data = "<user_form>")]
pub fn login_post(user_form: Form<LoginForm>) -> Result<Redirect, Template> {
    match user_form.validate(){
        Ok(_) => {
            if user_form.username_and_password_is_match(&user_form.username,&user_form.password) {
                Ok(Redirect::to("/home"))
            } else {
                Err(Template::render("user/login", context! {
                    title: "登录",
                    error: "用户名或密码错误",
                }))
            }
        },
        Err(e)=>{
            Err(Template::render("user/login", context! {
                title: "登录",
                error: e,
            }))
        }
    }

}

