//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/progs/linked_list.h
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
pub struct bar {
    pub node: bpf_list_node,
    pub data: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct foo {
    pub node: bpf_list_node,
    pub node): bpf_list_head head __contains(bar,,
    pub lock: bpf_spin_lock,
    pub data: c_int,
    pub node2: bpf_list_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct map_value {
    pub lock: bpf_spin_lock,
    pub data: c_int,
    pub node2): bpf_list_head head __contains(foo,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct array_map {
    pub BPF_MAP_TYPE_ARRAY): __uint(type,,
    pub int): __type(key,,
    pub map_value): __type(value, struct,
    pub 1): __uint(max_entries,,
}

extern "C" {
    pub fn SEC(_arg: ".maps") -> array_map array_map;
}
extern "C" {
    pub fn SEC(_arg: ".maps") -> array_map inner_map;
}

