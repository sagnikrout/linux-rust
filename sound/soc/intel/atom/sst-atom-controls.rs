//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/intel/atom/sst-atom-controls.h
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
// sst-atom-controls.h - Intel MID Platform driver header file
//
// Copyright (C) 2013-14 Intel Corp
// Author: Ramesh Babu <ramesh.babu.koul@intel.com>
// Omair M Abdullah <omair.m.abdullah@intel.com>
// Samreen Nilofer <samreen.nilofer@intel.com>
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//

// define a bit for each mixer input

pub const SST_CMD_SWM_MAX_INPUTS: c_int = 6;
pub const SST_PATH_ID_SHIFT: c_int = 8;
pub const SST_DEFAULT_LOCATION_ID: c_uint = 0xFFFF;
pub const SST_DEFAULT_CELL_NBR: c_uint = 0xFF;
pub const SST_DEFAULT_MODULE_ID: c_uint = 0xFFFF;
//
// Audio DSP Path Ids. Specified by the audio DSP FW
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sst_path_index {
    SST_PATH_INDEX_MODEM_OUT                = (0x00 << SST_PATH_ID_SHIFT),
    SST_PATH_INDEX_CODEC_OUT0               = (0x02 << SST_PATH_ID_SHIFT),
    SST_PATH_INDEX_CODEC_OUT1               = (0x03 << SST_PATH_ID_SHIFT),

    SST_PATH_INDEX_SPROT_LOOP_OUT           = (0x04 << SST_PATH_ID_SHIFT),
    SST_PATH_INDEX_MEDIA_LOOP1_OUT          = (0x05 << SST_PATH_ID_SHIFT),
    SST_PATH_INDEX_MEDIA_LOOP2_OUT          = (0x06 << SST_PATH_ID_SHIFT),

    SST_PATH_INDEX_VOIP_OUT                 = (0x0C << SST_PATH_ID_SHIFT),
    SST_PATH_INDEX_PCM0_OUT                 = (0x0D << SST_PATH_ID_SHIFT),
    SST_PATH_INDEX_PCM1_OUT                 = (0x0E << SST_PATH_ID_SHIFT),
    SST_PATH_INDEX_PCM2_OUT                 = (0x0F << SST_PATH_ID_SHIFT),

    SST_PATH_INDEX_MEDIA0_OUT               = (0x12 << SST_PATH_ID_SHIFT),
    SST_PATH_INDEX_MEDIA1_OUT               = (0x13 << SST_PATH_ID_SHIFT),


// Start of input paths
    SST_PATH_INDEX_MODEM_IN                 = (0x80 << SST_PATH_ID_SHIFT),
    SST_PATH_INDEX_CODEC_IN0                = (0x82 << SST_PATH_ID_SHIFT),
    SST_PATH_INDEX_CODEC_IN1                = (0x83 << SST_PATH_ID_SHIFT),

    SST_PATH_INDEX_SPROT_LOOP_IN            = (0x84 << SST_PATH_ID_SHIFT),
    SST_PATH_INDEX_MEDIA_LOOP1_IN           = (0x85 << SST_PATH_ID_SHIFT),
    SST_PATH_INDEX_MEDIA_LOOP2_IN           = (0x86 << SST_PATH_ID_SHIFT),

    SST_PATH_INDEX_VOIP_IN                  = (0x8C << SST_PATH_ID_SHIFT),

    SST_PATH_INDEX_PCM0_IN                  = (0x8D << SST_PATH_ID_SHIFT),
    SST_PATH_INDEX_PCM1_IN                  = (0x8E << SST_PATH_ID_SHIFT),

    SST_PATH_INDEX_MEDIA0_IN                = (0x8F << SST_PATH_ID_SHIFT),
    SST_PATH_INDEX_MEDIA1_IN                = (0x90 << SST_PATH_ID_SHIFT),
    SST_PATH_INDEX_MEDIA2_IN                = (0x91 << SST_PATH_ID_SHIFT),

    SST_PATH_INDEX_MEDIA3_IN		= (0x9C << SST_PATH_ID_SHIFT),

    SST_PATH_INDEX_RESERVED                 = (0xFF << SST_PATH_ID_SHIFT),
}

