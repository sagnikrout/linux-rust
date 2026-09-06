//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/ipu-v3/ipu-smfc.c
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
// Copyright 2008-2010 Freescale Semiconductor, Inc. All Rights Reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu_smfc {
    pub priv: *mut ipu_smfc_priv,
    pub chno: c_int,
    pub inuse: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu_smfc_priv {
    pub base: *mut void __iomem,
    pub lock: spinlock_t,
    pub ipu: *mut ipu_soc,
    pub channel: [ipu_smfc; 4],
    pub use_count: c_int,
}

// SMFC Registers
pub const SMFC_MAP: c_uint = 0x0000;
pub const SMFC_WMC: c_uint = 0x0004;
pub const SMFC_BS: c_uint = 0x0008;
#[no_mangle]
pub unsafe extern "C" fn ipu_smfc_set_burstsize(smfc: *mut ipu_smfc, burstsize: c_int) -> c_int {
    int ipu_smfc_set_burstsize(struct ipu_smfc *smfc, int burstsize)
    {
    struct ipu_smfc_priv *priv = smfc.priv;
    unsigned long flags;
    u32 val, shift;
    spin_lock_irqsave(&priv.lock, flags);
    shift = smfc.chno * 4;
    val = readl(priv.base + SMFC_BS);
    val &= ~(0xf << shift);
    val |= burstsize << shift;
    writel(val, priv.base + SMFC_BS);
    spin_unlock_irqrestore(&priv.lock, flags);
    return 0;
    }
    EXPORT_SYMBOL_GPL(ipu_smfc_set_burstsize);
#[no_mangle]
pub unsafe extern "C" fn ipu_smfc_map_channel(smfc: *mut ipu_smfc, csi_id: c_int, mipi_id: c_int) -> c_int {
    int ipu_smfc_map_channel(struct ipu_smfc *smfc, int csi_id, int mipi_id)
    {
    struct ipu_smfc_priv *priv = smfc.priv;
    unsigned long flags;
    u32 val, shift;
    spin_lock_irqsave(&priv.lock, flags);
    shift = smfc.chno * 3;
    val = readl(priv.base + SMFC_MAP);
    val &= ~(0x7 << shift);
    val |= ((csi_id << 2) | mipi_id) << shift;
    writel(val, priv.base + SMFC_MAP);
    spin_unlock_irqrestore(&priv.lock, flags);
    return 0;
    }
    EXPORT_SYMBOL_GPL(ipu_smfc_map_channel);
#[no_mangle]
pub unsafe extern "C" fn ipu_smfc_set_watermark(smfc: *mut ipu_smfc, set_level: u32, clr_level: u32) -> c_int {
    int ipu_smfc_set_watermark(struct ipu_smfc *smfc, u32 set_level, u32 clr_level)
    {
    struct ipu_smfc_priv *priv = smfc.priv;
    unsigned long flags;
    u32 val, shift;
    spin_lock_irqsave(&priv.lock, flags);
    shift = smfc.chno * 6 + (smfc.chno > 1 ? 4 : 0);
    val = readl(priv.base + SMFC_WMC);
    val &= ~(0x3f << shift);
    val |= ((clr_level << 3) | set_level) << shift;
    writel(val, priv.base + SMFC_WMC);
    spin_unlock_irqrestore(&priv.lock, flags);
    return 0;
    }
    EXPORT_SYMBOL_GPL(ipu_smfc_set_watermark);
#[no_mangle]
pub unsafe extern "C" fn ipu_smfc_enable(smfc: *mut ipu_smfc) -> c_int {
    int ipu_smfc_enable(struct ipu_smfc *smfc)
    {
    struct ipu_smfc_priv *priv = smfc.priv;
    unsigned long flags;
    spin_lock_irqsave(&priv.lock, flags);
    if (!priv.use_count)
    ipu_module_enable(priv.ipu, IPU_CONF_SMFC_EN);
    priv.use_count++;
    spin_unlock_irqrestore(&priv.lock, flags);
    return 0;
    }
    EXPORT_SYMBOL_GPL(ipu_smfc_enable);
#[no_mangle]
pub unsafe extern "C" fn ipu_smfc_disable(smfc: *mut ipu_smfc) -> c_int {
    int ipu_smfc_disable(struct ipu_smfc *smfc)
    {
    struct ipu_smfc_priv *priv = smfc.priv;
    unsigned long flags;
    spin_lock_irqsave(&priv.lock, flags);
    priv.use_count--;
    if (!priv.use_count)
    ipu_module_disable(priv.ipu, IPU_CONF_SMFC_EN);
    if (priv.use_count < 0)
    priv.use_count = 0;
    spin_unlock_irqrestore(&priv.lock, flags);
    return 0;
    }
    EXPORT_SYMBOL_GPL(ipu_smfc_disable);
    struct ipu_smfc *ipu_smfc_get(struct ipu_soc *ipu, unsigned int chno)
    {
    struct ipu_smfc_priv *priv = ipu.smfc_priv;
    struct ipu_smfc *smfc, *ret;
    unsigned long flags;
    if (chno >= 4)
    return ERR_PTR(-EINVAL);
    smfc = &priv.channel[chno];
    ret = smfc;
    spin_lock_irqsave(&priv.lock, flags);
    if (smfc.inuse) {
    ret = ERR_PTR(-EBUSY);
    goto unlock;
    }
    smfc.inuse = true;
    unlock:
    spin_unlock_irqrestore(&priv.lock, flags);
    return ret;
    }
    EXPORT_SYMBOL_GPL(ipu_smfc_get);
#[no_mangle]
pub unsafe extern "C" fn ipu_smfc_put(smfc: *mut ipu_smfc) {
    void ipu_smfc_put(struct ipu_smfc *smfc)
    {
    struct ipu_smfc_priv *priv = smfc.priv;
    unsigned long flags;
    spin_lock_irqsave(&priv.lock, flags);
    smfc.inuse = false;
    spin_unlock_irqrestore(&priv.lock, flags);
    }
    EXPORT_SYMBOL_GPL(ipu_smfc_put);
    int ipu_smfc_init(struct ipu_soc *ipu, struct device *dev,
    unsigned long base)
    {
    struct ipu_smfc_priv *priv;
    int i;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    ipu.smfc_priv = priv;
    spin_lock_init(&priv.lock);
    priv.ipu = ipu;
    priv.base = devm_ioremap(dev, base, PAGE_SIZE);
    if (!priv.base)
    return -ENOMEM;
    for (i = 0; i < 4; i++) {
    priv.channel[i].priv = priv;
    priv.channel[i].chno = i;
    }
    pr_debug("%s: ioremap 0x%08lx . %p\n", __func__, base, priv.base);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ipu_smfc_exit(ipu: *mut ipu_soc) {
    void ipu_smfc_exit(struct ipu_soc *ipu)
    {
    }
