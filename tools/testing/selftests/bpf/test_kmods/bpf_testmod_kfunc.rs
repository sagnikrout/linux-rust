//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/test_kmods/bpf_testmod_kfunc.h
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

// Macro flag: #define __ksym
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prog_test_member1 {
    pub a: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prog_test_member {
    pub m: prog_test_member1,
    pub c: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prog_test_ref_kfunc {
    pub a: c_int,
    pub b: c_int,
    pub memb: prog_test_member,
    pub next: *mut prog_test_ref_kfunc,
    pub cnt: refcount_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prog_test_pass1 {
    pub x0: c_int,
    pub x1: c_int,
    pub x2: c_int,
    pub x3: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prog_test_pass2 {
    pub len: c_int,
    pub arr1: [c_short; 4],
    pub arr2: [c_char; 4],
    pub arr3: [c_ulong; 8],
    pub x: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prog_test_big_arg {
    pub a: __u64,
    pub b: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prog_test_fail1 {
    pub p: *mut c_void,
    pub x: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prog_test_fail2 {
    pub x8: c_int,
    pub x: prog_test_pass1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prog_test_fail3 {
    pub len: c_int,
    pub arr1: [c_char; 2],
    pub arr2: [c_char; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct init_sock_args {
    pub af: c_int,
    pub type: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct addr_args {
    pub __kernel_sockaddr_storage)]: char addr[sizeof(struct,
    pub addrlen: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sendmsg_args {
    pub addr: addr_args,
    pub msg: [c_char; 10],
    pub msglen: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_testmod_ctx {
    pub rcu: callback_head,
    pub usage: refcount_t,
}

// The bpf_kfunc_call_test_static_unused_arg is defined as static,
// but bpf program compilation needs to see it as global symbol.
//

extern "C" {
    pub fn bpf_kfunc_call_test_offset(p: *mut prog_test_ref_kfunc);
}
extern "C" {
    pub fn bpf_kfunc_call_memb1_release(p: *mut prog_test_member1);
}
extern "C" {
    pub fn bpf_kfunc_call_test_fail1(p: *mut prog_test_fail1);
}
extern "C" {
    pub fn bpf_kfunc_call_test_fail2(p: *mut prog_test_fail2);
}
extern "C" {
    pub fn bpf_kfunc_call_test_fail3(p: *mut prog_test_fail3);
}
extern "C" {
    pub fn bpf_kfunc_call_test_mem_len_fail1(mem: *mut c_void, len: c_int);
}

extern "C" {
    pub fn bpf_testmod_test_hardirq_fn();
}
extern "C" {
    pub fn bpf_testmod_test_softirq_fn();
}
