//! Automatically rewritten from C to Rust
//! Source: arch/x86/entry/vdso/vdso32-setup.c
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
// (C) Copyright 2002 Linus Torvalds
// Portions based on the vdso-randomization code from exec-shield:
// Copyright(C) 2005-2006, Red Hat, Inc., Ingo Molnar
//
// This file contains the needed initializations to support sysenter.
//

pub const VDSO_DEFAULT: c_int = 0;

pub const VDSO_DEFAULT: c_int = 1;

//
// Should the kernel map a VDSO page into processes and pass its
// address down to glibc upon exec()?
//
    let mut vdso32_enabled: unsigned int __read_mostly = VDSO_DEFAULT;
#[no_mangle]
unsafe extern "C" fn vdso32_setup(s: *mut c_char) -> int __init {
    static int __init vdso32_setup(char *s)
    {
    vdso32_enabled = simple_strtoul(s, core::ptr::null_mut(), 0);
    if (vdso32_enabled > 1) {
    pr_warn("vdso32 values other than 0 and 1 are no longer allowed; vdso disabled\n");
    vdso32_enabled = 0;
    }
    return 1;
    }
//
// For consistency, the argument vdso32=[012] affects the 32-bit vDSO
// behavior on both 64-bit and 32-bit kernels.
// On 32-bit kernels, vdso=[012] means the same thing.
//
    __setup("vdso32=", vdso32_setup);

    __setup_param("vdso=", vdso_setup, vdso32_setup, 0);

    static const struct ctl_table vdso_table[] = {
    {

    .procname	= "vsyscall32",

    .procname	= "vdso_enabled",

    .data		= &vdso32_enabled,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_minmax,
    .extra1		= SYSCTL_ZERO,
    .extra2		= SYSCTL_ONE,
    },
    };
#[no_mangle]
unsafe extern "C" fn ia32_binfmt_init() -> __init int {
    static __init int ia32_binfmt_init(void)
    {

// Register vsyscall32 into the ABI table
    register_sysctl("abi", vdso_table);

    register_sysctl_init("vm", vdso_table);

    return 0;
    }
    __initcall(ia32_binfmt_init);
