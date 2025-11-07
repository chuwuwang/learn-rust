use bytes::Bytes;
use http::{ Request, Response };
use http_body_util::Full;
use std::error::Error;
use tower::{ Service, ServiceBuilder };

pub async fn test_server() -> Result< (), Box<dyn Error> > {
    async fn handler(_req: Request< Full<Bytes> > ) -> Result< Response< Full<Bytes> >, Box<dyn Error> > {
        let data = Bytes::from("Hello, World!");
        let body = Full::new(data);
        let response = http::Response::builder()
            .status(200)
            .body(body)
            .unwrap();
        Ok(response)
    }

    let mut service = ServiceBuilder::new().service_fn(handler);

    let request = Request::builder()
        .uri("http://localhost:8080/")
        .body( Full::new( Bytes::new() ) )
        .unwrap();

    let response = service.call(request).await ? ;
    println!("response: {:?}", response);

    Ok( () )
}