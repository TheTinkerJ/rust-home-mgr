use diesel::result::Error;
use rocket::serde::Serialize;

pub struct ResBundle(i32, &'static str);

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

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CommonRespose<T> {
    code: i32,
    message: String,
    data: Option<T>,
}

impl<T> CommonRespose<T> {
    pub fn build(result_response: Result<T, Error>) -> CommonRespose<T> {
        CommonRespose {
            code: ResBundle::SUCCESS.get_code(),
            message: ResBundle::SUCCESS.get_message().to_string(),
            data: result_response.ok(),
        }
    }
}

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

// todo Result to Option?
impl<T> CommonPageRespose<T> {
    pub fn build_page(
        result_response: Result<T, Error>,
        total: Option<i32>,
        page: Option<i32>,
        limit: Option<i32>,
    ) -> CommonPageRespose<T> {
        CommonPageRespose {
            code: ResBundle::SUCCESS.get_code(),
            message: ResBundle::SUCCESS.get_message().to_string(),
            data: result_response.ok(),
            total,
            page,
            limit,
        }
    }
}
