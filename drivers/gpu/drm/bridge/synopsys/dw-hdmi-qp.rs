//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/bridge/synopsys/dw-hdmi-qp.h
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
// Copyright (C) Rockchip Electronics Co., Ltd.
// Author:
// Algea Cao <algea.cao@rock-chips.com>
//

// Main Unit Registers
pub const CORE_ID: c_uint = 0x0;
pub const VER_NUMBER: c_uint = 0x4;
pub const VER_TYPE: c_uint = 0x8;
pub const CONFIG_REG: c_uint = 0xc;

pub const CORE_TIMESTAMP_HHMM: c_uint = 0x14;
pub const CORE_TIMESTAMP_MMDD: c_uint = 0x18;
pub const CORE_TIMESTAMP_YYYY: c_uint = 0x1c;
// Reset Manager Registers
pub const GLOBAL_SWRESET_REQUEST: c_uint = 0x40;

pub const GLOBAL_SWDISABLE: c_uint = 0x44;

pub const RESET_MANAGER_CONFIG0: c_uint = 0x48;
pub const RESET_MANAGER_STATUS0: c_uint = 0x50;
pub const RESET_MANAGER_STATUS1: c_uint = 0x54;
pub const RESET_MANAGER_STATUS2: c_uint = 0x58;
// Timer Base Registers
pub const TIMER_BASE_CONFIG0: c_uint = 0x80;
pub const TIMER_BASE_STATUS0: c_uint = 0x84;
// CMU Registers
pub const CMU_CONFIG0: c_uint = 0xa0;
pub const CMU_CONFIG1: c_uint = 0xa4;
pub const CMU_CONFIG2: c_uint = 0xa8;
pub const CMU_CONFIG3: c_uint = 0xac;
pub const CMU_STATUS: c_uint = 0xb0;
pub const DISPLAY_CLK_MONITOR: c_uint = 0x3f;

pub const CMU_IPI_CLK_FREQ: c_uint = 0xb4;
pub const CMU_VIDQPCLK_FREQ: c_uint = 0xb8;
pub const CMU_LINKQPCLK_FREQ: c_uint = 0xbc;
pub const CMU_AUDQPCLK_FREQ: c_uint = 0xc0;
pub const CMU_EARC_BPCLK_FREQ: c_uint = 0xc4;
// I2CM Registers
pub const I2CM_SM_SCL_CONFIG0: c_uint = 0xe0;
pub const I2CM_FM_SCL_CONFIG0: c_uint = 0xe4;
pub const I2CM_CONFIG0: c_uint = 0xe8;
pub const I2CM_CONTROL0: c_uint = 0xec;
pub const I2CM_STATUS0: c_uint = 0xf0;
pub const I2CM_INTERFACE_CONTROL0: c_uint = 0xf4;
pub const I2CM_ADDR: c_uint = 0xff000;
pub const I2CM_SLVADDR: c_uint = 0xfe0;
pub const I2CM_WR_MASK: c_uint = 0x1e;

pub const I2CM_INTERFACE_CONTROL1: c_uint = 0xf8;
pub const I2CM_SEG_PTR: c_uint = 0x7f80;
pub const I2CM_SEG_ADDR: c_uint = 0x7f;
pub const I2CM_INTERFACE_WRDATA_0_3: c_uint = 0xfc;
pub const I2CM_INTERFACE_WRDATA_4_7: c_uint = 0x100;
pub const I2CM_INTERFACE_WRDATA_8_11: c_uint = 0x104;
pub const I2CM_INTERFACE_WRDATA_12_15: c_uint = 0x108;
pub const I2CM_INTERFACE_RDDATA_0_3: c_uint = 0x10c;
pub const I2CM_INTERFACE_RDDATA_4_7: c_uint = 0x110;
pub const I2CM_INTERFACE_RDDATA_8_11: c_uint = 0x114;
pub const I2CM_INTERFACE_RDDATA_12_15: c_uint = 0x118;
// SCDC Registers
pub const SCDC_CONFIG0: c_uint = 0x140;

pub const SCDC_CONTROL0: c_uint = 0x148;
pub const SCDC_STATUS0: c_uint = 0x150;

// FLT Registers
pub const FLT_CONFIG0: c_uint = 0x160;
pub const FLT_CONFIG1: c_uint = 0x164;
pub const FLT_CONFIG2: c_uint = 0x168;
pub const FLT_CONTROL0: c_uint = 0x170;
// Main Unit 2 Registers
pub const MAINUNIT_STATUS0: c_uint = 0x180;
// Video Interface Registers
pub const VIDEO_INTERFACE_CONFIG0: c_uint = 0x800;
pub const VIDEO_INTERFACE_CONFIG1: c_uint = 0x804;
pub const VIDEO_INTERFACE_CONFIG2: c_uint = 0x808;
pub const VIDEO_INTERFACE_CONTROL0: c_uint = 0x80c;
pub const VIDEO_INTERFACE_STATUS0: c_uint = 0x814;
// Video Packing Registers
pub const VIDEO_PACKING_CONFIG0: c_uint = 0x81c;
// Audio Interface Registers
pub const AUDIO_INTERFACE_CONFIG0: c_uint = 0x820;
pub const AUD_IF_SEL_MSK: c_uint = 0x3;
pub const AUD_IF_SPDIF: c_uint = 0x2;
pub const AUD_IF_I2S: c_uint = 0x1;
pub const AUD_IF_PAI: c_uint = 0x0;

pub const I2S_BPCUV_RCV_DIS: c_int = 0;

pub const AUDIO_INTERFACE_CONFIG1: c_uint = 0x824;
pub const AUDIO_INTERFACE_CONTROL0: c_uint = 0x82c;

pub const AUDIO_INTERFACE_STATUS0: c_uint = 0x834;
// Frame Composer Registers
pub const FRAME_COMPOSER_CONFIG0: c_uint = 0x840;
pub const FRAME_COMPOSER_CONFIG1: c_uint = 0x844;
pub const FRAME_COMPOSER_CONFIG2: c_uint = 0x848;
pub const FRAME_COMPOSER_CONFIG3: c_uint = 0x84c;
pub const FRAME_COMPOSER_CONFIG4: c_uint = 0x850;
pub const FRAME_COMPOSER_CONFIG5: c_uint = 0x854;
pub const FRAME_COMPOSER_CONFIG6: c_uint = 0x858;
pub const FRAME_COMPOSER_CONFIG7: c_uint = 0x85c;
pub const FRAME_COMPOSER_CONFIG8: c_uint = 0x860;
pub const FRAME_COMPOSER_CONFIG9: c_uint = 0x864;
pub const FRAME_COMPOSER_CONTROL0: c_uint = 0x86c;
// Video Monitor Registers
pub const VIDEO_MONITOR_CONFIG0: c_uint = 0x880;
pub const VIDEO_MONITOR_STATUS0: c_uint = 0x884;
pub const VIDEO_MONITOR_STATUS1: c_uint = 0x888;
pub const VIDEO_MONITOR_STATUS2: c_uint = 0x88c;
pub const VIDEO_MONITOR_STATUS3: c_uint = 0x890;
pub const VIDEO_MONITOR_STATUS4: c_uint = 0x894;
pub const VIDEO_MONITOR_STATUS5: c_uint = 0x898;
pub const VIDEO_MONITOR_STATUS6: c_uint = 0x89c;
// HDCP2 Logic Registers
pub const HDCP2LOGIC_CONFIG0: c_uint = 0x8e0;

