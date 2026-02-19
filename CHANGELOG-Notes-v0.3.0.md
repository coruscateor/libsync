commit fc07bc5b482ca9f35b2048bda9a944da47be4ccd -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Mon Feb 16 18:20:01 2026 +1300

    - Updated the package description.
    
    - Updated the package keywords.
    
    - Updated the minimum expected version of the Tokio dependency to “1.49.0”.
    
    - Updated the minimum expected version of the Delegate dependency to "0.13.5".
    
    - Updated the minimum expected version of the SCC dependency to "3.6.2".
    
    - Updated the readme.

commit d25c2e80e1b3fd5c2d157e5b776e009e28111106 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Mon Feb 16 15:18:03 2026 +1300

    - Updated the package version string to "0.3.0-beta".
    
    - Added more documentation.
    
    - Updated old documentation.
    
    - Removed the DropWaker struct.
    
    - Removed the FuturesPoller struct.
    
    - Disabled object definitions which relate to SPSC queues (to be re-enabled later?).
    
    - Removed the BoundedSendErrorType struct.
    
    - Removed the JoinHandlesPoller struct form the tokio_helpers sub-module.

commit d51af7daf9439cf55c7577a3123be30c192a140b -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Fri Feb 13 20:54:06 2026 +1300

    - Added add channel tests to the crossbeam_queue::mpmc::array_queue sub-module.
    
    - Continued work on the crossbeam_queue::mpmc::array_queue::Receiver.
    
    - Added add channel tests to the crossbeam_queue::mpmc::seg_queue sub-module.
    
    - Continued work on the crossbeam_queue::mpmc::seg_queue::Receiver.
    
    - Added add channel tests to the crossbeam_queue::spsc::seg_queue sub-module.
    
    - Continued work on the crossbeam_queue::spsc::array_queue::Sender.
    
    - Continued work on the crossbeam_queue::spsc::seg_queue::Receiver.
    
    - Continued work on the LimitedSingleWakerMultiPermit.
    
    - Added PartialEq and Eq conditional generic constraints to SendError.
    
    - Renamed the NoReceivers variant to Closed in the BoundedSendError enum and updated the project accordingly.
    
    - The BoundedSendError enum now has Debug, PartialEq and Eq as conditional generic constraints.

commit a981a659731cedaff2d97d778bdee6bcadfc0cce -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Thu Feb 12 19:49:06 2026 +1300

    Added a couple more LimitedSingleWakerMultiPermit tests.

commit 53e76305e939fdef79ee8b6da427471813a67237 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Thu Feb 12 10:00:44 2026 +1300

    - Added tests for the LimitedSingleWakerMultiPermit struct.
    
    - Continued work on the LimitedWakerPermitQueue tests.

commit 07e733bdb6849c245e6366ccbc7da0641a087501 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue Feb 10 21:04:47 2026 +1300

    - Continued work on the LimitedWakerPermitQueue struct and related structs.
    
    - Added the waiting and cancellation test functions for the LimitedWakerPermitQueue struct.
    
    - Continued work on the WakerPermitQueue struct.

commit b01311c5f92d71010ce497292483bfd40dd835f0 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Mon Feb 9 12:25:07 2026 +1300

    - Updated the package edition to 2024.
    
    - Added the crossbeam_queue::spsc::array_queue::channel function.
    
    - Added the crossbeam_queue::spsc::array_queue::Receiver and Sender structs.
    
    - Added the array_queue sub-module to rossbeam_queue::spsc.
    
    - Continued work on LimitedSingleWakerMultiPermit, LimtedWakerPermitQueue, SingleWakerMultiPermit and related structs.

commit a1f1a51b8ed2c71a946cfdf1aee78ce6a624306f -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Sat Feb 7 19:48:49 2026 +1300

    - Continued work on the crossbeam_queue::spsc::seg_queue::Receiver.
    
    - Added LimitedSingleWakerMultiPermitError, LimitedSingleWakerMultiPermitInternalState, LimitedSingleWakerMultiPermit, LimitedSingleWakerMultiPermitDecrementPermitsOrWait and LimitedSingleWakerMultiPermitIncrementPermitsOrWait structs.
    
    - Continued work on the LimitedWakerPermitQueue.

commit 3475d8c518fe0934eacb4fbb7f4669241b485a0e -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Thu Feb 5 20:07:44 2026 +1300

    - Added the crossbeam-queue optional dependency.
    
    - Added the crossbeam_queue module with mpmc and spsc sub-modules.
    
    - Moved the contents of crossbeam::mpmc::array_queue to crossbeam_queue::mpmc::array_queue.
    
    - Moved the contents of crossbeam::mpmc::seg_queue to crossbeam_queue::mpmc::seg_queue.
    
    - Added the crossbeam::spsc::seg_queue sub-module with a channel function and Receiver and Sender structs.

