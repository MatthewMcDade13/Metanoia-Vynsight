


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
