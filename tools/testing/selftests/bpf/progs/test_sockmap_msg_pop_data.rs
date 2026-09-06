//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_sockmap_msg_pop_data.c
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

    struct {
    __uint(type, BPF_MAP_TYPE_SOCKMAP);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, int);
    } sock_map SEC(".maps");
pub const POP_START: c_uint = 0x48a3;
pub const POP_LEN: c_uint = 0xfffffffd;
    let mut pop_data_ret: c_long = 1;
    SEC("sk_msg")
#[no_mangle]
pub unsafe extern "C" fn prog_msg_pop_data(msg: *mut sk_msg_md) -> c_int {
    int prog_msg_pop_data(struct sk_msg_md *msg)
    {
    if (msg.size <= POP_START)
    return SK_PASS;
    pop_data_ret = bpf_msg_pop_data(msg, POP_START, POP_LEN, 0);
    return SK_PASS;
    }
    char _license[] SEC("license") = "GPL";
