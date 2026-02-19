# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Version 0.3.0 (__/02/2026)

### Added

commit d25c2e80e1b3fd5c2d157e5b776e009e28111106

- Added more documentation.

commit d51af7daf9439cf55c7577a3123be30c192a140b

- Added add channel tests to the crossbeam_queue::mpmc::array_queue sub-module.

- Added add channel tests to the crossbeam_queue::mpmc::seg_queue sub-module.

- Added add channel tests to the crossbeam_queue::spsc::seg_queue sub-module.

-- Added PartialEq and Eq conditional generic constraints to SendError. - Added in this version

commit a981a659731cedaff2d97d778bdee6bcadfc0cce

-- Added a couple more LimitedSingleWakerMultiPermit tests.

commit 53e76305e939fdef79ee8b6da427471813a67237

- Added tests for the LimitedSingleWakerMultiPermit struct.

commit 07e733bdb6849c245e6366ccbc7da0641a087501

-- Added the waiting and cancellation test functions for the LimitedWakerPermitQueue struct. - Added in this version

commit b01311c5f92d71010ce497292483bfd40dd835f0

- Added the crossbeam_queue::spsc::array_queue::channel function.

- Added the crossbeam_queue::spsc::array_queue::Receiver and Sender structs.

- Added the array_queue sub-module to crossbeam_queue::spsc.

--Spell-checked (above)

commit a1f1a51b8ed2c71a946cfdf1aee78ce6a624306f

-- Added LimitedSingleWakerMultiPermitError, LimitedSingleWakerMultiPermitInternalState, LimitedSingleWakerMultiPermit, LimitedSingleWakerMultiPermitDecrementPermitsOrWait and LimitedSingleWakerMultiPermitIncrementPermitsOrWait structs. - Disabled d25c2e80e1b3fd5c2d157e5b776e009e28111106

commit 3475d8c518fe0934eacb4fbb7f4669241b485a0e

- Added the crossbeam-queue optional dependency.

- Added the crossbeam_queue module with mpmc and spsc sub-modules.

-- Added the crossbeam::spsc::seg_queue sub-module with a channel function and Receiver and Sender structs. - Disabled

commit eabe02237a3381fbd5975e565b96b8b2eb18cab0

-- Added mpmc and spsc sub-modules to the scc module. - Disabled

- Added a queue sub-module to the mpmc module.

- Added the scc::mpmc::queue::channel function.

- Added the scc::mpmc::queue::io_channels::IOClient and IOServer structs and io_channels function.

- Added the scc::mpmc::queue::Receiver and Sender structs.

-- Added the queue sub-module to the spsc module. - Disabled

- Added the scc::mpmc::queue::channel function as well as Receiver and Sender structs.

commit 214194063af027da13472321a3c02f9dd1821063

-- Added a crossbeam::mpmc::array_queue::channel function. - Re-write

- Added a crossbeam_queue::mpmc::array_queue::channel function.

-- Added IOClient and IOServer structs and io_channels and io_channel_both to the crossbeam::mpmc::array_queue::io_channel module. - Re-write

- Added IOClient and IOServer structs and io_channels and io_channel_both to the crossbeam_queue::mpmc::array_queue::io_channel module.

-- Added a crossbeam::mpmc::array_queue::Receiver. - Re-write

- Added a crossbeam_queue::mpmc::array_queue::Receiver.

commit 8ef33bfe2f2eaa032a91d493ca891fed38d9c9ac

-- Added the crossbeam::mpmc::array_queue::Sender struct. - Re-write

- Added the crossbeam_queue::mpmc::array_queue::Sender struct.

- Added LimitedWakerPermitQueue test functions.

-- Added the SendError struct. - Removed

commit 3bc28cd0f8e547397609db7cee2d4786495ac3d0

-- Added the LimitedWakerPermitQueueInternals, LimitedWakerPermitQueue, LimitedWakerPermitQueueClosedError and LimitedWakerPermitQueueDecrementPermitsOrWait structs. - Re-write

