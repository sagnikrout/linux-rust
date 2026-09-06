//! Automatically rewritten from C to Rust
//! Source: drivers/perf/arm-ni.c
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
// Copyright (C) 2022-2024 Arm Limited
// NI-700 Network-on-Chip PMU driver

// Common registers
pub const NI_NODE_TYPE: c_uint = 0x000;

pub const NI_CHILD_NODE_INFO: c_uint = 0x004;

pub const NI_NUM_SUB_FEATURES: c_uint = 0x100;

pub const NI_SUB_FEATURE_TYPE_FCU: c_uint = 0x2;
pub const NI700_PMUSELA: c_uint = 0x00c;
// Config node
pub const NI_PERIPHERAL_ID0: c_uint = 0xfe0;

pub const NI_PERIPHERAL_ID1: c_uint = 0xfe4;

pub const NI_PERIPHERAL_ID2: c_uint = 0xfe8;

// PMU node

pub const NI700_PMCCNTR_L: c_uint = 0x0f8;

pub const NI_PMCCNTR_L: c_uint = 0x2f8;

pub const NI_PMCNTENSET: c_uint = 0xc00;
pub const NI_PMCNTENCLR: c_uint = 0xc20;
pub const NI_PMINTENSET: c_uint = 0xc40;
pub const NI_PMINTENCLR: c_uint = 0xc60;
pub const NI_PMOVSCLR: c_uint = 0xc80;
pub const NI_PMOVSSET: c_uint = 0xcc0;
pub const NI_PMCFGR: c_uint = 0xe00;
pub const NI_PMCR: c_uint = 0xe04;

