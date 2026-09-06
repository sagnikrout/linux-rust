//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/display/drm_dp.h
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


//
// Copyright © 2008 Keith Packard
//
// Permission to use, copy, modify, distribute, and sell this software and its
// documentation for any purpose is hereby granted without fee, provided that
// the above copyright notice appear in all copies and that both that copyright
// notice and this permission notice appear in supporting documentation, and
// that the name of the copyright holders not be used in advertising or
// publicity pertaining to distribution of the software without specific,
// written prior permission.  The copyright holders make no representations
// about the suitability of this software for any purpose.  It is provided "as
// is" without express or implied warranty.
//
// THE COPYRIGHT HOLDERS DISCLAIM ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
// INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
// EVENT SHALL THE COPYRIGHT HOLDERS BE LIABLE FOR ANY SPECIAL, INDIRECT OR
// CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
// DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
// TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE
// OF THIS SOFTWARE.
//

//
// Unless otherwise noted, all values are from the DP 1.1a spec.  Note that
// DP and DPCD versions are independent.  Differences from 1.0 are not noted,
// 1.0 devices basically don't exist in the wild.
//
// Abbreviations, in chronological order:
//
// eDP: Embedded DisplayPort version 1
// DPI: DisplayPort Interoperability Guideline v1.1a
// 1.2: DisplayPort 1.2
// MST: Multistream Transport - part of DP 1.2a
//
// 1.2 formally includes both eDP and DPI definitions.
//
// MSA (Main Stream Attribute) MISC bits (as MISC1<<8|MISC0)

// bits per component for non-RAW

// bits per component for RAW

// pixel encoding/colorimetry format

pub const DP_AUX_MAX_PAYLOAD_BYTES: c_int = 16;
pub const DP_AUX_I2C_WRITE: c_uint = 0x0;
pub const DP_AUX_I2C_READ: c_uint = 0x1;
pub const DP_AUX_I2C_WRITE_STATUS_UPDATE: c_uint = 0x2;
pub const DP_AUX_I2C_MOT: c_uint = 0x4;
pub const DP_AUX_NATIVE_WRITE: c_uint = 0x8;
pub const DP_AUX_NATIVE_READ: c_uint = 0x9;

// DPCD Field Address Mapping
// Receiver Capability
pub const DP_DPCD_REV: c_uint = 0x000;

pub const DP_MAX_LINK_RATE: c_uint = 0x001;
pub const DP_MAX_LANE_COUNT: c_uint = 0x002;

pub const DP_MAX_DOWNSPREAD: c_uint = 0x003;

pub const DP_NORP: c_uint = 0x004;
pub const DP_DOWNSTREAMPORT_PRESENT: c_uint = 0x005;

pub const DP_MAIN_LINK_CHANNEL_CODING: c_uint = 0x006;

pub const DP_DOWN_STREAM_PORT_COUNT: c_uint = 0x007;

pub const DP_RECEIVE_PORT_0_CAP_0: c_uint = 0x008;

pub const DP_RECEIVE_PORT_0_BUFFER_SIZE: c_uint = 0x009;
pub const DP_RECEIVE_PORT_1_CAP_0: c_uint = 0x00a;
pub const DP_RECEIVE_PORT_1_BUFFER_SIZE: c_uint = 0x00b;
pub const DP_I2C_SPEED_CAP: c_uint = 0x00c    /* DPI */;

pub const DP_EDP_CONFIGURATION_CAP: c_uint = 0x00d   /* XXX 1.2? */;

pub const DP_TRAINING_AUX_RD_INTERVAL: c_uint = 0x00e   /* XXX 1.2? */;

pub const DP_ADAPTER_CAP: c_uint = 0x00f   /* 1.2 */;

pub const DP_SUPPORTED_LINK_RATES: c_uint = 0x010 /* eDP 1.4 */;

// Multiple stream transport
pub const DP_FAUX_CAP: c_uint = 0x020   /* 1.2 */;

pub const DP_SINK_VIDEO_FALLBACK_FORMATS: c_uint = 0x020   /* 2.0 */;

pub const DP_MSTM_CAP: c_uint = 0x021   /* 1.2 */;

pub const DP_NUMBER_OF_AUDIO_ENDPOINTS: c_uint = 0x022   /* 1.2 */;
// AV_SYNC_DATA_BLOCK                                  1.2
pub const DP_AV_GRANULARITY: c_uint = 0x023;

pub const DP_AUD_DEC_LAT0: c_uint = 0x024;
pub const DP_AUD_DEC_LAT1: c_uint = 0x025;
pub const DP_AUD_PP_LAT0: c_uint = 0x026;
pub const DP_AUD_PP_LAT1: c_uint = 0x027;
pub const DP_VID_INTER_LAT: c_uint = 0x028;
pub const DP_VID_PROG_LAT: c_uint = 0x029;
pub const DP_REP_LAT: c_uint = 0x02a;
pub const DP_AUD_DEL_INS0: c_uint = 0x02b;
pub const DP_AUD_DEL_INS1: c_uint = 0x02c;
pub const DP_AUD_DEL_INS2: c_uint = 0x02d;
// End of AV_SYNC_DATA_BLOCK
pub const DP_RECEIVER_ALPM_CAP: c_uint = 0x02e   /* eDP 1.4 */;

pub const DP_SINK_DEVICE_AUX_FRAME_SYNC_CAP: c_uint = 0x02f   /* eDP 1.4 */;

pub const DP_GUID: c_uint = 0x030   /* 1.2 */;
pub const DP_DSC_SUPPORT: c_uint = 0x060   /* DP 1.4 */;

pub const DP_DSC_REV: c_uint = 0x061;

pub const DP_DSC_RC_BUF_BLK_SIZE: c_uint = 0x062;

pub const DP_DSC_RC_BUF_SIZE: c_uint = 0x063;
pub const DP_DSC_SLICE_CAP_1: c_uint = 0x064;

pub const DP_DSC_LINE_BUF_BIT_DEPTH: c_uint = 0x065;

pub const DP_DSC_BLK_PREDICTION_SUPPORT: c_uint = 0x066;

pub const DP_DSC_MAX_BITS_PER_PIXEL_LOW: c_uint = 0x067   /* eDP 1.4 */;
pub const DP_DSC_MAX_BITS_PER_PIXEL_HI: c_uint = 0x068   /* eDP 1.4 */;

pub const DP_DSC_DEC_COLOR_FORMAT_CAP: c_uint = 0x069;

pub const DP_DSC_DEC_COLOR_DEPTH_CAP: c_uint = 0x06A;

pub const DP_DSC_PEAK_THROUGHPUT: c_uint = 0x06B;

pub const DP_DSC_MAX_SLICE_WIDTH: c_uint = 0x06C;
pub const DP_DSC_MIN_SLICE_WIDTH_VALUE: c_int = 2560;
pub const DP_DSC_SLICE_WIDTH_MULTIPLIER: c_int = 320;
pub const DP_DSC_SLICE_CAP_2: c_uint = 0x06D;

pub const DP_DSC_MAX_BPP_DELTA_VERSION_1: c_uint = 0x06E;

pub const DP_DSC_BITS_PER_PIXEL_INC: c_uint = 0x06F;

pub const DP_PSR_SUPPORT: c_uint = 0x070   /* XXX 1.2? */;

pub const DP_PSR_CAPS: c_uint = 0x071   /* XXX 1.2? */;

