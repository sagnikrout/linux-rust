//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/radix-tree/test.h
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
pub struct item {
    pub rcu_head: rcu_head,
    pub index: c_ulong,
    pub order: c_uint,
}

extern "C" {
    pub fn item_insert(root: *mut radix_tree_root, index: c_ulong) -> c_int;
}
extern "C" {
    pub fn item_sanity(item: *mut item, index: c_ulong);
}
extern "C" {
    pub fn item_free(item: *mut item, index: c_ulong);
}
extern "C" {
    pub fn item_delete(root: *mut radix_tree_root, index: c_ulong) -> c_int;
}
extern "C" {
    pub fn item_delete_rcu(xa: *mut xarray, index: c_ulong) -> c_int;
}
extern "C" {
    pub fn item_check_present(root: *mut radix_tree_root, index: c_ulong);
}
extern "C" {
    pub fn item_check_absent(root: *mut radix_tree_root, index: c_ulong);
}
extern "C" {
    pub fn item_kill_tree(root: *mut radix_tree_root);
}
extern "C" {
    pub fn xarray_tests();
}
extern "C" {
    pub fn tag_check();
}
extern "C" {
    pub fn multiorder_checks();
}
extern "C" {
    pub fn iteration_test(order: unsigned, duration: unsigned);
}
extern "C" {
    pub fn iteration_test2(duration: unsigned);
}
extern "C" {
    pub fn benchmark();
}
extern "C" {
    pub fn idr_checks();
}
extern "C" {
    pub fn ida_tests();
}
extern "C" {
    pub fn item_tag_get(root: *mut radix_tree_root, index: c_ulong, tag: c_int) -> c_int;
}
extern "C" {
    pub fn tree_verify_min_height(root: *mut radix_tree_root, maxindex: c_int);
}
extern "C" {
    pub fn verify_tag_consistency(root: *mut radix_tree_root, tag: c_uint);
}
// Normally private parts of lib/radix-tree.c
extern "C" {
    pub fn radix_tree_dump(root: *mut radix_tree_root);
}
extern "C" {
    pub fn root_tag_get(root: *mut radix_tree_root, tag: c_uint) -> c_int;
}
extern "C" {
    pub fn node_maxindex(: *mut radix_tree_node) -> c_ulong;
}
extern "C" {
    pub fn shift_maxindex(shift: c_uint) -> c_ulong;
}
extern "C" {
    pub fn radix_tree_cpu_dead(cpu: c_uint) -> c_int;
}
