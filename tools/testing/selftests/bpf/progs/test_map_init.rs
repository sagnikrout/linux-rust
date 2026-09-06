//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_map_init.c
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
// Copyright (c) 2020 Tessares SA <http://www.tessares.net>

    let mut inKey: __u64 = 0;
    let mut inValue: __u64 = 0;
    let mut inPid: __u32 = 0;
    struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_HASH);
    __uint(max_entries, 2);
    __type(key, __u64);
    __type(value, __u64);
    } hashmap1 SEC(".maps");
    SEC("tp/syscalls/sys_enter_getpgid")
#[no_mangle]
pub unsafe extern "C" fn sysenter_getpgid(ctx: *const c_void) -> c_int {
    int sysenter_getpgid(const void *ctx)
    {
// Just do it for once, when called from our own test prog. This
// ensures the map value is only updated for a single CPU.
//
    let mut cur_pid: c_int = bpf_get_current_pid_tgid() >> 32;
    if (cur_pid == inPid)
    bpf_map_update_elem(&hashmap1, &inKey, &inValue, BPF_NOEXIST);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
