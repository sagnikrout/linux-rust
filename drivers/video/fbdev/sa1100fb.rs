//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/sa1100fb.h
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
// linux/drivers/video/sa1100fb.h
// -- StrongARM 1100 LCD Controller Frame Buffer Device
//
// Copyright (C) 1999 Eric A. Thomas
// Based on acornfb.c Copyright (C) Russell King.
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of this archive
// for more details.
//
pub const LCCR0: c_uint = 0x0000          /* LCD Control Reg. 0 */;
pub const LCSR: c_uint = 0x0004          /* LCD Status Reg. */;
pub const DBAR1: c_uint = 0x0010          /* LCD DMA Base Address Reg. channel 1 */;
pub const DCAR1: c_uint = 0x0014          /* LCD DMA Current Address Reg. channel 1 */;
pub const DBAR2: c_uint = 0x0018          /* LCD DMA Base Address Reg.  channel 2 */;
pub const DCAR2: c_uint = 0x001C          /* LCD DMA Current Address Reg. channel 2 */;
pub const LCCR1: c_uint = 0x0020          /* LCD Control Reg. 1 */;
pub const LCCR2: c_uint = 0x0024          /* LCD Control Reg. 2 */;
pub const LCCR3: c_uint = 0x0028          /* LCD Control Reg. 3 */;
// Shadows for LCD controller registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sa1100fb_lcd_reg {
    pub lccr0: c_ulong,
    pub lccr1: c_ulong,
    pub lccr2: c_ulong,
    pub lccr3: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sa1100fb_info {
    pub fb: fb_info,
    pub dev: *mut device,
    pub rgb: [*const sa1100fb_rgb; NR_RGB],
    pub base: *mut void __iomem,
    pub shannon_lcden: *mut gpio_desc,
//
// These are the addresses we mapped
// the framebuffer memory region to.
//
    pub map_dma: dma_addr_t,
    pub map_cpu: *mut *mut u_char,
    pub map_size: u_int,
    pub screen_cpu: *mut *mut u_char,
    pub screen_dma: dma_addr_t,
    pub palette_cpu: *mut *mut u16,
    pub palette_dma: dma_addr_t,
    pub palette_size: u_int,
    pub dbar1: dma_addr_t,
    pub dbar2: dma_addr_t,
    pub reg_lccr0: u_int,
    pub reg_lccr1: u_int,
    pub reg_lccr2: u_int,
    pub reg_lccr3: u_int,
    pub state: volatile u_char,
    pub task_state: volatile u_char,
    pub ctrlr_lock: mutex,
    pub ctrlr_wait: wait_queue_head_t,
    pub task: work_struct,

    pub freq_transition: notifier_block,

    pub inf: *const sa1100fb_mach_info,
    pub clk: *mut clk,
    pub pseudo_palette: [u32; 16],
}

//
// These are the actions for set_ctrlr_state
//

//
// Minimum X and Y resolutions
//
pub const MIN_XRES: c_int = 64;
pub const MIN_YRES: c_int = 64;
