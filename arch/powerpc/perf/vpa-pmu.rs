//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/perf/vpa-pmu.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Performance monitoring support for Virtual Processor Area(VPA) based counters
//
// Copyright (C) 2024 IBM Corporation
//

    static ssize_t vpa_pmu_events_sysfs_show(struct device *dev,
    struct device_attribute *attr, char *page)
    {
    struct perf_pmu_events_attr *pmu_attr;
    pmu_attr = container_of(attr, struct perf_pmu_events_attr, attr);
    return sysfs_emit(page, "event=0x%02llx\n", pmu_attr.id);
    }

    PMU_EVENT_ATTR(_name, VPA_PMU_EVENT_VAR(_id), _id,	\
    vpa_pmu_events_sysfs_show)
    EVENT(L1_TO_L2_CS_LAT,	0x1);
    EVENT(L2_TO_L1_CS_LAT,	0x2);
    EVENT(L2_RUNTIME_AGG,	0x3);
    VPA_PMU_EVENT_ATTR(l1_to_l2_lat,  L1_TO_L2_CS_LAT);
    VPA_PMU_EVENT_ATTR(l2_to_l1_lat,  L2_TO_L1_CS_LAT);
    VPA_PMU_EVENT_ATTR(l2_runtime_agg, L2_RUNTIME_AGG);
    static struct attribute *vpa_pmu_events_attr[] = {
    VPA_PMU_EVENT_PTR(L1_TO_L2_CS_LAT),
    VPA_PMU_EVENT_PTR(L2_TO_L1_CS_LAT),
    VPA_PMU_EVENT_PTR(L2_RUNTIME_AGG),
    core::ptr::null_mut()
    };
    static const struct attribute_group vpa_pmu_events_group = {
    .name = "events",
    .attrs = vpa_pmu_events_attr,
    };
    PMU_FORMAT_ATTR(event, "config:0-31");
    static struct attribute *vpa_pmu_format_attr[] = {
    &format_attr_event.attr,
    core::ptr::null_mut(),
    };
    static struct attribute_group vpa_pmu_format_group = {
    .name = "format",
    .attrs = vpa_pmu_format_attr,
    };
    static const struct attribute_group *vpa_pmu_attr_groups[] = {
    &vpa_pmu_events_group,
    &vpa_pmu_format_group,
    core::ptr::null_mut()
    };
#[no_mangle]
unsafe extern "C" fn vpa_pmu_event_init(event: *mut perf_event) -> c_int {
    static int vpa_pmu_event_init(struct perf_event *event)
    {
    if (event.attr.type != event.pmu.type)
    return -ENOENT;
// it does not support event sampling mode
    if (is_sampling_event(event))
    return -EOPNOTSUPP;
// no branch sampling
    if (has_branch_stack(event))
    return -EOPNOTSUPP;
// Invalid event code
    if ((event.attr.config <= 0) || (event.attr.config > 3))
    return -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_counter_data(event: *mut perf_event) -> c_ulong {
    static unsigned long get_counter_data(struct perf_event *event)
    {
    let mut config: c_uint = event.attr.config;
    u64 data;
    switch (config) {
    case L1_TO_L2_CS_LAT:
    if (event.attach_state & PERF_ATTACH_TASK)
    data = kvmhv_get_l1_to_l2_cs_time_vcpu();
    else
    data = kvmhv_get_l1_to_l2_cs_time();
    break;
    case L2_TO_L1_CS_LAT:
    if (event.attach_state & PERF_ATTACH_TASK)
    data = kvmhv_get_l2_to_l1_cs_time_vcpu();
    else
    data = kvmhv_get_l2_to_l1_cs_time();
    break;
    case L2_RUNTIME_AGG:
    if (event.attach_state & PERF_ATTACH_TASK)
    data = kvmhv_get_l2_runtime_agg_vcpu();
    else
    data = kvmhv_get_l2_runtime_agg();
    break;
    default:
    data = 0;
    break;
    }
    return data;
    }
#[no_mangle]
unsafe extern "C" fn vpa_pmu_add(event: *mut perf_event, flags: c_int) -> c_int {
    static int vpa_pmu_add(struct perf_event *event, int flags)
    {
    u64 data;
    kvmhv_set_l2_counters_status(smp_processor_id(), true);
    data = get_counter_data(event);
    local64_set(&event.hw.prev_count, data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vpa_pmu_read(event: *mut perf_event) {
    static void vpa_pmu_read(struct perf_event *event)
    {
    u64 prev_data, new_data, final_data;
    prev_data = local64_read(&event.hw.prev_count);
    new_data = get_counter_data(event);
    final_data = new_data - prev_data;
    local64_add(final_data, &event.count);
    }
#[no_mangle]
unsafe extern "C" fn vpa_pmu_del(event: *mut perf_event, flags: c_int) {
    static void vpa_pmu_del(struct perf_event *event, int flags)
    {
    vpa_pmu_read(event);
//
// Disable vpa counter accumulation
//
    kvmhv_set_l2_counters_status(smp_processor_id(), false);
    }
    static struct pmu vpa_pmu = {
    .module		= THIS_MODULE,
    .task_ctx_nr	= perf_sw_context,
    .name		= "vpa_pmu",
    .event_init	= vpa_pmu_event_init,
    .add		= vpa_pmu_add,
    .del		= vpa_pmu_del,
    .read		= vpa_pmu_read,
    .attr_groups	= vpa_pmu_attr_groups,
    .capabilities	= PERF_PMU_CAP_NO_EXCLUDE | PERF_PMU_CAP_NO_INTERRUPT,
    };
#[no_mangle]
unsafe extern "C" fn pseries_vpa_pmu_init() -> int __init {
    static int __init pseries_vpa_pmu_init(void)
    {
//
// List of current Linux on Power platforms and
// this driver is supported only in PowerVM LPAR
// (L1) platform.
//
// Enabled    Linux on Power Platforms
// ----------------------------------------
// [X]      PowerVM LPAR (L1)
// [ ]      KVM Guest On PowerVM KoP(L2)
// [ ]      Baremetal(PowerNV)
// [ ]      KVM Guest On PowerNV
//
    if (!firmware_has_feature(FW_FEATURE_LPAR) || is_kvm_guest())
    return -ENODEV;
    perf_pmu_register(&vpa_pmu, vpa_pmu.name, -1);
    pr_info("Virtual Processor Area PMU registered.\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pseries_vpa_pmu_cleanup() -> void __exit {
    static void __exit pseries_vpa_pmu_cleanup(void)
    {
    perf_pmu_unregister(&vpa_pmu);
    pr_info("Virtual Processor Area PMU unregistered.\n");
    }
    module_init(pseries_vpa_pmu_init);
    module_exit(pseries_vpa_pmu_cleanup);
    MODULE_DESCRIPTION("Perf Driver for pSeries VPA pmu counter");
    MODULE_AUTHOR("Kajol Jain <kjain@linux.ibm.com>");
    MODULE_AUTHOR("Madhavan Srinivasan <maddy@linux.ibm.com>");
    MODULE_LICENSE("GPL");
