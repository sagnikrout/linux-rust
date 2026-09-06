//! Automatically rewritten from C to Rust
//! Source: arch/x86/boot/cmdline.c
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
// -*- linux-c -*- -------------------------------------------------------
//
// Copyright (C) 1991, 1992 Linus Torvalds
// Copyright 2007 rPath, Inc. - All Rights Reserved
//
// -----------------------------------------------------------------------
//
// Simple command-line parser for early boot.
//

#[no_mangle]
pub unsafe extern "C" fn myisspace(c: u8) -> c_int {
    static inline int myisspace(u8 c)
    {
    return c <= ' ';	/* Close enough approximation */
    }
//
// Find a non-boolean option, that is, "option=argument".  In accordance
// with standard Linux practice, if this option is repeated, this returns
// the last instance on the command line.
//
// Returns the length of the argument (regardless of if it was
// truncated to fit in the buffer), or -1 on not found.
//
#[no_mangle]
pub unsafe extern "C" fn __cmdline_find_option(cmdline_ptr: c_ulong, option: *const c_char, buffer: *mut c_char, bufsize: c_int) -> c_int {
    int __cmdline_find_option(unsigned long cmdline_ptr, const char *option, char *buffer, int bufsize)
    {
    addr_t cptr;
    char c;
    let mut len: c_int = -1;
    const char *opptr = core::ptr::null_mut();
    char *bufptr = buffer;
    enum {
    st_wordstart,	/* Start of word/after whitespace */
    st_wordcmp,	/* Comparing this word */
    st_wordskip,	/* Miscompare, skip */
    st_bufcpy	/* Copying this to buffer */
    } state = st_wordstart;
    if (!cmdline_ptr)
    return -1;      /* No command line */
    cptr = cmdline_ptr & 0xf;
    set_fs(cmdline_ptr >> 4);
    while (cptr < 0x10000 && (c = rdfs8(cptr++))) {
    switch (state) {
    case st_wordstart:
    if (myisspace(c))
    break;
// else
    state = st_wordcmp;
    opptr = option;
    fallthrough;
    case st_wordcmp:
    if (c == '=' && !*opptr) {
    len = 0;
    bufptr = buffer;
    state = st_bufcpy;
    } else if (myisspace(c)) {
    state = st_wordstart;
    } else if (c != *opptr++) {
    state = st_wordskip;
    }
    break;
    case st_wordskip:
    if (myisspace(c))
    state = st_wordstart;
    break;
    case st_bufcpy:
    if (myisspace(c)) {
    state = st_wordstart;
    } else {
    if (len < bufsize-1)
// bufptr++ = c;
    len++;
    }
    break;
    }
    }
    if (bufsize)
// bufptr = '\0';
    return len;
    }
//
// Find a boolean option (like quiet,noapic,nosmp....)
//
// Returns the position of that option (starts counting with 1)
// or 0 on not found
//
#[no_mangle]
pub unsafe extern "C" fn __cmdline_find_option_bool(cmdline_ptr: c_ulong, option: *const c_char) -> c_int {
    int __cmdline_find_option_bool(unsigned long cmdline_ptr, const char *option)
    {
    addr_t cptr;
    char c;
    let mut pos: c_int = 0, wstart = 0;
    const char *opptr = core::ptr::null_mut();
    enum {
    st_wordstart,	/* Start of word/after whitespace */
    st_wordcmp,	/* Comparing this word */
    st_wordskip,	/* Miscompare, skip */
    } state = st_wordstart;
    if (!cmdline_ptr)
    return -1;      /* No command line */
    cptr = cmdline_ptr & 0xf;
    set_fs(cmdline_ptr >> 4);
    while (cptr < 0x10000) {
    c = rdfs8(cptr++);
    pos++;
    switch (state) {
    case st_wordstart:
    if (!c)
    return 0;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: myisspace(c)) -> else {
    else if (myisspace(c))
    break;
    state = st_wordcmp;
    opptr = option;
    wstart = pos;
    fallthrough;
    case st_wordcmp:
    if (!*opptr)
    if (!c || myisspace(c))
    return wstart;
    else
    state = st_wordskip;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !c) -> else {
    else if (!c)
    return 0;
#[no_mangle]
pub unsafe extern "C" fn if(opptr++: *mut c !=) -> else {
    else if (c != *opptr++)
    state = st_wordskip;
    break;
    case st_wordskip:
    if (!c)
    return 0;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: myisspace(c)) -> else {
    else if (myisspace(c))
    state = st_wordstart;
    break;
    }
    }
    return 0;	/* Buffer overrun */
    }
