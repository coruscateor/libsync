use std::sync::atomic::{AtomicBool, AtomicUsize};

use scc::{HashMap, Queue};

use crate::QueuedWaker;

pub struct WakerPermitQueue
{

    is_closed: AtomicBool,
    queue: Queue<QueuedWaker>,
    latest_handle: AtomicUsize,
    active_handles: HashMap<usize, bool>,
    permits: AtomicUsize,

}

impl WakerPermitQueue
{

    pub fn new() -> Self
    {

        Self
        {

            is_closed: AtomicBool::new(false),
            queue: Queue::new(),
            latest_handle: AtomicUsize::new(0),
            active_handles: HashMap::new(),
            permits: AtomicUsize::new(0)

        }

    }



}