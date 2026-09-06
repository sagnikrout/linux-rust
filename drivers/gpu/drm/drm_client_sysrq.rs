//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/drm_client_sysrq.c
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


// SPDX-License-Identifier: GPL-2.0 or MIT

    static LIST_HEAD(drm_client_sysrq_dev_list);
    static DEFINE_MUTEX(drm_client_sysrq_dev_lock);
// emergency restore, don't bother with error reporting
#[no_mangle]
unsafe extern "C" fn drm_client_sysrq_restore_work_fn(ignored: *mut work_struct) {
    static void drm_client_sysrq_restore_work_fn(struct work_struct *ignored)
    {
    struct drm_device *dev;
    guard(mutex)(&drm_client_sysrq_dev_lock);
    list_for_each_entry(dev, &drm_client_sysrq_dev_list, client_sysrq_list) {
    if (dev.switch_power_state == DRM_SWITCH_POWER_OFF)
    continue;
    drm_client_dev_restore(dev, true);
    }
    }
    static DECLARE_WORK(drm_client_sysrq_restore_work, drm_client_sysrq_restore_work_fn);
#[no_mangle]
unsafe extern "C" fn drm_client_sysrq_restore_handler(ignored: u8) {
    static void drm_client_sysrq_restore_handler(u8 ignored)
    {
    schedule_work(&drm_client_sysrq_restore_work);
    }
    static const struct sysrq_key_op drm_client_sysrq_restore_op = {
    .handler = drm_client_sysrq_restore_handler,
    .help_msg = "force-fb(v)",
    .action_msg = "Restore framebuffer console",
    };
#[no_mangle]
pub unsafe extern "C" fn drm_client_sysrq_register(dev: *mut drm_device) {
    void drm_client_sysrq_register(struct drm_device *dev)
    {
    guard(mutex)(&drm_client_sysrq_dev_lock);
    if (list_empty(&drm_client_sysrq_dev_list))
    register_sysrq_key('v', &drm_client_sysrq_restore_op);
    list_add(&dev.client_sysrq_list, &drm_client_sysrq_dev_list);
    }
#[no_mangle]
pub unsafe extern "C" fn drm_client_sysrq_unregister(dev: *mut drm_device) {
    void drm_client_sysrq_unregister(struct drm_device *dev)
    {
    guard(mutex)(&drm_client_sysrq_dev_lock);
// remove device from global restore list
    if (!drm_WARN_ON(dev, list_empty(&dev.client_sysrq_list)))
    list_del(&dev.client_sysrq_list);
// no devices left; unregister key
    if (list_empty(&drm_client_sysrq_dev_list))
    unregister_sysrq_key('v', &drm_client_sysrq_restore_op);
    }
