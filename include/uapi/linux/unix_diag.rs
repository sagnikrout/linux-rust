//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/unix_diag.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct unix_diag_req {
    pub sdiag_family: __u8,
    pub sdiag_protocol: __u8,
    pub pad: __u16,
    pub udiag_states: __u32,
    pub udiag_ino: __u32,
    pub udiag_show: __u32,
    pub udiag_cookie: [__u32; 2],
}

pub const UDIAG_SHOW_NAME: c_uint = 0x00000001	/* show name (not path) */;
pub const UDIAG_SHOW_VFS: c_uint = 0x00000002	/* show VFS inode info */;
pub const UDIAG_SHOW_PEER: c_uint = 0x00000004	/* show peer socket info */;
pub const UDIAG_SHOW_ICONS: c_uint = 0x00000008	/* show pending connections */;
pub const UDIAG_SHOW_RQLEN: c_uint = 0x00000010	/* show skb receive queue len */;
pub const UDIAG_SHOW_MEMINFO: c_uint = 0x00000020	/* show memory info of a socket */;
pub const UDIAG_SHOW_UID: c_uint = 0x00000040	/* show socket's UID */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct unix_diag_msg {
    pub udiag_family: __u8,
    pub udiag_type: __u8,
    pub udiag_state: __u8,
    pub pad: __u8,
    pub udiag_ino: __u32,
    pub udiag_cookie: [__u32; 2],
}

// UNIX_DIAG_NONE, standard nl API requires this attribute!

#[repr(C)]
#[derive(Copy, Clone)]
pub struct unix_diag_vfs {
    pub udiag_vfs_ino: __u32,
    pub udiag_vfs_dev: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct unix_diag_rqlen {
    pub udiag_rqueue: __u32,
    pub udiag_wqueue: __u32,
}
