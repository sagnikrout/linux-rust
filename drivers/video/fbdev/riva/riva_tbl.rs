//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/riva/riva_tbl.h
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


// \
//
// GPL licensing note -- nVidia is allowing a liberal interpretation of
// the documentation restriction above, to merely say that this nVidia's
// copyright and disclaimer should be included with all code derived
// from this source.  -- Jeff Garzik <jgarzik@pobox.com>, 01/Nov/99
//
// $XFree86: xc/programs/Xserver/hw/xfree86/drivers/nv/riva_tbl.h,v 1.9 2002/01/30 01:35:03 mvojkovi Exp $
//
// RIVA Fixed Functionality Init Tables.
//
// 0xXXXXX3XX For  MSB mono format
// 0xXXXXX2XX For  LSB mono format
// 0xXXXXX2XX For  MSB mono format
// 0xXXXXX3XX For  LSB mono format
// 0xXXXXX3XX For  MSB mono format
// 0xXXXXX2XX For  LSB mono format

// 0xXXXXXX01 For  MSB mono format
// 0xXXXXXX02 For  LSB mono format
// 0xXXXXXX01 For  MSB mono format
// 0xXXXXXX02 For  LSB mono format
// 0xXXXXXX01 For  MSB mono format
// 0xXXXXXX02 For  LSB mono format
// 0xXXXXXX01 For  MSB mono format
// 0xXXXXXX02 For  LSB mono format

// 0xXXXXXX01 For  MSB mono format
// 0xXXXXXX02 For  LSB mono format
// 0xXXXXXX01 For  MSB mono format
// 0xXXXXXX02 For  LSB mono format
// 0xXXXXXX01 For  MSB mono format
// 0xXXXXXX02 For  LSB mono format
// 0xXXXXXX01 For  MSB mono format
// 0xXXXXXX02 For  LSB mono format
