use handlebars::Handlebars;

/// Function creates HandleBars instance and registers html files
pub fn generate_template() -> Handlebars<'static> {
    let mut template: Handlebars = Handlebars::new();
    template
        .register_template_file("index", "templates/index.html.hbs")
        .unwrap();
    template
        .register_template_file("base", "templates/base.html.hbs")
        .unwrap();
    template
        .register_template_file("articles", "templates/articles.html.hbs")
        .unwrap();
    template
        .register_template_file("404", "templates/404.html.hbs")
        .unwrap();
    template
        .register_template_file("signup", "templates/signup.html.hbs")
        .unwrap();

    // return generated template
    template
}
