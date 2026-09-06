//! Automatically rewritten from C to Rust
//! Source: drivers/perf/dwc_pcie_pmu.c
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
// Synopsys DesignWare PCIe PMU driver
//
// Copyright (C) 2021-2023 Alibaba Inc.
//

pub const DWC_PCIE_EVENT_CNT_CTL: c_uint = 0x8;
//
// Event Counter Data Select includes two parts:
// - 27-24: Group number(4-bit: 0..0x7)
// - 23-16: Event number(8-bit: 0..0x13) within the Group
//
// Put them together as in TRM.
//

pub const DWC_PCIE_PER_EVENT_OFF: c_uint = 0x1;
pub const DWC_PCIE_PER_EVENT_ON: c_uint = 0x3;

pub const DWC_PCIE_EVENT_PER_CLEAR: c_uint = 0x1;
// Event Selection Field has two subfields

pub const DWC_PCIE_EVENT_CNT_DATA: c_uint = 0xC;
pub const DWC_PCIE_TIME_BASED_ANAL_CTL: c_uint = 0x10;

pub const DWC_PCIE_DURATION_MANUAL_CTL: c_uint = 0x0;
pub const DWC_PCIE_DURATION_1MS: c_uint = 0x1;
pub const DWC_PCIE_DURATION_10MS: c_uint = 0x2;
pub const DWC_PCIE_DURATION_100MS: c_uint = 0x3;
pub const DWC_PCIE_DURATION_1S: c_uint = 0x4;
pub const DWC_PCIE_DURATION_2S: c_uint = 0x5;
pub const DWC_PCIE_DURATION_4S: c_uint = 0x6;
pub const DWC_PCIE_DURATION_4US: c_uint = 0xFF;

pub const DWC_PCIE_TIME_BASED_CNT_ENABLE: c_uint = 0x1;
pub const DWC_PCIE_TIME_BASED_ANAL_DATA_REG_LOW: c_uint = 0x14;
pub const DWC_PCIE_TIME_BASED_ANAL_DATA_REG_HIGH: c_uint = 0x18;
// Event attributes

    enum dwc_pcie_event_type {
    DWC_PCIE_TIME_BASE_EVENT,
    DWC_PCIE_LANE_EVENT,
    DWC_PCIE_EVENT_TYPE_MAX,
    };