pub const DP_PSR2_SU_X_GRANULARITY: c_uint = 0x072 /* eDP 1.4b */;
pub const DP_PSR2_SU_Y_GRANULARITY: c_uint = 0x074 /* eDP 1.4b */;
//
// 0x80-0x8f describe downstream port capabilities, but there are two layouts
// based on whether DP_DETAILED_CAP_INFO_AVAILABLE was set.  If it was not,
// each port's descriptor is one byte wide.  If it was set, each port's is
// four bytes wide, starting with the one byte from the base info.  As of
// DP interop v1.1a only VGA defines additional detail.
//
// offset 0
pub const DP_DOWNSTREAM_PORT_0: c_uint = 0x80;

// offset 1 for VGA is maximum megapixels per second / 8
// offset 1 for DVI/HDMI is maximum TMDS clock in Mbps / 2.5
// offset 2 for VGA/DVI/HDMI

// HDMI2.1 PCON FRL CONFIGURATION

// offset 3 for DVI

// offset 3 for HDMI

//
// VESA DP-to-HDMI PCON Specification adds caps for colorspace
// conversion in DFP cap DPCD 83h. Sec6.1 Table-3.
// Based on the available support the source can enable
// color conversion by writing into PROTOCOL_COVERTER_CONTROL_2
// DPCD 3052h.
//

pub const DP_MAX_DOWNSTREAM_PORTS: c_uint = 0x10;
// DP Forward error Correction Registers
pub const DP_FEC_CAPABILITY: c_uint = 0x090    /* 1.4 */;

pub const DP_FEC_CAPABILITY_1: c_uint = 0x091   /* 2.0 */;
// DP-HDMI2.1 PCON DSC ENCODER SUPPORT
pub const DP_PCON_DSC_ENCODER_CAP_SIZE: c_uint = 0xD	/* 0x92 through 0x9E */;
pub const DP_PCON_DSC_ENCODER: c_uint = 0x092;

// DP-HDMI2.1 PCON DSC Version
pub const DP_PCON_DSC_VERSION: c_uint = 0x093;

// DP-HDMI2.1 PCON DSC RC Buffer block size
pub const DP_PCON_DSC_RC_BUF_BLK_INFO: c_uint = 0x094;

// DP-HDMI2.1 PCON DSC RC Buffer size
pub const DP_PCON_DSC_RC_BUF_SIZE: c_uint = 0x095;
// DP-HDMI2.1 PCON DSC Slice capabilities-1
pub const DP_PCON_DSC_SLICE_CAP_1: c_uint = 0x096;

pub const DP_PCON_DSC_BUF_BIT_DEPTH: c_uint = 0x097;

pub const DP_PCON_DSC_BLOCK_PREDICTION: c_uint = 0x098;

pub const DP_PCON_DSC_ENC_COLOR_FMT_CAP: c_uint = 0x099;

pub const DP_PCON_DSC_ENC_COLOR_DEPTH_CAP: c_uint = 0x09A;

pub const DP_PCON_DSC_MAX_SLICE_WIDTH: c_uint = 0x09B;
// DP-HDMI2.1 PCON DSC Slice capabilities-2
pub const DP_PCON_DSC_SLICE_CAP_2: c_uint = 0x09C;

// DP-HDMI2.1 PCON HDMI TX Encoder Bits/pixel increment
pub const DP_PCON_DSC_BPP_INCR: c_uint = 0x09E;

// DP Extended DSC Capabilities
pub const DP_DSC_BRANCH_OVERALL_THROUGHPUT_0: c_uint = 0x0a0   /* DP 1.4a SCR */;
pub const DP_DSC_BRANCH_OVERALL_THROUGHPUT_1: c_uint = 0x0a1;
pub const DP_DSC_BRANCH_MAX_LINE_WIDTH: c_uint = 0x0a2;
// DFP Capability Extension
pub const DP_DFP_CAPABILITY_EXTENSION_SUPPORT: c_uint = 0x0a3	/* 2.0 */;
pub const DP_PANEL_REPLAY_CAP_SUPPORT: c_uint = 0x0b0  /* DP 2.0 */;

pub const DP_PANEL_REPLAY_CAP_SIZE: c_int = 7;
pub const DP_PANEL_REPLAY_CAP_CAPABILITY: c_uint = 0xb1;

pub const DP_PANEL_REPLAY_CAP_X_GRANULARITY: c_uint = 0xb2;

pub const DP_PANEL_REPLAY_CAP_Y_GRANULARITY: c_uint = 0xb4;
// Link Configuration
pub const DP_LINK_BW_SET: c_uint = 0x100;

pub const DP_LANE_COUNT_SET: c_uint = 0x101;

pub const DP_TRAINING_PATTERN_SET: c_uint = 0x102;

// DPCD 1.1 only. For DPCD >= 1.2 see per-lane DP_LINK_QUAL_LANEn_SET

pub const DP_TRAINING_LANE0_SET: c_uint = 0x103;
pub const DP_TRAINING_LANE1_SET: c_uint = 0x104;
pub const DP_TRAINING_LANE2_SET: c_uint = 0x105;
pub const DP_TRAINING_LANE3_SET: c_uint = 0x106;

pub const DP_DOWNSPREAD_CTRL: c_uint = 0x107;

pub const DP_MAIN_LINK_CHANNEL_CODING_SET: c_uint = 0x108;

pub const DP_I2C_SPEED_CONTROL_STATUS: c_uint = 0x109   /* DPI */;
// bitmask as for DP_I2C_SPEED_CAP
pub const DP_EDP_CONFIGURATION_SET: c_uint = 0x10a   /* XXX 1.2? */;

pub const DP_LINK_QUAL_LANE0_SET: c_uint = 0x10b   /* DPCD >= 1.2 */;
pub const DP_LINK_QUAL_LANE1_SET: c_uint = 0x10c;
pub const DP_LINK_QUAL_LANE2_SET: c_uint = 0x10d;
pub const DP_LINK_QUAL_LANE3_SET: c_uint = 0x10e;

// DP 2.0 UHBR10, UHBR13.5, UHBR20

pub const DP_TRAINING_LANE0_1_SET2: c_uint = 0x10f;
pub const DP_TRAINING_LANE2_3_SET2: c_uint = 0x110;

pub const DP_MSTM_CTRL: c_uint = 0x111   /* 1.2 */;

pub const DP_AUDIO_DELAY0: c_uint = 0x112   /* 1.2 */;
pub const DP_AUDIO_DELAY1: c_uint = 0x113;
pub const DP_AUDIO_DELAY2: c_uint = 0x114;
pub const DP_LINK_RATE_SET: c_uint = 0x115   /* eDP 1.4 */;

pub const DP_RECEIVER_ALPM_CONFIG: c_uint = 0x116   /* eDP 1.4 */;

pub const DP_SINK_DEVICE_AUX_FRAME_SYNC_CONF: c_uint = 0x117   /* eDP 1.4 */;

pub const DP_UPSTREAM_DEVICE_DP_PWR_NEED: c_uint = 0x118   /* 1.2 */;

pub const DP_EXTENDED_DPRX_SLEEP_WAKE_TIMEOUT_GRANT: c_uint = 0x119   /* 1.4a */;

pub const PANEL_REPLAY_CONFIG3: c_uint = 0x11a /* DP 2.1 */;

pub const DP_FEC_CONFIGURATION: c_uint = 0x120    /* 1.4 */;

