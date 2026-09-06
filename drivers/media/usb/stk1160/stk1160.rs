//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/stk1160/stk1160.h
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
// STK1160 driver
//
// Copyright (C) 2012 Ezequiel Garcia
// <elezegarcia--a.t--gmail.com>
//
// Based on Easycap driver by R.M. Thomas
// Copyright (C) 2010 R.M. Thomas
// <rmthomas--a.t--sciolus.org>
//

pub const STK1160_VERSION_NUM: c_uint = 0x000905;
// Decide on number of packets for each buffer
pub const STK1160_NUM_PACKETS: c_int = 64;
// Number of buffers for isoc transfers
pub const STK1160_NUM_BUFS: c_int = 16;
pub const STK1160_MIN_BUFS: c_int = 1;
// TODO: This endpoint address should be retrieved
pub const STK1160_EP_VIDEO: c_uint = 0x82;
pub const STK1160_EP_AUDIO: c_uint = 0x81;
// Max and min video buffers
pub const STK1160_MIN_VIDEO_BUFFERS: c_int = 8;
pub const STK1160_MAX_VIDEO_BUFFERS: c_int = 32;
pub const STK1160_MIN_PKT_SIZE: c_int = 3072;
pub const STK1160_MAX_INPUT: c_int = 4;
pub const STK1160_SVIDEO_INPUT: c_int = 4;
pub const STK1160_AC97_TIMEOUT: c_int = 50;
pub const STK1160_I2C_TIMEOUT: c_int = 100;
// TODO: Print helpers
// I could use dev_xxx, pr_xxx, v4l2_xxx or printk.
// However, there isn't a solid consensus on which
// new drivers should use.
//

// Buffer for one video frame
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stk1160_buffer {
// common v4l buffer stuff -- must be first
    pub vb: vb2_v4l2_buffer,
    pub list: list_head,
    pub mem: *mut c_void,
    pub /: *mut *mut unsigned int length; / buffer length,
    pub /: *mut *mut unsigned int bytesused; / bytes written,
    pub /: *mut *mut int odd; / current oddity,
//
// Since we interlace two fields per frame,
// this is different from bytesused.
//
    pub /: *mut *mut unsigned int pos; / current pos inside buffer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stk1160_urb {
    pub urb: *mut urb,
    pub transfer_buffer: *mut c_char,
    pub sgt: *mut sg_table,
    pub dev: *mut stk1160,
    pub dma: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stk1160_isoc_ctl {
// max packet size of isoc transaction
    pub max_pkt_size: c_int,
// number of allocated urbs
    pub num_bufs: c_int,
    pub urb_ctl: [stk1160_urb; STK1160_NUM_BUFS],
// current buffer
    pub buf: *mut stk1160_buffer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stk1160_fmt {
    pub /: *mut *mut u32 fourcc; / v4l2 format id,
    pub depth: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stk1160 {
    pub v4l2_dev: v4l2_device,
    pub vdev: video_device,
    pub ctrl_handler: v4l2_ctrl_handler,
    pub dev: *mut device,
    pub udev: *mut usb_device,
// saa7115 subdev
    pub sd_saa7115: *mut v4l2_subdev,
// isoc control struct
    pub avail_bufs: list_head,
// video capture
    pub vb_vidq: vb2_queue,
// max packet size of isoc transaction
    pub max_pkt_size: c_int,
// array of wMaxPacketSize
    pub alt_max_pkt_size: *mut c_uint,
// alternate
    pub alt: c_int,
// Number of alternative settings
    pub num_alt: c_int,
    pub isoc_ctl: stk1160_isoc_ctl,
// frame properties
    pub /: *mut *mut int width; / current frame width,
    pub /: *mut *mut int height; / current frame height,
    pub /: *mut *mut unsigned int ctl_input; / selected input,
    pub /: *mut *mut v4l2_std_id norm; / current norm,
    pub /: *mut *mut *mut stk1160_fmt fmt; / selected format,
    pub sequence: c_uint,
// i2c i/o
    pub i2c_adap: i2c_adapter,
    pub i2c_client: i2c_client,
    pub v4l_lock: mutex,
    pub vb_queue_lock: mutex,
    pub buf_lock: spinlock_t,
    pub /: *mut *mut *mut file fh_owner; / filehandle ownership,
// EXPERIMENTAL
    pub snd_card: *mut snd_card,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regval {
    pub reg: u16,
    pub val: u16,
}

// Provided by stk1160-v4l.c
extern "C" {
    pub fn stk1160_vb2_setup(dev: *mut stk1160) -> c_int;
}
extern "C" {
    pub fn stk1160_video_register(dev: *mut stk1160) -> c_int;
}
extern "C" {
    pub fn stk1160_video_unregister(dev: *mut stk1160);
}
extern "C" {
    pub fn stk1160_clear_queue(dev: *mut stk1160, vb2_state: vb2_buffer_state);
}
// Provided by stk1160-video.c
extern "C" {
    pub fn stk1160_alloc_isoc(dev: *mut stk1160) -> c_int;
}
extern "C" {
    pub fn stk1160_free_isoc(dev: *mut stk1160);
}
extern "C" {
    pub fn stk1160_cancel_isoc(dev: *mut stk1160);
}
extern "C" {
    pub fn stk1160_uninit_isoc(dev: *mut stk1160);
}
// Provided by stk1160-i2c.c
extern "C" {
    pub fn stk1160_i2c_register(dev: *mut stk1160) -> c_int;
}
extern "C" {
    pub fn stk1160_i2c_unregister(dev: *mut stk1160) -> c_int;
}
// Provided by stk1160-core.c
extern "C" {
    pub fn stk1160_read_reg(dev: *mut stk1160, reg: u16, value: *mut u8) -> c_int;
}
extern "C" {
    pub fn stk1160_write_reg(dev: *mut stk1160, reg: u16, value: u16) -> c_int;
}
extern "C" {
    pub fn stk1160_select_input(dev: *mut stk1160);
}
// Provided by stk1160-ac97.c
extern "C" {
    pub fn stk1160_ac97_setup(dev: *mut stk1160);
}