- Added the LimitedWakerPermitQueueInternals, LimitedWakerPermitQueue, LimitedWakerPermitQueueClosedError, LimitedWakerPermitQueueIncrementPermitsOrWait and LimitedWakerPermitQueueDecrementPermitsOrWait structs.

commit 885c0ff1df917aa7a17fca0146ddcdb8663e552d

-- Added SingleWakerWaiter

-- Added a test for SingleWaker.

commit 0d49bf95c7da07f6ee014eef105e29cb76cf7f71

-- Added test_basics and tokio_tests::test_1 test functions for SingleWakerMultiPermit.

commit b5f2afcb77763db7206b2092fc4d3f8e6d0d2637

-- Added the SingleWakerMultiPermitError, SingleWakerMultiPermitInternalState, SingleWakerMultiPermit and SingleWakerMultiPermitDecrementPermitsOrWait structs.

commit 87bcce1c8666fb1d9d0c2ed94a42fb2203cce8a3

-- Added IOClient, IOServer structs and an io_channels function to the crossbeam::mpmc::seg_queue::io_channels sub-module. - Re-write

- Added IOClient, IOServer structs and an io_channels function to the crossbeam_queue::mpmc::seg_queue::io_channels sub-module.

commit 6f7dc130b0fb1fa8fd1764f2a7f31f7f25a97c1e

-- Renamed the use_std_mutexes feature to use_std_sync, use_parking_lot_mutexes to use_parking_lot_sync and use_parking_lot_fair_mutexes to use_parking_lot_fair_sync.

--- RE: commit 34597b3e1682149ae74411655e78325a116ee1f0

- Added the use_std_sync, use_parking_lot_sync and use_parking_lot_fair_sync features.

commit 5e92a4e83dc5de37b8ed063bcd1542c1c53c9df7

- Added the src::crossbeam_queue::mpmc::array_queue::Sender object.

--- RE: The first bullet-point.

commit d4fb4a224ebdb4d6ac5b956c5290cfbaa62cfc86

-- Added the crossbeam::mpmc::seg_queue::channel function. - Re-write

- Added the crossbeam_queue::mpmc::seg_queue::channel function.

commit 34597b3e1682149ae74411655e78325a116ee1f0

- Added the tokio dependency features rt, macros and rt-multi-thread.

--- The above is adapted from the first point.

- Re-added the parking_lot dependency, made it optional and set its version to 0.12.5.

-- Added the use_std_mutexes, use_parking_lot_mutexes and use_parking_lot_fair_mutexes features. - Re-written, see above

- Added the PreferredMutexType type.

-- Added the test_1 test to waker_permit_queue_tests::tokio_tests. - Redundant see: a49851a5a86814ec9a930883945ec4110b8c2a48

commit a49851a5a86814ec9a930883945ec4110b8c2a48

-- Added WakerPermitQueue tests and tokio_tests sub-modules. - Re-write

- Added WakerPermitQueue tests

commit 3de221f9a304b6d8b81c692ff960c7aae1b1c446

- Added SingleWaker and SingleWakerError. - Disabled



### Changed

commit fc07bc5b482ca9f35b2048bda9a944da47be4ccd

- Updated the package description.

- Updated the package keywords.

-- Updated the minimum expected version of the Tokio dependency to “1.49.0”.

- Updated the minimum expected version of the tokio dependency to “1.49.0”.

-- Updated the minimum expected version of the Delegate dependency to "0.13.5".

- Updated the minimum expected version of the delegate dependency to "0.13.5".

-- Updated the minimum expected version of the SCC dependency to "3.6.2".

- Updated the minimum expected version of the scc dependency to "3.6.2".

- Updated the readme.

commit d25c2e80e1b3fd5c2d157e5b776e009e28111106

- Updated the package version string to "0.3.0-beta".

- Updated old documentation.

-- Disabled object definitions which relate to SPSC queues (to be re-enabled later?). - Add in this version

commit d51af7daf9439cf55c7577a3123be30c192a140b

-- Continued work on the crossbeam_queue::mpmc::array_queue::Receiver.

-- Continued work on the crossbeam_queue::mpmc::seg_queue::Receiver.

