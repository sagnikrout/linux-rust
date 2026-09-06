//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/bridge/synopsys/dw-mipi-dsi.c
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
// Copyright (c) 2016, Fuzhou Rockchip Electronics Co., Ltd
// Copyright (C) STMicroelectronics SA 2017
//
// Modified by Philippe Cornu <philippe.cornu@st.com>
// This generic Synopsys DesignWare MIPI DSI host driver is based on the
// Rockchip version from rockchip/dw-mipi-dsi.c with phy & bridge APIs.
//

pub const HWVER_131: c_uint = 0x31333100	/* IP version 1.31 */;
pub const DSI_VERSION: c_uint = 0x00;

pub const DSI_PWR_UP: c_uint = 0x04;
pub const RESET: c_int = 0;

pub const DSI_CLKMGR_CFG: c_uint = 0x08;

pub const DSI_DPI_VCID: c_uint = 0x0c;

pub const DSI_DPI_COLOR_CODING: c_uint = 0x10;

pub const DPI_COLOR_CODING_16BIT_1: c_uint = 0x0;
pub const DPI_COLOR_CODING_16BIT_2: c_uint = 0x1;
pub const DPI_COLOR_CODING_16BIT_3: c_uint = 0x2;
pub const DPI_COLOR_CODING_18BIT_1: c_uint = 0x3;
pub const DPI_COLOR_CODING_18BIT_2: c_uint = 0x4;
pub const DPI_COLOR_CODING_24BIT: c_uint = 0x5;
pub const DSI_DPI_CFG_POL: c_uint = 0x14;

pub const DSI_DPI_LP_CMD_TIM: c_uint = 0x18;

pub const DSI_DBI_VCID: c_uint = 0x1c;
pub const DSI_DBI_CFG: c_uint = 0x20;
pub const DSI_DBI_PARTITIONING_EN: c_uint = 0x24;
pub const DSI_DBI_CMDSIZE: c_uint = 0x28;
pub const DSI_PCKHDL_CFG: c_uint = 0x2c;

pub const DSI_GEN_VCID: c_uint = 0x30;
pub const DSI_MODE_CFG: c_uint = 0x34;
pub const ENABLE_VIDEO_MODE: c_int = 0;

pub const DSI_VID_MODE_CFG: c_uint = 0x38;

pub const VID_MODE_TYPE_NON_BURST_SYNC_PULSES: c_uint = 0x0;
pub const VID_MODE_TYPE_NON_BURST_SYNC_EVENTS: c_uint = 0x1;
pub const VID_MODE_TYPE_BURST: c_uint = 0x2;
pub const VID_MODE_TYPE_MASK: c_uint = 0x3;

pub const DSI_VID_PKT_SIZE: c_uint = 0x3c;

pub const DSI_VID_NUM_CHUNKS: c_uint = 0x40;

pub const DSI_VID_NULL_SIZE: c_uint = 0x44;

pub const DSI_VID_HSA_TIME: c_uint = 0x48;
pub const DSI_VID_HBP_TIME: c_uint = 0x4c;
pub const DSI_VID_HLINE_TIME: c_uint = 0x50;
pub const DSI_VID_VSA_LINES: c_uint = 0x54;
pub const DSI_VID_VBP_LINES: c_uint = 0x58;
pub const DSI_VID_VFP_LINES: c_uint = 0x5c;
pub const DSI_VID_VACTIVE_LINES: c_uint = 0x60;
pub const DSI_EDPI_CMD_SIZE: c_uint = 0x64;
pub const DSI_CMD_MODE_CFG: c_uint = 0x68;

    DCS_LW_TX_LP | \
    DCS_SR_0P_TX_LP | \
    DCS_SW_1P_TX_LP | \
    DCS_SW_0P_TX_LP | \
    GEN_LW_TX_LP | \
    GEN_SR_2P_TX_LP | \
    GEN_SR_1P_TX_LP | \
    GEN_SR_0P_TX_LP | \
    GEN_SW_2P_TX_LP | \
    GEN_SW_1P_TX_LP | \
    GEN_SW_0P_TX_LP)
pub const DSI_GEN_HDR: c_uint = 0x6c;
pub const DSI_GEN_PLD_DATA: c_uint = 0x70;
pub const DSI_CMD_PKT_STATUS: c_uint = 0x74;

pub const DSI_TO_CNT_CFG: c_uint = 0x78;

pub const DSI_HS_RD_TO_CNT: c_uint = 0x7c;
pub const DSI_LP_RD_TO_CNT: c_uint = 0x80;
pub const DSI_HS_WR_TO_CNT: c_uint = 0x84;
pub const DSI_LP_WR_TO_CNT: c_uint = 0x88;
pub const DSI_BTA_TO_CNT: c_uint = 0x8c;
pub const DSI_LPCLK_CTRL: c_uint = 0x94;

pub const DSI_PHY_TMR_LPCLK_CFG: c_uint = 0x98;

pub const DSI_PHY_TMR_CFG: c_uint = 0x9c;

pub const DSI_PHY_RSTZ: c_uint = 0xa0;
pub const PHY_DISFORCEPLL: c_int = 0;

pub const PHY_DISABLECLK: c_int = 0;

pub const PHY_RSTZ: c_int = 0;

pub const PHY_SHUTDOWNZ: c_int = 0;

pub const DSI_PHY_IF_CFG: c_uint = 0xa4;

pub const DSI_PHY_ULPS_CTRL: c_uint = 0xa8;
pub const DSI_PHY_TX_TRIGGERS: c_uint = 0xac;
pub const DSI_PHY_STATUS: c_uint = 0xb0;

pub const DSI_PHY_TST_CTRL0: c_uint = 0xb4;

pub const PHY_UNTESTCLK: c_int = 0;

pub const PHY_UNTESTCLR: c_int = 0;
pub const DSI_PHY_TST_CTRL1: c_uint = 0xb8;

pub const PHY_UNTESTEN: c_int = 0;

pub const DSI_INT_ST0: c_uint = 0xbc;
pub const DSI_INT_ST1: c_uint = 0xc0;
pub const DSI_INT_MSK0: c_uint = 0xc4;
pub const DSI_INT_MSK1: c_uint = 0xc8;
pub const DSI_PHY_TMR_RD_CFG: c_uint = 0xf4;

