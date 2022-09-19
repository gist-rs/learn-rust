use tokio::task;

pub async fn fetch(target: String) -> String {
    let body = reqwest::get(target).await.unwrap().text().await.unwrap();
    body
}

#[tokio::main]
async fn main() {
    // let foo = task::spawn(fetch(
    //     "https://raw.githubusercontent.com/katopz/hello-rust-cloudrun/main/Cargo.toml".to_owned(),
    // ));
    // let bar = task::spawn(fetch(
    //     "https://raw.githubusercontent.com/katopz/hello-rust-cloudrun/main/Dockerfile".to_owned(),
    // ));

    // let foo_body = foo.await.unwrap();
    // let bar_body = bar.await.unwrap();

    // println!("{:?}", foo_body);
    // println!("{:?}", bar_body);

    // -------------------------------

    let foo_list = vec![
        "https://raw.githubusercontent.com/katopz/hello-rust-cloudrun/main/Cargo.toml",
        "https://raw.githubusercontent.com/katopz/hello-rust-cloudrun/main/Dockerfile",
    ];
    let tasks = foo_list
        .into_iter()
        .map(|target| task::spawn(reqwest::get(target.to_owned())));

    let mut bodies = vec![];
    for task in tasks {
        let body = task.await;
        bodies.push(body.unwrap());
    }

    println!("{:#?}", bodies);
}
