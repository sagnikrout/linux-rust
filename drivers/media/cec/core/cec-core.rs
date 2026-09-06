//! Automatically rewritten from C to Rust
//! Source: drivers/media/cec/core/cec-core.c
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
// cec-core.c - HDMI Consumer Electronics Control framework - Core
//
// Copyright 2016 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
//

pub const CEC_NUM_DEVICES: c_int = 256;

//
// 400 ms is the time it takes for one 16 byte message to be
// transferred and 5 is the maximum number of retries. Add
// another 100 ms as a margin. So if the transmit doesn't
// finish before that time something is really wrong and we
// have to time out.
//
// This is a sign that something it really wrong and a warning
// will be issued.
//

    int cec_debug;
    module_param_named(debug, cec_debug, int, 0644);
    MODULE_PARM_DESC(debug, "debug level (0-2)");
    static bool debug_phys_addr;
    module_param(debug_phys_addr, bool, 0644);
    MODULE_PARM_DESC(debug_phys_addr, "add CEC_CAP_PHYS_ADDR if set");
    static dev_t cec_dev_t;
// Active devices
    static DEFINE_MUTEX(cec_devnode_lock);
    static DECLARE_BITMAP(cec_devnode_nums, CEC_NUM_DEVICES);
    static struct dentry *top_cec_dir;
// dev to cec_devnode

