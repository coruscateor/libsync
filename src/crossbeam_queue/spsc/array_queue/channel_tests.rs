
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

        println!("Hello Start");

        //

        let task = tokio::spawn(async move
        {

            assert!(!receiver.is_closed());

            println!("Hello Task 1");

            let result = receiver.recv().await;

            assert_eq!(result, Ok(1));

            println!("Hello Task 2");

            let result = receiver.recv().await;

            assert_eq!(result, Ok(2));

            //assert!(receiver.is_full());

            println!("Hello Task notify_one");

            arc_sender_notify_task.notify_one();

            println!("Hello Task 3");

            let result = receiver.recv().await;

            assert_eq!(result, Ok(3));

            println!("Hello Task 4");

            let result = receiver.recv().await;

            assert_eq!(result, Ok(4));

            println!("Hello Task receiver notified await");

            arc_receiver_notify_task.notified().await;

        });

        assert!(!sender.is_closed());

        assert_eq!(sender.head_room(), 2);

        println!("Hello 1");

        let result = sender.send(1).await;

        assert!(result.is_ok());

        println!("Hello 2");

        let result = sender.send(2).await;

        assert!(result.is_ok());

        println!("Hello sender notified await");

        arc_sender_notify.notified().await;

        println!("Hello 3");

        let result = sender.try_send(3);

        assert!(result.is_ok());

        println!("Hello 4");

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

        println!("Hello task await is_ok");

        assert!(task.await.is_ok());

        println!("Hello 6");

        let result = sender.try_send(6);

        assert_eq!(result, Err(BoundedSendError::Closed(6)));

        println!("Hello 7");

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

        println!("Hello Start");

        //

        let task = tokio::spawn(async move {

            assert!(!sender.is_closed());

            assert_eq!(sender.head_room(), 2);

            println!("Hello task 1");

            let result = sender.send(1).await;

            assert!(result.is_ok());

            println!("Hello task 2");

            let result = sender.send(2).await;

            assert!(result.is_ok());

            println!("Hello task 3");

            let result = sender.send(3).await;

            assert!(result.is_ok());

            println!("Hello task 4");

            let result = sender.send(4).await;

            assert!(result.is_ok());

            assert!(sender.is_full());

            arc_receiver_notify_task.notify_one();

            println!("Hello task notified await");

            arc_sender_notify_task.notified().await;

        });

        assert!(!receiver.is_closed());

        println!("Hello 1");

        let result = receiver.recv().await;

        assert_eq!(result, Ok(1));

        println!("Hello 2");

        let result = receiver.recv().await;

        assert_eq!(result, Ok(2));

        println!("Hello notified await");

        arc_receiver_notify.notified().await;

        assert!(receiver.is_full());

        let result = receiver.try_recv();

        println!("Hello 3");

        assert_eq!(result, Ok(3));

        let result = receiver.try_recv();

        println!("Hello 4");

        assert_eq!(result, Ok(4));

        let result = receiver.try_recv();

        assert_eq!(result, Err(ReceiveError::Empty));

        arc_sender_notify.notify_one();

        //Make sure the task is finished

        println!("Hello await is_ok");

        assert!(task.await.is_ok());

        let result = receiver.try_recv();

        assert_eq!(result, Err(ReceiveError::Closed));

        println!("Hello recv await");

        let result = receiver.recv().await;

        assert_eq!(result, Err(()));

        assert!(receiver.is_closed());

    }

}