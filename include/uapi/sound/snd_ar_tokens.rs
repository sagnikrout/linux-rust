//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/sound/snd_ar_tokens.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

pub const APM_SUB_GRAPH_PERF_MODE_LOW_POWER: c_uint = 0x1;
pub const APM_SUB_GRAPH_PERF_MODE_LOW_LATENCY: c_uint = 0x2;
pub const APM_SUB_GRAPH_DIRECTION_TX: c_uint = 0x1;
pub const APM_SUB_GRAPH_DIRECTION_RX: c_uint = 0x2;
// Scenario ID Audio Playback
pub const APM_SUB_GRAPH_SID_AUDIO_PLAYBACK: c_uint = 0x1;
// Scenario ID Audio Record
pub const APM_SUB_GRAPH_SID_AUDIO_RECORD: c_uint = 0x2;
// Scenario ID Voice call.
pub const APM_SUB_GRAPH_SID_VOICE_CALL: c_uint = 0x3;
// container capability ID Pre/Post Processing (PP)
pub const APM_CONTAINER_CAP_ID_PP: c_uint = 0x1;
// container capability ID Compression/Decompression (CD)
pub const APM_CONTAINER_CAP_ID_CD: c_uint = 0x2;
// container capability ID End Point(EP)
pub const APM_CONTAINER_CAP_ID_EP: c_uint = 0x3;
// container capability ID Offload (OLC)
pub const APM_CONTAINER_CAP_ID_OLC: c_uint = 0x4;
// container graph position Stream
pub const APM_CONT_GRAPH_POS_STREAM: c_uint = 0x1;
// container graph position Per Stream Per Device
pub const APM_CONT_GRAPH_POS_PER_STR_PER_DEV: c_uint = 0x2;
// container graph position Stream-Device
pub const APM_CONT_GRAPH_POS_STR_DEV: c_uint = 0x3;
// container graph position Global Device
pub const APM_CONT_GRAPH_POS_GLOBAL_DEV: c_uint = 0x4;
pub const APM_PROC_DOMAIN_ID_MDSP: c_uint = 0x1;
pub const APM_PROC_DOMAIN_ID_ADSP: c_uint = 0x2;
pub const APM_PROC_DOMAIN_ID_SDSP: c_uint = 0x4;
pub const APM_PROC_DOMAIN_ID_CDSP: c_uint = 0x5;
pub const PCM_INTERLEAVED: c_int = 1;
pub const PCM_DEINTERLEAVED_PACKED: c_int = 2;
pub const PCM_DEINTERLEAVED_UNPACKED: c_int = 3;
pub const AR_I2S_WS_SRC_EXTERNAL: c_int = 0;
pub const AR_I2S_WS_SRC_INTERNAL: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ar_event_types {
    AR_EVENT_NONE = 0,
    AR_PGA_DAPM_EVENT
}

