//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/synopsys/hdmirx/snps_hdmirx.h
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
// Copyright (c) 2021 Rockchip Electronics Co. Ltd.
//
// Author: Dingxian Wen <shawn.wen@rock-chips.com>
//

// SYS_GRF
pub const SYS_GRF_SOC_CON1: c_uint = 0x0304;

pub const SYS_GRF_SOC_STATUS1: c_uint = 0x0384;

pub const SYS_GRF_CHIP_ID: c_uint = 0x0600;
// VO1_GRF
pub const VO1_GRF_VO1_CON2: c_uint = 0x0008;

// HDMIRX PHY
pub const SUP_DIG_ANA_CREGS_SUP_ANA_NC: c_uint = 0x004f;
pub const LANE0_DIG_ASIC_RX_OVRD_OUT_0: c_uint = 0x100f;
pub const LANE1_DIG_ASIC_RX_OVRD_OUT_0: c_uint = 0x110f;
pub const LANE2_DIG_ASIC_RX_OVRD_OUT_0: c_uint = 0x120f;
pub const LANE3_DIG_ASIC_RX_OVRD_OUT_0: c_uint = 0x130f;

pub const LANE0_DIG_RX_VCOCAL_RX_VCO_CAL_CTRL_2: c_uint = 0x104a;
pub const LANE1_DIG_RX_VCOCAL_RX_VCO_CAL_CTRL_2: c_uint = 0x114a;
pub const LANE2_DIG_RX_VCOCAL_RX_VCO_CAL_CTRL_2: c_uint = 0x124a;
pub const LANE3_DIG_RX_VCOCAL_RX_VCO_CAL_CTRL_2: c_uint = 0x134a;

pub const HDMIPCS_DIG_CTRL_PATH_MAIN_FSM_FSM_CONFIG: c_uint = 0x20c4;
pub const HDMIPCS_DIG_CTRL_PATH_MAIN_FSM_ADAPT_REF_FOM: c_uint = 0x20c7;
pub const HDMIPCS_DIG_CTRL_PATH_MAIN_FSM_RATE_CALC_HDMI14_CDR_SETTING_3_REG: c_uint = 0x20e9;
pub const CDR_SETTING_BOUNDARY_3_DEFAULT: c_uint = 0x52da;
pub const HDMIPCS_DIG_CTRL_PATH_MAIN_FSM_RATE_CALC_HDMI14_CDR_SETTING_4_REG: c_uint = 0x20ea;
pub const CDR_SETTING_BOUNDARY_4_DEFAULT: c_uint = 0x43cd;
pub const HDMIPCS_DIG_CTRL_PATH_MAIN_FSM_RATE_CALC_HDMI14_CDR_SETTING_5_REG: c_uint = 0x20eb;
pub const CDR_SETTING_BOUNDARY_5_DEFAULT: c_uint = 0x35b3;
pub const HDMIPCS_DIG_CTRL_PATH_MAIN_FSM_RATE_CALC_HDMI14_CDR_SETTING_6_REG: c_uint = 0x20fb;
pub const CDR_SETTING_BOUNDARY_6_DEFAULT: c_uint = 0x2799;
pub const HDMIPCS_DIG_CTRL_PATH_MAIN_FSM_RATE_CALC_HDMI14_CDR_SETTING_7_REG: c_uint = 0x20fc;
pub const CDR_SETTING_BOUNDARY_7_DEFAULT: c_uint = 0x1b65;
pub const RAWLANE0_DIG_PCS_XF_RX_OVRD_OUT: c_uint = 0x300e;
pub const RAWLANE1_DIG_PCS_XF_RX_OVRD_OUT: c_uint = 0x310e;
pub const RAWLANE2_DIG_PCS_XF_RX_OVRD_OUT: c_uint = 0x320e;
pub const RAWLANE3_DIG_PCS_XF_RX_OVRD_OUT: c_uint = 0x330e;

pub const RAWLANE0_DIG_AON_FAST_FLAGS: c_uint = 0x305c;
pub const RAWLANE1_DIG_AON_FAST_FLAGS: c_uint = 0x315c;
pub const RAWLANE2_DIG_AON_FAST_FLAGS: c_uint = 0x325c;
pub const RAWLANE3_DIG_AON_FAST_FLAGS: c_uint = 0x335c;
// HDMIRX Ctrler
pub const GLOBAL_SWRESET_REQUEST: c_uint = 0x0020;

pub const GLOBAL_SWENABLE: c_uint = 0x0024;

pub const GLOBAL_TIMER_REF_BASE: c_uint = 0x0028;
pub const CORE_CONFIG: c_uint = 0x0050;
pub const CMU_CONFIG0: c_uint = 0x0060;

pub const CMU_STATUS: c_uint = 0x007c;

