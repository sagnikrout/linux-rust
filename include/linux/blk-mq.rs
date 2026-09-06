//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/blk-mq.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0

pub const BLKDEV_MIN_RQ: c_int = 4;
pub const BLKDEV_DEFAULT_RQ: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rq_end_io_ret {
    RQ_END_IO_NONE,
    RQ_END_IO_FREE,
}

//
// request flags
pub type req_flags_t = __u32 ;
// Keep rqf_name[] in sync with the definitions below
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rqf_flags {
// drive already may have started this one
    __RQF_STARTED,
// request for flush sequence
    __RQF_FLUSH_SEQ,
// merge of different types, fail separately
    __RQF_MIXED_MERGE,
// don't call prep for this one
    __RQF_DONTPREP,
// use hctx->sched_tags
    __RQF_SCHED_TAGS,
// use an I/O scheduler for this request
    __RQF_USE_SCHED,
// vaguely specified driver internal error.  Ignored by block layer
    __RQF_FAILED,
// don't warn about errors
    __RQF_QUIET,
// account into disk and partition IO statistics
    __RQF_IO_STAT,
// runtime pm request
    __RQF_PM,
// on IO scheduler merge hash
    __RQF_HASHED,
// track IO completion time
    __RQF_STATS,
// Look at ->special_vec for the actual data payload instead of the
    bio chain. */
    __RQF_SPECIAL_PAYLOAD,
// request completion needs to be signaled to zone write plugging.
    __RQF_ZONE_WRITE_PLUGGING,
// ->timeout has been called, don't expire again
    __RQF_TIMED_OUT,
    __RQF_RESV,
    __RQF_BITS
}

// flags that prevent us from merging requests:

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mq_rq_state {
    MQ_RQ_IDLE		= 0,
    MQ_RQ_IN_FLIGHT		= 1,
    MQ_RQ_COMPLETE		= 2,
}

//
// Try to put the fields that are referenced together in the same cacheline.
//
// If you modify this structure, make sure to update blk_rq_init() and
// especially blk_mq_rq_ctx_init() to take care of the added fields.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct request {
    pub q: *mut request_queue,
    pub mq_ctx: *mut blk_mq_ctx,
    pub mq_hctx: *mut blk_mq_hw_ctx,
    pub /: *mut *mut blk_opf_t cmd_flags; / op and common flags,
    pub rq_flags: req_flags_t,
    pub tag: c_int,
    pub internal_tag: c_int,
    pub timeout: c_uint,
// the following two fields are internal, NEVER access directly
    pub /: *mut *mut unsigned int __data_len; / total data len,
    pub /: *mut *mut sector_t __sector; / sector cursor,
    pub bio: *mut bio,
    pub biotail: *mut bio,
    pub queuelist: list_head,
    pub rq_next: *mut request,
}

// Time that the first bio started allocating this request.

// Time that this request was allocated for this IO.
// Time that I/O was submitted to the device.

//
// rq sectors used for blk stats. It has the same value
// with blk_rq_sectors(rq), except that it never be zeroed
// by completion.
//
// Number of scatter-gather DMA addr+len pairs after
// physical address coalescing is performed.
//
// The lowest set bit for address gaps between physical segments. This
// provides information necessary for dma optimization opprotunities,
// like for testing if the segments can be coalesced against the
// device's iommu granule.
//

//
// The hash is used inside the scheduler, and killed once the
// request reaches the dispatch list. The ipi_list is only used
// to queue the request for softirq completion, which is long
// after the request has been unhashed (and even removed from
// the dispatch list).
//
// The rb_node is only used inside the io scheduler, requests
// are pruned when moved to the dispatch queue. special_vec must
// only be used if RQF_SPECIAL_PAYLOAD is set, and those cannot be
// insert into an IO scheduler.
//
// Three pointers are available for the IO schedulers, if they need
// more they have to dynamically allocate it.
//
// completion callback.
//
// Returns a mask with all bits starting at req->phys_gap_bit set to 1.
//
extern "C" {
    pub fn blk_op_is_passthrough(_arg: rq->cmd_flags) -> return;
}

