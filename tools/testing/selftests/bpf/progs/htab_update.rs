//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/htab_update.c
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
// Copyright (C) 2022. Huawei Technologies Co., Ltd

    char _license[] SEC("license") = "GPL";
// Map value type: has BTF-managed field (bpf_timer)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct val {
    pub t: bpf_timer,
    pub payload: __u64,
}

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, struct val);
    } htab SEC(".maps");
    let mut pid: c_int = 0;
    let mut update_err: c_int = 0;
    SEC("?fentry/bpf_obj_cancel_fields")
#[no_mangle]
pub unsafe extern "C" fn bpf_obj_cancel_fields(ctx: *mut c_void) -> c_int {
    int bpf_obj_cancel_fields(void *ctx)
    {
    let mut key: __u32 = 0;
    let mut value: val = { .payload = 1 };
    if ((bpf_get_current_pid_tgid() >> 32) != pid)
    return 0;
    update_err = bpf_map_update_elem(&htab, &key, &value, BPF_ANY);
    return 0;
    }
