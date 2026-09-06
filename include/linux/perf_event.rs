//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/perf_event.h
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


//
// Performance events:
//
// Copyright (C) 2008-2009, Linutronix GmbH, Thomas Gleixner <tglx@kernel.org>
// Copyright (C) 2008-2011, Red Hat, Inc., Ingo Molnar
// Copyright (C) 2008-2011, Red Hat, Inc., Peter Zijlstra
//
// Data type definitions, declarations, prototypes.
//
// Started by: Thomas Gleixner and Ingo Molnar
//
// For licencing details see kernel-base/COPYING
//

//
// Kernel-internal data types and definitions:
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_callchain_entry {
    pub nr: u64,
    pub /: *mut *mut u64 ip[]; / /proc/sys/kernel/perf_event_max_stack,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_callchain_entry_ctx {
    pub entry: *mut perf_callchain_entry,
    pub max_stack: u32,
    pub nr: u32,
    pub contexts: c_short,
    pub contexts_maxed: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_raw_frag {
    pub next: *mut perf_raw_frag,
    pub pad: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_raw_record {
    pub frag: perf_raw_frag,
    pub size: u32,
}

//
// branch stack layout:
// nr: number of taken branches stored in entries[]
// hw_idx: The low level index of raw branch records
// for the most recent branch.
// -1ULL means invalid/unknown.
//
// Note that nr can vary from sample to sample
// branches (to, from) are stored from most recent
// to least recent, i.e., entries[0] contains the most
// recent branch.
// The entries[] is an abstraction of raw branch records,
// which may not be stored in age order in HW, e.g. Intel LBR.
// The hw_idx is to expose the low level index of raw
// branch record for the most recent branch aka entries[0].
// The hw_idx index is between -1 (unknown) and max depth,
// which can be retrieved in /sys/devices/cpu/caps/branches.
// For the architectures whose raw branch records are
// already stored in age order, the hw_idx should be 0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_branch_stack {
    pub nr: u64,
    pub hw_idx: u64,
    pub entries: [perf_branch_entry; ],
}

//
// extra PMU register associated with an event
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_perf_event_extra {
    pub /: *mut *mut u64 config; / register value,
    pub /: *mut *mut unsigned int reg; / register address or index,
    pub /: *mut *mut int alloc; / extra register already allocated,
    pub /: *mut *mut int idx; / index in shared_regs->regs[],
}

//
// hw_perf_event::flag values
//
// PERF_EVENT_FLAG_ARCH bits are reserved for architecture-specific
// usage.
//
pub const PERF_EVENT_FLAG_ARCH: c_uint = 0x0fffffff;
pub const PERF_EVENT_FLAG_USER_READ_CNT: c_uint = 0x80000000;
//
// struct hw_perf_event - performance event hardware details:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_perf_event {

    pub config: u64,
    pub config1: u64,
    pub last_tag: u64,
    pub dyn_constraint: u64,
    pub config_base: c_ulong,
    pub event_base: c_ulong,
    pub event_base_rdpmc: c_int,
    pub idx: c_int,
    pub last_cpu: c_int,
    pub flags: c_int,
    pub extra_reg: hw_perf_event_extra,
    pub branch_reg: hw_perf_event_extra,
}

//
// For AUX area events, aux_paused cannot be a state
// flag because it can be updated asynchronously to
// state.
//
// for tp_event->class

//
// Crufty hack to avoid the chicken and egg
// problem hw_breakpoint has with context
// creation and event initalization.
//

//
// If the event is a per task event, this will point to the task in
// question. See the comment in perf_event_alloc().
//
// PMU would store hardware filter configuration
// here.
//
// Last sync'ed generation of filters
//
// hw_perf_event::state flags; used to track the PERF_EF_* state.
//
// the counter is stopped
pub const PERF_HES_STOPPED: c_uint = 0x01;
// event->count up-to-date
pub const PERF_HES_UPTODATE: c_uint = 0x02;
pub const PERF_HES_ARCH: c_uint = 0x04;
//
// The last observed hardware counter value, updated with a
// local64_cmpxchg() such that pmu::read() can be called nested.
//
// The period to start the next sample with.
//
// The period we started this sample with.
//
// However much is left of the current period;
// note that this is a full 64bit value and
// allows for generation of periods longer
// than hardware might allow.
//
// State for throttling the event, see __perf_event_overflow() and
// perf_adjust_freq_unthr_context().
//
// State for freq target events, see __perf_event_overflow() and
// perf_adjust_freq_unthr_context().
//

//
// Common implementation detail of pmu::{start,commit,cancel}_txn
//
// txn to add/schedule event on PMU
pub const PERF_PMU_TXN_ADD: c_uint = 0x1;
// txn to read event group from PMU
pub const PERF_PMU_TXN_READ: c_uint = 0x2;
//
// pmu::capabilities flags
//
pub const PERF_PMU_CAP_NO_INTERRUPT: c_uint = 0x0001;
pub const PERF_PMU_CAP_NO_NMI: c_uint = 0x0002;
pub const PERF_PMU_CAP_AUX_NO_SG: c_uint = 0x0004;
pub const PERF_PMU_CAP_EXTENDED_REGS: c_uint = 0x0008;
pub const PERF_PMU_CAP_EXCLUSIVE: c_uint = 0x0010;
pub const PERF_PMU_CAP_ITRACE: c_uint = 0x0020;
pub const PERF_PMU_CAP_NO_EXCLUDE: c_uint = 0x0040;
pub const PERF_PMU_CAP_AUX_OUTPUT: c_uint = 0x0080;
pub const PERF_PMU_CAP_EXTENDED_HW_TYPE: c_uint = 0x0100;
pub const PERF_PMU_CAP_AUX_PAUSE: c_uint = 0x0200;
pub const PERF_PMU_CAP_AUX_PREFER_LARGE: c_uint = 0x0400;
pub const PERF_PMU_CAP_MEDIATED_VPMU: c_uint = 0x0800;
//
// pmu::scope
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum perf_pmu_scope {
    PERF_PMU_SCOPE_NONE = 0,
    PERF_PMU_SCOPE_CORE,
    PERF_PMU_SCOPE_DIE,
    PERF_PMU_SCOPE_CLUSTER,
    PERF_PMU_SCOPE_PKG,
    PERF_PMU_SCOPE_SYS_WIDE,
    PERF_PMU_MAX_SCOPE,
}

//
// struct pmu - generic performance monitoring unit
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmu {
    pub entry: list_head,
    pub events_lock: spinlock_t,
    pub events: list_head,
    pub module: *mut module,
    pub dev: *mut device,
    pub parent: *mut device,
    pub attr_groups: *const attribute_group,
    pub attr_update: *const attribute_group,
    pub name: *const c_char,
    pub type: c_int,
//
// various common per-pmu feature flags
//
    pub capabilities: c_int,
//
// PMU scope
//
    pub scope: c_uint,
    pub cpu_pmu_context: *mut *mut perf_cpu_pmu_context  __percpu,
    pub /: *mut *mut atomic_t exclusive_cnt; / < 0: cpu; > 0: tsk,
    pub task_ctx_nr: c_int,
    pub hrtimer_interval_ms: c_int,
// number of address filters this PMU can do
    pub nr_addr_filters: c_uint,
//
// Fully disable/enable this PMU, can be used to protect from the PMI
// as well as for lazy/batch writing of the MSRs.
//
    pub /: *mut *mut *mut *mut void (pmu_enable) (struct pmu pmu); / optional,
    pub /: *mut *mut *mut *mut void (pmu_disable) (struct pmu pmu); / optional,
//
// Try and initialize the event for this PMU.
//
// Returns:
// -ENOENT	-- @event is not for this PMU
//
// -ENODEV	-- @event is for this PMU but PMU not present
// -EBUSY	-- @event is for this PMU but PMU temporarily unavailable
// -EINVAL	-- @event is for this PMU but @event is not valid
// -EOPNOTSUPP -- @event is for this PMU, @event is valid, but not supported
// -EACCES	-- @event is for this PMU, @event is valid, but no privileges
//
// 0		-- @event is for this PMU and valid
//
// Other error return values are allowed.
//
    pub event): *mut *mut int (event_init) (struct perf_event,
//
// Notification that the event was mapped or unmapped.  Called
// in the context of the mapping task.
//
    pub /: *mut *mut *mut *mut *mut void (event_mapped) (struct perf_event event, struct mm_struct mm); / optional,
    pub /: *mut *mut *mut *mut *mut void (event_unmapped) (struct perf_event event, struct mm_struct mm); / optional,
//
// Flags for ->add()/->del()/ ->start()/->stop(). There are
// matching hw_perf_event::state flags.
//
// start the counter when adding
pub const PERF_EF_START: c_uint = 0x01;
// reload the counter when starting
pub const PERF_EF_RELOAD: c_uint = 0x02;
// update the counter when stopping
pub const PERF_EF_UPDATE: c_uint = 0x04;
// AUX area event, pause tracing
pub const PERF_EF_PAUSE: c_uint = 0x08;
// AUX area event, resume tracing
pub const PERF_EF_RESUME: c_uint = 0x10;
//
// Adds/Removes a counter to/from the PMU, can be done inside a
// transaction, see the ->*_txn() methods.
//
// The add/del callbacks will reserve all hardware resources required
// to service the event, this includes any counter constraint
// scheduling etc.
//
// Called with IRQs disabled and the PMU disabled on the CPU the event
// is on.
//
// ->add() called without PERF_EF_START should result in the same state
// as ->add() followed by ->stop().
//
// ->del() must always PERF_EF_UPDATE stop an event. If it calls
// ->stop() that must deal with already being stopped without
// PERF_EF_UPDATE.
//
    pub flags): *mut *mut *mut int (add) (struct perf_event event, int,
    pub flags): *mut *mut *mut void (del) (struct perf_event event, int,
//
// Starts/Stops a counter present on the PMU.
//
// The PMI handler should stop the counter when perf_event_overflow()
// returns !0. ->start() will be used to continue.
//
// Also used to change the sample period.
//
// Called with IRQs disabled and the PMU disabled on the CPU the event
// is on -- will be called from NMI context with the PMU generates
// NMIs.
//
// ->stop() with PERF_EF_UPDATE will read the counter and update
// period/count values like ->read() would.
//
// ->start() with PERF_EF_RELOAD will reprogram the counter
// value, must be preceded by a ->stop() with PERF_EF_UPDATE.
//
// ->stop() with PERF_EF_PAUSE will stop as simply as possible. Will not
// overlap another ->stop() with PERF_EF_PAUSE nor ->start() with
// PERF_EF_RESUME.
//
// ->start() with PERF_EF_RESUME will start as simply as possible but
// only if the counter is not otherwise stopped. Will not overlap
// another ->start() with PERF_EF_RESUME nor ->stop() with
// PERF_EF_PAUSE.
//
// Notably, PERF_EF_PAUSE/PERF_EF_RESUME *can* be concurrent with other
// ->stop()/->start() invocations, just not itself.
//
    pub flags): *mut *mut *mut void (start) (struct perf_event event, int,
    pub flags): *mut *mut *mut void (stop) (struct perf_event event, int,
//
// Updates the counter value of the event.
//
// For sampling capable PMUs this will also update the software period
// hw_perf_event::period_left field.
//
    pub event): *mut *mut void (read) (struct perf_event,
//
// Group events scheduling is treated as a transaction, add
// group events as a whole and perform one schedulability test.
// If the test fails, roll back the whole group
//
// Start the transaction, after this ->add() doesn't need to
// do schedulability tests.
//
// Optional.
//
    pub txn_flags): *mut *mut *mut void (start_txn) (struct pmu pmu, unsigned int,
//
// If ->start_txn() disabled the ->add() schedulability test
// then ->commit_txn() is required to perform one. On success
// the transaction is closed. On error the transaction is kept
// open until ->cancel_txn() is called.
//
// Optional.
//
    pub pmu): *mut *mut int (commit_txn) (struct pmu,
//
// Will cancel the transaction, assumes ->del() is called
// for each successful ->add() during the transaction.
//
// Optional.
//
    pub pmu): *mut *mut void (cancel_txn) (struct pmu,
//
// Will return the value for perf_event_mmap_page::index for this event,
// if no implementation is provided it will default to 0 (see
// perf_event_idx_default).
//
    pub /: *mut *mut *mut *mut int (event_idx) (struct perf_event event); /optional,
//
// context-switches callback
//
    pub sched_in): *mut *mut task_task, bool,
//
// Kmem cache of PMU specific data
//
    pub task_ctx_cache: *mut kmem_cache,
//
// Set up pmu-private data structures for an AUX area
//
    pub overwrite): int nr_pages, bool,