//
// enum blk_eh_timer_return - How the timeout handler should proceed
// @BLK_EH_DONE: The block driver completed the command or will complete it at
// a later time.
// @BLK_EH_RESET_TIMER: Reset the request timer and continue waiting for the
// request to complete.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blk_eh_timer_return {
    BLK_EH_DONE,
    BLK_EH_RESET_TIMER,
}

//
// struct blk_mq_hw_ctx - State for a hardware queue facing the hardware
// block device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_mq_hw_ctx {
// @lock: Protects the dispatch list.
    pub lock: spinlock_t,
//
// @dispatch: Used for requests that are ready to be
// dispatched to the hardware but for some reason (e.g. lack of
// resources) could not be sent to the hardware. As soon as the
// driver can send new requests, requests at this list will
// be sent first for a fairer dispatch.
//
    pub dispatch: list_head,
//
// @state: BLK_MQ_S_* flags. Defines the state of the hw
// queue (active, scheduled to restart, stopped).
//
    pub state: c_ulong,
    pub ____cacheline_aligned_in_smp: },
//
// @run_work: Used for scheduling a hardware queue run at a later time.
//
    pub run_work: delayed_work,
// @cpumask: Map of available CPUs where this hctx can run.
    pub cpumask: cpumask_var_t,
//
// @next_cpu: Used by blk_mq_hctx_next_cpu() for round-robin CPU
// selection from @cpumask.
//
    pub next_cpu: c_int,
//
// @next_cpu_batch: Counter of how many works left in the batch before
// changing to the next CPU.
//
    pub next_cpu_batch: c_int,
// @flags: BLK_MQ_F_* flags. Defines the behaviour of the queue.
    pub flags: c_ulong,
//
// @sched_data: Pointer owned by the IO scheduler attached to a request
// queue. It's up to the IO scheduler how to use this pointer.
//
    pub sched_data: *mut c_void,
//
// @queue: Pointer to the request queue that owns this hardware context.
//
    pub queue: *mut request_queue,
// @fq: Queue of requests that need to perform a flush operation.
    pub fq: *mut blk_flush_queue,
//
// @driver_data: Pointer to data owned by the block driver that created
// this hctx
//
    pub driver_data: *mut c_void,
//
// @ctx_map: Bitmap for each software queue. If bit is on, there is a
// pending request in that software queue.
//
    pub ctx_map: sbitmap,
//
// @dispatch_from: Software queue to be used when no scheduler was
// selected.
//
    pub dispatch_from: *mut blk_mq_ctx,
//
// @dispatch_busy: Number used by blk_mq_update_dispatch_busy() to
// decide if the hw_queue is busy using Exponential Weighted Moving
// Average algorithm.
//
    pub dispatch_busy: c_uint,
// @type: HCTX_TYPE_* flags. Type of hardware queue.
    pub type: c_ushort,
// @nr_ctx: Number of software queues.
    pub nr_ctx: c_ushort,
// @ctxs: Array of software queues.
    pub ctxs: *mut blk_mq_ctx,
// @dispatch_wait_lock: Lock for dispatch_wait queue.
    pub dispatch_wait_lock: spinlock_t,
//
// @dispatch_wait: Waitqueue to put requests when there is no tag
// available at the moment, to wait for another try in the future.
//
    pub dispatch_wait: wait_queue_entry_t,
//
// @wait_index: Index of next available dispatch_wait queue to insert
// requests.
//
    pub wait_index: core::sync::atomic::AtomicI32,
//
// @tags: Tags owned by the block driver. A tag at this set is only
// assigned when a request is dispatched from a hardware queue.
//
    pub tags: *mut blk_mq_tags,
//
// @sched_tags: Tags owned by I/O scheduler. If there is an I/O
// scheduler associated with a request queue, a tag is assigned when
// that request is allocated. Else, this member is not used.
//
    pub sched_tags: *mut blk_mq_tags,
// @numa_node: NUMA node the storage adapter has been connected to.
    pub numa_node: c_int,
// @queue_num: Index of this hardware queue.
    pub queue_num: c_uint,
