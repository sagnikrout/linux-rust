//! Automatically rewritten from C to Rust
//! Source: drivers/perf/arm_dsu_pmu.c
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
// ARM DynamIQ Shared Unit (DSU) PMU driver
//
// Copyright (C) ARM Limited, 2017.
//
// Based on ARM CCI-PMU, ARMv8 PMU-v3 drivers.
//

// PMU event codes
pub const DSU_PMU_EVT_CYCLES: c_uint = 0x11;
pub const DSU_PMU_EVT_CHAIN: c_uint = 0x1e;
pub const DSU_PMU_MAX_COMMON_EVENTS: c_uint = 0x40;
pub const DSU_PMU_MAX_HW_CNTRS: c_int = 32;

pub const CLUSTERPMCR_N_SHIFT: c_int = 11;
pub const CLUSTERPMCR_N_MASK: c_uint = 0x1f;
pub const CLUSTERPMCR_IDCODE_SHIFT: c_int = 16;
pub const CLUSTERPMCR_IDCODE_MASK: c_uint = 0xff;
pub const CLUSTERPMCR_IMP_SHIFT: c_int = 24;
pub const CLUSTERPMCR_IMP_MASK: c_uint = 0xff;
pub const CLUSTERPMCR_RES_MASK: c_uint = 0x7e8;
pub const CLUSTERPMCR_RES_VAL: c_uint = 0x40;
pub const DSU_ACTIVE_CPU_MASK: c_uint = 0x0;
pub const DSU_ASSOCIATED_CPU_MASK: c_uint = 0x1;
//
// We use the index of the counters as they appear in the counter
// bit maps in the PMU registers (e.g CLUSTERPMSELR).
// i.e,
// counter 0	- Bit 0
// counter 1	- Bit 1
// ...
// Cycle counter	- Bit 31
//
pub const DSU_PMU_IDX_CYCLE_COUNTER: c_int = 31;

    (&((struct dev_ext_attribute[]) {				\
    {							\
    .attr = __ATTR(_name, 0444, _func, core::ptr::null_mut()),	\
    .var = (void *)_config				\
    }							\
    })[0].attr.attr)

    DSU_EXT_ATTR(_name, dsu_pmu_sysfs_event_show, (unsigned long)_config)

    DSU_EXT_ATTR(_name, device_show_string, _config)

    DSU_EXT_ATTR(_name, dsu_pmu_cpumask_show, (unsigned long)_config)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsu_hw_events {
    pub DSU_PMU_MAX_HW_CNTRS): DECLARE_BITMAP(used_mask,,
    pub events: [*mut perf_event; DSU_PMU_MAX_HW_CNTRS],
}

