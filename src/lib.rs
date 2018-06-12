extern crate rocket;
extern crate line_messaging_api_rust as line;

pub use line::*;

pub mod rocket_line;

#[cfg(test)]
mod tests {
    use super::rocket_line::models;

    #[test]
    fn it_works() {
        let _sig = models::Signature { key: String::from("test") };
        let body = models::Body{ data: String::from("hi") };

        println!("{}", body.data);
    }
}