//
// @nr_active: Number of active requests. Only used when a tag set is
// shared across request queues.
//
    pub nr_active: core::sync::atomic::AtomicI32,
// @cpuhp_online: List to store request if CPU is going to die
    pub cpuhp_online: hlist_node,
// @cpuhp_dead: List to store request if some CPU die.
    pub cpuhp_dead: hlist_node,
// @kobj: Kernel object for sysfs.
    pub kobj: kobject,

//
// @debugfs_dir: debugfs directory for this hardware queue. Named
// as cpu<cpu_number>.
//
    pub debugfs_dir: *mut dentry,
// @sched_debugfs_dir:	debugfs directory for the scheduler.
    pub sched_debugfs_dir: *mut dentry,

//
// @hctx_list: if this hctx is not in use, this is an entry in
// q->unused_hctx_list.
//
    pub hctx_list: list_head,
}

//
// struct blk_mq_queue_map - Map software queues to hardware queues
// @mq_map:       CPU ID to hardware queue index map. This is an array
// with nr_cpu_ids elements. Each element has a value in the range
// [@queue_offset, @queue_offset + @nr_queues).
// @nr_queues:    Number of hardware queues to map CPU IDs onto.
// @queue_offset: First hardware queue to map onto. Used by the PCIe NVMe
// driver to map each hardware queue type (enum hctx_type) onto a distinct
// set of hardware queues.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_mq_queue_map {
    pub mq_map: *mut c_uint,
    pub nr_queues: c_uint,
    pub queue_offset: c_uint,
}

//
// enum hctx_type - Type of hardware queue
// @HCTX_TYPE_DEFAULT:	All I/O not otherwise accounted for.
// @HCTX_TYPE_READ:	Just for READ I/O.
// @HCTX_TYPE_POLL:	Polled I/O of any kind.
// @HCTX_MAX_TYPES:	Number of types of hctx.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hctx_type {
    HCTX_TYPE_DEFAULT,
    HCTX_TYPE_READ,
    HCTX_TYPE_POLL,

    HCTX_MAX_TYPES,
}

//
// struct blk_mq_tag_set - tag set that can be shared between request queues
// @ops:	   Pointers to functions that implement block driver behavior.
// @map:	   One or more ctx -> hctx mappings. One map exists for each
// hardware queue type (enum hctx_type) that the driver wishes
// to support. There are no restrictions on maps being of the
// same size, and it's perfectly legal to share maps between
// types.
// @nr_maps:	   Number of elements in the @map array. A number in the range
// [1, HCTX_MAX_TYPES].
// @nr_hw_queues:  Number of hardware queues supported by the block driver that
// owns this data structure.
// @queue_depth:   Number of tags per hardware queue, reserved tags included.
// @reserved_tags: Number of tags to set aside for BLK_MQ_REQ_RESERVED tag
// allocations.
// @cmd_size:	   Number of additional bytes to allocate per request. The block
// driver owns these additional bytes.
// @numa_node:	   NUMA node the storage adapter has been connected to.
// @timeout:	   Request processing timeout in jiffies.
// @flags:	   Zero or more BLK_MQ_F_* flags.
// @driver_data:   Pointer to data owned by the block driver that created this
// tag set.
// @tags:	   Tag sets. One tag set per hardware queue. Has @nr_hw_queues
// elements.
// @shared_tags:
// Shared set of tags. Has @nr_hw_queues elements. If set,
// shared by all @tags.
// @tag_list_lock: Serializes tag_list accesses.
// @tag_list:	   List of the request queues that use this tag set. See also
// request_queue.tag_set_list.
// @srcu:	   Use as lock when type of the request queue is blocking
// (BLK_MQ_F_BLOCKING).
// @tags_srcu:	   SRCU used to defer freeing of tags page_list to prevent
// use-after-free when iterating tags.
// @update_nr_hwq_lock:
// Synchronize updating nr_hw_queues with add/del disk &
// switching elevator.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_mq_tag_set {
    pub ops: *const blk_mq_ops,
    pub map: [blk_mq_queue_map; HCTX_MAX_TYPES],
    pub nr_maps: c_uint,
    pub nr_hw_queues: c_uint,
    pub queue_depth: c_uint,
    pub reserved_tags: c_uint,
    pub cmd_size: c_uint,
    pub numa_node: c_int,
    pub timeout: c_uint,
    pub flags: c_uint,
    pub driver_data: *mut c_void,
    pub tags: *mut blk_mq_tags,
    pub shared_tags: *mut blk_mq_tags,
    pub tag_list_lock: mutex,
    pub tag_list: list_head,
    pub srcu: *mut srcu_struct,
    pub tags_srcu: srcu_struct,
    pub update_nr_hwq_lock: rw_semaphore,
}

