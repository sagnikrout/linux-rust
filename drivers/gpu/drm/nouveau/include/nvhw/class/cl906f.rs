//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvhw/class/cl906f.h
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

// Macro flag: #define _cl906f_h_
// fields and values

pub const NV906F_SEMAPHORED_OPERATION_ACQUIRE: c_uint = 0x00000001;
pub const NV906F_SEMAPHORED_OPERATION_RELEASE: c_uint = 0x00000002;
pub const NV906F_SEMAPHORED_OPERATION_ACQ_GEQ: c_uint = 0x00000004;
pub const NV906F_SEMAPHORED_OPERATION_ACQ_AND: c_uint = 0x00000008;

pub const NV906F_SEMAPHORED_ACQUIRE_SWITCH_DISABLED: c_uint = 0x00000000;
pub const NV906F_SEMAPHORED_ACQUIRE_SWITCH_ENABLED: c_uint = 0x00000001;

pub const NV906F_SEMAPHORED_RELEASE_WFI_EN: c_uint = 0x00000000;
pub const NV906F_SEMAPHORED_RELEASE_WFI_DIS: c_uint = 0x00000001;

pub const NV906F_SEMAPHORED_RELEASE_SIZE_16BYTE: c_uint = 0x00000000;
pub const NV906F_SEMAPHORED_RELEASE_SIZE_4BYTE: c_uint = 0x00000001;

// dma method formats

