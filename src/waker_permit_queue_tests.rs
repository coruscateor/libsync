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

        assert!(wpc.add_permit());

        assert_eq!(wpc.avalible_permits(), Some(1));

        assert!(wpc.add_permit());

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

    async fn test_1()
    {



    }

}