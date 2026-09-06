//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/bridge/tc358762.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 2020 Marek Vasut <marex@denx.de>
//
// Based on tc358764.c by
// Andrzej Hajda <a.hajda@samsung.com>
// Maciej Purski <m.purski@samsung.com>
//
// Based on rpi_touchscreen.c by
// Eric Anholt <eric@anholt.net>
//

// PPI layer registers
pub const PPI_STARTPPI: c_uint = 0x0104 /* START control bit */;
pub const PPI_LPTXTIMECNT: c_uint = 0x0114 /* LPTX timing signal */;
pub const PPI_D0S_ATMR: c_uint = 0x0144;
pub const PPI_D1S_ATMR: c_uint = 0x0148;
pub const PPI_D0S_CLRSIPOCOUNT: c_uint = 0x0164 /* Assertion timer for Lane 0 */;
pub const PPI_D1S_CLRSIPOCOUNT: c_uint = 0x0168 /* Assertion timer for Lane 1 */;
pub const PPI_START_FUNCTION: c_int = 1;
// DSI layer registers
pub const DSI_STARTDSI: c_uint = 0x0204 /* START control bit of DSI-TX */;
pub const DSI_LANEENABLE: c_uint = 0x0210 /* Enables each lane */;
pub const DSI_RX_START: c_int = 1;
// LCDC/DPI Host Registers, based on guesswork that this matches TC358764
pub const LCDCTRL: c_uint = 0x0420 /* Video Path Control */;

// SPI Master Registers
pub const SPICMR: c_uint = 0x0450;
pub const SPITCR: c_uint = 0x0454;
// System Controller Registers
pub const SYSCTRL: c_uint = 0x0464;
// System registers
pub const LPX_PERIOD: c_int = 3;
// Lane enable PPI and DSI register bits

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc358762 {
    pub dev: *mut device,
    pub bridge: drm_bridge,
    pub regulator: *mut regulator,
    pub panel_bridge: *mut drm_bridge,
    pub reset_gpio: *mut gpio_desc,
    pub mode: drm_display_mode,
    pub pre_enabled: bool,
    pub error: c_int,
}

