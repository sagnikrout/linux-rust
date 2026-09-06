//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ipc_namespace.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_ids {
    pub in_use: c_int,
    pub seq: c_ushort,
    pub rwsem: rw_semaphore,
    pub ipcs_idr: idr,
    pub max_idx: c_int,
    pub /: *mut *mut int last_idx; / For wrap around detection,

    pub next_id: c_int,

    pub key_ht: rhashtable,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_namespace {
    pub ids: [ipc_ids; 3],
    pub sem_ctls: [c_int; 4],
    pub used_sems: c_int,
    pub msg_ctlmax: c_uint,
    pub msg_ctlmnb: c_uint,
    pub msg_ctlmni: c_uint,
    pub percpu_msg_bytes: percpu_counter,
    pub percpu_msg_hdrs: percpu_counter,
    pub shm_ctlmax: usize,
    pub shm_ctlall: usize,
    pub shm_tot: c_ulong,
    pub shm_ctlmni: c_int,
//
// Defines whether IPC_RMID is forced for _all_ shm segments regardless
// of shmctl()
//
    pub shm_rmid_forced: c_int,
    pub ipcns_nb: notifier_block,
// The kern_mount of the mqueuefs sb.  We take a ref on it
    pub mq_mnt: *mut vfsmount,
// # queues in this ns, protected by mq_lock
    pub mq_queues_count: c_uint,
// next fields are set through sysctl
    pub /: *mut *mut unsigned int mq_queues_max; / initialized to DFLT_QUEUESMAX,
    pub /: *mut *mut unsigned int mq_msg_max; / initialized to DFLT_MSGMAX,
    pub /: *mut *mut unsigned int mq_msgsize_max; / initialized to DFLT_MSGSIZEMAX,
    pub mq_msg_default: c_uint,
    pub mq_msgsize_default: c_uint,
    pub mq_set: ctl_table_set,
    pub mq_sysctls: *mut ctl_table_header,
    pub ipc_set: ctl_table_set,
    pub ipc_sysctls: *mut ctl_table_header,
// user_ns which owns the ipc ns
    pub user_ns: *mut user_namespace,
    pub ucounts: *mut ucounts,
    pub mnt_llist: llist_node,
    pub ns: ns_common,
    pub __randomize_layout: },
    pub init_ipc_ns: extern struct ipc_namespace,
    pub mq_lock: extern spinlock_t,

    pub ns): *mut extern void shm_destroy_orphaned(struct ipc_namespace,

    pub ns): *mut extern int mq_init_ns(struct ipc_namespace,
//
// POSIX Message Queue default values:
//
// MIN_*: Lowest value an admin can set the maximum unprivileged limit to
// DFLT_*MAX: Default values for the maximum unprivileged limits
// DFLT_{MSG,MSGSIZE}: Default values used when the user doesn't supply
// an attribute to the open call and the queue must be created
// HARD_*: Highest value the maximums can be set to.  These are enforced
// on CAP_SYS_RESOURCE apps as well making them inviolate (so make them
// suitably high)
//
// POSIX Requirements:
// Per app minimum openable message queues - 8.  This does not map well
// to the fact that we limit the number of queues on a per namespace
// basis instead of a per app basis.  So, make the default high enough
// that no given app should have a hard time opening 8 queues.
// Minimum maximum for HARD_MSGMAX - 32767.  I bumped this to 65536.
// Minimum maximum for HARD_MSGSIZEMAX - POSIX is silent on this.  However,
// we have run into a situation where running applications in the wild
// require this to be at least 5MB, and preferably 10MB, so I set the
// value to 16MB in hopes that this user is the worst of the bunch and
// the new maximum will handle anyone else.  I may have to revisit this
// in the future.
//
pub const DFLT_QUEUESMAX: c_int = 256;
pub const MIN_MSGMAX: c_int = 1;

pub const DFLT_MSGMAX: c_int = 10;
pub const HARD_MSGMAX: c_int = 65536;
pub const MIN_MSGSIZEMAX: c_int = 128;

pub const DFLT_MSGSIZEMAX: c_int = 8192;

    pub }: *mut *mut static inline int mq_init_ns(struct ipc_namespace ns) { return 0;,

    pub ns): return container_of(ns, struct ipc_namespace,,
    pub ns): *mut *mut user_namespace user_ns, ipc_namespace,
    pub ns: return,
    pub ns: return,
    pub NULL: return,
    pub ns): *mut extern void put_ipc_ns(struct ipc_namespace,

    pub ERR_PTR(-EINVAL): return,
    pub ns: return,
    pub ns: return,
    pub ns: return,

    pub ns): *mut void retire_mq_sysctls(struct ipc_namespace,
    pub ns): *mut bool setup_mq_sysctls(struct ipc_namespace,

    pub true: return,

    pub ns): *mut bool setup_ipc_sysctls(struct ipc_namespace,
    pub ns): *mut void retire_ipc_sysctls(struct ipc_namespace,

    pub true: return,

