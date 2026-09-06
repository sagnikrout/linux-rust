//! Automatically rewritten from C Header to Rust Module
//! Source: include/video/udlfb.h
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
//
// TODO: Propose standard fb.h ioctl for reporting damage,
// using _IOWR() and one of the existing area structs from fb.h
// Consider these ioctls deprecated, but they're still used by the
// DisplayLink X server as yet - need both to be modified in tandem
// when new ioctl(s) are ready.
//
pub const DLFB_IOCTL_RETURN_EDID: c_uint = 0xAD;
pub const DLFB_IOCTL_REPORT_DAMAGE: c_uint = 0xAA;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dloarea {
    pub y: int x,,
    pub h: int w,,
    pub y2: int x2,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct urb_node {
    pub entry: list_head,
    pub dlfb: *mut dlfb_data,
    pub urb: *mut urb,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct urb_list {
    pub list: list_head,
    pub lock: spinlock_t,
    pub limit_sem: semaphore,
    pub available: c_int,
    pub count: c_int,
    pub size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlfb_data {
    pub udev: *mut usb_device,
    pub info: *mut fb_info,
    pub urbs: urb_list,
    pub backing_buffer: *mut c_char,
    pub fb_count: c_int,
    pub /: *mut *mut bool virtualized; / true when physical usb device not present,
    pub /: *mut *mut atomic_t usb_active; / 0 = update virtual buffer, but no usb traffic,
    pub /: *mut *mut atomic_t lost_pixels; / 1 = a render op failed. Need screen refresh,
    pub /: *mut *mut *mut char edid; / null until we read edid from hw or get from sysfs,
    pub edid_size: usize,
    pub sku_pixel_limit: c_int,
    pub base16: c_int,
    pub base8: c_int,
    pub pseudo_palette: [u32; 256],
    pub /: *mut *mut int blank_mode; /one of FB_BLANK_,
    pub render_mutex: mutex,
    pub damage_x: c_int,
    pub damage_y: c_int,
    pub damage_x2: c_int,
    pub damage_y2: c_int,
    pub damage_lock: spinlock_t,
    pub damage_work: work_struct,
    pub ops: fb_ops,
    pub mmap_count: core::sync::atomic::AtomicI32,
// blit-only rendering path metrics, exposed through sysfs
    pub /: *mut *mut atomic_t bytes_rendered; / raw pixel-bytes driver asked to render,
    pub /: *mut *mut atomic_t bytes_identical; / saved effort with backbuffer comparison,
    pub /: *mut *mut atomic_t bytes_sent; / to usb, after compression including overhead,
    pub /: *mut *mut atomic_t cpu_kcycles_used; / transpired during pixel processing,
    pub current_mode: fb_var_screeninfo,
    pub deferred_free: list_head,
}

pub const NR_USB_REQUEST_I2C_SUB_IO: c_uint = 0x02;
pub const NR_USB_REQUEST_CHANNEL: c_uint = 0x12;
// -BULK_SIZE as per usb-skeleton. Can we get full page and avoid overhead?
pub const BULK_SIZE: c_int = 512;

pub const MAX_VENDOR_DESCRIPTOR_SIZE: c_int = 256;

pub const BPP: c_int = 2;
pub const MAX_CMD_PIXELS: c_int = 255;
pub const RLX_HEADER_BYTES: c_int = 7;
pub const MIN_RLX_PIX_BYTES: c_int = 4;

pub const RLE_HEADER_BYTES: c_int = 6;
pub const MIN_RLE_PIX_BYTES: c_int = 3;

pub const RAW_HEADER_BYTES: c_int = 6;
pub const MIN_RAW_PIX_BYTES: c_int = 2;

// remove these once align.h patch is taken into kernel

