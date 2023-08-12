use crate::{
    nextcloud::{Credentials, JobType},
    Config, Nextcloud,
};

use threadpool::ThreadPool;

pub struct Client {
    service: Nextcloud,
    thread_pool: ThreadPool,
}

impl Client {
    pub fn new(credentials: Option<Credentials>, config: Config) -> Self {
        Client {
            service: Nextcloud::new(credentials, config.clone()),
            thread_pool: ThreadPool::new(4),
        }
    }

    pub fn queue(&self, job_type: JobType, file: String, dest: Option<String>) {
        let mut worker = self.service.clone();
        self.thread_pool
            .execute(move || worker.work(job_type, file, dest).unwrap());
    }

    pub fn upload_file(&mut self, file: String) {
        self.queue(JobType::Upload, file, None);
    }

    pub fn upload_files(&mut self, files: Vec<String>) {
        for file in files {
            self.queue(JobType::Upload, file, None);
        }
    }

    pub fn move_file(&mut self, path: String, dest: String) {
        self.queue(JobType::Move, path, Some(dest));
    }

    pub fn delete_file(&mut self, path: String) {
        self.queue(JobType::Delete, path, None);
    }
}
