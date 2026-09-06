//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/512x/mpc512x_generic.c
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
// Copyright (C) 2007,2008 Freescale Semiconductor, Inc. All rights reserved.
//
// Author: John Rigby, <jrigby@freescale.com>
//
// Description:
// MPC512x SoC setup
//

//
// list of supported boards
//
    static const char * const board[] __initconst = {
    "prt,prtlvt",
    "fsl,mpc5125ads",
    "ifm,ac14xx",
    core::ptr::null_mut()
    };
//
// Called very early, MMU is off, device-tree isn't unflattened
//
#[no_mangle]
unsafe extern "C" fn mpc512x_generic_probe() -> int __init {
    static int __init mpc512x_generic_probe(void)
    {
    mpc512x_init_early();
    return 1;
    }
    define_machine(mpc512x_generic) {
    .name			= "MPC512x generic",
    .compatibles		= board,
    .probe			= mpc512x_generic_probe,
    .init			= mpc512x_init,
    .setup_arch		= mpc512x_setup_arch,
    .init_IRQ		= mpc512x_init_IRQ,
    .get_irq		= ipic_get_irq,
    .restart		= mpc512x_restart,
    };
