//! Automatically rewritten from C to Rust
//! Source: drivers/fpga/dfl-fme-perf.c
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
// Driver for FPGA Management Engine (FME) Global Performance Reporting
//
// Copyright 2019 Intel Corporation, Inc.
//
// Authors:
// Kang Luwei <luwei.kang@intel.com>
// Xiao Guangrong <guangrong.xiao@linux.intel.com>
// Wu Hao <hao.wu@intel.com>
// Xu Yilun <yilun.xu@intel.com>
// Joseph Grecco <joe.grecco@intel.com>
// Enno Luebbers <enno.luebbers@intel.com>
// Tim Whisonant <tim.whisonant@intel.com>
// Ananda Ravuri <ananda.ravuri@intel.com>
// Mitchel, Henry <henry.mitchel@intel.com>
//

//
// Performance Counter Registers for Cache.
//
// Cache Events are listed below as CACHE_EVNT_*.
//
pub const CACHE_CTRL: c_uint = 0x8;

pub const CACHE_EVNT_RD_HIT: c_uint = 0x0;
pub const CACHE_EVNT_WR_HIT: c_uint = 0x1;
pub const CACHE_EVNT_RD_MISS: c_uint = 0x2;
pub const CACHE_EVNT_WR_MISS: c_uint = 0x3;
pub const CACHE_EVNT_RSVD: c_uint = 0x4;
pub const CACHE_EVNT_HOLD_REQ: c_uint = 0x5;
pub const CACHE_EVNT_DATA_WR_PORT_CONTEN: c_uint = 0x6;
pub const CACHE_EVNT_TAG_WR_PORT_CONTEN: c_uint = 0x7;
pub const CACHE_EVNT_TX_REQ_STALL: c_uint = 0x8;
pub const CACHE_EVNT_RX_REQ_STALL: c_uint = 0x9;
pub const CACHE_EVNT_EVICTIONS: c_uint = 0xa;

pub const CACHE_CHANNEL_RD: c_int = 0;
pub const CACHE_CHANNEL_WR: c_int = 1;
pub const CACHE_CNTR0: c_uint = 0x10;
pub const CACHE_CNTR1: c_uint = 0x18;

//
// Performance Counter Registers for Fabric.
//
// Fabric Events are listed below as FAB_EVNT_
//
pub const FAB_CTRL: c_uint = 0x20;

pub const FAB_EVNT_PCIE0_RD: c_uint = 0x0;
pub const FAB_EVNT_PCIE0_WR: c_uint = 0x1;
pub const FAB_EVNT_PCIE1_RD: c_uint = 0x2;
pub const FAB_EVNT_PCIE1_WR: c_uint = 0x3;
pub const FAB_EVNT_UPI_RD: c_uint = 0x4;
pub const FAB_EVNT_UPI_WR: c_uint = 0x5;
pub const FAB_EVNT_MMIO_RD: c_uint = 0x6;
pub const FAB_EVNT_MMIO_WR: c_uint = 0x7;

pub const FAB_PORT_FILTER_DISABLE: c_int = 0;
pub const FAB_PORT_FILTER_ENABLE: c_int = 1;
pub const FAB_CNTR: c_uint = 0x28;

//
// Performance Counter Registers for Clock.
//
// Clock Counter can't be reset or frozen by SW.
//
pub const CLK_CNTR: c_uint = 0x30;
pub const BASIC_EVNT_CLK: c_uint = 0x0;

//
// Performance Counter Registers for IOMMU / VT-D.
//
// VT-D Events are listed below as VTD_EVNT_* and VTD_SIP_EVNT_
//
pub const VTD_CTRL: c_uint = 0x38;

pub const VTD_EVNT_AFU_MEM_RD_TRANS: c_uint = 0x0;
pub const VTD_EVNT_AFU_MEM_WR_TRANS: c_uint = 0x1;
pub const VTD_EVNT_AFU_DEVTLB_RD_HIT: c_uint = 0x2;
pub const VTD_EVNT_AFU_DEVTLB_WR_HIT: c_uint = 0x3;
pub const VTD_EVNT_DEVTLB_4K_FILL: c_uint = 0x4;
pub const VTD_EVNT_DEVTLB_2M_FILL: c_uint = 0x5;
pub const VTD_EVNT_DEVTLB_1G_FILL: c_uint = 0x6;

pub const VTD_CNTR: c_uint = 0x40;

pub const VTD_SIP_CTRL: c_uint = 0x48;

pub const VTD_SIP_EVNT_IOTLB_4K_HIT: c_uint = 0x0;
pub const VTD_SIP_EVNT_IOTLB_2M_HIT: c_uint = 0x1;
pub const VTD_SIP_EVNT_IOTLB_1G_HIT: c_uint = 0x2;
pub const VTD_SIP_EVNT_SLPWC_L3_HIT: c_uint = 0x3;
pub const VTD_SIP_EVNT_SLPWC_L4_HIT: c_uint = 0x4;
pub const VTD_SIP_EVNT_RCC_HIT: c_uint = 0x5;
pub const VTD_SIP_EVNT_IOTLB_4K_MISS: c_uint = 0x6;
pub const VTD_SIP_EVNT_IOTLB_2M_MISS: c_uint = 0x7;
pub const VTD_SIP_EVNT_IOTLB_1G_MISS: c_uint = 0x8;
pub const VTD_SIP_EVNT_SLPWC_L3_MISS: c_uint = 0x9;
pub const VTD_SIP_EVNT_SLPWC_L4_MISS: c_uint = 0xa;
pub const VTD_SIP_EVNT_RCC_MISS: c_uint = 0xb;

