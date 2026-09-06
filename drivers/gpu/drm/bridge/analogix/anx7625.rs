//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/bridge/analogix/anx7625.h
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
// Copyright(c) 2020, Analogix Semiconductor. All rights reserved.
//

// Loading OCM re-trying times
pub const OCM_LOADING_TIME: c_int = 10;
// ANX7625 Register
pub const TX_P0_ADDR: c_uint = 0x70;
pub const TX_P1_ADDR: c_uint = 0x7A;
pub const TX_P2_ADDR: c_uint = 0x72;
pub const RX_P0_ADDR: c_uint = 0x7e;
pub const RX_P1_ADDR: c_uint = 0x84;
pub const RX_P2_ADDR: c_uint = 0x54;
pub const RSVD_00_ADDR: c_uint = 0x00;
pub const RSVD_D1_ADDR: c_uint = 0xD1;
pub const RSVD_60_ADDR: c_uint = 0x60;
pub const RSVD_39_ADDR: c_uint = 0x39;
pub const RSVD_7F_ADDR: c_uint = 0x7F;
pub const TCPC_INTERFACE_ADDR: c_uint = 0x58;
// Clock frequency in Hz

pub const POST_DIVIDER_MIN: c_int = 1;
pub const POST_DIVIDER_MAX: c_int = 16;

//
// Register definition of device address 0x58
pub const PRODUCT_ID_L: c_uint = 0x02;
pub const PRODUCT_ID_H: c_uint = 0x03;
pub const INTR_ALERT_1: c_uint = 0xCC;

pub const SYSTEM_STSTUS: c_uint = 0x45;
pub const INTERFACE_CHANGE_INT_MASK: c_uint = 0x43;
pub const INTERFACE_CHANGE_INT: c_uint = 0x44;

pub const NEW_CC_STATUS: c_uint = 0x46;

pub const CMD_SEND_BUF: c_uint = 0xC0;
pub const CMD_RECV_BUF: c_uint = 0xE0;
// END of I2C Address 0x58
//
// Register definition of device address 0x70
pub const TX_HDCP_CTRL0: c_uint = 0x01;

pub const SP_TX_WAIT_R0_TIME: c_uint = 0x40;
pub const SP_TX_WAIT_KSVR_TIME: c_uint = 0x42;
pub const SP_TX_SYS_CTRL1_REG: c_uint = 0x80;

pub const SP_TX_LINK_BW_SET_REG: c_uint = 0xA0;
pub const SP_TX_LANE_COUNT_SET_REG: c_uint = 0xA1;
pub const M_VID_0: c_uint = 0xC0;
pub const M_VID_1: c_uint = 0xC1;
pub const M_VID_2: c_uint = 0xC2;
pub const N_VID_0: c_uint = 0xC3;
pub const N_VID_1: c_uint = 0xC4;
pub const N_VID_2: c_uint = 0xC5;
pub const KEY_START_ADDR: c_uint = 0x9000;
pub const KEY_RESERVED: c_int = 416;

