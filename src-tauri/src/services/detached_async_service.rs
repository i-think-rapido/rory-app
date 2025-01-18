use tokio::runtime::Runtime;

pub fn create_runtime() -> Runtime {
    Runtime::new().expect("Failed to create Tokio runtime")
}

pub struct DetachedAsyncRunner {
    rt: Runtime,
    runner: tokio::task::JoinHandle<()>,
}

impl DetachedAsyncRunner {
    fn run<F>(function: F) -> Self
    where
        F: FnOnce() -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> + Send + 'static,
    {
        let rt = create_runtime();
    
        let runner = rt.spawn(function());

        Self {
            rt,
            runner,
        }
    }
}

