//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/sis/init.h
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


// $XFree86$
// $XdotOrg$
//
// Data and prototypes for init.c
//
// Copyright (C) 2001-2005 by Thomas Winischhofer, Vienna, Austria
//
// If distributed as part of the Linux kernel, the following license terms
// apply:
//
// * This program is free software; you can redistribute it and/or modify
// * it under the terms of the GNU General Public License as published by
// * the Free Software Foundation; either version 2 of the named License,
// * or any later version.
//
// * This program is distributed in the hope that it will be useful,
// * but WITHOUT ANY WARRANTY; without even the implied warranty of
// * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// * GNU General Public License for more details.
//
// * You should have received a copy of the GNU General Public License
// * along with this program; if not, write to the Free Software
// * Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307, USA
//
// Otherwise, the following license terms apply:
//
// * Redistribution and use in source and binary forms, with or without
// * modification, are permitted provided that the following conditions
// * are met:
// * 1) Redistributions of source code must retain the above copyright
// *    notice, this list of conditions and the following disclaimer.
// * 2) Redistributions in binary form must reproduce the above copyright
// *    notice, this list of conditions and the following disclaimer in the
// *    documentation and/or other materials provided with the distribution.
// * 3) The name of the author may not be used to endorse or promote products
// *    derived from this software without specific prior written permission.
//
// * THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR
// * IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
// * OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
// * IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT,
// * INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
// * NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
// * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// Author: 	Thomas Winischhofer <thomas@winischhofer.net>
//

// Mode numbers

// 0x00: MD_0_200
// 0x01: MD_1_200
// 0x02: MD_2_200
// 0x03: MD_3_200 - mode 0x03 - 0
// 0x04: MD_4
// 0x05: MD_5
// 0x06: MD_6
// 0x07: MD_7
// 0x08: MDA_DAC
// 0x09: CGA_DAC
// 0x0a: EGA_DAC
// 0x0b: VGA_DAC
// 0x0c
// 0x0d: MD_D
// 0x0e: MD_E
// 0x0f: ExtVGATable - modes > 0x13
// 0x10: ROM_SAVEPTR - totally different for 300
// 0x11: MD_F
// 0x12: MD_10
// 0x13: MD_0_350
// 0x14: MD_1_350
// 0x15: MD_2_350
// 0x16: MD_3_350 - mode 0x03 - 1
// 0x17: MD_0_1_400
// 0x18: MD_2_3_400 - mode 0x03 - 2
// 0x19: MD_7_400
// 0x1a: MD_11
// 0x1b: ExtEGATable - Modes <= 0x02
// 0x1c: MD_13

//
// SIS VIDEO BRIDGE -----------------------------------------
//

// About 1280x768: For TMDS, Panel_1280x768 will only be set if
// the panel is a Fujitsu 7911 (VL-17WDX8) (with clock 81, 1688x802)
// Other TMDS panels of this resolution will be treated as custom.
// For LVDS, we know another type (_2).
// (Note: 1280x768_3 is now special for SiS301/NetVista
//

// { 211,   60, 1260,  410, 1688, 1066 },    640x400 (6330)
// { 211,   80, 1400,  490, 1688, 1066 },    640x480 (6330)
// { 211,  117, 1638,  613, 1688, 1066 },    800x600 (6330)
// {27, 4, 800, 500, 2160, 1250 },    640x400 (6235)
// {27, 1, 800, 500, 2160, 1250 },    640x480 (6325)
//
// LVDS -----------------------------------------------------
//
// FSTN/DSTN 320x240, 2 variants
// CRT1 CRTC data for slave modes

