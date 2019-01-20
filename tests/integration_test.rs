#[cfg(test)]
mod tests {
    mod helper {
        use rumblr::data::User;
        use rumblr::Client;
        pub fn mock(method: &str, path: &str) -> mockito::Mock {
            mockito::mock(method, path)
                .match_header("content-type", "application/json")
                .with_header("content-type", "application/json")
                .with_body_from_file(&format!("tests/mock{}.json", path))
                .with_status(200)
                .create()
        }

        pub fn create_client<'a>() -> Client<'a> {
            Client::default().set_url(mockito::server_url())
        }

        pub fn get_user() -> User {
            let _m = mock("GET", "/user/info");
            create_client()
                .user()
                .expect("Failed to get endpoint or parse user.")
        }
    }

    mod user {
        use crate::tests::helper;
        #[test]
        fn get_user_name() {
            let _m = helper::mock("GET", "/user/info");
            let user = helper::get_user();
            assert_eq!(user.name, "name");
        }

        #[test]
        fn get_num_likes() {
            let _m = helper::mock("GET", "/user/info");
            let user = helper::get_user();
            assert_eq!(user.likes, 1000);
        }

        #[test]
        fn get_num_following() {
            let _m = helper::mock("GET", "/user/info");
            let user = helper::get_user();
            assert_eq!(user.following, 500);
        }

        #[test]
        fn get_default_post_format() {
            let _m = helper::mock("GET", "/user/info");
            let user = helper::get_user();
            assert_eq!(user.default_post_format, "html");
        }

        #[test]
        fn get_num_blocgs() {
            let _m = helper::mock("GET", "/user/info");
            let user = helper::get_user();
            assert_eq!(user.blogs.len(), 2);
        }
    }

}
