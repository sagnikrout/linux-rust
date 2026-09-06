//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/au88x0/au8830.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Macro flag: #define CHIP_AU8830

pub const NR_ADB: c_uint = 0x20;
pub const NR_SRC: c_uint = 0x10;
pub const NR_A3D: c_uint = 0x10;
pub const NR_MIXIN: c_uint = 0x20;
pub const NR_MIXOUT: c_uint = 0x10;
pub const NR_WT: c_uint = 0x40;
// ADBDMA
pub const VORTEX_ADBDMA_STAT: c_uint = 0x27e00	/* read only, subbuffer, DMA pos */;
pub const POS_MASK: c_uint = 0x00000fff;
pub const POS_SHIFT: c_uint = 0x0;
pub const ADB_SUBBUF_MASK: c_uint = 0x00003000	/* ADB only. */;
pub const ADB_SUBBUF_SHIFT: c_uint = 0xc	/* ADB only. */;
pub const VORTEX_ADBDMA_CTRL: c_uint = 0x27a00	/* write only; format, flags, DMA pos */;
pub const OFFSET_MASK: c_uint = 0x00000fff;
pub const OFFSET_SHIFT: c_uint = 0x0;
pub const IE_MASK: c_uint = 0x00001000	/* interrupt enable. */;
pub const IE_SHIFT: c_uint = 0xc;
pub const DIR_MASK: c_uint = 0x00002000	/* Direction. */;
pub const DIR_SHIFT: c_uint = 0xd;
pub const FMT_MASK: c_uint = 0x0003c000;
pub const FMT_SHIFT: c_uint = 0xe;
pub const ADB_FIFO_EN_SHIFT: c_uint = 0x15;

// The ADB masks and shift also are valid for the wtdma, except if specified otherwise.
pub const VORTEX_ADBDMA_BUFCFG0: c_uint = 0x27800;
pub const VORTEX_ADBDMA_BUFCFG1: c_uint = 0x27804;
pub const VORTEX_ADBDMA_BUFBASE: c_uint = 0x27400;
pub const VORTEX_ADBDMA_START: c_uint = 0x27c00	/* Which subbuffer starts */;
pub const VORTEX_ADBDMA_STATUS: c_uint = 0x27A90	/* stored at AdbDma->this_10 / 2 DWORD in size. */;
// Starting at the MSB, each pair of bits seem to be the current DMA page.
// This current page bits are consistent (same value) with VORTEX_ADBDMA_STAT)
// DMA
pub const VORTEX_ENGINE_CTRL: c_uint = 0x27ae8;
pub const ENGINE_INIT: c_uint = 0x1380000;
// WTDMA
pub const VORTEX_WTDMA_CTRL: c_uint = 0x27900	/* format, DMA pos */;
pub const VORTEX_WTDMA_STAT: c_uint = 0x27d00	/* DMA subbuf, DMA pos */;
pub const WT_SUBBUF_MASK: c_uint = 0x3;
pub const WT_SUBBUF_SHIFT: c_uint = 0xc;
pub const VORTEX_WTDMA_BUFBASE: c_uint = 0x27000;
pub const VORTEX_WTDMA_BUFCFG0: c_uint = 0x27600;
pub const VORTEX_WTDMA_BUFCFG1: c_uint = 0x27604;
pub const VORTEX_WTDMA_START: c_uint = 0x27b00	/* which subbuffer is first */;
// ADB
pub const VORTEX_ADB_SR: c_uint = 0x28400	/* Samplerates enable/disable */;
pub const VORTEX_ADB_RTBASE: c_uint = 0x28000;
pub const VORTEX_ADB_RTBASE_COUNT: c_int = 173;
pub const VORTEX_ADB_CHNBASE: c_uint = 0x282b4;
pub const VORTEX_ADB_CHNBASE_COUNT: c_int = 24;
pub const ROUTE_MASK: c_uint = 0xffff;
pub const SOURCE_MASK: c_uint = 0xff00;
pub const ADB_MASK: c_uint = 0xff;
pub const ADB_SHIFT: c_uint = 0x8;
// ADB address
pub const OFFSET_ADBDMA: c_uint = 0x00;
pub const OFFSET_ADBDMAB: c_uint = 0x20;
pub const OFFSET_SRCIN: c_uint = 0x40;
pub const OFFSET_SRCOUT: c_uint = 0x20	/* ch 0x11 */;
pub const OFFSET_MIXIN: c_uint = 0x50	/* ch 0x11 */;
pub const OFFSET_MIXOUT: c_uint = 0x30	/* ch 0x11 */;
pub const OFFSET_CODECIN: c_uint = 0x70 /* ch 0x11 */	/* adb source */;
pub const OFFSET_CODECOUT: c_uint = 0x88 /* ch 0x11 */	/* adb target */;
pub const OFFSET_SPORTIN: c_uint = 0x78	/* ch 0x13 ADB source. 2 routes. */;
pub const OFFSET_SPORTOUT: c_uint = 0x90	/* ch 0x13 ADB sink. 2 routes. */;
pub const OFFSET_SPDIFIN: c_uint = 0x7A	/* ch 0x14 ADB source. */;
pub const OFFSET_SPDIFOUT: c_uint = 0x92	/* ch 0x14 ADB sink. */;
pub const OFFSET_AC98IN: c_uint = 0x7c	/* ch 0x14 ADB source. */;
pub const OFFSET_AC98OUT: c_uint = 0x94	/* ch 0x14 ADB sink. */;
pub const OFFSET_EQIN: c_uint = 0xa0	/* ch 0x11 */;
pub const OFFSET_EQOUT: c_uint = 0x7e /* ch 0x11 */	/* 2 routes on ch 0x11 */;
pub const OFFSET_A3DIN: c_uint = 0x70	/* ADB sink. */;
pub const OFFSET_A3DOUT: c_uint = 0xA6	/* ADB source. 2 routes per slice = 8 */;
pub const OFFSET_WT0: c_uint = 0x40	/* WT bank 0 output. 0x40 - 0x65 */;
pub const OFFSET_WT1: c_uint = 0x80	/* WT bank 1 output. 0x80 - 0xA5 */;
// WT sources offset : 0x00-0x1f Direct stream.
// WT sources offset : 0x20-0x25 Mixed Output.
pub const OFFSET_XTALKOUT: c_uint = 0x66	/* crosstalk canceller (source) 2 routes */;
pub const OFFSET_XTALKIN: c_uint = 0x96	/* crosstalk canceller (sink). 10 routes */;
pub const OFFSET_EFXOUT: c_uint = 0x68	/* ADB source. 8 routes. */;
pub const OFFSET_EFXIN: c_uint = 0x80	/* ADB sink. 8 routes. */;
// ADB route translate helper