pub const DP_SDP_ERROR_DETECTION_CONFIGURATION: c_uint = 0x121	/* DP 2.0 E11 */;

pub const DP_AUX_FRAME_SYNC_VALUE: c_uint = 0x15c   /* eDP 1.4 */;

pub const DP_DSC_ENABLE: c_uint = 0x160   /* DP 1.4 */;

pub const DP_DSC_CONFIGURATION: c_uint = 0x161	/* DP 2.0 */;
pub const DP_PSR_EN_CFG: c_uint = 0x170   /* XXX 1.2? */;

pub const DP_ADAPTER_CTRL: c_uint = 0x1a0;

pub const DP_BRANCH_DEVICE_CTRL: c_uint = 0x1a1;

pub const PANEL_REPLAY_CONFIG: c_uint = 0x1b0  /* DP 2.0 */;

pub const PANEL_REPLAY_CONFIG2: c_uint = 0x1b1 /* eDP 1.5 */;

pub const DP_PAYLOAD_ALLOCATE_SET: c_uint = 0x1c0;
pub const DP_PAYLOAD_ALLOCATE_START_TIME_SLOT: c_uint = 0x1c1;
pub const DP_PAYLOAD_ALLOCATE_TIME_SLOT_COUNT: c_uint = 0x1c2;
// Link/Sink Device Status
pub const DP_SINK_COUNT: c_uint = 0x200;
// prior to 1.2 bit 7 was reserved mbz

pub const DP_DEVICE_SERVICE_IRQ_VECTOR: c_uint = 0x201;

pub const DP_LANE0_1_STATUS: c_uint = 0x202;
pub const DP_LANE2_3_STATUS: c_uint = 0x203;

pub const DP_LANE_ALIGN_STATUS_UPDATED: c_uint = 0x204;

pub const DP_SINK_STATUS: c_uint = 0x205;

pub const DP_ADJUST_REQUEST_LANE0_1: c_uint = 0x206;
pub const DP_ADJUST_REQUEST_LANE2_3: c_uint = 0x207;

// DP 2.0 128b/132b Link Layer

pub const DP_ADJUST_REQUEST_POST_CURSOR2: c_uint = 0x20c;

pub const DP_TEST_REQUEST: c_uint = 0x218;

pub const DP_TEST_LINK_RATE: c_uint = 0x219;

pub const DP_TEST_LANE_COUNT: c_uint = 0x220;
pub const DP_TEST_PATTERN: c_uint = 0x221;

pub const DP_TEST_H_TOTAL_HI: c_uint = 0x222;
pub const DP_TEST_H_TOTAL_LO: c_uint = 0x223;
pub const DP_TEST_V_TOTAL_HI: c_uint = 0x224;
pub const DP_TEST_V_TOTAL_LO: c_uint = 0x225;
pub const DP_TEST_H_START_HI: c_uint = 0x226;
pub const DP_TEST_H_START_LO: c_uint = 0x227;
pub const DP_TEST_V_START_HI: c_uint = 0x228;
pub const DP_TEST_V_START_LO: c_uint = 0x229;
pub const DP_TEST_HSYNC_HI: c_uint = 0x22A;

pub const DP_TEST_HSYNC_WIDTH_LO: c_uint = 0x22B;
pub const DP_TEST_VSYNC_HI: c_uint = 0x22C;

pub const DP_TEST_VSYNC_WIDTH_LO: c_uint = 0x22D;
pub const DP_TEST_H_WIDTH_HI: c_uint = 0x22E;
pub const DP_TEST_H_WIDTH_LO: c_uint = 0x22F;
pub const DP_TEST_V_HEIGHT_HI: c_uint = 0x230;
pub const DP_TEST_V_HEIGHT_LO: c_uint = 0x231;
pub const DP_TEST_MISC0: c_uint = 0x232;

pub const DP_TEST_MISC1: c_uint = 0x233;

pub const DP_TEST_REFRESH_RATE_NUMERATOR: c_uint = 0x234;
pub const DP_TEST_MISC0: c_uint = 0x232;
pub const DP_TEST_CRC_R_CR: c_uint = 0x240;
pub const DP_TEST_CRC_G_Y: c_uint = 0x242;
pub const DP_TEST_CRC_B_CB: c_uint = 0x244;
pub const DP_TEST_SINK_MISC: c_uint = 0x246;

pub const DP_PHY_TEST_PATTERN: c_uint = 0x248;

pub const DP_PHY_SQUARE_PATTERN: c_uint = 0x249;
pub const DP_TEST_HBR2_SCRAMBLER_RESET: c_uint = 0x24A;
pub const DP_TEST_80BIT_CUSTOM_PATTERN_7_0: c_uint = 0x250;
pub const DP_TEST_80BIT_CUSTOM_PATTERN_15_8: c_uint = 0x251;
pub const DP_TEST_80BIT_CUSTOM_PATTERN_23_16: c_uint = 0x252;
pub const DP_TEST_80BIT_CUSTOM_PATTERN_31_24: c_uint = 0x253;
pub const DP_TEST_80BIT_CUSTOM_PATTERN_39_32: c_uint = 0x254;
pub const DP_TEST_80BIT_CUSTOM_PATTERN_47_40: c_uint = 0x255;
pub const DP_TEST_80BIT_CUSTOM_PATTERN_55_48: c_uint = 0x256;
pub const DP_TEST_80BIT_CUSTOM_PATTERN_63_56: c_uint = 0x257;
pub const DP_TEST_80BIT_CUSTOM_PATTERN_71_64: c_uint = 0x258;
pub const DP_TEST_80BIT_CUSTOM_PATTERN_79_72: c_uint = 0x259;
pub const DP_TEST_RESPONSE: c_uint = 0x260;

pub const DP_TEST_EDID_CHECKSUM: c_uint = 0x261;
pub const DP_TEST_SINK: c_uint = 0x270;

pub const DP_TEST_AUDIO_MODE: c_uint = 0x271;
pub const DP_TEST_AUDIO_PATTERN_TYPE: c_uint = 0x272;
pub const DP_TEST_AUDIO_PERIOD_CH1: c_uint = 0x273;
pub const DP_TEST_AUDIO_PERIOD_CH2: c_uint = 0x274;
pub const DP_TEST_AUDIO_PERIOD_CH3: c_uint = 0x275;
pub const DP_TEST_AUDIO_PERIOD_CH4: c_uint = 0x276;
pub const DP_TEST_AUDIO_PERIOD_CH5: c_uint = 0x277;
pub const DP_TEST_AUDIO_PERIOD_CH6: c_uint = 0x278;
pub const DP_TEST_AUDIO_PERIOD_CH7: c_uint = 0x279;
pub const DP_TEST_AUDIO_PERIOD_CH8: c_uint = 0x27A;
pub const DP_FEC_STATUS: c_uint = 0x280    /* 1.4 */;

pub const DP_FEC_ERROR_COUNT_LSB: c_uint = 0x0281    /* 1.4 */;
pub const DP_FEC_ERROR_COUNT_MSB: c_uint = 0x0282    /* 1.4 */;

pub const DP_PAYLOAD_TABLE_UPDATE_STATUS: c_uint = 0x2c0   /* 1.2 MST */;

