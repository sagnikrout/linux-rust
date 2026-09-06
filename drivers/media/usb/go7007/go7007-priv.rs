//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/go7007/go7007-priv.h
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
// Copyright (C) 2005-2006 Micronas USA Inc.
//
// This is the private include file for the go7007 driver.  It should not
// be included by anybody but the driver itself, and especially not by
// user-space applications.
//

// IDs to activate board-specific support code
pub const GO7007_BOARDID_MATRIX_II: c_int = 0;
pub const GO7007_BOARDID_MATRIX_RELOAD: c_int = 1;
pub const GO7007_BOARDID_STAR_TREK: c_int = 2;
pub const GO7007_BOARDID_PCI_VOYAGER: c_int = 3;
pub const GO7007_BOARDID_XMEN: c_int = 4;
pub const GO7007_BOARDID_XMEN_II: c_int = 5;
pub const GO7007_BOARDID_XMEN_III: c_int = 6;
pub const GO7007_BOARDID_MATRIX_REV: c_int = 7;
pub const GO7007_BOARDID_PX_M402U: c_int = 8;
pub const GO7007_BOARDID_PX_TV402U: c_int = 9;

pub const GO7007_BOARDID_ENDURA: c_int = 11;
pub const GO7007_BOARDID_ADLINK_MPG24: c_int = 12;

pub const GO7007_BOARDID_ADS_USBAV_709: c_int = 14;
// Various characteristics of each board

// Characteristics of sensor devices

pub const GO7007_SENSOR_CONFIG_MASK: c_uint = 0x7f;

// Characteristics of audio sensor devices

#[repr(C)]
#[derive(Copy, Clone)]
pub struct go7007_board_info {
    pub flags: c_uint,
    pub hpi_buffer_cap: c_int,
    pub sensor_flags: c_uint,
    pub sensor_width: c_int,
    pub sensor_height: c_int,
    pub sensor_framerate: c_int,
    pub sensor_h_offset: c_int,
    pub sensor_v_offset: c_int,
    pub audio_flags: c_uint,
    pub audio_rate: c_int,
    pub audio_bclk_div: c_int,
    pub audio_main_div: c_int,
    pub num_i2c_devs: c_int,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct go_i2c {
    pub type: *const c_char,
    pub is_video:1: c_uint,
    pub is_audio:1: c_uint,
    pub addr: c_int,
    pub flags: u32,
    pub i2c_devs: [}; 5],
    pub num_inputs: c_int,
    pub video_input: c_int,
    pub audio_index: c_int,
    pub name: *mut c_char,
    pub inputs: [}; 4],
    pub video_config: c_int,
    pub num_aud_inputs: c_int,
    pub audio_input: c_int,
    pub name: *mut c_char,
    pub aud_inputs: [}; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct go7007_hpi_ops {
    pub go): *mut *mut int (interface_reset)(struct go7007,
    pub data): *mut *mut *mut int (write_interrupt)(struct go7007 go, int addr, int,
    pub go): *mut *mut int (read_interrupt)(struct go7007,
    pub go): *mut *mut int (stream_start)(struct go7007,
    pub go): *mut *mut int (stream_stop)(struct go7007,
    pub len): *mut *mut *mut *mut int (send_firmware)(struct go7007 go, u8 data, int,
    pub arg): *mut *mut *mut int (send_command)(struct go7007 go, unsigned int cmd, void,
    pub go): *mut *mut void (release)(struct go7007,
}

// The video buffer size must be a multiple of PAGE_SIZE

#[repr(C)]
#[derive(Copy, Clone)]
pub struct go7007_buffer {
    pub vb: vb2_v4l2_buffer,
    pub list: list_head,
    pub frame_offset: c_uint,
    pub modet_active: u32,
}