pub const HDCP2LOGIC_ESM_GPIO_IN: c_uint = 0x8e4;
pub const HDCP2LOGIC_ESM_GPIO_OUT: c_uint = 0x8e8;
// HDCP14 Registers
pub const HDCP14_CONFIG0: c_uint = 0x900;
pub const HDCP14_CONFIG1: c_uint = 0x904;
pub const HDCP14_CONFIG2: c_uint = 0x908;
pub const HDCP14_CONFIG3: c_uint = 0x90c;
pub const HDCP14_KEY_SEED: c_uint = 0x914;
pub const HDCP14_KEY_H: c_uint = 0x918;
pub const HDCP14_KEY_L: c_uint = 0x91c;
pub const HDCP14_KEY_STATUS: c_uint = 0x920;
pub const HDCP14_AKSV_H: c_uint = 0x924;
pub const HDCP14_AKSV_L: c_uint = 0x928;
pub const HDCP14_AN_H: c_uint = 0x92c;
pub const HDCP14_AN_L: c_uint = 0x930;
pub const HDCP14_STATUS0: c_uint = 0x934;
pub const HDCP14_STATUS1: c_uint = 0x938;
// Scrambler Registers
pub const SCRAMB_CONFIG0: c_uint = 0x960;
// Video Configuration Registers
pub const LINK_CONFIG0: c_uint = 0x968;

// TMDS FIFO Registers
pub const TMDS_FIFO_CONFIG0: c_uint = 0x970;
pub const TMDS_FIFO_CONTROL0: c_uint = 0x974;
// FRL RSFEC Registers
pub const FRL_RSFEC_CONFIG0: c_uint = 0xa20;
pub const FRL_RSFEC_STATUS0: c_uint = 0xa30;
// FRL Packetizer Registers
pub const FRL_PKTZ_CONFIG0: c_uint = 0xa40;
pub const FRL_PKTZ_CONTROL0: c_uint = 0xa44;
pub const FRL_PKTZ_CONTROL1: c_uint = 0xa50;
pub const FRL_PKTZ_STATUS1: c_uint = 0xa54;
// Packet Scheduler Registers
pub const PKTSCHED_CONFIG0: c_uint = 0xa80;
pub const PKTSCHED_PRQUEUE0_CONFIG0: c_uint = 0xa84;
pub const PKTSCHED_PRQUEUE1_CONFIG0: c_uint = 0xa88;
pub const PKTSCHED_PRQUEUE2_CONFIG0: c_uint = 0xa8c;
pub const PKTSCHED_PRQUEUE2_CONFIG1: c_uint = 0xa90;
pub const PKTSCHED_PRQUEUE2_CONFIG2: c_uint = 0xa94;
pub const PKTSCHED_PKT_CONFIG0: c_uint = 0xa98;
pub const PKTSCHED_PKT_CONFIG1: c_uint = 0xa9c;

pub const PKTSCHED_PKT_CONFIG2: c_uint = 0xaa0;
pub const PKTSCHED_PKT_CONFIG3: c_uint = 0xaa4;
pub const PKTSCHED_PKT_EN: c_uint = 0xaa8;

pub const PKTSCHED_PKT_CONTROL0: c_uint = 0xaac;
pub const PKTSCHED_PKT_SEND: c_uint = 0xab0;
pub const PKTSCHED_PKT_STATUS0: c_uint = 0xab4;
pub const PKTSCHED_PKT_STATUS1: c_uint = 0xab8;
pub const PKT_NULL_CONTENTS0: c_uint = 0xb00;
pub const PKT_NULL_CONTENTS1: c_uint = 0xb04;
pub const PKT_NULL_CONTENTS2: c_uint = 0xb08;
pub const PKT_NULL_CONTENTS3: c_uint = 0xb0c;
pub const PKT_NULL_CONTENTS4: c_uint = 0xb10;
pub const PKT_NULL_CONTENTS5: c_uint = 0xb14;
pub const PKT_NULL_CONTENTS6: c_uint = 0xb18;
pub const PKT_NULL_CONTENTS7: c_uint = 0xb1c;
pub const PKT_ACP_CONTENTS0: c_uint = 0xb20;
pub const PKT_ACP_CONTENTS1: c_uint = 0xb24;
pub const PKT_ACP_CONTENTS2: c_uint = 0xb28;
pub const PKT_ACP_CONTENTS3: c_uint = 0xb2c;
pub const PKT_ACP_CONTENTS4: c_uint = 0xb30;
pub const PKT_ACP_CONTENTS5: c_uint = 0xb34;
pub const PKT_ACP_CONTENTS6: c_uint = 0xb38;
pub const PKT_ACP_CONTENTS7: c_uint = 0xb3c;
pub const PKT_ISRC1_CONTENTS0: c_uint = 0xb40;
pub const PKT_ISRC1_CONTENTS1: c_uint = 0xb44;
pub const PKT_ISRC1_CONTENTS2: c_uint = 0xb48;
pub const PKT_ISRC1_CONTENTS3: c_uint = 0xb4c;
pub const PKT_ISRC1_CONTENTS4: c_uint = 0xb50;
pub const PKT_ISRC1_CONTENTS5: c_uint = 0xb54;
pub const PKT_ISRC1_CONTENTS6: c_uint = 0xb58;
pub const PKT_ISRC1_CONTENTS7: c_uint = 0xb5c;
pub const PKT_ISRC2_CONTENTS0: c_uint = 0xb60;
pub const PKT_ISRC2_CONTENTS1: c_uint = 0xb64;
pub const PKT_ISRC2_CONTENTS2: c_uint = 0xb68;
pub const PKT_ISRC2_CONTENTS3: c_uint = 0xb6c;
pub const PKT_ISRC2_CONTENTS4: c_uint = 0xb70;
pub const PKT_ISRC2_CONTENTS5: c_uint = 0xb74;
pub const PKT_ISRC2_CONTENTS6: c_uint = 0xb78;
pub const PKT_ISRC2_CONTENTS7: c_uint = 0xb7c;
pub const PKT_GMD_CONTENTS0: c_uint = 0xb80;
pub const PKT_GMD_CONTENTS1: c_uint = 0xb84;
pub const PKT_GMD_CONTENTS2: c_uint = 0xb88;
pub const PKT_GMD_CONTENTS3: c_uint = 0xb8c;
pub const PKT_GMD_CONTENTS4: c_uint = 0xb90;
pub const PKT_GMD_CONTENTS5: c_uint = 0xb94;
pub const PKT_GMD_CONTENTS6: c_uint = 0xb98;
pub const PKT_GMD_CONTENTS7: c_uint = 0xb9c;
pub const PKT_AMD_CONTENTS0: c_uint = 0xba0;
pub const PKT_AMD_CONTENTS1: c_uint = 0xba4;
pub const PKT_AMD_CONTENTS2: c_uint = 0xba8;
pub const PKT_AMD_CONTENTS3: c_uint = 0xbac;
pub const PKT_AMD_CONTENTS4: c_uint = 0xbb0;
pub const PKT_AMD_CONTENTS5: c_uint = 0xbb4;
pub const PKT_AMD_CONTENTS6: c_uint = 0xbb8;
pub const PKT_AMD_CONTENTS7: c_uint = 0xbbc;
pub const PKT_VSI_CONTENTS0: c_uint = 0xbc0;
pub const PKT_VSI_CONTENTS1: c_uint = 0xbc4;
pub const PKT_VSI_CONTENTS2: c_uint = 0xbc8;
pub const PKT_VSI_CONTENTS3: c_uint = 0xbcc;
pub const PKT_VSI_CONTENTS4: c_uint = 0xbd0;
pub const PKT_VSI_CONTENTS5: c_uint = 0xbd4;
pub const PKT_VSI_CONTENTS6: c_uint = 0xbd8;
pub const PKT_VSI_CONTENTS7: c_uint = 0xbdc;
pub const PKT_AVI_CONTENTS0: c_uint = 0xbe0;

