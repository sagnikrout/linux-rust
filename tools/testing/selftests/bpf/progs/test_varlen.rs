//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_varlen.c
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
// Copyright (c) 2020 Facebook

pub const MAX_LEN: c_int = 256;
    char buf_in1[MAX_LEN] = {};
    char buf_in2[MAX_LEN] = {};
    let mut test_pid: c_int = 0;
    let mut capture: bool = false;
// .bss
    let mut payload1_len1: __u64 = 0;
    let mut payload1_len2: __u64 = 0;
    let mut total1: __u64 = 0;
    char payload1[MAX_LEN + MAX_LEN] = {};
    let mut ret_bad_read: __u64 = 0;
// .data
    let mut payload2_len1: c_int = -1;
    let mut payload2_len2: c_int = -1;
    let mut total2: c_int = -1;
    char payload2[MAX_LEN + MAX_LEN] = { 1 };
    let mut payload3_len1: c_int = -1;
    let mut payload3_len2: c_int = -1;
    let mut total3: c_int = -1;
    char payload3[MAX_LEN + MAX_LEN] = { 1 };
    let mut payload4_len1: c_int = -1;
    let mut payload4_len2: c_int = -1;
    let mut total4: c_int = -1;
    char payload4[MAX_LEN + MAX_LEN] = { 1 };
    char payload_bad[5] = { 0x42, 0x42, 0x42, 0x42, 0x42 };
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn handler64_unsigned(regs: *mut c_void) -> c_int {
    int handler64_unsigned(void *regs)
    {
    let mut pid: c_int = bpf_get_current_pid_tgid() >> 32;
    void *payload = payload1;
    long len;
// ignore irrelevant invocations
    if (test_pid != pid || !capture)
    return 0;
    len = bpf_probe_read_kernel_str(payload, MAX_LEN, &buf_in1[0]);
    if (len >= 0) {
    payload += len;
    payload1_len1 = len;
    }
    len = bpf_probe_read_kernel_str(payload, MAX_LEN, &buf_in2[0]);
    if (len >= 0) {
    payload += len;
    payload1_len2 = len;
    }
    total1 = payload - (void *)payload1;
    ret_bad_read = bpf_probe_read_kernel_str(payload_bad + 2, 1, (void *) -1);
    return 0;
    }
    SEC("raw_tp/sys_exit")
#[no_mangle]
pub unsafe extern "C" fn handler64_signed(regs: *mut c_void) -> c_int {
    int handler64_signed(void *regs)
    {
    let mut pid: c_int = bpf_get_current_pid_tgid() >> 32;
    void *payload = payload3;
    long len;
// ignore irrelevant invocations
    if (test_pid != pid || !capture)
    return 0;
    len = bpf_probe_read_kernel_str(payload, MAX_LEN, &buf_in1[0]);
    if (len >= 0) {
    payload += len;
    payload3_len1 = len;
    }
    len = bpf_probe_read_kernel_str(payload, MAX_LEN, &buf_in2[0]);
    if (len >= 0) {
    payload += len;
    payload3_len2 = len;
    }
    total3 = payload - (void *)payload3;
    return 0;
    }
    SEC("tp/raw_syscalls/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn handler32_unsigned(regs: *mut c_void) -> c_int {
    int handler32_unsigned(void *regs)
    {
    let mut pid: c_int = bpf_get_current_pid_tgid() >> 32;
    void *payload = payload2;
    u32 len;
// ignore irrelevant invocations
    if (test_pid != pid || !capture)
    return 0;
    len = bpf_probe_read_kernel_str(payload, MAX_LEN, &buf_in1[0]);
    if (len <= MAX_LEN) {
    payload += len;
    payload2_len1 = len;
    }
    len = bpf_probe_read_kernel_str(payload, MAX_LEN, &buf_in2[0]);
    if (len <= MAX_LEN) {
    payload += len;
    payload2_len2 = len;
    }
    total2 = payload - (void *)payload2;
    return 0;
    }
    SEC("tp/raw_syscalls/sys_exit")
#[no_mangle]
pub unsafe extern "C" fn handler32_signed(regs: *mut c_void) -> c_int {
    int handler32_signed(void *regs)
    {
    let mut pid: c_int = bpf_get_current_pid_tgid() >> 32;
    void *payload = payload4;
    long len;
// ignore irrelevant invocations
    if (test_pid != pid || !capture)
    return 0;
    len = bpf_probe_read_kernel_str(payload, MAX_LEN, &buf_in1[0]);
    if (len >= 0) {
    payload += len;
    payload4_len1 = len;
    }
    len = bpf_probe_read_kernel_str(payload, MAX_LEN, &buf_in2[0]);
    if (len >= 0) {
    payload += len;
    payload4_len2 = len;
    }
    total4 = payload - (void *)payload4;
    return 0;
    }
    SEC("tp/syscalls/sys_exit_getpid")
#[no_mangle]
pub unsafe extern "C" fn handler_exit(regs: *mut c_void) -> c_int {
    int handler_exit(void *regs)
    {
    long bla;
    if (bpf_probe_read_kernel(&bla, sizeof(bla), 0))
    return 1;
    else
    return 0;
    }
    char LICENSE[] SEC("license") = "GPL";