pub const DP_VC_PAYLOAD_ID_SLOT_1: c_uint = 0x2c1   /* 1.2 MST */;
// up to ID_SLOT_63 at 0x2ff
// Source Device-specific
pub const DP_SOURCE_OUI: c_uint = 0x300;
// Sink Device-specific
pub const DP_SINK_OUI: c_uint = 0x400;
// Branch Device-specific
pub const DP_BRANCH_OUI: c_uint = 0x500;
pub const DP_BRANCH_ID: c_uint = 0x503;
pub const DP_BRANCH_REVISION_START: c_uint = 0x509;
pub const DP_BRANCH_HW_REV: c_uint = 0x509;
pub const DP_BRANCH_SW_REV: c_uint = 0x50A;
// Link/Sink Device Power Control
pub const DP_SET_POWER: c_uint = 0x600;

// eDP-specific
pub const DP_EDP_DPCD_REV: c_uint = 0x700    /* eDP 1.2 */;

pub const DP_EDP_GENERAL_CAP_1: c_uint = 0x701;

pub const DP_EDP_BACKLIGHT_ADJUSTMENT_CAP: c_uint = 0x702;

pub const DP_EDP_GENERAL_CAP_2: c_uint = 0x703;

pub const DP_EDP_GENERAL_CAP_3: c_uint = 0x704    /* eDP 1.4 */;

pub const DP_EDP_DISPLAY_CONTROL_REGISTER: c_uint = 0x720;

pub const DP_EDP_BACKLIGHT_MODE_SET_REGISTER: c_uint = 0x721;

pub const DP_EDP_BACKLIGHT_BRIGHTNESS_MSB: c_uint = 0x722;
pub const DP_EDP_BACKLIGHT_BRIGHTNESS_LSB: c_uint = 0x723;
pub const DP_EDP_PWMGEN_BIT_COUNT: c_uint = 0x724;
pub const DP_EDP_PWMGEN_BIT_COUNT_CAP_MIN: c_uint = 0x725;
pub const DP_EDP_PWMGEN_BIT_COUNT_CAP_MAX: c_uint = 0x726;

pub const DP_EDP_BACKLIGHT_CONTROL_STATUS: c_uint = 0x727;
pub const DP_EDP_BACKLIGHT_FREQ_SET: c_uint = 0x728;

pub const DP_EDP_BACKLIGHT_FREQ_CAP_MIN_MSB: c_uint = 0x72a;
pub const DP_EDP_BACKLIGHT_FREQ_CAP_MIN_MID: c_uint = 0x72b;
pub const DP_EDP_BACKLIGHT_FREQ_CAP_MIN_LSB: c_uint = 0x72c;
pub const DP_EDP_BACKLIGHT_FREQ_CAP_MAX_MSB: c_uint = 0x72d;
pub const DP_EDP_BACKLIGHT_FREQ_CAP_MAX_MID: c_uint = 0x72e;
pub const DP_EDP_BACKLIGHT_FREQ_CAP_MAX_LSB: c_uint = 0x72f;
pub const DP_EDP_DBC_MINIMUM_BRIGHTNESS_SET: c_uint = 0x732;
pub const DP_EDP_DBC_MAXIMUM_BRIGHTNESS_SET: c_uint = 0x733;
pub const DP_EDP_PANEL_TARGET_LUMINANCE_VALUE: c_uint = 0x734;
pub const DP_EDP_REGIONAL_BACKLIGHT_BASE: c_uint = 0x740    /* eDP 1.4 */;
pub const DP_EDP_REGIONAL_BACKLIGHT_0: c_uint = 0x741    /* eDP 1.4 */;
pub const DP_EDP_MSO_LINK_CAPABILITIES: c_uint = 0x7a4    /* eDP 1.4 */;

// Sideband MSG Buffers
pub const DP_SIDEBAND_MSG_DOWN_REQ_BASE: c_uint = 0x1000   /* 1.2 MST */;
pub const DP_SIDEBAND_MSG_UP_REP_BASE: c_uint = 0x1200   /* 1.2 MST */;
pub const DP_SIDEBAND_MSG_DOWN_REP_BASE: c_uint = 0x1400   /* 1.2 MST */;
pub const DP_SIDEBAND_MSG_UP_REQ_BASE: c_uint = 0x1600   /* 1.2 MST */;
// DPRX Event Status Indicator
pub const DP_SINK_COUNT_ESI: c_uint = 0x2002   /* same as 0x200 */;
pub const DP_DEVICE_SERVICE_IRQ_VECTOR_ESI0: c_uint = 0x2003   /* same as 0x201 */;
pub const DP_DEVICE_SERVICE_IRQ_VECTOR_ESI1: c_uint = 0x2004   /* 1.2 */;

pub const DP_LINK_SERVICE_IRQ_VECTOR_ESI0: c_uint = 0x2005   /* 1.2 */;

pub const DP_PSR_ERROR_STATUS: c_uint = 0x2006  /* XXX 1.2? */;

pub const DP_PSR_ESI: c_uint = 0x2007  /* XXX 1.2? */;

pub const DP_PSR_STATUS: c_uint = 0x2008  /* XXX 1.2? */;

pub const DP_SYNCHRONIZATION_LATENCY_IN_SINK: c_uint = 0x2009 /* edp 1.4 */;

pub const DP_LAST_RECEIVED_PSR_SDP: c_uint = 0x200a /* eDP 1.2 */;

pub const DP_RECEIVER_ALPM_STATUS: c_uint = 0x200b  /* eDP 1.4 */;

pub const DP_LANE0_1_STATUS_ESI: c_uint = 0x200c /* status same as 0x202 */;
pub const DP_LANE2_3_STATUS_ESI: c_uint = 0x200d /* status same as 0x203 */;
pub const DP_LANE_ALIGN_STATUS_UPDATED_ESI: c_uint = 0x200e /* status same as 0x204 */;
pub const DP_SINK_STATUS_ESI: c_uint = 0x200f /* status same as 0x205 */;
pub const DP_PANEL_REPLAY_ERROR_STATUS: c_uint = 0x2020  /* DP 2.1*/;

pub const DP_SINK_DEVICE_PR_AND_FRAME_LOCK_STATUS: c_uint = 0x2022  /* DP 2.1 */;

// Extended Receiver Capability: See DP_DPCD_REV for definitions
pub const DP_DP13_DPCD_REV: c_uint = 0x2200;
pub const DP_DPRX_FEATURE_ENUMERATION_LIST: c_uint = 0x2210  /* DP 1.3 */;

pub const DP_EXTENDED_DPRX_SLEEP_WAKE_TIMEOUT_REQUEST: c_uint = 0x2211  /* 1.4a */;

pub const DP_DPRX_FEATURE_ENUMERATION_LIST_CONT_1: c_uint = 0x2214 /* 2.0 E11 */;

pub const DP_128B132B_SUPPORTED_LINK_RATES: c_uint = 0x2215 /* 2.0 */;

pub const DP_128B132B_TRAINING_AUX_RD_INTERVAL: c_uint = 0x2216 /* 2.0 */;

pub const DP_TEST_264BIT_CUSTOM_PATTERN_7_0: c_uint = 0x2230;
pub const DP_TEST_264BIT_CUSTOM_PATTERN_263_256: c_uint = 0x2250;
// DSC Extended Capability Branch Total DSC Resources
pub const DP_DSC_SUPPORT_AND_DSC_DECODER_COUNT: c_uint = 0x2260	/* 2.0 */;

pub const DP_DSC_MAX_SLICE_COUNT_AND_AGGREGATION_0: c_uint = 0x2270	/* 2.0 */;

