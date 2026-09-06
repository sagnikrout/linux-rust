//! Automatically rewritten from C to Rust
//! Source: drivers/base/test/test_async_driver_probe.c
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
// Copyright (C) 2014 Google, Inc.
//

    static atomic_t warnings, errors, timeout, async_completed;
#[no_mangle]
unsafe extern "C" fn test_probe(pdev: *mut platform_device) -> c_int {
    static int test_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
//
// Determine if we have hit the "timeout" limit for the test if we
// have then report it as an error, otherwise we wil sleep for the
// required amount of time and then report completion.
//
    if (atomic_read(&timeout)) {
    dev_err(dev, "async probe took too long\n");
    atomic_inc(&errors);
    } else {
    dev_dbg(&pdev.dev, "sleeping for %d msecs in probe\n",
    TEST_PROBE_DELAY);
    msleep(TEST_PROBE_DELAY);
    dev_dbg(&pdev.dev, "done sleeping\n");
    }
//
// Report NUMA mismatch if device node is set and we are not
// performing an async init on that node.
//
    if (dev.driver.probe_type == PROBE_PREFER_ASYNCHRONOUS) {
    if (IS_ENABLED(CONFIG_NUMA) &&
    dev_to_node(dev) != numa_node_id()) {
    dev_warn(dev, "NUMA node mismatch %d != %d\n",
    dev_to_node(dev), numa_node_id());
    atomic_inc(&warnings);
    }
    atomic_inc(&async_completed);
    }
    return 0;
    }
    static struct platform_driver async_driver = {
    .driver = {
    .name = "test_async_driver",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    .probe = test_probe,
    };
    static struct platform_driver sync_driver = {
    .driver = {
    .name = "test_sync_driver",
    .probe_type = PROBE_FORCE_SYNCHRONOUS,
    },
    .probe = test_probe,
    };
    static struct platform_device *async_dev[NR_CPUS * 2];
    static struct platform_device *sync_dev[2];
    static struct platform_device *
    test_platform_device_register_node(char *name, int id, int nid)
    {
    struct platform_device *pdev;
    int ret;
    pdev = platform_device_alloc(name, id);
    if (!pdev)
    return ERR_PTR(-ENOMEM);
    if (nid != NUMA_NO_NODE)
    set_dev_node(&pdev.dev, nid);
    ret = platform_device_add(pdev);
    if (ret) {
    platform_device_put(pdev);
    return ERR_PTR(ret);
    }
    return pdev;
    }
#[no_mangle]
unsafe extern "C" fn test_async_probe_init() -> int __init {
    static int __init test_async_probe_init(void)
    {
    struct platform_device **pdev = core::ptr::null_mut();
    let mut async_id: c_int = 0, sync_id = 0;
    unsigned long long duration;
    ktime_t calltime;
    int err, nid, cpu;
    pr_info("registering first set of asynchronous devices...\n");
    for_each_online_cpu(cpu) {
    nid = cpu_to_node(cpu);
    pdev = &async_dev[async_id];
// pdev =	test_platform_device_register_node("test_async_driver",
    async_id,
    nid);
    if (IS_ERR(*pdev)) {
    err = PTR_ERR(*pdev);
// pdev = NULL;
    pr_err("failed to create async_dev: %d\n", err);
    goto err_unregister_async_devs;
    }
    async_id++;
    }
    pr_info("registering asynchronous driver...\n");
    calltime = ktime_get();
    err = platform_driver_register(&async_driver);
    if (err) {
    pr_err("Failed to register async_driver: %d\n", err);
    goto err_unregister_async_devs;
    }
    duration = (unsigned long long)ktime_ms_delta(ktime_get(), calltime);
    pr_info("registration took %lld msecs\n", duration);
    if (duration > TEST_PROBE_THRESHOLD) {
    pr_err("test failed: probe took too long\n");
    err = -ETIMEDOUT;
    goto err_unregister_async_driver;
    }
    pr_info("registering second set of asynchronous devices...\n");
    calltime = ktime_get();
    for_each_online_cpu(cpu) {
    nid = cpu_to_node(cpu);
    pdev = &async_dev[async_id];
// pdev = test_platform_device_register_node("test_async_driver",
    async_id,
    nid);
    if (IS_ERR(*pdev)) {
    err = PTR_ERR(*pdev);
// pdev = NULL;
    pr_err("failed to create async_dev: %d\n", err);
    goto err_unregister_async_driver;
    }
    async_id++;
    }
    duration = (unsigned long long)ktime_ms_delta(ktime_get(), calltime);
    dev_info(&(*pdev).dev,
    "registration took %lld msecs\n", duration);
    if (duration > TEST_PROBE_THRESHOLD) {
    dev_err(&(*pdev).dev,
    "test failed: probe took too long\n");
    err = -ETIMEDOUT;
    goto err_unregister_async_driver;
    }
    pr_info("registering first synchronous device...\n");
    nid = cpu_to_node(cpu);
    pdev = &sync_dev[sync_id];
// pdev = test_platform_device_register_node("test_sync_driver",
    sync_id,
    NUMA_NO_NODE);
    if (IS_ERR(*pdev)) {
    err = PTR_ERR(*pdev);
// pdev = NULL;
    pr_err("failed to create sync_dev: %d\n", err);
    goto err_unregister_async_driver;
    }
    sync_id++;
    pr_info("registering synchronous driver...\n");
    calltime = ktime_get();
    err = platform_driver_register(&sync_driver);
    if (err) {
    pr_err("Failed to register async_driver: %d\n", err);
    goto err_unregister_sync_devs;
    }
    duration = (unsigned long long)ktime_ms_delta(ktime_get(), calltime);
    pr_info("registration took %lld msecs\n", duration);
    if (duration < TEST_PROBE_THRESHOLD) {
    dev_err(&(*pdev).dev,
    "test failed: probe was too quick\n");
    err = -ETIMEDOUT;
    goto err_unregister_sync_driver;
    }
    pr_info("registering second synchronous device...\n");
    pdev = &sync_dev[sync_id];
    calltime = ktime_get();
