//! Automatically rewritten from C to Rust
//! Source: drivers/perf/nvidia_t410_cmem_latency_pmu.c
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
// NVIDIA Tegra410 CPU Memory (CMEM) Latency PMU driver.
//
// Copyright (c) 2026, NVIDIA CORPORATION & AFFILIATES. All rights reserved.
//

pub const NUM_INSTANCES: c_int = 14;
// Register offsets.
pub const CMEM_LAT_CG_CTRL: c_uint = 0x800;
pub const CMEM_LAT_CTRL: c_uint = 0x808;
pub const CMEM_LAT_STATUS: c_uint = 0x810;
pub const CMEM_LAT_CYCLE_CNTR: c_uint = 0x818;
pub const CMEM_LAT_MC0_REQ_CNTR: c_uint = 0x820;
pub const CMEM_LAT_MC0_AOR_CNTR: c_uint = 0x830;
pub const CMEM_LAT_MC1_REQ_CNTR: c_uint = 0x838;
pub const CMEM_LAT_MC1_AOR_CNTR: c_uint = 0x848;
pub const CMEM_LAT_MC2_REQ_CNTR: c_uint = 0x850;
pub const CMEM_LAT_MC2_AOR_CNTR: c_uint = 0x860;
// CMEM_LAT_CTRL values.
pub const CMEM_LAT_CTRL_DISABLE: c_uint = 0x0ULL;
pub const CMEM_LAT_CTRL_ENABLE: c_uint = 0x1ULL;
pub const CMEM_LAT_CTRL_CLR: c_uint = 0x2ULL;
// CMEM_LAT_CG_CTRL values.
pub const CMEM_LAT_CG_CTRL_DISABLE: c_uint = 0x0ULL;
pub const CMEM_LAT_CG_CTRL_ENABLE: c_uint = 0x1ULL;
// CMEM_LAT_STATUS register field.

