//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/83xx/mpc830x_rdb.c
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
// arch/powerpc/platforms/83xx/mpc830x_rdb.c
//
// Description: MPC830x RDB board specific routines.
// This file is based on mpc831x_rdb.c
//
// Copyright (C) Freescale Semiconductor, Inc. 2009. All rights reserved.
// Copyright (C) 2010. Ilya Yanok, Emcraft Systems, yanok@emcraft.com
//

//
// Setup the architecture
//
#[no_mangle]
unsafe extern "C" fn mpc830x_rdb_setup_arch() -> void __init {
    static void __init mpc830x_rdb_setup_arch(void)
    {
    mpc83xx_setup_arch();
    mpc831x_usb_cfg();
    }
    static const char *board[] __initdata = {
    "MPC8308RDB",
    "fsl,mpc8308rdb",
    "denx,mpc8308_p1m",
    core::ptr::null_mut()
    };
    machine_device_initcall(mpc830x_rdb, mpc83xx_declare_of_platform_devices);
    define_machine(mpc830x_rdb) {
    .name			= "MPC830x RDB",
    .compatibles		= board,
    .setup_arch		= mpc830x_rdb_setup_arch,
    .discover_phbs		= mpc83xx_setup_pci,
    .init_IRQ		= mpc83xx_ipic_init_IRQ,
    .get_irq		= ipic_get_irq,
    .restart		= mpc83xx_restart,
    .time_init		= mpc83xx_time_init,
    .progress		= udbg_progress,
    };
