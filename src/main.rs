
extern crate tokio;
extern crate log;
extern crate reqwest;
extern crate mlua;
extern crate serde_json;

pub const BASE_API_URL: &'static str = "https://api.streamelements.com/kappa/v2";

fn get(client: &reqwest::Client, endpoint: &str) -> reqwest::RequestBuilder {
    let url = format!("{}/{}", BASE_API_URL, endpoint);
    client.get(&url)
}

async fn fetch_channel_id(client: &reqwest::Client) -> String {
    get(client, "channels/moscowwbish/")
        .send()
        .await.expect("Failed to get channel id")
        .json::<serde_json::Value>()
        .await
        .map(|v| v["_id"].as_str().unwrap().to_owned())
        .expect("No \"_id\"")
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::builder().build().unwrap();

    let lua = mlua::Lua::new();
    let fetch_id = lua.create_async_function(move |lua: &mlua::Lua, _: Option<String> | {
        let client = client.clone();
        async move {
            let id = fetch_channel_id(&client).await;

            let resp = lua.create_table()?;
            resp.set("id", id)?;

            Ok(resp)
        }
    }
).expect("Failed to create async function");

    lua.globals().set("fetch_id", fetch_id).expect("Failed to set global");
    lua.load(r#"
        local resp = fetch_id()
        print("Received id: " .. resp.id)
    "#).exec_async()
    .await
    .expect("Failed to execute lua code");
}