// Protocol Converter Extension
// HDMI CEC tunneling over AUX DP 1.3 section 5.3.3.3.1 DPCD 1.4+
pub const DP_CEC_TUNNELING_CAPABILITY: c_uint = 0x3000;

pub const DP_CEC_TUNNELING_CONTROL: c_uint = 0x3001;

pub const DP_CEC_RX_MESSAGE_INFO: c_uint = 0x3002;

pub const DP_CEC_TX_MESSAGE_INFO: c_uint = 0x3003;

pub const DP_CEC_TUNNELING_IRQ_FLAGS: c_uint = 0x3004;

pub const DP_CEC_LOGICAL_ADDRESS_MASK: c_uint = 0x300E /* 0x300F word */;

pub const DP_CEC_LOGICAL_ADDRESS_MASK_2: c_uint = 0x300F /* 0x300E word */;

pub const DP_CEC_RX_MESSAGE_BUFFER: c_uint = 0x3010;
pub const DP_CEC_TX_MESSAGE_BUFFER: c_uint = 0x3020;
pub const DP_CEC_MESSAGE_BUFFER_LENGTH: c_uint = 0x10;
// PCON CONFIGURE-1 FRL FOR HDMI SINK
pub const DP_PCON_HDMI_LINK_CONFIG_1: c_uint = 0x305A;

// PCON CONFIGURE-2 FRL FOR HDMI SINK
pub const DP_PCON_HDMI_LINK_CONFIG_2: c_uint = 0x305B;

// PCON HDMI LINK STATUS
pub const DP_PCON_HDMI_TX_LINK_STATUS: c_uint = 0x303B;

// PCON HDMI POST FRL STATUS
pub const DP_PCON_HDMI_POST_FRL_STATUS: c_uint = 0x3036;

pub const DP_PROTOCOL_CONVERTER_CONTROL_0: c_uint = 0x3050 /* DP 1.3 */;

pub const DP_PROTOCOL_CONVERTER_CONTROL_1: c_uint = 0x3051 /* DP 1.3 */;

pub const DP_PROTOCOL_CONVERTER_CONTROL_2: c_uint = 0x3052 /* DP 1.3 */;

// PCON Downstream HDMI ERROR Status per Lane
pub const DP_PCON_HDMI_ERROR_STATUS_LN0: c_uint = 0x3037;
pub const DP_PCON_HDMI_ERROR_STATUS_LN1: c_uint = 0x3038;
pub const DP_PCON_HDMI_ERROR_STATUS_LN2: c_uint = 0x3039;
pub const DP_PCON_HDMI_ERROR_STATUS_LN3: c_uint = 0x303A;

// PCON HDMI CONFIG PPS Override Buffer
// Valid Offsets to be added to Base : 0-127
//
pub const DP_PCON_HDMI_PPS_OVERRIDE_BASE: c_uint = 0x3100;
// PCON HDMI CONFIG PPS Override Parameter: Slice height
// Offset-0 8LSBs of the Slice height.
// Offset-1 8MSBs of the Slice height.
//
pub const DP_PCON_HDMI_PPS_OVRD_SLICE_HEIGHT: c_uint = 0x3180;
// PCON HDMI CONFIG PPS Override Parameter: Slice width
// Offset-0 8LSBs of the Slice width.
// Offset-1 8MSBs of the Slice width.
//
pub const DP_PCON_HDMI_PPS_OVRD_SLICE_WIDTH: c_uint = 0x3182;
// PCON HDMI CONFIG PPS Override Parameter: bits_per_pixel
// Offset-0 8LSBs of the bits_per_pixel.
// Offset-1 2MSBs of the bits_per_pixel.
//
pub const DP_PCON_HDMI_PPS_OVRD_BPP: c_uint = 0x3184;
// HDCP 1.3 and HDCP 2.2
pub const DP_AUX_HDCP_BKSV: c_uint = 0x68000;
pub const DP_AUX_HDCP_RI_PRIME: c_uint = 0x68005;
pub const DP_AUX_HDCP_AKSV: c_uint = 0x68007;
pub const DP_AUX_HDCP_AN: c_uint = 0x6800C;

pub const DP_AUX_HDCP_BCAPS: c_uint = 0x68028;

pub const DP_AUX_HDCP_BSTATUS: c_uint = 0x68029;

pub const DP_AUX_HDCP_BINFO: c_uint = 0x6802A;
pub const DP_AUX_HDCP_KSV_FIFO: c_uint = 0x6802C;
pub const DP_AUX_HDCP_AINFO: c_uint = 0x6803B;
// DP HDCP2.2 parameter offsets in DPCD address space
pub const DP_HDCP_2_2_REG_RTX_OFFSET: c_uint = 0x69000;
pub const DP_HDCP_2_2_REG_TXCAPS_OFFSET: c_uint = 0x69008;
pub const DP_HDCP_2_2_REG_CERT_RX_OFFSET: c_uint = 0x6900B;
pub const DP_HDCP_2_2_REG_RRX_OFFSET: c_uint = 0x69215;
pub const DP_HDCP_2_2_REG_RX_CAPS_OFFSET: c_uint = 0x6921D;
pub const DP_HDCP_2_2_REG_EKPUB_KM_OFFSET: c_uint = 0x69220;
pub const DP_HDCP_2_2_REG_EKH_KM_WR_OFFSET: c_uint = 0x692A0;
pub const DP_HDCP_2_2_REG_M_OFFSET: c_uint = 0x692B0;
pub const DP_HDCP_2_2_REG_HPRIME_OFFSET: c_uint = 0x692C0;
pub const DP_HDCP_2_2_REG_EKH_KM_RD_OFFSET: c_uint = 0x692E0;
pub const DP_HDCP_2_2_REG_RN_OFFSET: c_uint = 0x692F0;
pub const DP_HDCP_2_2_REG_LPRIME_OFFSET: c_uint = 0x692F8;
pub const DP_HDCP_2_2_REG_EDKEY_KS_OFFSET: c_uint = 0x69318;
pub const DP_HDCP_2_2_REG_RIV_OFFSET: c_uint = 0x69328;
pub const DP_HDCP_2_2_REG_RXINFO_OFFSET: c_uint = 0x69330;
pub const DP_HDCP_2_2_REG_SEQ_NUM_V_OFFSET: c_uint = 0x69332;
pub const DP_HDCP_2_2_REG_VPRIME_OFFSET: c_uint = 0x69335;
pub const DP_HDCP_2_2_REG_RECV_ID_LIST_OFFSET: c_uint = 0x69345;
pub const DP_HDCP_2_2_REG_V_OFFSET: c_uint = 0x693E0;
pub const DP_HDCP_2_2_REG_SEQ_NUM_M_OFFSET: c_uint = 0x693F0;
pub const DP_HDCP_2_2_REG_K_OFFSET: c_uint = 0x693F3;
pub const DP_HDCP_2_2_REG_STREAM_ID_TYPE_OFFSET: c_uint = 0x693F5;
pub const DP_HDCP_2_2_REG_MPRIME_OFFSET: c_uint = 0x69473;
pub const DP_HDCP_2_2_REG_RXSTATUS_OFFSET: c_uint = 0x69493;
pub const DP_HDCP_2_2_REG_STREAM_TYPE_OFFSET: c_uint = 0x69494;
pub const DP_HDCP_2_2_REG_DBG_OFFSET: c_uint = 0x69518;
// DP-tunneling
pub const DP_TUNNELING_OUI: c_uint = 0xe0000;
pub const DP_TUNNELING_OUI_BYTES: c_int = 3;
pub const DP_TUNNELING_DEV_ID: c_uint = 0xe0003;
pub const DP_TUNNELING_DEV_ID_BYTES: c_int = 6;
pub const DP_TUNNELING_HW_REV: c_uint = 0xe0009;
pub const DP_TUNNELING_HW_REV_MAJOR_SHIFT: c_int = 4;

