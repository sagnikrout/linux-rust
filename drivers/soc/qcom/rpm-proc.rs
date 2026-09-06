//! Automatically rewritten from C to Rust
//! Source: drivers/soc/qcom/rpm-proc.c
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
// Copyright (c) 2021-2023, Stephan Gerhold <stephan@gerhold.net>

#[no_mangle]
unsafe extern "C" fn rpm_proc_probe(pdev: *mut platform_device) -> c_int {
    static int rpm_proc_probe(struct platform_device *pdev)
    {
    struct qcom_smd_edge *edge = core::ptr::null_mut();
    struct device *dev = &pdev.dev;
    struct device_node *edge_node;
    int ret;
    edge_node = of_get_child_by_name(dev.of_node, "smd-edge");
    if (edge_node) {
    edge = qcom_smd_register_edge(dev, edge_node);
    of_node_put(edge_node);
    if (IS_ERR(edge))
    return dev_err_probe(dev, PTR_ERR(edge),
    "Failed to register smd-edge\n");
    }
    ret = devm_of_platform_populate(dev);
    if (ret) {
    dev_err(dev, "Failed to populate child devices: %d\n", ret);
    goto err;
    }
    platform_set_drvdata(pdev, edge);
    return 0;
    err:
    if (edge)
    qcom_smd_unregister_edge(edge);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rpm_proc_remove(pdev: *mut platform_device) {
    static void rpm_proc_remove(struct platform_device *pdev)
    {
    struct qcom_smd_edge *edge = platform_get_drvdata(pdev);
    if (edge)
    qcom_smd_unregister_edge(edge);
    }
    static const struct of_device_id rpm_proc_of_match[] = {
    { .compatible = "qcom,rpm-proc", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, rpm_proc_of_match);
    static struct platform_driver rpm_proc_driver = {
    .probe = rpm_proc_probe,
    .remove = rpm_proc_remove,
    .driver = {
    .name = "qcom-rpm-proc",
    .of_match_table = rpm_proc_of_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn rpm_proc_init() -> int __init {
    static int __init rpm_proc_init(void)
    {
    return platform_driver_register(&rpm_proc_driver);
    }
    arch_initcall(rpm_proc_init);
#[no_mangle]
unsafe extern "C" fn rpm_proc_exit() -> void __exit {
    static void __exit rpm_proc_exit(void)
    {
    platform_driver_unregister(&rpm_proc_driver);
    }
    module_exit(rpm_proc_exit);
    MODULE_DESCRIPTION("Qualcomm RPM processor/subsystem driver");
    MODULE_AUTHOR("Stephan Gerhold <stephan@gerhold.net>");
    MODULE_LICENSE("GPL");