// optional
//
// Free pmu-private AUX data structures
//
    pub /: *mut *mut *mut *mut void (free_aux) (void aux); / optional,
//
// Take a snapshot of the AUX buffer without touching the event
// state, so that preempting ->start()/->stop() callbacks does
// not interfere with their logic. Called in PMI context.
//
// Returns the size of AUX data copied to the output handle.
//
// Optional.
//
    pub size): c_ulong,
//
// Validate address range filters: make sure the HW supports the
// requested configuration and number of filters; return 0 if the
// supplied filters are valid, -errno otherwise.
//
// Runs in the context of the ioctl()ing process and is not serialized
// with the rest of the PMU callbacks.
//
    pub filters): *mut *mut int (addr_filters_validate) (struct list_head,
// optional
//
// Synchronize address range filter configuration:
// translate hw-agnostic filters into hardware configuration in
// event::hw::addr_filters.
//
// Runs as a part of filter sync sequence that is done in ->start()
// callback by calling perf_event_addr_filters_sync().
//
// May (and should) traverse event::addr_filters::list, for which its
// caller provides necessary serialization.
//
    pub event): *mut *mut void (addr_filters_sync) (struct perf_event,
// optional
//
// Check if event can be used for aux_output purposes for
// events of this PMU.
//
// Runs from perf_event_open(). Should return 0 for "no match"
// or non-zero for "match".
//
    pub event): *mut *mut int (aux_output_match) (struct perf_event,
// optional
//
// Skip programming this PMU on the given CPU. Typically needed for
// big.LITTLE things.
//
    pub /: *mut *mut *mut *mut bool (filter) (struct pmu pmu, int cpu); / optional,
//
// Check period value for PERF_EVENT_IOC_PERIOD ioctl.
//
    pub /: *mut *mut *mut *mut int (check_period) (struct perf_event event, u64 value); / optional,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum perf_addr_filter_action_t {
    PERF_ADDR_FILTER_ACTION_STOP = 0,
    PERF_ADDR_FILTER_ACTION_START,
    PERF_ADDR_FILTER_ACTION_FILTER,
}

//
// struct perf_addr_filter - address range filter definition
// @entry:	event's filter list linkage
// @path:	object file's path for file-based filters
// @offset:	filter range offset
// @size:	filter range size (size==0 means single address trigger)
// @action:	filter/start/stop
//
// This is a hardware-agnostic filter configuration as specified by the user.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_addr_filter {
    pub entry: list_head,
    pub path: path,
    pub offset: c_ulong,
    pub size: c_ulong,
    pub action: perf_addr_filter_action_t,
}

