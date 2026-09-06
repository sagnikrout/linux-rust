//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/display/drm_hdmi_cec_notifier_helper.c
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
// Copyright (c) 2024 Linaro Ltd
//

#[no_mangle]
unsafe extern "C" fn drm_connector_hdmi_cec_notifier_phys_addr_invalidate(connector: *mut drm_connector) {
    static void drm_connector_hdmi_cec_notifier_phys_addr_invalidate(struct drm_connector *connector)
    {
    cec_notifier_phys_addr_invalidate(connector.cec.data);
    }
    static void drm_connector_hdmi_cec_notifier_phys_addr_set(struct drm_connector *connector,
    u16 addr)
    {
    cec_notifier_set_phys_addr(connector.cec.data, addr);
    }
#[no_mangle]
unsafe extern "C" fn drm_connector_hdmi_cec_notifier_unregister(dev: *mut drm_device, res: *mut c_void) {
    static void drm_connector_hdmi_cec_notifier_unregister(struct drm_device *dev, void *res)
    {
    struct drm_connector *connector = res;
    cec_notifier_conn_unregister(connector.cec.data);
    connector.cec.data = core::ptr::null_mut();
    }
    static const struct drm_connector_cec_funcs drm_connector_cec_notifier_funcs = {
    .phys_addr_invalidate = drm_connector_hdmi_cec_notifier_phys_addr_invalidate,
    .phys_addr_set = drm_connector_hdmi_cec_notifier_phys_addr_set,
    };
    int drmm_connector_hdmi_cec_notifier_register(struct drm_connector *connector,
    const char *port_name,
    struct device *dev)
    {
    struct cec_connector_info conn_info;
    struct cec_notifier *notifier;
    cec_fill_conn_info_from_drm(&conn_info, connector);
    notifier = cec_notifier_conn_register(dev, port_name, &conn_info);
    if (!notifier)
    return -ENOMEM;
    mutex_lock(&connector.cec.mutex);
    connector.cec.data = notifier;
    connector.cec.funcs = &drm_connector_cec_notifier_funcs;
    mutex_unlock(&connector.cec.mutex);
    return drmm_add_action_or_reset(connector.dev,
    drm_connector_hdmi_cec_notifier_unregister,
    connector);
    }
    EXPORT_SYMBOL(drmm_connector_hdmi_cec_notifier_register);
