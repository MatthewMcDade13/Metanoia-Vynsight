


pub mod spider {
    use std::fmt::Display;

    type Reason = String;
    #[derive(Debug, Clone)]
    pub struct SpiderFailure(pub Reason);
    
    impl Display for SpiderFailure {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
    
    impl std::error::Error for SpiderFailure {}

    #[derive(Debug, Clone)]
    pub enum CrawlErrorInfo {
        Connection(ConnectionErrorType, Reason),
        Parse(Reason)
    }

    #[derive(Debug, Clone)]
    pub struct CrawlError {
        uri: String,
        info: CrawlErrorInfo
    }

    impl CrawlError {
        pub fn at_uri(&self) -> &str { self.uri.as_str() }
        pub const fn info(&self) -> &CrawlErrorInfo { &self.info }
    }


    #[derive(Debug, Clone)]
    pub enum ConnectionErrorType {
        HTTP,
        HTTPS,
        TLS,
        FTP,
        SFTP,
        SSH,
        WebSocket,
    }

}

pub mod db {
    
    use std::fmt::Display;


    type Reason = String;

    #[derive(Debug, Clone)]
    pub struct TransactionError(pub Reason);

    impl Display for TransactionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
    
    impl std::error::Error for TransactionError {}
}