commit eabe02237a3381fbd5975e565b96b8b2eb18cab0 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Thu Feb 5 13:34:33 2026 +1300

    - Continued work on crossbeam::mpmc::seg_queue::Sender.
    
    - Added mpmc and spsc sub-modules to the scc module.
    
    - Added a queue sub-module to the mpmc module.
    
    - Added the scc::mpmc::queue::channel function.
    
    - Added the scc::mpmc::queue::io_channels::IOClient and IOServer structs and io_channels function.
    
    - Added the scc::mpmc::queue::Receiver and Sender structs.
    
    - Added the queue sub-module to the spsc module.
    
    - Added the scc::mpmc::queue::channel function as well as Receiver and Sender structs.

commit 214194063af027da13472321a3c02f9dd1821063 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Wed Feb 4 14:14:16 2026 +1300

    - Added a crossbeam::mpmc::array_queue::channel function.
    
    - Added IOClient and IOServer structs and io_channels and io_channel_both to the crossbeam::mpmc::array_queue::io_channel module.
    
    - Added a crossbeam::mpmc::array_queue::Receiver.
    
    - Continued work on the crossbeam::mpmc::array_queue::Sender struct.
    
    - Continued work on the crossbeam::mpmc::seg_queue::channel function.
    
    - Continued work on the crossbeam::mpmc::seg_queue::Sender struct.

commit 8ef33bfe2f2eaa032a91d493ca891fed38d9c9ac -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue Feb 3 18:25:48 2026 +1300

    - Added the crossbeam::mpmc::array_queue::Sender struct.
    
    - Continued work on the LimitedWakerPermitQueue.
    
    - Added LimitedWakerPermitQueue test functions.
    
    - Added the SendError struct.
    
    - Continued work on the WakerPermitQueueDecrementPermitsOrWait struct.

commit ced112623e27bff1e9905ef04db8c75924afb882 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Mon Feb 2 20:17:27 2026 +1300

    Continued work on the LimitedWakerPermitQueue and WakerPermitQueue objects.

commit 3bc28cd0f8e547397609db7cee2d4786495ac3d0 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Sun Feb 1 20:54:26 2026 +1300

    Added the LimitedWakerPermitQueueInternals, LimitedWakerPermitQueue, LimitedWakerPermitQueueClosedError and LimitedWakerPermitQueueDecrementPermitsOrWait structs.

commit a6a426cce01fd8e840a10dc0f7bfa099c7bcdf56
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Sun Feb 1 14:34:03 2026 +1300

    - Continued work on the SingleWaker.
    
    - Continued work on WakerPermitQueue tests.

commit 653a27c05229514c9cca67e744931d06cd4a63a8 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Sun Feb 1 14:00:34 2026 +1300

    - Updated the package version String to "0.3.0-alpha".
    
    - Decorated the single_waker_tests with the test configuration.
    
    - Continued work on the WakerPermitQueue.

commit 885c0ff1df917aa7a17fca0146ddcdb8663e552d -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Sat Jan 31 20:20:49 2026 +1300

    - Disabled SingleWakerMultiPermitError, SingleWakerMultiPermitInternalState, SingleWakerMultiPermit and SingleWakerMultiPermitDecrementPermitsOrWait and related test code.
    
    - Re-enabled SingleWakerError, SingleWakerInternalState, SingleWaker and continued working on them.
    
    - Added SingleWakerWaiter
    
    - Added a test for SingleWaker.

commit 0d49bf95c7da07f6ee014eef105e29cb76cf7f71 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Fri Jan 30 17:50:15 2026 +1300

    - Made SingleWakerMultiPermitError an enum.
    
    - Continued work on SingleWakerMultiPermit.
    
    - Added test_basics and tokio_tests::test_1 test functions for SingleWakerMultiPermit.
    
    - Continued work on the WakerPermitQueue.

commit b5f2afcb77763db7206b2092fc4d3f8e6d0d2637 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Thu Jan 29 21:12:20 2026 +1300

    - Disabled SingleWaker and related structs.
    
    - Added the SingleWakerMultiPermitError, SingleWakerMultiPermitInternalState, SingleWakerMultiPermit and SingleWakerMultiPermitDecrementPermitsOrWait structs.
    
    - Continued work on the WakerPermitQueue.