pub const GO7007_RATIO_1_1: c_int = 0;
pub const GO7007_RATIO_4_3: c_int = 1;
pub const GO7007_RATIO_16_9: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum go7007_parser_state {
    STATE_DATA,
    STATE_00,
    STATE_00_00,
    STATE_00_00_01,
    STATE_FF,
    STATE_VBI_LEN_A,
    STATE_VBI_LEN_B,
    STATE_MODET_MAP,
    STATE_UNPARSED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct go7007 {
    pub dev: *mut device,
    pub bus_info: [u8; 32],
    pub board_info: *const go7007_board_info,
    pub board_id: c_uint,
    pub tuner_type: c_int,
    pub /: *mut *mut int channel_number; / for multi-channel boards like Adlink PCI-MPG24,
    pub name: [c_char; 64],
    pub vdev: video_device,
    pub boot_fw: *mut c_void,
    pub boot_fw_len: unsigned,
    pub v4l2_dev: v4l2_device,
    pub hdl: v4l2_ctrl_handler,
    pub mpeg_video_encoding: *mut v4l2_ctrl,
    pub mpeg_video_gop_size: *mut v4l2_ctrl,
    pub mpeg_video_gop_closure: *mut v4l2_ctrl,
    pub mpeg_video_bitrate: *mut v4l2_ctrl,
    pub mpeg_video_aspect_ratio: *mut v4l2_ctrl,
    pub mpeg_video_b_frames: *mut v4l2_ctrl,
    pub mpeg_video_rep_seqheader: *mut v4l2_ctrl,
    pub modet_mode: *mut v4l2_ctrl,
    pub status: { STATUS_INIT, STATUS_ONLINE, STATUS_SHUTDOWN },
    pub spinlock: spinlock_t,
    pub hw_lock: mutex,
    pub serialize_lock: mutex,
    pub audio_enabled: c_int,
    pub sd_video: *mut v4l2_subdev,
    pub sd_audio: *mut v4l2_subdev,
    pub usb_buf: [u8; 16],
// Video input
    pub input: c_int,
    pub aud_input: c_int,
    pub standard: { GO7007_STD_NTSC, GO7007_STD_PAL, GO7007_STD_OTHER },
    pub std: v4l2_std_id,
    pub sensor_framerate: c_int,
    pub width: c_int,
    pub height: c_int,
    pub encoder_h_offset: c_int,
    pub encoder_v_offset: c_int,
    pub encoder_h_halve:1: c_uint,
    pub encoder_v_halve:1: c_uint,
    pub encoder_subsample:1: c_uint,
// Encoder config
    pub format: u32,
    pub bitrate: c_int,
    pub fps_scale: c_int,
    pub pali: c_int,
    pub aspect_ratio: c_int,
    pub gop_size: c_int,
    pub ipb:1: c_uint,
    pub closed_gop:1: c_uint,
    pub repeat_seqhead:1: c_uint,
    pub seq_header_enable:1: c_uint,
    pub gop_header_enable:1: c_uint,
    pub dvd_mode:1: c_uint,
    pub interlace_coding:1: c_uint,
// Motion detection
    pub modet_enable:1: c_uint,
    pub enable:1: c_uint,
    pub pixel_threshold: c_int,
    pub motion_threshold: c_int,
    pub mb_threshold: c_int,
    pub modet: [}; 4],
    pub modet_map: [c_uchar; 1624],
    pub active_map: [c_uchar; 216],
    pub modet_event_status: u32,
// Video streaming
    pub queue_lock: mutex,
    pub vidq: vb2_queue,
    pub state: go7007_parser_state,
    pub parse_length: c_int,
    pub modet_word: u16,
    pub seen_frame: c_int,
    pub next_seq: u32,
    pub vidq_active: list_head,
    pub frame_waitq: wait_queue_head_t,
    pub active_buf: *mut go7007_buffer,
// Audio streaming
    pub length): *mut *mut *mut *mut void (audio_deliver)(struct go7007 go, u8 buf, int,
    pub snd_context: *mut c_void,
// I2C
    pub i2c_adapter_online: c_int,
    pub i2c_adapter: i2c_adapter,
// HPI driver
    pub hpi_ops: *const go7007_hpi_ops,
    pub hpi_context: *mut c_void,
    pub interrupt_available: c_int,
    pub interrupt_waitq: wait_queue_head_t,
    pub interrupt_value: c_ushort,
    pub interrupt_data: c_ushort,
}

extern "C" {
    pub fn container_of(_arg: v4l2_dev, go7007: struct, _arg: v4l2_dev) -> return;
}
// All of these must be called with the hpi_lock mutex held!

// go7007-driver.c
extern "C" {
    pub fn go7007_read_addr(go: *mut go7007, addr: u16, data: *mut u16) -> c_int;
}
extern "C" {
    pub fn go7007_read_interrupt(go: *mut go7007, value: *mut u16, data: *mut u16) -> c_int;
}
extern "C" {
    pub fn go7007_boot_encoder(go: *mut go7007, init_i2c: c_int) -> c_int;
}
extern "C" {
    pub fn go7007_reset_encoder(go: *mut go7007) -> c_int;
}
extern "C" {
    pub fn go7007_register_encoder(go: *mut go7007, num_i2c_devs: unsigned) -> c_int;
}
extern "C" {
    pub fn go7007_start_encoder(go: *mut go7007) -> c_int;
}
extern "C" {
    pub fn go7007_parse_video_stream(go: *mut go7007, buf: *mut u8, length: c_int);
}
extern "C" {
    pub fn go7007_update_board(go: *mut go7007);
}
// go7007-fw.c
extern "C" {
    pub fn go7007_construct_fw_image(go: *mut go7007, fw: *mut u8, fwlen: *mut c_int) -> c_int;
}
// go7007-i2c.c
extern "C" {
    pub fn go7007_i2c_init(go: *mut go7007) -> c_int;
}
extern "C" {
    pub fn go7007_i2c_remove(go: *mut go7007) -> c_int;
}
// go7007-v4l2.c
extern "C" {
    pub fn go7007_v4l2_init(go: *mut go7007) -> c_int;
}
extern "C" {
    pub fn go7007_v4l2_ctrl_init(go: *mut go7007) -> c_int;
}
extern "C" {
    pub fn go7007_v4l2_remove(go: *mut go7007);
}
// snd-go7007.c
extern "C" {
    pub fn go7007_snd_init(go: *mut go7007) -> c_int;
}
extern "C" {
    pub fn go7007_snd_remove(go: *mut go7007) -> c_int;
}
