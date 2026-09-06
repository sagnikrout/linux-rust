//! Automatically rewritten from C Header to Rust Module
//! Source: include/kunit/of.h
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
    pub fn of_node_put_kunit(test: *mut kunit, node: *mut device_node);
}

//
// __of_overlay_apply_kunit() - Test managed of_overlay_fdt_apply() variant
// @test: test context
// @overlay_begin: start address of overlay to apply
// @overlay_end: end address of overlay to apply
//
// This is mostly internal API. See of_overlay_apply_kunit() for the wrapper
// that makes this easier to use.
//
// Similar to of_overlay_fdt_apply(), except the overlay is managed by the test
// case and is automatically removed with of_overlay_remove() after the test
// case concludes.
//
// Return: 0 on success, negative errno on failure
//

//
// of_overlay_apply_kunit() - Test managed of_overlay_fdt_apply() for built-in overlays
// @test: test context
// @overlay_name: name of overlay to apply
//
// This macro is used to apply a device tree overlay built with the
// cmd_dt_S_dtbo rule in scripts/Makefile.lib that has been compiled into the
// kernel image or KUnit test module. The overlay is automatically removed when
// the test is finished.
//
// Unit tests that need device tree nodes should compile an overlay file with
// @overlay_name\.dtbo.o in their Makefile along with their unit test and then
// load the overlay during their test. The @overlay_name matches the filename
// of the overlay without the dtbo filename extension. If CONFIG_OF_OVERLAY is
// not enabled, the @test will be skipped.
//
// In the Makefile
//
// .. code-block:: none
//
// obj-$(CONFIG_OF_OVERLAY_KUNIT_TEST) += overlay_test.o kunit_overlay_test.dtbo.o
//
// In the test
//
// .. code-block:: c
//
// static void of_overlay_kunit_of_overlay_apply(struct kunit *test)
// {
// struct device_node *np;
//
// KUNIT_ASSERT_EQ(test, 0,
// of_overlay_apply_kunit(test, kunit_overlay_test));
//
// np = of_find_node_by_name(NULL, "test-kunit");
// KUNIT_EXPECT_NOT_ERR_OR_NULL(test, np);
// of_node_put(np);
// }
//
// Return: 0 on success, negative errno on failure.
//

