//! Automatically rewritten from C Header to Rust Module
//! Source: block/bfq-iosched.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Header file for the BFQ I/O scheduler: data structures and
// prototypes of interface functions among BFQ components.
//

pub const BFQ_IOPRIO_CLASSES: c_int = 3;

pub const BFQ_MIN_WEIGHT: c_int = 1;
pub const BFQ_MAX_WEIGHT: c_int = 1000;
pub const BFQ_WEIGHT_CONVERSION_COEFF: c_int = 10;
pub const BFQ_DEFAULT_QUEUE_IOPRIO: c_int = 4;
pub const BFQ_DEFAULT_GRP_IOPRIO: c_int = 0;

pub const MAX_BFQQ_NAME_LENGTH: c_int = 16;
//
// Soft real-time applications are extremely more latency sensitive
// than interactive ones. Over-raise the weight of the former to
// privilege them against the latter.
//
pub const BFQ_SOFTRT_WEIGHT_FACTOR: c_int = 100;
//
// Maximum number of actuators supported. This constant is used simply
// to define the size of the static array that will contain
// per-actuator data. The current value is hopefully a good upper
// bound to the possible number of actuators of any actual drive.
//
pub const BFQ_MAX_ACTUATORS: c_int = 8;
//
// struct bfq_service_tree - per ioprio_class service tree.
//
// Each service tree represents a B-WF2Q+ scheduler on its own.  Each
// ioprio_class has its own independent scheduler, and so its own
// bfq_service_tree.  All the fields are protected by the queue lock
// of the containing bfqd.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfq_service_tree {
// tree for active entities (i.e., those backlogged)
    pub active: rb_root,
// tree for idle entities (i.e., not backlogged, with V < F_i)
    pub idle: rb_root,
// idle entity with minimum F_i
    pub first_idle: *mut bfq_entity,
// idle entity with maximum F_i
    pub last_idle: *mut bfq_entity,
// scheduler virtual time
    pub vtime: u64,
// scheduler weight sum; active and idle entities contribute to it
    pub wsum: c_ulong,
}

//
// struct bfq_sched_data - multi-class scheduler.
//
// bfq_sched_data is the basic scheduler queue.  It supports three
// ioprio_classes, and can be used either as a toplevel queue or as an
// intermediate queue in a hierarchical setup.
//
// The supported ioprio_classes are the same as in CFQ, in descending
// priority order, IOPRIO_CLASS_RT, IOPRIO_CLASS_BE, IOPRIO_CLASS_IDLE.
// Requests from higher priority queues are served before all the
// requests from lower priority queues; among requests of the same
// queue requests are served according to B-WF2Q+.
//
// The schedule is implemented by the service trees, plus the field
// @next_in_service, which points to the entity on the active trees
// that will be served next, if 1) no changes in the schedule occurs
// before the current in-service entity is expired, 2) the in-service
// queue becomes idle when it expires, and 3) if the entity pointed by
// in_service_entity is not a queue, then the in-service child entity
// of the entity pointed by in_service_entity becomes idle on
// expiration. This peculiar definition allows for the following
// optimization, not yet exploited: while a given entity is still in
// service, we already know which is the best candidate for next
// service among the other active entities in the same parent
// entity. We can then quickly compare the timestamps of the
// in-service entity with those of such best candidate.
//
// All fields are protected by the lock of the containing bfqd.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfq_sched_data {
// entity in service
    pub in_service_entity: *mut bfq_entity,
// head-of-line entity (see comments above)
    pub next_in_service: *mut bfq_entity,
// array of service trees, one per ioprio_class
    pub service_tree: [bfq_service_tree; BFQ_IOPRIO_CLASSES],
// last time CLASS_IDLE was served
    pub bfq_class_idle_last_service: c_ulong,
}

//
// struct bfq_weight_counter - counter of the number of all active queues
// with a given weight.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfq_weight_counter {
    pub /: *mut *mut unsigned int weight; / weight of the queues this counter refers to,
    pub /: *mut *mut unsigned int num_active; / nr of active queues with this weight,
//
// Weights tree member (see bfq_data's @queue_weights_tree)
//
    pub weights_node: rb_node,
}

