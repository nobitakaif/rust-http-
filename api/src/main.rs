use poem::listener::TcpListener;
use poem::post;
use poem::web::Json;
use poem::{Route, Server, handler,middleware::Tracing, web::Path,get,EndpointExt };

use crate::reponse_output::CreateWebsiteOutput;
use crate::request_input::CreateWebsiteInput;


#[handler]
// Path args is dynamic routing just like we do in exp /home/:name but rather than we get req.params.name we get only :name string here we can also get use multiple dynamic like Path(name):Path<string>, Path(city):Path<String> 
fn hello(Path(name):Path<String>)->String{
    format!("hello : {name}")
}
pub mod request_input;
pub mod reponse_output;


#[handler]
fn website(Json(data): Json<CreateWebsiteInput>)-> Json<CreateWebsiteOutput>{
    let url = data.url;
    let res = CreateWebsiteOutput{
        id : url
    };
    Json(res)
}
#[tokio::main]
async fn main()-> Result<(), std::io::Error>{
    let app = Route::new()
        .at("/home/:name", get(hello))
        .at("/website", post(website));

    Server::new(TcpListener::bind("0.0.0.0:8000")).name("hello-world").run(app).await
}