use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    Error,
};
use actix_web_lab::middleware::Next;

async fn _check_auth_mw(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // Do something with the request here
    // let v = req.app_data::<AppState>();
    // let jwt = match req.cookie(ACCESS_TOKEN_NAME) {
    //     Some(jwt) => jwt.value(),
    //     None => return Err("slfkjd"),
    // };

    next.call(req).await
}
