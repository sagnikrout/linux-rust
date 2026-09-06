//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/sof/topology.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
//

//
// Component
//
// types of component
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_comp_type {
    SOF_COMP_NONE = 0,
    SOF_COMP_HOST,
    SOF_COMP_DAI,
    SOF_COMP_SG_HOST,	/**< scatter gather variant */
    SOF_COMP_SG_DAI,	/**< scatter gather variant */
    SOF_COMP_VOLUME,
    SOF_COMP_MIXER,
    SOF_COMP_MUX,
    SOF_COMP_SRC,
    SOF_COMP_DEPRECATED0, /* Formerly SOF_COMP_SPLITTER */
    SOF_COMP_TONE,
    SOF_COMP_DEPRECATED1, /* Formerly SOF_COMP_SWITCH */
    SOF_COMP_BUFFER,
    SOF_COMP_EQ_IIR,
    SOF_COMP_EQ_FIR,
    SOF_COMP_KEYWORD_DETECT,
    SOF_COMP_KPB,			/* A key phrase buffer component */
    SOF_COMP_SELECTOR,		/**< channel selector component */
    SOF_COMP_DEMUX,
    SOF_COMP_ASRC,		/**< Asynchronous sample rate converter */
    SOF_COMP_DCBLOCK,
    SOF_COMP_SMART_AMP,             /**< smart amplifier component */
    SOF_COMP_MODULE_ADAPTER,		/**< module adapter */
// keep FILEREAD/FILEWRITE as the last ones
    SOF_COMP_FILEREAD = 10000,	/**< host test based file IO */
    SOF_COMP_FILEWRITE = 10001,	/**< host test based file IO */
}

// XRUN action for component

// create new generic component - SOF_IPC_TPLG_COMP_NEW
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_comp {
    pub hdr: sof_ipc_cmd_hdr,
    pub id: u32,
    pub type: u32,
    pub pipeline_id: u32,
    pub core: u32,
// extended data length, 0 if no extended data
    pub ext_data_length: u32,
    pub __aligned(4): } __packed,
//
// Component Buffers
//
// SOF memory capabilities, add new ones at the end
//

//
// overrun will cause ring buffer overwrite, instead of XRUN.
//

//
// underrun will cause readback of 0s, instead of XRUN.
//

// the UUID size in bytes, shared between FW and host
pub const SOF_UUID_SIZE: c_int = 16;
// create new component buffer - SOF_IPC_TPLG_BUFFER_NEW
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_buffer {
    pub comp: sof_ipc_comp,
    pub /: *mut *mut *mut uint32_t size; /< buffer size in bytes,
    pub /: *mut *mut *mut uint32_t caps; /< SOF_MEM_CAPS_,
    pub /: *mut *mut *mut uint32_t flags; /< SOF_BUF_ flags defined above,
    pub /: *mut *mut *mut uint32_t reserved; /< reserved for future use,
    pub __aligned(4): } __packed,
// generic component config data - must always be after struct sof_ipc_comp
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_comp_config {
    pub hdr: sof_ipc_cmd_hdr,
    pub /: *mut *mut *mut uint32_t periods_sink; /< 0 means variable,
    pub /: *mut *mut *mut uint32_t periods_source;/< 0 means variable,
    pub /: *mut *mut *mut uint32_t reserved1; /< reserved,
    pub /: *mut *mut *mut uint32_t frame_fmt; /< SOF_IPC_FRAME_,
    pub xrun_action: u32,
// reserved for future use
    pub reserved: [u32; 2],
    pub __aligned(4): } __packed,
// generic host component
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_comp_host {
    pub comp: sof_ipc_comp,
    pub config: sof_ipc_comp_config,
    pub /: *mut *mut *mut uint32_t direction; /< SOF_IPC_STREAM_,
    pub /: *mut *mut *mut uint32_t no_irq; /< don't send periodic IRQ to host/DSP,
    pub /: *mut *mut *mut uint32_t dmac_config; /< DMA engine specific,
    pub __aligned(4): } __packed,
// generic DAI component
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_comp_dai {
    pub comp: sof_ipc_comp,
    pub config: sof_ipc_comp_config,
    pub /: *mut *mut *mut uint32_t direction; /< SOF_IPC_STREAM_,
    pub /: *mut *mut *mut uint32_t dai_index; /< index of this type dai,
    pub /: *mut *mut *mut uint32_t type; /< DAI type - SOF_DAI_,
    pub /: *mut *mut *mut uint32_t reserved; /< reserved,
    pub __aligned(4): } __packed,
// generic mixer component
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_comp_mixer {
    pub comp: sof_ipc_comp,
    pub config: sof_ipc_comp_config,
    pub __aligned(4): } __packed,
// volume ramping types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_volume_ramp {
    SOF_VOLUME_LINEAR	= 0,
    SOF_VOLUME_LOG,
    SOF_VOLUME_LINEAR_ZC,
    SOF_VOLUME_LOG_ZC,
    SOF_VOLUME_WINDOWS_FADE,
    SOF_VOLUME_WINDOWS_NO_FADE,
}

// generic volume component
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_comp_volume {
    pub comp: sof_ipc_comp,
    pub config: sof_ipc_comp_config,
    pub channels: u32,
    pub min_value: u32,
    pub max_value: u32,
    pub /: *mut *mut *mut uint32_t ramp; /< SOF_VOLUME_,
    pub /: *mut *mut *mut uint32_t initial_ramp; /< ramp space in ms,
    pub __aligned(4): } __packed,
// generic SRC component
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_comp_src {
    pub comp: sof_ipc_comp,
    pub config: sof_ipc_comp_config,
// either source or sink rate must be non zero
    pub /: *mut *mut *mut uint32_t source_rate; /< source rate or 0 for variable,
    pub /: *mut *mut *mut uint32_t sink_rate; /< sink rate or 0 for variable,
    pub /: *mut *mut *mut uint32_t rate_mask; /< SOF_RATE_ supported rates,
    pub __aligned(4): } __packed,