// #define ADB_WTOUT(x) ((x<x20)?(x + OFFSET_WT0):(x + OFFSET_WT1))

pub const MIX_DEFIGAIN: c_uint = 0x08;
pub const MIX_DEFOGAIN: c_uint = 0x08	/* 0x8->6dB  (6dB = x4) 16 to 18 bit conversion? */;
// MIXER
pub const VORTEX_MIXER_SR: c_uint = 0x21f00;
pub const VORTEX_MIXER_CLIP: c_uint = 0x21f80;
pub const VORTEX_MIXER_CHNBASE: c_uint = 0x21e40;
pub const VORTEX_MIXER_RTBASE: c_uint = 0x21e00;
pub const MIXER_RTBASE_SIZE: c_uint = 0x38;
pub const VORTEX_MIX_ENIN: c_uint = 0x21a00	/* Input enable bits. 4 bits wide. */;
pub const VORTEX_MIX_SMP: c_uint = 0x21c00	/* wave data buffers. AU8820: 0x9c00 */;
// MIX
pub const VORTEX_MIX_INVOL_B: c_uint = 0x20000	/* Input volume current */;
pub const VORTEX_MIX_VOL_B: c_uint = 0x20800	/* Output Volume current */;
pub const VORTEX_MIX_INVOL_A: c_uint = 0x21000	/* Input Volume target */;
pub const VORTEX_MIX_VOL_A: c_uint = 0x21800	/* Output Volume target */;
pub const VOL_MIN: c_uint = 0x80	/* Input volume when muted. */;
pub const VOL_MAX: c_uint = 0x7f	/* FIXME: Not confirmed! Just guessed. */;
// SRC
pub const VORTEX_SRC_CHNBASE: c_uint = 0x26c40;
pub const VORTEX_SRC_RTBASE: c_uint = 0x26c00;
pub const VORTEX_SRCBLOCK_SR: c_uint = 0x26cc0;
pub const VORTEX_SRC_SOURCE: c_uint = 0x26cc4;
pub const VORTEX_SRC_SOURCESIZE: c_uint = 0x26cc8;
// Params
//
pub const VORTEX_SRC_CONVRATIO: c_uint = 0x26e40;
pub const VORTEX_SRC_DRIFT0: c_uint = 0x26e80;
pub const VORTEX_SRC_DRIFT1: c_uint = 0x26ec0;
pub const VORTEX_SRC_DRIFT2: c_uint = 0x26f40;
pub const VORTEX_SRC_U0: c_uint = 0x26e00;
pub const U0_SLOWLOCK: c_uint = 0x200;
pub const VORTEX_SRC_U1: c_uint = 0x26f00;
pub const VORTEX_SRC_U2: c_uint = 0x26f80;
pub const VORTEX_SRC_DATA: c_uint = 0x26800	/* 0xc800 */;
pub const VORTEX_SRC_DATA0: c_uint = 0x26000;
// FIFO
pub const VORTEX_FIFO_ADBCTRL: c_uint = 0x16100	/* Control bits. */;
pub const VORTEX_FIFO_WTCTRL: c_uint = 0x16000;
pub const FIFO_RDONLY: c_uint = 0x00000001;
pub const FIFO_CTRL: c_uint = 0x00000002	/* Allow ctrl. ? */;
pub const FIFO_VALID: c_uint = 0x00000010;
pub const FIFO_EMPTY: c_uint = 0x00000020;
pub const FIFO_U0: c_uint = 0x00002000	/* Unknown. */;
pub const FIFO_U1: c_uint = 0x00040000;
pub const FIFO_SIZE_BITS: c_int = 6;

