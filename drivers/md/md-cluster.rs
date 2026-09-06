//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/md-cluster.h
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
pub struct md_cluster_operations {
    pub head: md_submodule_head,
    pub nodes): *mut *mut *mut int (join)(struct mddev mddev, int,
    pub mddev): *mut *mut int (leave)(struct mddev,
    pub mddev): *mut *mut int (slot_number)(struct mddev,
    pub hi): *mut *mut *mut int (resync_info_update)(struct mddev mddev, sector_t lo, sector_t,
    pub mddev): *mut *mut int (resync_start_notify)(struct mddev,
    pub mddev): *mut *mut int (resync_status_get)(struct mddev,
    pub hi): *mut *mut *mut *mut void (resync_info_get)(struct mddev mddev, sector_t lo, sector_t,
    pub mddev): *mut *mut int (metadata_update_start)(struct mddev,
    pub mddev): *mut *mut int (metadata_update_finish)(struct mddev,
    pub mddev): *mut *mut void (metadata_update_cancel)(struct mddev,
    pub mddev): *mut *mut int (resync_start)(struct mddev,
    pub mddev): *mut *mut int (resync_finish)(struct mddev,
    pub hi): *mut *mut *mut int (area_resyncing)(struct mddev mddev, int direction, sector_t lo, sector_t,
    pub rdev): *mut *mut *mut int (add_new_disk)(struct mddev mddev, struct md_rdev,
    pub mddev): *mut *mut void (add_new_disk_cancel)(struct mddev,
    pub ack): *mut *mut *mut int (new_disk_ack)(struct mddev mddev, bool,
    pub rdev): *mut *mut *mut int (remove_disk)(struct mddev mddev, struct md_rdev,
    pub total_slots): *mut *mut *mut void (load_bitmaps)(struct mddev mddev, int,
    pub rdev): *mut *mut int (gather_bitmaps)(struct md_rdev,
    pub oldsize): *mut *mut *mut int (resize_bitmaps)(struct mddev mddev, sector_t newsize, sector_t,
    pub mddev): *mut *mut int (lock_all_bitmaps)(struct mddev,
    pub mddev): *mut *mut void (unlock_all_bitmaps)(struct mddev,
    pub old_dev_sectors): *mut *mut *mut void (update_size)(struct mddev mddev, sector_t,
}

extern "C" {
    pub fn md_setup_cluster(mddev: *mut mddev, nodes: c_int) -> c_int;
}
extern "C" {
    pub fn md_cluster_stop(mddev: *mut mddev);
}
extern "C" {
    pub fn md_reload_sb(mddev: *mut mddev, raid_disk: c_int);
}