// generic ASRC component
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_comp_asrc {
    pub comp: sof_ipc_comp,
    pub config: sof_ipc_comp_config,
// either source or sink rate must be non zero
    pub /: *mut *mut *mut uint32_t source_rate; /< Define fixed source rate or,
// < use 0 to indicate need to get
// < the rate from stream
    pub /: *mut *mut *mut uint32_t sink_rate; /< Define fixed sink rate or,
// < use 0 to indicate need to get
// < the rate from stream
    pub /: *mut *mut *mut uint32_t asynchronous_mode; /< synchronous 0, asynchronous 1,
// < When 1 the ASRC tracks and
// < compensates for drift.
    pub /: *mut *mut *mut uint32_t operation_mode; /< push 0, pull 1, In push mode the,
// < ASRC consumes a defined number
// < of frames at input, with varying
// < number of frames at output.
// < In pull mode the ASRC outputs
// < a defined number of frames while
// < number of input frames varies.
// reserved for future use
    pub reserved: [u32; 4],
    pub __aligned(4): } __packed,
// generic MUX component
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_comp_mux {
    pub comp: sof_ipc_comp,
    pub config: sof_ipc_comp_config,
    pub __aligned(4): } __packed,
// generic tone generator component
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_comp_tone {
    pub comp: sof_ipc_comp,
    pub config: sof_ipc_comp_config,
    pub sample_rate: i32,
    pub frequency: i32,
    pub amplitude: i32,
    pub freq_mult: i32,
    pub ampl_mult: i32,
    pub length: i32,
    pub period: i32,
    pub repeats: i32,
    pub ramp_step: i32,
    pub __aligned(4): } __packed,
// \brief Types of processing components
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_ipc_process_type {
    SOF_PROCESS_NONE = 0,		/**< None */
    SOF_PROCESS_EQFIR,		/**< Intel FIR */
    SOF_PROCESS_EQIIR,		/**< Intel IIR */
    SOF_PROCESS_KEYWORD_DETECT,	/**< Keyword Detection */
    SOF_PROCESS_KPB,		/**< KeyPhrase Buffer Manager */
    SOF_PROCESS_CHAN_SELECTOR,	/**< Channel Selector */
    SOF_PROCESS_MUX,
    SOF_PROCESS_DEMUX,
    SOF_PROCESS_DCBLOCK,
    SOF_PROCESS_SMART_AMP,	/**< Smart Amplifier */
}

// generic "effect", "codec" or proprietary processing component
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_comp_process {
    pub comp: sof_ipc_comp,
    pub config: sof_ipc_comp_config,
    pub /: *mut *mut *mut uint32_t size; /< size of bespoke data section in bytes,
    pub /: *mut *mut *mut uint32_t type; /< sof_ipc_process_type,
// reserved for future use
    pub reserved: [u32; 7],
    pub data: [c_uchar; ],
    pub __aligned(4): } __packed,
// frees components, buffers and pipelines
// SOF_IPC_TPLG_COMP_FREE, SOF_IPC_TPLG_PIPE_FREE, SOF_IPC_TPLG_BUFFER_FREE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_free {
    pub hdr: sof_ipc_cmd_hdr,
    pub id: u32,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_comp_reply {
    pub rhdr: sof_ipc_reply,
    pub id: u32,
    pub offset: u32,
    pub __aligned(4): } __packed,
//
// Pipeline
//
// \brief Types of pipeline scheduling time domains
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_ipc_pipe_sched_time_domain {
    SOF_TIME_DOMAIN_DMA = 0,	/**< DMA interrupt */
    SOF_TIME_DOMAIN_TIMER,		/**< Timer interrupt */
}

// new pipeline - SOF_IPC_TPLG_PIPE_NEW
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_pipe_new {
    pub hdr: sof_ipc_cmd_hdr,
    pub /: *mut *mut *mut uint32_t comp_id; /< component id for pipeline,
    pub /: *mut *mut *mut uint32_t pipeline_id; /< pipeline id,
    pub /: *mut *mut *mut uint32_t sched_id; /< Scheduling component id,
    pub /: *mut *mut *mut uint32_t core; /< core we run on,
    pub us*/: *mut *mut *mut uint32_t period; /< execution period in,
    pub /: *mut *mut *mut uint32_t priority; /< priority level 0 (low) to 10 (max),
    pub /: *mut *mut *mut uint32_t period_mips; /< worst case instruction count per period,
    pub /: *mut *mut *mut uint32_t frames_per_sched;/< output frames of pipeline, 0 is variable,
    pub /: *mut *mut *mut uint32_t xrun_limit_usecs; /< report xruns greater than limit,
    pub /: *mut *mut *mut uint32_t time_domain; /< scheduling time domain,
    pub __aligned(4): } __packed,
// pipeline construction complete - SOF_IPC_TPLG_PIPE_COMPLETE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_pipe_ready {
    pub hdr: sof_ipc_cmd_hdr,
    pub comp_id: u32,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_pipe_free {
    pub hdr: sof_ipc_cmd_hdr,
    pub comp_id: u32,
    pub __aligned(4): } __packed,
// connect two components in pipeline - SOF_IPC_TPLG_COMP_CONNECT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_pipe_comp_connect {
    pub hdr: sof_ipc_cmd_hdr,
    pub source_id: u32,
    pub sink_id: u32,
    pub __aligned(4): } __packed,
// external events
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_event_types {
    SOF_EVENT_NONE = 0,
    SOF_KEYWORD_DETECT_DAPM_EVENT,
}
