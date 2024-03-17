use actix_web::{web, App, HttpRequest, HttpServer, HttpResponse, Result};
use serde::{Deserialize};

#[derive(Deserialize)]
struct GcdParam {
    n: u64,
    m: u64,
}

async fn index(req: HttpRequest) -> HttpResponse {
    println!("REQ: {req:?}");
    // "Hello world!"
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method=POST>
                    <input name="n"/>
                    <input name="m"/>
                    <button type="submit">Compute GCD</button>
                </form>
            "#,
        )
}

async fn post_gcd(form: web::Form<GcdParam>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring.");
    }
    // println!("{}", form.n);
    // println!("{}", form.m);

    let response =
        format!("The greatest common divisor of the number {} and {} is <b>{}</b>\n",
            form.n, form.m, gcd(form.n, form.m));

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/gcd", web::post().to(post_gcd))
            // .service(web::resource("/gcd").to(post_gcd))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}