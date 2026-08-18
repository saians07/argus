use serde::Deserialize;
use std::borrow::Cow;
use thiserror::Error;

#[derive(Deserialize, Error, Debug)]
pub enum CustomErr {
    #[error("Operation failed: {operation}")]
    #[serde(skip)]
    Operation {
        operation: Cow<'static, str>,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("No sufficient privilege to access the resources.")]
    #[serde(skip)]
    UNAUTHENTICATED,

    #[error("This resource is not available for now.")]
    #[serde(skip)]
    FORBIDDEN,
}

impl CustomErr {
    pub fn operation<S, E>(operation: S, source: E) -> Self
    where
        S: Into<Cow<'static, str>>,
        E: std::error::Error + Sync + Send + 'static,
    {
        CustomErr::Operation {
            operation: operation.into(),
            source: Some(Box::new(source)),
        }
    }

    pub fn operation_ori<S: Into<Cow<'static, str>>>(operation: S) -> Self {
        CustomErr::Operation {
            operation: operation.into(),
            source: None,
        }
    }

    pub fn error_code(&self) -> i32 {
        match self {
            CustomErr::FORBIDDEN => 403,
            CustomErr::UNAUTHENTICATED => 401,
            CustomErr::Operation {
                operation: _,
                source: _,
            } => 501,
        }
    }
}
