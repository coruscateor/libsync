
#[cfg(feature="tokio")]
mod tokio_tests
{

    use crate::{BoundedSendError, ReceiveError};

    use inc_dec::IncDecSelf;
    
    use tokio::{sync::{Notify, Semaphore, Barrier}, task::JoinSet};

    use std::sync::Arc;

    use super::super::{SharedReader, SharedWriter};

    #[derive(Default, Clone)]
    struct ImportantDetails
    {

        pub label: String,
        pub amount: u32,
        pub somthing_or_nothing: Option<bool>,
        pub precision: f32

    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn basic_readers_and_writers()
    {

        let sw = SharedWriter::new(ImportantDetails::default());

        let sr1 = sw.get_shared_reader();

        let sr2 = sw.get_shared_reader();

        let sr3 = sw.get_shared_reader();

        let arc_barrier = Arc::new(Barrier::new(4));

        let arc_barrier1 = arc_barrier.clone();

        let arc_barrier2 = arc_barrier.clone();

        let arc_barrier3 = arc_barrier.clone();

        let _ = tokio::spawn(async move
        {

            arc_barrier1.wait().await;

            let cln = sr1.read_clone();

            assert_eq!(cln.amount, 5);

            assert_eq!(cln.label, "Test");

            assert_eq!(cln.precision, 0.75);

            assert_eq!(cln.somthing_or_nothing, Some(true));

            arc_barrier1.wait().await;

        });

        let _ = tokio::spawn(async move
        {

            arc_barrier2.wait().await;

            {

                let rdr = sr2.read();

                assert_eq!(rdr.amount, 5);

                assert_eq!(rdr.label, "Test");

                assert_eq!(rdr.precision, 0.75);

                assert_eq!(rdr.somthing_or_nothing, Some(true));

            }

            arc_barrier2.wait().await;


        });

        let _ = tokio::spawn(async move
        {

            arc_barrier3.wait().await;

            {

                let rdr = sr3.read();

                assert_eq!(rdr.amount, 5);

                assert_eq!(rdr.label, "Test");

                assert_eq!(rdr.precision, 0.75);

                assert_eq!(rdr.somthing_or_nothing, Some(true));

            }

            arc_barrier3.wait().await;

        });

        {

            let mut w = sw.write();

            w.amount = 5;

            w.label = "Test".to_string();

            w.precision = 0.75;

            w.somthing_or_nothing = Some(true);


        }
        
        arc_barrier.wait().await;

        arc_barrier.wait().await;

    }

}