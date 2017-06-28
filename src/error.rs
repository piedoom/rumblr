quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Unknown {
            description("An unknown error happened.")
        }
        Hyper {
            description("HTTP error")
        }
        Deserialize {
            description("Deserialize error")
        }
    }
}