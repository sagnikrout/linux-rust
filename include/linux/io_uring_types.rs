//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/io_uring_types.h
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
// A hint to not wake right away but delay until there are enough of
// tw's queued to match the number of CQEs the task is waiting for.
//
// Must not be used with requests generating more than one CQE.
// It's also ignored unless IORING_SETUP_DEFER_TASKRUN is set.
//
// Set when task_work is queued from a waitqueue wakeup handler, where
// an arbitrary provider waitqueue lock is held. Signaling the CQ ring
// eventfd inline from there can recurse back into that lock through
// epoll, so the eventfd signal must be deferred.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum io_uring_cmd_flags {
    IO_URING_F_COMPLETE_DEFER	= 1,
    IO_URING_F_UNLOCKED		= 2,
// the request is executed from poll, it should not be freed
    IO_URING_F_MULTISHOT		= 4,
// executed by io-wq
    IO_URING_F_IOWQ			= 8,
// executed inline from syscall
    IO_URING_F_INLINE		= 16,
// int's last bit, sign checks are usually faster than a bit test
    IO_URING_F_NONBLOCK		= INT_MIN,

// ctx state flags, for URING_CMD
    IO_URING_F_SQE128		= (1 << 8),
    IO_URING_F_CQE32		= (1 << 9),
    IO_URING_F_IOPOLL		= (1 << 10),

