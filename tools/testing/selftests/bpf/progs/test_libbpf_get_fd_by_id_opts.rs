//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_libbpf_get_fd_by_id_opts.c
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
// Copyright (C) 2022 Huawei Technologies Duesseldorf GmbH
//
// Author: Roberto Sassu <roberto.sassu@huawei.com>
//

// From include/linux/mm.h.
pub const FMODE_WRITE: c_uint = 0x2;
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u32);
    } data_input SEC(".maps");
    char _license[] SEC("license") = "GPL";
    SEC("lsm/bpf_map")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: check_access, map: *mut bpf_map, fmode: fmode_t) -> c_int {
    int BPF_PROG(check_access, struct bpf_map *map, fmode_t fmode)
    {
    if (map != (struct bpf_map *)&data_input)
    return 0;
    if (fmode & FMODE_WRITE)
    return -EACCES;
    barrier();
    return 0;
    }
