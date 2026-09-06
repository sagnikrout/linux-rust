//! Automatically rewritten from C to Rust
//! Source: drivers/xen/xen-balloon.c
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


//
// Xen balloon driver - enables returning/claiming memory to/from Xen.
//
// Copyright (c) 2003, B Dragovic
// Copyright (c) 2003-2004, M Williamson, K Fraser
// Copyright (c) 2005 Dan M. Smith, IBM Corporation
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License version 2
// as published by the Free Software Foundation; or, when distributed
// separately from the Linux kernel or incorporated into other
// software packages, subject to the following license:
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this source file (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy, modify,
// merge, publish, distribute, sublicense, and/or sell copies of the Software,
// and to permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.
//

    let mut xen_saved_max_mem_size: u64 = 0;

    static struct device balloon_dev;
    static int register_balloon(struct device *dev);
// React to a change in the target key
    static void watch_target(struct xenbus_watch *watch,
    const char *path, const char *token)
    {
    unsigned long long new_target, static_max;
    int err;
    static bool watch_fired;
    static long target_diff;

// The balloon driver will take care of adding memory now.
    if (xen_saved_max_mem_size)
    max_mem_size = xen_saved_max_mem_size;

    err = xenbus_scanf(XBT_NIL, "memory", "target", "%llu", &new_target);
    if (err != 1) {
// This is ok (for domain0 at least) - so just return
    return;
    }
// The given memory/target value is in KiB, so it needs converting to
// pages. PAGE_SHIFT converts bytes to pages, hence PAGE_SHIFT - 10.
//
    new_target >>= PAGE_SHIFT - 10;
    if (!watch_fired) {
    watch_fired = true;
    if ((xenbus_scanf(XBT_NIL, "memory", "static-max",
    "%llu", &static_max) == 1) ||
    (xenbus_scanf(XBT_NIL, "memory", "memory_static_max",
    "%llu", &static_max) == 1))
    static_max >>= PAGE_SHIFT - 10;
    else
    static_max = balloon_stats.current_pages;
    target_diff = (xen_pv_domain() || xen_initial_domain()) ? 0
    : static_max - balloon_stats.target_pages;
    }
    balloon_set_new_target(new_target - target_diff);
    }
    static struct xenbus_watch target_watch = {
    .node = "memory/target",
    .callback = watch_target,
    };
    static int balloon_init_watcher(struct notifier_block *notifier,
    unsigned long event,
    void *data)
    {
    int err;
    err = register_xenbus_watch(&target_watch);
    if (err)
    pr_err("Failed to set balloon watcher\n");
    return NOTIFY_DONE;
    }
    static struct notifier_block xenstore_notifier = {
    .notifier_call = balloon_init_watcher,
    };
#[no_mangle]
pub unsafe extern "C" fn xen_balloon_init() {
    void xen_balloon_init(void)
    {
    register_balloon(&balloon_dev);
    register_xenstore_notifier(&xenstore_notifier);
    }
    EXPORT_SYMBOL_GPL(xen_balloon_init);

    static ssize_t name##_show(struct device *dev,			\
    struct device_attribute *attr,	\
    char *buf)				\
    {								\
    return sysfs_emit(buf, format, ##args);			\
    }								\
#[no_mangle]
pub unsafe extern "C" fn DEVICE_ATTR_RO(_arg: name) -> static {
    static DEVICE_ATTR_RO(name)
    BALLOON_SHOW(current_kb, "%lu\n", PAGES2KB(balloon_stats.current_pages));
    BALLOON_SHOW(low_kb, "%lu\n", PAGES2KB(balloon_stats.balloon_low));
    BALLOON_SHOW(high_kb, "%lu\n", PAGES2KB(balloon_stats.balloon_high));
    static DEVICE_ULONG_ATTR(schedule_delay, 0444, balloon_stats.schedule_delay);
    static DEVICE_ULONG_ATTR(max_schedule_delay, 0644, balloon_stats.max_schedule_delay);
    static DEVICE_ULONG_ATTR(retry_count, 0444, balloon_stats.retry_count);
    static DEVICE_ULONG_ATTR(max_retry_count, 0644, balloon_stats.max_retry_count);
    static DEVICE_BOOL_ATTR(scrub_pages, 0644, xen_scrub_pages);
    static ssize_t target_kb_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    return sysfs_emit(buf, "%lu\n", PAGES2KB(balloon_stats.target_pages));
    }
    static ssize_t target_kb_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    char *endchar;
    unsigned long long target_bytes;
    if (!capable(CAP_SYS_ADMIN))
    return -EPERM;
    target_bytes = simple_strtoull(buf, &endchar, 0) * 1024;
    balloon_set_new_target(target_bytes >> PAGE_SHIFT);
    return count;
    }
    static DEVICE_ATTR_RW(target_kb);
    static ssize_t target_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    return sysfs_emit(buf, "%llu\n",
    (unsigned long long)balloon_stats.target_pages
    << PAGE_SHIFT);
    }
    static ssize_t target_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    char *endchar;
    unsigned long long target_bytes;
    if (!capable(CAP_SYS_ADMIN))
    return -EPERM;
    target_bytes = memparse(buf, &endchar);
    balloon_set_new_target(target_bytes >> PAGE_SHIFT);
    return count;
    }
    static DEVICE_ATTR_RW(target);
    static struct attribute *balloon_attrs[] = {
    &dev_attr_target_kb.attr,
    &dev_attr_target.attr,
    &dev_attr_schedule_delay.attr.attr,
    &dev_attr_max_schedule_delay.attr.attr,
    &dev_attr_retry_count.attr.attr,
    &dev_attr_max_retry_count.attr.attr,
    &dev_attr_scrub_pages.attr.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group balloon_group = {
    .attrs = balloon_attrs
    };
    static struct attribute *balloon_info_attrs[] = {
    &dev_attr_current_kb.attr,
    &dev_attr_low_kb.attr,
    &dev_attr_high_kb.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group balloon_info_group = {
    .name = "info",
    .attrs = balloon_info_attrs
    };
    static const struct attribute_group *balloon_groups[] = {
    &balloon_group,
    &balloon_info_group,
    core::ptr::null_mut()
    };
    static const struct bus_type balloon_subsys = {
    .name = BALLOON_CLASS_NAME,
    .dev_name = BALLOON_CLASS_NAME,
    };
#[no_mangle]
unsafe extern "C" fn register_balloon(dev: *mut device) -> c_int {
    static int register_balloon(struct device *dev)
    {
    int error;
    error = subsys_system_register(&balloon_subsys, core::ptr::null_mut());
    if (error)
    return error;
    dev.id = 0;
    dev.bus = &balloon_subsys;
    dev.groups = balloon_groups;
    error = device_register(dev);
    if (error) {
    bus_unregister(&balloon_subsys);
    return error;
    }
    return 0;
    }
