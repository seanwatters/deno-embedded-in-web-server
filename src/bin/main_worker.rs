use actix_web::{get, App, HttpServer, Responder};

use std::rc::Rc;
use std::sync::Arc;

use deno_core::error::AnyError;
use deno_core::DetachedBuffer;
use deno_core::FsModuleLoader;
use deno_core::serde_v8;
use deno_core::v8;

use deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_runtime::deno_web::BlobStore;
use deno_runtime::permissions::PermissionsContainer;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;
use deno_runtime::BootstrapOptions;

fn get_error_class_name(e: &AnyError) -> &'static str {
    deno_runtime::errors::get_error_class_name(e).unwrap_or("Error")
}

fn run_js(src: &str) -> DetachedBuffer {
    let module_loader = Rc::new(FsModuleLoader);
    let create_web_worker_cb = Arc::new(|_| {
        todo!("Web workers are not supported in the example");
    });
    let web_worker_event_cb = Arc::new(|_| {
        todo!("Web workers are not supported in the example");
    });

    let options = WorkerOptions {
        bootstrap: BootstrapOptions {
            args: vec![],
            cpu_count: 1,
            debug_flag: false,
            enable_testing_features: false,
            locale: deno_core::v8::icu::get_language_tag(),
            location: None,
            no_color: false,
            is_tty: false,
            runtime_version: "x".to_string(),
            ts_version: "x".to_string(),
            unstable: true,
            user_agent: "hello_runtime".to_string(),
            inspect: false,
        },
        extensions: vec![],
        extensions_with_js: vec![],
        startup_snapshot: None,
        unsafely_ignore_certificate_errors: None,
        root_cert_store: None,
        seed: None,
        source_map_getter: None,
        format_js_error_fn: None,
        web_worker_preload_module_cb: web_worker_event_cb.clone(),
        web_worker_pre_execute_module_cb: web_worker_event_cb,
        create_web_worker_cb,
        maybe_inspector_server: None,
        should_break_on_first_statement: false,
        should_wait_for_inspector_session: false,
        module_loader,
        npm_resolver: None,
        get_error_class_fn: Some(&get_error_class_name),
        cache_storage_dir: None,
        origin_storage_dir: None,
        blob_store: BlobStore::default(),
        broadcast_channel: InMemoryBroadcastChannel::default(),
        shared_array_buffer_store: None,
        compiled_wasm_module_store: None,
        stdio: Default::default(),
    };

    let main_module = deno_core::resolve_path("").unwrap();
    let permissions = PermissionsContainer::allow_all();

    let mut main_worker = MainWorker::bootstrap_from_options(
        main_module,
        permissions,
        options,
    );


    let result = main_worker.execute_script("<usage>", src).unwrap();

    let scope = &mut main_worker.js_runtime.handle_scope();
    let local = v8::Local::new(scope, result);

    serde_v8::from_v8::<DetachedBuffer>(scope, local).unwrap()
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