// Events.
pub const CMEM_LAT_EVENT_CYCLES: c_uint = 0x0;
pub const CMEM_LAT_EVENT_REQ: c_uint = 0x1;
pub const CMEM_LAT_EVENT_AOR: c_uint = 0x2;
pub const CMEM_LAT_NUM_EVENTS: c_uint = 0x3;
pub const CMEM_LAT_MASK_EVENT: c_uint = 0x3;
pub const CMEM_LAT_MAX_ACTIVE_EVENTS: c_int = 32;
pub const CMEM_LAT_ACTIVE_CPU_MASK: c_uint = 0x0;
pub const CMEM_LAT_ASSOCIATED_CPU_MASK: c_uint = 0x1;
    static unsigned long cmem_lat_pmu_cpuhp_state;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmem_lat_pmu_hw_events {
    pub events: [*mut perf_event; CMEM_LAT_MAX_ACTIVE_EVENTS],
    pub CMEM_LAT_MAX_ACTIVE_EVENTS): DECLARE_BITMAP(used_ctrs,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmem_lat_pmu {
    pub pmu: pmu,
    pub dev: *mut device,
    pub name: *const c_char,
    pub identifier: *const c_char,
    pub base_broadcast: *mut void __iomem,
    pub base: [*mut void __iomem; NUM_INSTANCES],
    pub associated_cpus: cpumask_t,
    pub active_cpu: cpumask_t,
    pub node: hlist_node,
    pub hw_events: cmem_lat_pmu_hw_events,
}

    container_of(p, struct cmem_lat_pmu, pmu)
// Get event type from perf_event.
#[no_mangle]
pub unsafe extern "C" fn get_event_type(event: *mut perf_event) -> u32 {
    static inline u32 get_event_type(struct perf_event *event)
    {
    return (event.attr.config) & CMEM_LAT_MASK_EVENT;
    }
// PMU operations.
    static int cmem_lat_pmu_get_event_idx(struct cmem_lat_pmu_hw_events *hw_events,
    struct perf_event *event)
    {
    unsigned int idx;
    idx = find_first_zero_bit(hw_events.used_ctrs, CMEM_LAT_MAX_ACTIVE_EVENTS);
    if (idx >= CMEM_LAT_MAX_ACTIVE_EVENTS)
    return -EAGAIN;
    set_bit(idx, hw_events.used_ctrs);
    return idx;
    }
    static bool cmem_lat_pmu_validate_event(struct pmu *pmu,
    struct cmem_lat_pmu_hw_events *hw_events,
    struct perf_event *event)
    {
    int ret;
    if (is_software_event(event))
    return true;
// Reject groups spanning multiple HW PMUs.
    if (event.pmu != pmu)
    return false;
    ret = cmem_lat_pmu_get_event_idx(hw_events, event);
    if (ret < 0)
    return false;
    return true;
    }
// Make sure the group of events can be scheduled at once on the PMU.
#[no_mangle]
unsafe extern "C" fn cmem_lat_pmu_validate_group(event: *mut perf_event) -> bool {
    static bool cmem_lat_pmu_validate_group(struct perf_event *event)
    {
    struct perf_event *sibling, *leader = event.group_leader;
    struct cmem_lat_pmu_hw_events fake_hw_events;
    if (event.group_leader == event)
    return true;
    memset(&fake_hw_events, 0, sizeof(fake_hw_events));
    if (!cmem_lat_pmu_validate_event(event.pmu, &fake_hw_events, leader))
    return false;
    for_each_sibling_event(sibling, leader) {
    if (!cmem_lat_pmu_validate_event(event.pmu, &fake_hw_events, sibling))
    return false;
    }
    return cmem_lat_pmu_validate_event(event.pmu, &fake_hw_events, event);
    }
#[no_mangle]
unsafe extern "C" fn cmem_lat_pmu_event_init(event: *mut perf_event) -> c_int {
    static int cmem_lat_pmu_event_init(struct perf_event *event)
    {
    struct cmem_lat_pmu *cmem_lat_pmu = to_cmem_lat_pmu(event.pmu);
    struct hw_perf_event *hwc = &event.hw;
    let mut event_type: u32 = get_event_type(event);
    if (event.attr.type != event.pmu.type ||
    event_type >= CMEM_LAT_NUM_EVENTS)
    return -ENOENT;
//
// Sampling, per-process mode, and per-task counters are not supported
// since this PMU is shared across all CPUs.
//
    if (is_sampling_event(event) || event.attach_state & PERF_ATTACH_TASK) {
    dev_dbg(cmem_lat_pmu.pmu.dev,
    "Can't support sampling and per-process mode\n");
    return -EOPNOTSUPP;
    }
    if (event.cpu < 0) {
    dev_dbg(cmem_lat_pmu.pmu.dev, "Can't support per-task counters\n");
    return -EINVAL;
    }
//
// Make sure the CPU assignment is on one of the CPUs associated with
// this PMU.
//
    if (!cpumask_test_cpu(event.cpu, &cmem_lat_pmu.associated_cpus)) {
    dev_dbg(cmem_lat_pmu.pmu.dev,
    "Requested cpu is not associated with the PMU\n");
    return -EINVAL;
    }
// Enforce the current active CPU to handle the events in this PMU.
    event.cpu = cpumask_first(&cmem_lat_pmu.active_cpu);
    if (event.cpu >= nr_cpu_ids)
    return -EINVAL;
    if (!cmem_lat_pmu_validate_group(event))
    return -EINVAL;
    hwc.idx = -1;
    hwc.config = event_type;
    return 0;
    }
    static u64 cmem_lat_pmu_read_status(struct cmem_lat_pmu *cmem_lat_pmu,
    unsigned int inst)
    {
    return readq(cmem_lat_pmu.base[inst] + CMEM_LAT_STATUS);
    }
#[no_mangle]
unsafe extern "C" fn cmem_lat_pmu_read_cycle_counter(event: *mut perf_event) -> u64 {
    static u64 cmem_lat_pmu_read_cycle_counter(struct perf_event *event)
    {
    let mut instance: c_uint = 0;
    u64 status;
    struct cmem_lat_pmu *cmem_lat_pmu = to_cmem_lat_pmu(event.pmu);
    struct device *dev = cmem_lat_pmu.dev;
//
// Use the reading from first instance since all instances are
// identical.
//
    status = cmem_lat_pmu_read_status(cmem_lat_pmu, instance);
    if (status & CMEM_LAT_STATUS_CYCLE_OVF)
    dev_warn(dev, "Cycle counter overflow\n");
    return readq(cmem_lat_pmu.base[instance] + CMEM_LAT_CYCLE_CNTR);
    }
#[no_mangle]
unsafe extern "C" fn cmem_lat_pmu_read_req_counter(event: *mut perf_event) -> u64 {
    static u64 cmem_lat_pmu_read_req_counter(struct perf_event *event)
    {
    unsigned int i;
    u64 status, val = 0;
    struct cmem_lat_pmu *cmem_lat_pmu = to_cmem_lat_pmu(event.pmu);
    struct device *dev = cmem_lat_pmu.dev;
// Sum up the counts from all instances.
    for (i = 0; i < NUM_INSTANCES; i++) {
    status = cmem_lat_pmu_read_status(cmem_lat_pmu, i);
    if (status & CMEM_LAT_STATUS_MC0_REQ_OVF)
    dev_warn(dev, "MC0 request counter overflow\n");
    if (status & CMEM_LAT_STATUS_MC1_REQ_OVF)
    dev_warn(dev, "MC1 request counter overflow\n");
    if (status & CMEM_LAT_STATUS_MC2_REQ_OVF)
    dev_warn(dev, "MC2 request counter overflow\n");
    val += readq(cmem_lat_pmu.base[i] + CMEM_LAT_MC0_REQ_CNTR);
    val += readq(cmem_lat_pmu.base[i] + CMEM_LAT_MC1_REQ_CNTR);
    val += readq(cmem_lat_pmu.base[i] + CMEM_LAT_MC2_REQ_CNTR);
    }
    return val;
    }
#[no_mangle]
unsafe extern "C" fn cmem_lat_pmu_read_aor_counter(event: *mut perf_event) -> u64 {
    static u64 cmem_lat_pmu_read_aor_counter(struct perf_event *event)
    {
    unsigned int i;
    u64 status, val = 0;
    struct cmem_lat_pmu *cmem_lat_pmu = to_cmem_lat_pmu(event.pmu);
    struct device *dev = cmem_lat_pmu.dev;
// Sum up the counts from all instances.
    for (i = 0; i < NUM_INSTANCES; i++) {
    status = cmem_lat_pmu_read_status(cmem_lat_pmu, i);
    if (status & CMEM_LAT_STATUS_MC0_AOR_OVF)
    dev_warn(dev, "MC0 AOR counter overflow\n");
    if (status & CMEM_LAT_STATUS_MC1_AOR_OVF)
    dev_warn(dev, "MC1 AOR counter overflow\n");
    if (status & CMEM_LAT_STATUS_MC2_AOR_OVF)
    dev_warn(dev, "MC2 AOR counter overflow\n");
    val += readq(cmem_lat_pmu.base[i] + CMEM_LAT_MC0_AOR_CNTR);
    val += readq(cmem_lat_pmu.base[i] + CMEM_LAT_MC1_AOR_CNTR);
    val += readq(cmem_lat_pmu.base[i] + CMEM_LAT_MC2_AOR_CNTR);
    }
    return val;
    }
    static u64 (*read_counter_fn[CMEM_LAT_NUM_EVENTS])(struct perf_event *) = {
    [CMEM_LAT_EVENT_CYCLES] = cmem_lat_pmu_read_cycle_counter,
    [CMEM_LAT_EVENT_REQ] = cmem_lat_pmu_read_req_counter,
    [CMEM_LAT_EVENT_AOR] = cmem_lat_pmu_read_aor_counter,
    };
#[no_mangle]
unsafe extern "C" fn cmem_lat_pmu_event_update(event: *mut perf_event) {
    static void cmem_lat_pmu_event_update(struct perf_event *event)
    {
    u32 event_type;
    u64 prev, now;
    struct hw_perf_event *hwc = &event.hw;
    if (hwc.state & PERF_HES_STOPPED)
    return;
    event_type = hwc.config;
    do {
    prev = local64_read(&hwc.prev_count);
    now = read_counter_fn[event_type](event);
    } while (local64_cmpxchg(&hwc.prev_count, prev, now) != prev);
    local64_add(now - prev, &event.count);
    hwc.state |= PERF_HES_UPTODATE;
    }
#[no_mangle]
unsafe extern "C" fn cmem_lat_pmu_start(event: *mut perf_event, pmu_flags: c_int) {
    static void cmem_lat_pmu_start(struct perf_event *event, int pmu_flags)
    {
    event.hw.state = 0;
    }
#[no_mangle]
unsafe extern "C" fn cmem_lat_pmu_stop(event: *mut perf_event, pmu_flags: c_int) {
    static void cmem_lat_pmu_stop(struct perf_event *event, int pmu_flags)
    {
    event.hw.state |= PERF_HES_STOPPED;
    }
#[no_mangle]
unsafe extern "C" fn cmem_lat_pmu_add(event: *mut perf_event, flags: c_int) -> c_int {
    static int cmem_lat_pmu_add(struct perf_event *event, int flags)
    {
    struct cmem_lat_pmu *cmem_lat_pmu = to_cmem_lat_pmu(event.pmu);
    struct cmem_lat_pmu_hw_events *hw_events = &cmem_lat_pmu.hw_events;
    struct hw_perf_event *hwc = &event.hw;
    int idx;
    if (WARN_ON_ONCE(!cpumask_test_cpu(smp_processor_id(),
    &cmem_lat_pmu.associated_cpus)))
    return -ENOENT;
    idx = cmem_lat_pmu_get_event_idx(hw_events, event);
    if (idx < 0)
    return idx;
    hw_events.events[idx] = event;
    hwc.idx = idx;
    hwc.state = PERF_HES_STOPPED | PERF_HES_UPTODATE;
    if (flags & PERF_EF_START)
    cmem_lat_pmu_start(event, PERF_EF_RELOAD);
// Propagate changes to the userspace mapping.
    perf_event_update_userpage(event);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cmem_lat_pmu_del(event: *mut perf_event, flags: c_int) {
    static void cmem_lat_pmu_del(struct perf_event *event, int flags)
    {
    struct cmem_lat_pmu *cmem_lat_pmu = to_cmem_lat_pmu(event.pmu);
    struct cmem_lat_pmu_hw_events *hw_events = &cmem_lat_pmu.hw_events;
    struct hw_perf_event *hwc = &event.hw;
    let mut idx: c_int = hwc.idx;
    cmem_lat_pmu_stop(event, PERF_EF_UPDATE);
    hw_events.events[idx] = core::ptr::null_mut();
    clear_bit(idx, hw_events.used_ctrs);
    perf_event_update_userpage(event);
    }
#[no_mangle]
unsafe extern "C" fn cmem_lat_pmu_read(event: *mut perf_event) {
    static void cmem_lat_pmu_read(struct perf_event *event)
    {
    cmem_lat_pmu_event_update(event);
    }
    static inline void cmem_lat_pmu_cg_ctrl(struct cmem_lat_pmu *cmem_lat_pmu,
    u64 val)
    {
    writeq(val, cmem_lat_pmu.base_broadcast + CMEM_LAT_CG_CTRL);
    }
#[no_mangle]
pub unsafe extern "C" fn cmem_lat_pmu_ctrl(cmem_lat_pmu: *mut cmem_lat_pmu, val: u64) {
    static inline void cmem_lat_pmu_ctrl(struct cmem_lat_pmu *cmem_lat_pmu, u64 val)
    {
    writeq(val, cmem_lat_pmu.base_broadcast + CMEM_LAT_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn cmem_lat_pmu_enable(pmu: *mut pmu) {
    static void cmem_lat_pmu_enable(struct pmu *pmu)
    {
    bool disabled;
    struct cmem_lat_pmu *cmem_lat_pmu = to_cmem_lat_pmu(pmu);
    disabled = bitmap_empty(cmem_lat_pmu.hw_events.used_ctrs,
    CMEM_LAT_MAX_ACTIVE_EVENTS);
    if (disabled)
    return;
// Enable all the counters.
    cmem_lat_pmu_cg_ctrl(cmem_lat_pmu, CMEM_LAT_CG_CTRL_ENABLE);
    cmem_lat_pmu_ctrl(cmem_lat_pmu, CMEM_LAT_CTRL_ENABLE);
    }
#[no_mangle]
unsafe extern "C" fn cmem_lat_pmu_disable(pmu: *mut pmu) {
    static void cmem_lat_pmu_disable(struct pmu *pmu)
    {
    int idx;
    struct cmem_lat_pmu *cmem_lat_pmu = to_cmem_lat_pmu(pmu);
// Disable all the counters.
    cmem_lat_pmu_ctrl(cmem_lat_pmu, CMEM_LAT_CTRL_DISABLE);
//
// The counters will start from 0 again on restart.
// Update the events immediately to avoid losing the counts.
//
    for_each_set_bit(idx, cmem_lat_pmu.hw_events.used_ctrs,
    CMEM_LAT_MAX_ACTIVE_EVENTS) {
    struct perf_event *event = cmem_lat_pmu.hw_events.events[idx];
    if (!event)
    continue;
    cmem_lat_pmu_event_update(event);
    local64_set(&event.hw.prev_count, 0ULL);
    }
    cmem_lat_pmu_ctrl(cmem_lat_pmu, CMEM_LAT_CTRL_CLR);
    cmem_lat_pmu_cg_ctrl(cmem_lat_pmu, CMEM_LAT_CG_CTRL_DISABLE);
    }
// PMU identifier attribute.
    static ssize_t cmem_lat_pmu_identifier_show(struct device *dev,
    struct device_attribute *attr,
    char *page)
    {
    struct cmem_lat_pmu *cmem_lat_pmu = to_cmem_lat_pmu(dev_get_drvdata(dev));
    return sysfs_emit(page, "%s\n", cmem_lat_pmu.identifier);
    }
    static struct device_attribute cmem_lat_pmu_identifier_attr =
    __ATTR(identifier, 0444, cmem_lat_pmu_identifier_show, core::ptr::null_mut());
    static struct attribute *cmem_lat_pmu_identifier_attrs[] = {
    &cmem_lat_pmu_identifier_attr.attr,
    core::ptr::null_mut()
    };
    static struct attribute_group cmem_lat_pmu_identifier_attr_group = {
    .attrs = cmem_lat_pmu_identifier_attrs,
    };
// Format attributes.

    (&((struct dev_ext_attribute[]){				\
    {							\
    .attr = __ATTR(_name, 0444, _func, core::ptr::null_mut()),	\
    .var = (void *)_config				\
    }							\
    })[0].attr.attr)
    static struct attribute *cmem_lat_pmu_formats[] = {
    NV_PMU_EXT_ATTR(event, device_show_string, "config:0-1"),
    core::ptr::null_mut()
    };
    static const struct attribute_group cmem_lat_pmu_format_group = {
    .name = "format",
    .attrs = cmem_lat_pmu_formats,
    };
// Event attributes.
    static ssize_t cmem_lat_pmu_sysfs_event_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct perf_pmu_events_attr *pmu_attr;
    pmu_attr = container_of(attr, typeof(*pmu_attr), attr);
    return sysfs_emit(buf, "event=0x%llx\n", pmu_attr.id);
    }

    PMU_EVENT_ATTR_ID(_name, cmem_lat_pmu_sysfs_event_show, _config)
    static struct attribute *cmem_lat_pmu_events[] = {
    NV_PMU_EVENT_ATTR(cycles, CMEM_LAT_EVENT_CYCLES),
    NV_PMU_EVENT_ATTR(rd_req, CMEM_LAT_EVENT_REQ),
    NV_PMU_EVENT_ATTR(rd_cum_outs, CMEM_LAT_EVENT_AOR),
    core::ptr::null_mut()
    };
    static const struct attribute_group cmem_lat_pmu_events_group = {
    .name = "events",
    .attrs = cmem_lat_pmu_events,
    };
// Cpumask attributes.
    static ssize_t cmem_lat_pmu_cpumask_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct pmu *pmu = dev_get_drvdata(dev);
    struct cmem_lat_pmu *cmem_lat_pmu = to_cmem_lat_pmu(pmu);
    struct dev_ext_attribute *eattr =
    container_of(attr, struct dev_ext_attribute, attr);
    let mut mask_id: c_ulong = (unsigned long)eattr.var;
    const cpumask_t *cpumask;
    switch (mask_id) {
    case CMEM_LAT_ACTIVE_CPU_MASK:
    cpumask = &cmem_lat_pmu.active_cpu;
    break;
    case CMEM_LAT_ASSOCIATED_CPU_MASK:
    cpumask = &cmem_lat_pmu.associated_cpus;
    break;
    default:
    return 0;
    }
    return sysfs_emit(buf, "%*pbl\n", cpumask_pr_args(cpumask));
    }

    NV_PMU_EXT_ATTR(_name, cmem_lat_pmu_cpumask_show,	\
    (unsigned long)_config)
    static struct attribute *cmem_lat_pmu_cpumask_attrs[] = {
    NV_PMU_CPUMASK_ATTR(cpumask, CMEM_LAT_ACTIVE_CPU_MASK),
    NV_PMU_CPUMASK_ATTR(associated_cpus, CMEM_LAT_ASSOCIATED_CPU_MASK),
    core::ptr::null_mut()
    };
    static const struct attribute_group cmem_lat_pmu_cpumask_attr_group = {
    .attrs = cmem_lat_pmu_cpumask_attrs,
    };
// Per PMU device attribute groups.
    static const struct attribute_group *cmem_lat_pmu_attr_groups[] = {
    &cmem_lat_pmu_identifier_attr_group,
    &cmem_lat_pmu_format_group,
    &cmem_lat_pmu_events_group,
    &cmem_lat_pmu_cpumask_attr_group,
    core::ptr::null_mut()
    };
#[no_mangle]
unsafe extern "C" fn cmem_lat_pmu_cpu_online(cpu: c_uint, node: *mut hlist_node) -> c_int {
    static int cmem_lat_pmu_cpu_online(unsigned int cpu, struct hlist_node *node)
    {
    struct cmem_lat_pmu *cmem_lat_pmu =
    hlist_entry_safe(node, struct cmem_lat_pmu, node);
    if (!cpumask_test_cpu(cpu, &cmem_lat_pmu.associated_cpus))
    return 0;
// If the PMU is already managed, there is nothing to do
    if (!cpumask_empty(&cmem_lat_pmu.active_cpu))
    return 0;
// Use this CPU for event counting
    cpumask_set_cpu(cpu, &cmem_lat_pmu.active_cpu);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cmem_lat_pmu_cpu_teardown(cpu: c_uint, node: *mut hlist_node) -> c_int {
    static int cmem_lat_pmu_cpu_teardown(unsigned int cpu, struct hlist_node *node)
    {
    unsigned int dst;
    struct cmem_lat_pmu *cmem_lat_pmu =
    hlist_entry_safe(node, struct cmem_lat_pmu, node);
// Nothing to do if this CPU doesn't own the PMU
    if (!cpumask_test_and_clear_cpu(cpu, &cmem_lat_pmu.active_cpu))
    return 0;
// Choose a new CPU to migrate ownership of the PMU to
    dst = cpumask_any_and_but(&cmem_lat_pmu.associated_cpus,
    cpu_online_mask, cpu);
    if (dst >= nr_cpu_ids)
    return 0;
// Use this CPU for event counting
    perf_pmu_migrate_context(&cmem_lat_pmu.pmu, cpu, dst);
    cpumask_set_cpu(dst, &cmem_lat_pmu.active_cpu);
    return 0;
    }
    static int cmem_lat_pmu_get_cpus(struct cmem_lat_pmu *cmem_lat_pmu,
    unsigned int socket)
    {
    int cpu;
    for_each_possible_cpu(cpu) {
    if (cpu_to_node(cpu) == socket)
    cpumask_set_cpu(cpu, &cmem_lat_pmu.associated_cpus);
    }
    if (cpumask_empty(&cmem_lat_pmu.associated_cpus)) {
    dev_dbg(cmem_lat_pmu.dev,
    "No cpu associated with PMU socket-%u\n", socket);
    return -ENODEV;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cmem_lat_pmu_probe(pdev: *mut platform_device) -> c_int {
    static int cmem_lat_pmu_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct acpi_device *acpi_dev;
    struct cmem_lat_pmu *cmem_lat_pmu;
    char *name, *uid_str;
    int ret, i;
    u32 socket;
    acpi_dev = ACPI_COMPANION(dev);
    if (!acpi_dev)
    return -ENODEV;
    uid_str = acpi_device_uid(acpi_dev);
    if (!uid_str)
    return -ENODEV;
    ret = kstrtou32(uid_str, 0, &socket);
    if (ret)
    return ret;
    cmem_lat_pmu = devm_kzalloc(dev, sizeof(*cmem_lat_pmu), GFP_KERNEL);
    name = devm_kasprintf(dev, GFP_KERNEL, "nvidia_cmem_latency_pmu_%u", socket);
    if (!cmem_lat_pmu || !name)
    return -ENOMEM;
    cmem_lat_pmu.dev = dev;
    cmem_lat_pmu.name = name;
    cmem_lat_pmu.identifier = acpi_device_hid(acpi_dev);
    platform_set_drvdata(pdev, cmem_lat_pmu);
    cmem_lat_pmu.pmu = (struct pmu) {
    .parent		= &pdev.dev,
    .task_ctx_nr	= perf_invalid_context,
    .pmu_enable	= cmem_lat_pmu_enable,
    .pmu_disable	= cmem_lat_pmu_disable,
    .event_init	= cmem_lat_pmu_event_init,
    .add		= cmem_lat_pmu_add,
    .del		= cmem_lat_pmu_del,
    .start		= cmem_lat_pmu_start,
    .stop		= cmem_lat_pmu_stop,
    .read		= cmem_lat_pmu_read,
    .attr_groups	= cmem_lat_pmu_attr_groups,
    .capabilities	= PERF_PMU_CAP_NO_EXCLUDE |
    PERF_PMU_CAP_NO_INTERRUPT,
    };
// Map the address of all the instances.
    for (i = 0; i < NUM_INSTANCES; i++) {
    cmem_lat_pmu.base[i] = devm_platform_ioremap_resource(pdev, i);
    if (IS_ERR(cmem_lat_pmu.base[i])) {
    dev_err(dev, "Failed map address for instance %d\n", i);
    return PTR_ERR(cmem_lat_pmu.base[i]);
    }
    }
// Map broadcast address.
    cmem_lat_pmu.base_broadcast = devm_platform_ioremap_resource(pdev,
    NUM_INSTANCES);
    if (IS_ERR(cmem_lat_pmu.base_broadcast)) {
    dev_err(dev, "Failed map broadcast address\n");
    return PTR_ERR(cmem_lat_pmu.base_broadcast);
    }
    ret = cmem_lat_pmu_get_cpus(cmem_lat_pmu, socket);
    if (ret)
    return ret;
    ret = cpuhp_state_add_instance(cmem_lat_pmu_cpuhp_state,
    &cmem_lat_pmu.node);
    if (ret) {
    dev_err(&pdev.dev, "Error %d registering hotplug\n", ret);
    return ret;
    }
    cmem_lat_pmu_cg_ctrl(cmem_lat_pmu, CMEM_LAT_CG_CTRL_ENABLE);
    cmem_lat_pmu_ctrl(cmem_lat_pmu, CMEM_LAT_CTRL_CLR);
    cmem_lat_pmu_cg_ctrl(cmem_lat_pmu, CMEM_LAT_CG_CTRL_DISABLE);
    ret = perf_pmu_register(&cmem_lat_pmu.pmu, name, -1);
    if (ret) {
    dev_err(&pdev.dev, "Failed to register PMU: %d\n", ret);
    cpuhp_state_remove_instance(cmem_lat_pmu_cpuhp_state,
    &cmem_lat_pmu.node);
    return ret;
    }
    dev_dbg(&pdev.dev, "Registered %s PMU\n", name);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cmem_lat_pmu_device_remove(pdev: *mut platform_device) {
    static void cmem_lat_pmu_device_remove(struct platform_device *pdev)
    {
    struct cmem_lat_pmu *cmem_lat_pmu = platform_get_drvdata(pdev);
    perf_pmu_unregister(&cmem_lat_pmu.pmu);
    cpuhp_state_remove_instance(cmem_lat_pmu_cpuhp_state,
    &cmem_lat_pmu.node);
    }
    static const struct acpi_device_id cmem_lat_pmu_acpi_match[] = {
    { "NVDA2021" },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, cmem_lat_pmu_acpi_match);
    static struct platform_driver cmem_lat_pmu_driver = {
    .driver = {
    .name = "nvidia-t410-cmem-latency-pmu",
    .acpi_match_table = ACPI_PTR(cmem_lat_pmu_acpi_match),
    .suppress_bind_attrs = true,
    },
    .probe = cmem_lat_pmu_probe,
    .remove = cmem_lat_pmu_device_remove,
    };
#[no_mangle]
unsafe extern "C" fn cmem_lat_pmu_init() -> int __init {
    static int __init cmem_lat_pmu_init(void)
    {
    int ret;
    ret = cpuhp_setup_state_multi(CPUHP_AP_ONLINE_DYN,
    "perf/nvidia/cmem_latency:online",
    cmem_lat_pmu_cpu_online,
    cmem_lat_pmu_cpu_teardown);
    if (ret < 0)
    return ret;
    cmem_lat_pmu_cpuhp_state = ret;
    return platform_driver_register(&cmem_lat_pmu_driver);
    }
#[no_mangle]
unsafe extern "C" fn cmem_lat_pmu_exit() -> void __exit {
    static void __exit cmem_lat_pmu_exit(void)
    {
    platform_driver_unregister(&cmem_lat_pmu_driver);
    cpuhp_remove_multi_state(cmem_lat_pmu_cpuhp_state);
    }
    module_init(cmem_lat_pmu_init);
    module_exit(cmem_lat_pmu_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("NVIDIA Tegra410 CPU Memory Latency PMU driver");
    MODULE_AUTHOR("Besar Wicaksono <bwicaksono@nvidia.com>");
