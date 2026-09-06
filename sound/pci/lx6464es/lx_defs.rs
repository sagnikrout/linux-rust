//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/lx6464es/lx_defs.h
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
// -*- linux-c -*-
//
// ALSA driver for the digigram lx6464es interface
// adapted upstream headers
//
// Copyright (c) 2009 Tim Blechmann <tim@klingt.org>
//
// code adapted from ethersound.h
pub const XES_FREQ_COUNT8_MASK: c_uint = 0x00001FFF /* compteur 25MHz entre 8 ech. */;
pub const XES_FREQ_COUNT8_44_MIN: c_uint = 0x00001288 /* 25M /;
// [ 44k - ( 44.1k + 48k ) / 2 ]
// * 8
pub const XES_FREQ_COUNT8_44_MAX: c_uint = 0x000010F0 /* 25M / [ ( 44.1k + 48k ) / 2 ];
// * 8
pub const XES_FREQ_COUNT8_48_MAX: c_uint = 0x00000F08 /* 25M /;
// [ 48k + ( 44.1k + 48k ) / 2 ]
// * 8
// code adapted from LXES_registers.h

// ConfES register.

// ConfES register.

// ConfES register.
pub const FREQ_RATIO_SINGLE_MODE: c_uint = 0x01 /* value for single mode frequency ratio:;
// sample rate = frequency rate.
pub const CONFES_READ_PART_MASK: c_uint = 0x00070000;
pub const CONFES_WRITE_PART_MASK: c_uint = 0x00F80000;
// code adapted from if_drv_mb.h

// not yet pending

// XES

pub const MASK_SYS_TIMER_COUNT: c_uint = 0x0000FFFF;

// internal: reserved fo end
// of plx dma

// internal: pending XES
// IRQ

// management: notify driver
// instead of polling

pub const MICROBLAZE_IBL_MIN: c_int = 32;
pub const MICROBLAZE_IBL_DEFAULT: c_int = 128;
pub const MICROBLAZE_IBL_MAX: c_int = 512;
// #define MASK_GRANULARITY		(2*MICROBLAZE_IBL_MAX-1)
// command opcodes, see reference for details
//

// 1st command word.
pub const ID_CH_MASK: c_uint = 0x3F;

// command word.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cmd_mb_opcodes {
    CMD_00_INFO_DEBUG	        = 0x00,
    CMD_01_GET_SYS_CFG		= 0x01,
    CMD_02_SET_GRANULARITY		= 0x02,
    CMD_03_SET_TIMER_IRQ		= 0x03,
    CMD_04_GET_EVENT		= 0x04,
    CMD_05_GET_PIPES		= 0x05,

    CMD_06_ALLOCATE_PIPE            = 0x06,
    CMD_07_RELEASE_PIPE		= 0x07,
    CMD_08_ASK_BUFFERS		= 0x08,
    CMD_09_STOP_PIPE		= 0x09,
    CMD_0A_GET_PIPE_SPL_COUNT	= 0x0a,
    CMD_0B_TOGGLE_PIPE_STATE	= 0x0b,

    CMD_0C_DEF_STREAM		= 0x0c,
    CMD_0D_SET_MUTE			= 0x0d,
    CMD_0E_GET_STREAM_SPL_COUNT     = 0x0e,
    CMD_0F_UPDATE_BUFFER		= 0x0f,
    CMD_10_GET_BUFFER		= 0x10,
    CMD_11_CANCEL_BUFFER		= 0x11,
    CMD_12_GET_PEAK			= 0x12,
    CMD_13_SET_STREAM_STATE		= 0x13,
    CMD_14_INVALID			= 0x14,
}

// pipe states
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pipe_state_t {
    PSTATE_IDLE	= 0,	/* the pipe is not processed in the XES_IRQ
// (free or stopped, or paused).
    PSTATE_RUN	= 1,	/* sustained play/record state. */
    PSTATE_PURGE	= 2,	/* the ES channels are now off, render pipes do
// not DMA, record pipe do a last DMA.
    PSTATE_ACQUIRE	= 3,	/* the ES channels are now on, render pipes do
// not yet increase their sample count, record
// pipes do not DMA.
    PSTATE_CLOSING	= 4,	/* the pipe is releasing, and may not yet
// receive an "alloc" command.
}

// stream states
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stream_state_t {
    SSTATE_STOP	=  0x00,       /* setting to stop resets the stream spl
// count.
    SSTATE_RUN	= (0x01 << 0), /* start DMA and spl count handling. */
    SSTATE_PAUSE	= (0x01 << 1), /* pause DMA and spl count handling. */
}

