//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/intel/atom/sst-mfld-dsp.h
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
// sst_mfld_dsp.h - Intel SST Driver for audio engine
//
// Copyright (C) 2008-14 Intel Corporation
// Authors:	Vinod Koul <vinod.koul@linux.intel.com>
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
pub const SST_MAX_BIN_BYTES: c_int = 1024;
pub const MAX_DBG_RW_BYTES: c_int = 80;
pub const MAX_NUM_SCATTER_BUFFERS: c_int = 8;
pub const MAX_LOOP_BACK_DWORDS: c_int = 8;
// IPC base address and mailbox, timestamp offsets
pub const SST_MAILBOX_SIZE: c_uint = 0x0400;
pub const SST_MAILBOX_SEND: c_uint = 0x0000;
pub const SST_TIME_STAMP: c_uint = 0x1800;
pub const SST_TIME_STAMP_MRFLD: c_uint = 0x800;
pub const SST_RESERVED_OFFSET: c_uint = 0x1A00;
pub const SST_SCU_LPE_MAILBOX: c_uint = 0x1000;
pub const SST_LPE_SCU_MAILBOX: c_uint = 0x1400;

pub const PROCESS_MSG: c_uint = 0x80;
// Message ID's for IPC messages
// Bits B7: SST or IA/SC ; B6-B4: Msg Category; B3-B0: Msg Type
// I2L Firmware/Codec Download msgs
pub const IPC_IA_PREP_LIB_DNLD: c_uint = 0x01;
pub const IPC_IA_LIB_DNLD_CMPLT: c_uint = 0x02;
pub const IPC_IA_GET_FW_VERSION: c_uint = 0x04;
pub const IPC_IA_GET_FW_BUILD_INF: c_uint = 0x05;
pub const IPC_IA_GET_FW_INFO: c_uint = 0x06;
pub const IPC_IA_GET_FW_CTXT: c_uint = 0x07;
pub const IPC_IA_SET_FW_CTXT: c_uint = 0x08;
pub const IPC_IA_PREPARE_SHUTDOWN: c_uint = 0x31;
// I2L Codec Config/control msgs
pub const IPC_PREP_D3: c_uint = 0x10;
pub const IPC_IA_SET_CODEC_PARAMS: c_uint = 0x10;
pub const IPC_IA_GET_CODEC_PARAMS: c_uint = 0x11;
pub const IPC_IA_SET_PPP_PARAMS: c_uint = 0x12;
pub const IPC_IA_GET_PPP_PARAMS: c_uint = 0x13;
pub const IPC_SST_PERIOD_ELAPSED_MRFLD: c_uint = 0xA;
pub const IPC_IA_ALG_PARAMS: c_uint = 0x1A;
pub const IPC_IA_TUNING_PARAMS: c_uint = 0x1B;
pub const IPC_IA_SET_RUNTIME_PARAMS: c_uint = 0x1C;
pub const IPC_IA_SET_PARAMS: c_uint = 0x1;
pub const IPC_IA_GET_PARAMS: c_uint = 0x2;
pub const IPC_EFFECTS_CREATE: c_uint = 0xE;
pub const IPC_EFFECTS_DESTROY: c_uint = 0xF;
// I2L Stream config/control msgs
pub const IPC_IA_ALLOC_STREAM_MRFLD: c_uint = 0x2;
pub const IPC_IA_ALLOC_STREAM: c_uint = 0x20 /* Allocate a stream ID */;
pub const IPC_IA_FREE_STREAM_MRFLD: c_uint = 0x03;
pub const IPC_IA_FREE_STREAM: c_uint = 0x21 /* Free the stream ID */;
pub const IPC_IA_SET_STREAM_PARAMS: c_uint = 0x22;
pub const IPC_IA_SET_STREAM_PARAMS_MRFLD: c_uint = 0x12;
pub const IPC_IA_GET_STREAM_PARAMS: c_uint = 0x23;
pub const IPC_IA_PAUSE_STREAM: c_uint = 0x24;
pub const IPC_IA_PAUSE_STREAM_MRFLD: c_uint = 0x4;
pub const IPC_IA_RESUME_STREAM: c_uint = 0x25;
pub const IPC_IA_RESUME_STREAM_MRFLD: c_uint = 0x5;
pub const IPC_IA_DROP_STREAM: c_uint = 0x26;
pub const IPC_IA_DROP_STREAM_MRFLD: c_uint = 0x07;
pub const IPC_IA_DRAIN_STREAM: c_uint = 0x27 /* Short msg with str_id */;
pub const IPC_IA_DRAIN_STREAM_MRFLD: c_uint = 0x8;
pub const IPC_IA_CONTROL_ROUTING: c_uint = 0x29;
pub const IPC_IA_VTSV_UPDATE_MODULES: c_uint = 0x20;
pub const IPC_IA_VTSV_DETECTED: c_uint = 0x21;

