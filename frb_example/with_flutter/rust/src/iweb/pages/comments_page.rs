use std::collections::HashMap;

use crate::{app::system::error::no_value::NoValueFoundError, iweb::entity::cms::site_info::SiteInfo};
use crate::iweb::entity::cms::comment::Comment;
use crate::model::state::global_state::GlobalState;
use actix_web::{web, HttpRequest, Responder, Result};
use actix_web_lab::respond::Html;
use askama::Template;

#[derive(Template)]
#[template(path = "comments.html")]
struct CommentsPage<'a> {
    // pub site_details: &'a SiteDetails<'a>,
    data: &'a Vec<Comment>,
    site_info: SiteInfo,
}

pub async fn comments_page(
    req: HttpRequest,
    gs: web::Data<GlobalState>,
    query: web::Query<HashMap<String, String>>,
) -> Result<impl Responder> {
    let html = get_html_for_comments_page(req, gs, query).await;
    match html {
        Ok(val) => Ok(Html(val)),
        Err(err) => {
            println!("Error in getting comments. Err {:?}", err);
            let comments_page = CommentsPage {
                data: &vec![Comment::no_content()],
                site_info: SiteInfo::no_content(),
            }
            .render();
            match comments_page {
                Ok(val) => Ok(Html(val)),
                Err(_err) => Ok(Html("No data found".to_string())),
            }
        }
    }
}

pub async fn get_html_for_comments_page(
    _req: HttpRequest,
    gs: web::Data<GlobalState>,
    query: web::Query<HashMap<String, String>>,
) -> Result<String, NoValueFoundError> {
    let pools = gs.conn_pools.lock().await;
    let conn_mapping = pools
        .get("ierp")
        .ok_or_else(|| NoValueFoundError::new("NO_DOC_FOUND"))?;

    let id = if let Some(id) = query.get("id") {
        id.as_str()
    } else {
        "1"
    };
    let all_rows = Comment::find_all(conn_mapping, id).await?;
    let site_info: SiteInfo = SiteInfo::get_site_info(conn_mapping).await?;
    if all_rows.is_empty() {
        let comments_page = CommentsPage {
            data: &vec![Comment::no_content()],
            site_info
        }
        .render()
        .map_err(|err| {
            NoValueFoundError::new(format!("Invalid template. Error {:?}", err).as_str())
        })?;
        return Ok(comments_page);
    }

    let html = CommentsPage {
        // site_details: &SiteDetails {
        //     name: "WebSiteName",
        //     description: "Learn Oracle",
        // },
        data: &all_rows,
        site_info
    }
    .render()
    .map_err(|err| NoValueFoundError::new(format!("Invalid template. Error {:?}", err).as_str()))?;

    Ok(html)
}
