use color_eyre::eyre::eyre;
use serde::de::DeserializeOwned;

#[derive(serde::Deserialize)]
struct ApiResponse<T> {
    code: i32,
    message: String,
    data: T,
}

pub struct AcaciaApi;

impl AcaciaApi {
    const ADDRESS: &str = "http://127.0.0.1:52000";
    const ADMIN_TOKEN: &str = "8vCzZoprtbQALRt1VzrJi60Xw6RTjjUHxLwk5v69f3VuHzgyXZ1xmd3Ydhz072kA";

    fn deserialize_data<T: DeserializeOwned>(body: &str) -> color_eyre::Result<T> {
        let response = serde_json::from_str::<ApiResponse<T>>(body)?;
        if response.code != 0 {
            return Err(eyre!(response.message));
        }
        Ok(response.data)
    }

    pub fn get<T: DeserializeOwned>(path: &str) -> color_eyre::Result<T> {
        let url = format!("{}{}", Self::ADDRESS, path);
        let mut response = ureq::get(&url).header("X-Token", Self::ADMIN_TOKEN).config().http_status_as_error(false).build().call()?;
        let body = response.body_mut().read_to_string()?;
        Self::deserialize_data(&body)
    }

    pub fn post<T: DeserializeOwned>(path: &str, body: String) -> color_eyre::Result<T> {
        let url = format!("{}{}", Self::ADDRESS, path);
        let mut response = ureq::post(&url)
            .header("X-Token", Self::ADMIN_TOKEN)
            .header("Content-Type", "application/json")
            .config()
            .http_status_as_error(false)
            .build()
            .send(body)?;
        let body = response.body_mut().read_to_string()?;
        Self::deserialize_data(&body)
    }
}
