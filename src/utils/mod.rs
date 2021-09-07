use diesel::result::Error;
use rocket::serde::Serialize;

/// 返回值包
pub struct ResBundle(i32, &'static str);

/// 目前定义了两个返回值常量
/// - SUCCESS 请求处理成功
/// - ERROR 请求处理失败
impl ResBundle {
    pub const SUCCESS: Self = Self(10000, "success");
    pub const ERROR: Self = Self(10001, "error");

    pub fn get_code(&self) -> i32 {
        self.0
    }

    pub fn get_message(&self) -> &'static str {
        self.1
    }
}

/// 通用返回值
#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CommonRespose<T> {
    code: i32,
    message: String,
    data: Option<T>,
}

impl<T> CommonRespose<T> {
    pub fn build(result_response: Result<T, Error>) -> CommonRespose<T> {
        match db_response {
            Ok(t) => CommonRespose {
                code: ResBundle::SUCCESS.get_code(),
                message: ResBundle::SUCCESS.get_message().to_string(),
                data: Some(t),
            },
            Err(e) => CommonRespose {
                code: ResBundle::ERROR.get_code(),
                message: ResBundle::ERROR.get_message().to_string(),
                data: None,
            },
        }
    }
}

/// 通用分页返回
#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CommonPageRespose<T> {
    code: i32,
    message: String,
    data: Option<T>,
    total: Option<i32>,
    page: Option<i32>,
    limit: Option<i32>,
}

impl<T> CommonPageRespose<T> {
    pub fn build_page(
        result_response: Result<T, Error>,
        total: Option<i32>,
        page: Option<i32>,
        limit: Option<i32>,
    ) -> CommonPageRespose<T> {
        match db_response {
            Ok(t) => CommonRespose {
                code: ResBundle::SUCCESS.get_code(),
                message: ResBundle::SUCCESS.get_message().to_string(),
                data: Some(t),
            },
            Err(e) => CommonRespose {
                code: ResBundle::ERROR.get_code(),
                message: ResBundle::ERROR.get_message().to_string(),
                data: None,
            },
        }
    }
}
