//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/scsi_sysctl.c
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
//
// Copyright (C) 2003 Christoph Hellwig.
//

    static const struct ctl_table scsi_table[] = {
    { .procname	= "logging_level",
    .data		= &scsi_logging_level,
    .maxlen	= sizeof(scsi_logging_level),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_minmax,
    .extra1	= SYSCTL_ZERO,
    .extra2	= SYSCTL_INT_MAX },
    };
    static struct ctl_table_header *scsi_table_header;
#[no_mangle]
pub unsafe extern "C" fn scsi_init_sysctl() -> int __init {
    int __init scsi_init_sysctl(void)
    {
    scsi_table_header = register_sysctl("dev/scsi", scsi_table);
    if (!scsi_table_header)
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn scsi_exit_sysctl() {
    void scsi_exit_sysctl(void)
    {
    unregister_sysctl_table(scsi_table_header);
    }
