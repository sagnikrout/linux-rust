//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/msg.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

// ipcs ctl commands
pub const MSG_STAT: c_int = 11;
pub const MSG_INFO: c_int = 12;
pub const MSG_STAT_ANY: c_int = 13;
// msgrcv options

// Obsolete, used only for backwards compatibility and libc5 compiles
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msqid_ds {
    pub msg_perm: ipc_perm,
    pub /: *mut *mut *mut msg msg_first; / first message on queue,unused,
    pub /: *mut *mut *mut msg msg_last; / last message in queue,unused,
    pub /: *mut *mut __kernel_old_time_t msg_stime; / last msgsnd time,
    pub /: *mut *mut __kernel_old_time_t msg_rtime; / last msgrcv time,
    pub /: *mut *mut __kernel_old_time_t msg_ctime; / last change time,
    pub /: *mut *mut unsigned long msg_lcbytes; / Reuse junk fields for 32 bit,
    pub /: *mut *mut unsigned long msg_lqbytes; / ditto,
    pub /: *mut *mut unsigned short msg_cbytes; / current number of bytes on queue,
    pub /: *mut *mut unsigned short msg_qnum; / number of messages in queue,
    pub /: *mut *mut unsigned short msg_qbytes; / max number of bytes on queue,
    pub /: *mut *mut __kernel_ipc_pid_t msg_lspid; / pid of last msgsnd,
    pub /: *mut *mut __kernel_ipc_pid_t msg_lrpid; / last receive pid,
}

// Include the definition of msqid64_ds

// message buffer for msgsnd and msgrcv calls
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msgbuf {
    pub /: *mut *mut __kernel_long_t mtype; / type of message,
    pub /: *mut *mut char mtext[1]; / message text,
}

// buffer for msgctl calls IPC_INFO, MSG_INFO
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msginfo {
    pub msgpool: c_int,
    pub msgmap: c_int,
    pub msgmax: c_int,
    pub msgmnb: c_int,
    pub msgmni: c_int,
    pub msgssz: c_int,
    pub msgtql: c_int,
    pub msgseg: c_ushort,
}

//
// MSGMNI, MSGMAX and MSGMNB are default values which can be
// modified by sysctl.
//
// MSGMNI is the upper limit for the number of messages queues per
// namespace.
// It has been chosen to be as large possible without facilitating
// scenarios where userspace causes overflows when adjusting the limits via
// operations of the form retrieve current limit; add X; update limit".
//
// MSGMNB is the default size of a new message queue. Non-root tasks can
// decrease the size with msgctl(IPC_SET), root tasks
// (actually: CAP_SYS_RESOURCE) can both increase and decrease the queue
// size. The optimal value is application dependent.
// 16384 is used because it was always used (since 0.99.10)
//
// MAXMAX is the maximum size of an individual message, it's a global
// (per-namespace) limit that applies for all message queues.
// It's set to 1/2 of MSGMNB, to ensure that at least two messages fit into
// the queue. This is also an arbitrary choice (since 2.6.0).
//

// unused

