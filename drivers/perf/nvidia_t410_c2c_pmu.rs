//! Automatically rewritten from C to Rust
//! Source: drivers/perf/nvidia_t410_c2c_pmu.c
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
// NVIDIA Tegra410 C2C PMU driver.
//
// Copyright (c) 2026, NVIDIA CORPORATION & AFFILIATES. All rights reserved.
//

// The C2C interface types in Tegra410.
pub const C2C_TYPE_NVLINK: c_uint = 0x0;
pub const C2C_TYPE_NVCLINK: c_uint = 0x1;
pub const C2C_TYPE_NVDLINK: c_uint = 0x2;
pub const C2C_TYPE_COUNT: c_uint = 0x3;
// The type of the peer device connected to the C2C interface.
pub const C2C_PEER_TYPE_CPU: c_uint = 0x0;
pub const C2C_PEER_TYPE_GPU: c_uint = 0x1;
pub const C2C_PEER_TYPE_CXLMEM: c_uint = 0x2;
pub const C2C_PEER_TYPE_COUNT: c_uint = 0x3;
// The number of peer devices can be connected to the C2C interface.
pub const C2C_NR_PEER_CPU: c_uint = 0x1;
pub const C2C_NR_PEER_GPU: c_uint = 0x2;
pub const C2C_NR_PEER_CXLMEM: c_uint = 0x1;
pub const C2C_NR_PEER_MAX: c_uint = 0x2;
// Number of instances on each interface.
pub const C2C_NR_INST_NVLINK: c_int = 14;
pub const C2C_NR_INST_NVCLINK: c_int = 12;
pub const C2C_NR_INST_NVDLINK: c_int = 16;
pub const C2C_NR_INST_MAX: c_int = 16;
// Register offsets.
pub const C2C_CTRL: c_uint = 0x864;
pub const C2C_IN_STATUS: c_uint = 0x868;
pub const C2C_CYCLE_CNTR: c_uint = 0x86c;
pub const C2C_IN_RD_CUM_OUTS_CNTR: c_uint = 0x874;
pub const C2C_IN_RD_REQ_CNTR: c_uint = 0x87c;
pub const C2C_IN_WR_CUM_OUTS_CNTR: c_uint = 0x884;
pub const C2C_IN_WR_REQ_CNTR: c_uint = 0x88c;
pub const C2C_OUT_STATUS: c_uint = 0x890;
pub const C2C_OUT_RD_CUM_OUTS_CNTR: c_uint = 0x898;
pub const C2C_OUT_RD_REQ_CNTR: c_uint = 0x8a0;
pub const C2C_OUT_WR_CUM_OUTS_CNTR: c_uint = 0x8a8;
pub const C2C_OUT_WR_REQ_CNTR: c_uint = 0x8b0;
// C2C_IN_STATUS register field.

// C2C_OUT_STATUS register field.

// Events.
pub const C2C_EVENT_CYCLES: c_uint = 0x0;
pub const C2C_EVENT_IN_RD_CUM_OUTS: c_uint = 0x1;
pub const C2C_EVENT_IN_RD_REQ: c_uint = 0x2;
pub const C2C_EVENT_IN_WR_CUM_OUTS: c_uint = 0x3;
pub const C2C_EVENT_IN_WR_REQ: c_uint = 0x4;
pub const C2C_EVENT_OUT_RD_CUM_OUTS: c_uint = 0x5;
pub const C2C_EVENT_OUT_RD_REQ: c_uint = 0x6;
pub const C2C_EVENT_OUT_WR_CUM_OUTS: c_uint = 0x7;
pub const C2C_EVENT_OUT_WR_REQ: c_uint = 0x8;
pub const C2C_NUM_EVENTS: c_uint = 0x9;
pub const C2C_MASK_EVENT: c_uint = 0xFF;
pub const C2C_MAX_ACTIVE_EVENTS: c_int = 32;
pub const C2C_ACTIVE_CPU_MASK: c_uint = 0x0;
pub const C2C_ASSOCIATED_CPU_MASK: c_uint = 0x1;
//
// Maximum poll count for reading counter value using high-low-high sequence.
//
pub const HILOHI_MAX_POLL: c_int = 1000;
    static unsigned long nv_c2c_pmu_cpuhp_state;
// PMU descriptor.
// C2C type information.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv_c2c_pmu_data {
    pub c2c_type: c_uint,
    pub nr_inst: c_uint,
    pub name_fmt: *const c_char,
}

    static const struct nv_c2c_pmu_data nv_c2c_pmu_data[] = {
    [C2C_TYPE_NVLINK] = {
    .c2c_type = C2C_TYPE_NVLINK,
    .nr_inst = C2C_NR_INST_NVLINK,
    .name_fmt = "nvidia_nvlink_c2c_pmu_%u",
    },
    [C2C_TYPE_NVCLINK] = {
    .c2c_type = C2C_TYPE_NVCLINK,
    .nr_inst = C2C_NR_INST_NVCLINK,
    .name_fmt = "nvidia_nvclink_pmu_%u",
    },
    [C2C_TYPE_NVDLINK] = {
    .c2c_type = C2C_TYPE_NVDLINK,
    .nr_inst = C2C_NR_INST_NVDLINK,
    .name_fmt = "nvidia_nvdlink_pmu_%u",
    },
    };
// Tracks the events assigned to the PMU for a given logical index.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv_c2c_pmu_hw_events {
// The events that are active.
    pub events: [*mut perf_event; C2C_MAX_ACTIVE_EVENTS],
