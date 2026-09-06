//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/c2p_core.h
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
// Fast C2P (Chunky-to-Planar) Conversion
//
// Copyright (C) 2003-2008 Geert Uytterhoeven
//
// NOTES:
// - This code was inspired by Scout's C2P tutorial
// - It assumes to run on a big endian system
//
// This file is subject to the terms and conditions of the GNU General Public
// License. See the file COPYING in the main directory of this archive
// for more details.
//

//
// Basic transpose step
//
// Transpose operations on 8 32-bit words
//
// First n x 1 block
// Second n x 1 block
// Third n x 1 block
// Fourth n x 1 block
// First n x 2 block
// Second n x 2 block
// Single n x 4 block
//
// Transpose operations on 4 32-bit words
//
// First n x 1 block
// Second n x 1 block
// Single n x 2 block
//
// Transpose operations on 4 32-bit words (reverse order)
//
// Single n x 2 block
//
// Compose two values, using a bitmask as decision value
// This is equivalent to (a & mask) | (b & ~mask)
//
