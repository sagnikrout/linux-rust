//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/raid5-log.h
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
extern "C" {
    pub fn r5l_init_log(conf: *mut r5conf, rdev: *mut md_rdev) -> c_int;
}
extern "C" {
    pub fn r5l_exit_log(conf: *mut r5conf);
}
extern "C" {
    pub fn r5l_write_stripe(log: *mut r5l_log, head_sh: *mut stripe_head) -> c_int;
}
extern "C" {
    pub fn r5l_write_stripe_run(log: *mut r5l_log);
}
extern "C" {
    pub fn r5l_flush_stripe_to_raid(log: *mut r5l_log);
}
extern "C" {
    pub fn r5l_stripe_write_finished(sh: *mut stripe_head);
}
extern "C" {
    pub fn r5l_handle_flush_request(log: *mut r5l_log, bio: *mut bio) -> c_int;
}
extern "C" {
    pub fn r5l_quiesce(log: *mut r5l_log, quiesce: c_int);
}
extern "C" {
    pub fn r5l_log_disk_error(conf: *mut r5conf) -> bool;
}
extern "C" {
    pub fn r5c_is_writeback(log: *mut r5l_log) -> bool;
}
extern "C" {
    pub fn r5c_release_extra_page(sh: *mut stripe_head);
}
extern "C" {
    pub fn r5c_use_extra_page(sh: *mut stripe_head);
}
extern "C" {
    pub fn r5l_wake_reclaim(log: *mut r5l_log, space: sector_t);
}
extern "C" {
    pub fn r5c_cache_data(log: *mut r5l_log, sh: *mut stripe_head) -> c_int;
}
extern "C" {
    pub fn r5c_make_stripe_write_out(sh: *mut stripe_head);
}
extern "C" {
    pub fn r5c_flush_cache(conf: *mut r5conf, num: c_int);
}
extern "C" {
    pub fn r5c_check_stripe_cache_usage(conf: *mut r5conf);
}
extern "C" {
    pub fn r5c_check_cached_full_stripe(conf: *mut r5conf);
}
extern "C" {
    pub fn r5c_update_on_rdev_error(mddev: *mut mddev, rdev: *mut md_rdev);
}
extern "C" {
    pub fn r5c_big_stripe_cached(conf: *mut r5conf, sect: sector_t) -> bool;
}
extern "C" {
    pub fn r5l_start(log: *mut r5l_log) -> c_int;
}
extern "C" {
    pub fn ppl_init_log(conf: *mut r5conf) -> c_int;
}
extern "C" {
    pub fn ppl_exit_log(conf: *mut r5conf);
}
extern "C" {
    pub fn ppl_write_stripe(conf: *mut r5conf, sh: *mut stripe_head) -> c_int;
}
extern "C" {
    pub fn ppl_write_stripe_run(conf: *mut r5conf);
}
extern "C" {
    pub fn ppl_stripe_write_finished(sh: *mut stripe_head);
}
extern "C" {
    pub fn ppl_modify_log(conf: *mut r5conf, rdev: *mut md_rdev, add: bool) -> c_int;
}
extern "C" {
    pub fn ppl_quiesce(conf: *mut r5conf, quiesce: c_int);
}
extern "C" {
    pub fn ppl_handle_flush_request(bio: *mut bio) -> c_int;
}
extern "C" {
    pub fn test_bit(_arg: MD_HAS_JOURNAL, _arg: &conf->mddev->flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: MD_HAS_PPL, _arg: &conf->mddev->flags) -> return;
}
// writing out phase
extern "C" {
    pub fn r5l_write_stripe(_arg: conf->log, _arg: sh) -> return;
}
// caching phase
extern "C" {
    pub fn r5c_cache_data(_arg: conf->log, _arg: sh) -> return;
}
extern "C" {
    pub fn ppl_write_stripe(_arg: conf, _arg: sh) -> return;
}
extern "C" {
    pub fn r5l_init_log(_arg: conf, _arg: journal_dev) -> return;
}
extern "C" {
    pub fn ppl_init_log(_arg: conf) -> return;
}
extern "C" {
    pub fn ppl_modify_log(_arg: conf, _arg: rdev, _arg: add) -> return;
}
