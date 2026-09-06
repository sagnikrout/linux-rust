//! Automatically rewritten from C to Rust
//! Source: arch/x86/boot/cpu.c
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
// Copyright 2007-2008 rPath, Inc. - All Rights Reserved
//
// -----------------------------------------------------------------------
//
// arch/x86/boot/cpu.c
//
// Check for obligatory CPU features and abort if the features are not
// present.
//

    static char *cpu_name(int level)
    {
    static char buf[6];
    if (level == 64) {
    return "x86-64";
    } else {
    if (level == 15)
    level = 6;
    sprintf(buf, "i%d86", level);
    return buf;
    }
    }
#[no_mangle]
unsafe extern "C" fn show_cap_strs(err_flags: *mut u32) {
    static void show_cap_strs(u32 *err_flags)
    {
    int i, j;
    const unsigned char *msg_strs = (const unsigned char *)x86_cap_strs;
    for (i = 0; i < NCAPINTS; i++) {
    let mut e: u32 = err_flags[i];
    for (j = 0; j < 32; j++) {
    if (msg_strs[0] < i ||
    (msg_strs[0] == i && msg_strs[1] < j)) {
// Skip to the next string
    msg_strs += 2;
    while (*msg_strs++)
    ;
    }
    if (e & 1) {
    if (msg_strs[0] == i &&
    msg_strs[1] == j &&
    msg_strs[2])
    printf("%s ", msg_strs+2);
    else
    printf("%d:%d ", i, j);
    }
    e >>= 1;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn validate_cpu() -> c_int {
    int validate_cpu(void)
    {
    u32 *err_flags;
    int cpu_level, req_level;
    check_cpu(&cpu_level, &req_level, &err_flags);
    if (cpu_level < req_level) {
    printf("This kernel requires an %s CPU, ",
    cpu_name(req_level));
    printf("but only detected an %s CPU.\n",
    cpu_name(cpu_level));
    return -1;
    }
    if (err_flags) {
    puts("This kernel requires the following features "
    "not present on the CPU:\n");
    show_cap_strs(err_flags);
    putchar('\n');
    return -1;
    } else if (check_knl_erratum()) {
    return -1;
    } else {
    return 0;
    }
    }
