//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/ima.c
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
// Copyright 2020 Google LLC.
//

    let mut monitored_pid: u32 = 0;
    struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 1 << 12);
    } ringbuf SEC(".maps");
    char _license[] SEC("license") = "GPL";
    bool use_ima_file_hash;
    bool enable_bprm_creds_for_exec;
    bool enable_kernel_read_file;
    bool test_deny;
#[no_mangle]
unsafe extern "C" fn ima_test_common(file: *mut file) {
    static void ima_test_common(struct file *file)
    {
    let mut ima_hash: u64 = 0;
    u64 *sample;
    int ret;
    u32 pid;
    pid = bpf_get_current_pid_tgid() >> 32;
    if (pid == monitored_pid) {
    if (!use_ima_file_hash)
    ret = bpf_ima_inode_hash(file.f_inode, &ima_hash,
    sizeof(ima_hash));
    else
    ret = bpf_ima_file_hash(file, &ima_hash,
    sizeof(ima_hash));
    if (ret < 0 || ima_hash == 0)
    return;
    sample = bpf_ringbuf_reserve(&ringbuf, sizeof(u64), 0);
    if (!sample)
    return;
// sample = ima_hash;
    bpf_ringbuf_submit(sample, 0);
    }
    return;
    }
#[no_mangle]
unsafe extern "C" fn ima_test_deny() -> c_int {
    static int ima_test_deny(void)
    {
    u32 pid;
    pid = bpf_get_current_pid_tgid() >> 32;
    if (pid == monitored_pid && test_deny)
    return -EPERM;
    return 0;
    }
    SEC("lsm.s/bprm_committed_creds")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: bprm_committed_creds, bprm: *mut linux_binprm) {
    void BPF_PROG(bprm_committed_creds, struct linux_binprm *bprm)
    {
    ima_test_common(bprm.file);
    }
    SEC("lsm.s/bprm_creds_for_exec")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: bprm_creds_for_exec, bprm: *mut linux_binprm) -> c_int {
    int BPF_PROG(bprm_creds_for_exec, struct linux_binprm *bprm)
    {
    if (!enable_bprm_creds_for_exec)
    return 0;
    ima_test_common(bprm.file);
    return 0;
    }
    SEC("lsm.s/kernel_read_file")
    int BPF_PROG(kernel_read_file, struct file *file, enum kernel_read_file_id id,
    bool contents)
    {
    int ret;
    if (!enable_kernel_read_file)
    return 0;
    if (!contents)
    return 0;
    if (id != READING_POLICY)
    return 0;
    ret = ima_test_deny();
    if (ret < 0)
    return ret;
    ima_test_common(file);
    return 0;
    }
