//! Automatically rewritten from C to Rust
//! Source: drivers/bcma/core.c
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
// Core ops
//
// Licensed under the GNU/GPL. See COPYING for details.
//

    static bool bcma_core_wait_value(struct bcma_device *core, u16 reg, u32 mask,
    u32 value, int timeout)
    {
    let mut deadline: c_ulong = jiffies + timeout;
    u32 val;
    do {
    val = bcma_aread32(core, reg);
    if ((val & mask) == value)
    return true;
    cpu_relax();
    udelay(10);
    } while (!time_after_eq(jiffies, deadline));
    bcma_warn(core.bus, "Timeout waiting for register 0x%04X!\n", reg);
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn bcma_core_is_enabled(core: *mut bcma_device) -> bool {
    bool bcma_core_is_enabled(struct bcma_device *core)
    {
    if ((bcma_aread32(core, BCMA_IOCTL) & (BCMA_IOCTL_CLK | BCMA_IOCTL_FGC))
    != BCMA_IOCTL_CLK)
    return false;
    if (bcma_aread32(core, BCMA_RESET_CTL) & BCMA_RESET_CTL_RESET)
    return false;
    return true;
    }
    EXPORT_SYMBOL_GPL(bcma_core_is_enabled);
#[no_mangle]
pub unsafe extern "C" fn bcma_core_disable(core: *mut bcma_device, flags: u32) {
    void bcma_core_disable(struct bcma_device *core, u32 flags)
    {
    if (bcma_aread32(core, BCMA_RESET_CTL) & BCMA_RESET_CTL_RESET)
    return;
    bcma_core_wait_value(core, BCMA_RESET_ST, ~0, 0, 300);
    bcma_awrite32(core, BCMA_RESET_CTL, BCMA_RESET_CTL_RESET);
    bcma_aread32(core, BCMA_RESET_CTL);
    udelay(1);
    bcma_awrite32(core, BCMA_IOCTL, flags);
    bcma_aread32(core, BCMA_IOCTL);
    udelay(10);
    }
    EXPORT_SYMBOL_GPL(bcma_core_disable);
#[no_mangle]
pub unsafe extern "C" fn bcma_core_enable(core: *mut bcma_device, flags: u32) -> c_int {
    int bcma_core_enable(struct bcma_device *core, u32 flags)
    {
    bcma_core_disable(core, flags);
    bcma_awrite32(core, BCMA_IOCTL, (BCMA_IOCTL_CLK | BCMA_IOCTL_FGC | flags));
    bcma_aread32(core, BCMA_IOCTL);
    bcma_awrite32(core, BCMA_RESET_CTL, 0);
    bcma_aread32(core, BCMA_RESET_CTL);
    udelay(1);
    bcma_awrite32(core, BCMA_IOCTL, (BCMA_IOCTL_CLK | flags));
    bcma_aread32(core, BCMA_IOCTL);
    udelay(1);
    return 0;
    }
    EXPORT_SYMBOL_GPL(bcma_core_enable);
    void bcma_core_set_clockmode(struct bcma_device *core,
    enum bcma_clkmode clkmode)
    {
    u16 i;
    WARN_ON(core.id.id != BCMA_CORE_CHIPCOMMON &&
    core.id.id != BCMA_CORE_PCIE &&
    core.id.id != BCMA_CORE_80211);
    switch (clkmode) {
    case BCMA_CLKMODE_FAST:
    bcma_set32(core, BCMA_CLKCTLST, BCMA_CLKCTLST_FORCEHT);
    usleep_range(64, 300);
    for (i = 0; i < 1500; i++) {
    if (bcma_read32(core, BCMA_CLKCTLST) &
    BCMA_CLKCTLST_HAVEHT) {
    i = 0;
    break;
    }
    udelay(10);
    }
    if (i)
    bcma_err(core.bus, "HT force timeout\n");
    break;
    case BCMA_CLKMODE_DYNAMIC:
    bcma_set32(core, BCMA_CLKCTLST, ~BCMA_CLKCTLST_FORCEHT);
    break;
    }
    }
    EXPORT_SYMBOL_GPL(bcma_core_set_clockmode);
#[no_mangle]
pub unsafe extern "C" fn bcma_core_pll_ctl(core: *mut bcma_device, req: u32, status: u32, on: bool) {
    void bcma_core_pll_ctl(struct bcma_device *core, u32 req, u32 status, bool on)
    {
    u16 i;
    WARN_ON(req & ~BCMA_CLKCTLST_EXTRESREQ);
    WARN_ON(status & ~BCMA_CLKCTLST_EXTRESST);
    if (on) {
    bcma_set32(core, BCMA_CLKCTLST, req);
    for (i = 0; i < 10000; i++) {
    if ((bcma_read32(core, BCMA_CLKCTLST) & status) ==
    status) {
    i = 0;
    break;
    }
    udelay(10);
    }
    if (i)
    bcma_err(core.bus, "PLL enable timeout\n");
    } else {
//
// Mask the PLL but don't wait for it to be disabled. PLL may be
// shared between cores and will be still up if there is another
// core using it.
//
    bcma_mask32(core, BCMA_CLKCTLST, ~req);
    bcma_read32(core, BCMA_CLKCTLST);
    }
    }
    EXPORT_SYMBOL_GPL(bcma_core_pll_ctl);
#[no_mangle]
pub unsafe extern "C" fn bcma_core_dma_translation(core: *mut bcma_device) -> u32 {
    u32 bcma_core_dma_translation(struct bcma_device *core)
    {
    switch (core.bus.hosttype) {
    case BCMA_HOSTTYPE_SOC:
    return 0;
    case BCMA_HOSTTYPE_PCI:
    if (bcma_aread32(core, BCMA_IOST) & BCMA_IOST_DMA64)
    return BCMA_DMA_TRANSLATION_DMA64_CMT;
    else
    return BCMA_DMA_TRANSLATION_DMA32_CMT;
    default:
    bcma_err(core.bus, "DMA translation unknown for host %d\n",
    core.bus.hosttype);
    }
    return BCMA_DMA_TRANSLATION_NONE;
    }
    EXPORT_SYMBOL(bcma_core_dma_translation);