pub const DWC_PCIE_LANE_GROUP_6: c_int = 6;
pub const DWC_PCIE_LANE_GROUP_7: c_int = 7;
pub const DWC_PCIE_LANE_MAX_EVENTS_PER_GROUP: c_int = 256;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc_pcie_pmu {
    pub pmu: pmu,
    pub /: *mut *mut *mut pci_dev pdev; / Root Port device,
    pub ras_des_offset: u16,
    pub nr_lanes: u32,
// Groups #6 and #7
    pub DWC_PCIE_LANE_MAX_EVENTS_PER_GROUP): *mut *mut DECLARE_BITMAP(lane_events, 2,
    pub time_based_event: *mut perf_event,
    pub timer_enable: bool,
    pub hrtimer: hrtimer,
    pub cpuhp_node: hlist_node,
    pub on_cpu: c_int,
}

    static int dwc_pcie_pmu_hp_state;
    static struct list_head dwc_pcie_dev_info_head =
    LIST_HEAD_INIT(dwc_pcie_dev_info_head);
    static bool notify;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc_pcie_dev_info {
    pub plat_dev: *mut platform_device,
    pub pdev: *mut pci_dev,
    pub dev_node: list_head,
}

    static ssize_t cpumask_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct dwc_pcie_pmu *pcie_pmu = to_dwc_pcie_pmu(dev_get_drvdata(dev));
    return sysfs_emit(buf, "%*pbl\n", cpumask_pr_args(cpumask_of(pcie_pmu.on_cpu)));
    }
    static DEVICE_ATTR_RO(cpumask);
    static struct attribute *dwc_pcie_pmu_cpumask_attrs[] = {
    &dev_attr_cpumask.attr,
    core::ptr::null_mut()
    };
    static struct attribute_group dwc_pcie_cpumask_attr_group = {
    .attrs = dwc_pcie_pmu_cpumask_attrs,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc_pcie_format_attr {
    pub attr: device_attribute,
    pub field: u64,
    pub config: c_int,
}

    PMU_FORMAT_ATTR(eventid, "config:0-15");
    PMU_FORMAT_ATTR(type, "config:16-19");
    PMU_FORMAT_ATTR(lane, "config:20-27");
    static struct attribute *dwc_pcie_format_attrs[] = {
    &format_attr_type.attr,
    &format_attr_eventid.attr,
    &format_attr_lane.attr,
    core::ptr::null_mut(),
    };
    static struct attribute_group dwc_pcie_format_attrs_group = {
    .name = "format",
    .attrs = dwc_pcie_format_attrs,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc_pcie_event_attr {
    pub attr: device_attribute,
    pub type: enum dwc_pcie_event_type,
    pub eventid: u16,
    pub lane: u8,
}

    static ssize_t dwc_pcie_event_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct dwc_pcie_event_attr *eattr;
    eattr = container_of(attr, typeof(*eattr), attr);
    if (eattr.type == DWC_PCIE_LANE_EVENT)
    return sysfs_emit(buf, "eventid=0x%x,type=0x%x,lane=?\n",
    eattr.eventid, eattr.type);
#[no_mangle]
pub unsafe extern "C" fn if(DWC_PCIE_TIME_BASE_EVENT: eattr->type ==) -> else {
    else if (eattr.type == DWC_PCIE_TIME_BASE_EVENT)
    return sysfs_emit(buf, "eventid=0x%x,type=0x%x\n",
    eattr.eventid, eattr.type);
    return 0;
    }

    (&((struct dwc_pcie_event_attr[]) {{				\
    .attr = __ATTR(_name, 0444, dwc_pcie_event_show, core::ptr::null_mut()),	\
    .type = _type,						\
    .eventid = _eventid,					\
    .lane = _lane,						\
    }})[0].attr.attr)

    DWC_PCIE_EVENT_ATTR(_name, DWC_PCIE_TIME_BASE_EVENT, _eventid, 0)

    DWC_PCIE_EVENT_ATTR(_name, DWC_PCIE_LANE_EVENT, _eventid, 0)
    static struct attribute *dwc_pcie_pmu_time_event_attrs[] = {
// Group #0
    DWC_PCIE_PMU_TIME_BASE_EVENT_ATTR(one_cycle, 0x00),
    DWC_PCIE_PMU_TIME_BASE_EVENT_ATTR(TX_L0S, 0x01),
    DWC_PCIE_PMU_TIME_BASE_EVENT_ATTR(RX_L0S, 0x02),
    DWC_PCIE_PMU_TIME_BASE_EVENT_ATTR(L0, 0x03),
    DWC_PCIE_PMU_TIME_BASE_EVENT_ATTR(L1, 0x04),
    DWC_PCIE_PMU_TIME_BASE_EVENT_ATTR(L1_1, 0x05),
    DWC_PCIE_PMU_TIME_BASE_EVENT_ATTR(L1_2, 0x06),
    DWC_PCIE_PMU_TIME_BASE_EVENT_ATTR(CFG_RCVRY, 0x07),
    DWC_PCIE_PMU_TIME_BASE_EVENT_ATTR(L1_AUX, 0x08),
    DWC_PCIE_PMU_TIME_BASE_EVENT_ATTR(TX_RX_L0S, 0x09),
// Group #1
    DWC_PCIE_PMU_TIME_BASE_EVENT_ATTR(tx_pcie_tlp_data_payload, 0x20),
    DWC_PCIE_PMU_TIME_BASE_EVENT_ATTR(rx_pcie_tlp_data_payload, 0x21),
    DWC_PCIE_PMU_TIME_BASE_EVENT_ATTR(tx_ccix_tlp_data_payload, 0x22),
    DWC_PCIE_PMU_TIME_BASE_EVENT_ATTR(rx_ccix_tlp_data_payload, 0x23),
//
// Leave it to the user to specify the lane ID to avoid generating
// a list of hundreds of events.
//
    DWC_PCIE_PMU_LANE_EVENT_ATTR(tx_ack_dllp, 0x600),
    DWC_PCIE_PMU_LANE_EVENT_ATTR(tx_update_fc_dllp, 0x601),
    DWC_PCIE_PMU_LANE_EVENT_ATTR(rx_ack_dllp, 0x602),
    DWC_PCIE_PMU_LANE_EVENT_ATTR(rx_update_fc_dllp, 0x603),
    DWC_PCIE_PMU_LANE_EVENT_ATTR(rx_nullified_tlp, 0x604),
    DWC_PCIE_PMU_LANE_EVENT_ATTR(tx_nullified_tlp, 0x605),
    DWC_PCIE_PMU_LANE_EVENT_ATTR(rx_duplicate_tlp, 0x606),
    DWC_PCIE_PMU_LANE_EVENT_ATTR(tx_memory_write, 0x700),
    DWC_PCIE_PMU_LANE_EVENT_ATTR(tx_memory_read, 0x701),
    DWC_PCIE_PMU_LANE_EVENT_ATTR(tx_configuration_write, 0x702),
    DWC_PCIE_PMU_LANE_EVENT_ATTR(tx_configuration_read, 0x703),
    DWC_PCIE_PMU_LANE_EVENT_ATTR(tx_io_write, 0x704),
    DWC_PCIE_PMU_LANE_EVENT_ATTR(tx_io_read, 0x705),
    DWC_PCIE_PMU_LANE_EVENT_ATTR(tx_completion_without_data, 0x706),
    DWC_PCIE_PMU_LANE_EVENT_ATTR(tx_completion_with_data, 0x707),
    DWC_PCIE_PMU_LANE_EVENT_ATTR(tx_message_tlp, 0x708),
    DWC_PCIE_PMU_LANE_EVENT_ATTR(tx_atomic, 0x709),
    DWC_PCIE_PMU_LANE_EVENT_ATTR(tx_tlp_with_prefix, 0x70A),
    DWC_PCIE_PMU_LANE_EVENT_ATTR(rx_memory_write, 0x70B),
    DWC_PCIE_PMU_LANE_EVENT_ATTR(rx_memory_read, 0x70C),
    DWC_PCIE_PMU_LANE_EVENT_ATTR(rx_io_write, 0x70F),
    DWC_PCIE_PMU_LANE_EVENT_ATTR(rx_io_read, 0x710),
    DWC_PCIE_PMU_LANE_EVENT_ATTR(rx_completion_without_data, 0x711),
    DWC_PCIE_PMU_LANE_EVENT_ATTR(rx_completion_with_data, 0x712),
    DWC_PCIE_PMU_LANE_EVENT_ATTR(rx_message_tlp, 0x713),
    DWC_PCIE_PMU_LANE_EVENT_ATTR(rx_atomic, 0x714),
    DWC_PCIE_PMU_LANE_EVENT_ATTR(rx_tlp_with_prefix, 0x715),
    DWC_PCIE_PMU_LANE_EVENT_ATTR(tx_ccix_tlp, 0x716),
    DWC_PCIE_PMU_LANE_EVENT_ATTR(rx_ccix_tlp, 0x717),
    core::ptr::null_mut()
    };
    static const struct attribute_group dwc_pcie_event_attrs_group = {
    .name = "events",
    .attrs = dwc_pcie_pmu_time_event_attrs,
    };
    static const struct attribute_group *dwc_pcie_attr_groups[] = {
    &dwc_pcie_event_attrs_group,
    &dwc_pcie_format_attrs_group,
    &dwc_pcie_cpumask_attr_group,
    core::ptr::null_mut()
    };
    static void dwc_pcie_pmu_lane_event_enable(struct dwc_pcie_pmu *pcie_pmu,
    struct perf_event *event,
    bool enable)
    {
    struct pci_dev *pdev = pcie_pmu.pdev;
    let mut ras_des_offset: u16 = pcie_pmu.ras_des_offset;
    let mut event_id: c_int = DWC_PCIE_EVENT_ID(event);
    let mut lane: c_int = DWC_PCIE_EVENT_LANE(event);
    u32 ctrl;
    ctrl = FIELD_PREP(DWC_PCIE_CNT_EVENT_SEL, event_id) |
    FIELD_PREP(DWC_PCIE_CNT_LANE_SEL, lane) |
    FIELD_PREP(DWC_PCIE_EVENT_CLEAR, DWC_PCIE_EVENT_PER_CLEAR);
    if (enable)
    ctrl |= FIELD_PREP(DWC_PCIE_CNT_ENABLE, DWC_PCIE_PER_EVENT_ON);
    else
    ctrl |= FIELD_PREP(DWC_PCIE_CNT_ENABLE, DWC_PCIE_PER_EVENT_OFF);
    pci_write_config_dword(pdev, ras_des_offset + DWC_PCIE_EVENT_CNT_CTL,
    ctrl);
    }
    static void dwc_pcie_pmu_time_based_event_enable(struct dwc_pcie_pmu *pcie_pmu,
    bool enable)
    {
    struct pci_dev *pdev = pcie_pmu.pdev;
    let mut ras_des_offset: u16 = pcie_pmu.ras_des_offset;
    pci_clear_and_set_config_dword(pdev,
    ras_des_offset + DWC_PCIE_TIME_BASED_ANAL_CTL,
    DWC_PCIE_TIME_BASED_TIMER_START, enable);
    }
#[no_mangle]
unsafe extern "C" fn dwc_pcie_pmu_read_lane_event_counter(event: *mut perf_event) -> u64 {
    static u64 dwc_pcie_pmu_read_lane_event_counter(struct perf_event *event)
    {
    struct dwc_pcie_pmu *pcie_pmu = to_dwc_pcie_pmu(event.pmu);
    struct pci_dev *pdev = pcie_pmu.pdev;
    let mut event_id: c_int = DWC_PCIE_EVENT_ID(event);
    let mut lane: c_int = DWC_PCIE_EVENT_LANE(event);
    let mut ras_des_offset: u16 = pcie_pmu.ras_des_offset;
    u32 val, ctrl;
    ctrl = FIELD_PREP(DWC_PCIE_CNT_EVENT_SEL, event_id) |
    FIELD_PREP(DWC_PCIE_CNT_LANE_SEL, lane) |
    FIELD_PREP(DWC_PCIE_CNT_ENABLE, DWC_PCIE_PER_EVENT_ON);
    pci_write_config_dword(pdev, ras_des_offset + DWC_PCIE_EVENT_CNT_CTL,
    ctrl);
    pci_read_config_dword(pdev, ras_des_offset + DWC_PCIE_EVENT_CNT_DATA, &val);
    ctrl |= FIELD_PREP(DWC_PCIE_EVENT_CLEAR, DWC_PCIE_EVENT_PER_CLEAR);
    pci_write_config_dword(pdev, ras_des_offset + DWC_PCIE_EVENT_CNT_CTL,
    ctrl);
    return val;
    }
#[no_mangle]
unsafe extern "C" fn dwc_pcie_pmu_read_time_based_counter(event: *mut perf_event) -> u64 {
    static u64 dwc_pcie_pmu_read_time_based_counter(struct perf_event *event)
    {
    struct dwc_pcie_pmu *pcie_pmu = to_dwc_pcie_pmu(event.pmu);
    struct pci_dev *pdev = pcie_pmu.pdev;
    let mut event_id: c_int = DWC_PCIE_EVENT_ID(event);
    let mut ras_des_offset: u16 = pcie_pmu.ras_des_offset;
    u32 lo, hi, ss;
    u64 val;
//
// The 64-bit value of the data counter is spread across two
// registers that are not synchronized. In order to read them
// atomically, ensure that the high 32 bits match before and after
// reading the low 32 bits.
//
    pci_read_config_dword(pdev,
    ras_des_offset + DWC_PCIE_TIME_BASED_ANAL_DATA_REG_HIGH, &hi);
    do {
// snapshot the high 32 bits
    ss = hi;
    pci_read_config_dword(
    pdev, ras_des_offset + DWC_PCIE_TIME_BASED_ANAL_DATA_REG_LOW,
    &lo);
    pci_read_config_dword(
    pdev, ras_des_offset + DWC_PCIE_TIME_BASED_ANAL_DATA_REG_HIGH,
    &hi);
    } while (hi != ss);
    val = ((u64)hi << 32) | lo;
//
// The Group#1 event measures the amount of data processed in 16-byte
// units. Simplify the end-user interface by multiplying the counter
// at the point of read.
//
    if (event_id >= 0x20 && event_id <= 0x23)
    val *= 16;
    return val;
    }
#[no_mangle]
unsafe extern "C" fn dwc_pcie_pmu_reset_time_based_counter(event: *mut perf_event) {
    static void dwc_pcie_pmu_reset_time_based_counter(struct perf_event *event)
    {
    struct dwc_pcie_pmu *pcie_pmu = to_dwc_pcie_pmu(event.pmu);
    struct hw_perf_event *hwc = &event.hw;
    u64 prev;
    dwc_pcie_pmu_time_based_event_enable(pcie_pmu, false);
//
// The hardware counter is reset to zero when disabled. Synchronize
// prev_count so that the next event_update() computes the correct
// delta against the new counter baseline.
//
    do {
    prev = local64_read(&hwc.prev_count);
    } while (local64_cmpxchg(&hwc.prev_count, prev, 0) != prev);
    dwc_pcie_pmu_time_based_event_enable(pcie_pmu, true);
    }
#[no_mangle]
unsafe extern "C" fn dwc_pcie_pmu_event_update(event: *mut perf_event) {
    static void dwc_pcie_pmu_event_update(struct perf_event *event)
    {
    struct hw_perf_event *hwc = &event.hw;
    let mut type: enum dwc_pcie_event_type = DWC_PCIE_EVENT_TYPE(event);
    u64 delta, prev, now;
    if (type == DWC_PCIE_LANE_EVENT) {
    now = dwc_pcie_pmu_read_lane_event_counter(event) &
    DWC_PCIE_LANE_EVENT_MAX_PERIOD;
    local64_add(now, &event.count);
    return;
    }
    do {
    prev = local64_read(&hwc.prev_count);
    now = dwc_pcie_pmu_read_time_based_counter(event);
    } while (local64_cmpxchg(&hwc.prev_count, prev, now) != prev);
    delta = (now - prev) & DWC_PCIE_MAX_PERIOD;
    local64_add(delta, &event.count);
    }
    static int dwc_pcie_pmu_validate_add_lane_event(struct perf_event *event,
    unsigned long val_lane_events[])
    {
    int event_id, event_nr, group;
    event_id = DWC_PCIE_EVENT_ID(event);
    event_nr = FIELD_GET(DWC_PCIE_CNT_EVENT_SEL_EVID, event_id);
    group = FIELD_GET(DWC_PCIE_CNT_EVENT_SEL_GROUP, event_id);
    if (group != DWC_PCIE_LANE_GROUP_6 && group != DWC_PCIE_LANE_GROUP_7)
    return -EINVAL;
    group -= DWC_PCIE_LANE_GROUP_6;
    if (test_and_set_bit(group * DWC_PCIE_LANE_MAX_EVENTS_PER_GROUP + event_nr,
    val_lane_events))
    return -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc_pcie_pmu_validate_group(event: *mut perf_event) -> c_int {
    static int dwc_pcie_pmu_validate_group(struct perf_event *event)
    {
    struct perf_event *sibling, *leader = event.group_leader;
    DECLARE_BITMAP(val_lane_events, 2 * DWC_PCIE_LANE_MAX_EVENTS_PER_GROUP);
    let mut time_event: bool = false;
    int type;
    type = DWC_PCIE_EVENT_TYPE(leader);
    if (type == DWC_PCIE_TIME_BASE_EVENT)
    time_event = true;
    else
    if (dwc_pcie_pmu_validate_add_lane_event(leader, val_lane_events))
    return -ENOSPC;
    for_each_sibling_event(sibling, leader) {
    type = DWC_PCIE_EVENT_TYPE(sibling);
    if (type == DWC_PCIE_TIME_BASE_EVENT) {
    if (time_event)
    return -ENOSPC;
    time_event = true;
    continue;
    }
    if (dwc_pcie_pmu_validate_add_lane_event(sibling, val_lane_events))
    return -ENOSPC;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc_pcie_pmu_hrtimer_callback(hrtimer: *mut hrtimer) -> enum hrtimer_restart {
    static enum hrtimer_restart dwc_pcie_pmu_hrtimer_callback(struct hrtimer *hrtimer)
    {
    struct dwc_pcie_pmu *pcie_pmu = container_of(hrtimer, struct dwc_pcie_pmu, hrtimer);
    struct perf_event *event = pcie_pmu.time_based_event;
    struct hw_perf_event *hwc;
    if (!event)
    return HRTIMER_NORESTART;
    hwc = &event.hw;
    if (hwc.state & PERF_HES_STOPPED)
    return HRTIMER_NORESTART;
    dwc_pcie_pmu_event_update(event);
    dwc_pcie_pmu_reset_time_based_counter(event);
    hrtimer_forward_now(hrtimer, ns_to_ktime(DWC_PCIE_PMU_TIMER_PERIOD_NS));
    return HRTIMER_RESTART;
    }
#[no_mangle]
unsafe extern "C" fn dwc_pcie_pmu_event_init(event: *mut perf_event) -> c_int {
    static int dwc_pcie_pmu_event_init(struct perf_event *event)
    {
    struct dwc_pcie_pmu *pcie_pmu = to_dwc_pcie_pmu(event.pmu);
    let mut type: enum dwc_pcie_event_type = DWC_PCIE_EVENT_TYPE(event);
    struct perf_event *sibling;
    u32 lane;
    if (event.attr.type != event.pmu.type)
    return -ENOENT;
// We don't support sampling
    if (is_sampling_event(event))
    return -EINVAL;
// We cannot support task bound events
    if (event.cpu < 0 || event.attach_state & PERF_ATTACH_TASK)
    return -EINVAL;
    for_each_sibling_event(sibling, event.group_leader) {
    if (sibling.pmu != event.pmu && !is_software_event(sibling))
    return -EINVAL;
    }
    if (type < 0 || type >= DWC_PCIE_EVENT_TYPE_MAX)
    return -EINVAL;
    if (type == DWC_PCIE_LANE_EVENT) {
    lane = DWC_PCIE_EVENT_LANE(event);
    if (lane < 0 || lane >= pcie_pmu.nr_lanes)
    return -EINVAL;
    }
    if (dwc_pcie_pmu_validate_group(event))
    return -ENOSPC;
    event.cpu = pcie_pmu.on_cpu;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc_pcie_pmu_event_start(event: *mut perf_event, flags: c_int) {
    static void dwc_pcie_pmu_event_start(struct perf_event *event, int flags)
    {
    struct hw_perf_event *hwc = &event.hw;
    struct dwc_pcie_pmu *pcie_pmu = to_dwc_pcie_pmu(event.pmu);
    let mut type: enum dwc_pcie_event_type = DWC_PCIE_EVENT_TYPE(event);
    hwc.state = 0;
    local64_set(&hwc.prev_count, 0);
    if (type == DWC_PCIE_LANE_EVENT) {
    dwc_pcie_pmu_lane_event_enable(pcie_pmu, event, true);
    } else if (type == DWC_PCIE_TIME_BASE_EVENT) {
    dwc_pcie_pmu_time_based_event_enable(pcie_pmu, true);
    if (pcie_pmu.timer_enable)
    hrtimer_start(&pcie_pmu.hrtimer,
    ns_to_ktime(DWC_PCIE_PMU_TIMER_PERIOD_NS),
    HRTIMER_MODE_REL_PINNED_HARD);
    }
    }
#[no_mangle]
unsafe extern "C" fn dwc_pcie_pmu_event_stop(event: *mut perf_event, flags: c_int) {
    static void dwc_pcie_pmu_event_stop(struct perf_event *event, int flags)
    {
    struct dwc_pcie_pmu *pcie_pmu = to_dwc_pcie_pmu(event.pmu);
    let mut type: enum dwc_pcie_event_type = DWC_PCIE_EVENT_TYPE(event);
    struct hw_perf_event *hwc = &event.hw;
    if (event.hw.state & PERF_HES_STOPPED)
    return;
    dwc_pcie_pmu_event_update(event);
    if (type == DWC_PCIE_LANE_EVENT) {
    dwc_pcie_pmu_lane_event_enable(pcie_pmu, event, false);
    } else if (type == DWC_PCIE_TIME_BASE_EVENT) {
    dwc_pcie_pmu_time_based_event_enable(pcie_pmu, false);
    if (pcie_pmu.timer_enable)
    hrtimer_cancel(&pcie_pmu.hrtimer);
    }
    hwc.state |= PERF_HES_STOPPED | PERF_HES_UPTODATE;
    }
#[no_mangle]
unsafe extern "C" fn dwc_pcie_pmu_event_add(event: *mut perf_event, flags: c_int) -> c_int {
    static int dwc_pcie_pmu_event_add(struct perf_event *event, int flags)
    {
    struct dwc_pcie_pmu *pcie_pmu = to_dwc_pcie_pmu(event.pmu);
    struct pci_dev *pdev = pcie_pmu.pdev;
    struct hw_perf_event *hwc = &event.hw;
    let mut type: enum dwc_pcie_event_type = DWC_PCIE_EVENT_TYPE(event);
    let mut event_id: c_int = DWC_PCIE_EVENT_ID(event);
    let mut lane: c_int = DWC_PCIE_EVENT_LANE(event);
    let mut ras_des_offset: u16 = pcie_pmu.ras_des_offset;
    u32 ctrl;
    hwc.state = PERF_HES_STOPPED | PERF_HES_UPTODATE;
    if (type == DWC_PCIE_LANE_EVENT) {
    let mut event_nr: c_int = FIELD_GET(DWC_PCIE_CNT_EVENT_SEL_EVID, event_id);
    int group = FIELD_GET(DWC_PCIE_CNT_EVENT_SEL_GROUP, event_id) -
    DWC_PCIE_LANE_GROUP_6;
    if (test_and_set_bit(group * DWC_PCIE_LANE_MAX_EVENTS_PER_GROUP + event_nr,
    pcie_pmu.lane_events))
    return -ENOSPC;
// EVENT_COUNTER_DATA_REG needs clear manually
    ctrl = FIELD_PREP(DWC_PCIE_CNT_EVENT_SEL, event_id) |
    FIELD_PREP(DWC_PCIE_CNT_LANE_SEL, lane) |
    FIELD_PREP(DWC_PCIE_CNT_ENABLE, DWC_PCIE_PER_EVENT_OFF) |
    FIELD_PREP(DWC_PCIE_EVENT_CLEAR, DWC_PCIE_EVENT_PER_CLEAR);
    pci_write_config_dword(pdev, ras_des_offset + DWC_PCIE_EVENT_CNT_CTL,
    ctrl);
    } else if (type == DWC_PCIE_TIME_BASE_EVENT) {
    if (pcie_pmu.time_based_event)
    return -ENOSPC;
    pcie_pmu.time_based_event = event;
//
// TIME_BASED_ANAL_DATA_REG is a 64 bit register, we can safely
// use it with any manually controlled duration. And it is
// cleared when next measurement starts.
//
    ctrl = FIELD_PREP(DWC_PCIE_TIME_BASED_REPORT_SEL, event_id) |
    FIELD_PREP(DWC_PCIE_TIME_BASED_DURATION_SEL,
    DWC_PCIE_DURATION_MANUAL_CTL) |
    DWC_PCIE_TIME_BASED_CNT_ENABLE;
    pci_write_config_dword(
    pdev, ras_des_offset + DWC_PCIE_TIME_BASED_ANAL_CTL, ctrl);
    }
    if (flags & PERF_EF_START)
    dwc_pcie_pmu_event_start(event, PERF_EF_RELOAD);
    perf_event_update_userpage(event);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc_pcie_pmu_event_del(event: *mut perf_event, flags: c_int) {
    static void dwc_pcie_pmu_event_del(struct perf_event *event, int flags)
    {
    struct dwc_pcie_pmu *pcie_pmu = to_dwc_pcie_pmu(event.pmu);
    let mut type: enum dwc_pcie_event_type = DWC_PCIE_EVENT_TYPE(event);
    dwc_pcie_pmu_event_stop(event, flags | PERF_EF_UPDATE);
    perf_event_update_userpage(event);
    if (type == DWC_PCIE_TIME_BASE_EVENT) {
    pcie_pmu.time_based_event = core::ptr::null_mut();
    } else {
    let mut event_id: c_int = DWC_PCIE_EVENT_ID(event);
    let mut event_nr: c_int = FIELD_GET(DWC_PCIE_CNT_EVENT_SEL_EVID, event_id);
    int group    = FIELD_GET(DWC_PCIE_CNT_EVENT_SEL_GROUP, event_id) -
    DWC_PCIE_LANE_GROUP_6;
    clear_bit(group * DWC_PCIE_LANE_MAX_EVENTS_PER_GROUP + event_nr,
    pcie_pmu.lane_events);
    }
    }
#[no_mangle]
unsafe extern "C" fn dwc_pcie_pmu_remove_cpuhp_instance(hotplug_node: *mut c_void) {
    static void dwc_pcie_pmu_remove_cpuhp_instance(void *hotplug_node)
    {
    cpuhp_state_remove_instance_nocalls(dwc_pcie_pmu_hp_state, hotplug_node);
    }
//
// Find the binded DES capability device info of a PCI device.
// @pdev: The PCI device.
//
    static struct dwc_pcie_dev_info *dwc_pcie_find_dev_info(struct pci_dev *pdev)
    {
    struct dwc_pcie_dev_info *dev_info;
    list_for_each_entry(dev_info, &dwc_pcie_dev_info_head, dev_node)
    if (dev_info.pdev == pdev)
    return dev_info;
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn dwc_pcie_unregister_pmu(data: *mut c_void) {
    static void dwc_pcie_unregister_pmu(void *data)
    {
    struct dwc_pcie_pmu *pcie_pmu = data;
    perf_pmu_unregister(&pcie_pmu.pmu);
    }
#[no_mangle]
unsafe extern "C" fn dwc_pcie_des_cap(pdev: *mut pci_dev) -> u16 {
    static u16 dwc_pcie_des_cap(struct pci_dev *pdev)
    {
    const struct dwc_pcie_vsec_id *vid;
    u16 vsec;
    u32 val;
    if (!pci_is_pcie(pdev) || !(pci_pcie_type(pdev) == PCI_EXP_TYPE_ROOT_PORT))
    return 0;
    for (vid = dwc_pcie_rasdes_vsec_ids; vid.vendor_id; vid++) {
    vsec = pci_find_vsec_capability(pdev, vid.vendor_id,
    vid.vsec_id);
    if (vsec) {
    pci_read_config_dword(pdev, vsec + PCI_VNDR_HEADER,
    &val);
    if (PCI_VNDR_HEADER_REV(val) == vid.vsec_rev) {
    pci_dbg(pdev, "Detected PCIe Vendor-Specific Extended Capability RAS DES\n");
    return vsec;
    }
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc_pcie_unregister_dev(dev_info: *mut dwc_pcie_dev_info) {
    static void dwc_pcie_unregister_dev(struct dwc_pcie_dev_info *dev_info)
    {
    platform_device_unregister(dev_info.plat_dev);
    list_del(&dev_info.dev_node);
    kfree(dev_info);
    }
#[no_mangle]
unsafe extern "C" fn dwc_pcie_register_dev(pdev: *mut pci_dev) -> c_int {
    static int dwc_pcie_register_dev(struct pci_dev *pdev)
    {
    struct platform_device *plat_dev;
    struct dwc_pcie_dev_info *dev_info;
    u32 sbdf;
    sbdf = (pci_domain_nr(pdev.bus) << 16) | PCI_DEVID(pdev.bus.number, pdev.devfn);
    plat_dev = platform_device_register_simple("dwc_pcie_pmu", sbdf, core::ptr::null_mut(), 0);
    if (IS_ERR(plat_dev))
    return PTR_ERR(plat_dev);
    dev_info = kzalloc_obj(*dev_info);
    if (!dev_info) {
    platform_device_unregister(plat_dev);
    return -ENOMEM;
    }
// Cache platform device to handle pci device hotplug
    dev_info.plat_dev = plat_dev;
    dev_info.pdev = pdev;
    list_add(&dev_info.dev_node, &dwc_pcie_dev_info_head);
    return 0;
    }
    static int dwc_pcie_pmu_notifier(struct notifier_block *nb,
    unsigned long action, void *data)
    {
    struct device *dev = data;
    struct pci_dev *pdev = to_pci_dev(dev);
    struct dwc_pcie_dev_info *dev_info;
    switch (action) {
    case BUS_NOTIFY_ADD_DEVICE:
    if (!dwc_pcie_des_cap(pdev))
    return NOTIFY_DONE;
    if (dwc_pcie_register_dev(pdev))
    return NOTIFY_BAD;
    break;
    case BUS_NOTIFY_DEL_DEVICE:
    dev_info = dwc_pcie_find_dev_info(pdev);
    if (!dev_info)
    return NOTIFY_DONE;
    dwc_pcie_unregister_dev(dev_info);
    break;
    }
    return NOTIFY_OK;
    }
    static struct notifier_block dwc_pcie_pmu_nb = {
    .notifier_call = dwc_pcie_pmu_notifier,
    };
#[no_mangle]
unsafe extern "C" fn dwc_pcie_pmu_probe(plat_dev: *mut platform_device) -> c_int {
    static int dwc_pcie_pmu_probe(struct platform_device *plat_dev)
    {
    struct pci_dev *pdev;
    struct dwc_pcie_pmu *pcie_pmu;
    char *name;
    u32 sbdf;
    u16 vsec;
    int ret;
    sbdf = plat_dev.id;
    pdev = pci_get_domain_bus_and_slot(sbdf >> 16, PCI_BUS_NUM(sbdf & 0xffff),
    sbdf & 0xff);
    if (!pdev) {
    pr_err("No pdev found for the sbdf 0x%x\n", sbdf);
    return -ENODEV;
    }
    vsec = dwc_pcie_des_cap(pdev);
    if (!vsec)
    return -ENODEV;
    pci_dev_put(pdev);
    name = devm_kasprintf(&plat_dev.dev, GFP_KERNEL, "dwc_rootport_%x", sbdf);
    if (!name)
    return -ENOMEM;
    pcie_pmu = devm_kzalloc(&plat_dev.dev, sizeof(*pcie_pmu), GFP_KERNEL);
    if (!pcie_pmu)
    return -ENOMEM;
    pcie_pmu.pdev = pdev;
    pcie_pmu.ras_des_offset = vsec;
    pcie_pmu.nr_lanes = pcie_get_width_cap(pdev);
    pcie_pmu.on_cpu = -1;
    hrtimer_setup(&pcie_pmu.hrtimer, dwc_pcie_pmu_hrtimer_callback,
    CLOCK_MONOTONIC, HRTIMER_MODE_REL_PINNED_HARD);
//
// Use timer for updating time-based counts on platforms known
// to have narrowed counter.
//
    if (pdev.vendor == PCI_VENDOR_ID_PICOHEART)
    pcie_pmu.timer_enable = true;
    pcie_pmu.pmu = (struct pmu){
    .name		= name,
    .parent		= &plat_dev.dev,
    .module		= THIS_MODULE,
    .attr_groups	= dwc_pcie_attr_groups,
    .capabilities	= PERF_PMU_CAP_NO_EXCLUDE,
    .task_ctx_nr	= perf_invalid_context,
    .event_init	= dwc_pcie_pmu_event_init,
    .add		= dwc_pcie_pmu_event_add,
    .del		= dwc_pcie_pmu_event_del,
    .start		= dwc_pcie_pmu_event_start,
    .stop		= dwc_pcie_pmu_event_stop,
    .read		= dwc_pcie_pmu_event_update,
    };
// Add this instance to the list used by the offline callback
    ret = cpuhp_state_add_instance(dwc_pcie_pmu_hp_state,
    &pcie_pmu.cpuhp_node);
    if (ret) {
    pci_err(pdev, "Error %d registering hotplug @%x\n", ret, sbdf);
    return ret;
    }
// Unwind when platform driver removes
    ret = devm_add_action_or_reset(&plat_dev.dev,
    dwc_pcie_pmu_remove_cpuhp_instance,
    &pcie_pmu.cpuhp_node);
    if (ret)
    return ret;
    ret = perf_pmu_register(&pcie_pmu.pmu, name, -1);
    if (ret) {
    pci_err(pdev, "Error %d registering PMU @%x\n", ret, sbdf);
    return ret;
    }
    ret = devm_add_action_or_reset(&plat_dev.dev, dwc_pcie_unregister_pmu,
    pcie_pmu);
    if (ret)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc_pcie_pmu_online_cpu(cpu: c_uint, cpuhp_node: *mut hlist_node) -> c_int {
    static int dwc_pcie_pmu_online_cpu(unsigned int cpu, struct hlist_node *cpuhp_node)
    {
    struct dwc_pcie_pmu *pcie_pmu;
    pcie_pmu = hlist_entry_safe(cpuhp_node, struct dwc_pcie_pmu, cpuhp_node);
    if (pcie_pmu.on_cpu == -1)
    pcie_pmu.on_cpu = cpumask_local_spread(
    0, dev_to_node(&pcie_pmu.pdev.dev));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dwc_pcie_pmu_offline_cpu(cpu: c_uint, cpuhp_node: *mut hlist_node) -> c_int {
    static int dwc_pcie_pmu_offline_cpu(unsigned int cpu, struct hlist_node *cpuhp_node)
    {
    struct dwc_pcie_pmu *pcie_pmu;
    struct pci_dev *pdev;
    unsigned int target;
    int node;
    pcie_pmu = hlist_entry_safe(cpuhp_node, struct dwc_pcie_pmu, cpuhp_node);
// Nothing to do if this CPU doesn't own the PMU
    if (cpu != pcie_pmu.on_cpu)
    return 0;
    pcie_pmu.on_cpu = -1;
    pdev = pcie_pmu.pdev;
    node = dev_to_node(&pdev.dev);
    target = cpumask_any_and_but(cpumask_of_node(node), cpu_online_mask, cpu);
    if (target >= nr_cpu_ids)
    target = cpumask_any_but(cpu_online_mask, cpu);
    if (target >= nr_cpu_ids) {
    pci_err(pdev, "There is no CPU to set\n");
    return 0;
    }
// This PMU does NOT support interrupt, just migrate context.
    perf_pmu_migrate_context(&pcie_pmu.pmu, cpu, target);
    pcie_pmu.on_cpu = target;
    return 0;
    }
    static struct platform_driver dwc_pcie_pmu_driver = {
    .probe = dwc_pcie_pmu_probe,
    .driver = {.name = "dwc_pcie_pmu",},
    };
#[no_mangle]
unsafe extern "C" fn dwc_pcie_cleanup_devices() {
    static void dwc_pcie_cleanup_devices(void)
    {
    struct dwc_pcie_dev_info *dev_info, *tmp;
    list_for_each_entry_safe(dev_info, tmp, &dwc_pcie_dev_info_head, dev_node) {
    dwc_pcie_unregister_dev(dev_info);
    }
    }
#[no_mangle]
unsafe extern "C" fn dwc_pcie_pmu_init() -> int __init {
    static int __init dwc_pcie_pmu_init(void)
    {
    struct pci_dev *pdev = core::ptr::null_mut();
    int ret;
    for_each_pci_dev(pdev) {
    if (!dwc_pcie_des_cap(pdev))
    continue;
    ret = dwc_pcie_register_dev(pdev);
    if (ret) {
    pci_dev_put(pdev);
    goto err_cleanup;
    }
    }
    ret = cpuhp_setup_state_multi(CPUHP_AP_ONLINE_DYN,
    "perf/dwc_pcie_pmu:online",
    dwc_pcie_pmu_online_cpu,
    dwc_pcie_pmu_offline_cpu);
    if (ret < 0)
    goto err_cleanup;
    dwc_pcie_pmu_hp_state = ret;
    ret = platform_driver_register(&dwc_pcie_pmu_driver);
    if (ret)
    goto err_remove_cpuhp;
    ret = bus_register_notifier(&pci_bus_type, &dwc_pcie_pmu_nb);
    if (ret)
    goto err_unregister_driver;
    notify = true;
    return 0;
    err_unregister_driver:
    platform_driver_unregister(&dwc_pcie_pmu_driver);
    err_remove_cpuhp:
    cpuhp_remove_multi_state(dwc_pcie_pmu_hp_state);
    err_cleanup:
    dwc_pcie_cleanup_devices();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dwc_pcie_pmu_exit() -> void __exit {
    static void __exit dwc_pcie_pmu_exit(void)
    {
    if (notify)
    bus_unregister_notifier(&pci_bus_type, &dwc_pcie_pmu_nb);
    dwc_pcie_cleanup_devices();
    platform_driver_unregister(&dwc_pcie_pmu_driver);
    cpuhp_remove_multi_state(dwc_pcie_pmu_hp_state);
    }
    module_init(dwc_pcie_pmu_init);
    module_exit(dwc_pcie_pmu_exit);
    MODULE_DESCRIPTION("PMU driver for DesignWare Cores PCI Express Controller");
    MODULE_AUTHOR("Shuai Xue <xueshuai@linux.alibaba.com>");
    MODULE_LICENSE("GPL v2");
