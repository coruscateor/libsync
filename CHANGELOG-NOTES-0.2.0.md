
da52b8c0318e7e238548ee0a28e5ed0cea57be31 -

- Corrected spelling

- Both channel a parts now keep their own sender and receiver counts.

- Partially cleaned up the code.

- SenderTrait, BoundedSenderTrait and ReceiverTrait have been removed.

- In crossbeam/tokio/array_queue/Recevier recv_notify_one has been changed to try_recv, recv_or_wait has been changed to recv and a drop implementation has been added.

- In crossbeam/tokio/array_queue/Sender send_notify_one as been changed to try_send, send_or_wait has been changed to send and send_or_wait_notify_one has been removed.

- A crossbeam/tokio/seg_queue module has been added with channel, Sender and Receiver implementations.




a44d392bfbff64dfc7d7f78acf999662aa804936 -

Added the return_store module with ReturnStoreState, BaseReturnStore, Ba…
…seReturner structs and the Returns trait directly under it and a tokio sub-module with ReturnStore and Returner structs.




5ed939d608b414d762509d3efc147927e31dbe06 -

- Improved handling of waiting and potentially waiting tasks when sides …
…of channels are dropped.

- Did not re-enable blocking methods in channel parts.

- Replaced the ThreadRng oriented implementation of state id generation of the BaseReturnStore with an incrementation centric one.

- Expanded on BaseReturnStore and ReturnStore functionality.



0d6ed92cf6c485263369b7688bd93a732fab3cd7 -

- Began the process of switching Tokio Notify objects with Semaphores in…
… the tokio module, starting with seg_queue.

- Added ChannelSemaphore to the tokio module.

- Added “is_full”, “has_no_receivers” and “take” methods to BoundedSendError.



b4ff48b454cb584804d2c8d8b7f750f5d8a428f1 -

- Disabled the count_waiting_senders_and_receivers feature.

- Added a tokio_helpers module at the base with a SemaphoreController object.

- Got about three-quarters of the way into re-implementing the channels.



95849eeee62bb904bb545d38538391d3cabeffc9 -

- Re-implemented the crossbeam/mpmc/tokio/array_queue Receiver object.

- Added a clone implementation to the crossbeam/mpmc/tokio/seg_queue Receiver object.

- The std/return_store/tokio ReturnStore object now uses a SemaphoreController instead of a Notify object to manage task permits.



ec4d21dc4ff019f27a15cdb1ec6acf568b544e80 -

- Updated the std Notifier struct level documentation.

- Disabled NotifierWaitResult and NotifierWaitTimeoutResult in the std module and updated the methods of the Notifier object to reflect these changes.

- Added try_lock_set_notify_one and try_lock_set_notify_all methods to the std Notifier object.



8821ea6a5723845e873f77c8e60727e4af276e99 -

Fixed the feature flags on modules.



6667e4abc857d39594face2935dee4ab9eb73093 -

I got tokio versioning problems...



853006ec1b4b55302c5f67e79559dcb7abc25d2f -

- Updated the Tokio dependency to 1.40.0.

- Cleaned up the code a bit.

- Made certain that the senders and receivers Semaphores are closed when ever one side is dropped in the Crossbeam Tokio channel implementations.

- In the Crossbeam Tokio Seq Queue Receiver struct, the try_recv method now only forgets a receivers Semaphore permit if it has successfully received a value.



d4e2a5f7a0435519e85e0ac8515905be555114cd -

- The Crossbeam MPMC Tokio array and seg queue channels Receiver objects…
… recv methods now return Options instead of ReceiveResults.

- The Crossbeam MPMC Tokio array queue Sender object now manually implements Clone.

- The BoundedSendError, ReceiveError, TimeoutSendError and TimeoutReceiveError enums now implement the Debug and Display traits.

- Added PipelineMessageCounter, IncrementedPipelineMessageCounter and CountedPipelineMessage structs.



b82b76100cb7421543ff49d28533f5bf217717e6 -

- Manually implemented Clone on the crossbeam/mpmc/tokio/seg_queue/Sende…
…r object.

- Renamed increment_with_data to increment_with_message, replaced increment_with_data_opt with increment_with_message_mut and increment_with_data_opt_none with increment_without_message_mut in the std/PipelineMessageCounter object.

- Added MutCountedPipelineMessage to the std module.



