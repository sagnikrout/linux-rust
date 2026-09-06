//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvif/log.h
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
// SPDX-FileCopyrightText: Copyright (c) 2024 NVIDIA CORPORATION & AFFILIATES.

//
// nvif_log - structure for tracking logging buffers
// @entry: an entry in a list of struct nvif_logs
// @shutdown: pointer to function to call to clean up
//
// Structure used to track logging buffers so that they can be cleaned up
// when the module exits.
//
// The @shutdown function is called when the module exits. It should free all
// backing resources, such as logging buffers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_log {
    pub entry: list_head,
    pub log): *mut *mut void (shutdown)(struct nvif_log,
}

//
// nvif_logs - linked list of nvif_log objects
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_logs {
    pub head: list_head,
}

// shutdown() should also delete the log entry

