//! Automatically rewritten from C to Rust
//! Source: drivers/tee/optee/core.c
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
// Copyright (c) 2015-2021, Linaro Limited
// Copyright (c) 2016, EPAM Systems
//

    struct blocking_notifier_head optee_rpmb_intf_added =
    BLOCKING_NOTIFIER_INIT(optee_rpmb_intf_added);
#[no_mangle]
unsafe extern "C" fn rpmb_add_dev(dev: *mut device) -> c_int {
    static int rpmb_add_dev(struct device *dev)
    {
    blocking_notifier_call_chain(&optee_rpmb_intf_added, 0,
    to_rpmb_dev(dev));
    return 0;
    }
    static struct class_interface rpmb_class_intf = {
    .add_dev = rpmb_add_dev,
    };
#[no_mangle]
pub unsafe extern "C" fn optee_bus_scan_rpmb(work: *mut work_struct) {
    void optee_bus_scan_rpmb(struct work_struct *work)
    {
    struct optee *optee = container_of(work, struct optee,
    rpmb_scan_bus_work);
    int ret;
    if (!optee.rpmb_scan_bus_done) {
    ret = optee_enumerate_devices(PTA_CMD_GET_DEVICES_RPMB);
    optee.rpmb_scan_bus_done = !ret;
    if (ret && ret != -ENODEV)
    pr_info("Scanning for RPMB device: ret %d\n", ret);
    }
    }
    int optee_rpmb_intf_rdev(struct notifier_block *intf, unsigned long action,
    void *data)
    {
    struct optee *optee = container_of(intf, struct optee, rpmb_intf);
    schedule_work(&optee.rpmb_scan_bus_work);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn optee_set_dma_mask(optee: *mut optee, pa_width: u_int) -> c_int {
    int optee_set_dma_mask(struct optee *optee, u_int pa_width)
    {
    let mut mask: u64 = DMA_BIT_MASK(min(64, pa_width));
    return dma_coerce_mask_and_coherent(&optee.teedev.dev, mask);
    }
#[no_mangle]
pub unsafe extern "C" fn optee_get_revision(teedev: *mut tee_device, buf: *mut c_char, len: usize) -> c_int {
    int optee_get_revision(struct tee_device *teedev, char *buf, size_t len)
    {
    struct optee *optee = tee_get_drvdata(teedev);
    u64 build_id;
    if (!optee)
    return -ENODEV;
    if (!buf || !len)
    return -EINVAL;
    build_id = optee.revision.os_build_id;
    if (build_id)
    scnprintf(buf, len, "%u.%u (%016llx)",
    optee.revision.os_major,
    optee.revision.os_minor,
    (unsigned long long)build_id);
    else
    scnprintf(buf, len, "%u.%u", optee.revision.os_major,
    optee.revision.os_minor);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn optee_bus_scan(work: *mut work_struct) {
    static void optee_bus_scan(struct work_struct *work)
    {
    WARN_ON(optee_enumerate_devices(PTA_CMD_GET_DEVICES_SUPP));
    }
    static ssize_t rpmb_routing_model_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct optee *optee = dev_get_drvdata(dev);
    const char *s;
    if (optee.in_kernel_rpmb_routing)
    s = "kernel";
    else
    s = "user";
    return sysfs_emit(buf, "%s\n", s);
    }
    static DEVICE_ATTR_RO(rpmb_routing_model);
    static struct attribute *optee_dev_attrs[] = {
    &dev_attr_rpmb_routing_model.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(optee_dev);
#[no_mangle]
pub unsafe extern "C" fn optee_set_dev_group(optee: *mut optee) {
    void optee_set_dev_group(struct optee *optee)
    {
    tee_device_set_dev_groups(optee.teedev, optee_dev_groups);
    tee_device_set_dev_groups(optee.supp_teedev, optee_dev_groups);
    }
#[no_mangle]
pub unsafe extern "C" fn optee_open(ctx: *mut tee_context, cap_memref_null: bool) -> c_int {
    int optee_open(struct tee_context *ctx, bool cap_memref_null)
    {
    struct optee_context_data *ctxdata;
    struct tee_device *teedev = ctx.teedev;
    struct optee *optee = tee_get_drvdata(teedev);
    ctxdata = kzalloc_obj(*ctxdata);
    if (!ctxdata)
    return -ENOMEM;
    if (teedev == optee.supp_teedev) {
    let mut busy: bool = true;
    mutex_lock(&optee.supp.mutex);
    if (!optee.supp.ctx) {
    busy = false;
    optee.supp.ctx = ctx;
    }
    mutex_unlock(&optee.supp.mutex);
    if (busy) {
    kfree(ctxdata);
    return -EBUSY;
    }
    if (!optee.scan_bus_done) {
    INIT_WORK(&optee.scan_bus_work, optee_bus_scan);
    schedule_work(&optee.scan_bus_work);
    optee.scan_bus_done = true;
    }
    }
    mutex_init(&ctxdata.mutex);
    INIT_LIST_HEAD(&ctxdata.sess_list);
    ctx.cap_memref_null = cap_memref_null;
    ctx.data = ctxdata;
    return 0;
    }
    static void optee_release_helper(struct tee_context *ctx,
    int (*close_session)(struct tee_context *ctx,
    u32 session,
    bool system_thread))
    {
    struct optee_context_data *ctxdata = ctx.data;
    struct optee_session *sess;
    struct optee_session *sess_tmp;
    if (!ctxdata)
    return;
    list_for_each_entry_safe(sess, sess_tmp, &ctxdata.sess_list,
    list_node) {
    list_del(&sess.list_node);
    close_session(ctx, sess.session_id, sess.use_sys_thread);
    kfree(sess);
    }
    kfree(ctxdata);
    ctx.data = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn optee_release(ctx: *mut tee_context) {
    void optee_release(struct tee_context *ctx)
    {
    optee_release_helper(ctx, optee_close_session_helper);
    }
#[no_mangle]
pub unsafe extern "C" fn optee_release_supp(ctx: *mut tee_context) {
    void optee_release_supp(struct tee_context *ctx)
    {
    struct optee *optee = tee_get_drvdata(ctx.teedev);
    optee_release_helper(ctx, optee_close_session_helper);
    optee_supp_release(&optee.supp);
    }
#[no_mangle]
pub unsafe extern "C" fn optee_remove_common(optee: *mut optee) {
    void optee_remove_common(struct optee *optee)
    {
    blocking_notifier_chain_unregister(&optee_rpmb_intf_added,
    &optee.rpmb_intf);
    cancel_work_sync(&optee.rpmb_scan_bus_work);
// Unregister OP-TEE specific client devices on TEE bus
    optee_unregister_devices();
    optee_notif_uninit(optee);
    optee_shm_arg_cache_uninit(optee);
    teedev_close_context(optee.ctx);
//
// The two devices have to be unregistered before we can free the
// other resources.
//
    tee_device_unregister(optee.supp_teedev);
    tee_device_unregister(optee.teedev);
    tee_shm_pool_free(optee.pool);
    optee_supp_uninit(&optee.supp);
    mutex_destroy(&optee.call_queue.mutex);
    rpmb_dev_put(optee.rpmb_dev);
    mutex_destroy(&optee.rpmb_dev_mutex);
    }
    static int smc_abi_rc;
    static int ffa_abi_rc;
    static bool intf_is_regged;
#[no_mangle]
unsafe extern "C" fn optee_core_init() -> int __init {
    static int __init optee_core_init(void)
    {
    int rc;
//
// The kernel may have crashed at the same time that all available
// secure world threads were suspended and we cannot reschedule the
// suspended threads without access to the crashed kernel's wait_queue.
// Therefore, we cannot reliably initialize the OP-TEE driver in the
// kdump kernel.
//
    if (is_kdump_kernel())
    return -ENODEV;
    if (IS_REACHABLE(CONFIG_RPMB)) {
    rc = rpmb_interface_register(&rpmb_class_intf);
    if (rc)
    return rc;
    intf_is_regged = true;
    }
    smc_abi_rc = optee_smc_abi_register();
    ffa_abi_rc = optee_ffa_abi_register();
// If both failed there's no point with this module
    if (smc_abi_rc && ffa_abi_rc) {
    if (IS_REACHABLE(CONFIG_RPMB)) {
    rpmb_interface_unregister(&rpmb_class_intf);
    intf_is_regged = false;
    }
    return smc_abi_rc;
    }
    return 0;
    }
    module_init(optee_core_init);
#[no_mangle]
unsafe extern "C" fn optee_core_exit() -> void __exit {
    static void __exit optee_core_exit(void)
    {
    if (IS_REACHABLE(CONFIG_RPMB) && intf_is_regged) {
    rpmb_interface_unregister(&rpmb_class_intf);
    intf_is_regged = false;
    }
    if (!smc_abi_rc)
    optee_smc_abi_unregister();
    if (!ffa_abi_rc)
    optee_ffa_abi_unregister();
    }
    module_exit(optee_core_exit);
    MODULE_AUTHOR("Linaro");
    MODULE_DESCRIPTION("OP-TEE driver");
    MODULE_VERSION("1.0");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:optee");
