//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sunrpc/sched.h
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
//
// linux/include/linux/sunrpc/sched.h
//
// Scheduling primitives for kernel Sun RPC.
//
// Copyright (C) 1996, Olaf Kirch <okir@monad.swb.de>
//

//
// This is the actual RPC procedure call info.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_message {
    pub /: *const *const *const rpc_procinfo rpc_proc; / Procedure information,
    pub /: *mut *mut *mut void  rpc_argp; / Arguments,
    pub /: *mut *mut *mut void  rpc_resp; / Result,
    pub /: *const *const *const cred  rpc_cred; / Credentials,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_wait {
    pub /: *mut *mut list_head list; / wait queue links,
    pub /: *mut *mut list_head links; / Links to related tasks,
    pub /: *mut *mut list_head timer_list; / Timer list,
}

//
// This describes a timeout strategy
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_timeout {
    pub /: *mut *mut to_increment; / if !exponential,
    pub /: *mut *mut unsigned int to_retries; / max # of retries,
    pub to_exponential: c_uchar,
}

//
// This is the RPC task struct
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_task {
    pub /: *mut *mut atomic_t tk_count; / Reference count,
    pub /: *mut *mut int tk_status; / result of last operation,
    pub /: *mut *mut list_head tk_task; / global list of tasks,
//
// callback	to be executed after waking up
// action	next procedure for async tasks
//
    pub ): *mut *mut void (tk_callback)(struct rpc_task,
    pub ): *mut *mut void (tk_action)(struct rpc_task,
    pub /: *mut *mut unsigned long tk_timeout; / timeout for rpc_sleep(),
    pub /: *mut *mut unsigned long tk_runstate; / Task run status,
    pub /: *mut *mut *mut rpc_wait_queue tk_waitqueue; / RPC wait queue we're on,
    pub /: *mut *mut work_tk_work; / Async task work queue,
    pub /: *mut *mut rpc_wait tk_wait; / RPC wait,
    pub u: },
//
// RPC call state
//
    pub /: *mut *mut rpc_message tk_msg; / RPC call info,
    pub /: *mut *mut *mut void  tk_calldata; / Caller private data,
    pub /: *const *const *const rpc_call_ops tk_ops; / Caller callbacks,
    pub /: *mut *mut *mut rpc_clnt  tk_client; / RPC client,
    pub /: *mut *mut *mut rpc_xprt  tk_xprt; / Transport,
    pub /: *mut *mut *mut rpc_cred  tk_op_cred; / cred being operated on,
    pub /: *mut *mut *mut rpc_rqst  tk_rqstp; / RPC request,
    pub could: *mut *mut *mut workqueue_tk_workqueue; / Normally rpciod, but,
// be any workqueue
//
    pub /: *mut *mut ktime_t tk_start; / RPC task init timestamp,
    pub /: *mut *mut pid_t tk_owner; / Process id for batching tasks,
    pub /: *mut *mut int tk_rpc_status; / Result of last RPC operation,
    pub /: *mut *mut unsigned short tk_flags; / misc flags,
    pub /: *mut *mut unsigned short tk_timeouts; / maj timeouts,
    pub /: *mut *mut unsigned short tk_pid; / debugging aid,
    pub 2: tk_cred_retry :,
}

