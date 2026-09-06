//! Automatically rewritten from C Header to Rust Module
//! Source: sound/usb/caiaq/device.h
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

pub const USB_VID_NATIVEINSTRUMENTS: c_uint = 0x17cc;
pub const USB_PID_RIGKONTROL2: c_uint = 0x1969;
pub const USB_PID_RIGKONTROL3: c_uint = 0x1940;
pub const USB_PID_KORECONTROLLER: c_uint = 0x4711;
pub const USB_PID_KORECONTROLLER2: c_uint = 0x4712;
pub const USB_PID_AK1: c_uint = 0x0815;
pub const USB_PID_AUDIO2DJ: c_uint = 0x041c;
pub const USB_PID_AUDIO4DJ: c_uint = 0x0839;
pub const USB_PID_AUDIO8DJ: c_uint = 0x1978;
pub const USB_PID_SESSIONIO: c_uint = 0x1915;
pub const USB_PID_GUITARRIGMOBILE: c_uint = 0x0d8d;
pub const USB_PID_TRAKTORKONTROLX1: c_uint = 0x2305;
pub const USB_PID_TRAKTORKONTROLS4: c_uint = 0xbaff;
pub const USB_PID_TRAKTORAUDIO2: c_uint = 0x041d;
pub const USB_PID_MASCHINECONTROLLER: c_uint = 0x0808;
pub const EP1_BUFSIZE: c_int = 64;
pub const EP4_BUFSIZE: c_int = 512;
pub const CAIAQ_USB_STR_LEN: c_uint = 0xff;
pub const MAX_STREAMS: c_int = 32;

pub const EP1_CMD_GET_DEVICE_INFO: c_uint = 0x1;
pub const EP1_CMD_READ_ERP: c_uint = 0x2;
pub const EP1_CMD_READ_ANALOG: c_uint = 0x3;
pub const EP1_CMD_READ_IO: c_uint = 0x4;
pub const EP1_CMD_WRITE_IO: c_uint = 0x5;
pub const EP1_CMD_MIDI_READ: c_uint = 0x6;
pub const EP1_CMD_MIDI_WRITE: c_uint = 0x7;
pub const EP1_CMD_AUDIO_PARAMS: c_uint = 0x9;
pub const EP1_CMD_AUTO_MSG: c_uint = 0xb;
pub const EP1_CMD_DIMM_LEDS: c_uint = 0xc;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct caiaq_device_spec {
    pub fw_version: c_ushort,
    pub hw_subtype: c_uchar,
    pub num_erp: c_uchar,
    pub num_analog_in: c_uchar,
    pub num_digital_in: c_uchar,
    pub num_digital_out: c_uchar,
    pub num_analog_audio_out: c_uchar,
    pub num_analog_audio_in: c_uchar,
    pub num_digital_audio_out: c_uchar,
    pub num_digital_audio_in: c_uchar,
    pub num_midi_out: c_uchar,
    pub num_midi_in: c_uchar,
    pub data_alignment: c_uchar,
    pub __packed: },
    pub snd_usb_caiaq_cb_info: struct,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_usb_caiaqdev {
    pub chip: snd_usb_audio,
    pub ep1_in_urb: urb,
    pub midi_out_urb: urb,
    pub data_urbs_in: *mut urb,
    pub data_urbs_out: *mut urb,
    pub data_cb_info: *mut snd_usb_caiaq_cb_info,
    pub ep1_in_buf: [c_uchar; EP1_BUFSIZE],
    pub ep1_out_buf: [c_uchar; EP1_BUFSIZE],
    pub midi_out_buf: [c_uchar; EP1_BUFSIZE],
    pub spec: caiaq_device_spec,
    pub spinlock: spinlock_t,
    pub ep1_wait_queue: wait_queue_head_t,
    pub prepare_wait_queue: wait_queue_head_t,
    pub audio_parm_answer: int spec_received,,
    pub midi_out_active: c_int,
    pub vendor_name: [c_char; CAIAQ_USB_STR_LEN],
    pub product_name: [c_char; CAIAQ_USB_STR_LEN],
    pub n_audio_out: int n_streams, n_audio_in,,
    pub output_running: int streaming, first_packet,,
    pub audio_in_buf_pos: [c_int; MAX_STREAMS],
    pub audio_out_buf_pos: [c_int; MAX_STREAMS],
    pub period_in_count: [c_int; MAX_STREAMS],
    pub period_out_count: [c_int; MAX_STREAMS],
    pub warned: int input_panic, output_panic,,
    pub audio_out_buf: *mut *mut char audio_in_buf,,
    pub bpp: unsigned int samplerates,,
    pub outurb_active_mask: c_ulong,
    pub sub_playback: [*mut snd_pcm_substream; MAX_STREAMS],
    pub sub_capture: [*mut snd_pcm_substream; MAX_STREAMS],
// Controls
    pub control_state: [c_uchar; 256],
    pub ep8_out_buf: [c_uchar; 2],
// Linux input

    pub input_dev: *mut input_dev,
    pub /: *mut *mut char phys[64]; / physical device path,
    pub keycode: [c_ushort; 128],
    pub ep4_in_urb: *mut urb,
    pub ep4_in_buf: [c_uchar; EP4_BUFSIZE],
// ALSA
    pub pcm: *mut snd_pcm,
    pub pcm_info: snd_pcm_hardware,
    pub rmidi: *mut snd_rawmidi,
    pub midi_receive_substream: *mut snd_rawmidi_substream,
    pub midi_out_substream: *mut snd_rawmidi_substream,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_usb_caiaq_cb_info {
    pub cdev: *mut snd_usb_caiaqdev,
    pub index: c_int,
}

extern "C" {
    pub fn snd_usb_caiaq_set_audio_params(cdev: *mut snd_usb_caiaqdev, rate: c_int, depth: c_int, bbp: c_int) -> c_int;
}
extern "C" {
    pub fn snd_usb_caiaq_set_auto_msg(cdev: *mut snd_usb_caiaqdev, digital: c_int, analog: c_int, erp: c_int) -> c_int;
}
