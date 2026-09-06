//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/gspca/gspca.h
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

// GSPCA debug codes
pub const D_PROBE: c_int = 1;
pub const D_CONF: c_int = 2;
pub const D_STREAM: c_int = 3;
pub const D_FRAM: c_int = 4;
pub const D_PACK: c_int = 5;
pub const D_USBI: c_int = 6;
pub const D_USBO: c_int = 7;

// image transfers

// used to list framerates supported by a camera mode (resolution)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct framerates {
    pub rates: *const u8,
    pub nrates: c_int,
}

// device information - set at probe time
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cam {
    pub /: *const *const *const v4l2_pix_format cam_mode; / size nmodes,
    pub nmodes,: *const *const *const framerates mode_framerates; / must have size,
// just like cam_mode
    pub /: *mut *mut u32 bulk_size; / buffer size when image transfer by bulk,
    pub /: *mut *mut u32 input_flags; / value for ENUM_INPUT status flags,
    pub /: *mut *mut u8 nmodes; / size of cam_mode,
    pub /: *mut *mut u8 no_urb_create; / don't create transfer URBs,
    pub mode: *mut *mut u8 bulk_nurbs; / number of URBs in bulk,
// - cannot be > MAX_NURBS
// - when 0 and bulk_size != 0 means
// 1 URB and submit done by subdriver
    pub /: *mut *mut u8 bulk; / image transfer by 0:isoc / 1:bulk,
    pub message: *mut *mut u8 npkt; / number of packets in an ISOC,
// 0 is the default value: 32 packets
    pub calc.: *mut *mut u8 needs_full_bandwidth;/ Set this flag to notify the bandwidth,
// code that the cam fills all image buffers to
// the max, even when using compression.
}

// subdriver operations
extern "C" {
    pub fn int(: *mut *mut cam_op) (struct gspca_dev) -> typedef;
}
extern "C" {
    pub fn void(: *mut *mut cam_v_op) (struct gspca_dev) -> typedef;
}
extern "C" {
    pub fn int(: *mut *mut cam_cf_op) (struct gspca_dev, : *const usb_device_id) -> typedef;
}
// subdriver description
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sd_desc {
// information
    pub /: *const *const *const char name; / sub-driver name,
// mandatory operations
    pub /: *mut *mut cam_cf_op config; / called on probe,
    pub /: *mut *mut cam_op init; / called on probe and resume,
    pub /: *mut *mut cam_op init_controls; / called on probe,
    pub /: *mut *mut cam_v_op probe_error; / called if probe failed, do cleanup here,
    pub /: *mut *mut cam_op start; / called on stream on after URBs creation,
    pub pkt_scan: cam_pkt_op,
// optional operations
    pub /: *mut *mut cam_op isoc_init; / called on stream on before getting the EP,
    pub /: *mut *mut cam_op isoc_nego; / called when URB submit failed with NOSPC,
    pub /: *mut *mut cam_v_op stopN; / called on stream off - main alt,
    pub /: *mut *mut cam_v_op stop0; / called on stream off & disconnect - alt 0,
    pub /: *mut *mut cam_v_op dq_callback; / called when a frame has been dequeued,
    pub get_jcomp: cam_get_jpg_op,
    pub set_jcomp: cam_set_jpg_op,
    pub get_streamparm: cam_streamparm_op,
    pub set_streamparm: cam_streamparm_op,
    pub try_fmt: cam_format_op,
    pub enum_framesizes: cam_frmsize_op,

    pub set_register: cam_set_reg_op,
    pub get_register: cam_get_reg_op,
    pub get_chip_info: cam_chip_info_op,

    pub int_pkt_scan: cam_int_pkt_op,
// other_input makes the gspca core create gspca_dev->input even when
    pub other_input: u8,

}

// packet types when moving from iso buf to frame buf
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gspca_packet_type {
    DISCARD_PACKET,
    FIRST_PACKET,
    INTER_PACKET,
    LAST_PACKET
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gspca_buffer {
    pub vb: vb2_v4l2_buffer,
    pub list: list_head,
}

extern "C" {
    pub fn container_of(_arg: vb2, gspca_buffer: struct, _arg: vb.vb2_buf) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gspca_dev {
    pub /: *mut *mut video_device vdev; / !! must be the first item,
    pub /: *mut *mut *mut module module; / subdriver handling the device,
    pub v4l2_dev: v4l2_device,
    pub dev: *mut usb_device,

    pub input_dev: *mut input_dev,
    pub /: *mut *mut char phys[64]; / physical device path,

    pub /: *mut *mut cam cam; / device information,
    pub /: *const *const *const sd_desc sd_desc; / subdriver description,
    pub ctrl_handler: v4l2_ctrl_handler,
// autogain and exposure or gain control cluster, these are global as
    pub autogain: *mut v4l2_ctrl,
    pub exposure: *mut v4l2_ctrl,
    pub gain: *mut v4l2_ctrl,
    pub exp_too_high_cnt: int exp_too_low_cnt,,
}

pub const USB_BUF_SZ: c_int = 64;

// (*) These variables are proteced by both usb_lock and queue_lock,
extern "C" {
    pub fn gspca_disconnect(intf: *mut usb_interface);
}

extern "C" {
    pub fn gspca_suspend(intf: *mut usb_interface, message: pm_message_t) -> c_int;
}
extern "C" {
    pub fn gspca_resume(intf: *mut usb_interface) -> c_int;
}