pub const HDCP14KEY_SIZE: c_int = 624;
//
// Register definition of device address 0x72
pub const AUX_RST: c_uint = 0x04;
pub const RST_CTRL2: c_uint = 0x07;
pub const SP_TX_TOTAL_LINE_STA_L: c_uint = 0x24;
pub const SP_TX_TOTAL_LINE_STA_H: c_uint = 0x25;
pub const SP_TX_ACT_LINE_STA_L: c_uint = 0x26;
pub const SP_TX_ACT_LINE_STA_H: c_uint = 0x27;
pub const SP_TX_V_F_PORCH_STA: c_uint = 0x28;
pub const SP_TX_V_SYNC_STA: c_uint = 0x29;
pub const SP_TX_V_B_PORCH_STA: c_uint = 0x2A;
pub const SP_TX_TOTAL_PIXEL_STA_L: c_uint = 0x2B;
pub const SP_TX_TOTAL_PIXEL_STA_H: c_uint = 0x2C;
pub const SP_TX_ACT_PIXEL_STA_L: c_uint = 0x2D;
pub const SP_TX_ACT_PIXEL_STA_H: c_uint = 0x2E;
pub const SP_TX_H_F_PORCH_STA_L: c_uint = 0x2F;
pub const SP_TX_H_F_PORCH_STA_H: c_uint = 0x30;
pub const SP_TX_H_SYNC_STA_L: c_uint = 0x31;
pub const SP_TX_H_SYNC_STA_H: c_uint = 0x32;
pub const SP_TX_H_B_PORCH_STA_L: c_uint = 0x33;
pub const SP_TX_H_B_PORCH_STA_H: c_uint = 0x34;
pub const SP_TX_VID_CTRL: c_uint = 0x84;
pub const SP_TX_BPC_MASK: c_uint = 0xE0;
pub const SP_TX_BPC_6: c_uint = 0x00;
pub const SP_TX_BPC_8: c_uint = 0x20;
pub const SP_TX_BPC_10: c_uint = 0x40;
pub const SP_TX_BPC_12: c_uint = 0x60;
pub const VIDEO_BIT_MATRIX_12: c_uint = 0x4c;
pub const AUDIO_CHANNEL_STATUS_1: c_uint = 0xd0;
pub const AUDIO_CHANNEL_STATUS_2: c_uint = 0xd1;
pub const AUDIO_CHANNEL_STATUS_3: c_uint = 0xd2;
pub const AUDIO_CHANNEL_STATUS_4: c_uint = 0xd3;
pub const AUDIO_CHANNEL_STATUS_5: c_uint = 0xd4;
pub const AUDIO_CHANNEL_STATUS_6: c_uint = 0xd5;
pub const TDM_SLAVE_MODE: c_uint = 0x10;
pub const I2S_SLAVE_MODE: c_uint = 0x08;
pub const AUDIO_LAYOUT: c_uint = 0x01;
pub const HPD_DET_TIMER_BIT0_7: c_uint = 0xea;
pub const HPD_DET_TIMER_BIT8_15: c_uint = 0xeb;
pub const HPD_DET_TIMER_BIT16_23: c_uint = 0xec;
// HPD debounce time 2ms for 27M clock
pub const HPD_TIME: c_int = 54000;
pub const AUDIO_CONTROL_REGISTER: c_uint = 0xe6;
pub const TDM_TIMING_MODE: c_uint = 0x08;
pub const I2C_ADDR_72_DPTX: c_uint = 0x72;
pub const HP_MIN: c_int = 8;
pub const HBLANKING_MIN: c_int = 80;
pub const SYNC_LEN_DEF: c_int = 32;

pub const VIDEO_CONTROL_0: c_uint = 0x08;
pub const ACTIVE_LINES_L: c_uint = 0x14;
pub const ACTIVE_LINES_H: c_uint = 0x15  /* Bit[7:6] are reserved */;
pub const VERTICAL_FRONT_PORCH: c_uint = 0x16;
pub const VERTICAL_SYNC_WIDTH: c_uint = 0x17;
pub const VERTICAL_BACK_PORCH: c_uint = 0x18;
pub const HORIZONTAL_TOTAL_PIXELS_L: c_uint = 0x19;
pub const HORIZONTAL_TOTAL_PIXELS_H: c_uint = 0x1A  /* Bit[7:6] are reserved */;
pub const HORIZONTAL_ACTIVE_PIXELS_L: c_uint = 0x1B;
pub const HORIZONTAL_ACTIVE_PIXELS_H: c_uint = 0x1C  /* Bit[7:6] are reserved */;
pub const HORIZONTAL_FRONT_PORCH_L: c_uint = 0x1D;
pub const HORIZONTAL_FRONT_PORCH_H: c_uint = 0x1E  /* Bit[7:4] are reserved */;
pub const HORIZONTAL_SYNC_WIDTH_L: c_uint = 0x1F;
pub const HORIZONTAL_SYNC_WIDTH_H: c_uint = 0x20  /* Bit[7:4] are reserved */;
pub const HORIZONTAL_BACK_PORCH_L: c_uint = 0x21;
pub const HORIZONTAL_BACK_PORCH_H: c_uint = 0x22  /* Bit[7:4] are reserved */;
// END of I2C Address 0x72
//
// Register definition of device address 0x7a
pub const DP_TX_SWING_REG_CNT: c_uint = 0x14;
pub const DP_TX_LANE0_SWING_REG0: c_uint = 0x00;
pub const DP_TX_LANE1_SWING_REG0: c_uint = 0x14;
// END of I2C Address 0x7a
//
// Register definition of device address 0x7e
pub const I2C_ADDR_7E_FLASH_CONTROLLER: c_uint = 0x7E;
pub const R_BOOT_RETRY: c_uint = 0x00;
pub const R_RAM_ADDR_H: c_uint = 0x01;
pub const R_RAM_ADDR_L: c_uint = 0x02;
pub const R_RAM_LEN_H: c_uint = 0x03;
pub const R_RAM_LEN_L: c_uint = 0x04;
pub const FLASH_LOAD_STA: c_uint = 0x05;