// set when uring wants to cancel a previously issued command
    IO_URING_F_CANCEL		= (1 << 11),
    IO_URING_F_COMPAT		= (1 << 12),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_wq_work_node {
    pub next: *mut io_wq_work_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_wq_work_list {
    pub first: *mut io_wq_work_node,
    pub last: *mut io_wq_work_node,
}

//
// Lockless multi-producer, single-consumer FIFO queue, see
// io_uring/mpscq.h for the implementation and rules. Defined here so
// that it can be embedded in io_ring_ctx. This is the producer side
// only - the consumer cursor is kept separately, on a cacheline that
// isn't dirtied by the producers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpscq {
    pub /: *mut *mut *mut llist_node tail; / producers,
    pub stub: llist_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_wq_work {
    pub list: io_wq_work_node,
    pub flags: core::sync::atomic::AtomicI32,
// place it here instead of io_kiocb as it fills padding and saves 4B
    pub cancel_seq: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_rsrc_data {
    pub nr: c_uint,
    pub nodes: *mut io_rsrc_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_file_table {
    pub data: io_rsrc_data,
    pub bitmap: *mut c_ulong,
    pub alloc_hint: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_hash_bucket {
    pub list: hlist_head,
    pub ____cacheline_aligned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_hash_table {
    pub hbs: *mut io_hash_bucket,
    pub hash_bits: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_mapped_region {
    pub pages: *mut page,
    pub ptr: *mut c_void,
    pub nr_pages: unsigned,
    pub flags: unsigned,
}

//
// Return value from io_buffer_list selection, to avoid stashing it in
// struct io_kiocb. For legacy/classic provided buffers, keeping a reference
// across execution contexts are fine. But for ring provided buffers, the
// list may go away as soon as ->uring_lock is dropped. As the io_kiocb
// persists, it's better to just keep the buffer local for those cases.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_br_sel {
    pub buf_list: *mut io_buffer_list,
//
// Some selection parts return the user address, others return an error.
//
    pub addr: *mut void __user,
    pub val: isize,
}

//
// Arbitrary limit, can be raised if need be
//
pub const IO_RINGFD_REG_MAX: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_task {
// submission side
    pub cached_refs: c_int,
    pub last: *const io_ring_ctx,
    pub task: *mut task_struct,
    pub io_wq: *mut io_wq,
//
// Consumer cursor for ->task_list. Only popped by the task itself,
// or by ->fallback_work once the task can no longer run task_work.
//
    pub task_head: *mut llist_node,
    pub registered_rings: [*mut file; IO_RINGFD_REG_MAX],
    pub xa: xarray,
    pub wait: wait_queue_head,
    pub in_cancel: core::sync::atomic::AtomicI32,
    pub inflight_tracked: core::sync::atomic::AtomicI32,
    pub inflight: percpu_counter,
// drains ->task_list once the task can no longer run task_work
    pub fallback_work: work_struct,
    pub task_list: mpscq,
    pub task_work: callback_head,
    pub ____cacheline_aligned_in_smp: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iou_vec {
    pub iovec: *mut iovec,
    pub bvec: *mut bio_vec,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring {
    pub head: u32,
    pub tail: u32,
}

//
// This data is shared with the application through the mmap at offsets
// IORING_OFF_SQ_RING and IORING_OFF_CQ_RING.
//
// The offsets to the member fields are published through struct
// io_sqring_offsets when calling io_uring_setup.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_rings {
//
// Head and tail offsets into the ring; the offsets need to be
// masked to get valid indices.
//
// The kernel controls head of the sq ring and the tail of the cq ring,
// and the application controls tail of the sq ring and the head of the
// cq ring.
//
    pub cq: io_uring sq,,
//
// Bitmasks to apply to head and tail offsets (constant, equals
// ring_entries - 1)
//
    pub cq_ring_mask: u32 sq_ring_mask,,
// Ring sizes (constant, power of 2)
    pub cq_ring_entries: u32 sq_ring_entries,,
//
// Number of invalid entries dropped by the kernel due to
// invalid index stored in array
//
// Written by the kernel, shouldn't be modified by the
// application (i.e. get number of "new events" by comparing to
// cached value).
//
// After a new SQ head value was read by the application this
// counter includes all submissions that were dropped reaching
// the new SQ head (and possibly more).
//
    pub sq_dropped: u32,
//
// Runtime SQ flags
//
// Written by the kernel, shouldn't be modified by the
// application.
//
// The application needs a full memory barrier before checking
// for IORING_SQ_NEED_WAKEUP after updating the sq tail.
//
    pub sq_flags: core::sync::atomic::AtomicI32,
//
// Runtime CQ flags
//
// Written by the application, shouldn't be modified by the
// kernel.
//
    pub cq_flags: u32,
//
// Number of completion events lost because the queue was full;
// this should be avoided by the application by making sure
// there are not more requests pending than there is space in
// the completion queue.
//
// Written by the kernel, shouldn't be modified by the
// application (i.e. get number of "new events" by comparing to
// cached value).
//
// As completion events come in out of order this counter is not
// ordered with any other data.
//
    pub cq_overflow: u32,
//
// Ring buffer of completion events.
//
// The kernel writes completion events fresh every time they are
// produced, so the application is allowed to modify pending
// entries.
//
    pub ____cacheline_aligned_in_smp: io_uring_cqe cqes[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_bpf_filters {
    pub /: *mut *mut refcount_t refs; / ref for ->bpf_filters,
    pub /: *mut *mut spinlock_t lock; / protects ->bpf_filters modifications,
    pub filters: *mut io_bpf_filter __rcu,
    pub rcu_head: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_restriction {
    pub IORING_REGISTER_LAST): DECLARE_BITMAP(register_op,,
    pub IORING_OP_LAST): DECLARE_BITMAP(sqe_op,,
    pub bpf_filters: *mut io_bpf_filters,
// ->bpf_filters needs COW on modification
    pub bpf_filters_cow: bool,
    pub sqe_flags_allowed: u8,
    pub sqe_flags_required: u8,
// IORING_OP_* restrictions exist
    pub op_registered: bool,
// IORING_REGISTER_* restrictions exist
    pub reg_registered: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_submit_link {
    pub head: *mut io_kiocb,
    pub last: *mut io_kiocb,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_submit_state {
// inline/task_work completion list, under ->uring_lock
    pub free_list: io_wq_work_node,
// batch completion logic
    pub compl_reqs: io_wq_work_list,
    pub link: io_submit_link,
    pub plug_started: bool,
    pub need_plug: bool,
    pub cq_flush: bool,
    pub submit_nr: c_ushort,
    pub plug: blk_plug,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_alloc_cache {
    pub entries: *mut c_void,
    pub nr_cached: c_uint,
    pub max_cached: c_uint,
    pub elem_size: c_uint,
    pub init_clear: c_uint,
}

// all CQEs should be posted only by the submitter task
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iou_ctx {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_ring_ctx {
// const or read-mostly hot data
// ring setup flags
    pub flags: c_uint,
// internal state flags IO_RING_F_* flags , mostly read-only
    pub int_flags: c_uint,
    pub submitter_task: *mut task_struct,
    pub rings: *mut io_rings,
// cache of ->restrictions.bpf_filters->filters
    pub bpf_filters: *mut io_bpf_filter __rcu,
    pub refs: percpu_ref,
    pub clockid: clockid_t,
    pub clock_offset: tk_offsets,
    pub notify_method: task_work_notify_mode,
    pub sq_thread_idle: unsigned,
    pub ____cacheline_aligned_in_smp: },
// submission data
    pub uring_lock: mutex,
//
// Ring buffer of indices into array of io_uring_sqe, which is
// mmapped by the application using the IORING_OFF_SQES offset.
//
// This indirection could e.g. be used to assign fixed
// io_uring_sqe entries to operations and only submit them to
// the queue when needed.
//
// The kernel modifies neither the indices array nor the entries
// array.
//
    pub sq_array: *mut u32,
    pub sq_sqes: *mut io_uring_sqe,
    pub cached_sq_head: unsigned,
    pub sq_entries: unsigned,
//
// Fixed resources fast path, should be accessed only under
// uring_lock, and updated through io_uring_register(2)
//
    pub cancel_seq: core::sync::atomic::AtomicI32,
//
// ->iopoll_list is protected by the ctx->uring_lock for
// io_uring instances that don't use IORING_SETUP_SQPOLL.
// For SQPOLL, only the single threaded io_sq_thread() will
// manipulate the list, hence no extra locking is needed there.
//
    pub poll_multi_queue: bool,
    pub iopoll_list: list_head,
//
// Consumer cursor for ->work_list, protected by ->uring_lock.
// Deliberately kept away from the producer side of the queue,
// as it's written for every popped entry, and the producer
// cacheline is contended enough as it is.
//
    pub work_head: *mut llist_node,
    pub file_table: io_file_table,
    pub buf_table: io_rsrc_data,
    pub node_cache: io_alloc_cache,
    pub imu_cache: io_alloc_cache,
    pub submit_state: io_submit_state,
//
// Modifications are protected by ->uring_lock and ->mmap_lock.
// The buffer list's io mapped region should be stable once
// published.
//
    pub io_bl_xa: xarray,
    pub cancel_table: io_hash_table,
    pub apoll_cache: io_alloc_cache,
    pub netmsg_cache: io_alloc_cache,
    pub rw_cache: io_alloc_cache,
    pub cmd_cache: io_alloc_cache,
    pub ): *mut iou_loop_params,
//
// Any cancelable uring_cmd is added to this list in
// ->uring_cmd() by io_uring_cmd_insert_cancelable()
//
    pub cancelable_uring_cmd: hlist_head,
//
// For Hybrid IOPOLL, runtime in hybrid polling, without
// scheduling time
//
    pub hybrid_poll_time: u64,
    pub ____cacheline_aligned_in_smp: },
//
// We cache a range of free CQEs we can use, once exhausted it
// should go through a slower range setup, see __io_get_cqe()
//
    pub cqe_cached: *mut io_uring_cqe,
    pub cqe_sentinel: *mut io_uring_cqe,
    pub cached_cq_tail: unsigned,
    pub cq_entries: unsigned,
    pub io_ev_fd: *mut io_ev_fd __rcu,
    pub cq_wait_arg: *mut c_void,
    pub cq_wait_size: usize,
    pub ____cacheline_aligned_in_smp: },
//
// task_work and async notification delivery cacheline. Expected to
// regularly bounce b/w CPUs.
//
    pub rings_rcu: *mut io_rings __rcu,
    pub work_list: mpscq,
    pub check_cq: c_ulong,
    pub cq_wait_nr: core::sync::atomic::AtomicI32,
    pub cq_timeouts: core::sync::atomic::AtomicI32,
    pub cq_wait: wait_queue_head,
    pub ____cacheline_aligned_in_smp: },
// timeouts
    pub timeout_lock: raw_spinlock_t,
    pub timeout_list: list_head,
    pub ltimeout_list: list_head,
    pub cq_last_tm_flush: unsigned,
    pub ____cacheline_aligned_in_smp: },
    pub completion_lock: spinlock_t,
    pub cq_overflow_list: list_head,
    pub waitid_list: hlist_head,

    pub futex_list: hlist_head,
    pub futex_cache: io_alloc_cache,

    pub /: *const *const *const cred sq_creds; / cred used for __io_sq_thread(),
    pub /: *mut *mut *mut io_sq_data sq_data; / if using sq thread polling,
    pub sqo_sq_wait: wait_queue_head,
    pub sqd_list: list_head,
    pub file_alloc_start: c_uint,
    pub file_alloc_end: c_uint,
// Keep this last, we don't need it for the fast path
    pub poll_wq: wait_queue_head,
    pub restrictions: io_restriction,
// Stores zcrx object pointers of type struct io_zcrx_ifq
    pub zcrx_ctxs: xarray,
// Used for accounting references on pages in registered buffers
    pub hpage_acct: xarray,
    pub pers_next: u32,
    pub personalities: xarray,
// hashed buffered write serialization
    pub hash_map: *mut io_wq_hash,
// Only used for accounting purposes
    pub user: *mut user_struct,
    pub mm_account: *mut mm_struct,
//
// List of tctx nodes for this ctx, protected by tctx_lock. For
// cancelation purposes, nests under uring_lock.
//
    pub tctx_list: list_head,
    pub tctx_lock: mutex,
// ctx exit and cancelation
    pub exit_work: work_struct,
    pub ref_comp: completion,
// io-wq management, e.g. thread count
    pub iowq_limits: [u32; 2],
    pub poll_wq_task_work: callback_head,
    pub defer_list: list_head,
    pub nr_drained: unsigned,
// protected by ->completion_lock
    pub nr_req_allocated: unsigned,

    pub /: *mut *mut list_head napi_list; / track busy poll napi_id,
    pub /: *mut *mut spinlock_t napi_lock; / napi_list lock,
// napi busy poll default timeout
    pub napi_busy_poll_dt: ktime_t,
    pub napi_prefer_busy_poll: bool,
    pub napi_track_mode: u8,
    pub 4): DECLARE_HASHTABLE(napi_ht,,

    pub bpf_ops: *mut io_uring_bpf_ops,
//
// Protection for resize vs mmap races - both the mmap and resize
// side will need to grab this lock, to prevent either side from
// being run concurrently with the other.
//
    pub mmap_lock: mutex,
    pub sq_region: io_mapped_region,
    pub ring_region: io_mapped_region,
// used for optimised request parameter and wait argument passing
    pub param_region: io_mapped_region,
    pub kcov_handle: kcov_common_handle_id,
}

//
// Token indicating function is called in task work context:
// ctx->uring_lock is held and any completions generated will be flushed.
// ONLY core io_uring.c should instantiate this struct.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_tw_state {
    pub cancel: bool,
}

// Alias to use in code that doesn't instantiate struct io_tw_state
pub type io_tw_token_t = io_tw_state;
// first byte is taken by user flags, shift it to not overlap
// keep async read/write and isreg together and in order
// not a real bit, just to check we're not overflowing the space
pub type io_req_flags_t = u64 ;

// ctx owns file
// drain existing IO first
// linked sqes
// doesn't sever on completion < 0
// IOSQE_ASYNC
// IOSQE_BUFFER_SELECT
// IOSQE_CQE_SKIP_SUCCESS
// fail rest of links
// on inflight list, should be cancelled and waited on exit reliably
// read/write uses file position
// must not punt to workers
// has or had linked timeout
// needs cleanup
// already went through poll handler
// every req only blocks once in hybrid poll
// buffer already selected
// buffer selected from ring, needs commit
// caller should reissue async
// supports async reads/writes
// regular file
// has creds assigned
// skip refcounting if not set
// there is a linked timeout that has to be armed
// ->async_data allocated
// don't post CQEs while failing linked requests
// single poll may be active
// double poll may active
// request posts multiple completions, should be set at prep time
// fast poll multishot mode
// recvmsg special flag, clear EPOLLIN
// don't use lazy poll wake for this request
// file is pollable
// buffer list was empty after selection of buffer
// don't recycle provided buffers for this request
// buffer ring head needs incrementing on put
// buf node is valid
// incremental buffer consumption, more space available
// request has read/write metadata assigned
//
// For vectored fixed buffers, resolve iovec to registered buffers.
// For SEND_ZC, whether to import buffers (i.e. the first issue).
//
// ->sqe_copy() has been called, if necessary
// request must be iopolled to completion (set in ->issue())
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_tw_req {
    pub req: *mut io_kiocb,
}

extern "C" {
    pub fn void(tw_req: *mut *mut io_req_tw_func_t)(struct io_tw_req, tw: io_tw_token_t) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_task_work {
    pub node: llist_node,
    pub func: io_req_tw_func_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_cqe {
    pub user_data: __u64,
    pub res: __s32,
// fd initially, then cflags for completion
    pub flags: __u32,
    pub fd: c_int,
}

//
// Each request type overlays its private data structure on top of this one.
// They must not exceed this one in size.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_cmd_data {
    pub file: *mut file,
// each command gets 56 bytes of data
    pub data: [__u8; 56],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_kiocb {
//
// NOTE! Each of the io_kiocb union members has the file pointer
// as the first entry in their struct definition. So you can
// access the file pointer through any of the sub-structs,
// or directly as just 'file' in this struct.
//
    pub file: *mut file,
    pub cmd: io_cmd_data,
}

// polled IO has completed
//
// Can be either a fixed buffer index, or used with provided buffers.
// For the latter, it points to the selected buffer ID.
//
// REQ_F_* flags
// stores selected buf, valid IFF REQ_F_BUFFER_SELECTED is set
// used by request caches, completion batching and iopoll
// cache ->apoll->events
// For IOPOLL setup queues, with hybrid polling
//
// for polled requests, i.e. IORING_OP_POLL_ADD and async armed
// poll
//
// IOPOLL completion handling
// for private io_kiocb freeing
// internal polling, see IORING_FEAT_FAST_POLL
// opcode allocated if it needs to store data for async defer
// linked requests, IFF REQ_F_HARDLINK or REQ_F_LINK are set
// custom credentials, valid IFF REQ_F_CREDS is set
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_big_cqe {
    pub extra1: u64,
    pub extra2: u64,
    pub big_cqe: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_overflow_cqe {
    pub list: list_head,
    pub cqe: io_uring_cqe,
}