pub const HDMI_FC_AVICONF0_BAR_DATA_VERT_BAR: c_uint = 0x04;
pub const HDMI_FC_AVICONF0_BAR_DATA_HORIZ_BAR: c_uint = 0x08;
pub const HDMI_FC_AVICONF2_IT_CONTENT_VALID: c_uint = 0x80;
pub const PKT_AVI_CONTENTS1: c_uint = 0xbe4;
pub const PKT_AVI_CONTENTS2: c_uint = 0xbe8;
pub const PKT_AVI_CONTENTS3: c_uint = 0xbec;
pub const PKT_AVI_CONTENTS4: c_uint = 0xbf0;
pub const PKT_AVI_CONTENTS5: c_uint = 0xbf4;
pub const PKT_AVI_CONTENTS6: c_uint = 0xbf8;
pub const PKT_AVI_CONTENTS7: c_uint = 0xbfc;
pub const PKT_SPDI_CONTENTS0: c_uint = 0xc00;
pub const PKT_SPDI_CONTENTS1: c_uint = 0xc04;
pub const PKT_SPDI_CONTENTS2: c_uint = 0xc08;
pub const PKT_SPDI_CONTENTS3: c_uint = 0xc0c;
pub const PKT_SPDI_CONTENTS4: c_uint = 0xc10;
pub const PKT_SPDI_CONTENTS5: c_uint = 0xc14;
pub const PKT_SPDI_CONTENTS6: c_uint = 0xc18;
pub const PKT_SPDI_CONTENTS7: c_uint = 0xc1c;
pub const PKT_AUDI_CONTENTS0: c_uint = 0xc20;
pub const PKT_AUDI_CONTENTS1: c_uint = 0xc24;
pub const PKT_AUDI_CONTENTS2: c_uint = 0xc28;
pub const PKT_AUDI_CONTENTS3: c_uint = 0xc2c;
pub const PKT_AUDI_CONTENTS4: c_uint = 0xc30;
pub const PKT_AUDI_CONTENTS5: c_uint = 0xc34;
pub const PKT_AUDI_CONTENTS6: c_uint = 0xc38;
pub const PKT_AUDI_CONTENTS7: c_uint = 0xc3c;
pub const PKT_NVI_CONTENTS0: c_uint = 0xc40;
pub const PKT_NVI_CONTENTS1: c_uint = 0xc44;
pub const PKT_NVI_CONTENTS2: c_uint = 0xc48;
pub const PKT_NVI_CONTENTS3: c_uint = 0xc4c;
pub const PKT_NVI_CONTENTS4: c_uint = 0xc50;
pub const PKT_NVI_CONTENTS5: c_uint = 0xc54;
pub const PKT_NVI_CONTENTS6: c_uint = 0xc58;
pub const PKT_NVI_CONTENTS7: c_uint = 0xc5c;
pub const PKT_DRMI_CONTENTS0: c_uint = 0xc60;
pub const PKT_DRMI_CONTENTS1: c_uint = 0xc64;
pub const PKT_DRMI_CONTENTS2: c_uint = 0xc68;
pub const PKT_DRMI_CONTENTS3: c_uint = 0xc6c;
pub const PKT_DRMI_CONTENTS4: c_uint = 0xc70;
pub const PKT_DRMI_CONTENTS5: c_uint = 0xc74;
pub const PKT_DRMI_CONTENTS6: c_uint = 0xc78;
pub const PKT_DRMI_CONTENTS7: c_uint = 0xc7c;
pub const PKT_GHDMI1_CONTENTS0: c_uint = 0xc80;
pub const PKT_GHDMI1_CONTENTS1: c_uint = 0xc84;
pub const PKT_GHDMI1_CONTENTS2: c_uint = 0xc88;
pub const PKT_GHDMI1_CONTENTS3: c_uint = 0xc8c;
pub const PKT_GHDMI1_CONTENTS4: c_uint = 0xc90;
pub const PKT_GHDMI1_CONTENTS5: c_uint = 0xc94;
pub const PKT_GHDMI1_CONTENTS6: c_uint = 0xc98;
pub const PKT_GHDMI1_CONTENTS7: c_uint = 0xc9c;
pub const PKT_GHDMI2_CONTENTS0: c_uint = 0xca0;
pub const PKT_GHDMI2_CONTENTS1: c_uint = 0xca4;
pub const PKT_GHDMI2_CONTENTS2: c_uint = 0xca8;
pub const PKT_GHDMI2_CONTENTS3: c_uint = 0xcac;
pub const PKT_GHDMI2_CONTENTS4: c_uint = 0xcb0;
pub const PKT_GHDMI2_CONTENTS5: c_uint = 0xcb4;
pub const PKT_GHDMI2_CONTENTS6: c_uint = 0xcb8;
pub const PKT_GHDMI2_CONTENTS7: c_uint = 0xcbc;
// EMP Packetizer Registers
pub const PKT_EMP_CONFIG0: c_uint = 0xce0;
pub const PKT_EMP_CONTROL0: c_uint = 0xcec;
pub const PKT_EMP_CONTROL1: c_uint = 0xcf0;
pub const PKT_EMP_CONTROL2: c_uint = 0xcf4;
pub const PKT_EMP_VTEM_CONTENTS0: c_uint = 0xd00;
pub const PKT_EMP_VTEM_CONTENTS1: c_uint = 0xd04;
pub const PKT_EMP_VTEM_CONTENTS2: c_uint = 0xd08;
pub const PKT_EMP_VTEM_CONTENTS3: c_uint = 0xd0c;
pub const PKT_EMP_VTEM_CONTENTS4: c_uint = 0xd10;
pub const PKT_EMP_VTEM_CONTENTS5: c_uint = 0xd14;
pub const PKT_EMP_VTEM_CONTENTS6: c_uint = 0xd18;
pub const PKT_EMP_VTEM_CONTENTS7: c_uint = 0xd1c;
pub const PKT0_EMP_CVTEM_CONTENTS0: c_uint = 0xd20;
pub const PKT0_EMP_CVTEM_CONTENTS1: c_uint = 0xd24;
pub const PKT0_EMP_CVTEM_CONTENTS2: c_uint = 0xd28;
pub const PKT0_EMP_CVTEM_CONTENTS3: c_uint = 0xd2c;
pub const PKT0_EMP_CVTEM_CONTENTS4: c_uint = 0xd30;
pub const PKT0_EMP_CVTEM_CONTENTS5: c_uint = 0xd34;
pub const PKT0_EMP_CVTEM_CONTENTS6: c_uint = 0xd38;
pub const PKT0_EMP_CVTEM_CONTENTS7: c_uint = 0xd3c;
pub const PKT1_EMP_CVTEM_CONTENTS0: c_uint = 0xd40;
pub const PKT1_EMP_CVTEM_CONTENTS1: c_uint = 0xd44;
pub const PKT1_EMP_CVTEM_CONTENTS2: c_uint = 0xd48;
pub const PKT1_EMP_CVTEM_CONTENTS3: c_uint = 0xd4c;
pub const PKT1_EMP_CVTEM_CONTENTS4: c_uint = 0xd50;
pub const PKT1_EMP_CVTEM_CONTENTS5: c_uint = 0xd54;
pub const PKT1_EMP_CVTEM_CONTENTS6: c_uint = 0xd58;
pub const PKT1_EMP_CVTEM_CONTENTS7: c_uint = 0xd5c;
pub const PKT2_EMP_CVTEM_CONTENTS0: c_uint = 0xd60;
pub const PKT2_EMP_CVTEM_CONTENTS1: c_uint = 0xd64;
pub const PKT2_EMP_CVTEM_CONTENTS2: c_uint = 0xd68;
pub const PKT2_EMP_CVTEM_CONTENTS3: c_uint = 0xd6c;
pub const PKT2_EMP_CVTEM_CONTENTS4: c_uint = 0xd70;
pub const PKT2_EMP_CVTEM_CONTENTS5: c_uint = 0xd74;
pub const PKT2_EMP_CVTEM_CONTENTS6: c_uint = 0xd78;
pub const PKT2_EMP_CVTEM_CONTENTS7: c_uint = 0xd7c;
pub const PKT3_EMP_CVTEM_CONTENTS0: c_uint = 0xd80;
pub const PKT3_EMP_CVTEM_CONTENTS1: c_uint = 0xd84;
pub const PKT3_EMP_CVTEM_CONTENTS2: c_uint = 0xd88;
pub const PKT3_EMP_CVTEM_CONTENTS3: c_uint = 0xd8c;
pub const PKT3_EMP_CVTEM_CONTENTS4: c_uint = 0xd90;
pub const PKT3_EMP_CVTEM_CONTENTS5: c_uint = 0xd94;
pub const PKT3_EMP_CVTEM_CONTENTS6: c_uint = 0xd98;
pub const PKT3_EMP_CVTEM_CONTENTS7: c_uint = 0xd9c;
pub const PKT4_EMP_CVTEM_CONTENTS0: c_uint = 0xda0;
pub const PKT4_EMP_CVTEM_CONTENTS1: c_uint = 0xda4;
pub const PKT4_EMP_CVTEM_CONTENTS2: c_uint = 0xda8;
pub const PKT4_EMP_CVTEM_CONTENTS3: c_uint = 0xdac;
pub const PKT4_EMP_CVTEM_CONTENTS4: c_uint = 0xdb0;
pub const PKT4_EMP_CVTEM_CONTENTS5: c_uint = 0xdb4;
pub const PKT4_EMP_CVTEM_CONTENTS6: c_uint = 0xdb8;
pub const PKT4_EMP_CVTEM_CONTENTS7: c_uint = 0xdbc;
pub const PKT5_EMP_CVTEM_CONTENTS0: c_uint = 0xdc0;
pub const PKT5_EMP_CVTEM_CONTENTS1: c_uint = 0xdc4;
pub const PKT5_EMP_CVTEM_CONTENTS2: c_uint = 0xdc8;
pub const PKT5_EMP_CVTEM_CONTENTS3: c_uint = 0xdcc;
pub const PKT5_EMP_CVTEM_CONTENTS4: c_uint = 0xdd0;
pub const PKT5_EMP_CVTEM_CONTENTS5: c_uint = 0xdd4;
pub const PKT5_EMP_CVTEM_CONTENTS6: c_uint = 0xdd8;
pub const PKT5_EMP_CVTEM_CONTENTS7: c_uint = 0xddc;
// Audio Packetizer Registers
pub const AUDPKT_CONTROL0: c_uint = 0xe20;

