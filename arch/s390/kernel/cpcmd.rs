//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/cpcmd.c
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
// S390 version
// Copyright IBM Corp. 1999, 2007
// Author(s): Martin Schwidefsky (schwidefsky@de.ibm.com),
// Christian Borntraeger (cborntra@de.ibm.com),
//

    static DEFINE_SPINLOCK(cpcmd_lock);
    static char cpcmd_buf[241];
#[no_mangle]
unsafe extern "C" fn diag8_noresponse(cmdlen: c_int) -> c_int {
    static int diag8_noresponse(int cmdlen)
    {
    asm volatile(
    "	diag	%[rx],%[ry],0x8\n"
    : [ry] "+&d" (cmdlen)
    : [rx] "d" (__pa(cpcmd_buf))
    : "cc");
    return cmdlen;
    }
#[no_mangle]
unsafe extern "C" fn diag8_response(cmdlen: c_int, response: *mut c_char, rlen: *mut c_int) -> c_int {
    static int diag8_response(int cmdlen, char *response, int *rlen)
    {
    union register_pair rx, ry;
    int cc;
    rx.even = __pa(cpcmd_buf);
    rx.odd	= __pa(response);
    ry.even = cmdlen | 0x40000000L;
    ry.odd	= *rlen;
    asm volatile(
    "	diag	%[rx],%[ry],0x8\n"
    CC_IPM(cc)
    : CC_OUT(cc, cc), [ry] "+d" (ry.pair)
    : [rx] "d" (rx.pair)
    : CC_CLOBBER);
    if (CC_TRANSFORM(cc))
// rlen += ry.odd;
    else
// rlen = ry.odd;
    return ry.even;
    }
//
// __cpcmd has some restrictions over cpcmd
// - __cpcmd is unlocked and therefore not SMP-safe
//
#[no_mangle]
pub unsafe extern "C" fn __cpcmd(cmd: *const c_char, response: *mut c_char, rlen: c_int, response_code: *mut c_int) -> c_int {
    int  __cpcmd(const char *cmd, char *response, int rlen, int *response_code)
    {
    int cmdlen;
    int rc;
    int response_len;
    cmdlen = strlen(cmd);
    BUG_ON(cmdlen > 240);
    memcpy(cpcmd_buf, cmd, cmdlen);
    ASCEBC(cpcmd_buf, cmdlen);
    diag_stat_inc(DIAG_STAT_X008);
    if (response) {
    memset(response, 0, rlen);
    response_len = rlen;
    rc = diag8_response(cmdlen, response, &rlen);
    EBCASC(response, response_len);
    } else {
    rc = diag8_noresponse(cmdlen);
    }
    if (response_code)
// response_code = rc;
    return rlen;
    }
    EXPORT_SYMBOL(__cpcmd);
#[no_mangle]
pub unsafe extern "C" fn cpcmd(cmd: *const c_char, response: *mut c_char, rlen: c_int, response_code: *mut c_int) -> c_int {
    int cpcmd(const char *cmd, char *response, int rlen, int *response_code)
    {
    unsigned long flags;
    char *lowbuf;
    int len;
    if (is_vmalloc_or_module_addr(response)) {
    lowbuf = kmalloc(rlen, GFP_KERNEL);
    if (!lowbuf) {
    pr_warn("The cpcmd kernel function failed to allocate a response buffer\n");
    return -ENOMEM;
    }
    spin_lock_irqsave(&cpcmd_lock, flags);
    len = __cpcmd(cmd, lowbuf, rlen, response_code);
    spin_unlock_irqrestore(&cpcmd_lock, flags);
    memcpy(response, lowbuf, rlen);
    kfree(lowbuf);
    } else {
    spin_lock_irqsave(&cpcmd_lock, flags);
    len = __cpcmd(cmd, response, rlen, response_code);
    spin_unlock_irqrestore(&cpcmd_lock, flags);
    }
    return len;
    }
    EXPORT_SYMBOL(cpcmd);