//
// path IDs
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sst_swm_inputs {
    SST_SWM_IN_MODEM	= (SST_PATH_INDEX_MODEM_IN	  | SST_DEFAULT_CELL_NBR),
    SST_SWM_IN_CODEC0	= (SST_PATH_INDEX_CODEC_IN0	  | SST_DEFAULT_CELL_NBR),
    SST_SWM_IN_CODEC1	= (SST_PATH_INDEX_CODEC_IN1	  | SST_DEFAULT_CELL_NBR),
    SST_SWM_IN_SPROT_LOOP	= (SST_PATH_INDEX_SPROT_LOOP_IN	  | SST_DEFAULT_CELL_NBR),
    SST_SWM_IN_MEDIA_LOOP1	= (SST_PATH_INDEX_MEDIA_LOOP1_IN  | SST_DEFAULT_CELL_NBR),
    SST_SWM_IN_MEDIA_LOOP2	= (SST_PATH_INDEX_MEDIA_LOOP2_IN  | SST_DEFAULT_CELL_NBR),
    SST_SWM_IN_VOIP		= (SST_PATH_INDEX_VOIP_IN	  | SST_DEFAULT_CELL_NBR),
    SST_SWM_IN_PCM0		= (SST_PATH_INDEX_PCM0_IN	  | SST_DEFAULT_CELL_NBR),
    SST_SWM_IN_PCM1		= (SST_PATH_INDEX_PCM1_IN	  | SST_DEFAULT_CELL_NBR),
    SST_SWM_IN_MEDIA0	= (SST_PATH_INDEX_MEDIA0_IN	  | SST_DEFAULT_CELL_NBR), /* Part of Media Mixer */
    SST_SWM_IN_MEDIA1	= (SST_PATH_INDEX_MEDIA1_IN	  | SST_DEFAULT_CELL_NBR), /* Part of Media Mixer */
    SST_SWM_IN_MEDIA2	= (SST_PATH_INDEX_MEDIA2_IN	  | SST_DEFAULT_CELL_NBR), /* Part of Media Mixer */
    SST_SWM_IN_MEDIA3	= (SST_PATH_INDEX_MEDIA3_IN	  | SST_DEFAULT_CELL_NBR), /* Part of Media Mixer */
    SST_SWM_IN_END		= (SST_PATH_INDEX_RESERVED	  | SST_DEFAULT_CELL_NBR)
}