pub const DP_TUNNELING_HW_REV_MINOR_SHIFT: c_int = 0;

pub const DP_TUNNELING_SW_REV_MAJOR: c_uint = 0xe000a;
pub const DP_TUNNELING_SW_REV_MINOR: c_uint = 0xe000b;
pub const DP_TUNNELING_CAPABILITIES: c_uint = 0xe000d;

pub const DP_IN_ADAPTER_INFO: c_uint = 0xe000e;
pub const DP_IN_ADAPTER_NUMBER_BITS: c_int = 7;

pub const DP_USB4_DRIVER_ID: c_uint = 0xe000f;
pub const DP_USB4_DRIVER_ID_BITS: c_int = 4;

pub const DP_USB4_DRIVER_BW_CAPABILITY: c_uint = 0xe0020;

pub const DP_IN_ADAPTER_TUNNEL_INFORMATION: c_uint = 0xe0021;
pub const DP_GROUP_ID_BITS: c_int = 3;

pub const DP_BW_GRANULARITY: c_uint = 0xe0022;
pub const DP_BW_GRANULARITY_MASK: c_uint = 0x3;
pub const DP_ESTIMATED_BW: c_uint = 0xe0023;
pub const DP_ALLOCATED_BW: c_uint = 0xe0024;
pub const DP_TUNNELING_STATUS: c_uint = 0xe0025;

pub const DP_TUNNELING_MAX_LINK_RATE: c_uint = 0xe0028;
pub const DP_TUNNELING_MAX_LANE_COUNT: c_uint = 0xe0029;
pub const DP_TUNNELING_MAX_LANE_COUNT_MASK: c_uint = 0x1f;
pub const DP_TUNNELING_MAIN_LINK_CHANNEL_CODING: c_uint = 0xe002b;

pub const DP_TUNNELING_128B132B_LINK_RATE: c_uint = 0xe002c;

pub const DP_DPTX_BW_ALLOCATION_MODE_CONTROL: c_uint = 0xe0030;

pub const DP_REQUEST_BW: c_uint = 0xe0031;
pub const MAX_DP_REQUEST_BW: c_int = 255;
// LTTPR: Link Training (LT)-tunable PHY Repeaters
pub const DP_LT_TUNABLE_PHY_REPEATER_FIELD_DATA_STRUCTURE_REV: c_uint = 0xf0000 /* 1.3 */;
pub const DP_MAX_LINK_RATE_PHY_REPEATER: c_uint = 0xf0001 /* 1.4a */;
pub const DP_PHY_REPEATER_CNT: c_uint = 0xf0002 /* 1.3 */;
pub const DP_PHY_REPEATER_MODE: c_uint = 0xf0003 /* 1.3 */;
pub const DP_MAX_LANE_COUNT_PHY_REPEATER: c_uint = 0xf0004 /* 1.4a */;
pub const DP_Repeater_FEC_CAPABILITY: c_uint = 0xf0004 /* 1.4 */;
pub const DP_PHY_REPEATER_EXTENDED_WAIT_TIMEOUT: c_uint = 0xf0005 /* 1.4a */;

pub const DP_MAIN_LINK_CHANNEL_CODING_PHY_REPEATER: c_uint = 0xf0006 /* 2.0 */;

// See DP_128B132B_SUPPORTED_LINK_RATES for values
pub const DP_PHY_REPEATER_128B132B_RATES: c_uint = 0xf0007 /* 2.0 */;
pub const DP_PHY_REPEATER_EQ_DONE: c_uint = 0xf0008 /* 2.0 E11 */;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_dp_phy {
    DP_PHY_DPRX,

    DP_PHY_LTTPR1,
    DP_PHY_LTTPR2,
    DP_PHY_LTTPR3,
    DP_PHY_LTTPR4,
    DP_PHY_LTTPR5,
    DP_PHY_LTTPR6,
    DP_PHY_LTTPR7,
    DP_PHY_LTTPR8,

    DP_MAX_LTTPR_COUNT = DP_PHY_LTTPR8,
}

pub const __DP_LTTPR1_BASE: c_uint = 0xf0010 /* 1.3 */;
pub const __DP_LTTPR2_BASE: c_uint = 0xf0060 /* 1.3 */;

pub const DP_TRAINING_PATTERN_SET_PHY_REPEATER1: c_uint = 0xf0010 /* 1.3 */;

pub const DP_TRAINING_LANE0_SET_PHY_REPEATER1: c_uint = 0xf0011 /* 1.3 */;

pub const DP_TRAINING_LANE1_SET_PHY_REPEATER1: c_uint = 0xf0012 /* 1.3 */;
pub const DP_TRAINING_LANE2_SET_PHY_REPEATER1: c_uint = 0xf0013 /* 1.3 */;
pub const DP_TRAINING_LANE3_SET_PHY_REPEATER1: c_uint = 0xf0014 /* 1.3 */;
pub const DP_TRAINING_AUX_RD_INTERVAL_PHY_REPEATER1: c_uint = 0xf0020 /* 1.4a */;

pub const DP_TRANSMITTER_CAPABILITY_PHY_REPEATER1: c_uint = 0xf0021 /* 1.4a */;

pub const DP_128B132B_TRAINING_AUX_RD_INTERVAL_PHY_REPEATER1: c_uint = 0xf0022 /* 2.0 */;

// see DP_128B132B_TRAINING_AUX_RD_INTERVAL for values
pub const DP_LANE0_1_STATUS_PHY_REPEATER1: c_uint = 0xf0030 /* 1.3 */;

pub const DP_LANE2_3_STATUS_PHY_REPEATER1: c_uint = 0xf0031 /* 1.3 */;
pub const DP_LANE_ALIGN_STATUS_UPDATED_PHY_REPEATER1: c_uint = 0xf0032 /* 1.3 */;
pub const DP_ADJUST_REQUEST_LANE0_1_PHY_REPEATER1: c_uint = 0xf0033 /* 1.3 */;
pub const DP_ADJUST_REQUEST_LANE2_3_PHY_REPEATER1: c_uint = 0xf0034 /* 1.3 */;
pub const DP_SYMBOL_ERROR_COUNT_LANE0_PHY_REPEATER1: c_uint = 0xf0035 /* 1.3 */;
pub const DP_SYMBOL_ERROR_COUNT_LANE1_PHY_REPEATER1: c_uint = 0xf0037 /* 1.3 */;
pub const DP_SYMBOL_ERROR_COUNT_LANE2_PHY_REPEATER1: c_uint = 0xf0039 /* 1.3 */;
pub const DP_SYMBOL_ERROR_COUNT_LANE3_PHY_REPEATER1: c_uint = 0xf003b /* 1.3 */;
pub const DP_OUI_PHY_REPEATER1: c_uint = 0xf003d /* 1.3 */;

pub const __DP_FEC1_BASE: c_uint = 0xf0290 /* 1.4 */;
pub const __DP_FEC2_BASE: c_uint = 0xf0298 /* 1.4 */;

