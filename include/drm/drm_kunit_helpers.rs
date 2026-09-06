//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_kunit_helpers.h
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

extern "C" {
    pub fn drm_kunit_helper_free_device(test: *mut kunit, dev: *mut device);
}
//
// drm_kunit_helper_alloc_drm_device_with_driver - Allocates a mock DRM device for KUnit tests
// @_test: The test context object
// @_dev: The parent device object
// @_type: the type of the struct which contains struct &drm_device
// @_member: the name of the &drm_device within @_type.
// @_drv: Mocked DRM device driver features
//
// This function creates a struct &drm_device from @_dev and @_drv.
//
// @_dev should be allocated using drm_kunit_helper_alloc_device().
//
// The driver is tied to the @_test context and will get cleaned at the
// end of the test. The drm_device is allocated through
// devm_drm_dev_alloc() and will thus be freed through a device-managed
// resource.
//
// Returns:
// A pointer to the new drm_device, or an ERR_PTR() otherwise.
//

//
// drm_kunit_helper_alloc_drm_device - Allocates a mock DRM device for KUnit tests
// @_test: The test context object
// @_dev: The parent device object
// @_type: the type of the struct which contains struct &drm_device
// @_member: the name of the &drm_device within @_type.
// @_feat: Mocked DRM device driver features
//
// This function creates a struct &drm_driver and will create a struct
// &drm_device from @_dev and that driver.
//
// @_dev should be allocated using drm_kunit_helper_alloc_device().
//
// The driver is tied to the @_test context and will get cleaned at the
// end of the test. The drm_device is allocated through
// devm_drm_dev_alloc() and will thus be freed through a device-managed
// resource.
//
// Returns:
// A pointer to the new drm_device, or an ERR_PTR() otherwise.
//

