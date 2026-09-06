//! Automatically rewritten from C to Rust
//! Source: tools/perf/trace/beauty/socket.c
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


// SPDX-License-Identifier: LGPL-2.1
//
// trace/beauty/socket.c
//
// Copyright (C) 2018, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>
//

#[no_mangle]
unsafe extern "C" fn socket__scnprintf_ipproto(protocol: c_int, bf: *mut c_char, size: usize, show_prefix: bool) -> usize {
    static size_t socket__scnprintf_ipproto(int protocol, char *bf, size_t size, bool show_prefix)
    {
    static DEFINE_STRARRAY(socket_ipproto, "IPPROTO_");
    return strarray__scnprintf(&strarray__socket_ipproto, bf, size, "%d", show_prefix, protocol);
    }
#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_socket_protocol(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize {
    size_t syscall_arg__scnprintf_socket_protocol(char *bf, size_t size, struct syscall_arg *arg)
    {
    let mut domain: c_int = syscall_arg__val(arg, 0);
    if (domain == AF_INET || domain == AF_INET6)
    return socket__scnprintf_ipproto(arg.val, bf, size, arg.show_string_prefix);
    return syscall_arg__scnprintf_int(bf, size, arg);
    }
#[no_mangle]
unsafe extern "C" fn socket__scnprintf_level(level: c_int, bf: *mut c_char, size: usize, show_prefix: bool) -> usize {
    static size_t socket__scnprintf_level(int level, char *bf, size_t size, bool show_prefix)
    {

    let mut sol_socket: c_int = 0xffff;

    let mut sol_socket: c_int = 1;

    if (level == sol_socket)
    return scnprintf(bf, size, "%sSOCKET", show_prefix ? "SOL_" : "");
    return strarray__scnprintf(&strarray__socket_level, bf, size, "%d", show_prefix, level);
    }
#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_socket_level(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize {
    size_t syscall_arg__scnprintf_socket_level(char *bf, size_t size, struct syscall_arg *arg)
    {
    return socket__scnprintf_level(arg.val, bf, size, arg.show_string_prefix);
    }
