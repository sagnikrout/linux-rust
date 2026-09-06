//! Automatically rewritten from C Header to Rust Module
//! Source: sound/ppc/snd_ps3_reg.h
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
// Audio support for PS3
// Copyright (C) 2007 Sony Computer Entertainment Inc.
// Copyright 2006, 2007 Sony Corporation
// All rights reserved.
//
// interrupt / configure registers
//

//
// DMAC registers
// n:0..9
//

//
// mute control
//

//
// three wire serial
// n:0..3
//

//
// S/PDIF
// n:0..1
// x:0..11
// y:0..5
//

//

//
// The CLEAR field cancels all pending transfers, and stops any running DMA
//

//
// 3 Wire Audio Serial Output Channel Mutes (0..3)

// S/PDIF mutes (0,1)

// All 3 Wire Serial Outputs Mute

// All S/PDIF Mute

// All Audio Outputs Mute

//

//

//

//

//

//

//

// S/PDIF Output Channel Buffer Underflow Interrupt Enables

// S/PDIF Output Channel One Block Transfer Completion Interrupt Enables

// 3-Wire Audio Serial Output Channel Buffer Empty Interrupt Enables

// S/PDIF Output Channel Buffer Empty Interrupt Enables

//

//

//

//

//

//

// LRCK Output Disable

// Bit Clock Output Disable

//

//

//

//

//

//

//

//

//

//

//

//

//

//

//

//
// DMAC register
//

//
// The EVENT field is used to set the event in which
// the DMA request becomes active.
//

//

//

//

//

//

//

//
// source/destination address for internal fifos
//

//
// field attiribute
//
// Read
// ' ' = Other Information
// '-' = Field is part of a write-only register
// 'C' = Value read is always the same, constant value line follows (C)
// 'R' = Value is read
//
// Write
// ' ' = Other Information
// '-' = Must not be written (D), value ignored when written (R,A,F)
// 'W' = Can be written
//
// Internal State
// ' ' = Other Information
// '-' = No internal state
// 'X' = Internal state, initial value is unknown
// 'I' = Internal state, initial value is known and follows (I)
//
// Declaration/Size
// ' ' = Other Information
// '-' = Does Not Apply
// 'V' = Type is void
// 'U' = Type is unsigned integer
// 'S' = Type is signed integer
// 'F' = Type is IEEE floating point
// '1' = Byte size (008)
// '2' = Short size (016)
// '3' = Three byte size (024)
// '4' = Word size (032)
// '8' = Double size (064)
//
// Define Indicator
// ' ' = Other Information
// 'D' = Device
// 'M' = Memory
// 'R' = Register
// 'A' = Array of Registers
// 'F' = Field
// 'V' = Value
// 'T' = Task
//