pub const PHY_STATUS_TIMEOUT_US: c_int = 10000;
pub const CMD_PKT_STATUS_TIMEOUT_US: c_int = 20000;

    ((void  *)&((*dsi).vpg_defs.name))

    { #name, VPG_DEFS(name, dsi), mask, dsi }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct debugfs_entries {
    pub name: *const c_char,
    pub reg: *mut bool,
    pub mask: u32,
    pub dsi: *mut dw_mipi_dsi,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_mipi_dsi {
    pub bridge: drm_bridge,
    pub dsi_host: mipi_dsi_host,
    pub panel_bridge: *mut drm_bridge,
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub pclk: *mut clk,
    pub /: *mut *mut unsigned int lane_mbps; / per lane,
    pub channel: u32,
    pub lanes: u32,
    pub format: u32,
    pub mode_flags: c_ulong,

    pub debugfs: *mut dentry,
    pub debugfs_vpg: *mut debugfs_entries,
    struct {
    pub vpg: bool,
    pub vpg_horizontal: bool,
    pub vpg_ber_pattern: bool,
    pub vpg_defs: },

    pub /: *mut *mut *mut dw_mipi_dsi master; / dual-dsi master ptr,
    pub /: *mut *mut *mut dw_mipi_dsi slave; / dual-dsi slave ptr,
    pub mode: drm_display_mode,
    pub plat_data: *const dw_mipi_dsi_plat_data,
}

//
// Check if either a link to a master or slave is present
//
#[no_mangle]
pub unsafe extern "C" fn dw_mipi_is_dual_mode(dsi: *mut dw_mipi_dsi) -> bool {
    static inline bool dw_mipi_is_dual_mode(struct dw_mipi_dsi *dsi)
    {
    return dsi.slave || dsi.master;
    }
//
// The controller should generate 2 frames before
// preparing the peripheral.
//
#[no_mangle]
unsafe extern "C" fn dw_mipi_dsi_wait_for_two_frames(mode: *const drm_display_mode) {
    static void dw_mipi_dsi_wait_for_two_frames(const struct drm_display_mode *mode)
    {
    int refresh, two_frames;
    refresh = drm_mode_vrefresh(mode);
    two_frames = DIV_ROUND_UP(MSEC_PER_SEC, refresh) * 2;
    msleep(two_frames);
    }
    static inline struct dw_mipi_dsi *host_to_dsi(struct mipi_dsi_host *host)
    {
    return container_of(host, struct dw_mipi_dsi, dsi_host);
    }
    static inline struct dw_mipi_dsi *bridge_to_dsi(struct drm_bridge *bridge)
    {
    return container_of(bridge, struct dw_mipi_dsi, bridge);
    }
#[no_mangle]
pub unsafe extern "C" fn dsi_write(dsi: *mut dw_mipi_dsi, reg: u32, val: u32) {
    static inline void dsi_write(struct dw_mipi_dsi *dsi, u32 reg, u32 val)
    {
    writel(val, dsi.base + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn dsi_read(dsi: *mut dw_mipi_dsi, reg: u32) -> u32 {
    static inline u32 dsi_read(struct dw_mipi_dsi *dsi, u32 reg)
    {
    return readl(dsi.base + reg);
    }
    static int dw_mipi_dsi_host_attach(struct mipi_dsi_host *host,
    struct mipi_dsi_device *device)
    {
    struct dw_mipi_dsi *dsi = host_to_dsi(host);
    const struct dw_mipi_dsi_plat_data *pdata = dsi.plat_data;
    struct drm_bridge *bridge;
    int ret;
    if (device.lanes > dsi.plat_data.max_data_lanes) {
    dev_err(dsi.dev, "the number of data lanes(%u) is too many\n",
    device.lanes);
    return -EINVAL;
    }
    dsi.lanes = device.lanes;
    dsi.channel = device.channel;
    dsi.format = device.format;
    dsi.mode_flags = device.mode_flags;
    bridge = devm_drm_of_get_bridge(dsi.dev, dsi.dev.of_node, 1, 0);
    if (IS_ERR(bridge))
    return PTR_ERR(bridge);
    bridge.pre_enable_prev_first = true;
    dsi.panel_bridge = bridge;
    drm_bridge_add(&dsi.bridge);
    if (pdata.host_ops && pdata.host_ops.attach) {
    ret = pdata.host_ops.attach(pdata.priv_data, device);
    if (ret < 0)
    goto err_remove_bridge;
    }
    return 0;
    err_remove_bridge:
    drm_bridge_remove(&dsi.bridge);
    return ret;
    }
    static int dw_mipi_dsi_host_detach(struct mipi_dsi_host *host,
    struct mipi_dsi_device *device)
    {
    struct dw_mipi_dsi *dsi = host_to_dsi(host);
    const struct dw_mipi_dsi_plat_data *pdata = dsi.plat_data;
    int ret;
    if (pdata.host_ops && pdata.host_ops.detach) {
    ret = pdata.host_ops.detach(pdata.priv_data, device);
    if (ret < 0)
    return ret;
    }
    drm_of_panel_bridge_remove(host.dev.of_node, 1, 0);
    drm_bridge_remove(&dsi.bridge);
    return 0;
    }
    static void dw_mipi_message_config(struct dw_mipi_dsi *dsi,
    const struct mipi_dsi_msg *msg)
    {
    let mut lpm: bool = msg.flags & MIPI_DSI_MSG_USE_LPM;
    let mut val: u32 = 0;
//
// TODO dw drv improvements
// largest packet sizes during hfp or during vsa/vpb/vfp
// should be computed according to byte lane, lane number and only
// if sending lp cmds in high speed is enable (PHY_TXREQUESTCLKHS)
//
    dsi_write(dsi, DSI_DPI_LP_CMD_TIM, OUTVACT_LPCMD_TIME(16)
    | INVACT_LPCMD_TIME(4));
    if (msg.flags & MIPI_DSI_MSG_REQ_ACK)
    val |= ACK_RQST_EN;
    if (lpm)
    val |= CMD_MODE_ALL_LP;
    dsi_write(dsi, DSI_CMD_MODE_CFG, val);
    val = dsi_read(dsi, DSI_VID_MODE_CFG);
    if (lpm)
    val |= ENABLE_LOW_POWER_CMD;
    else
    val &= ~ENABLE_LOW_POWER_CMD;
    dsi_write(dsi, DSI_VID_MODE_CFG, val);
    }
#[no_mangle]
unsafe extern "C" fn dw_mipi_dsi_gen_pkt_hdr_write(dsi: *mut dw_mipi_dsi, hdr_val: u32) -> c_int {
    static int dw_mipi_dsi_gen_pkt_hdr_write(struct dw_mipi_dsi *dsi, u32 hdr_val)
    {
    int ret;
    u32 val, mask;
    ret = readl_poll_timeout(dsi.base + DSI_CMD_PKT_STATUS,
    val, !(val & GEN_CMD_FULL), 1000,
    CMD_PKT_STATUS_TIMEOUT_US);
    if (ret) {
    dev_err(dsi.dev, "failed to get available command FIFO\n");
    return ret;
    }
    dsi_write(dsi, DSI_GEN_HDR, hdr_val);
    mask = GEN_CMD_EMPTY | GEN_PLD_W_EMPTY;
    ret = readl_poll_timeout(dsi.base + DSI_CMD_PKT_STATUS,
    val, (val & mask) == mask,
    1000, CMD_PKT_STATUS_TIMEOUT_US);
    if (ret) {
    dev_err(dsi.dev, "failed to write command FIFO\n");
    return ret;
    }
    return 0;
    }
    static int dw_mipi_dsi_write(struct dw_mipi_dsi *dsi,
    const struct mipi_dsi_packet *packet)
    {
    const u8 *tx_buf = packet.payload;
    let mut len: c_int = packet.payload_length, pld_data_bytes = sizeof(u32), ret;
    __le32 word;
    u32 val;
    while (len) {
    if (len < pld_data_bytes) {
    word = 0;
    memcpy(&word, tx_buf, len);
    dsi_write(dsi, DSI_GEN_PLD_DATA, le32_to_cpu(word));
    len = 0;
    } else {
    memcpy(&word, tx_buf, pld_data_bytes);
    dsi_write(dsi, DSI_GEN_PLD_DATA, le32_to_cpu(word));
    tx_buf += pld_data_bytes;
    len -= pld_data_bytes;
    }
    ret = readl_poll_timeout(dsi.base + DSI_CMD_PKT_STATUS,
    val, !(val & GEN_PLD_W_FULL), 1000,
    CMD_PKT_STATUS_TIMEOUT_US);
    if (ret) {
    dev_err(dsi.dev,
    "failed to get available write payload FIFO\n");
    return ret;
    }
    }
    word = 0;
    memcpy(&word, packet.header, sizeof(packet.header));
    return dw_mipi_dsi_gen_pkt_hdr_write(dsi, le32_to_cpu(word));
    }
    static int dw_mipi_dsi_read(struct dw_mipi_dsi *dsi,
    const struct mipi_dsi_msg *msg)
    {
    int i, j, ret, len = msg.rx_len;
    u8 *buf = msg.rx_buf;
    u32 val;
// Wait end of the read operation
    ret = readl_poll_timeout(dsi.base + DSI_CMD_PKT_STATUS,
    val, !(val & GEN_RD_CMD_BUSY),
    1000, CMD_PKT_STATUS_TIMEOUT_US);
    if (ret) {
    dev_err(dsi.dev, "Timeout during read operation\n");
    return ret;
    }
    for (i = 0; i < len; i += 4) {
// Read fifo must not be empty before all bytes are read
    ret = readl_poll_timeout(dsi.base + DSI_CMD_PKT_STATUS,
    val, !(val & GEN_PLD_R_EMPTY),
    1000, CMD_PKT_STATUS_TIMEOUT_US);
    if (ret) {
    dev_err(dsi.dev, "Read payload FIFO is empty\n");
    return ret;
    }
    val = dsi_read(dsi, DSI_GEN_PLD_DATA);
    for (j = 0; j < 4 && j + i < len; j++)
    buf[i + j] = val >> (8 * j);
    }
    return ret;
    }
    static ssize_t dw_mipi_dsi_host_transfer(struct mipi_dsi_host *host,
    const struct mipi_dsi_msg *msg)
    {
    struct dw_mipi_dsi *dsi = host_to_dsi(host);
    struct mipi_dsi_packet packet;
    int ret, nb_bytes;
    ret = mipi_dsi_create_packet(&packet, msg);
    if (ret) {
    dev_err(dsi.dev, "failed to create packet: %d\n", ret);
    return ret;
    }
    dw_mipi_message_config(dsi, msg);
    if (dsi.slave)
    dw_mipi_message_config(dsi.slave, msg);
    ret = dw_mipi_dsi_write(dsi, &packet);
    if (ret)
    return ret;
    if (dsi.slave) {
    ret = dw_mipi_dsi_write(dsi.slave, &packet);
    if (ret)
    return ret;
    }
    if (msg.rx_buf && msg.rx_len) {
    ret = dw_mipi_dsi_read(dsi, msg);
    if (ret)
    return ret;
    nb_bytes = msg.rx_len;
    } else {
    nb_bytes = packet.size;
    }
    return nb_bytes;
    }
    static const struct mipi_dsi_host_ops dw_mipi_dsi_host_ops = {
    .attach = dw_mipi_dsi_host_attach,
    .detach = dw_mipi_dsi_host_detach,
    .transfer = dw_mipi_dsi_host_transfer,
    };
    static u32 *
    dw_mipi_dsi_bridge_atomic_get_input_bus_fmts(struct drm_bridge *bridge,
    struct drm_bridge_state *bridge_state,
    struct drm_crtc_state *crtc_state,
    struct drm_connector_state *conn_state,
    u32 output_fmt,
    unsigned int *num_input_fmts)
    {
    struct dw_mipi_dsi *dsi = bridge_to_dsi(bridge);
    const struct dw_mipi_dsi_plat_data *pdata = dsi.plat_data;
    u32 *input_fmts;
    if (pdata.get_input_bus_fmts)
    return pdata.get_input_bus_fmts(pdata.priv_data,
    bridge, bridge_state,
    crtc_state, conn_state,
    output_fmt, num_input_fmts);
// Fall back to MEDIA_BUS_FMT_FIXED as the only input format.
    input_fmts = kmalloc_obj(*input_fmts);
    if (!input_fmts)
    return core::ptr::null_mut();
    input_fmts[0] = MEDIA_BUS_FMT_FIXED;
// num_input_fmts = 1;
    return input_fmts;
    }
    static int dw_mipi_dsi_bridge_atomic_check(struct drm_bridge *bridge,
    struct drm_bridge_state *bridge_state,
    struct drm_crtc_state *crtc_state,
    struct drm_connector_state *conn_state)
    {
    struct dw_mipi_dsi *dsi = bridge_to_dsi(bridge);
    const struct dw_mipi_dsi_plat_data *pdata = dsi.plat_data;
    bool ret;
    bridge_state.input_bus_cfg.flags =
    DRM_BUS_FLAG_DE_HIGH | DRM_BUS_FLAG_PIXDATA_SAMPLE_NEGEDGE;
    if (pdata.mode_fixup) {
    ret = pdata.mode_fixup(pdata.priv_data, &crtc_state.mode,
    &crtc_state.adjusted_mode);
    if (!ret) {
    DRM_DEBUG_DRIVER("failed to fixup mode " DRM_MODE_FMT "\n",
    DRM_MODE_ARG(&crtc_state.mode));
    return -EINVAL;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dw_mipi_dsi_video_mode_config(dsi: *mut dw_mipi_dsi) {
    static void dw_mipi_dsi_video_mode_config(struct dw_mipi_dsi *dsi)
    {
    u32 val;
//
// TODO dw drv improvements
// enabling low power is panel-dependent, we should use the
// panel configuration here...
//
    val = ENABLE_LOW_POWER;
    if (dsi.mode_flags & MIPI_DSI_MODE_VIDEO_BURST)
    val |= VID_MODE_TYPE_BURST;
#[no_mangle]
pub unsafe extern "C" fn if(MIPI_DSI_MODE_VIDEO_SYNC_PULSE: dsi->mode_flags &) -> else {
    else if (dsi.mode_flags & MIPI_DSI_MODE_VIDEO_SYNC_PULSE)
    val |= VID_MODE_TYPE_NON_BURST_SYNC_PULSES;
    else
    val |= VID_MODE_TYPE_NON_BURST_SYNC_EVENTS;

    if (dsi.vpg_defs.vpg) {
    val |= VID_MODE_VPG_ENABLE;
    val |= dsi.vpg_defs.vpg_horizontal ?
    VID_MODE_VPG_HORIZONTAL : 0;
    val |= dsi.vpg_defs.vpg_ber_pattern ? VID_MODE_VPG_MODE : 0;
    }

    dsi_write(dsi, DSI_VID_MODE_CFG, val);
    }
    static void dw_mipi_dsi_set_mode(struct dw_mipi_dsi *dsi,
    unsigned long mode_flags)
    {
    u32 val;
    dsi_write(dsi, DSI_PWR_UP, RESET);
    if (mode_flags & MIPI_DSI_MODE_VIDEO) {
    dsi_write(dsi, DSI_MODE_CFG, ENABLE_VIDEO_MODE);
    dw_mipi_dsi_video_mode_config(dsi);
    } else {
    dsi_write(dsi, DSI_MODE_CFG, ENABLE_CMD_MODE);
    }
    val = PHY_TXREQUESTCLKHS;
    if (dsi.mode_flags & MIPI_DSI_CLOCK_NON_CONTINUOUS)
    val |= AUTO_CLKLANE_CTRL;
    dsi_write(dsi, DSI_LPCLK_CTRL, val);
    dsi_write(dsi, DSI_PWR_UP, POWERUP);
    }
#[no_mangle]
unsafe extern "C" fn dw_mipi_dsi_disable(dsi: *mut dw_mipi_dsi) {
    static void dw_mipi_dsi_disable(struct dw_mipi_dsi *dsi)
    {
    dsi_write(dsi, DSI_PWR_UP, RESET);
    dsi_write(dsi, DSI_PHY_RSTZ, PHY_RSTZ);
    }
#[no_mangle]
unsafe extern "C" fn dw_mipi_dsi_init(dsi: *mut dw_mipi_dsi) {
    static void dw_mipi_dsi_init(struct dw_mipi_dsi *dsi)
    {
    const struct dw_mipi_dsi_phy_ops *phy_ops = dsi.plat_data.phy_ops;
    unsigned int esc_rate; /* in MHz */
    u32 esc_clk_division;
    int ret;
//
// The maximum permitted escape clock is 20MHz and it is derived from
// lanebyteclk, which is running at "lane_mbps / 8".
//
    if (phy_ops.get_esc_clk_rate) {
    ret = phy_ops.get_esc_clk_rate(dsi.plat_data.priv_data,
    &esc_rate);
    if (ret)
    DRM_DEBUG_DRIVER("Phy get_esc_clk_rate() failed\n");
    } else
    esc_rate = 20; /* Default to 20MHz */
//
// We want :
// (lane_mbps >> 3) / esc_clk_division < X
// which is:
// (lane_mbps >> 3) / X > esc_clk_division
//
    esc_clk_division = (dsi.lane_mbps >> 3) / esc_rate + 1;
    dsi_write(dsi, DSI_PWR_UP, RESET);
//
// TODO dw drv improvements
// timeout clock division should be computed with the
// high speed transmission counter timeout and byte lane...
//
    dsi_write(dsi, DSI_CLKMGR_CFG, TO_CLK_DIVISION(0) |
    TX_ESC_CLK_DIVISION(esc_clk_division));
    }
    static void dw_mipi_dsi_dpi_config(struct dw_mipi_dsi *dsi,
    const struct drm_display_mode *mode)
    {
    let mut val: u32 = 0, color = 0;
    switch (dsi.format) {
    case MIPI_DSI_FMT_RGB888:
    color = DPI_COLOR_CODING_24BIT;
    break;
    case MIPI_DSI_FMT_RGB666:
    color = DPI_COLOR_CODING_18BIT_2 | LOOSELY18_EN;
    break;
    case MIPI_DSI_FMT_RGB666_PACKED:
    color = DPI_COLOR_CODING_18BIT_1;
    break;
    case MIPI_DSI_FMT_RGB565:
    color = DPI_COLOR_CODING_16BIT_1;
    break;
    }
    if (mode.flags & DRM_MODE_FLAG_NVSYNC)
    val |= VSYNC_ACTIVE_LOW;
    if (mode.flags & DRM_MODE_FLAG_NHSYNC)
    val |= HSYNC_ACTIVE_LOW;
    dsi_write(dsi, DSI_DPI_VCID, DPI_VCID(dsi.channel));
    dsi_write(dsi, DSI_DPI_COLOR_CODING, color);
    dsi_write(dsi, DSI_DPI_CFG_POL, val);
    }
#[no_mangle]
unsafe extern "C" fn dw_mipi_dsi_packet_handler_config(dsi: *mut dw_mipi_dsi) {
    static void dw_mipi_dsi_packet_handler_config(struct dw_mipi_dsi *dsi)
    {
    let mut val: u32 = CRC_RX_EN | ECC_RX_EN | BTA_EN | EOTP_TX_EN;
    if (dsi.mode_flags & MIPI_DSI_MODE_NO_EOT_PACKET)
    val &= ~EOTP_TX_EN;
    dsi_write(dsi, DSI_PCKHDL_CFG, val);
    }
    static void dw_mipi_dsi_video_packet_config(struct dw_mipi_dsi *dsi,
    const struct drm_display_mode *mode)
    {
//
// TODO dw drv improvements
// only burst mode is supported here. For non-burst video modes,
// we should compute DSI_VID_PKT_SIZE, DSI_VCCR.NUMC &
// DSI_VNPCR.NPSIZE... especially because this driver supports
// non-burst video modes, see dw_mipi_dsi_video_mode_config()...
//
    dsi_write(dsi, DSI_VID_PKT_SIZE,
    dw_mipi_is_dual_mode(dsi) ?
    VID_PKT_SIZE(mode.hdisplay / 2) :
    VID_PKT_SIZE(mode.hdisplay));
    }
#[no_mangle]
unsafe extern "C" fn dw_mipi_dsi_command_mode_config(dsi: *mut dw_mipi_dsi) {
    static void dw_mipi_dsi_command_mode_config(struct dw_mipi_dsi *dsi)
    {
//
// TODO dw drv improvements
// compute high speed transmission counter timeout according
// to the timeout clock division (TO_CLK_DIVISION) and byte lane...
//
    dsi_write(dsi, DSI_TO_CNT_CFG, HSTX_TO_CNT(0) | LPRX_TO_CNT(0));
//
// TODO dw drv improvements
// the Bus-Turn-Around Timeout Counter should be computed
// according to byte lane...
//
    dsi_write(dsi, DSI_BTA_TO_CNT, 0xd00);
    dsi_write(dsi, DSI_MODE_CFG, ENABLE_CMD_MODE);
    }
    static const u32 minimum_lbccs[] = {10, 5, 4, 3};
#[no_mangle]
pub unsafe extern "C" fn dw_mipi_dsi_get_minimum_lbcc(dsi: *mut dw_mipi_dsi) -> u32 {
    static inline u32 dw_mipi_dsi_get_minimum_lbcc(struct dw_mipi_dsi *dsi)
    {
    return minimum_lbccs[dsi.lanes - 1];
    }
// Get lane byte clock cycles.
    static u32 dw_mipi_dsi_get_hcomponent_lbcc(struct dw_mipi_dsi *dsi,
    const struct drm_display_mode *mode,
    u32 hcomponent)
    {
    u32 frac, lbcc, minimum_lbcc;
    int bpp;
    if (dsi.mode_flags & MIPI_DSI_MODE_VIDEO_BURST) {
// lbcc based on lane_mbps
    lbcc = hcomponent * dsi.lane_mbps * MSEC_PER_SEC / 8;
    } else {
// lbcc based on pixel clock rate
    bpp = mipi_dsi_pixel_format_to_bpp(dsi.format);
    if (bpp < 0) {
    dev_err(dsi.dev, "failed to get bpp\n");
    return 0;
    }
    lbcc = div_u64((u64)hcomponent * mode.clock * bpp, dsi.lanes * 8);
    }
    frac = lbcc % mode.clock;
    lbcc = lbcc / mode.clock;
    if (frac)
    lbcc++;
    minimum_lbcc = dw_mipi_dsi_get_minimum_lbcc(dsi);
    if (lbcc < minimum_lbcc)
    lbcc = minimum_lbcc;
    return lbcc;
    }
    static void dw_mipi_dsi_line_timer_config(struct dw_mipi_dsi *dsi,
    const struct drm_display_mode *mode)
    {
    u32 htotal, hsa, hbp, lbcc;
    htotal = mode.htotal;
    hsa = mode.hsync_end - mode.hsync_start;
    hbp = mode.htotal - mode.hsync_end;
//
// TODO dw drv improvements
// computations below may be improved...
//
    lbcc = dw_mipi_dsi_get_hcomponent_lbcc(dsi, mode, htotal);
    dsi_write(dsi, DSI_VID_HLINE_TIME, lbcc);
    lbcc = dw_mipi_dsi_get_hcomponent_lbcc(dsi, mode, hsa);
    dsi_write(dsi, DSI_VID_HSA_TIME, lbcc);
    lbcc = dw_mipi_dsi_get_hcomponent_lbcc(dsi, mode, hbp);
    dsi_write(dsi, DSI_VID_HBP_TIME, lbcc);
    }
    static void dw_mipi_dsi_vertical_timing_config(struct dw_mipi_dsi *dsi,
    const struct drm_display_mode *mode)
    {
    u32 vactive, vsa, vfp, vbp;
    vactive = mode.vdisplay;
    vsa = mode.vsync_end - mode.vsync_start;
    vfp = mode.vsync_start - mode.vdisplay;
    vbp = mode.vtotal - mode.vsync_end;
    dsi_write(dsi, DSI_VID_VACTIVE_LINES, vactive);
    dsi_write(dsi, DSI_VID_VSA_LINES, vsa);
    dsi_write(dsi, DSI_VID_VFP_LINES, vfp);
    dsi_write(dsi, DSI_VID_VBP_LINES, vbp);
    }
#[no_mangle]
unsafe extern "C" fn dw_mipi_dsi_dphy_timing_config(dsi: *mut dw_mipi_dsi) {
    static void dw_mipi_dsi_dphy_timing_config(struct dw_mipi_dsi *dsi)
    {
    const struct dw_mipi_dsi_phy_ops *phy_ops = dsi.plat_data.phy_ops;
    struct dw_mipi_dsi_dphy_timing timing;
    u32 hw_version;
    int ret;
    ret = phy_ops.get_timing(dsi.plat_data.priv_data,
    dsi.lane_mbps, &timing);
    if (ret)
    DRM_DEV_ERROR(dsi.dev, "Retrieving phy timings failed\n");
//
// TODO dw drv improvements
// data & clock lane timers should be computed according to panel
// blankings and to the automatic clock lane control mode...
// note: DSI_PHY_TMR_CFG.MAX_RD_TIME should be in line with
// DSI_CMD_MODE_CFG.MAX_RD_PKT_SIZE_LP (see CMD_MODE_ALL_LP)
//
    hw_version = dsi_read(dsi, DSI_VERSION) & VERSION;
    if (hw_version >= HWVER_131) {
    dsi_write(dsi, DSI_PHY_TMR_CFG,
    PHY_HS2LP_TIME_V131(timing.data_hs2lp) |
    PHY_LP2HS_TIME_V131(timing.data_lp2hs));
    dsi_write(dsi, DSI_PHY_TMR_RD_CFG, MAX_RD_TIME_V131(10000));
    } else {
    dsi_write(dsi, DSI_PHY_TMR_CFG,
    PHY_HS2LP_TIME(timing.data_hs2lp) |
    PHY_LP2HS_TIME(timing.data_lp2hs) |
    MAX_RD_TIME(10000));
    }
    dsi_write(dsi, DSI_PHY_TMR_LPCLK_CFG,
    PHY_CLKHS2LP_TIME(timing.clk_hs2lp) |
    PHY_CLKLP2HS_TIME(timing.clk_lp2hs));
    }
#[no_mangle]
unsafe extern "C" fn dw_mipi_dsi_dphy_interface_config(dsi: *mut dw_mipi_dsi) {
    static void dw_mipi_dsi_dphy_interface_config(struct dw_mipi_dsi *dsi)
    {
//
// TODO dw drv improvements
// stop wait time should be the maximum between host dsi
// and panel stop wait times
//
    dsi_write(dsi, DSI_PHY_IF_CFG, PHY_STOP_WAIT_TIME(0x20) |
    N_LANES(dsi.lanes));
    }
#[no_mangle]
unsafe extern "C" fn dw_mipi_dsi_dphy_init(dsi: *mut dw_mipi_dsi) {
    static void dw_mipi_dsi_dphy_init(struct dw_mipi_dsi *dsi)
    {
// Clear PHY state
    dsi_write(dsi, DSI_PHY_RSTZ, PHY_DISFORCEPLL | PHY_DISABLECLK
    | PHY_RSTZ | PHY_SHUTDOWNZ);
    dsi_write(dsi, DSI_PHY_TST_CTRL0, PHY_UNTESTCLR);
    dsi_write(dsi, DSI_PHY_TST_CTRL0, PHY_TESTCLR);
    dsi_write(dsi, DSI_PHY_TST_CTRL0, PHY_UNTESTCLR);
    }
#[no_mangle]
unsafe extern "C" fn dw_mipi_dsi_dphy_enable(dsi: *mut dw_mipi_dsi) {
    static void dw_mipi_dsi_dphy_enable(struct dw_mipi_dsi *dsi)
    {
    u32 val;
    int ret;
    dsi_write(dsi, DSI_PHY_RSTZ, PHY_ENFORCEPLL | PHY_ENABLECLK |
    PHY_UNRSTZ | PHY_UNSHUTDOWNZ);
    ret = readl_poll_timeout(dsi.base + DSI_PHY_STATUS, val,
    val & PHY_LOCK, 1000, PHY_STATUS_TIMEOUT_US);
    if (ret)
    DRM_DEBUG_DRIVER("failed to wait phy lock state\n");
    ret = readl_poll_timeout(dsi.base + DSI_PHY_STATUS,
    val, val & PHY_STOP_STATE_CLK_LANE, 1000,
    PHY_STATUS_TIMEOUT_US);
    if (ret)
    DRM_DEBUG_DRIVER("failed to wait phy clk lane stop state\n");
    }
#[no_mangle]
unsafe extern "C" fn dw_mipi_dsi_clear_err(dsi: *mut dw_mipi_dsi) {
    static void dw_mipi_dsi_clear_err(struct dw_mipi_dsi *dsi)
    {
    dsi_read(dsi, DSI_INT_ST0);
    dsi_read(dsi, DSI_INT_ST1);
    dsi_write(dsi, DSI_INT_MSK0, 0);
    dsi_write(dsi, DSI_INT_MSK1, 0);
    }
    static void dw_mipi_dsi_bridge_post_atomic_disable(struct drm_bridge *bridge,
    struct drm_atomic_commit *state)
    {
    struct dw_mipi_dsi *dsi = bridge_to_dsi(bridge);
    const struct dw_mipi_dsi_phy_ops *phy_ops = dsi.plat_data.phy_ops;
//
// Switch to command mode before panel-bridge post_disable &
// panel unprepare.
// Note: panel-bridge disable & panel disable has been called
// before by the drm framework.
//
    dw_mipi_dsi_set_mode(dsi, 0);
    if (phy_ops.power_off)
    phy_ops.power_off(dsi.plat_data.priv_data);
    if (dsi.slave) {
    dw_mipi_dsi_disable(dsi.slave);
    clk_disable_unprepare(dsi.slave.pclk);
    pm_runtime_put(dsi.slave.dev);
    }
    dw_mipi_dsi_disable(dsi);
    clk_disable_unprepare(dsi.pclk);
    pm_runtime_put(dsi.dev);
    }
#[no_mangle]
unsafe extern "C" fn dw_mipi_dsi_get_lanes(dsi: *mut dw_mipi_dsi) -> c_uint {
    static unsigned int dw_mipi_dsi_get_lanes(struct dw_mipi_dsi *dsi)
    {
// this instance is the slave, so add the master's lanes
    if (dsi.master)
    return dsi.master.lanes + dsi.lanes;
// this instance is the master, so add the slave's lanes
    if (dsi.slave)
    return dsi.lanes + dsi.slave.lanes;
// single-dsi, so no other instance to consider
    return dsi.lanes;
    }
    static void dw_mipi_dsi_mode_set(struct dw_mipi_dsi *dsi,
    const struct drm_display_mode *adjusted_mode)
    {
    const struct dw_mipi_dsi_phy_ops *phy_ops = dsi.plat_data.phy_ops;
    void *priv_data = dsi.plat_data.priv_data;
    int ret;
    let mut lanes: u32 = dw_mipi_dsi_get_lanes(dsi);
    clk_prepare_enable(dsi.pclk);
    ret = phy_ops.get_lane_mbps(priv_data, adjusted_mode, dsi.mode_flags,
    lanes, dsi.format, &dsi.lane_mbps);
    if (ret)
    DRM_DEBUG_DRIVER("Phy get_lane_mbps() failed\n");
    pm_runtime_get_sync(dsi.dev);
    dw_mipi_dsi_init(dsi);
    dw_mipi_dsi_dpi_config(dsi, adjusted_mode);
    dw_mipi_dsi_packet_handler_config(dsi);
    dw_mipi_dsi_video_mode_config(dsi);
    dw_mipi_dsi_video_packet_config(dsi, adjusted_mode);
    dw_mipi_dsi_command_mode_config(dsi);
    dw_mipi_dsi_line_timer_config(dsi, adjusted_mode);
    dw_mipi_dsi_vertical_timing_config(dsi, adjusted_mode);
    dw_mipi_dsi_dphy_init(dsi);
    dw_mipi_dsi_dphy_timing_config(dsi);
    dw_mipi_dsi_dphy_interface_config(dsi);
    dw_mipi_dsi_clear_err(dsi);
    ret = phy_ops.init(priv_data);
    if (ret)
    DRM_DEBUG_DRIVER("Phy init() failed\n");
    dw_mipi_dsi_dphy_enable(dsi);
    dw_mipi_dsi_wait_for_two_frames(adjusted_mode);
// Switch to cmd mode for panel-bridge pre_enable & panel prepare
    dw_mipi_dsi_set_mode(dsi, 0);
    if (phy_ops.power_on)
    phy_ops.power_on(dsi.plat_data.priv_data);
    }
    static void dw_mipi_dsi_bridge_atomic_pre_enable(struct drm_bridge *bridge,
    struct drm_atomic_commit *state)
    {
    struct dw_mipi_dsi *dsi = bridge_to_dsi(bridge);
// Power up the dsi ctl into a command mode
    dw_mipi_dsi_mode_set(dsi, &dsi.mode);
    if (dsi.slave)
    dw_mipi_dsi_mode_set(dsi.slave, &dsi.mode);
    }
    static void dw_mipi_dsi_bridge_mode_set(struct drm_bridge *bridge,
    const struct drm_display_mode *mode,
    const struct drm_display_mode *adjusted_mode)
    {
    struct dw_mipi_dsi *dsi = bridge_to_dsi(bridge);
// Store the display mode for later use in pre_enable callback
    drm_mode_copy(&dsi.mode, adjusted_mode);
    }
    static void dw_mipi_dsi_bridge_atomic_enable(struct drm_bridge *bridge,
    struct drm_atomic_commit *state)
    {
    struct dw_mipi_dsi *dsi = bridge_to_dsi(bridge);
// Switch to video mode for panel-bridge enable & panel enable
    dw_mipi_dsi_set_mode(dsi, MIPI_DSI_MODE_VIDEO);
    if (dsi.slave)
    dw_mipi_dsi_set_mode(dsi.slave, MIPI_DSI_MODE_VIDEO);
    }
    static enum drm_mode_status
    dw_mipi_dsi_bridge_mode_valid(struct drm_bridge *bridge,
    const struct drm_display_info *info,
    const struct drm_display_mode *mode)
    {
    struct dw_mipi_dsi *dsi = bridge_to_dsi(bridge);
    const struct dw_mipi_dsi_plat_data *pdata = dsi.plat_data;
    let mut mode_status: enum drm_mode_status = MODE_OK;
    if (pdata.mode_valid)
    mode_status = pdata.mode_valid(pdata.priv_data, mode,
    dsi.mode_flags,
    dw_mipi_dsi_get_lanes(dsi),
    dsi.format);
    return mode_status;
    }
    static int dw_mipi_dsi_bridge_attach(struct drm_bridge *bridge,
    struct drm_encoder *encoder,
    enum drm_bridge_attach_flags flags)
    {
    struct dw_mipi_dsi *dsi = bridge_to_dsi(bridge);
// Set the encoder type as caller does not know it
    encoder.encoder_type = DRM_MODE_ENCODER_DSI;
// Attach the panel-bridge to the dsi bridge
    return drm_bridge_attach(encoder, dsi.panel_bridge, bridge,
    flags);
    }
    static const struct drm_bridge_funcs dw_mipi_dsi_bridge_funcs = {
    .atomic_duplicate_state	= drm_atomic_helper_bridge_duplicate_state,
    .atomic_destroy_state	= drm_atomic_helper_bridge_destroy_state,
    .atomic_get_input_bus_fmts = dw_mipi_dsi_bridge_atomic_get_input_bus_fmts,
    .atomic_check		= dw_mipi_dsi_bridge_atomic_check,
    .atomic_create_state		= drm_atomic_helper_bridge_create_state,
    .atomic_pre_enable	= dw_mipi_dsi_bridge_atomic_pre_enable,
    .atomic_enable		= dw_mipi_dsi_bridge_atomic_enable,
    .atomic_post_disable	= dw_mipi_dsi_bridge_post_atomic_disable,
    .mode_set		= dw_mipi_dsi_bridge_mode_set,
    .mode_valid		= dw_mipi_dsi_bridge_mode_valid,
    .attach			= dw_mipi_dsi_bridge_attach,
    };

#[no_mangle]
unsafe extern "C" fn dw_mipi_dsi_debugfs_write(data: *mut c_void, val: u64) -> c_int {
    static int dw_mipi_dsi_debugfs_write(void *data, u64 val)
    {
    struct debugfs_entries *vpg = data;
    struct dw_mipi_dsi *dsi;
    u32 mode_cfg;
    if (!vpg)
    return -ENODEV;
    dsi = vpg.dsi;
// vpg->reg = (bool)val;
    mode_cfg = dsi_read(dsi, DSI_VID_MODE_CFG);
    if (*vpg.reg)
    mode_cfg |= vpg.mask;
    else
    mode_cfg &= ~vpg.mask;
    dsi_write(dsi, DSI_VID_MODE_CFG, mode_cfg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dw_mipi_dsi_debugfs_show(data: *mut c_void, val: *mut u64) -> c_int {
    static int dw_mipi_dsi_debugfs_show(void *data, u64 *val)
    {
    struct debugfs_entries *vpg = data;
    if (!vpg)
    return -ENODEV;
// val = *vpg->reg;
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(fops_x32, dw_mipi_dsi_debugfs_show,
    dw_mipi_dsi_debugfs_write, "%llu\n");
#[no_mangle]
unsafe extern "C" fn debugfs_create_files(data: *mut c_void) {
    static void debugfs_create_files(void *data)
    {
    struct dw_mipi_dsi *dsi = data;
    struct debugfs_entries debugfs[] = {
    REGISTER(vpg, VID_MODE_VPG_ENABLE, dsi),
    REGISTER(vpg_horizontal, VID_MODE_VPG_HORIZONTAL, dsi),
    REGISTER(vpg_ber_pattern, VID_MODE_VPG_MODE, dsi),
    };
    int i;
    dsi.debugfs_vpg = kmemdup(debugfs, sizeof(debugfs), GFP_KERNEL);
    if (!dsi.debugfs_vpg)
    return;
    for (i = 0; i < ARRAY_SIZE(debugfs); i++)
    debugfs_create_file(dsi.debugfs_vpg[i].name, 0644,
    dsi.debugfs, &dsi.debugfs_vpg[i],
    &fops_x32);
    }
#[no_mangle]
unsafe extern "C" fn dw_mipi_dsi_debugfs_init(dsi: *mut dw_mipi_dsi) {
    static void dw_mipi_dsi_debugfs_init(struct dw_mipi_dsi *dsi)
    {
    dsi.debugfs = debugfs_create_dir(dev_name(dsi.dev), core::ptr::null_mut());
    if (IS_ERR(dsi.debugfs)) {
    dev_err(dsi.dev, "failed to create debugfs root\n");
    return;
    }
    debugfs_create_files(dsi);
    }
#[no_mangle]
unsafe extern "C" fn dw_mipi_dsi_debugfs_remove(dsi: *mut dw_mipi_dsi) {
    static void dw_mipi_dsi_debugfs_remove(struct dw_mipi_dsi *dsi)
    {
    debugfs_remove_recursive(dsi.debugfs);
    kfree(dsi.debugfs_vpg);
    }

    static void dw_mipi_dsi_debugfs_init(struct dw_mipi_dsi *dsi) { }
    static void dw_mipi_dsi_debugfs_remove(struct dw_mipi_dsi *dsi) { }

    static struct dw_mipi_dsi *
    __dw_mipi_dsi_probe(struct platform_device *pdev,
    const struct dw_mipi_dsi_plat_data *plat_data)
    {
    struct device *dev = &pdev.dev;
    struct reset_control *apb_rst;
    struct dw_mipi_dsi *dsi;
    int ret;
    dsi = devm_drm_bridge_alloc(dev, struct dw_mipi_dsi, bridge,
    &dw_mipi_dsi_bridge_funcs);
    if (IS_ERR(dsi))
    return ERR_CAST(dsi);
    dsi.dev = dev;
    dsi.plat_data = plat_data;
    if (!plat_data.phy_ops.init || !plat_data.phy_ops.get_lane_mbps ||
    !plat_data.phy_ops.get_timing) {
    DRM_ERROR("Phy not properly configured\n");
    return ERR_PTR(-ENODEV);
    }
    if (!plat_data.base) {
    dsi.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(dsi.base))
    return ERR_PTR(-ENODEV);
    } else {
    dsi.base = plat_data.base;
    }
    dsi.pclk = devm_clk_get(dev, "pclk");
    if (IS_ERR(dsi.pclk)) {
    ret = PTR_ERR(dsi.pclk);
    dev_err(dev, "Unable to get pclk: %d\n", ret);
    return ERR_PTR(ret);
    }
//
// Note that the reset was not defined in the initial device tree, so
// we have to be prepared for it not being found.
//
    apb_rst = devm_reset_control_get_optional_exclusive(dev, "apb");
    if (IS_ERR(apb_rst)) {
    ret = PTR_ERR(apb_rst);
    if (ret != -EPROBE_DEFER)
    dev_err(dev, "Unable to get reset control: %d\n", ret);
    return ERR_PTR(ret);
    }
    if (apb_rst) {
    ret = clk_prepare_enable(dsi.pclk);
    if (ret) {
    dev_err(dev, "%s: Failed to enable pclk\n", __func__);
    return ERR_PTR(ret);
    }
    reset_control_assert(apb_rst);
    usleep_range(10, 20);
    reset_control_deassert(apb_rst);
    clk_disable_unprepare(dsi.pclk);
    }
    dw_mipi_dsi_debugfs_init(dsi);
    pm_runtime_enable(dev);
    dsi.dsi_host.ops = &dw_mipi_dsi_host_ops;
    dsi.dsi_host.dev = dev;
    ret = mipi_dsi_host_register(&dsi.dsi_host);
    if (ret) {
    dev_err(dev, "Failed to register MIPI host: %d\n", ret);
    pm_runtime_disable(dev);
    dw_mipi_dsi_debugfs_remove(dsi);
    return ERR_PTR(ret);
    }
    dsi.bridge.driver_private = dsi;
    dsi.bridge.of_node = pdev.dev.of_node;
    return dsi;
    }
#[no_mangle]
unsafe extern "C" fn __dw_mipi_dsi_remove(dsi: *mut dw_mipi_dsi) {
    static void __dw_mipi_dsi_remove(struct dw_mipi_dsi *dsi)
    {
    mipi_dsi_host_unregister(&dsi.dsi_host);
    pm_runtime_disable(dsi.dev);
    dw_mipi_dsi_debugfs_remove(dsi);
    }
#[no_mangle]
pub unsafe extern "C" fn dw_mipi_dsi_set_slave(dsi: *mut dw_mipi_dsi, slave: *mut dw_mipi_dsi) {
    void dw_mipi_dsi_set_slave(struct dw_mipi_dsi *dsi, struct dw_mipi_dsi *slave)
    {
// introduce controllers to each other
    dsi.slave = slave;
    dsi.slave.master = dsi;
// migrate settings for already attached displays
    dsi.slave.lanes = dsi.lanes;
    dsi.slave.channel = dsi.channel;
    dsi.slave.format = dsi.format;
    dsi.slave.mode_flags = dsi.mode_flags;
    }
    EXPORT_SYMBOL_GPL(dw_mipi_dsi_set_slave);
    struct drm_bridge *dw_mipi_dsi_get_bridge(struct dw_mipi_dsi *dsi)
    {
    return &dsi.bridge;
    }
    EXPORT_SYMBOL_GPL(dw_mipi_dsi_get_bridge);
//
// Probe/remove API, used from platforms based on the DRM bridge API.
//
    struct dw_mipi_dsi *
    dw_mipi_dsi_probe(struct platform_device *pdev,
    const struct dw_mipi_dsi_plat_data *plat_data)
    {
    return __dw_mipi_dsi_probe(pdev, plat_data);
    }
    EXPORT_SYMBOL_GPL(dw_mipi_dsi_probe);
#[no_mangle]
pub unsafe extern "C" fn dw_mipi_dsi_remove(dsi: *mut dw_mipi_dsi) {
    void dw_mipi_dsi_remove(struct dw_mipi_dsi *dsi)
    {
    __dw_mipi_dsi_remove(dsi);
    }
    EXPORT_SYMBOL_GPL(dw_mipi_dsi_remove);
//
// Bind/unbind API, used from platforms based on the component framework.
//
#[no_mangle]
pub unsafe extern "C" fn dw_mipi_dsi_bind(dsi: *mut dw_mipi_dsi, encoder: *mut drm_encoder) -> c_int {
    int dw_mipi_dsi_bind(struct dw_mipi_dsi *dsi, struct drm_encoder *encoder)
    {
    return drm_bridge_attach(encoder, &dsi.bridge, core::ptr::null_mut(), 0);
    }
    EXPORT_SYMBOL_GPL(dw_mipi_dsi_bind);
#[no_mangle]
pub unsafe extern "C" fn dw_mipi_dsi_unbind(dsi: *mut dw_mipi_dsi) {
    void dw_mipi_dsi_unbind(struct dw_mipi_dsi *dsi)
    {
    }
    EXPORT_SYMBOL_GPL(dw_mipi_dsi_unbind);
    MODULE_AUTHOR("Chris Zhong <zyw@rock-chips.com>");
    MODULE_AUTHOR("Philippe Cornu <philippe.cornu@st.com>");
    MODULE_DESCRIPTION("DW MIPI DSI host controller driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:dw-mipi-dsi");