pub const IPC_IA_START_STREAM: c_uint = 0x30 /* Short msg with str_id */;
pub const IPC_IA_SET_GAIN_MRFLD: c_uint = 0x21;
// Debug msgs
pub const IPC_IA_DBG_MEM_READ: c_uint = 0x40;
pub const IPC_IA_DBG_MEM_WRITE: c_uint = 0x41;
pub const IPC_IA_DBG_LOOP_BACK: c_uint = 0x42;
pub const IPC_IA_DBG_LOG_ENABLE: c_uint = 0x45;
pub const IPC_IA_DBG_SET_PROBE_PARAMS: c_uint = 0x47;
// L2I Firmware/Codec Download msgs
pub const IPC_IA_FW_INIT_CMPLT: c_uint = 0x81;
pub const IPC_IA_FW_INIT_CMPLT_MRFLD: c_uint = 0x01;
pub const IPC_IA_FW_ASYNC_ERR_MRFLD: c_uint = 0x11;
// L2I Codec Config/control msgs
pub const IPC_SST_FRAGMENT_ELPASED: c_uint = 0x90 /* Request IA more data */;
pub const IPC_SST_BUF_UNDER_RUN: c_uint = 0x92 /* PB Under run and stopped */;
pub const IPC_SST_BUF_OVER_RUN: c_uint = 0x93 /* CAP Under run and stopped */;
pub const IPC_SST_DRAIN_END: c_uint = 0x94 /* PB Drain complete and stopped */;
pub const IPC_SST_CHNGE_SSP_PARAMS: c_uint = 0x95 /* PB SSP parameters changed */;
pub const IPC_SST_STREAM_PROCESS_FATAL_ERR: c_uint = 0x96/* error in processing a stream */;
pub const IPC_SST_PERIOD_ELAPSED: c_uint = 0x97 /* period elapsed */;
pub const IPC_SST_ERROR_EVENT: c_uint = 0x99 /* Buffer over run occurred */;
// L2S messages
pub const IPC_SC_DDR_LINK_UP: c_uint = 0xC0;
pub const IPC_SC_DDR_LINK_DOWN: c_uint = 0xC1;
pub const IPC_SC_SET_LPECLK_REQ: c_uint = 0xC2;
pub const IPC_SC_SSP_BIT_BANG: c_uint = 0xC3;
// L2I Error reporting msgs
pub const IPC_IA_MEM_ALLOC_FAIL: c_uint = 0xE0;
pub const IPC_IA_PROC_ERR: c_uint = 0xE1 /* error in processing a;
// L2I Debug msgs
pub const IPC_IA_PRINT_STRING: c_uint = 0xF0;
// Buffer under-run
pub const IPC_IA_BUF_UNDER_RUN_MRFLD: c_uint = 0x0B;
// Mrfld specific defines:
// For asynchronous messages(INIT_CMPLT, PERIOD_ELAPSED, ASYNC_ERROR)
// received from FW, the format is:
// - IPC High: pvt_id is set to zero. Always short message.
// - msg_id is in lower 16-bits of IPC low payload.
// - pipe_id is in higher 16-bits of IPC low payload for period_elapsed.
// - error id is in higher 16-bits of IPC low payload for async errors.
//
pub const SST_ASYNC_DRV_ID: c_int = 0;
// Command Response or Acknowledge message to any IPC message will have
// same message ID and stream ID information which is sent.
// There is no specific Ack message ID. The data field is used as response
// meaning.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ackData {
    IPC_ACK_SUCCESS = 0,
    IPC_ACK_FAILURE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipc_ia_msg_id {
    IPC_CMD = 1,		/*!< Task Control message ID */
    IPC_SET_PARAMS = 2,/*!< Task Set param message ID */
    IPC_GET_PARAMS = 3,	/*!< Task Get param message ID */
    IPC_INVALID = 0xFF,	/*!<Task Get param message ID */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sst_codec_types {
// AUDIO/MUSIC	CODEC Type Definitions
    SST_CODEC_TYPE_UNKNOWN = 0,
    SST_CODEC_TYPE_PCM,	/* Pass through Audio codec */
    SST_CODEC_TYPE_MP3,
    SST_CODEC_TYPE_MP24,
    SST_CODEC_TYPE_AAC,
    SST_CODEC_TYPE_AACP,
    SST_CODEC_TYPE_eAACP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stream_type {
    SST_STREAM_TYPE_NONE = 0,
    SST_STREAM_TYPE_MUSIC = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sst_error_codes {
// Error code,response to msgId: Description
// Common error codes
    SST_SUCCESS = 0,        /* Success */
    SST_ERR_INVALID_STREAM_ID = 1,
    SST_ERR_INVALID_MSG_ID = 2,
    SST_ERR_INVALID_STREAM_OP = 3,
    SST_ERR_INVALID_PARAMS = 4,
    SST_ERR_INVALID_CODEC = 5,
    SST_ERR_INVALID_MEDIA_TYPE = 6,
    SST_ERR_STREAM_ERR = 7,

    SST_ERR_STREAM_IN_USE = 15,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_dsp_hdr {
    pub /: *mut *mut u16 mod_index_id:8; /!< DSP Command ID specific to tasks,
    pub /: *mut *mut u16 pipe_id:8; /!< instance of the module in the pipeline,
    pub /: *mut *mut u16 mod_id; /!< Pipe_id,
    pub /: *mut *mut u16 cmd_id; /!< Module ID = lpe_algo_types_t,
    pub /: *mut *mut u16 length; /!< Length of the payload only,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union ipc_header_high {
    pub /: *mut *mut u32 msg_id:8; / Message ID - Max 256 Message Types,
    pub /: *mut *mut u32 task_id:4; / Task ID associated with this comand,
    pub track*/: *mut *mut u32 drv_id:4; / Identifier for the driver to,
    pub /: *mut *mut u32 rsvd1:8; / Reserved,
    pub /: *mut *mut u32 result:4; / Reserved,
    pub /: *mut *mut u32 res_rqd:1; / Response rqd,
    pub /: *mut *mut u32 large:1; / Large Message if large = 1,
    pub /: *mut *mut u32 done:1; / bit 30 - Done bit,
    pub bit*/: *mut *mut u32 busy:1; / bit 31 - busy,
    pub part: },
    pub full: u32,
    pub __packed: },
// IPC header
#[repr(C)]
#[derive(Copy, Clone)]
pub union ipc_header_mrfld {
    pub header_low_payload: u32,
    pub header_high: ipc_header_high,
    pub p: },
    pub full: u64,
    pub __packed: },
// CAUTION NOTE: All IPC message body must be multiple of 32 bits.
// IPC Header
#[repr(C)]
#[derive(Copy, Clone)]
pub union ipc_header {
    pub /: *mut *mut u32 msg_id:8; / Message ID - Max 256 Message Types,
    pub str_id:5: u32,
    pub /: *mut *mut u32 large:1; / Large Message if large = 1,
    pub /: *mut *mut u32 reserved:2; / Reserved for future use,
    pub /: *mut *mut u32 data:14; / Ack/Info for msg, size of msg in Mailbox,
    pub /: *mut *mut u32 done:1; / bit 30,
    pub /: *mut *mut u32 busy:1; / bit 31,
    pub part: },
    pub full: u32,
    pub __packed: },
// Firmware build info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_fw_build_info {
    pub /: *mut *mut unsigned char date[16]; / Firmware build date,
    pub /: *mut *mut unsigned char time[16]; / Firmware build time,
    pub __packed: },
