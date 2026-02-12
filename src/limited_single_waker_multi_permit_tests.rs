
use crate::LimitedSingleWakerMultiPermit;

#[test]
fn test_basics()
{

    let wpc = LimitedSingleWakerMultiPermit::new(2);

    assert_eq!(wpc.avalible_permits(), Some(0));

    //Adding Permits

    assert!(wpc.add_permit() == Some(true));

    assert_eq!(wpc.avalible_permits(), Some(1));

    assert!(wpc.add_permit() == Some(true));

    assert_eq!(wpc.avalible_permits(), Some(2));

    //Removing Permits

    assert!(wpc.remove_permit() == Some(true));

    assert_eq!(wpc.avalible_permits(), Some(1));

    assert!(wpc.remove_permit() == Some(true));

    assert_eq!(wpc.avalible_permits(), Some(0));

    //Closing the LimitedSingleWakerMultiPermit

    wpc.close();

    assert!(wpc.is_closed());

    assert_eq!(wpc.avalible_permits(), None);

}

#[cfg(feature="tokio")]
mod tokio_tests
{

    use std::{sync::Arc, time::Duration};

    use futures::join;
    
    use tokio::{task::JoinSet, time::{sleep, timeout}};

    use crate::{LimitedSingleWakerMultiPermit, LimitedSingleWakerMultiPermitError, WakerPermitQueueClosedError};

    #[tokio::test(flavor = "multi_thread", worker_threads = 3)]
    async fn test_1()
    {

        let wpc = LimitedSingleWakerMultiPermit::new(2); //::with_capacity(2);

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

        assert_eq!(arc_wpc.add_permit(), Some(true));

        assert_eq!(arc_wpc.add_permit(), Some(true));

        let join_result = join!(t1, t2);

        if let (Err(_err1), Err(_err2)) = join_result
        {
        
            assert!(false)

        }

        //

        assert_eq!(arc_wpc.avalible_permits(), Some(0));

        assert_eq!(arc_wpc.has_max_permits(), Some(false));

        // Add the permits back

        assert_eq!(arc_wpc.add_permit(), Some(true));

        assert_eq!(arc_wpc.add_permit(), Some(true));

        //

        assert_eq!(arc_wpc.avalible_permits(), Some(2));

        assert_eq!(arc_wpc.has_max_permits(), Some(true));

        //Incrementing

        let arc_wpc_task = arc_wpc.clone();

        let t3 = tokio::spawn(async move
        {

            let ft = arc_wpc_task.increment_permits_or_wait();

            let to_ft = timeout(Duration::from_secs(10), ft);

            assert_eq!(to_ft.await, Ok(Ok(())));

        });

        let arc_wpc_task = arc_wpc.clone();

        let t4 = tokio::spawn(async move
        {

            let ft = arc_wpc_task.increment_permits_or_wait();

            let to_ft = timeout(Duration::from_secs(10), ft);

            assert_eq!(to_ft.await, Ok(Ok(())));

        });

        assert_eq!(arc_wpc.remove_permit(), Some(true));

        assert_eq!(arc_wpc.remove_permit(), Some(true));

        let join_result = join!(t3, t4);

        if let (Err(_err3), Err(_err4)) = join_result
        {
        
            assert!(false)

        }

        //

        assert_eq!(arc_wpc.avalible_permits(), Some(2));

        assert_eq!(arc_wpc.has_max_permits(), Some(true));

    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn waiting()
    {

        let wpc = LimitedSingleWakerMultiPermit::new(2);

        let arc_wpc = Arc::new(wpc);

        let arc_wpc_task = arc_wpc.clone();

        //Decrementing

        let t1 = tokio::spawn(async move
        {

            let ft = arc_wpc_task.decrement_permits_or_wait();

            let to_ft = timeout(Duration::from_secs(10), ft);

            //assert_eq!(to_ft.await, Ok(Err(LimitedSingleWakerMultiPermitError::Occupied)));

            assert_eq!(to_ft.await, Ok(Ok(())));

        });

        /*
        let arc_wpc_task = arc_wpc.clone();

        let t2 = tokio::spawn(async move
        {

            let ft = arc_wpc_task.decrement_permits_or_wait();

            let to_ft = timeout(Duration::from_secs(10), ft);

            assert_eq!(to_ft.await, Ok(Err(LimitedSingleWakerMultiPermitError::Occupied)));

            //assert_eq!(to_ft.await, Ok(Ok(())));

        });
        */

        //sleep(Duration::from_secs(1)).await;

        let ft = arc_wpc.increment_permits_or_wait();

        let to_ft = timeout(Duration::from_secs(10), ft);

        assert_eq!(to_ft.await, Ok(Ok(())));

        let ft = arc_wpc.increment_permits_or_wait();

        let to_ft = timeout(Duration::from_secs(10), ft);

        assert_eq!(to_ft.await, Ok(Ok(())));

        let join_result = join!(t1);

        if let (Err(_err1),) = join_result
        {
        
            assert!(false)

        }

        /* 
        let join_result = join!(t1, t2);

        if let (Err(_err1), Err(_err2)) = join_result
        {
        
            assert!(false)

        }
        */

        assert_eq!(arc_wpc.avalible_permits(), Some(1)); //Some(2));

        //assert_eq!(arc_wpc.avalible_permits(), Some(0));

    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn closing()
    {
        
        let wpc = LimitedSingleWakerMultiPermit::new(2);

        let arc_wpc = Arc::new(wpc);

        let arc_wpc_task = arc_wpc.clone();

        let t1 = tokio::spawn(async move
        {

            let ft = arc_wpc_task.decrement_permits_or_wait();

            let to_ft = timeout(Duration::from_mins(1), ft);

            assert_eq!(to_ft.await, Ok(Err(LimitedSingleWakerMultiPermitError::Closed)));

        });
        
        assert_eq!(arc_wpc.avalible_permits(), Some(0));

        arc_wpc.close();

        match t1.await
        {

            Ok(_) => assert!(true),
            Err(_) => assert!(false)

        }

        assert_eq!(arc_wpc.avalible_permits(), None);

        //println!("\narc_wpc: {:?}\n", arc_wpc);

    }

}