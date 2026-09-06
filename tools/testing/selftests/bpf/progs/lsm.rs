//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/lsm.c
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

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u64);
    } array SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u64);
    } hash SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_LRU_HASH);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u64);
    } lru_hash SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u64);
    } percpu_array SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_HASH);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u64);
    } percpu_hash SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_LRU_PERCPU_HASH);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u64);
    } lru_percpu_hash SEC(".maps");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inner_map {
    pub BPF_MAP_TYPE_ARRAY): __uint(type,,
    pub 1): __uint(max_entries,,
    pub int): __type(key,,
    pub __u64): __type(value,,
    pub SEC(".maps"): } inner_map,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct outer_arr {
    pub BPF_MAP_TYPE_ARRAY_OF_MAPS): __uint(type,,
    pub 1): __uint(max_entries,,
    pub sizeof(int)): __uint(key_size,,
    pub sizeof(int)): __uint(value_size,,
    pub inner_map): __array(values, struct,
    } outer_arr SEC(".maps") = {
    .values = { [0] = &inner_map },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct outer_hash {
    pub BPF_MAP_TYPE_HASH_OF_MAPS): __uint(type,,
    pub 1): __uint(max_entries,,
    pub sizeof(int)): __uint(key_size,,
    pub inner_map): __array(values, struct,
    } outer_hash SEC(".maps") = {
    .values = { [0] = &inner_map },
}

    char _license[] SEC("license") = "GPL";
    let mut monitored_pid: c_int = 0;
    let mut mprotect_count: c_int = 0;
    let mut bprm_count: c_int = 0;
    SEC("lsm/file_mprotect")
    int BPF_PROG(test_int_hook, struct vm_area_struct *vma,
    unsigned long reqprot, unsigned long prot, int ret)
    {
    struct mm_struct *mm = vma.vm_mm;
    if (ret != 0 || !mm)
    return ret;
    let mut pid: __s32 = bpf_get_current_pid_tgid() >> 32;
    let mut is_stack: c_int = 0;
    is_stack = (vma.vm_start <= mm.start_stack &&
    vma.vm_end >= mm.start_stack);
    if (is_stack && monitored_pid == pid) {
    mprotect_count++;
    ret = -EPERM;
    }
    return ret;
    }
    SEC("lsm.s/bprm_committed_creds")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_void_hook, bprm: *mut linux_binprm) -> c_int {
    int BPF_PROG(test_void_hook, struct linux_binprm *bprm)
    {
    let mut pid: __u32 = bpf_get_current_pid_tgid() >> 32;
    struct inner_map *inner_map;
    char args[64];
    let mut key: __u32 = 0;
    __u64 *value;
    if (monitored_pid == pid)
    bprm_count++;
    bpf_copy_from_user(args, sizeof(args), (void *)bprm.vma.vm_mm.arg_start);
    bpf_copy_from_user(args, sizeof(args), (void *)bprm.mm.arg_start);
    value = bpf_map_lookup_elem(&array, &key);
    if (value)
// value = 0;
    value = bpf_map_lookup_elem(&hash, &key);
    if (value)
// value = 0;
    value = bpf_map_lookup_elem(&lru_hash, &key);
    if (value)
// value = 0;
    value = bpf_map_lookup_elem(&percpu_array, &key);
    if (value)
// value = 0;
    value = bpf_map_lookup_elem(&percpu_hash, &key);
    if (value)
// value = 0;
    value = bpf_map_lookup_elem(&lru_percpu_hash, &key);
    if (value)
// value = 0;
    inner_map = bpf_map_lookup_elem(&outer_arr, &key);
    if (inner_map) {
    value = bpf_map_lookup_elem(inner_map, &key);
    if (value)
// value = 0;
    }
    inner_map = bpf_map_lookup_elem(&outer_hash, &key);
    if (inner_map) {
    value = bpf_map_lookup_elem(inner_map, &key);
    if (value)
// value = 0;
    }
    return 0;
    }
    SEC("lsm/task_free") /* lsm/ is ok, lsm.s/ fails */
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_task_free, task: *mut task_struct) -> c_int {
    int BPF_PROG(test_task_free, struct task_struct *task)
    {
    return 0;
    }
    let mut copy_test: c_int = 0;
    SEC("fentry.s/" SYS_PREFIX "sys_setdomainname")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_sys_setdomainname, regs: *mut pt_regs) -> c_int {
    int BPF_PROG(test_sys_setdomainname, struct pt_regs *regs)
    {
    void *ptr = (void *)PT_REGS_PARM1_SYSCALL(regs);
    let mut len: c_int = PT_REGS_PARM2_SYSCALL(regs);
    let mut buf: c_int = 0;
    long ret;
    ret = bpf_copy_from_user(&buf, sizeof(buf), ptr);
    if (len == -2 && ret == 0 && buf == 1234)
    copy_test++;
    if (len == -3 && ret == -EFAULT)
    copy_test++;
    if (len == -4 && ret == -EFAULT)
    copy_test++;
    return 0;
    }