// Firmware Version info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sst_fw_version {
    pub number*/: *mut *mut u8 build; / build,
    pub number*/: *mut *mut u8 minor; / minor,
    pub number*/: *mut *mut u8 major; / major,
    pub /: *mut *mut u8 type; / build type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_header_fw_init {
    pub /: *mut *mut snd_sst_fw_version fw_version;/ Firmware version details,
    pub build_info: sst_fw_build_info,
    pub /: *mut *mut u16 result; / Fw init result,
    pub /: *mut *mut u8 module_id; / Module ID in case of error,
    pub /: *mut *mut u8 debug_info; / Debug info from Module ID in case of fail,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sst_tstamp {
    pub /: *mut *mut u64 ring_buffer_counter; / PB/CP: Bytes copied from/to DDR.,
    pub /: *mut *mut u64 hardware_counter; / PB/CP: Bytes DMAed to/from SSP.,
    pub frames_decoded: u64,
    pub bytes_decoded: u64,
    pub bytes_copied: u64,
    pub sampling_frequency: u32,
    pub channel_peak: [u32; 8],
    pub __packed: },
// Stream type params structure for Alloc stream
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sst_str_type {
    pub /: *mut *mut u8 codec_type; / Codec type,
    pub /: *mut *mut u8 str_type; / 1 = voice 2 = music,
    pub /: *mut *mut u8 operation; / Playback or Capture,
    pub /: *mut *mut u8 protected_str; / 0=Non DRM, 1=DRM,
    pub time_slots: u8,
    pub /: *mut *mut u8 reserved; / Reserved,
    pub /: *mut *mut u16 result; / Result used for acknowledgment,
    pub __packed: },
// Library info structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct module_info {
    pub lib_version: u32,
    pub lib_type;*/: *mut *mut u32 lib_type;/TBD- KLOCKWORK u8,
    pub media_type: u32,
    pub lib_name: [u8; 12],
    pub lib_caps: u32,
    pub /: *mut *mut unsigned char b_date[16]; / Lib build date,
    pub /: *mut *mut unsigned char b_time[16]; / Lib build time,
    pub __packed: },
