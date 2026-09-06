//! Automatically rewritten from C to Rust
//! Source: fs/coda/sysctl.c
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
// Sysctl operations for Coda filesystem
// Original version: (C) 1996 P. Braam and M. Callahan
// Rewritten for Linux 2.1. (C) 1997 Carnegie Mellon University
//
// Carnegie Mellon encourages users to contribute improvements to
// the Coda project. Contact Peter Braam (coda@cs.cmu.edu).
//

    static struct ctl_table_header *fs_table_header;
    static const struct ctl_table coda_table[] = {
    {
    .procname	= "timeout",
    .data		= &coda_timeout,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec
    },
    {
    .procname	= "hard",
    .data		= &coda_hard,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec
    },
    {
    .procname	= "fake_statfs",
    .data		= &coda_fake_statfs,
    .maxlen		= sizeof(int),
    .mode		= 0600,
    .proc_handler	= proc_dointvec
    },
    };
#[no_mangle]
pub unsafe extern "C" fn coda_sysctl_init() {
    void coda_sysctl_init(void)
    {
    if ( !fs_table_header )
    fs_table_header = register_sysctl("coda", coda_table);
    }
#[no_mangle]
pub unsafe extern "C" fn coda_sysctl_clean() {
    void coda_sysctl_clean(void)
    {
    if ( fs_table_header ) {
    unregister_sysctl_table(fs_table_header);
    fs_table_header = core::ptr::null_mut();
    }
    }
