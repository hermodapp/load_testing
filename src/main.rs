use goose::prelude::*;
use rand::Rng;

const JWT_TOKEN : &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIzNDRlM2I0NS1lZjMyLTRhYjAtODllOS02MTIwMWJkMTU1NTkiLCJpYXQiOjE2MzUyOTYwNDUsImV4cCI6MTYzNTI5OTY0NX0.9aVTDTDintPSKFiUvDQX4w2tbwxcSyMO2u2odCFgpRw";

#[tokio::main]
async fn main() -> Result<(), GooseError> {
    GooseAttack::initialize()?
        // In this example, we only create a single taskset, named "WebsiteUser".
        .register_taskset(taskset!("WebsiteUser").register_task(task!(authenticated_index)))
        .execute()
        .await?
        .print();

    Ok(())
}

/// A very simple task that simply loads the front page.
async fn authenticated_index(user: &mut GooseUser) -> GooseTaskResult {
    let uid = rand::thread_rng().gen_range(100_000_000..999_000_000);
    let url = format!("/qr_code/store?slug={}&generation_data={}", &uid, &uid);
    let request = user.goose_get(&url)?.header("Authorization", JWT_TOKEN);
    let _goose = user.goose_send(request, None).await?;

    Ok(())
}