//
// struct perf_addr_filters_head - container for address range filters
// @list:	list of filters for this event
// @lock:	spinlock that serializes accesses to the @list and event's
// (and its children's) filter generations.
// @nr_file_filters:	number of file-based filters
//
// A child event will use parent's @list (and therefore @lock), so they are
// bundled together; see perf_event_addr_filters().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_addr_filters_head {
    pub list: list_head,
    pub lock: raw_spinlock_t,
    pub nr_file_filters: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_addr_filter_range {
    pub start: c_ulong,
    pub size: c_ulong,
}

//
// The normal states are:
//
// ACTIVE    --.
// ^        |
// |        |
// sched_{in,out}() |
// |        |
// v        |
// ,---> INACTIVE  --+ <-.
// |                 |   |
// |                {dis,en}able()
// sched_in()           |   |
// |       OFF    <--' --+
// |                     |
// `--->  ERROR    ------'
//
// That is:
//
// sched_in:       INACTIVE          -> {ACTIVE,ERROR}
// sched_out:      ACTIVE            -> INACTIVE
// disable:        {ACTIVE,INACTIVE} -> OFF
// enable:         {OFF,ERROR}       -> INACTIVE
//
// Where {OFF,ERROR} are disabled states.
//
// Then we have the {EXIT,REVOKED,DEAD} states which are various shades of
// defunct events:
//
// - EXIT means task that the even was assigned to died, but child events
// still live, and further children can still be created. But the event
// itself will never be active again. It can only transition to
// {REVOKED,DEAD};
//
// - REVOKED means the PMU the event was associated with is gone; all
// functionality is stopped but the event is still alive. Can only
// transition to DEAD;
//
// - DEAD event really is DYING tearing down state and freeing bits.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum perf_event_state {
    PERF_EVENT_STATE_DEAD		= -5,
    PERF_EVENT_STATE_REVOKED	= -4, /* pmu gone, must not touch */
    PERF_EVENT_STATE_EXIT		= -3, /* task died, still inherit */
    PERF_EVENT_STATE_ERROR		= -2, /* scheduling error, can enable */
    PERF_EVENT_STATE_OFF		= -1,
    PERF_EVENT_STATE_INACTIVE	=  0,
    PERF_EVENT_STATE_ACTIVE		=  1,
}

