//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/bpf-utils.h
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


// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)

// libbpf 1.7+ support the btf_dump_type_data_opts.emit_strings option.
pub const HAVE_LIBBPF_STRINGS_SUPPORT: c_int = 1;

//
// Get bpf_prog_info in continuous memory
//
// struct bpf_prog_info has multiple arrays. The user has option to choose
// arrays to fetch from kernel. The following APIs provide an uniform way to
// fetch these data. All arrays in bpf_prog_info are stored in a single
// continuous memory region. This makes it easy to store the info in a
// file.
//
// Before writing perf_bpil to files, it is necessary to
// translate pointers in bpf_prog_info to offsets. Helper functions
// bpil_addr_to_offs() and bpil_offs_to_addr()
// are introduced to switch between pointers and offsets.
//
// Examples:
// # To fetch map_ids and prog_tags:
// __u64 arrays = (1UL << PERF_BPIL_MAP_IDS) |
// (1UL << PERF_BPIL_PROG_TAGS);
// struct perf_bpil *info_linear =
// get_bpf_prog_info_linear(fd, arrays);
//
// # To save data in file
// bpil_addr_to_offs(info_linear);
// write(f, info_linear, sizeof(*info_linear) + info_linear->data_len);
//
// # To read data from file
// read(f, info_linear, <proper_size>);
// bpil_offs_to_addr(info_linear);
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum perf_bpil_array_types {
    PERF_BPIL_FIRST_ARRAY = 0,
    PERF_BPIL_JITED_INSNS = 0,
    PERF_BPIL_XLATED_INSNS,
    PERF_BPIL_MAP_IDS,
    PERF_BPIL_JITED_KSYMS,
    PERF_BPIL_JITED_FUNC_LENS,
    PERF_BPIL_FUNC_INFO,
    PERF_BPIL_LINE_INFO,
    PERF_BPIL_JITED_LINE_INFO,
    PERF_BPIL_PROG_TAGS,
    PERF_BPIL_LAST_ARRAY,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_bpil {
// size of struct bpf_prog_info, when the tool is compiled
    pub info_len: __u32,
// total bytes allocated for data, round up to 8 bytes
    pub data_len: __u32,
// which arrays are included in data
    pub arrays: __u64,
    pub info: bpf_prog_info,
    pub data: [__u8; ],
}

