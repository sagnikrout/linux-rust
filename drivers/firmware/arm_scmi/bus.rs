//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/arm_scmi/bus.c
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
// System Control and Management Interface (SCMI) Message Protocol bus layer
//
// Copyright (C) 2018-2021 ARM Ltd.
//

    BLOCKING_NOTIFIER_HEAD(scmi_requested_devices_nh);
    EXPORT_SYMBOL_GPL(scmi_requested_devices_nh);
    static DEFINE_IDA(scmi_bus_id);
    static DEFINE_IDR(scmi_requested_devices);
// Protect access to scmi_requested_devices
    static DEFINE_MUTEX(scmi_requested_devices_mtx);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_requested_dev {
    pub id_table: *const scmi_device_id,
    pub node: list_head,
}

// Track globally the SCMI SystemPower protocol device.
    static struct scmi_device *scmi_syspower_registered;
//
// scmi_protocol_device_request  - Helper to request a device
//
// @id_table: A protocol/name pair descriptor for the device to be created.
//
// This helper let an SCMI driver request specific devices identified by the
// @id_table to be created for each active SCMI instance.
//
// The requested device name MUST NOT be already existent for this protocol;
// at first the freshly requested @id_table is annotated in the IDR table
// @scmi_requested_devices and then the requested device is advertised to any
// registered party via the @scmi_requested_devices_nh notification chain.
//
// Return: 0 on Success
//
#[no_mangle]
unsafe extern "C" fn scmi_protocol_device_request(id_table: *const scmi_device_id) -> c_int {
    static int scmi_protocol_device_request(const struct scmi_device_id *id_table)
    {
    let mut ret: c_int = 0;
    struct list_head *head, *phead = core::ptr::null_mut();
    struct scmi_requested_dev *rdev;
    pr_debug("Requesting SCMI device (%s) for protocol %x\n",
    id_table.name, id_table.protocol_id);
    if (IS_ENABLED(CONFIG_ARM_SCMI_RAW_MODE_SUPPORT) &&
    !IS_ENABLED(CONFIG_ARM_SCMI_RAW_MODE_SUPPORT_COEX)) {
    pr_warn("SCMI Raw mode active. Rejecting '%s'/0x%02X\n",
    id_table.name, id_table.protocol_id);
    return -EINVAL;
    }
//
// Find the matching protocol rdev list and then search of any
// existent equally named device...fails if any duplicate found.
//
    mutex_lock(&scmi_requested_devices_mtx);
    phead = idr_find(&scmi_requested_devices, id_table.protocol_id);
    if (phead) {
    head = phead;
    list_for_each_entry(rdev, head, node) {
    if (!strcmp(rdev.id_table.name, id_table.name)) {
    pr_err("Ignoring duplicate request [%d] %s\n",
    rdev.id_table.protocol_id,
    rdev.id_table.name);
    ret = -EINVAL;
    goto out;
    }
    }
    }
//
// No duplicate found for requested id_table, so let's create a new
// requested device entry for this new valid request.
//
    rdev = kzalloc_obj(*rdev);
    if (!rdev) {
    ret = -ENOMEM;
    goto out;
    }
    rdev.id_table = id_table;
//
// Append the new requested device table descriptor to the head of the
// related protocol list, eventually creating such head if not already
// there.
//
    if (!phead) {
    phead = kzalloc_obj(*phead);
    if (!phead) {
    kfree(rdev);
    ret = -ENOMEM;
    goto out;
    }
    INIT_LIST_HEAD(phead);
    ret = idr_alloc(&scmi_requested_devices, (void *)phead,
    id_table.protocol_id,
    id_table.protocol_id + 1, GFP_KERNEL);
    if (ret != id_table.protocol_id) {
    pr_err("Failed to save SCMI device - ret:%d\n", ret);
    kfree(rdev);
    kfree(phead);
    ret = -EINVAL;
    goto out;
    }
    ret = 0;
    }
    list_add(&rdev.node, phead);
    out:
    mutex_unlock(&scmi_requested_devices_mtx);
    if (!ret)
    blocking_notifier_call_chain(&scmi_requested_devices_nh,
    SCMI_BUS_NOTIFY_DEVICE_REQUEST,
    (void *)rdev.id_table);
    return ret;
    }
//
// scmi_protocol_device_unrequest  - Helper to unrequest a device
//
// @id_table: A protocol/name pair descriptor for the device to be unrequested.
//
// The unrequested device, described by the provided id_table, is at first
// removed from the IDR @scmi_requested_devices and then the removal is
// advertised to any registered party via the @scmi_requested_devices_nh
// notification chain.
//
#[no_mangle]
unsafe extern "C" fn scmi_protocol_device_unrequest(id_table: *const scmi_device_id) {
    static void scmi_protocol_device_unrequest(const struct scmi_device_id *id_table)
    {
    struct scmi_requested_dev *rdev, *victim = core::ptr::null_mut();
    struct list_head *phead;
    pr_debug("Unrequesting SCMI device (%s) for protocol %x\n",
    id_table.name, id_table.protocol_id);
    mutex_lock(&scmi_requested_devices_mtx);
    phead = idr_find(&scmi_requested_devices, id_table.protocol_id);
    if (phead) {
    list_for_each_entry(rdev, phead, node) {
    if (!strcmp(rdev.id_table.name, id_table.name)) {
    victim = rdev;
    list_del(&rdev.node);
    break;
    }
    }
    if (victim && list_empty(phead)) {
    idr_remove(&scmi_requested_devices,
    id_table.protocol_id);
    kfree(phead);
    }
    }
    mutex_unlock(&scmi_requested_devices_mtx);
    if (victim) {
    blocking_notifier_call_chain(&scmi_requested_devices_nh,
    SCMI_BUS_NOTIFY_DEVICE_UNREQUEST,
    (void *)victim.id_table);
    kfree(victim);
    }
    }
#[no_mangle]
unsafe extern "C" fn scmi_protocol_table_register(id_table: *const scmi_device_id) -> c_int {
    static int scmi_protocol_table_register(const struct scmi_device_id *id_table)
    {
    const struct scmi_device_id *entry;
    int ret;
    for (entry = id_table; entry.name; entry++) {
    ret = scmi_protocol_device_request(entry);
    if (ret)
    goto err_unrequest;
    }
    return 0;
    err_unrequest:
    while (entry != id_table)
    scmi_protocol_device_unrequest(--entry);
    return ret;
    }
    static void
    scmi_protocol_table_unregister(const struct scmi_device_id *id_table)
    {
    const struct scmi_device_id *entry;
    for (entry = id_table; entry.name; entry++)
    scmi_protocol_device_unrequest(entry);
    }
#[no_mangle]
unsafe extern "C" fn scmi_device_is_transport(scmi_dev: *const scmi_device) -> bool {
    static bool scmi_device_is_transport(const struct scmi_device *scmi_dev)
    {
    return !strncmp(scmi_dev.name, SCMI_TRANSPORT_DEVNAME_PREFIX,
    strlen(SCMI_TRANSPORT_DEVNAME_PREFIX));
    }
    static int __scmi_dev_match_by_id_table(struct scmi_device *scmi_dev,
    const struct scmi_device_id *id_table,
    bool skip_transport)
    {
    if (!id_table || !id_table.name)
    return 0;
    for (; id_table.protocol_id && id_table.name; id_table++)
    if (id_table.protocol_id == scmi_dev.protocol_id &&
    !(skip_transport && scmi_device_is_transport(scmi_dev)) &&
    !strcmp(id_table.name, scmi_dev.name))
    return 1;
    return 0;
    }
    static int scmi_dev_match_by_id_table(struct scmi_device *scmi_dev,
    const struct scmi_device_id *id_table)
    {
    return __scmi_dev_match_by_id_table(scmi_dev, id_table, true);
    }
    static int scmi_dev_match_id(struct scmi_device *scmi_dev,
    const struct scmi_driver *scmi_drv)
    {
    return scmi_dev_match_by_id_table(scmi_dev, scmi_drv.id_table);
    }
#[no_mangle]
unsafe extern "C" fn scmi_dev_match(dev: *mut device, drv: *const device_driver) -> c_int {
    static int scmi_dev_match(struct device *dev, const struct device_driver *drv)
    {
    const struct scmi_driver *scmi_drv = to_scmi_driver(drv);
    struct scmi_device *scmi_dev = to_scmi_dev(dev);
    return scmi_dev_match_id(scmi_dev, scmi_drv);
    }
#[no_mangle]
unsafe extern "C" fn scmi_match_by_id_table(dev: *mut device, data: *const c_void) -> c_int {
    static int scmi_match_by_id_table(struct device *dev, const void *data)
    {
    struct scmi_device *scmi_dev = to_scmi_dev(dev);
    const struct scmi_device_id *id_table = data;
    return __scmi_dev_match_by_id_table(scmi_dev, id_table, false);
    }
// Returns a device_find_child() reference which must be dropped by caller.
    static struct scmi_device *
    scmi_child_dev_find_get(struct device *parent, int prot_id, const char *name)
    {
    struct scmi_device_id id_table[2] = { 0 };
    struct device *dev;
    id_table[0].protocol_id = prot_id;
    id_table[0].name = name;
    dev = device_find_child(parent, &id_table, scmi_match_by_id_table);
    if (!dev)
    return core::ptr::null_mut();
    return to_scmi_dev(dev);
    }
#[no_mangle]
unsafe extern "C" fn scmi_dev_probe(dev: *mut device) -> c_int {
    static int scmi_dev_probe(struct device *dev)
    {
    struct scmi_driver *scmi_drv = to_scmi_driver(dev.driver);
    struct scmi_device *scmi_dev = to_scmi_dev(dev);
    if (!scmi_dev.handle)
    return -EPROBE_DEFER;
    return scmi_drv.probe(scmi_dev);
    }
#[no_mangle]
unsafe extern "C" fn scmi_dev_remove(dev: *mut device) {
    static void scmi_dev_remove(struct device *dev)
    {
    struct scmi_driver *scmi_drv = to_scmi_driver(dev.driver);
    struct scmi_device *scmi_dev = to_scmi_dev(dev);
    if (scmi_drv.remove)
    scmi_drv.remove(scmi_dev);
    }
#[no_mangle]
unsafe extern "C" fn scmi_device_uevent(dev: *const device, env: *mut kobj_uevent_env) -> c_int {
    static int scmi_device_uevent(const struct device *dev, struct kobj_uevent_env *env)
    {
    const struct scmi_device *scmi_dev = to_scmi_dev(dev);
    return add_uevent_var(env, "MODALIAS=" SCMI_UEVENT_MODALIAS_FMT,
    dev_name(&scmi_dev.dev), scmi_dev.protocol_id,
    scmi_dev.name);
    }
    static ssize_t modalias_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct scmi_device *scmi_dev = to_scmi_dev(dev);
    return sysfs_emit(buf, SCMI_UEVENT_MODALIAS_FMT,
    dev_name(&scmi_dev.dev), scmi_dev.protocol_id,
    scmi_dev.name);
    }
    static DEVICE_ATTR_RO(modalias);
    static ssize_t protocol_id_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct scmi_device *scmi_dev = to_scmi_dev(dev);
    return sprintf(buf, "0x%02x\n", scmi_dev.protocol_id);
    }
    static DEVICE_ATTR_RO(protocol_id);
    static ssize_t name_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct scmi_device *scmi_dev = to_scmi_dev(dev);
    return sprintf(buf, "%s\n", scmi_dev.name);
    }
    static DEVICE_ATTR_RO(name);
    static struct attribute *scmi_device_attributes_attrs[] = {
    &dev_attr_protocol_id.attr,
    &dev_attr_name.attr,
    &dev_attr_modalias.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(scmi_device_attributes);
#[no_mangle]
unsafe extern "C" fn scmi_pm_suspend(dev: *mut device) -> c_int {
    static int scmi_pm_suspend(struct device *dev)
    {
    const struct device_driver *drv = dev.driver;
    if (drv && drv.pm && drv.pm.suspend)
    return drv.pm.suspend(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn scmi_pm_resume(dev: *mut device) -> c_int {
    static int scmi_pm_resume(struct device *dev)
    {
    const struct device_driver *drv = dev.driver;
    if (drv && drv.pm && drv.pm.resume)
    return drv.pm.resume(dev);
    return 0;
    }
    static const struct dev_pm_ops scmi_dev_pm_ops = {
    .suspend = pm_sleep_ptr(scmi_pm_suspend),
    .resume = pm_sleep_ptr(scmi_pm_resume),
    };
    const struct bus_type scmi_bus_type = {
    .name =	"scmi_protocol",
    .match = scmi_dev_match,
    .probe = scmi_dev_probe,
    .remove = scmi_dev_remove,
    .uevent	= scmi_device_uevent,
    .dev_groups = scmi_device_attributes_groups,
    .pm = &scmi_dev_pm_ops,
    };
    EXPORT_SYMBOL_GPL(scmi_bus_type);
    int scmi_driver_register(struct scmi_driver *driver, struct module *owner,
    const char *mod_name)
    {
    int retval;
    if (!driver.probe)
    return -EINVAL;
    retval = scmi_protocol_table_register(driver.id_table);
    if (retval)
    return retval;
    driver.driver.bus = &scmi_bus_type;
    driver.driver.name = driver.name;
    driver.driver.owner = owner;
    driver.driver.mod_name = mod_name;
    retval = driver_register(&driver.driver);
    if (retval) {
    scmi_protocol_table_unregister(driver.id_table);
    return retval;
    }
    pr_debug("Registered new scmi driver %s\n", driver.name);
    return 0;
    }
    EXPORT_SYMBOL_GPL(scmi_driver_register);
#[no_mangle]
pub unsafe extern "C" fn scmi_driver_unregister(driver: *mut scmi_driver) {
    void scmi_driver_unregister(struct scmi_driver *driver)
    {
    driver_unregister(&driver.driver);
    scmi_protocol_table_unregister(driver.id_table);
    }
    EXPORT_SYMBOL_GPL(scmi_driver_unregister);
#[no_mangle]
unsafe extern "C" fn scmi_device_release_resources(scmi_dev: *mut scmi_device) {
    static void scmi_device_release_resources(struct scmi_device *scmi_dev)
    {
    if (scmi_dev.protocol_id == SCMI_PROTOCOL_SYSTEM)
    cmpxchg(&scmi_syspower_registered, scmi_dev, core::ptr::null_mut());
    if (scmi_dev.id) {
    ida_free(&scmi_bus_id, scmi_dev.id);
    scmi_dev.id = 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn scmi_device_release(dev: *mut device) {
    static void scmi_device_release(struct device *dev)
    {
    struct scmi_device *scmi_dev = to_scmi_dev(dev);
    scmi_device_release_resources(scmi_dev);
    of_node_put(dev.of_node);
    kfree_const(scmi_dev.name);
    kfree(scmi_dev);
    }
#[no_mangle]
unsafe extern "C" fn __scmi_device_destroy(scmi_dev: *mut scmi_device) {
    static void __scmi_device_destroy(struct scmi_device *scmi_dev)
    {
    pr_debug("(%pOF) Destroying SCMI device '%s' for protocol 0x%x (%s)\n",
    scmi_dev.dev.parent.of_node,
    dev_name(&scmi_dev.dev), scmi_dev.protocol_id,
    scmi_dev.name);
    device_del(&scmi_dev.dev);
    scmi_device_release_resources(scmi_dev);
    put_device(&scmi_dev.dev);
    }
    static struct scmi_device *
    __scmi_device_create(struct device_node *np, struct device *parent,
    int protocol, const char *name)
    {
    int id, retval;
    struct scmi_device *scmi_dev;
    let mut syspower: bool = (protocol == SCMI_PROTOCOL_SYSTEM);
//
// If the same protocol/name device already exist under the same parent
// (i.e. SCMI instance) just return the existent device.
// This avoids any race between the SCMI driver, creating devices for
// each DT defined protocol at probe time, and the concurrent
// registration of SCMI drivers.
//
    scmi_dev = scmi_child_dev_find_get(parent, protocol, name);
    if (scmi_dev) {
    put_device(&scmi_dev.dev);
    return scmi_dev;
    }
    scmi_dev = kzalloc_obj(*scmi_dev);
    if (!scmi_dev)
    return core::ptr::null_mut();
    scmi_dev.protocol_id = protocol;
//
// Reserve the singleton SystemPower protocol device using the device
// pointer itself, so delayed release of an older device cannot clear
// a reservation owned by a newer device.
//
    if (syspower && cmpxchg(&scmi_syspower_registered, core::ptr::null_mut(), scmi_dev)) {
    dev_warn(parent,
    "SCMI SystemPower protocol device must be unique !\n");
    kfree(scmi_dev);
    return core::ptr::null_mut();
    }
    scmi_dev.name = kstrdup_const(name ?: "unknown", GFP_KERNEL);
    if (!scmi_dev.name)
    goto free_dev;
    id = ida_alloc_min(&scmi_bus_id, 1, GFP_KERNEL);
    if (id < 0)
    goto free_name;
    scmi_dev.id = id;
    scmi_dev.dev.parent = parent;
    device_set_node(&scmi_dev.dev, of_fwnode_handle(of_node_get(np)));
    scmi_dev.dev.bus = &scmi_bus_type;
    scmi_dev.dev.release = scmi_device_release;
    dev_set_name(&scmi_dev.dev, "scmi_dev.%d", id);
    retval = device_register(&scmi_dev.dev);
    if (retval)
    goto put_dev;
    pr_debug("(%pOF) Created SCMI device '%s' for protocol 0x%x (%s)\n",
    parent.of_node, dev_name(&scmi_dev.dev), protocol, name);
    return scmi_dev;
    put_dev:
    scmi_device_release_resources(scmi_dev);
    put_device(&scmi_dev.dev);
    return core::ptr::null_mut();
    free_name:
    kfree_const(scmi_dev.name);
    free_dev:
    scmi_device_release_resources(scmi_dev);
    kfree(scmi_dev);
    return core::ptr::null_mut();
    }
    static struct scmi_device *
    _scmi_device_create(struct device_node *np, struct device *parent,
    int protocol, const char *name)
    {
    struct scmi_device *sdev;
    sdev = __scmi_device_create(np, parent, protocol, name);
    if (!sdev)
    pr_err("(%pOF) Failed to create device for protocol 0x%x (%s)\n",
    parent.of_node, protocol, name);
    return sdev;
    }
//
// scmi_device_create  - A method to create one or more SCMI devices
//
// @np: A reference to the device node to use for the new device(s)
// @parent: The parent device to use identifying a specific SCMI instance
// @protocol: The SCMI protocol to be associated with this device
// @name: The requested-name of the device to be created; this is optional
// and if no @name is provided, all the devices currently known to
// be requested on the SCMI bus for @protocol will be created.
//
// This method can be invoked to create a single well-defined device (like
// a transport device or a device requested by an SCMI driver loaded after
// the core SCMI stack has been probed), or to create all the devices currently
// known to have been requested by the loaded SCMI drivers for a specific
// protocol (typically during SCMI core protocol enumeration at probe time).
//
// Return: The created device (or one of them if @name was NOT provided and
// multiple devices were created) or NULL if no device was created;
// note that NULL indicates an error ONLY in case a specific @name
// was provided: when @name param was not provided, a number of devices
// could have been potentially created for a whole protocol, unless no
// device was found to have been requested for that specific protocol.
//
    struct scmi_device *scmi_device_create(struct device_node *np,
    struct device *parent, int protocol,
    const char *name)
    {
    struct list_head *phead;
    struct scmi_requested_dev *rdev;
    struct scmi_device *scmi_dev = core::ptr::null_mut();
    if (name)
    return _scmi_device_create(np, parent, protocol, name);
    mutex_lock(&scmi_requested_devices_mtx);
    phead = idr_find(&scmi_requested_devices, protocol);
// Nothing to do.
    if (!phead) {
    mutex_unlock(&scmi_requested_devices_mtx);
    return core::ptr::null_mut();
    }
// Walk the list of requested devices for protocol and create them
    list_for_each_entry(rdev, phead, node) {
    struct scmi_device *sdev;
    sdev = _scmi_device_create(np, parent,
    rdev.id_table.protocol_id,
    rdev.id_table.name);
    if (sdev)
    scmi_dev = sdev;
    }
    mutex_unlock(&scmi_requested_devices_mtx);
    return scmi_dev;
    }
    EXPORT_SYMBOL_GPL(scmi_device_create);
#[no_mangle]
pub unsafe extern "C" fn scmi_device_destroy(parent: *mut device, protocol: c_int, name: *const c_char) {
    void scmi_device_destroy(struct device *parent, int protocol, const char *name)
    {
    struct scmi_device *scmi_dev;
    scmi_dev = scmi_child_dev_find_get(parent, protocol, name);
    if (scmi_dev) {
    __scmi_device_destroy(scmi_dev);
    put_device(&scmi_dev.dev);
    }
    }
    EXPORT_SYMBOL_GPL(scmi_device_destroy);
#[no_mangle]
unsafe extern "C" fn __scmi_devices_unregister(dev: *mut device, data: *mut c_void) -> c_int {
    static int __scmi_devices_unregister(struct device *dev, void *data)
    {
    struct scmi_device *scmi_dev = to_scmi_dev(dev);
    __scmi_device_destroy(scmi_dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn scmi_devices_unregister() {
    static void scmi_devices_unregister(void)
    {
    bus_for_each_dev(&scmi_bus_type, core::ptr::null_mut(), core::ptr::null_mut(), __scmi_devices_unregister);
    }
#[no_mangle]
unsafe extern "C" fn scmi_bus_init() -> int __init {
    static int __init scmi_bus_init(void)
    {
    int retval;
    retval = bus_register(&scmi_bus_type);
    if (retval)
    pr_err("SCMI protocol bus register failed (%d)\n", retval);
    pr_info("SCMI protocol bus registered\n");
    return retval;
    }
    subsys_initcall(scmi_bus_init);
#[no_mangle]
unsafe extern "C" fn scmi_bus_exit() -> void __exit {
    static void __exit scmi_bus_exit(void)
    {
//
// Destroy all remaining devices: just in case the drivers were
// manually unbound and at first and then the modules unloaded.
//
    scmi_devices_unregister();
    bus_unregister(&scmi_bus_type);
    ida_destroy(&scmi_bus_id);
    }
    module_exit(scmi_bus_exit);
    MODULE_ALIAS("scmi-core");
    MODULE_AUTHOR("Sudeep Holla <sudeep.holla@arm.com>");
    MODULE_DESCRIPTION("ARM SCMI protocol bus");
    MODULE_LICENSE("GPL");