//
// struct dsu_pmu	- DSU PMU descriptor
//
// @pmu_lock		: Protects accesses to DSU PMU register from normal vs
// interrupt handler contexts.
// @hw_events		: Holds the event counter state.
// @associated_cpus	: CPUs attached to the DSU.
// @active_cpu		: CPU to which the PMU is bound for accesses.
// @cpuhp_node		: Node for CPU hotplug notifier link.
// @num_counters	: Number of event counters implemented by the PMU,
// excluding the cycle counter.
// @irq			: Interrupt line for counter overflow.
// @has_32b_pmevcntr	: Are the non-cycle counters only 32-bit?
// @has_pmccntr		: Do we even have a dedicated cycle counter?
// @cpmceid_bitmap	: Bitmap for the availability of architected common
// events (event_code < 0x40).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsu_pmu {
    pub pmu: pmu,
    pub dev: *mut device,
    pub pmu_lock: raw_spinlock_t,
    pub hw_events: dsu_hw_events,
    pub associated_cpus: cpumask_t,
    pub active_cpu: cpumask_t,
    pub cpuhp_node: hlist_node,
    pub num_counters: i8,
    pub irq: c_int,
    pub has_32b_pmevcntr: bool,
    pub has_pmccntr: bool,
    pub DSU_PMU_MAX_COMMON_EVENTS): DECLARE_BITMAP(cpmceid_bitmap,,
}

    static unsigned long dsu_pmu_cpuhp_state;
    static inline struct dsu_pmu *to_dsu_pmu(struct pmu *pmu)
    {
    return container_of(pmu, struct dsu_pmu, pmu);
    }
    static ssize_t dsu_pmu_sysfs_event_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct dev_ext_attribute *eattr = container_of(attr,
    struct dev_ext_attribute, attr);
    return sysfs_emit(buf, "event=0x%lx\n", (unsigned long)eattr.var);
    }
    static ssize_t dsu_pmu_cpumask_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct pmu *pmu = dev_get_drvdata(dev);
    struct dsu_pmu *dsu_pmu = to_dsu_pmu(pmu);
    struct dev_ext_attribute *eattr = container_of(attr,
    struct dev_ext_attribute, attr);
    let mut mask_id: c_ulong = (unsigned long)eattr.var;
    const cpumask_t *cpumask;
    switch (mask_id) {
    case DSU_ACTIVE_CPU_MASK:
    cpumask = &dsu_pmu.active_cpu;
    break;
    case DSU_ASSOCIATED_CPU_MASK:
    cpumask = &dsu_pmu.associated_cpus;
    break;
    default:
    return 0;
    }
    return sysfs_emit(buf, "%*pbl\n", cpumask_pr_args(cpumask));
    }
    static struct attribute *dsu_pmu_format_attrs[] = {
    DSU_FORMAT_ATTR(event, "config:0-31"),
    core::ptr::null_mut(),
    };
    static const struct attribute_group dsu_pmu_format_attr_group = {
    .name = "format",
    .attrs = dsu_pmu_format_attrs,
    };
    static struct attribute *dsu_pmu_event_attrs[] = {
    DSU_EVENT_ATTR(cycles, 0x11),
    DSU_EVENT_ATTR(bus_access, 0x19),
    DSU_EVENT_ATTR(memory_error, 0x1a),
    DSU_EVENT_ATTR(bus_cycles, 0x1d),
    DSU_EVENT_ATTR(l3d_cache_allocate, 0x29),
    DSU_EVENT_ATTR(l3d_cache_refill, 0x2a),
    DSU_EVENT_ATTR(l3d_cache, 0x2b),
    DSU_EVENT_ATTR(l3d_cache_wb, 0x2c),
    core::ptr::null_mut(),
    };
    static umode_t
    dsu_pmu_event_attr_is_visible(struct kobject *kobj, struct attribute *attr,
    int unused)
    {
    struct pmu *pmu = dev_get_drvdata(kobj_to_dev(kobj));
    struct dsu_pmu *dsu_pmu = to_dsu_pmu(pmu);
    struct dev_ext_attribute *eattr = container_of(attr,
    struct dev_ext_attribute, attr.attr);
    let mut evt: c_ulong = (unsigned long)eattr.var;
    return test_bit(evt, dsu_pmu.cpmceid_bitmap) ? attr.mode : 0;
    }
    static const struct attribute_group dsu_pmu_events_attr_group = {
    .name = "events",
    .attrs = dsu_pmu_event_attrs,
    .is_visible = dsu_pmu_event_attr_is_visible,
    };
    static struct attribute *dsu_pmu_cpumask_attrs[] = {
    DSU_CPUMASK_ATTR(cpumask, DSU_ACTIVE_CPU_MASK),
    DSU_CPUMASK_ATTR(associated_cpus, DSU_ASSOCIATED_CPU_MASK),
    core::ptr::null_mut(),
    };
    static const struct attribute_group dsu_pmu_cpumask_attr_group = {
    .attrs = dsu_pmu_cpumask_attrs,
    };
    static const struct attribute_group *dsu_pmu_attr_groups[] = {
    &dsu_pmu_cpumask_attr_group,
    &dsu_pmu_events_attr_group,
    &dsu_pmu_format_attr_group,
    core::ptr::null_mut(),
    };