pub const AUDPKT_CONTROL1: c_uint = 0xe24;
pub const AUDPKT_ACR_CONTROL0: c_uint = 0xe40;
pub const AUDPKT_ACR_N_VALUE: c_uint = 0xfffff;
pub const AUDPKT_ACR_CONTROL1: c_uint = 0xe44;

pub const AUDPKT_ACR_STATUS0: c_uint = 0xe4c;
pub const AUDPKT_CHSTATUS_OVR0: c_uint = 0xe60;
pub const AUDPKT_CHSTATUS_OVR1: c_uint = 0xe64;
// IEC60958 Byte 3: Sampleing frenuency Bits 24 to 27

pub const AUDPKT_CHSTATUS_SR_22050: c_uint = 0x4;
pub const AUDPKT_CHSTATUS_SR_24000: c_uint = 0x6;
pub const AUDPKT_CHSTATUS_SR_32000: c_uint = 0x3;
pub const AUDPKT_CHSTATUS_SR_44100: c_uint = 0x0;
pub const AUDPKT_CHSTATUS_SR_48000: c_uint = 0x2;
pub const AUDPKT_CHSTATUS_SR_88200: c_uint = 0x8;
pub const AUDPKT_CHSTATUS_SR_96000: c_uint = 0xa;
pub const AUDPKT_CHSTATUS_SR_176400: c_uint = 0xc;
pub const AUDPKT_CHSTATUS_SR_192000: c_uint = 0xe;
pub const AUDPKT_CHSTATUS_SR_768000: c_uint = 0x9;
pub const AUDPKT_CHSTATUS_SR_NOT_INDICATED: c_uint = 0x1;
// IEC60958 Byte 4: Original Sampleing frenuency Bits 36 to 39

pub const AUDPKT_CHSTATUS_OSR_8000: c_uint = 0x6;
pub const AUDPKT_CHSTATUS_OSR_11025: c_uint = 0xa;
pub const AUDPKT_CHSTATUS_OSR_12000: c_uint = 0x2;
pub const AUDPKT_CHSTATUS_OSR_16000: c_uint = 0x8;
pub const AUDPKT_CHSTATUS_OSR_22050: c_uint = 0xb;
pub const AUDPKT_CHSTATUS_OSR_24000: c_uint = 0x9;
pub const AUDPKT_CHSTATUS_OSR_32000: c_uint = 0xc;
pub const AUDPKT_CHSTATUS_OSR_44100: c_uint = 0xf;
pub const AUDPKT_CHSTATUS_OSR_48000: c_uint = 0xd;
pub const AUDPKT_CHSTATUS_OSR_88200: c_uint = 0x7;
pub const AUDPKT_CHSTATUS_OSR_96000: c_uint = 0x5;
pub const AUDPKT_CHSTATUS_OSR_176400: c_uint = 0x3;
pub const AUDPKT_CHSTATUS_OSR_192000: c_uint = 0x1;
pub const AUDPKT_CHSTATUS_OSR_NOT_INDICATED: c_uint = 0x0;
pub const AUDPKT_CHSTATUS_OVR2: c_uint = 0xe68;
pub const AUDPKT_CHSTATUS_OVR3: c_uint = 0xe6c;
pub const AUDPKT_CHSTATUS_OVR4: c_uint = 0xe70;
pub const AUDPKT_CHSTATUS_OVR5: c_uint = 0xe74;
pub const AUDPKT_CHSTATUS_OVR6: c_uint = 0xe78;
pub const AUDPKT_CHSTATUS_OVR7: c_uint = 0xe7c;
pub const AUDPKT_CHSTATUS_OVR8: c_uint = 0xe80;
pub const AUDPKT_CHSTATUS_OVR9: c_uint = 0xe84;
pub const AUDPKT_CHSTATUS_OVR10: c_uint = 0xe88;
pub const AUDPKT_CHSTATUS_OVR11: c_uint = 0xe8c;
pub const AUDPKT_CHSTATUS_OVR12: c_uint = 0xe90;
pub const AUDPKT_CHSTATUS_OVR13: c_uint = 0xe94;
pub const AUDPKT_CHSTATUS_OVR14: c_uint = 0xe98;
pub const AUDPKT_USRDATA_OVR_MSG_GENERIC0: c_uint = 0xea0;
pub const AUDPKT_USRDATA_OVR_MSG_GENERIC1: c_uint = 0xea4;
pub const AUDPKT_USRDATA_OVR_MSG_GENERIC2: c_uint = 0xea8;
pub const AUDPKT_USRDATA_OVR_MSG_GENERIC3: c_uint = 0xeac;
pub const AUDPKT_USRDATA_OVR_MSG_GENERIC4: c_uint = 0xeb0;
pub const AUDPKT_USRDATA_OVR_MSG_GENERIC5: c_uint = 0xeb4;
pub const AUDPKT_USRDATA_OVR_MSG_GENERIC6: c_uint = 0xeb8;
pub const AUDPKT_USRDATA_OVR_MSG_GENERIC7: c_uint = 0xebc;
pub const AUDPKT_USRDATA_OVR_MSG_GENERIC8: c_uint = 0xec0;
pub const AUDPKT_USRDATA_OVR_MSG_GENERIC9: c_uint = 0xec4;
pub const AUDPKT_USRDATA_OVR_MSG_GENERIC10: c_uint = 0xec8;
pub const AUDPKT_USRDATA_OVR_MSG_GENERIC11: c_uint = 0xecc;
pub const AUDPKT_USRDATA_OVR_MSG_GENERIC12: c_uint = 0xed0;
pub const AUDPKT_USRDATA_OVR_MSG_GENERIC13: c_uint = 0xed4;
pub const AUDPKT_USRDATA_OVR_MSG_GENERIC14: c_uint = 0xed8;
pub const AUDPKT_USRDATA_OVR_MSG_GENERIC15: c_uint = 0xedc;
pub const AUDPKT_USRDATA_OVR_MSG_GENERIC16: c_uint = 0xee0;
pub const AUDPKT_USRDATA_OVR_MSG_GENERIC17: c_uint = 0xee4;
pub const AUDPKT_USRDATA_OVR_MSG_GENERIC18: c_uint = 0xee8;
pub const AUDPKT_USRDATA_OVR_MSG_GENERIC19: c_uint = 0xeec;
pub const AUDPKT_USRDATA_OVR_MSG_GENERIC20: c_uint = 0xef0;
pub const AUDPKT_USRDATA_OVR_MSG_GENERIC21: c_uint = 0xef4;
pub const AUDPKT_USRDATA_OVR_MSG_GENERIC22: c_uint = 0xef8;
pub const AUDPKT_USRDATA_OVR_MSG_GENERIC23: c_uint = 0xefc;
pub const AUDPKT_USRDATA_OVR_MSG_GENERIC24: c_uint = 0xf00;
pub const AUDPKT_USRDATA_OVR_MSG_GENERIC25: c_uint = 0xf04;
pub const AUDPKT_USRDATA_OVR_MSG_GENERIC26: c_uint = 0xf08;
pub const AUDPKT_USRDATA_OVR_MSG_GENERIC27: c_uint = 0xf0c;
pub const AUDPKT_USRDATA_OVR_MSG_GENERIC28: c_uint = 0xf10;
pub const AUDPKT_USRDATA_OVR_MSG_GENERIC29: c_uint = 0xf14;
pub const AUDPKT_USRDATA_OVR_MSG_GENERIC30: c_uint = 0xf18;
pub const AUDPKT_USRDATA_OVR_MSG_GENERIC31: c_uint = 0xf1c;
pub const AUDPKT_USRDATA_OVR_MSG_GENERIC32: c_uint = 0xf20;
pub const AUDPKT_VBIT_OVR0: c_uint = 0xf24;
// CEC Registers
pub const CEC_TX_CONTROL: c_uint = 0x1000;

