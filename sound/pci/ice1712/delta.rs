//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/ice1712/delta.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// ALSA driver for ICEnsemble ICE1712 (Envy24)
//
// Lowlevel functions for M-Audio Delta 1010, 44, 66, Dio2496, Audiophile
// Digigram VX442
//
// Copyright (c) 2000 Jaroslav Kysela <perex@perex.cz>
//

pub const ICE1712_SUBDEVICE_DELTA1010: c_uint = 0x121430d6;
pub const ICE1712_SUBDEVICE_DELTA1010E: c_uint = 0xff1430d6;
pub const ICE1712_SUBDEVICE_DELTADIO2496: c_uint = 0x121431d6;
pub const ICE1712_SUBDEVICE_DELTA66: c_uint = 0x121432d6;
pub const ICE1712_SUBDEVICE_DELTA66E: c_uint = 0xff1432d6;
pub const ICE1712_SUBDEVICE_DELTA44: c_uint = 0x121433d6;
pub const ICE1712_SUBDEVICE_AUDIOPHILE: c_uint = 0x121434d6;
pub const ICE1712_SUBDEVICE_DELTA410: c_uint = 0x121438d6;
pub const ICE1712_SUBDEVICE_DELTA1010LT: c_uint = 0x12143bd6;
pub const ICE1712_SUBDEVICE_VX442: c_uint = 0x12143cd6;
pub const ICE1712_SUBDEVICE_MEDIASTATION: c_uint = 0x694c0100;
pub const ICE1712_SUBDEVICE_EDIROLDA2496: c_uint = 0xce164010;
// entry point
//
// MidiMan M-Audio Delta GPIO definitions
//
// MidiMan M-Audio Delta shared pins
pub const ICE1712_DELTA_DFS: c_uint = 0x01		/* fast/slow sample rate mode */;
// (>48kHz must be 1)
pub const ICE1712_DELTA_SPDIF_IN_STAT: c_uint = 0x02;
// S/PDIF input status
// 0 = valid signal is present
// all except Delta44
// look to CS8414 datasheet
pub const ICE1712_DELTA_SPDIF_OUT_STAT_CLOCK: c_uint = 0x04;
// S/PDIF output status clock
// (writing on rising edge - 0->1)
// all except Delta44
// look to CS8404A datasheet
pub const ICE1712_DELTA_SPDIF_OUT_STAT_DATA: c_uint = 0x08;
// S/PDIF output status data
// all except Delta44
// look to CS8404A datasheet
// MidiMan M-Audio DeltaDiO
// 0x01 = DFS
// 0x02 = SPDIF_IN_STAT
// 0x04 = SPDIF_OUT_STAT_CLOCK
// 0x08 = SPDIF_OUT_STAT_DATA
pub const ICE1712_DELTA_SPDIF_INPUT_SELECT: c_uint = 0x10;
// coaxial (0), optical (1)
// S/PDIF input select
// MidiMan M-Audio Delta1010
// 0x01 = DFS
// 0x02 = SPDIF_IN_STAT
// 0x04 = SPDIF_OUT_STAT_CLOCK
// 0x08 = SPDIF_OUT_STAT_DATA
pub const ICE1712_DELTA_WORD_CLOCK_SELECT: c_uint = 0x10;
// 1 - clock are taken from S/PDIF input
// 0 - clock are taken from Word Clock input
// affected SPMCLKIN pin of Envy24
pub const ICE1712_DELTA_WORD_CLOCK_STATUS: c_uint = 0x20;
// 0 = valid word clock signal is present
// MidiMan M-Audio Delta66
// 0x01 = DFS
// 0x02 = SPDIF_IN_STAT
// 0x04 = SPDIF_OUT_STAT_CLOCK
// 0x08 = SPDIF_OUT_STAT_DATA
pub const ICE1712_DELTA_CODEC_SERIAL_DATA: c_uint = 0x10;
// AKM4524 serial data
pub const ICE1712_DELTA_CODEC_SERIAL_CLOCK: c_uint = 0x20;
// AKM4524 serial clock
// (writing on rising edge - 0->1
pub const ICE1712_DELTA_CODEC_CHIP_A: c_uint = 0x40;
pub const ICE1712_DELTA_CODEC_CHIP_B: c_uint = 0x80;
// 1 - select chip A or B
// MidiMan M-Audio Delta44
// 0x01 = DFS
// 0x10 = CODEC_SERIAL_DATA
// 0x20 = CODEC_SERIAL_CLOCK
// 0x40 = CODEC_CHIP_A
// 0x80 = CODEC_CHIP_B
// MidiMan M-Audio Audiophile/Delta410 definitions
// thanks to Kristof Pelckmans <Kristof.Pelckmans@antwerpen.be> for Delta410 info
// 0x01 = DFS
pub const ICE1712_DELTA_AP_CCLK: c_uint = 0x02	/* SPI clock */;
// (clocking on rising edge - 0->1)
pub const ICE1712_DELTA_AP_DIN: c_uint = 0x04	/* data input */;
pub const ICE1712_DELTA_AP_DOUT: c_uint = 0x08	/* data output */;
pub const ICE1712_DELTA_AP_CS_DIGITAL: c_uint = 0x10 /* CS8427 chip select */;
// low signal = select
pub const ICE1712_DELTA_AP_CS_CODEC: c_uint = 0x20	/* AK4528 (audiophile), AK4529 (Delta410) chip select */;
// low signal = select
// MidiMan M-Audio Delta1010LT definitions
// thanks to Anders Johansson <ajh@watri.uwa.edu.au>
// 0x01 = DFS
pub const ICE1712_DELTA_1010LT_CCLK: c_uint = 0x02	/* SPI clock (AK4524 + CS8427) */;
pub const ICE1712_DELTA_1010LT_DIN: c_uint = 0x04	/* data input (CS8427) */;
pub const ICE1712_DELTA_1010LT_DOUT: c_uint = 0x08	/* data output (AK4524 + CS8427) */;
pub const ICE1712_DELTA_1010LT_CS: c_uint = 0x70	/* mask for CS address */;
pub const ICE1712_DELTA_1010LT_CS_CHIP_A: c_uint = 0x00	/* AK4524 #0 */;
pub const ICE1712_DELTA_1010LT_CS_CHIP_B: c_uint = 0x10	/* AK4524 #1 */;
pub const ICE1712_DELTA_1010LT_CS_CHIP_C: c_uint = 0x20	/* AK4524 #2 */;
pub const ICE1712_DELTA_1010LT_CS_CHIP_D: c_uint = 0x30	/* AK4524 #3 */;
pub const ICE1712_DELTA_1010LT_CS_CS8427: c_uint = 0x40	/* CS8427 */;
pub const ICE1712_DELTA_1010LT_CS_NONE: c_uint = 0x50	/* nothing */;
pub const ICE1712_DELTA_1010LT_WORDCLOCK: c_uint = 0x80	/* sample clock source: 0 = Word Clock Input, 1 = S/PDIF Input ??? */;
// M-Audio Delta 66 rev. E definitions.
// Newer revisions of Delta 66 have CS8427 over SPI for
// S/PDIF transceiver instead of CS8404/CS8414.
// 0x01 = DFS
pub const ICE1712_DELTA_66E_CCLK: c_uint = 0x02	/* SPI clock */;
pub const ICE1712_DELTA_66E_DIN: c_uint = 0x04	/* data input */;
pub const ICE1712_DELTA_66E_DOUT: c_uint = 0x08	/* data output */;
pub const ICE1712_DELTA_66E_CS_CS8427: c_uint = 0x10	/* chip select, low = CS8427 */;
pub const ICE1712_DELTA_66E_CS_CHIP_A: c_uint = 0x20	/* AK4524 #0 */;
pub const ICE1712_DELTA_66E_CS_CHIP_B: c_uint = 0x40	/* AK4524 #1 */;
// Digigram VX442 definitions
pub const ICE1712_VX442_CCLK: c_uint = 0x02	/* SPI clock */;
pub const ICE1712_VX442_DIN: c_uint = 0x04	/* data input */;
pub const ICE1712_VX442_DOUT: c_uint = 0x08	/* data output */;
pub const ICE1712_VX442_CS_DIGITAL: c_uint = 0x10	/* chip select, low = CS8427 */;
pub const ICE1712_VX442_CODEC_CHIP_A: c_uint = 0x20	/* select chip A */;
pub const ICE1712_VX442_CODEC_CHIP_B: c_uint = 0x40	/* select chip B */;