pub const CMU_TMDSQPCLK_FREQ: c_uint = 0x0084;
pub const PHY_CONFIG: c_uint = 0x00c0;

pub const PHY_STATUS: c_uint = 0x00c8;

pub const PHYCREG_CONFIG0: c_uint = 0x00e0;

pub const PHYCREG_CONFIG1: c_uint = 0x00e4;
pub const PHYCREG_CONFIG2: c_uint = 0x00e8;
pub const PHYCREG_CONFIG3: c_uint = 0x00ec;
pub const PHYCREG_CONTROL: c_uint = 0x00f0;

pub const PHYCREG_STATUS: c_uint = 0x00f4;

pub const MAINUNIT_STATUS: c_uint = 0x0150;

pub const DESCRAND_EN_CONTROL: c_uint = 0x0210;

pub const DESCRAND_SYNC_CONTROL: c_uint = 0x0214;

pub const DESCRAND_SYNC_SEQ_CONFIG: c_uint = 0x022c;

pub const DESCRAND_SYNC_SEQ_STATUS: c_uint = 0x0234;
pub const DEFRAMER_CONFIG0: c_uint = 0x0270;

pub const DEFRAMER_VSYNC_CNT_CLEAR: c_uint = 0x0278;

pub const DEFRAMER_STATUS: c_uint = 0x027c;

pub const I2C_SLAVE_CONFIG1: c_uint = 0x0164;

pub const HDCP_INT_CLEAR: c_uint = 0x50d8;
pub const HDCP_1_INT_CLEAR: c_uint = 0x50e8;
pub const HDCP2_CONFIG: c_uint = 0x02f0;

pub const VIDEO_CONFIG2: c_uint = 0x042c;

pub const SCDC_CONFIG: c_uint = 0x0580;

pub const SCDC_REGBANK_STATUS1: c_uint = 0x058c;

pub const SCDC_REGBANK_STATUS3: c_uint = 0x0594;
pub const SCDC_REGBANK_CONFIG0: c_uint = 0x05c0;

pub const CED_CONFIG: c_uint = 0x0760;

pub const CED_DYN_CONFIG: c_uint = 0x0768;
pub const CED_DYN_CONTROL: c_uint = 0x076c;
pub const PKTEX_BCH_ERRFILT_CONFIG: c_uint = 0x07c4;
pub const PKTEX_CHKSUM_ERRFILT_CONFIG: c_uint = 0x07c8;
pub const PKTDEC_ACR_PH2_1: c_uint = 0x1100;
pub const PKTDEC_ACR_PB3_0: c_uint = 0x1104;
pub const PKTDEC_ACR_PB7_4: c_uint = 0x1108;
pub const PKTDEC_AVIIF_PH2_1: c_uint = 0x1200;
pub const PKTDEC_AVIIF_PB3_0: c_uint = 0x1204;
pub const PKTDEC_AVIIF_PB7_4: c_uint = 0x1208;

pub const PKTDEC_AVIIF_PB11_8: c_uint = 0x120c;
pub const PKTDEC_AVIIF_PB15_12: c_uint = 0x1210;
pub const PKTDEC_AVIIF_PB19_16: c_uint = 0x1214;
pub const PKTDEC_AVIIF_PB23_20: c_uint = 0x1218;
pub const PKTDEC_AVIIF_PB27_24: c_uint = 0x121c;
pub const PKTFIFO_CONFIG: c_uint = 0x1500;
pub const PKTFIFO_STORE_FILT_CONFIG: c_uint = 0x1504;
pub const PKTFIFO_THR_CONFIG0: c_uint = 0x1508;
pub const PKTFIFO_THR_CONFIG1: c_uint = 0x150c;
pub const PKTFIFO_CONTROL: c_uint = 0x1510;
pub const VMON_STATUS1: c_uint = 0x1580;
pub const VMON_STATUS2: c_uint = 0x1584;
pub const VMON_STATUS3: c_uint = 0x1588;
pub const VMON_STATUS4: c_uint = 0x158c;
pub const VMON_STATUS5: c_uint = 0x1590;
pub const VMON_STATUS6: c_uint = 0x1594;
pub const VMON_STATUS7: c_uint = 0x1598;

pub const CEC_TX_CONTROL: c_uint = 0x2000;
pub const CEC_STATUS: c_uint = 0x2004;
pub const CEC_CONFIG: c_uint = 0x2008;

pub const CEC_ADDR: c_uint = 0x200c;
pub const CEC_TX_COUNT: c_uint = 0x2020;
pub const CEC_TX_DATA3_0: c_uint = 0x2024;
pub const CEC_RX_COUNT_STATUS: c_uint = 0x2040;
pub const CEC_RX_DATA3_0: c_uint = 0x2044;
pub const CEC_LOCK_CONTROL: c_uint = 0x2054;
pub const CEC_RXQUAL_BITTIME_CONFIG: c_uint = 0x2060;
pub const CEC_RX_BITTIME_CONFIG: c_uint = 0x2064;
pub const CEC_TX_BITTIME_CONFIG: c_uint = 0x2068;
pub const DMA_CONFIG1: c_uint = 0x4400;

