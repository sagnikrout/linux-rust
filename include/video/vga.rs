//! Automatically rewritten from C Header to Rust Module
//! Source: include/video/vga.h
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
// linux/include/video/vga.h -- standard VGA chipset interaction
//
// Copyright 1999 Jeff Garzik <jgarzik@pobox.com>
//
// Copyright history from vga16fb.c:
// Copyright 1999 Ben Pfaff and Petr Vandrovec
// Based on VGA info at http://www.osdever.net/FreeVGA/home.htm
// Based on VESA framebuffer (c) 1998 Gerd Knorr
//
// This file is subject to the terms and conditions of the GNU General
// Public License.  See the file COPYING in the main directory of this
// archive for more details.
//

// Macro flag: #define __linux_video_vga_h__

pub const VGA_FB_PHYS_BASE: c_uint = 0xA0000 /* VGA framebuffer I/O base */;

// Some of the code below is taken from SVGAlib.  The original,
// VGAlib version 1.2 - (c) 1993 Tommy Frandsen
//
// This library is free software; you can redistribute it and/or
// modify it without any restrictions. This library is distributed
// in the hope that it will be useful, but without any warranty.
// Multi-chipset support Copyright 1993 Harm Hanemaayer
// partially copyrighted (C) 1993 by Hartmut Schirmer
// VGA data register ports
pub const VGA_CRT_DC: c_uint = 0x3D5	/* CRT Controller Data Register - color emulation */;
pub const VGA_CRT_DM: c_uint = 0x3B5	/* CRT Controller Data Register - mono emulation */;
pub const VGA_ATT_R: c_uint = 0x3C1	/* Attribute Controller Data Read Register */;
pub const VGA_ATT_W: c_uint = 0x3C0	/* Attribute Controller Data Write Register */;
pub const VGA_GFX_D: c_uint = 0x3CF	/* Graphics Controller Data Register */;
pub const VGA_SEQ_D: c_uint = 0x3C5	/* Sequencer Data Register */;
pub const VGA_MIS_R: c_uint = 0x3CC	/* Misc Output Read Register */;
pub const VGA_MIS_W: c_uint = 0x3C2	/* Misc Output Write Register */;
pub const VGA_FTC_R: c_uint = 0x3CA	/* Feature Control Read Register */;
pub const VGA_IS0_R: c_uint = 0x3C2	/* Input Status Register 0 */;
pub const VGA_IS1_RC: c_uint = 0x3DA	/* Input Status Register 1 - color emulation */;
pub const VGA_IS1_RM: c_uint = 0x3BA	/* Input Status Register 1 - mono emulation */;
pub const VGA_PEL_D: c_uint = 0x3C9	/* PEL Data Register */;
pub const VGA_PEL_MSK: c_uint = 0x3C6	/* PEL mask register */;
// EGA-specific registers
pub const EGA_GFX_E0: c_uint = 0x3CC	/* Graphics enable processor 0 */;
pub const EGA_GFX_E1: c_uint = 0x3CA	/* Graphics enable processor 1 */;
// VGA index register ports
pub const VGA_CRT_IC: c_uint = 0x3D4	/* CRT Controller Index - color emulation */;
pub const VGA_CRT_IM: c_uint = 0x3B4	/* CRT Controller Index - mono emulation */;
pub const VGA_ATT_IW: c_uint = 0x3C0	/* Attribute Controller Index & Data Write Register */;
pub const VGA_GFX_I: c_uint = 0x3CE	/* Graphics Controller Index */;
pub const VGA_SEQ_I: c_uint = 0x3C4	/* Sequencer Index */;
pub const VGA_PEL_IW: c_uint = 0x3C8	/* PEL Write Index */;
pub const VGA_PEL_IR: c_uint = 0x3C7	/* PEL Read Index */;
// standard VGA indexes max counts
pub const VGA_CRT_C: c_uint = 0x19	/* Number of CRT Controller Registers */;
pub const VGA_ATT_C: c_uint = 0x15	/* Number of Attribute Controller Registers */;
pub const VGA_GFX_C: c_uint = 0x09	/* Number of Graphics Controller Registers */;
pub const VGA_SEQ_C: c_uint = 0x05	/* Number of Sequencer Registers */;
pub const VGA_MIS_C: c_uint = 0x01	/* Number of Misc Output Register */;
// VGA misc register bit masks
pub const VGA_MIS_COLOR: c_uint = 0x01;
pub const VGA_MIS_ENB_MEM_ACCESS: c_uint = 0x02;
pub const VGA_MIS_DCLK_28322_720: c_uint = 0x04;

