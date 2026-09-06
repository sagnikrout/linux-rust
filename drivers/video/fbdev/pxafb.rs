//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/pxafb.h
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
// linux/drivers/video/pxafb.h
// -- Intel PXA250/210 LCD Controller Frame Buffer Device
//
// Copyright (C) 1999 Eric A. Thomas.
// Copyright (C) 2004 Jean-Frederic Clere.
// Copyright (C) 2004 Ian Campbell.
// Copyright (C) 2004 Jeff Lackey.
// Based on sa1100fb.c Copyright (C) 1999 Eric A. Thomas
// which in turn is
// Based on acornfb.c Copyright (C) Russell King.
//
// 2001-08-03: Cliff Brake <cbrake@acclent.com>
// - ported SA1100 code to PXA
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of this archive
// for more details.
//
// PXA LCD DMA descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxafb_dma_descriptor {
    pub fdadr: c_uint,
    pub fsadr: c_uint,
    pub fidr: c_uint,
    pub ldcmd: c_uint,
}

// maximum palette size - 256 entries, each 4 bytes long

// NOTE: the palette and frame dma descriptors are doubled to allow
// the 2nd set for branch settings (FBRx)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxafb_dma_buff {
    pub PALETTE_SIZE]: *mut *mut unsigned char palette[PAL_MAX,
    pub cmd_buff: [u16; CMD_BUFF_SIZE],
    pub 2]: *mut *mut pxafb_dma_descriptor pal_desc[PAL_MAX,
    pub 2]: *mut *mut pxafb_dma_descriptor dma_desc[DMA_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxafb_layer_ops {
    pub ): *mut *mut void (enable)(struct pxafb_layer,
    pub ): *mut *mut void (disable)(struct pxafb_layer,
    pub ): *mut *mut void (setup)(struct pxafb_layer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxafb_layer {
    pub fb: fb_info,
    pub id: c_int,
    pub registered: c_int,
    pub usage: u32,
    pub control: [u32; 2],
    pub ops: *mut pxafb_layer_ops,
    pub video_mem: *mut void __iomem,
    pub video_mem_phys: c_ulong,
    pub video_mem_size: usize,
    pub branch_done: completion,
    pub fbi: *mut pxafb_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxafb_info {
    pub fb: fb_info,
    pub dev: *mut device,
    pub clk: *mut clk,
    pub mmio_base: *mut void __iomem,
    pub dma_buff: *mut pxafb_dma_buff,
    pub dma_buff_size: usize,
    pub dma_buff_phys: dma_addr_t,
    pub 2]: *mut *mut dma_addr_t fdadr[DMA_MAX,
    pub /: *mut *mut *mut void __iomem video_mem; / virtual address of frame buffer,
    pub /: *mut *mut unsigned long video_mem_phys; / physical address of frame buffer,
    pub /: *mut *mut size_t video_mem_size; / size of the frame buffer,
    pub /: *mut *mut *mut u16  palette_cpu; / virtual address of palette memory,
    pub palette_size: u_int,
    pub lccr0: u_int,
    pub lccr3: u_int,
    pub lccr4: u_int,
    pub reg_lccr0: u_int,
    pub reg_lccr1: u_int,
    pub reg_lccr2: u_int,
    pub reg_lccr3: u_int,
    pub reg_lccr4: u_int,
    pub reg_cmdcr: u_int,
    pub hsync_time: c_ulong,
    pub state: volatile u_char,
    pub task_state: volatile u_char,
    pub ctrlr_lock: mutex,
    pub ctrlr_wait: wait_queue_head_t,
    pub task: work_struct,
    pub disable_done: completion,

    pub smart_cmds: *mut u16,
    pub n_smart_cmds: usize,
    pub command_done: completion,
    pub refresh_done: completion,
    pub smart_thread: *mut task_struct,
    pub overlay: [pxafb_layer; 2],
    pub freq_transition: notifier_block,

    pub lcd_supply: *mut regulator,
    pub lcd_supply_enabled: bool,
    pub ): *mut *mut void (lcd_power)(int, struct fb_var_screeninfo,
    pub (*backlight_power)(int): *mut c_void,
    pub inf: *mut pxafb_mach_info,
}

//
// These are the actions for set_ctrlr_state
//

//
// Minimum X and Y resolutions
//
pub const MIN_XRES: c_int = 64;
pub const MIN_YRES: c_int = 64;
// maximum X and Y resolutions - note these are limits from the register
// bits length instead of the real ones
//
pub const MAX_XRES: c_int = 1024;
pub const MAX_YRES: c_int = 1024;
