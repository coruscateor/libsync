use std::time::Instant;

use inc_dec::IntIncDecSelf;

use crate::ItemUpdater;


pub struct U32Updater
{
}

impl ItemUpdater<u32> for U32Updater
{

    fn init() -> u32
    {
        u32::default()
    }

    fn update(item: &mut u32)
    {
        
        item.wpp();

    }

}

pub struct InstantUpdater
{
}

impl ItemUpdater<Instant> for InstantUpdater
{

    fn init() -> Instant
    {

        Instant::now()

    }

    fn update(item: &mut Instant)
    {

        *item = Instant::now();

    }

}