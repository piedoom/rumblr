#[cfg(test)]
mod tests {
    use rumblr::Client;

    fn create_client<'a>() -> Client<'a> {
        Client::default().set_url(mockito::server_url())
    }

    fn mock(method: &str, path: &str) -> mockito::Mock {
        mockito::mock(method, path)
            .match_header("content-type", "application/json")
            .with_header("content-type", "application/json")
            .with_body_from_file(&format!("tests/mock{}.json", path))
            .with_status(200)
            .create()
    }

    #[test]
    fn get_user() {
        let _m = mock("GET", "/user/info");
        let result = create_client()
            .user()
            .expect("Failed to get endpoint or parse user.");
        assert_eq!(result.name, "name");
    }

}
