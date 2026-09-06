//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/map.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mapping_type {
// map__map_ip/map__unmap_ip are given as offsets in the DSO.
    MAPPING_TYPE__DSO,
// map__map_ip/map__unmap_ip are just the given ip value.
    MAPPING_TYPE__IDENTITY,
}

extern "C" {
    pub fn map__end(map__start(map: map) -) -> return;
}
// ip -> dso rip
// dso rip -> ip
extern "C" {
    pub fn map__dso_map_ip(_arg: map, _arg: ip_or_rip) -> return;
}
extern "C" {
    pub fn map__dso_unmap_ip(_arg: map, _arg: ip_or_rip) -> return;
}
// rip/ip <-> addr suitable for passing to `objdump --start-address=`
extern "C" {
    pub fn map__rip_2objdump(map: *const map, rip: u64) -> u64;
}
// objdump address -> memory address
extern "C" {
    pub fn map__objdump_2mem(map: *const map, ip: u64) -> u64;
}
// objdump address -> rip
extern "C" {
    pub fn map__objdump_2rip(map: *const map, ip: u64) -> u64;
}
// map__for_each_symbol - iterate over the symbols in the given map
//
// @map: the 'struct map *' in which symbols are iterated
// @pos: the 'struct symbol *' to use as a loop cursor
// @n: the 'struct rb_node *' to use as a temporary storage
// Note: caller must ensure map->dso is not NULL (map is loaded).
//

// map__for_each_symbol_with_name - iterate over the symbols in the given map
// that have the given name
//
// @map: the 'struct map *' in which symbols are iterated
// @sym_name: the symbol name
// @pos: the 'struct symbol *' to use as a loop cursor
// @idx: the cursor index in the symbol names array
//

extern "C" {
    pub fn map__delete(map: *mut map);
}
extern "C" {
    pub fn map__put(map: *mut map);
}
// map = NULL;

extern "C" {
    pub fn map__fprintf(map: *mut map, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn map__fprintf_dsoname(map: *mut map, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn map__fprintf_dsoname_dsoff(map: *mut map, print_off: bool, addr: u64, fp: *mut FILE) -> usize;
}
extern "C" {
    pub fn map__load(map: *mut map) -> c_int;
}
extern "C" {
    pub fn map__fixup_start(map: *mut map);
}
extern "C" {
    pub fn map__fixup_end(map: *mut map);
}
extern "C" {
    pub fn __map__is_kernel(map: *const map) -> bool;
}
extern "C" {
    pub fn __map__is_extra_kernel_map(map: *const map) -> bool;
}
extern "C" {
    pub fn __map__is_bpf_prog(map: *const map) -> bool;
}
extern "C" {
    pub fn __map__is_bpf_image(map: *const map) -> bool;
}
extern "C" {
    pub fn __map__is_ool(map: *const map) -> bool;
}
extern "C" {
    pub fn map__has_symbols(map: *const map) -> bool;
}
extern "C" {
    pub fn map__contains_symbol(map: *const map, sym: *const symbol) -> bool;
}

