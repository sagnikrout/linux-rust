//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/83xx/mpc834x_itx.c
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
// arch/powerpc/platforms/83xx/mpc834x_itx.c
//
// MPC834x ITX board specific routines
//
// Maintainer: Kumar Gala <galak@kernel.crashing.org>
//

    static const struct of_device_id mpc834x_itx_ids[] __initconst = {
    { .compatible = "fsl,pq2pro-localbus", },
    {},
    };
#[no_mangle]
unsafe extern "C" fn mpc834x_itx_declare_of_platform_devices() -> int __init {
    static int __init mpc834x_itx_declare_of_platform_devices(void)
    {
    mpc83xx_declare_of_platform_devices();
    return of_platform_bus_probe(core::ptr::null_mut(), mpc834x_itx_ids, core::ptr::null_mut());
    }
    machine_device_initcall(mpc834x_itx, mpc834x_itx_declare_of_platform_devices);
//
// Setup the architecture
//
#[no_mangle]
unsafe extern "C" fn mpc834x_itx_setup_arch() -> void __init {
    static void __init mpc834x_itx_setup_arch(void)
    {
    mpc83xx_setup_arch();
    mpc834x_usb_cfg();
    }
    define_machine(mpc834x_itx) {
    .name			= "MPC834x ITX",
    .compatible		= "MPC834xMITX",
    .setup_arch		= mpc834x_itx_setup_arch,
    .discover_phbs  	= mpc83xx_setup_pci,
    .init_IRQ		= mpc83xx_ipic_init_IRQ,
    .get_irq		= ipic_get_irq,
    .restart		= mpc83xx_restart,
    .time_init		= mpc83xx_time_init,
    .progress		= udbg_progress,
    };
