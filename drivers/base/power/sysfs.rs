//! Automatically rewritten from C to Rust
//! Source: drivers/base/power/sysfs.c
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
// sysfs entries for device PM

//
// control - Report/change current runtime PM setting of the device
//
// Runtime power management of a device can be blocked with the help of
// this attribute.  All devices have one of the following two values for
// the power/control file:
//
// + "auto\n" to allow the device to be power managed at run time;
// + "on\n" to prevent the device from being power managed at run time;
//
// The default for all devices is "auto", which means that devices may be
// subject to automatic power management, depending on their drivers.
// Changing this attribute to "on" prevents the driver from power managing
// the device at run time.  Doing that while the device is suspended causes
// it to be woken up.
//
// wakeup - Report/change current wakeup option for device
//
// Some devices support "wakeup" events, which are hardware signals
// used to activate devices from suspended or low power states.  Such
// devices have one of three values for the sysfs power/wakeup file:
//
// + "enabled\n" to issue the events;
// + "disabled\n" not to do so; or
// + "\n" for temporary or permanent inability to issue wakeup.
//
// (For example, unconfigured USB devices can't issue wakeups.)
//
// Familiar examples of devices that can issue wakeup events include
// keyboards and mice (both PS2 and USB styles), power buttons, modems,
// "Wake-On-LAN" Ethernet links, GPIO lines, and more.  Some events
// will wake the entire system from a suspend state; others may just
// wake up the device (if the system as a whole is already active).
// Some wakeup events use normal IRQ lines; other use special out
// of band signaling.
//
// It is the responsibility of device drivers to enable (or disable)
// wakeup signaling as part of changing device power states, respecting
// the policy choices provided through the driver model.
//
// Devices may not be able to generate wakeup events from all power
// states.  Also, the events may be ignored in some configurations;
// for example, they might need help from other devices that aren't
// active, or which may have wakeup disabled.  Some drivers rely on
// wakeup events internally (unless they are disabled), keeping
// their hardware in low power modes whenever they're unused.  This
// saves runtime power, without requiring system-wide sleep states.
//
// async - Report/change current async suspend setting for the device
//
// Asynchronous suspend and resume of the device during system-wide power
// state transitions can be enabled by writing "enabled" to this file.
// Analogously, if "disabled" is written to this file, the device will be
// suspended and resumed synchronously.
//
// All devices have one of the following two values for power/async:
//
// + "enabled\n" to permit the asynchronous suspend/resume of the device;
// + "disabled\n" to forbid it;
//
// NOTE: It generally is unsafe to permit the asynchronous suspend/resume
// of a device unless it is certain that all of the PM dependencies of the
// device are known to the PM core.  However, for some devices this
// attribute is set to "enabled" by bus type code or device drivers and in
// that cases it should be safe to leave the default value.
//
// autosuspend_delay_ms - Report/change a device's autosuspend_delay value
//
// Some drivers don't want to carry out a runtime suspend as soon as a
// device becomes idle; they want it always to remain idle for some period
// of time before suspending it.  This period is the autosuspend_delay
// value (expressed in milliseconds) and it can be controlled by the user.
// If the value is negative then the device will never be runtime
// suspended.
//
// NOTE: The autosuspend_delay_ms attribute and the autosuspend_delay
// value are used only if the driver calls pm_runtime_use_autosuspend().
//
// wakeup_count - Report the number of wakeup events related to the device
//
    const char power_group_name[] = "power";
    EXPORT_SYMBOL_GPL(power_group_name);
    static const char ctrl_auto[] = "auto";
    static const char ctrl_on[] = "on";
    static ssize_t control_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    return sysfs_emit(buf, "%s\n",
    dev.power.runtime_auto ? ctrl_auto : ctrl_on);
    }
    static ssize_t control_store(struct device * dev, struct device_attribute *attr,
    const char * buf, size_t n)
    {
    device_lock(dev);
    if (sysfs_streq(buf, ctrl_auto))
    pm_runtime_allow(dev);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: sysfs_streq(buf, _arg: ctrl_on)) -> else {
    else if (sysfs_streq(buf, ctrl_on))
    pm_runtime_forbid(dev);
    else
    n = -EINVAL;
    device_unlock(dev);
    return n;
    }
    static DEVICE_ATTR_RW(control);
    static ssize_t runtime_active_time_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    let mut tmp: u64 = pm_runtime_active_time(dev);
    do_div(tmp, NSEC_PER_MSEC);
    return sysfs_emit(buf, "%llu\n", tmp);
    }
    static DEVICE_ATTR_RO(runtime_active_time);
    static ssize_t runtime_suspended_time_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    let mut tmp: u64 = pm_runtime_suspended_time(dev);
    do_div(tmp, NSEC_PER_MSEC);
    return sysfs_emit(buf, "%llu\n", tmp);
    }
    static DEVICE_ATTR_RO(runtime_suspended_time);
    static ssize_t runtime_status_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    const char *output;
    if (dev.power.runtime_error) {
    output = "error";
    } else if (dev.power.disable_depth) {
    output = "unsupported";
    } else {
    switch (dev.power.runtime_status) {
    case RPM_SUSPENDED:
    output = "suspended";
    break;
    case RPM_SUSPENDING:
    output = "suspending";
    break;
    case RPM_RESUMING:
    output = "resuming";
    break;
    case RPM_ACTIVE:
    output = "active";
    break;
    default:
    return -EIO;
    }
    }
    return sysfs_emit(buf, "%s\n", output);
    }
    static DEVICE_ATTR_RO(runtime_status);
    static ssize_t autosuspend_delay_ms_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    if (!dev.power.use_autosuspend)
    return -EIO;
    return sysfs_emit(buf, "%d\n", dev.power.autosuspend_delay);
    }
    static ssize_t autosuspend_delay_ms_store(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t n)
    {
    long delay;
    if (!dev.power.use_autosuspend)
    return -EIO;
    if (kstrtol(buf, 10, &delay) != 0 || delay != (int) delay)
    return -EINVAL;
    device_lock(dev);
    pm_runtime_set_autosuspend_delay(dev, delay);
    device_unlock(dev);
    return n;
    }
    static DEVICE_ATTR_RW(autosuspend_delay_ms);
    static ssize_t pm_qos_resume_latency_us_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    let mut value: i32 = dev_pm_qos_requested_resume_latency(dev);
    if (value == 0)
    return sysfs_emit(buf, "n/a\n");
    if (value == PM_QOS_RESUME_LATENCY_NO_CONSTRAINT)
    value = 0;
    return sysfs_emit(buf, "%d\n", value);
    }
    static ssize_t pm_qos_resume_latency_us_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t n)
    {
    s32 value;
    int ret;
    if (!kstrtos32(buf, 0, &value)) {
//
// Prevent users from writing negative or "no constraint" values
// directly.
//
    if (value < 0 || value == PM_QOS_RESUME_LATENCY_NO_CONSTRAINT)
    return -EINVAL;
    if (value == 0)
    value = PM_QOS_RESUME_LATENCY_NO_CONSTRAINT;
    } else if (sysfs_streq(buf, "n/a")) {
    value = 0;
    } else {
    return -EINVAL;
    }
    ret = dev_pm_qos_update_request(dev.power.qos.resume_latency_req,
    value);
    return ret < 0 ? ret : n;
    }
    static DEVICE_ATTR_RW(pm_qos_resume_latency_us);
    static ssize_t pm_qos_latency_tolerance_us_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    let mut value: i32 = dev_pm_qos_get_user_latency_tolerance(dev);
    if (value < 0)
    return sysfs_emit(buf, "%s\n", "auto");
    if (value == PM_QOS_LATENCY_ANY)
    return sysfs_emit(buf, "%s\n", "any");
    return sysfs_emit(buf, "%d\n", value);
    }
    static ssize_t pm_qos_latency_tolerance_us_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t n)
    {
    s32 value;
    int ret;
    if (kstrtos32(buf, 0, &value) == 0) {
// Users can't write negative values directly
    if (value < 0)
    return -EINVAL;
    } else {
    if (sysfs_streq(buf, "auto"))
    value = PM_QOS_LATENCY_TOLERANCE_NO_CONSTRAINT;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: sysfs_streq(buf, _arg: "any")) -> else {
    else if (sysfs_streq(buf, "any"))
    value = PM_QOS_LATENCY_ANY;
    else
    return -EINVAL;
    }
    ret = dev_pm_qos_update_user_latency_tolerance(dev, value);
    return ret < 0 ? ret : n;
    }
    static DEVICE_ATTR_RW(pm_qos_latency_tolerance_us);
    static ssize_t pm_qos_no_power_off_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
