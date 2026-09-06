//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mmp/pwr-island.c
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
// MMP PMU power island support
//
// Copyright (C) 2020 Lubomir Rintel <lkundrak@v3.sk>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmp_pm_domain {
    pub genpd: generic_pm_domain,
    pub reg: *mut void __iomem,
    pub lock: *mut spinlock_t,
    pub power_on: u32,
    pub reset: u32,
    pub clock_enable: u32,
    pub flags: c_uint,
}

#[no_mangle]
unsafe extern "C" fn mmp_pm_domain_power_on(genpd: *mut generic_pm_domain) -> c_int {
    static int mmp_pm_domain_power_on(struct generic_pm_domain *genpd)
    {
    struct mmp_pm_domain *pm_domain = to_mmp_pm_domain(genpd);
    let mut flags: c_ulong = 0;
    u32 val;
    if (pm_domain.lock)
    spin_lock_irqsave(pm_domain.lock, flags);
    val = readl(pm_domain.reg);
// Turn on the power island
    val |= pm_domain.power_on;
    writel(val, pm_domain.reg);
// Disable isolation
    val |= 0x100;
    writel(val, pm_domain.reg);
// Some blocks need to be reset after a power up
    if (pm_domain.reset || pm_domain.clock_enable) {
    let mut after_power_on: u32 = val;
    val &= ~pm_domain.reset;
    writel(val, pm_domain.reg);
    val |= pm_domain.clock_enable;
    writel(val, pm_domain.reg);
    val |= pm_domain.reset;
    writel(val, pm_domain.reg);
    writel(after_power_on, pm_domain.reg);
    }
    if (pm_domain.lock)
    spin_unlock_irqrestore(pm_domain.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mmp_pm_domain_power_off(genpd: *mut generic_pm_domain) -> c_int {
    static int mmp_pm_domain_power_off(struct generic_pm_domain *genpd)
    {
    struct mmp_pm_domain *pm_domain = to_mmp_pm_domain(genpd);
    let mut flags: c_ulong = 0;
    u32 val;
    if (pm_domain.flags & MMP_PM_DOMAIN_NO_DISABLE)
    return 0;
    if (pm_domain.lock)
    spin_lock_irqsave(pm_domain.lock, flags);
// Turn off and isolate the power island.
    val = readl(pm_domain.reg);
    val &= ~pm_domain.power_on;
    val &= ~0x100;
    writel(val, pm_domain.reg);
    if (pm_domain.lock)
    spin_unlock_irqrestore(pm_domain.lock, flags);
    return 0;
    }
    struct generic_pm_domain *mmp_pm_domain_register(const char *name,
    void __iomem *reg,
    u32 power_on, u32 reset, u32 clock_enable,
    unsigned int flags, spinlock_t *lock)
    {
    struct mmp_pm_domain *pm_domain;
    pm_domain = kzalloc_obj(*pm_domain);
    if (!pm_domain)
    return ERR_PTR(-ENOMEM);
    pm_domain.reg = reg;
    pm_domain.power_on = power_on;
    pm_domain.reset = reset;
    pm_domain.clock_enable = clock_enable;
    pm_domain.flags = flags;
    pm_domain.lock = lock;
    pm_domain.genpd.name = name;
    pm_domain.genpd.power_on = mmp_pm_domain_power_on;
    pm_domain.genpd.power_off = mmp_pm_domain_power_off;
    pm_genpd_init(&pm_domain.genpd, core::ptr::null_mut(), true);
    return &pm_domain.genpd;
    }
