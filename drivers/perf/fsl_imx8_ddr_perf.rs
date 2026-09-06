//! Automatically rewritten from C to Rust
//! Source: drivers/perf/fsl_imx8_ddr_perf.c
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
// Copyright 2017 NXP
// Copyright 2016 Freescale Semiconductor, Inc.
//

pub const COUNTER_CNTL: c_uint = 0x0;
pub const COUNTER_READ: c_uint = 0x20;
pub const COUNTER_DPCR1: c_uint = 0x30;
pub const COUNTER_MUX_CNTL: c_uint = 0x50;
pub const COUNTER_MASK_COMP: c_uint = 0x54;
pub const CNTL_OVER: c_uint = 0x1;
pub const CNTL_CLEAR: c_uint = 0x2;
pub const CNTL_EN: c_uint = 0x4;
pub const CNTL_EN_MASK: c_uint = 0xFFFFFFFB;
pub const CNTL_CLEAR_MASK: c_uint = 0xFFFFFFFD;
pub const CNTL_OVER_MASK: c_uint = 0xFFFFFFFE;
pub const CNTL_CP_SHIFT: c_int = 16;

pub const CNTL_CSV_SHIFT: c_int = 24;

pub const READ_PORT_SHIFT: c_int = 0;

pub const READ_CHANNEL_REVERT: c_uint = 0x00000008	/* bit 3 for read channel select */;
pub const WRITE_PORT_SHIFT: c_int = 8;

pub const WRITE_CHANNEL_REVERT: c_uint = 0x00000800	/* bit 11 for write channel select */;
pub const EVENT_CYCLES_ID: c_int = 0;
pub const EVENT_CYCLES_COUNTER: c_int = 0;
pub const NUM_COUNTERS: c_int = 4;
// For removing bias if cycle counter CNTL.CP is set to 0xf0
pub const CYCLES_COUNTER_MASK: c_uint = 0x0FFFFFFF;
pub const AXI_MASKING_REVERT: c_uint = 0xffff0000	/* AXI_MASKING(MSB 16bits) + AXI_ID(LSB 16bits) */;

    static DEFINE_IDA(ddr_ida);
    static DEFINE_IDA(db_ida);
