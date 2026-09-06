//! Automatically rewritten from C to Rust
//! Source: drivers/soc/qcom/rpm_master_stats.c
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
// Copyright (c) 2012-2020, The Linux Foundation. All rights reserved.
// Copyright (c) 2023, Linaro Limited
//
// This driver supports what is known as "Master Stats v2" in Qualcomm
// downstream kernel terms, which seems to be the only version which has
// ever shipped, all the way from 2013 to 2023.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct master_stats_data {
    pub base: *mut void __iomem,
    pub label: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpm_master_stats {
    pub active_cores: u32,
    pub num_shutdowns: u32,
    pub shutdown_req: u64,
    pub wakeup_idx: u64,
    pub bringup_req: u64,
    pub bringup_ack: u64,
    pub /: *mut *mut u32 wakeup_reason; / 0 = "rude wakeup", 1 = scheduled wakeup,
    pub last_sleep_trans_dur: u32,
    pub last_wake_trans_dur: u32,
// Per-subsystem (*not necessarily* SoC-wide) XO shutdown stats
    pub xo_count: u32,
    pub xo_last_enter: u64,
    pub last_exit: u64,
    pub xo_total_dur: u64,
    pub __packed: },
#[no_mangle]
unsafe extern "C" fn master_stats_show(s: *mut seq_file, unused: *mut c_void) -> c_int {
    static int master_stats_show(struct seq_file *s, void *unused)
    {
    pub s->private: *mut *mut master_stats_data data =,
    pub stat: rpm_master_stats,
    pub sizeof(stat)): memcpy_fromio(&stat, data->base,,
    pub data->label): seq_printf(s, "%s:\n",,
    pub stat.shutdown_req): seq_printf(s, "\tLast shutdown @ %llu\n",,
    pub stat.bringup_req): seq_printf(s, "\tLast bringup req @ %llu\n",,
    pub stat.bringup_ack): seq_printf(s, "\tLast bringup ack @ %llu\n",,
    pub stat.wakeup_idx): seq_printf(s, "\tLast wakeup idx: %llu\n",,
    pub stat.xo_last_enter): seq_printf(s, "\tLast XO shutdown enter @ %llu\n",,
    pub stat.last_exit): seq_printf(s, "\tLast XO shutdown exit @ %llu\n",,
    pub stat.xo_total_dur): seq_printf(s, "\tXO total duration: %llu\n",,
    pub stat.last_sleep_trans_dur): seq_printf(s, "\tLast sleep transition duration: %u\n",,
    pub stat.last_wake_trans_dur): seq_printf(s, "\tLast wake transition duration: %u\n",,
    pub stat.xo_count): seq_printf(s, "\tXO shutdown count: %u\n",,
    pub stat.wakeup_reason): seq_printf(s, "\tWakeup reason: 0x%x\n",,
    pub stat.num_shutdowns): seq_printf(s, "\tShutdown count: %u\n",,
    pub stat.active_cores): seq_printf(s, "\tActive cores bitmask: 0x%x\n",,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn master_stats_probe(pdev: *mut platform_device) -> c_int {
    static int master_stats_probe(struct platform_device *pdev)
    {
    pub &pdev->dev: *mut *mut device dev =,
    pub data: *mut master_stats_data,
    pub msgram_np: *mut device_node,
    pub root: *mut *mut dentry dent,,
    pub res: resource,
    pub ret: int count, i,,
    pub "qcom,master-names"): count = of_property_count_strings(dev->of_node,,
    if (count < 0)
    pub count: return,
    pub GFP_KERNEL): *mut *mut data = devm_kcalloc(dev, count, sizeof(data),,
    if (!data)
    pub -ENOMEM: return,
    pub NULL): root = debugfs_create_dir("qcom_rpm_master_stats",,
    pub root): platform_set_drvdata(pdev,,
    pub {: for (i = 0; i < count; i++),
    pub i): msgram_np = of_parse_phandle(dev->of_node, "qcom,rpm-msg-ram",,
    if (!msgram_np) {
    return dev_err_probe(dev, -ENODEV,
    pub i): "Couldn't parse MSG RAM phandle idx %d",,
    }
//
// Purposefully skip devm_platform helpers as we're using a
// shared resource.
//
    pub &res): ret = of_address_to_resource(msgram_np, 0,,
    if (ret < 0) {
    pub ret: return,
    }
    pub resource_size(&res)): data[i].base = devm_ioremap(dev, res.start,,
    if (!data[i].base) {
    return dev_err_probe(dev, -EINVAL,
    pub i): "Could not map the MSG RAM slice idx %d!\n",,
    }
    ret = of_property_read_string_index(dev.of_node, "qcom,master-names", i,
    if (ret < 0) {
    return dev_err_probe(dev, ret,
    pub i): "Could not read name idx %d!\n",,
    }
//
// Generally it's not advised to fail on debugfs errors, but this
// driver's only job is exposing data therein.
//
    dent = debugfs_create_file(data[i].label, 0444, root,
    pub &master_stats_fops): &data[i],,
    if (IS_ERR(dent)) {
    return dev_err_probe(dev, PTR_ERR(dent),
    pub data[i].label): "Failed to create debugfs file %s!\n",,
    }
    }
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn master_stats_remove(pdev: *mut platform_device) {
    static void master_stats_remove(struct platform_device *pdev)
    {
    pub platform_get_drvdata(pdev): *mut *mut dentry root =,
    }
    static const struct of_device_id rpm_master_table[] = {
    { .compatible = "qcom,rpm-master-stats" },
    { },
}

//
// No MODULE_DEVICE_TABLE intentionally: that's a debugging module, to be
// loaded manually only.
//
    static struct platform_driver master_stats_driver = {
    .probe = master_stats_probe,
    .remove = master_stats_remove,
    .driver = {
    .name = "qcom_rpm_master_stats",
    .of_match_table = rpm_master_table,
    },
    };
    module_platform_driver(master_stats_driver);
    MODULE_DESCRIPTION("Qualcomm RPM Master Statistics driver");
    MODULE_LICENSE("GPL");
