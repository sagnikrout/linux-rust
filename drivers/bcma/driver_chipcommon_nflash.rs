//! Automatically rewritten from C to Rust
//! Source: drivers/bcma/driver_chipcommon_nflash.c
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


//
// Broadcom specific AMBA
// ChipCommon NAND flash interface
//
// Licensed under the GNU/GPL. See COPYING for details.
//

// Alternate NAND controller driver name in order to allow both bcm47xxnflash
// and bcma_brcmnand to be built into the same kernel image.
//
    static const char *bcma_nflash_alt_name = "bcma_brcmnand";
    struct platform_device bcma_nflash_dev = {
    .name		= "bcma_nflash",
    .num_resources	= 0,
    };
    static const char *probes[] = { "bcm47xxpart", core::ptr::null_mut() };
// Initialize NAND flash access
#[no_mangle]
pub unsafe extern "C" fn bcma_nflash_init(cc: *mut bcma_drv_cc) -> c_int {
    int bcma_nflash_init(struct bcma_drv_cc *cc)
    {
    struct bcma_bus *bus = cc.core.bus;
    u32 reg;
    if (bus.chipinfo.id != BCMA_CHIP_ID_BCM4706 &&
    cc.core.id.rev != 38) {
    bcma_err(bus, "NAND flash on unsupported board!\n");
    return -ENOTSUPP;
    }
    if (!(cc.capabilities & BCMA_CC_CAP_NFLASH)) {
    bcma_err(bus, "NAND flash not present according to ChipCommon\n");
    return -ENODEV;
    }
    cc.nflash.present = true;
    if (cc.core.id.rev == 38 &&
    (cc.status & BCMA_CC_CHIPST_5357_NAND_BOOT)) {
    cc.nflash.boot = true;
// Determine the chip select that is being used
    reg = bcma_cc_read32(cc, BCMA_CC_NAND_CS_NAND_SELECT) & 0xff;
    cc.nflash.brcmnand_info.chip_select = ffs(reg) - 1;
    cc.nflash.brcmnand_info.part_probe_types = probes;
    cc.nflash.brcmnand_info.ecc_stepsize = 512;
    cc.nflash.brcmnand_info.ecc_strength = 1;
    bcma_nflash_dev.name = bcma_nflash_alt_name;
    }
// Prepare platform device, but don't register it yet. It's too early,
// malloc (required by device_private_init) is not available yet.
    bcma_nflash_dev.dev.platform_data = &cc.nflash;
    return 0;
    }