pub const NI_NUM_COUNTERS: c_int = 8;
pub const NI_CCNT_IDX: c_int = 31;
// Event attributes

    enum ni_part {
    PART_NI_700 = 0x43b,
    PART_NI_710AE = 0x43d,
    PART_NOC_S3 = 0x43f,
    PART_SI_L1 = 0x455,
    };
    enum ni_node_type {
    NI_GLOBAL,
    NI_VOLTAGE,
    NI_POWER,
    NI_CLOCK,
    NI_ASNI,
    NI_AMNI,
    NI_PMU,
    NI_HSNI,
    NI_HMNI,
    NI_PMNI,
    NI_TSNI,
    NI_TMNI,
    NI_CMNI = 0x0e,
    NI_MCN = 0x63,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_ni_node {
    pub base: *mut void __iomem,
    pub type: enum ni_node_type,
    pub id: u16,
    pub num_components: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_ni_unit {
    pub pmusela: *mut void __iomem,
    pub type: enum ni_node_type,
    pub id: u16,
    pub ns: bool,
    union {
    pub pmusel: __le64,
    pub event: [u8; 8],
}

    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_ni_cd {
    pub pmu_base: *mut void __iomem,
    pub id: u16,
    pub irq_friend: i8,
    pub num_units: c_int,
    pub irq: c_int,
    pub pmu: pmu,
    pub units: *mut arm_ni_unit,
    pub evcnt: [*mut perf_event; NI_NUM_COUNTERS],
    pub ccnt: *mut perf_event,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_ni {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub part: enum ni_part,
    pub id: c_int,
    pub cpu: c_int,
    pub num_cds: c_int,
    pub cpuhp_node: hlist_node,
    pub __counted_by(num_cds): arm_ni_cd cds[],
}

    for (struct arm_ni_cd *c = n.cds; c < n.cds + n.num_cds; c++) if (c.pmu_base)

    for (struct arm_ni_unit *u = cd.units; u < cd.units + cd.num_units; u++)
    static int arm_ni_hp_state;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_ni_event_attr {
    pub attr: device_attribute,
    pub type: enum ni_node_type,
}

    (&((struct arm_ni_event_attr[]) {{				\
    .attr = __ATTR(_name, 0444, arm_ni_event_show, core::ptr::null_mut()),	\
    .type = _type,						\
    }})[0].attr.attr)
    static ssize_t arm_ni_event_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct arm_ni_event_attr *eattr = container_of(attr, typeof(*eattr), attr);
    if (eattr.type == NI_PMU)
    return sysfs_emit(buf, "type=0x%x\n", eattr.type);
    return sysfs_emit(buf, "type=0x%x,eventid=?,nodeid=?\n", eattr.type);
    }
    static umode_t arm_ni_event_attr_is_visible(struct kobject *kobj,
    struct attribute *attr, int unused)
    {
    struct device *dev = kobj_to_dev(kobj);
    struct arm_ni_cd *cd = pmu_to_cd(dev_get_drvdata(dev));
    struct arm_ni_event_attr *eattr;
    eattr = container_of(attr, typeof(*eattr), attr.attr);
    cd_for_each_unit(cd, unit) {
    if (unit.type == eattr.type && unit.ns)
    return attr.mode;
    }
    return 0;
    }
    static struct attribute *arm_ni_event_attrs[] = {
    NI_EVENT_ATTR(asni, NI_ASNI),
    NI_EVENT_ATTR(amni, NI_AMNI),
    NI_EVENT_ATTR(cycles, NI_PMU),
    NI_EVENT_ATTR(hsni, NI_HSNI),
    NI_EVENT_ATTR(hmni, NI_HMNI),
    NI_EVENT_ATTR(pmni, NI_PMNI),
    NI_EVENT_ATTR(tsni, NI_TSNI),
    NI_EVENT_ATTR(tmni, NI_TMNI),
    NI_EVENT_ATTR(cmni, NI_CMNI),
    core::ptr::null_mut()
    };
    static const struct attribute_group arm_ni_event_attrs_group = {
    .name = "events",
    .attrs = arm_ni_event_attrs,
    .is_visible = arm_ni_event_attr_is_visible,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_ni_format_attr {
    pub attr: device_attribute,
    pub field: u64,
}

    (&((struct arm_ni_format_attr[]) {{				\
    .attr = __ATTR(_name, 0444, arm_ni_format_show, core::ptr::null_mut()),	\
    .field = _fld,						\
    }})[0].attr.attr)
    static ssize_t arm_ni_format_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct arm_ni_format_attr *fmt = container_of(attr, typeof(*fmt), attr);
    return sysfs_emit(buf, "config:%*pbl\n", 64, &fmt.field);
    }
    static struct attribute *arm_ni_format_attrs[] = {
    NI_FORMAT_ATTR(type, NI_CONFIG_TYPE),
    NI_FORMAT_ATTR(nodeid, NI_CONFIG_NODEID),
    NI_FORMAT_ATTR(eventid, NI_CONFIG_EVENTID),
    core::ptr::null_mut()
    };
    static const struct attribute_group arm_ni_format_attrs_group = {
    .name = "format",
    .attrs = arm_ni_format_attrs,
    };
    static ssize_t arm_ni_cpumask_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct arm_ni *ni = cd_to_ni(pmu_to_cd(dev_get_drvdata(dev)));
    return sysfs_emit(buf, "%*pbl\n", cpumask_pr_args(cpumask_of(ni.cpu)));
    }
    static struct device_attribute arm_ni_cpumask_attr =
    __ATTR(cpumask, 0444, arm_ni_cpumask_show, core::ptr::null_mut());
    static ssize_t arm_ni_identifier_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct arm_ni *ni = cd_to_ni(pmu_to_cd(dev_get_drvdata(dev)));
    let mut reg: u32 = readl_relaxed(ni.base + NI_PERIPHERAL_ID2);
    let mut version: c_int = FIELD_GET(NI_PIDR2_VERSION, reg);
    return sysfs_emit(buf, "%03x%02x\n", ni.part, version);
    }
    static struct device_attribute arm_ni_identifier_attr =
    __ATTR(identifier, 0444, arm_ni_identifier_show, core::ptr::null_mut());
    static struct attribute *arm_ni_other_attrs[] = {
    &arm_ni_cpumask_attr.attr,
    &arm_ni_identifier_attr.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group arm_ni_other_attr_group = {
    .attrs = arm_ni_other_attrs,
    };
    static const struct attribute_group *arm_ni_attr_groups[] = {
    &arm_ni_event_attrs_group,
    &arm_ni_format_attrs_group,
    &arm_ni_other_attr_group,
    core::ptr::null_mut()
    };
#[no_mangle]
unsafe extern "C" fn arm_ni_pmu_enable(pmu: *mut pmu) {
    static void arm_ni_pmu_enable(struct pmu *pmu)
    {
    writel_relaxed(NI_PMCR_ENABLE, pmu_to_cd(pmu).pmu_base + NI_PMCR);
    }
#[no_mangle]
unsafe extern "C" fn arm_ni_pmu_disable(pmu: *mut pmu) {
    static void arm_ni_pmu_disable(struct pmu *pmu)
    {
    writel_relaxed(0, pmu_to_cd(pmu).pmu_base + NI_PMCR);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_ni_val {
    pub evcnt: c_uint,
    pub ccnt: c_uint,
}

#[no_mangle]
unsafe extern "C" fn arm_ni_val_count_event(evt: *mut perf_event, val: *mut arm_ni_val) -> bool {
    static bool arm_ni_val_count_event(struct perf_event *evt, struct arm_ni_val *val)
    {
    if (is_software_event(evt))
    return true;
    if (NI_EVENT_TYPE(evt) == NI_PMU) {
    val.ccnt++;
    return val.ccnt <= 1;
    }
    val.evcnt++;
    return val.evcnt <= NI_NUM_COUNTERS;
    }
#[no_mangle]
unsafe extern "C" fn arm_ni_validate_group(event: *mut perf_event) -> c_int {
    static int arm_ni_validate_group(struct perf_event *event)
    {
    struct perf_event *sibling, *leader = event.group_leader;
    let mut val: arm_ni_val = { 0 };
    if (leader == event)
    return 0;
    arm_ni_val_count_event(event, &val);
    if (!arm_ni_val_count_event(leader, &val))
    return -EINVAL;
    for_each_sibling_event(sibling, leader) {
    if (!arm_ni_val_count_event(sibling, &val))
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn arm_ni_is_7xx(ni: *const arm_ni) -> bool {
    static bool arm_ni_is_7xx(const struct arm_ni *ni)
    {
    return ni.part == PART_NI_700 || ni.part == PART_NI_710AE;
    }
#[no_mangle]
unsafe extern "C" fn arm_ni_event_init(event: *mut perf_event) -> c_int {
    static int arm_ni_event_init(struct perf_event *event)
    {
    struct arm_ni_cd *cd = pmu_to_cd(event.pmu);
    struct arm_ni *ni;
    if (event.attr.type != event.pmu.type)
    return -ENOENT;
    if (is_sampling_event(event))
    return -EINVAL;
    ni = cd_to_ni(cd);
    event.cpu = ni.cpu;
    event.hw.flags = arm_ni_is_7xx(ni);
    if (NI_EVENT_TYPE(event) == NI_PMU)
    return arm_ni_validate_group(event);
    cd_for_each_unit(cd, unit) {
    if (unit.type == NI_EVENT_TYPE(event) &&
    unit.id == NI_EVENT_NODEID(event) && unit.ns) {
    event.hw.config_base = (unsigned long)unit;
    return arm_ni_validate_group(event);
    }
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn arm_ni_read_ccnt(pmccntr: *mut void __iomem) -> u64 {
    static u64 arm_ni_read_ccnt(void __iomem *pmccntr)
    {
    u64 l, u_old, u_new;
    int retries = 3; /* 1st time unlucky, 2nd improbable, 3rd just broken */
    u_new = readl_relaxed(pmccntr + 4);
    do {
    u_old = u_new;
    l = readl_relaxed(pmccntr);
    u_new = readl_relaxed(pmccntr + 4);
    } while (u_new != u_old && --retries);
    WARN_ON(!retries);
    return (u_new << 32) | l;
    }
#[no_mangle]
unsafe extern "C" fn arm_ni_event_read(event: *mut perf_event) {
    static void arm_ni_event_read(struct perf_event *event)
    {
    struct hw_perf_event *hw = &event.hw;
    u64 count, prev;
    let mut ccnt: bool = hw.idx == NI_CCNT_IDX;
    do {
    prev = local64_read(&hw.prev_count);
    if (ccnt)
    count = arm_ni_read_ccnt((void __iomem *)event.hw.event_base);
    else
    count = readl_relaxed((void __iomem *)event.hw.event_base);
    } while (local64_cmpxchg(&hw.prev_count, prev, count) != prev);
    count -= prev;
    if (!ccnt)
    count = (u32)count;
    local64_add(count, &event.count);
    }
#[no_mangle]
unsafe extern "C" fn arm_ni_event_start(event: *mut perf_event, flags: c_int) {
    static void arm_ni_event_start(struct perf_event *event, int flags)
    {
    struct arm_ni_cd *cd = pmu_to_cd(event.pmu);
    writel_relaxed(1U << event.hw.idx, cd.pmu_base + NI_PMCNTENSET);
    }
#[no_mangle]
unsafe extern "C" fn arm_ni_event_stop(event: *mut perf_event, flags: c_int) {
    static void arm_ni_event_stop(struct perf_event *event, int flags)
    {
    struct arm_ni_cd *cd = pmu_to_cd(event.pmu);
    writel_relaxed(1U << event.hw.idx, cd.pmu_base + NI_PMCNTENCLR);
    if (flags & PERF_EF_UPDATE)
    arm_ni_event_read(event);
    }
#[no_mangle]
unsafe extern "C" fn arm_ni_init_ccnt(hw: *mut hw_perf_event) {
    static void arm_ni_init_ccnt(struct hw_perf_event *hw)
    {
    local64_set(&hw.prev_count, S64_MIN);
    lo_hi_writeq_relaxed(S64_MIN, (void __iomem *)hw.event_base);
    }
#[no_mangle]
unsafe extern "C" fn arm_ni_init_evcnt(hw: *mut hw_perf_event) {
    static void arm_ni_init_evcnt(struct hw_perf_event *hw)
    {
    local64_set(&hw.prev_count, S32_MIN);
    writel_relaxed(S32_MIN, (void __iomem *)hw.event_base);
    }
#[no_mangle]
unsafe extern "C" fn arm_ni_event_add(event: *mut perf_event, flags: c_int) -> c_int {
    static int arm_ni_event_add(struct perf_event *event, int flags)
    {
    struct arm_ni_cd *cd = pmu_to_cd(event.pmu);
    struct hw_perf_event *hw = &event.hw;
    struct arm_ni_unit *unit;
    let mut type: enum ni_node_type = NI_EVENT_TYPE(event);
    u32 reg;
    if (type == NI_PMU) {
    if (cd.ccnt)
    return -ENOSPC;
    hw.idx = NI_CCNT_IDX;
    hw.event_base = (unsigned long)cd.pmu_base +
    (hw.flags ? NI700_PMCCNTR_L : NI_PMCCNTR_L);
    cd.ccnt = event;
    arm_ni_init_ccnt(hw);
    } else {
    hw.idx = 0;
    while (cd.evcnt[hw.idx]) {
    if (++hw.idx == NI_NUM_COUNTERS)
    return -ENOSPC;
    }
    cd.evcnt[hw.idx] = event;
    unit = (void *)hw.config_base;
    unit.event[hw.idx] = NI_EVENT_EVENTID(event);
    hw.event_base = (unsigned long)cd.pmu_base +
    (hw.flags ? NI700_PMEVCNTR(hw.idx) : NI_PMEVCNTR(hw.idx));
    arm_ni_init_evcnt(hw);
    lo_hi_writeq_relaxed(le64_to_cpu(unit.pmusel), unit.pmusela);
    reg = FIELD_PREP(NI_PMEVTYPER_NODE_TYPE, type) |
    FIELD_PREP(NI_PMEVTYPER_NODE_ID, NI_EVENT_NODEID(event));
    writel_relaxed(reg, cd.pmu_base + NI_PMEVTYPER(hw.idx));
    }
    if (flags & PERF_EF_START)
    arm_ni_event_start(event, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn arm_ni_event_del(event: *mut perf_event, flags: c_int) {
    static void arm_ni_event_del(struct perf_event *event, int flags)
    {
    struct arm_ni_cd *cd = pmu_to_cd(event.pmu);
    struct hw_perf_event *hw = &event.hw;
    arm_ni_event_stop(event, PERF_EF_UPDATE);
    if (hw.idx == NI_CCNT_IDX)
    cd.ccnt = core::ptr::null_mut();
    else
    cd.evcnt[hw.idx] = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn arm_ni_handle_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t arm_ni_handle_irq(int irq, void *dev_id)
    {
    struct arm_ni_cd *cd = dev_id;
    let mut ret: irqreturn_t = IRQ_NONE;
    for (;;) {
    let mut reg: u32 = readl_relaxed(cd.pmu_base + NI_PMOVSCLR);
    if (reg & (1U << NI_CCNT_IDX)) {
    ret = IRQ_HANDLED;
    if (!(WARN_ON(!cd.ccnt))) {
    arm_ni_event_read(cd.ccnt);
    arm_ni_init_ccnt(&cd.ccnt.hw);
    }
    }
    for (int i = 0; i < NI_NUM_COUNTERS; i++) {
    if (!(reg & (1U << i)))
    continue;
    ret = IRQ_HANDLED;
    if (!(WARN_ON(!cd.evcnt[i]))) {
    arm_ni_event_read(cd.evcnt[i]);
    arm_ni_init_evcnt(&cd.evcnt[i].hw);
    }
    }
    writel_relaxed(reg, cd.pmu_base + NI_PMOVSCLR);
    if (!cd.irq_friend)
    return ret;
    cd += cd.irq_friend;
    }
    }
    static void __iomem *arm_ni_get_pmusel(struct arm_ni *ni, void __iomem *unit_base)
    {
    u32 type, ptr, num;
    if (arm_ni_is_7xx(ni))
    return unit_base + NI700_PMUSELA;
    num = readl_relaxed(unit_base + NI_NUM_SUB_FEATURES);
    for (int i = 0; i < num; i++) {
    type = readl_relaxed(unit_base + NI_SUB_FEATURE_TYPE(i));
    if (type != NI_SUB_FEATURE_TYPE_FCU)
    continue;
    ptr = readl_relaxed(unit_base + NI_SUB_FEATURE_PTR(i));
    return ni.base + ptr;
    }
// Should be impossible
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn arm_ni_init_cd(ni: *mut arm_ni, node: *mut arm_ni_node, res_start: u64) -> c_int {
    static int arm_ni_init_cd(struct arm_ni *ni, struct arm_ni_node *node, u64 res_start)
    {
    struct arm_ni_cd *cd = ni.cds + node.id;
    const char *name;
    cd.id = node.id;
    cd.num_units = node.num_components;
    cd.units = devm_kcalloc(ni.dev, cd.num_units, sizeof(*(cd.units)), GFP_KERNEL);
    if (!cd.units)
    return -ENOMEM;
    for (int i = 0; i < cd.num_units; i++) {
    let mut reg: u32 = readl_relaxed(node.base + NI_CHILD_PTR(i));
    void __iomem *unit_base = ni.base + reg;
    struct arm_ni_unit *unit = cd.units + i;
    reg = readl_relaxed(unit_base + NI_NODE_TYPE);
    unit.type = FIELD_GET(NI_NODE_TYPE_NODE_TYPE, reg);
    unit.id = FIELD_GET(NI_NODE_TYPE_NODE_ID, reg);
    switch (unit.type) {
    case NI_PMU:
    reg = readl_relaxed(unit_base + NI_PMCFGR);
    if (!reg) {
    dev_info(ni.dev, "No access to PMU %d\n", cd.id);
    devm_kfree(ni.dev, cd.units);
    return 0;
    }
    unit.ns = true;
    cd.pmu_base = unit_base;
    break;
    case NI_ASNI:
    case NI_AMNI:
    case NI_HSNI:
    case NI_HMNI:
    case NI_PMNI:
    case NI_TSNI:
    case NI_TMNI:
    case NI_CMNI:
    unit.pmusela = arm_ni_get_pmusel(ni, unit_base);
    writel_relaxed(1, unit.pmusela);
    if (readl_relaxed(unit.pmusela) != 1)
    dev_info(ni.dev, "No access to node 0x%04x%04x\n", unit.id, unit.type);
    else
    unit.ns = true;
    break;
    case NI_MCN:
    break;
    default:
//
// e.g. FMU - thankfully bits 3:2 of FMU_ERR_FR0 are RES0 so
// can't alias any of the leaf node types we're looking for.
//
    dev_dbg(ni.dev, "Mystery node 0x%04x%04x\n", unit.id, unit.type);
    break;
    }
    }
    res_start += cd.pmu_base - ni.base;
    if (!devm_request_mem_region(ni.dev, res_start, SZ_4K, dev_name(ni.dev))) {
    dev_err(ni.dev, "Failed to request PMU region 0x%llx\n", res_start);
    return -EBUSY;
    }
    writel_relaxed(NI_PMCR_RESET_CCNT | NI_PMCR_RESET_EVCNT,
    cd.pmu_base + NI_PMCR);
    writel_relaxed(U32_MAX, cd.pmu_base + NI_PMCNTENCLR);
    writel_relaxed(U32_MAX, cd.pmu_base + NI_PMOVSCLR);
    cd.irq = platform_get_irq(to_platform_device(ni.dev), cd.id);
    if (cd.irq < 0)
    return cd.irq;
    cd.pmu = (struct pmu) {
    .module = THIS_MODULE,
    .parent = ni.dev,
    .attr_groups = arm_ni_attr_groups,
    .capabilities = PERF_PMU_CAP_NO_EXCLUDE,
    .task_ctx_nr = perf_invalid_context,
    .pmu_enable = arm_ni_pmu_enable,
    .pmu_disable = arm_ni_pmu_disable,
    .event_init = arm_ni_event_init,
    .add = arm_ni_event_add,
    .del = arm_ni_event_del,
    .start = arm_ni_event_start,
    .stop = arm_ni_event_stop,
    .read = arm_ni_event_read,
    };
    name = devm_kasprintf(ni.dev, GFP_KERNEL, "arm_ni_%d_cd_%d", ni.id, cd.id);
    if (!name)
    return -ENOMEM;
    return perf_pmu_register(&cd.pmu, name, -1);
    }
#[no_mangle]
unsafe extern "C" fn arm_ni_remove(pdev: *mut platform_device) {
    static void arm_ni_remove(struct platform_device *pdev)
    {
    struct arm_ni *ni = platform_get_drvdata(pdev);
    ni_for_each_cd(ni, cd) {
    writel_relaxed(0, cd.pmu_base + NI_PMCR);
    writel_relaxed(U32_MAX, cd.pmu_base + NI_PMINTENCLR);
    perf_pmu_unregister(&cd.pmu);
    }
    cpuhp_state_remove_instance_nocalls(arm_ni_hp_state, &ni.cpuhp_node);
    }
#[no_mangle]
unsafe extern "C" fn arm_ni_probe_domain(base: *mut void __iomem, node: *mut arm_ni_node) {
    static void arm_ni_probe_domain(void __iomem *base, struct arm_ni_node *node)
    {
    let mut reg: u32 = readl_relaxed(base + NI_NODE_TYPE);
    node.base = base;
    node.type = FIELD_GET(NI_NODE_TYPE_NODE_TYPE, reg);
    node.id = FIELD_GET(NI_NODE_TYPE_NODE_ID, reg);
    node.num_components = readl_relaxed(base + NI_CHILD_NODE_INFO);
    }
#[no_mangle]
unsafe extern "C" fn arm_ni_init_irqs(ni: *mut arm_ni) -> c_int {
    static int arm_ni_init_irqs(struct arm_ni *ni)
    {
    int err;
    ni_for_each_cd(ni, cd) {
    for (struct arm_ni_cd *prev = cd; prev-- > ni.cds; ) {
    if (prev.irq == cd.irq) {
    prev.irq_friend = cd - prev;
    goto set_inten;
    }
    }
    err = devm_request_irq(ni.dev, cd.irq, arm_ni_handle_irq,
    IRQF_NOBALANCING | IRQF_NO_THREAD | IRQF_NO_AUTOEN,
    dev_name(ni.dev), cd);
    if (err)
    return err;
    irq_set_affinity(cd.irq, cpumask_of(ni.cpu));
    set_inten:
    writel_relaxed(U32_MAX, cd.pmu_base + NI_PMINTENSET);
    }
    ni_for_each_cd(ni, cd)
    if (!cd.irq_friend)
    enable_irq(cd.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn arm_ni_probe(pdev: *mut platform_device) -> c_int {
    static int arm_ni_probe(struct platform_device *pdev)
    {
    struct arm_ni_node cfg, vd, pd, cd;
    struct arm_ni *ni;
    struct resource *res;
    void __iomem *base;
    static atomic_t id;
    int ret, num_cds;
    u32 reg, part;
//
// We want to map the whole configuration space for ease of discovery,
// but the PMU pages are the only ones for which we can honestly claim
// exclusive ownership, so we'll request them explicitly once found.
//
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    base = devm_ioremap(&pdev.dev, res.start, resource_size(res));
    if (!base)
    return -ENOMEM;
    arm_ni_probe_domain(base, &cfg);
    if (cfg.type != NI_GLOBAL)
    return -ENODEV;
    reg = readl_relaxed(cfg.base + NI_PERIPHERAL_ID0);
    part = FIELD_GET(NI_PIDR0_PART_7_0, reg);
    reg = readl_relaxed(cfg.base + NI_PERIPHERAL_ID1);
    part |= FIELD_GET(NI_PIDR1_PART_11_8, reg) << 8;
    switch (part) {
    case PART_NI_700:
    case PART_NI_710AE:
    case PART_NOC_S3:
    case PART_SI_L1:
    break;
    default:
    dev_WARN(&pdev.dev, "Unknown part number: 0x%03x, this may go badly\n", part);
    break;
    }
    num_cds = 0;
    for (int v = 0; v < cfg.num_components; v++) {
    reg = readl_relaxed(cfg.base + NI_CHILD_PTR(v));
    arm_ni_probe_domain(base + reg, &vd);
    for (int p = 0; p < vd.num_components; p++) {
    reg = readl_relaxed(vd.base + NI_CHILD_PTR(p));
    arm_ni_probe_domain(base + reg, &pd);
    num_cds += pd.num_components;
    }
    }
    ni = devm_kzalloc(&pdev.dev, struct_size(ni, cds, num_cds), GFP_KERNEL);
    if (!ni)
    return -ENOMEM;
    ni.dev = &pdev.dev;
    ni.base = base;
    ni.num_cds = num_cds;
    ni.part = part;
    ni.id = atomic_fetch_inc(&id);
    ni.cpu = cpumask_local_spread(0, dev_to_node(ni.dev));
    platform_set_drvdata(pdev, ni);
    ret = cpuhp_state_add_instance_nocalls(arm_ni_hp_state, &ni.cpuhp_node);
    if (ret)
    return ret;
    for (int v = 0; v < cfg.num_components; v++) {
    reg = readl_relaxed(cfg.base + NI_CHILD_PTR(v));
    arm_ni_probe_domain(base + reg, &vd);
    for (int p = 0; p < vd.num_components; p++) {
    reg = readl_relaxed(vd.base + NI_CHILD_PTR(p));
    arm_ni_probe_domain(base + reg, &pd);
    for (int c = 0; c < pd.num_components; c++) {
    reg = readl_relaxed(pd.base + NI_CHILD_PTR(c));
    arm_ni_probe_domain(base + reg, &cd);
    ret = arm_ni_init_cd(ni, &cd, res.start);
    if (ret) {
    ni.cds[cd.id].pmu_base = core::ptr::null_mut();
    arm_ni_remove(pdev);
    return ret;
    }
    }
    }
    }
    ret = arm_ni_init_irqs(ni);
    if (ret)
    arm_ni_remove(pdev);
    return ret;
    }

    static const struct of_device_id arm_ni_of_match[] = {
    { .compatible = "arm,ni-700" },
    {}
    };
    MODULE_DEVICE_TABLE(of, arm_ni_of_match);

    static const struct acpi_device_id arm_ni_acpi_match[] = {
    { "ARMHCB70" },
    {}
    };
    MODULE_DEVICE_TABLE(acpi, arm_ni_acpi_match);

    static struct platform_driver arm_ni_driver = {
    .driver = {
    .name = "arm-ni",
    .of_match_table = of_match_ptr(arm_ni_of_match),
    .acpi_match_table = ACPI_PTR(arm_ni_acpi_match),
    .suppress_bind_attrs = true,
    },
    .probe = arm_ni_probe,
    .remove = arm_ni_remove,
    };
#[no_mangle]
unsafe extern "C" fn arm_ni_pmu_migrate(ni: *mut arm_ni, cpu: c_uint) {
    static void arm_ni_pmu_migrate(struct arm_ni *ni, unsigned int cpu)
    {
    ni_for_each_cd(ni, cd) {
    perf_pmu_migrate_context(&cd.pmu, ni.cpu, cpu);
    irq_set_affinity(cd.irq, cpumask_of(cpu));
    }
    ni.cpu = cpu;
    }
#[no_mangle]
unsafe extern "C" fn arm_ni_pmu_online_cpu(cpu: c_uint, cpuhp_node: *mut hlist_node) -> c_int {
    static int arm_ni_pmu_online_cpu(unsigned int cpu, struct hlist_node *cpuhp_node)
    {
    struct arm_ni *ni;
    int node;
    ni = hlist_entry_safe(cpuhp_node, struct arm_ni, cpuhp_node);
    node = dev_to_node(ni.dev);
    if (cpu_to_node(ni.cpu) != node && cpu_to_node(cpu) == node)
    arm_ni_pmu_migrate(ni, cpu);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn arm_ni_pmu_offline_cpu(cpu: c_uint, cpuhp_node: *mut hlist_node) -> c_int {
    static int arm_ni_pmu_offline_cpu(unsigned int cpu, struct hlist_node *cpuhp_node)
    {
    struct arm_ni *ni;
    unsigned int target;
    int node;
    ni = hlist_entry_safe(cpuhp_node, struct arm_ni, cpuhp_node);
    if (cpu != ni.cpu)
    return 0;
    node = dev_to_node(ni.dev);
    target = cpumask_any_and_but(cpumask_of_node(node), cpu_online_mask, cpu);
    if (target >= nr_cpu_ids)
    target = cpumask_any_but(cpu_online_mask, cpu);
    if (target < nr_cpu_ids)
    arm_ni_pmu_migrate(ni, target);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn arm_ni_init() -> int __init {
    static int __init arm_ni_init(void)
    {
    int ret;
    ret = cpuhp_setup_state_multi(CPUHP_AP_ONLINE_DYN,
    "perf/arm/ni:online",
    arm_ni_pmu_online_cpu,
    arm_ni_pmu_offline_cpu);
    if (ret < 0)
    return ret;
    arm_ni_hp_state = ret;
    ret = platform_driver_register(&arm_ni_driver);
    if (ret)
    cpuhp_remove_multi_state(arm_ni_hp_state);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn arm_ni_exit() -> void __exit {
    static void __exit arm_ni_exit(void)
    {
    platform_driver_unregister(&arm_ni_driver);
    cpuhp_remove_multi_state(arm_ni_hp_state);
    }
    module_init(arm_ni_init);
    module_exit(arm_ni_exit);
    MODULE_AUTHOR("Robin Murphy <robin.murphy@arm.com>");
    MODULE_DESCRIPTION("Arm NI-700 PMU driver");
    MODULE_LICENSE("GPL v2");
