//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/trace_helpers.h
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
pub struct ksym {
    pub addr: c_long,
    pub name: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksyms {
    pub syms: *mut ksym,
    pub sym_cap: usize,
    pub sym_cnt: usize,
    pub filtered_syms: *mut c_char,
    pub filtered_cnt: usize,
}

extern "C" {
    pub fn int(p1: *const *const ksym_cmp_t)(void, p2: *const c_void) -> typedef;
}
extern "C" {
    pub fn int(p1: *const *const ksym_search_cmp_t)(void, p2: *const ksym) -> typedef;
}
extern "C" {
    pub fn load_kallsyms() -> c_int;
}
extern "C" {
    pub fn ksym_get_addr(name: *const c_char) -> c_long;
}
extern "C" {
    pub fn ksym_get_addr_local(ksyms: *mut ksyms, name: *const c_char) -> c_long;
}
extern "C" {
    pub fn free_kallsyms_local(ksyms: *mut ksyms);
}
// open kallsyms and find addresses on the fly, faster than load + search.
extern "C" {
    pub fn kallsyms_find(sym: *const c_char, addr: *mut c_ulonglong) -> c_int;
}
extern "C" {
    pub fn read_trace_pipe();
}
extern "C" {
    pub fn get_uprobe_offset(addr: *const c_void) -> isize;
}
extern "C" {
    pub fn get_rel_offset(addr: uintptr_t) -> isize;
}
extern "C" {
    pub fn read_build_id(path: *const c_char, build_id: *mut c_char, size: usize) -> c_int;
}
extern "C" {
    pub fn bpf_get_ksyms(ksymsp: *mut ksyms, kernel: bool) -> c_int;
}
extern "C" {
    pub fn bpf_get_addrs(addrsp: *mut c_ulong, cntp: *mut usize, kernel: bool) -> c_int;
}
extern "C" {
    pub fn is_unsafe_function(name: *const c_char) -> bool;
}