-- Continued work on the crossbeam_queue::spsc::array_queue::Sender.

-- Continued work on the crossbeam_queue::spsc::seg_queue::Receiver.

-- Continued work on the LimitedSingleWakerMultiPermit.
    
- Renamed the NoReceivers variant to Closed in the BoundedSendError enum and updated the project accordingly.

- The BoundedSendError enum now has Debug, PartialEq and Eq as conditional generic constraints.

commit 53e76305e939fdef79ee8b6da427471813a67237

-- Continued work on the LimitedWakerPermitQueue tests.

commit 07e733bdb6849c245e6366ccbc7da0641a087501

-- Continued work on the LimitedWakerPermitQueue struct and related structs.

-- Continued work on the WakerPermitQueue struct.

commit b01311c5f92d71010ce497292483bfd40dd835f0

- Updated the package edition to 2024.

-- Continued work on LimitedSingleWakerMultiPermit, LimtedWakerPermitQueue, SingleWakerMultiPermit and related structs.

commit a1f1a51b8ed2c71a946cfdf1aee78ce6a624306f

-- Continued work on the crossbeam_queue::spsc::seg_queue::Receiver.

-- Continued work on the LimitedWakerPermitQueue.

commit 3475d8c518fe0934eacb4fbb7f4669241b485a0e

-- Moved the contents of crossbeam::mpmc::array_queue to crossbeam_queue::mpmc::array_queue.

--- NOTE ITEMS MOVED: crossbeam -> crossbeam_queue

-- Moved the contents of crossbeam::mpmc::seg_queue to crossbeam_queue::mpmc::seg_queue.

--- NOTE ITEMS MOVED: crossbeam -> crossbeam_queue

commit eabe02237a3381fbd5975e565b96b8b2eb18cab0

-- Continued work on crossbeam::mpmc::seg_queue::Sender.

commit 214194063af027da13472321a3c02f9dd1821063

-- Continued work on the crossbeam::mpmc::array_queue::Sender struct.

-- Continued work on the crossbeam::mpmc::seg_queue::channel function.

-- Continued work on the crossbeam::mpmc::seg_queue::Sender struct.

commit 8ef33bfe2f2eaa032a91d493ca891fed38d9c9ac

-- Continued work on the LimitedWakerPermitQueue.

-- Continued work on the WakerPermitQueueDecrementPermitsOrWait struct.

commit ced112623e27bff1e9905ef04db8c75924afb882

-- Continued work on the LimitedWakerPermitQueue and WakerPermitQueue objects.

commit a6a426cce01fd8e840a10dc0f7bfa099c7bcdf56

-- Continued work on the SingleWaker.

-- Continued work on WakerPermitQueue tests.

commit 653a27c05229514c9cca67e744931d06cd4a63a8

- Updated the package version String to "0.3.0-alpha".

-- Decorated the single_waker_tests with the test configuration.

-- Continued work on the WakerPermitQueue.

commit 885c0ff1df917aa7a17fca0146ddcdb8663e552d

-- Disabled SingleWakerMultiPermitError, SingleWakerMultiPermitInternalState, SingleWakerMultiPermit and SingleWakerMultiPermitDecrementPermitsOrWait and related test code.

-- Re-enabled SingleWakerError, SingleWakerInternalState, SingleWaker and continued working on them.

commit 0d49bf95c7da07f6ee014eef105e29cb76cf7f71

-- Made SingleWakerMultiPermitError an enum.

-- Continued work on SingleWakerMultiPermit.

-- Continued work on the WakerPermitQueue.

commit b5f2afcb77763db7206b2092fc4d3f8e6d0d2637

-- Disabled SingleWaker and related structs.

-- Continued work on the WakerPermitQueue.

commit 87bcce1c8666fb1d9d0c2ed94a42fb2203cce8a3

-- Disabled the WakerQueue and the WakerQueueInternals structs.

-- Moved QueuedWaker to its own module file. - Added in this version.

-- Continued work on the WakerPermitQueue.

commit f95eaa8e85efa6a6f58eb42b6706b2e03cc2265a

