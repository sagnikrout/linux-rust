//! Automatically rewritten from C to Rust
//! Source: drivers/hwtracing/coresight/coresight-dummy.c
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
// Copyright (c) 2023 Qualcomm Innovation Center, Inc. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dummy_drvdata {
    pub dev: *mut device,
    pub csdev: *mut coresight_device,
    pub traceid: u8,
}

    static int dummy_source_enable(struct coresight_device *csdev,
    struct perf_event *event, enum cs_mode mode,
    __maybe_unused struct coresight_path *path)
    {
    if (!coresight_take_mode(csdev, mode))
    return -EBUSY;
    dev_dbg(csdev.dev.parent, "Dummy source enabled\n");
    return 0;
    }
    static void dummy_source_disable(struct coresight_device *csdev,
    struct perf_event *event)
    {
    coresight_set_mode(csdev, CS_MODE_DISABLED);
    dev_dbg(csdev.dev.parent, "Dummy source disabled\n");
    }
    static int dummy_source_trace_id(struct coresight_device *csdev, __maybe_unused enum cs_mode mode,
    __maybe_unused struct coresight_device *sink)
    {
    struct dummy_drvdata *drvdata;
    drvdata = dev_get_drvdata(csdev.dev.parent);
    return drvdata.traceid;
    }
    static int dummy_sink_enable(struct coresight_device *csdev, enum cs_mode mode,
    struct coresight_path *path)
    {
    dev_dbg(csdev.dev.parent, "Dummy sink enabled\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dummy_sink_disable(csdev: *mut coresight_device) -> c_int {
    static int dummy_sink_disable(struct coresight_device *csdev)
    {
    dev_dbg(csdev.dev.parent, "Dummy sink disabled\n");
    return 0;
    }
    static const struct coresight_ops_source dummy_source_ops = {
    .enable	= dummy_source_enable,
    .disable = dummy_source_disable,
    };
    static const struct coresight_ops dummy_source_cs_ops = {
    .trace_id	= dummy_source_trace_id,
    .source_ops	= &dummy_source_ops,
    };
    static const struct coresight_ops_sink dummy_sink_ops = {
    .enable	= dummy_sink_enable,
    .disable = dummy_sink_disable,
    };
    static const struct coresight_ops dummy_sink_cs_ops = {
    .sink_ops = &dummy_sink_ops,
    };
// User can get the trace id of dummy source from this node.
    static ssize_t traceid_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    unsigned long val;
    struct dummy_drvdata *drvdata = dev_get_drvdata(dev.parent);
    val = drvdata.traceid;
    return sysfs_emit(buf, "%#lx\n", val);
    }
    static DEVICE_ATTR_RO(traceid);
    static struct attribute *coresight_dummy_attrs[] = {
    &dev_attr_traceid.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group coresight_dummy_group = {
    .attrs = coresight_dummy_attrs,
    };
    static const struct attribute_group *coresight_dummy_groups[] = {
    &coresight_dummy_group,
    core::ptr::null_mut(),
    };
#[no_mangle]
unsafe extern "C" fn dummy_probe(pdev: *mut platform_device) -> c_int {
    static int dummy_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *node = dev.of_node;
    struct coresight_platform_data *pdata;
    struct dummy_drvdata *drvdata;
    let mut desc: coresight_desc = { 0 };
    let mut ret: c_int = 0, trace_id = 0;
    drvdata = devm_kzalloc(dev, sizeof(*drvdata), GFP_KERNEL);
    if (!drvdata)
    return -ENOMEM;
    if (of_device_is_compatible(node, "arm,coresight-dummy-source")) {
    desc.name = coresight_alloc_device_name("dummy_source", dev);
    if (!desc.name)
    return -ENOMEM;
    desc.type = CORESIGHT_DEV_TYPE_SOURCE;
    desc.subtype.source_subtype =
    CORESIGHT_DEV_SUBTYPE_SOURCE_OTHERS;
    desc.ops = &dummy_source_cs_ops;
    desc.groups = coresight_dummy_groups;
    ret = coresight_get_static_trace_id(dev, &trace_id);
    if (!ret) {
// Get the static id if id is set in device tree.
    ret = coresight_trace_id_get_static_system_id(trace_id);
    if (ret < 0) {
    dev_err(dev, "Fail to get static id.\n");
    return ret;
    }
    } else {
// Get next available id if id is not set in device tree.
    trace_id = coresight_trace_id_get_system_id();
    if (trace_id < 0) {
    ret = trace_id;
    return ret;
    }
    }
    drvdata.traceid = (u8)trace_id;
    } else if (of_device_is_compatible(node, "arm,coresight-dummy-sink")) {
    desc.name = coresight_alloc_device_name("dummy_sink", dev);
    if (!desc.name)
    return -ENOMEM;
    desc.type = CORESIGHT_DEV_TYPE_SINK;
    desc.subtype.sink_subtype = CORESIGHT_DEV_SUBTYPE_SINK_DUMMY;
    desc.ops = &dummy_sink_cs_ops;
    } else {
    dev_err(dev, "Device type not set\n");
    return -EINVAL;
    }
    pdata = coresight_get_platform_data(dev);
    if (IS_ERR(pdata)) {
    ret = PTR_ERR(pdata);
    goto free_id;
    }
    pdev.dev.platform_data = pdata;
    drvdata.dev = &pdev.dev;
    platform_set_drvdata(pdev, drvdata);
    desc.pdata = pdev.dev.platform_data;
    desc.dev = &pdev.dev;
    drvdata.csdev = coresight_register(&desc);
    if (IS_ERR(drvdata.csdev)) {
    ret = PTR_ERR(drvdata.csdev);
    goto free_id;
    }
    pm_runtime_enable(dev);
    dev_dbg(dev, "Dummy device initialized\n");
    ret = 0;
    goto out;
    free_id:
    if (IS_VALID_CS_TRACE_ID(drvdata.traceid))
    coresight_trace_id_put_system_id(drvdata.traceid);
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dummy_remove(pdev: *mut platform_device) {
    static void dummy_remove(struct platform_device *pdev)
    {
    struct dummy_drvdata *drvdata = platform_get_drvdata(pdev);
    struct device *dev = &pdev.dev;
    if (IS_VALID_CS_TRACE_ID(drvdata.traceid))
    coresight_trace_id_put_system_id(drvdata.traceid);
    pm_runtime_disable(dev);
    coresight_unregister(drvdata.csdev);
    }
    static const struct of_device_id dummy_match[] = {
    {.compatible = "arm,coresight-dummy-source"},
    {.compatible = "arm,coresight-dummy-sink"},
    {},
    };
    static struct platform_driver dummy_driver = {
    .probe	= dummy_probe,
    .remove = dummy_remove,
    .driver	= {
    .name   = "coresight-dummy",
    .of_match_table = dummy_match,
    },
    };
    module_platform_driver(dummy_driver);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("CoreSight dummy driver");
