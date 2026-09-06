//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/kmb/kmb_dsi.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright © 2019-2020 Intel Corporation
//

// MIPI TX CFG
pub const MIPI_TX_LANE_DATA_RATE_MBPS: c_int = 891;
pub const MIPI_TX_REF_CLK_KHZ: c_int = 24000;
pub const MIPI_TX_CFG_CLK_KHZ: c_int = 24000;
pub const MIPI_TX_BPP: c_int = 24;
// DPHY Tx test codes
pub const TEST_CODE_FSM_CONTROL: c_uint = 0x03;
pub const TEST_CODE_MULTIPLE_PHY_CTRL: c_uint = 0x0C;
pub const TEST_CODE_PLL_PROPORTIONAL_CHARGE_PUMP_CTRL: c_uint = 0x0E;
pub const TEST_CODE_PLL_INTEGRAL_CHARGE_PUMP_CTRL: c_uint = 0x0F;
pub const TEST_CODE_PLL_VCO_CTRL: c_uint = 0x12;
pub const TEST_CODE_PLL_GMP_CTRL: c_uint = 0x13;
pub const TEST_CODE_PLL_PHASE_ERR_CTRL: c_uint = 0x14;
pub const TEST_CODE_PLL_LOCK_FILTER: c_uint = 0x15;
pub const TEST_CODE_PLL_UNLOCK_FILTER: c_uint = 0x16;
pub const TEST_CODE_PLL_INPUT_DIVIDER: c_uint = 0x17;
pub const TEST_CODE_PLL_FEEDBACK_DIVIDER: c_uint = 0x18;

pub const TEST_CODE_PLL_OUTPUT_CLK_SEL: c_uint = 0x19;

pub const TEST_CODE_VOD_LEVEL: c_uint = 0x24;
pub const TEST_CODE_PLL_CHARGE_PUMP_BIAS: c_uint = 0x1C;
pub const TEST_CODE_PLL_LOCK_DETECTOR: c_uint = 0x1D;
pub const TEST_CODE_HS_FREQ_RANGE_CFG: c_uint = 0x44;
pub const TEST_CODE_PLL_ANALOG_PROG: c_uint = 0x1F;
pub const TEST_CODE_SLEW_RATE_OVERRIDE_CTRL: c_uint = 0xA0;
pub const TEST_CODE_SLEW_RATE_DDL_LOOP_CTRL: c_uint = 0xA3;
pub const TEST_CODE_SLEW_RATE_DDL_CYCLES: c_uint = 0xA4;
// DPHY params
pub const PLL_N_MIN: c_int = 0;
pub const PLL_N_MAX: c_int = 15;
pub const PLL_M_MIN: c_int = 62;
pub const PLL_M_MAX: c_int = 623;
pub const PLL_FVCO_MAX: c_int = 1250;
pub const TIMEOUT: c_int = 600;
pub const MIPI_TX_FRAME_GEN: c_int = 4;
pub const MIPI_TX_FRAME_GEN_SECTIONS: c_int = 4;
pub const MIPI_CTRL_VIRTUAL_CHANNELS: c_int = 4;
pub const MIPI_D_LANES_PER_DPHY: c_int = 2;
pub const MIPI_CTRL_2LANE_MAX_MC_FIFO_LOC: c_int = 255;
pub const MIPI_CTRL_4LANE_MAX_MC_FIFO_LOC: c_int = 511;
// 2 Data Lanes per D-PHY
pub const MIPI_DPHY_D_LANES: c_int = 2;
pub const MIPI_DPHY_DEFAULT_BIT_RATES: c_int = 63;
pub const KMB_MIPI_DEFAULT_CLK: c_int = 24000000;
pub const KMB_MIPI_DEFAULT_CFG_CLK: c_int = 24000000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kmb_dsi {
    pub base: drm_encoder,
    pub dev: *mut device,
    pub pdev: *mut platform_device,
    pub host: *mut mipi_dsi_host,
    pub device: *mut mipi_dsi_device,
    pub adv_bridge: *mut drm_bridge,
    pub mipi_mmio: *mut void __iomem,
    pub clk_mipi: *mut clk,
    pub clk_mipi_ecfg: *mut clk,
    pub clk_mipi_cfg: *mut clk,
    pub sys_clk_mhz: c_int,
}