//
// Kcontrol IDs
//
pub const SND_SOC_AR_TPLG_FE_BE_GRAPH_CTL_MIX: c_int = 256;
pub const SND_SOC_AR_TPLG_VOL_CTL: c_int = 257;
//
// %AR_TKN_U32_SUB_GRAPH_INSTANCE_ID:		Sub Graph Instance Id
//
// %AR_TKN_U32_SUB_GRAPH_PERF_MODE:		Performance mode of subgraph
// APM_SUB_GRAPH_PERF_MODE_LOW_POWER = 1,
// APM_SUB_GRAPH_PERF_MODE_LOW_LATENCY = 2
//
// %AR_TKN_U32_SUB_GRAPH_DIRECTION:		Direction of subgraph
// APM_SUB_GRAPH_DIRECTION_TX = 1,
// APM_SUB_GRAPH_DIRECTION_RX = 2
//
// %AR_TKN_U32_SUB_GRAPH_SCENARIO_ID:		Scenario ID for subgraph
// APM_SUB_GRAPH_SID_AUDIO_PLAYBACK = 1,
// APM_SUB_GRAPH_SID_AUDIO_RECORD = 2,
// APM_SUB_GRAPH_SID_VOICE_CALL = 3
//
// %AR_TKN_U32_CONTAINER_INSTANCE_ID:		Container Instance ID
//
// %AR_TKN_U32_CONTAINER_CAPABILITY_ID:		Container capability ID
// APM_CONTAINER_CAP_ID_PP = 1,
// APM_CONTAINER_CAP_ID_CD = 2,
// APM_CONTAINER_CAP_ID_EP = 3,
// APM_CONTAINER_CAP_ID_OLC = 4
//
// %AR_TKN_U32_CONTAINER_STACK_SIZE:		Stack size in the container.
//
// %AR_TKN_U32_CONTAINER_GRAPH_POS:		Graph Position
// APM_CONT_GRAPH_POS_STREAM = 1,
// APM_CONT_GRAPH_POS_PER_STR_PER_DEV = 2,
// APM_CONT_GRAPH_POS_STR_DEV = 3,
// APM_CONT_GRAPH_POS_GLOBAL_DEV = 4
//
// %AR_TKN_U32_CONTAINER_PROC_DOMAIN:		Processor domain of container
// APM_PROC_DOMAIN_ID_MDSP = 1,
// APM_PROC_DOMAIN_ID_ADSP = 2,
// APM_PROC_DOMAIN_ID_SDSP = 4,
// APM_PROC_DOMAIN_ID_CDSP = 5
//
// %AR_TKN_U32_MODULE_ID:			Module ID
//
// %AR_TKN_U32_MODULE_INSTANCE_ID:		Module Instance ID.
//
// %AR_TKN_U32_MODULE_MAX_IP_PORTS:		Module maximum input ports
//
// %AR_TKN_U32_MODULE_MAX_OP_PORTS:		Module maximum output ports.
//
// %AR_TKN_U32_MODULE_IN_PORTS:			Number of in ports
//
// %AR_TKN_U32_MODULE_OUT_PORTS:		Number of out ports.
//
// %AR_TKN_U32_MODULE_SRC_OP_PORT_ID:		Source module output port ID
//
// %AR_TKN_U32_MODULE_DST_IN_PORT_ID:		Destination module input port ID
//
// %AR_TKN_U32_MODULE_HW_IF_IDX:		Interface index types for I2S/LPAIF
//
// %AR_TKN_U32_MODULE_HW_IF_TYPE:		Interface type
// LPAIF = 0,
// LPAIF_RXTX = 1,
// LPAIF_WSA = 2,
// LPAIF_VA = 3,
// LPAIF_AXI = 4
// Possible values for MI2S
// I2S_INTF_TYPE_PRIMARY = 0,
// I2S_INTF_TYPE_SECONDARY = 1,
// I2S_INTF_TYPE_TERTIARY = 2,
// I2S_INTF_TYPE_QUATERNARY = 3,
// I2S_INTF_TYPE_QUINARY = 4,
//
// %AR_TKN_U32_MODULE_FMT_INTERLEAVE:		PCM Interleaving
// PCM_INTERLEAVED = 1,
// PCM_DEINTERLEAVED_PACKED = 2,
// PCM_DEINTERLEAVED_UNPACKED = 3
//
// %AR_TKN_U32_MODULE_FMT_DATA:			data format
// FIXED POINT = 1,
// IEC60958 PACKETIZED = 3,
// IEC60958 PACKETIZED NON LINEAR = 8,
// COMPR OVER PCM PACKETIZED = 7,
// IEC61937 PACKETIZED = 2,
// GENERIC COMPRESSED = 5
//
// %AR_TKN_U32_MODULE_FMT_SAMPLE_RATE:		sample rate
//
// %AR_TKN_U32_MODULE_FMT_BIT_DEPTH:		bit depth
//
// %AR_TKN_U32_MODULE_SD_LINE_IDX:		I2S serial data line idx
// I2S_SD0 = 1,
// I2S_SD1 = 2,
// I2S_SD2 = 3,
// I2S_SD3 = 4,
// I2S_QUAD01 = 5,
// I2S_QUAD23 = 6,
// I2S_6CHS = 7,
// I2S_8CHS = 8
//
// %AR_TKN_U32_MODULE_WS_SRC:			Word Select Source
// AR_I2S_WS_SRC_EXTERNAL = 0,
// AR_I2S_WS_SRC_INTERNAL = 1,
//
// %AR_TKN_U32_MODULE_FRAME_SZ_FACTOR:		Frame size factor
//
// %AR_TKN_U32_MODULE_LOG_CODE:			Log Module Code
//
// %AR_TKN_U32_MODULE_LOG_TAP_POINT_ID:		logging tap point of this module
//
// %AR_TKN_U32_MODULE_LOG_MODE:			logging mode
// LOG_WAIT = 0,
// LOG_IMMEDIATELY = 1
//
// %AR_TKN_U16_MODULE_SYNC_SRC:			Frame sync source
// AR_AUDIO_IF_SYNC_SRC_EXTERNAL = 0,
// AR_AUDIO_IF_SYNC_SRC_INTERNAL = 1
//
// %AR_TKN_U16_MODULE_CTRL_DATA_OUT_ENABLE:	Enable data-out tri-state control
// AR_AUDIO_IF_CTRL_DATA_OE_DISABLE = 0,
// AR_AUDIO_IF_CTRL_DATA_OE_ENABLE = 1
//
// %AR_TKN_U32_MODULE_SLOT_MASK:			Active TDM slot bitmask
//
// %AR_TKN_U16_MODULE_NSLOTS_PER_FRAME:		Number of slots per TDM frame
//
// %AR_TKN_U16_MODULE_SLOT_WIDTH:		Slot width in bits (16 or 32)
//
// %AR_TKN_U16_MODULE_SYNC_MODE:			Frame sync mode
// AR_AUDIO_IF_FRAME_SYNC_MODE_SHORT = 0,
// AR_AUDIO_IF_FRAME_SYNC_MODE_ONE_SLOT = 1,
// AR_AUDIO_IF_FRAME_SYNC_MODE_LONG = 2
//
// %AR_TKN_U16_MODULE_CTRL_INVERT_SYNC_PULSE:	Invert frame sync pulse polarity
// AR_AUDIO_IF_SYNC_NORMAL = 0,
// AR_AUDIO_IF_SYNC_INVERTED = 1
//
// %AR_TKN_U16_MODULE_CTRL_SYNC_DATA_DELAY:	Data delay relative to frame sync
// AR_AUDIO_IF_DATA_DELAY_NONE = 0,
// AR_AUDIO_IF_DATA_DELAY_1_CYCLE = 1,
// AR_AUDIO_IF_DATA_DELAY_2_CYCLE = 2
//
// %AR_TKN_U16_MODULE_INTF_MODE:			Audio IF interface mode
// AR_AUDIO_IF_INTF_MODE_TDM = 0,
// AR_AUDIO_IF_INTF_MODE_PCM = 1,
// AR_AUDIO_IF_INTF_MODE_I2S = 2
//
// %AR_TKN_U16_MODULE_QAIF_TYPE:			QAIF hardware port type index
// AR_AUDIO_IF_QAIF = 0,
// AR_AUDIO_IF_QAIF_VA = 1
//
// %AR_TKN_U32_MODULE_ACTIVE_LANE_MASK:		Active lane bitmask for multi-lane
//
// %AR_TKN_U32_MODULE_FRAME_SYNC_RATE:		Frame sync rate in Hz
//
// %AR_TKN_U16_MODULE_BIT_CLK_TYPE:		Bit clock type
// AR_AUDIO_IF_BIT_CLK_INTERNAL = 0,
// AR_AUDIO_IF_BIT_CLK_EXTERNAL = 1,
// AR_AUDIO_IF_BIT_CLK_SKIP = 2
//
// %AR_TKN_U8_MODULE_INV_INT_BIT_CLK:		Invert internal bit clock
// AR_AUDIO_IF_CLK_NORMAL = 0,
// AR_AUDIO_IF_CLK_INVERTED = 1
//
// %AR_TKN_U8_MODULE_INV_EXT_BIT_CLK:		Invert external bit clock
// AR_AUDIO_IF_CLK_NORMAL = 0,
// AR_AUDIO_IF_CLK_INVERTED = 1
//
// %AR_TKN_DAI_INDEX:				dai index
//
// DAI Tokens
pub const AR_TKN_DAI_INDEX: c_int = 1;
// SUB GRAPH Tokens
pub const AR_TKN_U32_SUB_GRAPH_INSTANCE_ID: c_int = 2;
pub const AR_TKN_U32_SUB_GRAPH_PERF_MODE: c_int = 3;
pub const AR_TKN_U32_SUB_GRAPH_DIRECTION: c_int = 4;
pub const AR_TKN_U32_SUB_GRAPH_SCENARIO_ID: c_int = 5;
// Container Tokens
pub const AR_TKN_U32_CONTAINER_INSTANCE_ID: c_int = 100;
pub const AR_TKN_U32_CONTAINER_CAPABILITY_ID: c_int = 101;
pub const AR_TKN_U32_CONTAINER_STACK_SIZE: c_int = 102;
pub const AR_TKN_U32_CONTAINER_GRAPH_POS: c_int = 103;
pub const AR_TKN_U32_CONTAINER_PROC_DOMAIN: c_int = 104;
// Module Tokens
pub const AR_TKN_U32_MODULE_ID: c_int = 200;
pub const AR_TKN_U32_MODULE_INSTANCE_ID: c_int = 201;
pub const AR_TKN_U32_MODULE_MAX_IP_PORTS: c_int = 202;
pub const AR_TKN_U32_MODULE_MAX_OP_PORTS: c_int = 203;

