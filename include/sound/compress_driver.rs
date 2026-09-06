//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/compress_driver.h
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
// compress_driver.h - compress offload driver definations
//
// Copyright (C) 2011 Intel Corporation
// Authors:	Vinod Koul <vinod.koul@linux.intel.com>
// Pierre-Louis Bossart <pierre-louis.bossart@linux.intel.com>
//

//
// struct snd_compr_task_runtime: task runtime description
// @list: list of all managed tasks
// @input: input DMA buffer
// @output: output DMA buffer
// @seqno: sequence number
// @input_size: really used data in the input buffer
// @output_size: really used data in the output buffer
// @flags: see SND_COMPRESS_TFLG_
// @state: actual task state
// @private_value: used by the lowlevel driver (opaque)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_compr_task_runtime {
    pub list: list_head,
    pub input: *mut dma_buf,
    pub output: *mut dma_buf,
    pub seqno: u64,
    pub input_size: u64,
    pub output_size: u64,
    pub flags: u32,
    pub state: u8,
    pub private_value: *mut c_void,
}

//
// struct snd_compr_runtime: runtime stream description
// @state: stream state
// @ops: pointer to DSP callbacks
// @buffer: pointer to kernel buffer, valid only when not in mmap mode or
// DSP doesn't implement copy
// @buffer_size: size of the above buffer
// @fragment_size: size of buffer fragment in bytes
// @fragments: number of such fragments
// @total_bytes_available: cumulative number of bytes made available in
// the ring buffer
// @total_bytes_transferred: cumulative bytes transferred by offload DSP
// @sleep: poll sleep
// @private_data: driver private data pointer
// @dma_area: virtual buffer address
// @dma_addr: physical buffer address (not accessible from main CPU)
// @dma_bytes: size of DMA area
// @dma_buffer_p: runtime dma buffer pointer
// @active_tasks: count of active tasks
// @total_tasks: count of all tasks
// @task_seqno: last task sequence number (!= 0)
// @tasks: list of all tasks
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_compr_runtime {
    pub state: snd_pcm_state_t,
    pub ops: *mut snd_compr_ops,
    pub buffer: *mut c_void,
    pub buffer_size: u64,
    pub fragment_size: u32,
    pub fragments: u32,
    pub total_bytes_available: u64,
    pub total_bytes_transferred: u64,
    pub sleep: wait_queue_head_t,
    pub private_data: *mut c_void,
    pub dma_area: *mut c_uchar,
    pub dma_addr: dma_addr_t,
    pub dma_bytes: usize,
    pub dma_buffer_p: *mut snd_dma_buffer,

    pub active_tasks: u32,
    pub total_tasks: u32,
    pub task_seqno: u64,
    pub tasks: list_head,

}

//
// struct snd_compr_stream: compressed stream
// @name: device name
// @ops: pointer to DSP callbacks
// @runtime: pointer to runtime structure
// @device: device pointer
// @error_work: delayed work used when closing the stream due to an error
// @direction: stream direction, playback/recording
// @metadata_set: metadata set flag, true when set
// @next_track: has userspace signal next track transition, true when set
// @partial_drain: undergoing partial_drain for stream, true when set
// @pause_in_draining: paused during draining state, true when set
// @private_data: pointer to DSP private data
// @dma_buffer: allocated buffer if any
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_compr_stream {
    pub name: *const c_char,
    pub ops: *mut snd_compr_ops,
    pub runtime: *mut snd_compr_runtime,
    pub device: *mut snd_compr,
    pub error_work: delayed_work,
    pub direction: snd_compr_direction,
    pub metadata_set: bool,
    pub next_track: bool,
    pub partial_drain: bool,
    pub pause_in_draining: bool,
    pub private_data: *mut c_void,
    pub dma_buffer: snd_dma_buffer,
}

