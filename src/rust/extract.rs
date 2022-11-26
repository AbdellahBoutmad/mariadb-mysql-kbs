use crate::{mariadb, mysql};
use futures::join;

pub async fn extract() {
    println!("Run build...");
    join!(extract_mysql(), extract_mariadb());
    println!("All done.");
    println!("End !");
}

async fn extract_mysql() {
    mysql::get_pages();
}

async fn extract_mariadb() {
    mariadb::get_pages();
}