// pdev = test_platform_device_register_node("test_sync_driver",
    sync_id,
    NUMA_NO_NODE);
    if (IS_ERR(*pdev)) {
    err = PTR_ERR(*pdev);
// pdev = NULL;
    pr_err("failed to create sync_dev: %d\n", err);
    goto err_unregister_sync_driver;
    }
    sync_id++;
    duration = (unsigned long long)ktime_ms_delta(ktime_get(), calltime);
    dev_info(&(*pdev).dev,
    "registration took %lld msecs\n", duration);
    if (duration < TEST_PROBE_THRESHOLD) {
    dev_err(&(*pdev).dev,
    "test failed: probe was too quick\n");
    err = -ETIMEDOUT;
    goto err_unregister_sync_driver;
    }
//
// The async events should have completed while we were taking care
// of the synchronous events. We will now terminate any outstanding
// asynchronous probe calls remaining by forcing timeout and remove
// the driver before we return which should force the flush of the
// pending asynchronous probe calls.
//
// Otherwise if they completed without errors or warnings then
// report successful completion.
//
    if (atomic_read(&async_completed) != async_id) {
    pr_err("async events still pending, forcing timeout\n");
    atomic_inc(&timeout);
    err = -ETIMEDOUT;
    } else if (!atomic_read(&errors) && !atomic_read(&warnings)) {
    pr_info("completed successfully\n");
    return 0;
    }
    err_unregister_sync_driver:
    platform_driver_unregister(&sync_driver);
    err_unregister_sync_devs:
    while (sync_id--)
    platform_device_unregister(sync_dev[sync_id]);
    err_unregister_async_driver:
    platform_driver_unregister(&async_driver);
    err_unregister_async_devs:
    while (async_id--)
    platform_device_unregister(async_dev[async_id]);
//
// If err is already set then count that as an additional error for
// the test. Otherwise we will report an invalid argument error and
// not count that as we should have reached here as a result of
// errors or warnings being reported by the probe routine.
//
    if (err)
    atomic_inc(&errors);
    else
    err = -EINVAL;
    pr_err("Test failed with %d errors and %d warnings\n",
    atomic_read(&errors), atomic_read(&warnings));
    return err;
    }
    module_init(test_async_probe_init);
#[no_mangle]
unsafe extern "C" fn test_async_probe_exit() -> void __exit {
    static void __exit test_async_probe_exit(void)
    {
    let mut id: c_int = 2;
    platform_driver_unregister(&async_driver);
    platform_driver_unregister(&sync_driver);
    while (id--)
    platform_device_unregister(sync_dev[id]);
    id = NR_CPUS * 2;
    while (id--)
    platform_device_unregister(async_dev[id]);
    }
    module_exit(test_async_probe_exit);
    MODULE_DESCRIPTION("Test module for asynchronous driver probing");
    MODULE_AUTHOR("Dmitry Torokhov <dtor@chromium.org>");
    MODULE_LICENSE("GPL");
