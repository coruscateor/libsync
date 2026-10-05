use crate::LimitedWakerPermitQueue;

#[test]
fn test_basics()
{

    let wpc = LimitedWakerPermitQueue::new(2);

    assert_eq!(wpc.avalible_permits(), Some(0));

    assert_eq!(wpc.has_max_permits(), Some(false));


    //Adding Permits

    assert!(wpc.add_permit() == Some(true));

    assert_eq!(wpc.avalible_permits(), Some(1));

    assert!(wpc.add_permit() == Some(true));

    //

    assert_eq!(wpc.avalible_permits(), Some(2));

    assert_eq!(wpc.has_max_permits(), Some(true));

    //Removing Permits

    assert!(wpc.remove_permit() == Some(true));

    assert_eq!(wpc.avalible_permits(), Some(1));

    assert!(wpc.remove_permit() == Some(true));

    //

    assert_eq!(wpc.avalible_permits(), Some(0));

    assert_eq!(wpc.has_max_permits(), Some(false));

    //Closing the LimitedWakerPermitQueue

    wpc.close();

    assert!(wpc.is_closed());

    assert_eq!(wpc.avalible_permits(), None);

}

#[cfg(feature="tokio")]
mod tokio_tests
{

    use std::{sync::Arc, time::Duration};

    use futures::join;
    
    use tokio::select;

    use tokio::task::{JoinError, JoinHandle, JoinSet};

    use tokio::time::{timeout, sleep};

    use crate::LimitedWakerPermitQueue; //{FuturesPoller,

