//! Automatically rewritten from C Header to Rust Module
//! Source: sound/usb/usx2y/us144mkii_pcm.h
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

//
// var tascam_pcm_hw - Hardware capabilities for TASCAM US-144MKII PCM.
//
// Defines the supported PCM formats, rates, channels, and buffer/period sizes
// for the TASCAM US-144MKII audio interface.
//
// var tascam_playback_ops - ALSA PCM operations for playback.
//
// This structure defines the callback functions for playback stream operations.
//
// var tascam_capture_ops - ALSA PCM operations for capture.
//
// This structure defines the callback functions for capture stream operations.
//
// playback_urb_complete() - Completion handler for playback isochronous URBs.
// @urb: the completed URB
//
// This function runs in interrupt context. It calculates the number of bytes
// to send in the next set of packets based on the feedback-driven clock,
// copies the audio data from the ALSA ring buffer, and resubmits the URB.
//
extern "C" {
    pub fn playback_urb_complete(urb: *mut urb);
}
//
// feedback_urb_complete() - Completion handler for feedback isochronous URBs.
// @urb: the completed URB
//
// This is the master clock for the driver. It runs in interrupt context.
// It reads the feedback value from the device, which indicates how many
// samples the device has consumed. This information is used to adjust the
// playback rate and to advance the capture stream pointer, keeping both
// streams in sync. It then calls snd_pcm_period_elapsed if necessary and
// resubmits itself.
//
extern "C" {
    pub fn feedback_urb_complete(urb: *mut urb);
}
//
// capture_urb_complete() - Completion handler for capture bulk URBs.
// @urb: the completed URB
//
// This function runs in interrupt context. It copies the received raw data
// into an intermediate ring buffer and then schedules the workqueue to process
// it. It then resubmits the URB to receive more data.
//
extern "C" {
    pub fn capture_urb_complete(urb: *mut urb);
}
//
// tascam_stop_pcm_work_handler() - Work handler to stop PCM streams.
// @work: Pointer to the work_struct.
//
// This function is scheduled to stop PCM streams (playback and capture)
// from a workqueue context, avoiding blocking operations in interrupt context.
//
extern "C" {
    pub fn tascam_stop_pcm_work_handler(work: *mut work_struct);
}
//
// tascam_init_pcm() - Initializes the ALSA PCM device.
// @pcm: Pointer to the ALSA PCM device to initialize.
//
// This function sets up the PCM operations, adds ALSA controls for routing
// and sample rate, and preallocates pages for the PCM buffer.
//
// Return: 0 on success, or a negative error code on failure.
//
extern "C" {
    pub fn tascam_init_pcm(pcm: *mut snd_pcm) -> c_int;
}
//
// us144mkii_configure_device_for_rate() - Set sample rate via USB control msgs
// @tascam: the tascam_card instance
// @rate: the target sample rate (e.g., 44100, 96000)
//
// This function sends a sequence of vendor-specific and UAC control messages
// to configure the device hardware for the specified sample rate.
//
// Return: 0 on success, or a negative error code on failure.
//
extern "C" {
    pub fn us144mkii_configure_device_for_rate(tascam: *mut tascam_card, rate: c_int) -> c_int;
}
//
// process_playback_routing_us144mkii() - Apply playback routing matrix
// @tascam: The driver instance.
// @src_buffer: Buffer containing 4 channels of S24_3LE audio from ALSA.
// @dst_buffer: Buffer to be filled for the USB device.
// @frames: Number of frames to process.
//
// process_capture_routing_us144mkii() - Apply capture routing matrix
// @tascam: The driver instance.
// @decoded_block: Buffer containing 4 channels of S32LE decoded audio.
// @routed_block: Buffer to be filled for ALSA.
//
// tascam_pcm_hw_params() - Configures hardware parameters for PCM streams.
// @substream: The ALSA PCM substream.
// @params: The hardware parameters to apply.
//
// This function allocates pages for the PCM buffer and, for playback streams,
// selects the appropriate feedback patterns based on the requested sample rate.
// It also configures the device hardware for the selected sample rate if it
// has changed.
//
// Return: 0 on success, or a negative error code on failure.
//
// tascam_pcm_hw_free() - Frees hardware parameters for PCM streams.
// @substream: The ALSA PCM substream.
//
// This function is a stub for freeing hardware-related resources.
//
// Return: 0 on success.
//
extern "C" {
    pub fn tascam_pcm_hw_free(substream: *mut snd_pcm_substream) -> c_int;
}
//
// tascam_pcm_trigger() - Triggers the start or stop of PCM streams.
// @substream: The ALSA PCM substream.
// @cmd: The trigger command (e.g., SNDRV_PCM_TRIGGER_START).
//
// This function handles starting and stopping of playback and capture streams
// by submitting or killing the associated URBs.
//
// Return: 0 on success, or a negative error code on failure.
//
extern "C" {
    pub fn tascam_pcm_trigger(substream: *mut snd_pcm_substream, cmd: c_int) -> c_int;
}
//
// tascam_capture_work_handler() - Deferred work for processing capture data.
// @work: the work_struct instance
//
// This function runs in a kernel thread context, not an IRQ context. It reads
// raw data from the capture ring buffer, decodes it, applies routing, and
// copies the final audio data into the ALSA capture ring buffer. This offloads
// the CPU-intensive decoding from the time-sensitive URB completion handlers.
//
extern "C" {
    pub fn tascam_capture_work_handler(work: *mut work_struct);
}
