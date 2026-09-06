//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/socket.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct socket_testcase {
    pub domain: c_int,
    pub type: c_int,
    pub protocol: c_int,
// 0    = valid file descriptor
// -foo = error foo
//
    pub expect: c_int,
// If non-zero, accept EAFNOSUPPORT to handle the case
// of the protocol not being configured into the kernel.
//
    pub nosupport_ok: c_int,
}

    static struct socket_testcase tests[] = {
    { AF_MAX,  0,           0,           -EAFNOSUPPORT,    0 },
    { AF_INET, SOCK_STREAM, IPPROTO_TCP, 0,                1  },
    { AF_INET, SOCK_DGRAM,  IPPROTO_TCP, -EPROTONOSUPPORT, 1  },
    { AF_INET, SOCK_DGRAM,  IPPROTO_UDP, 0,                1  },
    { AF_INET, SOCK_STREAM, IPPROTO_UDP, -EPROTONOSUPPORT, 1  },
    };
pub const ERR_STRING_SZ: c_int = 64;
#[no_mangle]
unsafe extern "C" fn run_tests() -> c_int {
    static int run_tests(void)
    {
    char err_string1[ERR_STRING_SZ];
    char err_string2[ERR_STRING_SZ];
    const char *msg1, *msg2;
    int i, err;
    err = 0;
    for (i = 0; i < ARRAY_SIZE(tests); i++) {
    struct socket_testcase *s = &tests[i];
    int fd;
    fd = socket(s.domain, s.type, s.protocol);
    if (fd < 0) {
    if (s.nosupport_ok &&
    errno == EAFNOSUPPORT)
    continue;
    if (s.expect < 0 &&
    errno == -s.expect)
    continue;
    msg1 = strerror_r(-s.expect, err_string1, ERR_STRING_SZ);
    msg2 = strerror_r(errno, err_string2, ERR_STRING_SZ);
    fprintf(stderr, "socket(%d, %d, %d) expected "
    "err (%s) got (%s)\n",
    s.domain, s.type, s.protocol,
    msg1, msg2);
    err = -1;
    break;
    } else {
    close(fd);
    if (s.expect < 0) {
    msg1 = strerror_r(errno, err_string1, ERR_STRING_SZ);
    fprintf(stderr, "socket(%d, %d, %d) expected "
    "success got err (%s)\n",
    s.domain, s.type, s.protocol,
    msg1);
    err = -1;
    break;
    }
    }
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    let mut err: c_int = run_tests();
    return err;
    }
