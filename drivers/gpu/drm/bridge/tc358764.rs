//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/bridge/tc358764.c
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
// Copyright (C) 2018 Samsung Electronics Co., Ltd
//
// Authors:
// Andrzej Hajda <a.hajda@samsung.com>
// Maciej Purski <m.purski@samsung.com>
//

// PPI layer registers
pub const PPI_STARTPPI: c_uint = 0x0104 /* START control bit */;
pub const PPI_LPTXTIMECNT: c_uint = 0x0114 /* LPTX timing signal */;
pub const PPI_LANEENABLE: c_uint = 0x0134 /* Enables each lane */;
pub const PPI_TX_RX_TA: c_uint = 0x013C /* BTA timing parameters */;
pub const PPI_D0S_CLRSIPOCOUNT: c_uint = 0x0164 /* Assertion timer for Lane 0 */;
pub const PPI_D1S_CLRSIPOCOUNT: c_uint = 0x0168 /* Assertion timer for Lane 1 */;
pub const PPI_D2S_CLRSIPOCOUNT: c_uint = 0x016C /* Assertion timer for Lane 2 */;
pub const PPI_D3S_CLRSIPOCOUNT: c_uint = 0x0170 /* Assertion timer for Lane 3 */;
pub const PPI_START_FUNCTION: c_int = 1;
// DSI layer registers
pub const DSI_STARTDSI: c_uint = 0x0204 /* START control bit of DSI-TX */;
pub const DSI_LANEENABLE: c_uint = 0x0210 /* Enables each lane */;
pub const DSI_RX_START: c_int = 1;
// Video path registers
pub const VP_CTRL: c_uint = 0x0450 /* Video Path Control */;

pub const VP_HTIM1: c_uint = 0x0454 /* Horizontal Timing Control 1 */;

pub const VP_HTIM2: c_uint = 0x0458 /* Horizontal Timing Control 2 */;

pub const VP_VTIM1: c_uint = 0x045C /* Vertical Timing Control 1 */;

pub const VP_VTIM2: c_uint = 0x0460 /* Vertical Timing Control 2 */;

pub const VP_VFUEN: c_uint = 0x0464 /* Video Frame Timing Update Enable */;
// LVDS registers
pub const LV_MX0003: c_uint = 0x0480 /* Mux input bit 0 to 3 */;
pub const LV_MX0407: c_uint = 0x0484 /* Mux input bit 4 to 7 */;
pub const LV_MX0811: c_uint = 0x0488 /* Mux input bit 8 to 11 */;
pub const LV_MX1215: c_uint = 0x048C /* Mux input bit 12 to 15 */;
pub const LV_MX1619: c_uint = 0x0490 /* Mux input bit 16 to 19 */;
pub const LV_MX2023: c_uint = 0x0494 /* Mux input bit 20 to 23 */;
pub const LV_MX2427: c_uint = 0x0498 /* Mux input bit 24 to 27 */;

    FLD_VAL(b2, 20, 16) | FLD_VAL(b3, 28, 24))
// Input bit numbers used in mux registers
    enum {
    LVI_R0,
    LVI_R1,
    LVI_R2,
    LVI_R3,
    LVI_R4,
    LVI_R5,
    LVI_R6,
    LVI_R7,
    LVI_G0,
    LVI_G1,
    LVI_G2,
    LVI_G3,
    LVI_G4,
    LVI_G5,
    LVI_G6,
    LVI_G7,
    LVI_B0,
    LVI_B1,
    LVI_B2,
    LVI_B3,
    LVI_B4,
    LVI_B5,
    LVI_B6,
    LVI_B7,
    LVI_HS,
    LVI_VS,
    LVI_DE,
    LVI_L0
    };
pub const LV_CFG: c_uint = 0x049C /* LVDS Configuration */;
pub const LV_PHY0: c_uint = 0x04A0 /* LVDS PHY 0 */;

// System registers
pub const SYS_RST: c_uint = 0x0504 /* System Reset */;
pub const SYS_ID: c_uint = 0x0580 /* System ID */;

pub const LPX_PERIOD: c_int = 2;
pub const TTA_SURE: c_int = 3;
pub const TTA_GET: c_uint = 0x20000;
// Lane enable PPI and DSI register bits

