//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/au88x0/au8820.h
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
// Macro flag: #define CHIP_AU8820

// Number of ADB and WT channels
pub const NR_ADB: c_uint = 0x10;
pub const NR_WT: c_uint = 0x20;
pub const NR_SRC: c_uint = 0x10;
pub const NR_A3D: c_uint = 0x00;
pub const NR_MIXIN: c_uint = 0x10;
pub const NR_MIXOUT: c_uint = 0x10;
// ADBDMA
pub const VORTEX_ADBDMA_STAT: c_uint = 0x105c0	/* read only, subbuffer, DMA pos */;
pub const POS_MASK: c_uint = 0x00000fff;
pub const POS_SHIFT: c_uint = 0x0;
pub const ADB_SUBBUF_MASK: c_uint = 0x00003000	/* ADB only. */;
pub const ADB_SUBBUF_SHIFT: c_uint = 0xc	/* ADB only. */;
pub const VORTEX_ADBDMA_CTRL: c_uint = 0x10580	/* write only, format, flags, DMA pos */;
pub const OFFSET_MASK: c_uint = 0x00000fff;
pub const OFFSET_SHIFT: c_uint = 0x0;
pub const IE_MASK: c_uint = 0x00001000	/* interrupt enable. */;
pub const IE_SHIFT: c_uint = 0xc;
pub const DIR_MASK: c_uint = 0x00002000	/* Direction. */;
pub const DIR_SHIFT: c_uint = 0xd;
pub const FMT_MASK: c_uint = 0x0003c000;
pub const FMT_SHIFT: c_uint = 0xe;
// The masks and shift also work for the wtdma, if not specified otherwise.
pub const VORTEX_ADBDMA_BUFCFG0: c_uint = 0x10400;
pub const VORTEX_ADBDMA_BUFCFG1: c_uint = 0x10404;
pub const VORTEX_ADBDMA_BUFBASE: c_uint = 0x10200;
pub const VORTEX_ADBDMA_START: c_uint = 0x106c0	/* Which subbuffer starts */;
pub const VORTEX_ADBDMA_STATUS: c_uint = 0x10600	/* stored at AdbDma->this_10 / 2 DWORD in size. */;
// ADB
pub const VORTEX_ADB_SR: c_uint = 0x10a00	/* Samplerates enable/disable */;
pub const VORTEX_ADB_RTBASE: c_uint = 0x10800;
pub const VORTEX_ADB_RTBASE_COUNT: c_int = 103;
pub const VORTEX_ADB_CHNBASE: c_uint = 0x1099c;
pub const VORTEX_ADB_CHNBASE_COUNT: c_int = 22;
pub const ROUTE_MASK: c_uint = 0x3fff;
pub const ADB_MASK: c_uint = 0x7f;
pub const ADB_SHIFT: c_uint = 0x7;
// #define     ADB_MIX_MASK 0xf
// ADB address
pub const OFFSET_ADBDMA: c_uint = 0x00;
pub const OFFSET_SRCOUT: c_uint = 0x10	/* on channel 0x11 */;
pub const OFFSET_SRCIN: c_uint = 0x10	/* on channel < 0x11 */;
pub const OFFSET_MIXOUT: c_uint = 0x20	/* source */;
pub const OFFSET_MIXIN: c_uint = 0x30	/* sink */;
pub const OFFSET_CODECIN: c_uint = 0x48	/* ADB source */;
pub const OFFSET_CODECOUT: c_uint = 0x58	/* ADB sink/target */;
pub const OFFSET_SPORTOUT: c_uint = 0x60	/* sink */;
pub const OFFSET_SPORTIN: c_uint = 0x50	/* source */;
pub const OFFSET_EFXOUT: c_uint = 0x50	/* sink */;
pub const OFFSET_EFXIN: c_uint = 0x40	/* source */;
pub const OFFSET_A3DOUT: c_uint = 0x00	/* This card has no HRTF :( */;
pub const OFFSET_A3DIN: c_uint = 0x00;
pub const OFFSET_WTOUT: c_uint = 0x58	/*  */;
// ADB route translate helper

// WTDMA
pub const VORTEX_WTDMA_CTRL: c_uint = 0x10500	/* format, DMA pos */;
pub const VORTEX_WTDMA_STAT: c_uint = 0x10500	/* DMA subbuf, DMA pos */;