pub const AR_TKN_U32_MODULE_SRC_OP_PORT_ID: c_int = 206;
pub const AR_TKN_U32_MODULE_DST_IN_PORT_ID: c_int = 207;
pub const AR_TKN_U32_MODULE_SRC_INSTANCE_ID: c_int = 208;
pub const AR_TKN_U32_MODULE_DST_INSTANCE_ID: c_int = 209;
pub const AR_TKN_U32_MODULE_SRC_OP_PORT_ID1: c_int = 210;
pub const AR_TKN_U32_MODULE_DST_IN_PORT_ID1: c_int = 211;
pub const AR_TKN_U32_MODULE_DST_INSTANCE_ID1: c_int = 212;
pub const AR_TKN_U32_MODULE_SRC_OP_PORT_ID2: c_int = 213;
pub const AR_TKN_U32_MODULE_DST_IN_PORT_ID2: c_int = 214;
pub const AR_TKN_U32_MODULE_DST_INSTANCE_ID2: c_int = 215;
pub const AR_TKN_U32_MODULE_SRC_OP_PORT_ID3: c_int = 216;
pub const AR_TKN_U32_MODULE_DST_IN_PORT_ID3: c_int = 217;
pub const AR_TKN_U32_MODULE_DST_INSTANCE_ID3: c_int = 218;
pub const AR_TKN_U32_MODULE_SRC_OP_PORT_ID4: c_int = 219;
pub const AR_TKN_U32_MODULE_DST_IN_PORT_ID4: c_int = 220;
pub const AR_TKN_U32_MODULE_DST_INSTANCE_ID4: c_int = 221;
pub const AR_TKN_U32_MODULE_SRC_OP_PORT_ID5: c_int = 222;
pub const AR_TKN_U32_MODULE_DST_IN_PORT_ID5: c_int = 223;
pub const AR_TKN_U32_MODULE_DST_INSTANCE_ID5: c_int = 224;
pub const AR_TKN_U32_MODULE_SRC_OP_PORT_ID6: c_int = 225;
pub const AR_TKN_U32_MODULE_DST_IN_PORT_ID6: c_int = 226;
pub const AR_TKN_U32_MODULE_DST_INSTANCE_ID6: c_int = 227;
pub const AR_TKN_U32_MODULE_SRC_OP_PORT_ID7: c_int = 228;
pub const AR_TKN_U32_MODULE_DST_IN_PORT_ID7: c_int = 229;
pub const AR_TKN_U32_MODULE_DST_INSTANCE_ID7: c_int = 230;
pub const AR_TKN_U32_MODULE_HW_IF_IDX: c_int = 250;
pub const AR_TKN_U32_MODULE_HW_IF_TYPE: c_int = 251;
pub const AR_TKN_U32_MODULE_FMT_INTERLEAVE: c_int = 252;
pub const AR_TKN_U32_MODULE_FMT_DATA: c_int = 253;
pub const AR_TKN_U32_MODULE_FMT_SAMPLE_RATE: c_int = 254;
pub const AR_TKN_U32_MODULE_FMT_BIT_DEPTH: c_int = 255;
pub const AR_TKN_U32_MODULE_SD_LINE_IDX: c_int = 256;
pub const AR_TKN_U32_MODULE_WS_SRC: c_int = 257;
pub const AR_TKN_U32_MODULE_FRAME_SZ_FACTOR: c_int = 258;
pub const AR_TKN_U32_MODULE_LOG_CODE: c_int = 259;
pub const AR_TKN_U32_MODULE_LOG_TAP_POINT_ID: c_int = 260;
pub const AR_TKN_U32_MODULE_LOG_MODE: c_int = 261;
pub const AR_TKN_U16_MODULE_SYNC_SRC: c_int = 262;
pub const AR_TKN_U16_MODULE_CTRL_DATA_OUT_ENABLE: c_int = 263;
pub const AR_TKN_U32_MODULE_SLOT_MASK: c_int = 264;
pub const AR_TKN_U16_MODULE_NSLOTS_PER_FRAME: c_int = 265;
pub const AR_TKN_U16_MODULE_SLOT_WIDTH: c_int = 266;
pub const AR_TKN_U16_MODULE_SYNC_MODE: c_int = 267;
pub const AR_TKN_U16_MODULE_CTRL_INVERT_SYNC_PULSE: c_int = 268;
pub const AR_TKN_U16_MODULE_CTRL_SYNC_DATA_DELAY: c_int = 269;
pub const AR_TKN_U16_MODULE_INTF_MODE: c_int = 270;
pub const AR_TKN_U16_MODULE_QAIF_TYPE: c_int = 271;
pub const AR_TKN_U32_MODULE_ACTIVE_LANE_MASK: c_int = 272;
pub const AR_TKN_U32_MODULE_FRAME_SYNC_RATE: c_int = 273;
pub const AR_TKN_U16_MODULE_BIT_CLK_TYPE: c_int = 274;
pub const AR_TKN_U8_MODULE_INV_INT_BIT_CLK: c_int = 275;
pub const AR_TKN_U8_MODULE_INV_EXT_BIT_CLK: c_int = 276;
pub const AR_AUDIO_IF_SYNC_SRC_EXTERNAL: c_int = 0;
pub const AR_AUDIO_IF_SYNC_SRC_INTERNAL: c_int = 1;
pub const AR_AUDIO_IF_CTRL_DATA_OE_DISABLE: c_int = 0;
pub const AR_AUDIO_IF_CTRL_DATA_OE_ENABLE: c_int = 1;
pub const AR_AUDIO_IF_INTF_MODE_TDM: c_int = 0;
pub const AR_AUDIO_IF_INTF_MODE_PCM: c_int = 1;
pub const AR_AUDIO_IF_INTF_MODE_I2S: c_int = 2;
pub const AR_AUDIO_IF_QAIF: c_int = 0;
pub const AR_AUDIO_IF_QAIF_VA: c_int = 1;
pub const AR_AUDIO_IF_FRAME_SYNC_MODE_SHORT: c_int = 0;
pub const AR_AUDIO_IF_FRAME_SYNC_MODE_ONE_SLOT: c_int = 1;
pub const AR_AUDIO_IF_FRAME_SYNC_MODE_LONG: c_int = 2;
pub const AR_AUDIO_IF_SYNC_NORMAL: c_int = 0;
pub const AR_AUDIO_IF_SYNC_INVERTED: c_int = 1;
pub const AR_AUDIO_IF_DATA_DELAY_NONE: c_int = 0;
pub const AR_AUDIO_IF_DATA_DELAY_1_CYCLE: c_int = 1;
pub const AR_AUDIO_IF_DATA_DELAY_2_CYCLE: c_int = 2;
pub const AR_AUDIO_IF_BIT_CLK_INTERNAL: c_int = 0;
pub const AR_AUDIO_IF_BIT_CLK_EXTERNAL: c_int = 1;
pub const AR_AUDIO_IF_BIT_CLK_SKIP: c_int = 2;
pub const AR_AUDIO_IF_CLK_NORMAL: c_int = 0;
pub const AR_AUDIO_IF_CLK_INVERTED: c_int = 1;
pub const SND_SOC_AR_TPLG_MODULE_CFG_TYPE: c_uint = 0x01001006;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct audioreach_module_priv_data {
    pub /: *mut *mut __le32 size; / size in bytes of the array, including all elements,
    pub /: *mut *mut __le32 type; / SND_SOC_AR_TPLG_MODULE_CFG_TYPE,
    pub /: *mut *mut __le32 priv[2]; / Private data for future expansion,
    pub /: *mut *mut __le32 data[0]; / config data,
}