pub const CEC_STATUS: c_uint = 0x1004;

pub const CEC_CONFIG: c_uint = 0x1008;
pub const CEC_ADDR: c_uint = 0x100c;

pub const CEC_TX_COUNT: c_uint = 0x1020;
pub const CEC_TX_DATA3_0: c_uint = 0x1024;
pub const CEC_TX_DATA7_4: c_uint = 0x1028;
pub const CEC_TX_DATA11_8: c_uint = 0x102c;
pub const CEC_TX_DATA15_12: c_uint = 0x1030;
pub const CEC_RX_COUNT_STATUS: c_uint = 0x1040;
pub const CEC_RX_DATA3_0: c_uint = 0x1044;
pub const CEC_RX_DATA7_4: c_uint = 0x1048;
pub const CEC_RX_DATA11_8: c_uint = 0x104c;
pub const CEC_RX_DATA15_12: c_uint = 0x1050;
pub const CEC_LOCK_CONTROL: c_uint = 0x1054;
pub const CEC_RXQUAL_BITTIME_CONFIG: c_uint = 0x1060;
pub const CEC_RX_BITTIME_CONFIG: c_uint = 0x1064;
pub const CEC_TX_BITTIME_CONFIG: c_uint = 0x1068;
// eARC RX CMDC Registers
pub const EARCRX_CMDC_CONFIG0: c_uint = 0x1800;

pub const EARCRX_CMDC_CONFIG1: c_uint = 0x1804;
pub const EARCRX_CMDC_CONTROL: c_uint = 0x1808;

