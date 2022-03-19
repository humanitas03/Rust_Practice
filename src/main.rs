use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize; // for Deserializer

mod gcd;

/*page 38
 * https://actix.rs/

*/
fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    /*
        https://doc.rust-lang.org/book/ch01-02-hello-world.html
        println! calls a Rust macro. If it called a function instead,
        it would be entered as println (without the !).
        We’ll discuss Rust macros in more detail in Chapter 19.
        For now, you just need to know that using a ! means that you’re calling a macro instead of a normal function
    */
    println!("Serving on http://localhost:3000 ....");
    server
        .bind("127.0.0.1:3000").expect("error binding server to address")
        .run().expect("error running server");
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
            <title>GCD Calculator</title>
            <form action="/gcd" method="post">
            <input type="text" name="n"/>
            <input type="text" name="m"/>
            <button type="submit">Compute GCD</button>
            </form>
            "#,
        )
}

fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest ()
            .content_type("text/html")
            .body("<h2>Computing the GCD with zero is boring. </h2>");
    }

    let response = format!("The greatest common divisor of the numbers {} and {} \
        is <b>{}</b> \n", form.n, form.m, gcd::gcd(form.n, form.m));

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

/*
tells the serde crate to examine the
type when the program is compiled and automatically
generate code to parse a value of this type from data in the
format that HTML forms use for POST requests.
*/
#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}