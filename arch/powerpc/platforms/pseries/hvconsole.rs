//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/pseries/hvconsole.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// hvconsole.c
// Copyright (C) 2004 Hollis Blanchard, IBM Corporation
// Copyright (C) 2004 IBM Corporation
//
// Additional Author(s):
// Ryan S. Arnold <rsa@us.ibm.com>
//
// LPAR console support.
//

//
// hvc_get_chars - retrieve characters from firmware for denoted vterm adapter
// @vtermno: The vtermno or unit_address of the adapter from which to fetch the
// data.
// @buf: The character buffer into which to put the character data fetched from
// firmware.
// @count: not used?
//
#[no_mangle]
pub unsafe extern "C" fn hvc_get_chars(vtermno: u32, buf: *mut u8, count: usize) -> isize {
    ssize_t hvc_get_chars(uint32_t vtermno, u8 *buf, size_t count)
    {
    long ret;
    unsigned long retbuf[PLPAR_HCALL_BUFSIZE];
    unsigned long *lbuf = (unsigned long *)buf;
    ret = plpar_hcall(H_GET_TERM_CHAR, retbuf, vtermno);
    lbuf[0] = be64_to_cpu(retbuf[1]);
    lbuf[1] = be64_to_cpu(retbuf[2]);
    if (ret == H_SUCCESS)
    return retbuf[0];
    return 0;
    }
    EXPORT_SYMBOL(hvc_get_chars);
//
// hvc_put_chars: send characters to firmware for denoted vterm adapter
// @vtermno: The vtermno or unit_address of the adapter from which the data
// originated.
// @buf: The character buffer that contains the character data to send to
// firmware. Must be at least 16 bytes, even if count is less than 16.
// @count: Send this number of characters.
//
#[no_mangle]
pub unsafe extern "C" fn hvc_put_chars(vtermno: u32, buf: *const u8, count: usize) -> isize {
    ssize_t hvc_put_chars(uint32_t vtermno, const u8 *buf, size_t count)
    {
    unsigned long *lbuf = (unsigned long *) buf;
    long ret;
// hcall will ret H_PARAMETER if 'count' exceeds firmware max.
    if (count > MAX_VIO_PUT_CHARS)
    count = MAX_VIO_PUT_CHARS;
    ret = plpar_hcall_norets(H_PUT_TERM_CHAR, vtermno, count,
    cpu_to_be64(lbuf[0]),
    cpu_to_be64(lbuf[1]));
    if (ret == H_SUCCESS)
    return count;
    if (ret == H_BUSY)
    return -EAGAIN;
    return -EIO;
    }
    EXPORT_SYMBOL(hvc_put_chars);