//
// Event capabilities. For event_caps and groups caps.
//
// PERF_EV_CAP_SOFTWARE: Is a software event.
// PERF_EV_CAP_READ_ACTIVE_PKG: A CPU event (or cgroup event) that can be read
// from any CPU in the package where it is active.
// PERF_EV_CAP_SIBLING: An event with this flag must be a group sibling and
// cannot be a group leader. If an event with this flag is detached from the
// group it is scheduled out and moved into an unrecoverable ERROR state.
// PERF_EV_CAP_READ_SCOPE: A CPU event that can be read from any CPU of the
// PMU scope where it is active.
//

pub const SWEVENT_HLIST_BITS: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct swevent_hlist {
    pub heads: [hlist_head; SWEVENT_HLIST_SIZE],
    pub rcu_head: rcu_head,
}

pub const PERF_ATTACH_CONTEXT: c_uint = 0x0001;
pub const PERF_ATTACH_GROUP: c_uint = 0x0002;
pub const PERF_ATTACH_TASK: c_uint = 0x0004;
pub const PERF_ATTACH_TASK_DATA: c_uint = 0x0008;
pub const PERF_ATTACH_GLOBAL_DATA: c_uint = 0x0010;
pub const PERF_ATTACH_SCHED_CB: c_uint = 0x0020;
pub const PERF_ATTACH_CHILD: c_uint = 0x0040;
pub const PERF_ATTACH_EXCLUSIVE: c_uint = 0x0080;
pub const PERF_ATTACH_CALLCHAIN: c_uint = 0x0100;
pub const PERF_ATTACH_ITRACE: c_uint = 0x0200;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmu_event_list {
    pub lock: raw_spinlock_t,
    pub list: list_head,
}

//
// event->sibling_list is modified whole holding both ctx->lock and ctx->mutex
// as such iteration must hold either lock. However, since ctx->lock is an IRQ
// safe lock, and is only held by the CPU doing the modification, having IRQs
// disabled is sufficient since it will hold-off the IPIs.
//

//
// struct perf_event - performance event kernel representation:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_event {

//
// entry onto perf_event_context::event_list;
// modifications require ctx->lock
// RCU safe iterations.
//
    pub event_entry: list_head,
//
// Locked for modification by both ctx->mutex and ctx->lock; holding
// either sufficies for read.
//
    pub sibling_list: list_head,
    pub active_list: list_head,
//
// Node on the pinned or flexible tree located at the event context;
//
    pub group_node: rb_node,
    pub group_index: u64,
//
// We need storage to track the entries in perf_pmu_migrate_context; we
// cannot use the event_entry because of RCU and we want to keep the
// group in tact which avoids us using the other two entries.
//
    pub migrate_entry: list_head,
    pub hlist_entry: hlist_node,
    pub active_entry: list_head,
    pub nr_siblings: c_int,
// Not serialized. Only written during event initialization.
    pub event_caps: c_int,
// The cumulative AND of all event_caps for events in this group.
    pub group_caps: c_int,
    pub group_generation: c_uint,
    pub group_leader: *mut perf_event,
//
// event->pmu will always point to pmu in which this event belongs.
// Whereas event->pmu_ctx->pmu may point to other pmu when group of
// different pmu events is created.
//
    pub pmu: *mut pmu,
    pub pmu_private: *mut c_void,
    pub state: perf_event_state,
    pub attach_state: c_uint,
    pub count: local64_t,
    pub child_count: core::sync::atomic::AtomicI64,
//
// These are the total time in nanoseconds that the event
// has been enabled (i.e. eligible to run, and the task has
// been scheduled in, if this is a per-task event)
// and running (scheduled onto the CPU), respectively.
//
    pub total_time_enabled: u64,
    pub total_time_running: u64,
    pub tstamp: u64,
    pub attr: perf_event_attr,
    pub header_size: u16,
    pub id_header_size: u16,
    pub read_size: u16,
    pub hw: hw_perf_event,
    pub ctx: *mut perf_event_context,
//
// event->pmu_ctx points to perf_event_pmu_context in which the event
// is added. This pmu_ctx can be of other pmu for sw event when that
// sw event is part of a group which also contains non-sw events.
//
    pub pmu_ctx: *mut perf_event_pmu_context,
    pub refcount: atomic_long_t,
//
// These accumulate total time (in nanoseconds) that children
// events have been enabled and running, respectively.
//
    pub child_total_time_enabled: core::sync::atomic::AtomicI64,
    pub child_total_time_running: core::sync::atomic::AtomicI64,
//
// Protect attach/detach and child_list:
//
    pub child_mutex: mutex,
    pub child_list: list_head,
    pub parent: *mut perf_event,
    pub oncpu: c_int,
    pub cpu: c_int,
    pub owner_entry: list_head,
    pub owner: *mut task_struct,
// mmap bits
    pub mmap_mutex: mutex,
    pub mmap_count: refcount_t,
    pub rb: *mut perf_buffer,
    pub rb_entry: list_head,
    pub rcu_batches: c_ulong,
    pub rcu_pending: c_int,
// poll related
    pub waitq: wait_queue_head_t,
    pub fasync: *mut fasync_struct,
// delayed work for NMIs and such
    pub pending_wakeup: c_uint,
    pub pending_kill: c_uint,
    pub pending_disable: c_uint,
    pub /: *mut *mut unsigned long pending_addr; / SIGTRAP,
    pub pending_irq: irq_work,
    pub pending_disable_irq: irq_work,
    pub pending_task: callback_head,
    pub pending_work: c_uint,
    pub event_limit: core::sync::atomic::AtomicI32,
// address range filters
    pub addr_filters: perf_addr_filters_head,
// vma address array for file-based filders
    pub addr_filter_ranges: *mut perf_addr_filter_range,
    pub addr_filters_gen: c_ulong,
