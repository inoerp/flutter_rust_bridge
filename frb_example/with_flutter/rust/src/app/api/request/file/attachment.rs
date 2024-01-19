use crate::app::api::request::get_request::GetRequest;
use crate::app::system::error::no_value::NoValueFoundError;
use actix_web::{ web::Bytes, HttpResponse};
use anyhow::Result;
use mime_guess::from_path;
use std::fs::File;
use std::io::{ Read};
use std::path::Path;

pub struct DownloadAttachment;

impl<'a> GetRequest<'a> {
    pub async fn download(&self) -> Result<HttpResponse, NoValueFoundError> {
        // Get the base file dir
        let server_path = std::env::current_dir()
            .map_err(|_err| NoValueFoundError::new("Unable to create pathBuf"))?;

        let ref_entity = &self.qd.request_data.url_data;
        let mut file_name = None;
        let mut file_path = None;

        for url_data in ref_entity {
            if url_data.key.eq_ignore_ascii_case("fileName") {
                file_name = Some(url_data.value.to_owned());
            } else if url_data.key.eq_ignore_ascii_case("filePath") {
                file_path = Some(url_data.value.to_owned());
            }
        }

        let file_name = match file_name {
            Some(name) => name.trim().replace('\'', "").replace("%27", ""),
            None => return Ok(HttpResponse::BadRequest().body("File name is not found")),
        };

        let file_path = match file_path {
            Some(path) => path.trim().replace('\'', "").replace("%27", ""),
            None => return Ok(HttpResponse::BadRequest().body("File path is not found")),
        };
        let file_name = file_name.replace('\'', "");
        let file_path = file_path.replace('\'', "");

        let out_path = server_path.join(Path::new(&file_path)).join(Path::new(&file_name));

        let mut file_to_download =
            File::open(out_path).map_err(|_| NoValueFoundError::new("File not found."))?;

        let mut file_content = Vec::new();
        file_to_download
            .read_to_end(&mut file_content)
            .map_err(|_err| NoValueFoundError::new("Unable to read the file."))?;

        let file_content_type = from_path(&file_name).first_or_octet_stream();
        let file_size = file_content.len().to_string();

        let res = HttpResponse::Ok()
            .append_header((
                "Content-Type",
                format!("{};file_download", file_content_type),
            ))
            .append_header((
                "Content-Disposition",
                format!("attachment; filename={}", file_name),
            ))
            .append_header(("Content-Length", file_size))
            .body(Bytes::from(file_content.to_owned()));

        Ok(res)
    }

}

impl DownloadAttachment {
}


#[cfg(test)]
mod tests{
    use std::path::Path;

    #[test]
    fn test_file_path(){
      let f_path1 = "'files\\upload'";
      let f_name1 = "'base-example.xml'";
      let f_path2 = f_path1.trim().replace('\'', "");
      let f_name2 = f_name1.trim().replace('\'', "");
      let f_path = f_path2.as_str();
      let f_name = f_name2.as_str();
      
      let curr_dir = std::env::current_dir().expect("Unable to find current dir");
      let final_path = curr_dir.join(f_path);
      let final_path2 = final_path.join(f_name);
      let curr_dir = std::env::current_dir().expect("Unable to find current dir");
      let final_path3 = curr_dir.join(Path::new(&f_path)).join(Path::new(&f_name));
      log::info!("final_path2 {:?} & final_path2 {:?}", final_path2, final_path3);
    }
}
