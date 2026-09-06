//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_lookup_and_delete.c
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

    let mut set_pid: __u32 = 0;
    let mut set_key: __u64 = 0;
    let mut set_value: __u64 = 0;
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 2);
    __type(key, __u64);
    __type(value, __u64);
    } hash_map SEC(".maps");
    SEC("tp/syscalls/sys_enter_getpgid")
#[no_mangle]
pub unsafe extern "C" fn bpf_lookup_and_delete_test(ctx: *const c_void) -> c_int {
    int bpf_lookup_and_delete_test(const void *ctx)
    {
    if (set_pid == bpf_get_current_pid_tgid() >> 32)
    bpf_map_update_elem(&hash_map, &set_key, &set_value, BPF_NOEXIST);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
