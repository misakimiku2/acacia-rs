use ureq::Error;
pub struct AcaciaApi;

impl AcaciaApi {
    const ADDRESS: &str = "http://127.0.0.1:52000";
    const ADMIN_TOKEN: &str = "8vCzZoprtbQALRt1VzrJi60Xw6RTjjUHxLwk5v69f3VuHzgyXZ1xmd3Ydhz072kA";

    pub fn get(path: &str) -> Result<String, Error> {
        let url = format!("{}{}", Self::ADDRESS, path);
        let mut response = ureq::get(&url).header("X-Token", Self::ADMIN_TOKEN).call()?;
        let body_string = response.body_mut().read_to_string()?;
        Ok(body_string)
    }

    pub fn post(path: &str, body: String) -> Result<String, Error> {
        let url = format!("{}{}", Self::ADDRESS, path);
        let mut response = ureq::post(&url).header("X-Token", Self::ADMIN_TOKEN).header("Content-Type", "application/json").send(body)?;
        let body_string = response.body_mut().read_to_string()?;
        Ok(body_string)
    }
}
