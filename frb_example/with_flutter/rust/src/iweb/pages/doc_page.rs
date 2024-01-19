use std::collections::HashMap;

use crate::{app::system::error::no_value::NoValueFoundError, iweb::entity::cms::site_info::SiteInfo};
use crate::iweb::entity::cms::content::Content;
use crate::model::state::global_state::GlobalState;
use actix_web::{web, HttpRequest, Responder, Result};
use actix_web_lab::respond::Html;
use askama::Template;

#[derive(Template)]
#[template(path = "docs-page.html")]
struct DocsPage<'a> {
    data: &'a Vec<Content>,
    site_info: SiteInfo,
}

pub async fn docs_page(
    req: HttpRequest,
    gs: web::Data<GlobalState>,
    query: web::Query<HashMap<String, String>>,
) -> Result<impl Responder> {
    let html = get_html_for_docs_page(req, gs, query).await;
    match html {
        Ok(val) => Ok(Html(val)),
        Err(err) => {
            println!("Error in getting docs_page. Err {:?}", err);
                 let no_data_content = DocsPage {
                data: &vec![Content::no_content()],
                site_info:SiteInfo::no_content()
            }
            .render();
            match no_data_content {
                Ok(val) => Ok(Html(val)),
                Err(_err) => Ok(Html("No data found".to_string())),
            }
        }
    }
}

pub async fn get_html_for_docs_page(
    _req: HttpRequest,
    gs: web::Data<GlobalState>,
    query: web::Query<HashMap<String, String>>,
) -> Result<String, NoValueFoundError> {
    let pools = gs.conn_pools.lock().await;
    let conn_mapping = pools
        .get("ierp")
        .ok_or_else(|| NoValueFoundError::new("No db connection found"))?;
    //let data = getData(conn_mapping).await;
    let id = if let Some(id) = query.get("id") {
        id.as_str()
    } else {
        "1"
    };
    let all_rows = Content::find_all(conn_mapping, id).await?;
    let site_info: SiteInfo = SiteInfo::get_site_info(conn_mapping).await?;

    if all_rows.is_empty() {
        let no_doc_page = DocsPage {
            data: &vec![Content::no_content()],
            site_info
        }
        .render()
        .map_err(|err| {
            NoValueFoundError::new(format!("Invalid template. Error {:?}", err).as_str())
        })?;
        return Ok(no_doc_page);
    }

    let html = DocsPage { data: &all_rows, site_info }.render().map_err(|err| {
        NoValueFoundError::new(format!("Invalid template. Error {:?}", err).as_str())
    })?;

    Ok(html)
}
