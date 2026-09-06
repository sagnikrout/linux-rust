//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/backlight.c
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
// Backlight Lowlevel Control Abstraction
//
// Copyright (C) 2003,2004 Hewlett-Packard Company
//

//
// DOC: overview
//
// The backlight core supports implementing backlight drivers.
//
// A backlight driver registers a driver using
// devm_backlight_device_register(). The properties of the backlight
// driver such as type and max_brightness must be specified.
// When the core detect changes in for example brightness or power state
// the update_status() operation is called. The backlight driver shall
// implement this operation and use it to adjust backlight.
//
// Several sysfs attributes are provided by the backlight core::
//
// - brightness         R/W, set the requested brightness level
// - actual_brightness  RO, the brightness level used by the HW
// - max_brightness     RO, the maximum  brightness level supported
//
// See Documentation/ABI/stable/sysfs-class-backlight for the full list.
//
// The backlight can be adjusted using the sysfs interface, and
// the backlight driver may also support adjusting backlight using
// a hot-key or some other platform or firmware specific way.
//
// The driver must implement the get_brightness() operation if
// the HW do not support all the levels that can be specified in
// brightness, thus providing user-space access to the actual level
// via the actual_brightness attribute.
//
// When the backlight changes this is reported to user-space using
// an uevent connected to the actual_brightness attribute.
// When brightness is set by platform specific means, for example
// a hot-key to adjust backlight, the driver must notify the backlight
// core that brightness has changed using backlight_force_update().
//
// Display drives can control the backlight device's status using
// backlight_notify_blank() and backlight_notify_blank_all(). If this
// results in a change in the backlight state the functions call the
// update_status() operation.
//
    static struct list_head backlight_dev_list;
    static struct mutex backlight_dev_list_mutex;
    static const char *const backlight_types[] = {
    [BACKLIGHT_RAW] = "raw",
    [BACKLIGHT_PLATFORM] = "platform",
    [BACKLIGHT_FIRMWARE] = "firmware",
    };
    static const char *const backlight_scale_types[] = {
    [BACKLIGHT_SCALE_UNKNOWN]	= "unknown",
    [BACKLIGHT_SCALE_LINEAR]	= "linear",
    [BACKLIGHT_SCALE_NON_LINEAR]	= "non-linear",
    };
    void backlight_notify_blank(struct backlight_device *bd, struct device *display_dev,
    bool fb_on, bool prev_fb_on)
    {
    guard(mutex)(&bd.ops_lock);
    if (!bd.ops)
    return;
    if (bd.ops.controls_device && !bd.ops.controls_device(bd, display_dev))
    return;
    if (fb_on && (!prev_fb_on || !bd.use_count)) {
    if (!bd.use_count++) {
    bd.props.state &= ~BL_CORE_FBBLANK;
    backlight_update_status(bd);
    }
    } else if (!fb_on && prev_fb_on && bd.use_count) {
    if (!(--bd.use_count)) {
    bd.props.state |= BL_CORE_FBBLANK;
    backlight_update_status(bd);
    }
    }
    }
    EXPORT_SYMBOL(backlight_notify_blank);
