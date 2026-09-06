//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/marvell/libertas/firmware.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Firmware loading and handling functions.
//

    static void load_next_firmware_from_table(struct lbs_private *private);
    static void lbs_fw_loaded(struct lbs_private *priv, int ret,
    const struct firmware *helper, const struct firmware *mainfw)
    {
    unsigned long flags;
    lbs_deb_fw("firmware load complete, code %d\n", ret);
// User must free helper/mainfw
    priv.fw_callback(priv, ret, helper, mainfw);
    spin_lock_irqsave(&priv.driver_lock, flags);
    priv.fw_callback = core::ptr::null_mut();
    wake_up(&priv.fw_waitq);
    spin_unlock_irqrestore(&priv.driver_lock, flags);
    }
    static void do_load_firmware(struct lbs_private *priv, const char *name,
    void (*cb)(const struct firmware *fw, void *context))
    {
    int ret;
    lbs_deb_fw("Requesting %s\n", name);
    ret = request_firmware_nowait(THIS_MODULE, true, name,
    priv.fw_device, GFP_KERNEL, priv, cb);
    if (ret) {
    lbs_deb_fw("request_firmware_nowait error %d\n", ret);
    lbs_fw_loaded(priv, ret, core::ptr::null_mut(), core::ptr::null_mut());
    }
    }
#[no_mangle]
unsafe extern "C" fn main_firmware_cb(firmware: *const firmware, context: *mut c_void) {
    static void main_firmware_cb(const struct firmware *firmware, void *context)
    {
    struct lbs_private *priv = context;
    if (!firmware) {
// Failed to find firmware: try next table entry
    load_next_firmware_from_table(priv);
    return;
    }
// Firmware found!
    lbs_fw_loaded(priv, 0, priv.helper_fw, firmware);
    if (priv.helper_fw) {
    release_firmware (priv.helper_fw);
    priv.helper_fw = core::ptr::null_mut();
    }
    release_firmware (firmware);
    }
#[no_mangle]
unsafe extern "C" fn helper_firmware_cb(firmware: *const firmware, context: *mut c_void) {
    static void helper_firmware_cb(const struct firmware *firmware, void *context)
    {
    struct lbs_private *priv = context;
    if (!firmware) {
// Failed to find firmware: try next table entry
    load_next_firmware_from_table(priv);
    return;
    }
// Firmware found!
    if (priv.fw_iter.fwname) {
    priv.helper_fw = firmware;
    do_load_firmware(priv, priv.fw_iter.fwname, main_firmware_cb);
    } else {
// No main firmware needed for this helper --> success!
    lbs_fw_loaded(priv, 0, firmware, core::ptr::null_mut());
    release_firmware(firmware);
    }
    }
#[no_mangle]
unsafe extern "C" fn load_next_firmware_from_table(priv: *mut lbs_private) {
    static void load_next_firmware_from_table(struct lbs_private *priv)
    {
    const struct lbs_fw_table *iter;
    if (!priv.fw_iter)
    iter = priv.fw_table;
    else
    iter = ++priv.fw_iter;
    if (priv.helper_fw) {
    release_firmware(priv.helper_fw);
    priv.helper_fw = core::ptr::null_mut();
    }
    next:
    if (!iter.helper) {
// End of table hit.
    lbs_fw_loaded(priv, -ENOENT, core::ptr::null_mut(), core::ptr::null_mut());
    return;
    }
    if (iter.model != priv.fw_model) {
    iter++;
    goto next;
    }
    priv.fw_iter = iter;
    do_load_firmware(priv, iter.helper, helper_firmware_cb);
    }
#[no_mangle]
pub unsafe extern "C" fn lbs_wait_for_firmware_load(priv: *mut lbs_private) {
    void lbs_wait_for_firmware_load(struct lbs_private *priv)
    {
    wait_event(priv.fw_waitq, priv.fw_callback == core::ptr::null_mut());
    }
//
// lbs_get_firmware_async - Retrieves firmware asynchronously. Can load
// either a helper firmware and a main firmware (2-stage), or just the helper.
//
// @priv:      Pointer to lbs_private instance
// @device:   	A pointer to &device structure
// @card_model: Bus-specific card model ID used to filter firmware table
// elements
// @fw_table:	Table of firmware file names and device model numbers
// terminated by an entry with a NULL helper name
// @callback:	User callback to invoke when firmware load succeeds or fails.
//
    int lbs_get_firmware_async(struct lbs_private *priv, struct device *device,
    u32 card_model, const struct lbs_fw_table *fw_table,
    lbs_fw_cb callback)
    {
    unsigned long flags;
    spin_lock_irqsave(&priv.driver_lock, flags);
    if (priv.fw_callback) {
    lbs_deb_fw("firmware load already in progress\n");
    spin_unlock_irqrestore(&priv.driver_lock, flags);
    return -EBUSY;
    }
    priv.fw_device = device;
    priv.fw_callback = callback;
    priv.fw_table = fw_table;
    priv.fw_iter = core::ptr::null_mut();
    priv.fw_model = card_model;
    spin_unlock_irqrestore(&priv.driver_lock, flags);
    lbs_deb_fw("Starting async firmware load\n");
    load_next_firmware_from_table(priv);
    return 0;
    }
    EXPORT_SYMBOL_GPL(lbs_get_firmware_async);
//
// lbs_get_firmware - Retrieves two-stage firmware
//
// @dev:     	A pointer to &device structure
// @card_model: Bus-specific card model ID used to filter firmware table
// elements
// @fw_table:	Table of firmware file names and device model numbers
// terminated by an entry with a NULL helper name
// @helper:	On success, the helper firmware; caller must free
// @mainfw:	On success, the main firmware; caller must free
//
// Deprecated: use lbs_get_firmware_async() instead.
//
// returns:		0 on success, non-zero on failure
//
    int lbs_get_firmware(struct device *dev, u32 card_model,
    const struct lbs_fw_table *fw_table,
    const struct firmware **helper,
    const struct firmware **mainfw)
    {
    const struct lbs_fw_table *iter;
    int ret;
    BUG_ON(helper == core::ptr::null_mut());
    BUG_ON(mainfw == core::ptr::null_mut());
// Search for firmware to use from the table.
    iter = fw_table;
    while (iter && iter.helper) {
    if (iter.model != card_model)
    goto next;
    if (*helper == core::ptr::null_mut()) {
    ret = request_firmware(helper, iter.helper, dev);
    if (ret)
    goto next;
// If the device has one-stage firmware (ie cf8305) and
// we've got it then we don't need to bother with the
// main firmware.
//
    if (iter.fwname == core::ptr::null_mut())
    return 0;
    }
    if (*mainfw == core::ptr::null_mut()) {
    ret = request_firmware(mainfw, iter.fwname, dev);
    if (ret) {
// Clear the helper to ensure we don't have
// mismatched firmware pairs.
//
    release_firmware(*helper);
// helper = NULL;
    }
    }
    if (*helper && *mainfw)
    return 0;
    next:
    iter++;
    }
// Failed
    release_firmware(*helper);
// helper = NULL;
    release_firmware(*mainfw);
// mainfw = NULL;
    return -ENOENT;
    }
    EXPORT_SYMBOL_GPL(lbs_get_firmware);
