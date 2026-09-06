//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/clients/drm_client_setup.c
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

    static char drm_client_default[16] = CONFIG_DRM_CLIENT_DEFAULT;
    module_param_string(active, drm_client_default, sizeof(drm_client_default), 0444);
    MODULE_PARM_DESC(active,
    "Choose which drm client to start, default is "
    CONFIG_DRM_CLIENT_DEFAULT);
//
// drm_client_setup() - Setup in-kernel DRM clients
// @dev: DRM device
// @format: Preferred pixel format for the device. Use NULL, unless
// there is clearly a driver-preferred format.
//
// This function sets up the in-kernel DRM clients. Restore, hotplug
// events and teardown are all taken care of.
//
// Drivers should call drm_client_setup() after registering the new
// DRM device with drm_dev_register(). This function is safe to call
// even when there are no connectors present. Setup will be retried
// on the next hotplug event.
//
// The clients are destroyed by drm_dev_unregister().
//
#[no_mangle]
pub unsafe extern "C" fn drm_client_setup(dev: *mut drm_device, format: *const drm_format_info) {
    void drm_client_setup(struct drm_device *dev, const struct drm_format_info *format)
    {
    if (!drm_core_check_feature(dev, DRIVER_MODESET)) {
    drm_dbg(dev, "driver does not support mode-setting, skipping DRM clients\n");
    return;
    }

    if (!strcmp(drm_client_default, "fbdev")) {
    int ret;
    ret = drm_fbdev_client_setup(dev, format);
    if (ret)
    drm_warn(dev, "Failed to set up DRM client; error %d\n", ret);
    return;
    }

    if (!strcmp(drm_client_default, "log")) {
    drm_log_register(dev);
    return;
    }

    if (strcmp(drm_client_default, ""))
    drm_warn(dev, "Unknown DRM client %s\n", drm_client_default);
    }
    EXPORT_SYMBOL(drm_client_setup);
//
// drm_client_setup_with_fourcc() - Setup in-kernel DRM clients for color mode
// @dev: DRM device
// @fourcc: Preferred pixel format as 4CC code for the device
//
// This function sets up the in-kernel DRM clients. It is equivalent
// to drm_client_setup(), but expects a 4CC code as second argument.
//
#[no_mangle]
pub unsafe extern "C" fn drm_client_setup_with_fourcc(dev: *mut drm_device, fourcc: u32) {
    void drm_client_setup_with_fourcc(struct drm_device *dev, u32 fourcc)
    {
    drm_client_setup(dev, drm_format_info(fourcc));
    }
    EXPORT_SYMBOL(drm_client_setup_with_fourcc);
//
// drm_client_setup_with_color_mode() - Setup in-kernel DRM clients for color mode
// @dev: DRM device
// @color_mode: Preferred color mode for the device
//
// This function sets up the in-kernel DRM clients. It is equivalent
// to drm_client_setup(), but expects a color mode as second argument.
//
// Do not use this function in new drivers. Prefer drm_client_setup() with a
// format of NULL.
//
#[no_mangle]
pub unsafe extern "C" fn drm_client_setup_with_color_mode(dev: *mut drm_device, color_mode: c_uint) {
    void drm_client_setup_with_color_mode(struct drm_device *dev, unsigned int color_mode)
    {
    let mut fourcc: u32 = drm_driver_color_mode_format(dev, color_mode);
    drm_client_setup_with_fourcc(dev, fourcc);
    }
    EXPORT_SYMBOL(drm_client_setup_with_color_mode);
    MODULE_DESCRIPTION("In-kernel DRM clients");
    MODULE_LICENSE("GPL and additional rights");
