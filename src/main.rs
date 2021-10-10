use goose::prelude::*;
use std::time::Duration;
use fake::Fake;
use fake::faker::internet::raw::*;
use fake::locales::*;


struct Session {
    user_agent: String,
    ip_address: String
}

#[tokio::main]
async fn main() -> Result<(), GooseError> {
    GooseAttack::initialize()?
        .register_taskset(
            taskset!("WebsiteVisitor;")
                .set_wait_time(Duration::from_secs(1), Duration::from_secs(30))?
                .register_task(task!(init_session).set_on_start())
                .register_task(task!(api_event))
        )
        .execute()
        .await?
        .print();

    Ok(())
}

async fn init_session(user: &mut GooseUser) -> GooseTaskResult {
    let user_agent: String = UserAgent(EN).fake();
    let ip_address: String = IPv4(EN).fake();

    user.set_session_data(Session {
        user_agent: user_agent,
        ip_address: ip_address
    });

    Ok(())
}

async fn api_event(user: &mut GooseUser) -> GooseTaskResult {
    let session = user.get_session_data_unchecked::<Session>();

    let request_builder = user
        .goose_post("api/event")?
        .header("User-Agent", &session.user_agent)
        .header("X-Forwarded-For", &session.ip_address)
        .header("Content-Type", "text/plain")
        .body("{\"n\":\"pageview\",\"u\":\"http://dummy.site\",\"d\":\"loadtest.site\",\"r\":null,\"w\":1666}");

    let _goose = user.goose_send(request_builder, Some("api/event")).await?;

    Ok(())
}