pub const EARCRX_CMDC_WHITELIST0_CONFIG: c_uint = 0x180c;
pub const EARCRX_CMDC_WHITELIST1_CONFIG: c_uint = 0x1810;
pub const EARCRX_CMDC_WHITELIST2_CONFIG: c_uint = 0x1814;
pub const EARCRX_CMDC_WHITELIST3_CONFIG: c_uint = 0x1818;
pub const EARCRX_CMDC_STATUS: c_uint = 0x181c;
pub const EARCRX_CMDC_XACT_INFO: c_uint = 0x1820;
pub const EARCRX_CMDC_XACT_ACTION: c_uint = 0x1824;
pub const EARCRX_CMDC_HEARTBEAT_RXSTAT_SE: c_uint = 0x1828;
pub const EARCRX_CMDC_HEARTBEAT_STATUS: c_uint = 0x182c;
pub const EARCRX_CMDC_XACT_WR0: c_uint = 0x1840;
pub const EARCRX_CMDC_XACT_WR1: c_uint = 0x1844;
pub const EARCRX_CMDC_XACT_WR2: c_uint = 0x1848;
pub const EARCRX_CMDC_XACT_WR3: c_uint = 0x184c;
pub const EARCRX_CMDC_XACT_WR4: c_uint = 0x1850;
pub const EARCRX_CMDC_XACT_WR5: c_uint = 0x1854;
pub const EARCRX_CMDC_XACT_WR6: c_uint = 0x1858;
pub const EARCRX_CMDC_XACT_WR7: c_uint = 0x185c;
pub const EARCRX_CMDC_XACT_WR8: c_uint = 0x1860;
pub const EARCRX_CMDC_XACT_WR9: c_uint = 0x1864;
pub const EARCRX_CMDC_XACT_WR10: c_uint = 0x1868;
pub const EARCRX_CMDC_XACT_WR11: c_uint = 0x186c;
pub const EARCRX_CMDC_XACT_WR12: c_uint = 0x1870;
pub const EARCRX_CMDC_XACT_WR13: c_uint = 0x1874;
pub const EARCRX_CMDC_XACT_WR14: c_uint = 0x1878;
pub const EARCRX_CMDC_XACT_WR15: c_uint = 0x187c;
pub const EARCRX_CMDC_XACT_WR16: c_uint = 0x1880;
pub const EARCRX_CMDC_XACT_WR17: c_uint = 0x1884;
pub const EARCRX_CMDC_XACT_WR18: c_uint = 0x1888;
pub const EARCRX_CMDC_XACT_WR19: c_uint = 0x188c;
pub const EARCRX_CMDC_XACT_WR20: c_uint = 0x1890;
pub const EARCRX_CMDC_XACT_WR21: c_uint = 0x1894;
pub const EARCRX_CMDC_XACT_WR22: c_uint = 0x1898;
pub const EARCRX_CMDC_XACT_WR23: c_uint = 0x189c;
pub const EARCRX_CMDC_XACT_WR24: c_uint = 0x18a0;
pub const EARCRX_CMDC_XACT_WR25: c_uint = 0x18a4;
pub const EARCRX_CMDC_XACT_WR26: c_uint = 0x18a8;
pub const EARCRX_CMDC_XACT_WR27: c_uint = 0x18ac;
pub const EARCRX_CMDC_XACT_WR28: c_uint = 0x18b0;
pub const EARCRX_CMDC_XACT_WR29: c_uint = 0x18b4;
pub const EARCRX_CMDC_XACT_WR30: c_uint = 0x18b8;
pub const EARCRX_CMDC_XACT_WR31: c_uint = 0x18bc;
pub const EARCRX_CMDC_XACT_WR32: c_uint = 0x18c0;
pub const EARCRX_CMDC_XACT_WR33: c_uint = 0x18c4;
pub const EARCRX_CMDC_XACT_WR34: c_uint = 0x18c8;
pub const EARCRX_CMDC_XACT_WR35: c_uint = 0x18cc;
pub const EARCRX_CMDC_XACT_WR36: c_uint = 0x18d0;
pub const EARCRX_CMDC_XACT_WR37: c_uint = 0x18d4;
pub const EARCRX_CMDC_XACT_WR38: c_uint = 0x18d8;
pub const EARCRX_CMDC_XACT_WR39: c_uint = 0x18dc;
pub const EARCRX_CMDC_XACT_WR40: c_uint = 0x18e0;
pub const EARCRX_CMDC_XACT_WR41: c_uint = 0x18e4;
pub const EARCRX_CMDC_XACT_WR42: c_uint = 0x18e8;
pub const EARCRX_CMDC_XACT_WR43: c_uint = 0x18ec;
pub const EARCRX_CMDC_XACT_WR44: c_uint = 0x18f0;
pub const EARCRX_CMDC_XACT_WR45: c_uint = 0x18f4;
pub const EARCRX_CMDC_XACT_WR46: c_uint = 0x18f8;
pub const EARCRX_CMDC_XACT_WR47: c_uint = 0x18fc;
pub const EARCRX_CMDC_XACT_WR48: c_uint = 0x1900;
pub const EARCRX_CMDC_XACT_WR49: c_uint = 0x1904;
pub const EARCRX_CMDC_XACT_WR50: c_uint = 0x1908;
pub const EARCRX_CMDC_XACT_WR51: c_uint = 0x190c;
pub const EARCRX_CMDC_XACT_WR52: c_uint = 0x1910;
pub const EARCRX_CMDC_XACT_WR53: c_uint = 0x1914;
pub const EARCRX_CMDC_XACT_WR54: c_uint = 0x1918;
pub const EARCRX_CMDC_XACT_WR55: c_uint = 0x191c;
pub const EARCRX_CMDC_XACT_WR56: c_uint = 0x1920;
pub const EARCRX_CMDC_XACT_WR57: c_uint = 0x1924;
pub const EARCRX_CMDC_XACT_WR58: c_uint = 0x1928;
pub const EARCRX_CMDC_XACT_WR59: c_uint = 0x192c;
pub const EARCRX_CMDC_XACT_WR60: c_uint = 0x1930;
pub const EARCRX_CMDC_XACT_WR61: c_uint = 0x1934;
pub const EARCRX_CMDC_XACT_WR62: c_uint = 0x1938;
pub const EARCRX_CMDC_XACT_WR63: c_uint = 0x193c;
pub const EARCRX_CMDC_XACT_WR64: c_uint = 0x1940;
pub const EARCRX_CMDC_XACT_RD0: c_uint = 0x1960;
pub const EARCRX_CMDC_XACT_RD1: c_uint = 0x1964;
pub const EARCRX_CMDC_XACT_RD2: c_uint = 0x1968;
pub const EARCRX_CMDC_XACT_RD3: c_uint = 0x196c;
pub const EARCRX_CMDC_XACT_RD4: c_uint = 0x1970;
pub const EARCRX_CMDC_XACT_RD5: c_uint = 0x1974;
pub const EARCRX_CMDC_XACT_RD6: c_uint = 0x1978;
pub const EARCRX_CMDC_XACT_RD7: c_uint = 0x197c;
pub const EARCRX_CMDC_XACT_RD8: c_uint = 0x1980;
pub const EARCRX_CMDC_XACT_RD9: c_uint = 0x1984;
pub const EARCRX_CMDC_XACT_RD10: c_uint = 0x1988;
pub const EARCRX_CMDC_XACT_RD11: c_uint = 0x198c;
pub const EARCRX_CMDC_XACT_RD12: c_uint = 0x1990;
pub const EARCRX_CMDC_XACT_RD13: c_uint = 0x1994;
pub const EARCRX_CMDC_XACT_RD14: c_uint = 0x1998;
pub const EARCRX_CMDC_XACT_RD15: c_uint = 0x199c;
pub const EARCRX_CMDC_XACT_RD16: c_uint = 0x19a0;
pub const EARCRX_CMDC_XACT_RD17: c_uint = 0x19a4;
pub const EARCRX_CMDC_XACT_RD18: c_uint = 0x19a8;
pub const EARCRX_CMDC_XACT_RD19: c_uint = 0x19ac;
pub const EARCRX_CMDC_XACT_RD20: c_uint = 0x19b0;
pub const EARCRX_CMDC_XACT_RD21: c_uint = 0x19b4;
pub const EARCRX_CMDC_XACT_RD22: c_uint = 0x19b8;
pub const EARCRX_CMDC_XACT_RD23: c_uint = 0x19bc;
pub const EARCRX_CMDC_XACT_RD24: c_uint = 0x19c0;
pub const EARCRX_CMDC_XACT_RD25: c_uint = 0x19c4;
pub const EARCRX_CMDC_XACT_RD26: c_uint = 0x19c8;
pub const EARCRX_CMDC_XACT_RD27: c_uint = 0x19cc;
pub const EARCRX_CMDC_XACT_RD28: c_uint = 0x19d0;
pub const EARCRX_CMDC_XACT_RD29: c_uint = 0x19d4;
pub const EARCRX_CMDC_XACT_RD30: c_uint = 0x19d8;
pub const EARCRX_CMDC_XACT_RD31: c_uint = 0x19dc;
pub const EARCRX_CMDC_XACT_RD32: c_uint = 0x19e0;
pub const EARCRX_CMDC_XACT_RD33: c_uint = 0x19e4;
pub const EARCRX_CMDC_XACT_RD34: c_uint = 0x19e8;
pub const EARCRX_CMDC_XACT_RD35: c_uint = 0x19ec;
pub const EARCRX_CMDC_XACT_RD36: c_uint = 0x19f0;
pub const EARCRX_CMDC_XACT_RD37: c_uint = 0x19f4;
pub const EARCRX_CMDC_XACT_RD38: c_uint = 0x19f8;
pub const EARCRX_CMDC_XACT_RD39: c_uint = 0x19fc;
pub const EARCRX_CMDC_XACT_RD40: c_uint = 0x1a00;
pub const EARCRX_CMDC_XACT_RD41: c_uint = 0x1a04;
pub const EARCRX_CMDC_XACT_RD42: c_uint = 0x1a08;
pub const EARCRX_CMDC_XACT_RD43: c_uint = 0x1a0c;
pub const EARCRX_CMDC_XACT_RD44: c_uint = 0x1a10;
pub const EARCRX_CMDC_XACT_RD45: c_uint = 0x1a14;
pub const EARCRX_CMDC_XACT_RD46: c_uint = 0x1a18;
pub const EARCRX_CMDC_XACT_RD47: c_uint = 0x1a1c;
pub const EARCRX_CMDC_XACT_RD48: c_uint = 0x1a20;
pub const EARCRX_CMDC_XACT_RD49: c_uint = 0x1a24;
pub const EARCRX_CMDC_XACT_RD50: c_uint = 0x1a28;
pub const EARCRX_CMDC_XACT_RD51: c_uint = 0x1a2c;
pub const EARCRX_CMDC_XACT_RD52: c_uint = 0x1a30;
pub const EARCRX_CMDC_XACT_RD53: c_uint = 0x1a34;
pub const EARCRX_CMDC_XACT_RD54: c_uint = 0x1a38;
pub const EARCRX_CMDC_XACT_RD55: c_uint = 0x1a3c;
pub const EARCRX_CMDC_XACT_RD56: c_uint = 0x1a40;
pub const EARCRX_CMDC_XACT_RD57: c_uint = 0x1a44;
pub const EARCRX_CMDC_XACT_RD58: c_uint = 0x1a48;
pub const EARCRX_CMDC_XACT_RD59: c_uint = 0x1a4c;
pub const EARCRX_CMDC_XACT_RD60: c_uint = 0x1a50;
pub const EARCRX_CMDC_XACT_RD61: c_uint = 0x1a54;
pub const EARCRX_CMDC_XACT_RD62: c_uint = 0x1a58;
pub const EARCRX_CMDC_XACT_RD63: c_uint = 0x1a5c;
pub const EARCRX_CMDC_XACT_RD64: c_uint = 0x1a60;
pub const EARCRX_CMDC_SYNC_CONFIG: c_uint = 0x1b00;
// eARC RX DMAC Registers
pub const EARCRX_DMAC_PHY_CONTROL: c_uint = 0x1c00;
pub const EARCRX_DMAC_CONFIG: c_uint = 0x1c08;
pub const EARCRX_DMAC_CONTROL0: c_uint = 0x1c0c;

