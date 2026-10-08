use std::hint::spin_loop;

use inc_dec::IncDecSelf;

pub fn spin_count(less_than: usize)
{

    let mut count = 0;

    while count < less_than
    {

        spin_loop();

        count.pp();

    }

}

pub fn spin_a_bit()
{

    spin_count(2);

}
