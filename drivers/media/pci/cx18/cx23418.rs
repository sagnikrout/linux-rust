//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/cx18/cx23418.h
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
// cx18 header containing common defines.
//
// Copyright (C) 2007  Hans Verkuil <hverkuil@kernel.org>
//

pub const MGR_CMD_MASK: c_uint = 0x40000000;
// The MSB of the command code indicates that this is the completion of a

// Description: This command creates a new instance of a certain task

// Description: This command destroys an instance of a task

// All commands for CPU have the following mask set
pub const CPU_CMD_MASK: c_uint = 0x20000000;

pub const EPU_CMD_MASK: c_uint = 0x02000000;

pub const APU_CMD_MASK: c_uint = 0x10000000;

// Description: Command APU to start audio

// Description: Command APU to stop audio

// Description: Command APU to reset the AI

// Description: This command indicates that a Memory Descriptor List has been

// Something interesting happened

// Reads memory/registers (32-bit)

// Description: This command starts streaming with the set channel type

// Description: This command stops streaming with the set channel type

// Description: This command pauses streaming with the set channel type

// Description: This command resumes streaming with the set channel type

pub const CAPTURE_CHANNEL_TYPE_NONE: c_int = 0;
pub const CAPTURE_CHANNEL_TYPE_MPEG: c_int = 1;
pub const CAPTURE_CHANNEL_TYPE_INDEX: c_int = 2;
pub const CAPTURE_CHANNEL_TYPE_YUV: c_int = 3;
pub const CAPTURE_CHANNEL_TYPE_PCM: c_int = 4;
pub const CAPTURE_CHANNEL_TYPE_VBI: c_int = 5;
pub const CAPTURE_CHANNEL_TYPE_SLICED_VBI: c_int = 6;
pub const CAPTURE_CHANNEL_TYPE_TS: c_int = 7;
pub const CAPTURE_CHANNEL_TYPE_MAX: c_int = 15;
// Description: This command sets the channel type. This can only be done

// Description: Set stream output type

// Description: Set video input resolution and frame rate

// Description: Set video frame rate

// Description: Set video output resolution

// Description: This command set filter parameters

// Description: This command set spatial filter type

// Description: This command set coring levels for median filter

// Description: This command set the picture type mask for index file

// Description: Set audio parameters

// Description: Set video mute

// Description: Set audio mute

// Description: Set stream output type

// Description: Set raw VBI parameters

// Description: Set capture line No.

// Description: Set copyright

// Description: Set audio PID

// Description: Set video PID

// Description: Set Vertical Crop Line

// Description: Set COP structure

// Description: Set Scene Change Detection

// Description: Set Aspect Ratio

// Description: Set Skip Input Frame

// Description: Set sliced VBI parameters -

// Description: Set the user data place holder

// Description:

// Description: Set VFC parameters
//

// Below is the list of commands related to the data exchange

// Description: This command provides the physical base address of the local

// Description: This command provides the offsets in the device memory where

// Description: This command provides the offset to a Memory Descriptor List

// Description: This command requests return of all current Memory

// Description: This command signals the cpu that the dat buffer has been
// #define CX18_CPU_DE_RELEASE_BUFFER           (CPU_CMD_MASK_DE | 0x0007)
// No Error / Success
pub const CNXT_OK: c_uint = 0x000000;
// Received unknown command
pub const CXERR_UNK_CMD: c_uint = 0x000001;
// First parameter in the command is invalid
pub const CXERR_INVALID_PARAM1: c_uint = 0x000002;
// Second parameter in the command is invalid
pub const CXERR_INVALID_PARAM2: c_uint = 0x000003;
// Device interface is not open/found
pub const CXERR_DEV_NOT_FOUND: c_uint = 0x000004;
// Requested function is not implemented/available
pub const CXERR_NOTSUPPORTED: c_uint = 0x000005;
// Invalid pointer is provided
pub const CXERR_BADPTR: c_uint = 0x000006;
// Unable to allocate memory
pub const CXERR_NOMEM: c_uint = 0x000007;
// Object/Link not found
pub const CXERR_LINK: c_uint = 0x000008;
// Device busy, command cannot be executed
pub const CXERR_BUSY: c_uint = 0x000009;
// File/device/handle is not open.
pub const CXERR_NOT_OPEN: c_uint = 0x00000A;
// Value is out of range
pub const CXERR_OUTOFRANGE: c_uint = 0x00000B;
// Buffer overflow
pub const CXERR_OVERFLOW: c_uint = 0x00000C;
// Version mismatch
pub const CXERR_BADVER: c_uint = 0x00000D;
// Operation timed out
pub const CXERR_TIMEOUT: c_uint = 0x00000E;
// Operation aborted
pub const CXERR_ABORT: c_uint = 0x00000F;
// Specified I2C device not found for read/write
pub const CXERR_I2CDEV_NOTFOUND: c_uint = 0x000010;
// Error in I2C data xfer (but I2C device is present)
pub const CXERR_I2CDEV_XFERERR: c_uint = 0x000011;
// Channel changing component not ready
pub const CXERR_CHANNELNOTREADY: c_uint = 0x000012;
// PPU (Presensation/Decoder) mail box is corrupted
pub const CXERR_PPU_MB_CORRUPT: c_uint = 0x000013;
// CPU (Capture/Encoder) mail box is corrupted
pub const CXERR_CPU_MB_CORRUPT: c_uint = 0x000014;
// APU (Audio) mail box is corrupted
pub const CXERR_APU_MB_CORRUPT: c_uint = 0x000015;
// Unable to open file for reading
pub const CXERR_FILE_OPEN_READ: c_uint = 0x000016;
// Unable to open file for writing
pub const CXERR_FILE_OPEN_WRITE: c_uint = 0x000017;
// Unable to find the I2C section specified
pub const CXERR_I2C_BADSECTION: c_uint = 0x000018;
// Error in I2C data xfer (but I2C device is present)
pub const CXERR_I2CDEV_DATALOW: c_uint = 0x000019;
// Error in I2C data xfer (but I2C device is present)
pub const CXERR_I2CDEV_CLOCKLOW: c_uint = 0x00001A;
// No Interrupt received from HW (for I2C access)
pub const CXERR_NO_HW_I2C_INTR: c_uint = 0x00001B;
// RPU is not ready to accept commands!
pub const CXERR_RPU_NOT_READY: c_uint = 0x00001C;
// RPU is not ready to accept commands!
pub const CXERR_RPU_NO_ACK: c_uint = 0x00001D;
// The are no buffers ready. Try again soon!
pub const CXERR_NODATA_AGAIN: c_uint = 0x00001E;
// The stream is stopping. Function not allowed now!
pub const CXERR_STOPPING_STATUS: c_uint = 0x00001F;
// Trying to access hardware when the power is turned OFF
pub const CXERR_DEVPOWER_OFF: c_uint = 0x000020;
