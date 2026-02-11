
use tokio::task::JoinHandle;

pub struct JoinHandlesPoller<T1, F1, FN1, T2, F2, FN2>
    where T1: Unpin,
          T2: Unpin,
          FN1: AsyncFnMut(T1) + Unpin,
          FN2: AsyncFnMut(T2) + Unpin
{

    index: usize,
    f1: JoinHandle<,
    fn1: FN1,
    f2: F2,
    fn2: FN2

}