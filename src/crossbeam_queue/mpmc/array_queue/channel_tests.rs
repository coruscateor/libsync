
#[cfg(feature="tokio")]
mod tokio_tests
{

    use crate::{BoundedSendError, ReceiveError};

    use inc_dec::IncDecSelf;
    use tokio::{sync::{Notify, Semaphore}, task::JoinSet};

    use std::sync::Arc;

    use super::super::{channel, Sender, Receiver};

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn basic_sender_oriented()
    {

        let (sender, receiver) = channel(2);

        assert_eq!(sender.capacity(), 2);

        assert_eq!(receiver.capacity(), 2);

        let task = tokio::spawn(async move {

            let result = receiver.recv().await;

            assert_eq!(result, Ok(1));

            let result = receiver.recv().await;

            assert_eq!(result, Ok(2));

        });

        assert!(!sender.is_closed());

        let result = sender.send(1).await;

        assert!(result.is_ok());

        let result = sender.send(2).await;

        assert!(result.is_ok());

        //Make sure the task is finished

        assert!(task.await.is_ok());

        let result = sender.send(3).await;

        assert_eq!(result, Err(3));

        assert!(sender.is_closed());

    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn basic_receiver_oriented()
    {

        let (sender, receiver) = channel(2);

        assert_eq!(receiver.capacity(), 2);

        assert_eq!(sender.capacity(), 2);

        assert_eq!(receiver.head_room(), 2);

        assert!(!receiver.is_full());

        assert!(receiver.is_empty());

        let task = tokio::spawn(async move {

            assert!(!sender.is_closed());

            assert_eq!(sender.head_room(), 2);

            let result = sender.send(1).await;

            assert!(result.is_ok());

            let result = sender.send(2).await;

            assert!(result.is_ok());

        });

        assert!(!receiver.is_closed());

        let result = receiver.recv().await;

        assert_eq!(result, Ok(1));

        let result = receiver.recv().await;

        assert_eq!(result, Ok(2));

        //Make sure the task is finished

        assert!(task.await.is_ok());

        let result = receiver.recv().await;

        assert_eq!(result, Err(()));

        assert!(receiver.is_closed());

    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn is_full_sender_oriented()
    {

        //

        let (sender, receiver) = channel(2);

        assert_eq!(sender.capacity(), 2);

        assert_eq!(receiver.capacity(), 2);

        assert_eq!(sender.head_room(), 2);

        assert!(!sender.is_full());

        assert!(sender.is_empty());

        let sender_notify = Notify::new();

        let arc_sender_notify = Arc::new(sender_notify);

        let arc_sender_notify_task = arc_sender_notify.clone();

        let receiver_notify = Notify::new();

        let arc_receiver_notify = Arc::new(receiver_notify);

        let arc_receiver_notify_task = arc_receiver_notify.clone();

        //

        let task = tokio::spawn(async move
        {

            assert!(!receiver.is_closed());

            let result = receiver.recv().await;

            assert_eq!(result, Ok(1));

            let result = receiver.recv().await;

            assert_eq!(result, Ok(2));

            //assert!(receiver.is_full());

            arc_sender_notify_task.notify_one();

            let result = receiver.recv().await;

            assert_eq!(result, Ok(3));

            let result = receiver.recv().await;

            assert_eq!(result, Ok(4));

            arc_receiver_notify_task.notified().await;

        });

        assert!(!sender.is_closed());

        assert_eq!(sender.head_room(), 2);

        let result = sender.send(1).await;

        assert!(result.is_ok());

        let result = sender.send(2).await;

        assert!(result.is_ok());

        arc_sender_notify.notified().await;

        let result = sender.try_send(3);

        assert!(result.is_ok());

        let result = sender.try_send(4);

        assert!(result.is_ok());

        //?

        /*

        let result = sender.try_send(5);

        assert_eq!(result, Err(BoundedSendError::Full(5)));

        assert!(sender.is_full());

        assert_eq!(sender.head_room(), 0);

        */

        arc_receiver_notify.notify_one();

        //Make sure the task is finished

        assert!(task.await.is_ok());

        let result = sender.try_send(6);

        assert_eq!(result, Err(BoundedSendError::Closed(6)));

        let result = sender.send(7).await;

        assert_eq!(result, Err(7));

        assert!(sender.is_closed());

    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn is_full_receiver_oriented()
    {

        let (sender, receiver) = channel(2);

        assert_eq!(receiver.capacity(), 2);

        assert_eq!(sender.capacity(), 2);

        assert_eq!(receiver.head_room(), 2);

        assert!(!receiver.is_full());

        assert!(receiver.is_empty());

        let receiver_notify = Notify::new();

        let arc_receiver_notify = Arc::new(receiver_notify);

        let arc_receiver_notify_task = arc_receiver_notify.clone();

        let sender_notify = Notify::new();

        let arc_sender_notify = Arc::new(sender_notify);

        let arc_sender_notify_task = arc_sender_notify.clone();

        let task = tokio::spawn(async move {

            assert!(!sender.is_closed());

            assert_eq!(sender.head_room(), 2);

            let result = sender.send(1).await;

            assert!(result.is_ok());

            let result = sender.send(2).await;

            assert!(result.is_ok());

            let result = sender.send(3).await;

            assert!(result.is_ok());

            let result = sender.send(4).await;

            assert!(result.is_ok());

            assert!(sender.is_full());

            arc_receiver_notify_task.notify_one();

            arc_sender_notify_task.notified().await;

        });

        assert!(!receiver.is_closed());

        let result = receiver.recv().await;

        assert_eq!(result, Ok(1));

        let result = receiver.recv().await;

        assert_eq!(result, Ok(2));

        arc_receiver_notify.notified().await;

        assert!(receiver.is_full());

        let result = receiver.try_recv();

        assert_eq!(result, Ok(3));

        let result = receiver.try_recv();

        assert_eq!(result, Ok(4));

        let result = receiver.try_recv();

        assert_eq!(result, Err(ReceiveError::Empty));

        arc_sender_notify.notify_one();

        //Make sure the task is finished

        assert!(task.await.is_ok());

        let result = receiver.try_recv();

        assert_eq!(result, Err(ReceiveError::Closed));

        let result = receiver.recv().await;

        assert_eq!(result, Err(()));

        assert!(receiver.is_closed());

    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 9)]
    async fn _8_senders_8_recivers()
    {

        let (sender1, receiver1) = channel(16);

        let mut js = JoinSet::new();

        let semaphore = Semaphore::new(0);

        let arc_semaphore = Arc::new(semaphore);

        //

        let sender2 = sender1.clone();

        let sender3 = sender1.clone();

        let sender4 = sender1.clone();

        let sender5 = sender1.clone();

        let sender6 = sender1.clone();

        let sender7 = sender1.clone();

        let sender8 = sender1.clone();

        //

        let receiver2 = receiver1.clone();

        let receiver3 = receiver1.clone();

        let receiver4 = receiver1.clone();

        let receiver5 = receiver1.clone();

        let receiver6 = receiver1.clone();

        let receiver7 = receiver1.clone();

        let receiver8 = receiver1.clone();

        //

        let arc_semaphore_task = arc_semaphore.clone();

        js.spawn(async move
        {

            let permit = arc_semaphore_task.acquire().await.unwrap();

            assert_eq!(sender1.send(1).await, Ok(()));

            permit.forget();

        });

        let arc_semaphore_task = arc_semaphore.clone();

        js.spawn(async move
        {

            let permit = arc_semaphore_task.acquire().await.unwrap();

            assert_eq!(sender2.send(2).await, Ok(()));

            permit.forget();

        });

        let arc_semaphore_task = arc_semaphore.clone();

        js.spawn(async move
        {

            let permit = arc_semaphore_task.acquire().await.unwrap();

            assert_eq!(sender3.send(3).await, Ok(()));

            permit.forget();

        });

        let arc_semaphore_task = arc_semaphore.clone();

        js.spawn(async move
        {

            let permit = arc_semaphore_task.acquire().await.unwrap();

            assert_eq!(sender4.send(4).await, Ok(()));

            permit.forget();

        });

        let arc_semaphore_task = arc_semaphore.clone();

        js.spawn(async move
        {

            let permit = arc_semaphore_task.acquire().await.unwrap();

            assert_eq!(sender5.send(5).await, Ok(()));

            permit.forget();

        });

        let arc_semaphore_task = arc_semaphore.clone();

        js.spawn(async move
        {

            let permit = arc_semaphore_task.acquire().await.unwrap();

            assert_eq!(sender6.send(6).await, Ok(()));

            permit.forget();

        });

        let arc_semaphore_task = arc_semaphore.clone();

        js.spawn(async move
        {

            let permit = arc_semaphore_task.acquire().await.unwrap();

            assert_eq!(sender7.send(7).await, Ok(()));

            permit.forget();

        });

        let arc_semaphore_task = arc_semaphore.clone();

        js.spawn(async move
        {

            let permit = arc_semaphore_task.acquire().await.unwrap();

            assert_eq!(sender8.send(8).await, Ok(()));

            permit.forget();

        });

        //

        let arc_semaphore_task = arc_semaphore.clone();

        js.spawn(async move
        {

            let permit = arc_semaphore_task.acquire().await.unwrap();

            let result = receiver1.recv().await;

            assert!(result.is_ok());

            println!("Result 1: {:?}", result);

            permit.forget();

        });

        let arc_semaphore_task = arc_semaphore.clone();

        js.spawn(async move
        {

            let permit = arc_semaphore_task.acquire().await.unwrap();

            let result = receiver2.recv().await;

            assert!(result.is_ok());

            println!("Result 2: {:?}", result);

            permit.forget();

        });

        let arc_semaphore_task = arc_semaphore.clone();

        js.spawn(async move
        {

            let permit = arc_semaphore_task.acquire().await.unwrap();

            let result = receiver3.recv().await;

            assert!(result.is_ok());

            println!("Result 3: {:?}", result);

            permit.forget();

        });

        let arc_semaphore_task = arc_semaphore.clone();

        js.spawn(async move
        {

            let permit = arc_semaphore_task.acquire().await.unwrap();

            let result = receiver4.recv().await;

            assert!(result.is_ok());

            println!("Result 4: {:?}", result);

            permit.forget();

        });

        let arc_semaphore_task = arc_semaphore.clone();

        js.spawn(async move
        {

            let permit = arc_semaphore_task.acquire().await.unwrap();

            let result = receiver5.recv().await;

            assert!(result.is_ok());

            println!("Result 5: {:?}", result);

            permit.forget();

        });

        let arc_semaphore_task = arc_semaphore.clone();

        js.spawn(async move
        {

            let permit = arc_semaphore_task.acquire().await.unwrap();

            let result = receiver6.recv().await;

            assert!(result.is_ok());

            println!("Result 6: {:?}", result);

            permit.forget();

        });

        let arc_semaphore_task = arc_semaphore.clone();

        js.spawn(async move
        {

            let permit = arc_semaphore_task.acquire().await.unwrap();

            let result = receiver7.recv().await;

            assert!(result.is_ok());

            println!("Result 7: {:?}", result);

            permit.forget();

        });

        let arc_semaphore_task = arc_semaphore.clone();

        js.spawn(async move
        {

            let permit = arc_semaphore_task.acquire().await.unwrap();

            let result = receiver8.recv().await;

            assert!(result.is_ok());

            println!("Result 8: {:?}", result);

            permit.forget();

        });

        arc_semaphore.add_permits(16);

        let mut index = 0;

        loop
        {

            match js.join_next().await
            {

                Some(val) =>
                {

                    assert!(val.is_ok());

                    index.pp();

                    println!("Task {} joined", index);

                }
                None =>
                {

                    break;

                }

            }

        }

    }

}