extern "C" {
    pub fn void(: *mut *mut rpc_action)(struct rpc_task) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_call_ops {
    pub ): *mut *mut *mut void (rpc_call_prepare)(struct rpc_task , void,
    pub ): *mut *mut *mut void (rpc_call_done)(struct rpc_task , void,
    pub ): *mut *mut *mut void (rpc_count_stats)(struct rpc_task , void,
    pub ): *mut *mut void (rpc_release)(void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_task_setup {
    pub task: *mut rpc_task,
    pub rpc_client: *mut rpc_clnt,
    pub rpc_xprt: *mut rpc_xprt,
    pub /: *mut *mut *mut rpc_cred rpc_op_cred; / credential being operated on,
    pub rpc_message: *const rpc_message,
    pub callback_ops: *const rpc_call_ops,
    pub callback_data: *mut c_void,
    pub workqueue: *mut workqueue_struct,
    pub flags: c_ushort,
    pub priority: signed char,
}

//
// RPC task flags
//
pub const RPC_TASK_ASYNC: c_uint = 0x0001		/* is an async task */;
pub const RPC_TASK_SWAPPER: c_uint = 0x0002		/* is swapping in/out */;
pub const RPC_TASK_MOVEABLE: c_uint = 0x0004		/* nfs4.1+ rpc tasks */;
pub const RPC_TASK_NULLCREDS: c_uint = 0x0010		/* Use AUTH_NULL credential */;
pub const RPC_CALL_MAJORSEEN: c_uint = 0x0020		/* major timeout seen */;
pub const RPC_TASK_NETUNREACH_FATAL: c_uint = 0x0040	/* ENETUNREACH is fatal */;
pub const RPC_TASK_DYNAMIC: c_uint = 0x0080		/* task was kmalloc'ed */;
pub const RPC_TASK_NO_ROUND_ROBIN: c_uint = 0x0100		/* send requests on "main" xprt */;
pub const RPC_TASK_SOFT: c_uint = 0x0200		/* Use soft timeouts */;
pub const RPC_TASK_SOFTCONN: c_uint = 0x0400		/* Fail if can't connect */;
pub const RPC_TASK_SENT: c_uint = 0x0800		/* message was sent */;
pub const RPC_TASK_TIMEOUT: c_uint = 0x1000		/* fail with ETIMEDOUT on timeout */;
pub const RPC_TASK_NOCONNECT: c_uint = 0x2000		/* return ENOTCONN if not connected */;
pub const RPC_TASK_NO_RETRANS_TIMEOUT: c_uint = 0x4000		/* wait forever for a reply */;
pub const RPC_TASK_CRED_NOREF: c_uint = 0x8000		/* No refcount on the credential */;

//
// Task priorities.
// Note: if you change these, you must also change
// the task initialization definitions below.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_timer {
    pub list: list_head,
    pub expires: c_ulong,
    pub dwork: delayed_work,
}

//
// RPC synchronization objects
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_wait_queue {
    pub lock: spinlock_t,
    pub /: *mut *mut list_head tasks[RPC_NR_PRIORITY]; / task queue for each priority level,
    pub /: *mut *mut unsigned char maxpriority; / maximum priority (0 if queue is not a priority queue),
    pub /: *mut *mut unsigned char priority; / current priority,
    pub /: *mut *mut unsigned char nr; / # tasks remaining for cookie,
    pub /: *mut *mut unsigned int qlen; / total # tasks waiting in queue,
    pub timer_list: rpc_timer,

    pub name: *const *const c_char,

}

//
// This is the # requests to send consecutively
// from a single cookie.  The aim is to improve
// performance of NFS operations such as read/write.
//

//
// Function prototypes
//
extern "C" {
    pub fn rpc_put_task(: *mut rpc_task);
}
extern "C" {
    pub fn rpc_put_task_async(: *mut rpc_task);
}
extern "C" {
    pub fn rpc_task_set_rpc_status(task: *mut rpc_task, rpc_status: c_int) -> bool;
}
extern "C" {
    pub fn rpc_task_try_cancel(task: *mut rpc_task, error: c_int);
}
extern "C" {
    pub fn rpc_signal_task(: *mut rpc_task);
}
extern "C" {
    pub fn rpc_exit_task(: *mut rpc_task);
}
extern "C" {
    pub fn rpc_exit(: *mut rpc_task, _arg: c_int);
}
extern "C" {
    pub fn rpc_release_calldata(: *const rpc_call_ops, : *mut c_void);
}
extern "C" {
    pub fn rpc_killall_tasks(: *mut rpc_clnt);
}
extern "C" {
    pub fn rpc_execute(: *mut rpc_task);
}
extern "C" {
    pub fn rpc_init_priority_wait_queue(: *mut rpc_wait_queue, : *const c_char);
}
extern "C" {
    pub fn rpc_init_wait_queue(: *mut rpc_wait_queue, : *const c_char);
}
extern "C" {
    pub fn rpc_destroy_wait_queue(: *mut rpc_wait_queue);
}
extern "C" {
    pub fn rpc_task_timeout(task: *const rpc_task) -> c_ulong;
}
extern "C" {
    pub fn rpc_wake_up(: *mut rpc_wait_queue);
}
extern "C" {
    pub fn rpc_wake_up_status(: *mut rpc_wait_queue, _arg: c_int);
}
extern "C" {
    pub fn rpc_delay(: *mut rpc_task, long: unsigned);
}
extern "C" {
    pub fn rpc_malloc(: *mut rpc_task) -> c_int;
}
extern "C" {
    pub fn rpc_free(: *mut rpc_task);
}
extern "C" {
    pub fn rpciod_up() -> c_int;
}
extern "C" {
    pub fn rpciod_down();
}
extern "C" {
    pub fn rpc_wait_for_completion_task(task: *mut rpc_task) -> c_int;
}

extern "C" {
    pub fn rpc_show_tasks(: *mut net);
}

extern "C" {
    pub fn rpc_init_mempool() -> c_int;
}
extern "C" {
    pub fn rpc_destroy_mempool();
}
extern "C" {
    pub fn rpc_prepare_task(task: *mut rpc_task);
}
extern "C" {
    pub fn rpc_task_gfp_mask() -> gfp_t;
}

extern "C" {
    pub fn rpc_clnt_swap_activate(clnt: *mut rpc_clnt) -> c_int;
}
extern "C" {
    pub fn rpc_clnt_swap_deactivate(clnt: *mut rpc_clnt);
}