    #[tokio::test(flavor = "multi_thread", worker_threads = 3)]
    async fn test_1()
    {

        let wpc = LimitedWakerPermitQueue::with_capacity(2, 2);

        let arc_wpc = Arc::new(wpc);

        let arc_wpc_task = arc_wpc.clone();

        //Decrementing

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

    #[tokio::test(flavor = "multi_thread", worker_threads = 3)]
    async fn waiting()
    {

        let wpc = LimitedWakerPermitQueue::with_capacity(2, 2);

        let arc_wpc = Arc::new(wpc);

        let arc_wpc_task = arc_wpc.clone();

        //Decrementing

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

        sleep(Duration::from_secs(1)).await;

        let ft = arc_wpc.increment_permits_or_wait();

        let to_ft = timeout(Duration::from_secs(10), ft);

        assert_eq!(to_ft.await, Ok(Ok(())));

        let ft = arc_wpc.increment_permits_or_wait();

        let to_ft = timeout(Duration::from_secs(10), ft);

        assert_eq!(to_ft.await, Ok(Ok(())));

        let join_result = join!(t1, t2);

        if let (Err(_err1), Err(_err2)) = join_result
        {
        
            assert!(false)

        }

        assert_eq!(arc_wpc.avalible_permits(), Some(0));

        /*
        select!
        {

            _ = t1 =>
            {

                t2.await;

            },
            _ = t2 =>
            {


            }

        }
        */

    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn cancellation()
    {
        
        let wpc = LimitedWakerPermitQueue::with_capacity(2, 2);

        let arc_wpc = Arc::new(wpc);

        let arc_wpc_task = arc_wpc.clone();

        assert_eq!(arc_wpc.active_ids_len(), Some(0));

        let incrementing_task;

        //let mut t1_cancelled = false;

        //let mut t2_cancelled = false;

        let res;

        {

            let mut js = JoinSet::new();

            //Decrementing

            let _t1 = js.spawn(async move
            {

                let ft = arc_wpc_task.decrement_permits_or_wait();

                //println!("\nt1 decrement_permits_or_wait: {:?}\n", ft);

                let to_ft = timeout(Duration::from_mins(5), ft); //from_secs(30), ft);

                assert_eq!(to_ft.await, Ok(Ok(())));

            });

            let arc_wpc_task = arc_wpc.clone();

            let _t2 = js.spawn(async move
            {

                let ft = arc_wpc_task.decrement_permits_or_wait();

                //println!("\nt2 decrement_permits_or_wait: {:?}\n", ft);

                let to_ft = timeout(Duration::from_mins(5), ft); //from_secs(30), ft);

                assert_eq!(to_ft.await, Ok(Ok(())));

            });

            //sleep(Duration::from_secs(1)).await;

            println!("\nactive_ids length {:?}\n", arc_wpc.active_ids_len());

            let arc_wpc_task = arc_wpc.clone();

            incrementing_task = tokio::spawn(async move
            {

                let ft = arc_wpc_task.increment_permits_or_wait();

                let to_ft = timeout(Duration::from_mins(5), ft); //from_secs(10), ft);

                assert_eq!(to_ft.await, Ok(Ok(())));

                /*
                let ft = arc_wpc_task.increment_permits_or_wait();

                let to_ft = timeout(Duration::from_secs(10), ft);

                assert_eq!(to_ft.await, Ok(Ok(())));
                */

            });

            res = js.join_next().await.unwrap();

        }

        //let res = cancellation_select(t1, t2, &mut t1_cancelled, &mut t2_cancelled).await;

        /*
        let res: Result<(), JoinError>;

        select!
        {

            res1 = t1 =>
            {

                t2_cancelled = true;

                res = res1;

            },
            res2 = t2 =>
            {

                t1_cancelled = true;

                res = res2;

            }

        }
        */

        //println!("\ntest {:?}\n", t2);

        //let _ = join!(t1, t2);

        match incrementing_task.await
        {

            Ok(_) => assert!(true),
            Err(_) => assert!(false)

        }

        sleep(Duration::from_secs(1)).await;

        assert_eq!(arc_wpc.avalible_permits(), Some(0)); //Some(1));

        println!("\nactive_ids length {:?}\n", arc_wpc.active_ids_len());

        //println!("\nt1_cancelled {}\n", t1_cancelled);

        //println!("t2_cancelled {}\n", t2_cancelled);

        match res
        {

            Ok(_) => assert!(true),
            Err(_) => assert!(false)

        }

        println!("\narc_wpc: {:?}\n", arc_wpc);

        sleep(Duration::from_secs(5)).await;

        assert_eq!(arc_wpc.active_ids_len(), Some(0));

    }

    //Disabled

    /*
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn futures_poller_cancellation()
    {
        
        let wpc = LimitedWakerPermitQueue::with_capacity(2, 2);

        let arc_wpc = Arc::new(wpc);

        let arc_wpc_task = arc_wpc.clone();

        assert_eq!(arc_wpc.active_ids_len(), Some(0));

        //Decrementing

        let t1 = tokio::spawn(async move
        {

            let ft = arc_wpc_task.decrement_permits_or_wait();

            //println!("\nt1 decrement_permits_or_wait: {:?}\n", ft);

            let to_ft = timeout(Duration::from_mins(5), ft); //from_secs(30), ft);

            assert_eq!(to_ft.await, Ok(Ok(())));

        });

        let arc_wpc_task = arc_wpc.clone();

        let t2 = tokio::spawn(async move
        {

            let ft = arc_wpc_task.decrement_permits_or_wait();

            //println!("\nt2 decrement_permits_or_wait: {:?}\n", ft);

            let to_ft = timeout(Duration::from_mins(5), ft); //from_secs(30), ft);

            assert_eq!(to_ft.await, Ok(Ok(())));

        });

        sleep(Duration::from_secs(1)).await;

        println!("\nactive_ids length {:?}\n", arc_wpc.active_ids_len());

        let mut t1_cancelled = false;

        let mut t2_cancelled = false;

        let arc_wpc_task = arc_wpc.clone();

        let incrementing = tokio::spawn(async move
        {

            let ft = arc_wpc_task.increment_permits_or_wait();

            let to_ft = timeout(Duration::from_mins(5), ft); //from_secs(10), ft);

            assert_eq!(to_ft.await, Ok(Ok(())));

            /*
            let ft = arc_wpc_task.increment_permits_or_wait();

            let to_ft = timeout(Duration::from_secs(10), ft);

            assert_eq!(to_ft.await, Ok(Ok(())));
            */

        });

        /*
        let fp = FuturesPoller::new(t1, async |res| { t2_cancelled = true; }, t2, async |res| { t1_cancelled = true; });

        loop
        {

            if let Some(val) = fp.await
            {

                println!("FuturesPoller result: {}", val);

                break;

            }
            else
            {

                tokio::task::yield_now().await;
                
            }

        }
        */

        //let res = cancellation_select(t1, t2, &mut t1_cancelled, &mut t2_cancelled).await;

        /*
        let res: Result<(), JoinError>;

        select!
        {

            res1 = t1 =>
            {

                t2_cancelled = true;

                res = res1;

            },
            res2 = t2 =>
            {

                t1_cancelled = true;

                res = res2;

            }

        }
        */

        //println!("\ntest {:?}\n", t2);

        //let _ = join!(t1, t2);

        match incrementing.await
        {

            Ok(_) => assert!(true),
            Err(_) => assert!(false)

        }