//
// Each bit indicates a logical counter is being used (or not) for an
// event.
//
    pub C2C_MAX_ACTIVE_EVENTS): DECLARE_BITMAP(used_ctrs,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv_c2c_pmu {
    pub pmu: pmu,
    pub dev: *mut device,
    pub acpi_dev: *mut acpi_device,
    pub name: *const c_char,
    pub identifier: *const c_char,
    pub data: *const nv_c2c_pmu_data,
    pub peer_type: c_uint,
    pub socket: c_uint,
    pub nr_peer: c_uint,
    pub peer_insts: [c_ulong; C2C_NR_PEER_MAX][BITS_TO_LONGS(C2C_NR_INST_MAX)],
    pub filter_default: u32,
    pub hw_events: nv_c2c_pmu_hw_events,
    pub associated_cpus: cpumask_t,
    pub active_cpu: cpumask_t,
    pub cpuhp_node: hlist_node,
    pub attr_groups: *const attribute_group,
    pub base_broadcast: *mut void __iomem,
    pub base: [*mut void __iomem; C2C_NR_INST_MAX],
}

// Get event type from perf_event.
#[no_mangle]
pub unsafe extern "C" fn get_event_type(event: *mut perf_event) -> u32 {
    static inline u32 get_event_type(struct perf_event *event)
    {
    return (event.attr.config) & C2C_MASK_EVENT;
    }
#[no_mangle]
pub unsafe extern "C" fn get_filter_mask(event: *mut perf_event) -> u32 {
    static inline u32 get_filter_mask(struct perf_event *event)
    {
    u32 filter;
    struct nv_c2c_pmu *c2c_pmu = to_c2c_pmu(event.pmu);
    filter = ((u32)event.attr.config1) & c2c_pmu.filter_default;
    if (filter == 0)
    filter = c2c_pmu.filter_default;
    return filter;
    }
// PMU operations.
    static int nv_c2c_pmu_get_event_idx(struct nv_c2c_pmu_hw_events *hw_events,
    struct perf_event *event)
    {
    u32 idx;
    idx = find_first_zero_bit(hw_events.used_ctrs, C2C_MAX_ACTIVE_EVENTS);
    if (idx >= C2C_MAX_ACTIVE_EVENTS)
    return -EAGAIN;
    set_bit(idx, hw_events.used_ctrs);
    return idx;
    }
    static bool
    nv_c2c_pmu_validate_event(struct pmu *pmu,
    struct nv_c2c_pmu_hw_events *hw_events,
    struct perf_event *event)
    {
    if (is_software_event(event))
    return true;
// Reject groups spanning multiple HW PMUs.
    if (event.pmu != pmu)
    return false;
    return nv_c2c_pmu_get_event_idx(hw_events, event) >= 0;
    }
//
// Make sure the group of events can be scheduled at once
// on the PMU.
//
#[no_mangle]
unsafe extern "C" fn nv_c2c_pmu_validate_group(event: *mut perf_event) -> bool {
    static bool nv_c2c_pmu_validate_group(struct perf_event *event)
    {
    struct perf_event *sibling, *leader = event.group_leader;
    struct nv_c2c_pmu_hw_events fake_hw_events;
    if (event.group_leader == event)
    return true;
    memset(&fake_hw_events, 0, sizeof(fake_hw_events));
    if (!nv_c2c_pmu_validate_event(event.pmu, &fake_hw_events, leader))
    return false;
    for_each_sibling_event(sibling, leader) {
    if (!nv_c2c_pmu_validate_event(event.pmu, &fake_hw_events,
    sibling))
    return false;
    }
    return nv_c2c_pmu_validate_event(event.pmu, &fake_hw_events, event);
    }
#[no_mangle]
unsafe extern "C" fn nv_c2c_pmu_event_init(event: *mut perf_event) -> c_int {
    static int nv_c2c_pmu_event_init(struct perf_event *event)
    {
    struct nv_c2c_pmu *c2c_pmu = to_c2c_pmu(event.pmu);
    struct hw_perf_event *hwc = &event.hw;
    let mut event_type: u32 = get_event_type(event);
    if (event.attr.type != event.pmu.type ||
    event_type >= C2C_NUM_EVENTS)
    return -ENOENT;
//
// Following other "uncore" PMUs, we do not support sampling mode or
// attach to a task (per-process mode).
//
    if (is_sampling_event(event)) {
    dev_dbg(c2c_pmu.pmu.dev, "Can't support sampling events\n");
    return -EOPNOTSUPP;
    }
    if (event.cpu < 0 || event.attach_state & PERF_ATTACH_TASK) {
    dev_dbg(c2c_pmu.pmu.dev, "Can't support per-task counters\n");
    return -EINVAL;
    }
//
// Make sure the CPU assignment is on one of the CPUs associated with
// this PMU.
//
    if (!cpumask_test_cpu(event.cpu, &c2c_pmu.associated_cpus)) {
    dev_dbg(c2c_pmu.pmu.dev,
    "Requested cpu is not associated with the PMU\n");
    return -EINVAL;
    }
// Enforce the current active CPU to handle the events in this PMU.
    event.cpu = cpumask_first(&c2c_pmu.active_cpu);
    if (event.cpu >= nr_cpu_ids)
    return -EINVAL;
    if (!nv_c2c_pmu_validate_group(event))
    return -EINVAL;
    hwc.idx = -1;
    hwc.config = event_type;
    return 0;
    }
//
// Read 64-bit register as a pair of 32-bit registers using hi-lo-hi sequence.
//
#[no_mangle]
unsafe extern "C" fn read_reg64_hilohi(addr: *const void __iomem, max_poll_count: u32) -> u64 {
    static u64 read_reg64_hilohi(const void __iomem *addr, u32 max_poll_count)
    {
    u32 val_lo, val_hi;
    u64 val;
// Use high-low-high sequence to avoid tearing
    do {
    if (max_poll_count-- == 0) {
    pr_err("NV C2C PMU: timeout hi-low-high sequence\n");
    return 0;
    }
    val_hi = readl(addr + 4);
    val_lo = readl(addr);
    } while (val_hi != readl(addr + 4));
    val = (((u64)val_hi << 32) | val_lo);
    return val;
    }
#[no_mangle]
unsafe extern "C" fn nv_c2c_pmu_check_status(c2c_pmu: *mut nv_c2c_pmu, instance: u32) {
    static void nv_c2c_pmu_check_status(struct nv_c2c_pmu *c2c_pmu, u32 instance)
    {
    u32 in_status, out_status;
    in_status = readl(c2c_pmu.base[instance] + C2C_IN_STATUS);
    out_status = readl(c2c_pmu.base[instance] + C2C_OUT_STATUS);
    if (in_status || out_status)
    dev_warn(c2c_pmu.dev,
    "C2C PMU overflow in: 0x%x, out: 0x%x\n",
    in_status, out_status);
    }
    static u32 nv_c2c_ctr_offset[C2C_NUM_EVENTS] = {
    [C2C_EVENT_CYCLES] = C2C_CYCLE_CNTR,
    [C2C_EVENT_IN_RD_CUM_OUTS] = C2C_IN_RD_CUM_OUTS_CNTR,
    [C2C_EVENT_IN_RD_REQ] = C2C_IN_RD_REQ_CNTR,
    [C2C_EVENT_IN_WR_CUM_OUTS] = C2C_IN_WR_CUM_OUTS_CNTR,
    [C2C_EVENT_IN_WR_REQ] = C2C_IN_WR_REQ_CNTR,
    [C2C_EVENT_OUT_RD_CUM_OUTS] = C2C_OUT_RD_CUM_OUTS_CNTR,
    [C2C_EVENT_OUT_RD_REQ] = C2C_OUT_RD_REQ_CNTR,
    [C2C_EVENT_OUT_WR_CUM_OUTS] = C2C_OUT_WR_CUM_OUTS_CNTR,
    [C2C_EVENT_OUT_WR_REQ] = C2C_OUT_WR_REQ_CNTR,
    };
#[no_mangle]
unsafe extern "C" fn nv_c2c_pmu_read_counter(event: *mut perf_event) -> u64 {
    static u64 nv_c2c_pmu_read_counter(struct perf_event *event)
    {
    u32 ctr_id, ctr_offset, filter_mask, filter_idx, inst_idx;
    unsigned long *inst_mask;
    DECLARE_BITMAP(filter_bitmap, C2C_NR_PEER_MAX);
    struct nv_c2c_pmu *c2c_pmu = to_c2c_pmu(event.pmu);
    let mut val: u64 = 0;
    filter_mask = get_filter_mask(event);
    bitmap_from_arr32(filter_bitmap, &filter_mask, c2c_pmu.nr_peer);
    ctr_id = event.hw.config;
    ctr_offset = nv_c2c_ctr_offset[ctr_id];
    for_each_set_bit(filter_idx, filter_bitmap, c2c_pmu.nr_peer) {
    inst_mask = c2c_pmu.peer_insts[filter_idx];
    for_each_set_bit(inst_idx, inst_mask, c2c_pmu.data.nr_inst) {
    nv_c2c_pmu_check_status(c2c_pmu, inst_idx);
//
// Each instance share same clock and the driver always
// enables all instances. So we can use the counts from
// one instance for cycle counter.
//
    if (ctr_id == C2C_EVENT_CYCLES)
    return read_reg64_hilohi(
    c2c_pmu.base[inst_idx] + ctr_offset,
    HILOHI_MAX_POLL);
//
// For other events, sum up the counts from all instances.
//
    val += read_reg64_hilohi(
    c2c_pmu.base[inst_idx] + ctr_offset,
    HILOHI_MAX_POLL);
    }
    }
    return val;
    }
#[no_mangle]
unsafe extern "C" fn nv_c2c_pmu_event_update(event: *mut perf_event) {
    static void nv_c2c_pmu_event_update(struct perf_event *event)
    {
    struct hw_perf_event *hwc = &event.hw;
    u64 prev, now;
    do {
    prev = local64_read(&hwc.prev_count);
    now = nv_c2c_pmu_read_counter(event);
    } while (local64_cmpxchg(&hwc.prev_count, prev, now) != prev);
    local64_add(now - prev, &event.count);
    }
#[no_mangle]
unsafe extern "C" fn nv_c2c_pmu_start(event: *mut perf_event, pmu_flags: c_int) {
    static void nv_c2c_pmu_start(struct perf_event *event, int pmu_flags)
    {
    event.hw.state = 0;
    }
#[no_mangle]
unsafe extern "C" fn nv_c2c_pmu_stop(event: *mut perf_event, pmu_flags: c_int) {
    static void nv_c2c_pmu_stop(struct perf_event *event, int pmu_flags)
    {
    event.hw.state |= PERF_HES_STOPPED;
    }
#[no_mangle]
unsafe extern "C" fn nv_c2c_pmu_add(event: *mut perf_event, flags: c_int) -> c_int {
    static int nv_c2c_pmu_add(struct perf_event *event, int flags)
    {
    struct nv_c2c_pmu *c2c_pmu = to_c2c_pmu(event.pmu);
    struct nv_c2c_pmu_hw_events *hw_events = &c2c_pmu.hw_events;
    struct hw_perf_event *hwc = &event.hw;
    int idx;
    if (WARN_ON_ONCE(!cpumask_test_cpu(smp_processor_id(),
    &c2c_pmu.associated_cpus)))
    return -ENOENT;
    idx = nv_c2c_pmu_get_event_idx(hw_events, event);
    if (idx < 0)
    return idx;
    hw_events.events[idx] = event;
    hwc.idx = idx;
    hwc.state = PERF_HES_STOPPED | PERF_HES_UPTODATE;
    if (flags & PERF_EF_START)
    nv_c2c_pmu_start(event, PERF_EF_RELOAD);
// Propagate changes to the userspace mapping.
    perf_event_update_userpage(event);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nv_c2c_pmu_del(event: *mut perf_event, flags: c_int) {
    static void nv_c2c_pmu_del(struct perf_event *event, int flags)
    {
    struct nv_c2c_pmu *c2c_pmu = to_c2c_pmu(event.pmu);
    struct nv_c2c_pmu_hw_events *hw_events = &c2c_pmu.hw_events;
    struct hw_perf_event *hwc = &event.hw;
    let mut idx: c_int = hwc.idx;
    nv_c2c_pmu_stop(event, PERF_EF_UPDATE);
    hw_events.events[idx] = core::ptr::null_mut();
    clear_bit(idx, hw_events.used_ctrs);
    perf_event_update_userpage(event);
    }
#[no_mangle]
unsafe extern "C" fn nv_c2c_pmu_read(event: *mut perf_event) {
    static void nv_c2c_pmu_read(struct perf_event *event)
    {
    nv_c2c_pmu_event_update(event);
    }
#[no_mangle]
unsafe extern "C" fn nv_c2c_pmu_enable(pmu: *mut pmu) {
    static void nv_c2c_pmu_enable(struct pmu *pmu)
    {
    void __iomem *bcast;
    struct nv_c2c_pmu *c2c_pmu = to_c2c_pmu(pmu);
// Check if any filter is enabled.
    if (bitmap_empty(c2c_pmu.hw_events.used_ctrs, C2C_MAX_ACTIVE_EVENTS))
    return;
// Enable all the counters.
    bcast = c2c_pmu.base_broadcast;
    writel(0x1UL, bcast + C2C_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn nv_c2c_pmu_disable(pmu: *mut pmu) {
    static void nv_c2c_pmu_disable(struct pmu *pmu)
    {
    unsigned int idx;
    void __iomem *bcast;
    struct perf_event *event;
    struct nv_c2c_pmu *c2c_pmu = to_c2c_pmu(pmu);
// Disable all the counters.
    bcast = c2c_pmu.base_broadcast;
    writel(0x0UL, bcast + C2C_CTRL);
//
// The counters will start from 0 again on restart.
// Update the events immediately to avoid losing the counts.
//
    for_each_set_bit(idx, c2c_pmu.hw_events.used_ctrs,
    C2C_MAX_ACTIVE_EVENTS) {
    event = c2c_pmu.hw_events.events[idx];
    if (!event)
    continue;
    nv_c2c_pmu_event_update(event);
    local64_set(&event.hw.prev_count, 0ULL);
    }
    }
// PMU identifier attribute.
    static ssize_t nv_c2c_pmu_identifier_show(struct device *dev,
    struct device_attribute *attr,
    char *page)
    {
    struct nv_c2c_pmu *c2c_pmu = to_c2c_pmu(dev_get_drvdata(dev));
    return sysfs_emit(page, "%s\n", c2c_pmu.identifier);
    }
    static struct device_attribute nv_c2c_pmu_identifier_attr =
    __ATTR(identifier, 0444, nv_c2c_pmu_identifier_show, core::ptr::null_mut());
    static struct attribute *nv_c2c_pmu_identifier_attrs[] = {
    &nv_c2c_pmu_identifier_attr.attr,
    core::ptr::null_mut(),
    };
    static struct attribute_group nv_c2c_pmu_identifier_attr_group = {
    .attrs = nv_c2c_pmu_identifier_attrs,
    };
// Peer attribute.
    static ssize_t nv_c2c_pmu_peer_show(struct device *dev,
    struct device_attribute *attr,
    char *page)
    {
    const char *peer_type[C2C_PEER_TYPE_COUNT] = {
    [C2C_PEER_TYPE_CPU] = "cpu",
    [C2C_PEER_TYPE_GPU] = "gpu",
    [C2C_PEER_TYPE_CXLMEM] = "cxlmem",
    };
    struct nv_c2c_pmu *c2c_pmu = to_c2c_pmu(dev_get_drvdata(dev));
    return sysfs_emit(page, "nr_%s=%u\n", peer_type[c2c_pmu.peer_type],
    c2c_pmu.nr_peer);
    }
    static struct device_attribute nv_c2c_pmu_peer_attr =
    __ATTR(peer, 0444, nv_c2c_pmu_peer_show, core::ptr::null_mut());
    static struct attribute *nv_c2c_pmu_peer_attrs[] = {
    &nv_c2c_pmu_peer_attr.attr,
    core::ptr::null_mut(),
    };
    static struct attribute_group nv_c2c_pmu_peer_attr_group = {
    .attrs = nv_c2c_pmu_peer_attrs,
    };
// Format attributes.

    (&((struct dev_ext_attribute[]){				\
    {							\
    .attr = __ATTR(_name, 0444, _func, core::ptr::null_mut()),	\
    .var = (void *)_config				\
    }							\
    })[0].attr.attr)

    NV_C2C_PMU_EXT_ATTR(_name, device_show_string, _config)

    NV_C2C_PMU_FORMAT_ATTR(event, "config:0-3")
    static struct attribute *nv_c2c_pmu_gpu_formats[] = {
    NV_C2C_PMU_FORMAT_EVENT_ATTR,
    NV_C2C_PMU_FORMAT_ATTR(gpu_mask, "config1:0-1"),
    core::ptr::null_mut(),
    };
    static const struct attribute_group nv_c2c_pmu_gpu_format_group = {
    .name = "format",
    .attrs = nv_c2c_pmu_gpu_formats,
    };
    static struct attribute *nv_c2c_pmu_formats[] = {
    NV_C2C_PMU_FORMAT_EVENT_ATTR,
    core::ptr::null_mut(),
    };
    static const struct attribute_group nv_c2c_pmu_format_group = {
    .name = "format",
    .attrs = nv_c2c_pmu_formats,
    };
// Event attributes.
    static ssize_t nv_c2c_pmu_sysfs_event_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct perf_pmu_events_attr *pmu_attr;
    pmu_attr = container_of(attr, typeof(*pmu_attr), attr);
    return sysfs_emit(buf, "event=0x%llx\n", pmu_attr.id);
    }

    PMU_EVENT_ATTR_ID(_name, nv_c2c_pmu_sysfs_event_show, _config)
    static struct attribute *nv_c2c_pmu_gpu_events[] = {
    NV_C2C_PMU_EVENT_ATTR(cycles, C2C_EVENT_CYCLES),
    NV_C2C_PMU_EVENT_ATTR(in_rd_cum_outs, C2C_EVENT_IN_RD_CUM_OUTS),
    NV_C2C_PMU_EVENT_ATTR(in_rd_req, C2C_EVENT_IN_RD_REQ),
    NV_C2C_PMU_EVENT_ATTR(in_wr_cum_outs, C2C_EVENT_IN_WR_CUM_OUTS),
    NV_C2C_PMU_EVENT_ATTR(in_wr_req, C2C_EVENT_IN_WR_REQ),
    NV_C2C_PMU_EVENT_ATTR(out_rd_cum_outs, C2C_EVENT_OUT_RD_CUM_OUTS),
    NV_C2C_PMU_EVENT_ATTR(out_rd_req, C2C_EVENT_OUT_RD_REQ),
    NV_C2C_PMU_EVENT_ATTR(out_wr_cum_outs, C2C_EVENT_OUT_WR_CUM_OUTS),
    NV_C2C_PMU_EVENT_ATTR(out_wr_req, C2C_EVENT_OUT_WR_REQ),
    core::ptr::null_mut()
    };
    static const struct attribute_group nv_c2c_pmu_gpu_events_group = {
    .name = "events",
    .attrs = nv_c2c_pmu_gpu_events,
    };
    static struct attribute *nv_c2c_pmu_cpu_events[] = {
    NV_C2C_PMU_EVENT_ATTR(cycles, C2C_EVENT_CYCLES),
    NV_C2C_PMU_EVENT_ATTR(in_rd_cum_outs, C2C_EVENT_IN_RD_CUM_OUTS),
    NV_C2C_PMU_EVENT_ATTR(in_rd_req, C2C_EVENT_IN_RD_REQ),
    NV_C2C_PMU_EVENT_ATTR(out_rd_cum_outs, C2C_EVENT_OUT_RD_CUM_OUTS),
    NV_C2C_PMU_EVENT_ATTR(out_rd_req, C2C_EVENT_OUT_RD_REQ),
    core::ptr::null_mut()
    };
    static const struct attribute_group nv_c2c_pmu_cpu_events_group = {
    .name = "events",
    .attrs = nv_c2c_pmu_cpu_events,
    };
    static struct attribute *nv_c2c_pmu_cxlmem_events[] = {
    NV_C2C_PMU_EVENT_ATTR(cycles, C2C_EVENT_CYCLES),
    NV_C2C_PMU_EVENT_ATTR(in_rd_cum_outs, C2C_EVENT_IN_RD_CUM_OUTS),
    NV_C2C_PMU_EVENT_ATTR(in_rd_req, C2C_EVENT_IN_RD_REQ),
    core::ptr::null_mut()
    };
    static const struct attribute_group nv_c2c_pmu_cxlmem_events_group = {
    .name = "events",
    .attrs = nv_c2c_pmu_cxlmem_events,
    };
// Cpumask attributes.
    static ssize_t nv_c2c_pmu_cpumask_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct pmu *pmu = dev_get_drvdata(dev);
    struct nv_c2c_pmu *c2c_pmu = to_c2c_pmu(pmu);
    struct dev_ext_attribute *eattr =
    container_of(attr, struct dev_ext_attribute, attr);
    let mut mask_id: c_ulong = (unsigned long)eattr.var;
    const cpumask_t *cpumask;
    switch (mask_id) {
    case C2C_ACTIVE_CPU_MASK:
    cpumask = &c2c_pmu.active_cpu;
    break;
    case C2C_ASSOCIATED_CPU_MASK:
    cpumask = &c2c_pmu.associated_cpus;
    break;
    default:
    return 0;
    }
    return sysfs_emit(buf, "%*pbl\n", cpumask_pr_args(cpumask));
    }

    NV_C2C_PMU_EXT_ATTR(_name, nv_c2c_pmu_cpumask_show,	\
    (unsigned long)_config)
    static struct attribute *nv_c2c_pmu_cpumask_attrs[] = {
    NV_C2C_PMU_CPUMASK_ATTR(cpumask, C2C_ACTIVE_CPU_MASK),
    NV_C2C_PMU_CPUMASK_ATTR(associated_cpus, C2C_ASSOCIATED_CPU_MASK),
    core::ptr::null_mut(),
    };
    static const struct attribute_group nv_c2c_pmu_cpumask_attr_group = {
    .attrs = nv_c2c_pmu_cpumask_attrs,
    };
// Attribute groups for C2C PMU connecting SoC and GPU
    static const struct attribute_group *nv_c2c_pmu_gpu_attr_groups[] = {
    &nv_c2c_pmu_gpu_format_group,
    &nv_c2c_pmu_gpu_events_group,
    &nv_c2c_pmu_cpumask_attr_group,
    &nv_c2c_pmu_identifier_attr_group,
    &nv_c2c_pmu_peer_attr_group,
    core::ptr::null_mut()
    };
// Attribute groups for C2C PMU connecting multiple SoCs
    static const struct attribute_group *nv_c2c_pmu_cpu_attr_groups[] = {
    &nv_c2c_pmu_format_group,
    &nv_c2c_pmu_cpu_events_group,
    &nv_c2c_pmu_cpumask_attr_group,
    &nv_c2c_pmu_identifier_attr_group,
    &nv_c2c_pmu_peer_attr_group,
    core::ptr::null_mut()
    };
// Attribute groups for C2C PMU connecting SoC and CXLMEM
    static const struct attribute_group *nv_c2c_pmu_cxlmem_attr_groups[] = {
    &nv_c2c_pmu_format_group,
    &nv_c2c_pmu_cxlmem_events_group,
    &nv_c2c_pmu_cpumask_attr_group,
    &nv_c2c_pmu_identifier_attr_group,
    &nv_c2c_pmu_peer_attr_group,
    core::ptr::null_mut()
    };
#[no_mangle]
unsafe extern "C" fn nv_c2c_pmu_online_cpu(cpu: c_uint, node: *mut hlist_node) -> c_int {
    static int nv_c2c_pmu_online_cpu(unsigned int cpu, struct hlist_node *node)
    {
    struct nv_c2c_pmu *c2c_pmu =
    hlist_entry_safe(node, struct nv_c2c_pmu, cpuhp_node);
    if (!cpumask_test_cpu(cpu, &c2c_pmu.associated_cpus))
    return 0;
// If the PMU is already managed, there is nothing to do
    if (!cpumask_empty(&c2c_pmu.active_cpu))
    return 0;
// Use this CPU for event counting
    cpumask_set_cpu(cpu, &c2c_pmu.active_cpu);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nv_c2c_pmu_cpu_teardown(cpu: c_uint, node: *mut hlist_node) -> c_int {
    static int nv_c2c_pmu_cpu_teardown(unsigned int cpu, struct hlist_node *node)
    {
    unsigned int dst;
    struct nv_c2c_pmu *c2c_pmu =
    hlist_entry_safe(node, struct nv_c2c_pmu, cpuhp_node);
// Nothing to do if this CPU doesn't own the PMU
    if (!cpumask_test_and_clear_cpu(cpu, &c2c_pmu.active_cpu))
    return 0;
// Choose a new CPU to migrate ownership of the PMU to
    dst = cpumask_any_and_but(&c2c_pmu.associated_cpus,
    cpu_online_mask, cpu);
    if (dst >= nr_cpu_ids)
    return 0;
// Use this CPU for event counting
    perf_pmu_migrate_context(&c2c_pmu.pmu, cpu, dst);
    cpumask_set_cpu(dst, &c2c_pmu.active_cpu);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nv_c2c_pmu_get_cpus(c2c_pmu: *mut nv_c2c_pmu) -> c_int {
    static int nv_c2c_pmu_get_cpus(struct nv_c2c_pmu *c2c_pmu)
    {
    let mut socket: c_int = c2c_pmu.socket, cpu;
    for_each_possible_cpu(cpu) {
    if (cpu_to_node(cpu) == socket)
    cpumask_set_cpu(cpu, &c2c_pmu.associated_cpus);
    }
    if (cpumask_empty(&c2c_pmu.associated_cpus)) {
    dev_dbg(c2c_pmu.dev,
    "No cpu associated with C2C PMU socket-%u\n", socket);
    return -ENODEV;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nv_c2c_pmu_init_socket(c2c_pmu: *mut nv_c2c_pmu) -> c_int {
    static int nv_c2c_pmu_init_socket(struct nv_c2c_pmu *c2c_pmu)
    {
    const char *uid_str;
    int ret, socket;
    uid_str = acpi_device_uid(c2c_pmu.acpi_dev);
    if (!uid_str) {
    dev_err(c2c_pmu.dev, "No ACPI device UID\n");
    return -ENODEV;
    }
    ret = kstrtou32(uid_str, 0, &socket);
    if (ret) {
    dev_err(c2c_pmu.dev, "Failed to parse ACPI device UID\n");
    return ret;
    }
    c2c_pmu.socket = socket;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nv_c2c_pmu_init_id(c2c_pmu: *mut nv_c2c_pmu) -> c_int {
    static int nv_c2c_pmu_init_id(struct nv_c2c_pmu *c2c_pmu)
    {
    char *name;
    name = devm_kasprintf(c2c_pmu.dev, GFP_KERNEL, c2c_pmu.data.name_fmt,
    c2c_pmu.socket);
    if (!name)
    return -ENOMEM;
    c2c_pmu.name = name;
    c2c_pmu.identifier = acpi_device_hid(c2c_pmu.acpi_dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nv_c2c_pmu_init_filter(c2c_pmu: *mut nv_c2c_pmu) -> c_int {
    static int nv_c2c_pmu_init_filter(struct nv_c2c_pmu *c2c_pmu)
    {
    let mut cpu_en: u32 = 0;
    struct device *dev = c2c_pmu.dev;
    const struct nv_c2c_pmu_data *data = c2c_pmu.data;
    if (data.c2c_type == C2C_TYPE_NVDLINK) {
    c2c_pmu.peer_type = C2C_PEER_TYPE_CXLMEM;
    c2c_pmu.peer_insts[0][0] = (1UL << data.nr_inst) - 1;
    c2c_pmu.nr_peer = C2C_NR_PEER_CXLMEM;
    c2c_pmu.filter_default = (1 << c2c_pmu.nr_peer) - 1;
    c2c_pmu.attr_groups = nv_c2c_pmu_cxlmem_attr_groups;
    return 0;
    }
    if (device_property_read_u32(dev, "cpu_en_mask", &cpu_en))
    dev_dbg(dev, "no cpu_en_mask property\n");
    if (cpu_en) {
    c2c_pmu.peer_type = C2C_PEER_TYPE_CPU;
// Fill peer_insts bitmap with instances connected to peer CPU.
    bitmap_from_arr32(c2c_pmu.peer_insts[0], &cpu_en, data.nr_inst);
    c2c_pmu.nr_peer = 1;
    c2c_pmu.attr_groups = nv_c2c_pmu_cpu_attr_groups;
    } else {
    u32 i;
    const char *props[C2C_NR_PEER_MAX] = {
    "gpu0_en_mask", "gpu1_en_mask"
    };
    for (i = 0; i < C2C_NR_PEER_MAX; i++) {
    let mut gpu_en: u32 = 0;
    if (device_property_read_u32(dev, props[i], &gpu_en))
    dev_dbg(dev, "no %s property\n", props[i]);
    if (gpu_en) {
// Fill peer_insts bitmap with instances connected to peer GPU.
    bitmap_from_arr32(c2c_pmu.peer_insts[i], &gpu_en,
    data.nr_inst);
    c2c_pmu.nr_peer++;
    }
    }
    if (c2c_pmu.nr_peer == 0) {
    dev_err(dev, "No GPU is enabled\n");
    return -EINVAL;
    }
    c2c_pmu.peer_type = C2C_PEER_TYPE_GPU;
    c2c_pmu.attr_groups = nv_c2c_pmu_gpu_attr_groups;
    }
    c2c_pmu.filter_default = (1 << c2c_pmu.nr_peer) - 1;
    return 0;
    }
    static void *nv_c2c_pmu_init_pmu(struct platform_device *pdev)
    {
    int ret;
    struct nv_c2c_pmu *c2c_pmu;
    struct acpi_device *acpi_dev;
    struct device *dev = &pdev.dev;
    acpi_dev = ACPI_COMPANION(dev);
    if (!acpi_dev)
    return ERR_PTR(-ENODEV);
    c2c_pmu = devm_kzalloc(dev, sizeof(*c2c_pmu), GFP_KERNEL);
    if (!c2c_pmu)
    return ERR_PTR(-ENOMEM);
    c2c_pmu.dev = dev;
    c2c_pmu.acpi_dev = acpi_dev;
    c2c_pmu.data = (const struct nv_c2c_pmu_data *)device_get_match_data(dev);
    if (!c2c_pmu.data)
    return ERR_PTR(-EINVAL);
    platform_set_drvdata(pdev, c2c_pmu);
    ret = nv_c2c_pmu_init_socket(c2c_pmu);
    if (ret)
    return ERR_PTR(ret);
    ret = nv_c2c_pmu_init_id(c2c_pmu);
    if (ret)
    return ERR_PTR(ret);
    ret = nv_c2c_pmu_init_filter(c2c_pmu);
    if (ret)
    return ERR_PTR(ret);
    return c2c_pmu;
    }
#[no_mangle]
unsafe extern "C" fn nv_c2c_pmu_init_mmio(c2c_pmu: *mut nv_c2c_pmu) -> c_int {
    static int nv_c2c_pmu_init_mmio(struct nv_c2c_pmu *c2c_pmu)
    {
    int i;
    struct device *dev = c2c_pmu.dev;
    struct platform_device *pdev = to_platform_device(dev);
    const struct nv_c2c_pmu_data *data = c2c_pmu.data;
// Map the address of all the instances.
    for (i = 0; i < data.nr_inst; i++) {
    c2c_pmu.base[i] = devm_platform_ioremap_resource(pdev, i);
    if (IS_ERR(c2c_pmu.base[i])) {
    dev_err(dev, "Failed map address for instance %d\n", i);
    return PTR_ERR(c2c_pmu.base[i]);
    }
    }
// Map broadcast address.
    c2c_pmu.base_broadcast = devm_platform_ioremap_resource(pdev,
    data.nr_inst);
    if (IS_ERR(c2c_pmu.base_broadcast)) {
    dev_err(dev, "Failed map broadcast address\n");
    return PTR_ERR(c2c_pmu.base_broadcast);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nv_c2c_pmu_register_pmu(c2c_pmu: *mut nv_c2c_pmu) -> c_int {
    static int nv_c2c_pmu_register_pmu(struct nv_c2c_pmu *c2c_pmu)
    {
    int ret;
    ret = cpuhp_state_add_instance(nv_c2c_pmu_cpuhp_state,
    &c2c_pmu.cpuhp_node);
    if (ret) {
    dev_err(c2c_pmu.dev, "Error %d registering hotplug\n", ret);
    return ret;
    }
    c2c_pmu.pmu = (struct pmu) {
    .parent		= c2c_pmu.dev,
    .task_ctx_nr	= perf_invalid_context,
    .pmu_enable	= nv_c2c_pmu_enable,
    .pmu_disable	= nv_c2c_pmu_disable,
    .event_init	= nv_c2c_pmu_event_init,
    .add		= nv_c2c_pmu_add,
    .del		= nv_c2c_pmu_del,
    .start		= nv_c2c_pmu_start,
    .stop		= nv_c2c_pmu_stop,
    .read		= nv_c2c_pmu_read,
    .attr_groups	= c2c_pmu.attr_groups,
    .capabilities	= PERF_PMU_CAP_NO_EXCLUDE |
    PERF_PMU_CAP_NO_INTERRUPT,
    };
    ret = perf_pmu_register(&c2c_pmu.pmu, c2c_pmu.name, -1);
    if (ret) {
    dev_err(c2c_pmu.dev, "Failed to register C2C PMU: %d\n", ret);
    cpuhp_state_remove_instance(nv_c2c_pmu_cpuhp_state,
    &c2c_pmu.cpuhp_node);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nv_c2c_pmu_probe(pdev: *mut platform_device) -> c_int {
    static int nv_c2c_pmu_probe(struct platform_device *pdev)
    {
    int ret;
    struct nv_c2c_pmu *c2c_pmu;
    c2c_pmu = nv_c2c_pmu_init_pmu(pdev);
    if (IS_ERR(c2c_pmu))
    return PTR_ERR(c2c_pmu);
    ret = nv_c2c_pmu_init_mmio(c2c_pmu);
    if (ret)
    return ret;
    ret = nv_c2c_pmu_get_cpus(c2c_pmu);
    if (ret)
    return ret;
    ret = nv_c2c_pmu_register_pmu(c2c_pmu);
    if (ret)
    return ret;
    dev_dbg(c2c_pmu.dev, "Registered %s PMU\n", c2c_pmu.name);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nv_c2c_pmu_device_remove(pdev: *mut platform_device) {
    static void nv_c2c_pmu_device_remove(struct platform_device *pdev)
    {
    struct nv_c2c_pmu *c2c_pmu = platform_get_drvdata(pdev);
    perf_pmu_unregister(&c2c_pmu.pmu);
    cpuhp_state_remove_instance(nv_c2c_pmu_cpuhp_state, &c2c_pmu.cpuhp_node);
    }
    static const struct acpi_device_id nv_c2c_pmu_acpi_match[] = {
    { "NVDA2023", (kernel_ulong_t)&nv_c2c_pmu_data[C2C_TYPE_NVLINK] },
    { "NVDA2022", (kernel_ulong_t)&nv_c2c_pmu_data[C2C_TYPE_NVCLINK] },
    { "NVDA2020", (kernel_ulong_t)&nv_c2c_pmu_data[C2C_TYPE_NVDLINK] },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, nv_c2c_pmu_acpi_match);
    static struct platform_driver nv_c2c_pmu_driver = {
    .driver = {
    .name = "nvidia-t410-c2c-pmu",
    .acpi_match_table = nv_c2c_pmu_acpi_match,
    .suppress_bind_attrs = true,
    },
    .probe = nv_c2c_pmu_probe,
    .remove = nv_c2c_pmu_device_remove,
    };
#[no_mangle]
unsafe extern "C" fn nv_c2c_pmu_init() -> int __init {
    static int __init nv_c2c_pmu_init(void)
    {
    int ret;
    ret = cpuhp_setup_state_multi(CPUHP_AP_ONLINE_DYN,
    "perf/nvidia/c2c:online",
    nv_c2c_pmu_online_cpu,
    nv_c2c_pmu_cpu_teardown);
    if (ret < 0)
    return ret;
    nv_c2c_pmu_cpuhp_state = ret;
    return platform_driver_register(&nv_c2c_pmu_driver);
    }
#[no_mangle]
unsafe extern "C" fn nv_c2c_pmu_exit() -> void __exit {
    static void __exit nv_c2c_pmu_exit(void)
    {
    platform_driver_unregister(&nv_c2c_pmu_driver);
    cpuhp_remove_multi_state(nv_c2c_pmu_cpuhp_state);
    }
    module_init(nv_c2c_pmu_init);
    module_exit(nv_c2c_pmu_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("NVIDIA Tegra410 C2C PMU driver");
    MODULE_AUTHOR("Besar Wicaksono <bwicaksono@nvidia.com>");
