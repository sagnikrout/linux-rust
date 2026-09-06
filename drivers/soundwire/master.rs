//! Automatically rewritten from C to Rust
//! Source: drivers/soundwire/master.c
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
// Copyright(c) 2019-2020 Intel Corporation.

//
// The 3s value for autosuspend will only be used if there are no
// devices physically attached on a bus segment. In practice enabling
// the bus operation will result in children devices become active and
// the master device will only suspend when all its children are no
// longer active.
//
pub const SDW_MASTER_SUSPEND_DELAY_MS: c_int = 3000;
//
// The sysfs for properties reflects the MIPI description as given
// in the MIPI DisCo spec
//
// Base file is:
// sdw-master-N
// |---- revision
// |---- clk_stop_modes
// |---- max_clk_freq
// |---- clk_freq
// |---- clk_gears
// |---- default_row
// |---- default_col
// |---- dynamic_shape
// |---- err_threshold
//

    static ssize_t field##_show(struct device *dev,				\
    struct device_attribute *attr,		\
    char *buf)					\
    {									\
    struct sdw_master_device *md = dev_to_sdw_master_device(dev);	\
    return sprintf(buf, format_string, md.bus.prop.field);	\
    }									\
#[no_mangle]
pub unsafe extern "C" fn DEVICE_ATTR_RO(_arg: field) -> static {
    static DEVICE_ATTR_RO(field)
    sdw_master_attr(revision, "0x%x\n");
    sdw_master_attr(clk_stop_modes, "0x%x\n");
    sdw_master_attr(max_clk_freq, "%d\n");
    sdw_master_attr(default_row, "%d\n");
    sdw_master_attr(default_col, "%d\n");
    sdw_master_attr(default_frame_rate, "%d\n");
    sdw_master_attr(dynamic_frame, "%d\n");
    sdw_master_attr(err_threshold, "%d\n");
    static ssize_t clock_frequencies_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct sdw_master_device *md = dev_to_sdw_master_device(dev);
    let mut size: isize = 0;
    int i;
    for (i = 0; i < md.bus.prop.num_clk_freq; i++)
    size += sprintf(buf + size, "%8d ",
    md.bus.prop.clk_freq[i]);
    size += sprintf(buf + size, "\n");
    return size;
    }
    static DEVICE_ATTR_RO(clock_frequencies);
    static ssize_t clock_gears_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct sdw_master_device *md = dev_to_sdw_master_device(dev);
    let mut size: isize = 0;
    int i;
    for (i = 0; i < md.bus.prop.num_clk_gears; i++)
    size += sprintf(buf + size, "%8d ",
    md.bus.prop.clk_gears[i]);
    size += sprintf(buf + size, "\n");
    return size;
    }
    static DEVICE_ATTR_RO(clock_gears);
    static struct attribute *master_node_attrs[] = {
    &dev_attr_revision.attr,
    &dev_attr_clk_stop_modes.attr,
    &dev_attr_max_clk_freq.attr,
    &dev_attr_default_row.attr,
    &dev_attr_default_col.attr,
    &dev_attr_default_frame_rate.attr,
    &dev_attr_dynamic_frame.attr,
    &dev_attr_err_threshold.attr,
    &dev_attr_clock_frequencies.attr,
    &dev_attr_clock_gears.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(master_node);
#[no_mangle]
unsafe extern "C" fn sdw_master_device_release(dev: *mut device) {
    static void sdw_master_device_release(struct device *dev)
    {
    struct sdw_master_device *md = dev_to_sdw_master_device(dev);
    kfree(md);
    }
    static const struct dev_pm_ops master_dev_pm = {
    SET_RUNTIME_PM_OPS(pm_generic_runtime_suspend,
    pm_generic_runtime_resume, core::ptr::null_mut())
    };
    const struct device_type sdw_master_type = {
    .name =		"soundwire_master",
    .release =	sdw_master_device_release,
    .pm = &master_dev_pm,
    };
//
// sdw_master_device_add() - create a Linux Master Device representation.
// @bus: SDW bus instance
// @parent: parent device
// @fwnode: firmware node handle
//
    int sdw_master_device_add(struct sdw_bus *bus, struct device *parent,
    struct fwnode_handle *fwnode)
    {
    struct sdw_master_device *md;
    int ret;
    if (!parent)
    return -EINVAL;
    md = kzalloc_obj(*md);
    if (!md)
    return -ENOMEM;
    md.dev.bus = &sdw_bus_type;
    md.dev.type = &sdw_master_type;
    md.dev.parent = parent;
    md.dev.groups = master_node_groups;
    md.dev.of_node = parent.of_node;
    md.dev.fwnode = fwnode;
    md.dev.dma_mask = parent.dma_mask;
    dev_set_name(&md.dev, "sdw-master-%d-%d", bus.controller_id, bus.link_id);
    ret = device_register(&md.dev);
    if (ret) {
    dev_err(parent, "Failed to add master: ret %d\n", ret);
//
// On err, don't free but drop ref as this will be freed
// when release method is invoked.
//
    put_device(&md.dev);
    goto device_register_err;
    }
// add shortcuts to improve code readability/compactness
    md.bus = bus;
    bus.dev = &md.dev;
    bus.md = md;
    pm_runtime_set_autosuspend_delay(&bus.md.dev, SDW_MASTER_SUSPEND_DELAY_MS);
    pm_runtime_use_autosuspend(&bus.md.dev);
    pm_runtime_mark_last_busy(&bus.md.dev);
    pm_runtime_set_active(&bus.md.dev);
    pm_runtime_enable(&bus.md.dev);
    pm_runtime_idle(&bus.md.dev);
    device_register_err:
    return ret;
    }
//
// sdw_master_device_del() - delete a Linux Master Device representation.
// @bus: bus handle
//
// This function is the dual of sdw_master_device_add()
//
#[no_mangle]
pub unsafe extern "C" fn sdw_master_device_del(bus: *mut sdw_bus) -> c_int {
    int sdw_master_device_del(struct sdw_bus *bus)
    {
    pm_runtime_disable(&bus.md.dev);
    device_unregister(bus.dev);
    return 0;
    }