#[no_mangle]
pub unsafe extern "C" fn sysfs_emit(_arg: buf, _arg: "%d\n", _arg: !!(dev_pm_qos_requested_flags(dev) -> return {
    return sysfs_emit(buf, "%d\n", !!(dev_pm_qos_requested_flags(dev)
    & PM_QOS_FLAG_NO_POWER_OFF));
    }
    static ssize_t pm_qos_no_power_off_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t n)
    {
    int ret;
    if (kstrtoint(buf, 0, &ret))
    return -EINVAL;
    if (ret != 0 && ret != 1)
    return -EINVAL;
    ret = dev_pm_qos_update_flags(dev, PM_QOS_FLAG_NO_POWER_OFF, ret);
    return ret < 0 ? ret : n;
    }
    static DEVICE_ATTR_RW(pm_qos_no_power_off);

    static const char _enabled[] = "enabled";
    static const char _disabled[] = "disabled";
    static ssize_t wakeup_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
#[no_mangle]
pub unsafe extern "C" fn sysfs_emit(_arg: buf, _arg: "%s\n", _arg: device_can_wakeup(dev) -> return {
    return sysfs_emit(buf, "%s\n", device_can_wakeup(dev)
    ? (device_may_wakeup(dev) ? _enabled : _disabled)
    : "");
    }
    static ssize_t wakeup_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t n)
    {
    if (!device_can_wakeup(dev))
    return -EINVAL;
    if (sysfs_streq(buf, _enabled))
    device_set_wakeup_enable(dev, 1);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: sysfs_streq(buf, _arg: _disabled)) -> else {
    else if (sysfs_streq(buf, _disabled))
    device_set_wakeup_enable(dev, 0);
    else
    return -EINVAL;
    return n;
    }
    static DEVICE_ATTR_RW(wakeup);
    static ssize_t wakeup_count_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    unsigned long count;
    let mut enabled: bool = false;
    spin_lock_irq(&dev.power.lock);
    if (dev.power.wakeup) {
    count = dev.power.wakeup.wakeup_count;
    enabled = true;
    }
    spin_unlock_irq(&dev.power.lock);
    if (!enabled)
    return sysfs_emit(buf, "\n");
    return sysfs_emit(buf, "%lu\n", count);
    }
    static DEVICE_ATTR_RO(wakeup_count);
    static ssize_t wakeup_active_count_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    unsigned long count;
    let mut enabled: bool = false;
    spin_lock_irq(&dev.power.lock);
    if (dev.power.wakeup) {
    count = dev.power.wakeup.active_count;
    enabled = true;
    }
    spin_unlock_irq(&dev.power.lock);
    if (!enabled)
    return sysfs_emit(buf, "\n");
    return sysfs_emit(buf, "%lu\n", count);
    }
    static DEVICE_ATTR_RO(wakeup_active_count);
    static ssize_t wakeup_abort_count_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    unsigned long count;
    let mut enabled: bool = false;
    spin_lock_irq(&dev.power.lock);
    if (dev.power.wakeup) {
    count = dev.power.wakeup.wakeup_count;
    enabled = true;
    }
    spin_unlock_irq(&dev.power.lock);
    if (!enabled)
    return sysfs_emit(buf, "\n");
    return sysfs_emit(buf, "%lu\n", count);
    }
    static DEVICE_ATTR_RO(wakeup_abort_count);
    static ssize_t wakeup_expire_count_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    unsigned long count;
    let mut enabled: bool = false;
    spin_lock_irq(&dev.power.lock);
    if (dev.power.wakeup) {
    count = dev.power.wakeup.expire_count;
    enabled = true;
    }
    spin_unlock_irq(&dev.power.lock);
    if (!enabled)
    return sysfs_emit(buf, "\n");
    return sysfs_emit(buf, "%lu\n", count);
    }
    static DEVICE_ATTR_RO(wakeup_expire_count);
    static ssize_t wakeup_active_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    unsigned int active;
    let mut enabled: bool = false;
    spin_lock_irq(&dev.power.lock);
    if (dev.power.wakeup) {
    active = dev.power.wakeup.active;
    enabled = true;
    }
    spin_unlock_irq(&dev.power.lock);
    if (!enabled)
    return sysfs_emit(buf, "\n");
    return sysfs_emit(buf, "%u\n", active);
    }
    static DEVICE_ATTR_RO(wakeup_active);
    static ssize_t wakeup_total_time_ms_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    s64 msec;
    let mut enabled: bool = false;
    spin_lock_irq(&dev.power.lock);
    if (dev.power.wakeup) {
    msec = ktime_to_ms(dev.power.wakeup.total_time);
    enabled = true;
    }
    spin_unlock_irq(&dev.power.lock);
    if (!enabled)
    return sysfs_emit(buf, "\n");
    return sysfs_emit(buf, "%lld\n", msec);
    }
    static DEVICE_ATTR_RO(wakeup_total_time_ms);
    static ssize_t wakeup_max_time_ms_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    s64 msec;
    let mut enabled: bool = false;
    spin_lock_irq(&dev.power.lock);
    if (dev.power.wakeup) {
    msec = ktime_to_ms(dev.power.wakeup.max_time);
    enabled = true;
    }
    spin_unlock_irq(&dev.power.lock);
    if (!enabled)
    return sysfs_emit(buf, "\n");
    return sysfs_emit(buf, "%lld\n", msec);
    }
    static DEVICE_ATTR_RO(wakeup_max_time_ms);
    static ssize_t wakeup_last_time_ms_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    s64 msec;
    let mut enabled: bool = false;
    spin_lock_irq(&dev.power.lock);
    if (dev.power.wakeup) {
    msec = ktime_to_ms(dev.power.wakeup.last_time);
    enabled = true;
    }
    spin_unlock_irq(&dev.power.lock);
    if (!enabled)
    return sysfs_emit(buf, "\n");
    return sysfs_emit(buf, "%lld\n", msec);
    }
    static DEVICE_ATTR_RO(wakeup_last_time_ms);

    static ssize_t wakeup_prevent_sleep_time_ms_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    s64 msec;
    let mut enabled: bool = false;
    spin_lock_irq(&dev.power.lock);
    if (dev.power.wakeup) {
    msec = ktime_to_ms(dev.power.wakeup.prevent_sleep_time);
    enabled = true;
    }
    spin_unlock_irq(&dev.power.lock);
    if (!enabled)
    return sysfs_emit(buf, "\n");
    return sysfs_emit(buf, "%lld\n", msec);
    }
    static DEVICE_ATTR_RO(wakeup_prevent_sleep_time_ms);

    static inline int dpm_sysfs_wakeup_change_owner(struct device *dev, kuid_t kuid,
    kgid_t kgid)
    {
    if (dev.power.wakeup && dev.power.wakeup.dev)
    return device_change_owner(dev.power.wakeup.dev, kuid, kgid);
    return 0;
    }

    static inline int dpm_sysfs_wakeup_change_owner(struct device *dev, kuid_t kuid,
    kgid_t kgid)
    {
    return 0;
    }

    static ssize_t runtime_usage_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    return sysfs_emit(buf, "%d\n", atomic_read(&dev.power.usage_count));
    }
    static DEVICE_ATTR_RO(runtime_usage);
    static ssize_t runtime_active_kids_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    return sysfs_emit(buf, "%d\n", dev.power.ignore_children ?
    0 : atomic_read(&dev.power.child_count));
    }
    static DEVICE_ATTR_RO(runtime_active_kids);
    static ssize_t runtime_enabled_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    const char *output;
    if (dev.power.disable_depth && !dev.power.runtime_auto)
    output = "disabled & forbidden";
