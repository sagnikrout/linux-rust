//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/bridge/chipone-icn6211.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2020 Amarula Solutions(India)
// Author: Jagan Teki <jagan@amarulasolutions.com>
//

pub const VENDOR_ID: c_uint = 0x00;
pub const DEVICE_ID_H: c_uint = 0x01;
pub const DEVICE_ID_L: c_uint = 0x02;
pub const VERSION_ID: c_uint = 0x03;
pub const FIRMWARE_VERSION: c_uint = 0x08;
pub const CONFIG_FINISH: c_uint = 0x09;

pub const CLK_PHASE_0: c_int = 0;
pub const CLK_PHASE_1_4: c_int = 1;
pub const CLK_PHASE_1_2: c_int = 2;
pub const CLK_PHASE_3_4: c_int = 3;

pub const RGB_TEST_CTRL: c_uint = 0x1e;
pub const ATE_PLL_EN: c_uint = 0x1f;
pub const HACTIVE_LI: c_uint = 0x20;
pub const VACTIVE_LI: c_uint = 0x21;
pub const VACTIVE_HACTIVE_HI: c_uint = 0x22;
pub const HFP_LI: c_uint = 0x23;
pub const HSYNC_LI: c_uint = 0x24;
pub const HBP_LI: c_uint = 0x25;
pub const HFP_HSW_HBP_HI: c_uint = 0x26;

pub const VFP: c_uint = 0x27;
pub const VSYNC: c_uint = 0x28;
pub const VBP: c_uint = 0x29;
pub const BIST_POL: c_uint = 0x2a;

pub const BIST_RED: c_uint = 0x2b;
pub const BIST_GREEN: c_uint = 0x2c;
pub const BIST_BLUE: c_uint = 0x2d;
pub const BIST_CHESS_X: c_uint = 0x2e;
pub const BIST_CHESS_Y: c_uint = 0x2f;
pub const BIST_CHESS_XY_H: c_uint = 0x30;
pub const BIST_FRAME_TIME_L: c_uint = 0x31;
pub const BIST_FRAME_TIME_H: c_uint = 0x32;
pub const FIFO_MAX_ADDR_LOW: c_uint = 0x33;
pub const SYNC_EVENT_DLY: c_uint = 0x34;
pub const HSW_MIN: c_uint = 0x35;
pub const HFP_MIN: c_uint = 0x36;
pub const LOGIC_RST_NUM: c_uint = 0x37;

pub const BG_CTRL: c_uint = 0x4e;
pub const LDO_PLL: c_uint = 0x4f;

pub const PLL_CTRL_6_EXTERNAL: c_uint = 0x90;
pub const PLL_CTRL_6_MIPI_CLK: c_uint = 0x92;
pub const PLL_CTRL_6_INTERNAL: c_uint = 0x93;

pub const PLL_REF_DIV: c_uint = 0x6b;

pub const GPIO_OEN: c_uint = 0x79;
pub const MIPI_CFG_PW: c_uint = 0x7a;
pub const MIPI_CFG_PW_CONFIG_DSI: c_uint = 0xc1;
pub const MIPI_CFG_PW_CONFIG_I2C: c_uint = 0x3e;

pub const IRQ_SEL: c_uint = 0x7d;
pub const DBG_SEL: c_uint = 0x7e;
pub const DBG_SIGNAL: c_uint = 0x7f;
pub const MIPI_ERR_VECTOR_L: c_uint = 0x80;
pub const MIPI_ERR_VECTOR_H: c_uint = 0x81;
pub const MIPI_ERR_VECTOR_EN_L: c_uint = 0x82;
pub const MIPI_ERR_VECTOR_EN_H: c_uint = 0x83;
pub const MIPI_MAX_SIZE_L: c_uint = 0x84;
pub const MIPI_MAX_SIZE_H: c_uint = 0x85;
pub const DSI_CTRL: c_uint = 0x86;
pub const DSI_CTRL_UNKNOWN: c_uint = 0x28;

pub const MIPI_PN_SWAP: c_uint = 0x87;