-- Continued work on the crossbeam::mpmc::seg_queue::Receiver struct.

-- Continued work on the WakerPermitQueue and updated its tests.

commit f0438781d6c523253a80e4e20a28952db370bbdd

-- Continued work on the crossbeam::mpmc::seg_queue::Receiver struct.

-- Continued work on the WakerPermitQueue and updated its tests.

commit 6f7dc130b0fb1fa8fd1764f2a7f31f7f25a97c1e

-- Renamed the use_std_mutexes feature to use_std_sync, use_parking_lot_mutexes to use_parking_lot_sync and use_parking_lot_fair_mutexes to use_parking_lot_fair_sync.

--- NOTE FEATURES RENAMED

commit 5e92a4e83dc5de37b8ed063bcd1542c1c53c9df7

-- Started wok on the src::crossbeam::mpmc::array_queue::Sender object.

--- NOTE - ADDED ACTUALLY

-- Continued work on the src::crossbeam::mpmc::seg_queue::Sender and Receiver objects.

commit d4fb4a224ebdb4d6ac5b956c5290cfbaa62cfc86

-- Continued work on the crossbeam::mpmc::seg_queue::Receiver and Sender structs.

-- Continued work on the WakerPermitQueue, WakerPermitQueueInternals and QueuedWaker structs.

commit 34597b3e1682149ae74411655e78325a116ee1f0

-- Updated the tokio dependency to version 1.48.0 and added features rt, macros and rt-multi-thread. - Re-write -> Added Section

-- Continued work on the WakerPermitQueue struct.

commit a49851a5a86814ec9a930883945ec4110b8c2a48

-- Continued work on the WakerPermitQueue struct.

commit 1e30149593ea4a9ac3944095122361f3a5cf8d24

-- Continued work on the WakerPermitQueue, WakerQueue and other related structs.

commit 3de221f9a304b6d8b81c692ff960c7aae1b1c446

-- Continued work on the WakerPermitQueue.



### Deprecated



### Removed

commit fc07bc5b482ca9f35b2048bda9a944da47be4ccd

-- Removed the DropWaker struct. - Added in this version
    
-- Removed the FuturesPoller struct. - Added in this version

- Removed the BoundedSendErrorType struct.

-- Removed the JoinHandlesPoller struct form the tokio_helpers sub-module. - Added in this version



### Fixed



### Security



</br>

## Version 0.2.0 (23/04/2025)

### Added

- Added the try_send method to the crossbeam/mpmc/tokio/array_queue/Sender object.

- A crossbeam/mpmc/tokio/seg_queue module has been added with channel, Sender and Receiver implementations.

- Added the std/return_store module with ReturnStoreState, BaseReturnStore, BaseReturner structs and the Returns trait directly under it and a tokio sub-module with ReturnStore and Returner structs.

- Added “is_full”, “has_no_receivers” and “take” methods to BoundedSendError.

- Added a tokio_helpers module at the base with a SemaphoreController object.

- Added a clone implementation to the crossbeam/mpmc/tokio/seg_queue Receiver object.

- Added try_lock_set_notify_one and try_lock_set_notify_all methods to the std Notifier object.

- Added feature attributes to the std and tokio module declarations under the crossbeam/mpmc module.

- Added a feature attribute to the tokio module declaration under the std/return_store module.

- Added PipelineMessageCounter, IncrementedPipelineMessageCounter and CountedPipelineMessage structs to the std module.

- Added CountedPipelineMessageMut to the std module.

- Added an instance_count method to std/PipelineMessageCounter.

- Added a has_messages method to std/IncrementedPipelineMessageCounter.

- Added an option constructor to std/CountedPipelineMessageMut.

- Added methods take, take_incremented and take_both to the CountedPipelineMessage object.

- Added methods take_incremented and take_both to the CountedPipelineMessageMut object.

- Added traits PipelineMessageContainer, PipelineMessageContainerMut and PipelineMessageContainerFactory to the std module.

- Added structs PlainPipelineMessageContainer, PlainPipelineMessageContainerMut, PlainPipelineMessageContainerFactory, CountedPipelineMessageContainer, CountedPipelineMessageContainerMut and CountedPipelineMessageContainerFactory to the std module.

