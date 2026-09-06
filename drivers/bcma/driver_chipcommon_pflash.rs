//! Automatically rewritten from C to Rust
//! Source: drivers/bcma/driver_chipcommon_pflash.c
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
// ChipCommon parallel flash
//
// Licensed under the GNU/GPL. See COPYING for details.
//

    static const char * const part_probes[] = { "bcm47xxpart", core::ptr::null_mut() };
    static struct physmap_flash_data bcma_pflash_data = {
    .part_probe_types	= part_probes,
    };
    static struct resource bcma_pflash_resource = {
    .name	= "bcma_pflash",
    .flags  = IORESOURCE_MEM,
    };
    struct platform_device bcma_pflash_dev = {
    .name		= "physmap-flash",
    .dev		= {
    .platform_data  = &bcma_pflash_data,
    },
    .resource	= &bcma_pflash_resource,
    .num_resources	= 1,
    };
#[no_mangle]
pub unsafe extern "C" fn bcma_pflash_init(cc: *mut bcma_drv_cc) -> c_int {
    int bcma_pflash_init(struct bcma_drv_cc *cc)
    {
    struct bcma_pflash *pflash = &cc.pflash;
    pflash.present = true;
    if (!(bcma_read32(cc.core, BCMA_CC_FLASH_CFG) & BCMA_CC_FLASH_CFG_DS))
    bcma_pflash_data.width = 1;
    else
    bcma_pflash_data.width = 2;
    bcma_pflash_resource.start = BCMA_SOC_FLASH2;
    bcma_pflash_resource.end = BCMA_SOC_FLASH2 + BCMA_SOC_FLASH2_SZ;
    return 0;
    }