#[no_mangle]
pub unsafe extern "C" fn backlight_notify_blank_all(display_dev: *mut device, fb_on: bool, prev_fb_on: bool) {
    void backlight_notify_blank_all(struct device *display_dev, bool fb_on, bool prev_fb_on)
    {
    struct backlight_device *bd;
    guard(mutex)(&backlight_dev_list_mutex);
    list_for_each_entry(bd, &backlight_dev_list, entry)
    backlight_notify_blank(bd, display_dev, fb_on, prev_fb_on);
    }
    EXPORT_SYMBOL(backlight_notify_blank_all);
    static void backlight_generate_event(struct backlight_device *bd,
    enum backlight_update_reason reason)
    {
    char *envp[2];
    switch (reason) {
    case BACKLIGHT_UPDATE_SYSFS:
    envp[0] = "SOURCE=sysfs";
    break;
    case BACKLIGHT_UPDATE_HOTKEY:
    envp[0] = "SOURCE=hotkey";
    break;
    default:
    envp[0] = "SOURCE=unknown";
    break;
    }
    envp[1] = core::ptr::null_mut();
    kobject_uevent_env(&bd.dev.kobj, KOBJ_CHANGE, envp);
    sysfs_notify(&bd.dev.kobj, core::ptr::null_mut(), "actual_brightness");
    }
    static ssize_t bl_power_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct backlight_device *bd = to_backlight_device(dev);
    return sysfs_emit(buf, "%d\n", bd.props.power);
    }
    static ssize_t bl_power_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    int rc;
    struct backlight_device *bd = to_backlight_device(dev);
    unsigned long power, old_power;
    rc = kstrtoul(buf, 0, &power);
    if (rc)
    return rc;
    rc = -ENXIO;
    mutex_lock(&bd.ops_lock);
    if (bd.ops) {
    pr_debug("set power to %lu\n", power);
    if (bd.props.power != power) {
    old_power = bd.props.power;
    bd.props.power = power;
    rc = backlight_update_status(bd);
    if (rc)
    bd.props.power = old_power;
    else
    rc = count;
    } else {
    rc = count;
    }
    }
    mutex_unlock(&bd.ops_lock);
    return rc;
    }
    static DEVICE_ATTR_RW(bl_power);
    static ssize_t brightness_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct backlight_device *bd = to_backlight_device(dev);
    return sysfs_emit(buf, "%d\n", bd.props.brightness);
    }
    int backlight_device_set_brightness(struct backlight_device *bd,
    unsigned long brightness)
    {
    let mut rc: c_int = -ENXIO;
    mutex_lock(&bd.ops_lock);
    if (bd.ops) {
    if (brightness > bd.props.max_brightness)
    rc = -EINVAL;
    else {
    pr_debug("set brightness to %lu\n", brightness);
    bd.props.brightness = brightness;
    rc = backlight_update_status(bd);
    }
    }
    mutex_unlock(&bd.ops_lock);
    backlight_generate_event(bd, BACKLIGHT_UPDATE_SYSFS);
    return rc;
    }
    EXPORT_SYMBOL(backlight_device_set_brightness);
    static ssize_t brightness_store(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    int rc;
    struct backlight_device *bd = to_backlight_device(dev);
    unsigned long brightness;
    rc = kstrtoul(buf, 0, &brightness);
    if (rc)
    return rc;
    rc = backlight_device_set_brightness(bd, brightness);
    return rc ? rc : count;
    }
    static DEVICE_ATTR_RW(brightness);
    static ssize_t type_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct backlight_device *bd = to_backlight_device(dev);
    return sysfs_emit(buf, "%s\n", backlight_types[bd.props.type]);
    }
    static DEVICE_ATTR_RO(type);
    static ssize_t max_brightness_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct backlight_device *bd = to_backlight_device(dev);
    return sysfs_emit(buf, "%d\n", bd.props.max_brightness);
    }
    static DEVICE_ATTR_RO(max_brightness);
    static ssize_t actual_brightness_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    let mut rc: c_int = -ENXIO;
    struct backlight_device *bd = to_backlight_device(dev);
    mutex_lock(&bd.ops_lock);
    if (bd.ops && bd.ops.get_brightness) {
    rc = bd.ops.get_brightness(bd);
    if (rc >= 0)
    rc = sysfs_emit(buf, "%d\n", rc);
    } else {
    rc = sysfs_emit(buf, "%d\n", bd.props.brightness);
    }
    mutex_unlock(&bd.ops_lock);
    return rc;
    }
    static DEVICE_ATTR_RO(actual_brightness);
    static ssize_t scale_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct backlight_device *bd = to_backlight_device(dev);
    if (WARN_ON(bd.props.scale > BACKLIGHT_SCALE_NON_LINEAR))
    return sysfs_emit(buf, "unknown\n");
    return sysfs_emit(buf, "%s\n", backlight_scale_types[bd.props.scale]);
    }
    static DEVICE_ATTR_RO(scale);

