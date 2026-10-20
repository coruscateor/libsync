
mod faux_rwlock;

pub use faux_rwlock::*;

cfg_select!
{

    feature="use_std_sync" =>
    {

        mod std_sync_mutex_guard;

        pub use std_sync_mutex_guard::*;

        mod std_sync_rwlock_read_write_guards;

        pub use std_sync_rwlock_read_write_guards::*;

    }

}