pub const DMA_CONFIG2: c_uint = 0x4404;
pub const DMA_CONFIG3: c_uint = 0x4408;
pub const DMA_CONFIG4: c_uint = 0x440c // dma irq en;
pub const DMA_CONFIG5: c_uint = 0x4410 // dma irq clear status;

pub const DMA_CONFIG6: c_uint = 0x4414;

pub const DMA_CONFIG7: c_uint = 0x4418;

pub const DMA_CONFIG8: c_uint = 0x441c;

pub const DMA_CONFIG9: c_uint = 0x4420;
pub const DMA_CONFIG10: c_uint = 0x4424;
pub const DMA_CONFIG11: c_uint = 0x4428;

pub const DMA_STATUS1: c_uint = 0x4430 // dma irq status;
pub const DMA_STATUS2: c_uint = 0x4434;
pub const DMA_STATUS3: c_uint = 0x4438;
pub const DMA_STATUS4: c_uint = 0x443c;
pub const DMA_STATUS5: c_uint = 0x4440;
pub const DMA_STATUS6: c_uint = 0x4444;
pub const DMA_STATUS7: c_uint = 0x4448;
pub const DMA_STATUS8: c_uint = 0x444c;
pub const DMA_STATUS9: c_uint = 0x4450;
pub const DMA_STATUS10: c_uint = 0x4454;

pub const DMA_STATUS11: c_uint = 0x4458;

pub const DMA_STATUS12: c_uint = 0x445c;
pub const DMA_STATUS13: c_uint = 0x4460;
pub const DMA_STATUS14: c_uint = 0x4464;
pub const MAINUNIT_INTVEC_INDEX: c_uint = 0x5000;
pub const MAINUNIT_0_INT_STATUS: c_uint = 0x5010;

pub const MAINUNIT_0_INT_MASK_N: c_uint = 0x5014;
pub const MAINUNIT_0_INT_CLEAR: c_uint = 0x5018;
pub const MAINUNIT_0_INT_FORCE: c_uint = 0x501c;

pub const MAINUNIT_1_INT_STATUS: c_uint = 0x5020;
pub const MAINUNIT_1_INT_MASK_N: c_uint = 0x5024;
pub const MAINUNIT_1_INT_CLEAR: c_uint = 0x5028;
pub const MAINUNIT_1_INT_FORCE: c_uint = 0x502c;
pub const MAINUNIT_2_INT_STATUS: c_uint = 0x5030;
pub const MAINUNIT_2_INT_MASK_N: c_uint = 0x5034;
pub const MAINUNIT_2_INT_CLEAR: c_uint = 0x5038;
pub const MAINUNIT_2_INT_FORCE: c_uint = 0x503c;

pub const AVPUNIT_0_INT_STATUS: c_uint = 0x5040;
pub const AVPUNIT_0_INT_MASK_N: c_uint = 0x5044;
pub const AVPUNIT_0_INT_CLEAR: c_uint = 0x5048;
pub const AVPUNIT_0_INT_FORCE: c_uint = 0x504c;

pub const AVPUNIT_1_INT_STATUS: c_uint = 0x5050;

pub const AVPUNIT_1_INT_MASK_N: c_uint = 0x5054;

pub const AVPUNIT_1_INT_CLEAR: c_uint = 0x5058;

pub const PKT_0_INT_STATUS: c_uint = 0x5080;

pub const PKT_0_INT_MASK_N: c_uint = 0x5084;

pub const PKT_0_INT_CLEAR: c_uint = 0x5088;
pub const PKT_1_INT_STATUS: c_uint = 0x5090;
pub const PKT_1_INT_MASK_N: c_uint = 0x5094;
pub const PKT_1_INT_CLEAR: c_uint = 0x5098;
pub const PKT_2_INT_STATUS: c_uint = 0x50a0;

pub const PKT_2_INT_MASK_N: c_uint = 0x50a4;

pub const PKT_2_INT_CLEAR: c_uint = 0x50a8;

pub const SCDC_INT_STATUS: c_uint = 0x50c0;
pub const SCDC_INT_MASK_N: c_uint = 0x50c4;
pub const SCDC_INT_CLEAR: c_uint = 0x50c8;

pub const CEC_INT_STATUS: c_uint = 0x5100;
pub const CEC_INT_MASK_N: c_uint = 0x5104;
pub const CEC_INT_CLEAR: c_uint = 0x5108;
