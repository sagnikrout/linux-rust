//! Automatically rewritten from C to Rust
//! Source: drivers/perf/hisilicon/hisi_uncore_sllc_pmu.c
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
// HiSilicon SLLC uncore Hardware event counters support
//
// Copyright (C) 2020 HiSilicon Limited
// Author: Shaokun Zhang <zhangshaokun@hisilicon.com>
//
// This code is based on the uncore PMUs like arm-cci and arm-ccn.
//

// SLLC register definition
pub const SLLC_INT_MASK: c_uint = 0x0814;
pub const SLLC_INT_STATUS: c_uint = 0x0818;
pub const SLLC_INT_CLEAR: c_uint = 0x081c;
pub const SLLC_PERF_CTRL: c_uint = 0x1c00;
pub const SLLC_SRCID_CTRL: c_uint = 0x1c04;
pub const SLLC_TGTID_CTRL: c_uint = 0x1c08;
pub const SLLC_EVENT_CTRL: c_uint = 0x1c14;
pub const SLLC_EVENT_TYPE0: c_uint = 0x1c18;
pub const SLLC_VERSION: c_uint = 0x1cf0;
pub const SLLC_EVENT_CNT0_L: c_uint = 0x1d00;
// SLLC registers definition in v3
pub const SLLC_V3_INT_MASK: c_uint = 0x6834;
pub const SLLC_V3_INT_STATUS: c_uint = 0x6838;
pub const SLLC_V3_INT_CLEAR: c_uint = 0x683c;
pub const SLLC_V3_VERSION: c_uint = 0x6c00;
pub const SLLC_V3_PERF_CTRL: c_uint = 0x6d00;
pub const SLLC_V3_SRCID_CTRL: c_uint = 0x6d04;
pub const SLLC_V3_TGTID_CTRL: c_uint = 0x6d08;
pub const SLLC_V3_EVENT_CTRL: c_uint = 0x6d14;
pub const SLLC_V3_EVENT_TYPE0: c_uint = 0x6d18;
pub const SLLC_V3_EVENT_CNT0_L: c_uint = 0x6e00;
pub const SLLC_EVTYPE_MASK: c_uint = 0xff;

pub const SLLC_SRCID_NONE: c_uint = 0x0;

pub const SLLC_TGTID_NONE: c_uint = 0x0;
pub const SLLC_TGTID_MIN_SHIFT: c_int = 1;
pub const SLLC_TGTID_MAX_SHIFT: c_int = 12;
pub const SLLC_SRCID_CMD_SHIFT: c_int = 1;
pub const SLLC_SRCID_MSK_SHIFT: c_int = 12;
pub const SLLC_V3_TGTID_MIN_SHIFT: c_int = 1;
pub const SLLC_V3_TGTID_MAX_SHIFT: c_int = 10;
pub const SLLC_V3_SRCID_CMD_SHIFT: c_int = 1;
pub const SLLC_V3_SRCID_MSK_SHIFT: c_int = 10;
pub const SLLC_NR_EVENTS: c_uint = 0xff;

    HISI_PMU_EVENT_ATTR_EXTRACTOR(tgtid_min, config1, 10, 0);
    HISI_PMU_EVENT_ATTR_EXTRACTOR(tgtid_max, config1, 21, 11);
    HISI_PMU_EVENT_ATTR_EXTRACTOR(srcid_cmd, config1, 32, 22);
    HISI_PMU_EVENT_ATTR_EXTRACTOR(srcid_msk, config1, 43, 33);
    HISI_PMU_EVENT_ATTR_EXTRACTOR(tracetag_en, config1, 44, 44);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_sllc_pmu_regs {
    pub int_mask: u32,
    pub int_clear: u32,
    pub int_status: u32,
    pub perf_ctrl: u32,
    pub srcid_ctrl: u32,
    pub srcid_cmd_shift: u32,
    pub srcid_mask_shift: u32,
    pub tgtid_ctrl: u32,
    pub tgtid_min_shift: u32,
    pub tgtid_max_shift: u32,
    pub event_ctrl: u32,
    pub event_type0: u32,
    pub version: u32,
    pub event_cnt0: u32,
}