pub const FIFO_BITS: c_uint = 0x1c400000;
pub const VORTEX_FIFO_ADBDATA: c_uint = 0x14000;
pub const VORTEX_FIFO_WTDATA: c_uint = 0x10000;
pub const VORTEX_FIFO_GIRT: c_uint = 0x17000	/* wt0, wt1, adb */;
pub const GIRT_COUNT: c_int = 3;
// CODEC
pub const VORTEX_CODEC_CHN: c_uint = 0x29080	/* The name "CHN" is wrong. */;
pub const VORTEX_CODEC_CTRL: c_uint = 0x29184;
pub const VORTEX_CODEC_IO: c_uint = 0x29188;
pub const VORTEX_CODEC_SPORTCTRL: c_uint = 0x2918c;
pub const VORTEX_CODEC_EN: c_uint = 0x29190;
pub const EN_AUDIO0: c_uint = 0x00000300;
pub const EN_MODEM: c_uint = 0x00000c00;
pub const EN_AUDIO1: c_uint = 0x00003000;
pub const EN_SPORT: c_uint = 0x00030000;
pub const EN_SPDIF: c_uint = 0x000c0000;

pub const VORTEX_SPDIF_SMPRATE: c_uint = 0x29194;
pub const VORTEX_SPDIF_FLAGS: c_uint = 0x2205c;
pub const VORTEX_SPDIF_CFG0: c_uint = 0x291D0	/* status data */;
pub const VORTEX_SPDIF_CFG1: c_uint = 0x291D4;
pub const VORTEX_SMP_TIME: c_uint = 0x29198	/* Sample counter/timer */;
pub const VORTEX_SMP_TIMER: c_uint = 0x2919c;
pub const VORTEX_CODEC2_CTRL: c_uint = 0x291a0;
pub const VORTEX_MODEM_CTRL: c_uint = 0x291ac;
// IRQ
pub const VORTEX_IRQ_SOURCE: c_uint = 0x2a000	/* Interrupt source flags. */;
pub const VORTEX_IRQ_CTRL: c_uint = 0x2a004	/* Interrupt source mask. */;
// #define VORTEX_IRQ_U0 0x2a008 /* ??
pub const VORTEX_STAT: c_uint = 0x2a008	/* Some sort of status */;
pub const STAT_IRQ: c_uint = 0x00000001	/* This bitis set if the IRQ is valid. */;
pub const VORTEX_CTRL: c_uint = 0x2a00c;
pub const CTRL_MIDI_EN: c_uint = 0x00000001;
pub const CTRL_MIDI_PORT: c_uint = 0x00000060;
pub const CTRL_GAME_EN: c_uint = 0x00000008;
pub const CTRL_GAME_PORT: c_uint = 0x00000e00;
pub const CTRL_IRQ_ENABLE: c_uint = 0x00004000;
pub const CTRL_SPDIF: c_uint = 0x00000000	/* unknown. Please find this value */;
pub const CTRL_SPORT: c_uint = 0x00200000;
pub const CTRL_RST: c_uint = 0x00800000;
pub const CTRL_UNKNOWN: c_uint = 0x01000000;
// write: Timer period config / read: TIMER IRQ ack.
pub const VORTEX_IRQ_STAT: c_uint = 0x2919c;
// MIDI *//* GAME.
pub const VORTEX_MIDI_DATA: c_uint = 0x28800;
pub const VORTEX_MIDI_CMD: c_uint = 0x28804	/* Write command / Read status */;
pub const VORTEX_GAME_LEGACY: c_uint = 0x28808;
pub const VORTEX_CTRL2: c_uint = 0x2880c;
pub const CTRL2_GAME_ADCMODE: c_uint = 0x40;
pub const VORTEX_GAME_AXIS: c_uint = 0x28810	/* Axis base register. 4 axis's */;
pub const AXIS_SIZE: c_int = 4;
pub const AXIS_RANGE: c_uint = 0x1fff;
