//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/scsi_dh.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// SCSI device handler infrastructure.
//
// Copyright IBM Corporation, 2007
// Authors:
// Chandra Seetharaman <sekharan@us.ibm.com>
// Mike Anderson <andmike@linux.vnet.ibm.com>
//

    static DEFINE_SPINLOCK(list_lock);
    static LIST_HEAD(scsi_dh_list);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_dh_blist {
    pub vendor: *const c_char,
    pub model: *const c_char,
    pub driver: *const c_char,
}

    static const struct scsi_dh_blist scsi_dh_blist[] = {
    {"DGC", "RAID",			"emc" },
    {"DGC", "DISK",			"emc" },
    {"DGC", "VRAID",		"emc" },
    {"COMPAQ", "MSA1000 VOLUME",	"hp_sw" },
    {"COMPAQ", "HSV110",		"hp_sw" },
    {"HP", "HSV100",		"hp_sw"},
    {"DEC", "HSG80",		"hp_sw"},
    {"IBM", "1722",			"rdac", },
    {"IBM", "1724",			"rdac", },
    {"IBM", "1726",			"rdac", },
    {"IBM", "1742",			"rdac", },
    {"IBM", "1745",			"rdac", },
    {"IBM", "1746",			"rdac", },
    {"IBM", "1813",			"rdac", },
    {"IBM", "1814",			"rdac", },
    {"IBM", "1815",			"rdac", },
    {"IBM", "1818",			"rdac", },
    {"IBM", "3526",			"rdac", },
    {"IBM", "3542",			"rdac", },
    {"IBM", "3552",			"rdac", },
    {"SGI", "TP9300",		"rdac", },
    {"SGI", "TP9400",		"rdac", },
    {"SGI", "TP9500",		"rdac", },
    {"SGI", "TP9700",		"rdac", },
    {"SGI", "IS",			"rdac", },
    {"STK", "OPENstorage",		"rdac", },
    {"STK", "FLEXLINE 380",		"rdac", },
    {"STK", "BladeCtlr",		"rdac", },
    {"SUN", "CSM",			"rdac", },
    {"SUN", "LCSM100",		"rdac", },
    {"SUN", "STK6580_6780",		"rdac", },
    {"SUN", "SUN_6180",		"rdac", },
    {"SUN", "ArrayStorage",		"rdac", },
    {"DELL", "MD3",			"rdac", },
    {"NETAPP", "INF-01-00",		"rdac", },
    {"LSI", "INF-01-00",		"rdac", },
    {"ENGENIO", "INF-01-00",	"rdac", },
    {"LENOVO", "DE_Series",		"rdac", },
    {"FUJITSU", "ETERNUS_AHB",	"rdac", },
    {core::ptr::null_mut(), core::ptr::null_mut(),			core::ptr::null_mut() },
    };
    static const char *
    scsi_dh_find_driver(struct scsi_device *sdev)
    {
    const struct scsi_dh_blist *b;
    if (scsi_device_tpgs(sdev))
    return "alua";
    for (b = scsi_dh_blist; b.vendor; b++) {
    if (!strncmp(sdev.vendor, b.vendor, strlen(b.vendor)) &&
    !strncmp(sdev.model, b.model, strlen(b.model))) {
    return b.driver;
    }
    }
    return core::ptr::null_mut();
    }
    static struct scsi_device_handler *__scsi_dh_lookup(const char *name)
    {
    struct scsi_device_handler *tmp, *found = core::ptr::null_mut();
    spin_lock(&list_lock);
    list_for_each_entry(tmp, &scsi_dh_list, list) {
    if (!strncmp(tmp.name, name, strlen(tmp.name))) {
    found = tmp;
    break;
    }
    }
    spin_unlock(&list_lock);
    return found;
    }
    static struct scsi_device_handler *scsi_dh_lookup(const char *name)
    {
    struct scsi_device_handler *dh;
    if (!name || strlen(name) == 0)
    return core::ptr::null_mut();
    dh = __scsi_dh_lookup(name);
    if (!dh) {
    request_module("scsi_dh_%s", name);
    dh = __scsi_dh_lookup(name);
    }
    return dh;
    }