//
// struct bfq_entity - schedulable entity.
//
// A bfq_entity is used to represent either a bfq_queue (leaf node in the
// cgroup hierarchy) or a bfq_group into the upper level scheduler.  Each
// entity belongs to the sched_data of the parent group in the cgroup
// hierarchy.  Non-leaf entities have also their own sched_data, stored
// in @my_sched_data.
//
// Each entity stores independently its priority values; this would
// allow different weights on different devices, but this
// functionality is not exported to userspace by now.  Priorities and
// weights are updated lazily, first storing the new values into the
// new_* fields, then setting the @prio_changed flag.  As soon as
// there is a transition in the entity state that allows the priority
// update to take place the effective and the requested priority
// values are synchronized.
//
// Unless cgroups are used, the weight value is calculated from the
// ioprio to export the same interface as CFQ.  When dealing with
// "well-behaved" queues (i.e., queues that do not spend too much
// time to consume their budget and have true sequential behavior, and
// when there are no external factors breaking anticipation) the
// relative weights at each level of the cgroups hierarchy should be
// guaranteed.  All the fields are protected by the queue lock of the
// containing bfqd.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfq_entity {
// service_tree member
    pub rb_node: rb_node,
//
// Flag, true if the entity is on a tree (either the active or
// the idle one of its service_tree) or is in service.
//
    pub on_st_or_in_serv: bool,
// B-WF2Q+ start and finish timestamps [sectors/weight]
    pub finish: u64 start,,
// tree the entity is enqueued into; %NULL if not on a tree
    pub tree: *mut rb_root,
//
// minimum start time of the (active) subtree rooted at this
// entity; used for O(log N) lookups into active trees
//
    pub min_start: u64,
// amount of service received during the last service slot
    pub service: c_int,
// budget, used also to calculate F_i: F_i = S_i + @budget / @weight
    pub budget: c_int,
// Number of requests allocated in the subtree of this entity
    pub allocated: c_int,
// device weight, if non-zero, it overrides the default weight of
// bfq_group_data
    pub dev_weight: c_int,
// weight of the queue
    pub weight: c_int,
// next weight if a change is in progress
    pub new_weight: c_int,
// original weight, used to implement weight boosting
    pub orig_weight: c_int,
// parent entity, for hierarchical scheduling
    pub parent: *mut bfq_entity,
//
// For non-leaf nodes in the hierarchy, the associated
// scheduler queue, %NULL on leaf nodes.
//
    pub my_sched_data: *mut bfq_sched_data,
// the scheduler queue this entity belongs to
    pub sched_data: *mut bfq_sched_data,
// flag, set to request a weight, ioprio or ioprio_class change
    pub prio_changed: c_int,

// flag, set if the entity is counted in groups_with_pending_reqs
    pub in_groups_with_pending_reqs: bool,

// last child queue of entity created (for non-leaf entities)
    pub last_bfqq_created: *mut bfq_queue,
}

//
// struct bfq_ttime - per process thinktime stats.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfq_ttime {
// completion time of the last request
    pub last_end_request: u64,
// total process thinktime
    pub ttime_total: u64,
// number of thinktime samples
    pub ttime_samples: c_ulong,
// average process thinktime
    pub ttime_mean: u64,
}

//
// struct bfq_queue - leaf schedulable entity.
//
// A bfq_queue is a leaf request queue; it can be associated with an
// io_context or more, if it is async or shared between cooperating
// processes. Besides, it contains I/O requests for only one actuator
// (an io_context is associated with a different bfq_queue for each
// actuator it generates I/O for). @cgroup holds a reference to the
// cgroup, to be sure that it does not disappear while a bfqq still
// references it (mostly to avoid races between request issuing and
// task migration followed by cgroup destruction).  All the fields are
// protected by the queue lock of the containing bfqd.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfq_queue {
// reference counter
    pub ref: c_int,
// counter of references from other queues for delayed stable merge
    pub stable_ref: c_int,
// parent bfq_data
    pub bfqd: *mut bfq_data,
// current ioprio and ioprio class
    pub ioprio_class: unsigned short ioprio,,
// next ioprio and ioprio class if a change is in progress
    pub new_ioprio_class: unsigned short new_ioprio,,
// last total-service-time sample, see bfq_update_inject_limit()
    pub last_serv_time_ns: u64,
// limit for request injection
    pub inject_limit: c_uint,
// last time the inject limit has been decreased, in jiffies
    pub decrease_time_jif: c_ulong,
//
// Shared bfq_queue if queue is cooperating with one or more
// other queues.
//
    pub new_bfqq: *mut bfq_queue,
// request-position tree member (see bfq_group's @rq_pos_tree)
    pub pos_node: rb_node,
// request-position tree root (see bfq_group's @rq_pos_tree)
    pub pos_root: *mut rb_root,
// sorted list of pending requests
    pub sort_list: rb_root,
// if fifo isn't expired, next request to serve
    pub next_rq: *mut request,
// number of sync and async requests queued
    pub queued: [c_int; 2],
// number of pending metadata requests
    pub meta_pending: c_int,
// fifo list of requests in sort_list
    pub fifo: list_head,
// entity representing this queue in the scheduler
    pub entity: bfq_entity,