#[no_mangle]
pub unsafe extern "C" fn dsu_pmu_counter_valid(dsu_pmu: *mut dsu_pmu, idx: u32) -> bool {
    static inline bool dsu_pmu_counter_valid(struct dsu_pmu *dsu_pmu, u32 idx)
    {
    return (idx < dsu_pmu.num_counters) ||
    (idx == DSU_PMU_IDX_CYCLE_COUNTER);
    }
#[no_mangle]
pub unsafe extern "C" fn dsu_pmu_read_counter(event: *mut perf_event) -> u64 {
    static inline u64 dsu_pmu_read_counter(struct perf_event *event)
    {
    u64 val;
    unsigned long flags;
    struct dsu_pmu *dsu_pmu = to_dsu_pmu(event.pmu);
    let mut idx: c_int = event.hw.idx;
    if (WARN_ON(!cpumask_test_cpu(smp_processor_id(),
    &dsu_pmu.associated_cpus)))
    return 0;
    if (!dsu_pmu_counter_valid(dsu_pmu, idx)) {
    dev_err(event.pmu.dev,
    "Trying reading invalid counter %d\n", idx);
    return 0;
    }
    raw_spin_lock_irqsave(&dsu_pmu.pmu_lock, flags);
    if (idx == DSU_PMU_IDX_CYCLE_COUNTER)
    val = __dsu_pmu_read_pmccntr();
    else
    val = __dsu_pmu_read_counter(idx);
    raw_spin_unlock_irqrestore(&dsu_pmu.pmu_lock, flags);
    return val;
    }
#[no_mangle]
unsafe extern "C" fn dsu_pmu_write_counter(event: *mut perf_event, val: u64) {
    static void dsu_pmu_write_counter(struct perf_event *event, u64 val)
    {
    unsigned long flags;
    struct dsu_pmu *dsu_pmu = to_dsu_pmu(event.pmu);
    let mut idx: c_int = event.hw.idx;
    if (WARN_ON(!cpumask_test_cpu(smp_processor_id(),
    &dsu_pmu.associated_cpus)))
    return;
    if (!dsu_pmu_counter_valid(dsu_pmu, idx)) {
    dev_err(event.pmu.dev,
    "writing to invalid counter %d\n", idx);
    return;
    }
    raw_spin_lock_irqsave(&dsu_pmu.pmu_lock, flags);
    if (idx == DSU_PMU_IDX_CYCLE_COUNTER)
    __dsu_pmu_write_pmccntr(val);
    else
    __dsu_pmu_write_counter(idx, val);
    raw_spin_unlock_irqrestore(&dsu_pmu.pmu_lock, flags);
    }
    static int dsu_pmu_get_event_idx(struct dsu_hw_events *hw_events,
    struct perf_event *event)
    {
    int idx;
    let mut evtype: c_ulong = event.attr.config;
    struct dsu_pmu *dsu_pmu = to_dsu_pmu(event.pmu);
    unsigned long *used_mask = hw_events.used_mask;
    if (evtype == DSU_PMU_EVT_CYCLES && dsu_pmu.has_pmccntr) {
    if (!test_and_set_bit(DSU_PMU_IDX_CYCLE_COUNTER, used_mask))
    return DSU_PMU_IDX_CYCLE_COUNTER;
    }
    idx = find_first_zero_bit(used_mask, dsu_pmu.num_counters);
    if (idx >= dsu_pmu.num_counters)
    return -EAGAIN;
    set_bit(idx, hw_events.used_mask);
    return idx;
    }
#[no_mangle]
unsafe extern "C" fn dsu_pmu_enable_counter(dsu_pmu: *mut dsu_pmu, idx: c_int) {
    static void dsu_pmu_enable_counter(struct dsu_pmu *dsu_pmu, int idx)
    {
    __dsu_pmu_counter_interrupt_enable(idx);
    __dsu_pmu_enable_counter(idx);
    }
#[no_mangle]
unsafe extern "C" fn dsu_pmu_disable_counter(dsu_pmu: *mut dsu_pmu, idx: c_int) {
    static void dsu_pmu_disable_counter(struct dsu_pmu *dsu_pmu, int idx)
    {
    __dsu_pmu_disable_counter(idx);
    __dsu_pmu_counter_interrupt_disable(idx);
    }
    static inline void dsu_pmu_set_event(struct dsu_pmu *dsu_pmu,
    struct perf_event *event)
    {
    let mut idx: c_int = event.hw.idx;
    unsigned long flags;
    if (!dsu_pmu_counter_valid(dsu_pmu, idx)) {
    dev_err(event.pmu.dev,
    "Trying to set invalid counter %d\n", idx);
    return;
    }
    raw_spin_lock_irqsave(&dsu_pmu.pmu_lock, flags);
    __dsu_pmu_set_event(idx, event.hw.config_base);
    raw_spin_unlock_irqrestore(&dsu_pmu.pmu_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn dsu_pmu_counter_mask(hw: *mut hw_perf_event) -> u64 {
    static u64 dsu_pmu_counter_mask(struct hw_perf_event *hw)
    {
    return (hw.flags && hw.idx != DSU_PMU_IDX_CYCLE_COUNTER) ? U32_MAX : U64_MAX;
    }
#[no_mangle]
unsafe extern "C" fn dsu_pmu_event_update(event: *mut perf_event) {
    static void dsu_pmu_event_update(struct perf_event *event)
    {
    struct hw_perf_event *hwc = &event.hw;
    u64 delta, prev_count, new_count;
    do {
// We may also be called from the irq handler
    prev_count = local64_read(&hwc.prev_count);
    new_count = dsu_pmu_read_counter(event);
    } while (local64_cmpxchg(&hwc.prev_count, prev_count, new_count) !=
    prev_count);
    delta = (new_count - prev_count) & dsu_pmu_counter_mask(hwc);
    local64_add(delta, &event.count);
    }
#[no_mangle]
unsafe extern "C" fn dsu_pmu_read(event: *mut perf_event) {
    static void dsu_pmu_read(struct perf_event *event)
    {
    dsu_pmu_event_update(event);
    }
#[no_mangle]
pub unsafe extern "C" fn dsu_pmu_get_reset_overflow() -> u32 {
    static inline u32 dsu_pmu_get_reset_overflow(void)
    {
    return __dsu_pmu_get_reset_overflow();
    }
//
// dsu_pmu_set_event_period: Set the period for the counter.
//
// All DSU PMU event counters, except the cycle counter are 32bit
// counters. To handle cases of extreme interrupt latency, we program
// the counter with half of the max count for the counters.
//
#[no_mangle]
unsafe extern "C" fn dsu_pmu_set_event_period(event: *mut perf_event) {
    static void dsu_pmu_set_event_period(struct perf_event *event)
    {
    let mut val: u64 = dsu_pmu_counter_mask(&event.hw) >> 1;
    local64_set(&event.hw.prev_count, val);
    dsu_pmu_write_counter(event, val);
    }
#[no_mangle]
unsafe extern "C" fn dsu_pmu_handle_irq(irq_num: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t dsu_pmu_handle_irq(int irq_num, void *dev)
    {
    int i;
    let mut handled: bool = false;
    struct dsu_pmu *dsu_pmu = dev;
    struct dsu_hw_events *hw_events = &dsu_pmu.hw_events;
    unsigned long overflow;
    overflow = dsu_pmu_get_reset_overflow();
    if (!overflow)
    return IRQ_NONE;
    for_each_set_bit(i, &overflow, DSU_PMU_MAX_HW_CNTRS) {
    struct perf_event *event = hw_events.events[i];
    if (!event)
    continue;
    dsu_pmu_event_update(event);
    dsu_pmu_set_event_period(event);
    handled = true;
    }
    return IRQ_RETVAL(handled);
    }
#[no_mangle]
unsafe extern "C" fn dsu_pmu_start(event: *mut perf_event, pmu_flags: c_int) {
    static void dsu_pmu_start(struct perf_event *event, int pmu_flags)
    {
    struct dsu_pmu *dsu_pmu = to_dsu_pmu(event.pmu);
// We always reprogram the counter
    if (pmu_flags & PERF_EF_RELOAD)
    WARN_ON(!(event.hw.state & PERF_HES_UPTODATE));
    dsu_pmu_set_event_period(event);
    if (event.hw.idx != DSU_PMU_IDX_CYCLE_COUNTER)
    dsu_pmu_set_event(dsu_pmu, event);
    event.hw.state = 0;
    dsu_pmu_enable_counter(dsu_pmu, event.hw.idx);
    }
#[no_mangle]
unsafe extern "C" fn dsu_pmu_stop(event: *mut perf_event, pmu_flags: c_int) {
    static void dsu_pmu_stop(struct perf_event *event, int pmu_flags)
    {
    struct dsu_pmu *dsu_pmu = to_dsu_pmu(event.pmu);
    if (event.hw.state & PERF_HES_STOPPED)
    return;
    dsu_pmu_disable_counter(dsu_pmu, event.hw.idx);
    dsu_pmu_event_update(event);
    event.hw.state |= PERF_HES_STOPPED | PERF_HES_UPTODATE;
    }
#[no_mangle]
unsafe extern "C" fn dsu_pmu_add(event: *mut perf_event, flags: c_int) -> c_int {
    static int dsu_pmu_add(struct perf_event *event, int flags)
    {
    struct dsu_pmu *dsu_pmu = to_dsu_pmu(event.pmu);
    struct dsu_hw_events *hw_events = &dsu_pmu.hw_events;
    struct hw_perf_event *hwc = &event.hw;
    int idx;
    if (WARN_ON_ONCE(!cpumask_test_cpu(smp_processor_id(),
    &dsu_pmu.associated_cpus)))
    return -ENOENT;
    idx = dsu_pmu_get_event_idx(hw_events, event);
    if (idx < 0)
    return idx;
    hwc.idx = idx;
    hw_events.events[idx] = event;
    hwc.state = PERF_HES_STOPPED | PERF_HES_UPTODATE;
    if (flags & PERF_EF_START)
    dsu_pmu_start(event, PERF_EF_RELOAD);
    perf_event_update_userpage(event);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dsu_pmu_del(event: *mut perf_event, flags: c_int) {
    static void dsu_pmu_del(struct perf_event *event, int flags)
    {
    struct dsu_pmu *dsu_pmu = to_dsu_pmu(event.pmu);
    struct dsu_hw_events *hw_events = &dsu_pmu.hw_events;
    struct hw_perf_event *hwc = &event.hw;
    let mut idx: c_int = hwc.idx;
    dsu_pmu_stop(event, PERF_EF_UPDATE);
    hw_events.events[idx] = core::ptr::null_mut();
    clear_bit(idx, hw_events.used_mask);
    perf_event_update_userpage(event);
    }
#[no_mangle]
unsafe extern "C" fn dsu_pmu_enable(pmu: *mut pmu) {
    static void dsu_pmu_enable(struct pmu *pmu)
    {
    u32 pmcr;
    unsigned long flags;
    struct dsu_pmu *dsu_pmu = to_dsu_pmu(pmu);
// If no counters are added, skip enabling the PMU
    if (bitmap_empty(dsu_pmu.hw_events.used_mask, DSU_PMU_MAX_HW_CNTRS))
    return;
    raw_spin_lock_irqsave(&dsu_pmu.pmu_lock, flags);
    pmcr = __dsu_pmu_read_pmcr();
    pmcr |= CLUSTERPMCR_E;
    __dsu_pmu_write_pmcr(pmcr);
    raw_spin_unlock_irqrestore(&dsu_pmu.pmu_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn dsu_pmu_disable(pmu: *mut pmu) {
    static void dsu_pmu_disable(struct pmu *pmu)
    {
    u32 pmcr;
    unsigned long flags;
    struct dsu_pmu *dsu_pmu = to_dsu_pmu(pmu);
    raw_spin_lock_irqsave(&dsu_pmu.pmu_lock, flags);
    pmcr = __dsu_pmu_read_pmcr();
    pmcr &= ~CLUSTERPMCR_E;
    __dsu_pmu_write_pmcr(pmcr);
    raw_spin_unlock_irqrestore(&dsu_pmu.pmu_lock, flags);
    }
    static bool dsu_pmu_validate_event(struct pmu *pmu,
    struct dsu_hw_events *hw_events,
    struct perf_event *event)
    {
    if (is_software_event(event))
    return true;
// Reject groups spanning multiple HW PMUs.
    if (event.pmu != pmu)
    return false;
    return dsu_pmu_get_event_idx(hw_events, event) >= 0;
    }
//
// Make sure the group of events can be scheduled at once
// on the PMU.
//
#[no_mangle]
unsafe extern "C" fn dsu_pmu_validate_group(event: *mut perf_event) -> bool {
    static bool dsu_pmu_validate_group(struct perf_event *event)
    {
    struct perf_event *sibling, *leader = event.group_leader;
    struct dsu_hw_events fake_hw;
    if (event.group_leader == event)
    return true;
    memset(fake_hw.used_mask, 0, sizeof(fake_hw.used_mask));
    if (!dsu_pmu_validate_event(event.pmu, &fake_hw, leader))
    return false;
    for_each_sibling_event(sibling, leader) {
    if (!dsu_pmu_validate_event(event.pmu, &fake_hw, sibling))
    return false;
    }
    return dsu_pmu_validate_event(event.pmu, &fake_hw, event);
    }
#[no_mangle]
unsafe extern "C" fn dsu_pmu_event_init(event: *mut perf_event) -> c_int {
    static int dsu_pmu_event_init(struct perf_event *event)
    {
    struct dsu_pmu *dsu_pmu = to_dsu_pmu(event.pmu);
    if (event.attr.type != event.pmu.type)
    return -ENOENT;
// We don't support sampling
    if (is_sampling_event(event)) {
    dev_dbg(dsu_pmu.pmu.dev, "Can't support sampling events\n");
    return -EOPNOTSUPP;
    }
// We cannot support task bound events
    if (event.cpu < 0 || event.attach_state & PERF_ATTACH_TASK) {
    dev_dbg(dsu_pmu.pmu.dev, "Can't support per-task counters\n");
    return -EINVAL;
    }
    if (has_branch_stack(event)) {
    dev_dbg(dsu_pmu.pmu.dev, "Can't support filtering\n");
    return -EINVAL;
    }
    if (!cpumask_test_cpu(event.cpu, &dsu_pmu.associated_cpus)) {
    dev_dbg(dsu_pmu.pmu.dev,
    "Requested cpu is not associated with the DSU\n");
    return -EINVAL;
    }
//
// Choose the current active CPU to read the events. We don't want
// to migrate the event contexts, irq handling etc to the requested
// CPU. As long as the requested CPU is within the same DSU, we
// are fine.
//
    event.cpu = cpumask_first(&dsu_pmu.active_cpu);
    if (event.cpu >= nr_cpu_ids)
    return -EINVAL;
    if (!dsu_pmu_validate_group(event))
    return -EINVAL;
    event.hw.config_base = event.attr.config;
    event.hw.flags = dsu_pmu.has_32b_pmevcntr;
    return 0;
    }
    static struct dsu_pmu *dsu_pmu_alloc(struct platform_device *pdev)
    {
    struct dsu_pmu *dsu_pmu;
    dsu_pmu = devm_kzalloc(&pdev.dev, sizeof(*dsu_pmu), GFP_KERNEL);
    if (!dsu_pmu)
    return ERR_PTR(-ENOMEM);
    raw_spin_lock_init(&dsu_pmu.pmu_lock);
//
// Initialise the number of counters to -1, until we probe
// the real number on a connected CPU.
//
    dsu_pmu.num_counters = -1;
    return dsu_pmu;
    }
//
// dsu_pmu_dt_get_cpus: Get the list of CPUs in the cluster
// from device tree.
//
#[no_mangle]
unsafe extern "C" fn dsu_pmu_dt_get_cpus(dev: *mut device, mask: *mut cpumask_t) -> c_int {
    static int dsu_pmu_dt_get_cpus(struct device *dev, cpumask_t *mask)
    {
    let mut i: c_int = 0, n, cpu;
    struct device_node *cpu_node;
    n = of_count_phandle_with_args(dev.of_node, "cpus", core::ptr::null_mut());
    if (n <= 0)
    return -ENODEV;
    for (; i < n; i++) {
    cpu_node = of_parse_phandle(dev.of_node, "cpus", i);
    if (!cpu_node)
    break;
    cpu = of_cpu_node_to_id(cpu_node);
    of_node_put(cpu_node);
//
// We have to ignore the failures here and continue scanning
// the list to handle cases where the nr_cpus could be capped
// in the running kernel.
//
    if (cpu < 0)
    continue;
    cpumask_set_cpu(cpu, mask);
    }
    return 0;
    }
//
// dsu_pmu_acpi_get_cpus: Get the list of CPUs in the cluster
// from ACPI.
//
#[no_mangle]
unsafe extern "C" fn dsu_pmu_acpi_get_cpus(dev: *mut device, mask: *mut cpumask_t) -> c_int {
    static int dsu_pmu_acpi_get_cpus(struct device *dev, cpumask_t *mask)
    {

    struct acpi_device *parent_adev = acpi_dev_parent(ACPI_COMPANION(dev));
    int cpu;
//
// A dsu pmu node is inside a cluster parent node along with cpu nodes.
// We need to find out all cpus that have the same parent with this pmu.
//
    for_each_possible_cpu(cpu) {
    struct acpi_device *acpi_dev;
    struct device *cpu_dev = get_cpu_device(cpu);
    if (!cpu_dev)
    continue;
    acpi_dev = ACPI_COMPANION(cpu_dev);
    if (acpi_dev && acpi_dev_parent(acpi_dev) == parent_adev)
    cpumask_set_cpu(cpu, mask);
    }

    return 0;
    }
//
// dsu_pmu_probe_pmu: Probe the PMU details on a CPU in the cluster.
//
#[no_mangle]
unsafe extern "C" fn dsu_pmu_probe_pmu(dsu_pmu: *mut dsu_pmu) {
    static void dsu_pmu_probe_pmu(struct dsu_pmu *dsu_pmu)
    {
    u64 num_counters;
    u32 cpmceid[2];
    num_counters = (__dsu_pmu_read_pmcr() >> CLUSTERPMCR_N_SHIFT) &
    CLUSTERPMCR_N_MASK;
// We can only support up to 31 independent counters
    if (WARN_ON(num_counters > 31))
    num_counters = 31;
    dsu_pmu.num_counters = num_counters;
    if (!dsu_pmu.num_counters)
    return;
    cpmceid[0] = __dsu_pmu_read_pmceid(0);
    cpmceid[1] = __dsu_pmu_read_pmceid(1);
    bitmap_from_arr32(dsu_pmu.cpmceid_bitmap, cpmceid,
    DSU_PMU_MAX_COMMON_EVENTS);
// Newer DSUs have 64-bit counters
    __dsu_pmu_write_counter(0, U64_MAX);
    if (__dsu_pmu_read_counter(0) != U64_MAX)
    dsu_pmu.has_32b_pmevcntr = true;
// On even newer DSUs, PMCCNTR is RAZ/WI
    __dsu_pmu_write_pmccntr(U64_MAX);
    if (__dsu_pmu_read_pmccntr() == U64_MAX)
    dsu_pmu.has_pmccntr = true;
    }
#[no_mangle]
unsafe extern "C" fn dsu_pmu_set_active_cpu(cpu: c_int, dsu_pmu: *mut dsu_pmu) {
    static void dsu_pmu_set_active_cpu(int cpu, struct dsu_pmu *dsu_pmu)
    {
    cpumask_set_cpu(cpu, &dsu_pmu.active_cpu);
    if (irq_set_affinity(dsu_pmu.irq, &dsu_pmu.active_cpu))
    pr_warn("Failed to set irq affinity to %d\n", cpu);
    }
//
// dsu_pmu_init_pmu: Initialise the DSU PMU configurations if
// we haven't done it already.
//
#[no_mangle]
unsafe extern "C" fn dsu_pmu_init_pmu(dsu_pmu: *mut dsu_pmu) {
    static void dsu_pmu_init_pmu(struct dsu_pmu *dsu_pmu)
    {
    if (dsu_pmu.num_counters == -1)
    dsu_pmu_probe_pmu(dsu_pmu);
// Reset the interrupt overflow mask
    dsu_pmu_get_reset_overflow();
    }
#[no_mangle]
unsafe extern "C" fn dsu_pmu_device_probe(pdev: *mut platform_device) -> c_int {
    static int dsu_pmu_device_probe(struct platform_device *pdev)
    {
    int irq, rc;
    struct dsu_pmu *dsu_pmu;
    struct fwnode_handle *fwnode = dev_fwnode(&pdev.dev);
    char *name;
    let mut pmu_idx: static atomic_t = ATOMIC_INIT(-1);
    dsu_pmu = dsu_pmu_alloc(pdev);
    if (IS_ERR(dsu_pmu))
    return PTR_ERR(dsu_pmu);
    if (is_of_node(fwnode))
    rc = dsu_pmu_dt_get_cpus(&pdev.dev, &dsu_pmu.associated_cpus);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: is_acpi_device_node(fwnode)) -> else {
    else if (is_acpi_device_node(fwnode))
    rc = dsu_pmu_acpi_get_cpus(&pdev.dev, &dsu_pmu.associated_cpus);
    else
    return -ENOENT;
    if (rc) {
    dev_warn(&pdev.dev, "Failed to parse the CPUs\n");
    return rc;
    }
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return -EINVAL;
    name = devm_kasprintf(&pdev.dev, GFP_KERNEL, "%s_%d",
    PMUNAME, atomic_inc_return(&pmu_idx));
    if (!name)
    return -ENOMEM;
    rc = devm_request_irq(&pdev.dev, irq, dsu_pmu_handle_irq,
    IRQF_NOBALANCING, name, dsu_pmu);
    if (rc) {
    dev_warn(&pdev.dev, "Failed to request IRQ %d\n", irq);
    return rc;
    }
    dsu_pmu.irq = irq;
    platform_set_drvdata(pdev, dsu_pmu);
    rc = cpuhp_state_add_instance(dsu_pmu_cpuhp_state,
    &dsu_pmu.cpuhp_node);
    if (rc)
    return rc;
    dsu_pmu.pmu = (struct pmu) {
    .task_ctx_nr	= perf_invalid_context,
    .parent		= &pdev.dev,
    .module		= THIS_MODULE,
    .pmu_enable	= dsu_pmu_enable,
    .pmu_disable	= dsu_pmu_disable,
    .event_init	= dsu_pmu_event_init,
    .add		= dsu_pmu_add,
    .del		= dsu_pmu_del,
    .start		= dsu_pmu_start,
    .stop		= dsu_pmu_stop,
    .read		= dsu_pmu_read,
    .attr_groups	= dsu_pmu_attr_groups,
    .capabilities	= PERF_PMU_CAP_NO_EXCLUDE,
    };
    rc = perf_pmu_register(&dsu_pmu.pmu, name, -1);
    if (rc) {
    cpuhp_state_remove_instance(dsu_pmu_cpuhp_state,
    &dsu_pmu.cpuhp_node);
    }
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn dsu_pmu_device_remove(pdev: *mut platform_device) {
    static void dsu_pmu_device_remove(struct platform_device *pdev)
    {
    struct dsu_pmu *dsu_pmu = platform_get_drvdata(pdev);
    perf_pmu_unregister(&dsu_pmu.pmu);
    cpuhp_state_remove_instance(dsu_pmu_cpuhp_state, &dsu_pmu.cpuhp_node);
    }
    static const struct of_device_id dsu_pmu_of_match[] = {
    { .compatible = "arm,dsu-pmu", },
    {},
    };
    MODULE_DEVICE_TABLE(of, dsu_pmu_of_match);

    static const struct acpi_device_id dsu_pmu_acpi_match[] = {
    { "ARMHD500", 0},
    {},
    };
    MODULE_DEVICE_TABLE(acpi, dsu_pmu_acpi_match);

    static struct platform_driver dsu_pmu_driver = {
    .driver = {
    .name	= DRVNAME,
    .of_match_table = of_match_ptr(dsu_pmu_of_match),
    .acpi_match_table = ACPI_PTR(dsu_pmu_acpi_match),
    .suppress_bind_attrs = true,
    },
    .probe = dsu_pmu_device_probe,
    .remove = dsu_pmu_device_remove,
    };
#[no_mangle]
unsafe extern "C" fn dsu_pmu_cpu_online(cpu: c_uint, node: *mut hlist_node) -> c_int {
    static int dsu_pmu_cpu_online(unsigned int cpu, struct hlist_node *node)
    {
    struct dsu_pmu *dsu_pmu = hlist_entry_safe(node, struct dsu_pmu,
    cpuhp_node);
    if (!cpumask_test_cpu(cpu, &dsu_pmu.associated_cpus))
    return 0;
// If the PMU is already managed, there is nothing to do
    if (!cpumask_empty(&dsu_pmu.active_cpu))
    return 0;
    dsu_pmu_init_pmu(dsu_pmu);
    dsu_pmu_set_active_cpu(cpu, dsu_pmu);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dsu_pmu_cpu_teardown(cpu: c_uint, node: *mut hlist_node) -> c_int {
    static int dsu_pmu_cpu_teardown(unsigned int cpu, struct hlist_node *node)
    {
    struct dsu_pmu *dsu_pmu;
    unsigned int dst;
    dsu_pmu = hlist_entry_safe(node, struct dsu_pmu, cpuhp_node);
    if (!cpumask_test_and_clear_cpu(cpu, &dsu_pmu.active_cpu))
    return 0;
    dst = cpumask_any_and_but(&dsu_pmu.associated_cpus,
    cpu_online_mask, cpu);
// If there are no active CPUs in the DSU, leave IRQ disabled
    if (dst >= nr_cpu_ids)
    return 0;
    perf_pmu_migrate_context(&dsu_pmu.pmu, cpu, dst);
    dsu_pmu_set_active_cpu(dst, dsu_pmu);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dsu_pmu_init() -> int __init {
    static int __init dsu_pmu_init(void)
    {
    int ret;
    ret = cpuhp_setup_state_multi(CPUHP_AP_ONLINE_DYN,
    DRVNAME,
    dsu_pmu_cpu_online,
    dsu_pmu_cpu_teardown);
    if (ret < 0)
    return ret;
    dsu_pmu_cpuhp_state = ret;
    ret = platform_driver_register(&dsu_pmu_driver);
    if (ret)
    cpuhp_remove_multi_state(dsu_pmu_cpuhp_state);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dsu_pmu_exit() -> void __exit {
    static void __exit dsu_pmu_exit(void)
    {
    platform_driver_unregister(&dsu_pmu_driver);
    cpuhp_remove_multi_state(dsu_pmu_cpuhp_state);
    }
    module_init(dsu_pmu_init);
    module_exit(dsu_pmu_exit);
    MODULE_DESCRIPTION("Perf driver for ARM DynamIQ Shared Unit");
    MODULE_AUTHOR("Suzuki K Poulose <suzuki.poulose@arm.com>");
    MODULE_LICENSE("GPL v2");