commit 87bcce1c8666fb1d9d0c2ed94a42fb2203cce8a3 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Wed Jan 28 11:33:27 2026 +1300

    - Added IOClient, IOServer structs and an io_channels function to the crossbeam::mpmc::seg_queue::io_channels sub-module.
    
    - Disabled the WakerQueue and the WakerQueueInternals structs.
    
    - Moved QueuedWaker to its own module file.
    
    - Continued work on the WakerPermitQueue.

commit f95eaa8e85efa6a6f58eb42b6706b2e03cc2265a -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue Jan 27 12:02:55 2026 +1300

    - Continued work on the crossbeam::mpmc::seg_queue::Receiver struct.
    
    - Continued work on the WakerPermitQueue and updated its tests.

commit f0438781d6c523253a80e4e20a28952db370bbdd -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Thu Jan 22 20:36:11 2026 +1300

    - Continued work on the crossbeam::mpmc::seg_queue::Receiver struct.
    
    - Continued work on the WakerPermitQueue and updated its tests.

commit 6f7dc130b0fb1fa8fd1764f2a7f31f7f25a97c1e -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Mon Jan 19 20:15:40 2026 +1300

    Renamed the use_std_mutexes feature to use_std_sync, use_parking_lot_mutexes to use_parking_lot_sync and use_parking_lot_fair_mutexes to use_parking_lot_fair_sync.

commit 5e92a4e83dc5de37b8ed063bcd1542c1c53c9df7 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Mon Jan 19 12:51:52 2026 +1300

    - Started wok on the src::crossbeam::mpmc::array_queue::Sender object.
    
    - Continued work on the src::crossbeam::mpmc::seg_queue::Sender and Receiver objects.

commit d4fb4a224ebdb4d6ac5b956c5290cfbaa62cfc86 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Sat Jan 17 19:02:28 2026 +1300

    - Added the crossbeam::mpmc::seg_queue::channel function.
    
    - Continued work on the crossbeam::mpmc::seg_queue::Receiver and Sender structs.
    
    - Continued work on the WakerPermitQueue, WakerPermitQueueInternals and QueuedWaker structs.

commit 34597b3e1682149ae74411655e78325a116ee1f0 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Mon Dec 29 19:28:12 2025 +1300

    - Updated the tokio dependency to version 1.48.0 and added features rt, macros and rt-multi-thread.
    
    - Re-added the parking_lot dependency, made it optional and set its version to 0.12.5.
    
    - Added the use_std_mutexes, use_parking_lot_mutexes and use_parking_lot_fair_mutexes features.
    
    - Added the PreferredMutexType type.
    
    - Continued work on the WakerPermitQueue struct.
    
    - Added the test_1 test to waker_permit_queue_tests::tokio_tests.

commit a49851a5a86814ec9a930883945ec4110b8c2a48 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Mon Dec 29 13:11:42 2025 +1300

    - Continued work on the WakerPermitQueue struct.
    
    - Added WakerPermitQueue tests and tokio_tests sub-modules.

commit 1e30149593ea4a9ac3944095122361f3a5cf8d24 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue Dec 23 19:10:25 2025 +1300

    Continued work on the WakerPermitQueue, WakerQueue and other related structs.

commit 3de221f9a304b6d8b81c692ff960c7aae1b1c446 -
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue Dec 23 15:36:06 2025 +1300

    - Added SingleWaker and SingleWakerError.
    
    - Continued work on the WakerPermitQueue.

commit 3b0b40a4080860ccacac0e987e071a5b44aa4ed1
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Fri Dec 5 19:58:18 2025 +1300

    - Added the ChannelSharedDetails struct.
    
    - Added the seq_queue sub module under crossbeam/mpmc.
    
    - Added the Receiver and Sender structs to the crossbeam/mpmc/seg_queue module.
    
    - Continued work on the WakerPermitQueue.

commit 76e8aa26c456002ed1eeb2c35ae74f77b018759f
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Thu Dec 4 15:50:28 2025 +1300

    Added the array_queue and the seg_queue sub-modules to crossbeam/mpmc. These modules each contain channel, io_channels, receiver and sender sub-modules.

commit 4582aa5e38e93091d096d5a3bbf813a6a16e3357
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Wed Dec 3 16:53:22 2025 +1300

    Continued working on the WakerPermitQueue.

commit 1c3a9f2b453f310dc4b8037b0099b959ede943d4
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue Dec 2 15:06:56 2025 +1300

    Continued working on the WakerPermitQueue.

commit f0120635aa9b4e48eb909e5680e96a3fcc53742b
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Mon Dec 1 15:40:01 2025 +1300

    Continued working on the WakerPermitQueue.