// pointer to the weight counter associated with this entity
    pub weight_counter: *mut bfq_weight_counter,
// maximum budget allowed from the feedback mechanism
    pub max_budget: c_int,
// budget expiration (in jiffies)
    pub budget_timeout: c_ulong,
// number of requests on the dispatch list or inside driver
    pub dispatched: c_int,
// status flags
    pub flags: c_ulong,
// node for active/idle bfqq list inside parent bfqd
    pub bfqq_list: list_head,
// associated @bfq_ttime struct
    pub ttime: bfq_ttime,
// when bfqq started to do I/O within the last observation window
    pub io_start_time: u64,
// how long bfqq has remained empty during the last observ. window
    pub tot_idle_time: u64,
// bit vector: a 1 for each seeky requests in history
    pub seek_history: u32,
// node for the device's burst list
    pub burst_list_node: hlist_node,
// position of the last request enqueued
    pub last_request_pos: sector_t,
// Number of consecutive pairs of request completion and
// arrival, such that the queue becomes idle after the
// completion, but the next request arrives within an idle
// time slice; used only if the queue's IO_bound flag has been
// cleared.
//
    pub requests_within_timer: c_uint,
// pid of the process owning the queue, used for logging purposes
    pub pid: pid_t,
//
// Pointer to the bfq_io_cq owning the bfq_queue, set to %NULL
// if the queue is shared.
//
    pub bic: *mut bfq_io_cq,
// current maximum weight-raising time for this queue
    pub wr_cur_max_time: c_ulong,
//
// Minimum time instant such that, only if a new request is
// enqueued after this time instant in an idle @bfq_queue with
// no outstanding requests, then the task associated with the
// queue it is deemed as soft real-time (see the comments on
// the function bfq_bfqq_softrt_next_start())
//
    pub soft_rt_next_start: c_ulong,
//
// Start time of the current weight-raising period if
// the @bfq-queue is being weight-raised, otherwise
// finish time of the last weight-raising period.
//
    pub last_wr_start_finish: c_ulong,
// factor by which the weight of this queue is multiplied
    pub wr_coeff: c_uint,
//
// Time of the last transition of the @bfq_queue from idle to
// backlogged.
//
    pub last_idle_bklogged: c_ulong,
//
// Cumulative service received from the @bfq_queue since the
// last transition from idle to backlogged.
//
    pub service_from_backlogged: c_ulong,
//
// Cumulative service received from the @bfq_queue since its
// last transition to weight-raised state.
//
    pub service_from_wr: c_ulong,
//
// Value of wr start time when switching to soft rt
//
    pub wr_start_at_switch_to_srt: c_ulong,
    pub /: *mut *mut unsigned long split_time; / time of last split,
    pub /: *mut *mut unsigned long first_IO_time; / time of first I/O for this queue,
    pub /: *mut *mut unsigned long creation_time; / when this queue is created,
//
// Pointer to the waker queue for this queue, i.e., to the
// queue Q such that this queue happens to get new I/O right
// after some I/O request of Q is completed. For details, see
// the comments on the choice of the queue for injection in
// bfq_select_queue().
//
    pub waker_bfqq: *mut bfq_queue,
// pointer to the curr. tentative waker queue, see bfq_check_waker()
    pub tentative_waker_bfqq: *mut bfq_queue,
// number of times the same tentative waker has been detected
    pub num_waker_detections: c_uint,
// time when we started considering this waker
    pub waker_detection_started: u64,
// node for woken_list, see below
    pub woken_list_node: hlist_node,
//
// Head of the list of the woken queues for this queue, i.e.,
// of the list of the queues for which this queue is a waker
// queue. This list is used to reset the waker_bfqq pointer in
// the woken queues when this queue exits.
//
    pub woken_list: hlist_head,
// index of the actuator this queue is associated with
    pub actuator_idx: c_uint,
}

//
// struct bfq_data - bfqq data unique and persistent for associated bfq_io_cq
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfq_iocq_bfqq_data {
//
// Snapshot of the has_short_time flag before merging; taken
// to remember its values while the queue is merged, so as to
// be able to restore it in case of split.
//
    pub saved_has_short_ttime: bool,
//
// Same purpose as the previous two fields for the I/O bound
// classification of a queue.
//
    pub saved_IO_bound: bool,
//
// Same purpose as the previous fields for the values of the
// field keeping the queue's belonging to a large burst
//
    pub saved_in_large_burst: bool,
//
// True if the queue belonged to a burst list before its merge
// with another cooperating queue.
//
    pub was_in_burst_list: bool,
//
// Save the weight when a merge occurs, to be able
// to restore it in case of split. If the weight is not
// correctly resumed when the queue is recycled,
// then the weight of the recycled queue could differ
// from the weight of the original queue.
//
    pub saved_weight: c_uint,
    pub saved_io_start_time: u64,
    pub saved_tot_idle_time: u64,
//
// Similar to previous fields: save wr information.
//
    pub saved_wr_coeff: c_ulong,
    pub saved_last_wr_start_finish: c_ulong,
    pub saved_service_from_wr: c_ulong,
    pub saved_wr_start_at_switch_to_srt: c_ulong,
    pub saved_ttime: bfq_ttime,
    pub saved_wr_cur_max_time: c_uint,
// Save also injection state
    pub saved_inject_limit: c_uint,
    pub saved_decrease_time_jif: c_ulong,
    pub saved_last_serv_time_ns: u64,
// candidate queue for a stable merge (due to close creation time)
    pub stable_merge_bfqq: *mut bfq_queue,
    pub /: *mut *mut bool stably_merged; / non splittable if true,
}

