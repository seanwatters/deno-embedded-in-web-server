use actix_web::{get, App, HttpServer, Responder};

use deno_core::DetachedBuffer;
use deno_core::serde_v8;
use deno_core::v8;

fn run_js(src: &str) -> DetachedBuffer {
    let isolate = &mut v8::Isolate::new(v8::CreateParams::default());
    
    let handle_scope = &mut v8::HandleScope::new(isolate);
    
    let context = v8::Context::new(handle_scope);
    
    let scope = &mut v8::ContextScope::new(handle_scope, context);

    let code = v8::String::new(scope, src).unwrap();

    let script = v8::Script::compile(scope, code, None).unwrap();
    let result = script.run(scope).unwrap();

    serde_v8::from_v8::<DetachedBuffer>(scope, result).unwrap()
}

#[get("/js")]
async fn greet() -> impl Responder {
    format!("Result: {:?}", run_js("new Uint8Array([0,1,2,3])").as_ref())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let platform = v8::new_default_platform(0, false).make_shared();

    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    HttpServer::new(|| {
        App::new().service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}