pub const MIPI_ULPS_CTRL: c_uint = 0x8a;
pub const MIPI_CLK_CHK_VAR: c_uint = 0x8e;
pub const MIPI_CLK_CHK_INI: c_uint = 0x8f;
pub const MIPI_T_TERM_EN: c_uint = 0x90;
pub const MIPI_T_HS_SETTLE: c_uint = 0x91;
pub const MIPI_T_TA_SURE_PRE: c_uint = 0x92;
pub const MIPI_T_LPX_SET: c_uint = 0x94;
pub const MIPI_T_CLK_MISS: c_uint = 0x95;
pub const MIPI_INIT_TIME_L: c_uint = 0x96;
pub const MIPI_INIT_TIME_H: c_uint = 0x97;
pub const MIPI_T_CLK_TERM_EN: c_uint = 0x99;
pub const MIPI_T_CLK_SETTLE: c_uint = 0x9a;
pub const MIPI_TO_HS_RX_L: c_uint = 0x9e;
pub const MIPI_TO_HS_RX_H: c_uint = 0x9f;

pub const MIPI_PD_RX: c_uint = 0xb0;
pub const MIPI_PD_TERM: c_uint = 0xb1;
pub const MIPI_PD_HSRX: c_uint = 0xb2;
pub const MIPI_PD_LPTX: c_uint = 0xb3;
pub const MIPI_PD_LPRX: c_uint = 0xb4;
pub const MIPI_PD_CK_LANE: c_uint = 0xb5;
pub const MIPI_FORCE_0: c_uint = 0xb6;
pub const MIPI_RST_CTRL: c_uint = 0xb7;
pub const MIPI_RST_NUM: c_uint = 0xb8;

