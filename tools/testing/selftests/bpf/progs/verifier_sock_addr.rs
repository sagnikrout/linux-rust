//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_sock_addr.c
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
// Copyright (c) 2024 Google LLC

    SEC("cgroup/recvmsg4")
    __success
#[no_mangle]
pub unsafe extern "C" fn recvmsg4_good_return_code(ctx: *mut bpf_sock_addr) -> c_int {
    int recvmsg4_good_return_code(struct bpf_sock_addr *ctx)
    {
    return 1;
    }
    SEC("cgroup/recvmsg4")
#[no_mangle]
pub unsafe extern "C" fn __msg([1: "At program exit the register R0 has smin=0 smax=0 should have been in, _arg: 1]") -> __failure {
    __failure __msg("At program exit the register R0 has smin=0 smax=0 should have been in [1, 1]")
#[no_mangle]
pub unsafe extern "C" fn recvmsg4_bad_return_code(ctx: *mut bpf_sock_addr) -> c_int {
    int recvmsg4_bad_return_code(struct bpf_sock_addr *ctx)
    {
    return 0;
    }
    SEC("cgroup/recvmsg6")
    __success
#[no_mangle]
pub unsafe extern "C" fn recvmsg6_good_return_code(ctx: *mut bpf_sock_addr) -> c_int {
    int recvmsg6_good_return_code(struct bpf_sock_addr *ctx)
    {
    return 1;
    }
    SEC("cgroup/recvmsg6")
#[no_mangle]
pub unsafe extern "C" fn __msg([1: "At program exit the register R0 has smin=0 smax=0 should have been in, _arg: 1]") -> __failure {
    __failure __msg("At program exit the register R0 has smin=0 smax=0 should have been in [1, 1]")
#[no_mangle]
pub unsafe extern "C" fn recvmsg6_bad_return_code(ctx: *mut bpf_sock_addr) -> c_int {
    int recvmsg6_bad_return_code(struct bpf_sock_addr *ctx)
    {
    return 0;
    }
    SEC("cgroup/recvmsg_unix")
    __success
#[no_mangle]
pub unsafe extern "C" fn recvmsg_unix_good_return_code(ctx: *mut bpf_sock_addr) -> c_int {
    int recvmsg_unix_good_return_code(struct bpf_sock_addr *ctx)
    {
    return 1;
    }
    SEC("cgroup/recvmsg_unix")
#[no_mangle]
pub unsafe extern "C" fn __msg([1: "At program exit the register R0 has smin=0 smax=0 should have been in, _arg: 1]") -> __failure {
    __failure __msg("At program exit the register R0 has smin=0 smax=0 should have been in [1, 1]")
#[no_mangle]
pub unsafe extern "C" fn recvmsg_unix_bad_return_code(ctx: *mut bpf_sock_addr) -> c_int {
    int recvmsg_unix_bad_return_code(struct bpf_sock_addr *ctx)
    {
    return 0;
    }
    SEC("cgroup/sendmsg4")
    __success
#[no_mangle]
pub unsafe extern "C" fn sendmsg4_good_return_code_0(ctx: *mut bpf_sock_addr) -> c_int {
    int sendmsg4_good_return_code_0(struct bpf_sock_addr *ctx)
    {
    return 0;
    }
    SEC("cgroup/sendmsg4")
    __success
#[no_mangle]
pub unsafe extern "C" fn sendmsg4_good_return_code_1(ctx: *mut bpf_sock_addr) -> c_int {
    int sendmsg4_good_return_code_1(struct bpf_sock_addr *ctx)
    {
    return 1;
    }
    SEC("cgroup/sendmsg4")
#[no_mangle]
pub unsafe extern "C" fn __msg([0: "At program exit the register R0 has smin=2 smax=2 should have been in, _arg: 1]") -> __failure {
    __failure __msg("At program exit the register R0 has smin=2 smax=2 should have been in [0, 1]")
#[no_mangle]
pub unsafe extern "C" fn sendmsg4_bad_return_code(ctx: *mut bpf_sock_addr) -> c_int {
    int sendmsg4_bad_return_code(struct bpf_sock_addr *ctx)
    {
    return 2;
    }
    SEC("cgroup/sendmsg6")
    __success
#[no_mangle]
pub unsafe extern "C" fn sendmsg6_good_return_code_0(ctx: *mut bpf_sock_addr) -> c_int {
    int sendmsg6_good_return_code_0(struct bpf_sock_addr *ctx)
    {
    return 0;
    }
    SEC("cgroup/sendmsg6")
    __success
#[no_mangle]
pub unsafe extern "C" fn sendmsg6_good_return_code_1(ctx: *mut bpf_sock_addr) -> c_int {
    int sendmsg6_good_return_code_1(struct bpf_sock_addr *ctx)
    {
    return 1;
    }
    SEC("cgroup/sendmsg6")
#[no_mangle]
pub unsafe extern "C" fn __msg([0: "At program exit the register R0 has smin=2 smax=2 should have been in, _arg: 1]") -> __failure {
    __failure __msg("At program exit the register R0 has smin=2 smax=2 should have been in [0, 1]")
#[no_mangle]
pub unsafe extern "C" fn sendmsg6_bad_return_code(ctx: *mut bpf_sock_addr) -> c_int {
    int sendmsg6_bad_return_code(struct bpf_sock_addr *ctx)
    {
    return 2;
    }
    SEC("cgroup/sendmsg_unix")
    __success
#[no_mangle]
pub unsafe extern "C" fn sendmsg_unix_good_return_code_0(ctx: *mut bpf_sock_addr) -> c_int {
    int sendmsg_unix_good_return_code_0(struct bpf_sock_addr *ctx)
    {
    return 0;
    }
    SEC("cgroup/sendmsg_unix")
    __success
#[no_mangle]
pub unsafe extern "C" fn sendmsg_unix_good_return_code_1(ctx: *mut bpf_sock_addr) -> c_int {
    int sendmsg_unix_good_return_code_1(struct bpf_sock_addr *ctx)
    {
    return 1;
    }
    SEC("cgroup/sendmsg_unix")
#[no_mangle]
pub unsafe extern "C" fn __msg([0: "At program exit the register R0 has smin=2 smax=2 should have been in, _arg: 1]") -> __failure {
    __failure __msg("At program exit the register R0 has smin=2 smax=2 should have been in [0, 1]")
#[no_mangle]
pub unsafe extern "C" fn sendmsg_unix_bad_return_code(ctx: *mut bpf_sock_addr) -> c_int {
    int sendmsg_unix_bad_return_code(struct bpf_sock_addr *ctx)
    {
    return 2;
    }
    SEC("cgroup/getpeername4")
    __success
#[no_mangle]
pub unsafe extern "C" fn getpeername4_good_return_code(ctx: *mut bpf_sock_addr) -> c_int {
    int getpeername4_good_return_code(struct bpf_sock_addr *ctx)
    {
    return 1;
    }
    SEC("cgroup/getpeername4")
#[no_mangle]
pub unsafe extern "C" fn __msg([1: "At program exit the register R0 has smin=0 smax=0 should have been in, _arg: 1]") -> __failure {
    __failure __msg("At program exit the register R0 has smin=0 smax=0 should have been in [1, 1]")
#[no_mangle]
pub unsafe extern "C" fn getpeername4_bad_return_code(ctx: *mut bpf_sock_addr) -> c_int {
    int getpeername4_bad_return_code(struct bpf_sock_addr *ctx)
    {
    return 0;
    }
    SEC("cgroup/getpeername6")
    __success
#[no_mangle]
pub unsafe extern "C" fn getpeername6_good_return_code(ctx: *mut bpf_sock_addr) -> c_int {
    int getpeername6_good_return_code(struct bpf_sock_addr *ctx)
    {
    return 1;
    }
    SEC("cgroup/getpeername6")
#[no_mangle]
pub unsafe extern "C" fn __msg([1: "At program exit the register R0 has smin=0 smax=0 should have been in, _arg: 1]") -> __failure {
    __failure __msg("At program exit the register R0 has smin=0 smax=0 should have been in [1, 1]")
#[no_mangle]
pub unsafe extern "C" fn getpeername6_bad_return_code(ctx: *mut bpf_sock_addr) -> c_int {
    int getpeername6_bad_return_code(struct bpf_sock_addr *ctx)
    {
    return 0;
    }
    SEC("cgroup/getpeername_unix")
    __success
#[no_mangle]
pub unsafe extern "C" fn getpeername_unix_good_return_code(ctx: *mut bpf_sock_addr) -> c_int {
    int getpeername_unix_good_return_code(struct bpf_sock_addr *ctx)
    {
    return 1;
    }
    SEC("cgroup/getpeername_unix")
#[no_mangle]
pub unsafe extern "C" fn __msg([1: "At program exit the register R0 has smin=0 smax=0 should have been in, _arg: 1]") -> __failure {
    __failure __msg("At program exit the register R0 has smin=0 smax=0 should have been in [1, 1]")
#[no_mangle]
pub unsafe extern "C" fn getpeername_unix_bad_return_code(ctx: *mut bpf_sock_addr) -> c_int {
    int getpeername_unix_bad_return_code(struct bpf_sock_addr *ctx)
    {
    return 0;
    }
    SEC("cgroup/getsockname4")
    __success
#[no_mangle]
pub unsafe extern "C" fn getsockname4_good_return_code(ctx: *mut bpf_sock_addr) -> c_int {
    int getsockname4_good_return_code(struct bpf_sock_addr *ctx)
    {
    return 1;
    }
    SEC("cgroup/getsockname4")
#[no_mangle]
pub unsafe extern "C" fn __msg([1: "At program exit the register R0 has smin=0 smax=0 should have been in, _arg: 1]") -> __failure {
    __failure __msg("At program exit the register R0 has smin=0 smax=0 should have been in [1, 1]")
#[no_mangle]
pub unsafe extern "C" fn getsockname4_bad_return_code(ctx: *mut bpf_sock_addr) -> c_int {
    int getsockname4_bad_return_code(struct bpf_sock_addr *ctx)
    {
    return 0;
    }
    SEC("cgroup/getsockname6")
    __success
#[no_mangle]
pub unsafe extern "C" fn getsockname6_good_return_code(ctx: *mut bpf_sock_addr) -> c_int {
    int getsockname6_good_return_code(struct bpf_sock_addr *ctx)
    {
    return 1;
    }
    SEC("cgroup/getsockname6")
#[no_mangle]
pub unsafe extern "C" fn __msg([1: "At program exit the register R0 has smin=0 smax=0 should have been in, _arg: 1]") -> __failure {
    __failure __msg("At program exit the register R0 has smin=0 smax=0 should have been in [1, 1]")
#[no_mangle]
pub unsafe extern "C" fn getsockname6_bad_return_code(ctx: *mut bpf_sock_addr) -> c_int {
    int getsockname6_bad_return_code(struct bpf_sock_addr *ctx)
    {
    return 0;
    }
    SEC("cgroup/getsockname_unix")
    __success
#[no_mangle]
pub unsafe extern "C" fn getsockname_unix_good_return_code(ctx: *mut bpf_sock_addr) -> c_int {
    int getsockname_unix_good_return_code(struct bpf_sock_addr *ctx)
    {
    return 1;
    }
    SEC("cgroup/getsockname_unix")
#[no_mangle]
pub unsafe extern "C" fn __msg([1: "At program exit the register R0 has smin=0 smax=0 should have been in, _arg: 1]") -> __failure {
    __failure __msg("At program exit the register R0 has smin=0 smax=0 should have been in [1, 1]")
#[no_mangle]
pub unsafe extern "C" fn getsockname_unix_unix_bad_return_code(ctx: *mut bpf_sock_addr) -> c_int {
    int getsockname_unix_unix_bad_return_code(struct bpf_sock_addr *ctx)
    {
    return 0;
    }
    SEC("cgroup/bind4")
    __success
#[no_mangle]
pub unsafe extern "C" fn bind4_good_return_code_0(ctx: *mut bpf_sock_addr) -> c_int {
    int bind4_good_return_code_0(struct bpf_sock_addr *ctx)
    {
    return 0;
    }
    SEC("cgroup/bind4")
    __success
#[no_mangle]
pub unsafe extern "C" fn bind4_good_return_code_1(ctx: *mut bpf_sock_addr) -> c_int {
    int bind4_good_return_code_1(struct bpf_sock_addr *ctx)
    {
    return 1;
    }
    SEC("cgroup/bind4")
    __success
#[no_mangle]
pub unsafe extern "C" fn bind4_good_return_code_2(ctx: *mut bpf_sock_addr) -> c_int {
    int bind4_good_return_code_2(struct bpf_sock_addr *ctx)
    {
    return 2;
    }
    SEC("cgroup/bind4")
    __success
#[no_mangle]
pub unsafe extern "C" fn bind4_good_return_code_3(ctx: *mut bpf_sock_addr) -> c_int {
    int bind4_good_return_code_3(struct bpf_sock_addr *ctx)
    {
    return 3;
    }
    SEC("cgroup/bind4")
#[no_mangle]
pub unsafe extern "C" fn __msg([0: "At program exit the register R0 has smin=4 smax=4 should have been in, _arg: 3]") -> __failure {
    __failure __msg("At program exit the register R0 has smin=4 smax=4 should have been in [0, 3]")
#[no_mangle]
pub unsafe extern "C" fn bind4_bad_return_code(ctx: *mut bpf_sock_addr) -> c_int {
    int bind4_bad_return_code(struct bpf_sock_addr *ctx)
    {
    return 4;
    }
    SEC("cgroup/bind6")
    __success
#[no_mangle]
pub unsafe extern "C" fn bind6_good_return_code_0(ctx: *mut bpf_sock_addr) -> c_int {
    int bind6_good_return_code_0(struct bpf_sock_addr *ctx)
    {
    return 0;
    }
    SEC("cgroup/bind6")
    __success
#[no_mangle]
pub unsafe extern "C" fn bind6_good_return_code_1(ctx: *mut bpf_sock_addr) -> c_int {
    int bind6_good_return_code_1(struct bpf_sock_addr *ctx)
    {
    return 1;
    }
    SEC("cgroup/bind6")
    __success
#[no_mangle]
pub unsafe extern "C" fn bind6_good_return_code_2(ctx: *mut bpf_sock_addr) -> c_int {
    int bind6_good_return_code_2(struct bpf_sock_addr *ctx)
    {
    return 2;
    }
    SEC("cgroup/bind6")
    __success
#[no_mangle]
pub unsafe extern "C" fn bind6_good_return_code_3(ctx: *mut bpf_sock_addr) -> c_int {
    int bind6_good_return_code_3(struct bpf_sock_addr *ctx)
    {
    return 3;
    }
    SEC("cgroup/bind6")
#[no_mangle]
pub unsafe extern "C" fn __msg([0: "At program exit the register R0 has smin=4 smax=4 should have been in, _arg: 3]") -> __failure {
    __failure __msg("At program exit the register R0 has smin=4 smax=4 should have been in [0, 3]")
#[no_mangle]
pub unsafe extern "C" fn bind6_bad_return_code(ctx: *mut bpf_sock_addr) -> c_int {
    int bind6_bad_return_code(struct bpf_sock_addr *ctx)
    {
    return 4;
    }
    SEC("cgroup/connect4")
    __success
#[no_mangle]
pub unsafe extern "C" fn connect4_good_return_code_0(ctx: *mut bpf_sock_addr) -> c_int {
    int connect4_good_return_code_0(struct bpf_sock_addr *ctx)
    {
    return 0;
    }
    SEC("cgroup/connect4")
    __success
#[no_mangle]
pub unsafe extern "C" fn connect4_good_return_code_1(ctx: *mut bpf_sock_addr) -> c_int {
    int connect4_good_return_code_1(struct bpf_sock_addr *ctx)
    {
    return 1;
    }
    SEC("cgroup/connect4")
#[no_mangle]
pub unsafe extern "C" fn __msg([0: "At program exit the register R0 has smin=2 smax=2 should have been in, _arg: 1]") -> __failure {
    __failure __msg("At program exit the register R0 has smin=2 smax=2 should have been in [0, 1]")
#[no_mangle]
pub unsafe extern "C" fn connect4_bad_return_code(ctx: *mut bpf_sock_addr) -> c_int {
    int connect4_bad_return_code(struct bpf_sock_addr *ctx)
    {
    return 2;
    }
    SEC("cgroup/connect6")
    __success
#[no_mangle]
pub unsafe extern "C" fn connect6_good_return_code_0(ctx: *mut bpf_sock_addr) -> c_int {
    int connect6_good_return_code_0(struct bpf_sock_addr *ctx)
    {
    return 0;
    }
    SEC("cgroup/connect6")
    __success
#[no_mangle]
pub unsafe extern "C" fn connect6_good_return_code_1(ctx: *mut bpf_sock_addr) -> c_int {
    int connect6_good_return_code_1(struct bpf_sock_addr *ctx)
    {
    return 1;
    }
    SEC("cgroup/connect6")
#[no_mangle]
pub unsafe extern "C" fn __msg([0: "At program exit the register R0 has smin=2 smax=2 should have been in, _arg: 1]") -> __failure {
    __failure __msg("At program exit the register R0 has smin=2 smax=2 should have been in [0, 1]")
#[no_mangle]
pub unsafe extern "C" fn connect6_bad_return_code(ctx: *mut bpf_sock_addr) -> c_int {
    int connect6_bad_return_code(struct bpf_sock_addr *ctx)
    {
    return 2;
    }
    SEC("cgroup/connect_unix")
    __success
#[no_mangle]
pub unsafe extern "C" fn connect_unix_good_return_code_0(ctx: *mut bpf_sock_addr) -> c_int {
    int connect_unix_good_return_code_0(struct bpf_sock_addr *ctx)
    {
    return 0;
    }
    SEC("cgroup/connect_unix")
    __success
#[no_mangle]
pub unsafe extern "C" fn connect_unix_good_return_code_1(ctx: *mut bpf_sock_addr) -> c_int {
    int connect_unix_good_return_code_1(struct bpf_sock_addr *ctx)
    {
    return 1;
    }
    SEC("cgroup/connect_unix")
#[no_mangle]
pub unsafe extern "C" fn __msg([0: "At program exit the register R0 has smin=2 smax=2 should have been in, _arg: 1]") -> __failure {
    __failure __msg("At program exit the register R0 has smin=2 smax=2 should have been in [0, 1]")
#[no_mangle]
pub unsafe extern "C" fn connect_unix_bad_return_code(ctx: *mut bpf_sock_addr) -> c_int {
    int connect_unix_bad_return_code(struct bpf_sock_addr *ctx)
    {
    return 2;
    }
    char _license[] SEC("license") = "GPL";
