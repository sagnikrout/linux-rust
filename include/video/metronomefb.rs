//! Automatically rewritten from C Header to Rust Module
//! Source: include/video/metronomefb.h
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


//
// metronomefb.h - definitions for the metronome framebuffer driver
//
// Copyright (C) 2008 by Jaya Kumar
//
// This file is subject to the terms and conditions of the GNU General Public
// License. See the file COPYING in the main directory of this archive for
// more details.
//
// command structure used by metronome controller
#[repr(C)]
#[derive(Copy, Clone)]
pub struct metromem_cmd {
    pub opcode: u16,
    pub args: [u16; ((64-2)/2)],
    pub csum: u16,
}

// struct used by metronome. board specific stuff comes from *board
#[repr(C)]
#[derive(Copy, Clone)]
pub struct metronomefb_par {
    pub metromem_cmd: *mut metromem_cmd,
    pub metromem_wfm: *mut c_uchar,
    pub metromem_img: *mut c_uchar,
    pub metromem_img_csum: *mut u16,
    pub csum_table: *mut u16,
    pub metromem_dma: dma_addr_t,
    pub info: *mut fb_info,
    pub board: *mut metronome_board,
    pub waitq: wait_queue_head_t,
    pub frame_count: u8,
    pub extra_size: c_int,
    pub dt: c_int,
}

// board specific routines and data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct metronome_board {
    pub /: *mut *mut *mut module owner; / the platform device,
    pub int): *mut *mut *mut void (set_rst)(struct metronomefb_par ,,
    pub int): *mut *mut *mut void (set_stdby)(struct metronomefb_par ,,
    pub ): *mut *mut void (cleanup)(struct metronomefb_par,
    pub ): *mut *mut int (met_wait_event)(struct metronomefb_par,
    pub ): *mut *mut int (met_wait_event_intr)(struct metronomefb_par,
    pub ): *mut *mut int (setup_irq)(struct fb_info,
    pub ): *mut *mut int (setup_fb)(struct metronomefb_par,
    pub ): *mut *mut int (setup_io)(struct metronomefb_par,
    pub (*get_panel_type)(void): *mut c_int,
    pub metromem: *mut c_uchar,
    pub fw: c_int,
    pub fh: c_int,
    pub wfm_size: c_int,
    pub /: *mut *mut *mut fb_info host_fbinfo; / the host LCD controller's fbi,
}
