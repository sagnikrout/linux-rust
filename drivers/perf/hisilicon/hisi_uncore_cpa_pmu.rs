//! Automatically rewritten from C to Rust
//! Source: drivers/perf/hisilicon/hisi_uncore_cpa_pmu.c
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
// HiSilicon SoC CPA(Coherency Protocol Agent) hardware event counters support
//
// Copyright (C) 2022 HiSilicon Limited
// Author: Qi Liu <liuqi115@huawei.com>
//
// This code is based on the uncore PMUs like arm-cci and arm-ccn.
//

// CPA register definition
pub const CPA_PERF_CTRL: c_uint = 0x1c00;
pub const CPA_EVENT_CTRL: c_uint = 0x1c04;
pub const CPA_INT_MASK: c_uint = 0x1c70;
pub const CPA_INT_STATUS: c_uint = 0x1c78;
pub const CPA_INT_CLEAR: c_uint = 0x1c7c;
pub const CPA_EVENT_TYPE0: c_uint = 0x1c80;
pub const CPA_VERSION: c_uint = 0x1cf0;
pub const CPA_CNT0_LOWER: c_uint = 0x1d00;
pub const CPA_CFG_REG: c_uint = 0x0534;
// CPA operation command

pub const CPA_EVTYPE_MASK: c_uint = 0xffUL;

