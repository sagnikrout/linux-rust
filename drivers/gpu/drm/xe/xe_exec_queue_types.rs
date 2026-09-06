//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_exec_queue_types.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2022 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_exec_queue_priority {
    XE_EXEC_QUEUE_PRIORITY_UNSET = -2, /* For execlist usage only */
    XE_EXEC_QUEUE_PRIORITY_LOW = 0,
    XE_EXEC_QUEUE_PRIORITY_NORMAL,
    XE_EXEC_QUEUE_PRIORITY_HIGH,
    XE_EXEC_QUEUE_PRIORITY_KERNEL,

    XE_EXEC_QUEUE_PRIORITY_COUNT
}

//
// enum xe_multi_queue_priority - Multi Queue priority values
//
// The priority values of the queues within the multi queue group.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_multi_queue_priority {
// @XE_MULTI_QUEUE_PRIORITY_LOW: Priority low
    XE_MULTI_QUEUE_PRIORITY_LOW = 0,
// @XE_MULTI_QUEUE_PRIORITY_NORMAL: Priority normal
    XE_MULTI_QUEUE_PRIORITY_NORMAL,
// @XE_MULTI_QUEUE_PRIORITY_HIGH: Priority high
    XE_MULTI_QUEUE_PRIORITY_HIGH,
}

//
// struct xe_exec_queue_group - Execution multi queue group
//
// Contains multi queue group information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_exec_queue_group {
// @primary: Primary queue of this group
    pub primary: *mut xe_exec_queue,
// @cgp_bo: BO for the Context Group Page
    pub cgp_bo: *mut xe_bo,
// @xa: xarray to store LRCs
    pub xa: xarray,
// @list: List of all secondary queues in the group
    pub list: list_head,
// @list_lock: Secondary queue list lock
    pub list_lock: mutex,
//
// @suspend_lock: Makes a secondary's suspend/resume and its forwarding
// to the primary atomic. Nested outside of the queue's message lock
// (@xe_guc_exec_queue.sched.msg_lock).
//
    pub suspend_lock: spinlock_t,
// @sync_pending: CGP_SYNC_DONE g2h response pending
    pub sync_pending: bool,
// @banned: Group banned
    pub banned: bool,
// @stopped: Group is stopped, protected by list_lock
    pub stopped: bool,
}

//
// struct xe_exec_queue - Execution queue
//
// Contains all state necessary for submissions. Can either be a user object or
// a kernel object.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_exec_queue {
// @xef: Back pointer to xe file if this is user created exec queue
    pub xef: *mut xe_file,
// @gt: GT structure this exec queue can submit to
    pub gt: *mut xe_gt,
//
// @hwe: A hardware of the same class. May (physical engine) or may not
// (virtual engine) be where jobs actual engine up running. Should never
// really be used for submissions.
//
    pub hwe: *mut xe_hw_engine,
// @refcount: ref count of this exec queue
    pub refcount: kref,
// @vm: VM (address space) for this exec queue
    pub vm: *mut xe_vm,
//
// @user_vm: User VM (address space) for this exec queue (bind queues
// only)
//
    pub user_vm: *mut xe_vm,
// @class: class of this exec queue
    pub class: xe_engine_class,
//
// @logical_mask: logical mask of where job submitted to exec queue can run
//
    pub logical_mask: u32,
// @name: name of this exec queue
    pub name: [c_char; MAX_FENCE_NAME_LEN],
// @width: width (number BB submitted per exec) of this exec queue
    pub width: u16,
// @msix_vec: MSI-X vector (for platforms that support it)
    pub msix_vec: u16,
// @fence_irq: fence IRQ used to signal job completion
    pub fence_irq: *mut xe_hw_fence_irq,
//
// @last_fence: last fence on exec queue, protected by vm->lock in write
// mode if bind exec queue, protected by dma resv lock if non-bind exec
// queue
//
    pub last_fence: *mut dma_fence,
// queue used for kernel submission only

// kernel engine only destroyed at driver unload

// for VM jobs. Caller needs to hold rpm ref when creating queue with this flag

// child of VM queue for multi-tile VM jobs

// kernel exec_queue only, set priority to highest level

// flag to indicate low latency hint to guc

// for migration (kernel copy, clear, bind) jobs

// for programming COMMON_SLICE_CHICKEN3 on first submission

//
// @flags: flags for this exec queue, should statically setup aside from ban
// bit
//
    pub flags: c_ulong,
// @multi_gt_list: list head for VM bind engines if multi-GT
    pub multi_gt_list: list_head,
// @multi_gt_link: link for VM bind engines if multi-GT
    pub multi_gt_link: list_head,
}

