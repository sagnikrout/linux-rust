//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/ipu-v3/ipu-dc.c
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

pub const DC_EVT_NF: c_int = 0;
pub const DC_EVT_NL: c_int = 1;
pub const DC_EVT_EOF: c_int = 2;
pub const DC_EVT_NFIELD: c_int = 3;
pub const DC_EVT_EOL: c_int = 4;
pub const DC_EVT_EOFIELD: c_int = 5;
pub const DC_EVT_NEW_ADDR: c_int = 6;
pub const DC_EVT_NEW_CHAN: c_int = 7;
pub const DC_EVT_NEW_DATA: c_int = 8;
pub const DC_EVT_NEW_ADDR_W_0: c_int = 0;
pub const DC_EVT_NEW_ADDR_W_1: c_int = 1;
pub const DC_EVT_NEW_CHAN_W_0: c_int = 2;
pub const DC_EVT_NEW_CHAN_W_1: c_int = 3;
pub const DC_EVT_NEW_DATA_W_0: c_int = 4;
pub const DC_EVT_NEW_DATA_W_1: c_int = 5;
pub const DC_EVT_NEW_ADDR_R_0: c_int = 6;
pub const DC_EVT_NEW_ADDR_R_1: c_int = 7;
pub const DC_EVT_NEW_CHAN_R_0: c_int = 8;
pub const DC_EVT_NEW_CHAN_R_1: c_int = 9;
pub const DC_EVT_NEW_DATA_R_0: c_int = 10;
pub const DC_EVT_NEW_DATA_R_1: c_int = 11;
pub const DC_WR_CH_CONF: c_uint = 0x0;
pub const DC_WR_CH_ADDR: c_uint = 0x4;

pub const DC_GEN: c_uint = 0xd4;

pub const DC_STAT: c_uint = 0x1c8;

pub const WRG: c_uint = 0x01;
pub const WCLK: c_uint = 0xc9;
pub const SYNC_WAVE: c_int = 0;