pub const EARCRX_DMAC_CONTROL1: c_uint = 0x1c10;
pub const EARCRX_DMAC_STATUS: c_uint = 0x1c14;
pub const EARCRX_DMAC_CHSTATUS0: c_uint = 0x1c18;
pub const EARCRX_DMAC_CHSTATUS1: c_uint = 0x1c1c;
pub const EARCRX_DMAC_CHSTATUS2: c_uint = 0x1c20;
pub const EARCRX_DMAC_CHSTATUS3: c_uint = 0x1c24;
pub const EARCRX_DMAC_CHSTATUS4: c_uint = 0x1c28;
pub const EARCRX_DMAC_CHSTATUS5: c_uint = 0x1c2c;
pub const EARCRX_DMAC_USRDATA_MSG_HDMI_AC0: c_uint = 0x1c30;
pub const EARCRX_DMAC_USRDATA_MSG_HDMI_AC1: c_uint = 0x1c34;
pub const EARCRX_DMAC_USRDATA_MSG_HDMI_AC2: c_uint = 0x1c38;
pub const EARCRX_DMAC_USRDATA_MSG_HDMI_AC3: c_uint = 0x1c3c;
pub const EARCRX_DMAC_USRDATA_MSG_HDMI_AC4: c_uint = 0x1c40;
pub const EARCRX_DMAC_USRDATA_MSG_HDMI_AC5: c_uint = 0x1c44;
pub const EARCRX_DMAC_USRDATA_MSG_HDMI_AC6: c_uint = 0x1c48;
pub const EARCRX_DMAC_USRDATA_MSG_HDMI_AC7: c_uint = 0x1c4c;
pub const EARCRX_DMAC_USRDATA_MSG_HDMI_AC8: c_uint = 0x1c50;
pub const EARCRX_DMAC_USRDATA_MSG_HDMI_AC9: c_uint = 0x1c54;
pub const EARCRX_DMAC_USRDATA_MSG_HDMI_AC10: c_uint = 0x1c58;
pub const EARCRX_DMAC_USRDATA_MSG_HDMI_AC11: c_uint = 0x1c5c;
pub const EARCRX_DMAC_USRDATA_MSG_HDMI_ISRC1_PKT0: c_uint = 0x1c60;
pub const EARCRX_DMAC_USRDATA_MSG_HDMI_ISRC1_PKT1: c_uint = 0x1c64;
pub const EARCRX_DMAC_USRDATA_MSG_HDMI_ISRC1_PKT2: c_uint = 0x1c68;
pub const EARCRX_DMAC_USRDATA_MSG_HDMI_ISRC1_PKT3: c_uint = 0x1c6c;
pub const EARCRX_DMAC_USRDATA_MSG_HDMI_ISRC1_PKT4: c_uint = 0x1c70;
pub const EARCRX_DMAC_USRDATA_MSG_HDMI_ISRC1_PKT5: c_uint = 0x1c74;
pub const EARCRX_DMAC_USRDATA_MSG_HDMI_ISRC1_PKT6: c_uint = 0x1c78;
pub const EARCRX_DMAC_USRDATA_MSG_HDMI_ISRC1_PKT7: c_uint = 0x1c7c;
pub const EARCRX_DMAC_USRDATA_MSG_HDMI_ISRC1_PKT8: c_uint = 0x1c80;
pub const EARCRX_DMAC_USRDATA_MSG_HDMI_ISRC1_PKT9: c_uint = 0x1c84;
pub const EARCRX_DMAC_USRDATA_MSG_HDMI_ISRC1_PKT10: c_uint = 0x1c88;
pub const EARCRX_DMAC_USRDATA_MSG_HDMI_ISRC1_PKT11: c_uint = 0x1c8c;
pub const EARCRX_DMAC_USRDATA_MSG_HDMI_ISRC2_PKT0: c_uint = 0x1c90;
pub const EARCRX_DMAC_USRDATA_MSG_HDMI_ISRC2_PKT1: c_uint = 0x1c94;
pub const EARCRX_DMAC_USRDATA_MSG_HDMI_ISRC2_PKT2: c_uint = 0x1c98;
pub const EARCRX_DMAC_USRDATA_MSG_HDMI_ISRC2_PKT3: c_uint = 0x1c9c;
pub const EARCRX_DMAC_USRDATA_MSG_HDMI_ISRC2_PKT4: c_uint = 0x1ca0;
pub const EARCRX_DMAC_USRDATA_MSG_HDMI_ISRC2_PKT5: c_uint = 0x1ca4;
pub const EARCRX_DMAC_USRDATA_MSG_HDMI_ISRC2_PKT6: c_uint = 0x1ca8;
pub const EARCRX_DMAC_USRDATA_MSG_HDMI_ISRC2_PKT7: c_uint = 0x1cac;
pub const EARCRX_DMAC_USRDATA_MSG_HDMI_ISRC2_PKT8: c_uint = 0x1cb0;
pub const EARCRX_DMAC_USRDATA_MSG_HDMI_ISRC2_PKT9: c_uint = 0x1cb4;
pub const EARCRX_DMAC_USRDATA_MSG_HDMI_ISRC2_PKT10: c_uint = 0x1cb8;
pub const EARCRX_DMAC_USRDATA_MSG_HDMI_ISRC2_PKT11: c_uint = 0x1cbc;
pub const EARCRX_DMAC_USRDATA_MSG_GENERIC0: c_uint = 0x1cc0;
pub const EARCRX_DMAC_USRDATA_MSG_GENERIC1: c_uint = 0x1cc4;
pub const EARCRX_DMAC_USRDATA_MSG_GENERIC2: c_uint = 0x1cc8;
pub const EARCRX_DMAC_USRDATA_MSG_GENERIC3: c_uint = 0x1ccc;
pub const EARCRX_DMAC_USRDATA_MSG_GENERIC4: c_uint = 0x1cd0;
pub const EARCRX_DMAC_USRDATA_MSG_GENERIC5: c_uint = 0x1cd4;
pub const EARCRX_DMAC_USRDATA_MSG_GENERIC6: c_uint = 0x1cd8;
pub const EARCRX_DMAC_USRDATA_MSG_GENERIC7: c_uint = 0x1cdc;
pub const EARCRX_DMAC_USRDATA_MSG_GENERIC8: c_uint = 0x1ce0;
pub const EARCRX_DMAC_USRDATA_MSG_GENERIC9: c_uint = 0x1ce4;
pub const EARCRX_DMAC_USRDATA_MSG_GENERIC10: c_uint = 0x1ce8;
pub const EARCRX_DMAC_USRDATA_MSG_GENERIC11: c_uint = 0x1cec;
pub const EARCRX_DMAC_USRDATA_MSG_GENERIC12: c_uint = 0x1cf0;
pub const EARCRX_DMAC_USRDATA_MSG_GENERIC13: c_uint = 0x1cf4;
pub const EARCRX_DMAC_USRDATA_MSG_GENERIC14: c_uint = 0x1cf8;
pub const EARCRX_DMAC_USRDATA_MSG_GENERIC15: c_uint = 0x1cfc;
pub const EARCRX_DMAC_USRDATA_MSG_GENERIC16: c_uint = 0x1d00;
pub const EARCRX_DMAC_USRDATA_MSG_GENERIC17: c_uint = 0x1d04;
pub const EARCRX_DMAC_USRDATA_MSG_GENERIC18: c_uint = 0x1d08;
pub const EARCRX_DMAC_USRDATA_MSG_GENERIC19: c_uint = 0x1d0c;
pub const EARCRX_DMAC_USRDATA_MSG_GENERIC20: c_uint = 0x1d10;
pub const EARCRX_DMAC_USRDATA_MSG_GENERIC21: c_uint = 0x1d14;
pub const EARCRX_DMAC_USRDATA_MSG_GENERIC22: c_uint = 0x1d18;
pub const EARCRX_DMAC_USRDATA_MSG_GENERIC23: c_uint = 0x1d1c;
pub const EARCRX_DMAC_USRDATA_MSG_GENERIC24: c_uint = 0x1d20;
pub const EARCRX_DMAC_USRDATA_MSG_GENERIC25: c_uint = 0x1d24;
pub const EARCRX_DMAC_USRDATA_MSG_GENERIC26: c_uint = 0x1d28;
pub const EARCRX_DMAC_USRDATA_MSG_GENERIC27: c_uint = 0x1d2c;
pub const EARCRX_DMAC_USRDATA_MSG_GENERIC28: c_uint = 0x1d30;
pub const EARCRX_DMAC_USRDATA_MSG_GENERIC29: c_uint = 0x1d34;
pub const EARCRX_DMAC_USRDATA_MSG_GENERIC30: c_uint = 0x1d38;
pub const EARCRX_DMAC_USRDATA_MSG_GENERIC31: c_uint = 0x1d3c;
pub const EARCRX_DMAC_USRDATA_MSG_GENERIC32: c_uint = 0x1d40;
pub const EARCRX_DMAC_CHSTATUS_STREAMER0: c_uint = 0x1d44;
pub const EARCRX_DMAC_CHSTATUS_STREAMER1: c_uint = 0x1d48;
pub const EARCRX_DMAC_CHSTATUS_STREAMER2: c_uint = 0x1d4c;
pub const EARCRX_DMAC_CHSTATUS_STREAMER3: c_uint = 0x1d50;
pub const EARCRX_DMAC_CHSTATUS_STREAMER4: c_uint = 0x1d54;
pub const EARCRX_DMAC_CHSTATUS_STREAMER5: c_uint = 0x1d58;
pub const EARCRX_DMAC_CHSTATUS_STREAMER6: c_uint = 0x1d5c;
pub const EARCRX_DMAC_CHSTATUS_STREAMER7: c_uint = 0x1d60;
pub const EARCRX_DMAC_CHSTATUS_STREAMER8: c_uint = 0x1d64;
pub const EARCRX_DMAC_CHSTATUS_STREAMER9: c_uint = 0x1d68;
pub const EARCRX_DMAC_CHSTATUS_STREAMER10: c_uint = 0x1d6c;
pub const EARCRX_DMAC_CHSTATUS_STREAMER11: c_uint = 0x1d70;
pub const EARCRX_DMAC_CHSTATUS_STREAMER12: c_uint = 0x1d74;
pub const EARCRX_DMAC_CHSTATUS_STREAMER13: c_uint = 0x1d78;
pub const EARCRX_DMAC_CHSTATUS_STREAMER14: c_uint = 0x1d7c;
pub const EARCRX_DMAC_USRDATA_STREAMER0: c_uint = 0x1d80;
// Main Unit Interrupt Registers
pub const MAIN_INTVEC_INDEX: c_uint = 0x3000;
pub const MAINUNIT_0_INT_STATUS: c_uint = 0x3010;
pub const MAINUNIT_0_INT_MASK_N: c_uint = 0x3014;
pub const MAINUNIT_0_INT_CLEAR: c_uint = 0x3018;
pub const MAINUNIT_0_INT_FORCE: c_uint = 0x301c;
pub const MAINUNIT_1_INT_STATUS: c_uint = 0x3020;

