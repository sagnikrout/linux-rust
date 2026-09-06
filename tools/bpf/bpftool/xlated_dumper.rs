//! Automatically rewritten from C Header to Rust Module
//! Source: tools/bpf/bpftool/xlated_dumper.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2018 Netronome Systems, Inc.
pub const SYM_MAX_NAME: c_int = 256;
pub const MODULE_MAX_NAME: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernel_sym {
    pub address: c_ulong,
    pub name: [c_char; SYM_MAX_NAME],
    pub module: [c_char; MODULE_MAX_NAME],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dump_data {
    pub address_call_base: c_ulong,
    pub sym_mapping: *mut kernel_sym,
    pub sym_count: __u32,
    pub jited_ksyms: *mut __u64,
    pub nr_jited_ksyms: __u32,
    pub btf: *mut btf,
    pub func_info: *mut c_void,
    pub finfo_rec_size: __u32,
    pub prog_linfo: *const bpf_prog_linfo,
    pub 8]: char scratch_buff[SYM_MAX_NAME +,
}

extern "C" {
    pub fn kernel_syms_load(dd: *mut dump_data);
}
extern "C" {
    pub fn kernel_syms_destroy(dd: *mut dump_data);
}
