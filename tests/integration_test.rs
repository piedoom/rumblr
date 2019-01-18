#[cfg(test)]
mod tests {
    use mockito;
    use rumblr::client::Client;

    #[test]
    fn get_user() {
        let result = Client::user();
        assert_eq!(result, "staff");
    }
}