// @execlist: execlist backend specific state for exec queue
// @guc: GuC backend specific state for exec queue
// @multi_queue: Multi queue information
// @multi_queue.group: Queue group information
// @multi_queue.link: Link into group's secondary queues list
//
// @multi_queue.priority: Queue priority within the multi-queue group.
// It is protected by @multi_queue.lock.
//
// @multi_queue.lock: Lock for protecting certain members
// @multi_queue.pos: Position of queue within the multi-queue group
// @multi_queue.valid: Queue belongs to a multi queue group
// @multi_queue.is_primary: Is primary queue (Q0) of the group
// @sched_props: scheduling properties
// @sched_props.timeslice_us: timeslice period in micro-seconds
// @sched_props.preempt_timeout_us: preemption timeout in micro-seconds
// @sched_props.job_timeout_ms: job timeout in milliseconds
// @sched_props.priority: priority of this exec queue
// @lr: long-running exec queue state
// @lr.pfence: preemption fence
// @lr.context: preemption fence context
// @lr.seqno: preemption fence seqno
// @lr.link: link into VM's list of exec queues
//
// @lr.suspended: Tracks whether the consumer-issued suspend()
// succeeded and a matching resume() is still owed. suspend() can
// fail (e.g. killed/banned/wedged), leaving the queue
// un-suspended, so consumers must only resume() queues that were
// actually suspended. Set by the suspend caller on success and
// cleared by the resume caller. A queue is only ever suspended by
// a single consumer at a time (preempt-fence mode and hw engine
// group fault mode are mutually exclusive), so a single flag is
// sufficient.
//
pub const XE_EXEC_QUEUE_TLB_INVAL_PRIMARY_GT: c_int = 0;
pub const XE_EXEC_QUEUE_TLB_INVAL_MEDIA_GT: c_int = 1;

// @tlb_inval: TLB invalidations exec queue state
//
// @tlb_inval.dep_scheduler: The TLB invalidation
// dependency scheduler
//
// @tlb_inval.last_fence: last fence for tlb invalidation, protected by
// vm->lock in write mode
//
// @vm_exec_queue_link: Link to track exec queue within a VM's list of exec queues.
// @pxp: PXP info tracking
// @pxp.type: PXP session type used by this queue
// @pxp.link: link into the list of PXP exec queues
// @ufence_syncobj: User fence syncobj
// @ufence_timeline_value: User fence timeline value
// @replay_state: GPU hang replay state
// @ops: submission backend exec queue operations
// @ring_ops: ring operations for this exec queue
// @entity: DRM sched entity for this exec queue (1 to 1 relationship)
pub const XE_MAX_JOB_COUNT_PER_EXEC_QUEUE: c_int = 1000;
// @job_cnt: number of drm jobs in this exec queue
//
// @tlb_flush_seqno: The seqno of the last rebind tlb flush performed
// Protected by @vm's resv. Unused if @vm == NULL.
//
// @hw_engine_group_link: link into exec queues in the same hw engine group
//
// @lrc_lookup_lock: Lock for protecting lrc array access. Only used when
// running in parallel to queue creation is possible.
//
// @lrc: logical ring context for this exec queue
//
// struct xe_exec_queue_ops - Submission backend exec queue operations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_exec_queue_ops {
// @init: Initialize exec queue for submission backend
    pub q): *mut *mut int (init)(struct xe_exec_queue,
// @kill: Kill inflight submissions for backend
    pub q): *mut *mut void (kill)(struct xe_exec_queue,
// @fini: Undoes the init() for submission backend
    pub q): *mut *mut void (fini)(struct xe_exec_queue,
//
// @destroy: Destroy exec queue for submission backend. The backend
// function must call xe_exec_queue_fini() (which will in turn call the
// fini() backend function) to ensure the queue is properly cleaned up.
//
    pub q): *mut *mut void (destroy)(struct xe_exec_queue,
// @set_priority: Set priority for exec queue
    pub priority): xe_exec_queue_priority,
// @set_timeslice: Set timeslice for exec queue
    pub timeslice_us): *mut *mut *mut int (set_timeslice)(struct xe_exec_queue q, u32,
// @set_preempt_timeout: Set preemption timeout for exec queue
    pub preempt_timeout_us): *mut *mut *mut int (set_preempt_timeout)(struct xe_exec_queue q, u32,
// @set_multi_queue_priority: Set multi queue priority
    pub priority): xe_multi_queue_priority,
//
// @suspend: Suspend exec queue from executing, allowed to be called
// multiple times in a row before resume with the caveat that
// suspend_wait returns before calling suspend again.
//
    pub q): *mut *mut int (suspend)(struct xe_exec_queue,
//
// @suspend_wait: Wait for an exec queue to suspend executing, should be
// call after suspend. In dma-fencing path thus must return within a
// reasonable amount of time. -ETIME return shall indicate an error
// waiting for suspend resulting in associated VM getting killed.
// -EAGAIN return indicates the wait should be tried again, if the wait
// is within a work item, the work item should be requeued as deadlock
// avoidance mechanism.
//
    pub q): *mut *mut int (suspend_wait)(struct xe_exec_queue,
//
// @suspend_wait_blocking: Like @suspend_wait, but waits uninterruptibly
// (does not abort on the calling task's signals). For cleanup/undo paths
// that must complete a suspend on behalf of a queue that may belong to a
// different process than the caller: a signal to the caller must not
// abandon the wait, which would leave the other process's queue
// suspended forever (cross-process DoS). A timeout bans like suspend_wait.
//
    pub q): *mut *mut int (suspend_wait_blocking)(struct xe_exec_queue,
//
// @resume: Resume exec queue execution, exec queue must be in a suspended
// state and dma fence returned from most recent suspend call must be
// signalled when this function is called.
//
    pub q): *mut *mut void (resume)(struct xe_exec_queue,
// @reset_status: check exec queue reset status
    pub q): *mut *mut bool (reset_status)(struct xe_exec_queue,
}