pub const MAINUNIT_1_INT_MASK_N: c_uint = 0x3024;

pub const MAINUNIT_1_INT_CLEAR: c_uint = 0x3028;

pub const MAINUNIT_1_INT_FORCE: c_uint = 0x302c;
// AVPUNIT Interrupt Registers
pub const AVP_INTVEC_INDEX: c_uint = 0x3800;
pub const AVP_0_INT_STATUS: c_uint = 0x3810;
pub const AVP_0_INT_MASK_N: c_uint = 0x3814;
pub const AVP_0_INT_CLEAR: c_uint = 0x3818;
pub const AVP_0_INT_FORCE: c_uint = 0x381c;
pub const AVP_1_INT_STATUS: c_uint = 0x3820;
pub const AVP_1_INT_MASK_N: c_uint = 0x3824;

pub const AVP_1_INT_CLEAR: c_uint = 0x3828;
pub const AVP_1_INT_FORCE: c_uint = 0x382c;
pub const AVP_2_INT_STATUS: c_uint = 0x3830;
pub const AVP_2_INT_MASK_N: c_uint = 0x3834;
pub const AVP_2_INT_CLEAR: c_uint = 0x3838;
pub const AVP_2_INT_FORCE: c_uint = 0x383c;
pub const AVP_3_INT_STATUS: c_uint = 0x3840;
pub const AVP_3_INT_MASK_N: c_uint = 0x3844;
pub const AVP_3_INT_CLEAR: c_uint = 0x3848;
pub const AVP_3_INT_FORCE: c_uint = 0x384c;
pub const AVP_4_INT_STATUS: c_uint = 0x3850;
pub const AVP_4_INT_MASK_N: c_uint = 0x3854;
pub const AVP_4_INT_CLEAR: c_uint = 0x3858;
pub const AVP_4_INT_FORCE: c_uint = 0x385c;
pub const AVP_5_INT_STATUS: c_uint = 0x3860;
pub const AVP_5_INT_MASK_N: c_uint = 0x3864;
pub const AVP_5_INT_CLEAR: c_uint = 0x3868;
pub const AVP_5_INT_FORCE: c_uint = 0x386c;
pub const AVP_6_INT_STATUS: c_uint = 0x3870;
pub const AVP_6_INT_MASK_N: c_uint = 0x3874;
pub const AVP_6_INT_CLEAR: c_uint = 0x3878;
pub const AVP_6_INT_FORCE: c_uint = 0x387c;
// CEC Interrupt Registers
pub const CEC_INT_STATUS: c_uint = 0x4000;
pub const CEC_INT_MASK_N: c_uint = 0x4004;
pub const CEC_INT_CLEAR: c_uint = 0x4008;
pub const CEC_INT_FORCE: c_uint = 0x400c;
// eARC RX Interrupt Registers
pub const EARCRX_INTVEC_INDEX: c_uint = 0x4800;
pub const EARCRX_0_INT_STATUS: c_uint = 0x4810;

pub const EARCRX_0_INT_MASK_N: c_uint = 0x4814;
pub const EARCRX_0_INT_CLEAR: c_uint = 0x4818;
pub const EARCRX_0_INT_FORCE: c_uint = 0x481c;
pub const EARCRX_1_INT_STATUS: c_uint = 0x4820;
pub const EARCRX_1_INT_MASK_N: c_uint = 0x4824;
pub const EARCRX_1_INT_CLEAR: c_uint = 0x4828;
pub const EARCRX_1_INT_FORCE: c_uint = 0x482c;