pub const VGA_MIS_SEL_HIGH_PAGE: c_uint = 0x20;
// VGA CRT controller register indices
pub const VGA_CRTC_H_TOTAL: c_int = 0;
pub const VGA_CRTC_H_DISP: c_int = 1;
pub const VGA_CRTC_H_BLANK_START: c_int = 2;
pub const VGA_CRTC_H_BLANK_END: c_int = 3;
pub const VGA_CRTC_H_SYNC_START: c_int = 4;
pub const VGA_CRTC_H_SYNC_END: c_int = 5;
pub const VGA_CRTC_V_TOTAL: c_int = 6;
pub const VGA_CRTC_OVERFLOW: c_int = 7;
pub const VGA_CRTC_PRESET_ROW: c_int = 8;
pub const VGA_CRTC_MAX_SCAN: c_int = 9;
pub const VGA_CRTC_CURSOR_START: c_uint = 0x0A;
pub const VGA_CRTC_CURSOR_END: c_uint = 0x0B;
pub const VGA_CRTC_START_HI: c_uint = 0x0C;
pub const VGA_CRTC_START_LO: c_uint = 0x0D;
pub const VGA_CRTC_CURSOR_HI: c_uint = 0x0E;
pub const VGA_CRTC_CURSOR_LO: c_uint = 0x0F;
pub const VGA_CRTC_V_SYNC_START: c_uint = 0x10;
pub const VGA_CRTC_V_SYNC_END: c_uint = 0x11;
pub const VGA_CRTC_V_DISP_END: c_uint = 0x12;
pub const VGA_CRTC_OFFSET: c_uint = 0x13;
pub const VGA_CRTC_UNDERLINE: c_uint = 0x14;
pub const VGA_CRTC_V_BLANK_START: c_uint = 0x15;
pub const VGA_CRTC_V_BLANK_END: c_uint = 0x16;
pub const VGA_CRTC_MODE: c_uint = 0x17;
pub const VGA_CRTC_LINE_COMPARE: c_uint = 0x18;

// VGA CRT controller bit masks
pub const VGA_CR11_LOCK_CR0_CR7: c_uint = 0x80 /* lock writes to CR0 - CR7 */;
pub const VGA_CR17_H_V_SIGNALS_ENABLED: c_uint = 0x80;
// VGA attribute controller register indices
pub const VGA_ATC_PALETTE0: c_uint = 0x00;
pub const VGA_ATC_PALETTE1: c_uint = 0x01;
pub const VGA_ATC_PALETTE2: c_uint = 0x02;
pub const VGA_ATC_PALETTE3: c_uint = 0x03;
pub const VGA_ATC_PALETTE4: c_uint = 0x04;
pub const VGA_ATC_PALETTE5: c_uint = 0x05;
pub const VGA_ATC_PALETTE6: c_uint = 0x06;
pub const VGA_ATC_PALETTE7: c_uint = 0x07;
pub const VGA_ATC_PALETTE8: c_uint = 0x08;
pub const VGA_ATC_PALETTE9: c_uint = 0x09;
pub const VGA_ATC_PALETTEA: c_uint = 0x0A;
pub const VGA_ATC_PALETTEB: c_uint = 0x0B;
pub const VGA_ATC_PALETTEC: c_uint = 0x0C;
pub const VGA_ATC_PALETTED: c_uint = 0x0D;
pub const VGA_ATC_PALETTEE: c_uint = 0x0E;
pub const VGA_ATC_PALETTEF: c_uint = 0x0F;
pub const VGA_ATC_MODE: c_uint = 0x10;
pub const VGA_ATC_OVERSCAN: c_uint = 0x11;
pub const VGA_ATC_PLANE_ENABLE: c_uint = 0x12;
pub const VGA_ATC_PEL: c_uint = 0x13;
pub const VGA_ATC_COLOR_PAGE: c_uint = 0x14;
pub const VGA_AR_ENABLE_DISPLAY: c_uint = 0x20;
// VGA sequencer register indices
pub const VGA_SEQ_RESET: c_uint = 0x00;
pub const VGA_SEQ_CLOCK_MODE: c_uint = 0x01;
pub const VGA_SEQ_PLANE_WRITE: c_uint = 0x02;
pub const VGA_SEQ_CHARACTER_MAP: c_uint = 0x03;
pub const VGA_SEQ_MEMORY_MODE: c_uint = 0x04;
// VGA sequencer register bit masks
pub const VGA_SR01_CHAR_CLK_8DOTS: c_uint = 0x01 /* bit 0: character clocks 8 dots wide are generated */;
pub const VGA_SR01_SCREEN_OFF: c_uint = 0x20 /* bit 5: Screen is off */;
pub const VGA_SR02_ALL_PLANES: c_uint = 0x0F /* bits 3-0: enable access to all planes */;
pub const VGA_SR04_EXT_MEM: c_uint = 0x02 /* bit 1: allows complete mem access to 256K */;
pub const VGA_SR04_SEQ_MODE: c_uint = 0x04 /* bit 2: directs system to use a sequential addressing mode */;
pub const VGA_SR04_CHN_4M: c_uint = 0x08 /* bit 3: selects modulo 4 addressing for CPU access to display memory */;
// VGA graphics controller register indices
pub const VGA_GFX_SR_VALUE: c_uint = 0x00;
pub const VGA_GFX_SR_ENABLE: c_uint = 0x01;
pub const VGA_GFX_COMPARE_VALUE: c_uint = 0x02;
pub const VGA_GFX_DATA_ROTATE: c_uint = 0x03;
pub const VGA_GFX_PLANE_READ: c_uint = 0x04;
pub const VGA_GFX_MODE: c_uint = 0x05;
pub const VGA_GFX_MISC: c_uint = 0x06;
pub const VGA_GFX_COMPARE_MASK: c_uint = 0x07;
pub const VGA_GFX_BIT_MASK: c_uint = 0x08;
// VGA graphics controller bit masks
pub const VGA_GR06_GRAPHICS_MODE: c_uint = 0x01;
// macro for composing an 8-bit VGA register index and value
// into a single 16-bit quantity