//
// struct blk_mq_queue_data - Data about a request inserted in a queue
//
// @rq:   Request pointer.
// @last: If it is the last request in the queue.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_mq_queue_data {
    pub rq: *mut request,
    pub last: bool,
}

extern "C" {
    pub fn bool(: *mut busy_tag_iter_fn)(struct request, : *mut c_void) -> typedef;
}
//
// struct blk_mq_ops - Callback functions that implements block driver
// behaviour.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_mq_ops {
//
// @queue_rq: Queue a new request from block IO.
//
    pub ): *const blk_mq_queue_data,
//
// @commit_rqs: If a driver uses bd->last to judge when to submit
// requests to hardware, it must define this function. In case of errors
// that make us stop issuing further requests, this hook serves the
// purpose of kicking the hardware (which the last request otherwise
// would have done).
//
    pub ): *mut *mut void (commit_rqs)(struct blk_mq_hw_ctx,
//
// @queue_rqs: Queue a list of new requests. Driver is guaranteed
// that each request belongs to the same queue. If the driver doesn't
// empty the @rqlist completely, then the rest will be queued
// individually by the block layer upon return.
//
    pub rqlist): *mut *mut void (queue_rqs)(struct rq_list,
//
// @get_budget: Reserve budget before queue request, once .queue_rq is
// run, it is driver's responsibility to release the
// reserved budget. Also we have to handle failure case
// of .get_budget for avoiding I/O deadlock.
//
    pub ): *mut *mut int (get_budget)(struct request_queue,
//
// @put_budget: Release the reserved budget.
//
    pub int): *mut *mut *mut void (put_budget)(struct request_queue ,,
//
// @set_rq_budget_token: store rq's budget token
//
    pub int): *mut *mut *mut void (set_rq_budget_token)(struct request ,,
//
// @get_rq_budget_token: retrieve rq's budget token
//
    pub ): *mut *mut int (get_rq_budget_token)(struct request,
//
// @timeout: Called on request timeout.
//
    pub ): *mut *mut blk_eh_timer_return (timeout)(struct request,
//
// @poll: Called to poll for completion of a specific tag.
//
    pub ): *mut *mut *mut int (poll)(struct blk_mq_hw_ctx , struct io_comp_batch,
//
// @complete: Mark the request as complete.
//
    pub ): *mut *mut void (complete)(struct request,
//
// @init_hctx: Called when the block layer side of a hardware queue has
// been set up, allowing the driver to allocate/init matching
// structures.
//
    pub int): *mut *mut *mut *mut int (init_hctx)(struct blk_mq_hw_ctx , void , unsigned,
//
// @exit_hctx: Ditto for exit/teardown.
//
    pub int): *mut *mut *mut void (exit_hctx)(struct blk_mq_hw_ctx , unsigned,
//
// @init_request: Called for every command allocated by the block layer
// to allow the driver to set up driver specific data.
//
// Tag greater than or equal to queue_depth is for setting up
// flush request.
//
    pub int): unsigned int,,
//
// @exit_request: Ditto for exit/teardown.
//
    pub int): unsigned,
