//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/44x/ebony.c
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
// Ebony board specific routines
//
// Matt Porter <mporter@kernel.crashing.org>
// Copyright 2002-2005 MontaVista Software Inc.
//
// Eugene Surovegin <eugene.surovegin@zultys.com> or <ebs@ebshome.net>
// Copyright (c) 2003-2005 Zultys Technologies
//
// Rewritten and ported to the merged powerpc tree:
// Copyright 2007 David Gibson <dwg@au1.ibm.com>, IBM Corporation.
//

    static const struct of_device_id ebony_of_bus[] __initconst = {
    { .compatible = "ibm,plb4", },
    { .compatible = "ibm,opb", },
    { .compatible = "ibm,ebc", },
    {},
    };
#[no_mangle]
unsafe extern "C" fn ebony_device_probe() -> int __init {
    static int __init ebony_device_probe(void)
    {
    of_platform_bus_probe(core::ptr::null_mut(), ebony_of_bus, core::ptr::null_mut());
    of_instantiate_rtc();
    return 0;
    }
    machine_device_initcall(ebony, ebony_device_probe);
//
// Called very early, MMU is off, device-tree isn't unflattened
//
#[no_mangle]
unsafe extern "C" fn ebony_probe() -> int __init {
    static int __init ebony_probe(void)
    {
    pci_set_flags(PCI_REASSIGN_ALL_RSRC);
    return 1;
    }
    define_machine(ebony) {
    .name			= "Ebony",
    .compatible		= "ibm,ebony",
    .probe			= ebony_probe,
    .progress		= udbg_progress,
    .init_IRQ		= uic_init_tree,
    .get_irq		= uic_get_irq,
    .restart		= ppc4xx_reset_system,
    };