// LVCFG fields

    static const char * const tc358764_supplies[] = {
    "vddc", "vddio", "vddlvds"
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc358764 {
    pub dev: *mut device,
    pub bridge: drm_bridge,
    pub next_bridge: *mut drm_bridge,
    pub supplies: [regulator_bulk_data; ARRAY_SIZE(tc358764_supplies)],
    pub gpio_reset: *mut gpio_desc,
    pub error: c_int,
}

#[no_mangle]
unsafe extern "C" fn tc358764_clear_error(ctx: *mut tc358764) -> c_int {
    static int tc358764_clear_error(struct tc358764 *ctx)
    {
    let mut ret: c_int = ctx.error;
    ctx.error = 0;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tc358764_read(ctx: *mut tc358764, addr: u16, val: *mut u32) {
    static void tc358764_read(struct tc358764 *ctx, u16 addr, u32 *val)
    {
    struct mipi_dsi_device *dsi = to_mipi_dsi_device(ctx.dev);
    ssize_t ret;
    if (ctx.error)
    return;
    cpu_to_le16s(&addr);
    ret = mipi_dsi_generic_read(dsi, &addr, sizeof(addr), val, sizeof(*val));
    if (ret >= 0)
    le32_to_cpus(val);
    dev_dbg(ctx.dev, "read: addr=0x%04x data=0x%08x\n", addr, *val);
    }
#[no_mangle]
unsafe extern "C" fn tc358764_write(ctx: *mut tc358764, addr: u16, val: u32) {
    static void tc358764_write(struct tc358764 *ctx, u16 addr, u32 val)
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
    static inline struct tc358764 *bridge_to_tc358764(struct drm_bridge *bridge)
    {
    return container_of(bridge, struct tc358764, bridge);
    }
#[no_mangle]
unsafe extern "C" fn tc358764_init(ctx: *mut tc358764) -> c_int {
    static int tc358764_init(struct tc358764 *ctx)
    {
    let mut v: u32 = 0;
    tc358764_read(ctx, SYS_ID, &v);
    if (ctx.error)
    return tc358764_clear_error(ctx);
    dev_info(ctx.dev, "ID: %#x\n", v);
// configure PPI counters
    tc358764_write(ctx, PPI_TX_RX_TA, TTA_GET | TTA_SURE);
    tc358764_write(ctx, PPI_LPTXTIMECNT, LPX_PERIOD);
    tc358764_write(ctx, PPI_D0S_CLRSIPOCOUNT, 5);
    tc358764_write(ctx, PPI_D1S_CLRSIPOCOUNT, 5);
    tc358764_write(ctx, PPI_D2S_CLRSIPOCOUNT, 5);
    tc358764_write(ctx, PPI_D3S_CLRSIPOCOUNT, 5);
// enable four data lanes and clock lane
    tc358764_write(ctx, PPI_LANEENABLE, LANEENABLE_L3EN | LANEENABLE_L2EN |
    LANEENABLE_L1EN | LANEENABLE_L0EN | LANEENABLE_CLEN);
    tc358764_write(ctx, DSI_LANEENABLE, LANEENABLE_L3EN | LANEENABLE_L2EN |
    LANEENABLE_L1EN | LANEENABLE_L0EN | LANEENABLE_CLEN);
// start
    tc358764_write(ctx, PPI_STARTPPI, PPI_START_FUNCTION);
    tc358764_write(ctx, DSI_STARTDSI, DSI_RX_START);
// configure video path
    tc358764_write(ctx, VP_CTRL, VP_CTRL_VSDELAY(15) | VP_CTRL_RGB888 |
    VP_CTRL_EVTMODE | VP_CTRL_HSPOL | VP_CTRL_VSPOL);
// reset PHY
    tc358764_write(ctx, LV_PHY0, LV_PHY0_RST(1) |
    LV_PHY0_PRBS_ON(4) | LV_PHY0_IS(2) | LV_PHY0_ND(6));
    tc358764_write(ctx, LV_PHY0, LV_PHY0_PRBS_ON(4) | LV_PHY0_IS(2) |
    LV_PHY0_ND(6));
// reset bridge
    tc358764_write(ctx, SYS_RST, SYS_RST_LCD);
// set bit order
    tc358764_write(ctx, LV_MX0003, LV_MX(LVI_R0, LVI_R1, LVI_R2, LVI_R3));
    tc358764_write(ctx, LV_MX0407, LV_MX(LVI_R4, LVI_R7, LVI_R5, LVI_G0));
    tc358764_write(ctx, LV_MX0811, LV_MX(LVI_G1, LVI_G2, LVI_G6, LVI_G7));
    tc358764_write(ctx, LV_MX1215, LV_MX(LVI_G3, LVI_G4, LVI_G5, LVI_B0));
    tc358764_write(ctx, LV_MX1619, LV_MX(LVI_B6, LVI_B7, LVI_B1, LVI_B2));
    tc358764_write(ctx, LV_MX2023, LV_MX(LVI_B3, LVI_B4, LVI_B5, LVI_L0));
    tc358764_write(ctx, LV_MX2427, LV_MX(LVI_HS, LVI_VS, LVI_DE, LVI_R6));
    tc358764_write(ctx, LV_CFG, LV_CFG_CLKPOL2 | LV_CFG_CLKPOL1 |
    LV_CFG_LVEN);
    return tc358764_clear_error(ctx);
    }
#[no_mangle]
unsafe extern "C" fn tc358764_reset(ctx: *mut tc358764) {
    static void tc358764_reset(struct tc358764 *ctx)
    {
    gpiod_set_value(ctx.gpio_reset, 1);
    usleep_range(1000, 2000);
    gpiod_set_value(ctx.gpio_reset, 0);
    usleep_range(1000, 2000);
    }
    static void tc358764_post_disable(struct drm_bridge *bridge,
    struct drm_atomic_commit *commit)
    {
    struct tc358764 *ctx = bridge_to_tc358764(bridge);
    int ret;
    tc358764_reset(ctx);
    usleep_range(10000, 15000);
    ret = regulator_bulk_disable(ARRAY_SIZE(ctx.supplies), ctx.supplies);
    if (ret < 0)
    dev_err(ctx.dev, "error disabling regulators (%d)\n", ret);
    }
    static void tc358764_pre_enable(struct drm_bridge *bridge,
    struct drm_atomic_commit *commit)
    {
    struct tc358764 *ctx = bridge_to_tc358764(bridge);
    int ret;
    ret = regulator_bulk_enable(ARRAY_SIZE(ctx.supplies), ctx.supplies);
    if (ret < 0)
    dev_err(ctx.dev, "error enabling regulators (%d)\n", ret);
    usleep_range(10000, 15000);
    tc358764_reset(ctx);
    ret = tc358764_init(ctx);
    if (ret < 0)
    dev_err(ctx.dev, "error initializing bridge (%d)\n", ret);
    }
    static int tc358764_attach(struct drm_bridge *bridge,
    struct drm_encoder *encoder,
    enum drm_bridge_attach_flags flags)
    {
    struct tc358764 *ctx = bridge_to_tc358764(bridge);
    return drm_bridge_attach(encoder, ctx.next_bridge, bridge, flags);
    }
    static const struct drm_bridge_funcs tc358764_bridge_funcs = {
    .atomic_create_state = drm_atomic_helper_bridge_create_state,
    .atomic_destroy_state = drm_atomic_helper_bridge_destroy_state,
    .atomic_duplicate_state = drm_atomic_helper_bridge_duplicate_state,
    .atomic_post_disable = tc358764_post_disable,
    .atomic_pre_enable = tc358764_pre_enable,
    .attach = tc358764_attach,
    };
#[no_mangle]
unsafe extern "C" fn tc358764_parse_dt(ctx: *mut tc358764) -> c_int {
    static int tc358764_parse_dt(struct tc358764 *ctx)
    {
    struct device *dev = ctx.dev;
    ctx.gpio_reset = devm_gpiod_get(dev, "reset", GPIOD_OUT_LOW);
    if (IS_ERR(ctx.gpio_reset)) {
    dev_err(dev, "no reset GPIO pin provided\n");
    return PTR_ERR(ctx.gpio_reset);
    }
    ctx.next_bridge = devm_drm_of_get_bridge(dev, dev.of_node, 1, 0);
    if (IS_ERR(ctx.next_bridge))
    return PTR_ERR(ctx.next_bridge);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tc358764_configure_regulators(ctx: *mut tc358764) -> c_int {
    static int tc358764_configure_regulators(struct tc358764 *ctx)
    {
    int i, ret;
    for (i = 0; i < ARRAY_SIZE(ctx.supplies); ++i)
    ctx.supplies[i].supply = tc358764_supplies[i];
    ret = devm_regulator_bulk_get(ctx.dev, ARRAY_SIZE(ctx.supplies),
    ctx.supplies);
    if (ret < 0)
    dev_err(ctx.dev, "failed to get regulators: %d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tc358764_probe(dsi: *mut mipi_dsi_device) -> c_int {
    static int tc358764_probe(struct mipi_dsi_device *dsi)
    {
    struct device *dev = &dsi.dev;
    struct tc358764 *ctx;
    int ret;
    ctx = devm_drm_bridge_alloc(dev, struct tc358764, bridge,
    &tc358764_bridge_funcs);
    if (IS_ERR(ctx))
    return PTR_ERR(ctx);
    mipi_dsi_set_drvdata(dsi, ctx);
    ctx.dev = dev;
    dsi.lanes = 4;
    dsi.format = MIPI_DSI_FMT_RGB888;
    dsi.mode_flags = MIPI_DSI_MODE_VIDEO | MIPI_DSI_MODE_VIDEO_BURST
    | MIPI_DSI_MODE_VIDEO_AUTO_VERT | MIPI_DSI_MODE_LPM;
    ret = tc358764_parse_dt(ctx);
    if (ret < 0)
    return ret;
    ret = tc358764_configure_regulators(ctx);
    if (ret < 0)
    return ret;
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
    static const struct of_device_id tc358764_of_match[] = {
    { .compatible = "toshiba,tc358764" },
    { }
    };
    MODULE_DEVICE_TABLE(of, tc358764_of_match);
    static struct mipi_dsi_driver tc358764_driver = {
    .probe = tc358764_probe,
    .driver = {
    .name = "tc358764",
    .of_match_table = tc358764_of_match,
    },
    };
    module_mipi_dsi_driver(tc358764_driver);
    MODULE_AUTHOR("Andrzej Hajda <a.hajda@samsung.com>");
    MODULE_AUTHOR("Maciej Purski <m.purski@samsung.com>");
    MODULE_DESCRIPTION("MIPI-DSI based Driver for TC358764 DSI/LVDS Bridge");
    MODULE_LICENSE("GPL v2");
