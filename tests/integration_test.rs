#[cfg(test)]
mod tests {
    use mockito::mock;
    use rumblr::Client;

    fn create_client<'a>() -> Client<'a> {
        Client::default().set_url(mockito::server_url())
    }

    #[test]
    fn get_user() {
        let _m = mock("GET", "/user/info")
            .match_header("content-type", "application/json")
            .with_header("content-type", "application/json")
            .with_body_from_file("tests/mock/user/info.json")
            .with_status(200)
            .create();

        let result = create_client()
            .user()
            .expect("Failed to get endpoint or parse user.");
        assert_eq!(result.name, "name");
    }
}
