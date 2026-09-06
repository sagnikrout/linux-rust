//! Automatically rewritten from C to Rust
//! Source: tools/perf/trace/beauty/sockaddr.c
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
// Copyright (C) 2018, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>

    DEFINE_STRARRAY(socket_families, "PF_");
#[no_mangle]
unsafe extern "C" fn af_inet__scnprintf(sa: *mut sockaddr, bf: *mut c_char, size: usize) -> usize {
    static size_t af_inet__scnprintf(struct sockaddr *sa, char *bf, size_t size)
    {
    struct sockaddr_in *sin = (struct sockaddr_in *)sa;
    char tmp[16];
    return scnprintf(bf, size, ", port: %d, addr: %s", ntohs(sin.sin_port),
    inet_ntop(sin.sin_family, &sin.sin_addr, tmp, sizeof(tmp)));
    }
#[no_mangle]
unsafe extern "C" fn af_inet6__scnprintf(sa: *mut sockaddr, bf: *mut c_char, size: usize) -> usize {
    static size_t af_inet6__scnprintf(struct sockaddr *sa, char *bf, size_t size)
    {
    struct sockaddr_in6 *sin6 = (struct sockaddr_in6 *)sa;
    let mut flowinfo: u32 = ntohl(sin6.sin6_flowinfo);
    char tmp[512];
    size_t printed = scnprintf(bf, size, ", port: %d, addr: %s", ntohs(sin6.sin6_port),
    inet_ntop(sin6.sin6_family, &sin6.sin6_addr, tmp, sizeof(tmp)));
    if (flowinfo != 0)
    printed += scnprintf(bf + printed, size - printed, ", flowinfo: %lu", flowinfo);
    if (sin6.sin6_scope_id != 0)
    printed += scnprintf(bf + printed, size - printed, ", scope_id: %lu", sin6.sin6_scope_id);
    return printed;
    }
#[no_mangle]
unsafe extern "C" fn af_local__scnprintf(sa: *mut sockaddr, bf: *mut c_char, size: usize) -> usize {
    static size_t af_local__scnprintf(struct sockaddr *sa, char *bf, size_t size)
    {
    struct sockaddr_un *sun = (struct sockaddr_un *)sa;
    return scnprintf(bf, size, ", path: %s", sun.sun_path);
    }
    static size_t (*af_scnprintfs[])(struct sockaddr *sa, char *bf, size_t size) = {
    [AF_LOCAL] = af_local__scnprintf,
    [AF_INET]  = af_inet__scnprintf,
    [AF_INET6] = af_inet6__scnprintf,
    };
#[no_mangle]
unsafe extern "C" fn syscall_arg__scnprintf_augmented_sockaddr(arg: *mut syscall_arg, bf: *mut c_char, size: usize) -> usize {
    static size_t syscall_arg__scnprintf_augmented_sockaddr(struct syscall_arg *arg, char *bf, size_t size)
    {
    struct sockaddr *sa = (struct sockaddr *)&arg.augmented.args.value;
    char family[32];
    size_t printed;
    strarray__scnprintf(&strarray__socket_families, family, sizeof(family), "%d", arg.show_string_prefix, sa.sa_family);
    printed = scnprintf(bf, size, "{ .family: %s", family);
    if (sa.sa_family < ARRAY_SIZE(af_scnprintfs) && af_scnprintfs[sa.sa_family])
    printed += af_scnprintfs[sa.sa_family](sa, bf + printed, size - printed);
    return printed + scnprintf(bf + printed, size - printed, " }");
    }
#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_sockaddr(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize {
    size_t syscall_arg__scnprintf_sockaddr(char *bf, size_t size, struct syscall_arg *arg)
    {
    if (arg.augmented.args)
    return syscall_arg__scnprintf_augmented_sockaddr(arg, bf, size);
    return scnprintf(bf, size, "%#lx", arg.val);
    }