// for aux_output events
    pub aux_event: *mut perf_event,
    pub ): *mut *mut void (destroy)(struct perf_event,
    pub rcu_head: rcu_head,
    pub ns: *mut pid_namespace,
    pub id: u64,
    pub lost_samples: core::sync::atomic::AtomicI64,
    pub (*clock)(void): *mut u64,
    pub overflow_handler: perf_overflow_handler_t,
    pub overflow_handler_context: *mut c_void,
    pub prog: *mut bpf_prog,
    pub bpf_cookie: u64,

    pub tp_event: *mut trace_event_call,
    pub filter: *mut event_filter,

    pub ftrace_ops: ftrace_ops,

    pub /: *mut *mut *mut perf_cgroup cgrp; / cgroup event is attach to,

    pub security: *mut c_void,

    pub sb_list: list_head,
    pub pmu_list: list_head,
//
// Certain events gets forwarded to another pmu internally by over-
// writing kernel copy of event->attr.type without user being aware
// of it. event->orig_type contains original 'type' requested by
// user.
//
    pub orig_type: u32,

}

//
// ,-----------------------[1:n]------------------------.
// V                                                    V
// perf_event_context <-[1:n]-> perf_event_pmu_context <-[1:n]- perf_event
// |                       |
// `--[n:1]-> pmu <-[1:n]--'
//
// struct perf_event_pmu_context  lifetime is refcount based and RCU freed
// (similar to perf_event_context). Locking is as if it were a member of
// perf_event_context; specifically:
//
// modification, both: ctx->mutex && ctx->lock
// reading, either:    ctx->mutex || ctx->lock
//
// There is one exception to this; namely put_pmu_ctx() isn't always called
// with ctx->mutex held; this means that as long as we can guarantee the epc
// has events the above rules hold.
//
// Specificially, sys_perf_event_open()'s group_leader case depends on
// ctx->mutex pinning the configuration. Since we hold a reference on
// group_leader (through the filedesc) it can't go away, therefore it's
// associated pmu_ctx must exist and cannot change due to ctx->mutex.
//
// perf_event holds a refcount on perf_event_context
// perf_event holds a refcount on perf_event_pmu_context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_event_pmu_context {
    pub pmu: *mut pmu,
    pub ctx: *mut perf_event_context,
    pub pmu_ctx_entry: list_head,
    pub pinned_active: list_head,
    pub flexible_active: list_head,
// Used to identify the per-cpu perf_event_pmu_context
    pub 1: unsigned int embedded :,
    pub nr_events: c_uint,
    pub nr_cgroups: c_uint,
    pub nr_freq: c_uint,
    pub /: *mut *mut atomic_t refcount; / event <-> epc,
    pub rcu_head: rcu_head,
//
// Set when one or more (plausibly active) event can't be scheduled
// due to pmu overcommit or pmu constraints, except tolerant to
// events not necessary to be active due to scheduling constraints,
// such as cgroups.
//
    pub rotate_necessary: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_event_groups {
    pub tree: rb_root,
    pub index: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_time_ctx {
    pub time: u64,
    pub stamp: u64,
    pub offset: u64,
}

//
// struct perf_event_context - event context structure
//
// Used as a container for task events and CPU events as well:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_event_context {
//
// Protect the states of the events in the list,
// nr_active, and the list:
//
    pub lock: raw_spinlock_t,
//
// Protect the list of events.  Locking either mutex or lock
// is sufficient to ensure the list doesn't change; to change
// the list you need to lock both the mutex and the spinlock.
//
    pub mutex: mutex,
    pub pmu_ctx_list: list_head,
    pub pinned_groups: perf_event_groups,
    pub flexible_groups: perf_event_groups,
    pub event_list: list_head,
    pub nr_events: c_int,
    pub nr_user: c_int,
    pub is_active: c_int,
    pub nr_stat: c_int,
    pub nr_freq: c_int,
    pub rotate_disable: c_int,
    pub /: *mut *mut refcount_t refcount; / event <-> ctx,
    pub task: *mut task_struct,
//
// Context clock, runs when context enabled.
//
    pub time: perf_time_ctx,
//
// Context clock, runs when in the guest mode.
//
    pub timeguest: perf_time_ctx,
//
// These fields let us detect when two contexts have both
// been cloned (inherited) from a common ancestor.
//
    pub parent_ctx: *mut perf_event_context,
    pub parent_gen: u64,
    pub generation: u64,
    pub pin_count: c_int,

    pub /: *mut *mut int nr_cgroups; / cgroup evts,

    pub rcu_head: rcu_head,
//
// The count of events for which using the switch-out fast path
// should be avoided.
//
// Sum (event->pending_work + events with
// (attr->inherit && (attr->sample_type & PERF_SAMPLE_READ)))
//
// The SIGTRAP is targeted at ctx->task, as such it won't do changing
// that until the signal is delivered.
//
    pub nr_no_switch_fast: local_t,
}

//
// struct perf_ctx_data - PMU specific data for a task
// @rcu_head:  To avoid the race on free PMU specific data
// @refcount:  To track users
// @global:    To track system-wide users
// @ctx_cache: Kmem cache of PMU specific data
// @data:      PMU specific data
//
// Currently, the struct is only used in Intel LBR call stack mode to
// save/restore the call stack of a task on context switches.
//
// The rcu_head is used to prevent the race on free the data.
// The data only be allocated when Intel LBR call stack mode is enabled.
// The data will be freed when the mode is disabled.
// The content of the data will only be accessed in context switch, which
// should be protected by rcu_read_lock().
//
// Because of the alignment requirement of Intel Arch LBR, the Kmem cache
// is used to allocate the PMU specific data. The ctx_cache is to track
// the Kmem cache.
//
// Careful: Struct perf_ctx_data is added as a pointer in struct task_struct.
// When system-wide Intel LBR call stack mode is enabled, a buffer with
// constant size will be allocated for each task.
// Also, system memory consumption can further grow when the size of
// struct perf_ctx_data enlarges.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_ctx_data {
    pub rcu_head: rcu_head,
    pub refcount: refcount_t,
    pub global: c_int,
    pub ctx_cache: *mut kmem_cache,
    pub data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_cpu_pmu_context {
    pub epc: perf_event_pmu_context,
    pub task_epc: *mut perf_event_pmu_context,
    pub sched_cb_entry: list_head,
    pub sched_cb_usage: c_int,
    pub active_oncpu: c_int,
    pub exclusive: c_int,
    pub pmu_disable_count: c_int,
    pub hrtimer_lock: raw_spinlock_t,
    pub hrtimer: hrtimer,
    pub hrtimer_interval: ktime_t,
    pub hrtimer_active: c_uint,
}