pub const WT_SUBBUF_SHIFT: c_uint = 0x15;
pub const VORTEX_WTDMA_BUFBASE: c_uint = 0x10000;
pub const VORTEX_WTDMA_BUFCFG0: c_uint = 0x10300;
pub const VORTEX_WTDMA_BUFCFG1: c_uint = 0x10304;
pub const VORTEX_WTDMA_START: c_uint = 0x10640	/* which subbuffer is first */;
pub const VORTEX_WT_BASE: c_uint = 0x9000;
// MIXER
pub const VORTEX_MIXER_SR: c_uint = 0x9f00;
pub const VORTEX_MIXER_CLIP: c_uint = 0x9f80;
pub const VORTEX_MIXER_CHNBASE: c_uint = 0x9e40;
pub const VORTEX_MIXER_RTBASE: c_uint = 0x9e00;
pub const MIXER_RTBASE_SIZE: c_uint = 0x26;
pub const VORTEX_MIX_ENIN: c_uint = 0x9a00	/* Input enable bits. 4 bits wide. */;
pub const VORTEX_MIX_SMP: c_uint = 0x9c00;
// MIX
pub const VORTEX_MIX_INVOL_A: c_uint = 0x9000	/* in? */;
pub const VORTEX_MIX_INVOL_B: c_uint = 0x8000	/* out? */;
pub const VORTEX_MIX_VOL_A: c_uint = 0x9800;
pub const VORTEX_MIX_VOL_B: c_uint = 0x8800;
pub const VOL_MIN: c_uint = 0x80	/* Input volume when muted. */;
pub const VOL_MAX: c_uint = 0x7f	/* FIXME: Not confirmed! Just guessed. */;
// #define MIX_OUTL    0xe
// #define MIX_OUTR    0xf
// #define MIX_INL     0xe
// #define MIX_INR     0xf
pub const MIX_DEFIGAIN: c_uint = 0x08	/* 0x8 => 6dB */;
pub const MIX_DEFOGAIN: c_uint = 0x08;
// SRC
pub const VORTEX_SRCBLOCK_SR: c_uint = 0xccc0;
pub const VORTEX_SRC_CHNBASE: c_uint = 0xcc40;
pub const VORTEX_SRC_RTBASE: c_uint = 0xcc00;
pub const VORTEX_SRC_SOURCE: c_uint = 0xccc4;
pub const VORTEX_SRC_SOURCESIZE: c_uint = 0xccc8;
pub const VORTEX_SRC_U0: c_uint = 0xce00;
pub const VORTEX_SRC_DRIFT0: c_uint = 0xce80;
pub const VORTEX_SRC_DRIFT1: c_uint = 0xcec0;
pub const VORTEX_SRC_U1: c_uint = 0xcf00;
pub const VORTEX_SRC_DRIFT2: c_uint = 0xcf40;
pub const VORTEX_SRC_U2: c_uint = 0xcf80;
pub const VORTEX_SRC_DATA: c_uint = 0xc800;
pub const VORTEX_SRC_DATA0: c_uint = 0xc000;
pub const VORTEX_SRC_CONVRATIO: c_uint = 0xce40;
// #define     SRC_RATIO(x) ((((x<<15)/48000) + 1)/2) /* Playback
// #define     SRC_RATIO2(x) ((((48000<<15)/x) + 1)/2) /* Recording
// FIFO
pub const VORTEX_FIFO_ADBCTRL: c_uint = 0xf800	/* Control bits. */;
pub const VORTEX_FIFO_WTCTRL: c_uint = 0xf840;
pub const FIFO_RDONLY: c_uint = 0x00000001;
pub const FIFO_CTRL: c_uint = 0x00000002	/* Allow ctrl. ? */;
pub const FIFO_VALID: c_uint = 0x00000010;
pub const FIFO_EMPTY: c_uint = 0x00000020;
pub const FIFO_U0: c_uint = 0x00001000	/* Unknown. */;
pub const FIFO_U1: c_uint = 0x00010000;
pub const FIFO_SIZE_BITS: c_int = 5;

pub const VORTEX_FIFO_ADBDATA: c_uint = 0xe000;
pub const VORTEX_FIFO_WTDATA: c_uint = 0xe800;
// CODEC
pub const VORTEX_CODEC_CTRL: c_uint = 0x11984;
pub const VORTEX_CODEC_EN: c_uint = 0x11990;
pub const EN_CODEC: c_uint = 0x00000300;
pub const EN_SPORT: c_uint = 0x00030000;
pub const EN_SPDIF: c_uint = 0x000c0000;
pub const VORTEX_CODEC_CHN: c_uint = 0x11880;
pub const VORTEX_CODEC_IO: c_uint = 0x11988;
pub const VORTEX_SPDIF_FLAGS: c_uint = 0x1005c	/* FIXME */;
pub const VORTEX_SPDIF_CFG0: c_uint = 0x119D0;
pub const VORTEX_SPDIF_CFG1: c_uint = 0x119D4;
pub const VORTEX_SPDIF_SMPRATE: c_uint = 0x11994;
// Sample timer
pub const VORTEX_SMP_TIME: c_uint = 0x11998;
// IRQ
pub const VORTEX_IRQ_SOURCE: c_uint = 0x12800	/* Interrupt source flags. */;
pub const VORTEX_IRQ_CTRL: c_uint = 0x12804	/* Interrupt source mask. */;
pub const VORTEX_STAT: c_uint = 0x12808	/* ?? */;
pub const VORTEX_CTRL: c_uint = 0x1280c;
pub const CTRL_MIDI_EN: c_uint = 0x00000001;
pub const CTRL_MIDI_PORT: c_uint = 0x00000060;
pub const CTRL_GAME_EN: c_uint = 0x00000008;
pub const CTRL_GAME_PORT: c_uint = 0x00000e00;
pub const CTRL_IRQ_ENABLE: c_uint = 0x4000;
// write: Timer period config / read: TIMER IRQ ack.
pub const VORTEX_IRQ_STAT: c_uint = 0x1199c;
// DMA
pub const VORTEX_DMA_BUFFER: c_uint = 0x10200;
pub const VORTEX_ENGINE_CTRL: c_uint = 0x1060c;
pub const ENGINE_INIT: c_uint = 0x0L;
// MIDI *//* GAME.
pub const VORTEX_MIDI_DATA: c_uint = 0x11000;
pub const VORTEX_MIDI_CMD: c_uint = 0x11004	/* Write command / Read status */;
pub const VORTEX_GAME_LEGACY: c_uint = 0x11008;
pub const VORTEX_CTRL2: c_uint = 0x1100c;
pub const CTRL2_GAME_ADCMODE: c_uint = 0x40;
pub const VORTEX_GAME_AXIS: c_uint = 0x11010;
pub const AXIS_SIZE: c_int = 4;
pub const AXIS_RANGE: c_uint = 0x1fff;
