//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/video-pxafb.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Support for the xscale frame buffer.
//
// Author:     Jean-Frederic Clere
// Created:    Sep 22, 2003
// Copyright:  jfclere@sinix.net
//

//
// Supported LCD connections
//
// bits 0 - 3: for LCD panel type:
//
// STN  - for passive matrix
// DSTN - for dual scan passive matrix
// TFT  - for active matrix
//
// bits 4 - 9 : for bus width
// bits 10-17 : for AC Bias Pin Frequency
// bit     18 : for output enable polarity
// bit     19 : for pixel clock edge
// bit     20 : for output pixel format when base is RGBT16
//

pub const LCD_TYPE_MASK: c_uint = 0xf;
pub const LCD_TYPE_UNKNOWN: c_int = 0;
pub const LCD_TYPE_MONO_STN: c_int = 1;
pub const LCD_TYPE_MONO_DSTN: c_int = 2;
pub const LCD_TYPE_COLOR_STN: c_int = 3;
pub const LCD_TYPE_COLOR_DSTN: c_int = 4;
pub const LCD_TYPE_COLOR_TFT: c_int = 5;
pub const LCD_TYPE_SMART_PANEL: c_int = 6;
pub const LCD_TYPE_MAX: c_int = 7;

//
// This structure describes the machine which we are running on.
// It is set in linux/arch/arm/mach-pxa/machine_name.c and used in the probe routine
// of linux/drivers/video/pxafb.c
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxafb_mode_info {
    pub pixclock: u_long,
    pub xres: u_short,
    pub yres: u_short,
    pub bpp: u_char,
// Parallel Mode Timing
    pub hsync_len: u_char,
    pub left_margin: u_char,
    pub right_margin: u_char,
    pub vsync_len: u_char,
    pub upper_margin: u_char,
    pub lower_margin: u_char,
    pub sync: u_char,
// Smart Panel Mode Timing - see PXA27x DM 7.4.15.0.3 for details
// Note:
// 1. all parameters in nanosecond (ns)
// 2. a0cs{rd,wr}_set_hld are controlled by the same register bits
// in pxa27x and pxa3xx, initialize them to the same value or
// the larger one will be used
// 3. same to {rd,wr}_pulse_width
//
// 4. LCD_PCLK_EDGE_{RISE,FALL} controls the L_PCLK_WR polarity
// 5. sync & FB_SYNC_HOR_HIGH_ACT controls the L_LCLK_A0
// 6. sync & FB_SYNC_VERT_HIGH_ACT controls the L_LCLK_RD
//
    pub /: *mut *mut unsigned a0csrd_set_hld; / A0 and CS Setup/Hold Time before/after L_FCLK_RD,
    pub /: *mut *mut unsigned a0cswr_set_hld; / A0 and CS Setup/Hold Time before/after L_PCLK_WR,
    pub /: *mut *mut unsigned wr_pulse_width; / L_PCLK_WR pulse width,
    pub /: *mut *mut unsigned rd_pulse_width; / L_FCLK_RD pulse width,
    pub /: *mut *mut unsigned cmd_inh_time; / Command Inhibit time between two writes,
    pub /: *mut *mut unsigned op_hold_time; / Output Hold time from L_FCLK_RD negation,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxafb_mach_info {
    pub modes: *mut pxafb_mode_info,
    pub num_modes: c_uint,
    pub lcd_conn: c_uint,
    pub video_mem_size: c_ulong,
// The following should be defined in LCCR0
// LCCR0_Act or LCCR0_Pas          Active or Passive
// LCCR0_Sngl or LCCR0_Dual        Single/Dual panel
// LCCR0_Mono or LCCR0_Color       Mono/Color
// LCCR0_4PixMono or LCCR0_8PixMono (in mono single mode)
// LCCR0_DMADel(Tcpu) (optional)   DMA request delay
//
// The following should not be defined in LCCR0:
// LCCR0_OUM, LCCR0_BM, LCCR0_QDM, LCCR0_DIS, LCCR0_EFM
// LCCR0_IUM, LCCR0_SFM, LCCR0_LDM, LCCR0_ENB
//
    pub lccr0: u_int,
// The following should be defined in LCCR3
// LCCR3_OutEnH or LCCR3_OutEnL    Output enable polarity
// LCCR3_PixRsEdg or LCCR3_PixFlEdg Pixel clock edge type
// LCCR3_Acb(X)                    AB Bias pin frequency
// LCCR3_DPC (optional)            Double Pixel Clock mode (untested)
//
// The following should not be defined in LCCR3
// LCCR3_HSP, LCCR3_VSP, LCCR0_Pcd(x), LCCR3_Bpp
//
    pub lccr3: u_int,
// The following should be defined in LCCR4
// LCCR4_PAL_FOR_0 or LCCR4_PAL_FOR_1 or LCCR4_PAL_FOR_2
//
// All other bits in LCCR4 should be left alone.
//
    pub lccr4: u_int,
    pub (*pxafb_backlight_power)(int): *mut c_void,
    pub ): *mut *mut void (pxafb_lcd_power)(int, struct fb_var_screeninfo,
    pub ): *mut *mut void (smart_update)(struct fb_info,
}

extern "C" {
    pub fn pxa_set_fb_info(: *mut device, : *mut pxafb_mach_info);
}
// smartpanel related

// SMART_DELAY() is introduced for software controlled delay primitive which
// can be inserted between command sequences, unused command 0x6 is used here
// and delay ranges from 0ms ~ 255ms
//

extern "C" {
    pub fn pxafb_smart_queue(info: *mut fb_info, cmds: *mut u16, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn pxafb_smart_flush(info: *mut fb_info) -> c_int;
}

