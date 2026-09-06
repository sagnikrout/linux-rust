//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/85xx/p1023_rdb.c
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
// Copyright 2010-2011, 2013 Freescale Semiconductor, Inc.
//
// Author: Roy Zang <tie-fei.zang@freescale.com>
//
// Description:
// P1023 RDB Board Setup
//

//
// Setup the architecture
//
#[no_mangle]
unsafe extern "C" fn p1023_rdb_setup_arch() -> void __init {
    static void __init p1023_rdb_setup_arch(void)
    {
    struct device_node *np;
    if (ppc_md.progress)
    ppc_md.progress("p1023_rdb_setup_arch()", 0);
// Map BCSR area
    np = of_find_node_by_name(core::ptr::null_mut(), "bcsr");
    if (np != core::ptr::null_mut()) {
    static u8 __iomem *bcsr_regs;
    bcsr_regs = of_iomap(np, 0);
    of_node_put(np);
    if (!bcsr_regs) {
    printk(KERN_ERR
    "BCSR: Failed to map bcsr register space\n");
    return;
    } else {
pub const BCSR15_I2C_BUS0_SEG_CLR: c_uint = 0x07;
pub const BCSR15_I2C_BUS0_SEG2: c_uint = 0x02;
//
// Note: Accessing exclusively i2c devices.
//
// The i2c controller selects initially ID EEPROM in the u-boot;
// but if menu configuration selects RTC support in the kernel,
// the i2c controller switches to select RTC chip in the kernel.
//

// Enable RTC chip on the segment #2 of i2c
    clrbits8(&bcsr_regs[15], BCSR15_I2C_BUS0_SEG_CLR);
    setbits8(&bcsr_regs[15], BCSR15_I2C_BUS0_SEG2);

    iounmap(bcsr_regs);
    }
    }
    mpc85xx_smp_init();
    fsl_pci_assign_primary();
    }
    machine_arch_initcall(p1023_rdb, mpc85xx_common_publish_devices);
#[no_mangle]
unsafe extern "C" fn p1023_rdb_pic_init() -> void __init {
    static void __init p1023_rdb_pic_init(void)
    {
    struct mpic *mpic = mpic_alloc(core::ptr::null_mut(), 0, MPIC_BIG_ENDIAN |
    MPIC_SINGLE_DEST_CPU,
    0, 256, " OpenPIC  ");
    BUG_ON(mpic == core::ptr::null_mut());
    mpic_init(mpic);
    }
    define_machine(p1023_rdb) {
    .name			= "P1023 RDB",
    .compatible		= "fsl,P1023RDB",
    .setup_arch		= p1023_rdb_setup_arch,
    .init_IRQ		= p1023_rdb_pic_init,
    .get_irq		= mpic_get_irq,
    .progress		= udbg_progress,

    .pcibios_fixup_bus	= fsl_pcibios_fixup_bus,
    .pcibios_fixup_phb      = fsl_pcibios_fixup_phb,

    };
