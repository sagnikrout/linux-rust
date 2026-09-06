//! Automatically rewritten from C Header to Rust Module
//! Source: sound/usb/usx2y/us144mkii.h
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
// Copyright (c) 2025 Šerif Rami <ramiserifpersia@gmail.com>

// --- USB Device Identification ---
pub const USB_VID_TASCAM: c_uint = 0x0644;
pub const USB_PID_TASCAM_US144: c_uint = 0x800f;
pub const USB_PID_TASCAM_US144MKII: c_uint = 0x8020;
// --- USB Endpoints (Alternate Setting 1) ---
pub const EP_PLAYBACK_FEEDBACK: c_uint = 0x81;
pub const EP_AUDIO_OUT: c_uint = 0x02;
pub const EP_MIDI_IN: c_uint = 0x83;
pub const EP_MIDI_OUT: c_uint = 0x04;
pub const EP_AUDIO_IN: c_uint = 0x86;
// --- USB Control Message Protocol ---

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uac_request {
    UAC_SET_CUR = 0x01,
    UAC_GET_CUR = 0x81,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uac_control_selector {
    UAC_SAMPLING_FREQ_CONTROL = 0x0100,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tascam_vendor_request {
    VENDOR_REQ_REGISTER_WRITE = 0x41,
    VENDOR_REQ_DEEP_SLEEP = 0x44,
    VENDOR_REQ_MODE_CONTROL = 0x49,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tascam_mode_value {
    MODE_VAL_HANDSHAKE_READ = 0x0000,
    MODE_VAL_CONFIG = 0x0010,
    MODE_VAL_STREAM_START = 0x0030,
}

pub const HANDSHAKE_SUCCESS_VAL: c_uint = 0x12;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tascam_register {
    REG_ADDR_UNKNOWN_0D = 0x0d04,
    REG_ADDR_UNKNOWN_0E = 0x0e00,
    REG_ADDR_UNKNOWN_0F = 0x0f00,
    REG_ADDR_RATE_44100 = 0x1000,
    REG_ADDR_RATE_48000 = 0x1002,
    REG_ADDR_RATE_88200 = 0x1008,
    REG_ADDR_RATE_96000 = 0x100a,
    REG_ADDR_UNKNOWN_11 = 0x110b,
}

pub const REG_VAL_ENABLE: c_uint = 0x0101;
// --- URB Configuration ---
pub const NUM_PLAYBACK_URBS: c_int = 4;
pub const PLAYBACK_URB_PACKETS: c_int = 8;
pub const NUM_FEEDBACK_URBS: c_int = 4;
pub const FEEDBACK_URB_PACKETS: c_int = 1;
pub const FEEDBACK_PACKET_SIZE: c_int = 3;
pub const NUM_CAPTURE_URBS: c_int = 8;
pub const CAPTURE_URB_SIZE: c_int = 512;

pub const NUM_MIDI_IN_URBS: c_int = 4;
pub const MIDI_IN_BUF_SIZE: c_int = 64;

pub const MIDI_OUT_BUF_SIZE: c_int = 64;
pub const NUM_MIDI_OUT_URBS: c_int = 4;
pub const USB_CTRL_TIMEOUT_MS: c_int = 1000;
pub const FEEDBACK_SYNC_LOSS_THRESHOLD: c_int = 41;
// --- Audio Format Configuration ---
pub const BYTES_PER_SAMPLE: c_int = 3;
pub const NUM_CHANNELS: c_int = 4;

pub const FEEDBACK_ACCUMULATOR_SIZE: c_int = 128;
// --- Capture Decoding Defines ---
pub const DECODED_CHANNELS_PER_FRAME: c_int = 4;
pub const DECODED_SAMPLE_SIZE: c_int = 4;
pub const FRAMES_PER_DECODE_BLOCK: c_int = 8;
pub const RAW_BYTES_PER_DECODE_BLOCK: c_int = 512;
//
// struct us144mkii_frame_pattern_observer - State for dynamic feedback
// patterns.
// @sample_rate_khz: The current sample rate in kHz.
// @base_feedback_value: The nominal feedback value for the current rate.
// @feedback_offset: An offset to align the feedback value range.
// @full_frame_patterns: A 2D array of pre-calculated packet size patterns.
// @current_index: The current index into the pattern array.
// @previous_index: The previous index, used for state tracking.
// @sync_locked: A flag indicating if the pattern has locked to the stream.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct us144mkii_frame_pattern_observer {
    pub sample_rate_khz: c_uint,
    pub base_feedback_value: c_uint,
    pub feedback_offset: c_int,
    pub full_frame_patterns: [c_uint; 5][8],
    pub current_index: c_uint,
    pub previous_index: c_uint,
    pub sync_locked: bool,
}

//
// struct tascam_card - Main driver data structure for the TASCAM US-144MKII.
// @dev: Pointer to the USB device.
// @iface0: Pointer to USB interface 0 (audio).
// @iface1: Pointer to USB interface 1 (MIDI).
// @card: Pointer to the ALSA sound card instance.
// @pcm: Pointer to the ALSA PCM device.
// @rmidi: Pointer to the ALSA rawmidi device.
//
// @playback_substream: Pointer to the active playback PCM substream.
// @playback_urbs: Array of URBs for playback.
// @playback_urb_alloc_size: Size of allocated buffer for each playback URB.
// @feedback_urbs: Array of URBs for feedback.
// @feedback_urb_alloc_size: Size of allocated buffer for each feedback URB.
// @playback_active: Atomic flag indicating if playback is active.
// @playback_frames_consumed: Total frames consumed by playback.
// @driver_playback_pos: Current position in the ALSA playback buffer (frames).
// @last_period_pos: Last reported period position for playback.
//
// @capture_substream: Pointer to the active capture PCM substream.
// @capture_urbs: Array of URBs for capture.
// @capture_urb_alloc_size: Size of allocated buffer for each capture URB.
// @capture_active: Atomic flag indicating if capture is active.
// @driver_capture_pos: Current position in the ALSA capture buffer (frames).
// @capture_frames_processed: Total frames processed for capture.
// @last_capture_period_pos: Last reported period position for capture.
// @capture_ring_buffer: Ring buffer for raw capture data from USB.
// @capture_ring_buffer_read_ptr: Read pointer for the capture ring buffer.
// @capture_ring_buffer_write_ptr: Write pointer for the capture ring buffer.
// @capture_decode_raw_block: Buffer for a raw 512-byte capture block.
// @capture_decode_dst_block: Buffer for decoded 32-bit capture samples.
// @capture_routing_buffer: Intermediate buffer for capture routing.
// @capture_work: Work struct for deferred capture processing.
// @stop_work: Work struct for deferred stream stopping.
// @stop_pcm_work: Work struct for stopping PCM due to a fatal error (e.g.
// xrun).
//
// @midi_in_substream: Pointer to the active MIDI input substream.
// @midi_out_substream: Pointer to the active MIDI output substream.
// @midi_in_urbs: Array of URBs for MIDI input.
// @midi_out_urbs: Array of URBs for MIDI output.
// @midi_in_active: Atomic flag indicating if MIDI input is active.
// @midi_out_active: Atomic flag indicating if MIDI output is active.
// @midi_in_fifo: FIFO for raw MIDI input data.
// @midi_in_work: Work struct for deferred MIDI input processing.
// @midi_out_work: Work struct for deferred MIDI output processing.
// @midi_in_lock: Spinlock for MIDI input FIFO.
// @midi_out_lock: Spinlock for MIDI output.
// @midi_out_urbs_in_flight: Bitmap of MIDI output URBs currently in flight.
// @midi_running_status: Stores the last MIDI status byte for running status.
// @error_timer: Timer for MIDI error retry logic.
//
// @lock: Main spinlock for protecting shared driver state.
// @active_urbs: Atomic counter for active URBs.
// @current_rate: Currently configured sample rate of the device.
// @line_out_source: Source for Line Outputs (0: Playback 1-2, 1: Playback 3-4).
// @digital_out_source: Source for Digital Outputs (0: Playback 1-2, 1: Playback
// 3-4).
// @capture_12_source: Source for Capture channels 1-2 (0: Analog In, 1: Digital
// In).
// @capture_34_source: Source for Capture channels 3-4 (0: Analog In, 1: Digital
// In).
//
// @feedback_accumulator_pattern: Stores the calculated frames per packet for
// feedback.
// @feedback_pattern_out_idx: Read index for feedback_accumulator_pattern.
// @feedback_pattern_in_idx: Write index for feedback_accumulator_pattern.
// @feedback_synced: Flag indicating if feedback is synced.
// @feedback_consecutive_errors: Counter for consecutive feedback errors.
// @feedback_urb_skip_count: Number of feedback URBs to skip initially for
// stabilization.
// @fpo: Holds the state for the dynamic feedback pattern generation.
//
// @playback_anchor: USB anchor for playback URBs.
// @capture_anchor: USB anchor for capture URBs.
// @feedback_anchor: USB anchor for feedback URBs.
// @midi_in_anchor: USB anchor for MIDI input URBs.
// @midi_out_anchor: USB anchor for MIDI output URBs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tascam_card {
// --- Core device pointers ---
    pub dev: *mut usb_device,
    pub iface0: *mut usb_interface,
    pub iface1: *mut usb_interface,
    pub card: *mut snd_card,
    pub pcm: *mut snd_pcm,
    pub rmidi: *mut snd_rawmidi,
// --- PCM Substreams ---
    pub playback_substream: *mut snd_pcm_substream,
    pub capture_substream: *mut snd_pcm_substream,
// --- URBs and Anchors ---
    pub playback_urbs: [*mut urb; NUM_PLAYBACK_URBS],
    pub playback_urb_alloc_size: usize,
    pub feedback_urbs: [*mut urb; NUM_FEEDBACK_URBS],
    pub feedback_urb_alloc_size: usize,
    pub capture_urbs: [*mut urb; NUM_CAPTURE_URBS],
    pub capture_urb_alloc_size: usize,
    pub midi_in_urbs: [*mut urb; NUM_MIDI_IN_URBS],
    pub midi_out_urbs: [*mut urb; NUM_MIDI_OUT_URBS],
    pub playback_anchor: usb_anchor,
    pub capture_anchor: usb_anchor,
    pub feedback_anchor: usb_anchor,
    pub midi_in_anchor: usb_anchor,
    pub midi_out_anchor: usb_anchor,
// --- Stream State ---
    pub lock: spinlock_t,
    pub playback_active: core::sync::atomic::AtomicI32,
    pub capture_active: core::sync::atomic::AtomicI32,
    pub active_urbs: core::sync::atomic::AtomicI32,
    pub current_rate: c_int,
// --- Playback State ---
    pub playback_frames_consumed: u64,
    pub driver_playback_pos: snd_pcm_uframes_t,
    pub last_period_pos: u64,
// --- Capture State ---
    pub capture_frames_processed: u64,
    pub driver_capture_pos: snd_pcm_uframes_t,
    pub last_capture_period_pos: u64,
    pub capture_ring_buffer: *mut u8,
    pub capture_ring_buffer_read_ptr: usize,
    pub capture_ring_buffer_write_ptr: usize,
    pub capture_decode_raw_block: *mut u8,
    pub capture_decode_dst_block: *mut i32,
    pub capture_routing_buffer: *mut i32,
// --- MIDI State ---
    pub midi_in_substream: *mut snd_rawmidi_substream,
    pub midi_out_substream: *mut snd_rawmidi_substream,
    pub midi_in_active: core::sync::atomic::AtomicI32,
    pub midi_out_active: core::sync::atomic::AtomicI32,
    pub midi_in_fifo: kfifo,
    pub midi_in_lock: spinlock_t,
    pub midi_out_lock: spinlock_t,
    pub midi_out_urbs_in_flight: c_ulong,
    pub midi_running_status: u8,
    pub error_timer: timer_list,
    pub midi_out_drain_completion: completion,
// --- Feedback Sync State ---
    pub feedback_accumulator_pattern: [c_uint; FEEDBACK_ACCUMULATOR_SIZE],
    pub feedback_pattern_out_idx: c_uint,
    pub feedback_pattern_in_idx: c_uint,
    pub feedback_synced: bool,
    pub feedback_consecutive_errors: c_uint,
    pub feedback_urb_skip_count: c_uint,
    pub fpo: us144mkii_frame_pattern_observer,
// --- Workqueues ---
    pub stop_work: work_struct,
    pub stop_pcm_work: work_struct,
    pub capture_work: work_struct,
    pub midi_in_work: work_struct,
    pub midi_out_work: work_struct,
// --- Mixer/Routing State ---
    pub line_out_source: c_uint,
    pub digital_out_source: c_uint,
    pub capture_12_source: c_uint,
    pub capture_34_source: c_uint,
}

// main.c
//
// tascam_free_urbs() - Free all allocated URBs and associated buffers.
// @tascam: the tascam_card instance
//
// This function kills, unlinks, and frees all playback, feedback, capture,
// and MIDI URBs, along with their transfer buffers and the capture
// ring/decode buffers.
//
extern "C" {
    pub fn tascam_free_urbs(tascam: *mut tascam_card);
}
//
// tascam_alloc_urbs() - Allocate all URBs and associated buffers.
// @tascam: the tascam_card instance
//
// This function allocates and initializes all URBs for playback, feedback,
// capture, and MIDI, as well as the necessary buffers for data processing.
//
// Return: 0 on success, or a negative error code on failure.
//
extern "C" {
    pub fn tascam_alloc_urbs(tascam: *mut tascam_card) -> c_int;
}
//
// tascam_stop_work_handler() - Work handler to stop all active streams.
// @work: Pointer to the work_struct.
//
// This function is scheduled to stop all active URBs (playback, feedback,
// capture) and reset the active_urbs counter.
//
extern "C" {
    pub fn tascam_stop_work_handler(work: *mut work_struct);
}
// us144mkii_pcm.h

// us144mkii_midi.c
//
// tascam_midi_in_urb_complete() - Completion handler for MIDI IN URBs
// @urb: The completed URB.
//
// This function runs in interrupt context. It places the raw data from the
// USB endpoint into a kfifo and schedules a work item to process it later,
// ensuring the interrupt handler remains fast.
//
extern "C" {
    pub fn tascam_midi_in_urb_complete(urb: *mut urb);
}
//
// tascam_midi_out_urb_complete() - Completion handler for MIDI OUT bulk URB.
// @urb: The completed URB.
//
// This function runs in interrupt context. It marks the output URB as no
// longer in-flight. It then re-schedules the work handler to check for and
// send any more data waiting in the ALSA buffer. This is a safe, non-blocking
// way to continue the data transmission chain.
//
extern "C" {
    pub fn tascam_midi_out_urb_complete(urb: *mut urb);
}
//
// tascam_create_midi() - Create and initialize the ALSA rawmidi device.
// @tascam: The driver instance.
//
// Return: 0 on success, or a negative error code on failure.
//
extern "C" {
    pub fn tascam_create_midi(tascam: *mut tascam_card) -> c_int;
}
// us144mkii_controls.c
//
// tascam_create_controls() - Creates and adds ALSA mixer controls for the
// device.
// @tascam: The driver instance.
//
// This function registers custom ALSA controls for managing audio routing
// (line out source, digital out source, capture 1-2 source, capture 3-4 source)
// and displaying the current sample rate.
//
// Return: 0 on success, or a negative error code on failure.
//
extern "C" {
    pub fn tascam_create_controls(tascam: *mut tascam_card) -> c_int;
}
