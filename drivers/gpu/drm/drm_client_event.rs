//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/drm_client_event.c
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
//
// Copyright 2018 Noralf Trønnes
//

//
// drm_client_dev_unregister - Unregister clients
// @dev: DRM device
//
// This function releases all clients by calling each client's
// &drm_client_funcs.unregister callback. The callback function
// is responsibe for releaseing all resources including the client
// itself.
//
// The helper drm_dev_unregister() calls this function. Drivers
// that use it don't need to call this function themselves.
//
#[no_mangle]
pub unsafe extern "C" fn drm_client_dev_unregister(dev: *mut drm_device) {
    void drm_client_dev_unregister(struct drm_device *dev)
    {
    struct drm_client_dev *client, *tmp;
    if (!drm_core_check_feature(dev, DRIVER_MODESET))
    return;
    mutex_lock(&dev.clientlist_mutex);
    list_for_each_entry_safe(client, tmp, &dev.clientlist, list) {
    list_del(&client.list);
//
// Unregistering consumes and frees the client.
//
    if (client.funcs && client.funcs.unregister)
    client.funcs.unregister(client);
    else
    drm_client_release(client);
    }
    mutex_unlock(&dev.clientlist_mutex);
    }
    EXPORT_SYMBOL(drm_client_dev_unregister);
#[no_mangle]
unsafe extern "C" fn drm_client_hotplug(client: *mut drm_client_dev) {
    static void drm_client_hotplug(struct drm_client_dev *client)
    {
    struct drm_device *dev = client.dev;
    int ret;
    if (!client.funcs || !client.funcs.hotplug)
    return;
    if (client.hotplug_failed)
    return;
    if (client.suspended) {
    client.hotplug_pending = true;
    return;
    }
    client.hotplug_pending = false;
    ret = client.funcs.hotplug(client);
    drm_dbg_kms(dev, "%s: ret=%d\n", client.name, ret);
    if (ret)
    client.hotplug_failed = true;
    }
//
// drm_client_dev_hotplug - Send hotplug event to clients
// @dev: DRM device
//
// This function calls the &drm_client_funcs.hotplug callback on the attached clients.
//
// drm_kms_helper_hotplug_event() calls this function, so drivers that use it
// don't need to call this function themselves.
//
#[no_mangle]
pub unsafe extern "C" fn drm_client_dev_hotplug(dev: *mut drm_device) {
    void drm_client_dev_hotplug(struct drm_device *dev)
    {
    struct drm_client_dev *client;
    if (!drm_core_check_feature(dev, DRIVER_MODESET))
    return;
    if (!dev.mode_config.num_connector) {
    drm_dbg_kms(dev, "No connectors found, will not send hotplug events!\n");
    return;
    }
    mutex_lock(&dev.clientlist_mutex);
    list_for_each_entry(client, &dev.clientlist, list)
    drm_client_hotplug(client);
    mutex_unlock(&dev.clientlist_mutex);
    }
    EXPORT_SYMBOL(drm_client_dev_hotplug);
#[no_mangle]
pub unsafe extern "C" fn drm_client_dev_restore(dev: *mut drm_device, force: bool) {
    void drm_client_dev_restore(struct drm_device *dev, bool force)
    {
    struct drm_client_dev *client;
    int ret;
    if (!drm_core_check_feature(dev, DRIVER_MODESET))
    return;
    mutex_lock(&dev.clientlist_mutex);
    list_for_each_entry(client, &dev.clientlist, list) {
    if (!client.funcs || !client.funcs.restore)
    continue;
    ret = client.funcs.restore(client, force);
    drm_dbg_kms(dev, "%s: ret=%d\n", client.name, ret);
    if (!ret) /* The first one to return zero gets the privilege to restore */
    break;
    }
    mutex_unlock(&dev.clientlist_mutex);
    }
#[no_mangle]
unsafe extern "C" fn drm_client_suspend(client: *mut drm_client_dev) -> c_int {
    static int drm_client_suspend(struct drm_client_dev *client)
    {
    struct drm_device *dev = client.dev;
    let mut ret: c_int = 0;
    if (drm_WARN_ON_ONCE(dev, client.suspended))
    return 0;
    if (client.funcs && client.funcs.suspend)
    ret = client.funcs.suspend(client);
    drm_dbg_kms(dev, "%s: ret=%d\n", client.name, ret);
    client.suspended = true;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn drm_client_dev_suspend(dev: *mut drm_device) {
    void drm_client_dev_suspend(struct drm_device *dev)
    {
    struct drm_client_dev *client;
    mutex_lock(&dev.clientlist_mutex);
    list_for_each_entry(client, &dev.clientlist, list) {
    if (!client.suspended)
    drm_client_suspend(client);
    }
    mutex_unlock(&dev.clientlist_mutex);
    }
    EXPORT_SYMBOL(drm_client_dev_suspend);
#[no_mangle]
unsafe extern "C" fn drm_client_resume(client: *mut drm_client_dev) -> c_int {
    static int drm_client_resume(struct drm_client_dev *client)
    {
    struct drm_device *dev = client.dev;
    let mut ret: c_int = 0;
    if (drm_WARN_ON_ONCE(dev, !client.suspended))
    return 0;
    if (client.funcs && client.funcs.resume)
    ret = client.funcs.resume(client);
    drm_dbg_kms(dev, "%s: ret=%d\n", client.name, ret);
    client.suspended = false;
    if (client.hotplug_pending)
    drm_client_hotplug(client);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn drm_client_dev_resume(dev: *mut drm_device) {
    void drm_client_dev_resume(struct drm_device *dev)
    {
    struct drm_client_dev *client;
    mutex_lock(&dev.clientlist_mutex);
    list_for_each_entry(client, &dev.clientlist, list) {
    if  (client.suspended)
    drm_client_resume(client);
    }
    mutex_unlock(&dev.clientlist_mutex);
    }
    EXPORT_SYMBOL(drm_client_dev_resume);

#[no_mangle]
unsafe extern "C" fn drm_client_debugfs_internal_clients(m: *mut seq_file, data: *mut c_void) -> c_int {
    static int drm_client_debugfs_internal_clients(struct seq_file *m, void *data)
    {
    struct drm_debugfs_entry *entry = m.private;
    struct drm_device *dev = entry.dev;
    let mut p: drm_printer = drm_seq_file_printer(m);
    struct drm_client_dev *client;
    mutex_lock(&dev.clientlist_mutex);
    list_for_each_entry(client, &dev.clientlist, list)
    drm_printf(&p, "%s\n", client.name);
    mutex_unlock(&dev.clientlist_mutex);
    return 0;
    }
    static const struct drm_debugfs_info drm_client_debugfs_list[] = {
    { "internal_clients", drm_client_debugfs_internal_clients, 0 },
    };
#[no_mangle]
pub unsafe extern "C" fn drm_client_debugfs_init(dev: *mut drm_device) {
    void drm_client_debugfs_init(struct drm_device *dev)
    {
    drm_debugfs_add_files(dev, drm_client_debugfs_list,
    ARRAY_SIZE(drm_client_debugfs_list));
    }