//
// @cleanup_rq: Called before freeing one request which isn't completed
// yet, and usually for freeing the driver private data.
//
    pub ): *mut *mut void (cleanup_rq)(struct request,
//
// @busy: If set, returns whether or not this queue currently is busy.
//
    pub ): *mut *mut bool (busy)(struct request_queue,
//
// @map_queues: This allows drivers specify their own queue mapping by
// overriding the setup-time function that builds the mq_map.
//
    pub set): *mut *mut void (map_queues)(struct blk_mq_tag_set,

//
// @show_rq: Used by the debugfs implementation to show driver-specific
// information about a request.
//
    pub rq): *mut *mut *mut void (show_rq)(struct seq_file m, struct request,

}

// Keep hctx_flag_name[] in sync with the definitions below
//
// Set when this device requires underlying blk-mq device for
// completing IO:
//
// Alloc tags on a round-robin base instead of the first available one.
//
// Select 'none' during queue registration in case of a single hwq
// or shared hwqs instead of 'mq-deadline'.
//

// Keep hctx_state_name[] in sync with the definitions below
// hw queue is inactive after all its CPUs become offline

extern "C" {
    pub fn blk_mq_destroy_queue(: *mut request_queue);
}
extern "C" {
    pub fn blk_mq_alloc_tag_set(set: *mut blk_mq_tag_set) -> c_int;
}
extern "C" {
    pub fn blk_mq_free_tag_set(set: *mut blk_mq_tag_set);
}
extern "C" {
    pub fn blk_mq_free_request(rq: *mut request);
}
extern "C" {
    pub fn blk_mq_queue_inflight(q: *mut request_queue) -> bool;
}
// return when out of requests
// allocate from reserved pool
// set RQF_PM
//
// Tag address space map.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_mq_tags {
    pub nr_tags: c_uint,
    pub nr_reserved_tags: c_uint,
    pub active_queues: c_uint,
    pub bitmap_tags: sbitmap_queue,
    pub breserved_tags: sbitmap_queue,
    pub rqs: *mut request,
    pub static_rqs: *mut request,
    pub page_list: list_head,
//
// used to clear request reference in rqs[] before freeing one
// request pool
//
    pub lock: spinlock_t,
    pub rcu_head: rcu_head,
}