//
// scsi_dh_handler_attach - Attach a device handler to a device
// @sdev - SCSI device the device handler should attach to
// @scsi_dh - The device handler to attach
//
    static int scsi_dh_handler_attach(struct scsi_device *sdev,
    struct scsi_device_handler *scsi_dh)
    {
    int error, ret = 0;
    if (!try_module_get(scsi_dh.module))
    return -EINVAL;
    error = scsi_dh.attach(sdev);
    if (error != SCSI_DH_OK) {
    switch (error) {
    case SCSI_DH_NOMEM:
    ret = -ENOMEM;
    break;
    case SCSI_DH_RES_TEMP_UNAVAIL:
    ret = -EAGAIN;
    break;
    case SCSI_DH_DEV_UNSUPP:
    case SCSI_DH_NOSYS:
    ret = -ENODEV;
    break;
    default:
    ret = -EINVAL;
    break;
    }
    if (ret != -ENODEV)
    sdev_printk(KERN_ERR, sdev, "%s: Attach failed (%d)\n",
    scsi_dh.name, error);
    module_put(scsi_dh.module);
    } else
    sdev.handler = scsi_dh;
    return ret;
    }
//
// scsi_dh_handler_detach - Detach a device handler from a device
// @sdev - SCSI device the device handler should be detached from
//
#[no_mangle]
unsafe extern "C" fn scsi_dh_handler_detach(sdev: *mut scsi_device) {
    static void scsi_dh_handler_detach(struct scsi_device *sdev)
    {
    sdev.handler.detach(sdev);
    sdev_printk(KERN_NOTICE, sdev, "%s: Detached\n", sdev.handler.name);
    module_put(sdev.handler.module);
    }
#[no_mangle]
pub unsafe extern "C" fn scsi_dh_add_device(sdev: *mut scsi_device) {
    void scsi_dh_add_device(struct scsi_device *sdev)
    {
    struct scsi_device_handler *devinfo = core::ptr::null_mut();
    const char *drv;
    drv = scsi_dh_find_driver(sdev);
    if (drv)
    devinfo = __scsi_dh_lookup(drv);
//
// device_handler is optional, so ignore errors
// from scsi_dh_handler_attach()
//
    if (devinfo)
    (void)scsi_dh_handler_attach(sdev, devinfo);
    }
#[no_mangle]
pub unsafe extern "C" fn scsi_dh_release_device(sdev: *mut scsi_device) {
    void scsi_dh_release_device(struct scsi_device *sdev)
    {
    if (sdev.handler)
    scsi_dh_handler_detach(sdev);
    }
//
// scsi_register_device_handler - register a device handler personality
// module.
// @scsi_dh - device handler to be registered.
//
// Returns 0 on success, -EBUSY if handler already registered.
//
#[no_mangle]
pub unsafe extern "C" fn scsi_register_device_handler(scsi_dh: *mut scsi_device_handler) -> c_int {
    int scsi_register_device_handler(struct scsi_device_handler *scsi_dh)
    {
    if (__scsi_dh_lookup(scsi_dh.name))
    return -EBUSY;
    if (!scsi_dh.attach || !scsi_dh.detach)
    return -EINVAL;
    spin_lock(&list_lock);
    list_add(&scsi_dh.list, &scsi_dh_list);
    spin_unlock(&list_lock);
    printk(KERN_INFO "%s: device handler registered\n", scsi_dh.name);
    return SCSI_DH_OK;
    }
    EXPORT_SYMBOL_GPL(scsi_register_device_handler);
//
// scsi_unregister_device_handler - register a device handler personality
// module.
// @scsi_dh - device handler to be unregistered.
//
// Returns 0 on success, -ENODEV if handler not registered.
//
#[no_mangle]
pub unsafe extern "C" fn scsi_unregister_device_handler(scsi_dh: *mut scsi_device_handler) -> c_int {
    int scsi_unregister_device_handler(struct scsi_device_handler *scsi_dh)
    {
    if (!__scsi_dh_lookup(scsi_dh.name))
    return -ENODEV;
    spin_lock(&list_lock);
    list_del(&scsi_dh.list);
    spin_unlock(&list_lock);
    printk(KERN_INFO "%s: device handler unregistered\n", scsi_dh.name);
    return SCSI_DH_OK;
    }
    EXPORT_SYMBOL_GPL(scsi_unregister_device_handler);
//
// scsi_dh_activate - activate the path associated with the scsi_device
// corresponding to the given request queue.
// Returns immediately without waiting for activation to be completed.
// @q    - Request queue that is associated with the scsi_device to be
// activated.
// @fn   - Function to be called upon completion of the activation.
// Function fn is called with data (below) and the error code.
// Function fn may be called from the same calling context. So,
// do not hold the lock in the caller which may be needed in fn.
// @data - data passed to the function fn upon completion.
//
#[no_mangle]
pub unsafe extern "C" fn scsi_dh_activate(q: *mut request_queue, fn: activate_complete, data: *mut c_void) -> c_int {
    int scsi_dh_activate(struct request_queue *q, activate_complete fn, void *data)
    {
    struct scsi_device *sdev;
    let mut err: c_int = SCSI_DH_NOSYS;
    sdev = scsi_device_from_queue(q);
    if (!sdev) {
    if (fn)
    fn(data, err);
    return err;
    }
    if (!sdev.handler)
    goto out_fn;
    err = SCSI_DH_NOTCONN;
    if (sdev.sdev_state == SDEV_CANCEL ||
    sdev.sdev_state == SDEV_DEL)
    goto out_fn;
    err = SCSI_DH_DEV_OFFLINED;
    if (sdev.sdev_state == SDEV_OFFLINE)
    goto out_fn;
    if (sdev.handler.activate)
    err = sdev.handler.activate(sdev, fn, data);
    out_put_device:
    put_device(&sdev.sdev_gendev);
    return err;
    out_fn:
    if (fn)
    fn(data, err);
    goto out_put_device;
    }
    EXPORT_SYMBOL_GPL(scsi_dh_activate);
//
// scsi_dh_set_params - set the parameters for the device as per the
// string specified in params.
// @q - Request queue that is associated with the scsi_device for
// which the parameters to be set.
// @params - parameters in the following format
// "no_of_params\0param1\0param2\0param3\0...\0"
// for example, string for 2 parameters with value 10 and 21
// is specified as "2\010\021\0".
//
#[no_mangle]
pub unsafe extern "C" fn scsi_dh_set_params(q: *mut request_queue, params: *const c_char) -> c_int {
    int scsi_dh_set_params(struct request_queue *q, const char *params)
    {
    struct scsi_device *sdev;
    let mut err: c_int = -SCSI_DH_NOSYS;
    sdev = scsi_device_from_queue(q);
    if (!sdev)
    return err;
    if (sdev.handler && sdev.handler.set_params)
    err = sdev.handler.set_params(sdev, params);
    put_device(&sdev.sdev_gendev);
    return err;
    }
    EXPORT_SYMBOL_GPL(scsi_dh_set_params);
//
// scsi_dh_attach - Attach device handler
// @q - Request queue that is associated with the scsi_device
// the handler should be attached to
// @name - name of the handler to attach
//
#[no_mangle]
pub unsafe extern "C" fn scsi_dh_attach(q: *mut request_queue, name: *const c_char) -> c_int {
    int scsi_dh_attach(struct request_queue *q, const char *name)
    {
    struct scsi_device *sdev;
    struct scsi_device_handler *scsi_dh;
    let mut err: c_int = 0;
    sdev = scsi_device_from_queue(q);
    if (!sdev)
    return -ENODEV;
    scsi_dh = scsi_dh_lookup(name);
    if (!scsi_dh) {
    err = -EINVAL;
    goto out_put_device;
    }
    if (sdev.handler) {
    if (sdev.handler != scsi_dh)
    err = -EBUSY;
    goto out_put_device;
    }
    err = scsi_dh_handler_attach(sdev, scsi_dh);
    out_put_device:
    put_device(&sdev.sdev_gendev);
    return err;
    }
    EXPORT_SYMBOL_GPL(scsi_dh_attach);
//
// scsi_dh_attached_handler_name - Get attached device handler's name
// @q - Request queue that is associated with the scsi_device
// that may have a device handler attached
// @gfp - the GFP mask used in the kmalloc() call when allocating memory
//
// Returns name of attached handler, NULL if no handler is attached, or
// and error pointer if an error occurred.
// Caller must take care to free the returned string.
//
    const char *scsi_dh_attached_handler_name(struct request_queue *q, gfp_t gfp)
    {
    struct scsi_device *sdev;
    const char *handler_name = core::ptr::null_mut();
    sdev = scsi_device_from_queue(q);
    if (!sdev)
    return ERR_PTR(-ENODEV);
    if (sdev.handler)
    handler_name = kstrdup(sdev.handler.name, gfp) ? :
    ERR_PTR(-ENOMEM);
    put_device(&sdev.sdev_gendev);
    return handler_name;
    }
    EXPORT_SYMBOL_GPL(scsi_dh_attached_handler_name);
