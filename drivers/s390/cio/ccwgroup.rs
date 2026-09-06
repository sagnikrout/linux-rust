//! Automatically rewritten from C to Rust
//! Source: drivers/s390/cio/ccwgroup.c
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
// bus driver for ccwgroup
//
// Copyright IBM Corp. 2002, 2012
//
// Author(s): Arnd Bergmann (arndb@de.ibm.com)
// Cornelia Huck (cornelia.huck@de.ibm.com)
//

pub const CCW_BUS_ID_SIZE: c_int = 10;
// In Linux 2.4, we had a channel device layer called "chandev"
// that did all sorts of obscure stuff for networking devices.
// This is another driver that serves as a replacement for just
// one of its functions, namely the translation of single subchannels
// to devices that use multiple subchannels.
//
    static const struct bus_type ccwgroup_bus_type;
#[no_mangle]
unsafe extern "C" fn __ccwgroup_remove_symlinks(gdev: *mut ccwgroup_device) {
    static void __ccwgroup_remove_symlinks(struct ccwgroup_device *gdev)
    {
    int i;
    char str[16];
    for (i = 0; i < gdev.count; i++) {
    scnprintf(str, sizeof(str), "cdev%d", i);
    sysfs_remove_link(&gdev.dev.kobj, str);
    sysfs_remove_link(&gdev.cdev[i].dev.kobj, "group_device");
    }
    }
//
// ccwgroup_set_online() - enable a ccwgroup device
// @gdev: target ccwgroup device
//
// This function attempts to put the ccwgroup device into the online state.
// Returns:
// %0 on success and a negative error value on failure.
//
#[no_mangle]
pub unsafe extern "C" fn ccwgroup_set_online(gdev: *mut ccwgroup_device) -> c_int {
    int ccwgroup_set_online(struct ccwgroup_device *gdev)
    {
    struct ccwgroup_driver *gdrv = to_ccwgroupdrv(gdev.dev.driver);
    let mut ret: c_int = -EINVAL;
    if (atomic_cmpxchg(&gdev.onoff, 0, 1) != 0)
    return -EAGAIN;
    if (gdev.state == CCWGROUP_ONLINE)
    goto out;
    if (gdrv.set_online)
    ret = gdrv.set_online(gdev);
    if (ret)
    goto out;
    gdev.state = CCWGROUP_ONLINE;
    out:
    atomic_set(&gdev.onoff, 0);
    return ret;
    }
    EXPORT_SYMBOL(ccwgroup_set_online);
//
// ccwgroup_set_offline() - disable a ccwgroup device
// @gdev: target ccwgroup device
// @call_gdrv: Call the registered gdrv set_offline function
//
// This function attempts to put the ccwgroup device into the offline state.
// Returns:
// %0 on success and a negative error value on failure.
//
#[no_mangle]
pub unsafe extern "C" fn ccwgroup_set_offline(gdev: *mut ccwgroup_device, call_gdrv: bool) -> c_int {
    int ccwgroup_set_offline(struct ccwgroup_device *gdev, bool call_gdrv)
    {
    struct ccwgroup_driver *gdrv = to_ccwgroupdrv(gdev.dev.driver);
    let mut ret: c_int = -EINVAL;
    if (atomic_cmpxchg(&gdev.onoff, 0, 1) != 0)
    return -EAGAIN;
    if (gdev.state == CCWGROUP_OFFLINE)
    goto out;
    if (!call_gdrv) {
    ret = 0;
    goto offline;
    }
    if (gdrv.set_offline)
    ret = gdrv.set_offline(gdev);
    if (ret)
    goto out;
    offline:
    gdev.state = CCWGROUP_OFFLINE;
    out:
    atomic_set(&gdev.onoff, 0);
    return ret;
    }
    EXPORT_SYMBOL(ccwgroup_set_offline);
    static ssize_t ccwgroup_online_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct ccwgroup_device *gdev = to_ccwgroupdev(dev);
    unsigned long value;
    int ret;
    device_lock(dev);
    if (!dev.driver) {
    ret = -EINVAL;
    goto out;
    }
    ret = kstrtoul(buf, 0, &value);
    if (ret)
    goto out;
    if (value == 1)
    ret = ccwgroup_set_online(gdev);
#[no_mangle]
pub unsafe extern "C" fn if(0: value ==) -> else {
    else if (value == 0)
    ret = ccwgroup_set_offline(gdev, true);
    else
    ret = -EINVAL;
    out:
    device_unlock(dev);
    return (ret == 0) ? count : ret;
    }
    static ssize_t ccwgroup_online_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct ccwgroup_device *gdev = to_ccwgroupdev(dev);
    int online;
    online = (gdev.state == CCWGROUP_ONLINE) ? 1 : 0;
    return sysfs_emit(buf, "%d\n", online);
    }
//
// Provide an 'ungroup' attribute so the user can remove group devices no
// longer needed or accidentally created. Saves memory :)
//
#[no_mangle]
unsafe extern "C" fn ccwgroup_ungroup(gdev: *mut ccwgroup_device) {
    static void ccwgroup_ungroup(struct ccwgroup_device *gdev)
    {
    mutex_lock(&gdev.reg_mutex);
    if (device_is_registered(&gdev.dev)) {
    __ccwgroup_remove_symlinks(gdev);
    device_unregister(&gdev.dev);
    }
    mutex_unlock(&gdev.reg_mutex);
    }
    static ssize_t ccwgroup_ungroup_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct ccwgroup_device *gdev = to_ccwgroupdev(dev);
    let mut rc: c_int = 0;
// Prevent concurrent online/offline processing and ungrouping.
    if (atomic_cmpxchg(&gdev.onoff, 0, 1) != 0)
    return -EAGAIN;
    if (gdev.state != CCWGROUP_OFFLINE) {
    rc = -EINVAL;
    goto out;
    }
    if (device_remove_file_self(dev, attr))
    ccwgroup_ungroup(gdev);
    else
    rc = -ENODEV;
    out:
    if (rc) {
// Release onoff "lock" when ungrouping failed.
    atomic_set(&gdev.onoff, 0);
    return rc;
    }
    return count;
    }
    static DEVICE_ATTR(ungroup, 0200, core::ptr::null_mut(), ccwgroup_ungroup_store);
    static DEVICE_ATTR(online, 0644, ccwgroup_online_show, ccwgroup_online_store);
    static struct attribute *ccwgroup_dev_attrs[] = {
    &dev_attr_online.attr,
    &dev_attr_ungroup.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(ccwgroup_dev);
#[no_mangle]
unsafe extern "C" fn ccwgroup_ungroup_workfn(work: *mut work_struct) {
    static void ccwgroup_ungroup_workfn(struct work_struct *work)
    {
    struct ccwgroup_device *gdev =
    container_of(work, struct ccwgroup_device, ungroup_work);
    ccwgroup_ungroup(gdev);
    put_device(&gdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn ccwgroup_release(dev: *mut device) {
    static void ccwgroup_release(struct device *dev)
    {
    struct ccwgroup_device *gdev = to_ccwgroupdev(dev);
    unsigned int i;
    for (i = 0; i < gdev.count; i++) {
    struct ccw_device *cdev = gdev.cdev[i];
    unsigned long flags;
    if (cdev) {
    spin_lock_irqsave(cdev.ccwlock, flags);
    if (dev_get_drvdata(&cdev.dev) == gdev)
    dev_set_drvdata(&cdev.dev, core::ptr::null_mut());
    spin_unlock_irqrestore(cdev.ccwlock, flags);
    put_device(&cdev.dev);
    }
    }
    kfree(gdev);
    }
#[no_mangle]
unsafe extern "C" fn __ccwgroup_create_symlinks(gdev: *mut ccwgroup_device) -> c_int {
    static int __ccwgroup_create_symlinks(struct ccwgroup_device *gdev)
    {
    char str[16];
    int i, rc;
    for (i = 0; i < gdev.count; i++) {
    rc = sysfs_create_link(&gdev.cdev[i].dev.kobj,
    &gdev.dev.kobj, "group_device");
    if (rc) {
    while (i--)
    sysfs_remove_link(&gdev.cdev[i].dev.kobj,
    "group_device");
    return rc;
    }
    }
    for (i = 0; i < gdev.count; i++) {
    scnprintf(str, sizeof(str), "cdev%d", i);
    rc = sysfs_create_link(&gdev.dev.kobj,
    &gdev.cdev[i].dev.kobj, str);
    if (rc) {
    while (i--) {
    scnprintf(str, sizeof(str), "cdev%d", i);
    sysfs_remove_link(&gdev.dev.kobj, str);
    }
    for (i = 0; i < gdev.count; i++)
    sysfs_remove_link(&gdev.cdev[i].dev.kobj,
    "group_device");
    return rc;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __get_next_id(buf: *const c_char, id: *mut ccw_dev_id) -> c_int {
    static int __get_next_id(const char **buf, struct ccw_dev_id *id)
    {
    unsigned int cssid, ssid, devno;
    let mut ret: c_int = 0, len;
    char *start, *end;
    start = (char *)*buf;
    end = strchr(start, ',');
    if (!end) {
// Last entry. Strip trailing newline, if applicable.
    end = strchr(start, '\n');
    if (end)
// end = '\0';
    len = strlen(start) + 1;
    } else {
    len = end - start + 1;
    end++;
    }
    if (len <= CCW_BUS_ID_SIZE) {
    if (sscanf(start, "%2x.%1x.%04x", &cssid, &ssid, &devno) != 3)
    ret = -EINVAL;
    } else
    ret = -EINVAL;
    if (!ret) {
    id.ssid = ssid;
    id.devno = devno;
    }
// buf = end;
    return ret;
    }
//
// ccwgroup_create_dev() - create and register a ccw group device
// @parent: parent device for the new device
// @gdrv: driver for the new group device
// @num_devices: number of slave devices
// @buf: buffer containing comma separated bus ids of slave devices
//
// Create and register a new ccw group device as a child of @parent. Slave
// devices are obtained from the list of bus ids given in @buf.
// Returns:
// %0 on success and an error code on failure.
// Context:
// non-atomic
//
    int ccwgroup_create_dev(struct device *parent, struct ccwgroup_driver *gdrv,
    int num_devices, const char *buf)
    {
    struct ccwgroup_device *gdev;
    struct ccw_dev_id dev_id;
    int rc, i;
    if (num_devices < 1)
    return -EINVAL;
    gdev = kzalloc_flex(*gdev, cdev, num_devices);
    if (!gdev)
    return -ENOMEM;
    atomic_set(&gdev.onoff, 0);
    mutex_init(&gdev.reg_mutex);
    mutex_lock(&gdev.reg_mutex);
    INIT_WORK(&gdev.ungroup_work, ccwgroup_ungroup_workfn);
    gdev.count = num_devices;
    gdev.dev.bus = &ccwgroup_bus_type;
    gdev.dev.parent = parent;
    gdev.dev.release = ccwgroup_release;
    device_initialize(&gdev.dev);
    for (i = 0; i < num_devices && buf; i++) {
    rc = __get_next_id(&buf, &dev_id);
    if (rc != 0)
    goto error;
    gdev.cdev[i] = get_ccwdev_by_dev_id(&dev_id);
//
// All devices have to be of the same type in
// order to be grouped.
//
    if (!gdev.cdev[i] || !gdev.cdev[i].drv ||
    gdev.cdev[i].drv != gdev.cdev[0].drv ||
    gdev.cdev[i].id.driver_info !=
    gdev.cdev[0].id.driver_info) {
    rc = -EINVAL;
    goto error;
    }
// Don't allow a device to belong to more than one group.
    spin_lock_irq(gdev.cdev[i].ccwlock);
    if (dev_get_drvdata(&gdev.cdev[i].dev)) {
    spin_unlock_irq(gdev.cdev[i].ccwlock);
    rc = -EINVAL;
    goto error;
    }
    dev_set_drvdata(&gdev.cdev[i].dev, gdev);
    spin_unlock_irq(gdev.cdev[i].ccwlock);
    }
// Check for sufficient number of bus ids.
    if (i < num_devices) {
    rc = -EINVAL;
    goto error;
    }
// Check for trailing stuff.
    if (i == num_devices && buf && strlen(buf) > 0) {
    rc = -EINVAL;
    goto error;
    }
// Check if the devices are bound to the required ccw driver.
    if (gdrv && gdrv.ccw_driver &&
    gdev.cdev[0].drv != gdrv.ccw_driver) {
    rc = -EINVAL;
    goto error;
    }
    dev_set_name(&gdev.dev, "%s", dev_name(&gdev.cdev[0].dev));
    if (gdrv) {
    gdev.dev.driver = &gdrv.driver;
    rc = gdrv.setup ? gdrv.setup(gdev) : 0;
    if (rc)
    goto error;
    }
    rc = device_add(&gdev.dev);
    if (rc)
    goto error;
    rc = __ccwgroup_create_symlinks(gdev);
    if (rc) {
    device_del(&gdev.dev);
    goto error;
    }
    mutex_unlock(&gdev.reg_mutex);
    return 0;
    error:
    mutex_unlock(&gdev.reg_mutex);
    put_device(&gdev.dev);
    return rc;
    }
    EXPORT_SYMBOL(ccwgroup_create_dev);
    static int ccwgroup_notifier(struct notifier_block *nb, unsigned long action,
    void *data)
    {
    struct ccwgroup_device *gdev = to_ccwgroupdev(data);
    if (action == BUS_NOTIFY_UNBOUND_DRIVER) {
    get_device(&gdev.dev);
    schedule_work(&gdev.ungroup_work);
    }
    return NOTIFY_OK;
    }
    static struct notifier_block ccwgroup_nb = {
    .notifier_call = ccwgroup_notifier
    };
#[no_mangle]
unsafe extern "C" fn init_ccwgroup() -> int __init {
    static int __init init_ccwgroup(void)
    {
    int ret;
    ret = bus_register(&ccwgroup_bus_type);
    if (ret)
    return ret;
    ret = bus_register_notifier(&ccwgroup_bus_type, &ccwgroup_nb);
    if (ret)
    bus_unregister(&ccwgroup_bus_type);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cleanup_ccwgroup() -> void __exit {
    static void __exit cleanup_ccwgroup(void)
    {
    bus_unregister_notifier(&ccwgroup_bus_type, &ccwgroup_nb);
    bus_unregister(&ccwgroup_bus_type);
    }
    module_init(init_ccwgroup);
    module_exit(cleanup_ccwgroup);
// driver stuff
#[no_mangle]
unsafe extern "C" fn ccwgroup_remove(dev: *mut device) {
    static void ccwgroup_remove(struct device *dev)
    {
    struct ccwgroup_device *gdev = to_ccwgroupdev(dev);
    struct ccwgroup_driver *gdrv = to_ccwgroupdrv(dev.driver);
    if (gdrv.remove)
    gdrv.remove(gdev);
    }
#[no_mangle]
unsafe extern "C" fn ccwgroup_shutdown(dev: *mut device) {
    static void ccwgroup_shutdown(struct device *dev)
    {
    struct ccwgroup_device *gdev = to_ccwgroupdev(dev);
    struct ccwgroup_driver *gdrv = to_ccwgroupdrv(dev.driver);
    if (!dev.driver)
    return;
    if (gdrv.shutdown)
    gdrv.shutdown(gdev);
    }
    static const struct bus_type ccwgroup_bus_type = {
    .name   = "ccwgroup",
    .dev_groups = ccwgroup_dev_groups,
    .remove = ccwgroup_remove,
    .shutdown = ccwgroup_shutdown,
    };
#[no_mangle]
pub unsafe extern "C" fn dev_is_ccwgroup(dev: *mut device) -> bool {
    bool dev_is_ccwgroup(struct device *dev)
    {
    return dev.bus == &ccwgroup_bus_type;
    }
    EXPORT_SYMBOL(dev_is_ccwgroup);
//
// ccwgroup_driver_register() - register a ccw group driver
// @cdriver: driver to be registered
//
// This function is mainly a wrapper around driver_register().
//
#[no_mangle]
pub unsafe extern "C" fn ccwgroup_driver_register(cdriver: *mut ccwgroup_driver) -> c_int {
    int ccwgroup_driver_register(struct ccwgroup_driver *cdriver)
    {
// register our new driver with the core
    cdriver.driver.bus = &ccwgroup_bus_type;
    return driver_register(&cdriver.driver);
    }
    EXPORT_SYMBOL(ccwgroup_driver_register);
//
// ccwgroup_driver_unregister() - deregister a ccw group driver
// @cdriver: driver to be deregistered
//
// This function is mainly a wrapper around driver_unregister().
//
#[no_mangle]
pub unsafe extern "C" fn ccwgroup_driver_unregister(cdriver: *mut ccwgroup_driver) {
    void ccwgroup_driver_unregister(struct ccwgroup_driver *cdriver)
    {
    driver_unregister(&cdriver.driver);
    }
    EXPORT_SYMBOL(ccwgroup_driver_unregister);
//
// ccwgroup_probe_ccwdev() - probe function for slave devices
// @cdev: ccw device to be probed
//
// This is a dummy probe function for ccw devices that are slave devices in
// a ccw group device.
// Returns:
// always %0
//
#[no_mangle]
pub unsafe extern "C" fn ccwgroup_probe_ccwdev(cdev: *mut ccw_device) -> c_int {
    int ccwgroup_probe_ccwdev(struct ccw_device *cdev)
    {
    return 0;
    }
    EXPORT_SYMBOL(ccwgroup_probe_ccwdev);
//
// ccwgroup_remove_ccwdev() - remove function for slave devices
// @cdev: ccw device to be removed
//
// This is a remove function for ccw devices that are slave devices in a ccw
// group device. It sets the ccw device offline and also deregisters the
// embedding ccw group device.
//
#[no_mangle]
pub unsafe extern "C" fn ccwgroup_remove_ccwdev(cdev: *mut ccw_device) {
    void ccwgroup_remove_ccwdev(struct ccw_device *cdev)
    {
    struct ccwgroup_device *gdev;
// Ignore offlining errors, device is gone anyway.
    ccw_device_set_offline(cdev);
// If one of its devices is gone, the whole group is done for.
    spin_lock_irq(cdev.ccwlock);
    gdev = dev_get_drvdata(&cdev.dev);
    if (!gdev) {
    spin_unlock_irq(cdev.ccwlock);
    return;
    }
// Get ccwgroup device reference for local processing.
    get_device(&gdev.dev);
    spin_unlock_irq(cdev.ccwlock);
// Unregister group device.
    ccwgroup_ungroup(gdev);
// Release ccwgroup device reference for local processing.
    put_device(&gdev.dev);
    }
    EXPORT_SYMBOL(ccwgroup_remove_ccwdev);
    MODULE_DESCRIPTION("ccwgroup bus driver");
    MODULE_LICENSE("GPL");
