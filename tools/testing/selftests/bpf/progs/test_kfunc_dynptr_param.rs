//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_kfunc_dynptr_param.c
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

    struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 4096);
    } ringbuf SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u32);
    } array_map SEC(".maps");
    int err, pid;
    char _license[] SEC("license") = "GPL";
    SEC("?lsm.s/bpf")
#[no_mangle]
pub unsafe extern "C" fn __msg(offset=-8": "cannot pass in dynptr at an) -> __failure {
    __failure __msg("cannot pass in dynptr at an offset=-8")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: not_valid_dynptr, cmd: c_int, attr: *mut union bpf_attr, size: c_uint, kernel: bool) -> c_int {
    int BPF_PROG(not_valid_dynptr, int cmd, union bpf_attr *attr, unsigned int size, bool kernel)
    {
    let mut val: c_ulong = 0;
    return bpf_verify_pkcs7_signature((struct bpf_dynptr *)&val,
    (struct bpf_dynptr *)&val, core::ptr::null_mut());
    }
    SEC("?lsm.s/bpf")
#[no_mangle]
pub unsafe extern "C" fn __msg(bpf_dynptr": "R1 expected pointer to stack or struct) -> __failure {
    __failure __msg("R1 expected pointer to stack or const struct bpf_dynptr")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: not_ptr_to_stack, cmd: c_int, attr: *mut union bpf_attr, size: c_uint, kernel: bool) -> c_int {
    int BPF_PROG(not_ptr_to_stack, int cmd, union bpf_attr *attr, unsigned int size, bool kernel)
    {
    static struct bpf_dynptr val;
    return bpf_verify_pkcs7_signature(&val, &val, core::ptr::null_mut());
    }
    SEC("lsm.s/bpf")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: dynptr_data_null, cmd: c_int, attr: *mut union bpf_attr, size: c_uint, kernel: bool) -> c_int {
    int BPF_PROG(dynptr_data_null, int cmd, union bpf_attr *attr, unsigned int size, bool kernel)
    {
    struct bpf_key *trusted_keyring;
    struct bpf_dynptr ptr;
    __u32 *value;
    int ret, zero = 0;
    if (bpf_get_current_pid_tgid() >> 32 != pid)
    return 0;
    value = bpf_map_lookup_elem(&array_map, &zero);
    if (!value)
    return 0;
// Pass invalid flags.
    ret = bpf_dynptr_from_mem(value, sizeof(*value), ((__u64)~0ULL), &ptr);
    if (ret != -EINVAL)
    return 0;
    trusted_keyring = bpf_lookup_system_key(0);
    if (!trusted_keyring)
    return 0;
    err = bpf_verify_pkcs7_signature(&ptr, &ptr, trusted_keyring);
    bpf_key_put(trusted_keyring);
    return 0;
    }
