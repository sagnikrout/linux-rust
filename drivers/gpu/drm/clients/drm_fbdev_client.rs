//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/clients/drm_fbdev_client.c
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

//
// struct drm_client_funcs
//
#[no_mangle]
unsafe extern "C" fn drm_fbdev_client_free(client: *mut drm_client_dev) {
    static void drm_fbdev_client_free(struct drm_client_dev *client)
    {
    struct drm_fb_helper *fb_helper = drm_fb_helper_from_client(client);
    drm_fb_helper_unprepare(fb_helper);
    kfree(fb_helper);
    }
#[no_mangle]
unsafe extern "C" fn drm_fbdev_client_unregister(client: *mut drm_client_dev) {
    static void drm_fbdev_client_unregister(struct drm_client_dev *client)
    {
    struct drm_fb_helper *fb_helper = drm_fb_helper_from_client(client);
    if (fb_helper.info) {
//
// Fully probed framebuffer device
//
    drm_fb_helper_unregister_info(fb_helper);
    } else {
//
// Partially initialized client, no framebuffer device yet
//
    drm_client_release(&fb_helper.client);
    }
    }
#[no_mangle]
unsafe extern "C" fn drm_fbdev_client_restore(client: *mut drm_client_dev, force: bool) -> c_int {
    static int drm_fbdev_client_restore(struct drm_client_dev *client, bool force)
    {
    struct drm_fb_helper *fb_helper = drm_fb_helper_from_client(client);
    drm_fb_helper_restore_fbdev_mode_unlocked(fb_helper, force);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn drm_fbdev_client_hotplug(client: *mut drm_client_dev) -> c_int {
    static int drm_fbdev_client_hotplug(struct drm_client_dev *client)
    {
    struct drm_fb_helper *fb_helper = drm_fb_helper_from_client(client);
    struct drm_device *dev = client.dev;
    int ret;
    if (dev.fb_helper)
    return drm_fb_helper_hotplug_event(dev.fb_helper);
    ret = drm_fb_helper_init(dev, fb_helper);
    if (ret)
    goto err_drm_err;
    if (!drm_drv_uses_atomic_modeset(dev))
    drm_helper_disable_unused_functions(dev);
    ret = drm_fb_helper_initial_config(fb_helper);
    if (ret)
    goto err_drm_fb_helper_fini;
    return 0;
    err_drm_fb_helper_fini:
    drm_fb_helper_fini(fb_helper);
    err_drm_err:
    drm_err(dev, "fbdev: Failed to setup emulation (ret=%d)\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn drm_fbdev_client_suspend(client: *mut drm_client_dev) -> c_int {
    static int drm_fbdev_client_suspend(struct drm_client_dev *client)
    {
    struct drm_fb_helper *fb_helper = drm_fb_helper_from_client(client);
    drm_fb_helper_set_suspend_unlocked(fb_helper, true);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn drm_fbdev_client_resume(client: *mut drm_client_dev) -> c_int {
    static int drm_fbdev_client_resume(struct drm_client_dev *client)
    {
    struct drm_fb_helper *fb_helper = drm_fb_helper_from_client(client);
    drm_fb_helper_set_suspend_unlocked(fb_helper, false);
    return 0;
    }
    static const struct drm_client_funcs drm_fbdev_client_funcs = {
    .owner		= THIS_MODULE,
    .free		= drm_fbdev_client_free,
    .unregister	= drm_fbdev_client_unregister,
    .restore	= drm_fbdev_client_restore,
    .hotplug	= drm_fbdev_client_hotplug,
    .suspend	= drm_fbdev_client_suspend,
    .resume		= drm_fbdev_client_resume,
    };
//
// drm_fbdev_client_setup() - Setup fbdev emulation
// @dev: DRM device
// @format: Preferred color format for the device. DRM_FORMAT_XRGB8888
// is used if this is zero.
//
// This function sets up fbdev emulation. Restore, hotplug events and
// teardown are all taken care of. Drivers that do suspend/resume need
// to call drm_client_dev_suspend() and drm_client_dev_resume() by
// themselves. Simple drivers might use drm_mode_config_helper_suspend().
//
// This function is safe to call even when there are no connectors present.
// Setup will be retried on the next hotplug event.
//
// The fbdev client is destroyed by drm_dev_unregister().
//
// Returns:
// 0 on success, or a negative errno code otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn drm_fbdev_client_setup(dev: *mut drm_device, format: *const drm_format_info) -> c_int {
    int drm_fbdev_client_setup(struct drm_device *dev, const struct drm_format_info *format)
    {
    struct drm_fb_helper *fb_helper;
    unsigned int color_mode;
    int ret;
// TODO: Use format info throughout DRM
    if (format) {
    let mut bpp: c_uint = drm_format_info_bpp(format, 0);
    switch (bpp) {
    case 16:
    color_mode = format.depth; // could also be 15
    break;
    default:
    color_mode = bpp;
    }
    } else {
    switch (dev.mode_config.preferred_depth) {
    case 0:
    case 24:
    color_mode = 32;
    break;
    default:
    color_mode = dev.mode_config.preferred_depth;
    }
    }
    drm_WARN(dev, !dev.registered, "Device has not been registered.\n");
    drm_WARN(dev, dev.fb_helper, "fb_helper is already set!\n");
    fb_helper = kzalloc_obj(*fb_helper);
    if (!fb_helper)
    return -ENOMEM;
    drm_fb_helper_prepare(dev, fb_helper, color_mode, core::ptr::null_mut());
    ret = drm_client_init(dev, &fb_helper.client, "fbdev", &drm_fbdev_client_funcs);
    if (ret) {
    drm_err(dev, "Failed to register client: %d\n", ret);
    goto err_drm_client_init;
    }
    drm_client_register(&fb_helper.client);
    return 0;
    err_drm_client_init:
    drm_fb_helper_unprepare(fb_helper);
    kfree(fb_helper);
    return ret;
    }
