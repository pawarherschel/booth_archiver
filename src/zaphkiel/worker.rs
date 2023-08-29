use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    Response::ok(format!(
        "Hello from Rust!\n Request path is {}.\n Context is {:#?}",
        req.path(),
        ctx
    ))
}
