//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/netns/can.h
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
// can in net namespaces
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netns_can {

    pub proc_dir: *mut proc_dir_entry,
    pub pde_stats: *mut proc_dir_entry,
    pub pde_reset_stats: *mut proc_dir_entry,
    pub pde_rcvlist_all: *mut proc_dir_entry,
    pub pde_rcvlist_fil: *mut proc_dir_entry,
    pub pde_rcvlist_inv: *mut proc_dir_entry,
    pub pde_rcvlist_sff: *mut proc_dir_entry,
    pub pde_rcvlist_eff: *mut proc_dir_entry,
    pub pde_rcvlist_err: *mut proc_dir_entry,
    pub bcmproc_dir: *mut proc_dir_entry,

// receive filters subscribed for 'all' CAN devices
    pub rx_alldev_list: *mut can_dev_rcv_lists,
    pub rcvlists_lock: spinlock_t,
    pub /: *mut *mut timer_list stattimer; / timer for statistics update,
    pub pkg_stats: *mut can_pkg_stats,
    pub rcv_lists_stats: *mut can_rcv_lists_stats,
// CAN GW per-net gateway jobs
    pub cgw_list: hlist_head,
}