extern "C" {
    pub fn blk_mq_unique_tag(rq: *mut request) -> u32;
}
//
// blk_mq_rq_state() - read the current MQ_RQ_* state of a request
// @rq: target request.
//
extern "C" {
    pub fn READ_ONCE(_arg: rq->state) -> return;
}
//
// Set the state to complete when completing a request from inside ->queue_rq.
// This is used by drivers that want to ensure special complete actions that
// need access to the request are called on failure, e.g. by nvme for
// multipathing.
//
// Complete the request directly instead of deferring it to softirq or
// completing it another CPU. Useful in preemptible instead of an interrupt.
//
extern "C" {
    pub fn blk_mq_start_request(rq: *mut request);
}
extern "C" {
    pub fn blk_mq_end_request(rq: *mut request, error: blk_status_t);
}
extern "C" {
    pub fn __blk_mq_end_request(rq: *mut request, error: blk_status_t);
}
extern "C" {
    pub fn blk_mq_end_request_batch(ib: *mut io_comp_batch);
}
//
// Only need start/end time stamping if we have iostat or
// blk stats enabled, or using an IO scheduler.
//
// blk_mq_add_to_batch() - add a request to the completion batch
// @req: The request to add to batch
// @iob: The batch to add the request
// @is_error: Specify true if the request failed with an error
// @complete: The completaion handler for the request
//
// Batched completions only work when there is no I/O error and no special
// ->end_io handler.
//
// Return: true when the request was added to the batch, otherwise false
//
// Check various conditions that exclude batch processing:
// 1) No batch container
// 2) Has scheduler data attached
// 3) Not a passthrough request and end_io set
// 4) Not a passthrough request and failed with an error
//
extern "C" {
    pub fn blk_mq_requeue_request(rq: *mut request, kick_requeue_list: bool);
}
extern "C" {
    pub fn blk_mq_kick_requeue_list(q: *mut request_queue);
}
extern "C" {
    pub fn blk_mq_delay_kick_requeue_list(q: *mut request_queue, msecs: c_ulong);
}
extern "C" {
    pub fn blk_mq_complete_request(rq: *mut request);
}
extern "C" {
    pub fn blk_mq_complete_request_remote(rq: *mut request) -> bool;
}
extern "C" {
    pub fn blk_mq_stop_hw_queue(hctx: *mut blk_mq_hw_ctx);
}
extern "C" {
    pub fn blk_mq_start_hw_queue(hctx: *mut blk_mq_hw_ctx);
}
extern "C" {
    pub fn blk_mq_stop_hw_queues(q: *mut request_queue);
}
extern "C" {
    pub fn blk_mq_start_hw_queues(q: *mut request_queue);
}
extern "C" {
    pub fn blk_mq_start_stopped_hw_queue(hctx: *mut blk_mq_hw_ctx, async: bool);
}
extern "C" {
    pub fn blk_mq_start_stopped_hw_queues(q: *mut request_queue, async: bool);
}
extern "C" {
    pub fn blk_mq_quiesce_queue(q: *mut request_queue);
}
extern "C" {
    pub fn blk_mq_wait_quiesce_done(set: *mut blk_mq_tag_set);
}
extern "C" {
    pub fn blk_mq_quiesce_tagset(set: *mut blk_mq_tag_set);
}
extern "C" {
    pub fn blk_mq_unquiesce_tagset(set: *mut blk_mq_tag_set);
}
extern "C" {
    pub fn blk_mq_unquiesce_queue(q: *mut request_queue);
}
extern "C" {
    pub fn blk_mq_delay_run_hw_queue(hctx: *mut blk_mq_hw_ctx, msecs: c_ulong);
}
extern "C" {
    pub fn blk_mq_run_hw_queue(hctx: *mut blk_mq_hw_ctx, async: bool);
}
extern "C" {
    pub fn blk_mq_run_hw_queues(q: *mut request_queue, async: bool);
}
extern "C" {
    pub fn blk_mq_delay_run_hw_queues(q: *mut request_queue, msecs: c_ulong);
}
extern "C" {
    pub fn blk_mq_tagset_wait_completed_request(tagset: *mut blk_mq_tag_set);
}
extern "C" {
    pub fn blk_mq_freeze_queue_nomemsave(q: *mut request_queue);
}
extern "C" {
    pub fn blk_mq_unfreeze_queue_nomemrestore(q: *mut request_queue);
}
extern "C" {
    pub fn blk_freeze_queue_start(q: *mut request_queue);
}
extern "C" {
    pub fn blk_mq_freeze_queue_wait(q: *mut request_queue);
}
extern "C" {
    pub fn blk_mq_unfreeze_queue_non_owner(q: *mut request_queue);
}
extern "C" {
    pub fn blk_freeze_queue_start_non_owner(q: *mut request_queue);
}
extern "C" {
    pub fn blk_mq_num_possible_queues(max_queues: c_uint) -> c_uint;
}
extern "C" {
    pub fn blk_mq_num_online_queues(max_queues: c_uint) -> c_uint;
}
extern "C" {
    pub fn blk_mq_map_queues(qmap: *mut blk_mq_queue_map);
}
extern "C" {
    pub fn blk_mq_update_nr_hw_queues(set: *mut blk_mq_tag_set, nr_hw_queues: c_int);
}
extern "C" {
    pub fn blk_mq_quiesce_queue_nowait(q: *mut request_queue);
}
extern "C" {
    pub fn blk_mq_rq_cpu(rq: *mut request) -> c_uint;
}
extern "C" {
    pub fn __blk_should_fake_timeout(q: *mut request_queue) -> bool;
}
extern "C" {
    pub fn __blk_should_fake_timeout(_arg: q) -> return;
}
//
// blk_mq_rq_from_pdu - cast a PDU to a request
// @pdu: the PDU (Protocol Data Unit) to be casted
//
// Return: request
//
// Driver command data is immediately after the request. So subtract request
// size to get back to the original request.
//
// blk_mq_rq_to_pdu - cast a request to a PDU
// @rq: the request to be casted
//
// Return: pointer to the PDU
//
// Driver command data is immediately after the request. So add request to get
// the PDU.
//