//
// struct bfq_io_cq - per (request_queue, io_context) structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfq_io_cq {
// associated io_cq structure
    pub /: *mut *mut io_cq icq; / must be the first member,
//
// Matrix of associated process queues: first row for async
// queues, second row sync queues. Each row contains one
// column for each actuator. An I/O request generated by the
// process is inserted into the queue pointed by bfqq[i][j] if
// the request is to be served by the j-th actuator of the
// drive, where i==0 or i==1, depending on whether the request
// is async or sync. So there is a distinct queue for each
// actuator.
//
    pub bfqq: [*mut bfq_queue; 2][BFQ_MAX_ACTUATORS],
// per (request_queue, blkcg) ioprio
    pub ioprio: c_int,

    pub /: *mut *mut uint64_t blkcg_serial_nr; / the current blkcg serial,

//
// Persistent data for associated synchronous process queues
// (one queue per actuator, see field bfqq above). In
// particular, each of these queues may undergo a merge.
//
    pub bfqq_data: [bfq_iocq_bfqq_data; BFQ_MAX_ACTUATORS],
    pub /: *mut *mut unsigned int requests; / Number of requests this process has in flight,
}

//
// struct bfq_data - per-device data structure.
//
// All the fields are protected by @lock.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfq_data {
// device request queue
    pub queue: *mut request_queue,
// dispatch queue
    pub dispatch: list_head,
// root bfq_group for the device
    pub root_group: *mut bfq_group,
//
// rbtree of weight counters of @bfq_queues, sorted by
// weight. Used to keep track of whether all @bfq_queues have
// the same weight. The tree contains one counter for each
// distinct weight associated to some active and not
// weight-raised @bfq_queue (see the comments to the functions
// bfq_weights_tree_[add|remove] for further details).
//
    pub queue_weights_tree: rb_root_cached,

//
// Number of groups with at least one process that
// has at least one request waiting for completion. Note that
// this accounts for also requests already dispatched, but not
// yet completed. Therefore this number of groups may differ
// (be larger) than the number of active groups, as a group is
// considered active only if its corresponding entity has
// queues with at least one request queued. This
// number is used to decide whether a scenario is symmetric.
// For a detailed explanation see comments on the computation
// of the variable asymmetric_scenario in the function
// bfq_better_to_idle().
//
// However, it is hard to compute this number exactly, for
// groups with multiple processes. Consider a group
// that is inactive, i.e., that has no process with
// pending I/O inside BFQ queues. Then suppose that
// num_groups_with_pending_reqs is still accounting for this
// group, because the group has processes with some
// I/O request still in flight. num_groups_with_pending_reqs
// should be decremented when the in-flight request of the
// last process is finally completed (assuming that
// nothing else has changed for the group in the meantime, in
// terms of composition of the group and active/inactive state of child
// groups and processes). To accomplish this, an additional
// pending-request counter must be added to entities, and must
// be updated correctly. To avoid this additional field and operations,
// we resort to the following tradeoff between simplicity and
// accuracy: for an inactive group that is still counted in
// num_groups_with_pending_reqs, we decrement
// num_groups_with_pending_reqs when the first
// process of the group remains with no request waiting for
// completion.
//
// Even this simpler decrement strategy requires a little
// carefulness: to avoid multiple decrements, we flag a group,
// more precisely an entity representing a group, as still
// counted in num_groups_with_pending_reqs when it becomes
// inactive. Then, when the first queue of the
// entity remains with no request waiting for completion,
// num_groups_with_pending_reqs is decremented, and this flag
// is reset. After this flag is reset for the entity,
// num_groups_with_pending_reqs won't be decremented any
// longer in case a new queue of the entity remains
// with no request waiting for completion.
//
    pub num_groups_with_pending_reqs: c_uint,