//
// struct snd_compr_ops: compressed path DSP operations
// @open: Open the compressed stream
// This callback is mandatory and shall keep dsp ready to receive the stream
// parameter
// @free: Close the compressed stream, mandatory
// @set_params: Sets the compressed stream parameters, mandatory
// This can be called in during stream creation only to set codec params
// and the stream properties
// @get_params: retrieve the codec parameters, mandatory
// @set_metadata: Set the metadata values for a stream
// @get_metadata: retrieves the requested metadata values from stream
// @trigger: Trigger operations like start, pause, resume, drain, stop.
// This callback is mandatory
// @pointer: Retrieve current h/w pointer information. Mandatory
// @copy: Copy the compressed data to/from userspace, Optional
// Can't be implemented if DSP supports mmap
// @mmap: DSP mmap method to mmap DSP memory
// @ack: Ack for DSP when data is written to audio buffer, Optional
// Not valid if copy is implemented
// @get_caps: Retrieve DSP capabilities, mandatory
// @get_codec_caps: Retrieve capabilities for a specific codec, mandatory
// @task_create: Create a set of input/output buffers for accel operations
// @task_start: Start (queue) a task for accel operations
// @task_stop: Stop (dequeue) a task for accel operations
// @task_free: Free a set of input/output buffers for accel operations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_compr_ops {
    pub stream): *mut *mut int (open)(struct snd_compr_stream,
    pub stream): *mut *mut int (free)(struct snd_compr_stream,
    pub params): *mut snd_compr_params,
    pub params): *mut snd_codec,
    pub metadata): *mut snd_compr_metadata,
    pub metadata): *mut snd_compr_metadata,
    pub cmd): *mut *mut *mut int (trigger)(struct snd_compr_stream stream, int,
    pub tstamp): *mut snd_compr_tstamp64,
    pub count): usize,
    pub vma): *mut vm_area_struct,
    pub bytes): *mut *mut *mut int (ack)(struct snd_compr_stream stream, size_t,
    pub caps): *mut snd_compr_caps,
    pub codec): *mut snd_compr_codec_caps,

    pub task): *mut *mut *mut int (task_create) (struct snd_compr_stream stream, struct snd_compr_task_runtime,
    pub task): *mut *mut *mut int (task_start) (struct snd_compr_stream stream, struct snd_compr_task_runtime,
    pub task): *mut *mut *mut int (task_stop) (struct snd_compr_stream stream, struct snd_compr_task_runtime,
    pub task): *mut *mut *mut int (task_free) (struct snd_compr_stream stream, struct snd_compr_task_runtime,

}

//
// struct snd_compr: Compressed device
// @name: DSP device name
// @dev: associated device instance
// @ops: pointer to DSP callbacks
// @private_data: pointer to DSP pvt data
// @card: sound card pointer
// @direction: Playback or capture direction
// @lock: device lock
// @device: device id
// @use_pause_in_draining: allow pause in draining, true when set
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_compr {
    pub name: *const c_char,
    pub dev: *mut device,
    pub ops: *mut snd_compr_ops,
    pub private_data: *mut c_void,
    pub card: *mut snd_card,
    pub direction: c_uint,
    pub lock: mutex,
    pub device: c_int,
    pub use_pause_in_draining: bool,

// private:
    pub id: [c_char; 64],
    pub proc_root: *mut snd_info_entry,
    pub proc_info_entry: *mut snd_info_entry,

}

// compress device register APIs
//
// snd_compr_use_pause_in_draining - Allow pause and resume in draining state
// @substream: compress substream to set
//
// Allow pause and resume in draining state.
// Only HW driver supports this transition can call this API.
//
// dsp driver callback apis
// For playback: driver should call snd_compress_fragment_elapsed() to let the
// framework know that a fragment has been consumed from the ring buffer
//
// For recording: we want to know when a frame is available or when
// at least one frame is available so snd_compress_frame_elapsed()
// callback should be called when a encodeded frame is available
//
// for partial_drain case we are back to running state on success
//
// snd_compr_set_runtime_buffer - Set the Compress runtime buffer
// @stream: compress stream to set
// @bufp: the buffer information, NULL to clear
//
// Copy the buffer information to runtime buffer when @bufp is non-NULL.
// Otherwise it clears the current buffer information.
//
extern "C" {
    pub fn snd_compr_malloc_pages(stream: *mut snd_compr_stream, size: usize) -> c_int;
}
extern "C" {
    pub fn snd_compr_free_pages(stream: *mut snd_compr_stream) -> c_int;
}