pub const DP_FEC_STATUS_PHY_REPEATER1: c_uint = 0xf0290 /* 1.4 */;

pub const DP_FEC_ERROR_COUNT_PHY_REPEATER1: c_uint = 0xf0291 /* 1.4 */;
pub const DP_FEC_CAPABILITY_PHY_REPEATER1: c_uint = 0xf0294 /* 1.4a */;
pub const DP_LTTPR_MAX_ADD: c_uint = 0xf02ff /* 1.4 */;
pub const DP_DPCD_MAX_ADD: c_uint = 0xfffff /* 1.4 */;
// Repeater modes
pub const DP_PHY_REPEATER_MODE_TRANSPARENT: c_uint = 0x55    /* 1.3 */;
pub const DP_PHY_REPEATER_MODE_NON_TRANSPARENT: c_uint = 0xaa    /* 1.3 */;
// DP HDCP message start offsets in DPCD address space

pub const HDCP_2_2_DP_RXSTATUS_LEN: c_int = 1;

// DP 1.2 Sideband message defines
// peer device type - DP 1.2a Table 2-92
pub const DP_PEER_DEVICE_NONE: c_uint = 0x0;
pub const DP_PEER_DEVICE_SOURCE_OR_SST: c_uint = 0x1;
pub const DP_PEER_DEVICE_MST_BRANCHING: c_uint = 0x2;
pub const DP_PEER_DEVICE_SST_SINK: c_uint = 0x3;
pub const DP_PEER_DEVICE_DP_LEGACY_CONV: c_uint = 0x4;
// DP 1.2 MST sideband request names DP 1.2a Table 2-80
pub const DP_GET_MSG_TRANSACTION_VERSION: c_uint = 0x00 /* DP 1.3 */;
pub const DP_LINK_ADDRESS: c_uint = 0x01;
pub const DP_CONNECTION_STATUS_NOTIFY: c_uint = 0x02;
pub const DP_ENUM_PATH_RESOURCES: c_uint = 0x10;
pub const DP_ALLOCATE_PAYLOAD: c_uint = 0x11;
pub const DP_QUERY_PAYLOAD: c_uint = 0x12;
pub const DP_RESOURCE_STATUS_NOTIFY: c_uint = 0x13;
pub const DP_CLEAR_PAYLOAD_ID_TABLE: c_uint = 0x14;
pub const DP_REMOTE_DPCD_READ: c_uint = 0x20;
pub const DP_REMOTE_DPCD_WRITE: c_uint = 0x21;
pub const DP_REMOTE_I2C_READ: c_uint = 0x22;
pub const DP_REMOTE_I2C_WRITE: c_uint = 0x23;
pub const DP_POWER_UP_PHY: c_uint = 0x24;
pub const DP_POWER_DOWN_PHY: c_uint = 0x25;
pub const DP_SINK_EVENT_NOTIFY: c_uint = 0x30;
pub const DP_QUERY_STREAM_ENC_STATUS: c_uint = 0x38;
pub const DP_QUERY_STREAM_ENC_STATUS_STATE_NO_EXIST: c_int = 0;
pub const DP_QUERY_STREAM_ENC_STATUS_STATE_INACTIVE: c_int = 1;
pub const DP_QUERY_STREAM_ENC_STATUS_STATE_ACTIVE: c_int = 2;
// DP 1.2 MST sideband reply types
pub const DP_SIDEBAND_REPLY_ACK: c_uint = 0x00;
pub const DP_SIDEBAND_REPLY_NAK: c_uint = 0x01;
// DP 1.2 MST sideband nak reasons - table 2.84
pub const DP_NAK_WRITE_FAILURE: c_uint = 0x01;
pub const DP_NAK_INVALID_READ: c_uint = 0x02;
pub const DP_NAK_CRC_FAILURE: c_uint = 0x03;
pub const DP_NAK_BAD_PARAM: c_uint = 0x04;
pub const DP_NAK_DEFER: c_uint = 0x05;
pub const DP_NAK_LINK_FAILURE: c_uint = 0x06;
pub const DP_NAK_NO_RESOURCES: c_uint = 0x07;
pub const DP_NAK_DPCD_FAIL: c_uint = 0x08;
pub const DP_NAK_I2C_NAK: c_uint = 0x09;
pub const DP_NAK_ALLOCATE_FAIL: c_uint = 0x0a;
pub const MODE_I2C_START: c_int = 1;
pub const MODE_I2C_WRITE: c_int = 2;
pub const MODE_I2C_READ: c_int = 4;
pub const MODE_I2C_STOP: c_int = 8;
// DP 1.2 MST PORTs - Section 2.5.1 v1.2a spec
pub const DP_MST_PHYSICAL_PORT_0: c_int = 0;
pub const DP_MST_LOGICAL_PORT_0: c_int = 8;
pub const DP_LINK_CONSTANT_N_VALUE: c_uint = 0x8000;
pub const DP_LINK_STATUS_SIZE: c_int = 6;
pub const DP_BRANCH_OUI_HEADER_SIZE: c_uint = 0xc;
pub const DP_RECEIVER_CAP_SIZE: c_uint = 0xf;
pub const DP_DSC_RECEIVER_CAP_SIZE: c_uint = 0x10 /* DSC Capabilities 0x60 through 0x6F */;
pub const DP_DSC_BRANCH_CAP_SIZE: c_int = 3;
pub const EDP_PSR_RECEIVER_CAP_SIZE: c_int = 2;
pub const EDP_DISPLAY_CTL_CAP_SIZE: c_int = 5;
pub const DP_LTTPR_COMMON_CAP_SIZE: c_int = 8;
pub const DP_LTTPR_PHY_CAP_SIZE: c_int = 3;
pub const DP_SDP_AUDIO_TIMESTAMP: c_uint = 0x01;
pub const DP_SDP_AUDIO_STREAM: c_uint = 0x02;
pub const DP_SDP_EXTENSION: c_uint = 0x04 /* DP 1.1 */;
pub const DP_SDP_AUDIO_COPYMANAGEMENT: c_uint = 0x05 /* DP 1.2 */;
pub const DP_SDP_ISRC: c_uint = 0x06 /* DP 1.2 */;
pub const DP_SDP_VSC: c_uint = 0x07 /* DP 1.2 */;
pub const DP_SDP_ADAPTIVE_SYNC: c_uint = 0x22 /* DP 1.4 */;

pub const DP_SDP_PPS: c_uint = 0x10 /* DP 1.4 */;
pub const DP_SDP_VSC_EXT_VESA: c_uint = 0x20 /* DP 1.4 */;
pub const DP_SDP_VSC_EXT_CEA: c_uint = 0x21 /* DP 1.4 */;
// 0x80+ CEA-861 infoframe types
pub const DP_SDP_AUDIO_INFOFRAME_HB2: c_uint = 0x1b;
//
// struct dp_sdp_header - DP secondary data packet header
// @HB0: Secondary Data Packet ID
// @HB1: Secondary Data Packet Type
// @HB2: Secondary Data Packet Specific header, Byte 0
// @HB3: Secondary Data packet Specific header, Byte 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_sdp_header {
    pub HB0: u8,
    pub HB1: u8,
    pub HB2: u8,
    pub HB3: u8,
    pub __packed: },