//
// Per-class (RT, BE, IDLE) number of bfq_queues containing
// requests (including the queue in service, even if it is
// idling).
//
    pub busy_queues: [c_uint; 3],
// number of weight-raised busy @bfq_queues
    pub wr_busy_queues: c_int,
// number of queued requests
    pub queued: c_int,
// number of requests dispatched and waiting for completion
    pub tot_rq_in_driver: c_int,
//
// number of requests dispatched and waiting for completion
// for each actuator
//
    pub rq_in_driver: [c_int; BFQ_MAX_ACTUATORS],
// true if the device is non rotational and performs queueing
    pub nonrot_with_queueing: bool,
//
// Maximum number of requests in driver in the last
// @hw_tag_samples completed requests.
//
    pub max_rq_in_driver: c_int,
// number of samples used to calculate hw_tag
    pub hw_tag_samples: c_int,
// flag set to one if the driver is showing a queueing behavior
    pub hw_tag: c_int,
// number of budgets assigned
    pub budgets_assigned: c_int,
//
// Timer set when idling (waiting) for the next request from
// the queue in service.
//
    pub idle_slice_timer: hrtimer,
// bfq_queue in service
    pub in_service_queue: *mut bfq_queue,
// on-disk position of the last served request
    pub last_position: sector_t,
// position of the last served request for the in-service queue
    pub in_serv_last_pos: sector_t,
// time of last request completion (ns)
    pub last_completion: u64,
// bfqq owning the last completed rq
    pub last_completed_rq_bfqq: *mut bfq_queue,
// last bfqq created, among those in the root group
    pub last_bfqq_created: *mut bfq_queue,
// time of last transition from empty to non-empty (ns)
    pub last_empty_occupied_ns: u64,
//
// Flag set to activate the sampling of the total service time
// of a just-arrived first I/O request (see
// bfq_update_inject_limit()). This will cause the setting of
// waited_rq when the request is finally dispatched.
//
    pub wait_dispatch: bool,
//
// If set, then bfq_update_inject_limit() is invoked when
// waited_rq is eventually completed.
//
    pub waited_rq: *mut request,
//
// True if some request has been injected during the last service hole.
//
    pub rqs_injected: bool,
// time of first rq dispatch in current observation interval (ns)
    pub first_dispatch: u64,
// time of last rq dispatch in current observation interval (ns)
    pub last_dispatch: u64,
// beginning of the last budget
    pub last_budget_start: ktime_t,
// beginning of the last idle slice
    pub last_idling_start: ktime_t,
    pub last_idling_start_jiffies: c_ulong,
// number of samples in current observation interval
    pub peak_rate_samples: c_int,
// num of samples of seq dispatches in current observation interval
    pub sequential_samples: u32,
// total num of sectors transferred in current observation interval
    pub tot_sectors_dispatched: u64,
// max rq size seen during current observation interval (sectors)
    pub last_rq_max_size: u32,
// time elapsed from first dispatch in current observ. interval (us)
    pub delta_from_first: u64,
//
// Current estimate of the device peak rate, measured in
// [(sectors/usec) / 2^BFQ_RATE_SHIFT]. The left-shift by
// BFQ_RATE_SHIFT is performed to increase precision in
// fixed-point calculations.
//
    pub peak_rate: u32,
// maximum budget allotted to a bfq_queue before rescheduling
    pub bfq_max_budget: c_int,
//
// List of all the bfq_queues active for a specific actuator
// on the device. Keeping active queues separate on a
// per-actuator basis helps implementing per-actuator
// injection more efficiently.
//
    pub active_list: [list_head; BFQ_MAX_ACTUATORS],
// list of all the bfq_queues idle on the device
    pub idle_list: list_head,
//
// Timeout for async/sync requests; when it fires, requests
// are served in fifo order.
//
    pub bfq_fifo_expire: [u64; 2],
// weight of backward seeks wrt forward ones
    pub bfq_back_penalty: c_uint,
// maximum allowed backward seek
    pub bfq_back_max: c_uint,
// maximum idling time
    pub bfq_slice_idle: u32,
// user-configured max budget value (0 for auto-tuning)
    pub bfq_user_max_budget: c_int,
//
// Timeout for bfq_queues to consume their budget; used to
// prevent seeky queues from imposing long latencies to
// sequential or quasi-sequential ones (this also implies that
// seeky queues cannot receive guarantees in the service
// domain; after a timeout they are charged for the time they
// have been in service, to preserve fairness among them, but
// without service-domain guarantees).
//
    pub bfq_timeout: c_uint,
//
// Force device idling whenever needed to provide accurate
// service guarantees, without caring about throughput
// issues. CAVEAT: this may even increase latencies, in case
// of useless idling for processes that did stop doing I/O.
//
    pub strict_guarantees: bool,