#[no_mangle]
unsafe extern "C" fn tgtid_is_valid(max: u32, min: u32) -> bool {
    static bool tgtid_is_valid(u32 max, u32 min)
    {
    return max > 0 && max >= min;
    }
#[no_mangle]
unsafe extern "C" fn hisi_sllc_pmu_enable_tracetag(event: *mut perf_event) {
    static void hisi_sllc_pmu_enable_tracetag(struct perf_event *event)
    {
    struct hisi_pmu *sllc_pmu = to_hisi_pmu(event.pmu);
    struct hisi_sllc_pmu_regs *regs = sllc_pmu.dev_info.private;
    let mut tt_en: u32 = hisi_get_tracetag_en(event);
    if (tt_en) {
    u32 val;
    val = readl(sllc_pmu.base + regs.perf_ctrl);
    val |= SLLC_TRACETAG_EN | SLLC_FILT_EN;
    writel(val, sllc_pmu.base + regs.perf_ctrl);
    }
    }
#[no_mangle]
unsafe extern "C" fn hisi_sllc_pmu_disable_tracetag(event: *mut perf_event) {
    static void hisi_sllc_pmu_disable_tracetag(struct perf_event *event)
    {
    struct hisi_pmu *sllc_pmu = to_hisi_pmu(event.pmu);
    struct hisi_sllc_pmu_regs *regs = sllc_pmu.dev_info.private;
    let mut tt_en: u32 = hisi_get_tracetag_en(event);
    if (tt_en) {
    u32 val;
    val = readl(sllc_pmu.base + regs.perf_ctrl);
    val &= ~(SLLC_TRACETAG_EN | SLLC_FILT_EN);
    writel(val, sllc_pmu.base + regs.perf_ctrl);
    }
    }
#[no_mangle]
unsafe extern "C" fn hisi_sllc_pmu_config_tgtid(event: *mut perf_event) {
    static void hisi_sllc_pmu_config_tgtid(struct perf_event *event)
    {
    struct hisi_pmu *sllc_pmu = to_hisi_pmu(event.pmu);
    struct hisi_sllc_pmu_regs *regs = sllc_pmu.dev_info.private;
    let mut min: u32 = hisi_get_tgtid_min(event);
    let mut max: u32 = hisi_get_tgtid_max(event);
    if (tgtid_is_valid(max, min)) {
    u32 val = (max << regs.tgtid_max_shift) |
    (min << regs.tgtid_min_shift);
    writel(val, sllc_pmu.base + regs.tgtid_ctrl);
// Enable the tgtid
    val = readl(sllc_pmu.base + regs.perf_ctrl);
    val |= SLLC_TGTID_EN | SLLC_FILT_EN;
    writel(val, sllc_pmu.base + regs.perf_ctrl);
    }
    }
#[no_mangle]
unsafe extern "C" fn hisi_sllc_pmu_clear_tgtid(event: *mut perf_event) {
    static void hisi_sllc_pmu_clear_tgtid(struct perf_event *event)
    {
    struct hisi_pmu *sllc_pmu = to_hisi_pmu(event.pmu);
    struct hisi_sllc_pmu_regs *regs = sllc_pmu.dev_info.private;
    let mut min: u32 = hisi_get_tgtid_min(event);
    let mut max: u32 = hisi_get_tgtid_max(event);
    if (tgtid_is_valid(max, min)) {
    u32 val;
    writel(SLLC_TGTID_NONE, sllc_pmu.base + regs.tgtid_ctrl);
// Disable the tgtid
    val = readl(sllc_pmu.base + regs.perf_ctrl);
    val &= ~(SLLC_TGTID_EN | SLLC_FILT_EN);
    writel(val, sllc_pmu.base + regs.perf_ctrl);
    }
    }
#[no_mangle]
unsafe extern "C" fn hisi_sllc_pmu_config_srcid(event: *mut perf_event) {
    static void hisi_sllc_pmu_config_srcid(struct perf_event *event)
    {
    struct hisi_pmu *sllc_pmu = to_hisi_pmu(event.pmu);
    struct hisi_sllc_pmu_regs *regs = sllc_pmu.dev_info.private;
    let mut cmd: u32 = hisi_get_srcid_cmd(event);
    if (cmd) {
    u32 val, msk;
    msk = hisi_get_srcid_msk(event);
    val = (cmd << regs.srcid_cmd_shift) |
    (msk << regs.srcid_mask_shift);
    writel(val, sllc_pmu.base + regs.srcid_ctrl);
// Enable the srcid
    val = readl(sllc_pmu.base + regs.perf_ctrl);
    val |= SLLC_SRCID_EN | SLLC_FILT_EN;
    writel(val, sllc_pmu.base + regs.perf_ctrl);
    }
    }
#[no_mangle]
unsafe extern "C" fn hisi_sllc_pmu_clear_srcid(event: *mut perf_event) {
    static void hisi_sllc_pmu_clear_srcid(struct perf_event *event)
    {
    struct hisi_pmu *sllc_pmu = to_hisi_pmu(event.pmu);
    struct hisi_sllc_pmu_regs *regs = sllc_pmu.dev_info.private;
    let mut cmd: u32 = hisi_get_srcid_cmd(event);
    if (cmd) {
    u32 val;
    writel(SLLC_SRCID_NONE, sllc_pmu.base + regs.srcid_ctrl);
// Disable the srcid
    val = readl(sllc_pmu.base + regs.perf_ctrl);
    val &= ~(SLLC_SRCID_EN | SLLC_FILT_EN);
    writel(val, sllc_pmu.base + regs.perf_ctrl);
    }
    }
#[no_mangle]
unsafe extern "C" fn hisi_sllc_pmu_enable_filter(event: *mut perf_event) {
    static void hisi_sllc_pmu_enable_filter(struct perf_event *event)
    {
    if (event.attr.config1 != 0x0) {
    hisi_sllc_pmu_enable_tracetag(event);
    hisi_sllc_pmu_config_srcid(event);
    hisi_sllc_pmu_config_tgtid(event);
    }
    }
#[no_mangle]
unsafe extern "C" fn hisi_sllc_pmu_clear_filter(event: *mut perf_event) {
    static void hisi_sllc_pmu_clear_filter(struct perf_event *event)
    {
    if (event.attr.config1 != 0x0) {
    hisi_sllc_pmu_disable_tracetag(event);
    hisi_sllc_pmu_clear_srcid(event);
    hisi_sllc_pmu_clear_tgtid(event);
    }
    }
    static u64 hisi_sllc_pmu_read_counter(struct hisi_pmu *sllc_pmu,
    struct hw_perf_event *hwc)
    {
    struct hisi_sllc_pmu_regs *regs = sllc_pmu.dev_info.private;
    return readq(sllc_pmu.base + SLLC_EVENT_CNTn(regs.event_cnt0, hwc.idx));
    }
    static void hisi_sllc_pmu_write_counter(struct hisi_pmu *sllc_pmu,
    struct hw_perf_event *hwc, u64 val)
    {
    struct hisi_sllc_pmu_regs *regs = sllc_pmu.dev_info.private;
    writeq(val, sllc_pmu.base + SLLC_EVENT_CNTn(regs.event_cnt0, hwc.idx));
    }
    static void hisi_sllc_pmu_write_evtype(struct hisi_pmu *sllc_pmu, int idx,
    u32 type)
    {
    struct hisi_sllc_pmu_regs *regs = sllc_pmu.dev_info.private;
    u32 reg, val;
//
// Select the appropriate event select register(SLLC_EVENT_TYPE0/1).
// There are 2 event select registers for the 8 hardware counters.
// Event code is 8-bits and for the former 4 hardware counters,
// SLLC_EVENT_TYPE0 is chosen. For the latter 4 hardware counters,
// SLLC_EVENT_TYPE1 is chosen.
//
    reg = regs.event_type0 + (idx / 4) * 4;
// Write event code to SLLC_EVENT_TYPEx Register
    val = readl(sllc_pmu.base + reg);
    val &= ~(SLLC_EVTYPE_MASK << HISI_PMU_EVTYPE_SHIFT(idx));
    val |= (type << HISI_PMU_EVTYPE_SHIFT(idx));
    writel(val, sllc_pmu.base + reg);
    }
#[no_mangle]
unsafe extern "C" fn hisi_sllc_pmu_start_counters(sllc_pmu: *mut hisi_pmu) {
    static void hisi_sllc_pmu_start_counters(struct hisi_pmu *sllc_pmu)
    {
    struct hisi_sllc_pmu_regs *regs = sllc_pmu.dev_info.private;
    u32 val;
    val = readl(sllc_pmu.base + regs.perf_ctrl);
    val |= SLLC_PERF_CTRL_EN;
    writel(val, sllc_pmu.base + regs.perf_ctrl);
    }
#[no_mangle]
unsafe extern "C" fn hisi_sllc_pmu_stop_counters(sllc_pmu: *mut hisi_pmu) {
    static void hisi_sllc_pmu_stop_counters(struct hisi_pmu *sllc_pmu)
    {
    struct hisi_sllc_pmu_regs *regs = sllc_pmu.dev_info.private;
    u32 val;
    val = readl(sllc_pmu.base + regs.perf_ctrl);
    val &= ~(SLLC_PERF_CTRL_EN);
    writel(val, sllc_pmu.base + regs.perf_ctrl);
    }
    static void hisi_sllc_pmu_enable_counter(struct hisi_pmu *sllc_pmu,
    struct hw_perf_event *hwc)
    {
    struct hisi_sllc_pmu_regs *regs = sllc_pmu.dev_info.private;
    u32 val;
    val = readl(sllc_pmu.base + regs.event_ctrl);
    val |= BIT_ULL(hwc.idx);
    writel(val, sllc_pmu.base + regs.event_ctrl);
    }
    static void hisi_sllc_pmu_disable_counter(struct hisi_pmu *sllc_pmu,
    struct hw_perf_event *hwc)
    {
    struct hisi_sllc_pmu_regs *regs = sllc_pmu.dev_info.private;
    u32 val;
    val = readl(sllc_pmu.base + regs.event_ctrl);
    val &= ~BIT_ULL(hwc.idx);
    writel(val, sllc_pmu.base + regs.event_ctrl);
    }
    static void hisi_sllc_pmu_enable_counter_int(struct hisi_pmu *sllc_pmu,
    struct hw_perf_event *hwc)
    {
    struct hisi_sllc_pmu_regs *regs = sllc_pmu.dev_info.private;
    u32 val;
    val = readl(sllc_pmu.base + regs.int_mask);
    val &= ~BIT_ULL(hwc.idx);
    writel(val, sllc_pmu.base + regs.int_mask);
    }
    static void hisi_sllc_pmu_disable_counter_int(struct hisi_pmu *sllc_pmu,
    struct hw_perf_event *hwc)
    {
    struct hisi_sllc_pmu_regs *regs = sllc_pmu.dev_info.private;
    u32 val;
    val = readl(sllc_pmu.base + regs.int_mask);
    val |= BIT_ULL(hwc.idx);
    writel(val, sllc_pmu.base + regs.int_mask);
    }
#[no_mangle]
unsafe extern "C" fn hisi_sllc_pmu_get_int_status(sllc_pmu: *mut hisi_pmu) -> u32 {
    static u32 hisi_sllc_pmu_get_int_status(struct hisi_pmu *sllc_pmu)
    {
    struct hisi_sllc_pmu_regs *regs = sllc_pmu.dev_info.private;
    return readl(sllc_pmu.base + regs.int_status);
    }
#[no_mangle]
unsafe extern "C" fn hisi_sllc_pmu_clear_int_status(sllc_pmu: *mut hisi_pmu, idx: c_int) {
    static void hisi_sllc_pmu_clear_int_status(struct hisi_pmu *sllc_pmu, int idx)
    {
    struct hisi_sllc_pmu_regs *regs = sllc_pmu.dev_info.private;
    writel(BIT_ULL(idx), sllc_pmu.base + regs.int_clear);
    }
    static int hisi_sllc_pmu_init_data(struct platform_device *pdev,
    struct hisi_pmu *sllc_pmu)
    {
    struct hisi_sllc_pmu_regs *regs;
    hisi_uncore_pmu_init_topology(sllc_pmu, &pdev.dev);
//
// Use the SCCL_ID and the index ID to identify the SLLC PMU,
// while SCCL_ID is from MPIDR_EL1 by CPU.
//
    if (sllc_pmu.topo.sccl_id < 0) {
    dev_err(&pdev.dev, "Cannot read sccl-id!\n");
    return -EINVAL;
    }
    if (sllc_pmu.topo.index_id < 0) {
    dev_err(&pdev.dev, "Cannot read idx-id!\n");
    return -EINVAL;
    }
    sllc_pmu.dev_info = device_get_match_data(&pdev.dev);
    if (!sllc_pmu.dev_info)
    return -ENODEV;
    sllc_pmu.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(sllc_pmu.base)) {
    dev_err(&pdev.dev, "ioremap failed for sllc_pmu resource.\n");
    return PTR_ERR(sllc_pmu.base);
    }
    regs = sllc_pmu.dev_info.private;
    sllc_pmu.identifier = readl(sllc_pmu.base + regs.version);
    return 0;
    }
    static struct attribute *hisi_sllc_pmu_v2_format_attr[] = {
    HISI_PMU_FORMAT_ATTR(event, "config:0-7"),
    HISI_PMU_FORMAT_ATTR(tgtid_min, "config1:0-10"),
    HISI_PMU_FORMAT_ATTR(tgtid_max, "config1:11-21"),
    HISI_PMU_FORMAT_ATTR(srcid_cmd, "config1:22-32"),
    HISI_PMU_FORMAT_ATTR(srcid_msk, "config1:33-43"),
    HISI_PMU_FORMAT_ATTR(tracetag_en, "config1:44"),
    core::ptr::null_mut()
    };
    static const struct attribute_group hisi_sllc_pmu_v2_format_group = {
    .name = "format",
    .attrs = hisi_sllc_pmu_v2_format_attr,
    };
    static struct attribute *hisi_sllc_pmu_v2_events_attr[] = {
    HISI_PMU_EVENT_ATTR(rx_req,             0x30),
    HISI_PMU_EVENT_ATTR(rx_data,            0x31),
    HISI_PMU_EVENT_ATTR(tx_req,             0x34),
    HISI_PMU_EVENT_ATTR(tx_data,            0x35),
    HISI_PMU_EVENT_ATTR(cycles,             0x09),
    core::ptr::null_mut()
    };
    static const struct attribute_group hisi_sllc_pmu_v2_events_group = {
    .name = "events",
    .attrs = hisi_sllc_pmu_v2_events_attr,
    };
    static const struct attribute_group *hisi_sllc_pmu_v2_attr_groups[] = {
    &hisi_sllc_pmu_v2_format_group,
    &hisi_sllc_pmu_v2_events_group,
    &hisi_pmu_cpumask_attr_group,
    &hisi_pmu_identifier_group,
    core::ptr::null_mut()
    };
    static struct hisi_sllc_pmu_regs hisi_sllc_v2_pmu_regs = {
    .int_mask = SLLC_INT_MASK,
    .int_clear = SLLC_INT_CLEAR,
    .int_status = SLLC_INT_STATUS,
    .perf_ctrl = SLLC_PERF_CTRL,
    .srcid_ctrl = SLLC_SRCID_CTRL,
    .srcid_cmd_shift = SLLC_SRCID_CMD_SHIFT,
    .srcid_mask_shift = SLLC_SRCID_MSK_SHIFT,
    .tgtid_ctrl = SLLC_TGTID_CTRL,
    .tgtid_min_shift = SLLC_TGTID_MIN_SHIFT,
    .tgtid_max_shift = SLLC_TGTID_MAX_SHIFT,
    .event_ctrl = SLLC_EVENT_CTRL,
    .event_type0 = SLLC_EVENT_TYPE0,
    .version = SLLC_VERSION,
    .event_cnt0 = SLLC_EVENT_CNT0_L,
    };
    static const struct hisi_pmu_dev_info hisi_sllc_v2 = {
    .private = &hisi_sllc_v2_pmu_regs,
    };
    static struct hisi_sllc_pmu_regs hisi_sllc_v3_pmu_regs = {
    .int_mask = SLLC_V3_INT_MASK,
    .int_clear = SLLC_V3_INT_CLEAR,
    .int_status = SLLC_V3_INT_STATUS,
    .perf_ctrl = SLLC_V3_PERF_CTRL,
    .srcid_ctrl = SLLC_V3_SRCID_CTRL,
    .srcid_cmd_shift = SLLC_V3_SRCID_CMD_SHIFT,
    .srcid_mask_shift = SLLC_V3_SRCID_MSK_SHIFT,
    .tgtid_ctrl = SLLC_V3_TGTID_CTRL,
    .tgtid_min_shift = SLLC_V3_TGTID_MIN_SHIFT,
    .tgtid_max_shift = SLLC_V3_TGTID_MAX_SHIFT,
    .event_ctrl = SLLC_V3_EVENT_CTRL,
    .event_type0 = SLLC_V3_EVENT_TYPE0,
    .version = SLLC_V3_VERSION,
    .event_cnt0 = SLLC_V3_EVENT_CNT0_L,
    };
    static const struct hisi_pmu_dev_info hisi_sllc_v3 = {
    .private = &hisi_sllc_v3_pmu_regs,
    };
    static const struct hisi_uncore_ops hisi_uncore_sllc_ops = {
    .write_evtype		= hisi_sllc_pmu_write_evtype,
    .get_event_idx		= hisi_uncore_pmu_get_event_idx,
    .start_counters		= hisi_sllc_pmu_start_counters,
    .stop_counters		= hisi_sllc_pmu_stop_counters,
    .enable_counter		= hisi_sllc_pmu_enable_counter,
    .disable_counter	= hisi_sllc_pmu_disable_counter,
    .enable_counter_int	= hisi_sllc_pmu_enable_counter_int,
    .disable_counter_int	= hisi_sllc_pmu_disable_counter_int,
    .write_counter		= hisi_sllc_pmu_write_counter,
    .read_counter		= hisi_sllc_pmu_read_counter,
    .get_int_status		= hisi_sllc_pmu_get_int_status,
    .clear_int_status	= hisi_sllc_pmu_clear_int_status,
    .enable_filter		= hisi_sllc_pmu_enable_filter,
    .disable_filter		= hisi_sllc_pmu_clear_filter,
    };
    static int hisi_sllc_pmu_dev_probe(struct platform_device *pdev,
    struct hisi_pmu *sllc_pmu)
    {
    int ret;
    ret = hisi_sllc_pmu_init_data(pdev, sllc_pmu);
    if (ret)
    return ret;
    ret = hisi_uncore_pmu_init_irq(sllc_pmu, pdev);
    if (ret)
    return ret;
    sllc_pmu.pmu_events.attr_groups = hisi_sllc_pmu_v2_attr_groups;
    sllc_pmu.ops = &hisi_uncore_sllc_ops;
    sllc_pmu.check_event = SLLC_NR_EVENTS;
    sllc_pmu.counter_bits = 64;
    sllc_pmu.num_counters = 8;
    sllc_pmu.dev = &pdev.dev;
    sllc_pmu.on_cpu = -1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hisi_sllc_pmu_probe(pdev: *mut platform_device) -> c_int {
    static int hisi_sllc_pmu_probe(struct platform_device *pdev)
    {
    struct hisi_pmu *sllc_pmu;
    char *name;
    int ret;
    sllc_pmu = devm_kzalloc(&pdev.dev, sizeof(*sllc_pmu), GFP_KERNEL);
    if (!sllc_pmu)
    return -ENOMEM;
    ret = hisi_sllc_pmu_dev_probe(pdev, sllc_pmu);
    if (ret)
    return ret;
    name = devm_kasprintf(&pdev.dev, GFP_KERNEL, "hisi_sccl%d_sllc%d",
    sllc_pmu.topo.sccl_id, sllc_pmu.topo.index_id);
    if (!name)
    return -ENOMEM;
    ret = cpuhp_state_add_instance(CPUHP_AP_PERF_ARM_HISI_SLLC_ONLINE,
    &sllc_pmu.node);
    if (ret) {
    dev_err(&pdev.dev, "Error %d registering hotplug\n", ret);
    return ret;
    }
    hisi_pmu_init(sllc_pmu, THIS_MODULE);
    ret = perf_pmu_register(&sllc_pmu.pmu, name, -1);
    if (ret) {
    dev_err(sllc_pmu.dev, "PMU register failed, ret = %d\n", ret);
    cpuhp_state_remove_instance_nocalls(CPUHP_AP_PERF_ARM_HISI_SLLC_ONLINE,
    &sllc_pmu.node);
    return ret;
    }
    platform_set_drvdata(pdev, sllc_pmu);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hisi_sllc_pmu_remove(pdev: *mut platform_device) {
    static void hisi_sllc_pmu_remove(struct platform_device *pdev)
    {
    struct hisi_pmu *sllc_pmu = platform_get_drvdata(pdev);
    perf_pmu_unregister(&sllc_pmu.pmu);
    cpuhp_state_remove_instance_nocalls(CPUHP_AP_PERF_ARM_HISI_SLLC_ONLINE,
    &sllc_pmu.node);
    }
    static const struct acpi_device_id hisi_sllc_pmu_acpi_match[] = {
    { "HISI0263", (kernel_ulong_t)&hisi_sllc_v2 },
    { "HISI0264", (kernel_ulong_t)&hisi_sllc_v3 },
    {}
    };
    MODULE_DEVICE_TABLE(acpi, hisi_sllc_pmu_acpi_match);
    static struct platform_driver hisi_sllc_pmu_driver = {
    .driver = {
    .name = "hisi_sllc_pmu",
    .acpi_match_table = hisi_sllc_pmu_acpi_match,
    .suppress_bind_attrs = true,
    },
    .probe = hisi_sllc_pmu_probe,
    .remove = hisi_sllc_pmu_remove,
    };
#[no_mangle]
unsafe extern "C" fn hisi_sllc_pmu_module_init() -> int __init {
    static int __init hisi_sllc_pmu_module_init(void)
    {
    int ret;
    ret = cpuhp_setup_state_multi(CPUHP_AP_PERF_ARM_HISI_SLLC_ONLINE,
    "AP_PERF_ARM_HISI_SLLC_ONLINE",
    hisi_uncore_pmu_online_cpu,
    hisi_uncore_pmu_offline_cpu);
    if (ret) {
    pr_err("SLLC PMU: cpuhp state setup failed, ret = %d\n", ret);
    return ret;
    }
    ret = platform_driver_register(&hisi_sllc_pmu_driver);
    if (ret)
    cpuhp_remove_multi_state(CPUHP_AP_PERF_ARM_HISI_SLLC_ONLINE);
    return ret;
    }
    module_init(hisi_sllc_pmu_module_init);
#[no_mangle]
unsafe extern "C" fn hisi_sllc_pmu_module_exit() -> void __exit {
    static void __exit hisi_sllc_pmu_module_exit(void)
    {
    platform_driver_unregister(&hisi_sllc_pmu_driver);
    cpuhp_remove_multi_state(CPUHP_AP_PERF_ARM_HISI_SLLC_ONLINE);
    }
    module_exit(hisi_sllc_pmu_module_exit);
    MODULE_IMPORT_NS("HISI_PMU");
    MODULE_DESCRIPTION("HiSilicon SLLC uncore PMU driver");
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Shaokun Zhang <zhangshaokun@hisilicon.com>");
    MODULE_AUTHOR("Qi Liu <liuqi115@huawei.com>");