pub const IPU_DC_NUM_CHANNELS: c_int = 10;
    struct ipu_dc_priv;
    enum ipu_dc_map {
    IPU_DC_MAP_RGB24,
    IPU_DC_MAP_RGB565,
    IPU_DC_MAP_GBR24, /* TVEv2 */
    IPU_DC_MAP_BGR666,
    IPU_DC_MAP_LVDS666,
    IPU_DC_MAP_BGR24,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu_dc {
// The display interface number assigned to this dc channel
    pub di: c_uint,
    pub base: *mut void __iomem,
    pub priv: *mut ipu_dc_priv,
    pub chno: c_int,
    pub in_use: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu_dc_priv {
    pub dc_reg: *mut void __iomem,
    pub dc_tmpl_reg: *mut void __iomem,
    pub ipu: *mut ipu_soc,
    pub dev: *mut device,
    pub channels: [ipu_dc; IPU_DC_NUM_CHANNELS],
    pub mutex: mutex,
    pub comp: completion,
    pub use_count: c_int,
}

#[no_mangle]
unsafe extern "C" fn dc_link_event(dc: *mut ipu_dc, event: c_int, addr: c_int, priority: c_int) {
    static void dc_link_event(struct ipu_dc *dc, int event, int addr, int priority)
    {
    u32 reg;
    reg = readl(dc.base + DC_RL_CH(event));
    reg &= ~(0xffff << (16 * (event & 0x1)));
    reg |= ((addr << 8) | priority) << (16 * (event & 0x1));
    writel(reg, dc.base + DC_RL_CH(event));
    }
    static void dc_write_tmpl(struct ipu_dc *dc, int word, u32 opcode, u32 operand,
    int map, int wave, int glue, int sync, int stop)
    {
    struct ipu_dc_priv *priv = dc.priv;
    u32 reg1, reg2;
    if (opcode == WCLK) {
    reg1 = (operand << 20) & 0xfff00000;
    reg2 = operand >> 12 | opcode << 1 | stop << 9;
    } else if (opcode == WRG) {
    reg1 = sync | glue << 4 | ++wave << 11 | ((operand << 15) & 0xffff8000);
    reg2 = operand >> 17 | opcode << 7 | stop << 9;
    } else {
    reg1 = sync | glue << 4 | ++wave << 11 | ++map << 15 | ((operand << 20) & 0xfff00000);
    reg2 = operand >> 12 | opcode << 4 | stop << 9;
    }
    writel(reg1, priv.dc_tmpl_reg + word * 8);
    writel(reg2, priv.dc_tmpl_reg + word * 8 + 4);
    }
#[no_mangle]
unsafe extern "C" fn ipu_bus_format_to_map(fmt: u32) -> c_int {
    static int ipu_bus_format_to_map(u32 fmt)
    {
    switch (fmt) {
    default:
    WARN_ON(1);
    fallthrough;
    case MEDIA_BUS_FMT_RGB888_1X24:
    return IPU_DC_MAP_RGB24;
    case MEDIA_BUS_FMT_RGB565_1X16:
    return IPU_DC_MAP_RGB565;
    case MEDIA_BUS_FMT_GBR888_1X24:
    return IPU_DC_MAP_GBR24;
    case MEDIA_BUS_FMT_RGB666_1X18:
    return IPU_DC_MAP_BGR666;
    case MEDIA_BUS_FMT_RGB666_1X24_CPADHI:
    return IPU_DC_MAP_LVDS666;
    case MEDIA_BUS_FMT_BGR888_1X24:
    return IPU_DC_MAP_BGR24;
    }
    }
    int ipu_dc_init_sync(struct ipu_dc *dc, struct ipu_di *di, bool interlaced,
    u32 bus_format, u32 width)
    {
    struct ipu_dc_priv *priv = dc.priv;
    int addr, sync;
    let mut reg: u32 = 0;
    int map;
    dc.di = ipu_di_get_num(di);
    if (!IS_ALIGNED(width, 8)) {
    dev_warn(priv.dev,
    "%s: hactive does not align to 8 byte\n", __func__);
    }
    map = ipu_bus_format_to_map(bus_format);
//
// In interlaced mode we need more counters to create the asymmetric
// per-field VSYNC signals. The pixel active signal synchronising DC
// to DI moves to signal generator #6 (see ipu-di.c). In progressive
// mode counter #5 is used.
//
    sync = interlaced ? 6 : 5;
// Reserve 5 microcode template words for each DI
    if (dc.di)
    addr = 5;
    else
    addr = 0;
    if (interlaced) {
    dc_link_event(dc, DC_EVT_NL, addr, 3);
    dc_link_event(dc, DC_EVT_EOL, addr, 2);
    dc_link_event(dc, DC_EVT_NEW_DATA, addr, 1);
// Init template microcode
    dc_write_tmpl(dc, addr, WROD(0), 0, map, SYNC_WAVE, 0, sync, 1);
    } else {
    dc_link_event(dc, DC_EVT_NL, addr + 2, 3);
    dc_link_event(dc, DC_EVT_EOL, addr + 3, 2);
    dc_link_event(dc, DC_EVT_NEW_DATA, addr + 1, 1);
// Init template microcode
    dc_write_tmpl(dc, addr + 2, WROD(0), 0, map, SYNC_WAVE, 8, sync, 1);
    dc_write_tmpl(dc, addr + 3, WROD(0), 0, map, SYNC_WAVE, 4, sync, 0);
    dc_write_tmpl(dc, addr + 4, WRG, 0, map, NULL_WAVE, 0, 0, 1);
    dc_write_tmpl(dc, addr + 1, WROD(0), 0, map, SYNC_WAVE, 0, sync, 1);
    }
    dc_link_event(dc, DC_EVT_NF, 0, 0);
    dc_link_event(dc, DC_EVT_NFIELD, 0, 0);
    dc_link_event(dc, DC_EVT_EOF, 0, 0);
    dc_link_event(dc, DC_EVT_EOFIELD, 0, 0);
    dc_link_event(dc, DC_EVT_NEW_CHAN, 0, 0);
    dc_link_event(dc, DC_EVT_NEW_ADDR, 0, 0);
    reg = readl(dc.base + DC_WR_CH_CONF);
    if (interlaced)
    reg |= DC_WR_CH_CONF_FIELD_MODE;
    else
    reg &= ~DC_WR_CH_CONF_FIELD_MODE;
    writel(reg, dc.base + DC_WR_CH_CONF);
    writel(0x0, dc.base + DC_WR_CH_ADDR);
    writel(width, priv.dc_reg + DC_DISP_CONF2(dc.di));
    return 0;
    }
    EXPORT_SYMBOL_GPL(ipu_dc_init_sync);
#[no_mangle]
pub unsafe extern "C" fn ipu_dc_enable(ipu: *mut ipu_soc) {
    void ipu_dc_enable(struct ipu_soc *ipu)
    {
    struct ipu_dc_priv *priv = ipu.dc_priv;
    mutex_lock(&priv.mutex);
    if (!priv.use_count)
    ipu_module_enable(priv.ipu, IPU_CONF_DC_EN);
    priv.use_count++;
    mutex_unlock(&priv.mutex);
    }
    EXPORT_SYMBOL_GPL(ipu_dc_enable);
#[no_mangle]
pub unsafe extern "C" fn ipu_dc_enable_channel(dc: *mut ipu_dc) {
    void ipu_dc_enable_channel(struct ipu_dc *dc)
    {
    u32 reg;
    reg = readl(dc.base + DC_WR_CH_CONF);
    reg |= DC_WR_CH_CONF_PROG_TYPE_NORMAL;
    writel(reg, dc.base + DC_WR_CH_CONF);
    }
    EXPORT_SYMBOL_GPL(ipu_dc_enable_channel);
#[no_mangle]
pub unsafe extern "C" fn ipu_dc_disable_channel(dc: *mut ipu_dc) {
    void ipu_dc_disable_channel(struct ipu_dc *dc)
    {
    u32 val;
    val = readl(dc.base + DC_WR_CH_CONF);
    val &= ~DC_WR_CH_CONF_PROG_TYPE_MASK;
    writel(val, dc.base + DC_WR_CH_CONF);
    }
    EXPORT_SYMBOL_GPL(ipu_dc_disable_channel);
#[no_mangle]
pub unsafe extern "C" fn ipu_dc_disable(ipu: *mut ipu_soc) {
    void ipu_dc_disable(struct ipu_soc *ipu)
    {
    struct ipu_dc_priv *priv = ipu.dc_priv;
    mutex_lock(&priv.mutex);
    priv.use_count--;
    if (!priv.use_count)
    ipu_module_disable(priv.ipu, IPU_CONF_DC_EN);
    if (priv.use_count < 0)
    priv.use_count = 0;
    mutex_unlock(&priv.mutex);
    }
    EXPORT_SYMBOL_GPL(ipu_dc_disable);
    static void ipu_dc_map_config(struct ipu_dc_priv *priv, enum ipu_dc_map map,
    int byte_num, int offset, int mask)
    {
    let mut ptr: c_int = map * 3 + byte_num;
    u32 reg;
    reg = readl(priv.dc_reg + DC_MAP_CONF_VAL(ptr));
    reg &= ~(0xffff << (16 * (ptr & 0x1)));
    reg |= ((offset << 8) | mask) << (16 * (ptr & 0x1));
    writel(reg, priv.dc_reg + DC_MAP_CONF_VAL(ptr));
    reg = readl(priv.dc_reg + DC_MAP_CONF_PTR(map));
    reg &= ~(0x1f << ((16 * (map & 0x1)) + (5 * byte_num)));
    reg |= ptr << ((16 * (map & 0x1)) + (5 * byte_num));
    writel(reg, priv.dc_reg + DC_MAP_CONF_PTR(map));
    }
#[no_mangle]
unsafe extern "C" fn ipu_dc_map_clear(priv: *mut ipu_dc_priv, map: c_int) {
    static void ipu_dc_map_clear(struct ipu_dc_priv *priv, int map)
    {
    let mut reg: u32 = readl(priv.dc_reg + DC_MAP_CONF_PTR(map));
    writel(reg & ~(0xffff << (16 * (map & 0x1))),
    priv.dc_reg + DC_MAP_CONF_PTR(map));
    }
    struct ipu_dc *ipu_dc_get(struct ipu_soc *ipu, int channel)
    {
    struct ipu_dc_priv *priv = ipu.dc_priv;
    struct ipu_dc *dc;
    if (channel >= IPU_DC_NUM_CHANNELS)
    return ERR_PTR(-ENODEV);
    dc = &priv.channels[channel];
    mutex_lock(&priv.mutex);
    if (dc.in_use) {
    mutex_unlock(&priv.mutex);
    return ERR_PTR(-EBUSY);
    }
    dc.in_use = true;
    mutex_unlock(&priv.mutex);
    return dc;
    }
    EXPORT_SYMBOL_GPL(ipu_dc_get);
#[no_mangle]
pub unsafe extern "C" fn ipu_dc_put(dc: *mut ipu_dc) {
    void ipu_dc_put(struct ipu_dc *dc)
    {
    struct ipu_dc_priv *priv = dc.priv;
    mutex_lock(&priv.mutex);
    dc.in_use = false;
    mutex_unlock(&priv.mutex);
    }
    EXPORT_SYMBOL_GPL(ipu_dc_put);
    int ipu_dc_init(struct ipu_soc *ipu, struct device *dev,
    unsigned long base, unsigned long template_base)
    {
    struct ipu_dc_priv *priv;
    static const int channel_offsets[] = {
    0, 0x1c, 0x38, 0x54, 0x58, 0x5c, 0x78, 0, 0x94, 0xb4
    };
    int i;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    mutex_init(&priv.mutex);
    priv.dev = dev;
    priv.ipu = ipu;
    priv.dc_reg = devm_ioremap(dev, base, PAGE_SIZE);
    priv.dc_tmpl_reg = devm_ioremap(dev, template_base, PAGE_SIZE);
    if (!priv.dc_reg || !priv.dc_tmpl_reg)
    return -ENOMEM;
    for (i = 0; i < IPU_DC_NUM_CHANNELS; i++) {
    priv.channels[i].chno = i;
    priv.channels[i].priv = priv;
    priv.channels[i].base = priv.dc_reg + channel_offsets[i];
    }
    writel(DC_WR_CH_CONF_WORD_SIZE_24 | DC_WR_CH_CONF_DISP_ID_PARALLEL(1) |
    DC_WR_CH_CONF_PROG_DI_ID,
    priv.channels[1].base + DC_WR_CH_CONF);
    writel(DC_WR_CH_CONF_WORD_SIZE_24 | DC_WR_CH_CONF_DISP_ID_PARALLEL(0),
    priv.channels[5].base + DC_WR_CH_CONF);
    writel(DC_GEN_SYNC_1_6_SYNC | DC_GEN_SYNC_PRIORITY_1,
    priv.dc_reg + DC_GEN);
    ipu.dc_priv = priv;
    dev_dbg(dev, "DC base: 0x%08lx template base: 0x%08lx\n",
    base, template_base);
// rgb24
    ipu_dc_map_clear(priv, IPU_DC_MAP_RGB24);
    ipu_dc_map_config(priv, IPU_DC_MAP_RGB24, 0, 7, 0xff); /* blue */
    ipu_dc_map_config(priv, IPU_DC_MAP_RGB24, 1, 15, 0xff); /* green */
    ipu_dc_map_config(priv, IPU_DC_MAP_RGB24, 2, 23, 0xff); /* red */
// rgb565
    ipu_dc_map_clear(priv, IPU_DC_MAP_RGB565);
    ipu_dc_map_config(priv, IPU_DC_MAP_RGB565, 0, 4, 0xf8); /* blue */
    ipu_dc_map_config(priv, IPU_DC_MAP_RGB565, 1, 10, 0xfc); /* green */
    ipu_dc_map_config(priv, IPU_DC_MAP_RGB565, 2, 15, 0xf8); /* red */
// gbr24
    ipu_dc_map_clear(priv, IPU_DC_MAP_GBR24);
    ipu_dc_map_config(priv, IPU_DC_MAP_GBR24, 2, 15, 0xff); /* green */
    ipu_dc_map_config(priv, IPU_DC_MAP_GBR24, 1, 7, 0xff); /* blue */
    ipu_dc_map_config(priv, IPU_DC_MAP_GBR24, 0, 23, 0xff); /* red */
// bgr666
    ipu_dc_map_clear(priv, IPU_DC_MAP_BGR666);
    ipu_dc_map_config(priv, IPU_DC_MAP_BGR666, 0, 5, 0xfc); /* blue */
    ipu_dc_map_config(priv, IPU_DC_MAP_BGR666, 1, 11, 0xfc); /* green */
    ipu_dc_map_config(priv, IPU_DC_MAP_BGR666, 2, 17, 0xfc); /* red */
// lvds666
    ipu_dc_map_clear(priv, IPU_DC_MAP_LVDS666);
    ipu_dc_map_config(priv, IPU_DC_MAP_LVDS666, 0, 5, 0xfc); /* blue */
    ipu_dc_map_config(priv, IPU_DC_MAP_LVDS666, 1, 13, 0xfc); /* green */
    ipu_dc_map_config(priv, IPU_DC_MAP_LVDS666, 2, 21, 0xfc); /* red */
// bgr24
    ipu_dc_map_clear(priv, IPU_DC_MAP_BGR24);
    ipu_dc_map_config(priv, IPU_DC_MAP_BGR24, 2, 7, 0xff); /* red */
    ipu_dc_map_config(priv, IPU_DC_MAP_BGR24, 1, 15, 0xff); /* green */
    ipu_dc_map_config(priv, IPU_DC_MAP_BGR24, 0, 23, 0xff); /* blue */
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ipu_dc_exit(ipu: *mut ipu_soc) {
    void ipu_dc_exit(struct ipu_soc *ipu)
    {
    }