commit 3c4046b64161ae52f99502abf29d2b4a210bcc0e
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Fri Nov 21 20:07:21 2025 +1300

    - Added the scc optional dependency.
    
    - Added the scc module.
    
    - Added WakerPermitQueue to the scc module.

commit f029c08f83683c3f965dd9ced5503adeb76afcc8
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Fri Nov 21 15:58:52 2025 +1300

    - Added DropWaker

commit 611223361fab754b8e5d0431afe81c7b22977246
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Thu Nov 20 12:36:46 2025 +1300

    Renamed LimitedNotifierInternals, LimitedNotifier and LimitedNotifierClosedError to WakerPermitQueueInternals, WakerPermitQueue and WakerPermitQueueClosedError respectively.

commit 2f15c2b6694543f368c5c53012ad2a0eeae89a79
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Wed Nov 19 13:49:37 2025 +1300

    Worked on updating LimitedNotifier to be more like QueuedWaker.

commit f20668b531c8d49d384eb406cdeded50479b609a
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue Nov 18 16:22:39 2025 +1300

    Continued work on the WakerQueue.

commit f932ff63de59b9cc1fda4533779641ccbbf04819
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Thu Nov 13 19:10:15 2025 +1300

    - Continued work on the WakerQueue.
    
    WIP

commit 14dfa158acd50be7a1d835ebc867b89c32e56793
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Thu Nov 13 17:18:41 2025 +1300

    - Added the inc_dec dependency.
    
    - Added QueuedWaker, WakerQueueInternals and continued work on the WakerQueue.
    
    WIP

commit 904dc70580e9f3cfdc58abc5ed41ae45bd7d4b3c
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Wed Nov 12 16:06:52 2025 +1300

    - Added WakerQueue, WakerQueueWakeMeClosedError and WakerQueueWakeMe.

commit 2df961f5befdcdfcf19a794355a2058152bdef05
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue Nov 4 19:22:56 2025 +1300

    - Added the paste dependency.
    
    - Added the accessorise dependency.
    
    - Added the LimitedNotifier, LimitedNotifierClosedError and LimitedNotifierAquire structs.

commit b61140f7346bf6bb321ca7a31591d5ee754e906e
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Wed Apr 23 17:10:46 2025 +1200

    Version 0.2.0

commit f8ddf1cba9a3714a2a7c3dce570d315e8de23bfd
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Wed Apr 23 17:00:17 2025 +1200

    Finalised the changelog.

commit 4a1a11d8d0b07e982d26cf673c09bd78d7ffa0bc
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Wed Apr 23 16:49:07 2025 +1200

    Nearly finalised the changelog.

commit bf6076124866c86067e5cb1e1b399ee456953922
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Wed Apr 23 16:20:35 2025 +1200

    Nearly finalised the changelog.

commit c416be448d5f35075413d210ab5d473f2c3649f1
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue Apr 22 18:46:42 2025 +1200

    Worked on the changelog.

commit 2697b5b2afd31be7f1682a380f25e48a25a07b0e
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue Apr 22 15:34:47 2025 +1200

    Compiled the changelog notes.

commit 4b9508ed89ae51168cc3617cfe113fb07ed30877
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Tue Apr 22 15:09:36 2025 +1200

    - Updated the readme
    
    - Updated some documentation

commit a82259234a1acd7854f86d84f40944a1a133e564
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Mon Apr 21 17:41:26 2025 +1200

    - Updated the readme
    
    - Updated some documentation

commit 7bbc8e879b458afe72a4777a8cfdf26395f67ddb
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Sun Apr 20 18:57:13 2025 +1200

    - Updated the crossbeam dependency to version 0.8.4.
    
    - Updated the tokio dependency to version 1.44.2.
    
    - Updated the delegate dependency to version 0.13.3.
    
    - Updated the futures dependency to version 0.3.31.
    
    - Updated the readme.
    
    - Further updated the documentation.

commit 1b95a216016d81cb890f84462d6a67ec2c48cb9e
Author: Paul Saunders <coruscateor@users.noreply.github.com>
Date:   Fri Apr 18 18:50:34 2025 +1200

    - Updated the version to 0.2.0-beta.
    
    - Added a package.metadata.docs.rs to the cargo file.
    
    - Removed the dependency notes.txt file.
    
    - Updated the copyright year in the LISENSE-MIT file.
    
    - Updated the readme
    
    - Added documentation
    
    - Added the readme include_str at the top of the lib file.
    
    - Added a docsrs cfg_attr to the lib file.
