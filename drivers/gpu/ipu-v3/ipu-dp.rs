//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/ipu-v3/ipu-dp.c
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

pub const DP_SYNC: c_int = 0;
pub const DP_ASYNC0: c_uint = 0x60;
pub const DP_ASYNC1: c_uint = 0xBC;
pub const DP_COM_CONF: c_uint = 0x0;
pub const DP_GRAPH_WIND_CTRL: c_uint = 0x0004;
pub const DP_FG_POS: c_uint = 0x0008;
pub const DP_CSC_A_0: c_uint = 0x0044;
pub const DP_CSC_A_1: c_uint = 0x0048;
pub const DP_CSC_A_2: c_uint = 0x004C;
pub const DP_CSC_A_3: c_uint = 0x0050;
pub const DP_CSC_0: c_uint = 0x0054;
pub const DP_CSC_1: c_uint = 0x0058;

pub const DP_COM_CONF_CSC_DEF_OFFSET: c_int = 8;

pub const IPUV3_NUM_FLOWS: c_int = 3;
    struct ipu_dp_priv;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu_dp {
    pub flow: u32,
    pub in_use: bool,
    pub foreground: bool,
    pub in_cs: enum ipu_color_space,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu_flow {
    pub foreground: ipu_dp,
    pub background: ipu_dp,
    pub out_cs: enum ipu_color_space,
    pub base: *mut void __iomem,
    pub priv: *mut ipu_dp_priv,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu_dp_priv {
    pub ipu: *mut ipu_soc,
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub flow: [ipu_flow; IPUV3_NUM_FLOWS],
    pub mutex: mutex,
    pub use_count: c_int,
}

    static u32 ipu_dp_flow_base[] = {DP_SYNC, DP_ASYNC0, DP_ASYNC1};
    static inline struct ipu_flow *to_flow(struct ipu_dp *dp)
    {
    if (dp.foreground)
    return container_of(dp, struct ipu_flow, foreground);
    else
    return container_of(dp, struct ipu_flow, background);
    }
    int ipu_dp_set_global_alpha(struct ipu_dp *dp, bool enable,
    u8 alpha, bool bg_chan)
    {
    struct ipu_flow *flow = to_flow(dp);
    struct ipu_dp_priv *priv = flow.priv;
    u32 reg;
    mutex_lock(&priv.mutex);
    reg = readl(flow.base + DP_COM_CONF);
    if (bg_chan)
    reg &= ~DP_COM_CONF_GWSEL;
    else
    reg |= DP_COM_CONF_GWSEL;
    writel(reg, flow.base + DP_COM_CONF);
    if (enable) {
    reg = readl(flow.base + DP_GRAPH_WIND_CTRL) & 0x00FFFFFFL;
    writel(reg | ((u32) alpha << 24),
    flow.base + DP_GRAPH_WIND_CTRL);
    reg = readl(flow.base + DP_COM_CONF);
    writel(reg | DP_COM_CONF_GWAM, flow.base + DP_COM_CONF);
    } else {
    reg = readl(flow.base + DP_COM_CONF);
    writel(reg & ~DP_COM_CONF_GWAM, flow.base + DP_COM_CONF);
    }
    ipu_srm_dp_update(priv.ipu, true);
    mutex_unlock(&priv.mutex);
    return 0;
    }
    EXPORT_SYMBOL_GPL(ipu_dp_set_global_alpha);
#[no_mangle]
pub unsafe extern "C" fn ipu_dp_set_window_pos(dp: *mut ipu_dp, x_pos: u16, y_pos: u16) -> c_int {
    int ipu_dp_set_window_pos(struct ipu_dp *dp, u16 x_pos, u16 y_pos)
    {
    struct ipu_flow *flow = to_flow(dp);
    struct ipu_dp_priv *priv = flow.priv;
    writel((x_pos << 16) | y_pos, flow.base + DP_FG_POS);
    ipu_srm_dp_update(priv.ipu, true);
    return 0;
    }
    EXPORT_SYMBOL_GPL(ipu_dp_set_window_pos);
    static void ipu_dp_csc_init(struct ipu_flow *flow,
    enum drm_color_encoding ycbcr_enc,
    enum drm_color_range range,
    enum ipu_color_space in,
    enum ipu_color_space out,
    u32 place)
    {
    u32 reg;
    reg = readl(flow.base + DP_COM_CONF);
    reg &= ~DP_COM_CONF_CSC_DEF_MASK;
    if (in == out) {
    writel(reg, flow.base + DP_COM_CONF);
    return;
    }
    if (in == IPUV3_COLORSPACE_RGB && out == IPUV3_COLORSPACE_YUV) {
    writel(0x099 | (0x12d << 16), flow.base + DP_CSC_A_0);
    writel(0x03a | (0x3a9 << 16), flow.base + DP_CSC_A_1);
    writel(0x356 | (0x100 << 16), flow.base + DP_CSC_A_2);
    writel(0x100 | (0x329 << 16), flow.base + DP_CSC_A_3);
    writel(0x3d6 | (0x0000 << 16) | (2 << 30),
    flow.base + DP_CSC_0);
    writel(0x200 | (2 << 14) | (0x200 << 16) | (2 << 30),
    flow.base + DP_CSC_1);
    } else if (ycbcr_enc == DRM_COLOR_YCBCR_BT709) {
// Rec.709 limited range
    writel(0x095 | (0x000 << 16), flow.base + DP_CSC_A_0);
    writel(0x0e5 | (0x095 << 16), flow.base + DP_CSC_A_1);
    writel(0x3e5 | (0x3bc << 16), flow.base + DP_CSC_A_2);
    writel(0x095 | (0x10e << 16), flow.base + DP_CSC_A_3);
    writel(0x000 | (0x3e10 << 16) | (1 << 30),
    flow.base + DP_CSC_0);
    writel(0x09a | (1 << 14) | (0x3dbe << 16) | (1 << 30),
    flow.base + DP_CSC_1);
    } else {
// BT.601 limited range
    writel(0x095 | (0x000 << 16), flow.base + DP_CSC_A_0);
    writel(0x0cc | (0x095 << 16), flow.base + DP_CSC_A_1);
    writel(0x3ce | (0x398 << 16), flow.base + DP_CSC_A_2);
    writel(0x095 | (0x0ff << 16), flow.base + DP_CSC_A_3);
    writel(0x000 | (0x3e42 << 16) | (1 << 30),
    flow.base + DP_CSC_0);
    writel(0x10a | (1 << 14) | (0x3dd6 << 16) | (1 << 30),
    flow.base + DP_CSC_1);
    }
    reg |= place;
    writel(reg, flow.base + DP_COM_CONF);
    }
    int ipu_dp_setup_channel(struct ipu_dp *dp,
    enum drm_color_encoding ycbcr_enc,
    enum drm_color_range range,
    enum ipu_color_space in,
    enum ipu_color_space out)
    {
    struct ipu_flow *flow = to_flow(dp);
    struct ipu_dp_priv *priv = flow.priv;
    mutex_lock(&priv.mutex);
    dp.in_cs = in;
    if (!dp.foreground)
    flow.out_cs = out;
    if (flow.foreground.in_cs == flow.background.in_cs) {
//
// foreground and background are of same colorspace, put
// colorspace converter after combining unit.
//
    ipu_dp_csc_init(flow, ycbcr_enc, range,
    flow.foreground.in_cs, flow.out_cs,
    DP_COM_CONF_CSC_DEF_BOTH);
    } else {
    if (flow.foreground.in_cs == IPUV3_COLORSPACE_UNKNOWN ||
    flow.foreground.in_cs == flow.out_cs)
//
// foreground identical to output, apply color
// conversion on background
//
    ipu_dp_csc_init(flow, ycbcr_enc, range,
    flow.background.in_cs,
    flow.out_cs, DP_COM_CONF_CSC_DEF_BG);
    else
    ipu_dp_csc_init(flow, ycbcr_enc, range,
    flow.foreground.in_cs,
    flow.out_cs, DP_COM_CONF_CSC_DEF_FG);
    }
    ipu_srm_dp_update(priv.ipu, true);
    mutex_unlock(&priv.mutex);
    return 0;
    }
    EXPORT_SYMBOL_GPL(ipu_dp_setup_channel);
#[no_mangle]
pub unsafe extern "C" fn ipu_dp_enable(ipu: *mut ipu_soc) -> c_int {
    int ipu_dp_enable(struct ipu_soc *ipu)
    {
    struct ipu_dp_priv *priv = ipu.dp_priv;
    mutex_lock(&priv.mutex);
    if (!priv.use_count)
    ipu_module_enable(priv.ipu, IPU_CONF_DP_EN);
    priv.use_count++;
    mutex_unlock(&priv.mutex);
    return 0;
    }
    EXPORT_SYMBOL_GPL(ipu_dp_enable);
#[no_mangle]
pub unsafe extern "C" fn ipu_dp_enable_channel(dp: *mut ipu_dp) -> c_int {
    int ipu_dp_enable_channel(struct ipu_dp *dp)
    {
    struct ipu_flow *flow = to_flow(dp);
    struct ipu_dp_priv *priv = flow.priv;
    u32 reg;
    if (!dp.foreground)
    return 0;
    mutex_lock(&priv.mutex);
    reg = readl(flow.base + DP_COM_CONF);
    reg |= DP_COM_CONF_FG_EN;
    writel(reg, flow.base + DP_COM_CONF);
    ipu_srm_dp_update(priv.ipu, true);
    mutex_unlock(&priv.mutex);
    return 0;
    }
    EXPORT_SYMBOL_GPL(ipu_dp_enable_channel);
#[no_mangle]
pub unsafe extern "C" fn ipu_dp_disable_channel(dp: *mut ipu_dp, sync: bool) {
    void ipu_dp_disable_channel(struct ipu_dp *dp, bool sync)
    {
    struct ipu_flow *flow = to_flow(dp);
    struct ipu_dp_priv *priv = flow.priv;
    u32 reg, csc;
    dp.in_cs = IPUV3_COLORSPACE_UNKNOWN;
    if (!dp.foreground)
    return;
    mutex_lock(&priv.mutex);
    reg = readl(flow.base + DP_COM_CONF);
    csc = reg & DP_COM_CONF_CSC_DEF_MASK;
    reg &= ~DP_COM_CONF_CSC_DEF_MASK;
    if (csc == DP_COM_CONF_CSC_DEF_BOTH || csc == DP_COM_CONF_CSC_DEF_BG)
    reg |= DP_COM_CONF_CSC_DEF_BG;
    reg &= ~DP_COM_CONF_FG_EN;
    writel(reg, flow.base + DP_COM_CONF);
    writel(0, flow.base + DP_FG_POS);
    ipu_srm_dp_update(priv.ipu, sync);
    mutex_unlock(&priv.mutex);
    }
    EXPORT_SYMBOL_GPL(ipu_dp_disable_channel);
#[no_mangle]
pub unsafe extern "C" fn ipu_dp_disable(ipu: *mut ipu_soc) {
    void ipu_dp_disable(struct ipu_soc *ipu)
    {
    struct ipu_dp_priv *priv = ipu.dp_priv;
    mutex_lock(&priv.mutex);
    priv.use_count--;
    if (!priv.use_count)
    ipu_module_disable(priv.ipu, IPU_CONF_DP_EN);
    if (priv.use_count < 0)
    priv.use_count = 0;
    mutex_unlock(&priv.mutex);
    }
    EXPORT_SYMBOL_GPL(ipu_dp_disable);
    struct ipu_dp *ipu_dp_get(struct ipu_soc *ipu, unsigned int flow)
    {
    struct ipu_dp_priv *priv = ipu.dp_priv;
    struct ipu_dp *dp;
    if ((flow >> 1) >= IPUV3_NUM_FLOWS)
    return ERR_PTR(-EINVAL);
    if (flow & 1)
    dp = &priv.flow[flow >> 1].foreground;
    else
    dp = &priv.flow[flow >> 1].background;
    if (dp.in_use)
    return ERR_PTR(-EBUSY);
    dp.in_use = true;
    return dp;
    }
    EXPORT_SYMBOL_GPL(ipu_dp_get);
#[no_mangle]
pub unsafe extern "C" fn ipu_dp_put(dp: *mut ipu_dp) {
    void ipu_dp_put(struct ipu_dp *dp)
    {
    dp.in_use = false;
    }
    EXPORT_SYMBOL_GPL(ipu_dp_put);
#[no_mangle]
pub unsafe extern "C" fn ipu_dp_init(ipu: *mut ipu_soc, dev: *mut device, base: c_ulong) -> c_int {
    int ipu_dp_init(struct ipu_soc *ipu, struct device *dev, unsigned long base)
    {
    struct ipu_dp_priv *priv;
    int i;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.dev = dev;
    priv.ipu = ipu;
    ipu.dp_priv = priv;
    priv.base = devm_ioremap(dev, base, PAGE_SIZE);
    if (!priv.base)
    return -ENOMEM;
    mutex_init(&priv.mutex);
    for (i = 0; i < IPUV3_NUM_FLOWS; i++) {
    priv.flow[i].background.in_cs = IPUV3_COLORSPACE_UNKNOWN;
    priv.flow[i].foreground.in_cs = IPUV3_COLORSPACE_UNKNOWN;
    priv.flow[i].foreground.foreground = true;
    priv.flow[i].base = priv.base + ipu_dp_flow_base[i];
    priv.flow[i].priv = priv;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ipu_dp_exit(ipu: *mut ipu_soc) {
    void ipu_dp_exit(struct ipu_soc *ipu)
    {
    }
