pub mod handlerequests {

    use std::{fs};

    use hyper::header::HOST;
    use hyper::{Request, Response, body::Incoming};
    use hyper::http::StatusCode;

    use http_body_util::Full;
    use bytes::Bytes;

    use crate::daisy_helpers::helperfunctions::generate_base;
    use crate::{
        daisy_conf::load_config,
        daisy_dr::load_domain,
        daisy_helpers::helperfunctions::css_request_parser
    }; 


    pub async fn handle_request(
        req: Request<Incoming>,
    ) -> Result<Response<Full<Bytes>>, hyper::Error> {

        let config = load_config();
        let router = load_domain();

        let uri_path = req.uri().path();
        let host_name = req.headers().get(HOST).and_then( | val| val.to_str().ok());

        let requested_path = uri_path.trim_matches('/');

        let is_styles = css_request_parser(requested_path);

        let base = generate_base(config, router, host_name);

        let file_path = if requested_path.is_empty() {
            base.join("index.html")
        } else if is_styles {
            base.join(requested_path)
        } else {
            base.join(format!("{}.html", requested_path))
        };


        println!("REQUESTED PATH: {}", uri_path);
        println!("REQUESTED HOST: {:?}", host_name.unwrap());
        println!("IS CSS: {}", is_styles);
        println!("FINAL FILE PATH: {:?}", file_path);


        let (status, contents) = match fs::read(&file_path) {
            Ok(file) => (StatusCode::OK, file),
            Err(_) => {
                let not_found = base.join("404.html");
                (
                    StatusCode::NOT_FOUND,
                    fs::read(not_found).unwrap_or(b"404 NOT FOUND".to_vec()),
                )
            }
        };
        

        let content_type = if is_styles {
            "text/css"
        } else {
            "text/html"
        };

        println!("CONTENT TYPE SENT: {}", content_type);

        let response = Response::builder()
                .status(status)
                .header("Content-Type", content_type)
                .body(Full::new(Bytes::from(contents)))
                .unwrap();


        Ok(response)
    }
}