//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvif/push006c.h
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


// Host methods
pub const PUSH006C_SUBC_NV06E: c_int = 0;
pub const PUSH006C_SUBC_NV176E: c_int = 0;
pub const PUSH006C_SUBC_NV826F: c_int = 0;
// ContextSurfaces2d
pub const PUSH006C_SUBC_NV042: c_int = 0;
pub const PUSH006C_SUBC_NV062: c_int = 0;
// ContextClipRectangle
pub const PUSH006C_SUBC_NV019: c_int = 0;
// ContextRop
pub const PUSH006C_SUBC_NV043: c_int = 0;
// ContextPattern
pub const PUSH006C_SUBC_NV044: c_int = 0;
// Misc dodginess...
pub const PUSH006C_SUBC_NV_SW: c_int = 1;
// ImageBlit
pub const PUSH006C_SUBC_NV05F: c_int = 2;
pub const PUSH006C_SUBC_NV09F: c_int = 2;
// GdiRectangleText
pub const PUSH006C_SUBC_NV04A: c_int = 3;
// Twod
pub const PUSH006C_SUBC_NV502D: c_int = 3;
// MemoryToMemoryFormat
pub const PUSH006C_SUBC_NV039: c_int = 4;
pub const PUSH006C_SUBC_NV5039: c_int = 4;
// DmaCopy
pub const PUSH006C_SUBC_NV85B5: c_int = 4;
// Cipher
pub const PUSH006C_SUBC_NV74C1: c_int = 4;