// DDR Perf hardware feature
pub const DDR_CAP_AXI_ID_FILTER: c_uint = 0x1     /* support AXI ID filter */;
pub const DDR_CAP_AXI_ID_FILTER_ENHANCED: c_uint = 0x3     /* support enhanced AXI ID filter */;
pub const DDR_CAP_AXI_ID_PORT_CHANNEL_FILTER: c_uint = 0x4	/* support AXI ID PORT CHANNEL filter */;
// Perf type
    enum fsl_ddr_type {
    DDR_PERF_TYPE = 0,	/* ddr Perf (default) */
    DB_PERF_TYPE,		/* db Perf */
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_ddr_devtype_data {
    pub /: *mut *mut unsigned int quirks; / quirks needed for different DDR Perf core,
    pub /: *const *const *const char identifier; / system PMU identifier for userspace,
    pub /: *mut *mut enum fsl_ddr_type type; / types of Perf, ddr or db,
}

    static const struct fsl_ddr_devtype_data imx8_devtype_data;
    static const struct fsl_ddr_devtype_data imx8m_devtype_data = {
    .quirks = DDR_CAP_AXI_ID_FILTER,
    };
    static const struct fsl_ddr_devtype_data imx8mq_devtype_data = {
    .quirks = DDR_CAP_AXI_ID_FILTER,
    .identifier = "i.MX8MQ",
    };
    static const struct fsl_ddr_devtype_data imx8mm_devtype_data = {
    .quirks = DDR_CAP_AXI_ID_FILTER,
    .identifier = "i.MX8MM",
    };
    static const struct fsl_ddr_devtype_data imx8mn_devtype_data = {
    .quirks = DDR_CAP_AXI_ID_FILTER,
    .identifier = "i.MX8MN",
    };
    static const struct fsl_ddr_devtype_data imx8mp_devtype_data = {
    .quirks = DDR_CAP_AXI_ID_FILTER_ENHANCED,
    .identifier = "i.MX8MP",
    };
    static const struct fsl_ddr_devtype_data imx8dxl_devtype_data = {
    .quirks = DDR_CAP_AXI_ID_PORT_CHANNEL_FILTER,
    .identifier = "i.MX8DXL",
    };
    static const struct fsl_ddr_devtype_data imx8dxl_db_devtype_data = {
    .quirks = DDR_CAP_AXI_ID_PORT_CHANNEL_FILTER,
    .identifier = "i.MX8DXL",
    .type = DB_PERF_TYPE,
    };
    static const struct of_device_id imx_ddr_pmu_dt_ids[] = {
    { .compatible = "fsl,imx8-ddr-pmu", .data = &imx8_devtype_data},
    { .compatible = "fsl,imx8m-ddr-pmu", .data = &imx8m_devtype_data},
    { .compatible = "fsl,imx8mq-ddr-pmu", .data = &imx8mq_devtype_data},
    { .compatible = "fsl,imx8mm-ddr-pmu", .data = &imx8mm_devtype_data},
    { .compatible = "fsl,imx8mn-ddr-pmu", .data = &imx8mn_devtype_data},
    { .compatible = "fsl,imx8mp-ddr-pmu", .data = &imx8mp_devtype_data},
    { .compatible = "fsl,imx8dxl-ddr-pmu", .data = &imx8dxl_devtype_data},
    { .compatible = "fsl,imx8dxl-db-pmu", .data = &imx8dxl_db_devtype_data},
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, imx_ddr_pmu_dt_ids);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddr_pmu {
    pub pmu: pmu,
    pub base: *mut void __iomem,
    pub cpu: c_uint,
    pub node: hlist_node,
    pub dev: *mut device,
    pub events: [*mut perf_event; NUM_COUNTERS],
    pub cpuhp_state: enum cpuhp_state,
    pub devtype_data: *const fsl_ddr_devtype_data,
    pub irq: c_int,
    pub id: c_int,
    pub active_counter: c_int,
}

    static ssize_t ddr_perf_identifier_show(struct device *dev,
    struct device_attribute *attr,
    char *page)
    {
    struct ddr_pmu *pmu = dev_get_drvdata(dev);
    return sysfs_emit(page, "%s\n", pmu.devtype_data.identifier);
    }
    static umode_t ddr_perf_identifier_attr_visible(struct kobject *kobj,
    struct attribute *attr,
    int n)
    {
    struct device *dev = kobj_to_dev(kobj);
    struct ddr_pmu *pmu = dev_get_drvdata(dev);
    if (!pmu.devtype_data.identifier)
    return 0;
    return attr.mode;
    };
    static struct device_attribute ddr_perf_identifier_attr =
    __ATTR(identifier, 0444, ddr_perf_identifier_show, core::ptr::null_mut());
    static struct attribute *ddr_perf_identifier_attrs[] = {
    &ddr_perf_identifier_attr.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group ddr_perf_identifier_attr_group = {
    .attrs = ddr_perf_identifier_attrs,
    .is_visible = ddr_perf_identifier_attr_visible,
    };
    enum ddr_perf_filter_capabilities {
    PERF_CAP_AXI_ID_FILTER = 0,
    PERF_CAP_AXI_ID_FILTER_ENHANCED,
    PERF_CAP_AXI_ID_PORT_CHANNEL_FILTER,
    PERF_CAP_AXI_ID_FEAT_MAX,
    };
#[no_mangle]
unsafe extern "C" fn ddr_perf_filter_cap_get(pmu: *mut ddr_pmu, cap: c_int) -> u32 {
    static u32 ddr_perf_filter_cap_get(struct ddr_pmu *pmu, int cap)
    {
    let mut quirks: u32 = pmu.devtype_data.quirks;
    switch (cap) {
    case PERF_CAP_AXI_ID_FILTER:
    return !!(quirks & DDR_CAP_AXI_ID_FILTER);
    case PERF_CAP_AXI_ID_FILTER_ENHANCED:
    quirks &= DDR_CAP_AXI_ID_FILTER_ENHANCED;
    let mut quirks: return = = DDR_CAP_AXI_ID_FILTER_ENHANCED;
    case PERF_CAP_AXI_ID_PORT_CHANNEL_FILTER:
    return !!(quirks & DDR_CAP_AXI_ID_PORT_CHANNEL_FILTER);
    default:
    WARN(1, "unknown filter cap %d\n", cap);
    }
    return 0;
    }
    static ssize_t ddr_perf_filter_cap_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct ddr_pmu *pmu = dev_get_drvdata(dev);
    struct dev_ext_attribute *ea =
    container_of(attr, struct dev_ext_attribute, attr);
    let mut cap: c_int = (long)ea.var;
    return sysfs_emit(buf, "%u\n", ddr_perf_filter_cap_get(pmu, cap));
    }

    (&((struct dev_ext_attribute) {					\
    __ATTR(_name, 0444, _func, core::ptr::null_mut()), (void *)_var		\
    }).attr.attr)

    PERF_EXT_ATTR_ENTRY(_name, ddr_perf_filter_cap_show, _var)
    static struct attribute *ddr_perf_filter_cap_attr[] = {
    PERF_FILTER_EXT_ATTR_ENTRY(filter, PERF_CAP_AXI_ID_FILTER),
    PERF_FILTER_EXT_ATTR_ENTRY(enhanced_filter, PERF_CAP_AXI_ID_FILTER_ENHANCED),
    PERF_FILTER_EXT_ATTR_ENTRY(super_filter, PERF_CAP_AXI_ID_PORT_CHANNEL_FILTER),
    core::ptr::null_mut(),
    };
    static const struct attribute_group ddr_perf_filter_cap_attr_group = {
    .name = "caps",
    .attrs = ddr_perf_filter_cap_attr,
    };
    static ssize_t ddr_perf_cpumask_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct ddr_pmu *pmu = dev_get_drvdata(dev);
    return sysfs_emit(buf, "%*pbl\n", cpumask_pr_args(cpumask_of(pmu.cpu)));
    }
    static struct device_attribute ddr_perf_cpumask_attr =
    __ATTR(cpumask, 0444, ddr_perf_cpumask_show, core::ptr::null_mut());
    static struct attribute *ddr_perf_cpumask_attrs[] = {
    &ddr_perf_cpumask_attr.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group ddr_perf_cpumask_attr_group = {
    .attrs = ddr_perf_cpumask_attrs,
    };
    static ssize_t
    ddr_pmu_event_show(struct device *dev, struct device_attribute *attr,
    char *page)
    {
    struct perf_pmu_events_attr *pmu_attr;
    pmu_attr = container_of(attr, struct perf_pmu_events_attr, attr);
    return sysfs_emit(page, "event=0x%02llx\n", pmu_attr.id);
    }

    PMU_EVENT_ATTR_ID(_name, ddr_pmu_event_show, _id)
    static struct attribute *ddr_perf_events_attrs[] = {
    IMX8_DDR_PMU_EVENT_ATTR(cycles, EVENT_CYCLES_ID),
    IMX8_DDR_PMU_EVENT_ATTR(selfresh, 0x01),
    IMX8_DDR_PMU_EVENT_ATTR(read-accesses, 0x04),
    IMX8_DDR_PMU_EVENT_ATTR(write-accesses, 0x05),
    IMX8_DDR_PMU_EVENT_ATTR(read-queue-depth, 0x08),
    IMX8_DDR_PMU_EVENT_ATTR(write-queue-depth, 0x09),
    IMX8_DDR_PMU_EVENT_ATTR(lp-read-credit-cnt, 0x10),
    IMX8_DDR_PMU_EVENT_ATTR(hp-read-credit-cnt, 0x11),
    IMX8_DDR_PMU_EVENT_ATTR(write-credit-cnt, 0x12),
    IMX8_DDR_PMU_EVENT_ATTR(read-command, 0x20),
    IMX8_DDR_PMU_EVENT_ATTR(write-command, 0x21),
    IMX8_DDR_PMU_EVENT_ATTR(read-modify-write-command, 0x22),
    IMX8_DDR_PMU_EVENT_ATTR(hp-read, 0x23),
    IMX8_DDR_PMU_EVENT_ATTR(hp-req-nocredit, 0x24),
    IMX8_DDR_PMU_EVENT_ATTR(hp-xact-credit, 0x25),
    IMX8_DDR_PMU_EVENT_ATTR(lp-req-nocredit, 0x26),
    IMX8_DDR_PMU_EVENT_ATTR(lp-xact-credit, 0x27),
    IMX8_DDR_PMU_EVENT_ATTR(wr-xact-credit, 0x29),
    IMX8_DDR_PMU_EVENT_ATTR(read-cycles, 0x2a),
    IMX8_DDR_PMU_EVENT_ATTR(write-cycles, 0x2b),
    IMX8_DDR_PMU_EVENT_ATTR(read-write-transition, 0x30),
    IMX8_DDR_PMU_EVENT_ATTR(precharge, 0x31),
    IMX8_DDR_PMU_EVENT_ATTR(activate, 0x32),
    IMX8_DDR_PMU_EVENT_ATTR(load-mode, 0x33),
    IMX8_DDR_PMU_EVENT_ATTR(perf-mwr, 0x34),
    IMX8_DDR_PMU_EVENT_ATTR(read, 0x35),
    IMX8_DDR_PMU_EVENT_ATTR(read-activate, 0x36),
    IMX8_DDR_PMU_EVENT_ATTR(refresh, 0x37),
    IMX8_DDR_PMU_EVENT_ATTR(write, 0x38),
    IMX8_DDR_PMU_EVENT_ATTR(raw-hazard, 0x39),
    IMX8_DDR_PMU_EVENT_ATTR(axid-read, 0x41),
    IMX8_DDR_PMU_EVENT_ATTR(axid-write, 0x42),
    core::ptr::null_mut(),
    };
    static const int ddr_perf_db_visible_event_list[] = {
    EVENT_CYCLES_ID,
    0x41,
    0x42,
    };
    static umode_t ddr_perf_events_attrs_is_visible(struct kobject *kobj,
    struct attribute *attr, int n)
    {
    struct device *dev = kobj_to_dev(kobj);
    struct ddr_pmu *pmu = dev_get_drvdata(dev);
    struct perf_pmu_events_attr *pmu_attr;
    unsigned int i;
    pmu_attr = container_of(attr, struct perf_pmu_events_attr, attr.attr);
    if (pmu.devtype_data.type == DDR_PERF_TYPE)
    return attr.mode;
// DB Type
    for (i = 0; i < ARRAY_SIZE(ddr_perf_db_visible_event_list); i++)
    if (pmu_attr.id == ddr_perf_db_visible_event_list[i])
    return attr.mode;
    return 0;
    }
    static const struct attribute_group ddr_perf_events_attr_group = {
    .name = "events",
    .attrs = ddr_perf_events_attrs,
    .is_visible = ddr_perf_events_attrs_is_visible,
    };
    PMU_FORMAT_ATTR(event, "config:0-7");
    PMU_FORMAT_ATTR(axi_id, "config1:0-15");
    PMU_FORMAT_ATTR(axi_mask, "config1:16-31");
    PMU_FORMAT_ATTR(axi_port, "config2:0-2");
    PMU_FORMAT_ATTR(axi_channel, "config2:3-3");
    static struct attribute *ddr_perf_format_attrs[] = {
    &format_attr_event.attr,
    &format_attr_axi_id.attr,
    &format_attr_axi_mask.attr,
    &format_attr_axi_port.attr,
    &format_attr_axi_channel.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group ddr_perf_format_attr_group = {
    .name = "format",
    .attrs = ddr_perf_format_attrs,
    };
    static const struct attribute_group *attr_groups[] = {
    &ddr_perf_events_attr_group,
    &ddr_perf_format_attr_group,
    &ddr_perf_cpumask_attr_group,
    &ddr_perf_filter_cap_attr_group,
    &ddr_perf_identifier_attr_group,
    core::ptr::null_mut(),
    };
#[no_mangle]
unsafe extern "C" fn ddr_perf_is_filtered(event: *mut perf_event) -> bool {
    static bool ddr_perf_is_filtered(struct perf_event *event)
    {
    return event.attr.config == 0x41 || event.attr.config == 0x42;
    }
#[no_mangle]
unsafe extern "C" fn ddr_perf_filter_val(event: *mut perf_event) -> u32 {
    static u32 ddr_perf_filter_val(struct perf_event *event)
    {
    return event.attr.config1;
    }
    static bool ddr_perf_filters_compatible(struct perf_event *a,
    struct perf_event *b)
    {
    if (!ddr_perf_is_filtered(a))
    return true;
    if (!ddr_perf_is_filtered(b))
    return true;
    return ddr_perf_filter_val(a) == ddr_perf_filter_val(b);
    }
#[no_mangle]
unsafe extern "C" fn ddr_perf_is_enhanced_filtered(event: *mut perf_event) -> bool {
    static bool ddr_perf_is_enhanced_filtered(struct perf_event *event)
    {
    unsigned int filt;
    struct ddr_pmu *pmu = to_ddr_pmu(event.pmu);
    filt = pmu.devtype_data.quirks & DDR_CAP_AXI_ID_FILTER_ENHANCED;
    return (filt == DDR_CAP_AXI_ID_FILTER_ENHANCED) &&
    ddr_perf_is_filtered(event);
    }
#[no_mangle]
unsafe extern "C" fn ddr_perf_alloc_counter(pmu: *mut ddr_pmu, event: c_int) -> u32 {
    static u32 ddr_perf_alloc_counter(struct ddr_pmu *pmu, int event)
    {
    int i;
//
// Always map cycle event to counter 0
// Cycles counter is dedicated for cycle event
// can't used for the other events
//
    if (event == EVENT_CYCLES_ID) {
    if (pmu.events[EVENT_CYCLES_COUNTER] == core::ptr::null_mut())
    return EVENT_CYCLES_COUNTER;
    else
    return -ENOENT;
    }
    for (i = 1; i < NUM_COUNTERS; i++) {
    if (pmu.events[i] == core::ptr::null_mut())
    return i;
    }
    return -ENOENT;
    }
#[no_mangle]
unsafe extern "C" fn ddr_perf_free_counter(pmu: *mut ddr_pmu, counter: c_int) {
    static void ddr_perf_free_counter(struct ddr_pmu *pmu, int counter)
    {
    pmu.events[counter] = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn ddr_perf_read_counter(pmu: *mut ddr_pmu, counter: c_int) -> u32 {
    static u32 ddr_perf_read_counter(struct ddr_pmu *pmu, int counter)
    {
    struct perf_event *event = pmu.events[counter];
    void __iomem *base = pmu.base;
//
// return bytes instead of bursts from ddr transaction for
// axid-read and axid-write event if PMU core supports enhanced
// filter.
//
    base += ddr_perf_is_enhanced_filtered(event) ? COUNTER_DPCR1 :
    COUNTER_READ;
    return readl_relaxed(base + counter * 4);
    }
#[no_mangle]
unsafe extern "C" fn ddr_perf_event_init(event: *mut perf_event) -> c_int {
    static int ddr_perf_event_init(struct perf_event *event)
    {
    struct ddr_pmu *pmu = to_ddr_pmu(event.pmu);
    struct hw_perf_event *hwc = &event.hw;
    struct perf_event *sibling;
    if (event.attr.type != event.pmu.type)
    return -ENOENT;
    if (is_sampling_event(event) || event.attach_state & PERF_ATTACH_TASK)
    return -EOPNOTSUPP;
    if (event.cpu < 0) {
    dev_warn(pmu.dev, "Can't provide per-task data!\n");
    return -EOPNOTSUPP;
    }
//
// We must NOT create groups containing mixed PMUs, although software
// events are acceptable (for example to create a CCN group
// periodically read when a hrtimer aka cpu-clock leader triggers).
//
    if (event.group_leader.pmu != event.pmu &&
    !is_software_event(event.group_leader))
    return -EINVAL;
    if (pmu.devtype_data.quirks & DDR_CAP_AXI_ID_FILTER) {
    if (!ddr_perf_filters_compatible(event, event.group_leader))
    return -EINVAL;
    for_each_sibling_event(sibling, event.group_leader) {
    if (!ddr_perf_filters_compatible(event, sibling))
    return -EINVAL;
    }
    }
    for_each_sibling_event(sibling, event.group_leader) {
    if (sibling.pmu != event.pmu &&
    !is_software_event(sibling))
    return -EINVAL;
    }
    event.cpu = pmu.cpu;
    hwc.idx = -1;
    return 0;
    }
    static void ddr_perf_counter_enable(struct ddr_pmu *pmu, int config,
    int counter, bool enable)
    {
    let mut reg: u8 = counter * 4 + COUNTER_CNTL;
    int val;
    if (enable) {
//
// cycle counter is special which should firstly write 0 then
// write 1 into CLEAR bit to clear it. Other counters only
// need write 0 into CLEAR bit and it turns out to be 1 by
// hardware. Below enable flow is harmless for all counters.
//
    writel(0, pmu.base + reg);
    val = CNTL_EN | CNTL_CLEAR;
    val |= FIELD_PREP(CNTL_CSV_MASK, config);
//
// On i.MX8MP we need to bias the cycle counter to overflow more often.
// We do this by initializing bits [23:16] of the counter value via the
// COUNTER_CTRL Counter Parameter (CP) field.
//
    if (pmu.devtype_data.quirks & DDR_CAP_AXI_ID_FILTER_ENHANCED) {
    if (counter == EVENT_CYCLES_COUNTER)
    val |= FIELD_PREP(CNTL_CP_MASK, 0xf0);
    }
    writel(val, pmu.base + reg);
    } else {
// Disable counter
    val = readl_relaxed(pmu.base + reg) & CNTL_EN_MASK;
    writel(val, pmu.base + reg);
    }
    }
#[no_mangle]
unsafe extern "C" fn ddr_perf_counter_overflow(pmu: *mut ddr_pmu, counter: c_int) -> bool {
    static bool ddr_perf_counter_overflow(struct ddr_pmu *pmu, int counter)
    {
    int val;
    val = readl_relaxed(pmu.base + counter * 4 + COUNTER_CNTL);
    return val & CNTL_OVER;
    }
#[no_mangle]
unsafe extern "C" fn ddr_perf_counter_clear(pmu: *mut ddr_pmu, counter: c_int) {
    static void ddr_perf_counter_clear(struct ddr_pmu *pmu, int counter)
    {
    let mut reg: u8 = counter * 4 + COUNTER_CNTL;
    int val;
    val = readl_relaxed(pmu.base + reg);
    val &= ~CNTL_CLEAR;
    writel(val, pmu.base + reg);
    val |= CNTL_CLEAR;
    writel(val, pmu.base + reg);
    }
#[no_mangle]
unsafe extern "C" fn ddr_perf_event_update(event: *mut perf_event) {
    static void ddr_perf_event_update(struct perf_event *event)
    {
    struct ddr_pmu *pmu = to_ddr_pmu(event.pmu);
    struct hw_perf_event *hwc = &event.hw;
    u64 new_raw_count;
    let mut counter: c_int = hwc.idx;
    int ret;
    new_raw_count = ddr_perf_read_counter(pmu, counter);
// Remove the bias applied in ddr_perf_counter_enable().
    if (pmu.devtype_data.quirks & DDR_CAP_AXI_ID_FILTER_ENHANCED) {
    if (counter == EVENT_CYCLES_COUNTER)
    new_raw_count &= CYCLES_COUNTER_MASK;
    }
    local64_add(new_raw_count, &event.count);
//
// For legacy SoCs: event counter continue counting when overflow,
// no need to clear the counter.
// For new SoCs: event counter stop counting when overflow, need
// clear counter to let it count again.
//
    if (counter != EVENT_CYCLES_COUNTER) {
    ret = ddr_perf_counter_overflow(pmu, counter);
    if (ret)
    dev_warn_ratelimited(pmu.dev,  "events lost due to counter overflow (config 0x%llx)\n",
    event.attr.config);
    }
// clear counter every time for both cycle counter and event counter
    ddr_perf_counter_clear(pmu, counter);
    }
#[no_mangle]
unsafe extern "C" fn ddr_perf_event_start(event: *mut perf_event, flags: c_int) {
    static void ddr_perf_event_start(struct perf_event *event, int flags)
    {
    struct ddr_pmu *pmu = to_ddr_pmu(event.pmu);
    struct hw_perf_event *hwc = &event.hw;
    let mut counter: c_int = hwc.idx;
    local64_set(&hwc.prev_count, 0);
    ddr_perf_counter_enable(pmu, event.attr.config, counter, true);
    if (!pmu.active_counter++)
    ddr_perf_counter_enable(pmu, EVENT_CYCLES_ID,
    EVENT_CYCLES_COUNTER, true);
    hwc.state = 0;
    }
#[no_mangle]
unsafe extern "C" fn ddr_perf_event_add(event: *mut perf_event, flags: c_int) -> c_int {
    static int ddr_perf_event_add(struct perf_event *event, int flags)
    {
    struct ddr_pmu *pmu = to_ddr_pmu(event.pmu);
    struct hw_perf_event *hwc = &event.hw;
    int counter;
    let mut cfg: c_int = event.attr.config;
    let mut cfg1: c_int = event.attr.config1;
    let mut cfg2: c_int = event.attr.config2;
    if (pmu.devtype_data.quirks & DDR_CAP_AXI_ID_FILTER) {
    int i;
    for (i = 1; i < NUM_COUNTERS; i++) {
    if (pmu.events[i] &&
    !ddr_perf_filters_compatible(event, pmu.events[i]))
    return -EINVAL;
    }
    if (ddr_perf_is_filtered(event)) {
// revert axi id masking(axi_mask) value
    cfg1 ^= AXI_MASKING_REVERT;
    writel(cfg1, pmu.base + COUNTER_DPCR1);
    }
    }
    counter = ddr_perf_alloc_counter(pmu, cfg);
    if (counter < 0) {
    dev_dbg(pmu.dev, "There are not enough counters\n");
    return -EOPNOTSUPP;
    }
    if (pmu.devtype_data.quirks & DDR_CAP_AXI_ID_PORT_CHANNEL_FILTER) {
    if (ddr_perf_is_filtered(event)) {
// revert axi id masking(axi_mask) value
    cfg1 ^= AXI_MASKING_REVERT;
    writel(cfg1, pmu.base + COUNTER_MASK_COMP + ((counter - 1) << 4));
    if (cfg == 0x41) {
// revert axi read channel(axi_channel) value
    cfg2 ^= READ_CHANNEL_REVERT;
    cfg2 |= FIELD_PREP(READ_PORT_MASK, cfg2);
    } else {
// revert axi write channel(axi_channel) value
    cfg2 ^= WRITE_CHANNEL_REVERT;
    cfg2 |= FIELD_PREP(WRITE_PORT_MASK, cfg2);
    }
    writel(cfg2, pmu.base + COUNTER_MUX_CNTL + ((counter - 1) << 4));
    }
    }
    pmu.events[counter] = event;
    hwc.idx = counter;
    hwc.state |= PERF_HES_STOPPED;
    if (flags & PERF_EF_START)
    ddr_perf_event_start(event, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ddr_perf_event_stop(event: *mut perf_event, flags: c_int) {
    static void ddr_perf_event_stop(struct perf_event *event, int flags)
    {
    struct ddr_pmu *pmu = to_ddr_pmu(event.pmu);
    struct hw_perf_event *hwc = &event.hw;
    let mut counter: c_int = hwc.idx;
    ddr_perf_counter_enable(pmu, event.attr.config, counter, false);
    ddr_perf_event_update(event);
    if (!--pmu.active_counter)
    ddr_perf_counter_enable(pmu, EVENT_CYCLES_ID,
    EVENT_CYCLES_COUNTER, false);
    hwc.state |= PERF_HES_STOPPED;
    }
#[no_mangle]
unsafe extern "C" fn ddr_perf_event_del(event: *mut perf_event, flags: c_int) {
    static void ddr_perf_event_del(struct perf_event *event, int flags)
    {
    struct ddr_pmu *pmu = to_ddr_pmu(event.pmu);
    struct hw_perf_event *hwc = &event.hw;
    let mut counter: c_int = hwc.idx;
    ddr_perf_event_stop(event, PERF_EF_UPDATE);
    ddr_perf_free_counter(pmu, counter);
    hwc.idx = -1;
    }
#[no_mangle]
unsafe extern "C" fn ddr_perf_pmu_enable(pmu: *mut pmu) {
    static void ddr_perf_pmu_enable(struct pmu *pmu)
    {
    }
#[no_mangle]
unsafe extern "C" fn ddr_perf_pmu_disable(pmu: *mut pmu) {
    static void ddr_perf_pmu_disable(struct pmu *pmu)
    {
    }
    static void ddr_perf_init(struct ddr_pmu *pmu, void __iomem *base,
    struct device *dev)
    {
// pmu = (struct ddr_pmu) {
    .pmu = (struct pmu) {
    .module	      = THIS_MODULE,
    .parent      = dev,
    .capabilities = PERF_PMU_CAP_NO_EXCLUDE,
    .task_ctx_nr = perf_invalid_context,
    .attr_groups = attr_groups,
    .event_init  = ddr_perf_event_init,
    .add	     = ddr_perf_event_add,
    .del	     = ddr_perf_event_del,
    .start	     = ddr_perf_event_start,
    .stop	     = ddr_perf_event_stop,
    .read	     = ddr_perf_event_update,
    .pmu_enable  = ddr_perf_pmu_enable,
    .pmu_disable = ddr_perf_pmu_disable,
    },
    .base = base,
    .dev = dev,
    };
    }
#[no_mangle]
unsafe extern "C" fn ddr_perf_irq_handler(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t ddr_perf_irq_handler(int irq, void *p)
    {
    int i;
    struct ddr_pmu *pmu = (struct ddr_pmu *) p;
    struct perf_event *event;
// all counter will stop if cycle counter disabled
    ddr_perf_counter_enable(pmu,
    EVENT_CYCLES_ID,
    EVENT_CYCLES_COUNTER,
    false);
//
// When the cycle counter overflows, all counters are stopped,
// and an IRQ is raised. If any other counter overflows, it
// continues counting, and no IRQ is raised. But for new SoCs,
// such as i.MX8MP, event counter would stop when overflow, so
// we need use cycle counter to stop overflow of event counter.
//
// Cycles occur at least 4 times as often as other events, so we
// can update all events on a cycle counter overflow and not
// lose events.
//
    for (i = 0; i < NUM_COUNTERS; i++) {
    if (!pmu.events[i])
    continue;
    event = pmu.events[i];
    ddr_perf_event_update(event);
    }
    ddr_perf_counter_enable(pmu,
    EVENT_CYCLES_ID,
    EVENT_CYCLES_COUNTER,
    true);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn ddr_perf_offline_cpu(cpu: c_uint, node: *mut hlist_node) -> c_int {
    static int ddr_perf_offline_cpu(unsigned int cpu, struct hlist_node *node)
    {
    struct ddr_pmu *pmu = hlist_entry_safe(node, struct ddr_pmu, node);
    int target;
    if (cpu != pmu.cpu)
    return 0;
    target = cpumask_any_but(cpu_online_mask, cpu);
    if (target >= nr_cpu_ids)
    return 0;
    perf_pmu_migrate_context(&pmu.pmu, cpu, target);
    pmu.cpu = target;
    WARN_ON(irq_set_affinity(pmu.irq, cpumask_of(pmu.cpu)));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ddr_perf_probe(pdev: *mut platform_device) -> c_int {
    static int ddr_perf_probe(struct platform_device *pdev)
    {
    struct clk_bulk_data *clks;
    struct ddr_pmu *pmu;
    struct device_node *np;
    void __iomem *base;
    struct ida *ida;
    char *name;
    int nclks;
    int num;
    int ret;
    int irq;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    np = pdev.dev.of_node;
    pmu = devm_kzalloc(&pdev.dev, sizeof(*pmu), GFP_KERNEL);
    if (!pmu)
    return -ENOMEM;
    ddr_perf_init(pmu, base, &pdev.dev);
    platform_set_drvdata(pdev, pmu);
    nclks = devm_clk_bulk_get_all_enabled(&pdev.dev, &clks);
    if (nclks < 0)
    return dev_err_probe(&pdev.dev, nclks, "Failure get clks\n");
    pmu.devtype_data = of_device_get_match_data(&pdev.dev);
    ida = pmu.devtype_data.type == DDR_PERF_TYPE ? &ddr_ida : &db_ida;
    num = ida_alloc(ida, GFP_KERNEL);
    if (num < 0)
    return num;
    pmu.id = num;
    if (pmu.devtype_data.type == DDR_PERF_TYPE)
    name = devm_kasprintf(&pdev.dev, GFP_KERNEL, DDR_PERF_DEV_NAME "%d", num);
    else
    name = devm_kasprintf(&pdev.dev, GFP_KERNEL, DB_PERF_DEV_NAME "%d", num);
    if (!name) {
    ret = -ENOMEM;
    goto idr_free;
    }
    pmu.cpu = raw_smp_processor_id();
    ret = cpuhp_setup_state_multi(CPUHP_AP_ONLINE_DYN,
    DDR_CPUHP_CB_NAME,
    core::ptr::null_mut(),
    ddr_perf_offline_cpu);
    if (ret < 0) {
    dev_err(&pdev.dev, "cpuhp_setup_state_multi failed\n");
    goto idr_free;
    }
    pmu.cpuhp_state = ret;
// Register the pmu instance for cpu hotplug
    ret = cpuhp_state_add_instance_nocalls(pmu.cpuhp_state, &pmu.node);
    if (ret) {
    dev_err(&pdev.dev, "Error %d registering hotplug\n", ret);
    goto cpuhp_instance_err;
    }
// Request irq
    irq = of_irq_get(np, 0);
    if (irq < 0) {
    dev_err(&pdev.dev, "Failed to get irq: %d", irq);
    ret = irq;
    goto ddr_perf_err;
    }
    ret = devm_request_irq(&pdev.dev, irq,
    ddr_perf_irq_handler,
    IRQF_NOBALANCING | IRQF_NO_THREAD,
    DDR_CPUHP_CB_NAME,
    pmu);
    if (ret < 0)
    goto ddr_perf_err;
    pmu.irq = irq;
    ret = irq_set_affinity(pmu.irq, cpumask_of(pmu.cpu));
    if (ret) {
    dev_err(pmu.dev, "Failed to set interrupt affinity!\n");
    goto ddr_perf_err;
    }
    ret = perf_pmu_register(&pmu.pmu, name, -1);
    if (ret)
    goto ddr_perf_err;
    return 0;
    ddr_perf_err:
    cpuhp_state_remove_instance_nocalls(pmu.cpuhp_state, &pmu.node);
    cpuhp_instance_err:
    cpuhp_remove_multi_state(pmu.cpuhp_state);
    idr_free:
    ida_free(ida, pmu.id);
    dev_warn(&pdev.dev, "i.MX8 DDR Perf PMU failed (%d), disabled\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ddr_perf_remove(pdev: *mut platform_device) {
    static void ddr_perf_remove(struct platform_device *pdev)
    {
    struct ddr_pmu *pmu = platform_get_drvdata(pdev);
    cpuhp_state_remove_instance_nocalls(pmu.cpuhp_state, &pmu.node);
    cpuhp_remove_multi_state(pmu.cpuhp_state);
    perf_pmu_unregister(&pmu.pmu);
    if (pmu.devtype_data.type == DDR_PERF_TYPE)
    ida_free(&ddr_ida, pmu.id);
    else
    ida_free(&db_ida, pmu.id);
    }
    static struct platform_driver imx_ddr_pmu_driver = {
    .driver         = {
    .name   = "imx-ddr-pmu",
    .of_match_table = imx_ddr_pmu_dt_ids,
    .suppress_bind_attrs = true,
    },
    .probe          = ddr_perf_probe,
    .remove         = ddr_perf_remove,
    };
    module_platform_driver(imx_ddr_pmu_driver);
    MODULE_DESCRIPTION("Freescale i.MX8 DDR Performance Monitor Driver");
    MODULE_LICENSE("GPL v2");