        sleep(Duration::from_secs(1)).await;

        assert_eq!(arc_wpc.avalible_permits(), Some(0)); //Some(1));

        println!("\nactive_ids length {:?}\n", arc_wpc.active_ids_len());

        println!("\nt1_cancelled {}\n", t1_cancelled);

        println!("t2_cancelled {}\n", t2_cancelled);

        match res
        {

            Ok(_) => assert!(true),
            Err(_) => assert!(false)

        }

        println!("\narc_wpc: {:?}\n", arc_wpc);

        sleep(Duration::from_secs(5)).await;

        assert_eq!(arc_wpc.active_ids_len(), Some(0));

    }
    */

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn wrong_cancellation()
    {
        
        let wpc = LimitedWakerPermitQueue::with_capacity(2, 2);

        let arc_wpc = Arc::new(wpc);

        let arc_wpc_task = arc_wpc.clone();

        assert_eq!(arc_wpc.active_ids_len(), Some(0));

        //Decrementing

        let t1 = tokio::spawn(async move
        {

            let ft = arc_wpc_task.decrement_permits_or_wait();

            //println!("\nt1 decrement_permits_or_wait: {:?}\n", ft);

            let to_ft = timeout(Duration::from_mins(5), ft); //from_secs(30), ft);

            assert_eq!(to_ft.await, Ok(Ok(())));

        });

        let arc_wpc_task = arc_wpc.clone();

        let t2 = tokio::spawn(async move
        {

            let ft = arc_wpc_task.decrement_permits_or_wait();

            //println!("\nt2 decrement_permits_or_wait: {:?}\n", ft);

            let to_ft = timeout(Duration::from_mins(5), ft); //from_secs(30), ft);

            assert_eq!(to_ft.await, Ok(Ok(())));

        });

        sleep(Duration::from_secs(1)).await;

        println!("\nactive_ids length {:?}\n", arc_wpc.active_ids_len());

        let mut t1_cancelled = false;

        let mut t2_cancelled = false;

        let arc_wpc_task = arc_wpc.clone();

        let incrementing = tokio::spawn(async move
        {

            let ft = arc_wpc_task.increment_permits_or_wait();

            let to_ft = timeout(Duration::from_mins(5), ft); //from_secs(10), ft);

            assert_eq!(to_ft.await, Ok(Ok(())));

            /*
            let ft = arc_wpc_task.increment_permits_or_wait();

            let to_ft = timeout(Duration::from_secs(10), ft);

            assert_eq!(to_ft.await, Ok(Ok(())));
            */

        });

        let res = wrong_cancellation_select(t1, t2, &mut t1_cancelled, &mut t2_cancelled).await;

        /*
        let res: Result<(), JoinError>;

        select!
        {

            res1 = t1 =>
            {

                t2_cancelled = true;

                res = res1;

            },
            res2 = t2 =>
            {

                t1_cancelled = true;

                res = res2;

            }

        }
        */

        //println!("\ntest {:?}\n", t2);

        //let _ = join!(t1, t2);

        match incrementing.await
        {

            Ok(_) => assert!(true),
            Err(_) => assert!(false)

        }

        sleep(Duration::from_secs(1)).await;

        assert_eq!(arc_wpc.avalible_permits(), Some(0)); //Some(1));

        println!("\nactive_ids length {:?}\n", arc_wpc.active_ids_len());

        println!("\nt1_cancelled {}\n", t1_cancelled);

        println!("t2_cancelled {}\n", t2_cancelled);

        match res
        {

            Ok(_) => assert!(true),
            Err(_) => assert!(false)

        }

        println!("\narc_wpc: {:?}\n", arc_wpc);

        sleep(Duration::from_secs(5)).await;

        assert_eq!(arc_wpc.active_ids_len(), Some(0));

    }

    async fn wrong_cancellation_select(t1: JoinHandle<()>, t2: JoinHandle<()>, t1_cancelled: &mut bool, t2_cancelled: &mut bool) -> Result<(), JoinError>
    {

        let res: Result<(), JoinError>;

        /*
        select!
        {

            res1 = t1 =>
            {

                *t2_cancelled = true;

                res = res1;

            },
            res2 = t2 =>
            {

                *t1_cancelled = true;

                res = res2;

            }

        }
        */

        select!
        {

            res1 = t1 =>
            {

                *t2_cancelled = true;

                res = res1;

            },
            res2 = t2 =>
            {

                *t1_cancelled = true;

                res = res2;

            }

        }

        res

    }

}

