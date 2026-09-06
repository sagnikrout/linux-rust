//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/drv-intf/saa7146_vv.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7146_video_dma {
    pub base_odd: u32,
    pub base_even: u32,
    pub prot_addr: u32,
    pub pitch: u32,
    pub base_page: u32,
    pub num_line_byte: u32,
}

pub const FORMAT_BYTE_SWAP: c_uint = 0x1;
pub const FORMAT_IS_PLANAR: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7146_format {
    pub pixelformat: u32,
    pub trans: u32,
    pub depth: u8,
    pub flags: u8,
    pub swap: u8,
}

// buffer for one video/vbi frame
#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7146_buf {
// common v4l buffer stuff -- must be first
    pub vb: vb2_v4l2_buffer,
    pub list: list_head,
// saa7146 specific
    pub next): *mut saa7146_buf,
// page tables
    pub pt: [saa7146_pgtable; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7146_dmaqueue {
    pub dev: *mut saa7146_dev,
    pub curr: *mut saa7146_buf,
    pub queue: list_head,
    pub timeout: timer_list,
    pub q: vb2_queue,
}

// vbi capture
// vbi workaround interrupt queue
// video capture
// common: fixme? shouldn't this be in saa7146_fh?
// flags
pub const SAA7146_USE_PORT_B_FOR_VBI: c_uint = 0x2     /* use input port b for vbi hardware bug workaround */;
// information about the video capabilities of the device
// additionally supported transmission standards
// the extension can override this
// pointer to the saa7146 core ops
#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7146_use_ops {
    pub ): *mut *mut *mut void (init)(struct saa7146_dev , struct saa7146_vv,
    pub status): *mut *mut *mut void (irq_done)(struct saa7146_dev , unsigned long,
}

// from saa7146_fops.c
extern "C" {
    pub fn saa7146_register_device(vid: *mut video_device, dev: *mut saa7146_dev, name: *mut c_char, type: c_int) -> c_int;
}
extern "C" {
    pub fn saa7146_unregister_device(vid: *mut video_device, dev: *mut saa7146_dev) -> c_int;
}
extern "C" {
    pub fn saa7146_buffer_finish(dev: *mut saa7146_dev, q: *mut saa7146_dmaqueue, state: c_int);
}
extern "C" {
    pub fn saa7146_buffer_next(dev: *mut saa7146_dev, q: *mut saa7146_dmaqueue, vbi: c_int);
}
extern "C" {
    pub fn saa7146_buffer_queue(dev: *mut saa7146_dev, q: *mut saa7146_dmaqueue, buf: *mut saa7146_buf) -> c_int;
}
extern "C" {
    pub fn saa7146_buffer_timeout(t: *mut timer_list);
}
extern "C" {
    pub fn saa7146_vv_init(dev: *mut *mut saa7146_dev, ext_vv: *mut saa7146_ext_vv) -> c_int;
}
extern "C" {
    pub fn saa7146_vv_release(dev: *mut *mut saa7146_dev) -> c_int;
}
// from saa7146_hlp.c
extern "C" {
    pub fn saa7146_set_capture(dev: *mut saa7146_dev, buf: *mut saa7146_buf, next: *mut saa7146_buf);
}
extern "C" {
    pub fn saa7146_write_out_dma(dev: *mut *mut saa7146_dev, which: c_int, vdma: *mut *mut saa7146_video_dma);
}
extern "C" {
    pub fn saa7146_set_hps_source_and_sync(saa: *mut saa7146_dev, source: c_int, sync: c_int);
}
extern "C" {
    pub fn saa7146_set_gpio(saa: *mut saa7146_dev, pin: u8, data: u8);
}
// from saa7146_video.c
extern "C" {
    pub fn saa7146_video_do_ioctl(file: *mut file, cmd: c_uint, arg: *mut c_void) -> c_long;
}
extern "C" {
    pub fn saa7146_s_ctrl(ctrl: *mut v4l2_ctrl) -> c_int;
}
// from saa7146_vbi.c
// resource management functions
extern "C" {
    pub fn saa7146_res_get(dev: *mut saa7146_dev, bit: c_uint) -> c_int;
}
extern "C" {
    pub fn saa7146_res_free(dev: *mut saa7146_dev, bits: c_uint);
}
pub const RESOURCE_DMA1_HPS: c_uint = 0x1;
pub const RESOURCE_DMA2_CLP: c_uint = 0x2;
pub const RESOURCE_DMA3_BRS: c_uint = 0x4;
// saa7146 source inputs
pub const SAA7146_HPS_SOURCE_PORT_A: c_uint = 0x00;
pub const SAA7146_HPS_SOURCE_PORT_B: c_uint = 0x01;
pub const SAA7146_HPS_SOURCE_YPB_CPA: c_uint = 0x02;
pub const SAA7146_HPS_SOURCE_YPA_CPB: c_uint = 0x03;
// sync inputs
pub const SAA7146_HPS_SYNC_PORT_A: c_uint = 0x00;
pub const SAA7146_HPS_SYNC_PORT_B: c_uint = 0x01;
// some memory sizes
// max. 16 clipping rectangles

// some defines for the various clipping-modes
pub const SAA7146_CLIPPING_RECT: c_uint = 0x4;
pub const SAA7146_CLIPPING_RECT_INVERTED: c_uint = 0x5;
pub const SAA7146_CLIPPING_MASK: c_uint = 0x6;
pub const SAA7146_CLIPPING_MASK_INVERTED: c_uint = 0x7;
// output formats: each entry holds four information
pub const RGB08_COMPOSED: c_uint = 0x0217 /* composed is used in the sense of "not-planar" */;
// this means: planar?=0, yuv2rgb-conversation-mode=2, dither=yes(=1), format-mode = 7
pub const RGB15_COMPOSED: c_uint = 0x0213;
pub const RGB16_COMPOSED: c_uint = 0x0210;
pub const RGB24_COMPOSED: c_uint = 0x0201;
pub const RGB32_COMPOSED: c_uint = 0x0202;
pub const Y8: c_uint = 0x0006;
pub const YUV411_COMPOSED: c_uint = 0x0003;
pub const YUV422_COMPOSED: c_uint = 0x0000;
// this means: planar?=1, yuv2rgb-conversion-mode=0, dither=no(=0), format-mode = b
pub const YUV411_DECOMPOSED: c_uint = 0x100b;
pub const YUV422_DECOMPOSED: c_uint = 0x1009;
pub const YUV420_DECOMPOSED: c_uint = 0x100a;

// misc defines

