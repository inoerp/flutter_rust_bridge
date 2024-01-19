mod api;
mod bridge_generated;
pub mod app;
pub mod configuration;
pub mod db;
pub mod iweb;
pub mod model;
pub mod startup;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{http, middleware::Logger, web, App, HttpServer};
use configuration::Settings;
use std::net::TcpListener;

use app::route::{auth, external, standard::user};
use model::state::global_state::GlobalState;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::{fs::File, io::BufReader};

pub async fn run(global_state: GlobalState, address: String) -> Result<(), std::io::Error> {
    let gs: web::Data<GlobalState> = web::Data::new(global_state);
    println!("running HTTP server at {}", address);
    let listener = TcpListener::bind(address)?;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["POST", "GET", "OPTIONS", "PUT", "DELETE", "PATCH"])
                    .allowed_headers(vec![
                        http::header::AUTHORIZATION,
                        http::header::CONTENT_TYPE,
                        http::header::ACCEPT,
                        http::header::CONTENT_LENGTH,
                        http::header::ACCEPT_ENCODING,
                        http::header::X_XSS_PROTECTION,
                    ])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .app_data(gs.clone())
            .service(auth::login::user_login)
            .service(external::get::get_config)
            .service(user::get_me_handler)
            .service(external::get::get_entity)
            .service(external::patch::patch_entity)
            .service(external::patch::patch_action)
            .service(external::post::post_entity)
            .service(external::delete::delete_entity)
            //.service(external::get::get_config)
            //.service(pages::index::index)
            /*required for web*/
            // .service(web::resource("/").route(web::get().to(pages::index::category_list)))
            // .service(web::resource("/index.html").route(web::get().to(pages::index::category_list)))
            // .service(web::resource("/docs-page").route(web::get().to(pages::doc_page::docs_page)))
            // .service(web::resource("/comments").route(web::get().to(pages::comments_page::comments_page)))
            // .service(web::resource("/contents").route(web::get().to(pages::index::category_list)))
            // .service(
            //     Files::new("/assets/", "./templates/assets")
            //     .index_file("index.html")
            //     .show_files_listing()
            //         .prefer_utf8(true),
            // )
            // .service(
            //     Files::new("/sites/", "./templates/assets/sites")
            //         .show_files_listing()
            //         .prefer_utf8(true),
            // )
            /*end of required for web*/
            /*used for rest api*/
            .service(
                Files::new("/", "./assets/static")
                    .index_file("index.html")
                    .show_files_listing()
                    .prefer_utf8(true),
            )
            /*end of used for rest api*/
        // .route("/index.html", web::get().to(external::get::home_page))
        // .route("/", web::get().to(external::get::home_page))
        //.service(external::get::home_page)
    })
    .listen(listener)?
    .run()
    .await
}

pub async fn run_ssl(global_state: GlobalState, address: String) -> Result<(), std::io::Error> {
    //let pool = get_connection_pool().await;
    let gs: web::Data<GlobalState> = web::Data::new(global_state);
    let config = load_rustls_config(&gs.settings);
    log::info!("starting HTTPS server at {}", address);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["POST", "GET", "OPTIONS", "PUT", "DELETE", "PATCH"])
                    .allowed_headers(vec![
                        http::header::AUTHORIZATION,
                        http::header::CONTENT_TYPE,
                        http::header::ACCEPT,
                        http::header::CONTENT_LENGTH,
                        http::header::ACCEPT_ENCODING,
                        http::header::X_XSS_PROTECTION,
                    ])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .app_data(gs.clone())
            .service(auth::login::user_login)
            .service(external::get::get_config)
            .service(user::get_me_handler)
            .service(external::get::get_entity)
            .service(external::patch::patch_entity)
            .service(external::patch::patch_action)
            .service(external::post::post_entity)
            .service(external::delete::delete_entity)
            .service(external::get::get_config)
            .service(
                Files::new("/", "./assets/static")
                    .index_file("index.html")
                    .show_files_listing()
                    .prefer_utf8(true),
            )
        // .route("/index.html", web::get().to(external::get::home_page))
        // .route("/", web::get().to(external::get::home_page))
        //.service(external::get::home_page)
    })
    .bind_rustls(address, config)?
    .run()
    .await
}

fn load_rustls_config(settings: &Settings) -> rustls::ServerConfig {
    // init server config builder with safe defaults
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    // load TLS key/cert files
    let cert_file =
        &mut BufReader::new(File::open(&settings.cert_file).expect("Missing ssl cert file"));
    let key_file =
        &mut BufReader::new(File::open(&settings.key_file).expect("Missing ssl key file"));

    // convert files to key/cert objects
    let cert_chain = certs(cert_file)
        .expect("Invalid ssl cert file")
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .expect("Invalid ssl key file")
        .into_iter()
        .map(PrivateKey)
        .collect();

    // exit if no keys could be parsed
    if keys.is_empty() {
        log::error!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}


mod off_topic_code;
