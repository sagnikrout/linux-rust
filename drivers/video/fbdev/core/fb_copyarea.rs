//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/core/fb_copyarea.h
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
// Generic bit area copy and twister engine for packed pixel framebuffers
//
// Rewritten by:
// Copyright (C)  2025 Zsolt Kajtar (soci@c64.rulez.org)
//
// Based on previous work of:
// Copyright (C)  1999-2005 James Simmons <jsimmons@www.infradead.org>
// Anton Vorontsov <avorontsov@ru.mvista.com>
// Pavel Pisa <pisa@cmp.felk.cvut.cz>
// Antonino Daplas <adaplas@hotpop.com>
// Geert Uytterhoeven
// and others
//
// NOTES:
//
// Handles native and foreign byte order on both endians, standard and
// reverse pixel order in a byte (<8 BPP), word length of 32/64 bits,
// bits per pixel from 1 to the word length. Handles line lengths at byte
// granularity while maintaining aligned accesses.
//
// Optimized routines for word aligned copying and byte aligned copying
// on reverse pixel framebuffers.
//

// used when no reversing is necessary
// modifies the masked area in a word
// copies the whole word
// forward aligned copy
// Same alignment for source and dest
// Single word
// Trailing bits
// Multiple destination words
// Leading bits
// Main chunk
// Trailing bits
// reverse aligned copy
// Single word
// Multiple destination words
// Trailing bits
// Main chunk
// Leading bits
// Single destination word
// Multiple destination words
// Leading bits
// 2 source words
// Main chunk
// Trailing bits
// Single destination word
// Multiple destination words
// 2 source words
// Trailing bits
// Main chunk
// Leading bits
