pub mod thread_pool {
    use std::time::Duration;
    use std::thread;

    pub trait ThreadPool {
        fn execute<F>(&self, runnable: F)
            where F: Fn() -> Unit;

        fn stop_with_timeout(timeout: Duration) -> bool;

        fn stop();
    }

    pub struct SimpleThreadPool {
        threads: Vec<thread>
    }



    impl ThreadPool for SimpleThreadPool {
        fn execute<F>(&self, runnable: F) where F: Fn() -> Unit {
            unimplemented!()
        }

        fn stop_with_timeout(timeout: Duration) -> bool {
            unimplemented!()
        }

        fn stop() {
            unimplemented!()
        }
    }
}
