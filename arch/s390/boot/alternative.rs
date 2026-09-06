//! Automatically rewritten from C to Rust
//! Source: arch/s390/boot/alternative.c
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

#[no_mangle]
unsafe extern "C" fn alt_debug_all(type: c_int) {
    static void alt_debug_all(int type)
    {
    int i;
    switch (type) {
    case ALT_TYPE_FACILITY:
    for (i = 0; i < ARRAY_SIZE(alt_debug.facilities); i++)
    alt_debug.facilities[i] = -1UL;
    break;
    case ALT_TYPE_FEATURE:
    for (i = 0; i < ARRAY_SIZE(alt_debug.mfeatures); i++)
    alt_debug.mfeatures[i] = -1UL;
    break;
    case ALT_TYPE_SPEC:
    alt_debug.spec = 1;
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn alt_debug_modify(type: c_int, nr: c_uint, clear: bool) {
    static void alt_debug_modify(int type, unsigned int nr, bool clear)
    {
    switch (type) {
    case ALT_TYPE_FACILITY:
    if (clear)
    __clear_facility(nr, alt_debug.facilities);
    else
    __set_facility(nr, alt_debug.facilities);
    break;
    case ALT_TYPE_FEATURE:
    if (clear)
    __clear_machine_feature(nr, alt_debug.mfeatures);
    else
    __set_machine_feature(nr, alt_debug.mfeatures);
    break;
    }
    }
    static char *alt_debug_parse(int type, char *str)
    {
    unsigned long val, endval, limit;
    char *endp;
    bool clear;
    int i;
    limit = type == ALT_TYPE_FACILITY ? MAX_FACILITY_BIT : MAX_MFEATURE_BIT;
    if (*str == ':') {
    str++;
    } else {
    alt_debug_all(type);
    return str;
    }
    clear = false;
    if (*str == '!') {
    alt_debug_all(type);
    clear = true;
    str++;
    }
    while (*str) {
    val = simple_strtoull(str, &endp, 0);
    if (str == endp)
    break;
    str = endp;
    if (*str == '-') {
    str++;
    endval = simple_strtoull(str, &endp, 0);
    if (str == endp)
    break;
    str = endp;
    while (val <= endval && val < limit) {
    alt_debug_modify(type, val, clear);
    val++;
    }
    } else {
    alt_debug_modify(type, val, clear);
    }
    if (*str != ',')
    break;
    str++;
    }
    return str;
    }
//
// Use debug-alternative command line parameter for debugging:
// "debug-alternative"
// -> print debug message for every single alternative
//
// "debug-alternative=0;2"
// -> print debug message for all alternatives with type 0 and 2
//
// "debug-alternative=0:0-7"
// -> print debug message for all alternatives with type 0 and with
// facility numbers within the range of 0-7
// (if type 0 is ALT_TYPE_FACILITY)
//
// "debug-alternative=0:!8;1"
// -> print debug message for all alternatives with type 0, for all
// facility number, except facility 8, and in addition print all
// alternatives with type 1
//
#[no_mangle]
pub unsafe extern "C" fn alt_debug_setup(str: *mut c_char) {
    void alt_debug_setup(char *str)
    {
    unsigned long type;
    char *endp;
    int i;
    if (!str) {
    alt_debug_all(ALT_TYPE_FACILITY);
    alt_debug_all(ALT_TYPE_FEATURE);
    alt_debug_all(ALT_TYPE_SPEC);
    return;
    }
    while (*str) {
    type = simple_strtoull(str, &endp, 0);
    if (str == endp)
    break;
    str = endp;
    switch (type) {
    case ALT_TYPE_FACILITY:
    case ALT_TYPE_FEATURE:
    str = alt_debug_parse(type, str);
    break;
    case ALT_TYPE_SPEC:
    alt_debug_all(ALT_TYPE_SPEC);
    break;
    }
    if (*str != ';')
    break;
    str++;
    }
    }