//
// struct perf_event_cpu_context - per cpu event context structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_cpu_context {
    pub ctx: perf_event_context,
    pub task_ctx: *mut perf_event_context,
    pub online: c_int,

    pub cgrp: *mut perf_cgroup,

//
// Per-CPU storage for iterators used in visit_groups_merge. The default
// storage is of size 2 to hold the CPU and any CPU event iterators.
//
    pub heap_size: c_int,
    pub heap: *mut perf_event,
    pub heap_default: [*mut perf_event; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_output_handle {
    pub event: *mut perf_event,
    pub rb: *mut perf_buffer,
    pub wakeup: c_ulong,
    pub size: c_ulong,
    pub /: *mut *mut *mut u64 flags; / perf_output(),
    pub /: *mut *mut *mut u64 aux_flags; / perf_aux_output(),
    pub 1: u64 skip_read :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_perf_event_data_kern {
    pub regs: *mut bpf_user_pt_regs_t,
    pub data: *mut perf_sample_data,
    pub event: *mut perf_event,
}

//
// perf_cgroup_info keeps track of time_enabled for a cgroup.
// This is a per-cpu dynamically allocated data structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_cgroup_info {
    pub time: perf_time_ctx,
    pub timeguest: perf_time_ctx,
    pub active: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_cgroup {
    pub css: cgroup_subsys_state,
    pub info: *mut perf_cgroup_info __percpu,
}

//
// Must ensure cgroup is pinned (css_get) before calling
// this function. In other words, we cannot call this function
// if there is no cgroup event for the current CPU context.
//

extern "C" {
    pub fn perf_aux_output_flag(handle: *mut perf_output_handle, flags: u64);
}
extern "C" {
    pub fn perf_event_itrace_started(event: *mut perf_event);
}
extern "C" {
    pub fn perf_pmu_register(pmu: *mut pmu, name: *const c_char, type: c_int) -> c_int;
}
extern "C" {
    pub fn perf_pmu_unregister(pmu: *mut pmu) -> c_int;
}
extern "C" {
    pub fn perf_event_init_task(child: *mut task_struct, clone_flags: u64) -> c_int;
}
extern "C" {
    pub fn perf_event_exit_task(child: *mut task_struct);
}
extern "C" {
    pub fn perf_event_free_task(task: *mut task_struct);
}
extern "C" {
    pub fn perf_event_delayed_put(task: *mut task_struct);
}
extern "C" {
    pub fn perf_event_print_debug();
}
extern "C" {
    pub fn perf_pmu_disable(pmu: *mut pmu);
}
extern "C" {
    pub fn perf_pmu_enable(pmu: *mut pmu);
}
extern "C" {
    pub fn perf_sched_cb_dec(pmu: *mut pmu);
}
extern "C" {
    pub fn perf_sched_cb_inc(pmu: *mut pmu);
}
extern "C" {
    pub fn perf_event_task_disable() -> c_int;
}
extern "C" {
    pub fn perf_event_task_enable() -> c_int;
}
extern "C" {
    pub fn perf_pmu_resched(pmu: *mut pmu);
}
extern "C" {
    pub fn perf_event_refresh(event: *mut perf_event, refresh: c_int) -> c_int;
}
extern "C" {
    pub fn perf_event_update_userpage(event: *mut perf_event);
}
extern "C" {
    pub fn perf_event_release_kernel(event: *mut perf_event) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_sample_data {
//
// Fields set by perf_sample_data_init() unconditionally,
// group so as to minimize the cachelines touched.
//
    pub sample_flags: u64,
    pub period: u64,
    pub dyn_size: u64,
//
// Fields commonly set by __perf_event_header__init_id(),
// group so as to minimize the cachelines touched.
//
    pub type: u64,
    pub pid: u32,
    pub tid: u32,
    pub tid_entry: },
    pub time: u64,
    pub id: u64,
    pub cpu: u32,
    pub reserved: u32,
    pub cpu_entry: },
//
// The other fields, optionally {set,used} by
// perf_{prepare,output}_sample().
//
    pub ip: u64,
    pub callchain: *mut perf_callchain_entry,
    pub raw: *mut perf_raw_record,
    pub br_stack: *mut perf_branch_stack,
    pub br_stack_cntr: *mut u64,
    pub weight: perf_sample_weight,
    pub data_src: perf_mem_data_src,
    pub txn: u64,
    pub regs_user: perf_regs,
    pub regs_intr: perf_regs,
    pub stack_user_size: u64,
    pub stream_id: u64,
    pub cgroup: u64,
    pub addr: u64,
    pub phys_addr: u64,
    pub data_page_size: u64,
    pub code_page_size: u64,
    pub aux_size: u64,
    pub ____cacheline_aligned: },
// default value for data source

// remaining struct members initialized in perf_prepare_sample()
    pub PERF_SAMPLE_PERIOD: data->sample_flags =,
    pub period: data->period =,
    pub 0: data->dyn_size =,
    pub addr: data->addr =,
    pub PERF_SAMPLE_ADDR: data->sample_flags |=,
    pub 1: int size =,
    pub regs): data->callchain = perf_callchain(event,,
    pub data->callchain->nr: size +=,
    pub sizeof(u64): *mut *mut data->dyn_size += size,
    pub PERF_SAMPLE_CALLCHAIN: data->sample_flags |=,
    pub &raw->frag: *mut *mut perf_raw_frag frag =,
    pub 0: u32 sum =,
    pub size: c_int,
    pub frag->size: sum +=,
    pub frag->next: frag =,
    pub (1): } while,
    pub sizeof(u64)): size = round_up(sum + sizeof(u32),,
    pub sizeof(u32): raw->size = size -,
    pub sum: frag->pad = raw->size -,
    pub raw: data->raw =,
    pub size: data->dyn_size +=,
    pub PERF_SAMPLE_RAW: data->sample_flags |=,
    pub PERF_SAMPLE_BRANCH_STACK: return event->attr.sample_type &,
    pub /: *mut *mut int size = sizeof(u64); / nr,
    pub sizeof(u64): size +=,
    pub brs->nr): brs->nr = min_t(u16, event->attr.sample_max_stack,,
    pub perf_branch_entry): *mut *mut size += brs->nr  sizeof(struct,
//
// The extension space for counters is appended after the
// struct perf_branch_stack. It is used to store the occurrences
// of events of each branch.
//
    pub sizeof(u64): *mut *mut size += brs->nr,
    pub brs: data->br_stack =,
    pub brs_cntr: data->br_stack_cntr =,
    pub size: data->dyn_size +=,
    pub PERF_SAMPLE_BRANCH_STACK: data->sample_flags |=,
    pub perf_event_header): u32 size = sizeof(struct,
    pub event->id_header_size: size += event->header_size +,
    pub data->dyn_size: size +=,
    pub size: return,
//
// Clear all bitfields in the perf_branch_entry.
// The to and from fields are not cleared because they are
// systematically modified by caller.
//
    pub 0: br->mispred =,
    pub 0: br->predicted =,
    pub 0: br->in_tx =,
    pub 0: br->abort =,
    pub 0: br->cycles =,
    pub 0: br->type =,
    pub PERF_BR_SPEC_NA: br->spec =,
    pub 0: br->reserved =,
    pub event): *mut perf_event,
    pub regs): *mut pt_regs,
    pub regs): *mut pt_regs,
    pub regs): *mut pt_regs,
    pub regs): *mut pt_regs,
    pub regs): *mut pt_regs,
    pub regs): *mut pt_regs,
    pub event->overflow_handler: perf_overflow_handler_t overflow_handler =,
    pub true: return,
    pub true: return,
    pub false: return,
    pub event): *mut perf_event,
    pub sample): *mut perf_sample_data,
    pub lost): *mut *mut perf_log_lost_samples(struct perf_event event, u64,
    pub &event->attr: *mut *mut perf_event_attr attr =,
    pub attr->exclude_host: attr->exclude_guest ||,
    pub 0: return event->attr.sample_period !=,
//
// Return 1 for a software event, 0 for a hardware event
//
    pub PERF_EV_CAP_SOFTWARE: return event->event_caps &,
//
// Return 1 for event in sw context, 0 for event in hw context
//
    pub perf_sw_context: return event->pmu_ctx->pmu->task_ctx_nr ==,
    pub PERF_PMU_CAP_EXCLUSIVE: return pmu->capabilities &,
    pub perf_swevent_enabled: [extern struct static_key; PERF_COUNT_SW_MAX],
    pub u64): *mut *mut extern void ___perf_sw_event(u32, u64, struct pt_regs ,,
    pub u64): *mut *mut extern void __perf_sw_event(u32, u64, struct pt_regs ,,

//
// When generating a perf sample in-line, instead of from an interrupt
// exception, we lack a pt_regs. This is typically used from software events
// like: SW_CONTEXT_SWITCHES, SW_MIGRATIONS and the tie-in with tracepoints.
//
// We typically don't need a full set, but (for x86) do require:
// - ip for PERF_SAMPLE_IP
// - cs for user_mode() tests
// - sp for PERF_SAMPLE_CALLCHAIN
// - eflags for MISC bits and CALLCHAIN (see: perf_hw_regs())
//
// NOTE: assumes @regs is otherwise already 0 filled; this is important for
// things like PERF_SAMPLE_REGS_INTR.
//
    pub CALLER_ADDR0): perf_arch_fetch_caller_regs(regs,,
    pub addr): __perf_sw_event(event_id, nr, regs,,
    pub __perf_regs[4]): DECLARE_PER_CPU(struct pt_regs,,
//
// 'Special' version for the scheduler, it hard assumes no recursion,
// which is guaranteed by us not actually scheduling inside other swevents
// because those disable preemption.
//
    pub this_cpu_ptr(&__perf_regs[0]): *mut *mut pt_regs regs =,
    pub addr): ___perf_sw_event(event_id, nr, regs,,
    pub perf_sched_events: extern struct static_key_false,
    pub static_key_false(&perf_swevent_enabled[swevt]): return,
    pub 1: task->sched_migrated =,
    pub task): __perf_event_task_sched_in(prev,,
    pub 0): __perf_sw_event_sched(PERF_COUNT_SW_CPU_MIGRATIONS, 1,,
    pub 0: task->sched_migrated =,
    pub 0): __perf_sw_event_sched(PERF_COUNT_SW_CONTEXT_SWITCHES, 1,,

    pub 0): __perf_sw_event_sched(PERF_COUNT_SW_CGROUP_SWITCHES, 1,,

    pub next): __perf_event_task_sched_out(prev,,
    pub vma): *mut extern void perf_event_mmap(struct vm_area_struct,
    pub sym): *const bool unregister, char,
    pub flags): u16,
