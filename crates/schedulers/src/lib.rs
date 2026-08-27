use std::error::Error;

use tokio_cron_scheduler::{Job, JobScheduler};

pub struct Supervisor {
    pub scheduler: JobScheduler,
}

impl Supervisor {
    pub async fn new() -> Self {
        Self {
            scheduler: JobScheduler::new().await.unwrap(),
        }
    }

    pub async fn get_schedule<T: Worker + Send + Sync + Clone + 'static>(
        &self,
        worker: &T,
    ) -> Box<str> {
        worker.get_schedule().await
    }

    pub async fn activate_worker<T: Worker + Send + Sync + Clone + 'static>(
        &self,
        worker: T,
    ) -> Result<(), DynError> {
        let schedule = self.get_schedule(&worker).await;
        let job = Job::new_async(schedule, move |_uuid, _l| {
            let worker_c = worker.clone();
            Box::pin(async move { worker_c.execute().await.unwrap() })
        })
        .unwrap();
        self.scheduler.add(job).await.unwrap();

        Ok(())
    }

    pub async fn start(&self) -> () {
        self.scheduler.start().await.unwrap();
    }
}

type DynError = Box<dyn Error + Send + Sync>;

#[async_trait::async_trait]
pub trait Worker {
    async fn execute(&self) -> Result<(), DynError>;
    async fn get_schedule(&self) -> Box<str>;
}
