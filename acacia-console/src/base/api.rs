use ureq::Error;

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

    pub fn get(path: &str) -> Result<String, Error> {
        let url = format!("{}{}", Self::ADDRESS, path);
        let mut response = ureq::get(&url)
            .header("X-Token", Self::ADMIN_TOKEN)
            .config()
            .http_status_as_error(false)
            .build()
            .call()?;
        let body_string = response.body_mut().read_to_string()?;
        Ok(body_string)
    }

    pub fn post(path: &str, body: String) -> Result<String, Error> {
        let url = format!("{}{}", Self::ADDRESS, path);
        let mut response = ureq::post(&url)
            .header("X-Token", Self::ADMIN_TOKEN)
            .header("Content-Type", "application/json")
            .config()
            .http_status_as_error(false)
            .build()
            .send(body)?;
        let body_string = response.body_mut().read_to_string()?;
        Ok(body_string)
    }

    pub fn response_to_string(result: Result<String, Error>) -> String {
        match result {
            Ok(body) => Self::parse_response(&body).unwrap_or_else(|error| format!("[ERR] {error}")),
            Err(error) => format!("[ERR] {error}"),
        }
    }

    fn parse_response(body: &str) -> Result<String, serde_json::Error> {
        let response = serde_json::from_str::<ApiResponse<serde_json::Value>>(body)?;
        if response.code != 0 {
            return Ok(format!("[ERR] {}", response.message));
        }
        if let Some(data) = response.data.as_str() {
            return Ok(data.to_string());
        }
        Ok(response.message)
    }
}

#[macro_export]
macro_rules! acacia_api_get {
    ($path:expr) => {{
        $crate::base::api::AcaciaApi::response_to_string($crate::base::api::AcaciaApi::get($path))
    }};
}

#[macro_export]
macro_rules! acacia_api_post {
    ($path:expr, $body:tt) => {{
        $crate::base::api::AcaciaApi::response_to_string($crate::base::api::AcaciaApi::post(
            $path,
            serde_json::json!($body).to_string(),
        ))
    }};
}
