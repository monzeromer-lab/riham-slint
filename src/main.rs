use slint::ComponentHandle;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let riham = Riham::new().unwrap();

    let weak = riham.as_weak();
    riham.on_login(move |username, password| {
        let riham = weak.unwrap();
        if username.as_str() == "admin" && password.as_str() == "admin" {
            riham.set_currentPage(slint::SharedString::from("dashboard"));
            return riham.set_loggedin(true);
        }
        riham.invoke_loginError();
    });
    riham.run()
}

