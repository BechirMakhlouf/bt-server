use actix_web::guard::{self, Guard};

pub fn cookie_guard() -> impl Guard {
    guard::fn_guard(|ctx| ctx.head().headers().contains_key("Cookie"))
}
