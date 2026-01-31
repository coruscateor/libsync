#[cfg(feature="tokio")]
mod tokio_tests
{

    use std::{sync::Arc, time::Duration};

    use futures::join;
    
    use tokio::time::{timeout, sleep};

    use crate::SingleWaker;

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_1()
    {

        let sw = SingleWaker::new();

        let arc_sw = Arc::new(sw);

        let arc_sw_2 = arc_sw.clone();

        assert_eq!(arc_sw.shouldve_awoken(), Some(false));

        assert!(!arc_sw.is_closed()); //Not closed

        let task = tokio::spawn(async move
        {

            arc_sw_2.wait().await

        });

        sleep(Duration::from_secs(1)).await;

        assert_eq!(arc_sw.wake(), Some(true));

        assert_eq!(task.await.unwrap(), Ok(()));

        assert_eq!(arc_sw.shouldve_awoken(), Some(true));

        assert_eq!(arc_sw.wake(), Some(false));

        arc_sw.close();

        assert!(arc_sw.is_closed());

        assert_eq!(arc_sw.shouldve_awoken(), None);

        assert_eq!(arc_sw.wake(), None);

    }

}
