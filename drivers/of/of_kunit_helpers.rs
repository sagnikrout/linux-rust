//! Automatically rewritten from C to Rust
//! Source: drivers/of/of_kunit_helpers.c
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
// Test managed DeviceTree APIs
//

//
// of_root_kunit_skip() - Skip test if the root node isn't populated
// @test: test to skip if the root node isn't populated
//
#[no_mangle]
pub unsafe extern "C" fn of_root_kunit_skip(test: *mut kunit) {
    void of_root_kunit_skip(struct kunit *test)
    {
    if ((IS_ENABLED(CONFIG_ARM64) || IS_ENABLED(CONFIG_RISCV)) &&
    IS_ENABLED(CONFIG_ACPI) && !of_root)
    kunit_skip(test, "arm64/riscv+acpi doesn't populate a root node");
    }
    EXPORT_SYMBOL_GPL(of_root_kunit_skip);

#[no_mangle]
unsafe extern "C" fn of_overlay_fdt_apply_kunit_exit(ovcs_id: *mut c_void) {
    static void of_overlay_fdt_apply_kunit_exit(void *ovcs_id)
    {
    of_overlay_remove(ovcs_id);
    }
//
// of_overlay_fdt_apply_kunit() - Test managed of_overlay_fdt_apply()
// @test: test context
// @overlay_fdt: device tree overlay to apply
// @overlay_fdt_size: size in bytes of @overlay_fdt
// @ovcs_id: identifier of overlay, used to remove the overlay
//
// Just like of_overlay_fdt_apply(), except the overlay is managed by the test
// case and is automatically removed with of_overlay_remove() after the test
// case concludes.
//
// Return: 0 on success, negative errno on failure
//
    int of_overlay_fdt_apply_kunit(struct kunit *test, void *overlay_fdt,
    u32 overlay_fdt_size, int *ovcs_id)
    {
    int ret;
    int *copy_id;
    of_root_kunit_skip(test);
    copy_id = kunit_kmalloc(test, sizeof(*copy_id), GFP_KERNEL);
    if (!copy_id)
    return -ENOMEM;
    ret = of_overlay_fdt_apply(overlay_fdt, overlay_fdt_size,
    ovcs_id, core::ptr::null_mut());
    if (ret)
    return ret;
// copy_id = *ovcs_id;
    return kunit_add_action_or_reset(test, of_overlay_fdt_apply_kunit_exit,
    copy_id);
    }
    EXPORT_SYMBOL_GPL(of_overlay_fdt_apply_kunit);

    KUNIT_DEFINE_ACTION_WRAPPER(of_node_put_wrapper, of_node_put, struct device_node *);
//
// of_node_put_kunit() - Test managed of_node_put()
// @test: test context
// @node: node to pass to `of_node_put()`
//
// Just like of_node_put(), except the node is managed by the test case and is
// automatically put with of_node_put() after the test case concludes.
//
#[no_mangle]
pub unsafe extern "C" fn of_node_put_kunit(test: *mut kunit, node: *mut device_node) {
    void of_node_put_kunit(struct kunit *test, struct device_node *node)
    {
    if (kunit_add_action(test, of_node_put_wrapper, node)) {
    KUNIT_FAIL(test,
    "Can't allocate a kunit resource to put of_node\n");
    }
    }
    EXPORT_SYMBOL_GPL(of_node_put_kunit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Test managed DeviceTree APIs");
