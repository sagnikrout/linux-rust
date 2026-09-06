//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/asequencer.h
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
// Main header file for the ALSA sequencer
// Copyright (c) 1998-1999 by Frank van de Pol <fvdpol@coil.demon.nl>
// (c) 1998-1999 by Jaroslav Kysela <perex@perex.cz>
//

//
// type check macros
//
// result events: 0-4

// channel specific events: 5-19

// note events: 5-9

// control events: 10-19

// queue control events: 30-39

// system status messages

// sample messages

// user-defined messages

// fixed length events: 0-99

// variable length events: 130-139

// reserved for kernel

// direct dispatched events

//
// macros to check event flags
//
// prior events

// event length type

// time-stamp type

// time-mode type

// check whether the given event is a UMP event

// queue sync port