pub const EDP_SDP_HEADER_REVISION_MASK: c_uint = 0x1F;
pub const EDP_SDP_HEADER_VALID_PAYLOAD_BYTES: c_uint = 0x1F;
pub const DP_SDP_PPS_HEADER_PAYLOAD_BYTES_MINUS_1: c_uint = 0x7F;
//
// struct dp_sdp - DP secondary data packet
// @sdp_header: DP secondary data packet header
// @db: DP secondaray data packet data blocks
// VSC SDP Payload for PSR
// db[0]: Stereo Interface
// db[1]: 0 - PSR State; 1 - Update RFB; 2 - CRC Valid
// db[2]: CRC value bits 7:0 of the R or Cr component
// db[3]: CRC value bits 15:8 of the R or Cr component
// db[4]: CRC value bits 7:0 of the G or Y component
// db[5]: CRC value bits 15:8 of the G or Y component
// db[6]: CRC value bits 7:0 of the B or Cb component
// db[7]: CRC value bits 15:8 of the B or Cb component
// db[8] - db[31]: Reserved
// VSC SDP Payload for Pixel Encoding/Colorimetry Format
// db[0] - db[15]: Reserved
// db[16]: Pixel Encoding and Colorimetry Formats
// db[17]: Dynamic Range and Component Bit Depth
// db[18]: Content Type
// db[19] - db[31]: Reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_sdp {
    pub sdp_header: dp_sdp_header,
    pub db: [u8; 32],
    pub __packed: },

//
// enum dp_pixelformat - drm DP Pixel encoding formats
//
// This enum is used to indicate DP VSC SDP Pixel encoding formats.
// It is based on DP 1.4 spec [Table 2-117: VSC SDP Payload for DB16 through
// DB18]
//
// @DP_PIXELFORMAT_RGB: RGB pixel encoding format
// @DP_PIXELFORMAT_YUV444: YCbCr 4:4:4 pixel encoding format
// @DP_PIXELFORMAT_YUV422: YCbCr 4:2:2 pixel encoding format
// @DP_PIXELFORMAT_YUV420: YCbCr 4:2:0 pixel encoding format
// @DP_PIXELFORMAT_Y_ONLY: Y Only pixel encoding format
// @DP_PIXELFORMAT_RAW: RAW pixel encoding format
// @DP_PIXELFORMAT_RESERVED: Reserved pixel encoding format
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dp_pixelformat {
    DP_PIXELFORMAT_RGB = 0,
    DP_PIXELFORMAT_YUV444 = 0x1,
    DP_PIXELFORMAT_YUV422 = 0x2,
    DP_PIXELFORMAT_YUV420 = 0x3,
    DP_PIXELFORMAT_Y_ONLY = 0x4,
    DP_PIXELFORMAT_RAW = 0x5,
    DP_PIXELFORMAT_RESERVED = 0x6,
}

//
// enum dp_colorimetry - drm DP Colorimetry formats
//
// This enum is used to indicate DP VSC SDP Colorimetry formats.
// It is based on DP 1.4 spec [Table 2-117: VSC SDP Payload for DB16 through
// DB18] and a name of enum member follows enum drm_colorimetry definition.
//
// @DP_COLORIMETRY_DEFAULT: sRGB (IEC 61966-2-1) or
// ITU-R BT.601 colorimetry format
// @DP_COLORIMETRY_RGB_WIDE_FIXED: RGB wide gamut fixed point colorimetry format
// @DP_COLORIMETRY_BT709_YCC: ITU-R BT.709 colorimetry format
// @DP_COLORIMETRY_RGB_WIDE_FLOAT: RGB wide gamut floating point
// (scRGB (IEC 61966-2-2)) colorimetry format
// @DP_COLORIMETRY_XVYCC_601: xvYCC601 colorimetry format
// @DP_COLORIMETRY_OPRGB: OpRGB colorimetry format
// @DP_COLORIMETRY_XVYCC_709: xvYCC709 colorimetry format
// @DP_COLORIMETRY_DCI_P3_RGB: DCI-P3 (SMPTE RP 431-2) colorimetry format
// @DP_COLORIMETRY_SYCC_601: sYCC601 colorimetry format
// @DP_COLORIMETRY_RGB_CUSTOM: RGB Custom Color Profile colorimetry format
// @DP_COLORIMETRY_OPYCC_601: opYCC601 colorimetry format
// @DP_COLORIMETRY_BT2020_RGB: ITU-R BT.2020 R' G' B' colorimetry format
// @DP_COLORIMETRY_BT2020_CYCC: ITU-R BT.2020 Y'c C'bc C'rc colorimetry format
// @DP_COLORIMETRY_BT2020_YCC: ITU-R BT.2020 Y' C'b C'r colorimetry format
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dp_colorimetry {
    DP_COLORIMETRY_DEFAULT = 0,
    DP_COLORIMETRY_RGB_WIDE_FIXED = 0x1,
    DP_COLORIMETRY_BT709_YCC = 0x1,
    DP_COLORIMETRY_RGB_WIDE_FLOAT = 0x2,
    DP_COLORIMETRY_XVYCC_601 = 0x2,
    DP_COLORIMETRY_OPRGB = 0x3,
    DP_COLORIMETRY_XVYCC_709 = 0x3,
    DP_COLORIMETRY_DCI_P3_RGB = 0x4,
    DP_COLORIMETRY_SYCC_601 = 0x4,
    DP_COLORIMETRY_RGB_CUSTOM = 0x5,
    DP_COLORIMETRY_OPYCC_601 = 0x5,
    DP_COLORIMETRY_BT2020_RGB = 0x6,
    DP_COLORIMETRY_BT2020_CYCC = 0x6,
    DP_COLORIMETRY_BT2020_YCC = 0x7,
}

//
// enum dp_dynamic_range - drm DP Dynamic Range
//
// This enum is used to indicate DP VSC SDP Dynamic Range.
// It is based on DP 1.4 spec [Table 2-117: VSC SDP Payload for DB16 through
// DB18]
//
// @DP_DYNAMIC_RANGE_VESA: VESA range
// @DP_DYNAMIC_RANGE_CTA: CTA range
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dp_dynamic_range {
    DP_DYNAMIC_RANGE_VESA = 0,
    DP_DYNAMIC_RANGE_CTA = 1,
}

//
// enum dp_content_type - drm DP Content Type
//
// This enum is used to indicate DP VSC SDP Content Types.
// It is based on DP 1.4 spec [Table 2-117: VSC SDP Payload for DB16 through
// DB18]
// CTA-861-G defines content types and expected processing by a sink device
//
// @DP_CONTENT_TYPE_NOT_DEFINED: Not defined type
// @DP_CONTENT_TYPE_GRAPHICS: Graphics type
// @DP_CONTENT_TYPE_PHOTO: Photo type
// @DP_CONTENT_TYPE_VIDEO: Video type
// @DP_CONTENT_TYPE_GAME: Game type
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dp_content_type {
    DP_CONTENT_TYPE_NOT_DEFINED = 0x00,
    DP_CONTENT_TYPE_GRAPHICS = 0x01,
    DP_CONTENT_TYPE_PHOTO = 0x02,
    DP_CONTENT_TYPE_VIDEO = 0x03,
    DP_CONTENT_TYPE_GAME = 0x04,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum operation_mode {
    DP_AS_SDP_AVT_DYNAMIC_VTOTAL = 0x00,
    DP_AS_SDP_AVT_FIXED_VTOTAL = 0x01,
    DP_AS_SDP_FAVT_TRR_NOT_REACHED = 0x02,
    DP_AS_SDP_FAVT_TRR_REACHED = 0x03
}