#[no_mangle]
unsafe extern "C" fn tc358762_clear_error(ctx: *mut tc358762) -> c_int {
    static int tc358762_clear_error(struct tc358762 *ctx)
    {
    let mut ret: c_int = ctx.error;
    ctx.error = 0;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tc358762_write(ctx: *mut tc358762, addr: u16, val: u32) {
    static void tc358762_write(struct tc358762 *ctx, u16 addr, u32 val)
    {
    struct mipi_dsi_device *dsi = to_mipi_dsi_device(ctx.dev);
    ssize_t ret;
    u8 data[6];
    if (ctx.error)
    return;
    data[0] = addr;
    data[1] = addr >> 8;
    data[2] = val;
    data[3] = val >> 8;
    data[4] = val >> 16;
    data[5] = val >> 24;
    ret = mipi_dsi_generic_write(dsi, data, sizeof(data));
    if (ret < 0)
    ctx.error = ret;
    }
    static inline struct tc358762 *bridge_to_tc358762(struct drm_bridge *bridge)
    {
    return container_of(bridge, struct tc358762, bridge);
    }
#[no_mangle]
unsafe extern "C" fn tc358762_init(ctx: *mut tc358762) -> c_int {
    static int tc358762_init(struct tc358762 *ctx)
    {
    u32 lcdctrl;
    tc358762_write(ctx, DSI_LANEENABLE,
    LANEENABLE_L0EN | LANEENABLE_CLEN);
    tc358762_write(ctx, PPI_D0S_CLRSIPOCOUNT, 5);
    tc358762_write(ctx, PPI_D1S_CLRSIPOCOUNT, 5);
    tc358762_write(ctx, PPI_D0S_ATMR, 0);
    tc358762_write(ctx, PPI_D1S_ATMR, 0);
    tc358762_write(ctx, PPI_LPTXTIMECNT, LPX_PERIOD);
    tc358762_write(ctx, SPICMR, 0x00);
    lcdctrl = LCDCTRL_VSDELAY(1) | LCDCTRL_RGB888 |
    LCDCTRL_UNK6 | LCDCTRL_VTGEN;
    if (ctx.mode.flags & DRM_MODE_FLAG_NHSYNC)
    lcdctrl |= LCDCTRL_HSPOL;
    if (ctx.mode.flags & DRM_MODE_FLAG_NVSYNC)
    lcdctrl |= LCDCTRL_VSPOL;
    tc358762_write(ctx, LCDCTRL, lcdctrl);
    tc358762_write(ctx, SYSCTRL, 0x040f);
    msleep(100);
    tc358762_write(ctx, PPI_STARTPPI, PPI_START_FUNCTION);
    tc358762_write(ctx, DSI_STARTDSI, DSI_RX_START);
    msleep(100);
    return tc358762_clear_error(ctx);
    }
    static void tc358762_post_disable(struct drm_bridge *bridge,
    struct drm_atomic_commit *state)
    {
    struct tc358762 *ctx = bridge_to_tc358762(bridge);
    int ret;
//
// The post_disable hook might be called multiple times.
// We want to avoid regulator imbalance below.
//
    if (!ctx.pre_enabled)
    return;
    ctx.pre_enabled = false;
    if (ctx.reset_gpio)
    gpiod_set_value_cansleep(ctx.reset_gpio, 0);
    ret = regulator_disable(ctx.regulator);
    if (ret < 0)
    dev_err(ctx.dev, "error disabling regulators (%d)\n", ret);
    }
    static void tc358762_pre_enable(struct drm_bridge *bridge,
    struct drm_atomic_commit *state)
    {
    struct tc358762 *ctx = bridge_to_tc358762(bridge);
    int ret;
    ret = regulator_enable(ctx.regulator);
    if (ret < 0)
    dev_err(ctx.dev, "error enabling regulators (%d)\n", ret);
    if (ctx.reset_gpio) {
    gpiod_set_value_cansleep(ctx.reset_gpio, 1);
    usleep_range(5000, 10000);
    }
    ctx.pre_enabled = true;
    }
    static void tc358762_enable(struct drm_bridge *bridge,
    struct drm_atomic_commit *state)
    {
    struct tc358762 *ctx = bridge_to_tc358762(bridge);
    int ret;
    ret = tc358762_init(ctx);
    if (ret < 0)
    dev_err(ctx.dev, "error initializing bridge (%d)\n", ret);
    }
    static int tc358762_attach(struct drm_bridge *bridge,
    struct drm_encoder *encoder,
    enum drm_bridge_attach_flags flags)
    {
    struct tc358762 *ctx = bridge_to_tc358762(bridge);
    return drm_bridge_attach(encoder, ctx.panel_bridge,
    bridge, flags);
    }
    static void tc358762_bridge_mode_set(struct drm_bridge *bridge,
    const struct drm_display_mode *mode,
    const struct drm_display_mode *adj)
    {
    struct tc358762 *ctx = bridge_to_tc358762(bridge);
    drm_mode_copy(&ctx.mode, mode);
    }
    static const struct drm_bridge_funcs tc358762_bridge_funcs = {
    .atomic_post_disable = tc358762_post_disable,
    .atomic_pre_enable = tc358762_pre_enable,
    .atomic_enable = tc358762_enable,
    .atomic_duplicate_state = drm_atomic_helper_bridge_duplicate_state,
    .atomic_destroy_state = drm_atomic_helper_bridge_destroy_state,
    .atomic_create_state = drm_atomic_helper_bridge_create_state,
    .attach = tc358762_attach,
    .mode_set = tc358762_bridge_mode_set,
    };
#[no_mangle]
unsafe extern "C" fn tc358762_parse_dt(ctx: *mut tc358762) -> c_int {
    static int tc358762_parse_dt(struct tc358762 *ctx)
    {
    struct drm_bridge *panel_bridge;
    struct device *dev = ctx.dev;
    panel_bridge = devm_drm_of_get_bridge(dev, dev.of_node, 1, 0);
    if (IS_ERR(panel_bridge))
    return PTR_ERR(panel_bridge);
    ctx.panel_bridge = panel_bridge;
// Reset GPIO is optional
    ctx.reset_gpio = devm_gpiod_get_optional(dev, "reset", GPIOD_OUT_LOW);
    if (IS_ERR(ctx.reset_gpio))
    return PTR_ERR(ctx.reset_gpio);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tc358762_configure_regulators(ctx: *mut tc358762) -> c_int {
    static int tc358762_configure_regulators(struct tc358762 *ctx)
    {
    ctx.regulator = devm_regulator_get(ctx.dev, "vddc");
    if (IS_ERR(ctx.regulator))
    return PTR_ERR(ctx.regulator);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tc358762_probe(dsi: *mut mipi_dsi_device) -> c_int {
    static int tc358762_probe(struct mipi_dsi_device *dsi)
    {
    struct device *dev = &dsi.dev;
    struct tc358762 *ctx;
    int ret;
    ctx = devm_drm_bridge_alloc(dev, struct tc358762, bridge,
    &tc358762_bridge_funcs);
    if (IS_ERR(ctx))
    return PTR_ERR(ctx);
    mipi_dsi_set_drvdata(dsi, ctx);
    ctx.dev = dev;
    ctx.pre_enabled = false;
// TODO: Find out how to get dual-lane mode working
    dsi.lanes = 1;
    dsi.format = MIPI_DSI_FMT_RGB888;
    dsi.mode_flags = MIPI_DSI_MODE_VIDEO | MIPI_DSI_MODE_VIDEO_SYNC_PULSE |
    MIPI_DSI_MODE_LPM | MIPI_DSI_MODE_VIDEO_HSE;
    ret = tc358762_parse_dt(ctx);
    if (ret < 0)
    return ret;
    ret = tc358762_configure_regulators(ctx);
    if (ret < 0)
    return ret;
    ctx.bridge.type = DRM_MODE_CONNECTOR_DPI;
    ctx.bridge.of_node = dev.of_node;
    ctx.bridge.pre_enable_prev_first = true;
    ret = devm_drm_bridge_add(dev, &ctx.bridge);
    if (ret < 0)
    return ret;
    ret = devm_mipi_dsi_attach(dev, dsi);
    if (ret < 0)
    dev_err(dev, "failed to attach dsi\n");
    return ret;
    }
    static const struct of_device_id tc358762_of_match[] = {
    { .compatible = "toshiba,tc358762" },
    { }
    };
    MODULE_DEVICE_TABLE(of, tc358762_of_match);
    static struct mipi_dsi_driver tc358762_driver = {
    .probe = tc358762_probe,
    .driver = {
    .name = "tc358762",
    .of_match_table = tc358762_of_match,
    },
    };
    module_mipi_dsi_driver(tc358762_driver);
    MODULE_AUTHOR("Marek Vasut <marex@denx.de>");
    MODULE_DESCRIPTION("MIPI-DSI based Driver for TC358762 DSI/DPI Bridge");
    MODULE_LICENSE("GPL v2");
