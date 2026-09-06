//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/dispnv50/crcc37d.h
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


// SPDX-License-Identifier: MIT

pub const CRCC37D_MAX_ENTRIES: c_int = 2047;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crcc37d_notifier {
    pub status: u32,
// reserved
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crcc37d_entry {
    pub status: [u32; 2],
    pub /: *mut *mut u32:32; / reserved,
    pub compositor_crc: u32,
    pub rg_crc: u32,
    pub output_crc: [u32; 2],
    pub /: *mut *mut u32:32; / reserved,
    pub entries: [}; CRCC37D_MAX_ENTRIES],
    pub __packed: },
    pub ctx): *mut *mut int crcc37d_set_ctx(struct nv50_head head, struct nv50_crc_notifier_ctx,
    pub idx): nv50_crc_source source, int,
    pub ctx): *mut *mut bool crcc37d_ctx_finished(struct nv50_head head, struct nv50_crc_notifier_ctx,