// DPHY Tx test codes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mipi_ctrl_num {
    MIPI_CTRL0 = 0,
    MIPI_CTRL1,
    MIPI_CTRL2,
    MIPI_CTRL3,
    MIPI_CTRL4,
    MIPI_CTRL5,
    MIPI_CTRL6,
    MIPI_CTRL7,
    MIPI_CTRL8,
    MIPI_CTRL9,
    MIPI_CTRL_NA
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mipi_dphy_num {
    MIPI_DPHY0 = 0,
    MIPI_DPHY1,
    MIPI_DPHY2,
    MIPI_DPHY3,
    MIPI_DPHY4,
    MIPI_DPHY5,
    MIPI_DPHY6,
    MIPI_DPHY7,
    MIPI_DPHY8,
    MIPI_DPHY9,
    MIPI_DPHY_NA
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mipi_dir {
    MIPI_RX,
    MIPI_TX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mipi_ctrl_type {
    MIPI_DSI,
    MIPI_CSI
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mipi_data_if {
    MIPI_IF_DMA,
    MIPI_IF_PARALLEL
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mipi_data_mode {
    MIPI_DATA_MODE0,
    MIPI_DATA_MODE1,
    MIPI_DATA_MODE2,
    MIPI_DATA_MODE3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mipi_dsi_video_mode {
    DSI_VIDEO_MODE_NO_BURST_PULSE,
    DSI_VIDEO_MODE_NO_BURST_EVENT,
    DSI_VIDEO_MODE_BURST
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mipi_dsi_blanking_mode {
    TRANSITION_TO_LOW_POWER,
    SEND_BLANK_PACKET
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mipi_dsi_eotp {
    DSI_EOTP_DISABLED,
    DSI_EOTP_ENABLES
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mipi_dsi_data_type {
    DSI_SP_DT_RESERVED_00 = 0x00,
    DSI_SP_DT_VSYNC_START = 0x01,
    DSI_SP_DT_COLOR_MODE_OFF = 0x02,
    DSI_SP_DT_GENERIC_SHORT_WR = 0x03,
    DSI_SP_DT_GENERIC_RD = 0x04,
    DSI_SP_DT_DCS_SHORT_WR = 0x05,
    DSI_SP_DT_DCS_RD = 0x06,
    DSI_SP_DT_EOTP = 0x08,
    DSI_LP_DT_NULL = 0x09,
    DSI_LP_DT_RESERVED_0A = 0x0a,
    DSI_LP_DT_RESERVED_0B = 0x0b,
    DSI_LP_DT_LPPS_YCBCR422_20B = 0x0c,
    DSI_LP_DT_PPS_RGB101010_30B = 0x0d,
    DSI_LP_DT_PPS_RGB565_16B = 0x0e,
    DSI_LP_DT_RESERVED_0F = 0x0f,

    DSI_SP_DT_RESERVED_10 = 0x10,
    DSI_SP_DT_VSYNC_END = 0x11,
    DSI_SP_DT_COLOR_MODE_ON = 0x12,
    DSI_SP_DT_GENERIC_SHORT_WR_1PAR = 0x13,
    DSI_SP_DT_GENERIC_RD_1PAR = 0x14,
    DSI_SP_DT_DCS_SHORT_WR_1PAR = 0x15,
    DSI_SP_DT_RESERVED_16 = 0x16,
    DSI_SP_DT_RESERVED_17 = 0x17,
    DSI_SP_DT_RESERVED_18 = 0x18,
    DSI_LP_DT_BLANK = 0x19,
    DSI_LP_DT_RESERVED_1A = 0x1a,
    DSI_LP_DT_RESERVED_1B = 0x1b,
    DSI_LP_DT_PPS_YCBCR422_24B = 0x1c,
    DSI_LP_DT_PPS_RGB121212_36B = 0x1d,
    DSI_LP_DT_PPS_RGB666_18B = 0x1e,
    DSI_LP_DT_RESERVED_1F = 0x1f,

    DSI_SP_DT_RESERVED_20 = 0x20,
    DSI_SP_DT_HSYNC_START = 0x21,
    DSI_SP_DT_SHUT_DOWN_PERIPH_CMD = 0x22,
    DSI_SP_DT_GENERIC_SHORT_WR_2PAR = 0x23,
    DSI_SP_DT_GENERIC_RD_2PAR = 0x24,
    DSI_SP_DT_RESERVED_25 = 0x25,
    DSI_SP_DT_RESERVED_26 = 0x26,
    DSI_SP_DT_RESERVED_27 = 0x27,
    DSI_SP_DT_RESERVED_28 = 0x28,
    DSI_LP_DT_GENERIC_LONG_WR = 0x29,
    DSI_LP_DT_RESERVED_2A = 0x2a,
    DSI_LP_DT_RESERVED_2B = 0x2b,
    DSI_LP_DT_PPS_YCBCR422_16B = 0x2c,
    DSI_LP_DT_RESERVED_2D = 0x2d,
    DSI_LP_DT_LPPS_RGB666_18B = 0x2e,
    DSI_LP_DT_RESERVED_2F = 0x2f,

    DSI_SP_DT_RESERVED_30 = 0x30,
    DSI_SP_DT_HSYNC_END = 0x31,
    DSI_SP_DT_TURN_ON_PERIPH_CMD = 0x32,
    DSI_SP_DT_RESERVED_33 = 0x33,
    DSI_SP_DT_RESERVED_34 = 0x34,
    DSI_SP_DT_RESERVED_35 = 0x35,
    DSI_SP_DT_RESERVED_36 = 0x36,
    DSI_SP_DT_SET_MAX_RETURN_PKT_SIZE = 0x37,
    DSI_SP_DT_RESERVED_38 = 0x38,
    DSI_LP_DT_DSC_LONG_WR = 0x39,
    DSI_LP_DT_RESERVED_3A = 0x3a,
    DSI_LP_DT_RESERVED_3B = 0x3b,
    DSI_LP_DT_RESERVED_3C = 0x3c,
    DSI_LP_DT_PPS_YCBCR420_12B = 0x3d,
    DSI_LP_DT_PPS_RGB888_24B = 0x3e,
    DSI_LP_DT_RESERVED_3F = 0x3f
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mipi_tx_hs_tp_sel {
    MIPI_TX_HS_TP_WHOLE_FRAME_COLOR0 = 0,
    MIPI_TX_HS_TP_WHOLE_FRAME_COLOR1,
    MIPI_TX_HS_TP_V_STRIPES,
    MIPI_TX_HS_TP_H_STRIPES,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dphy_mode {
    MIPI_DPHY_SLAVE = 0,
    MIPI_DPHY_MASTER
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dphy_tx_fsm {
    DPHY_TX_POWERDWN = 0,
    DPHY_TX_BGPON,
    DPHY_TX_TERMCAL,
    DPHY_TX_TERMCALUP,
    DPHY_TX_OFFSETCAL,
    DPHY_TX_LOCK,
    DPHY_TX_SRCAL,
    DPHY_TX_IDLE,
    DPHY_TX_ULP,
    DPHY_TX_LANESTART,
    DPHY_TX_CLKALIGN,
    DPHY_TX_DDLTUNNING,
    DPHY_TX_ULP_FORCE_PLL,
    DPHY_TX_LOCK_LOSS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mipi_data_type_params {
    pub size_constraint_pixels: u8,
    pub size_constraint_bytes: u8,
    pub pixels_per_pclk: u8,
    pub bits_per_pclk: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mipi_tx_dsi_cfg {
    pub /: *mut *mut u8 hfp_blank_en; / Horizontal front porch blanking enable,
    pub /: *mut *mut u8 eotp_en; / End of transmission packet enable,
// Last vertical front porch blanking mode
    pub lpm_last_vfp_line: u8,
// First vertical sync active blanking mode
    pub lpm_first_vsa_line: u8,
    pub /: *mut *mut u8 sync_pulse_eventn; / Sync type,
    pub /: *mut *mut u8 hfp_blanking; / Horizontal front porch blanking mode,
    pub /: *mut *mut u8 hbp_blanking; / Horizontal back porch blanking mode,
    pub /: *mut *mut u8 hsa_blanking; / Horizontal sync active blanking mode,
    pub /: *mut *mut u8 v_blanking; / Vertical timing blanking mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mipi_tx_frame_section_cfg {
    pub dma_v_stride: u32,
    pub dma_v_scale_cfg: u16,
    pub width_pixels: u16,
    pub height_lines: u16,
    pub dma_packed: u8,
    pub bpp: u8,
    pub bpp_unpacked: u8,
    pub dma_h_stride: u8,
    pub data_type: u8,
    pub data_mode: u8,
    pub dma_flip_rotate_sel: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mipi_tx_frame_timing_cfg {
    pub bpp: u32,
    pub lane_rate_mbps: u32,
    pub hsync_width: u32,
    pub h_backporch: u32,
    pub h_frontporch: u32,
    pub h_active: u32,
    pub vsync_width: u16,
    pub v_backporch: u16,
    pub v_frontporch: u16,
    pub v_active: u16,
    pub active_lanes: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mipi_tx_frame_sect_phcfg {
    pub wc: u32,
    pub data_mode: mipi_data_mode,
    pub data_type: mipi_dsi_data_type,
    pub vchannel: u8,
    pub dma_packed: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mipi_tx_frame_cfg {
    pub sections: [*mut mipi_tx_frame_section_cfg; MIPI_TX_FRAME_GEN_SECTIONS],
    pub /: *mut *mut u32 hsync_width; / in pixels,
    pub /: *mut *mut u32 h_backporch; / in pixels,
    pub /: *mut *mut u32 h_frontporch; / in pixels,
    pub /: *mut *mut u16 vsync_width; / in lines,
    pub /: *mut *mut u16 v_backporch; / in lines,
    pub /: *mut *mut u16 v_frontporch; / in lines,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mipi_tx_ctrl_cfg {
    pub frames: [*mut mipi_tx_frame_cfg; MIPI_TX_FRAME_GEN],
    pub tx_dsi_cfg: *const mipi_tx_dsi_cfg,
    pub line_sync_pkt_en: u8,
    pub line_counter_active: u8,
    pub frame_counter_active: u8,
    pub tx_hsclkkidle_cnt: u8,
    pub tx_hsexit_cnt: u8,
    pub tx_crc_en: u8,
    pub tx_hact_wait_stop: u8,
    pub tx_always_use_hact: u8,
    pub tx_wait_trig: u8,
    pub tx_wait_all_sect: u8,
}

// configuration structure for MIPI control
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mipi_ctrl_cfg {
    pub /: *mut *mut u8 active_lanes; / # active lanes per controller 2/4,
    pub /: *mut *mut u32 lane_rate_mbps; / MBPS,
    pub ref_clk_khz: u32,
    pub cfg_clk_khz: u32,
    pub tx_ctrl_cfg: mipi_tx_ctrl_cfg,
}

extern "C" {
    pub fn readl(reg: kmb_dsi->mipi_mmio +) -> return;
}
extern "C" {
    pub fn kmb_dsi_host_bridge_init(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn kmb_dsi_host_unregister(kmb_dsi: *mut kmb_dsi);
}
extern "C" {
    pub fn kmb_dsi_map_mmio(kmb_dsi: *mut kmb_dsi) -> c_int;
}
extern "C" {
    pub fn kmb_dsi_clk_init(kmb_dsi: *mut kmb_dsi) -> c_int;
}
extern "C" {
    pub fn kmb_dsi_encoder_init(dev: *mut drm_device, kmb_dsi: *mut kmb_dsi) -> c_int;
}
