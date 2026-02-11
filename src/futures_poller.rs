use std::{pin::{Pin, pin}, process::Output, task::Poll, thread::JoinHandle};

use futures::FutureExt;

//use tokio::pin;

//use std::future::pin;

//use futures::FutureExt;


pub struct FuturesPoller<T1, F1, FN1, T2, F2, FN2>
    where T1: Unpin,
          T2: Unpin,
          F1: Future<Output = T1> + Unpin,
          FN1: AsyncFnMut(T1) + Unpin,
          F2: Future<Output = T2> + Unpin,
          FN2: AsyncFnMut(T2) + Unpin
{

    index: usize,
    f1: F1,
    fn1: FN1,
    f2: F2,
    fn2: FN2

}

impl<T1, F1, FN1, T2, F2, FN2> FuturesPoller<T1, F1, FN1, T2, F2, FN2>
    where T1: Unpin,
          T2: Unpin,
          F1: Future<Output = T1> + Unpin,
          FN1: AsyncFnMut(T1) + Unpin,
          F2: Future<Output = T2> + Unpin,
          FN2: AsyncFnMut(T2) + Unpin
{

    pub fn new(f1: F1, fn1: FN1, f2: F2, fn2: FN2) -> Self
    {

        Self
        {

            index: 0,
            f1,
            fn1,
            f2,
            fn2

        }

    }

    fn set_next_index(&mut self)
    {

        match self.index
        {

            0 =>
            {

                self.index = 1;

            }
            1 =>
            {

                self.index = 0;

            }
            _ =>
            {

                self.index = 0;

            }
            
        }

    }

}

impl<T1, F1, FN1, T2, F2, FN2> Future for FuturesPoller<T1, F1, FN1, T2, F2, FN2>
    where T1: Unpin,
          T2: Unpin,
          F1: Future<Output = T1> + Unpin,
          FN1: AsyncFnMut(T1) + Unpin,
          F2: Future<Output = T2> + Unpin,
          FN2: AsyncFnMut(T2) + Unpin
{

    type Output = Option<usize>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output>
    {
        
        //let f1_pinned = pin!(&mut self.f1);

        let mut_self = self.get_mut();

        match mut_self.index
        {

            0 =>
            {

                match mut_self.f1.poll_unpin(cx) //.poll_unpin(cx)
                {

                    Poll::Ready(res) =>
                    {

                        {

                            let fn1 = &mut mut_self.fn1;

                            //fn1(res).poll_unpin(cx);

                            let fut = fn1(res);

                            let pinned_fut = pin!(fut);

                            let _fn1_res = pinned_fut.poll(cx); //.poll_unpin(cx); //.poll(cx);
                            
                        }

                        mut_self.set_next_index();

                        return Poll::Ready(Some(1));

                    }
                    Poll::Pending => {}
                }

            }
            1 =>
            {

                match mut_self.f2.poll_unpin(cx) //.poll_unpin(cx)
                {

                    Poll::Ready(res) =>
                    {

                        {

                            let fn2 = &mut mut_self.fn2;

                            //fn1(res).poll_unpin(cx);

                            let fut = fn2(res);

                            let pinned_fut = pin!(fut);

                            let _fn2_res = pinned_fut.poll(cx);

                        }

                        mut_self.set_next_index();

                        return Poll::Ready(Some(1));

                    }
                    Poll::Pending => {}
                }  

            }
            _ =>
            {

                println!("FuturesPoller Error: polling index out of bounds!")

            }
            
        }

        return Poll::Ready(None);

    }

}

//F1, 

//F2,

/*
impl<T1, FN1, T2, FN2> Future for FuturesPoller<T1, tokio::task::JoinHandle<T1>, FN1, T2, tokio::task::JoinHandle<T2>, FN2>
    where T1: Unpin,
          T2: Unpin,
          //F1: Future<Output = T1>,
          FN1: AsyncFnMut(T1) + Unpin,
          //F2: Future<Output = T2>,
          FN2: AsyncFnMut(T2) + Unpin
{

    type Output = Option<usize>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output>
    {
        
        //let f1_pinned = pin!(&mut self.f1);

        let mut_self = self.get_mut();

        match mut_self.index
        {

            0 =>
            {

                match mut_self.f1.poll_unpin(cx) //.poll_unpin(cx)
                {

                    Poll::Ready(res) =>
                    {

                        {

                            let fn1 = &mut mut_self.fn1;

                            //fn1(res).poll_unpin(cx);

                            let fut = fn1(res);

                            let pinned_fut = pin!(fut);

                            let _fn1_res = pinned_fut.poll(cx); //.poll_unpin(cx); //.poll(cx);
                            
                        }

                        mut_self.set_next_index();

                        return Poll::Ready(Some(1));

                    }
                    Poll::Pending => {}
                }

            }
            1 =>
            {

                match mut_self.f2.poll_unpin(cx) //.poll_unpin(cx)
                {

                    Poll::Ready(res) =>
                    {

                        {

                            let fn2 = &mut mut_self.fn2;

                            //fn1(res).poll_unpin(cx);

                            let fut = fn2(res);

                            let pinned_fut = pin!(fut);

                            let _fn2_res = pinned_fut.poll(cx);

                        }

                        mut_self.set_next_index();

                        return Poll::Ready(Some(1));

                    }
                    Poll::Pending => {}
                }  

            }
            _ =>
            {

                println!("FuturesPoller Error: polling index out of bounds!")

            }
            
        }

        return Poll::Ready(None);

    }

}
*/