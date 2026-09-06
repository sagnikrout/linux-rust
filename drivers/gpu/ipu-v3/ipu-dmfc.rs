//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/ipu-v3/ipu-dmfc.c
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
// Copyright (c) 2010 Sascha Hauer <s.hauer@pengutronix.de>
// Copyright (C) 2005-2009 Freescale Semiconductor, Inc.
//

pub const DMFC_RD_CHAN: c_uint = 0x0000;
pub const DMFC_WR_CHAN: c_uint = 0x0004;
pub const DMFC_WR_CHAN_DEF: c_uint = 0x0008;
pub const DMFC_DP_CHAN: c_uint = 0x000c;
pub const DMFC_DP_CHAN_DEF: c_uint = 0x0010;
pub const DMFC_GENERAL1: c_uint = 0x0014;
pub const DMFC_GENERAL2: c_uint = 0x0018;
pub const DMFC_IC_CTRL: c_uint = 0x001c;
pub const DMFC_WR_CHAN_ALT: c_uint = 0x0020;
pub const DMFC_WR_CHAN_DEF_ALT: c_uint = 0x0024;
pub const DMFC_DP_CHAN_ALT: c_uint = 0x0028;
pub const DMFC_DP_CHAN_DEF_ALT: c_uint = 0x002c;
pub const DMFC_GENERAL1_ALT: c_uint = 0x0030;
pub const DMFC_STAT: c_uint = 0x0034;
pub const DMFC_WR_CHAN_1_28: c_int = 0;
pub const DMFC_WR_CHAN_2_41: c_int = 8;
pub const DMFC_WR_CHAN_1C_42: c_int = 16;
pub const DMFC_WR_CHAN_2C_43: c_int = 24;
pub const DMFC_DP_CHAN_5B_23: c_int = 0;
pub const DMFC_DP_CHAN_5F_27: c_int = 8;
pub const DMFC_DP_CHAN_6B_24: c_int = 16;
pub const DMFC_DP_CHAN_6F_29: c_int = 24;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmfc_channel_data {
    pub ipu_channel: c_int,
    pub channel_reg: c_ulong,
    pub shift: c_ulong,
    pub eot_shift: unsigned,
    pub max_fifo_lines: unsigned,
}

    static const struct dmfc_channel_data dmfcdata[] = {
    {
    .ipu_channel	= IPUV3_CHANNEL_MEM_BG_SYNC,
    .channel_reg	= DMFC_DP_CHAN,
    .shift		= DMFC_DP_CHAN_5B_23,
    .eot_shift	= 20,
    .max_fifo_lines	= 3,
    }, {
    .ipu_channel	= 24,
    .channel_reg	= DMFC_DP_CHAN,
    .shift		= DMFC_DP_CHAN_6B_24,
    .eot_shift	= 22,
    .max_fifo_lines	= 1,
    }, {
    .ipu_channel	= IPUV3_CHANNEL_MEM_FG_SYNC,
    .channel_reg	= DMFC_DP_CHAN,
    .shift		= DMFC_DP_CHAN_5F_27,
    .eot_shift	= 21,
    .max_fifo_lines	= 2,
    }, {
    .ipu_channel	= IPUV3_CHANNEL_MEM_DC_SYNC,
    .channel_reg	= DMFC_WR_CHAN,
    .shift		= DMFC_WR_CHAN_1_28,
    .eot_shift	= 16,
    .max_fifo_lines	= 2,
    }, {
    .ipu_channel	= 29,
    .channel_reg	= DMFC_DP_CHAN,
    .shift		= DMFC_DP_CHAN_6F_29,
    .eot_shift	= 23,
    .max_fifo_lines	= 1,
    },
    };

    struct ipu_dmfc_priv;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmfc_channel {
    pub slots: unsigned,
    pub ipu: *mut ipu_soc,
    pub priv: *mut ipu_dmfc_priv,
    pub data: *const dmfc_channel_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu_dmfc_priv {
    pub ipu: *mut ipu_soc,
    pub dev: *mut device,
    pub channels: [dmfc_channel; DMFC_NUM_CHANNELS],
    pub mutex: mutex,
    pub base: *mut void __iomem,
    pub use_count: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn ipu_dmfc_enable_channel(dmfc: *mut dmfc_channel) -> c_int {
    int ipu_dmfc_enable_channel(struct dmfc_channel *dmfc)
    {
    struct ipu_dmfc_priv *priv = dmfc.priv;
    mutex_lock(&priv.mutex);
    if (!priv.use_count)
    ipu_module_enable(priv.ipu, IPU_CONF_DMFC_EN);
    priv.use_count++;
    mutex_unlock(&priv.mutex);
    return 0;
    }
    EXPORT_SYMBOL_GPL(ipu_dmfc_enable_channel);
#[no_mangle]
pub unsafe extern "C" fn ipu_dmfc_disable_channel(dmfc: *mut dmfc_channel) {
    void ipu_dmfc_disable_channel(struct dmfc_channel *dmfc)
    {
    struct ipu_dmfc_priv *priv = dmfc.priv;
    mutex_lock(&priv.mutex);
    priv.use_count--;
    if (!priv.use_count)
    ipu_module_disable(priv.ipu, IPU_CONF_DMFC_EN);
    if (priv.use_count < 0)
    priv.use_count = 0;
    mutex_unlock(&priv.mutex);
    }
    EXPORT_SYMBOL_GPL(ipu_dmfc_disable_channel);
#[no_mangle]
pub unsafe extern "C" fn ipu_dmfc_config_wait4eot(dmfc: *mut dmfc_channel, width: c_int) {
    void ipu_dmfc_config_wait4eot(struct dmfc_channel *dmfc, int width)
    {
    struct ipu_dmfc_priv *priv = dmfc.priv;
    u32 dmfc_gen1;
    mutex_lock(&priv.mutex);
    dmfc_gen1 = readl(priv.base + DMFC_GENERAL1);
    if ((dmfc.slots * 64 * 4) / width > dmfc.data.max_fifo_lines)
    dmfc_gen1 |= 1 << dmfc.data.eot_shift;
    else
    dmfc_gen1 &= ~(1 << dmfc.data.eot_shift);
    writel(dmfc_gen1, priv.base + DMFC_GENERAL1);
    mutex_unlock(&priv.mutex);
    }
    EXPORT_SYMBOL_GPL(ipu_dmfc_config_wait4eot);
    struct dmfc_channel *ipu_dmfc_get(struct ipu_soc *ipu, int ipu_channel)
    {
    struct ipu_dmfc_priv *priv = ipu.dmfc_priv;
    int i;
    for (i = 0; i < DMFC_NUM_CHANNELS; i++)
    if (dmfcdata[i].ipu_channel == ipu_channel)
    return &priv.channels[i];
    return ERR_PTR(-ENODEV);
    }
    EXPORT_SYMBOL_GPL(ipu_dmfc_get);
#[no_mangle]
pub unsafe extern "C" fn ipu_dmfc_put(dmfc: *mut dmfc_channel) {
    void ipu_dmfc_put(struct dmfc_channel *dmfc)
    {
    }
    EXPORT_SYMBOL_GPL(ipu_dmfc_put);
    int ipu_dmfc_init(struct ipu_soc *ipu, struct device *dev, unsigned long base,
    struct clk *ipu_clk)
    {
    struct ipu_dmfc_priv *priv;
    int i;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.base = devm_ioremap(dev, base, PAGE_SIZE);
    if (!priv.base)
    return -ENOMEM;
    priv.dev = dev;
    priv.ipu = ipu;
    mutex_init(&priv.mutex);
    ipu.dmfc_priv = priv;
    for (i = 0; i < DMFC_NUM_CHANNELS; i++) {
    priv.channels[i].priv = priv;
    priv.channels[i].ipu = ipu;
    priv.channels[i].data = &dmfcdata[i];
    if (dmfcdata[i].ipu_channel == IPUV3_CHANNEL_MEM_BG_SYNC ||
    dmfcdata[i].ipu_channel == IPUV3_CHANNEL_MEM_FG_SYNC ||
    dmfcdata[i].ipu_channel == IPUV3_CHANNEL_MEM_DC_SYNC)
    priv.channels[i].slots = 2;
    }
    writel(0x00000050, priv.base + DMFC_WR_CHAN);
    writel(0x00005654, priv.base + DMFC_DP_CHAN);
    writel(0x202020f6, priv.base + DMFC_WR_CHAN_DEF);
    writel(0x2020f6f6, priv.base + DMFC_DP_CHAN_DEF);
    writel(0x00000003, priv.base + DMFC_GENERAL1);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ipu_dmfc_exit(ipu: *mut ipu_soc) {
    void ipu_dmfc_exit(struct ipu_soc *ipu)
    {
    }
