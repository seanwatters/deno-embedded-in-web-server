use actix_web::{get, App, HttpServer, Responder};

use std::cell::RefCell;

use deno_core::DetachedBuffer;
use deno_core::JsRuntime;
use deno_core::RuntimeOptions;
use deno_core::serde_v8;
use deno_core::v8;

thread_local! {
    static JS_RUNTIME: RefCell<JsRuntime> = {
        RefCell::new(JsRuntime::new(RuntimeOptions::default()))
    }
}

fn run_js(src: &str) -> DetachedBuffer {
    JS_RUNTIME.with(|js_runtime| {
        let mut js_runtime = js_runtime.borrow_mut();        

        let result = js_runtime.execute_script("<usage>", src).unwrap();

        let scope = &mut js_runtime.handle_scope();
        let local = v8::Local::new(scope, result);

        serde_v8::from_v8::<DetachedBuffer>(scope, local).unwrap()
    })
}

#[get("/js")]
async fn greet() -> impl Responder {
    format!("Result: {:?}", run_js("new Uint8Array([0,1,2,3])").as_ref())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}