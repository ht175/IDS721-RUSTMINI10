use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

async fn index() -> HttpResponse {
    let html = r#"
        <html>
            <head>
                <title>Calculator</title>
            </head>
            <body>
                <h1>Calculator</h1>
                <form action="/calculate" method="post">
                    <label for="num1">First number:</label>
                    <input type="number" id="num1" name="num1"><br><br>
                    <label for="num2">Second number:</label>
                    <input type="number" id="num2" name="num2"><br><br>
                    <label for="op">Operation:</label>
                    <select id="op" name="op">
                        <option value="add">Addition (+)</option>
                        <option value="sub">Subtraction (-)</option>
                        <option value="mul">Multiplication (*)</option>
                        <option value="div">Division (/)</option>
                    </select><br><br>
                    <button type="submit">Calculate</button>
                </form>
            </body>
        </html>
    "#;
    HttpResponse::Ok().body(html)
}

async fn calculate(params: web::Form<CalculatorParams>) -> HttpResponse {
    let num1 = params.num1;
    let num2 = params.num2;
    let op = &params.op;
    let result = match op.as_str() {
        "add" => num1 + num2,
        "sub" => num1 - num2,
        "mul" => num1 * num2,
        "div" => num1 / num2,
        _ => panic!("Invalid operation"),
    };
    let html = format!("{} {} {} = {}", num1, op, num2, result);
    HttpResponse::Ok().body(html)
}

#[derive(Deserialize)]
struct CalculatorParams {
    num1: i32,
    num2: i32,
    op: String,
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/calculate", web::post().to(calculate))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
