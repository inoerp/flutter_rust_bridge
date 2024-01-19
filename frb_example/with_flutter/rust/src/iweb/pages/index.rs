use std::collections::HashMap;

use crate::{app::system::error::no_value::NoValueFoundError, iweb::entity::cms::site_info::SiteInfo};
use crate::iweb::entity::cms::content_summary::ContentSummary;
use crate::model::state::global_state::GlobalState;
use actix_web::{web, HttpRequest, Responder, Result};
use actix_web_lab::respond::Html;
use askama::Template;

#[derive(Template)]
#[template(path = "user.html")]
struct UserTemplate<'a> {
    name: &'a str,
    text: &'a str,
}

#[derive(Template)]
#[template(path = "category-list.html")]
struct CategoryList<'a> {
    data: &'a Vec<ContentSummary>,
    site_info: SiteInfo,
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index {
    // site_details: &'a SiteDetails<'a>,
}

pub async fn category_list(
    req: HttpRequest,
    gs: web::Data<GlobalState>,
    query: web::Query<HashMap<String, String>>,
) -> Result<impl Responder> {
    let html = category_html_for_list(req, gs, query).await;
    match html {
        Ok(val) => Ok(Html(val)),
        Err(err) => {
            println!("Error in getting content categories. Err {:?}", err);
            let no_data_content = CategoryList {
                data: &vec![ContentSummary::no_content()],
                site_info: SiteInfo::no_content(),
            }
            .render();
            match no_data_content {
                Ok(val) => Ok(Html(val)),
                Err(_err) => Ok(Html("No data found".to_string())),
            }
        }
    }
}

pub async fn category_html_for_list(
    _req: HttpRequest,
    gs: web::Data<GlobalState>,
    _query: web::Query<HashMap<String, String>>,
) -> Result<String, NoValueFoundError> {
    let pools = gs.conn_pools.lock().await;
    let conn_mapping = pools
        .get("ierp")
        .ok_or_else(|| NoValueFoundError::new("No db connection found"))?;
    //let data = getData(conn_mapping).await;
    let all_rows = ContentSummary::find_all(conn_mapping).await?;
    let site_info: SiteInfo = SiteInfo::get_site_info(conn_mapping).await?;
    if all_rows.is_empty() {
        let no_doc_page = CategoryList {
            data: &vec![ContentSummary::no_content()],
            site_info,
        }
        .render()
        .map_err(|err| {
            NoValueFoundError::new(format!("Invalid template. Error {:?}", err).as_str())
        })?;
        return Ok(no_doc_page);
    }

    let html = CategoryList {
        data: &all_rows,
        site_info,
    }
    .render()
    .map_err(|err| NoValueFoundError::new(format!("Invalid template. Error {:?}", err).as_str()))?;

    Ok(html)
}

pub async fn index(query: web::Query<HashMap<String, String>>) -> Result<impl Responder> {
    let html = if let Some(name) = query.get("name") {
        UserTemplate {
            name,
            text: "Welcome!",
        }
        .render()
        .expect("template should be valid")
    } else {
        Index {
            // site_details: &SiteDetails {
            //     name: "WebSiteName!",
            //     description: "Learn Oracle",
            // },
        }
        .render()
        .expect("template should be valid")
    };

    Ok(Html(html))
}