pub const PERF_GUEST_ACTIVE: c_uint = 0x01;
pub const PERF_GUEST_USER: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_guest_info_callbacks {
    pub (*state)(void): *mut c_uint,
    pub (*get_ip)(void): *mut c_ulong,
    pub (*handle_intel_pt_intr)(void): *mut c_uint,
    pub (*handle_mediated_pmi)(void): *mut c_void,
}

extern "C" {
    pub fn static_call(_arg: __perf_guest_state)() -> return;
}
extern "C" {
    pub fn static_call(_arg: __perf_guest_get_ip)() -> return;
}
extern "C" {
    pub fn static_call(_arg: __perf_guest_handle_intel_pt_intr)() -> return;
}
extern "C" {
    pub fn perf_register_guest_info_callbacks(cbs: *mut perf_guest_info_callbacks);
}
extern "C" {
    pub fn perf_unregister_guest_info_callbacks(cbs: *mut perf_guest_info_callbacks);
}

extern "C" {
    pub fn perf_event_exec();
}
extern "C" {
    pub fn perf_event_comm(tsk: *mut task_struct, exec: bool);
}
extern "C" {
    pub fn perf_event_namespaces(tsk: *mut task_struct);
}
extern "C" {
    pub fn perf_event_fork(tsk: *mut task_struct);
}
// Callchains
extern "C" {
    pub fn perf_callchain_user(entry: *mut perf_callchain_entry_ctx, regs: *mut pt_regs);
}
extern "C" {
    pub fn perf_callchain_kernel(entry: *mut perf_callchain_entry_ctx, regs: *mut pt_regs);
}
extern "C" {
    pub fn get_callchain_buffers(max_stack: c_int) -> c_int;
}
extern "C" {
    pub fn put_callchain_buffers();
}
extern "C" {
    pub fn put_callchain_entry(rctx: c_int);
}
extern "C" {
    pub fn perf_sample_event_took(sample_len_ns: u64);
}
// Access to perf_event_open(2) syscall.
pub const PERF_SECURITY_OPEN: c_int = 0;
// Finer grained perf_event_open(2) access control.
pub const PERF_SECURITY_CPU: c_int = 1;
pub const PERF_SECURITY_KERNEL: c_int = 2;
pub const PERF_SECURITY_TRACEPOINT: c_int = 3;
extern "C" {
    pub fn perf_allow_kernel() -> c_int;
}
extern "C" {
    pub fn perf_allow_cpu() -> c_int;
}
extern "C" {
    pub fn perf_allow_tracepoint() -> c_int;
}
extern "C" {
    pub fn perf_exclude_event(event: *mut perf_event, regs: *mut pt_regs) -> c_int;
}
extern "C" {
    pub fn perf_event_init();
}
extern "C" {
    pub fn perf_bp_event(event: *mut perf_event, data: *mut c_void);
}
extern "C" {
    pub fn perf_misc_flags(event: *mut perf_event, regs: *mut pt_regs) -> c_ulong;
}

