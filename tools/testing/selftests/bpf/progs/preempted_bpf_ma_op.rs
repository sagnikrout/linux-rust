//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/preempted_bpf_ma_op.c
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
// Copyright (C) 2023. Huawei Technologies Co., Ltd

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bin_data {
    pub data: [c_char; 256],
    pub lock: bpf_spin_lock,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct map_value {
    pub data: *mut *mut bin_data __kptr,
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __type(key, int);
    __type(value, struct map_value);
    __uint(max_entries, 2048);
    } array SEC(".maps");
    char _license[] SEC("license") = "GPL";
    let mut nomem_err: bool = false;
#[no_mangle]
unsafe extern "C" fn del_array(i: c_uint, from: *mut c_int) -> c_int {
    static int del_array(unsigned int i, int *from)
    {
    struct map_value *value;
    struct bin_data *old;
    value = bpf_map_lookup_elem(&array, from);
    if (!value)
    return 1;
    old = bpf_kptr_xchg(&value.data, core::ptr::null_mut());
    if (old)
    bpf_obj_drop(old);
    (*from)++;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn add_array(i: c_uint, from: *mut c_int) -> c_int {
    static int add_array(unsigned int i, int *from)
    {
    struct bin_data *old, *new;
    struct map_value *value;
    value = bpf_map_lookup_elem(&array, from);
    if (!value)
    return 1;
    new = bpf_obj_new(typeof(*new));
    if (!new) {
    nomem_err = true;
    return 1;
    }
    old = bpf_kptr_xchg(&value.data, new);
    if (old)
    bpf_obj_drop(old);
    (*from)++;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn del_then_add_array(from: c_int) {
    static void del_then_add_array(int from)
    {
    int i;
    i = from;
    bpf_loop(512, del_array, &i, 0);
    i = from;
    bpf_loop(512, add_array, &i, 0);
    }
    SEC("fentry/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG2(_arg: test0, _arg: c_int, _arg: a) -> c_int {
    int BPF_PROG2(test0, int, a)
    {
    del_then_add_array(0);
    return 0;
    }
    SEC("fentry/bpf_fentry_test2")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG2(_arg: test1, _arg: c_int, _arg: a, _arg: u64, _arg: b) -> c_int {
    int BPF_PROG2(test1, int, a, u64, b)
    {
    del_then_add_array(512);
    return 0;
    }
    SEC("fentry/bpf_fentry_test3")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG2(_arg: test2, _arg: c_char, _arg: a, _arg: c_int, _arg: b, _arg: u64, _arg: c) -> c_int {
    int BPF_PROG2(test2, char, a, int, b, u64, c)
    {
    del_then_add_array(1024);
    return 0;
    }
    SEC("fentry/bpf_fentry_test4")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG2(_arg: test3, : *mut c_void, _arg: a, _arg: c_char, _arg: b, _arg: c_int, _arg: c, _arg: u64, _arg: d) -> c_int {
    int BPF_PROG2(test3, void *, a, char, b, int, c, u64, d)
    {
    del_then_add_array(1536);
    return 0;
    }
