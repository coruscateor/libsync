# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Version 0.3.0 (__/02/2026)

### Added



### Changed



### Deprecated



### Removed



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