#[no_mangle]
unsafe extern "C" fn backlight_suspend(dev: *mut device) -> c_int {
    static int backlight_suspend(struct device *dev)
    {
    struct backlight_device *bd = to_backlight_device(dev);
    mutex_lock(&bd.ops_lock);
    if (bd.ops && bd.ops.options & BL_CORE_SUSPENDRESUME) {
    bd.props.state |= BL_CORE_SUSPENDED;
    backlight_update_status(bd);
    }
    mutex_unlock(&bd.ops_lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn backlight_resume(dev: *mut device) -> c_int {
    static int backlight_resume(struct device *dev)
    {
    struct backlight_device *bd = to_backlight_device(dev);
    mutex_lock(&bd.ops_lock);
    if (bd.ops && bd.ops.options & BL_CORE_SUSPENDRESUME) {
    bd.props.state &= ~BL_CORE_SUSPENDED;
    backlight_update_status(bd);
    }
    mutex_unlock(&bd.ops_lock);
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(backlight_class_dev_pm_ops, backlight_suspend,
    backlight_resume);
#[no_mangle]
unsafe extern "C" fn bl_device_release(dev: *mut device) {
    static void bl_device_release(struct device *dev)
    {
    struct backlight_device *bd = to_backlight_device(dev);
    kfree(bd);
    }
    static struct attribute *bl_device_attrs[] = {
    &dev_attr_bl_power.attr,
    &dev_attr_brightness.attr,
    &dev_attr_actual_brightness.attr,
    &dev_attr_max_brightness.attr,
    &dev_attr_scale.attr,
    &dev_attr_type.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(bl_device);
    static const struct class backlight_class = {
    .name = "backlight",
    .dev_groups = bl_device_groups,
    .pm = &backlight_class_dev_pm_ops,
    };
//
// backlight_force_update - tell the backlight subsystem that hardware state
// has changed
// @bd: the backlight device to update
// @reason: reason for update
//
// Updates the internal state of the backlight in response to a hardware event,
// and generates an uevent to notify userspace. A backlight driver shall call
// backlight_force_update() when the backlight is changed using, for example,
// a hot-key. The updated brightness is read using get_brightness() and the
// brightness value is reported using an uevent.
//
    void backlight_force_update(struct backlight_device *bd,
    enum backlight_update_reason reason)
    {
    int brightness;
    mutex_lock(&bd.ops_lock);
    if (bd.ops && bd.ops.get_brightness) {
    brightness = bd.ops.get_brightness(bd);
    if (brightness >= 0)
    bd.props.brightness = brightness;
    else
    dev_err(&bd.dev,
    "Could not update brightness from device: %pe\n",
    ERR_PTR(brightness));
    }
    mutex_unlock(&bd.ops_lock);
    backlight_generate_event(bd, reason);
    }
    EXPORT_SYMBOL(backlight_force_update);
// deprecated - use devm_backlight_device_register()
    struct backlight_device *backlight_device_register(const char *name,
    struct device *parent, void *devdata, const struct backlight_ops *ops,
    const struct backlight_properties *props)
    {
    struct backlight_device *new_bd;
    int rc;
    pr_debug("backlight_device_register: name=%s\n", name);
    new_bd = kzalloc_obj(struct backlight_device);
    if (!new_bd)
    return ERR_PTR(-ENOMEM);
    mutex_init(&new_bd.update_lock);
    mutex_init(&new_bd.ops_lock);
    new_bd.dev.class = &backlight_class;
    new_bd.dev.parent = parent;
    new_bd.dev.release = bl_device_release;
    dev_set_name(&new_bd.dev, "%s", name);
    dev_set_drvdata(&new_bd.dev, devdata);
// Set default properties
    if (props) {
    memcpy(&new_bd.props, props,
    sizeof(struct backlight_properties));
    if (props.type <= 0 || props.type >= BACKLIGHT_TYPE_MAX) {
    WARN(1, "%s: invalid backlight type", name);
    new_bd.props.type = BACKLIGHT_RAW;
    }
    } else {
    new_bd.props.type = BACKLIGHT_RAW;
    }
    rc = device_register(&new_bd.dev);
    if (rc) {
    put_device(&new_bd.dev);
    return ERR_PTR(rc);
    }
    new_bd.ops = ops;

    mutex_lock(&pmac_backlight_mutex);
    if (!pmac_backlight)
    pmac_backlight = new_bd;
    mutex_unlock(&pmac_backlight_mutex);

    mutex_lock(&backlight_dev_list_mutex);
    list_add(&new_bd.entry, &backlight_dev_list);
    mutex_unlock(&backlight_dev_list_mutex);
    return new_bd;
    }
    EXPORT_SYMBOL(backlight_device_register);
// backlight_device_get_by_type - find first backlight device of a type
// @type: the type of backlight device
//
// Look up the first backlight device of the specified type
//
// RETURNS:
//
// Pointer to backlight device if any was found. Otherwise NULL.
//
    struct backlight_device *backlight_device_get_by_type(enum backlight_type type)
    {
    let mut found: bool = false;
    struct backlight_device *bd;
    mutex_lock(&backlight_dev_list_mutex);
    list_for_each_entry(bd, &backlight_dev_list, entry) {
    if (bd.props.type == type) {
    found = true;
    break;
    }
    }
    mutex_unlock(&backlight_dev_list_mutex);
    return found ? bd : core::ptr::null_mut();
    }
    EXPORT_SYMBOL(backlight_device_get_by_type);
//
// backlight_device_get_by_name - Get backlight device by name
// @name: Device name
//
// This function looks up a backlight device by its name. It obtains a reference
// on the backlight device and it is the caller's responsibility to drop the
// reference by calling put_device().
//
// Returns:
// A pointer to the backlight device if found, otherwise NULL.
//
    struct backlight_device *backlight_device_get_by_name(const char *name)
    {
    struct device *dev;
    dev = class_find_device_by_name(&backlight_class, name);
    return dev ? to_backlight_device(dev) : core::ptr::null_mut();
    }
    EXPORT_SYMBOL(backlight_device_get_by_name);
// deprecated - use devm_backlight_device_unregister()
#[no_mangle]
pub unsafe extern "C" fn backlight_device_unregister(bd: *mut backlight_device) {
    void backlight_device_unregister(struct backlight_device *bd)
    {
    if (!bd)
    return;
    mutex_lock(&backlight_dev_list_mutex);
    list_del(&bd.entry);
    mutex_unlock(&backlight_dev_list_mutex);

    mutex_lock(&pmac_backlight_mutex);
    if (pmac_backlight == bd)
    pmac_backlight = core::ptr::null_mut();
    mutex_unlock(&pmac_backlight_mutex);

    mutex_lock(&bd.ops_lock);
    bd.ops = core::ptr::null_mut();
    mutex_unlock(&bd.ops_lock);
    device_unregister(&bd.dev);
    }
    EXPORT_SYMBOL(backlight_device_unregister);
#[no_mangle]
unsafe extern "C" fn devm_backlight_device_release(dev: *mut device, res: *mut c_void) {
    static void devm_backlight_device_release(struct device *dev, void *res)
    {
    struct backlight_device *backlight = *(struct backlight_device **)res;
    backlight_device_unregister(backlight);
    }
    static int devm_backlight_device_match(struct device *dev, void *res,
    void *data)
    {
    struct backlight_device **r = res;
    return *r == data;
    }
//
// devm_backlight_device_register - register a new backlight device
// @dev: the device to register
// @name: the name of the device
// @parent: a pointer to the parent device (often the same as @dev)
// @devdata: an optional pointer to be stored for private driver use
// @ops: the backlight operations structure
// @props: the backlight properties
//
// Creates and registers new backlight device. When a backlight device
// is registered the configuration must be specified in the @props
// parameter. See description of &backlight_properties.
//
// RETURNS:
//
// struct backlight on success, or an ERR_PTR on error
//
    struct backlight_device *devm_backlight_device_register(struct device *dev,
    const char *name, struct device *parent, void *devdata,
    const struct backlight_ops *ops,
    const struct backlight_properties *props)
    {
    struct backlight_device **ptr, *backlight;
    ptr = devres_alloc(devm_backlight_device_release, sizeof(*ptr),
    GFP_KERNEL);
    if (!ptr)
    return ERR_PTR(-ENOMEM);
    backlight = backlight_device_register(name, parent, devdata, ops,
    props);
    if (!IS_ERR(backlight)) {
// ptr = backlight;
    devres_add(dev, ptr);
    } else {
    devres_free(ptr);
    }
    return backlight;
    }
    EXPORT_SYMBOL(devm_backlight_device_register);
//
// devm_backlight_device_unregister - unregister backlight device
// @dev: the device to unregister
// @bd: the backlight device to unregister
//
// Deallocates a backlight allocated with devm_backlight_device_register().
// Normally this function will not need to be called and the resource management
// code will ensure that the resources are freed.
//
    void devm_backlight_device_unregister(struct device *dev,
    struct backlight_device *bd)
    {
    int rc;
    rc = devres_release(dev, devm_backlight_device_release,
    devm_backlight_device_match, bd);
    WARN_ON(rc);
    }
    EXPORT_SYMBOL(devm_backlight_device_unregister);

#[no_mangle]
unsafe extern "C" fn of_parent_match(dev: *mut device, data: *const c_void) -> c_int {
    static int of_parent_match(struct device *dev, const void *data)
    {
    return dev.parent && dev.parent.of_node == data;
    }
//
// of_find_backlight_by_node() - find backlight device by device-tree node
// @node: device-tree node of the backlight device
//
// Returns a pointer to the backlight device corresponding to the given DT
// node or NULL if no such backlight device exists or if the device hasn't
// been probed yet.
//
// This function obtains a reference on the backlight device and it is the
// caller's responsibility to drop the reference by calling put_device() on
// the backlight device's .dev field.
//
    struct backlight_device *of_find_backlight_by_node(struct device_node *node)
    {
    struct device *dev;
    dev = class_find_device(&backlight_class, core::ptr::null_mut(), node, of_parent_match);
    return dev ? to_backlight_device(dev) : core::ptr::null_mut();
    }
    EXPORT_SYMBOL(of_find_backlight_by_node);

    static struct backlight_device *of_find_backlight(struct device *dev)
    {
    struct backlight_device *bd = core::ptr::null_mut();
    struct device_node *np;
    if (!dev)
    return core::ptr::null_mut();
    if (IS_ENABLED(CONFIG_OF) && dev.of_node) {
    np = of_parse_phandle(dev.of_node, "backlight", 0);
    if (np) {
    bd = of_find_backlight_by_node(np);
    of_node_put(np);
    if (!bd)
    return ERR_PTR(-EPROBE_DEFER);
    }
    }
    return bd;
    }
#[no_mangle]
unsafe extern "C" fn devm_backlight_release(data: *mut c_void) {
    static void devm_backlight_release(void *data)
    {
    struct backlight_device *bd = data;
    put_device(&bd.dev);
    }
//
// devm_of_find_backlight - find backlight for a device
// @dev: the device
//
// This function looks for a property named 'backlight' on the DT node
// connected to @dev and looks up the backlight device. The lookup is
// device managed so the reference to the backlight device is automatically
// dropped on driver detach.
//
// RETURNS:
//
// A pointer to the backlight device if found.
// Error pointer -EPROBE_DEFER if the DT property is set, but no backlight
// device is found. NULL if there's no backlight property.
//
    struct backlight_device *devm_of_find_backlight(struct device *dev)
    {
    struct backlight_device *bd;
    int ret;
    bd = of_find_backlight(dev);
    if (IS_ERR_OR_NULL(bd))
    return bd;
    ret = devm_add_action_or_reset(dev, devm_backlight_release, bd);
    if (ret)
    return ERR_PTR(ret);
    return bd;
    }
    EXPORT_SYMBOL(devm_of_find_backlight);
#[no_mangle]
unsafe extern "C" fn backlight_class_exit() -> void __exit {
    static void __exit backlight_class_exit(void)
    {
    class_unregister(&backlight_class);
    }
#[no_mangle]
unsafe extern "C" fn backlight_class_init() -> int __init {
    static int __init backlight_class_init(void)
    {
    int ret;
    ret = class_register(&backlight_class);
    if (ret) {
    pr_warn("Unable to create backlight class; errno = %d\n", ret);
    return ret;
    }
    INIT_LIST_HEAD(&backlight_dev_list);
    mutex_init(&backlight_dev_list_mutex);
    return 0;
    }
//
// if this is compiled into the kernel, we need to ensure that the
// class is registered before users of the class try to register lcd's
//
    postcore_initcall(backlight_class_init);
    module_exit(backlight_class_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Jamey Hicks <jamey.hicks@hp.com>, Andrew Zabolotny <zap@homelink.ru>");
    MODULE_DESCRIPTION("Backlight Lowlevel Control Abstraction");
