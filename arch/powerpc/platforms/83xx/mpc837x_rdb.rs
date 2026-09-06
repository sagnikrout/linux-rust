//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/83xx/mpc837x_rdb.c
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
// arch/powerpc/platforms/83xx/mpc837x_rdb.c
//
// Copyright (C) 2007 Freescale Semiconductor, Inc. All rights reserved.
//
// MPC837x RDB board specific routines
//

#[no_mangle]
unsafe extern "C" fn mpc837x_rdb_sd_cfg() -> void __init {
    static void __init mpc837x_rdb_sd_cfg(void)
    {
    void __iomem *im;
    im = ioremap(get_immrbase(), 0x1000);
    if (!im) {
    WARN_ON(1);
    return;
    }
//
// On RDB boards (in contrast to MDS) USBB pins are used for SD only,
// so we can safely mux them away from the USB block.
//
    clrsetbits_be32(im + MPC83XX_SICRL_OFFS, MPC837X_SICRL_USBB_MASK,
    MPC837X_SICRL_SD);
    clrsetbits_be32(im + MPC83XX_SICRH_OFFS, MPC837X_SICRH_SPI_MASK,
    MPC837X_SICRH_SD);
    iounmap(im);
    }
//
// Setup the architecture
//
#[no_mangle]
unsafe extern "C" fn mpc837x_rdb_setup_arch() -> void __init {
    static void __init mpc837x_rdb_setup_arch(void)
    {
    mpc83xx_setup_arch();
    mpc837x_usb_cfg();
    mpc837x_rdb_sd_cfg();
    }
    machine_device_initcall(mpc837x_rdb, mpc83xx_declare_of_platform_devices);
    static const char * const board[] __initconst = {
    "fsl,mpc8377rdb",
    "fsl,mpc8378rdb",
    "fsl,mpc8379rdb",
    "fsl,mpc8377wlan",
    core::ptr::null_mut()
    };
    define_machine(mpc837x_rdb) {
    .name			= "MPC837x RDB/WLAN",
    .compatibles		= board,
    .setup_arch		= mpc837x_rdb_setup_arch,
    .discover_phbs  	= mpc83xx_setup_pci,
    .init_IRQ		= mpc83xx_ipic_init_IRQ,
    .get_irq		= ipic_get_irq,
    .restart		= mpc83xx_restart,
    .time_init		= mpc83xx_time_init,
    .progress		= udbg_progress,
    };