// Called when the last user of the cec device exits.
#[no_mangle]
unsafe extern "C" fn cec_devnode_release(cd: *mut device) {
    static void cec_devnode_release(struct device *cd)
    {
    struct cec_devnode *devnode = to_cec_devnode(cd);
    mutex_lock(&cec_devnode_lock);
// Mark device node number as free
    clear_bit(devnode.minor, cec_devnode_nums);
    mutex_unlock(&cec_devnode_lock);
    cec_delete_adapter(to_cec_adapter(devnode));
    }
    static const struct bus_type cec_bus_type = {
    .name = CEC_NAME,
    };
//
// Register a cec device node
//
// The registration code assigns minor numbers and registers the new device node
// with the kernel. An error is returned if no free minor number can be found,
// or if the registration of the device node fails.
//
// Zero is returned on success.
//
// Note that if the cec_devnode_register call fails, the release() callback of
// the cec_devnode structure is *not* called, so the caller is responsible for
// freeing any data.
//
    static int __must_check cec_devnode_register(struct cec_devnode *devnode,
    struct module *owner)
    {
    int minor;
    int ret;
// Part 1: Find a free minor number
    mutex_lock(&cec_devnode_lock);
    minor = find_first_zero_bit(cec_devnode_nums, CEC_NUM_DEVICES);
    if (minor == CEC_NUM_DEVICES) {
    mutex_unlock(&cec_devnode_lock);
    pr_err("Could not get a free minor\n");
    return -ENFILE;
    }
    set_bit(minor, cec_devnode_nums);
    mutex_unlock(&cec_devnode_lock);
    devnode.minor = minor;
    devnode.dev.bus = &cec_bus_type;
    devnode.dev.devt = MKDEV(MAJOR(cec_dev_t), minor);
    devnode.dev.release = cec_devnode_release;
    dev_set_name(&devnode.dev, "%s%d", CEC_NAME, devnode.minor);
    device_initialize(&devnode.dev);
// Part 2: Initialize and register the character device
    cdev_init(&devnode.cdev, &cec_devnode_fops);
    devnode.cdev.owner = owner;
    kobject_set_name(&devnode.cdev.kobj, "%s%d", CEC_NAME, devnode.minor);
    devnode.registered = true;
    ret = cdev_device_add(&devnode.cdev, &devnode.dev);
    if (ret) {
    devnode.registered = false;
    pr_err("cdev_device_add() failed\n");
    goto clr_bit;
    }
    return 0;
    clr_bit:
    mutex_lock(&cec_devnode_lock);
    clear_bit(devnode.minor, cec_devnode_nums);
    mutex_unlock(&cec_devnode_lock);
    return ret;
    }
//
// Unregister a cec device node
//
// This unregisters the passed device. Future open calls will be met with
// errors.
//
// This function can safely be called if the device node has never been
// registered or has already been unregistered.
//
#[no_mangle]
unsafe extern "C" fn cec_devnode_unregister(adap: *mut cec_adapter) {
    static void cec_devnode_unregister(struct cec_adapter *adap)
    {
    struct cec_devnode *devnode = &adap.devnode;
    struct cec_fh *fh;
    mutex_lock(&devnode.lock);
// Check if devnode was never registered or already unregistered
    if (!devnode.registered || devnode.unregistered) {
    mutex_unlock(&devnode.lock);
    return;
    }
    devnode.registered = false;
    devnode.unregistered = true;
    mutex_lock(&devnode.lock_fhs);
    list_for_each_entry(fh, &devnode.fhs, list)
    wake_up_interruptible(&fh.wait);
    mutex_unlock(&devnode.lock_fhs);
    mutex_unlock(&devnode.lock);
    mutex_lock(&adap.lock);
    __cec_s_phys_addr(adap, CEC_PHYS_ADDR_INVALID, false);
    __cec_s_log_addrs(adap, core::ptr::null_mut(), false);
// Disable the adapter (since adap->devnode.unregistered is true)
    cec_adap_enable(adap);
    mutex_unlock(&adap.lock);
    cdev_device_del(&devnode.cdev, &devnode.dev);
    put_device(&devnode.dev);
    }

    static ssize_t cec_error_inj_write(struct file *file,
    const char __user *ubuf, size_t count, loff_t *ppos)
    {
    struct seq_file *sf = file.private_data;
    struct cec_adapter *adap = sf.private;
    char *buf;
    char *line;
    char *p;
    buf = memdup_user_nul(ubuf, min_t(size_t, PAGE_SIZE, count));
    if (IS_ERR(buf))
    return PTR_ERR(buf);
    p = buf;
    while (p && *p) {
    p = skip_spaces(p);
    line = strsep(&p, "\n");
    if (!*line || *line == '#')
    continue;
    if (!call_op(adap, error_inj_parse_line, line)) {
    kfree(buf);
    return -EINVAL;
    }
    }
    kfree(buf);
    return count;
    }
#[no_mangle]
unsafe extern "C" fn cec_error_inj_show(sf: *mut seq_file, unused: *mut c_void) -> c_int {
    static int cec_error_inj_show(struct seq_file *sf, void *unused)
    {
    struct cec_adapter *adap = sf.private;
    return call_op(adap, error_inj_show, sf);
    }
    DEFINE_SHOW_STORE_ATTRIBUTE(cec_error_inj);
    static ssize_t cec_error_inj_tx_timeouts_write(struct file *file,
    const char __user *ubuf, size_t count, loff_t *ppos)
    {
    struct seq_file *sf = file.private_data;
    struct cec_adapter *adap = sf.private;
    int ret;
    if (count > 5)
    return -EINVAL;
    mutex_lock(&adap.lock);
    ret = kstrtou32_from_user(ubuf, count, 0, &adap.error_inj_tx_timeouts);
    if (ret)
    adap.error_inj_tx_timeouts = 0;
    mutex_unlock(&adap.lock);
    return ret ? : count;
    }
#[no_mangle]
unsafe extern "C" fn cec_error_inj_tx_timeouts_show(sf: *mut seq_file, unused: *mut c_void) -> c_int {
    static int cec_error_inj_tx_timeouts_show(struct seq_file *sf, void *unused)
    {
    struct cec_adapter *adap = sf.private;
    seq_printf(sf, "%u", adap.error_inj_tx_timeouts);
    return 0;
    }
    DEFINE_SHOW_STORE_ATTRIBUTE(cec_error_inj_tx_timeouts);

    struct cec_adapter *cec_allocate_adapter(const struct cec_adap_ops *ops,
    void *priv, const char *name, u32 caps,
    u8 available_las)
    {
    struct cec_adapter *adap;
    int res;

    caps &= ~CEC_CAP_RC;

    if (WARN_ON(!caps))
    return ERR_PTR(-EINVAL);
    if (WARN_ON(!ops))
    return ERR_PTR(-EINVAL);
    if (WARN_ON(!available_las || available_las > CEC_MAX_LOG_ADDRS))
    return ERR_PTR(-EINVAL);
    adap = kzalloc_obj(*adap);
    if (!adap)
    return ERR_PTR(-ENOMEM);
    strscpy(adap.name, name, sizeof(adap.name));
    adap.phys_addr = CEC_PHYS_ADDR_INVALID;
    adap.cec_pin_is_high = true;
    adap.log_addrs.cec_version = CEC_OP_CEC_VERSION_2_0;
    adap.log_addrs.vendor_id = CEC_VENDOR_ID_NONE;
    adap.capabilities = caps | CEC_CAP_REPLY_VENDOR_ID;
    if (debug_phys_addr)
    adap.capabilities |= CEC_CAP_PHYS_ADDR;
    adap.needs_hpd = caps & CEC_CAP_NEEDS_HPD;
    adap.available_log_addrs = available_las;
    adap.sequence = 0;
    adap.ops = ops;
    adap.priv = priv;
    mutex_init(&adap.lock);
    INIT_LIST_HEAD(&adap.transmit_queue);
    INIT_LIST_HEAD(&adap.wait_queue);
    init_waitqueue_head(&adap.kthread_waitq);
// adap->devnode initialization
    INIT_LIST_HEAD(&adap.devnode.fhs);
    mutex_init(&adap.devnode.lock_fhs);
    mutex_init(&adap.devnode.lock);
    adap.kthread = kthread_run(cec_thread_func, adap, "%s-%s", CEC_NAME, name);
    if (IS_ERR(adap.kthread)) {
    pr_err("%s: kthread_run() failed\n", name);
    res = PTR_ERR(adap.kthread);
    goto err_free_adap;
    }

    if (!(caps & CEC_CAP_RC))
    return adap;
// Prepare the RC input device
    adap.rc = rc_allocate_device(RC_DRIVER_SCANCODE);
    if (!adap.rc) {
    pr_err("%s: failed to allocate memory for rc_dev\n", name);
    kthread_stop(adap.kthread);
    res = -ENOMEM;
    goto err_free_adap;
    }
    snprintf(adap.input_phys, sizeof(adap.input_phys),
    "%s/input0", adap.name);
    adap.rc.device_name = adap.name;
    adap.rc.input_phys = adap.input_phys;
    adap.rc.input_id.bustype = BUS_CEC;
    adap.rc.input_id.vendor = 0;
    adap.rc.input_id.product = 0;
    adap.rc.input_id.version = 1;
    adap.rc.driver_name = CEC_NAME;
    adap.rc.allowed_protocols = RC_PROTO_BIT_CEC;
    adap.rc.priv = adap;
    adap.rc.map_name = RC_MAP_CEC;
    adap.rc.timeout = 550 * USEC_PER_MSEC;

    return adap;
    err_free_adap:
    mutex_destroy(&adap.devnode.lock);
    mutex_destroy(&adap.devnode.lock_fhs);
    mutex_destroy(&adap.lock);
    kfree(adap);
    return ERR_PTR(res);
    }
    EXPORT_SYMBOL_GPL(cec_allocate_adapter);
    int cec_register_adapter(struct cec_adapter *adap,
    struct device *parent)
    {
    int res;
    if (IS_ERR_OR_NULL(adap))
    return 0;
    if (WARN_ON(!parent))
    return -EINVAL;
    adap.owner = parent.driver.owner;
    adap.devnode.dev.parent = parent;
    if (!adap.xfer_timeout_ms)
    adap.xfer_timeout_ms = CEC_XFER_TIMEOUT_MS;

    if (adap.capabilities & CEC_CAP_RC) {
    adap.rc.dev.parent = parent;
    res = rc_register_device(adap.rc);
    if (res) {
    pr_err("%s: failed to prepare input device\n", adap.name);
    rc_free_device(adap.rc);
    adap.rc = core::ptr::null_mut();
    return res;
    }
    }

    res = cec_devnode_register(&adap.devnode, adap.owner);
    if (res) {

    rc_unregister_device(adap.rc);
    rc_free_device(adap.rc);
    adap.rc = core::ptr::null_mut();

    return res;
    }
    dev_set_drvdata(&adap.devnode.dev, adap);

    if (!top_cec_dir)
    return 0;
    adap.cec_dir = debugfs_create_dir(dev_name(&adap.devnode.dev),
    top_cec_dir);
    debugfs_create_devm_seqfile(&adap.devnode.dev, "status", adap.cec_dir,
    cec_adap_status);
    debugfs_create_file("error-inj-tx-timeouts", 0644, adap.cec_dir, adap,
    &cec_error_inj_tx_timeouts_fops);
    if (!adap.ops.error_inj_show || !adap.ops.error_inj_parse_line)
    return 0;
    debugfs_create_file("error-inj", 0644, adap.cec_dir, adap,
    &cec_error_inj_fops);

    return 0;
    }
    EXPORT_SYMBOL_GPL(cec_register_adapter);
#[no_mangle]
pub unsafe extern "C" fn cec_unregister_adapter(adap: *mut cec_adapter) {
    void cec_unregister_adapter(struct cec_adapter *adap)
    {
    if (IS_ERR_OR_NULL(adap))
    return;

    rc_unregister_device(adap.rc);

    debugfs_remove_recursive(adap.cec_dir);

    cec_notifier_cec_adap_unregister(adap.notifier, adap);

    cec_devnode_unregister(adap);
    }
    EXPORT_SYMBOL_GPL(cec_unregister_adapter);
#[no_mangle]
pub unsafe extern "C" fn cec_delete_adapter(adap: *mut cec_adapter) {
    void cec_delete_adapter(struct cec_adapter *adap)
    {
    if (IS_ERR_OR_NULL(adap))
    return;
    if (adap.kthread_config)
    kthread_stop(adap.kthread_config);
    kthread_stop(adap.kthread);
    if (adap.ops.adap_free)
    adap.ops.adap_free(adap);

    rc_free_device(adap.rc);

    mutex_destroy(&adap.devnode.lock);
    mutex_destroy(&adap.devnode.lock_fhs);
    mutex_destroy(&adap.lock);
    kfree(adap);
    }
    EXPORT_SYMBOL_GPL(cec_delete_adapter);
//
// Initialise cec for linux
//
#[no_mangle]
unsafe extern "C" fn cec_devnode_init() -> int __init {
    static int __init cec_devnode_init(void)
    {
    let mut ret: c_int = alloc_chrdev_region(&cec_dev_t, 0, CEC_NUM_DEVICES, CEC_NAME);
    if (ret < 0) {
    pr_warn("Unable to allocate major\n");
    return ret;
    }

    top_cec_dir = debugfs_create_dir(CEC_NAME, core::ptr::null_mut());
    if (IS_ERR_OR_NULL(top_cec_dir)) {
    pr_warn("Failed to create debugfs " CEC_NAME " dir\n");
    top_cec_dir = core::ptr::null_mut();
    }

    ret = bus_register(&cec_bus_type);
    if (ret < 0) {
    debugfs_remove_recursive(top_cec_dir);
    unregister_chrdev_region(cec_dev_t, CEC_NUM_DEVICES);
    pr_warn("bus_register() failed\n");
    return -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cec_devnode_exit() -> void __exit {
    static void __exit cec_devnode_exit(void)
    {
    debugfs_remove_recursive(top_cec_dir);
    bus_unregister(&cec_bus_type);
    unregister_chrdev_region(cec_dev_t, CEC_NUM_DEVICES);
    }
    subsys_initcall(cec_devnode_init);
    module_exit(cec_devnode_exit)
    MODULE_AUTHOR("Hans Verkuil <hverkuil@kernel.org>");
    MODULE_DESCRIPTION("Device node registration for cec drivers");
    MODULE_LICENSE("GPL");
