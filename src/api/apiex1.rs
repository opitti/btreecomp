#[macro_use] extern crate rocket;
use rocket::tokio::time::{sleep, Duration};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/* Solution 1
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
*/

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[rocket::main]
async fn main() {
    #[allow(unused_must_use)]
    rocket::build()
    .mount("/", routes![delay])
        .mount("/hello", routes![index])
        .launch()
        .await;



    }