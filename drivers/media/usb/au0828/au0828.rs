//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/au0828/au0828.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Driver for the Auvitek AU0828 USB bridge
//
// Copyright (c) 2008 Steven Toth <stoth@linuxtv.org>
//

// Analog

// DVB

pub const URB_COUNT: c_int = 16;

// Analog constants
pub const NTSC_STD_W: c_int = 720;
pub const NTSC_STD_H: c_int = 480;
pub const AU0828_INTERLACED_DEFAULT: c_int = 1;
// Definition for AU0828 USB transfer

pub const AU0828_ISO_PACKETS_PER_URB: c_int = 128;
pub const AU0828_MIN_BUF: c_int = 4;
pub const AU0828_DEF_BUF: c_int = 8;
pub const AU0828_MAX_INPUT: c_int = 4;
// au0828 resource types (used for res_get/res_lock etc
pub const AU0828_RESOURCE_VIDEO: c_uint = 0x01;
pub const AU0828_RESOURCE_VBI: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum au0828_itype {
    AU0828_VMUX_UNDEFINED = 0,
    AU0828_VMUX_COMPOSITE,
    AU0828_VMUX_SVIDEO,
    AU0828_VMUX_CABLE,
    AU0828_VMUX_TELEVISION,
    AU0828_VMUX_DVB,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct au0828_input {
    pub type: au0828_itype,
    pub vmux: c_uint,
    pub amux: c_uint,
    pub enable): *mut *mut *mut void (audio_setup) (void priv, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct au0828_board {
    pub name: *mut c_char,
    pub tuner_type: c_uint,
    pub tuner_addr: c_uchar,
    pub i2c_clk_divider: c_uchar,
    pub has_ir_i2c:1: c_uchar,
    pub has_analog:1: c_uchar,
    pub input: [au0828_input; AU0828_MAX_INPUT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct au0828_dvb {
    pub lock: mutex,
    pub adapter: dvb_adapter,
    pub frontend: *mut dvb_frontend,
    pub demux: dvb_demux,
    pub dmxdev: dmxdev,
    pub fe_hw: dmx_frontend,
    pub fe_mem: dmx_frontend,
    pub net: dvb_net,
    pub feeding: c_int,
    pub start_count: c_int,
    pub stop_count: c_int,
    pub fe): *mut *mut int (set_frontend)(struct dvb_frontend,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum au0828_stream_state {
    STREAM_OFF,
    STREAM_INTERRUPT,
    STREAM_ON
}

// device state
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum au0828_dev_state {
    DEV_INITIALIZED = 0,
    DEV_DISCONNECTED = 1,
    DEV_MISCONFIGURED = 2
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct au0828_usb_isoc_ctl {
// max packet size of isoc transaction
    pub max_pkt_size: c_int,
// number of allocated urbs
    pub num_bufs: c_int,
// urb for isoc transfers
    pub urb: *mut urb,
// transfer buffers for isoc transfer
    pub transfer_buffer: *mut c_char,
// Last buffer command and region
    pub cmd: u8,
    pub pktsize: int pos, size,,
// Last field: ODD or EVEN?
    pub field: c_int,
// Stores incomplete commands
    pub tmp_buf: u32,
    pub tmp_buf_len: c_int,
// Stores already requested buffers
    pub buf: *mut au0828_buffer,
    pub vbi_buf: *mut au0828_buffer,
// Stores the number of received fields
    pub nfields: c_int,
// isoc urb callback
    pub urb): *mut *mut *mut int (isoc_copy) (struct au0828_dev dev, struct urb,
}

// buffer for one video frame
#[repr(C)]
#[derive(Copy, Clone)]
pub struct au0828_buffer {
// common v4l buffer stuff -- must be first
    pub vb: vb2_v4l2_buffer,
    pub list: list_head,
    pub mem: *mut c_void,
    pub length: c_ulong,
    pub top_field: c_int,
// pointer to vmalloc memory address in vb
    pub vb_buf: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct au0828_dmaqueue {
    pub active: list_head,
// Counters to control buffer fill
    pub pos: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct au0828_dev {
    pub mutex: mutex,
    pub usbdev: *mut usb_device,
    pub boardnr: c_int,
    pub board: au0828_board,
    pub ctrlmsg: [u8; 64],
// I2C
    pub i2c_adap: i2c_adapter,
    pub i2c_algo: i2c_algorithm,
    pub i2c_client: i2c_client,
    pub i2c_rc: u32,
// Digital
    pub dvb: au0828_dvb,
    pub restart_streaming: work_struct,
    pub bulk_timeout: timer_list,
    pub bulk_timeout_running: c_int,

// Analog
    pub v4l2_dev: v4l2_device,
    pub v4l2_ctrl_hdl: v4l2_ctrl_handler,

    pub ir: *mut au0828_rc,

    pub vdev: video_device,
    pub vbi_dev: video_device,
// Videobuf2
    pub vb_vidq: vb2_queue,
    pub vb_vbiq: vb2_queue,
    pub vb_queue_lock: mutex,
    pub vb_vbi_queue_lock: mutex,
    pub frame_count: c_uint,
    pub vbi_frame_count: c_uint,
    pub vid_timeout: timer_list,
    pub vid_timeout_running: c_int,
    pub vbi_timeout: timer_list,
    pub vbi_timeout_running: c_int,
    pub users: c_int,
    pub streaming_users: c_int,
    pub width: c_int,
    pub height: c_int,
    pub vbi_width: c_int,
    pub vbi_height: c_int,
    pub vbi_read: u32,
    pub std: v4l2_std_id,
    pub field_size: u32,
    pub frame_size: u32,
    pub bytesperline: u32,
    pub type: c_int,
    pub ctrl_ainput: u8,
    pub isoc_in_endpointaddr: __u8,
    pub isoc_init_ok: u8,
    pub greenscreen_detected: c_int,
    pub ctrl_freq: c_int,
    pub input_type: c_int,
    pub std_set_in_tuner_core: c_int,
    pub ctrl_input: c_uint,
    pub /: *mut *mut long unsigned int dev_state; / defined at enum au0828_dev_state,
    pub stream_state: au0828_stream_state,
    pub open: wait_queue_head_t,
    pub lock: mutex,
// Isoc control struct
    pub vidq: au0828_dmaqueue,
    pub vbiq: au0828_dmaqueue,
    pub isoc_ctl: au0828_usb_isoc_ctl,
    pub slock: spinlock_t,
// usb transfer
    pub /: *mut *mut int alt; / alternate,
    pub /: *mut *mut int max_pkt_size; / max packet size of isoc transaction,
    pub /: *mut *mut int num_alt; / Number of alternative settings,
    pub /: *mut *mut *mut unsigned int alt_max_pkt_size; / array of wMaxPacketSize,
    pub /: *mut *mut *mut urb urb[AU0828_MAX_ISO_BUFS]; / urb for isoc transfers,
    pub isoc: *mut *mut *mut char transfer_buffer[AU0828_MAX_ISO_BUFS];/ transfer buffers for,
// DVB USB / URB Related
    pub need_urb_start: bool urb_streaming,,
    pub urbs: [*mut urb; URB_COUNT],
// Preallocated transfer digital transfer buffers
    pub dig_transfer_buffer: [*mut c_char; URB_COUNT],
    pub media_dev: *mut media_device,
    pub vbi_pad: media_pad video_pad,,
    pub decoder: *mut media_entity,
    pub input_ent: [media_entity; AU0828_MAX_INPUT],
    pub input_pad: [media_pad; AU0828_MAX_INPUT],
    pub entity_notify: media_entity_notify,
    pub tuner: *mut media_entity,
    pub active_link: *mut media_link,
    pub active_source: *mut media_entity,
    pub active_sink: *mut media_entity,
    pub active_link_owner: *mut media_entity,
    pub active_link_user: *mut media_entity,
    pub active_link_user_pipe: *mut media_pipeline,
    pub active_link_shared: bool,

}

// -----------------------------------------------------------

// -----------------------------------------------------------
// au0828-core.c
extern "C" {
    pub fn au0828_read(dev: *mut au0828_dev, reg: u16) -> u32;
}
extern "C" {
    pub fn au0828_write(dev: *mut au0828_dev, reg: u16, val: u32) -> u32;
}
extern "C" {
    pub fn au0828_usb_release(dev: *mut au0828_dev);
}
// -----------------------------------------------------------
// au0828-cards.c
extern "C" {
    pub fn au0828_gpio_setup(dev: *mut au0828_dev);
}
extern "C" {
    pub fn au0828_card_setup(dev: *mut au0828_dev);
}
// -----------------------------------------------------------
// au0828-i2c.c
extern "C" {
    pub fn au0828_i2c_register(dev: *mut au0828_dev) -> c_int;
}
extern "C" {
    pub fn au0828_i2c_unregister(dev: *mut au0828_dev) -> c_int;
}
// -----------------------------------------------------------
// au0828-video.c
extern "C" {
    pub fn au0828_stop_vbi_streaming(vq: *mut vb2_queue);
}

extern "C" {
    pub fn au0828_analog_unregister(dev: *mut au0828_dev) -> c_int;
}
extern "C" {
    pub fn au0828_usb_v4l2_media_release(dev: *mut au0828_dev);
}
extern "C" {
    pub fn au0828_v4l2_suspend(dev: *mut au0828_dev);
}
extern "C" {
    pub fn au0828_v4l2_resume(dev: *mut au0828_dev);
}

// -----------------------------------------------------------
// au0828-dvb.c
extern "C" {
    pub fn au0828_dvb_register(dev: *mut au0828_dev) -> c_int;
}
extern "C" {
    pub fn au0828_dvb_unregister(dev: *mut au0828_dev);
}
extern "C" {
    pub fn au0828_dvb_suspend(dev: *mut au0828_dev);
}
extern "C" {
    pub fn au0828_dvb_resume(dev: *mut au0828_dev);
}
// au0828-vbi.c

// au0828-input.c

extern "C" {
    pub fn au0828_rc_register(dev: *mut au0828_dev) -> c_int;
}
extern "C" {
    pub fn au0828_rc_unregister(dev: *mut au0828_dev);
}
extern "C" {
    pub fn au0828_rc_suspend(dev: *mut au0828_dev) -> c_int;
}
extern "C" {
    pub fn au0828_rc_resume(dev: *mut au0828_dev) -> c_int;
}