extern "C" {
    pub fn op_is_sync(_arg: rq->cmd_flags) -> return;
}
extern "C" {
    pub fn blk_rq_init(q: *mut request_queue, rq: *mut request);
}
extern "C" {
    pub fn blk_rq_unprep_clone(rq: *mut request);
}
extern "C" {
    pub fn blk_insert_cloned_request(rq: *mut request) -> blk_status_t;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rq_map_data {
    pub pages: *mut page,
    pub offset: c_ulong,
    pub page_order: c_ushort,
    pub nr_entries: c_ushort,
    pub null_mapped: bool,
    pub from_user: bool,
}

extern "C" {
    pub fn blk_rq_unmap_user(: *mut bio) -> c_int;
}
extern "C" {
    pub fn blk_rq_append_bio(rq: *mut request, bio: *mut bio) -> c_int;
}
extern "C" {
    pub fn blk_execute_rq_nowait(rq: *mut request, at_head: bool);
}
extern "C" {
    pub fn blk_execute_rq(rq: *mut request, at_head: bool) -> blk_status_t;
}
extern "C" {
    pub fn blk_rq_is_poll(rq: *mut request) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct req_iterator {
    pub iter: bvec_iter,
    pub bio: *mut bio,
}

//
// blk_rq_pos()			: the current sector
// blk_rq_bytes()		: bytes left in the entire request
// blk_rq_has_data()		: whether the request carries data
// blk_rq_cur_bytes()		: bytes left in the current segment
// blk_rq_sectors()		: sectors left in the entire request
// blk_rq_cur_sectors()		: sectors left in the current segment
// blk_rq_stats_sectors()	: sectors of the entire request used for stats
//
// Some commands like WRITE SAME have a payload or data transfer size which
// is different from the size of the request.  Any driver that supports such
// commands using the RQF_SPECIAL_PAYLOAD flag needs to use this helper to
// calculate the data transfer size.
//
extern "C" {
    pub fn blk_rq_bytes(_arg: rq) -> return;
}
//
// Return the first full biovec in the request.  The caller needs to check that
// there are any bvecs before calling this helper.
//
extern "C" {
    pub fn mp_bvec_iter_bvec(_arg: rq->bio->bi_io_vec, _arg: rq->bio->bi_iter) -> return;
}
extern "C" {
    pub fn blk_steal_bios(list: *mut bio_list, rq: *mut request);
}
//
// Request completion related functions.
//
// blk_update_request() completes given number of bytes and updates
// the request without completing it.
//
extern "C" {
    pub fn blk_abort_request(: *mut request);
}
//
// Number of physical segments as sent to the device.
//
// Normally this is the number of discontiguous data segments sent by the
// submitter.  But for data-less command like discard we might have no
// actual data segments submitted, but the driver might have to add it's
// own special payload.  In that case we still return 1 here so that this
// special payload will be mapped.
//
// Number of discard segments (or ranges) the driver needs to fill in.
// Each discard bio merged into a request is counted as one segment.
//
extern "C" {
    pub fn max_t(short: unsigned, _arg: rq->nr_phys_segments, _arg: 1) -> return;
}
//
// blk_rq_nr_bvec - return number of bvecs in a request
// @rq: request to calculate bvecs for
//
// Returns the number of bvecs.
//
extern "C" {
    pub fn __blk_rq_map_sg(_arg: rq, _arg: sglist, _arg: &last_sg) -> return;
}
extern "C" {
    pub fn blk_dump_rq_flags(: *mut request, : *mut c_char);
}
//
// blk_rq_passthrough_stats - check if this request should account stats
// @rq: request to check
// @q: the queue accumulating the stats
//
// Note, @q does not necessarily need to be the request_queue that provides
// @rq.
//
// Return: true if stats should be accounted.
//
// Requests without a bio do not transfer data.
//
// Stats are accumulated in the bdev, so must have one attached to a
// bio to track stats. Most drivers do not set the bdev for passthrough
// requests, but nvme is one that will set it.
//
// We don't know what a passthrough command does, but we know the
// payload size and data direction. Ensuring the size is aligned to the
// block size filters out most commands with payloads that don't
// represent sector access.
//