//
// An inherited event uses parent's filters
//
// Only the parent has fasync state
extern "C" {
    pub fn perf_event_addr_filters_sync(event: *mut perf_event);
}
extern "C" {
    pub fn perf_report_aux_output_id(event: *mut perf_event, hw_id: u64);
}
extern "C" {
    pub fn perf_output_end(handle: *mut perf_output_handle);
}
extern "C" {
    pub fn perf_swevent_get_recursion_context() -> c_int;
}
extern "C" {
    pub fn perf_swevent_put_recursion_context(rctx: c_int);
}
extern "C" {
    pub fn perf_swevent_set_period(event: *mut perf_event) -> u64;
}
extern "C" {
    pub fn perf_event_enable(event: *mut perf_event);
}
extern "C" {
    pub fn perf_event_disable(event: *mut perf_event);
}
extern "C" {
    pub fn perf_event_disable_local(event: *mut perf_event);
}
extern "C" {
    pub fn perf_event_disable_inatomic(event: *mut perf_event);
}
extern "C" {
    pub fn perf_event_task_tick();
}
extern "C" {
    pub fn perf_event_account_interrupt(event: *mut perf_event) -> c_int;
}
extern "C" {
    pub fn perf_event_period(event: *mut perf_event, value: u64) -> c_int;
}
extern "C" {
    pub fn perf_event_pause(event: *mut perf_event, reset: bool) -> u64;
}

extern "C" {
    pub fn perf_create_mediated_pmu() -> c_int;
}
extern "C" {
    pub fn perf_release_mediated_pmu();
}
extern "C" {
    pub fn perf_load_guest_context();
}
extern "C" {
    pub fn perf_put_guest_context();
}

extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}
extern "C" {
    pub fn int(name: *mut perf_ksymbol_get_name_f)(char, name_len: c_int, data: *mut c_void) -> typedef;
}

extern "C" {
    pub fn perf_restore_debug_store();
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_pmu_events_attr {
    pub attr: device_attribute,
    pub id: u64,
    pub event_str: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_pmu_events_ht_attr {
    pub attr: device_attribute,
    pub id: u64,
    pub event_str_ht: *const c_char,
    pub event_str_noht: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_pmu_events_hybrid_attr {
    pub attr: device_attribute,
    pub id: u64,
    pub event_str: *const c_char,
    pub pmu_type: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_pmu_format_hybrid_attr {
    pub attr: device_attribute,
    pub pmu_type: u64,
}

// Performance counter hotplug functions

extern "C" {
    pub fn perf_event_init_cpu(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn perf_event_exit_cpu(cpu: c_uint) -> c_int;
}

//
// Snapshot branch stack on software events.
//
// Branch stack can be very useful in understanding software events. For
// example, when a long function, e.g. sys_perf_event_open, returns an
// errno, it is not obvious why the function failed. Branch stack could
// provide very helpful information in this type of scenarios.
//
// On software event, it is necessary to stop the hardware branch recorder
// fast. Otherwise, the hardware register/buffer will be flushed with
// entries of the triggering event. Therefore, static call is used to
// stop the hardware recorder.
//
// cnt is the number of entries allocated for entries.
// Return number of entries copied to .
//

