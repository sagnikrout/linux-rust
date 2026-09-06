//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_verify_pkcs7_sig.c
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

pub const MAX_SIG_SIZE: c_int = 1024;
    __u32 monitored_pid;
    __s32 user_keyring_serial;
    __u64 system_keyring_id;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct data {
    pub data: [__u8; MAX_DATA_SIZE],
    pub data_len: __u32,
    pub sig: [__u8; MAX_SIG_SIZE],
    pub sig_len: __u32,
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, struct data);
    } data_input SEC(".maps");
    char _license[] SEC("license") = "GPL";
    SEC("lsm.s/bpf")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: bpf, cmd: c_int, attr: *mut union bpf_attr, size: c_uint, kernel: bool) -> c_int {
    int BPF_PROG(bpf, int cmd, union bpf_attr *attr, unsigned int size, bool kernel)
    {
    struct bpf_dynptr data_ptr, sig_ptr;
    struct data *data_val;
    struct bpf_key *trusted_keyring;
    __u32 pid;
    __u64 value;
    int ret, zero = 0;
    pid = bpf_get_current_pid_tgid() >> 32;
    if (pid != monitored_pid)
    return 0;
    data_val = bpf_map_lookup_elem(&data_input, &zero);
    if (!data_val)
    return 0;
    ret = bpf_probe_read_kernel(&value, sizeof(value), &attr.value);
    if (ret)
    goto out;
    ret = bpf_copy_from_user(data_val, sizeof(struct data),
    (void *)(unsigned long)value);
    if (ret)
    goto out;
    if (data_val.data_len > sizeof(data_val.data))
    return -EINVAL;
    bpf_dynptr_from_mem(data_val.data, data_val.data_len, 0, &data_ptr);
    if (data_val.sig_len > sizeof(data_val.sig))
    return -EINVAL;
    bpf_dynptr_from_mem(data_val.sig, data_val.sig_len, 0, &sig_ptr);
    if (user_keyring_serial)
    trusted_keyring = bpf_lookup_user_key(user_keyring_serial, 0);
    else
    trusted_keyring = bpf_lookup_system_key(system_keyring_id);
    if (!trusted_keyring)
    return -ENOENT;
    ret = bpf_verify_pkcs7_signature(&data_ptr, &sig_ptr, trusted_keyring);
    bpf_key_put(trusted_keyring);
    out:
    set_if_not_errno_or_zero(ret, -EFAULT);
    return ret;
    }
