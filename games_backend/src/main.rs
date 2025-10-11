#[macro_use]
extern crate rocket;

#[derive(Debug)]
struct RessiveResponse {
    message: String,
    code: u8,
}

impl RessiveResponse {
    fn from_string(str: String) -> Self {
        let new_str = str.split(',');
        let mut vec_str: Vec<String> = Vec::new();
        new_str.for_each(|mut x| {
            x = x.trim();
            let y = x.split(':');
            y.for_each(|mut val| {
                val = val.trim();
                vec_str.push(val.to_string())
            });
        });

        let code = vec_str.pop().expect("Invalid code!");
        let code: u8 = code.parse().expect("Couldn't parse code!");
        let _ = vec_str.pop();
        let message = vec_str.pop().expect("Couldn't parse message");
        let _ = vec_str.pop();

        println!("Ressive Response: {} and  {}", code, message);

        RessiveResponse {
            message: message,
            code: code,
        }
    }
}

#[get("/")] // route attribute
fn index() -> &'static str {
    "Hello, World!" // request handler
}

#[get("/get_all")]
fn give_all_games() -> String {
    let var = RessiveResponse::from_string(String::from("message: ok, code: 404"));
    format!("{:?}", var)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, give_all_games])
}
