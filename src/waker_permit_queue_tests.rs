//use super::WakerPermitQueue;

//#[cfg(test)]
mod tests
{

    use crate::WakerPermitQueue;

    #[test]
    fn test_basics()
    {

        let wpc = WakerPermitQueue::new();

        assert_eq!(wpc.avalible_permits(), Some(0));

        //Adding Permits

        assert!(wpc.add_permit() == Some(true));

        assert_eq!(wpc.avalible_permits(), Some(1));

        assert!(wpc.add_permit() == Some(true));

        assert_eq!(wpc.avalible_permits(), Some(2));

        //Removing Permits

        assert!(wpc.remove_permit());

        assert_eq!(wpc.avalible_permits(), Some(1));

        assert!(wpc.remove_permit());

        assert_eq!(wpc.avalible_permits(), Some(0));

        //Closing the WakerPermitQueue

        wpc.close();

        assert!(wpc.is_closed());

        assert_eq!(wpc.avalible_permits(), None);

    }

}

#[cfg(feature="tokio")]
mod tokio_tests
{

    use std::{sync::Arc, time::Duration};

    use futures::join;
    
    use tokio::time::timeout;

    use crate::WakerPermitQueue;

    #[tokio::test(flavor = "multi_thread", worker_threads = 3)]
    async fn test_1()
    {

        let wpc = WakerPermitQueue::with_capacity(2);

        let arc_wpc = Arc::new(wpc);

        let arc_wpc_task = arc_wpc.clone();

        //assert!(false);

        let t1 = tokio::spawn(async move
        {

            let ft = arc_wpc_task.decrement_permits_or_wait();

            let to_ft = timeout(Duration::from_secs(10), ft);

            assert_eq!(to_ft.await, Ok(Ok(())));

        });

        let arc_wpc_task = arc_wpc.clone();

        let t2 = tokio::spawn(async move
        {

            let ft = arc_wpc_task.decrement_permits_or_wait();

            let to_ft = timeout(Duration::from_secs(10), ft);

            assert_eq!(to_ft.await, Ok(Ok(())));

        });


        assert!(arc_wpc.add_permit() == Some(true));

        assert!(arc_wpc.add_permit() == Some(true));

        let join_result = join!(t1, t2);

        if let (Err(_err1), Err(_err2)) = join_result
        {
        
            assert!(false)

        }

        //assert_eq!(join_result, (Ok(()), Ok(())))

    }

}