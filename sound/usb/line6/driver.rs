//! Automatically rewritten from C Header to Rust Module
//! Source: sound/usb/line6/driver.h
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
// Line 6 Linux USB driver
//
// Copyright (C) 2004-2010 Markus Grabner (line6@grabner-graz.at)
//

// USB 1.1 speed configuration
pub const USB_LOW_INTERVALS_PER_SECOND: c_int = 1000;
pub const USB_LOW_ISO_BUFFERS: c_int = 2;
// USB 2.0+ speed configuration
pub const USB_HIGH_INTERVALS_PER_SECOND: c_int = 8000;
pub const USB_HIGH_ISO_BUFFERS: c_int = 16;
// Fallback USB interval and max packet size values
pub const LINE6_FALLBACK_INTERVAL: c_int = 10;
pub const LINE6_FALLBACK_MAXPACKETSIZE: c_int = 16;
pub const LINE6_TIMEOUT: c_int = 1000;
pub const LINE6_BUFSIZE_LISTEN: c_int = 64;
pub const LINE6_MIDI_MESSAGE_MAXLEN: c_int = 256;
pub const LINE6_RAW_MESSAGES_MAXCOUNT_ORDER: c_int = 7;
// 4k packets are common, BUFSIZE * MAXCOUNT should be bigger...

//
pub const LINE6_PARAM_CHANGE: c_uint = 0xb0;
pub const LINE6_PROGRAM_CHANGE: c_uint = 0xc0;
pub const LINE6_SYSEX_BEGIN: c_uint = 0xf0;
pub const LINE6_SYSEX_END: c_uint = 0xf7;
pub const LINE6_RESET: c_uint = 0xff;
//
pub const LINE6_CHANNEL_HOST: c_uint = 0x00;
//
pub const LINE6_CHANNEL_DEVICE: c_uint = 0x02;

pub const LINE6_CHANNEL_MASK: c_uint = 0x0f;

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct line6_properties {
// Card id string (maximum 16 characters).
// This can be used to address the device in ALSA programs as
// "default:CARD=<id>"
//
    pub id: *const c_char,
// Card short name (maximum 32 characters)
    pub name: *const c_char,
// Bit vector defining this device's capabilities in line6usb driver
    pub capabilities: c_int,
    pub altsetting: c_int,
    pub ctrl_if: c_uint,
    pub ep_ctrl_r: c_uint,
    pub ep_ctrl_w: c_uint,
    pub ep_audio_r: c_uint,
    pub ep_audio_w: c_uint,
}

// Capability bits
// device supports settings parameter via USB
// device supports PCM input/output via USB
// device supports hardware monitoring
// device requires output data when input is read
// device uses raw MIDI via USB (data endpoints)
// device provides low-level information
// device provides hardware monitoring volume control
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_line6 {
// USB device
    pub usbdev: *mut usb_device,
// Properties
    pub properties: *const line6_properties,
// Interval for data USB packets
    pub interval: c_int,
// ...for isochronous transfers framing
    pub intervals_per_second: c_int,
// Number of isochronous URBs used for frame transfers
    pub iso_buffers: c_int,
// Maximum size of data USB packet
    pub max_packet_size: c_int,
// Device representing the USB interface
    pub ifcdev: *mut device,
// Line 6 sound card data structure.
// Each device has at least MIDI or PCM.
//
    pub card: *mut snd_card,
// Line 6 PCM device data structure
    pub line6pcm: *mut snd_line6_pcm,
// Line 6 MIDI device data structure
    pub line6midi: *mut snd_line6_midi,
// URB for listening to POD data endpoint
    pub urb_listen: *mut urb,
// Buffer for incoming data from POD data endpoint
    pub buffer_listen: *mut c_uchar,
// Buffer for message to be processed, generated from MIDI layer
    pub buffer_message: *mut c_uchar,
// Length of message to be processed, generated from MIDI layer
    pub message_length: c_int,
// Circular buffer for non-MIDI control messages
    pub read_lock: mutex,
    pub wait_queue: wait_queue_head_t,
    pub active:1: c_uint,
    pub nonblock:1: c_uint,
    pub messages: },
// Work for delayed PCM startup
    pub startup_work: delayed_work,
// If MIDI is supported, buffer_message contains the pre-processed data;
// otherwise the data is only in urb_listen (buffer_incoming).
//
    pub ): *mut *mut void (process_message)(struct usb_line6,
    pub line6): *mut *mut void (disconnect)(struct usb_line6,
    pub line6): *mut *mut void (startup)(struct usb_line6,
}

extern "C" {
    pub fn line6_version_request_async(line6: *mut usb_line6) -> c_int;
}
extern "C" {
    pub fn line6_disconnect(interface: *mut usb_interface);
}

extern "C" {
    pub fn line6_suspend(interface: *mut usb_interface, message: pm_message_t) -> c_int;
}
extern "C" {
    pub fn line6_resume(interface: *mut usb_interface) -> c_int;
}