//
// Last time at which a queue entered the current burst of
// queues being activated shortly after each other; for more
// details about this and the following parameters related to
// a burst of activations, see the comments on the function
// bfq_handle_burst.
//
    pub last_ins_in_burst: c_ulong,
//
// Reference time interval used to decide whether a queue has
// been activated shortly after @last_ins_in_burst.
//
    pub bfq_burst_interval: c_ulong,
// number of queues in the current burst of queue activations
    pub burst_size: c_int,
// common parent entity for the queues in the burst
    pub burst_parent_entity: *mut bfq_entity,
// Maximum burst size above which the current queue-activation
// burst is deemed as 'large'.
//
    pub bfq_large_burst_thresh: c_ulong,
// true if a large queue-activation burst is in progress
    pub large_burst: bool,
//
// Head of the burst list (as for the above fields, more
// details in the comments on the function bfq_handle_burst).
//
    pub burst_list: hlist_head,
// if set to true, low-latency heuristics are enabled
    pub low_latency: bool,
//
// Maximum factor by which the weight of a weight-raised queue
// is multiplied.
//
    pub bfq_wr_coeff: c_uint,
// Maximum weight-raising duration for soft real-time processes
    pub bfq_wr_rt_max_time: c_uint,
//
// Minimum idle period after which weight-raising may be
// reactivated for a queue (in jiffies).
//
    pub bfq_wr_min_idle_time: c_uint,
//
// Minimum period between request arrivals after which
// weight-raising may be reactivated for an already busy async
// queue (in jiffies).
//
    pub bfq_wr_min_inter_arr_async: c_ulong,
// Max service-rate for a soft real-time queue, in sectors/sec
    pub bfq_wr_max_softrt_rate: c_uint,
//
// Cached value of the product ref_rate*ref_wr_duration, used
// for computing the maximum duration of weight raising
// automatically.
//
    pub rate_dur_prod: u64,
// fallback dummy bfqq for extreme OOM conditions
    pub oom_bfqq: bfq_queue,
    pub lock: spinlock_t,
//
// bic associated with the task issuing current bio for
// merging. This and the next field are used as a support to
// be able to perform the bic lookup, needed by bio-merge
// functions, before the scheduler lock is taken, and thus
// avoid taking the request-queue lock while the scheduler
// lock is being held.
//
    pub bio_bic: *mut bfq_io_cq,
// bfqq associated with the task issuing current bio for merging
    pub bio_bfqq: *mut bfq_queue,
//
// Depth limits used in bfq_limit_depth (see comments on the
// function)
//
    pub async_depths: [c_uint; 2][2],
//
// Number of independent actuators. This is equal to 1 in
// case of single-actuator drives.
//
    pub num_actuators: c_uint,
//
// Disk independent access ranges for each actuator
// in this device.
//
    pub sector: [sector_t; BFQ_MAX_ACTUATORS],
    pub nr_sectors: [sector_t; BFQ_MAX_ACTUATORS],
    pub ia_ranges: [blk_independent_access_range; BFQ_MAX_ACTUATORS],
//
// If the number of I/O requests queued in the device for a
// given actuator is below next threshold, then the actuator
// is deemed as underutilized. If this condition is found to
// hold for some actuator upon a dispatch, but (i) the
// in-service queue does not contain I/O for that actuator,
// while (ii) some other queue does contain I/O for that
// actuator, then the head I/O request of the latter queue is
// returned (injected), instead of the head request of the
// currently in-service queue.
//
// We set the threshold, empirically, to the minimum possible
// value for which an actuator is fully utilized, or close to
// be fully utilized. By doing so, injected I/O 'steals' as
// few drive-queue slots as possibile to the in-service
// queue. This reduces as much as possible the probability
// that the service of I/O from the in-service bfq_queue gets
// delayed because of slot exhaustion, i.e., because all the
// slots of the drive queue are filled with I/O injected from
// other queues (NCQ provides for 32 slots).
//
    pub actuator_load_threshold: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfqq_state_flags {
    BFQQF_just_created = 0,	/* queue just allocated */
    BFQQF_busy,		/* has requests or is in service */
    BFQQF_wait_request,	/* waiting for a request */
    BFQQF_non_blocking_wait_rq, /*
// waiting for a request
// without idling the device
//
    BFQQF_fifo_expire,	/* FIFO checked in this slice */
    BFQQF_has_short_ttime,	/* queue has a short think time */
    BFQQF_sync,		/* synchronous queue */
    BFQQF_IO_bound,		/*
// bfqq has timed-out at least once
// having consumed at most 2/10 of
// its budget
//
    BFQQF_in_large_burst,	/*
// bfqq activated in a large burst,
// see comments to bfq_handle_burst.
//
    BFQQF_softrt_update,	/*
// may need softrt-next-start
// update
//
    BFQQF_coop,		/* bfqq is shared */
    BFQQF_split_coop,	/* shared bfqq will be split */
}

