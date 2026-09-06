//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/tw68/tw68.h
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
// tw68 driver common header file
//
// Much of this code is derived from the cx88 and sa7134 drivers, which
// were in turn derived from the bt87x driver.  The original work was by
// Gerd Knorr; more recently the code was enhanced by Mauro Carvalho Chehab,
// Hans Verkuil, Andy Walls and many others.  Their work is gratefully
// acknowledged.  Full credit goes to them - any problems within this code
// are mine.
//
// Copyright (C) 2009  William M. Brack
//
// Refactored and updated to the latest v4l core frameworks:
//
// Copyright (C) 2014 Hans Verkuil <hverkuil@kernel.org>
//

// TW6800 chips have trouble with these, so we don't set them for that chip

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tw68_decoder_type {
    TW6800,
    TW6801,
    TW6804,
    TWXXXX,
}

// -----------------------------------------------------------
// static data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tw68_tvnorm {
    pub name: *mut c_char,
    pub id: v4l2_std_id,
// video decoder
    pub sync_control: u32,
    pub luma_control: u32,
    pub chroma_ctrl1: u32,
    pub chroma_gain: u32,
    pub chroma_ctrl2: u32,
    pub vgate_misc: u32,
// video scaler
    pub h_delay: u32,
    pub /: *mut *mut u32 h_delay0; / for TW6800,
    pub h_start: u32,
    pub h_stop: u32,
    pub v_delay: u32,
    pub video_v_start: u32,
    pub video_v_stop: u32,
    pub vbi_v_start_0: u32,
    pub vbi_v_stop_0: u32,
    pub vbi_v_start_1: u32,
// Techwell specific
    pub format: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tw68_format {
    pub fourcc: u32,
    pub depth: u32,
    pub twformat: u32,
}

// -----------------------------------------------------------
// card configuration

pub const TW68_BOARD_UNKNOWN: c_int = 0;
pub const TW68_BOARD_GENERIC_6802: c_int = 1;
pub const TW68_MAXBOARDS: c_int = 16;
pub const TW68_INPUT_MAX: c_int = 4;
// -----------------------------------------------------------
// device / file handle status

// buffer for one video/vbi/ts frame
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tw68_buf {
    pub vb: vb2_v4l2_buffer,
    pub list: list_head,
    pub size: c_uint,
    pub cpu: *mut __le32,
    pub jmp: *mut __le32,
    pub dma: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tw68_fmt {
    pub name: *mut c_char,
    pub /: *mut *mut u32 fourcc; / v4l2 format id,
    pub depth: c_int,
    pub flags: c_int,
    pub twformat: u32,
}

// global device status
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tw68_dev {
    pub lock: mutex,
    pub slock: spinlock_t,
    pub instance: u16,
    pub v4l2_dev: v4l2_device,
// various device info
    pub vdecoder: tw68_decoder_type,
    pub vdev: video_device,
    pub hdl: v4l2_ctrl_handler,
// pci i/o
    pub name: *mut c_char,
    pub pci: *mut pci_dev,
    pub pci_lat: unsigned char pci_rev,,
    pub lmmio: *mut u32 __iomem,
    pub bmmio: *mut u8 __iomem,
    pub pci_irqmask: u32,
// The irq mask to be used will depend upon the chip type
    pub board_virqmask: u32,
// video capture
    pub fmt: *const tw68_format,
    pub height: unsigned width,,
    pub seqnr: unsigned,
    pub field: unsigned,
    pub vidq: vb2_queue,
    pub active: list_head,
// various v4l controls
    pub /: *const *const *const tw68_tvnorm tvnorm; / video,
    pub input: c_int,
}

// -----------------------------------------------------------

// -----------------------------------------------------------
// tw68-video.c
extern "C" {
    pub fn tw68_set_tvnorm_hw(dev: *mut tw68_dev);
}
extern "C" {
    pub fn tw68_video_init1(dev: *mut tw68_dev) -> c_int;
}
extern "C" {
    pub fn tw68_video_init2(dev: *mut tw68_dev, video_nr: c_int) -> c_int;
}
extern "C" {
    pub fn tw68_irq_video_done(dev: *mut tw68_dev, status: c_ulong);
}
extern "C" {
    pub fn tw68_video_start_dma(dev: *mut tw68_dev, buf: *mut tw68_buf) -> c_int;
}
// -----------------------------------------------------------
// tw68-risc.c
