//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/screen_info.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

//
// These are set up by the setup-routine at boot-time:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct screen_info {
    pub /: *mut *mut __u8 orig_x; / 0x00,
    pub /: *mut *mut __u8 orig_y; / 0x01,
    pub /: *mut *mut __u16 ext_mem_k; / 0x02,
    pub /: *mut *mut __u16 orig_video_page; / 0x04,
    pub /: *mut *mut __u8 orig_video_mode; / 0x06,
    pub /: *mut *mut __u8 orig_video_cols; / 0x07,
    pub /: *mut *mut __u8 flags; / 0x08,
    pub /: *mut *mut __u8 unused2; / 0x09,
    pub /: *mut *mut __u16 orig_video_ega_bx;/ 0x0a,
    pub /: *mut *mut __u16 unused3; / 0x0c,
    pub /: *mut *mut __u8 orig_video_lines; / 0x0e,
    pub /: *mut *mut __u8 orig_video_isVGA; / 0x0f,
    pub /: *mut *mut __u16 orig_video_points;/ 0x10,
// VESA graphic mode -- linear frame buffer
    pub /: *mut *mut __u16 lfb_width; / 0x12,
    pub /: *mut *mut __u16 lfb_height; / 0x14,
    pub /: *mut *mut __u16 lfb_depth; / 0x16,
    pub /: *mut *mut __u32 lfb_base; / 0x18,
    pub /: *mut *mut __u32 lfb_size; / 0x1c,
    pub /: *mut *mut __u16 cl_magic, cl_offset; / 0x20,
    pub /: *mut *mut __u16 lfb_linelength; / 0x24,
    pub /: *mut *mut __u8 red_size; / 0x26,
    pub /: *mut *mut __u8 red_pos; / 0x27,
    pub /: *mut *mut __u8 green_size; / 0x28,
    pub /: *mut *mut __u8 green_pos; / 0x29,
    pub /: *mut *mut __u8 blue_size; / 0x2a,
    pub /: *mut *mut __u8 blue_pos; / 0x2b,
    pub /: *mut *mut __u8 rsvd_size; / 0x2c,
    pub /: *mut *mut __u8 rsvd_pos; / 0x2d,
    pub /: *mut *mut __u16 vesapm_seg; / 0x2e,
    pub /: *mut *mut __u16 vesapm_off; / 0x30,
    pub /: *mut *mut __u16 pages; / 0x32,
    pub /: *mut *mut __u16 vesa_attributes; / 0x34,
    pub /: *mut *mut __u32 capabilities; / 0x36,
    pub /: *mut *mut __u32 ext_lfb_base; / 0x3a,
    pub /: *mut *mut __u8 _reserved[2]; / 0x3e,
    pub __attribute__((packed)): },
pub const VIDEO_TYPE_MDA: c_uint = 0x10	/* Monochrome Text Display	*/;
pub const VIDEO_TYPE_CGA: c_uint = 0x11	/* CGA Display 			*/;
pub const VIDEO_TYPE_EGAM: c_uint = 0x20	/* EGA/VGA in Monochrome Mode	*/;
pub const VIDEO_TYPE_EGAC: c_uint = 0x21	/* EGA in Color Mode		*/;
pub const VIDEO_TYPE_VGAC: c_uint = 0x22	/* VGA+ in Color Mode		*/;
pub const VIDEO_TYPE_VLFB: c_uint = 0x23	/* VESA VGA in graphic mode	*/;
pub const VIDEO_TYPE_PICA_S3: c_uint = 0x30	/* ACER PICA-61 local S3 video	*/;
pub const VIDEO_TYPE_MIPS_G364: c_uint = 0x31    /* MIPS Magnum 4000 G364 video  */;
pub const VIDEO_TYPE_SGI: c_uint = 0x33    /* Various SGI graphics hardware */;
pub const VIDEO_TYPE_TGAC: c_uint = 0x40	/* DEC TGA */;
pub const VIDEO_TYPE_SUN: c_uint = 0x50    /* Sun frame buffer. */;
pub const VIDEO_TYPE_SUNPCI: c_uint = 0x51    /* Sun PCI based frame buffer. */;
pub const VIDEO_TYPE_PMAC: c_uint = 0x60	/* PowerMacintosh frame buffer. */;
pub const VIDEO_TYPE_EFI: c_uint = 0x70	/* EFI graphic mode		*/;