//
// path IDs
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sst_swm_outputs {
    SST_SWM_OUT_MODEM	= (SST_PATH_INDEX_MODEM_OUT	  | SST_DEFAULT_CELL_NBR),
    SST_SWM_OUT_CODEC0	= (SST_PATH_INDEX_CODEC_OUT0	  | SST_DEFAULT_CELL_NBR),
    SST_SWM_OUT_CODEC1	= (SST_PATH_INDEX_CODEC_OUT1	  | SST_DEFAULT_CELL_NBR),
    SST_SWM_OUT_SPROT_LOOP	= (SST_PATH_INDEX_SPROT_LOOP_OUT  | SST_DEFAULT_CELL_NBR),
    SST_SWM_OUT_MEDIA_LOOP1	= (SST_PATH_INDEX_MEDIA_LOOP1_OUT | SST_DEFAULT_CELL_NBR),
    SST_SWM_OUT_MEDIA_LOOP2	= (SST_PATH_INDEX_MEDIA_LOOP2_OUT | SST_DEFAULT_CELL_NBR),
    SST_SWM_OUT_VOIP	= (SST_PATH_INDEX_VOIP_OUT	  | SST_DEFAULT_CELL_NBR),
    SST_SWM_OUT_PCM0	= (SST_PATH_INDEX_PCM0_OUT	  | SST_DEFAULT_CELL_NBR),
    SST_SWM_OUT_PCM1	= (SST_PATH_INDEX_PCM1_OUT	  | SST_DEFAULT_CELL_NBR),
    SST_SWM_OUT_PCM2	= (SST_PATH_INDEX_PCM2_OUT	  | SST_DEFAULT_CELL_NBR),
    SST_SWM_OUT_MEDIA0	= (SST_PATH_INDEX_MEDIA0_OUT	  | SST_DEFAULT_CELL_NBR), /* Part of Media Mixer */
    SST_SWM_OUT_MEDIA1	= (SST_PATH_INDEX_MEDIA1_OUT	  | SST_DEFAULT_CELL_NBR), /* Part of Media Mixer */
    SST_SWM_OUT_END		= (SST_PATH_INDEX_RESERVED	  | SST_DEFAULT_CELL_NBR),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sst_ipc_msg {
    SST_IPC_IA_CMD = 1,
    SST_IPC_IA_SET_PARAMS,
    SST_IPC_IA_GET_PARAMS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sst_cmd_type {
    SST_CMD_BYTES_SET = 1,
    SST_CMD_BYTES_GET = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sst_task {
    SST_TASK_SBA = 1,
    SST_TASK_MMX = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sst_type {
    SST_TYPE_CMD = 1,
    SST_TYPE_PARAMS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sst_flag {
    SST_FLAG_BLOCKED = 1,
    SST_FLAG_NONBLOCK,
}

//
// Enumeration for indexing the gain cells in VB_SET_GAIN DSP command
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sst_gain_index {
// GAIN IDs for SB task start here
    SST_GAIN_INDEX_CODEC_OUT0,
    SST_GAIN_INDEX_CODEC_OUT1,
    SST_GAIN_INDEX_CODEC_IN0,
    SST_GAIN_INDEX_CODEC_IN1,

    SST_GAIN_INDEX_SPROT_LOOP_OUT,
    SST_GAIN_INDEX_MEDIA_LOOP1_OUT,
    SST_GAIN_INDEX_MEDIA_LOOP2_OUT,

    SST_GAIN_INDEX_PCM0_IN_LEFT,
    SST_GAIN_INDEX_PCM0_IN_RIGHT,

    SST_GAIN_INDEX_PCM1_OUT_LEFT,
    SST_GAIN_INDEX_PCM1_OUT_RIGHT,
    SST_GAIN_INDEX_PCM1_IN_LEFT,
    SST_GAIN_INDEX_PCM1_IN_RIGHT,
    SST_GAIN_INDEX_PCM2_OUT_LEFT,

    SST_GAIN_INDEX_PCM2_OUT_RIGHT,
    SST_GAIN_INDEX_VOIP_OUT,
    SST_GAIN_INDEX_VOIP_IN,

// Gain IDs for MMX task start here
    SST_GAIN_INDEX_MEDIA0_IN_LEFT,
    SST_GAIN_INDEX_MEDIA0_IN_RIGHT,
    SST_GAIN_INDEX_MEDIA1_IN_LEFT,
    SST_GAIN_INDEX_MEDIA1_IN_RIGHT,

    SST_GAIN_INDEX_MEDIA2_IN_LEFT,
    SST_GAIN_INDEX_MEDIA2_IN_RIGHT,

    SST_GAIN_INDEX_GAIN_END
}

//
// Audio DSP module IDs specified by FW spec
// TODO: Update with all modules
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sst_module_id {
    SST_MODULE_ID_PCM		  = 0x0001,
    SST_MODULE_ID_MP3		  = 0x0002,
    SST_MODULE_ID_MP24		  = 0x0003,
    SST_MODULE_ID_AAC		  = 0x0004,
    SST_MODULE_ID_AACP		  = 0x0005,
    SST_MODULE_ID_EAACP		  = 0x0006,
    SST_MODULE_ID_WMA9		  = 0x0007,
    SST_MODULE_ID_WMA10		  = 0x0008,
    SST_MODULE_ID_WMA10P		  = 0x0009,
    SST_MODULE_ID_RA		  = 0x000A,
    SST_MODULE_ID_DDAC3		  = 0x000B,
    SST_MODULE_ID_TRUE_HD		  = 0x000C,
    SST_MODULE_ID_HD_PLUS		  = 0x000D,

    SST_MODULE_ID_SRC		  = 0x0064,
    SST_MODULE_ID_DOWNMIX		  = 0x0066,
    SST_MODULE_ID_GAIN_CELL		  = 0x0067,
    SST_MODULE_ID_SPROT		  = 0x006D,
    SST_MODULE_ID_BASS_BOOST	  = 0x006E,
    SST_MODULE_ID_STEREO_WDNG	  = 0x006F,
    SST_MODULE_ID_AV_REMOVAL	  = 0x0070,
    SST_MODULE_ID_MIC_EQ		  = 0x0071,
    SST_MODULE_ID_SPL		  = 0x0072,
    SST_MODULE_ID_ALGO_VTSV           = 0x0073,
    SST_MODULE_ID_NR		  = 0x0076,
    SST_MODULE_ID_BWX		  = 0x0077,
    SST_MODULE_ID_DRP		  = 0x0078,
    SST_MODULE_ID_MDRP		  = 0x0079,

    SST_MODULE_ID_ANA		  = 0x007A,
    SST_MODULE_ID_AEC		  = 0x007B,
    SST_MODULE_ID_NR_SNS		  = 0x007C,
    SST_MODULE_ID_SER		  = 0x007D,
    SST_MODULE_ID_AGC		  = 0x007E,

    SST_MODULE_ID_CNI		  = 0x007F,
    SST_MODULE_ID_CONTEXT_ALGO_AWARE  = 0x0080,
    SST_MODULE_ID_FIR_24		  = 0x0081,
    SST_MODULE_ID_IIR_24		  = 0x0082,

    SST_MODULE_ID_ASRC		  = 0x0083,
    SST_MODULE_ID_TONE_GEN		  = 0x0084,
    SST_MODULE_ID_BMF		  = 0x0086,
    SST_MODULE_ID_EDL		  = 0x0087,
    SST_MODULE_ID_GLC		  = 0x0088,

    SST_MODULE_ID_FIR_16		  = 0x0089,
    SST_MODULE_ID_IIR_16		  = 0x008A,
    SST_MODULE_ID_DNR		  = 0x008B,

    SST_MODULE_ID_VIRTUALIZER	  = 0x008C,
    SST_MODULE_ID_VISUALIZATION	  = 0x008D,
    SST_MODULE_ID_LOUDNESS_OPTIMIZER  = 0x008E,
    SST_MODULE_ID_REVERBERATION	  = 0x008F,

    SST_MODULE_ID_CNI_TX		  = 0x0090,
    SST_MODULE_ID_REF_LINE		  = 0x0091,
    SST_MODULE_ID_VOLUME		  = 0x0092,
    SST_MODULE_ID_FILT_DCR		  = 0x0094,
    SST_MODULE_ID_SLV		  = 0x009A,
    SST_MODULE_ID_NLF		  = 0x009B,
    SST_MODULE_ID_TNR		  = 0x009C,
    SST_MODULE_ID_WNR		  = 0x009D,

    SST_MODULE_ID_LOG		  = 0xFF00,

    SST_MODULE_ID_TASK		  = 0xFFFF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sst_cmd {
    SBA_IDLE		= 14,
    SBA_VB_SET_SPEECH_PATH	= 26,
    MMX_SET_GAIN		= 33,
    SBA_VB_SET_GAIN		= 33,
    FBA_VB_RX_CNI		= 35,
    MMX_SET_GAIN_TIMECONST	= 36,
    SBA_VB_SET_TIMECONST	= 36,
    SBA_VB_START		= 85,
    SBA_SET_SWM		= 114,
    SBA_SET_MDRP            = 116,
    SBA_HW_SET_SSP		= 117,
    SBA_SET_MEDIA_LOOP_MAP	= 118,
    SBA_SET_MEDIA_PATH	= 119,
    MMX_SET_MEDIA_PATH	= 119,
    SBA_VB_LPRO             = 126,
    SBA_VB_SET_FIR          = 128,
    SBA_VB_SET_IIR          = 129,
    SBA_SET_SSP_SLOT_MAP	= 130,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sst_dsp_switch {
    SST_SWITCH_OFF = 0,
    SST_SWITCH_ON = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sst_path_switch {
    SST_PATH_OFF = 0,
    SST_PATH_ON = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sst_swm_state {
    SST_SWM_OFF = 0,
    SST_SWM_ON = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_destination_id {
#[repr(C)]
#[derive(Copy, Clone)]
pub union sst_location_id {
    pub /: *mut *mut u8 cell_nbr_idx; / module index,
    pub /: *mut *mut u8 path_id; / pipe_id,
    pub /: *mut *mut } __packed p; / part,
    pub /: *mut *mut u16 f; / full,
    pub location_id: } __packed,
    pub module_id: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_dsp_header {
    pub dst: sst_destination_id,
    pub command_id: u16,
    pub length: u16,
    pub __packed: },
//
// Common Commands
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_cmd_generic {
    pub header: sst_dsp_header,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct swm_input_ids {
    pub input_id: sst_destination_id,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_cmd_set_swm {
    pub header: sst_dsp_header,
    pub output_id: sst_destination_id,
    pub switch_state: u16,
    pub nb_inputs: u16,
    pub input: [swm_input_ids; SST_CMD_SWM_MAX_INPUTS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_cmd_set_media_path {
    pub header: sst_dsp_header,
    pub switch_state: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcm_cfg {
    pub s_length:2: u8,
    pub rate:3: u8,
    pub format:3: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_cmd_set_speech_path {
    pub header: sst_dsp_header,
    pub switch_state: u16,
    pub rsvd:8: u16,
    pub cfg: pcm_cfg,
    pub config: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gain_cell {
    pub dest: sst_destination_id,
    pub cell_gain_left: i16,
    pub cell_gain_right: i16,
    pub gain_time_constant: u16,
    pub __packed: },
pub const NUM_GAIN_CELLS: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_cmd_set_gain_dual {
    pub header: sst_dsp_header,
    pub gain_cell_num: u16,
    pub cell_gains: [gain_cell; NUM_GAIN_CELLS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_cmd_set_params {
    pub dst: sst_destination_id,
    pub command_id: u16,
    pub params: [c_char; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_cmd_sba_vb_start {
    pub header: sst_dsp_header,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union sba_media_loop_params {
    pub rsvd:8: u16,
    pub cfg: pcm_cfg,
    pub part: },
    pub full: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_cmd_sba_set_media_loop_map {
    pub header: sst_dsp_header,
    pub switch_state: u16,
    pub param: sba_media_loop_params,
    pub map: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_cmd_tone_stop {
    pub header: sst_dsp_header,
    pub switch_state: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sst_ssp_mode {
    SSP_MODE_PROVIDER = 0,
    SSP_MODE_CONSUMER = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sst_ssp_pcm_mode {
    SSP_PCM_MODE_NORMAL = 0,
    SSP_PCM_MODE_NETWORK = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sst_ssp_duplex {
    SSP_DUPLEX = 0,
    SSP_RX = 1,
    SSP_TX = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sst_ssp_fs_frequency {
    SSP_FS_8_KHZ = 0,
    SSP_FS_16_KHZ = 1,
    SSP_FS_44_1_KHZ = 2,
    SSP_FS_48_KHZ = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sst_ssp_fs_polarity {
    SSP_FS_ACTIVE_LOW = 0,
    SSP_FS_ACTIVE_HIGH = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sst_ssp_protocol {
    SSP_MODE_PCM = 0,
    SSP_MODE_I2S = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sst_ssp_port_id {
    SSP_MODEM = 0,
    SSP_BT = 1,
    SSP_FM = 2,
    SSP_CODEC = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_cmd_sba_hw_set_ssp {
    pub header: sst_dsp_header,
    pub /: *mut *mut u16 selection; / 0:SSP0(def), 1:SSP1, 2:SSP2,
    pub switch_state: u16,
    pub /: *mut *mut u16 nb_bits_per_slots:6; / 0-32 bits, 24 (def),
    pub /: *mut *mut u16 nb_slots:4; / 0-8: slots per frame,
    pub /: *mut *mut u16 mode:3; / 0:Master, 1: Slave,
    pub duplex:3: u16,
    pub /: *mut *mut u16 active_tx_slot_map:8; / Bit map, 0:off, 1:on,
    pub reserved1:8: u16,
    pub /: *mut *mut u16 active_rx_slot_map:8; / Bit map 0: Off, 1:On,
    pub reserved2:8: u16,
    pub frame_sync_frequency: u16,
    pub frame_sync_polarity:8: u16,
    pub data_polarity:8: u16,
    pub /: *mut *mut u16 frame_sync_width; / 1 to N clocks,
    pub ssp_protocol:8: u16,
    pub /: *mut *mut u16 start_delay:8; / Start delay in terms of clock ticks,
    pub __packed: },
pub const SST_MAX_TDM_SLOTS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_param_sba_ssp_slot_map {
    pub header: sst_dsp_header,
    pub param_id: u16,
    pub param_len: u16,
    pub ssp_index: u16,
    pub rx_slot_map: [u8; SST_MAX_TDM_SLOTS],
    pub tx_slot_map: [u8; SST_MAX_TDM_SLOTS],
    pub __packed: },
}

// widget defines
pub const SST_MODULE_GAIN: c_int = 1;
pub const SST_MODULE_ALGO: c_int = 2;
pub const SST_FMT_MONO: c_int = 0;
pub const SST_FMT_STEREO: c_int = 3;
// physical SSP numbers

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_module {
    pub kctl: *mut snd_kcontrol,
    pub node: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_ssp_config {
    pub ssp_id: u8,
    pub bits_per_slot: u8,
    pub slots: u8,
    pub ssp_mode: u8,
    pub pcm_mode: u8,
    pub duplex: u8,
    pub ssp_protocol: u8,
    pub fs_frequency: u8,
    pub active_slot_map: u8,
    pub start_delay: u8,
    pub fs_width: u16,
    pub frame_sync_polarity: u8,
    pub data_polarity: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_ssp_cfg {
    pub ssp_number: u8,
    pub mux_shift: *const c_int,
    pub (*domain_shift)[SST_MAX_SSP_MUX]: *const c_int,
    pub (*ssp_config)[SST_MAX_SSP_MUX][SST_MAX_SSP_DOMAINS]: *const sst_ssp_config,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_ids {
    pub location_id: u16,
    pub module_id: u16,
    pub task_id: u8,
    pub format: u8,
    pub reg: u8,
    pub parent_wname: *const c_char,
    pub parent_w: *mut snd_soc_dapm_widget,
    pub algo_list: list_head,
    pub gain_list: list_head,
    pub pcm_fmt: *const sst_pcm_format,
}

// output is triggered before input

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sst_gain_kcontrol_type {
    SST_GAIN_TLV,
    SST_GAIN_MUTE,
    SST_GAIN_RAMP_DURATION,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_gain_mixer_control {
    pub stereo: bool,
    pub type: sst_gain_kcontrol_type,
    pub gain_val: *mut sst_gain_value,
    pub max: c_int,
    pub min: c_int,
    pub instance_id: u16,
    pub module_id: u16,
    pub pipe_id: u16,
    pub task_id: u16,
    pub pname: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN],
    pub w: *mut snd_soc_dapm_widget,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_gain_value {
    pub ramp_duration: u16,
    pub l_gain: i16,
    pub r_gain: i16,
    pub mute: bool,
}

//
// 3 Controls for each Gain module
// e.g.	- pcm0_in Gain 0 Volume
// - pcm0_in Gain 0 Ramp Delay
// - pcm0_in Gain 0 Switch
//

pub const SST_GAIN_TC_MIN: c_int = 5;
pub const SST_GAIN_TC_MAX: c_int = 5000;

pub const SST_GAIN_MAX_VALUE: c_int = 360;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sst_algo_kcontrol_type {
    SST_ALGO_PARAMS,
    SST_ALGO_BYPASS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_algo_control {
    pub type: sst_algo_kcontrol_type,
    pub max: c_int,
    pub module_id: u16,
    pub pipe_id: u16,
    pub task_id: u16,
    pub cmd_id: u16,
    pub bypass: bool,
    pub params: *mut c_uchar,
    pub w: *mut snd_soc_dapm_widget,
}

// size of the control = size of params + size of length field

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_enum {
    pub tx: bool,
    pub reg: c_ushort,
    pub max: c_uint,
    pub texts: *const *const c_char,
    pub w: *mut snd_soc_dapm_widget,
}

// only 4 slots/channels supported atm

extern "C" {
    pub fn sst_fill_ssp_config(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int;
}
extern "C" {
    pub fn sst_fill_ssp_defaults(dai: *mut snd_soc_dai);
}
