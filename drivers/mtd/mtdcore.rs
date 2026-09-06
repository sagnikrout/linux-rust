//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mtd/mtdcore.h
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
// These are exported solely for the purpose of mtd_blkdevs.c and mtdchar.c.
// You should not use them for _anything_ else.
//
extern "C" {
    pub fn add_mtd_device(mtd: *mut mtd_info) -> int __must_check;
}
extern "C" {
    pub fn del_mtd_device(mtd: *mut mtd_info) -> c_int;
}
extern "C" {
    pub fn add_mtd_partitions(: *mut mtd_info, : *const mtd_partition, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn del_mtd_partitions(: *mut mtd_info) -> c_int;
}
extern "C" {
    pub fn release_mtd_partition(mtd: *mut mtd_info);
}
extern "C" {
    pub fn mtd_part_parser_cleanup(parts: *mut mtd_partitions);
}
extern "C" {
    pub fn init_mtdchar() -> int __init;
}
extern "C" {
    pub fn cleanup_mtdchar() -> void __exit;
}
