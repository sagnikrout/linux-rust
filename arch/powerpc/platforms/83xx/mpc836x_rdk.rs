//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/83xx/mpc836x_rdk.c
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
// MPC8360E-RDK board file.
//
// Copyright (c) 2006  Freescale Semiconductor, Inc.
// Copyright (c) 2007-2008  MontaVista Software, Inc.
//
// Author: Anton Vorontsov <avorontsov@ru.mvista.com>
//

    machine_device_initcall(mpc836x_rdk, mpc83xx_declare_of_platform_devices);
#[no_mangle]
unsafe extern "C" fn mpc836x_rdk_setup_arch() -> void __init {
    static void __init mpc836x_rdk_setup_arch(void)
    {
    mpc83xx_setup_arch();
    }
    define_machine(mpc836x_rdk) {
    .name		= "MPC836x RDK",
    .compatible	= "fsl,mpc8360rdk",
    .setup_arch	= mpc836x_rdk_setup_arch,
    .discover_phbs  = mpc83xx_setup_pci,
    .init_IRQ	= mpc83xx_ipic_init_IRQ,
    .get_irq	= ipic_get_irq,
    .restart	= mpc83xx_restart,
    .time_init	= mpc83xx_time_init,
    .progress	= udbg_progress,
    };