// CPA has 8-counters
pub const CPA_NR_COUNTERS: c_uint = 0x8;
pub const CPA_COUNTER_BITS: c_int = 64;
pub const CPA_NR_EVENTS: c_uint = 0xff;
pub const CPA_REG_OFFSET: c_uint = 0x8;
#[no_mangle]
unsafe extern "C" fn hisi_cpa_pmu_get_counter_offset(idx: c_int) -> u32 {
    static u32 hisi_cpa_pmu_get_counter_offset(int idx)
    {
    return (CPA_CNT0_LOWER + idx * CPA_REG_OFFSET);
    }
    static u64 hisi_cpa_pmu_read_counter(struct hisi_pmu *cpa_pmu,
    struct hw_perf_event *hwc)
    {
    return readq(cpa_pmu.base + hisi_cpa_pmu_get_counter_offset(hwc.idx));
    }
    static void hisi_cpa_pmu_write_counter(struct hisi_pmu *cpa_pmu,
    struct hw_perf_event *hwc, u64 val)
    {
    writeq(val, cpa_pmu.base + hisi_cpa_pmu_get_counter_offset(hwc.idx));
    }
    static void hisi_cpa_pmu_write_evtype(struct hisi_pmu *cpa_pmu, int idx,
    u32 type)
    {
    u32 reg, reg_idx, shift, val;
//
// Select the appropriate event select register(CPA_EVENT_TYPE0/1).
// There are 2 event select registers for the 8 hardware counters.
// Event code is 8-bits and for the former 4 hardware counters,
// CPA_EVENT_TYPE0 is chosen. For the latter 4 hardware counters,
// CPA_EVENT_TYPE1 is chosen.
//
    reg = CPA_EVENT_TYPE0 + (idx / 4) * 4;
    reg_idx = idx % 4;
    shift = CPA_REG_OFFSET * reg_idx;
// Write event code to CPA_EVENT_TYPEx Register
    val = readl(cpa_pmu.base + reg);
    val &= ~(CPA_EVTYPE_MASK << shift);
    val |= type << shift;
    writel(val, cpa_pmu.base + reg);
    }
#[no_mangle]
unsafe extern "C" fn hisi_cpa_pmu_start_counters(cpa_pmu: *mut hisi_pmu) {
    static void hisi_cpa_pmu_start_counters(struct hisi_pmu *cpa_pmu)
    {
    u32 val;
    val = readl(cpa_pmu.base + CPA_PERF_CTRL);
    val |= CPA_PERF_CTRL_EN;
    writel(val, cpa_pmu.base + CPA_PERF_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn hisi_cpa_pmu_stop_counters(cpa_pmu: *mut hisi_pmu) {
    static void hisi_cpa_pmu_stop_counters(struct hisi_pmu *cpa_pmu)
    {
    u32 val;
    val = readl(cpa_pmu.base + CPA_PERF_CTRL);
    val &= ~(CPA_PERF_CTRL_EN);
    writel(val, cpa_pmu.base + CPA_PERF_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn hisi_cpa_pmu_disable_pm(cpa_pmu: *mut hisi_pmu) {
    static void hisi_cpa_pmu_disable_pm(struct hisi_pmu *cpa_pmu)
    {
    u32 val;
    val = readl(cpa_pmu.base + CPA_CFG_REG);
    val |= CPA_PM_CTRL;
    writel(val, cpa_pmu.base + CPA_CFG_REG);
    }
#[no_mangle]
unsafe extern "C" fn hisi_cpa_pmu_enable_pm(cpa_pmu: *mut hisi_pmu) {
    static void hisi_cpa_pmu_enable_pm(struct hisi_pmu *cpa_pmu)
    {
    u32 val;
    val = readl(cpa_pmu.base + CPA_CFG_REG);
    val &= ~(CPA_PM_CTRL);
    writel(val, cpa_pmu.base + CPA_CFG_REG);
    }
    static void hisi_cpa_pmu_enable_counter(struct hisi_pmu *cpa_pmu,
    struct hw_perf_event *hwc)
    {
    u32 val;
// Enable counter index in CPA_EVENT_CTRL register
    val = readl(cpa_pmu.base + CPA_EVENT_CTRL);
    val |= 1 << hwc.idx;
    writel(val, cpa_pmu.base + CPA_EVENT_CTRL);
    }
    static void hisi_cpa_pmu_disable_counter(struct hisi_pmu *cpa_pmu,
    struct hw_perf_event *hwc)
    {
    u32 val;
// Clear counter index in CPA_EVENT_CTRL register
    val = readl(cpa_pmu.base + CPA_EVENT_CTRL);
    val &= ~(1UL << hwc.idx);
    writel(val, cpa_pmu.base + CPA_EVENT_CTRL);
    }
    static void hisi_cpa_pmu_enable_counter_int(struct hisi_pmu *cpa_pmu,
    struct hw_perf_event *hwc)
    {
    u32 val;
// Write 0 to enable interrupt
    val = readl(cpa_pmu.base + CPA_INT_MASK);
    val &= ~(1UL << hwc.idx);
    writel(val, cpa_pmu.base + CPA_INT_MASK);
    }
    static void hisi_cpa_pmu_disable_counter_int(struct hisi_pmu *cpa_pmu,
    struct hw_perf_event *hwc)
    {
    u32 val;
// Write 1 to mask interrupt
    val = readl(cpa_pmu.base + CPA_INT_MASK);
    val |= 1 << hwc.idx;
    writel(val, cpa_pmu.base + CPA_INT_MASK);
    }
#[no_mangle]
unsafe extern "C" fn hisi_cpa_pmu_get_int_status(cpa_pmu: *mut hisi_pmu) -> u32 {
    static u32 hisi_cpa_pmu_get_int_status(struct hisi_pmu *cpa_pmu)
    {
    return readl(cpa_pmu.base + CPA_INT_STATUS);
    }
#[no_mangle]
unsafe extern "C" fn hisi_cpa_pmu_clear_int_status(cpa_pmu: *mut hisi_pmu, idx: c_int) {
    static void hisi_cpa_pmu_clear_int_status(struct hisi_pmu *cpa_pmu, int idx)
    {
    writel(1 << idx, cpa_pmu.base + CPA_INT_CLEAR);
    }
    static const struct acpi_device_id hisi_cpa_pmu_acpi_match[] = {
    { "HISI0281", },
    {}
    };
    MODULE_DEVICE_TABLE(acpi, hisi_cpa_pmu_acpi_match);
    static int hisi_cpa_pmu_init_data(struct platform_device *pdev,
    struct hisi_pmu *cpa_pmu)
    {
    hisi_uncore_pmu_init_topology(cpa_pmu, &pdev.dev);
    if (cpa_pmu.topo.sicl_id < 0) {
    dev_err(&pdev.dev, "Can not read sicl-id\n");
    return -EINVAL;
    }
    if (cpa_pmu.topo.index_id < 0) {
    dev_err(&pdev.dev, "Cannot read idx-id\n");
    return -EINVAL;
    }
    cpa_pmu.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(cpa_pmu.base))
    return PTR_ERR(cpa_pmu.base);
    cpa_pmu.identifier = readl(cpa_pmu.base + CPA_VERSION);
    return 0;
    }
    static struct attribute *hisi_cpa_pmu_format_attr[] = {
    HISI_PMU_FORMAT_ATTR(event, "config:0-15"),
    core::ptr::null_mut()
    };
    static const struct attribute_group hisi_cpa_pmu_format_group = {
    .name = "format",
    .attrs = hisi_cpa_pmu_format_attr,
    };
    static struct attribute *hisi_cpa_pmu_events_attr[] = {
    HISI_PMU_EVENT_ATTR(cpa_cycles,		0x00),
    HISI_PMU_EVENT_ATTR(cpa_p1_wr_dat,	0x61),
    HISI_PMU_EVENT_ATTR(cpa_p1_rd_dat,	0x62),
    HISI_PMU_EVENT_ATTR(cpa_p0_wr_dat,	0xE1),
    HISI_PMU_EVENT_ATTR(cpa_p0_rd_dat,	0xE2),
    core::ptr::null_mut()
    };
    static const struct attribute_group hisi_cpa_pmu_events_group = {
    .name = "events",
    .attrs = hisi_cpa_pmu_events_attr,
    };
    static const struct attribute_group *hisi_cpa_pmu_attr_groups[] = {
    &hisi_cpa_pmu_format_group,
    &hisi_cpa_pmu_events_group,
    &hisi_pmu_cpumask_attr_group,
    &hisi_pmu_identifier_group,
    core::ptr::null_mut()
    };
    static const struct hisi_uncore_ops hisi_uncore_cpa_pmu_ops = {
    .write_evtype           = hisi_cpa_pmu_write_evtype,
    .get_event_idx		= hisi_uncore_pmu_get_event_idx,
    .start_counters		= hisi_cpa_pmu_start_counters,
    .stop_counters		= hisi_cpa_pmu_stop_counters,
    .enable_counter		= hisi_cpa_pmu_enable_counter,
    .disable_counter	= hisi_cpa_pmu_disable_counter,
    .enable_counter_int	= hisi_cpa_pmu_enable_counter_int,
    .disable_counter_int	= hisi_cpa_pmu_disable_counter_int,
    .write_counter		= hisi_cpa_pmu_write_counter,
    .read_counter		= hisi_cpa_pmu_read_counter,
    .get_int_status		= hisi_cpa_pmu_get_int_status,
    .clear_int_status	= hisi_cpa_pmu_clear_int_status,
    };
    static int hisi_cpa_pmu_dev_probe(struct platform_device *pdev,
    struct hisi_pmu *cpa_pmu)
    {
    int ret;
    ret = hisi_cpa_pmu_init_data(pdev, cpa_pmu);
    if (ret)
    return ret;
    ret = hisi_uncore_pmu_init_irq(cpa_pmu, pdev);
    if (ret)
    return ret;
    cpa_pmu.counter_bits = CPA_COUNTER_BITS;
    cpa_pmu.check_event = CPA_NR_EVENTS;
    cpa_pmu.pmu_events.attr_groups = hisi_cpa_pmu_attr_groups;
    cpa_pmu.ops = &hisi_uncore_cpa_pmu_ops;
    cpa_pmu.num_counters = CPA_NR_COUNTERS;
    cpa_pmu.dev = &pdev.dev;
    cpa_pmu.on_cpu = -1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hisi_cpa_pmu_probe(pdev: *mut platform_device) -> c_int {
    static int hisi_cpa_pmu_probe(struct platform_device *pdev)
    {
    struct hisi_pmu *cpa_pmu;
    char *name;
    int ret;
    cpa_pmu = devm_kzalloc(&pdev.dev, sizeof(*cpa_pmu), GFP_KERNEL);
    if (!cpa_pmu)
    return -ENOMEM;
    ret = hisi_cpa_pmu_dev_probe(pdev, cpa_pmu);
    if (ret)
    return ret;
    name = devm_kasprintf(&pdev.dev, GFP_KERNEL, "hisi_sicl%d_cpa%d",
    cpa_pmu.topo.sicl_id, cpa_pmu.topo.index_id);
    if (!name)
    return -ENOMEM;
    hisi_pmu_init(cpa_pmu, THIS_MODULE);
// Power Management should be disabled before using CPA PMU.
    hisi_cpa_pmu_disable_pm(cpa_pmu);
    ret = cpuhp_state_add_instance(CPUHP_AP_PERF_ARM_HISI_CPA_ONLINE,
    &cpa_pmu.node);
    if (ret) {
    dev_err(&pdev.dev, "Error %d registering hotplug\n", ret);
    hisi_cpa_pmu_enable_pm(cpa_pmu);
    return ret;
    }
    ret = perf_pmu_register(&cpa_pmu.pmu, name, -1);
    if (ret) {
    dev_err(cpa_pmu.dev, "PMU register failed\n");
    cpuhp_state_remove_instance_nocalls(
    CPUHP_AP_PERF_ARM_HISI_CPA_ONLINE, &cpa_pmu.node);
    hisi_cpa_pmu_enable_pm(cpa_pmu);
    return ret;
    }
    platform_set_drvdata(pdev, cpa_pmu);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hisi_cpa_pmu_remove(pdev: *mut platform_device) {
    static void hisi_cpa_pmu_remove(struct platform_device *pdev)
    {
    struct hisi_pmu *cpa_pmu = platform_get_drvdata(pdev);
    perf_pmu_unregister(&cpa_pmu.pmu);
    cpuhp_state_remove_instance_nocalls(CPUHP_AP_PERF_ARM_HISI_CPA_ONLINE,
    &cpa_pmu.node);
    hisi_cpa_pmu_enable_pm(cpa_pmu);
    }
    static struct platform_driver hisi_cpa_pmu_driver = {
    .driver = {
    .name = "hisi_cpa_pmu",
    .acpi_match_table = ACPI_PTR(hisi_cpa_pmu_acpi_match),
    .suppress_bind_attrs = true,
    },
    .probe = hisi_cpa_pmu_probe,
    .remove = hisi_cpa_pmu_remove,
    };
#[no_mangle]
unsafe extern "C" fn hisi_cpa_pmu_module_init() -> int __init {
    static int __init hisi_cpa_pmu_module_init(void)
    {
    int ret;
    ret = cpuhp_setup_state_multi(CPUHP_AP_PERF_ARM_HISI_CPA_ONLINE,
    "AP_PERF_ARM_HISI_CPA_ONLINE",
    hisi_uncore_pmu_online_cpu,
    hisi_uncore_pmu_offline_cpu);
    if (ret) {
    pr_err("setup hotplug failed: %d\n", ret);
    return ret;
    }
    ret = platform_driver_register(&hisi_cpa_pmu_driver);
    if (ret)
    cpuhp_remove_multi_state(CPUHP_AP_PERF_ARM_HISI_CPA_ONLINE);
    return ret;
    }
    module_init(hisi_cpa_pmu_module_init);
#[no_mangle]
unsafe extern "C" fn hisi_cpa_pmu_module_exit() -> void __exit {
    static void __exit hisi_cpa_pmu_module_exit(void)
    {
    platform_driver_unregister(&hisi_cpa_pmu_driver);
    cpuhp_remove_multi_state(CPUHP_AP_PERF_ARM_HISI_CPA_ONLINE);
    }
    module_exit(hisi_cpa_pmu_module_exit);
    MODULE_IMPORT_NS("HISI_PMU");
    MODULE_DESCRIPTION("HiSilicon SoC CPA PMU driver");
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Qi Liu <liuqi115@huawei.com>");
