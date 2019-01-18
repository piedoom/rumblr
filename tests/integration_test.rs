#[cfg(test)]
mod tests {
    use mockito::mock;
    use rumblr::Client;

    #[test]
    fn get_user() {
        let _m = mock("GET", "/user/info")
            .match_header("content-type", "application/json")
            .with_header("content-type", "application/json")
            .with_body_from_file("tests/mock/user.json")
            .with_status(200)
            .create();

        let result = Client::default().user();
        dbg!(result);
        //assert_eq!(result.name, "name");
    }
}