pub const R_RAM_CTRL: c_uint = 0x05;
// bit positions

pub const FLASH_ADDR_HIGH: c_uint = 0x0F;
pub const FLASH_ADDR_LOW: c_uint = 0x10;
pub const FLASH_LEN_HIGH: c_uint = 0x31;
pub const FLASH_LEN_LOW: c_uint = 0x32;
pub const R_FLASH_RW_CTRL: c_uint = 0x33;
// bit positions

pub const FLASH_BUF_BASE_ADDR: c_uint = 0x60;
pub const FLASH_BUF_LEN: c_uint = 0x20;
pub const XTAL_FRQ_SEL: c_uint = 0x3F;
// bit field positions
pub const XTAL_FRQ_SEL_POS: c_int = 5;
// bit field values

pub const R_DSC_CTRL_0: c_uint = 0x40;
pub const READ_STATUS_EN: c_int = 7;

pub const DSC_EN: c_uint = 0x01  /* 1=DSC enabled, 0=DSC disabled */;
pub const OCM_FW_VERSION: c_uint = 0x31;
pub const OCM_FW_REVERSION: c_uint = 0x32;
pub const AP_AUX_ADDR_7_0: c_uint = 0x11;
pub const AP_AUX_ADDR_15_8: c_uint = 0x12;
pub const AP_AUX_ADDR_19_16: c_uint = 0x13;
// Bit[0:3] AUX status, bit 4 op_en, bit 5 address only
pub const AP_AUX_CTRL_STATUS: c_uint = 0x14;
pub const AP_AUX_CTRL_OP_EN: c_uint = 0x10;
pub const AP_AUX_CTRL_ADDRONLY: c_uint = 0x20;
pub const AP_AUX_BUFF_START: c_uint = 0x15;
pub const PIXEL_CLOCK_L: c_uint = 0x25;
pub const PIXEL_CLOCK_H: c_uint = 0x26;
pub const AP_AUX_COMMAND: c_uint = 0x27  /* com+len */;
pub const LENGTH_SHIFT: c_int = 4;

// Bit 0&1: 3D video structure
// 0x01: frame packing,  0x02:Line alternative, 0x03:Side-by-side(full)
pub const AP_AV_STATUS: c_uint = 0x28;

pub const GPIO_CTRL_2: c_uint = 0x49;

