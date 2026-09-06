//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_core_reloc_kernel.c
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
// Copyright (c) 2019 Facebook

    char _license[] SEC("license") = "GPL";
    struct {
    char in[256];
    char out[256];
    bool skip;
    uint64_t my_pid_tgid;
    } data = {};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_kernel_output {
    pub valid: [c_int; 10],
// we have test_progs[-flavor], so cut flavor part
    pub comm: [c_char; sizeof("test_progs")],
    pub comm_len: c_int,
    pub local_task_struct_matches: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct {
    pub pid: c_int,
    pub tgid: c_int,
    pub comm: [c_char; 16],
    pub group_leader: *mut task_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm_struct___wrong {
    pub abc_whatever_should_not_exist: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct___local {
    pub pid: c_int,
    pub mm: *mut mm_struct___wrong,
}

    SEC("raw_tracepoint/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn test_core_kernel(ctx: *mut c_void) -> c_int {
    int test_core_kernel(void *ctx)
    {
// Support for the BPF_TYPE_MATCHES argument to the
// __builtin_preserve_type_info builtin was added at some point during
// development of clang 15 and it's what we require for this test.
//

    struct task_struct *task = (void *)bpf_get_current_task();
    struct core_reloc_kernel_output *out = (void *)&data.out;
    let mut pid_tgid: u64 = bpf_get_current_pid_tgid();
    let mut real_tgid: i32 = (int32_t)pid_tgid;
    int pid, tgid;
    if (data.my_pid_tgid != pid_tgid)
    return 0;
    if (CORE_READ(&pid, &task.pid) ||
    CORE_READ(&tgid, &task.tgid))
    return 1;
// validate pid + tgid matches
    out.valid[0] = (((uint64_t)pid << 32) | tgid) == pid_tgid;
// test variadic BPF_CORE_READ macros
    out.valid[1] = BPF_CORE_READ(task,
    tgid) == real_tgid;
    out.valid[2] = BPF_CORE_READ(task,
    group_leader,
    tgid) == real_tgid;
    out.valid[3] = BPF_CORE_READ(task,
    group_leader, group_leader,
    tgid) == real_tgid;
    out.valid[4] = BPF_CORE_READ(task,
    group_leader, group_leader, group_leader,
    tgid) == real_tgid;
    out.valid[5] = BPF_CORE_READ(task,
    group_leader, group_leader, group_leader,
    group_leader,
    tgid) == real_tgid;
    out.valid[6] = BPF_CORE_READ(task,
    group_leader, group_leader, group_leader,
    group_leader, group_leader,
    tgid) == real_tgid;
    out.valid[7] = BPF_CORE_READ(task,
    group_leader, group_leader, group_leader,
    group_leader, group_leader, group_leader,
    tgid) == real_tgid;
    out.valid[8] = BPF_CORE_READ(task,
    group_leader, group_leader, group_leader,
    group_leader, group_leader, group_leader,
    group_leader,
    tgid) == real_tgid;
    out.valid[9] = BPF_CORE_READ(task,
    group_leader, group_leader, group_leader,
    group_leader, group_leader, group_leader,
    group_leader, group_leader,
    tgid) == real_tgid;
// test BPF_CORE_READ_STR_INTO() returns correct code and contents
    out.comm_len = BPF_CORE_READ_STR_INTO(
    &out.comm, task,
    group_leader, group_leader, group_leader, group_leader,
    group_leader, group_leader, group_leader, group_leader,
    comm);
    out.local_task_struct_matches = bpf_core_type_matches(struct task_struct___local);

    data.skip = true;

    return 0;
    }
