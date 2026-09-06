//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/sound/compress_offload.h
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
//
// compress_offload.h - compress offload header definations
//
// Copyright (C) 2011 Intel Corporation
// Authors:	Vinod Koul <vinod.koul@linux.intel.com>
// Pierre-Louis Bossart <pierre-louis.bossart@linux.intel.com>
//

//
// struct snd_compressed_buffer - compressed buffer
// @fragment_size: size of buffer fragment in bytes
// @fragments: number of such fragments
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_compressed_buffer {
    pub fragment_size: __u32,
    pub fragments: __u32,
// C attribute field omitted
//
// struct snd_compr_params - compressed stream params
// @buffer: buffer description
// @codec: codec parameters
// @no_wake_mode: dont wake on fragment elapsed
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_compr_params {
    pub buffer: snd_compressed_buffer,
    pub codec: snd_codec,
    pub no_wake_mode: __u8,
// C attribute field omitted
//
// struct snd_compr_tstamp - timestamp descriptor
// @byte_offset: Byte offset in ring buffer to DSP
// @copied_total: Total number of bytes copied from/to ring buffer to/by DSP
// @pcm_frames: Frames decoded or encoded by DSP. This field will evolve by
// large steps and should only be used to monitor encoding/decoding
// progress. It shall not be used for timing estimates.
// @pcm_io_frames: Frames rendered or received by DSP into a mixer or an audio
// output/input. This field should be used for A/V sync or time estimates.
// @sampling_rate: sampling rate of audio
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_compr_tstamp {
    pub byte_offset: __u32,
    pub copied_total: __u32,
    pub pcm_frames: __u32,
    pub pcm_io_frames: __u32,
    pub sampling_rate: __u32,
// C attribute field omitted
//
// struct snd_compr_tstamp64 - timestamp descriptor with fields in 64 bit
// @byte_offset: Byte offset in ring buffer to DSP
// @copied_total: Total number of bytes copied from/to ring buffer to/by DSP
// @pcm_frames: Frames decoded or encoded by DSP. This field will evolve by
// large steps and should only be used to monitor encoding/decoding
// progress. It shall not be used for timing estimates.
// @pcm_io_frames: Frames rendered or received by DSP into a mixer or an audio
// output/input. This field should be used for A/V sync or time estimates.
// @sampling_rate: sampling rate of audio
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_compr_tstamp64 {
    pub byte_offset: __u32,
    pub copied_total: __u64,
    pub pcm_frames: __u64,
    pub pcm_io_frames: __u64,
    pub sampling_rate: __u32,
// C attribute field omitted
//
// struct snd_compr_avail - avail descriptor
// @avail: Number of bytes available in ring buffer for writing/reading
// @tstamp: timestamp information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_compr_avail {
    pub avail: __u64,
    pub tstamp: snd_compr_tstamp,
// C attribute field omitted
//
// struct snd_compr_avail64 - avail descriptor with tstamp in 64 bit format
// @avail: Number of bytes available in ring buffer for writing/reading
// @tstamp: timestamp information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_compr_avail64 {
    pub avail: __u64,
    pub tstamp: snd_compr_tstamp64,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_compr_direction {
    SND_COMPRESS_PLAYBACK = 0,
    SND_COMPRESS_CAPTURE,
    SND_COMPRESS_ACCEL
}

//
// struct snd_compr_caps - caps descriptor
// @codecs: pointer to array of codecs
// @direction: direction supported. Of type snd_compr_direction
// @min_fragment_size: minimum fragment supported by DSP
// @max_fragment_size: maximum fragment supported by DSP
// @min_fragments: min fragments supported by DSP
// @max_fragments: max fragments supported by DSP
// @num_codecs: number of codecs supported
// @reserved: reserved field
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_compr_caps {
    pub num_codecs: __u32,
    pub direction: __u32,
    pub min_fragment_size: __u32,
    pub max_fragment_size: __u32,
    pub min_fragments: __u32,
    pub max_fragments: __u32,
    pub codecs: [__u32; MAX_NUM_CODECS],
    pub reserved: [__u32; 11],
// C attribute field omitted
//
// struct snd_compr_codec_caps - query capability of codec
// @codec: codec for which capability is queried
// @num_descriptors: number of codec descriptors
// @descriptor: array of codec capability descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_compr_codec_caps {
    pub codec: __u32,
    pub num_descriptors: __u32,
    pub descriptor: [snd_codec_desc; MAX_NUM_CODEC_DESCRIPTORS],
// C attribute field omitted
//
// enum sndrv_compress_encoder - encoder metadata key
// @SNDRV_COMPRESS_ENCODER_PADDING: no of samples appended by the encoder at the
// end of the track
// @SNDRV_COMPRESS_ENCODER_DELAY: no of samples inserted by the encoder at the
// beginning of the track
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sndrv_compress_encoder {
    SNDRV_COMPRESS_ENCODER_PADDING = 1,
    SNDRV_COMPRESS_ENCODER_DELAY = 2,
}

//
// struct snd_compr_metadata - compressed stream metadata
// @key: key id
// @value: key value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_compr_metadata {
    pub key: __u32,
    pub value: [__u32; 8],
// C attribute field omitted
// flags for struct snd_compr_task

//
// struct snd_compr_task - task primitive for non-realtime operation
// @seqno: sequence number (task identifier)
// @origin_seqno: previous sequence number (task identifier) - for reuse
// @input_fd: data input file descriptor (dma-buf)
// @output_fd: data output file descriptor (dma-buf)
// @input_size: filled data in bytes (from caller, must not exceed fragment size)
// @flags: see SND_COMPRESS_TFLG_* defines
// @reserved: reserved for future extension
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_compr_task {
    pub seqno: __u64,
    pub origin_seqno: __u64,
    pub input_fd: c_int,
    pub output_fd: c_int,
    pub input_size: __u64,
    pub flags: __u32,
    pub reserved: [__u8; 16],
// C attribute field omitted
//
// enum snd_compr_state - task state
// @SND_COMPRESS_TASK_STATE_IDLE: task is not queued
// @SND_COMPRESS_TASK_STATE_ACTIVE: task is in the queue
// @SND_COMPRESS_TASK_STATE_FINISHED: task was processed, output is available
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_compr_state {
    SND_COMPRESS_TASK_STATE_IDLE = 0,
    SND_COMPRESS_TASK_STATE_ACTIVE,
    SND_COMPRESS_TASK_STATE_FINISHED
}

//
// struct snd_compr_task_status - task status
// @seqno: sequence number (task identifier)
// @input_size: filled data in bytes (from user space)
// @output_size: filled data in bytes (from driver)
// @output_flags: reserved for future (all zeros - from driver)
// @state: actual task state (SND_COMPRESS_TASK_STATE_*)
// @reserved: reserved for future extension
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_compr_task_status {
    pub seqno: __u64,
    pub input_size: __u64,
    pub output_size: __u64,
    pub output_flags: __u32,
    pub state: __u8,
    pub reserved: [__u8; 15],
// C attribute field omitted
//
// compress path ioctl definitions
// SNDRV_COMPRESS_GET_CAPS: Query capability of DSP
// SNDRV_COMPRESS_GET_CODEC_CAPS: Query capability of a codec
// SNDRV_COMPRESS_SET_PARAMS: Set codec and stream parameters
// Note: only codec params can be changed runtime and stream params cant be
// SNDRV_COMPRESS_GET_PARAMS: Query codec params
// SNDRV_COMPRESS_TSTAMP: get the current timestamp value
// SNDRV_COMPRESS_TSTAMP64: get the current timestamp value in 64 bit format
// SNDRV_COMPRESS_AVAIL: get the current buffer avail value.
// This also queries the tstamp properties
// SNDRV_COMPRESS_PAUSE: Pause the running stream
// SNDRV_COMPRESS_RESUME: resume a paused stream
// SNDRV_COMPRESS_START: Start a stream
// SNDRV_COMPRESS_STOP: stop a running stream, discarding ring buffer content
// and the buffers currently with DSP
// SNDRV_COMPRESS_DRAIN: Play till end of buffers and stop after that
// SNDRV_COMPRESS_IOCTL_VERSION: Query the API version
//

//
// TODO
// 1. add mmap support
//

pub const SND_COMPR_TRIGGER_NEXT_TRACK: c_int = 8;
pub const SND_COMPR_TRIGGER_PARTIAL_DRAIN: c_int = 9;