// buffer flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum buffer_flags {
    BF_VALID	= 0x80,	/* set if the buffer is valid, clear if free.*/
    BF_CURRENT	= 0x40,	/* set if this is the current buffer (there is
// always a current buffer).
    BF_NOTIFY_EOB	= 0x20,	/* set if this buffer must cause a PCI event
// when finished.
    BF_CIRCULAR	= 0x10,	/* set if buffer[1] must be copied to buffer[0]
// by the end of this buffer.
    BF_64BITS_ADR	= 0x08,	/* set if the hi part of the address is valid.*/
    BF_xx		= 0x04,	/* future extension.*/
    BF_EOB		= 0x02,	/* set if finished, but not yet free.*/
    BF_PAUSE	= 0x01,	/* pause stream at buffer end.*/
    BF_ZERO		= 0x00,	/* no flags (init).*/
}

//
// Stream Flags definitions
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stream_flags {
    SF_ZERO		= 0x00000000, /* no flags (stream invalid). */
    SF_VALID	= 0x10000000, /* the stream has a valid DMA_conf
// info (setstreamformat).
    SF_XRUN		= 0x20000000, /* the stream is un x-run state. */
    SF_START	= 0x40000000, /* the DMA is running.*/
    SF_ASIO		= 0x80000000, /* ASIO.*/
}

pub const MASK_SPL_COUNT_HI: c_uint = 0x00FFFFFF /* 4 MSBits are status bits */;

// bit in the command word.
pub const STREAM_FMT_16b: c_uint = 0x02;
pub const STREAM_FMT_intel: c_uint = 0x01;

// word

// response word.
pub const MASK_DATA_SIZE: c_uint = 0x00FFFFFF /* this must match the field size of;
// datasize in the buffer_t structure.
pub const MASK_BUFFER_ID: c_uint = 0xFF /* the cancel command awaits a buffer ID,;
// may be 0xFF for "current".
// code adapted from PcxErr_e.h
// Bits masks
pub const ERROR_MASK: c_uint = 0x8000;
pub const SOURCE_MASK: c_uint = 0x7800;
pub const E_SOURCE_BOARD: c_uint = 0x4000 /* 8 >> 1 */;
pub const E_SOURCE_DRV: c_uint = 0x2000 /* 4 >> 1 */;
pub const E_SOURCE_API: c_uint = 0x1000 /* 2 >> 1 */;
// Error tools
pub const E_SOURCE_TOOLS: c_uint = 0x0800 /* 1 >> 1 */;
// Error pcxaudio
pub const E_SOURCE_AUDIO: c_uint = 0x1800 /* 3 >> 1 */;
// Error virtual pcx
pub const E_SOURCE_VPCX: c_uint = 0x2800 /* 5 >> 1 */;
// Error dispatcher
pub const E_SOURCE_DISPATCHER: c_uint = 0x3000 /* 6 >> 1 */;
// Error from CobraNet firmware
pub const E_SOURCE_COBRANET: c_uint = 0x3800 /* 7 >> 1 */;
pub const E_SOURCE_USER: c_uint = 0x7800;
pub const CLASS_MASK: c_uint = 0x0700;
pub const CODE_MASK: c_uint = 0x00FF;
// Bits values
// Values for the error/warning bit
pub const ERROR_VALUE: c_uint = 0x8000;
pub const WARNING_VALUE: c_uint = 0x0000;
// Class values
pub const E_CLASS_GENERAL: c_uint = 0x0000;
pub const E_CLASS_INVALID_CMD: c_uint = 0x0100;
pub const E_CLASS_INVALID_STD_OBJECT: c_uint = 0x0200;
pub const E_CLASS_RSRC_IMPOSSIBLE: c_uint = 0x0300;
pub const E_CLASS_WRONG_CONTEXT: c_uint = 0x0400;
pub const E_CLASS_BAD_SPECIFIC_PARAMETER: c_uint = 0x0500;
pub const E_CLASS_REAL_TIME_ERROR: c_uint = 0x0600;
pub const E_CLASS_DIRECTSHOW: c_uint = 0x0700;
pub const E_CLASS_FREE: c_uint = 0x0700;
// Complete DRV error code for the general class

// pour RCX
// (old 0x28)
//

// PCX (old 0x14)

// Complete DRV error code for real time class

// Complete BOARD error code for the invaid standard object class

// Complete BOARD error code for impossible resource allocation class

// Complete BOARD error code for wrong call context class

