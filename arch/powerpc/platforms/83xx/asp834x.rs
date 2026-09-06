//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/83xx/asp834x.c
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
// arch/powerpc/platforms/83xx/asp834x.c
//
// Analogue & Micro ASP8347 board specific routines
// clone of mpc834x_itx
//
// Copyright 2008 Codehermit
//
// Maintainer: Bryan O'Donoghue <bodonoghue@codhermit.ie>
//

//
// Setup the architecture
//
#[no_mangle]
unsafe extern "C" fn asp834x_setup_arch() -> void __init {
    static void __init asp834x_setup_arch(void)
    {
    mpc83xx_setup_arch();
    mpc834x_usb_cfg();
    }
    machine_device_initcall(asp834x, mpc83xx_declare_of_platform_devices);
    define_machine(asp834x) {
    .name			= "ASP8347E",
    .compatible		= "analogue-and-micro,asp8347e",
    .setup_arch		= asp834x_setup_arch,
    .discover_phbs		= mpc83xx_setup_pci,
    .init_IRQ		= mpc83xx_ipic_init_IRQ,
    .get_irq		= ipic_get_irq,
    .restart		= mpc83xx_restart,
    .time_init		= mpc83xx_time_init,
    .progress		= udbg_progress,
    };
