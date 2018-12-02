error_chain! {
    foreign_links {
        Pug(pug::errors::Error);
        R2d2(r2d2::Error);
        Validation(validator::ValidationErrors);
    }
}