pub const PERF_TIMEOUT: c_int = 30;

//
// struct fme_perf_priv - priv data structure for fme perf driver
//
// @dev: parent device.
// @ioaddr: mapped base address of mmio region.
// @pmu: pmu data structure for fme perf counters.
// @id: id of this fme performance report private feature.
// @fab_users: current user number on fabric counters.
// @fab_port_id: used to indicate current working mode of fabric counters.
// @fab_lock: lock to protect fabric counters working mode.
// @cpu: active CPU to which the PMU is bound for accesses.
// @node: node for CPU hotplug notifier link.
// @cpuhp_state: state for CPU hotplug notification;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fme_perf_priv {
    pub dev: *mut device,
    pub ioaddr: *mut void __iomem,
    pub pmu: pmu,
    pub id: u16,
    pub fab_users: u32,
    pub fab_port_id: u32,
    pub fab_lock: spinlock_t,
    pub cpu: c_uint,
    pub node: hlist_node,
    pub cpuhp_state: enum cpuhp_state,
}

//
// struct fme_perf_event_ops - callbacks for fme perf events
//
// @event_init: callback invoked during event init.
// @event_destroy: callback invoked during event destroy.
// @read_counter: callback to read hardware counters.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fme_perf_event_ops {
    pub portid): *mut *mut *mut int (event_init)(struct fme_perf_priv priv, u32 event, u32,
    void (*event_destroy)(struct fme_perf_priv *priv, u32 event,
    pub portid): u32,
    pub portid): *mut *mut *mut u64 (read_counter)(struct fme_perf_priv priv, u32 event, u32,
}

    static ssize_t cpumask_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct pmu *pmu = dev_get_drvdata(dev);
    struct fme_perf_priv *priv;
    priv = to_fme_perf_priv(pmu);
    return sysfs_emit(buf, "%*pbl\n", cpumask_pr_args(cpumask_of(priv.cpu)));
    }
    static DEVICE_ATTR_RO(cpumask);
    static struct attribute *fme_perf_cpumask_attrs[] = {
    &dev_attr_cpumask.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group fme_perf_cpumask_group = {
    .attrs = fme_perf_cpumask_attrs,
    };

pub const FME_EVENT_SHIFT: c_int = 0;

pub const FME_EVTYPE_SHIFT: c_int = 12;
pub const FME_EVTYPE_BASIC: c_int = 0;
pub const FME_EVTYPE_CACHE: c_int = 1;
pub const FME_EVTYPE_FABRIC: c_int = 2;
pub const FME_EVTYPE_VTD: c_int = 3;
pub const FME_EVTYPE_VTD_SIP: c_int = 4;

pub const FME_PORTID_SHIFT: c_int = 16;

    PMU_FORMAT_ATTR(event,		"config:0-11");
    PMU_FORMAT_ATTR(evtype,		"config:12-15");
    PMU_FORMAT_ATTR(portid,		"config:16-23");
    static struct attribute *fme_perf_format_attrs[] = {
    &format_attr_event.attr,
    &format_attr_evtype.attr,
    &format_attr_portid.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group fme_perf_format_group = {
    .name = "format",
    .attrs = fme_perf_format_attrs,
    };
//
// There are no default events, but we need to create
// "events" group (with empty attrs) before updating
// it with detected events (using pmu->attr_update).
//
    static struct attribute *fme_perf_events_attrs_empty[] = {
    core::ptr::null_mut(),
    };
    static const struct attribute_group fme_perf_events_group = {
    .name = "events",
    .attrs = fme_perf_events_attrs_empty,
    };
    static const struct attribute_group *fme_perf_groups[] = {
    &fme_perf_format_group,
    &fme_perf_cpumask_group,
    &fme_perf_events_group,
    core::ptr::null_mut(),
    };
#[no_mangle]
unsafe extern "C" fn is_portid_root(portid: u32) -> bool {
    static bool is_portid_root(u32 portid)
    {
    let mut portid: return = = FME_PORTID_ROOT;
    }
#[no_mangle]
unsafe extern "C" fn is_portid_port(portid: u32) -> bool {
    static bool is_portid_port(u32 portid)
    {
    return portid < PERF_MAX_PORT_NUM;
    }
#[no_mangle]
unsafe extern "C" fn is_portid_root_or_port(portid: u32) -> bool {
    static bool is_portid_root_or_port(u32 portid)
    {
    return is_portid_root(portid) || is_portid_port(portid);
    }
#[no_mangle]
unsafe extern "C" fn fme_read_perf_cntr_reg(addr: *mut void __iomem) -> u64 {
    static u64 fme_read_perf_cntr_reg(void __iomem *addr)
    {
    u32 low;
    u64 v;
//
// For 64bit counter registers, the counter may increases and carries
// out of bit [31] between 2 32bit reads. So add extra reads to help
// to prevent this issue. This only happens in platforms which don't
// support 64bit read - readq is split into 2 readl.
//
    do {
    v = readq(addr);
    low = readl(addr);
    } while (((u32)v) > low);
    return v;
    }
#[no_mangle]
unsafe extern "C" fn basic_event_init(priv: *mut fme_perf_priv, event: u32, portid: u32) -> c_int {
    static int basic_event_init(struct fme_perf_priv *priv, u32 event, u32 portid)
    {
    if (event <= BASIC_EVNT_MAX && is_portid_root(portid))
    return 0;
    return -EINVAL;
    }
    static u64 basic_read_event_counter(struct fme_perf_priv *priv,
    u32 event, u32 portid)
    {
    void __iomem *base = priv.ioaddr;
    return fme_read_perf_cntr_reg(base + CLK_CNTR);
    }
#[no_mangle]
unsafe extern "C" fn cache_event_init(priv: *mut fme_perf_priv, event: u32, portid: u32) -> c_int {
    static int cache_event_init(struct fme_perf_priv *priv, u32 event, u32 portid)
    {
    if (priv.id == FME_FEATURE_ID_GLOBAL_IPERF &&
    event <= CACHE_EVNT_MAX && is_portid_root(portid))
    return 0;
    return -EINVAL;
    }
    static u64 cache_read_event_counter(struct fme_perf_priv *priv,
    u32 event, u32 portid)
    {
    void __iomem *base = priv.ioaddr;
    u64 v, count;
    u8 channel;
    if (event == CACHE_EVNT_WR_HIT || event == CACHE_EVNT_WR_MISS ||
    event == CACHE_EVNT_DATA_WR_PORT_CONTEN ||
    event == CACHE_EVNT_TAG_WR_PORT_CONTEN)
    channel = CACHE_CHANNEL_WR;
    else
    channel = CACHE_CHANNEL_RD;
// set channel access type and cache event code.
    v = readq(base + CACHE_CTRL);
    v &= ~(CACHE_CHANNEL_SEL | CACHE_CTRL_EVNT);
    v |= FIELD_PREP(CACHE_CHANNEL_SEL, channel);
    v |= FIELD_PREP(CACHE_CTRL_EVNT, event);
    writeq(v, base + CACHE_CTRL);
    if (readq_poll_timeout_atomic(base + CACHE_CNTR0, v,
    FIELD_GET(CACHE_CNTR_EVNT, v) == event,
    1, PERF_TIMEOUT)) {
    dev_err(priv.dev, "timeout, unmatched cache event code in counter register.\n");
    return 0;
    }
    v = fme_read_perf_cntr_reg(base + CACHE_CNTR0);
    count = FIELD_GET(CACHE_CNTR_EVNT_CNTR, v);
    v = fme_read_perf_cntr_reg(base + CACHE_CNTR1);
    count += FIELD_GET(CACHE_CNTR_EVNT_CNTR, v);
    return count;
    }
    static bool is_fabric_event_supported(struct fme_perf_priv *priv, u32 event,
    u32 portid)
    {
    if (event > FAB_EVNT_MAX || !is_portid_root_or_port(portid))
    return false;
    if (priv.id == FME_FEATURE_ID_GLOBAL_DPERF &&
    (event == FAB_EVNT_PCIE1_RD || event == FAB_EVNT_UPI_RD ||
    event == FAB_EVNT_PCIE1_WR || event == FAB_EVNT_UPI_WR))
    return false;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn fabric_event_init(priv: *mut fme_perf_priv, event: u32, portid: u32) -> c_int {
    static int fabric_event_init(struct fme_perf_priv *priv, u32 event, u32 portid)
    {
    void __iomem *base = priv.ioaddr;
    let mut ret: c_int = 0;
    u64 v;
    if (!is_fabric_event_supported(priv, event, portid))
    return -EINVAL;
//
// as fabric counter set only can be in either overall or port mode.
// In overall mode, it counts overall data for FPGA, and in port mode,
// it is configured to monitor on one individual port.
//
// so every time, a new event is initialized, driver checks
// current working mode and if someone is using this counter set.
//
    spin_lock(&priv.fab_lock);
    if (priv.fab_users && priv.fab_port_id != portid) {
    dev_dbg(priv.dev, "conflict fabric event monitoring mode.\n");
    ret = -EOPNOTSUPP;
    goto exit;
    }
    priv.fab_users++;
//
// skip if current working mode matches, otherwise change the working
// mode per input port_id, to monitor overall data or another port.
//
    if (priv.fab_port_id == portid)
    goto exit;
    priv.fab_port_id = portid;
    v = readq(base + FAB_CTRL);
    v &= ~(FAB_PORT_FILTER | FAB_PORT_ID);
    if (is_portid_root(portid)) {
    v |= FIELD_PREP(FAB_PORT_FILTER, FAB_PORT_FILTER_DISABLE);
    } else {
    v |= FIELD_PREP(FAB_PORT_FILTER, FAB_PORT_FILTER_ENABLE);
    v |= FIELD_PREP(FAB_PORT_ID, portid);
    }
    writeq(v, base + FAB_CTRL);
    exit:
    spin_unlock(&priv.fab_lock);
    return ret;
    }
    static void fabric_event_destroy(struct fme_perf_priv *priv, u32 event,
    u32 portid)
    {
    spin_lock(&priv.fab_lock);
    priv.fab_users--;
    spin_unlock(&priv.fab_lock);
    }
    static u64 fabric_read_event_counter(struct fme_perf_priv *priv, u32 event,
    u32 portid)
    {
    void __iomem *base = priv.ioaddr;
    u64 v;
    v = readq(base + FAB_CTRL);
    v &= ~FAB_CTRL_EVNT;
    v |= FIELD_PREP(FAB_CTRL_EVNT, event);
    writeq(v, base + FAB_CTRL);
    if (readq_poll_timeout_atomic(base + FAB_CNTR, v,
    FIELD_GET(FAB_CNTR_EVNT, v) == event,
    1, PERF_TIMEOUT)) {
    dev_err(priv.dev, "timeout, unmatched fab event code in counter register.\n");
    return 0;
    }
    v = fme_read_perf_cntr_reg(base + FAB_CNTR);
    return FIELD_GET(FAB_CNTR_EVNT_CNTR, v);
    }
#[no_mangle]
unsafe extern "C" fn vtd_event_init(priv: *mut fme_perf_priv, event: u32, portid: u32) -> c_int {
    static int vtd_event_init(struct fme_perf_priv *priv, u32 event, u32 portid)
    {
    if (priv.id == FME_FEATURE_ID_GLOBAL_IPERF &&
    event <= VTD_EVNT_MAX && is_portid_port(portid))
    return 0;
    return -EINVAL;
    }
    static u64 vtd_read_event_counter(struct fme_perf_priv *priv, u32 event,
    u32 portid)
    {
    void __iomem *base = priv.ioaddr;
    u64 v;
    event += (portid * (VTD_EVNT_MAX + 1));
    v = readq(base + VTD_CTRL);
    v &= ~VTD_CTRL_EVNT;
    v |= FIELD_PREP(VTD_CTRL_EVNT, event);
    writeq(v, base + VTD_CTRL);
    if (readq_poll_timeout_atomic(base + VTD_CNTR, v,
    FIELD_GET(VTD_CNTR_EVNT, v) == event,
    1, PERF_TIMEOUT)) {
    dev_err(priv.dev, "timeout, unmatched vtd event code in counter register.\n");
    return 0;
    }
    v = fme_read_perf_cntr_reg(base + VTD_CNTR);
    return FIELD_GET(VTD_CNTR_EVNT_CNTR, v);
    }
#[no_mangle]
unsafe extern "C" fn vtd_sip_event_init(priv: *mut fme_perf_priv, event: u32, portid: u32) -> c_int {
    static int vtd_sip_event_init(struct fme_perf_priv *priv, u32 event, u32 portid)
    {
    if (priv.id == FME_FEATURE_ID_GLOBAL_IPERF &&
    event <= VTD_SIP_EVNT_MAX && is_portid_root(portid))
    return 0;
    return -EINVAL;
    }
    static u64 vtd_sip_read_event_counter(struct fme_perf_priv *priv, u32 event,
    u32 portid)
    {
    void __iomem *base = priv.ioaddr;
    u64 v;
    v = readq(base + VTD_SIP_CTRL);
    v &= ~VTD_SIP_CTRL_EVNT;
    v |= FIELD_PREP(VTD_SIP_CTRL_EVNT, event);
    writeq(v, base + VTD_SIP_CTRL);
    if (readq_poll_timeout_atomic(base + VTD_SIP_CNTR, v,
    FIELD_GET(VTD_SIP_CNTR_EVNT, v) == event,
    1, PERF_TIMEOUT)) {
    dev_err(priv.dev, "timeout, unmatched vtd sip event code in counter register\n");
    return 0;
    }
    v = fme_read_perf_cntr_reg(base + VTD_SIP_CNTR);
    return FIELD_GET(VTD_SIP_CNTR_EVNT_CNTR, v);
    }
    static struct fme_perf_event_ops fme_perf_event_ops[] = {
    [FME_EVTYPE_BASIC]	= {.event_init = basic_event_init,
    .read_counter = basic_read_event_counter,},
    [FME_EVTYPE_CACHE]	= {.event_init = cache_event_init,
    .read_counter = cache_read_event_counter,},
    [FME_EVTYPE_FABRIC]	= {.event_init = fabric_event_init,
    .event_destroy = fabric_event_destroy,
    .read_counter = fabric_read_event_counter,},
    [FME_EVTYPE_VTD]	= {.event_init = vtd_event_init,
    .read_counter = vtd_read_event_counter,},
    [FME_EVTYPE_VTD_SIP]	= {.event_init = vtd_sip_event_init,
    .read_counter = vtd_sip_read_event_counter,},
    };
    static ssize_t fme_perf_event_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct dev_ext_attribute *eattr;
    unsigned long config;
    char *ptr = buf;
    eattr = container_of(attr, struct dev_ext_attribute, attr);
    config = (unsigned long)eattr.var;
    ptr += sprintf(ptr, "event=0x%02x", (unsigned int)get_event(config));
    ptr += sprintf(ptr, ",evtype=0x%02x", (unsigned int)get_evtype(config));
    if (is_portid_root(get_portid(config)))
    ptr += sprintf(ptr, ",portid=0x%02x\n", FME_PORTID_ROOT);
    else
    ptr += sprintf(ptr, ",portid=?\n");
    return (ssize_t)(ptr - buf);
    }

    __ATTR(_name, 0444, fme_perf_event_show, core::ptr::null_mut())

    (void *)((((_event) << FME_EVENT_SHIFT) & FME_EVENT_MASK) |	\
    (((_type) << FME_EVTYPE_SHIFT) & FME_EVTYPE_MASK))

    (void *)((((_event) << FME_EVENT_SHIFT) & FME_EVENT_MASK) |	\
    (((_type) << FME_EVTYPE_SHIFT) & FME_EVTYPE_MASK) |	\
    (FME_PORTID_ROOT << FME_PORTID_SHIFT))
// FME Perf Basic Events

    static struct dev_ext_attribute fme_perf_event_##_name = {		\
    .attr = FME_EVENT_ATTR(_name),					\
    .var = FME_EVENT_CONFIG(_event, FME_EVTYPE_BASIC),		\
    }
    FME_EVENT_BASIC(clock, BASIC_EVNT_CLK);
    static struct attribute *fme_perf_basic_events_attrs[] = {
    &fme_perf_event_clock.attr.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group fme_perf_basic_events_group = {
    .name = "events",
    .attrs = fme_perf_basic_events_attrs,
    };
// FME Perf Cache Events

    static struct dev_ext_attribute fme_perf_event_cache_##_name = {	\
    .attr = FME_EVENT_ATTR(cache_##_name),				\
    .var = FME_EVENT_CONFIG(_event, FME_EVTYPE_CACHE),		\
    }
    FME_EVENT_CACHE(read_hit,     CACHE_EVNT_RD_HIT);
    FME_EVENT_CACHE(read_miss,    CACHE_EVNT_RD_MISS);
    FME_EVENT_CACHE(write_hit,    CACHE_EVNT_WR_HIT);
    FME_EVENT_CACHE(write_miss,   CACHE_EVNT_WR_MISS);
    FME_EVENT_CACHE(hold_request, CACHE_EVNT_HOLD_REQ);
    FME_EVENT_CACHE(tx_req_stall, CACHE_EVNT_TX_REQ_STALL);
    FME_EVENT_CACHE(rx_req_stall, CACHE_EVNT_RX_REQ_STALL);
    FME_EVENT_CACHE(eviction,     CACHE_EVNT_EVICTIONS);
    FME_EVENT_CACHE(data_write_port_contention, CACHE_EVNT_DATA_WR_PORT_CONTEN);
    FME_EVENT_CACHE(tag_write_port_contention,  CACHE_EVNT_TAG_WR_PORT_CONTEN);
    static struct attribute *fme_perf_cache_events_attrs[] = {
    &fme_perf_event_cache_read_hit.attr.attr,
    &fme_perf_event_cache_read_miss.attr.attr,
    &fme_perf_event_cache_write_hit.attr.attr,
    &fme_perf_event_cache_write_miss.attr.attr,
    &fme_perf_event_cache_hold_request.attr.attr,
    &fme_perf_event_cache_tx_req_stall.attr.attr,
    &fme_perf_event_cache_rx_req_stall.attr.attr,
    &fme_perf_event_cache_eviction.attr.attr,
    &fme_perf_event_cache_data_write_port_contention.attr.attr,
    &fme_perf_event_cache_tag_write_port_contention.attr.attr,
    core::ptr::null_mut(),
    };
    static umode_t fme_perf_events_visible(struct kobject *kobj,
    struct attribute *attr, int n)
    {
    struct pmu *pmu = dev_get_drvdata(kobj_to_dev(kobj));
    struct fme_perf_priv *priv = to_fme_perf_priv(pmu);
    return (priv.id == FME_FEATURE_ID_GLOBAL_IPERF) ? attr.mode : 0;
    }
    static const struct attribute_group fme_perf_cache_events_group = {
    .name = "events",
    .attrs = fme_perf_cache_events_attrs,
    .is_visible = fme_perf_events_visible,
    };
// FME Perf Fabric Events

    static struct dev_ext_attribute fme_perf_event_fab_##_name = {		\
    .attr = FME_EVENT_ATTR(fab_##_name),				\
    .var = FME_EVENT_CONFIG(_event, FME_EVTYPE_FABRIC),		\
    }

    static struct dev_ext_attribute fme_perf_event_fab_port_##_name = {	\
    .attr = FME_EVENT_ATTR(fab_port_##_name),			\
    .var = FME_PORT_EVENT_CONFIG(_event, FME_EVTYPE_FABRIC),	\
    }
    FME_EVENT_FABRIC(pcie0_read,  FAB_EVNT_PCIE0_RD);
    FME_EVENT_FABRIC(pcie0_write, FAB_EVNT_PCIE0_WR);
    FME_EVENT_FABRIC(pcie1_read,  FAB_EVNT_PCIE1_RD);
    FME_EVENT_FABRIC(pcie1_write, FAB_EVNT_PCIE1_WR);
    FME_EVENT_FABRIC(upi_read,    FAB_EVNT_UPI_RD);
    FME_EVENT_FABRIC(upi_write,   FAB_EVNT_UPI_WR);
    FME_EVENT_FABRIC(mmio_read,   FAB_EVNT_MMIO_RD);
    FME_EVENT_FABRIC(mmio_write,  FAB_EVNT_MMIO_WR);
    FME_EVENT_FABRIC_PORT(pcie0_read,  FAB_EVNT_PCIE0_RD);
    FME_EVENT_FABRIC_PORT(pcie0_write, FAB_EVNT_PCIE0_WR);
    FME_EVENT_FABRIC_PORT(pcie1_read,  FAB_EVNT_PCIE1_RD);
    FME_EVENT_FABRIC_PORT(pcie1_write, FAB_EVNT_PCIE1_WR);
    FME_EVENT_FABRIC_PORT(upi_read,    FAB_EVNT_UPI_RD);
    FME_EVENT_FABRIC_PORT(upi_write,   FAB_EVNT_UPI_WR);
    FME_EVENT_FABRIC_PORT(mmio_read,   FAB_EVNT_MMIO_RD);
    FME_EVENT_FABRIC_PORT(mmio_write,  FAB_EVNT_MMIO_WR);
    static struct attribute *fme_perf_fabric_events_attrs[] = {
    &fme_perf_event_fab_pcie0_read.attr.attr,
    &fme_perf_event_fab_pcie0_write.attr.attr,
    &fme_perf_event_fab_pcie1_read.attr.attr,
    &fme_perf_event_fab_pcie1_write.attr.attr,
    &fme_perf_event_fab_upi_read.attr.attr,
    &fme_perf_event_fab_upi_write.attr.attr,
    &fme_perf_event_fab_mmio_read.attr.attr,
    &fme_perf_event_fab_mmio_write.attr.attr,
    &fme_perf_event_fab_port_pcie0_read.attr.attr,
    &fme_perf_event_fab_port_pcie0_write.attr.attr,
    &fme_perf_event_fab_port_pcie1_read.attr.attr,
    &fme_perf_event_fab_port_pcie1_write.attr.attr,
    &fme_perf_event_fab_port_upi_read.attr.attr,
    &fme_perf_event_fab_port_upi_write.attr.attr,
    &fme_perf_event_fab_port_mmio_read.attr.attr,
    &fme_perf_event_fab_port_mmio_write.attr.attr,
    core::ptr::null_mut(),
    };
    static umode_t fme_perf_fabric_events_visible(struct kobject *kobj,
    struct attribute *attr, int n)
    {
    struct pmu *pmu = dev_get_drvdata(kobj_to_dev(kobj));
    struct fme_perf_priv *priv = to_fme_perf_priv(pmu);
    struct dev_ext_attribute *eattr;
    unsigned long var;
    eattr = container_of(attr, struct dev_ext_attribute, attr.attr);
    var = (unsigned long)eattr.var;
    if (is_fabric_event_supported(priv, get_event(var), get_portid(var)))
    return attr.mode;
    return 0;
    }
    static const struct attribute_group fme_perf_fabric_events_group = {
    .name = "events",
    .attrs = fme_perf_fabric_events_attrs,
    .is_visible = fme_perf_fabric_events_visible,
    };
// FME Perf VTD Events

    static struct dev_ext_attribute fme_perf_event_vtd_port_##_name = {	\
    .attr = FME_EVENT_ATTR(vtd_port_##_name),			\
    .var = FME_PORT_EVENT_CONFIG(_event, FME_EVTYPE_VTD),		\
    }
    FME_EVENT_VTD_PORT(read_transaction,  VTD_EVNT_AFU_MEM_RD_TRANS);
    FME_EVENT_VTD_PORT(write_transaction, VTD_EVNT_AFU_MEM_WR_TRANS);
    FME_EVENT_VTD_PORT(devtlb_read_hit,   VTD_EVNT_AFU_DEVTLB_RD_HIT);
    FME_EVENT_VTD_PORT(devtlb_write_hit,  VTD_EVNT_AFU_DEVTLB_WR_HIT);
    FME_EVENT_VTD_PORT(devtlb_4k_fill,    VTD_EVNT_DEVTLB_4K_FILL);
    FME_EVENT_VTD_PORT(devtlb_2m_fill,    VTD_EVNT_DEVTLB_2M_FILL);
    FME_EVENT_VTD_PORT(devtlb_1g_fill,    VTD_EVNT_DEVTLB_1G_FILL);
    static struct attribute *fme_perf_vtd_events_attrs[] = {
    &fme_perf_event_vtd_port_read_transaction.attr.attr,
    &fme_perf_event_vtd_port_write_transaction.attr.attr,
    &fme_perf_event_vtd_port_devtlb_read_hit.attr.attr,
    &fme_perf_event_vtd_port_devtlb_write_hit.attr.attr,
    &fme_perf_event_vtd_port_devtlb_4k_fill.attr.attr,
    &fme_perf_event_vtd_port_devtlb_2m_fill.attr.attr,
    &fme_perf_event_vtd_port_devtlb_1g_fill.attr.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group fme_perf_vtd_events_group = {
    .name = "events",
    .attrs = fme_perf_vtd_events_attrs,
    .is_visible = fme_perf_events_visible,
    };
// FME Perf VTD SIP Events

    static struct dev_ext_attribute fme_perf_event_vtd_sip_##_name = {	\
    .attr = FME_EVENT_ATTR(vtd_sip_##_name),			\
    .var = FME_EVENT_CONFIG(_event, FME_EVTYPE_VTD_SIP),		\
    }
    FME_EVENT_VTD_SIP(iotlb_4k_hit,  VTD_SIP_EVNT_IOTLB_4K_HIT);
    FME_EVENT_VTD_SIP(iotlb_2m_hit,  VTD_SIP_EVNT_IOTLB_2M_HIT);
    FME_EVENT_VTD_SIP(iotlb_1g_hit,  VTD_SIP_EVNT_IOTLB_1G_HIT);
    FME_EVENT_VTD_SIP(slpwc_l3_hit,  VTD_SIP_EVNT_SLPWC_L3_HIT);
    FME_EVENT_VTD_SIP(slpwc_l4_hit,  VTD_SIP_EVNT_SLPWC_L4_HIT);
    FME_EVENT_VTD_SIP(rcc_hit,       VTD_SIP_EVNT_RCC_HIT);
    FME_EVENT_VTD_SIP(iotlb_4k_miss, VTD_SIP_EVNT_IOTLB_4K_MISS);
    FME_EVENT_VTD_SIP(iotlb_2m_miss, VTD_SIP_EVNT_IOTLB_2M_MISS);
    FME_EVENT_VTD_SIP(iotlb_1g_miss, VTD_SIP_EVNT_IOTLB_1G_MISS);
    FME_EVENT_VTD_SIP(slpwc_l3_miss, VTD_SIP_EVNT_SLPWC_L3_MISS);
    FME_EVENT_VTD_SIP(slpwc_l4_miss, VTD_SIP_EVNT_SLPWC_L4_MISS);
    FME_EVENT_VTD_SIP(rcc_miss,      VTD_SIP_EVNT_RCC_MISS);
    static struct attribute *fme_perf_vtd_sip_events_attrs[] = {
    &fme_perf_event_vtd_sip_iotlb_4k_hit.attr.attr,
    &fme_perf_event_vtd_sip_iotlb_2m_hit.attr.attr,
    &fme_perf_event_vtd_sip_iotlb_1g_hit.attr.attr,
    &fme_perf_event_vtd_sip_slpwc_l3_hit.attr.attr,
    &fme_perf_event_vtd_sip_slpwc_l4_hit.attr.attr,
    &fme_perf_event_vtd_sip_rcc_hit.attr.attr,
    &fme_perf_event_vtd_sip_iotlb_4k_miss.attr.attr,
    &fme_perf_event_vtd_sip_iotlb_2m_miss.attr.attr,
    &fme_perf_event_vtd_sip_iotlb_1g_miss.attr.attr,
    &fme_perf_event_vtd_sip_slpwc_l3_miss.attr.attr,
    &fme_perf_event_vtd_sip_slpwc_l4_miss.attr.attr,
    &fme_perf_event_vtd_sip_rcc_miss.attr.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group fme_perf_vtd_sip_events_group = {
    .name = "events",
    .attrs = fme_perf_vtd_sip_events_attrs,
    .is_visible = fme_perf_events_visible,
    };
    static const struct attribute_group *fme_perf_events_groups[] = {
    &fme_perf_basic_events_group,
    &fme_perf_cache_events_group,
    &fme_perf_fabric_events_group,
    &fme_perf_vtd_events_group,
    &fme_perf_vtd_sip_events_group,
    core::ptr::null_mut(),
    };
    static struct fme_perf_event_ops *get_event_ops(u32 evtype)
    {
    if (evtype > FME_EVTYPE_MAX)
    return core::ptr::null_mut();
    return &fme_perf_event_ops[evtype];
    }
#[no_mangle]
unsafe extern "C" fn fme_perf_event_destroy(event: *mut perf_event) {
    static void fme_perf_event_destroy(struct perf_event *event)
    {
    struct fme_perf_event_ops *ops = get_event_ops(event.hw.event_base);
    struct fme_perf_priv *priv = to_fme_perf_priv(event.pmu);
    if (ops.event_destroy)
    ops.event_destroy(priv, event.hw.idx, event.hw.config_base);
    }
#[no_mangle]
unsafe extern "C" fn fme_perf_event_init(event: *mut perf_event) -> c_int {
    static int fme_perf_event_init(struct perf_event *event)
    {
    struct fme_perf_priv *priv = to_fme_perf_priv(event.pmu);
    struct hw_perf_event *hwc = &event.hw;
    struct fme_perf_event_ops *ops;
    u32 eventid, evtype, portid;
// test the event attr type check for PMU enumeration
    if (event.attr.type != event.pmu.type)
    return -ENOENT;
//
// fme counters are shared across all cores.
// Therefore, it does not support per-process mode.
// Also, it does not support event sampling mode.
//
    if (is_sampling_event(event) || event.attach_state & PERF_ATTACH_TASK)
    return -EINVAL;
    if (event.cpu < 0)
    return -EINVAL;
    if (event.cpu != priv.cpu)
    return -EINVAL;
    eventid = get_event(event.attr.config);
    portid = get_portid(event.attr.config);
    evtype = get_evtype(event.attr.config);
    if (evtype > FME_EVTYPE_MAX)
    return -EINVAL;
    hwc.event_base = evtype;
    hwc.idx = (int)eventid;
    hwc.config_base = portid;
    event.destroy = fme_perf_event_destroy;
    dev_dbg(priv.dev, "%s event=0x%x, evtype=0x%x, portid=0x%x,\n",
    __func__, eventid, evtype, portid);
    ops = get_event_ops(evtype);
    if (ops.event_init)
    return ops.event_init(priv, eventid, portid);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fme_perf_event_update(event: *mut perf_event) {
    static void fme_perf_event_update(struct perf_event *event)
    {
    struct fme_perf_event_ops *ops = get_event_ops(event.hw.event_base);
    struct fme_perf_priv *priv = to_fme_perf_priv(event.pmu);
    struct hw_perf_event *hwc = &event.hw;
    u64 now, prev, delta;
    now = ops.read_counter(priv, (u32)hwc.idx, hwc.config_base);
    prev = local64_read(&hwc.prev_count);
    delta = now - prev;
    local64_add(delta, &event.count);
    }
#[no_mangle]
unsafe extern "C" fn fme_perf_event_start(event: *mut perf_event, flags: c_int) {
    static void fme_perf_event_start(struct perf_event *event, int flags)
    {
    struct fme_perf_event_ops *ops = get_event_ops(event.hw.event_base);
    struct fme_perf_priv *priv = to_fme_perf_priv(event.pmu);
    struct hw_perf_event *hwc = &event.hw;
    u64 count;
    count = ops.read_counter(priv, (u32)hwc.idx, hwc.config_base);
    local64_set(&hwc.prev_count, count);
    }
#[no_mangle]
unsafe extern "C" fn fme_perf_event_stop(event: *mut perf_event, flags: c_int) {
    static void fme_perf_event_stop(struct perf_event *event, int flags)
    {
    fme_perf_event_update(event);
    }
#[no_mangle]
unsafe extern "C" fn fme_perf_event_add(event: *mut perf_event, flags: c_int) -> c_int {
    static int fme_perf_event_add(struct perf_event *event, int flags)
    {
    if (flags & PERF_EF_START)
    fme_perf_event_start(event, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fme_perf_event_del(event: *mut perf_event, flags: c_int) {
    static void fme_perf_event_del(struct perf_event *event, int flags)
    {
    fme_perf_event_stop(event, PERF_EF_UPDATE);
    }
#[no_mangle]
unsafe extern "C" fn fme_perf_event_read(event: *mut perf_event) {
    static void fme_perf_event_read(struct perf_event *event)
    {
    fme_perf_event_update(event);
    }
#[no_mangle]
unsafe extern "C" fn fme_perf_setup_hardware(priv: *mut fme_perf_priv) {
    static void fme_perf_setup_hardware(struct fme_perf_priv *priv)
    {
    void __iomem *base = priv.ioaddr;
    u64 v;
// read and save current working mode for fabric counters
    v = readq(base + FAB_CTRL);
    if (FIELD_GET(FAB_PORT_FILTER, v) == FAB_PORT_FILTER_DISABLE)
    priv.fab_port_id = FME_PORTID_ROOT;
    else
    priv.fab_port_id = FIELD_GET(FAB_PORT_ID, v);
    }
    static int fme_perf_pmu_register(struct platform_device *pdev,
    struct fme_perf_priv *priv)
    {
    struct pmu *pmu = &priv.pmu;
    char *name;
    int ret;
    spin_lock_init(&priv.fab_lock);
    fme_perf_setup_hardware(priv);
    pmu.task_ctx_nr =	perf_invalid_context;
    pmu.attr_groups =	fme_perf_groups;
    pmu.attr_update =	fme_perf_events_groups;
    pmu.event_init =	fme_perf_event_init;
    pmu.add =		fme_perf_event_add;
    pmu.del =		fme_perf_event_del;
    pmu.start =		fme_perf_event_start;
    pmu.stop =		fme_perf_event_stop;
    pmu.read =		fme_perf_event_read;
    pmu.capabilities =	PERF_PMU_CAP_NO_INTERRUPT |
    PERF_PMU_CAP_NO_EXCLUDE;
    name = devm_kasprintf(priv.dev, GFP_KERNEL, "dfl_fme%d", pdev.id);
    if (!name)
    return -ENOMEM;
    ret = perf_pmu_register(pmu, name, -1);
    if (ret)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fme_perf_pmu_unregister(priv: *mut fme_perf_priv) {
    static void fme_perf_pmu_unregister(struct fme_perf_priv *priv)
    {
    perf_pmu_unregister(&priv.pmu);
    }
#[no_mangle]
unsafe extern "C" fn fme_perf_offline_cpu(cpu: c_uint, node: *mut hlist_node) -> c_int {
    static int fme_perf_offline_cpu(unsigned int cpu, struct hlist_node *node)
    {
    struct fme_perf_priv *priv;
    int target;
    priv = hlist_entry_safe(node, struct fme_perf_priv, node);
    if (cpu != priv.cpu)
    return 0;
    target = cpumask_any_but(cpu_online_mask, cpu);
    if (target >= nr_cpu_ids)
    return 0;
    priv.cpu = target;
    perf_pmu_migrate_context(&priv.pmu, cpu, target);
    return 0;
    }
    static int fme_perf_init(struct platform_device *pdev,
    struct dfl_feature *feature)
    {
    struct fme_perf_priv *priv;
    int ret;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.dev = &pdev.dev;
    priv.ioaddr = feature.ioaddr;
    priv.id = feature.id;
    priv.cpu = raw_smp_processor_id();
    ret = cpuhp_setup_state_multi(CPUHP_AP_ONLINE_DYN,
    "perf/fpga/dfl_fme:online",
    core::ptr::null_mut(), fme_perf_offline_cpu);
    if (ret < 0)
    return ret;
    priv.cpuhp_state = ret;
// Register the pmu instance for cpu hotplug
    ret = cpuhp_state_add_instance_nocalls(priv.cpuhp_state, &priv.node);
    if (ret)
    goto cpuhp_instance_err;
    ret = fme_perf_pmu_register(pdev, priv);
    if (ret)
    goto pmu_register_err;
    feature.priv = priv;
    return 0;
    pmu_register_err:
    cpuhp_state_remove_instance_nocalls(priv.cpuhp_state, &priv.node);
    cpuhp_instance_err:
    cpuhp_remove_multi_state(priv.cpuhp_state);
    return ret;
    }
    static void fme_perf_uinit(struct platform_device *pdev,
    struct dfl_feature *feature)
    {
    struct fme_perf_priv *priv = feature.priv;
    fme_perf_pmu_unregister(priv);
    cpuhp_state_remove_instance_nocalls(priv.cpuhp_state, &priv.node);
    cpuhp_remove_multi_state(priv.cpuhp_state);
    }
    const struct dfl_feature_id fme_perf_id_table[] = {
    {.id = FME_FEATURE_ID_GLOBAL_IPERF,},
    {.id = FME_FEATURE_ID_GLOBAL_DPERF,},
    {0,}
    };
    const struct dfl_feature_ops fme_perf_ops = {
    .init = fme_perf_init,
    .uinit = fme_perf_uinit,
    };
