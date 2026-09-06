//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/maps.h
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

pub const KMAP_NAME_LEN: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kmap {
    pub ref_reloc_sym: *mut ref_reloc_sym,
    pub kmaps: *mut maps,
    pub name: [c_char; KMAP_NAME_LEN],
}

extern "C" {
    pub fn maps__empty(maps: *mut maps) -> bool;
}
extern "C" {
    pub fn maps__copy_from(maps: *mut maps, parent: *mut maps) -> c_int;
}
extern "C" {
    pub fn maps__put(maps: *mut maps);
}
// map = NULL;

extern "C" {
    pub fn maps__equal(a: *mut maps, b: *mut maps) -> bool;
}
// Iterate over map calling cb for each entry.
extern "C" {
    pub fn maps__for_each_map(maps: *mut maps, map: *mut *mut int (cb)(struct map, data): *mut c_void, data: *mut c_void) -> c_int;
}
// Iterate over map removing an entry if cb returns true.
extern "C" {
    pub fn maps__remove_maps(maps: *mut maps, map: *mut *mut bool (cb)(struct map, data): *mut c_void, data: *mut c_void);
}

extern "C" {
    pub fn maps__set_addr_space(maps: *mut maps, addr_space: *mut c_void);
}
extern "C" {
    pub fn maps__e_machine(maps: *const maps) -> u16;
}
extern "C" {
    pub fn maps__set_e_machine(maps: *mut maps, e_machine: u16);
}

extern "C" {
    pub fn maps__set_libdw_addr_space_dwfl(maps: *mut maps, dwfl: *mut c_void);
}

extern "C" {
    pub fn maps__fprintf(maps: *mut maps, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn maps__load_maps(maps: *mut maps) -> c_int;
}
extern "C" {
    pub fn maps__insert(maps: *mut maps, map: *mut map) -> c_int;
}
extern "C" {
    pub fn maps__remove(maps: *mut maps, map: *mut map);
}
extern "C" {
    pub fn maps__find_ams(maps: *mut maps, ams: *mut addr_map_symbol) -> c_int;
}
extern "C" {
    pub fn maps__fixup_overlap_and_insert(maps: *mut maps, new: *mut map) -> c_int;
}
extern "C" {
    pub fn maps__merge_in(kmaps: *mut maps, new_map: *mut map) -> c_int;
}
extern "C" {
    pub fn maps__fixup_end(maps: *mut maps);
}
extern "C" {
    pub fn maps__load_first(maps: *mut maps);
}