// Expiration reasons.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bfqq_expiration {
    BFQQE_TOO_IDLE = 0,		/*
// queue has been idling for
// too long
//
    BFQQE_BUDGET_TIMEOUT,	/* budget took too long to be used */
    BFQQE_BUDGET_EXHAUSTED,	/* budget consumed */
    BFQQE_NO_MORE_REQUESTS,	/* the queue has no more requests */
    BFQQE_PREEMPTED		/* preemption in progress */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfq_stat {
    pub cpu_cnt: percpu_counter,
    pub aux_cnt: core::sync::atomic::AtomicI64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfqg_stats {
// basic stats
    pub bytes: blkg_rwstat,
    pub ios: blkg_rwstat,

// number of ios merged
    pub merged: blkg_rwstat,
// total time spent on device in ns, may not be accurate w/ queueing
    pub service_time: blkg_rwstat,
// total time spent waiting in scheduler queue in ns
    pub wait_time: blkg_rwstat,
// number of IOs queued up
    pub queued: blkg_rwstat,
// total disk time and nr sectors dispatched by this group
    pub time: bfq_stat,
// sum of number of ios queued across all samples
    pub avg_queue_size_sum: bfq_stat,
// count of samples taken for average
    pub avg_queue_size_samples: bfq_stat,
// how many times this group has been removed from service tree
    pub dequeue: bfq_stat,
// total time spent waiting for it to be assigned a timeslice.
    pub group_wait_time: bfq_stat,
// time spent idling for this blkcg_gq
    pub idle_time: bfq_stat,
// total time with empty current active q with other requests queued
    pub empty_time: bfq_stat,
// fields after this shouldn't be cleared on stat reset
    pub start_group_wait_time: u64,
    pub start_idle_time: u64,
    pub start_empty_time: u64,
    pub flags: u16,

}

//
// struct bfq_group_data - per-blkcg storage for the blkio subsystem.
//
// @ps: @blkcg_policy_storage that this structure inherits
// @weight: weight of the bfq_group
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfq_group_data {
// must be the first member
    pub pd: blkcg_policy_data,
    pub weight: c_uint,
}

//
// struct bfq_group - per (device, cgroup) data structure.
// @entity: schedulable entity to insert into the parent group sched_data.
// @sched_data: own sched_data, to contain child entities (they may be
// both bfq_queues and bfq_groups).
// @bfqd: the bfq_data for the device this group acts upon.
// @async_bfqq: array of async queues for all the tasks belonging to
// the group, one queue per ioprio value per ioprio_class,
// except for the idle class that has only one queue.
// @async_idle_bfqq: async queue for the idle class (ioprio is ignored).
// @my_entity: pointer to @entity, %NULL for the toplevel group; used
// to avoid too many special cases during group creation
// migration.
// @stats: stats for this bfqg.
// @active_entities: number of active entities belonging to the group;
// unused for the root group. Used to know whether there
// are groups with more than one active @bfq_entity
// (see the comments to the function
// bfq_better_to_idle()).
// @rq_pos_tree: rbtree sorted by next_request position, used when
// determining if two or more queues have interleaving
// requests (see bfq_find_close_cooperator()).
//
// Each (device, cgroup) pair has its own bfq_group, i.e., for each cgroup
// there is a set of bfq_groups, each one collecting the lower-level
// entities belonging to the group that are acting on the same device.
//
// Locking works as follows:
// o @bfqd is protected by the queue lock, RCU is used to access it
// from the readers.
// o All the other fields are protected by the @bfqd queue lock.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfq_group {
// must be the first member
    pub pd: blkg_policy_data,
// reference counter (see comments in bfq_bic_update_cgroup)
    pub ref: refcount_t,
    pub entity: bfq_entity,
    pub sched_data: bfq_sched_data,
    pub bfqd: *mut bfq_data,
    pub async_bfqq: [*mut bfq_queue; 2][IOPRIO_NR_LEVELS][BFQ_MAX_ACTUATORS],
    pub async_idle_bfqq: [*mut bfq_queue; BFQ_MAX_ACTUATORS],
    pub my_entity: *mut bfq_entity,
    pub active_entities: c_int,
    pub num_queues_with_pending_reqs: c_int,
    pub rq_pos_tree: rb_root,
    pub stats: bfqg_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bfq_group {
    pub entity: bfq_entity,
    pub sched_data: bfq_sched_data,
    pub async_bfqq: [*mut bfq_queue; 2][IOPRIO_NR_LEVELS][BFQ_MAX_ACTUATORS],
    pub async_idle_bfqq: [*mut bfq_queue; BFQ_MAX_ACTUATORS],
    pub rq_pos_tree: rb_root,
}

// --------------- main algorithm interface -----------------

extern "C" {
    pub fn bfq_pos_tree_add_move(bfqd: *mut bfq_data, bfqq: *mut bfq_queue);
}
extern "C" {
    pub fn bfq_weights_tree_add(bfqq: *mut bfq_queue);
}
extern "C" {
    pub fn bfq_weights_tree_remove(bfqq: *mut bfq_queue);
}
extern "C" {
    pub fn bfq_put_queue(bfqq: *mut bfq_queue);
}
extern "C" {
    pub fn bfq_put_cooperator(bfqq: *mut bfq_queue);
}
extern "C" {
    pub fn bfq_end_wr_async_queues(bfqd: *mut bfq_data, bfqg: *mut bfq_group);
}
extern "C" {
    pub fn bfq_release_process_ref(bfqd: *mut bfq_data, bfqq: *mut bfq_queue);
}
extern "C" {
    pub fn bfq_schedule_dispatch(bfqd: *mut bfq_data);
}
extern "C" {
    pub fn bfq_put_async_queues(bfqd: *mut bfq_data, bfqg: *mut bfq_group);
}
// ------------ end of main algorithm interface --------------
// ---------------- cgroups-support interface ----------------
extern "C" {
    pub fn bfqg_stats_update_legacy_io(q: *mut request_queue, rq: *mut request);
}
extern "C" {
    pub fn bfqg_stats_update_io_remove(bfqg: *mut bfq_group, opf: blk_opf_t);
}
extern "C" {
    pub fn bfqg_stats_update_io_merged(bfqg: *mut bfq_group, opf: blk_opf_t);
}
extern "C" {
    pub fn bfqg_stats_update_dequeue(bfqg: *mut bfq_group);
}
extern "C" {
    pub fn bfqg_stats_set_start_idle_time(bfqg: *mut bfq_group);
}

extern "C" {
    pub fn bfqg_stats_set_start_empty_time(bfqg: *mut bfq_group);
}
extern "C" {
    pub fn bfqg_stats_update_idle_time(bfqg: *mut bfq_group);
}
extern "C" {
    pub fn bfqg_stats_update_avg_queue_size(bfqg: *mut bfq_group);
}

extern "C" {
    pub fn bfq_init_entity(entity: *mut bfq_entity, bfqg: *mut bfq_group);
}
extern "C" {
    pub fn bfq_bic_update_cgroup(bic: *mut bfq_io_cq, bio: *mut bio);
}
extern "C" {
    pub fn bfq_end_wr_async(bfqd: *mut bfq_data);
}
extern "C" {
    pub fn bfqg_and_blkg_put(bfqg: *mut bfq_group);
}

// ------------- end of cgroups-support interface -------------
// - interface of the internal hierarchical B-WF2Q+ scheduler -

// both next loops stop at one of the child entities of the root group

//
// For each iteration, compute parent in advance, so as to be safe if
// entity is deallocated during the iteration. Such a deallocation may
// happen as a consequence of a bfq_put_queue that frees the bfq_queue
// containing entity.
//

//
// Next two macros are fake loops when cgroups support is not
// enabled. I fact, in such a case, there is only one level to go up
// (to reach the root group).
//

extern "C" {
    pub fn bfq_tot_busy_queues(bfqd: *mut bfq_data) -> c_uint;
}
extern "C" {
    pub fn bfq_ioprio_to_weight(ioprio: c_int) -> c_ushort;
}
extern "C" {
    pub fn bfq_bfqq_served(bfqq: *mut bfq_queue, served: c_int);
}
extern "C" {
    pub fn next_queue_may_preempt(bfqd: *mut bfq_data) -> bool;
}
extern "C" {
    pub fn __bfq_bfqd_reset_in_service(bfqd: *mut bfq_data) -> bool;
}
extern "C" {
    pub fn bfq_activate_bfqq(bfqd: *mut bfq_data, bfqq: *mut bfq_queue);
}
extern "C" {
    pub fn bfq_del_bfqq_busy(bfqq: *mut bfq_queue, expiration: bool);
}
extern "C" {
    pub fn bfq_add_bfqq_busy(bfqq: *mut bfq_queue);
}
extern "C" {
    pub fn bfq_add_bfqq_in_groups_with_pending_reqs(bfqq: *mut bfq_queue);
}
extern "C" {
    pub fn bfq_del_bfqq_in_groups_with_pending_reqs(bfqq: *mut bfq_queue);
}
// --------------- end of interface of B-WF2Q+ ----------------
// Logging facilities.

