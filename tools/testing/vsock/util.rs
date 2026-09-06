//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/vsock/util.h
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

// All known vsock transports, see callers of vsock_core_register()

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum transport {
    TRANSPORT_COUNTER_BASE = __COUNTER__ + 1,

    TRANSPORT_##name = BIT(__COUNTER__ - TRANSPORT_COUNTER_BASE),
    KNOWN_TRANSPORTS(x)
    TRANSPORT_NUM = __COUNTER__ - TRANSPORT_COUNTER_BASE,

}

// Tests can either run as the client or the server
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum test_mode {
    TEST_MODE_UNSET,
    TEST_MODE_CLIENT,
    TEST_MODE_SERVER
}

pub const DEFAULT_PEER_PORT: c_int = 1234;
// Test runner options
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_opts {
    pub mode: test_mode,
    pub peer_cid: c_uint,
    pub peer_port: c_uint,
}

// A test case definition.  Test functions must print failures to stderr and
// terminate with exit(EXIT_FAILURE).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_case {
    pub /: *const *const *const char name; / human-readable name,
// Called when test mode is TEST_MODE_CLIENT
    pub opts): *const *const void (run_client)(struct test_opts,
// Called when test mode is TEST_MODE_SERVER
    pub opts): *const *const void (run_server)(struct test_opts,
    pub skip: bool,
}

extern "C" {
    pub fn init_signals();
}
extern "C" {
    pub fn parse_cid(str: *const c_char) -> c_uint;
}
extern "C" {
    pub fn parse_port(str: *const c_char) -> c_uint;
}
extern "C" {
    pub fn vsock_connect_fd(fd: c_int, cid: c_uint, port: c_uint) -> c_int;
}
extern "C" {
    pub fn vsock_connect(cid: c_uint, port: c_uint, type: c_int) -> c_int;
}
extern "C" {
    pub fn vsock_stream_connect(cid: c_uint, port: c_uint) -> c_int;
}
extern "C" {
    pub fn vsock_bind_try(cid: c_uint, port: c_uint, type: c_int) -> c_int;
}
extern "C" {
    pub fn vsock_bind(cid: c_uint, port: c_uint, type: c_int) -> c_int;
}
extern "C" {
    pub fn vsock_seqpacket_connect(cid: c_uint, port: c_uint) -> c_int;
}
extern "C" {
    pub fn vsock_stream_listen(cid: c_uint, port: c_uint) -> c_int;
}
extern "C" {
    pub fn vsock_wait_remote_close(fd: c_int);
}
extern "C" {
    pub fn vsock_ioctl_int(fd: c_int, op: c_ulong, expected: c_int) -> bool;
}
extern "C" {
    pub fn vsock_wait_sent(fd: c_int) -> bool;
}
extern "C" {
    pub fn recv_buf(fd: c_int, buf: *mut c_void, len: usize, flags: c_int, expected_ret: isize);
}
extern "C" {
    pub fn send_byte(fd: c_int, expected_ret: c_int, flags: c_int);
}
extern "C" {
    pub fn recv_byte(fd: c_int, expected_ret: c_int, flags: c_int);
}
extern "C" {
    pub fn list_tests(test_cases: *const test_case);
}
extern "C" {
    pub fn hash_djb2(data: *const c_void, len: usize) -> c_ulong;
}
extern "C" {
    pub fn iovec_bytes(iov: *const iovec, iovnum: usize) -> usize;
}
extern "C" {
    pub fn iovec_hash_djb2(iov: *const iovec, iovnum: usize) -> c_ulong;
}
extern "C" {
    pub fn enable_so_zerocopy_check(fd: c_int);
}
extern "C" {
    pub fn enable_so_linger(fd: c_int, timeout: c_int);
}
extern "C" {
    pub fn get_transports() -> c_int;
}