//
// Register definition of device address 0x84
pub const MIPI_PHY_CONTROL_3: c_uint = 0x03;
pub const MIPI_HS_PWD_CLK: c_int = 7;
pub const MIPI_HS_RT_CLK: c_int = 6;
pub const MIPI_PD_CLK: c_int = 5;
pub const MIPI_CLK_RT_MANUAL_PD_EN: c_int = 4;
pub const MIPI_CLK_HS_MANUAL_PD_EN: c_int = 3;
pub const MIPI_CLK_DET_DET_BYPASS: c_int = 2;
pub const MIPI_CLK_MISS_CTRL: c_int = 1;
pub const MIPI_PD_LPTX_CH_MANUAL_PD_EN: c_int = 0;
pub const MIPI_LANE_CTRL_0: c_uint = 0x05;
pub const MIPI_TIME_HS_PRPR: c_uint = 0x08;
//
// After MIPI RX protocol layer received video frames,
// Protocol layer starts to reconstruct video stream from PHY
//
pub const MIPI_VIDEO_STABLE_CNT: c_uint = 0x0A;
pub const MIPI_LANE_CTRL_10: c_uint = 0x0F;
pub const MIPI_DIGITAL_ADJ_1: c_uint = 0x1B;
pub const IVO_MID: c_uint = 0x26CF;
pub const MIPI_PLL_M_NUM_23_16: c_uint = 0x1E;
pub const MIPI_PLL_M_NUM_15_8: c_uint = 0x1F;
pub const MIPI_PLL_M_NUM_7_0: c_uint = 0x20;
pub const MIPI_PLL_N_NUM_23_16: c_uint = 0x21;
pub const MIPI_PLL_N_NUM_15_8: c_uint = 0x22;
pub const MIPI_PLL_N_NUM_7_0: c_uint = 0x23;
pub const MIPI_DIGITAL_PLL_6: c_uint = 0x2A;
// Bit[7:6]: VCO band control, only effective
pub const MIPI_M_NUM_READY: c_uint = 0x10;
pub const MIPI_N_NUM_READY: c_uint = 0x08;
pub const STABLE_INTEGER_CNT_EN: c_uint = 0x04;
pub const MIPI_PLL_TEST_BIT: c_int = 0;
// Bit[1:0]: test point output select -
// 00: VCO power, 01: dvdd_pdt, 10: dvdd, 11: vcox
pub const MIPI_DIGITAL_PLL_7: c_uint = 0x2B;
pub const MIPI_PLL_FORCE_N_EN: c_int = 7;
pub const MIPI_PLL_FORCE_BAND_EN: c_int = 6;
pub const MIPI_PLL_VCO_TUNE_REG: c_int = 4;
// Bit[5:4]: VCO metal capacitance -
// 00: +20% fast, 01: +10% fast (default), 10: typical, 11: -10% slow
pub const MIPI_PLL_VCO_TUNE_REG_VAL: c_uint = 0x30;
pub const MIPI_PLL_PLL_LDO_BIT: c_int = 2;
// Bit[3:2]: vco_v2i power -
// 00: 1.40V, 01: 1.45V (default), 10: 1.50V, 11: 1.55V
pub const MIPI_PLL_RESET_N: c_uint = 0x02;
pub const MIPI_FRQ_FORCE_NDET: c_int = 0;
pub const MIPI_ALERT_CLR_0: c_uint = 0x2D;
pub const HS_link_error_clear: c_int = 7;
// This bit itself is S/C, and it clears 0x84:0x31[7]
pub const MIPI_ALERT_OUT_0: c_uint = 0x31;
pub const check_sum_err_hs_sync: c_int = 7;
// This bit is cleared by 0x84:0x2D[7]
pub const MIPI_DIGITAL_PLL_8: c_uint = 0x33;
pub const MIPI_POST_DIV_VAL: c_int = 4;
// N means divided by (n+1), n = 0~15
pub const MIPI_EN_LOCK_FRZ: c_int = 3;
pub const MIPI_FRQ_COUNTER_RST: c_int = 2;
pub const MIPI_FRQ_SET_REG_8: c_int = 1;
// Bit 0 is reserved
pub const MIPI_DIGITAL_PLL_9: c_uint = 0x34;
pub const MIPI_DIGITAL_PLL_16: c_uint = 0x3B;
pub const MIPI_FRQ_FREEZE_NDET: c_int = 7;
pub const MIPI_FRQ_REG_SET_ENABLE: c_int = 6;
pub const MIPI_REG_FORCE_SEL_EN: c_int = 5;
pub const MIPI_REG_SEL_DIV_REG: c_int = 4;
pub const MIPI_REG_FORCE_PRE_DIV_EN: c_int = 3;
// Bit 2 is reserved
pub const MIPI_FREF_D_IND: c_int = 1;
pub const REF_CLK_27000KHZ: c_int = 1;
pub const REF_CLK_19200KHZ: c_int = 0;
pub const MIPI_REG_PLL_PLL_TEST_ENABLE: c_int = 0;
pub const MIPI_DIGITAL_PLL_18: c_uint = 0x3D;
pub const FRQ_COUNT_RB_SEL: c_int = 7;
pub const REG_FORCE_POST_DIV_EN: c_int = 6;
pub const MIPI_DPI_SELECT: c_int = 5;
pub const SELECT_DSI: c_int = 1;
pub const SELECT_DPI: c_int = 0;
pub const REG_BAUD_DIV_RATIO: c_int = 0;
pub const H_BLANK_L: c_uint = 0x3E;
// For DSC only
pub const H_BLANK_H: c_uint = 0x3F;
// For DSC only; note: bit[7:6] are reserved
pub const MIPI_SWAP: c_uint = 0x4A;
pub const MIPI_SWAP_CH0: c_int = 7;
pub const MIPI_SWAP_CH1: c_int = 6;
pub const MIPI_SWAP_CH2: c_int = 5;
pub const MIPI_SWAP_CH3: c_int = 4;
pub const MIPI_SWAP_CLK: c_int = 3;
// Bit[2:0] are reserved
// END of I2C Address 0x84
// DPCD regs
pub const DPCD_DPCD_REV: c_uint = 0x00;
pub const DPCD_MAX_LINK_RATE: c_uint = 0x01;
pub const DPCD_MAX_LANE_COUNT: c_uint = 0x02;
// ANX7625 Register End
// Display
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum audio_fs {
    AUDIO_FS_441K  = 0x00,
    AUDIO_FS_48K   = 0x02,
    AUDIO_FS_32K   = 0x03,
    AUDIO_FS_882K  = 0x08,
    AUDIO_FS_96K   = 0x0a,
    AUDIO_FS_1764K = 0x0c,
    AUDIO_FS_192K  = 0x0e
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum audio_wd_len {
    AUDIO_W_LEN_16_20MAX = 0x02,
    AUDIO_W_LEN_18_20MAX = 0x04,
    AUDIO_W_LEN_17_20MAX = 0x0c,
    AUDIO_W_LEN_19_20MAX = 0x08,
    AUDIO_W_LEN_20_20MAX = 0x0a,
    AUDIO_W_LEN_20_24MAX = 0x03,
    AUDIO_W_LEN_22_24MAX = 0x05,
    AUDIO_W_LEN_21_24MAX = 0x0d,
    AUDIO_W_LEN_23_24MAX = 0x09,
    AUDIO_W_LEN_24_24MAX = 0x0b
}

pub const I2S_CH_2: c_uint = 0x01;
pub const TDM_CH_4: c_uint = 0x03;
pub const TDM_CH_6: c_uint = 0x05;
pub const TDM_CH_8: c_uint = 0x07;
pub const MAX_DPCD_BUFFER_SIZE: c_int = 16;
pub const ONE_BLOCK_SIZE: c_int = 128;

pub const MAX_EDID_BLOCK: c_int = 3;
pub const EDID_TRY_CNT: c_int = 3;
pub const SUPPORT_PIXEL_CLOCK: c_int = 300000;
// Display End
pub const MAX_LANES_SUPPORT: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct anx7625_platform_data {
    pub gpio_p_on: *mut gpio_desc,
    pub gpio_reset: *mut gpio_desc,
    pub supplies: [regulator_bulk_data; 3],
    pub panel_bridge: *mut drm_bridge,
    pub intp_irq: c_int,
    pub is_dpi: c_int,
    pub mipi_lanes: c_int,
    pub audio_en: c_int,
    pub dp_lane0_swing_reg_cnt: c_int,
    pub lane0_reg_data: [u8; DP_TX_SWING_REG_CNT],
    pub dp_lane1_swing_reg_cnt: c_int,
    pub lane1_reg_data: [u8; DP_TX_SWING_REG_CNT],
    pub low_power_mode: u32,
    pub mipi_host_node: *mut device_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct anx7625_i2c_client {
    pub tx_p0_client: *mut i2c_client,
    pub tx_p1_client: *mut i2c_client,
    pub tx_p2_client: *mut i2c_client,
    pub rx_p0_client: *mut i2c_client,
    pub rx_p1_client: *mut i2c_client,
    pub rx_p2_client: *mut i2c_client,
    pub tcpc_client: *mut i2c_client,
}

pub const MAX_BUF_LEN: c_int = 30;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_msg {
    pub msg_len: u8,
    pub msg_type: u8,
    pub buf: [u8; MAX_BUF_LEN],
    pub __packed: },
pub const HEADER_LEN: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct anx7625_data {
    pub pdata: anx7625_platform_data,
    pub audio_pdev: *mut platform_device,
    pub typec_port: *mut typec_port,
    pub role_sw: *mut usb_role_switch,
    pub typec_data_role: c_int,
    pub hpd_status: c_int,
    pub hpd_high_cnt: c_int,
    pub dp_en: c_int,
    pub hdcp_cp: c_int,
// Lock for work queue
    pub lock: mutex,
    pub dev: *mut device,
    pub i2c: anx7625_i2c_client,
    pub last_client: *mut i2c_client,
    pub hdcp_timer: timer_list,
    pub cached_drm_edid: *const drm_edid,
    pub codec_dev: *mut device,
    pub plugged_cb: hdmi_codec_plugged_cb,
    pub work: work_struct,
    pub workqueue: *mut workqueue_struct,
    pub hdcp_work: delayed_work,
    pub hdcp_workqueue: *mut workqueue_struct,
// Lock for hdcp work queue
    pub hdcp_wq_lock: mutex,
// Lock for aux transfer and disable
    pub aux_lock: mutex,
    pub edid_block: c_char,
    pub dt: display_timing,
    pub display_timing_valid: u8,
    pub bridge: drm_bridge,
    pub bridge_attached: u8,
    pub connector: *mut drm_connector,
    pub dsi: *mut mipi_dsi_device,
    pub aux: drm_dp_aux,
    pub send_msg: fw_msg,
}
