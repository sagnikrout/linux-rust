//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dc_dp_types.h
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
// Copyright 2016 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: AMD
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_lane_count {
    LANE_COUNT_UNKNOWN = 0,
    LANE_COUNT_ONE = 1,
    LANE_COUNT_TWO = 2,
    LANE_COUNT_FOUR = 4,
    LANE_COUNT_EIGHT = 8,
    LANE_COUNT_DP_MAX = LANE_COUNT_FOUR
}

// This is actually a reference clock (27MHz) multiplier
// 162MBps bandwidth for 1.62GHz like rate,
// 270MBps for 2.70GHz,
// 324MBps for 3.24Ghz,
// 540MBps for 5.40GHz
// 810MBps for 8.10GHz
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_link_rate {
    LINK_RATE_UNKNOWN = 0,
    LINK_RATE_LOW = 0x06,		// Rate_1 (RBR)  - 1.62 Gbps/Lane
    LINK_RATE_RATE_2 = 0x08,	// Rate_2        - 2.16 Gbps/Lane
    LINK_RATE_RATE_3 = 0x09,	// Rate_3        - 2.43 Gbps/Lane
    LINK_RATE_HIGH = 0x0A,		// Rate_4 (HBR)  - 2.70 Gbps/Lane
    LINK_RATE_RBR2 = 0x0C,		// Rate_5 (RBR2) - 3.24 Gbps/Lane
    LINK_RATE_RATE_6 = 0x10,	// Rate_6        - 4.32 Gbps/Lane
    LINK_RATE_HIGH2 = 0x14,		// Rate_7 (HBR2) - 5.40 Gbps/Lane
    LINK_RATE_RATE_8 = 0x19,	// Rate_8        - 6.75 Gbps/Lane
    LINK_RATE_HIGH3 = 0x1E,		// Rate_9 (HBR3) - 8.10 Gbps/Lane
// Starting from DP2.0 link rate enum directly represents actual
// link rate value in unit of 10 mbps
//
    LINK_RATE_UHBR10 = 1000,	// UHBR10 - 10.0 Gbps/Lane
    LINK_RATE_UHBR13_5 = 1350,	// UHBR13.5 - 13.5 Gbps/Lane
    LINK_RATE_UHBR20 = 2000,	// UHBR20 - 20.0 Gbps/Lane
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_link_spread {
    LINK_SPREAD_DISABLED = 0x00,
// 0.5 % downspread 30 kHz
    LINK_SPREAD_05_DOWNSPREAD_30KHZ = 0x10,
// 0.5 % downspread 33 kHz
    LINK_SPREAD_05_DOWNSPREAD_33KHZ = 0x11
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_voltage_swing {
    VOLTAGE_SWING_LEVEL0 = 0,	/* direct HW translation! */
    VOLTAGE_SWING_LEVEL1,
    VOLTAGE_SWING_LEVEL2,
    VOLTAGE_SWING_LEVEL3,
    VOLTAGE_SWING_MAX_LEVEL = VOLTAGE_SWING_LEVEL3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_pre_emphasis {
    PRE_EMPHASIS_DISABLED = 0,	/* direct HW translation! */
    PRE_EMPHASIS_LEVEL1,
    PRE_EMPHASIS_LEVEL2,
    PRE_EMPHASIS_LEVEL3,
    PRE_EMPHASIS_MAX_LEVEL = PRE_EMPHASIS_LEVEL3
}

// Post Cursor 2 is optional for transmitter
// and it applies only to the main link operating at HBR2
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_post_cursor2 {
    POST_CURSOR2_DISABLED = 0,	/* direct HW translation! */
    POST_CURSOR2_LEVEL1,
    POST_CURSOR2_LEVEL2,
    POST_CURSOR2_LEVEL3,
    POST_CURSOR2_MAX_LEVEL = POST_CURSOR2_LEVEL3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_dp_ffe_preset_level {
    DP_FFE_PRESET_LEVEL0 = 0,
    DP_FFE_PRESET_LEVEL1,
    DP_FFE_PRESET_LEVEL2,
    DP_FFE_PRESET_LEVEL3,
    DP_FFE_PRESET_LEVEL4,
    DP_FFE_PRESET_LEVEL5,
    DP_FFE_PRESET_LEVEL6,
    DP_FFE_PRESET_LEVEL7,
    DP_FFE_PRESET_LEVEL8,
    DP_FFE_PRESET_LEVEL9,
    DP_FFE_PRESET_LEVEL10,
    DP_FFE_PRESET_LEVEL11,
    DP_FFE_PRESET_LEVEL12,
    DP_FFE_PRESET_LEVEL13,
    DP_FFE_PRESET_LEVEL14,
    DP_FFE_PRESET_LEVEL15,
    DP_FFE_PRESET_MAX_LEVEL = DP_FFE_PRESET_LEVEL15,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_dp_training_pattern {
    DP_TRAINING_PATTERN_SEQUENCE_1 = 0,
    DP_TRAINING_PATTERN_SEQUENCE_2,
    DP_TRAINING_PATTERN_SEQUENCE_3,
    DP_TRAINING_PATTERN_SEQUENCE_4,
    DP_TRAINING_PATTERN_VIDEOIDLE,
    DP_128b_132b_TPS1,
    DP_128b_132b_TPS2,
    DP_128b_132b_TPS2_CDS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dp_link_encoding {
    DP_UNKNOWN_ENCODING = 0,
    DP_8b_10b_ENCODING = 1,
    DP_128b_132b_ENCODING = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dp_test_link_rate {
    DP_TEST_LINK_RATE_RBR		= 0x06,
    DP_TEST_LINK_RATE_RATE_2    = 0x08,	// Rate_2        - 2.16 Gbps/Lane
    DP_TEST_LINK_RATE_RATE_3    = 0x09,	// Rate_3        - 2.43 Gbps/Lane
    DP_TEST_LINK_RATE_HBR		= 0x0A,
    DP_TEST_LINK_RATE_RBR2      = 0x0C,	// Rate_5 (RBR2) - 3.24 Gbps/Lane
    DP_TEST_LINK_RATE_RATE_6    = 0x10,	// Rate_6        - 4.32 Gbps/Lane
    DP_TEST_LINK_RATE_HBR2		= 0x14,
    DP_TEST_LINK_RATE_RATE_8    = 0x19,	// Rate_8        - 6.75 Gbps/Lane
    DP_TEST_LINK_RATE_HBR3		= 0x1E,
    DP_TEST_LINK_RATE_UHBR10	= 0x01,
    DP_TEST_LINK_RATE_UHBR20	= 0x02,
    DP_TEST_LINK_RATE_UHBR13_5_LEGACY	= 0x03, /* For backward compatibility*/
    DP_TEST_LINK_RATE_UHBR13_5	= 0x04,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_link_settings {
    pub lane_count: dc_lane_count,
    pub link_rate: dc_link_rate,
    pub link_spread: dc_link_spread,
    pub use_link_rate_set: bool,
    pub link_rate_set: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_tunnel_settings {
    pub should_enable_dp_tunneling: bool,
    pub should_use_dp_bw_allocation: bool,
    pub cm_id: u8,
    pub group_id: u8,
    pub bw_granularity: u32,
    pub estimated_bw: u32,
    pub allocated_bw: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dc_dp_ffe_preset {
    pub 4: uint8_t level :,
    pub 1: uint8_t reserved :,
    pub 1: uint8_t no_preshoot :,
    pub 1: uint8_t no_deemphasis :,
    pub 1: uint8_t method2 :,
    pub settings: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_lane_settings {
    pub VOLTAGE_SWING: dc_voltage_swing,
    pub PRE_EMPHASIS: dc_pre_emphasis,
    pub POST_CURSOR2: dc_post_cursor2,
    pub FFE_PRESET: dc_dp_ffe_preset,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_link_training_overrides {
    pub voltage_swing: *mut dc_voltage_swing,
    pub pre_emphasis: *mut dc_pre_emphasis,
    pub post_cursor2: *mut dc_post_cursor2,
    pub ffe_preset: *mut dc_dp_ffe_preset,
    pub cr_pattern_time: *mut u16,
    pub eq_pattern_time: *mut u16,
    pub pattern_for_cr: *mut dc_dp_training_pattern,
    pub pattern_for_eq: *mut dc_dp_training_pattern,
    pub downspread: *mut dc_link_spread,
    pub alternate_scrambler_reset: *mut bool,
    pub enhanced_framing: *mut bool,
    pub mst_enable: *mut bool,
    pub fec_enable: *mut bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union payload_table_update_status {
    pub VC_PAYLOAD_TABLE_UPDATED:1: u8,
    pub ACT_HANDLED:1: u8,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dpcd_rev {
    pub MINOR:4: u8,
    pub MAJOR:4: u8,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union max_lane_count {
    pub MAX_LANE_COUNT:5: u8,
    pub POST_LT_ADJ_REQ_SUPPORTED:1: u8,
    pub TPS3_SUPPORTED:1: u8,
    pub ENHANCED_FRAME_CAP:1: u8,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union max_down_spread {
    pub MAX_DOWN_SPREAD:1: u8,
    pub RESERVED:5: u8,
    pub NO_AUX_HANDSHAKE_LINK_TRAINING:1: u8,
    pub TPS4_SUPPORTED:1: u8,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mstm_cap {
    pub MST_CAP:1: u8,
    pub RESERVED:7: u8,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union lane_count_set {
    pub LANE_COUNT_SET:5: u8,
    pub POST_LT_ADJ_REQ_GRANTED:1: u8,
    pub RESERVED:1: u8,
    pub ENHANCED_FRAMING:1: u8,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union lane_status {
    pub CR_DONE_0:1: u8,
    pub CHANNEL_EQ_DONE_0:1: u8,
    pub SYMBOL_LOCKED_0:1: u8,
    pub RESERVED0:1: u8,
    pub CR_DONE_1:1: u8,
    pub CHANNEL_EQ_DONE_1:1: u8,
    pub SYMBOL_LOCKED_1:1: u8,
    pub RESERVED_1:1: u8,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union device_service_irq {
    pub REMOTE_CONTROL_CMD_PENDING:1: u8,
    pub AUTOMATED_TEST:1: u8,
    pub CP_IRQ:1: u8,
    pub MCCS_IRQ:1: u8,
    pub DOWN_REP_MSG_RDY:1: u8,
    pub UP_REQ_MSG_RDY:1: u8,
    pub SINK_SPECIFIC:1: u8,
    pub reserved:1: u8,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union sink_count {
    pub SINK_COUNT:6: u8,
    pub CPREADY:1: u8,
    pub RESERVED:1: u8,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union lane_align_status_updated {
    pub INTERLANE_ALIGN_DONE:1: u8,
    pub POST_LT_ADJ_REQ_IN_PROGRESS:1: u8,
    pub EQ_INTERLANE_ALIGN_DONE_128b_132b:1: u8,
    pub CDS_INTERLANE_ALIGN_DONE_128b_132b:1: u8,
    pub LT_FAILED_128b_132b:1: u8,
    pub RESERVED:1: u8,
    pub DOWNSTREAM_PORT_STATUS_CHANGED:1: u8,
    pub LINK_STATUS_UPDATED:1: u8,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union link_service_irq_vector_esi0 {
    pub DP_LINK_RX_CAP_CHANGED:1: u8,
    pub DP_LINK_STATUS_CHANGED:1: u8,
    pub DP_LINK_STREAM_STATUS_CHANGED:1: u8,
    pub DP_LINK_HDMI_LINK_STATUS_CHANGED:1: u8,
    pub DP_LINK_CONNECTED_OFF_ENTRY_REQUESTED:1: u8,
    pub DP_LINK_TUNNELING_IRQ:1: u8,
    pub reserved:2: u8,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union lane_adjust {
    pub VOLTAGE_SWING_LANE:2: u8,
    pub PRE_EMPHASIS_LANE:2: u8,
    pub RESERVED:4: u8,
    pub bits: },
    pub :4: uint8_t PRESET_VALUE,
    pub :4: uint8_t RESERVED,
    pub tx_ffe: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dpcd_training_pattern {
    pub TRAINING_PATTERN_SET:4: u8,
    pub RECOVERED_CLOCK_OUT_EN:1: u8,
    pub SCRAMBLING_DISABLE:1: u8,
    pub SYMBOL_ERROR_COUNT_SEL:2: u8,
    pub v1_4: },
    pub TRAINING_PATTERN_SET:2: u8,
    pub LINK_QUAL_PATTERN_SET:2: u8,
    pub RESERVED:4: u8,
    pub v1_3: },
    pub raw: u8,
}

// Training Lane is used to configure downstream DP device's voltage swing
// The DPCD addresses are from 0x103 to 0x106
#[repr(C)]
#[derive(Copy, Clone)]
pub union dpcd_training_lane {
    pub VOLTAGE_SWING_SET:2: u8,
    pub MAX_SWING_REACHED:1: u8,
    pub PRE_EMPHASIS_SET:2: u8,
    pub MAX_PRE_EMPHASIS_REACHED:1: u8,
    pub RESERVED:2: u8,
    pub bits: },
    pub :4: uint8_t PRESET_VALUE,
    pub :4: uint8_t RESERVED,
    pub tx_ffe: },
    pub raw: u8,
}

// TMDS-converter related
#[repr(C)]
#[derive(Copy, Clone)]
pub union dwnstream_port_caps_byte0 {
    pub DWN_STRM_PORTX_TYPE:3: u8,
    pub DWN_STRM_PORTX_HPD:1: u8,
    pub RESERVERD:4: u8,
    pub bits: },
    pub raw: u8,
}

// these are the detailed types stored at DWN_STRM_PORTX_CAP (00080h)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpcd_downstream_port_detailed_type {
    DOWN_STREAM_DETAILED_DP = 0,
    DOWN_STREAM_DETAILED_VGA,
    DOWN_STREAM_DETAILED_DVI,
    DOWN_STREAM_DETAILED_HDMI,
    DOWN_STREAM_DETAILED_NONDDC,/* has no EDID (TV,CV)*/
    DOWN_STREAM_DETAILED_DP_PLUS_PLUS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dwnstream_port_caps_byte2 {
    pub MAX_BITS_PER_COLOR_COMPONENT:2: u8,
    pub MAX_ENCODED_LINK_BW_SUPPORT:3: u8,
    pub SOURCE_CONTROL_MODE_SUPPORT:1: u8,
    pub CONCURRENT_LINK_BRING_UP_SEQ_SUPPORT:1: u8,
    pub RESERVED:1: u8,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dp_downstream_port_present {
    pub byte: u8,
    pub PORT_PRESENT:1: u8,
    pub PORT_TYPE:2: u8,
    pub FMT_CONVERSION:1: u8,
    pub DETAILED_CAPS:1: u8,
    pub RESERVED:3: u8,
    pub fields: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dwnstream_port_caps_byte3_dvi {
    pub RESERVED1:1: u8,
    pub DUAL_LINK:1: u8,
    pub HIGH_COLOR_DEPTH:1: u8,
    pub RESERVED2:5: u8,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dwnstream_port_caps_byte3_hdmi {
    pub FRAME_SEQ_TO_FRAME_PACK:1: u8,
    pub YCrCr422_PASS_THROUGH:1: u8,
    pub YCrCr420_PASS_THROUGH:1: u8,
    pub YCrCr422_CONVERSION:1: u8,
    pub YCrCr420_CONVERSION:1: u8,
    pub RESERVED:3: u8,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hdmi_encoded_link_bw {
    pub 0: uint8_t FRL_MODE:1; // Bit,
    pub BW_9Gbps:1: u8,
    pub BW_18Gbps:1: u8,
    pub BW_24Gbps:1: u8,
    pub BW_32Gbps:1: u8,
    pub BW_40Gbps:1: u8,
    pub BW_48Gbps:1: u8,
    pub 7: uint8_t FRL_LINK_TRAINING_FINISHED:1; // Bit,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hdmi_tx_link_status {
    pub HDMI_TX_LINK_ACTIVE_STATUS:1: u8,
    pub HDMI_TX_READY_STATUS:1: u8,
    pub RESERVED:6: u8,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union autonomous_mode_and_frl_link_status {
    pub FRL_LT_IN_PROGRESS_STATUS:1: u8,
    pub FRL_LT_LINK_CONFIG_IN_PROGRESS:3: u8,
    pub RESERVED:1: u8,
    pub FALLBACK_POLICY:1: u8,
    pub FALLBACK_POLICY_VALID:1: u8,
    pub REGULATED_AUTONOMOUS_MODE_SUPPORTED:1: u8,
    pub bits: },
    pub raw: u8,
}

// 4-byte structure for detailed capabilities of a down-stream port
#[repr(C)]
#[derive(Copy, Clone)]
pub union dwnstream_portxcaps {
    pub byte0: dwnstream_port_caps_byte0,
    pub //byte1: unsigned char max_TMDS_clock;,
    pub byte2: dwnstream_port_caps_byte2,
    pub byteDVI: dwnstream_port_caps_byte3_dvi,
    pub byteHDMI: dwnstream_port_caps_byte3_hdmi,
    pub byte3: },
    pub bytes: },
    pub raw: [c_uchar; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union downstream_port {
    pub present:1: c_uchar,
    pub type:2: c_uchar,
    pub format_conv:1: c_uchar,
    pub detailed_caps:1: c_uchar,
    pub reserved:3: c_uchar,
    pub bits: },
    pub raw: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union sink_status {
    pub RX_PORT0_STATUS:1: u8,
    pub RX_PORT1_STATUS:1: u8,
    pub RESERVED:6: u8,
    pub bits: },
    pub raw: u8,
}

// 7-byte structure corresponding to 6 registers (200h-205h)
// and LINK_SERVICE_IRQ_ESI0 (2005h) for tunneling IRQ
// read during handling of HPD-IRQ
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union hpd_irq_data {
    pub /: *mut *mut sink_count sink_cnt;/ 200h,
    pub /: *mut *mut device_service_irq device_service_irq;/ 201h,
    pub /: *mut *mut lane_status lane01_status;/ 202h,
    pub /: *mut *mut lane_status lane23_status;/ 203h,
    pub /: *mut *mut lane_align_status_updated lane_status_updated;/ 204h,
    pub /: *mut *mut sink_status sink_status;/ 205h,
    pub /: *mut *mut link_service_irq_vector_esi0 link_service_irq_esi0;/ 2005h,
    pub bytes: },
    pub raw: [u8; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union down_stream_port_count {
    pub DOWN_STR_PORT_COUNT:4: u8,
    pub 0s.*/: *mut *mut uint8_t RESERVED:2; /Bits 5:4 = RESERVED. Read all,
// Bit 6 = MSA_TIMING_PAR_IGNORED
    pub IGNORE_MSA_TIMING_PARAM:1: u8,
// Bit 7 = OUI Support
    pub OUI_SUPPORT:1: u8,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union down_spread_ctrl {
    pub 0s*/: *mut *mut uint8_t RESERVED1:4;/ Bit 3:0 = RESERVED. Read all,
// Bits 4 = SPREAD_AMP. Spreading amplitude
    pub SPREAD_AMP:1: u8,
    pub 0s*/: *mut *mut uint8_t RESERVED2:1;/Bit 5 = RESERVED. Read all,
// Bit 6 = FIXED_VTOTAL_AS_SDP_EN_IN_PR_ACTIVE.
    pub FIXED_VTOTAL_AS_SDP_EN_IN_PR_ACTIVE:1: u8,
// Bit 7 = MSA_TIMING_PAR_IGNORE_EN
    pub IGNORE_MSA_TIMING_PARAM:1: u8,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dpcd_edp_config {
    pub PANEL_MODE_EDP:1: u8,
    pub FRAMING_CHANGE_ENABLE:1: u8,
    pub RESERVED:5: u8,
    pub PANEL_SELF_TEST_ENABLE:1: u8,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_device_vendor_id {
    pub OUI*/: *mut *mut uint8_t ieee_oui[3];/24-bit IEEE,
    pub name*/: *mut *mut uint8_t ieee_device_id[6];/usually 6-byte ASCII,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_sink_hw_fw_revision {
    pub ieee_hw_rev: u8,
    pub ieee_fw_rev: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpcd_vendor_signature {
    pub is_valid: bool,
#[repr(C)]
#[derive(Copy, Clone)]
pub union dpcd_ieee_vendor_signature {
    pub OUI*/: *mut *mut uint8_t ieee_oui[3];/24-bit IEEE,
    pub name*/: *mut *mut uint8_t ieee_device_id[6];/usually 6-byte ASCII,
    pub ieee_hw_rev: u8,
    pub ieee_fw_rev: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpcd_amd_signature {
    pub AMD_IEEE_TxSignature_byte1: u8,
    pub AMD_IEEE_TxSignature_byte2: u8,
    pub AMD_IEEE_TxSignature_byte3: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpcd_amd_device_id {
    pub device_id_byte1: u8,
    pub device_id_byte2: u8,
    pub zero: [u8; 4],
    pub dce_version: u8,
    pub dal_version_byte1: u8,
    pub dal_version_byte2: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct target_luminance_value {
    pub byte0: u8,
    pub byte1: u8,
    pub byte2: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpcd_source_backlight_set {
    pub byte0: u8,
    pub byte1: u8,
    pub byte2: u8,
    pub byte3: u8,
    pub backlight_level_millinits: },
    pub byte0: u8,
    pub byte1: u8,
    pub backlight_transition_time_ms: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dpcd_source_backlight_get {
    pub /: *mut *mut uint32_t backlight_millinits_peak; / 326h,
    pub /: *mut *mut uint32_t backlight_millinits_avg; / 32Ah,
    pub bytes: },
    pub raw: [u8; 8],
}

// DPCD register of DP receiver capability field bits-
#[repr(C)]
#[derive(Copy, Clone)]
pub union edp_configuration_cap {
    pub ALT_SCRAMBLER_RESET:1: u8,
    pub FRAMING_CHANGE:1: u8,
    pub RESERVED:1: u8,
    pub DPCD_DISPLAY_CONTROL_CAPABLE:1: u8,
    pub RESERVED2:4: u8,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dprx_feature {
    pub 1.3+: uint8_t GTC_CAP:1; // bit 0: DP,
    pub 1.4: uint8_t SST_SPLIT_SDP_CAP:1; // bit 1: DP,
    pub 1.3+: uint8_t AV_SYNC_CAP:1; // bit 2: DP,
    pub 1.3+: uint8_t VSC_SDP_COLORIMETRY_SUPPORTED:1; // bit 3: DP,
    pub 1.4: uint8_t VSC_EXT_VESA_SDP_SUPPORTED:1; // bit 4: DP,
    pub 1.4: uint8_t VSC_EXT_VESA_SDP_CHAINING_SUPPORTED:1; // bit 5: DP,
    pub 1.4: uint8_t VSC_EXT_CEA_SDP_SUPPORTED:1; // bit 6: DP,
    pub 1.4: uint8_t VSC_EXT_CEA_SDP_CHAINING_SUPPORTED:1; // bit 7: DP,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union training_aux_rd_interval {
    pub TRAINIG_AUX_RD_INTERVAL:7: u8,
    pub EXT_RECEIVER_CAP_FIELD_PRESENT:1: u8,
    pub bits: },
    pub raw: u8,
}

// Automated test structures
#[repr(C)]
#[derive(Copy, Clone)]
pub union test_request {
    pub :1: uint8_t LINK_TRAINING,
    pub :1: uint8_t LINK_TEST_PATTRN,
    pub :1: uint8_t EDID_READ,
    pub :1: uint8_t PHY_TEST_PATTERN,
    pub :2: uint8_t PHY_TEST_CHANNEL_CODING_TYPE,
    pub :1: uint8_t AUDIO_TEST_PATTERN,
    pub :1: uint8_t TEST_AUDIO_DISABLED_VIDEO,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union test_response {
    pub :1: uint8_t ACK,
    pub :1: uint8_t NO_ACK,
    pub EDID_CHECKSUM_WRITE:1: u8,
    pub :5: uint8_t RESERVED,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union phy_test_pattern {
// This field is 7 bits for DP2.0
    pub :7: uint8_t PATTERN,
    pub :1: uint8_t RESERVED,
    pub bits: },
    pub raw: u8,
}

// States of Compliance Test Specification (CTS DP1.2).
#[repr(C)]
#[derive(Copy, Clone)]
pub union compliance_test_state {
    pub 1: unsigned char STEREO_3D_RUNNING :,
    pub 7: unsigned char RESERVED :,
    pub bits: },
    pub raw: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union link_test_pattern {
// dpcd_link_test_patterns
    pub :2: unsigned char PATTERN,
    pub RESERVED:6: c_uchar,
    pub bits: },
    pub raw: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union test_misc {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpcd_test_misc_bits {
    pub :1: unsigned char SYNC_CLOCK,
// dpcd_test_color_format
    pub :2: unsigned char CLR_FORMAT,
// dpcd_test_dyn_range
    pub :1: unsigned char DYN_RANGE,
    pub :1: unsigned char YCBCR_COEFS,
// dpcd_test_bit_depth
    pub :3: unsigned char BPC,
    pub bits: },
    pub raw: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union audio_test_mode {
    pub :4: unsigned char sampling_rate,
    pub :4: unsigned char channel_count,
    pub bits: },
    pub raw: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union audio_test_pattern_period {
    pub :4: unsigned char pattern_period,
    pub :4: unsigned char reserved,
    pub bits: },
    pub raw: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audio_test_pattern_type {
    pub value: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_audio_test_data_flags {
    pub :1: uint8_t test_requested,
    pub :1: uint8_t disable_video,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_audio_test_data {
    pub flags: dp_audio_test_data_flags,
    pub sampling_rate: u8,
    pub channel_count: u8,
    pub pattern_type: u8,
    pub pattern_period: [u8; 8],
}

// FEC capability DPCD register field bits-
#[repr(C)]
#[derive(Copy, Clone)]
pub union dpcd_fec_capability {
    pub FEC_CAPABLE:1: u8,
    pub UNCORRECTED_BLOCK_ERROR_COUNT_CAPABLE:1: u8,
    pub CORRECTED_BLOCK_ERROR_COUNT_CAPABLE:1: u8,
    pub BIT_ERROR_COUNT_CAPABLE:1: u8,
    pub PARITY_BLOCK_ERROR_COUNT_CAPABLE:1: u8,
    pub ARITY_BIT_ERROR_COUNT_CAPABLE:1: u8,
    pub FEC_RUNNING_INDICATOR_SUPPORTED:1: u8,
    pub FEC_ERROR_REPORTING_POLICY_SUPPORTED:1: u8,
    pub bits: },
    pub raw: u8,
}

// DSC capability DPCD register field bits-
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpcd_dsc_support {
    pub :1: uint8_t DSC_SUPPORT,
    pub :1: uint8_t DSC_PASSTHROUGH_SUPPORT,
    pub :6: uint8_t RESERVED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpcd_dsc_algorithm_revision {
    pub :4: uint8_t DSC_VERSION_MAJOR,
    pub :4: uint8_t DSC_VERSION_MINOR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpcd_dsc_rc_buffer_block_size {
    pub :2: uint8_t RC_BLOCK_BUFFER_SIZE,
    pub :6: uint8_t RESERVED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpcd_dsc_slice_capability1 {
    pub :1: uint8_t ONE_SLICE_PER_DP_DSC_SINK_DEVICE,
    pub :1: uint8_t TWO_SLICES_PER_DP_DSC_SINK_DEVICE,
    pub :1: uint8_t RESERVED,
    pub :1: uint8_t FOUR_SLICES_PER_DP_DSC_SINK_DEVICE,
    pub :1: uint8_t SIX_SLICES_PER_DP_DSC_SINK_DEVICE,
    pub :1: uint8_t EIGHT_SLICES_PER_DP_DSC_SINK_DEVICE,
    pub :1: uint8_t TEN_SLICES_PER_DP_DSC_SINK_DEVICE,
    pub :1: uint8_t TWELVE_SLICES_PER_DP_DSC_SINK_DEVICE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpcd_dsc_line_buffer_bit_depth {
    pub :4: uint8_t LINE_BUFFER_BIT_DEPTH,
    pub :4: uint8_t RESERVED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpcd_dsc_block_prediction_support {
    pub BLOCK_PREDICTION_SUPPORT:1: u8,
    pub :7: uint8_t RESERVED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpcd_maximum_bits_per_pixel_supported_by_the_decompressor {
    pub :7: uint8_t MAXIMUM_BITS_PER_PIXEL_SUPPORTED_BY_THE_DECOMPRESSOR_LOW,
    pub :7: uint8_t MAXIMUM_BITS_PER_PIXEL_SUPPORTED_BY_THE_DECOMPRESSOR_HIGH,
    pub :2: uint8_t RESERVED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpcd_dsc_decoder_color_format_capabilities {
    pub :1: uint8_t RGB_SUPPORT,
    pub :1: uint8_t Y_CB_CR_444_SUPPORT,
    pub :1: uint8_t Y_CB_CR_SIMPLE_422_SUPPORT,
    pub :1: uint8_t Y_CB_CR_NATIVE_422_SUPPORT,
    pub :1: uint8_t Y_CB_CR_NATIVE_420_SUPPORT,
    pub :3: uint8_t RESERVED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpcd_dsc_decoder_color_depth_capabilities {
    pub :1: uint8_t RESERVED0,
    pub :1: uint8_t EIGHT_BITS_PER_COLOR_SUPPORT,
    pub :1: uint8_t TEN_BITS_PER_COLOR_SUPPORT,
    pub :1: uint8_t TWELVE_BITS_PER_COLOR_SUPPORT,
    pub :4: uint8_t RESERVED1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpcd_peak_dsc_throughput_dsc_sink {
    pub THROUGHPUT_MODE_0:4: u8,
    pub THROUGHPUT_MODE_1:4: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpcd_dsc_slice_capabilities_2 {
    pub :1: uint8_t SIXTEEN_SLICES_PER_DSC_SINK_DEVICE,
    pub :1: uint8_t TWENTY_SLICES_PER_DSC_SINK_DEVICE,
    pub :1: uint8_t TWENTYFOUR_SLICES_PER_DSC_SINK_DEVICE,
    pub :5: uint8_t RESERVED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpcd_bits_per_pixel_increment {
    pub :3: uint8_t INCREMENT_OF_BITS_PER_PIXEL_SUPPORTED,
    pub :5: uint8_t RESERVED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dpcd_dsc_basic_capabilities {
    pub dsc_support: dpcd_dsc_support,
    pub dsc_algorithm_revision: dpcd_dsc_algorithm_revision,
    pub dsc_rc_buffer_block_size: dpcd_dsc_rc_buffer_block_size,
    pub dsc_rc_buffer_size: u8,
    pub dsc_slice_capabilities_1: dpcd_dsc_slice_capability1,
    pub dsc_line_buffer_bit_depth: dpcd_dsc_line_buffer_bit_depth,
    pub dsc_block_prediction_support: dpcd_dsc_block_prediction_support,
    pub maximum_bits_per_pixel_supported_by_the_decompressor: dpcd_maximum_bits_per_pixel_supported_by_the_decompressor,
    pub dsc_decoder_color_format_capabilities: dpcd_dsc_decoder_color_format_capabilities,
    pub dsc_decoder_color_depth_capabilities: dpcd_dsc_decoder_color_depth_capabilities,
    pub peak_dsc_throughput_dsc_sink: dpcd_peak_dsc_throughput_dsc_sink,
    pub dsc_maximum_slice_width: u8,
    pub dsc_slice_capabilities_2: dpcd_dsc_slice_capabilities_2,
    pub reserved: u8,
    pub bits_per_pixel_increment: dpcd_bits_per_pixel_increment,
    pub fields: },
    pub raw: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dpcd_dsc_branch_decoder_capabilities {
    pub BRANCH_OVERALL_THROUGHPUT_0: u8,
    pub BRANCH_OVERALL_THROUGHPUT_1: u8,
    pub BRANCH_MAX_LINE_WIDTH: u8,
    pub fields: },
    pub raw: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpcd_dsc_capabilities {
    pub dsc_basic_caps: dpcd_dsc_basic_capabilities,
    pub dsc_branch_decoder_caps: dpcd_dsc_branch_decoder_capabilities,
}

// These parameters are from PSR capabilities reported by Sink DPCD
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psr_caps {
    pub psr_version: c_uchar,
    pub psr_rfb_setup_time: c_uint,
    pub psr_exit_link_training_required: bool,
    pub edp_revision: c_uchar,
    pub support_ver: c_uchar,
    pub su_granularity_required: bool,
    pub y_coordinate_required: bool,
    pub su_y_granularity: u8,
    pub alpm_cap: bool,
    pub standby_support: bool,
    pub rate_control_caps: u8,
    pub psr_power_opt_flag: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dpcd_dprx_feature_enumeration_list_cont_1 {
    pub ADAPTIVE_SYNC_SDP_SUPPORT:1: u8,
    pub 1: uint8_t AS_SDP_FIRST_HALF_LINE_OR_3840_PIXEL_CYCLE_WINDOW_NOT_SUPPORTED:,
    pub 2: uint8_t RESERVED0:,
    pub 1: uint8_t VSC_EXT_SDP_VER1_SUPPORT:,
    pub 3: uint8_t RESERVED1:,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adaptive_sync_caps {
    pub dp_adap_sync_caps: dpcd_dprx_feature_enumeration_list_cont_1,
}

// Length of router topology ID read from DPCD in bytes.
pub const DPCD_USB4_TOPOLOGY_ID_LEN: c_int = 5;
// DPCD[0xE000D] DP_TUNNELING_CAPABILITIES SUPPORT register.
#[repr(C)]
#[derive(Copy, Clone)]
pub union dp_tun_cap_support {
    pub :1: uint8_t dp_tunneling,
    pub :5: uint8_t rsvd,
    pub :1: uint8_t panel_replay_tun_opt,
    pub :1: uint8_t dpia_bw_alloc,
    pub bits: },
    pub raw: u8,
}

// DPCD[0xE000E] DP_IN_ADAPTER_INFO register.
#[repr(C)]
#[derive(Copy, Clone)]
pub union dpia_info {
    pub :6: uint8_t dpia_num,
    pub :2: uint8_t rsvd,
    pub bits: },
    pub raw: u8,
}

// DPCD[0xE0020] USB4_DRIVER_BW_CAPABILITY register.
#[repr(C)]
#[derive(Copy, Clone)]
pub union usb4_driver_bw_cap {
    pub :7: uint8_t rsvd,
    pub :1: uint8_t driver_bw_alloc_support,
    pub bits: },
    pub raw: u8,
}

// DPCD[0xE0021] DP_IN_ADAPTER_TUNNEL_INFORMATION register.
#[repr(C)]
#[derive(Copy, Clone)]
pub union dpia_tunnel_info {
    pub :3: uint8_t group_id,
    pub :5: uint8_t rsvd,
    pub bits: },
    pub raw: u8,
}

// DP Tunneling over USB4
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpcd_usb4_dp_tunneling_info {
    pub dp_tun_cap: dp_tun_cap_support,
    pub dpia_info: dpia_info,
    pub driver_bw_cap: usb4_driver_bw_cap,
    pub dpia_tunnel_info: dpia_tunnel_info,
    pub usb4_driver_id: u8,
    pub usb4_topology_id: [u8; DPCD_USB4_TOPOLOGY_ID_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dp_main_line_channel_coding_cap {
    pub :1: uint8_t DP_8b_10b_SUPPORTED,
    pub :1: uint8_t DP_128b_132b_SUPPORTED,
    pub :6: uint8_t RESERVED,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dp_main_link_channel_coding_lttpr_cap {
    pub :1: uint8_t DP_128b_132b_SUPPORTED,
    pub :7: uint8_t RESERVED,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dp_128b_132b_supported_link_rates {
    pub :1: uint8_t UHBR10,
    pub :1: uint8_t UHBR20,
    pub UHBR13_5:1: u8,
    pub RESERVED:5: u8,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dp_128b_132b_supported_lttpr_link_rates {
    pub :1: uint8_t UHBR10,
    pub :1: uint8_t UHBR20,
    pub UHBR13_5:1: u8,
    pub RESERVED:5: u8,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dp_alpm_lttpr_cap {
    pub :1: uint8_t AUX_LESS_ALPM_SUPPORTED,
    pub :1: uint8_t ASSR_SUPPORTED,
    pub :6: uint8_t RESERVED,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dp_sink_video_fallback_formats {
    pub :1: uint8_t dp_1024x768_60Hz_24bpp_support,
    pub :1: uint8_t dp_1280x720_60Hz_24bpp_support,
    pub :1: uint8_t dp_1920x1080_60Hz_24bpp_support,
    pub :5: uint8_t RESERVED,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dp_receive_port0_cap {
    pub :1: uint8_t RESERVED,
    pub :1: uint8_t LOCAL_EDID_PRESENT,
    pub ASSOCIATED_TO_PRECEDING_PORT:1: u8,
    pub :1: uint8_t HBLANK_EXPANSION_CAPABLE,
    pub :1: uint8_t BUFFER_SIZE_UNIT,
    pub :1: uint8_t BUFFER_SIZE_PER_PORT,
    pub :1: uint8_t HBLANK_REDUCTION_CAPABLE,
    pub RESERVED2:1: u8,
    pub BUFFER_SIZE:8: u8,
    pub bits: },
    pub raw: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dpcd_max_uncompressed_pixel_rate_cap {
    pub :15: uint16_t max_uncompressed_pixel_rate_cap,
    pub :1: uint16_t valid,
    pub bits: },
    pub raw: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dp_fec_capability1 {
    pub :1: uint8_t AGGREGATED_ERROR_COUNTERS_CAPABLE,
    pub :7: uint8_t RESERVED,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dp_cable_id {
    pub :2: uint8_t UHBR10_20_CAPABILITY,
    pub :1: uint8_t UHBR13_5_CAPABILITY,
    pub :3: uint8_t CABLE_TYPE,
    pub :2: uint8_t RESERVED,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_color_depth_caps {
    pub :1: uint8_t support_6bpc,
    pub :1: uint8_t support_8bpc,
    pub :1: uint8_t support_10bpc,
    pub :1: uint8_t support_12bpc,
    pub :1: uint8_t support_16bpc,
    pub :3: uint8_t RESERVED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_encoding_format_caps {
    pub :1: uint8_t support_rgb,
    pub support_ycbcr444:1: u8,
    pub support_ycbcr422:1: u8,
    pub support_ycbcr420:1: u8,
    pub :4: uint8_t RESERVED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dp_dfp_cap_ext {
    pub supported: u8,
    pub max_pixel_rate_in_mps: [u8; 2],
    pub max_video_h_active_width: [u8; 2],
    pub max_video_v_active_height: [u8; 2],
    pub encoding_format_caps: dp_encoding_format_caps,
    pub rgb_color_depth_caps: dp_color_depth_caps,
    pub ycbcr444_color_depth_caps: dp_color_depth_caps,
    pub ycbcr422_color_depth_caps: dp_color_depth_caps,
    pub ycbcr420_color_depth_caps: dp_color_depth_caps,
    pub fields: },
    pub raw: [u8; 12],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dp_128b_132b_training_aux_rd_interval {
    pub :7: uint8_t VALUE,
    pub :1: uint8_t UNIT,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union edp_alpm_caps {
    pub :1: uint8_t AUX_WAKE_ALPM_CAP,
    pub :1: uint8_t PM_STATE_2A_SUPPORT,
    pub :1: uint8_t AUX_LESS_ALPM_CAP,
    pub :1: uint8_t AUX_LESS_ALPM_ML_PHY_SLEEP_STATUS_SUPPORTED,
    pub :4: uint8_t RESERVED,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union edp_psr_dpcd_caps {
    pub :1: uint8_t LINK_TRAINING_ON_EXIT_NOT_REQUIRED,
    pub :3: uint8_t PSR_SETUP_TIME,
    pub :1: uint8_t Y_COORDINATE_REQUIRED,
    pub :1: uint8_t SU_GRANULARITY_REQUIRED,
    pub :1: uint8_t FRAME_SYNC_IS_NOT_NEEDED_FOR_SU,
    pub :1: uint8_t RESERVED,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct edp_psr_info {
    pub psr_version: u8,
    pub psr_dpcd_caps: edp_psr_dpcd_caps,
    pub psr2_su_y_granularity_cap: u8,
    pub force_psrsu_cap: u8,
    pub psr_active_vtotal_control_cap: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct replay_info {
    pub pixel_deviation_per_line: u8,
    pub max_deviation_line: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dprx_states {
    pub cable_id_written: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dpcd_panel_replay_capability_supported {
    pub :1: unsigned char PANEL_REPLAY_SUPPORT,
    pub :1: unsigned char SELECTIVE_UPDATE_SUPPORT,
    pub :1: unsigned char EARLY_TRANSPORT_SUPPORT,
    pub :5: unsigned char RESERVED,
    pub bits: },
    pub raw: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dpcd_panel_replay_capability {
    pub :2: unsigned char RESERVED,
    pub :1: unsigned char DSC_DECODE_NOT_SUPPORTED,
    pub :1: unsigned char ASYNC_VIDEO_TIMING_NOT_SUPPORTED,
    pub :1: unsigned char DSC_CRC_OF_MULTI_SU_SUPPORTED,
    pub :1: unsigned char PR_SU_GRANULARITY_NEEDED,
    pub :1: unsigned char SU_Y_GRANULARITY_EXT_CAP_SUPPORTED,
    pub :1: unsigned char LINK_OFF_SUPPORTED_IN_PR_ACTIVE,
    pub bits: },
    pub raw: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpcd_panel_replay_selective_update_info {
    pub pr_su_x_granularity: u16,
    pub pr_su_y_granularity: u8,
    pub pr_su_y_granularity_extended_caps: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpcd_downstream_port_max_bpc {
    DOWN_STREAM_MAX_8BPC = 0,
    DOWN_STREAM_MAX_10BPC,
    DOWN_STREAM_MAX_12BPC,
    DOWN_STREAM_MAX_16BPC
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum link_training_offset {
    DPRX                = 0,
    LTTPR_PHY_REPEATER1 = 1,
    LTTPR_PHY_REPEATER2 = 2,
    LTTPR_PHY_REPEATER3 = 3,
    LTTPR_PHY_REPEATER4 = 4,
    LTTPR_PHY_REPEATER5 = 5,
    LTTPR_PHY_REPEATER6 = 6,
    LTTPR_PHY_REPEATER7 = 7,
    LTTPR_PHY_REPEATER8 = 8
}

pub const MAX_REPEATER_CNT: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_lttpr_caps {
    pub revision: dpcd_rev,
    pub mode: u8,
    pub max_lane_count: u8,
    pub max_link_rate: u8,
    pub phy_repeater_cnt: u8,
    pub max_ext_timeout: u8,
    pub main_link_channel_coding: dp_main_link_channel_coding_lttpr_cap,
    pub supported_128b_132b_rates: dp_128b_132b_supported_lttpr_link_rates,
    pub alpm: dp_alpm_lttpr_cap,
    pub aux_rd_interval: [u8; MAX_REPEATER_CNT],
    pub host: uint8_t lttpr_ieee_oui[3]; // Always read from closest LTTPR to,
    pub host: uint8_t lttpr_device_id[6]; // Always read from closest LTTPR to,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_dongle_dfp_cap_ext {
    pub supported: bool,
    pub max_pixel_rate_in_mps: u16,
    pub max_video_h_active_width: u16,
    pub max_video_v_active_height: u16,
    pub encoding_format_caps: dp_encoding_format_caps,
    pub rgb_color_depth_caps: dp_color_depth_caps,
    pub ycbcr444_color_depth_caps: dp_color_depth_caps,
    pub ycbcr422_color_depth_caps: dp_color_depth_caps,
    pub ycbcr420_color_depth_caps: dp_color_depth_caps,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_dongle_caps {
// dongle type (DP converter, CV smart dongle)
    pub dongle_type: display_dongle_type,
    pub extendedCapValid: bool,
// If dongle_type == DISPLAY_DONGLE_DP_HDMI_CONVERTER,
    pub is_dp_hdmi_s3d_converter: bool,
    pub is_dp_hdmi_ycbcr422_pass_through: bool,
    pub is_dp_hdmi_ycbcr420_pass_through: bool,
    pub is_dp_hdmi_ycbcr422_converter: bool,
    pub is_dp_hdmi_ycbcr420_converter: bool,
    pub dp_hdmi_max_bpc: u32,
    pub dp_hdmi_max_pixel_clk_in_khz: u32,
    pub dp_hdmi_frl_max_link_bw_in_kbps: u32,
    pub dp_hdmi_regulated_autonomous_mode_support: u32,
    pub dfp_cap_ext: dc_dongle_dfp_cap_ext,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpcd_caps {
    pub dpcd_rev: dpcd_rev,
    pub max_ln_count: max_lane_count,
    pub max_down_spread: max_down_spread,
    pub dprx_feature: dprx_feature,
// valid only for eDP v1.4 or higher
    pub edp_supported_link_rates_count: u8,
    pub edp_supported_link_rates: [dc_link_rate; 8],
// dongle type (DP converter, CV smart dongle)
    pub dongle_type: display_dongle_type,
    pub is_dongle_type_one: bool,
// branch device or sink device
    pub is_branch_dev: bool,
// Dongle's downstream count.
    pub sink_count: sink_count,
    pub is_mst_capable: bool,
// If dongle_type == DISPLAY_DONGLE_DP_HDMI_CONVERTER,
    pub dongle_caps: dc_dongle_caps,
    pub sink_dev_id: u32,
    pub sink_dev_id_str: [i8; 6],
    pub sink_hw_revision: i8,
    pub sink_fw_revision: [i8; 2],
    pub branch_dev_id: u32,
    pub branch_dev_name: [i8; 6],
    pub branch_hw_revision: i8,
    pub branch_fw_revision: [i8; 2],
    pub branch_vendor_specific_data: [i8; 4],
    pub allow_invalid_MSA_timing_param: bool,
    pub panel_mode_edp: bool,
    pub dpcd_display_control_capable: bool,
    pub ext_receiver_cap_field_present: bool,
    pub set_power_state_capable_edp: bool,
    pub dynamic_backlight_capable_edp: bool,
    pub fec_cap: dpcd_fec_capability,
    pub dsc_caps: dpcd_dsc_capabilities,
    pub lttpr_caps: dc_lttpr_caps,
    pub adaptive_sync_caps: adaptive_sync_caps,
    pub usb4_dp_tun_info: dpcd_usb4_dp_tunneling_info,
    pub max_uncompressed_pixel_rate_cap: dpcd_max_uncompressed_pixel_rate_cap,
    pub dp_128b_132b_supported_link_rates: dp_128b_132b_supported_link_rates,
    pub channel_coding_cap: dp_main_line_channel_coding_cap,
    pub fallback_formats: dp_sink_video_fallback_formats,
    pub fec_cap1: dp_fec_capability1,
    pub panel_luminance_control: bool,
    pub cable_id: dp_cable_id,
    pub edp_rev: u8,
    pub alpm_caps: edp_alpm_caps,
    pub psr_info: edp_psr_info,
    pub pr_info: replay_info,
    pub vesa_replay_caps_supported: dpcd_panel_replay_capability_supported,
    pub vesa_replay_caps: dpcd_panel_replay_capability,
    pub vesa_replay_su_info: dpcd_panel_replay_selective_update_info,
    pub edp_oled_emission_rate: u16,
    pub receive_port0_cap: dp_receive_port0_cap,
// Indicates the number of SST links supported by MSO (Multi-Stream Output)
    pub mso_cap_sst_links_supported: u8,
    pub dp_edp_general_cap_2: u8,
    pub drr_granularity: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dpcd_sink_ext_caps {
// 0 - Sink supports backlight adjust via PWM during SDR/HDR mode
// 1 - Sink supports backlight adjust via AUX during SDR/HDR mode.
//
    pub 1: uint8_t sdr_aux_backlight_control :,
    pub 1: uint8_t hdr_aux_backlight_control :,
    pub 2: uint8_t reserved_1 :,
    pub 1: uint8_t oled :,
    pub 1: uint8_t reserved_2 :,
    pub 1: uint8_t miniled :,
    pub 1: uint8_t emission_output :,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_link_fec_state {
    dc_link_fec_not_ready,
    dc_link_fec_ready,
    dc_link_fec_enabled
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dpcd_psr_configuration {
    pub 1: unsigned char ENABLE :,
    pub 1: unsigned char TRANSMITTER_ACTIVE_IN_PSR :,
    pub 1: unsigned char CRC_VERIFICATION :,
    pub 1: unsigned char FRAME_CAPTURE_INDICATION :,
// For eDP 1.4, PSR v2
    pub 1: unsigned char LINE_CAPTURE_INDICATION :,
// For eDP 1.4, PSR v2
    pub 1: unsigned char IRQ_HPD_WITH_CRC_ERROR :,
    pub 1: unsigned char ENABLE_PSR2 :,
    pub 1: unsigned char EARLY_TRANSPORT_ENABLE :,
    pub bits: },
    pub raw: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union replay_enable_and_configuration {
    pub :1: unsigned char FREESYNC_PANEL_REPLAY_MODE,
    pub :1: unsigned char TIMING_DESYNC_ERROR_VERIFICATION,
    pub :1: unsigned char STATE_TRANSITION_ERROR_DETECTION,
    pub :1: unsigned char FSFT_ENABLED,
    pub :1: unsigned char FRAME_SKIPPING_ERROR_DETECTION,
    pub :1: unsigned char FRAME_SKIPPING_ENABLE,
    pub :2: unsigned char RESERVED,
    pub bits: },
    pub raw: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dpcd_replay_configuration {
    pub 1: unsigned char STATE_TRANSITION_ERROR_STATUS :,
    pub 1: unsigned char DESYNC_ERROR_STATUS :,
    pub 3: unsigned char SINK_DEVICE_REPLAY_STATUS :,
    pub 2: unsigned char SINK_FRAME_LOCKED :,
    pub 1: unsigned char FRAME_SKIPPING_ERROR_STATUS :,
    pub bits: },
    pub raw: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union panel_replay_enable_and_configuration_1 {
    pub :1: unsigned char PANEL_REPLAY_ENABLE,
    pub :1: unsigned char PANEL_REPLAY_CRC_ENABLE,
    pub :1: unsigned char IRQ_HPD_ASSDP_MISSING,
    pub :1: unsigned char IRQ_HPD_VSCSDP_UNCORRECTABLE_ERROR,
    pub :1: unsigned char IRQ_HPD_RFB_ERROR,
    pub :1: unsigned char IRQ_HPD_ACTIVE_FRAME_CRC_ERROR,
    pub :1: unsigned char PANEL_REPLAY_SELECTIVE_UPDATE_ENABLE,
    pub :1: unsigned char PANEL_REPLAY_EARLY_TRANSPORT_ENABLE,
    pub bits: },
    pub raw: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union panel_replay_enable_and_configuration_2 {
    pub :1: unsigned char SINK_REFRESH_RATE_UNLOCK_GRANTED,
    pub :1: unsigned char RESERVED,
    pub :1: unsigned char SU_Y_GRANULARITY_EXT_VALUE_ENABLED,
    pub :4: unsigned char SU_Y_GRANULARITY_EXT_VALUE,
    pub :1: unsigned char SU_REGION_SCAN_LINE_CAPTURE_INDICATION,
    pub bits: },
    pub raw: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dpcd_alpm_configuration {
    pub 1: unsigned char ENABLE :,
    pub 1: unsigned char IRQ_HPD_ENABLE :,
    pub 1: unsigned char ALPM_MODE_SEL :,
    pub 1: unsigned char ACDS_PERIOD_DURATION :,
    pub 4: unsigned char RESERVED :,
    pub bits: },
    pub raw: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dpcd_sink_active_vtotal_control_mode {
    pub 1: unsigned char ENABLE :,
    pub 7: unsigned char RESERVED :,
    pub bits: },
    pub raw: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pr_error_status {
    pub :1: unsigned char LINK_CRC_ERROR,
    pub :1: unsigned char RFB_STORAGE_ERROR,
    pub :1: unsigned char VSC_SDP_ERROR,
    pub :1: unsigned char ASSDP_MISSING_ERROR,
    pub :4: unsigned char RESERVED,
    pub bits: },
    pub raw: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union psr_error_status {
    pub :1: unsigned char LINK_CRC_ERROR,
    pub :1: unsigned char RFB_STORAGE_ERROR,
    pub :1: unsigned char VSC_SDP_ERROR,
    pub :5: unsigned char RESERVED,
    pub bits: },
    pub raw: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union psr_sink_psr_status {
    pub :3: unsigned char SINK_SELF_REFRESH_STATUS,
    pub :5: unsigned char RESERVED,
    pub bits: },
    pub raw: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct edp_trace_power_timestamps {
    pub poweroff: u64,
    pub poweron: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_trace_lt_counts {
    pub total: c_uint,
    pub fail: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum link_training_result {
    LINK_TRAINING_SUCCESS,
    LINK_TRAINING_CR_FAIL_LANE0,
    LINK_TRAINING_CR_FAIL_LANE1,
    LINK_TRAINING_CR_FAIL_LANE23,
// CR DONE bit is cleared during EQ step
    LINK_TRAINING_EQ_FAIL_CR,
// CR DONE bit is cleared but LANE0_CR_DONE is set during EQ step
    LINK_TRAINING_EQ_FAIL_CR_PARTIAL,
// other failure during EQ step
    LINK_TRAINING_EQ_FAIL_EQ,
    LINK_TRAINING_LQA_FAIL,
// one of the CR,EQ or symbol lock is dropped
    LINK_TRAINING_LINK_LOSS,
// Abort link training (because sink unplugged)
    LINK_TRAINING_ABORT,
    DP_128b_132b_LT_FAILED,
    DP_128b_132b_MAX_LOOP_COUNT_REACHED,
    DP_128b_132b_CHANNEL_EQ_DONE_TIMEOUT,
    DP_128b_132b_CDS_DONE_TIMEOUT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_trace_lt {
    pub counts: dp_trace_lt_counts,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_trace_timestamps {
    pub start: c_ulonglong,
    pub end: c_ulonglong,
    pub timestamps: },
    pub result: link_training_result,
    pub is_logged: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_trace {
    pub detect_lt_trace: dp_trace_lt,
    pub commit_lt_trace: dp_trace_lt,
    pub link_loss_count: c_uint,
    pub is_initialized: bool,
    pub edp_trace_power_timestamps: edp_trace_power_timestamps,
}

// TODO - This is a temporary location for any new DPCD definitions.
// We should move these to drm_dp header.
//

pub const DP_LINK_SQUARE_PATTERN: c_uint = 0x10F;

pub const DP_CABLE_ATTRIBUTES_UPDATED_BY_DPRX: c_uint = 0x2217;

pub const DP_CABLE_ATTRIBUTES_UPDATED_BY_DPTX: c_uint = 0x110;

pub const DPCD_MAX_UNCOMPRESSED_PIXEL_RATE_CAP: c_uint = 0x221c;

pub const DP_LTTPR_ALPM_CAPABILITIES: c_uint = 0xF0009;

pub const DP_REGULATED_AUTONOMOUS_MODE_SUPPORTED_AND_HDMI_LINK_TRAINING_STATUS: c_uint = 0x303C;

pub const DP_REPEATER_CONFIGURATION_AND_STATUS_SIZE: c_uint = 0x50;

pub const DP_BRANCH_VENDOR_SPECIFIC_START: c_uint = 0x50C;

pub const DP_LTTPR_IEEE_OUI: c_uint = 0xF003D;

pub const DP_LTTPR_DEVICE_ID: c_uint = 0xF0040;

// USB4 DPCD BW Allocation Registers Chapter 10.7

pub const DP_TUNNELING_CAPABILITIES: c_uint = 0xE000D /* 1.4a */;

pub const USB4_DRIVER_ID: c_uint = 0xE000F /* 1.4a */;

pub const USB4_DRIVER_BW_CAPABILITY: c_uint = 0xE0020 /* 1.4a */;

pub const DP_IN_ADAPTER_TUNNEL_INFO: c_uint = 0xE0021 /* 1.4a */;

pub const DP_BW_GRANULALITY: c_uint = 0xE0022 /* 1.4a */;

pub const ESTIMATED_BW: c_uint = 0xE0023 /* 1.4a */;

pub const ALLOCATED_BW: c_uint = 0xE0024 /* 1.4a */;

pub const DP_TUNNELING_STATUS: c_uint = 0xE0025 /* 1.4a */;

pub const DP_TUNNELING_MAX_LINK_RATE: c_uint = 0xE0028 /* 1.4a */;

pub const DP_TUNNELING_MAX_LANE_COUNT: c_uint = 0xE0029 /* 1.4a */;

pub const DPTX_BW_ALLOCATION_MODE_CONTROL: c_uint = 0xE0030 /* 1.4a */;

pub const REQUESTED_BW: c_uint = 0xE0031 /* 1.4a */;