// Library slot info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lib_slot_info {
    pub /: *mut *mut u8 slot_num; / 1 or 2,
    pub reserved1: u8,
    pub reserved2: u16,
    pub /: *mut *mut u32 iram_size; / slot size in IRAM,
    pub /: *mut *mut u32 dram_size; / slot size in DRAM,
    pub /: *mut *mut u32 iram_offset; / starting offset of slot in IRAM,
    pub /: *mut *mut u32 dram_offset; / starting offset of slot in DRAM,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ppp_mixer_params {
    pub /: *mut *mut __u32 type; /Type of the parameter,
    pub size: __u32,
    pub Map*/: *mut *mut __u32 input_stream_bitmap; /Input stream Bit,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sst_lib_download {
    pub /: *mut *mut module_info lib_info; / library info type, capabilities etc,
    pub /: *mut *mut lib_slot_info slot_info; / slot info to be downloaded,
    pub mod_entry_pt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sst_lib_download_info {
    pub dload_lib: snd_sst_lib_download,
    pub /: *mut *mut u16 result; / Result used for acknowledgment,
    pub /: *mut *mut u8 pvt_id; / Private ID,
    pub /: *mut *mut u8 reserved; / for alignment,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_params {
    pub /: *mut *mut u8 num_chan; / 1=Mono, 2=Stereo,
    pub bit*/: *mut *mut u8 pcm_wd_sz; / 16/24 -,
    pub interfaces: *mut *mut u8 use_offload_path; / 0-PCM using period elpased & ALSA,
    pub reserved2: u8,
    pub /: *mut *mut u32 sfreq; / Sampling rate in Hz,
    pub channel_map: [u8; 8],
    pub __packed: },
// MP3 Music Parameters Message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_mp3_params {
    pub /: *mut *mut u8 num_chan; / 1=Mono, 2=Stereo,
    pub bit*/: *mut *mut u8 pcm_wd_sz; / 16/24 -,
    pub /: *mut *mut u8 crc_check; / crc_check - disable (0) or enable (1),
    pub unused*/: *mut *mut u8 reserved1; /,
    pub /: *mut *mut u16 reserved2; / Unused,
    pub __packed: },
pub const AAC_BIT_STREAM_ADTS: c_int = 0;
pub const AAC_BIT_STREAM_ADIF: c_int = 1;
pub const AAC_BIT_STREAM_RAW: c_int = 2;
// AAC Music Parameters Message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_aac_params {
    pub 2=Stereo*/: *mut *mut u8 num_chan; / 1=Mono,,
    pub bit*/: *mut *mut u8 pcm_wd_sz; / 16/24 -,
    pub /: *mut *mut u8 bdownsample; /SBR downsampling 0 - disable 1 -enabled AAC+ only,
    pub /: *mut *mut u8 bs_format; / input bit stream format adts=0, adif=1, raw=2,
    pub reser2: u16,
    pub stream*/: *mut *mut u32 externalsr; /sampling rate of basic AAC raw bit,
    pub tool.AAC+*/: *mut *mut u8 sbr_signalling;/disable/enable/set automode the SBR,
    pub reser1: u8,
    pub reser3: u16,
    pub __packed: },
// WMA Music Parameters Message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_wma_params {
    pub /: *mut *mut u8 num_chan; / 1=Mono, 2=Stereo,
    pub bit*/: *mut *mut u8 pcm_wd_sz; / 16/24 -,
    pub reserved1: u16,
    pub /: *mut *mut u32 brate; / Use the hard coded value.,
    pub /: *mut *mut u32 sfreq; / Sampling freq eg. 8000, 441000, 48000,
    pub /: *mut *mut u32 channel_mask; / Channel Mask,
    pub /: *mut *mut u16 format_tag; / Format Tag,
    pub /: *mut *mut u16 block_align; / packet size,
    pub /: *mut *mut u16 wma_encode_opt;/ Encoder option,
    pub /: *mut *mut u8 op_align; / op align 0- 16 bit, 1- MSB, 2 LSB,
    pub /: *mut *mut u8 reserved; / reserved,
    pub __packed: },