// decide whether we should enable the faster 16-bit VGA register writes

// Macro flag: #define VGA_OUTW_WRITE

// VGA State Save and Restore

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vgastate {
    pub /: *mut *mut *mut void __iomem vgabase; / mmio base, if supported,
    pub /: *mut *mut unsigned long membase; / VGA window base, 0 for default - 0xA000,
    pub /: *mut *mut __u32 memsize; / VGA window size, 0 for default 64K,
    pub /: *mut *mut *mut __u32 flags; / what state[s] to save (see VGA_SAVE_),
    pub /: *mut *mut __u32 depth; / current fb depth, not important,
    pub /: *mut *mut __u32 num_attr; / number of att registers, 0 for default,
    pub /: *mut *mut __u32 num_crtc; / number of crt registers, 0 for default,
    pub /: *mut *mut __u32 num_gfx; / number of gfx registers, 0 for default,
    pub /: *mut *mut __u32 num_seq; / number of seq registers, 0 for default,
    pub vidstate: *mut c_void,
}

extern "C" {
    pub fn save_vga(state: *mut vgastate) -> c_int;
}
extern "C" {
    pub fn restore_vga(state: *mut vgastate) -> c_int;
}
extern "C" {
    pub fn readb(port: regbase +) -> return;
}
//
// generic VGA port read/write
//

extern "C" {
    pub fn inb_p(_arg: port) -> return;
}
extern "C" {
    pub fn vga_mm_r(_arg: regbase, _arg: port) -> return;
}
extern "C" {
    pub fn vga_io_r(_arg: port) -> return;
}

extern "C" {
    pub fn vga_mm_r(_arg: regbase, _arg: port) -> return;
}

//
// VGA CRTC register read/write
//
extern "C" {
    pub fn vga_r(_arg: regbase, _arg: VGA_CRT_DC) -> return;
}

extern "C" {
    pub fn vga_io_r(_arg: VGA_CRT_DC) -> return;
}

extern "C" {
    pub fn vga_mm_r(_arg: regbase, _arg: VGA_CRT_DC) -> return;
}

//
// VGA sequencer register read/write
//
extern "C" {
    pub fn vga_r(_arg: regbase, _arg: VGA_SEQ_D) -> return;
}

extern "C" {
    pub fn vga_io_r(_arg: VGA_SEQ_D) -> return;
}

extern "C" {
    pub fn vga_mm_r(_arg: regbase, _arg: VGA_SEQ_D) -> return;
}

//
// VGA graphics controller register read/write
//
extern "C" {
    pub fn vga_r(_arg: regbase, _arg: VGA_GFX_D) -> return;
}

extern "C" {
    pub fn vga_io_r(_arg: VGA_GFX_D) -> return;
}

extern "C" {
    pub fn vga_mm_r(_arg: regbase, _arg: VGA_GFX_D) -> return;
}

//
// VGA attribute controller register read/write
//
extern "C" {
    pub fn vga_r(_arg: regbase, _arg: VGA_ATT_R) -> return;
}

extern "C" {
    pub fn vga_io_r(_arg: VGA_ATT_R) -> return;
}

extern "C" {
    pub fn vga_mm_r(_arg: regbase, _arg: VGA_ATT_R) -> return;
}
