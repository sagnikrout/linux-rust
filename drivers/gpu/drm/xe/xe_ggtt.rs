//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_ggtt.h
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
// Copyright © 2021 Intel Corporation
//

extern "C" {
    pub fn xe_ggtt_init_early(ggtt: *mut xe_ggtt) -> c_int;
}
extern "C" {
    pub fn xe_ggtt_init_kunit(ggtt: *mut xe_ggtt, reserved: u32, size: u32) -> c_int;
}
extern "C" {
    pub fn xe_ggtt_init(ggtt: *mut xe_ggtt) -> c_int;
}
extern "C" {
    pub fn xe_ggtt_shift_nodes(ggtt: *mut xe_ggtt, new_base: u64);
}
extern "C" {
    pub fn xe_ggtt_start(ggtt: *mut xe_ggtt) -> u64;
}
extern "C" {
    pub fn xe_ggtt_size(ggtt: *mut xe_ggtt) -> u64;
}
extern "C" {
    pub fn xe_ggtt_node_remove(node: *mut xe_ggtt_node, invalidate: bool);
}
extern "C" {
    pub fn xe_ggtt_node_pt_size(node: *const xe_ggtt_node) -> usize;
}
extern "C" {
    pub fn xe_ggtt_map_bo_unlocked(ggtt: *mut xe_ggtt, bo: *mut xe_bo);
}
extern "C" {
    pub fn xe_ggtt_insert_bo(ggtt: *mut xe_ggtt, bo: *mut xe_bo, exec: *mut drm_exec) -> c_int;
}
extern "C" {
    pub fn xe_ggtt_remove_bo(ggtt: *mut xe_ggtt, bo: *mut xe_bo);
}
extern "C" {
    pub fn xe_ggtt_largest_hole(ggtt: *mut xe_ggtt, alignment: u64, spare: *mut u64) -> u64;
}
extern "C" {
    pub fn xe_ggtt_dump(ggtt: *mut xe_ggtt, p: *mut drm_printer) -> c_int;
}
extern "C" {
    pub fn xe_ggtt_print_holes(ggtt: *mut xe_ggtt, alignment: u64, p: *mut drm_printer) -> u64;
}

extern "C" {
    pub fn xe_ggtt_assign(node: *const xe_ggtt_node, vfid: u16);
}
extern "C" {
    pub fn xe_ggtt_node_save(node: *mut xe_ggtt_node, dst: *mut c_void, size: usize, vfid: u16) -> c_int;
}
extern "C" {
    pub fn xe_ggtt_node_load(node: *mut xe_ggtt_node, src: *const c_void, size: usize, vfid: u16) -> c_int;
}

extern "C" {
    pub fn xe_ggtt_might_lock(ggtt: *mut xe_ggtt);
}

extern "C" {
    pub fn xe_ggtt_encode_pte_flags(ggtt: *mut xe_ggtt, bo: *mut xe_bo, pat_index: u16) -> u64;
}
extern "C" {
    pub fn xe_ggtt_read_pte(ggtt: *mut xe_ggtt, offset: u64) -> u64;
}
extern "C" {
    pub fn xe_ggtt_node_addr(node: *const xe_ggtt_node) -> u64;
}
extern "C" {
    pub fn xe_ggtt_node_size(node: *const xe_ggtt_node) -> u64;
}
