//! Automatically rewritten from C to Rust
//! Source: drivers/bcma/driver_chipcommon_b.c
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
// ChipCommon B Unit driver
//
// Copyright 2014, Hauke Mehrtens <hauke@hauke-m.de>
//
// Licensed under the GNU/GPL. See COPYING for details.
//

    static bool bcma_wait_reg(struct bcma_bus *bus, void __iomem *addr, u32 mask,
    u32 value, int timeout)
    {
    let mut deadline: c_ulong = jiffies + timeout;
    u32 val;
    do {
    val = readl(addr);
    if ((val & mask) == value)
    return true;
    cpu_relax();
    udelay(10);
    } while (!time_after_eq(jiffies, deadline));
    bcma_err(bus, "Timeout waiting for register %p\n", addr);
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn bcma_chipco_b_mii_write(ccb: *mut bcma_drv_cc_b, offset: u32, value: u32) {
    void bcma_chipco_b_mii_write(struct bcma_drv_cc_b *ccb, u32 offset, u32 value)
    {
    struct bcma_bus *bus = ccb.core.bus;
    void __iomem *mii = ccb.mii;
    writel(offset, mii + BCMA_CCB_MII_MNG_CTL);
    bcma_wait_reg(bus, mii + BCMA_CCB_MII_MNG_CTL, 0x0100, 0x0000, 100);
    writel(value, mii + BCMA_CCB_MII_MNG_CMD_DATA);
    bcma_wait_reg(bus, mii + BCMA_CCB_MII_MNG_CTL, 0x0100, 0x0000, 100);
    }
    EXPORT_SYMBOL_GPL(bcma_chipco_b_mii_write);
#[no_mangle]
pub unsafe extern "C" fn bcma_core_chipcommon_b_init(ccb: *mut bcma_drv_cc_b) -> c_int {
    int bcma_core_chipcommon_b_init(struct bcma_drv_cc_b *ccb)
    {
    if (ccb.setup_done)
    return 0;
    ccb.setup_done = 1;
    ccb.mii = ioremap(ccb.core.addr_s[1], BCMA_CORE_SIZE);
    if (!ccb.mii)
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bcma_core_chipcommon_b_free(ccb: *mut bcma_drv_cc_b) {
    void bcma_core_chipcommon_b_free(struct bcma_drv_cc_b *ccb)
    {
    if (ccb.mii)
    iounmap(ccb.mii);
    }
