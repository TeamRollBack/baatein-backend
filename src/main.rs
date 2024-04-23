mod repositories;

use repositories::user_repo;

#[tokio::main]
async fn main() -> Result<(), ()> {
    println!("Hello, world!");

    let user_coll = user_repo::UserColl::init().await?;
    let u = user_repo::User {
        first_name: "Rohit".to_string(),
        last_name: "Mokashi".to_string(),
        username: "rohitmokashi".to_string(),
        gender: user_repo::Gender::Male,
        dob: "2003-06-12".to_string(),
    };

    user_coll.add_user(u).await;

    Ok(())
}