#[no_mangle]
pub unsafe extern "C" fn if(_arg: dev->power.disable_depth) -> else {
    else if (dev.power.disable_depth)
    output = "disabled";
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !dev->power.runtime_auto) -> else {
    else if (!dev.power.runtime_auto)
    output = "forbidden";
    else
    output = "enabled";
    return sysfs_emit(buf, "%s\n", output);
    }
    static DEVICE_ATTR_RO(runtime_enabled);

    static ssize_t async_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    return sysfs_emit(buf, "%s\n",
    device_async_suspend_enabled(dev) ?
    _enabled : _disabled);
    }
    static ssize_t async_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t n)
    {
    if (sysfs_streq(buf, _enabled))
    device_enable_async_suspend(dev);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: sysfs_streq(buf, _arg: _disabled)) -> else {
    else if (sysfs_streq(buf, _disabled))
    device_disable_async_suspend(dev);
    else
    return -EINVAL;
    return n;
    }
    static DEVICE_ATTR_RW(async);

    static struct attribute *power_attrs[] = {

    &dev_attr_async.attr,

    core::ptr::null_mut(),
    };
    static const struct attribute_group pm_attr_group = {
    .name	= power_group_name,
    .attrs	= power_attrs,
    };
    static struct attribute *wakeup_attrs[] = {

    &dev_attr_wakeup.attr,
    &dev_attr_wakeup_count.attr,
    &dev_attr_wakeup_active_count.attr,
    &dev_attr_wakeup_abort_count.attr,
    &dev_attr_wakeup_expire_count.attr,
    &dev_attr_wakeup_active.attr,
    &dev_attr_wakeup_total_time_ms.attr,
    &dev_attr_wakeup_max_time_ms.attr,
    &dev_attr_wakeup_last_time_ms.attr,

    &dev_attr_wakeup_prevent_sleep_time_ms.attr,

    core::ptr::null_mut(),
    };
    static const struct attribute_group pm_wakeup_attr_group = {
    .name	= power_group_name,
    .attrs	= wakeup_attrs,
    };
    static struct attribute *runtime_attrs[] = {
    &dev_attr_runtime_status.attr,
    &dev_attr_control.attr,
    &dev_attr_runtime_suspended_time.attr,
    &dev_attr_runtime_active_time.attr,
    &dev_attr_autosuspend_delay_ms.attr,

    &dev_attr_runtime_usage.attr,
    &dev_attr_runtime_active_kids.attr,
    &dev_attr_runtime_enabled.attr,

    core::ptr::null_mut(),
    };
    static const struct attribute_group pm_runtime_attr_group = {
    .name	= power_group_name,
    .attrs	= runtime_attrs,
    };
    static struct attribute *pm_qos_resume_latency_attrs[] = {
    &dev_attr_pm_qos_resume_latency_us.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group pm_qos_resume_latency_attr_group = {
    .name	= power_group_name,
    .attrs	= pm_qos_resume_latency_attrs,
    };
    static struct attribute *pm_qos_latency_tolerance_attrs[] = {
    &dev_attr_pm_qos_latency_tolerance_us.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group pm_qos_latency_tolerance_attr_group = {
    .name	= power_group_name,
    .attrs	= pm_qos_latency_tolerance_attrs,
    };
    static struct attribute *pm_qos_flags_attrs[] = {
    &dev_attr_pm_qos_no_power_off.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group pm_qos_flags_attr_group = {
    .name	= power_group_name,
    .attrs	= pm_qos_flags_attrs,
    };
#[no_mangle]
pub unsafe extern "C" fn dpm_sysfs_add(dev: *mut device) -> c_int {
    int dpm_sysfs_add(struct device *dev)
    {
    int rc;
// No need to create PM sysfs if explicitly disabled.
    if (device_pm_not_required(dev))
    return 0;
    rc = sysfs_create_group(&dev.kobj, &pm_attr_group);
    if (rc)
    return rc;
    if (!pm_runtime_has_no_callbacks(dev)) {
    rc = sysfs_merge_group(&dev.kobj, &pm_runtime_attr_group);
    if (rc)
    goto err_out;
    }
    if (device_can_wakeup(dev)) {
    rc = sysfs_merge_group(&dev.kobj, &pm_wakeup_attr_group);
    if (rc)
    goto err_runtime;
    }
    if (dev.power.set_latency_tolerance) {
    rc = sysfs_merge_group(&dev.kobj,
    &pm_qos_latency_tolerance_attr_group);
    if (rc)
    goto err_wakeup;
    }
    rc = pm_wakeup_source_sysfs_add(dev);
    if (rc)
    goto err_latency;
    return 0;
    err_latency:
    sysfs_unmerge_group(&dev.kobj, &pm_qos_latency_tolerance_attr_group);
    err_wakeup:
    sysfs_unmerge_group(&dev.kobj, &pm_wakeup_attr_group);
    err_runtime:
    sysfs_unmerge_group(&dev.kobj, &pm_runtime_attr_group);
    err_out:
    sysfs_remove_group(&dev.kobj, &pm_attr_group);
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn dpm_sysfs_change_owner(dev: *mut device, kuid: kuid_t, kgid: kgid_t) -> c_int {
    int dpm_sysfs_change_owner(struct device *dev, kuid_t kuid, kgid_t kgid)
    {
    int rc;
    if (device_pm_not_required(dev))
    return 0;
    rc = sysfs_group_change_owner(&dev.kobj, &pm_attr_group, kuid, kgid);
    if (rc)
    return rc;
    if (!pm_runtime_has_no_callbacks(dev)) {
    rc = sysfs_group_change_owner(
    &dev.kobj, &pm_runtime_attr_group, kuid, kgid);
    if (rc)
    return rc;
    }
    if (device_can_wakeup(dev)) {
    rc = sysfs_group_change_owner(&dev.kobj, &pm_wakeup_attr_group,
    kuid, kgid);
    if (rc)
    return rc;
    rc = dpm_sysfs_wakeup_change_owner(dev, kuid, kgid);
    if (rc)
    return rc;
    }
    if (dev.power.set_latency_tolerance) {
    rc = sysfs_group_change_owner(
    &dev.kobj, &pm_qos_latency_tolerance_attr_group, kuid,
    kgid);
    if (rc)
    return rc;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn wakeup_sysfs_add(dev: *mut device) -> c_int {
    int wakeup_sysfs_add(struct device *dev)
    {
    let mut ret: c_int = sysfs_merge_group(&dev.kobj, &pm_wakeup_attr_group);
    if (!ret)
    kobject_uevent(&dev.kobj, KOBJ_CHANGE);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn wakeup_sysfs_remove(dev: *mut device) {
    void wakeup_sysfs_remove(struct device *dev)
    {
    sysfs_unmerge_group(&dev.kobj, &pm_wakeup_attr_group);
    kobject_uevent(&dev.kobj, KOBJ_CHANGE);
    }
#[no_mangle]
pub unsafe extern "C" fn pm_qos_sysfs_add_resume_latency(dev: *mut device) -> c_int {
    int pm_qos_sysfs_add_resume_latency(struct device *dev)
    {
    return sysfs_merge_group(&dev.kobj, &pm_qos_resume_latency_attr_group);
    }
#[no_mangle]
pub unsafe extern "C" fn pm_qos_sysfs_remove_resume_latency(dev: *mut device) {
    void pm_qos_sysfs_remove_resume_latency(struct device *dev)
    {
    sysfs_unmerge_group(&dev.kobj, &pm_qos_resume_latency_attr_group);
    }
#[no_mangle]
pub unsafe extern "C" fn pm_qos_sysfs_add_flags(dev: *mut device) -> c_int {
    int pm_qos_sysfs_add_flags(struct device *dev)
    {
    return sysfs_merge_group(&dev.kobj, &pm_qos_flags_attr_group);
    }
#[no_mangle]
pub unsafe extern "C" fn pm_qos_sysfs_remove_flags(dev: *mut device) {
    void pm_qos_sysfs_remove_flags(struct device *dev)
    {
    sysfs_unmerge_group(&dev.kobj, &pm_qos_flags_attr_group);
    }
#[no_mangle]
pub unsafe extern "C" fn pm_qos_sysfs_add_latency_tolerance(dev: *mut device) -> c_int {
    int pm_qos_sysfs_add_latency_tolerance(struct device *dev)
    {
    return sysfs_merge_group(&dev.kobj,
    &pm_qos_latency_tolerance_attr_group);
    }
#[no_mangle]
pub unsafe extern "C" fn pm_qos_sysfs_remove_latency_tolerance(dev: *mut device) {
    void pm_qos_sysfs_remove_latency_tolerance(struct device *dev)
    {
    sysfs_unmerge_group(&dev.kobj, &pm_qos_latency_tolerance_attr_group);
    }
#[no_mangle]
pub unsafe extern "C" fn rpm_sysfs_remove(dev: *mut device) {
    void rpm_sysfs_remove(struct device *dev)
    {
    sysfs_unmerge_group(&dev.kobj, &pm_runtime_attr_group);
    }
#[no_mangle]
pub unsafe extern "C" fn dpm_sysfs_remove(dev: *mut device) {
    void dpm_sysfs_remove(struct device *dev)
    {
    if (device_pm_not_required(dev))
    return;
    sysfs_unmerge_group(&dev.kobj, &pm_qos_latency_tolerance_attr_group);
    dev_pm_qos_constraints_destroy(dev);
    rpm_sysfs_remove(dev);
    sysfs_unmerge_group(&dev.kobj, &pm_wakeup_attr_group);
    sysfs_remove_group(&dev.kobj, &pm_attr_group);
    }
