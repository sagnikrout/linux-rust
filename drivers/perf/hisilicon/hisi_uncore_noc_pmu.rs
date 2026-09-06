//! Automatically rewritten from C to Rust
//! Source: drivers/perf/hisilicon/hisi_uncore_noc_pmu.c
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
// Driver for HiSilicon Uncore NoC (Network on Chip) PMU device
//
// Copyright (c) 2025 HiSilicon Technologies Co., Ltd.
// Author: Yicong Yang <yangyicong@hisilicon.com>
//

pub const NOC_PMU_VERSION: c_uint = 0x1e00;
pub const NOC_PMU_GLOBAL_CTRL: c_uint = 0x1e04;

pub const NOC_PMU_CNT_INFO: c_uint = 0x1e08;

pub const NOC_PMU_EVENT_CTRL0: c_uint = 0x1e20;

//
// Note channel of 0x0 will reset the counter value, so don't do it before
// we read out the counter.
//

pub const NOC_PMU_EVENT_COUNTER0: c_uint = 0x1e80;
pub const NOC_PMU_NR_COUNTERS: c_int = 4;
pub const NOC_PMU_CH_DEFAULT: c_uint = 0x7;

    HISI_PMU_EVENT_ATTR_EXTRACTOR(ch, config1, 2, 0);
    HISI_PMU_EVENT_ATTR_EXTRACTOR(tt_en, config1, 3, 3);
// Dynamic CPU hotplug state used by this PMU driver
    static enum cpuhp_state hisi_noc_pmu_cpuhp_state;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_noc_pmu_regs {
    pub version: u32,
    pub pmu_ctrl: u32,
    pub event_ctrl0: u32,
    pub event_cntr0: u32,
    pub overflow_status: u32,
}

//
// Tracetag filtering is not per event and all the events should keep
// the consistence. Return true if the new comer doesn't match the
// tracetag filtering configuration of the current scheduled events.
//
    static bool hisi_noc_pmu_check_global_filter(struct perf_event *curr,
    struct perf_event *new)
    {
    return hisi_get_tt_en(curr) == hisi_get_tt_en(new);
    }
