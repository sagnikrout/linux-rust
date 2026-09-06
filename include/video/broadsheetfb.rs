//! Automatically rewritten from C Header to Rust Module
//! Source: include/video/broadsheetfb.h
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
// broadsheetfb.h - definitions for the broadsheet framebuffer driver
//
// Copyright (C) 2008 by Jaya Kumar
//
// This file is subject to the terms and conditions of the GNU General Public
// License. See the file COPYING in the main directory of this archive for
// more details.
//
// Broadsheet command defines
pub const BS_CMD_INIT_SYS_RUN: c_uint = 0x06;
pub const BS_CMD_INIT_DSPE_CFG: c_uint = 0x09;
pub const BS_CMD_INIT_DSPE_TMG: c_uint = 0x0A;
pub const BS_CMD_INIT_ROTMODE: c_uint = 0x0B;
pub const BS_CMD_RD_REG: c_uint = 0x10;
pub const BS_CMD_WR_REG: c_uint = 0x11;
pub const BS_CMD_LD_IMG: c_uint = 0x20;
pub const BS_CMD_LD_IMG_AREA: c_uint = 0x22;
pub const BS_CMD_LD_IMG_END: c_uint = 0x23;
pub const BS_CMD_WAIT_DSPE_TRG: c_uint = 0x28;
pub const BS_CMD_WAIT_DSPE_FREND: c_uint = 0x29;
pub const BS_CMD_RD_WFM_INFO: c_uint = 0x30;
pub const BS_CMD_UPD_INIT: c_uint = 0x32;
pub const BS_CMD_UPD_FULL: c_uint = 0x33;
pub const BS_CMD_UPD_GDRV_CLR: c_uint = 0x37;
// Broadsheet register interface defines
pub const BS_REG_REV: c_uint = 0x00;
pub const BS_REG_PRC: c_uint = 0x02;
// Broadsheet pin interface specific defines
pub const BS_CS: c_uint = 0x01;
pub const BS_DC: c_uint = 0x02;
pub const BS_WR: c_uint = 0x03;
// Broadsheet IO interface specific defines
pub const BS_MMIO_CMD: c_uint = 0x01;
pub const BS_MMIO_DATA: c_uint = 0x02;
// struct used by broadsheet. board specific stuff comes from *board
#[repr(C)]
#[derive(Copy, Clone)]
pub struct broadsheetfb_par {
    pub info: *mut fb_info,
    pub board: *mut broadsheet_board,
    pub val): *mut *mut *mut void (write_reg)(struct broadsheetfb_par , u16 reg, u16,
    pub reg): *mut *mut *mut u16 (read_reg)(struct broadsheetfb_par , u16,
    pub waitq: wait_queue_head_t,
    pub panel_index: c_int,
    pub io_lock: mutex,
}

// board specific routines
#[repr(C)]
#[derive(Copy, Clone)]
pub struct broadsheet_board {
    pub owner: *mut module,
    pub ): *mut *mut int (init)(struct broadsheetfb_par,
    pub ): *mut *mut int (wait_for_rdy)(struct broadsheetfb_par,
    pub ): *mut *mut void (cleanup)(struct broadsheetfb_par,
    pub (*get_panel_type)(void): *mut c_int,
    pub ): *mut *mut int (setup_irq)(struct fb_info,
// Functions for boards that use GPIO
    pub u8): *mut *mut *mut void (set_ctl)(struct broadsheetfb_par , unsigned char,,
    pub u16): *mut *mut *mut void (set_hdb)(struct broadsheetfb_par ,,
    pub ): *mut *mut u16 (get_hdb)(struct broadsheetfb_par,
// Functions for boards that have specialized MMIO
    pub u16): *mut *mut *mut void (mmio_write)(struct broadsheetfb_par , int type,,
    pub ): *mut *mut u16 (mmio_read)(struct broadsheetfb_par,
}
