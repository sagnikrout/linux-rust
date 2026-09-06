//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/nand/raw/brcmnand/bcma_nand.c
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
// Copyright © 2021 Broadcom
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmnand_bcma_soc {
    pub soc: brcmnand_soc,
    pub cc: *mut bcma_drv_cc,
}

#[no_mangle]
pub unsafe extern "C" fn brcmnand_bcma_needs_swapping(offset: u32) -> bool {
    static inline bool brcmnand_bcma_needs_swapping(u32 offset)
    {
    switch (offset) {
    case BCMA_CC_NAND_SPARE_RD0:
    case BCMA_CC_NAND_SPARE_RD4:
    case BCMA_CC_NAND_SPARE_RD8:
    case BCMA_CC_NAND_SPARE_RD12:
    case BCMA_CC_NAND_SPARE_WR0:
    case BCMA_CC_NAND_SPARE_WR4:
    case BCMA_CC_NAND_SPARE_WR8:
    case BCMA_CC_NAND_SPARE_WR12:
    case BCMA_CC_NAND_DEVID:
    case BCMA_CC_NAND_DEVID_X:
    case BCMA_CC_NAND_SPARE_RD16:
    case BCMA_CC_NAND_SPARE_RD20:
    case BCMA_CC_NAND_SPARE_RD24:
    case BCMA_CC_NAND_SPARE_RD28:
    return true;
    }
    return false;
    }
    static inline struct brcmnand_bcma_soc *to_bcma_soc(struct brcmnand_soc *soc)
    {
    return container_of(soc, struct brcmnand_bcma_soc, soc);
    }
#[no_mangle]
unsafe extern "C" fn brcmnand_bcma_read_reg(soc: *mut brcmnand_soc, offset: u32) -> u32 {
    static u32 brcmnand_bcma_read_reg(struct brcmnand_soc *soc, u32 offset)
    {
    struct brcmnand_bcma_soc *sc = to_bcma_soc(soc);
    u32 val;
// Offset into the NAND block and deal with the flash cache separately
    if (offset == BRCMNAND_NON_MMIO_FC_ADDR)
    offset = BCMA_CC_NAND_CACHE_DATA;
    else
    offset += BCMA_CC_NAND_REVISION;
    val = bcma_cc_read32(sc.cc, offset);
// Swap if necessary
    if (brcmnand_bcma_needs_swapping(offset))
    val = be32_to_cpu(( __be32)val);
    return val;
    }
    static void brcmnand_bcma_write_reg(struct brcmnand_soc *soc, u32 val,
    u32 offset)
    {
    struct brcmnand_bcma_soc *sc = to_bcma_soc(soc);
// Offset into the NAND block
    if (offset == BRCMNAND_NON_MMIO_FC_ADDR)
    offset = BCMA_CC_NAND_CACHE_DATA;
    else
    offset += BCMA_CC_NAND_REVISION;
// Swap if necessary
    if (brcmnand_bcma_needs_swapping(offset))
    val = ( u32)cpu_to_be32(val);
    bcma_cc_write32(sc.cc, offset, val);
    }
    static struct brcmnand_io_ops brcmnand_bcma_io_ops = {
    .read_reg	= brcmnand_bcma_read_reg,
    .write_reg	= brcmnand_bcma_write_reg,
    };
    static void brcmnand_bcma_prepare_data_bus(struct brcmnand_soc *soc, bool prepare,
    bool is_param)
    {
    struct brcmnand_bcma_soc *sc = to_bcma_soc(soc);
// Reset the cache address to ensure we are already accessing the
// beginning of a sub-page.
//
    bcma_cc_write32(sc.cc, BCMA_CC_NAND_CACHE_ADDR, 0);
    }
#[no_mangle]
unsafe extern "C" fn brcmnand_bcma_nand_probe(pdev: *mut platform_device) -> c_int {
    static int brcmnand_bcma_nand_probe(struct platform_device *pdev)
    {
    struct bcma_nflash *nflash = dev_get_platdata(&pdev.dev);
    struct brcmnand_bcma_soc *soc;
    soc = devm_kzalloc(&pdev.dev, sizeof(*soc), GFP_KERNEL);
    if (!soc)
    return -ENOMEM;
    soc.cc = container_of(nflash, struct bcma_drv_cc, nflash);
    soc.soc.prepare_data_bus = brcmnand_bcma_prepare_data_bus;
    soc.soc.ops = &brcmnand_bcma_io_ops;
    if (soc.cc.core.bus.chipinfo.id == BCMA_CHIP_ID_BCM4706) {
    dev_err(&pdev.dev, "Use bcm47xxnflash for 4706!\n");
    return -ENODEV;
    }
    return brcmnand_probe(pdev, &soc.soc);
    }
    static struct platform_driver brcmnand_bcma_nand_driver = {
    .probe			= brcmnand_bcma_nand_probe,
    .remove			= brcmnand_remove,
    .driver = {
    .name		= "bcma_brcmnand",
    .pm		= &brcmnand_pm_ops,
    }
    };
    module_platform_driver(brcmnand_bcma_nand_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Broadcom");
    MODULE_DESCRIPTION("NAND controller driver glue for BCMA chips");
