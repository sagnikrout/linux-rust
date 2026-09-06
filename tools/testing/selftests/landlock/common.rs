//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/landlock/common.h
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
// Landlock test helpers
//
// Copyright © 2017-2020 Mickaël Salaün <mic@digikod.net>
// Copyright © 2019-2020 ANSSI
// Copyright © 2021 Microsoft Corporation
//

// TEST_F_FORK() should not be used for new tests.

pub const LANDLOCK_MAX_NUM_LAYERS: c_int = 16;
// Only these three capabilities are useful for the tests.
// clang-format off
// clang-format on
// Automatically resets ambient capabilities.
// Quickly checks that ambient capabilities are cleared.
// We cannot put such helpers in a library because of kselftest_harness.h .
// Receives an FD from a UNIX socket. Returns the received FD, or -errno.
// Aligned ancillary data buffer.
// Sends an FD on a UNIX socket. Returns 0 on success or -errno.
// Aligned ancillary data buffer.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct protocol_variant {
    pub domain: c_int,
    pub type: c_int,
    pub protocol: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct service_fixture {
    pub protocol: protocol_variant,
// port is also stored in ipv4_addr.sin_port or ipv6_addr.sin6_port
    pub port: c_ushort,
    pub ipv4_addr: sockaddr_in,
    pub ipv6_addr: sockaddr_in6,
    pub unix_addr: sockaddr_un,
    pub unix_addr_len: socklen_t,
}

//
// regex_escape - Escape BRE metacharacters in a string
//
// @src: Source string to escape.
// @dst: Destination buffer for the escaped string.
// @dst_size: Size of the destination buffer.
//
// Escapes characters that have special meaning in POSIX Basic Regular
// Expressions: $ * . [ \ ] ^
//
// Returns a pointer to the NUL terminator in @dst (cursor-style API for
// chaining), or (char *)-ENOMEM if the buffer is too small.
//
// d++ = '\\';
// d++ = *s;
// d = '\0';
