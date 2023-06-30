use poem::{listener::TcpListener, Route};
use poem_openapi::payload::PlainText;
use poem_openapi::{OpenApi, OpenApiService};

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/", method = "get")]
    async fn index(&self) -> PlainText<String> {
        return PlainText("Hello World !".to_string());
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let api_service = OpenApiService::new(Api, "Management API", "0.1.0");
    let ui = api_service.swagger_ui();
    let redoc = api_service.redoc();

    let app = Route::new()
        .nest("/api", api_service)
        .nest("/swagger", ui)
        .nest("/redoc", redoc);

    poem::Server::new(TcpListener::bind("0.0.0.0:5000"))
        .run(app)
        .await
}
