commit cce9a7d74e64510c988688a4fc4392e138de4fab
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Fri May 15 16:49:20 2026 +1200

    - Updated the package version to 0.4.0-beta.
    
    - Updated the package description.
    
    - Updated some documentation.
    
    - Added a crossbeam_queue::mpmc::seg_queue::Broadcaster struct.
    
    - Added a scc::mpmc::queue::Broadcaster struct.

commit a2e36351219078497daafa0ab0299339c64f0be8
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Fri May 15 13:07:28 2026 +1200

    - Added a crossbeam_queue::mpmc::array_queue::Broadcaster struct.
    
    - Added same_channel, shared_details_ptr_addr and same_channel_sender methods to the crossbeam_queue::mpmc::array_queue::Receiver struct.
    
    - Added same_channel, shared_details_ptr_addr and same_channel_receiver
     methods to the crossbeam_queue::mpmc::array_queue::Sender struct.
    
    - Added same_channel, shared_details_ptr_addr and same_channel_sender methods to the crossbeam_queue::mpmc::seg_queue::Receiver struct.
    
    - Added same_channel, shared_details_ptr_addr and same_channel_receiver
    methods to the crossbeam_queue::mpmc::seg_queue::Sender struct.
    
    - Added same_channel, shared_details_ptr_addr and same_channel_sender methods to the scc::mpmc::queue::Receiver struct.
    
    - Added same_channel, shared_details_ptr_addr and same_channel_receiver
    methods to the scc::mpmc::queue::Sender struct.

commit 038c0b988f5f510fc025fadab10986e01c90ac4b
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue May 12 17:27:02 2026 +1200

    - Updated the tokio dependency to version 1.52.3.
    
    - Added a blocking_recv method to the crossbeam_queue::mpmc::array_queue::Receiver struct.
    
    - Added a blocking_send method to the crossbeam_queue::mpmc::array_queue::Sender struct.
    
    - Added a blocking_recv method to the crossbeam_queue::mpmc::seg_queue::Receiver struct.
    
    - Added a blocking_recv method to the scc::mpmc::queue::Receiver struct.
    
    - Continued work on the NotifyingSharedReader and NotifyingSharedWriter structs.

commit 5a5a88cb766b65c2172ba880389584a589caff59
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Fri May 8 20:44:33 2026 +1200

    - Added and disabled the Dropbox related structs.
    
    - Continued work on the new objects.
    
    - Added the WeakNotifyingSharedReader and WeakNotifyingSharedWriter structs to the shared_reading_and_writing module.

commit 4e2bcccbf6ac7eafb23cf459d819b6d300d47512
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue May 5 21:32:13 2026 +1200

    - Continued work on a bunch of objects.
    
    - Updated the package version to 0.4.0-alpha.
    
    - Removed the paste dependency.
    
    - Added the pastey dependency.
    
    - Added the DropBox and NotifyingWriter structs.

commit f860cc4dbaaf60c0bdceb9bea548298d1c6ba71d
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Mon May 4 20:49:35 2026 +1200

    Continued work on various objects.

commit 6d8912ad206c80df32ba0932edadae1363c1850b
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Sat May 2 19:14:28 2026 +1200

    Added the ItemUpdater trait.
    
    Added the U32Updater struct.
    
    Added the InstantUpdater struct.
    
    Added the NotifyingSharedInternals struct to the shared_reading_and_writing module.
    
    Added the NotifyingSharedReader struct to the shared_reading_and_writing module.
    
    Added the NotifyingSharedReader struct to the shared_reading_and_writing module.
    
    Added the WakerQueueWithUpdatedItemInternals struct.
    
    Added the WakerQueueWithUpdatedItem struct.
    
    Added the WakerQueueWakeMeWithItemClosedError struct.
    
    Added the WakerQueueWakeMeWithItem struct.
    
    WIP

commit a67cc629a0878a48ea21e35fdac2a69f6fbde232
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Fri May 1 20:14:14 2026 +1200

    - Added the WakerQueueInternals, WakerQueue, WakerQueueWakeMeClosedError and  WakerQueueWakeMe structs.

commit 2ff9fff5ba9193401c4e24cd91c52a1561cc14d8
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Thu Apr 30 20:21:45 2026 +1200

    - Added the Reader struct to the shared_reading_and_writing module.
    
    - Continued work on SharedReader, SharedWriter and WeakSharedReader.
    
    - Added the Writer struct to the shared_reading_and_writing module.

commit cfb240893d2033d4eb3c52e14788394d781b8a59
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Wed Apr 29 19:13:20 2026 +1200

    - Added PreferredRwLockType declarations.
    
    - Added the shared_reading_and_writing public module.
    
    - Added SharedReader struct to the shared_reading_and_writing module.
    
    - Added SharedWriter struct to the shared_reading_and_writing module.
    
    - Added WeakSharedReader struct to the shared_reading_and_writing module.
    
    - Added weakSharedWriter struct to the shared_reading_and_writing module.

commit 265f85f28da01d1de5cf43a58c5f67a8c411da77
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue Apr 28 20:08:07 2026 +1200

    - Updated the tokio dependency to version 1.52.1.
    
    - Added WeakReceiver and WeakSender structs to the crossbeam_queue::mpmc::array_queue, crossbeam_queue::mpmc::seg_queue and scc::mpmc::queue modules.
    
    - The shared_details parameters of the “new” methods of the Sender structs in the crossbeam_queue::mpmc::array_queue, crossbeam_queue::mpmc::seg_queue and scc::mpmc::queue modules now take values. The channel functions in each aforementioned module have been updated to reflect this.
    
    - Downgrade methods have been added to the Sender and Receiver struct implementations in the crossbeam_queue::mpmc::array_queue, crossbeam_queue::mpmc::seg_queue and scc::mpmc::queue modules.

commit 21e3ad5ac3e4ff0c16af8b54aff91c0092ec3f27
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Thu Apr 16 14:16:57 2026 +1200

    - Removed the return_store sub-module and its contents from the std sub-module.
    
    - Removed the CountedPipelineMessage, CountedPipelineMessageContainer, CountedPipelineMessageContainerFactory, CountedPipelineMessageContainerMut, CountedPipelineMessageMut, IncrementedPipelineMessageCounter, PlainPipelineMessageContainer, PlainPipelineMessageContainerFactory and PlainPipelineMessageContainerMut structs as well as the PipelineMessageContainer, PipelineMessageContainerFactory and PipelineMessageContainerMut traits from the std sub-module.

commit 2154d7f9fb657362467fd67a5aab2151afc752bb
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Wed Apr 15 19:28:54 2026 +1200

    - Added recv_timeout_tokio and recv_timeout_at_tokio methods to the crossbeam_queue::mpmc::array_queue::Receiver, crossbeam_queue::mpmc::seg_queue::Receiver and scc::mpmc::queue::Receiver struct implementations.
    
    - Added send_timeout_tokio and send_timeout_at_tokio methods to the crossbeam_queue::mpmc::array_queue::Sender struct implementation.
    
    - Fixed an error where the wrong type of SeqQueue was being imported at various places in the crossbeam_queue::mpmc::seg_queue module.
    
    - Added an is_closed method to the scc::mpmc::queue::Receiver struct implementation.
