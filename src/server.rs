use std::path::{Path, PathBuf};

use rocket::fs::NamedFile;
use rocket::{figment, get, routes, Build, Rocket};

pub struct Server {
    address: String,
    port: u16,
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("generates/index.html").await.ok()
}

#[get("/<file..>")]
async fn file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("generates/").join(file))
        .await
        .ok()
}

impl Server {
    pub fn new(address: String, port: u16) -> Server {
        Server { address, port }
    }

    fn rocket(&self) -> Rocket<Build> {
        let config = figment::Figment::from(rocket::Config::default());
        let config = config
            .merge(("port", self.port))
            .merge(("address", self.address.clone()));

        rocket::custom(config).mount("/", routes![file, index])
    }

    pub async fn serve(&self) {
        self.rocket().launch().await.ok();
    }
}
