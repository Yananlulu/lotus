use rocket_contrib::Template;

#[get("/")]
fn index() -> Template {
    let ctx = json!({});
    Template::render("index", &ctx)
}