// Codec params structure
#[repr(C)]
#[derive(Copy, Clone)]
pub union snd_sst_codec_params {
    pub pcm_params: snd_pcm_params,
    pub mp3_params: snd_mp3_params,
    pub aac_params: snd_aac_params,
    pub wma_params: snd_wma_params,
    pub __packed: },
// Address and size info of a frame buffer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_address_info {
    pub /: *mut *mut u32 addr; / Address at IA,
    pub /: *mut *mut u32 size; / Size of the buffer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sst_alloc_params_ext {
    pub sg_count: __u16,
    pub reserved: __u16,
    pub elapsed: *mut *mut __u32 frag_size; /Number of samples after which period,
    pub ring_buf_info: [sst_address_info; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sst_stream_params {
    pub uc: snd_sst_codec_params,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sst_params {
    pub result: u32,
    pub stream_id: u32,
    pub codec: u8,
    pub ops: u8,
    pub stream_type: u8,
    pub device_type: u8,
    pub task: u8,
    pub sparams: snd_sst_stream_params,
    pub aparams: snd_sst_alloc_params_ext,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sst_alloc_mrfld {
    pub codec_type: u16,
    pub operation: u8,
    pub sg_count: u8,
    pub ring_buf_info: [sst_address_info; 8],
    pub frag_size: u32,
    pub ts: u32,
    pub codec_params: snd_sst_stream_params,
    pub __packed: },
// Alloc stream params structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sst_alloc_params {
    pub str_type: snd_sst_str_type,
    pub stream_params: snd_sst_stream_params,
    pub alloc_params: snd_sst_alloc_params_ext,
    pub __packed: },
// Alloc stream response message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sst_alloc_response {
    pub /: *mut *mut snd_sst_str_type str_type; / Stream type for allocation,
    pub /: *mut *mut snd_sst_lib_download lib_dnld; / Valid only for codec dnld,
}

// Drop response
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sst_drop_response {
    pub result: u32,
    pub bytes: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sst_async_msg {
    pub /: *mut *mut u32 msg_id; / Async msg id,
    pub payload: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sst_async_err_msg {
    pub /: *mut *mut u32 fw_resp; / Firmware Result,
    pub /: *mut *mut u32 lib_resp; /Library result,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sst_vol {
    pub stream_id: u32,
    pub volume: i32,
    pub ramp_duration: u32,
    pub /: *mut *mut u32 ramp_type; / Ramp type, default=0,
}

// Gain library parameters for mrfld
// based on DSP command spec v0.82
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sst_gain_v2 {
    pub modify*/: *mut *mut u16 gain_cell_num; / num of gain cells to,
    pub index*/: *mut *mut u8 cell_nbr_idx; / instance,
    pub /: *mut *mut u8 cell_path_idx; / pipe-id,
    pub /: *mut *mut u16 module_id; /module id,
    pub dB*/: *mut *mut u16 left_cell_gain; / left gain value in,
    pub dB*/: *mut *mut u16 right_cell_gain; / right gain value in,
    pub constant*/: *mut *mut u16 gain_time_const; / gain time,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sst_mute {
    pub stream_id: u32,
    pub mute: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sst_runtime_params {
    pub type: u8,
    pub str_id: u8,
    pub size: u8,
    pub rsvd: u8,
    pub addr: *mut c_void,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stream_param_type {
    SST_SET_TIME_SLOT = 0,
    SST_SET_CHANNEL_INFO = 1,
    OTHERS = 2, /*reserved for future params*/
}

// CSV Voice call routing structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sst_control_routing {
    pub /: *mut *mut u8 control; / 0=start, 1=Stop,
    pub /: *mut *mut u8 reserved[3]; / Reserved- for 32 bit alignment,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_post {
    pub node: list_head,
    pub /: *mut *mut ipc_header header; / driver specific,
    pub is_large: bool,
    pub is_process_reply: bool,
    pub mrfld_header: ipc_header_mrfld,
    pub mailbox_data: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sst_ctxt_params {
    pub /: *mut *mut u32 address; / Physical Address in DDR where the context is stored,
    pub /: *mut *mut u32 size; / size of the context,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sst_lpe_log_params {
    pub dbg_type: u8,
    pub module_id: u8,
    pub log_level: u8,
    pub reserved: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_sst_bytes_type {
    SND_SST_BYTES_SET = 0x1,
    SND_SST_BYTES_GET = 0x2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sst_bytes_v2 {
    pub type: u8,
    pub ipc_msg: u8,
    pub block: u8,
    pub task_id: u8,
    pub pipe_id: u8,
    pub rsvd: u8,
    pub len: u16,
    pub bytes: [c_char; ],
}

pub const MAX_VTSV_FILES: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sst_vtsv_info {
    pub vfiles: [sst_address_info; MAX_VTSV_FILES],
    pub __packed: },
