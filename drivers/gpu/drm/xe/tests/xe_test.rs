//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/tests/xe_test.h
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


// SPDX-License-Identifier: GPL-2.0 AND MIT
//
// Copyright © 2022 Intel Corporation
//

//
// Each test that provides a kunit private test structure, place a test id
// here and point the kunit->priv to an embedded struct xe_test_priv.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_test_priv_id {
    XE_TEST_LIVE_DMA_BUF,
    XE_TEST_LIVE_MIGRATE,
}

//
// struct xe_test_priv - Base class for test private info
// @id: enum xe_test_priv_id to identify the subclass.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_test_priv {
    pub id: xe_test_priv_id,
}

//
// xe_cur_kunit_priv - Obtain the struct xe_test_priv pointed to by
// current->kunit->priv if it exists and is embedded in the expected subclass.
// @id: Id of the expected subclass.
//
// Return: NULL if the process is not a kunit test, and NULL if the
// current kunit->priv pointer is not pointing to an object of the expected
// subclass. A pointer to the embedded struct xe_test_priv otherwise.
//

// Macro flag: #define XE_TEST_DECLARE(x)
pub const XE_TEST_ONLY(x): c_int = 0;

