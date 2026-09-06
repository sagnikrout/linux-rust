//! Automatically rewritten from C Header to Rust Module
//! Source: samples/vfio-mdev/mdpy-defs.h
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
// Simple pci display device.
//
// Framebuffer memory is pci bar 0.
// Configuration (read-only) is in pci config space.
// Format field uses drm fourcc codes.
// ATM only DRM_FORMAT_XRGB8888 is supported.
//
// pci ids

pub const MDPY_PCI_DEVICE_ID: c_uint = 0x000f;

// pci cfg space offsets for fb config (dword)
pub const MDPY_VENDORCAP_OFFSET: c_uint = 0x40;
pub const MDPY_VENDORCAP_SIZE: c_uint = 0x10;