pub const MIPI_DBG_SEL: c_uint = 0xe0;
pub const MIPI_DBG_DATA: c_uint = 0xe1;
pub const MIPI_ATE_TEST_SEL: c_uint = 0xe2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chipone {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub client: *mut i2c_client,
    pub bridge: drm_bridge,
    pub mode: drm_display_mode,
    pub panel_bridge: *mut drm_bridge,
    pub dsi: *mut mipi_dsi_device,
    pub enable_gpio: *mut gpio_desc,
    pub vdd1: *mut regulator,
    pub vdd2: *mut regulator,
    pub vdd3: *mut regulator,
    pub refclk: *mut clk,
    pub refclk_rate: c_ulong,
    pub interface_i2c: bool,
}

    static const struct regmap_range chipone_dsi_readable_ranges[] = {
    regmap_reg_range(VENDOR_ID, VERSION_ID),
    regmap_reg_range(FIRMWARE_VERSION, PLL_SSC_OFFSET(3)),
    regmap_reg_range(GPIO_OEN, MIPI_ULPS_CTRL),
    regmap_reg_range(MIPI_CLK_CHK_VAR, MIPI_T_TA_SURE_PRE),
    regmap_reg_range(MIPI_T_LPX_SET, MIPI_INIT_TIME_H),
    regmap_reg_range(MIPI_T_CLK_TERM_EN, MIPI_T_CLK_SETTLE),
    regmap_reg_range(MIPI_TO_HS_RX_L, MIPI_PHY(5)),
    regmap_reg_range(MIPI_PD_RX, MIPI_RST_NUM),
    regmap_reg_range(MIPI_DBG_SET(0), MIPI_DBG_SET(9)),
    regmap_reg_range(MIPI_DBG_SEL, MIPI_ATE_STATUS(1)),
    };
    static const struct regmap_access_table chipone_dsi_readable_table = {
    .yes_ranges = chipone_dsi_readable_ranges,
    .n_yes_ranges = ARRAY_SIZE(chipone_dsi_readable_ranges),
    };
    static const struct regmap_range chipone_dsi_writeable_ranges[] = {
    regmap_reg_range(CONFIG_FINISH, PLL_SSC_OFFSET(3)),
    regmap_reg_range(GPIO_OEN, MIPI_ULPS_CTRL),
    regmap_reg_range(MIPI_CLK_CHK_VAR, MIPI_T_TA_SURE_PRE),
    regmap_reg_range(MIPI_T_LPX_SET, MIPI_INIT_TIME_H),
    regmap_reg_range(MIPI_T_CLK_TERM_EN, MIPI_T_CLK_SETTLE),
    regmap_reg_range(MIPI_TO_HS_RX_L, MIPI_PHY(5)),
    regmap_reg_range(MIPI_PD_RX, MIPI_RST_NUM),
    regmap_reg_range(MIPI_DBG_SET(0), MIPI_DBG_SET(9)),
    regmap_reg_range(MIPI_DBG_SEL, MIPI_ATE_STATUS(1)),
    };
    static const struct regmap_access_table chipone_dsi_writeable_table = {
    .yes_ranges = chipone_dsi_writeable_ranges,
    .n_yes_ranges = ARRAY_SIZE(chipone_dsi_writeable_ranges),
    };
    static const struct regmap_config chipone_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .rd_table = &chipone_dsi_readable_table,
    .wr_table = &chipone_dsi_writeable_table,
    .cache_type = REGCACHE_MAPLE,
    .max_register = MIPI_ATE_STATUS(1),
    };
    static int chipone_dsi_read(void *context,
    const void *reg, size_t reg_size,
    void *val, size_t val_size)
    {
    struct mipi_dsi_device *dsi = context;
    let mut reg16: u16 = (val_size << 8) | *(u8 *)reg;
    int ret;
    ret = mipi_dsi_generic_read(dsi, &reg16, 2, val, val_size);
    let mut ret: return = = val_size ? 0 : -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn chipone_dsi_write(context: *mut c_void, data: *const c_void, count: usize) -> c_int {
    static int chipone_dsi_write(void *context, const void *data, size_t count)
    {
    struct mipi_dsi_device *dsi = context;
    return mipi_dsi_generic_write(dsi, data, 2);
    }
    static const struct regmap_bus chipone_dsi_regmap_bus = {
    .read				= chipone_dsi_read,
    .write				= chipone_dsi_write,
    .reg_format_endian_default	= REGMAP_ENDIAN_NATIVE,
    .val_format_endian_default	= REGMAP_ENDIAN_NATIVE,
    };
    static inline struct chipone *bridge_to_chipone(struct drm_bridge *bridge)
    {
    return container_of(bridge, struct chipone, bridge);
    }
#[no_mangle]
unsafe extern "C" fn chipone_readb(icn: *mut chipone, reg: u8, val: *mut u8) {
    static void chipone_readb(struct chipone *icn, u8 reg, u8 *val)
    {
    int ret, pval;
    ret = regmap_read(icn.regmap, reg, &pval);
// val = ret ? 0 : pval & 0xff;
    }
#[no_mangle]
unsafe extern "C" fn chipone_writeb(icn: *mut chipone, reg: u8, val: u8) -> c_int {
    static int chipone_writeb(struct chipone *icn, u8 reg, u8 val)
    {
    return regmap_write(icn.regmap, reg, val);
    }
    static void chipone_configure_pll(struct chipone *icn,
    const struct drm_display_mode *mode)
    {
    let mut best_p: c_uint = 0, best_m = 0, best_s = 0;
    let mut mode_clock: c_uint = mode.clock * 1000;
    unsigned int delta, min_delta = 0xffffffff;
    unsigned int freq_p, freq_s, freq_out;
    unsigned int p_min, p_max;
    unsigned int p, m, s;
    unsigned int fin;
    bool best_p_pot;
    u8 ref_div;
//
// DSI byte clock frequency (input into PLL) is calculated as:
// DSI_CLK = HS clock / 4
//
// DPI pixel clock frequency (output from PLL) is mode clock.
//
// The chip contains fractional PLL which works as follows:
// DPI_CLK = ((DSI_CLK / P) * M) / S
// P is pre-divider, register PLL_REF_DIV[3:0] is 1:n divider
// register PLL_REF_DIV[4] is extra 1:2 divider
// M is integer multiplier, register PLL_INT(0) is multiplier
// S is post-divider, register PLL_REF_DIV[7:5] is 2^(n+1) divider
//
// It seems the PLL input clock after applying P pre-divider have
// to be lower than 20 MHz.
//
    if (icn.refclk)
    fin = icn.refclk_rate;
    else
    fin = icn.dsi.hs_rate / 4; /* in Hz */
// Minimum value of P predivider for PLL input in 5..20 MHz
    p_min = clamp(DIV_ROUND_UP(fin, 20000000), 1U, 31U);
    p_max = clamp(fin / 5000000, 1U, 31U);
    for (p = p_min; p < p_max; p++) {	/* PLL_REF_DIV[4,3:0] */
    if (p > 16 && p & 1)		/* P > 16 uses extra /2 */
    continue;
    freq_p = fin / p;
    if (freq_p == 0)		/* Divider too high */
    break;
    for (s = 0; s < 0x7; s++) {	/* PLL_REF_DIV[7:5] */
    freq_s = freq_p / BIT(s + 1);
    if (freq_s == 0)	/* Divider too high */
    break;
    m = mode_clock / freq_s;
// Multiplier is 8 bit
    if (m > 0xff)
    continue;
// Limit PLL VCO frequency to 1 GHz
    freq_out = (fin * m) / p;
    if (freq_out > 1000000000)
    continue;
// Apply post-divider
    freq_out /= BIT(s + 1);
    delta = abs(mode_clock - freq_out);
    if (delta < min_delta) {
    best_p = p;
    best_m = m;
    best_s = s;
    min_delta = delta;
    }
    }
    }
    best_p_pot = !(best_p & 1);
    dev_dbg(icn.dev,
    "PLL: P[3:0]=%d P[4]=2*%d M=%d S[7:5]=2^%d delta=%d => DSI f_in(%s)=%d Hz ; DPI f_out=%d Hz\n",
    best_p >> best_p_pot, best_p_pot, best_m, best_s + 1,
    min_delta, icn.refclk ? "EXT" : "DSI", fin,
    (fin * best_m) / (best_p << (best_s + 1)));
    ref_div = PLL_REF_DIV_P(best_p >> best_p_pot) | PLL_REF_DIV_S(best_s);
    if (best_p_pot)	/* Prefer /2 pre-divider */
    ref_div |= PLL_REF_DIV_Pe;
// Clock source selection either external clock or MIPI DSI clock lane
    chipone_writeb(icn, PLL_CTRL(6),
    icn.refclk ? PLL_CTRL_6_EXTERNAL : PLL_CTRL_6_MIPI_CLK);
    chipone_writeb(icn, PLL_REF_DIV, ref_div);
    chipone_writeb(icn, PLL_INT(0), best_m);
    }
    static void chipone_atomic_enable(struct drm_bridge *bridge,
    struct drm_atomic_commit *state)
    {
    struct chipone *icn = bridge_to_chipone(bridge);
    struct drm_display_mode *mode = &icn.mode;
    const struct drm_bridge_state *bridge_state;
    u16 hfp, hbp, hsync;
    u32 bus_flags;
    u8 pol, sys_ctrl_1, id[4];
    chipone_readb(icn, VENDOR_ID, id);
    chipone_readb(icn, DEVICE_ID_H, id + 1);
    chipone_readb(icn, DEVICE_ID_L, id + 2);
    chipone_readb(icn, VERSION_ID, id + 3);
    dev_dbg(icn.dev,
    "Chip IDs: Vendor=0x%02x Device=0x%02x:0x%02x Version=0x%02x\n",
    id[0], id[1], id[2], id[3]);
    if (id[0] != 0xc1 || id[1] != 0x62 || id[2] != 0x11) {
    dev_dbg(icn.dev, "Invalid Chip IDs, aborting configuration\n");
    return;
    }
// Get the DPI flags from the bridge state.
    bridge_state = drm_atomic_get_new_bridge_state(state, bridge);
    bus_flags = bridge_state.output_bus_cfg.flags;
    if (icn.interface_i2c)
    chipone_writeb(icn, MIPI_CFG_PW, MIPI_CFG_PW_CONFIG_I2C);
    else
    chipone_writeb(icn, MIPI_CFG_PW, MIPI_CFG_PW_CONFIG_DSI);
    chipone_writeb(icn, HACTIVE_LI, mode.hdisplay & 0xff);
    chipone_writeb(icn, VACTIVE_LI, mode.vdisplay & 0xff);
//
// lsb nibble: 2nd nibble of hdisplay
// msb nibble: 2nd nibble of vdisplay
//
    chipone_writeb(icn, VACTIVE_HACTIVE_HI,
    ((mode.hdisplay >> 8) & 0xf) |
    (((mode.vdisplay >> 8) & 0xf) << 4));
    hfp = mode.hsync_start - mode.hdisplay;
    hsync = mode.hsync_end - mode.hsync_start;
    hbp = mode.htotal - mode.hsync_end;
    chipone_writeb(icn, HFP_LI, hfp & 0xff);
    chipone_writeb(icn, HSYNC_LI, hsync & 0xff);
    chipone_writeb(icn, HBP_LI, hbp & 0xff);
// Top two bits of Horizontal Front porch/Sync/Back porch
    chipone_writeb(icn, HFP_HSW_HBP_HI,
    HFP_HSW_HBP_HI_HFP(hfp) |
    HFP_HSW_HBP_HI_HS(hsync) |
    HFP_HSW_HBP_HI_HBP(hbp));
    chipone_writeb(icn, VFP, mode.vsync_start - mode.vdisplay);
    chipone_writeb(icn, VSYNC, mode.vsync_end - mode.vsync_start);
    chipone_writeb(icn, VBP, mode.vtotal - mode.vsync_end);
// dsi specific sequence
    chipone_writeb(icn, SYNC_EVENT_DLY, 0x80);
    chipone_writeb(icn, HFP_MIN, hfp & 0xff);
// DSI data lane count
    chipone_writeb(icn, DSI_CTRL,
    DSI_CTRL_UNKNOWN | DSI_CTRL_DSI_LANES(icn.dsi.lanes - 1));
    chipone_writeb(icn, MIPI_PD_CK_LANE, 0xa0);
    chipone_writeb(icn, PLL_CTRL(12), 0xff);
    chipone_writeb(icn, MIPI_PN_SWAP, 0x00);
// DPI HS/VS/DE polarity
    pol = ((mode.flags & DRM_MODE_FLAG_PHSYNC) ? BIST_POL_HSYNC_POL : 0) |
    ((mode.flags & DRM_MODE_FLAG_PVSYNC) ? BIST_POL_VSYNC_POL : 0) |
    ((bus_flags & DRM_BUS_FLAG_DE_HIGH) ? BIST_POL_DE_POL : 0);
    chipone_writeb(icn, BIST_POL, pol);
// Configure PLL settings
    chipone_configure_pll(icn, mode);
    chipone_writeb(icn, SYS_CTRL(0), 0x40);
    sys_ctrl_1 = 0x88;
    if (bus_flags & DRM_BUS_FLAG_PIXDATA_DRIVE_POSEDGE)
    sys_ctrl_1 |= FIELD_PREP(SYS_CTRL_1_CLK_PHASE_MSK, CLK_PHASE_0);
    else
    sys_ctrl_1 |= FIELD_PREP(SYS_CTRL_1_CLK_PHASE_MSK, CLK_PHASE_1_2);
    chipone_writeb(icn, SYS_CTRL(1), sys_ctrl_1);
// icn6211 specific sequence
    chipone_writeb(icn, MIPI_FORCE_0, 0x20);
    chipone_writeb(icn, PLL_CTRL(1), 0x20);
    chipone_writeb(icn, CONFIG_FINISH, 0x10);
    usleep_range(10000, 11000);
    }
    static void chipone_atomic_pre_enable(struct drm_bridge *bridge,
    struct drm_atomic_commit *state)
    {
    struct chipone *icn = bridge_to_chipone(bridge);
    int ret;
    if (icn.vdd1) {
    ret = regulator_enable(icn.vdd1);
    if (ret)
    DRM_DEV_ERROR(icn.dev,
    "failed to enable VDD1 regulator: %d\n", ret);
    }
    if (icn.vdd2) {
    ret = regulator_enable(icn.vdd2);
    if (ret)
    DRM_DEV_ERROR(icn.dev,
    "failed to enable VDD2 regulator: %d\n", ret);
    }
    if (icn.vdd3) {
    ret = regulator_enable(icn.vdd3);
    if (ret)
    DRM_DEV_ERROR(icn.dev,
    "failed to enable VDD3 regulator: %d\n", ret);
    }
    ret = clk_prepare_enable(icn.refclk);
    if (ret)
    DRM_DEV_ERROR(icn.dev,
    "failed to enable RECLK clock: %d\n", ret);
    gpiod_set_value(icn.enable_gpio, 1);
    usleep_range(10000, 11000);
    }
    static void chipone_atomic_post_disable(struct drm_bridge *bridge,
    struct drm_atomic_commit *state)
    {
    struct chipone *icn = bridge_to_chipone(bridge);
    clk_disable_unprepare(icn.refclk);
    if (icn.vdd1)
    regulator_disable(icn.vdd1);
    if (icn.vdd2)
    regulator_disable(icn.vdd2);
    if (icn.vdd3)
    regulator_disable(icn.vdd3);
    gpiod_set_value(icn.enable_gpio, 0);
    }
    static void chipone_mode_set(struct drm_bridge *bridge,
    const struct drm_display_mode *mode,
    const struct drm_display_mode *adjusted_mode)
    {
    struct chipone *icn = bridge_to_chipone(bridge);
    drm_mode_copy(&icn.mode, adjusted_mode);
    };
#[no_mangle]
unsafe extern "C" fn chipone_dsi_attach(icn: *mut chipone) -> c_int {
    static int chipone_dsi_attach(struct chipone *icn)
    {
    struct mipi_dsi_device *dsi = icn.dsi;
    struct device *dev = icn.dev;
    int dsi_lanes, ret;
    dsi_lanes = drm_of_get_data_lanes_count_ep(dev.of_node, 0, 0, 1, 4);
//
// If the 'data-lanes' property does not exist in DT or is invalid,
// default to previously hard-coded behavior, which was 4 data lanes.
//
    if (dsi_lanes < 0)
    icn.dsi.lanes = 4;
    else
    icn.dsi.lanes = dsi_lanes;
    dsi.format = MIPI_DSI_FMT_RGB888;
    dsi.mode_flags = MIPI_DSI_MODE_VIDEO | MIPI_DSI_MODE_VIDEO_BURST |
    MIPI_DSI_MODE_LPM | MIPI_DSI_MODE_NO_EOT_PACKET;
    dsi.hs_rate = 500000000;
    dsi.lp_rate = 16000000;
    ret = mipi_dsi_attach(dsi);
    if (ret < 0)
    dev_err(icn.dev, "failed to attach dsi\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn chipone_dsi_host_attach(icn: *mut chipone) -> c_int {
    static int chipone_dsi_host_attach(struct chipone *icn)
    {
    struct device *dev = icn.dev;
    struct device_node *host_node;
    struct device_node *endpoint;
    struct mipi_dsi_device *dsi;
    struct mipi_dsi_host *host;
    let mut ret: c_int = 0;
    const struct mipi_dsi_device_info info = {
    .type = "chipone",
    .channel = 0,
    .node = core::ptr::null_mut(),
    };
    endpoint = of_graph_get_endpoint_by_regs(dev.of_node, 0, 0);
    host_node = of_graph_get_remote_port_parent(endpoint);
    of_node_put(endpoint);
    if (!host_node)
    return -EINVAL;
    host = of_find_mipi_dsi_host_by_node(host_node);
    of_node_put(host_node);
    if (!host)
    return dev_err_probe(dev, -EPROBE_DEFER, "failed to find dsi host\n");
    dsi = mipi_dsi_device_register_full(host, &info);
    if (IS_ERR(dsi)) {
    return dev_err_probe(dev, PTR_ERR(dsi),
    "failed to create dsi device\n");
    }
    icn.dsi = dsi;
    ret = chipone_dsi_attach(icn);
    if (ret < 0)
    mipi_dsi_device_unregister(dsi);
    return ret;
    }
    static int chipone_attach(struct drm_bridge *bridge,
    struct drm_encoder *encoder,
    enum drm_bridge_attach_flags flags)
    {
    struct chipone *icn = bridge_to_chipone(bridge);
    return drm_bridge_attach(encoder, icn.panel_bridge, bridge, flags);
    }
pub const MAX_INPUT_SEL_FORMATS: c_int = 1;
    static u32 *
    chipone_atomic_get_input_bus_fmts(struct drm_bridge *bridge,
    struct drm_bridge_state *bridge_state,
    struct drm_crtc_state *crtc_state,
    struct drm_connector_state *conn_state,
    u32 output_fmt,
    unsigned int *num_input_fmts)
    {
    u32 *input_fmts;
// num_input_fmts = 0;
    input_fmts = kcalloc(MAX_INPUT_SEL_FORMATS, sizeof(*input_fmts),
    GFP_KERNEL);
    if (!input_fmts)
    return core::ptr::null_mut();
// This is the DSI-end bus format
    input_fmts[0] = MEDIA_BUS_FMT_RGB888_1X24;
// num_input_fmts = 1;
    return input_fmts;
    }
    static const struct drm_bridge_funcs chipone_bridge_funcs = {
    .atomic_duplicate_state	= drm_atomic_helper_bridge_duplicate_state,
    .atomic_destroy_state	= drm_atomic_helper_bridge_destroy_state,
    .atomic_create_state		= drm_atomic_helper_bridge_create_state,
    .atomic_pre_enable	= chipone_atomic_pre_enable,
    .atomic_enable		= chipone_atomic_enable,
    .atomic_post_disable	= chipone_atomic_post_disable,
    .mode_set		= chipone_mode_set,
    .attach			= chipone_attach,
    .atomic_get_input_bus_fmts = chipone_atomic_get_input_bus_fmts,
    };
#[no_mangle]
unsafe extern "C" fn chipone_parse_dt(icn: *mut chipone) -> c_int {
    static int chipone_parse_dt(struct chipone *icn)
    {
    struct device *dev = icn.dev;
    int ret;
    icn.refclk = devm_clk_get_optional(dev, "refclk");
    if (IS_ERR(icn.refclk)) {
    ret = PTR_ERR(icn.refclk);
    DRM_DEV_ERROR(dev, "failed to get REFCLK clock: %d\n", ret);
    return ret;
    } else if (icn.refclk) {
    icn.refclk_rate = clk_get_rate(icn.refclk);
    if (icn.refclk_rate < 10000000 || icn.refclk_rate > 154000000) {
    DRM_DEV_ERROR(dev, "REFCLK out of range: %ld Hz\n",
    icn.refclk_rate);
    return -EINVAL;
    }
    }
    icn.vdd1 = devm_regulator_get_optional(dev, "vdd1");
    if (IS_ERR(icn.vdd1)) {
    ret = PTR_ERR(icn.vdd1);
    if (ret == -EPROBE_DEFER)
    return -EPROBE_DEFER;
    icn.vdd1 = core::ptr::null_mut();
    DRM_DEV_DEBUG(dev, "failed to get VDD1 regulator: %d\n", ret);
    }
    icn.vdd2 = devm_regulator_get_optional(dev, "vdd2");
    if (IS_ERR(icn.vdd2)) {
    ret = PTR_ERR(icn.vdd2);
    if (ret == -EPROBE_DEFER)
    return -EPROBE_DEFER;
    icn.vdd2 = core::ptr::null_mut();
    DRM_DEV_DEBUG(dev, "failed to get VDD2 regulator: %d\n", ret);
    }
    icn.vdd3 = devm_regulator_get_optional(dev, "vdd3");
    if (IS_ERR(icn.vdd3)) {
    ret = PTR_ERR(icn.vdd3);
    if (ret == -EPROBE_DEFER)
    return -EPROBE_DEFER;
    icn.vdd3 = core::ptr::null_mut();
    DRM_DEV_DEBUG(dev, "failed to get VDD3 regulator: %d\n", ret);
    }
    icn.enable_gpio = devm_gpiod_get(dev, "enable", GPIOD_OUT_LOW);
    if (IS_ERR(icn.enable_gpio)) {
    DRM_DEV_ERROR(dev, "failed to get enable GPIO\n");
    return PTR_ERR(icn.enable_gpio);
    }
    icn.panel_bridge = devm_drm_of_get_bridge(dev, dev.of_node, 1, 0);
    if (IS_ERR(icn.panel_bridge))
    return PTR_ERR(icn.panel_bridge);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn chipone_common_probe(dev: *mut device, icnr: *mut chipone) -> c_int {
    static int chipone_common_probe(struct device *dev, struct chipone **icnr)
    {
    struct chipone *icn;
    int ret;
    icn = devm_drm_bridge_alloc(dev, struct chipone, bridge,
    &chipone_bridge_funcs);
    if (IS_ERR(icn))
    return PTR_ERR(icn);
    icn.dev = dev;
    ret = chipone_parse_dt(icn);
    if (ret)
    return ret;
    icn.bridge.type = DRM_MODE_CONNECTOR_DPI;
    icn.bridge.of_node = dev.of_node;
// icnr = icn;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn chipone_dsi_probe(dsi: *mut mipi_dsi_device) -> c_int {
    static int chipone_dsi_probe(struct mipi_dsi_device *dsi)
    {
    struct device *dev = &dsi.dev;
    struct chipone *icn;
    int ret;
    ret = chipone_common_probe(dev, &icn);
    if (ret)
    return ret;
    icn.regmap = devm_regmap_init(dev, &chipone_dsi_regmap_bus,
    dsi, &chipone_regmap_config);
    if (IS_ERR(icn.regmap))
    return PTR_ERR(icn.regmap);
    icn.interface_i2c = false;
    icn.dsi = dsi;
    mipi_dsi_set_drvdata(dsi, icn);
    ret = devm_drm_bridge_add(dev, &icn.bridge);
    if (ret)
    return ret;
    return chipone_dsi_attach(icn);
    }
#[no_mangle]
unsafe extern "C" fn chipone_i2c_probe(client: *mut i2c_client) -> c_int {
    static int chipone_i2c_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct chipone *icn;
    int ret;
    ret = chipone_common_probe(dev, &icn);
    if (ret)
    return ret;
    icn.regmap = devm_regmap_init_i2c(client, &chipone_regmap_config);
    if (IS_ERR(icn.regmap))
    return PTR_ERR(icn.regmap);
    icn.interface_i2c = true;
    icn.client = client;
    dev_set_drvdata(dev, icn);
    i2c_set_clientdata(client, icn);
    ret = devm_drm_bridge_add(dev, &icn.bridge);
    if (ret)
    return ret;
    return chipone_dsi_host_attach(icn);
    }
#[no_mangle]
unsafe extern "C" fn chipone_dsi_remove(dsi: *mut mipi_dsi_device) {
    static void chipone_dsi_remove(struct mipi_dsi_device *dsi)
    {
    mipi_dsi_detach(dsi);
    }
    static const struct of_device_id chipone_of_match[] = {
    { .compatible = "chipone,icn6211", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, chipone_of_match);
    static struct mipi_dsi_driver chipone_dsi_driver = {
    .probe = chipone_dsi_probe,
    .remove = chipone_dsi_remove,
    .driver = {
    .name = "chipone-icn6211",
    .of_match_table = chipone_of_match,
    },
    };
    static const struct i2c_device_id chipone_i2c_id[] = {
    { .name = "chipone,icn6211" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, chipone_i2c_id);
    static struct i2c_driver chipone_i2c_driver = {
    .probe = chipone_i2c_probe,
    .id_table = chipone_i2c_id,
    .driver = {
    .name = "chipone-icn6211-i2c",
    .of_match_table = chipone_of_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn chipone_init() -> int __init {
    static int __init chipone_init(void)
    {
    if (IS_ENABLED(CONFIG_DRM_MIPI_DSI))
    mipi_dsi_driver_register(&chipone_dsi_driver);
    return i2c_add_driver(&chipone_i2c_driver);
    }
    module_init(chipone_init);
#[no_mangle]
unsafe extern "C" fn chipone_exit() -> void __exit {
    static void __exit chipone_exit(void)
    {
    i2c_del_driver(&chipone_i2c_driver);
    if (IS_ENABLED(CONFIG_DRM_MIPI_DSI))
    mipi_dsi_driver_unregister(&chipone_dsi_driver);
    }
    module_exit(chipone_exit);
    MODULE_AUTHOR("Jagan Teki <jagan@amarulasolutions.com>");
    MODULE_DESCRIPTION("Chipone ICN6211 MIPI-DSI to RGB Converter Bridge");
    MODULE_LICENSE("GPL");
