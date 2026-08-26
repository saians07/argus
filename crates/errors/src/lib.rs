use serde::Deserialize;
use std::borrow::Cow;
use thiserror::Error;

#[derive(Deserialize, Error, Debug)]
pub enum ArgusErr {
    #[error("Operation failed: {operation}")]
    #[serde(skip)]
    Operation {
        operation: Cow<'static, str>,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("No sufficient privilege to access the resources.")]
    UNAUTHENTICATED,

    #[error("This resource is not available for now.")]
    FORBIDDEN,

    #[error("Invalid request.")]
    BADREQUEST,
}

impl ArgusErr {
    pub fn operation<S, E>(operation: S, source: E) -> Self
    where
        S: Into<Cow<'static, str>>,
        E: std::error::Error + Sync + Send + 'static,
    {
        ArgusErr::Operation {
            operation: operation.into(),
            source: Some(Box::new(source)),
        }
    }

    pub fn operation_ori<S: Into<Cow<'static, str>>>(operation: S) -> Self {
        ArgusErr::Operation {
            operation: operation.into(),
            source: None,
        }
    }

    pub fn error_code(&self) -> i32 {
        match self {
            ArgusErr::FORBIDDEN => 403,
            ArgusErr::UNAUTHENTICATED => 401,
            ArgusErr::BADREQUEST => 400,
            ArgusErr::Operation {
                operation: _,
                source: _,
            } => 501,
        }
    }
}
