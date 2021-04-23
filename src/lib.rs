

#[derive(Debug, Clone)]
pub struct AssistantClient{
    url: String,
    version: String,
    api_key: String
}

impl AssistantClient {
    pub fn new()-> AssistantClient{
        AssistantClient{
            url: "".to_string(),
            version: "".to_string(),
            api_key: "".to_string()
        }
    }
    pub fn set_service_url(&mut self, url: &str){
        self.url = url.to_string();
    }
    pub fn set_api_key(&mut self, api_key: &str){
        self.api_key = api_key.to_string();
    }
    pub fn set_version(&mut self, version: &str){ self.version = version.to_string();}
}


#[cfg(test)]
mod tests {
    use crate::AssistantClient;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn new_assistant(){
        let mut client: AssistantClient = AssistantClient::new();
        let url = "";
        let version = "";
        let api_key = "";
        client.set_service_url(url);
        client.set_version(version);
        client.set_api_key(api_key);

        assert_eq!(client.url, url.to_string());
        assert_eq!(client.api_key, api_key.to_string());
        assert_eq!(client.version, version.to_string());
    }

    #[test]
    fn new_sessesion(){}
}
