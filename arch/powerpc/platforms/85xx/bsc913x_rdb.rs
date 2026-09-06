//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/85xx/bsc913x_rdb.c
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
// BSC913xRDB Board Setup
//
// Author: Priyanka Jain <Priyanka.Jain@freescale.com>
//
// Copyright 2011-2012 Freescale Semiconductor Inc.
//

#[no_mangle]
unsafe extern "C" fn bsc913x_rdb_pic_init() -> void __init {
    static void __init bsc913x_rdb_pic_init(void)
    {
    struct mpic *mpic = mpic_alloc(core::ptr::null_mut(), 0, MPIC_BIG_ENDIAN |
    MPIC_SINGLE_DEST_CPU,
    0, 256, " OpenPIC  ");
    if (!mpic)
    pr_err("bsc913x: Failed to allocate MPIC structure\n");
    else
    mpic_init(mpic);
    }
//
// Setup the architecture
//
#[no_mangle]
unsafe extern "C" fn bsc913x_rdb_setup_arch() -> void __init {
    static void __init bsc913x_rdb_setup_arch(void)
    {
    if (ppc_md.progress)
    ppc_md.progress("bsc913x_rdb_setup_arch()", 0);
    pr_info("bsc913x board from Freescale Semiconductor\n");
    }
    machine_device_initcall(bsc9131_rdb, mpc85xx_common_publish_devices);
    define_machine(bsc9131_rdb) {
    .name			= "BSC9131 RDB",
    .compatible		= "fsl,bsc9131rdb",
    .setup_arch		= bsc913x_rdb_setup_arch,
    .init_IRQ		= bsc913x_rdb_pic_init,
    .get_irq		= mpic_get_irq,
    .progress		= udbg_progress,
    };