- Added wait_fn, wait_timeout_fn, lock and try_lock methods to Notifier and updated other methods in the struct implementation.

- Added NotifyingReturnStoreState, NotifyingReturnStore and NotifyingReturner to the std/return_store module.

- Added PolledReturnStore and PolledReturner to the std/return_store module.

- Added io_channels modules to the crossbeam::mpmc::tokio::array_queue and seg_queue modules. These modules contain IOClient and IOServer structs and io_channels functions. Additionally the array_queue sub-module contains an io_channel_both function.

- Added a package.metadata.docs.rs to the cargo.toml file.



### Changed

- Corrected the spellings of various methods and struct fields.

- Both channel sides now keep their own sender and receiver counts.

- Partially cleaned up the code.

- In crossbeam/mpmc/tokio/array_queue/Recevier recv_notify_one has been changed to try_recv, recv_or_wait has been changed to recv and a drop implementation has been added.

- Improved handling of waiting and potentially waiting tasks when sides of channels are dropped.

- Replaced the ThreadRng oriented implementation of state id generation of the BaseReturnStore with an incrementation centric one.

- Disabled the count_waiting_senders_and_receivers feature.

- Re-implemented the crossbeam/mpmc/tokio/array_queue Receiver object to now use Tokio Semaphores instead of Notify objects.

- The std/return_store/tokio ReturnStore object now uses a SemaphoreController instead of a Notify object to manage task permits.

- Updated the std Notifier struct level documentation.

- Disabled NotifierWaitResult and NotifierWaitTimeoutResult in the std module and updated the methods of the Notifier object to reflect these changes.

- Made certain that the senders and receivers Semaphores are closed when ever one side is dropped in the Crossbeam Tokio channel implementations.

- In the crossbeam/mpmc/tokio/seg_queue/Receiver struct, the try_recv method now only forgets a receivers Semaphore permit if it has successfully received a value.

- The crossbeam/mpmc/tokio/array_queue/Sender object now manually implements Clone.

- The BoundedSendError, ReceiveError, TimeoutSendError and TimeoutReceiveError enums now implement the Debug and Display traits.

- Manually implemented Clone on the crossbeam/mpmc/tokio/seg_queue/Sender object.

- Updated how the std/PipelineMessageCounter and the std/IncrementedPipelineMessageCounter objects handle message counts.

- Made the as_ref and as_mut methods public in the CountedPipelineMessageMut object.

- Changed the FAILED_TO_UNLOCK_MUTEX_MESSAGE static string slice message in the std/notifier module.

- Conditionally implemented std::fmt::Debug on SharedDetails, BoundedSharedDetails,  crossbeam::mpmc::tokio::array_queue::Sender, Receiver, crossbeam::mpmc::tokio::array_queue::io_channels::IOClient and IOServer for the same-named objects in the crossbeam::mpmc::tokio::seg_queue sub-module.

- Moved the array_queue and seg_queue sub-modules under the crossbeam/mpmc module to the new crossbeam/mpmc/base sub-module.

- Renamed receivers_notifier to receivers_notifier_ref and senders_notifier to senders_notifier_ref in the moved Sender And Receivers object implementation definitions and updated the relevant parts of the project.

- Renamed queue to queue_ref in SharedDetails and updated the relevant parts of the project.

- Updated the copyright year in the LISENSE-MIT file.

- Updated the readme

- Updated the crossbeam dependency to version 0.8.4.

- Updated the tokio dependency to version 1.44.2.

- Updated the delegate dependency to version 0.13.3.

- Updated the futures dependency to version 0.3.31.

- Updated the documentation

- Updated the readme (Redundant)

- Updated some documentation (Redundant)

- Updated the readme (Redundant)

- Updated some documentation (Redundant)



### Removed

- SenderTrait, BoundedSenderTrait and ReceiverTrait have been removed.



### Fixed

- Fixed the feature attribute on the std module.

</br>

## Version 0.1.0 (08/05/2024)

- Initial release
