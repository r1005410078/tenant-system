use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ResponseBody<T: Serialize> {
    data: Option<T>,
    msg: String,
    code: u16,
}

impl<T: Serialize> ResponseBody<T> {
    pub fn success(data: T) -> Self {
        ResponseBody {
            data: Some(data),
            msg: "ok".to_string(),
            code: 200,
        }
    }

    pub fn error(msg: String) -> Self {
        ResponseBody::<T> {
            data: None,
            msg,
            code: 409,
        }
    }
}
