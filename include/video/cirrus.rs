//! Automatically rewritten from C Header to Rust Module
//! Source: include/video/cirrus.h
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
// drivers/video/clgenfb.h - Cirrus Logic chipset constants
//
// Copyright 1999 Jeff Garzik <jgarzik@pobox.com>
//
// Original clgenfb author:  Frank Neumann
//
// Based on retz3fb.c and clgen.c:
// Copyright (C) 1997 Jes Sorensen
// Copyright (C) 1996 Frank Neumann
//
// Format this code with GNU indent '-kr -i8 -pcs' options.
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of this archive
// for more details.
//
// OLD COMMENT: definitions for Piccolo/SD64 VGA controller chip
// OLD COMMENT: these definitions might most of the time also work
// OLD COMMENT: for other CL-GD542x/543x based boards..
// External/General Registers
pub const CL_POS102: c_uint = 0x102  	/* POS102 register */;
pub const CL_VSSM: c_uint = 0x46e8 	/* Adapter Sleep */;
pub const CL_VSSM2: c_uint = 0x3c3	/* Motherboard Sleep */;
// VGA Sequencer Registers
// the following are from the "extension registers" group
pub const CL_SEQR6: c_uint = 0x6	/* Unlock ALL Extensions */;
pub const CL_SEQR7: c_uint = 0x7	/* Extended Sequencer Mode */;
pub const CL_SEQR8: c_uint = 0x8	/* EEPROM Control */;
pub const CL_SEQR9: c_uint = 0x9	/* Scratch Pad 0 (do not access!) */;
pub const CL_SEQRA: c_uint = 0xa	/* Scratch Pad 1 (do not access!) */;
pub const CL_SEQRB: c_uint = 0xb	/* VCLK0 Numerator */;
pub const CL_SEQRC: c_uint = 0xc	/* VCLK1 Numerator */;
pub const CL_SEQRD: c_uint = 0xd	/* VCLK2 Numerator */;
pub const CL_SEQRE: c_uint = 0xe	/* VCLK3 Numerator */;
pub const CL_SEQRF: c_uint = 0xf	/* DRAM Control */;
pub const CL_SEQR10: c_uint = 0x10	/* Graphics Cursor X Position */;
pub const CL_SEQR11: c_uint = 0x11	/* Graphics Cursor Y Position */;
pub const CL_SEQR12: c_uint = 0x12	/* Graphics Cursor Attributes */;
pub const CL_SEQR13: c_uint = 0x13	/* Graphics Cursor Pattern Address Offset */;
pub const CL_SEQR14: c_uint = 0x14	/* Scratch Pad 2 (CL-GD5426/'28 Only) (do not access!) */;
pub const CL_SEQR15: c_uint = 0x15	/* Scratch Pad 3 (CL-GD5426/'28 Only) (do not access!) */;
pub const CL_SEQR16: c_uint = 0x16	/* Performance Tuning (CL-GD5424/'26/'28 Only) */;
pub const CL_SEQR17: c_uint = 0x17	/* Configuration ReadBack and Extended Control (CL-GF5428 Only) */;
pub const CL_SEQR18: c_uint = 0x18	/* Signature Generator Control (Not CL-GD5420) */;
pub const CL_SEQR19: c_uint = 0x19	/* Signature Generator Result Low Byte (Not CL-GD5420) */;
pub const CL_SEQR1A: c_uint = 0x1a	/* Signature Generator Result High Byte (Not CL-GD5420) */;
pub const CL_SEQR1B: c_uint = 0x1b	/* VCLK0 Denominator and Post-Scalar Value */;
pub const CL_SEQR1C: c_uint = 0x1c	/* VCLK1 Denominator and Post-Scalar Value */;
pub const CL_SEQR1D: c_uint = 0x1d	/* VCLK2 Denominator and Post-Scalar Value */;
pub const CL_SEQR1E: c_uint = 0x1e	/* VCLK3 Denominator and Post-Scalar Value */;
pub const CL_SEQR1F: c_uint = 0x1f	/* BIOS ROM write enable and MCLK Select */;
// CRT Controller Registers
pub const CL_CRT22: c_uint = 0x22	/* Graphics Data Latches ReadBack */;
pub const CL_CRT24: c_uint = 0x24	/* Attribute Controller Toggle ReadBack */;
pub const CL_CRT26: c_uint = 0x26	/* Attribute Controller Index ReadBack */;
// the following are from the "extension registers" group
pub const CL_CRT19: c_uint = 0x19	/* Interlace End */;
pub const CL_CRT1A: c_uint = 0x1a	/* Interlace Control */;
pub const CL_CRT1B: c_uint = 0x1b	/* Extended Display Controls */;
pub const CL_CRT1C: c_uint = 0x1c	/* Sync adjust and genlock register */;
pub const CL_CRT1D: c_uint = 0x1d	/* Overlay Extended Control register */;
pub const CL_CRT1E: c_uint = 0x1e	/* Another overflow register */;
pub const CL_CRT25: c_uint = 0x25	/* Part Status Register */;
pub const CL_CRT27: c_uint = 0x27	/* ID Register */;
pub const CL_CRT51: c_uint = 0x51	/* P4 disable "flicker fixer" */;
// Graphics Controller Registers
// the following are from the "extension registers" group
pub const CL_GR9: c_uint = 0x9	/* Offset Register 0 */;
pub const CL_GRA: c_uint = 0xa	/* Offset Register 1 */;
pub const CL_GRB: c_uint = 0xb	/* Graphics Controller Mode Extensions */;
pub const CL_GRC: c_uint = 0xc	/* Color Key (CL-GD5424/'26/'28 Only) */;
pub const CL_GRD: c_uint = 0xd	/* Color Key Mask (CL-GD5424/'26/'28 Only) */;
pub const CL_GRE: c_uint = 0xe	/* Miscellaneous Control (Cl-GD5428 Only) */;
pub const CL_GRF: c_uint = 0xf	/* Display Compression Control register */;
pub const CL_GR10: c_uint = 0x10	/* 16-bit Pixel BG Color High Byte (Not CL-GD5420) */;
pub const CL_GR11: c_uint = 0x11	/* 16-bit Pixel FG Color High Byte (Not CL-GD5420) */;
pub const CL_GR12: c_uint = 0x12	/* Background Color Byte 2 Register */;
pub const CL_GR13: c_uint = 0x13	/* Foreground Color Byte 2 Register */;
pub const CL_GR14: c_uint = 0x14	/* Background Color Byte 3 Register */;
pub const CL_GR15: c_uint = 0x15	/* Foreground Color Byte 3 Register */;
// the following are CL-GD5426/'28 specific blitter registers
pub const CL_GR20: c_uint = 0x20	/* BLT Width Low */;
pub const CL_GR21: c_uint = 0x21	/* BLT Width High */;
pub const CL_GR22: c_uint = 0x22	/* BLT Height Low */;
pub const CL_GR23: c_uint = 0x23	/* BLT Height High */;
pub const CL_GR24: c_uint = 0x24	/* BLT Destination Pitch Low */;
pub const CL_GR25: c_uint = 0x25	/* BLT Destination Pitch High */;
pub const CL_GR26: c_uint = 0x26	/* BLT Source Pitch Low */;
pub const CL_GR27: c_uint = 0x27	/* BLT Source Pitch High */;
pub const CL_GR28: c_uint = 0x28	/* BLT Destination Start Low */;
pub const CL_GR29: c_uint = 0x29	/* BLT Destination Start Mid */;
pub const CL_GR2A: c_uint = 0x2a	/* BLT Destination Start High */;
pub const CL_GR2C: c_uint = 0x2c	/* BLT Source Start Low */;
pub const CL_GR2D: c_uint = 0x2d	/* BLT Source Start Mid */;
pub const CL_GR2E: c_uint = 0x2e	/* BLT Source Start High */;
pub const CL_GR2F: c_uint = 0x2f	/* Picasso IV Blitter compat mode..? */;
pub const CL_GR30: c_uint = 0x30	/* BLT Mode */;
pub const CL_GR31: c_uint = 0x31	/* BLT Start/Status */;
pub const CL_GR32: c_uint = 0x32	/* BLT Raster Operation */;
pub const CL_GR33: c_uint = 0x33	/* another P4 "compat" register.. */;
pub const CL_GR34: c_uint = 0x34	/* Transparent Color Select Low */;
pub const CL_GR35: c_uint = 0x35	/* Transparent Color Select High */;
pub const CL_GR38: c_uint = 0x38	/* Source Transparent Color Mask Low */;
pub const CL_GR39: c_uint = 0x39	/* Source Transparent Color Mask High */;
// Attribute Controller Registers
pub const CL_AR33: c_uint = 0x33	/* The "real" Pixel Panning register (?) */;
pub const CL_AR34: c_uint = 0x34	/* TEST */;