684249464fbb6e823d27c4329db43bf75b730295 -

Renamed MutCountedPipelineMessage to CountedPipelineMessageMut in the st…
…d module.



4574f7f9752613e3be225f0eccfd9d9a49768acb

- Updated how the std/PipelineMessageCounter and the std/IncrementedPipe…
…lineMessageCounter objects handle message counts.

- Added an instance_count method to std/PipelineMessageCounter.

- Added a has_messages method to std/IncrementedPipelineMessageCounter.

- Added an option constructor to std/CountedPipelineMessageMut.



ba2942bf55301e72588c5deda32e403cfab973d3

- Added methods take, take_incremented and take_both to the CountedPipel…
…ineMessage object.

- Added methods take_incremented and take_both to the CountedPipelineMessageMut object.

- Made the as_ref and as_mut methods public in the CountedPipelineMessageMut object.

- Added traits PipelineMessageContainer, PipelineMessageContainerMut and PipelineMessageContainerFactory to the std module.

- Added structs PlainPipelineMessageContainer, PlainPipelineMessageContainerMut, PlainPipelineMessageContainerFactory, CountedPipelineMessageContainer, CountedPipelineMessageContainerMut and CountedPipelineMessageContainerFactory to the std module.



3f543310f0a777af3ef00c5d7edc5c8687884ffd

- PipelineMessageContainerFactory now requires all implementers to imple…
…ment Clone.

- PipelineMessageContainerFactory no longer has T as a struct level type parameter, instead its get and the get_mut method declarations now have this requirement.



e512ebbfe5cbd26d04457be5eec522aeacff22ac

- Updated the project dependencies.

- Changed the FAILED_TO_UNLOCK_MUTEX_MESSAGE static string slice in the std/notifier module.

- Added wait_fn, wait_timeout_fn, lock and try_lock methods to Notifier and updated other methods in the struct implementation.



821f7332958486d114d50325129ac56c52e52553

- Added PolledReturnStore and PolledReturner.

- Made various other changes.



acb21026c36e3ecb578acc0e078dd2c4d0229ec1

Added io_channels modules to the crossbeam::mpmc::tokio::array_queue and…
… seg_queue modules. These modules contain IOClient and IOServer structs and io_channels functions. Additionally the array_queue sub-module contains an io_channel_both function.



91d07bd562cf73efe551009101db797ba7672dd7

Fixed the Display implementations of the result types so they don’t caus…
…e stack overflows.



c45497fbaa90d5165d148a495353dde577252ca1

- Conditionally implemented std::fmt::Debug on SharedDetails, BoundedSha…
…redDetails, crossbeam::mpmc::tokio::array_queue::IOClient, IOServer, Sender, Receiver and for the same-named objects in crossbeam::mpmc::tokio::seg_queue sub-module.

- Moved the array_queue and seg_queue under the crossbeam/mpmc sub-module to the new crossbeam/mpmc/base sub-module.

- Renamed receivers_notifier to receivers_notifier_ref and senders_notifier to senders_notifier_ref in the moved Sender And Receivers object implementation definitions and updated the relevant parts of the project.

- Renamed queue to queue_ref in SharedDetails and updated the relevant parts of the project.



dbf25645563fbe231866862938e0ce5e4af0a134

- The the recv methods of both the crossbeam::mpmc::tokio::array_queue::…
…Receiver and crossbeam::mpmc::tokio::seg_queue::Receiver objects now return ReceiveResult objects instead of Options.



1b95a216016d81cb890f84462d6a67ec2c48cb9e

- Updated the version to 0.2.0-beta.

- Added a package.metadata.docs.rs to the cargo file.

- Removed the dependency notes.txt file.

- Updated the copyright year in the LISENSE-MIT file.

- Updated the readme



7bbc8e879b458afe72a4777a8cfdf26395f67ddb

- Updated the crossbeam dependency to version 0.8.4.

- Updated the tokio dependency to version 1.44.2.

- Updated the delegate dependency to version 0.13.3.

- Updated the futures dependency to version 0.3.31.

- Updated the readme.

- Further updated the documentation.



a82259234a1acd7854f86d84f40944a1a133e564

- Updated the readme

- Updated some documentation



4b9508ed89ae51168cc3617cfe113fb07ed30877

- Updated the readme

- Updated some documentation







