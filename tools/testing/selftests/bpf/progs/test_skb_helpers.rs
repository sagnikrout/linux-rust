//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_skb_helpers.c
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


// SPDX-License-Identifier: GPL-2.0-only

pub const TEST_COMM_LEN: c_int = 16;
    struct {
    __uint(type, BPF_MAP_TYPE_CGROUP_ARRAY);
    __uint(max_entries, 1);
    __type(key, u32);
    __type(value, u32);
    } cgroup_map SEC(".maps");
    char _license[] SEC("license") = "GPL";
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_skb_helpers(skb: *mut __sk_buff) -> c_int {
    int test_skb_helpers(struct __sk_buff *skb)
    {
    struct task_struct *task;
    char comm[TEST_COMM_LEN];
    __u32 tpid;
    task = (struct task_struct *)bpf_get_current_task();
    bpf_probe_read_kernel(&tpid , sizeof(tpid), &task.tgid);
    bpf_probe_read_kernel_str(&comm, sizeof(comm), &task.comm);
    return 0;
    }