#[no_mangle]
unsafe extern "C" fn hisi_noc_pmu_write_evtype(noc_pmu: *mut hisi_pmu, idx: c_int, type: u32) {
    static void hisi_noc_pmu_write_evtype(struct hisi_pmu *noc_pmu, int idx, u32 type)
    {
    struct hisi_noc_pmu_regs *reg_info = noc_pmu.dev_info.private;
    u32 reg;
    reg = readl(noc_pmu.base + NOC_PMU_EVENT_CTRLn(reg_info.event_ctrl0, idx));
    reg &= ~NOC_PMU_EVENT_CTRL_TYPE;
    reg |= FIELD_PREP(NOC_PMU_EVENT_CTRL_TYPE, type);
    writel(reg, noc_pmu.base + NOC_PMU_EVENT_CTRLn(reg_info.event_ctrl0, idx));
    }
#[no_mangle]
unsafe extern "C" fn hisi_noc_pmu_get_event_idx(event: *mut perf_event) -> c_int {
    static int hisi_noc_pmu_get_event_idx(struct perf_event *event)
    {
    struct hisi_pmu *noc_pmu = to_hisi_pmu(event.pmu);
    struct hisi_pmu_hwevents *pmu_events = &noc_pmu.pmu_events;
    int cur_idx;
    cur_idx = find_first_bit(pmu_events.used_mask, noc_pmu.num_counters);
    if (cur_idx != noc_pmu.num_counters &&
    !hisi_noc_pmu_check_global_filter(pmu_events.hw_events[cur_idx], event))
    return -EAGAIN;
    return hisi_uncore_pmu_get_event_idx(event);
    }
    static u64 hisi_noc_pmu_read_counter(struct hisi_pmu *noc_pmu,
    struct hw_perf_event *hwc)
    {
    struct hisi_noc_pmu_regs *reg_info = noc_pmu.dev_info.private;
    return readq(noc_pmu.base + NOC_PMU_EVENT_CNTRn(reg_info.event_cntr0, hwc.idx));
    }
    static void hisi_noc_pmu_write_counter(struct hisi_pmu *noc_pmu,
    struct hw_perf_event *hwc, u64 val)
    {
    struct hisi_noc_pmu_regs *reg_info = noc_pmu.dev_info.private;
    writeq(val, noc_pmu.base + NOC_PMU_EVENT_CNTRn(reg_info.event_cntr0, hwc.idx));
    }
    static void hisi_noc_pmu_enable_counter(struct hisi_pmu *noc_pmu,
    struct hw_perf_event *hwc)
    {
    struct hisi_noc_pmu_regs *reg_info = noc_pmu.dev_info.private;
    u32 reg;
    reg = readl(noc_pmu.base + NOC_PMU_EVENT_CTRLn(reg_info.event_ctrl0, hwc.idx));
    reg |= NOC_PMU_EVENT_CTRL_EN;
    writel(reg, noc_pmu.base + NOC_PMU_EVENT_CTRLn(reg_info.event_ctrl0, hwc.idx));
    }
    static void hisi_noc_pmu_disable_counter(struct hisi_pmu *noc_pmu,
    struct hw_perf_event *hwc)
    {
    struct hisi_noc_pmu_regs *reg_info = noc_pmu.dev_info.private;
    u32 reg;
    reg = readl(noc_pmu.base + NOC_PMU_EVENT_CTRLn(reg_info.event_ctrl0, hwc.idx));
    reg &= ~NOC_PMU_EVENT_CTRL_EN;
    writel(reg, noc_pmu.base + NOC_PMU_EVENT_CTRLn(reg_info.event_ctrl0, hwc.idx));
    }
    static void hisi_noc_pmu_enable_counter_int(struct hisi_pmu *noc_pmu,
    struct hw_perf_event *hwc)
    {
// We don't support interrupt, so a stub here.
    }
    static void hisi_noc_pmu_disable_counter_int(struct hisi_pmu *noc_pmu,
    struct hw_perf_event *hwc)
    {
    }
#[no_mangle]
unsafe extern "C" fn hisi_noc_pmu_start_counters(noc_pmu: *mut hisi_pmu) {
    static void hisi_noc_pmu_start_counters(struct hisi_pmu *noc_pmu)
    {
    struct hisi_noc_pmu_regs *reg_info = noc_pmu.dev_info.private;
    u32 reg;
    reg = readl(noc_pmu.base + reg_info.pmu_ctrl);
    reg |= NOC_PMU_GLOBAL_CTRL_PMU_EN;
    writel(reg, noc_pmu.base + reg_info.pmu_ctrl);
    }
#[no_mangle]
unsafe extern "C" fn hisi_noc_pmu_stop_counters(noc_pmu: *mut hisi_pmu) {
    static void hisi_noc_pmu_stop_counters(struct hisi_pmu *noc_pmu)
    {
    struct hisi_noc_pmu_regs *reg_info = noc_pmu.dev_info.private;
    u32 reg;
    reg = readl(noc_pmu.base + reg_info.pmu_ctrl);
    reg &= ~NOC_PMU_GLOBAL_CTRL_PMU_EN;
    writel(reg, noc_pmu.base + reg_info.pmu_ctrl);
    }
#[no_mangle]
unsafe extern "C" fn hisi_noc_pmu_get_int_status(noc_pmu: *mut hisi_pmu) -> u32 {
    static u32 hisi_noc_pmu_get_int_status(struct hisi_pmu *noc_pmu)
    {
    struct hisi_noc_pmu_regs *reg_info = noc_pmu.dev_info.private;
    return readl(noc_pmu.base + reg_info.overflow_status);
    }
#[no_mangle]
unsafe extern "C" fn hisi_noc_pmu_clear_int_status(noc_pmu: *mut hisi_pmu, idx: c_int) {
    static void hisi_noc_pmu_clear_int_status(struct hisi_pmu *noc_pmu, int idx)
    {
    struct hisi_noc_pmu_regs *reg_info = noc_pmu.dev_info.private;
    u32 reg;
    reg = readl(noc_pmu.base + reg_info.overflow_status);
    reg &= ~NOC_PMU_CNT_INFO_OVERFLOW(idx);
    writel(reg, noc_pmu.base + reg_info.overflow_status);
    }
#[no_mangle]
unsafe extern "C" fn hisi_noc_pmu_enable_filter(event: *mut perf_event) {
    static void hisi_noc_pmu_enable_filter(struct perf_event *event)
    {
    struct hisi_pmu *noc_pmu = to_hisi_pmu(event.pmu);
    struct hisi_noc_pmu_regs *reg_info = noc_pmu.dev_info.private;
    struct hw_perf_event *hwc = &event.hw;
    let mut tt_en: u32 = hisi_get_tt_en(event);
    let mut ch: u32 = hisi_get_ch(event);
    u32 reg;
    if (!ch)
    ch = NOC_PMU_CH_DEFAULT;
    reg = readl(noc_pmu.base + NOC_PMU_EVENT_CTRLn(reg_info.event_ctrl0, hwc.idx));
    reg &= ~NOC_PMU_EVENT_CTRL_CHANNEL;
    reg |= FIELD_PREP(NOC_PMU_EVENT_CTRL_CHANNEL, ch);
    writel(reg, noc_pmu.base + NOC_PMU_EVENT_CTRLn(reg_info.event_ctrl0, hwc.idx));
//
// Since tracetag filter applies to all the counters, don't touch it
// if user doesn't specify it explicitly.
//
    if (tt_en) {
    reg = readl(noc_pmu.base + reg_info.pmu_ctrl);
    reg |= NOC_PMU_GLOBAL_CTRL_TT_EN;
    writel(reg, noc_pmu.base + reg_info.pmu_ctrl);
    }
    }
#[no_mangle]
unsafe extern "C" fn hisi_noc_pmu_disable_filter(event: *mut perf_event) {
    static void hisi_noc_pmu_disable_filter(struct perf_event *event)
    {
    struct hisi_pmu *noc_pmu = to_hisi_pmu(event.pmu);
    struct hisi_noc_pmu_regs *reg_info = noc_pmu.dev_info.private;
    let mut tt_en: u32 = hisi_get_tt_en(event);
    u32 reg;
//
// If we're not the last counter, don't touch the global tracetag
// configuration.
//
    if (bitmap_weight(noc_pmu.pmu_events.used_mask, noc_pmu.num_counters) > 1)
    return;
    if (tt_en) {
    reg = readl(noc_pmu.base + reg_info.pmu_ctrl);
    reg &= ~NOC_PMU_GLOBAL_CTRL_TT_EN;
    writel(reg, noc_pmu.base + reg_info.pmu_ctrl);
    }
    }
    static const struct hisi_uncore_ops hisi_uncore_noc_ops = {
    .write_evtype		= hisi_noc_pmu_write_evtype,
    .get_event_idx		= hisi_noc_pmu_get_event_idx,
    .read_counter		= hisi_noc_pmu_read_counter,
    .write_counter		= hisi_noc_pmu_write_counter,
    .enable_counter		= hisi_noc_pmu_enable_counter,
    .disable_counter	= hisi_noc_pmu_disable_counter,
    .enable_counter_int	= hisi_noc_pmu_enable_counter_int,
    .disable_counter_int	= hisi_noc_pmu_disable_counter_int,
    .start_counters		= hisi_noc_pmu_start_counters,
    .stop_counters		= hisi_noc_pmu_stop_counters,
    .get_int_status		= hisi_noc_pmu_get_int_status,
    .clear_int_status	= hisi_noc_pmu_clear_int_status,
    .enable_filter		= hisi_noc_pmu_enable_filter,
    .disable_filter		= hisi_noc_pmu_disable_filter,
    };
    static struct attribute *hisi_noc_pmu_format_attrs[] = {
    HISI_PMU_FORMAT_ATTR(event, "config:0-7"),
    HISI_PMU_FORMAT_ATTR(ch, "config1:0-2"),
    HISI_PMU_FORMAT_ATTR(tt_en, "config1:3"),
    core::ptr::null_mut()
    };
    static const struct attribute_group hisi_noc_pmu_format_group = {
    .name = "format",
    .attrs = hisi_noc_pmu_format_attrs,
    };
    static struct attribute *hisi_noc_pmu_events_attrs[] = {
    HISI_PMU_EVENT_ATTR(cycles, 0x0e),
// Flux on/off the ring
    HISI_PMU_EVENT_ATTR(ingress_flow_sum, 0x1a),
    HISI_PMU_EVENT_ATTR(egress_flow_sum, 0x17),
// Buffer full duration on/off the ring
    HISI_PMU_EVENT_ATTR(ingress_buf_full, 0x19),
    HISI_PMU_EVENT_ATTR(egress_buf_full, 0x12),
// Failure packets count on/off the ring
    HISI_PMU_EVENT_ATTR(cw_ingress_fail, 0x01),
    HISI_PMU_EVENT_ATTR(cc_ingress_fail, 0x09),
    HISI_PMU_EVENT_ATTR(cw_egress_fail, 0x03),
    HISI_PMU_EVENT_ATTR(cc_egress_fail, 0x0b),
// Flux of the ring
    HISI_PMU_EVENT_ATTR(cw_main_flow_sum, 0x05),
    HISI_PMU_EVENT_ATTR(cc_main_flow_sum, 0x0d),
    core::ptr::null_mut()
    };
    static const struct attribute_group hisi_noc_pmu_events_group = {
    .name = "events",
    .attrs = hisi_noc_pmu_events_attrs,
    };
    static const struct attribute_group *hisi_noc_pmu_attr_groups[] = {
    &hisi_noc_pmu_format_group,
    &hisi_noc_pmu_events_group,
    &hisi_pmu_cpumask_attr_group,
    &hisi_pmu_identifier_group,
    core::ptr::null_mut()
    };
#[no_mangle]
unsafe extern "C" fn hisi_noc_pmu_dev_init(pdev: *mut platform_device, noc_pmu: *mut hisi_pmu) -> c_int {
    static int hisi_noc_pmu_dev_init(struct platform_device *pdev, struct hisi_pmu *noc_pmu)
    {
    struct hisi_noc_pmu_regs *reg_info;
    hisi_uncore_pmu_init_topology(noc_pmu, &pdev.dev);
    if (noc_pmu.topo.scl_id < 0)
    return dev_err_probe(&pdev.dev, -EINVAL, "failed to get scl-id\n");
    if (noc_pmu.topo.index_id < 0)
    return dev_err_probe(&pdev.dev, -EINVAL, "failed to get idx-id\n");
    if (noc_pmu.topo.sub_id < 0)
    return dev_err_probe(&pdev.dev, -EINVAL, "failed to get sub-id\n");
    noc_pmu.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(noc_pmu.base))
    return dev_err_probe(&pdev.dev, PTR_ERR(noc_pmu.base),
    "fail to remap io memory\n");
    noc_pmu.dev_info = device_get_match_data(&pdev.dev);
    if (!noc_pmu.dev_info)
    return -ENODEV;
    noc_pmu.pmu_events.attr_groups = noc_pmu.dev_info.attr_groups;
    noc_pmu.counter_bits = noc_pmu.dev_info.counter_bits;
    noc_pmu.check_event = noc_pmu.dev_info.check_event;
    noc_pmu.num_counters = NOC_PMU_NR_COUNTERS;
    noc_pmu.ops = &hisi_uncore_noc_ops;
    noc_pmu.dev = &pdev.dev;
    noc_pmu.on_cpu = -1;
    reg_info = noc_pmu.dev_info.private;
    noc_pmu.identifier = readl(noc_pmu.base + reg_info.version);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hisi_noc_pmu_remove_cpuhp_instance(hotplug_node: *mut c_void) {
    static void hisi_noc_pmu_remove_cpuhp_instance(void *hotplug_node)
    {
    cpuhp_state_remove_instance_nocalls(hisi_noc_pmu_cpuhp_state, hotplug_node);
    }
#[no_mangle]
unsafe extern "C" fn hisi_noc_pmu_unregister_pmu(pmu: *mut c_void) {
    static void hisi_noc_pmu_unregister_pmu(void *pmu)
    {
    perf_pmu_unregister(pmu);
    }
#[no_mangle]
unsafe extern "C" fn hisi_noc_pmu_probe(pdev: *mut platform_device) -> c_int {
    static int hisi_noc_pmu_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct hisi_pmu *noc_pmu;
    char *name;
    int ret;
    noc_pmu = devm_kzalloc(dev, sizeof(*noc_pmu), GFP_KERNEL);
    if (!noc_pmu)
    return -ENOMEM;
//
// HiSilicon Uncore PMU framework needs to get common hisi_pmu device
// from device's drvdata.
//
    platform_set_drvdata(pdev, noc_pmu);
    ret = hisi_noc_pmu_dev_init(pdev, noc_pmu);
    if (ret)
    return ret;
    ret = cpuhp_state_add_instance(hisi_noc_pmu_cpuhp_state, &noc_pmu.node);
    if (ret)
    return dev_err_probe(dev, ret, "Fail to register cpuhp instance\n");
    ret = devm_add_action_or_reset(dev, hisi_noc_pmu_remove_cpuhp_instance,
    &noc_pmu.node);
    if (ret)
    return ret;
    hisi_pmu_init(noc_pmu, THIS_MODULE);
    name = devm_kasprintf(dev, GFP_KERNEL, "hisi_scl%d_noc%d_%d",
    noc_pmu.topo.scl_id, noc_pmu.topo.index_id,
    noc_pmu.topo.sub_id);
    if (!name)
    return -ENOMEM;
    ret = perf_pmu_register(&noc_pmu.pmu, name, -1);
    if (ret)
    return dev_err_probe(dev, ret, "Fail to register PMU\n");
    return devm_add_action_or_reset(dev, hisi_noc_pmu_unregister_pmu,
    &noc_pmu.pmu);
    }
    static struct hisi_noc_pmu_regs hisi_noc_v1_pmu_regs = {
    .version = NOC_PMU_VERSION,
    .pmu_ctrl = NOC_PMU_GLOBAL_CTRL,
    .event_ctrl0 = NOC_PMU_EVENT_CTRL0,
    .event_cntr0 = NOC_PMU_EVENT_COUNTER0,
    .overflow_status = NOC_PMU_CNT_INFO,
    };
    static const struct hisi_pmu_dev_info hisi_noc_v1 = {
    .attr_groups = hisi_noc_pmu_attr_groups,
    .counter_bits = 64,
    .check_event = NOC_PMU_EVENT_CTRL_TYPE,
    .private = &hisi_noc_v1_pmu_regs,
    };
    static const struct acpi_device_id hisi_noc_pmu_ids[] = {
    { "HISI04E0", (kernel_ulong_t) &hisi_noc_v1 },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, hisi_noc_pmu_ids);
    static struct platform_driver hisi_noc_pmu_driver = {
    .driver = {
    .name = "hisi_noc_pmu",
    .acpi_match_table = hisi_noc_pmu_ids,
    .suppress_bind_attrs = true,
    },
    .probe = hisi_noc_pmu_probe,
    };
#[no_mangle]
unsafe extern "C" fn hisi_noc_pmu_module_init() -> int __init {
    static int __init hisi_noc_pmu_module_init(void)
    {
    int ret;
    ret = cpuhp_setup_state_multi(CPUHP_AP_ONLINE_DYN, "perf/hisi/noc:online",
    hisi_uncore_pmu_online_cpu,
    hisi_uncore_pmu_offline_cpu);
    if (ret < 0) {
    pr_err("hisi_noc_pmu: Fail to setup cpuhp callbacks, ret = %d\n", ret);
    return ret;
    }
    hisi_noc_pmu_cpuhp_state = ret;
    ret = platform_driver_register(&hisi_noc_pmu_driver);
    if (ret)
    cpuhp_remove_multi_state(hisi_noc_pmu_cpuhp_state);
    return ret;
    }
    module_init(hisi_noc_pmu_module_init);
#[no_mangle]
unsafe extern "C" fn hisi_noc_pmu_module_exit() -> void __exit {
    static void __exit hisi_noc_pmu_module_exit(void)
    {
    platform_driver_unregister(&hisi_noc_pmu_driver);
    cpuhp_remove_multi_state(hisi_noc_pmu_cpuhp_state);
    }
    module_exit(hisi_noc_pmu_module_exit);
    MODULE_IMPORT_NS("HISI_PMU");
    MODULE_DESCRIPTION("HiSilicon SoC Uncore NoC PMU driver");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Yicong Yang <yangyicong@hisilicon.com>");
