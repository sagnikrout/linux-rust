//! Automatically rewritten from C to Rust
//! Source: drivers/perf/qcom_l2_pmu.c
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
// Copyright (c) 2015-2017 The Linux Foundation. All rights reserved.
//

pub const MAX_L2_CTRS: c_int = 9;
pub const L2PMCR_NUM_EV_SHIFT: c_int = 11;
pub const L2PMCR_NUM_EV_MASK: c_uint = 0x1F;
pub const L2PMCR: c_uint = 0x400;
pub const L2PMCNTENCLR: c_uint = 0x403;
pub const L2PMCNTENSET: c_uint = 0x404;
pub const L2PMINTENCLR: c_uint = 0x405;
pub const L2PMINTENSET: c_uint = 0x406;
pub const L2PMOVSCLR: c_uint = 0x407;
pub const L2PMOVSSET: c_uint = 0x408;
pub const L2PMCCNTCR: c_uint = 0x409;
pub const L2PMCCNTR: c_uint = 0x40A;
pub const L2PMCCNTSR: c_uint = 0x40C;
pub const L2PMRESR: c_uint = 0x410;
pub const IA_L2PMXEVCNTCR_BASE: c_uint = 0x420;
pub const IA_L2PMXEVCNTR_BASE: c_uint = 0x421;
pub const IA_L2PMXEVFILTER_BASE: c_uint = 0x423;
pub const IA_L2PMXEVTYPER_BASE: c_uint = 0x424;
pub const IA_L2_REG_OFFSET: c_uint = 0x10;
pub const L2PMXEVFILTER_SUFILTER_ALL: c_uint = 0x000E0000;
pub const L2PMXEVFILTER_ORGFILTER_IDINDEP: c_uint = 0x00000004;
pub const L2PMXEVFILTER_ORGFILTER_ALL: c_uint = 0x00000003;
pub const L2EVTYPER_REG_SHIFT: c_int = 3;
pub const L2PMRESR_GROUP_BITS: c_int = 8;

pub const L2CYCLE_CTR_BIT: c_int = 31;
pub const L2CYCLE_CTR_RAW_CODE: c_uint = 0xFE;
pub const L2PMCR_RESET_ALL: c_uint = 0x6;
pub const L2PMCR_COUNTERS_ENABLE: c_uint = 0x1;
pub const L2PMCR_COUNTERS_DISABLE: c_uint = 0x0;

pub const L2_EVT_MASK: c_uint = 0x00000FFF;
pub const L2_EVT_CODE_MASK: c_uint = 0x00000FF0;
pub const L2_EVT_GRP_MASK: c_uint = 0x0000000F;
pub const L2_EVT_CODE_SHIFT: c_int = 4;
pub const L2_EVT_GRP_SHIFT: c_int = 0;

pub const L2_EVT_GROUP_MAX: c_int = 7;

//
// Events
//
pub const L2_EVENT_CYCLES: c_uint = 0xfe;
pub const L2_EVENT_DCACHE_OPS: c_uint = 0x400;
pub const L2_EVENT_ICACHE_OPS: c_uint = 0x401;
pub const L2_EVENT_TLBI: c_uint = 0x402;
pub const L2_EVENT_BARRIERS: c_uint = 0x403;
pub const L2_EVENT_TOTAL_READS: c_uint = 0x405;
pub const L2_EVENT_TOTAL_WRITES: c_uint = 0x406;
pub const L2_EVENT_TOTAL_REQUESTS: c_uint = 0x407;
pub const L2_EVENT_LDREX: c_uint = 0x420;
pub const L2_EVENT_STREX: c_uint = 0x421;
pub const L2_EVENT_CLREX: c_uint = 0x422;
    struct cluster_pmu;
//
// Aggregate PMU. Implements the core pmu functions and manages
// the hardware PMUs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2cache_pmu {
    pub node: hlist_node,
    pub num_pmus: u32,
    pub pmu: pmu,
    pub num_counters: c_int,
    pub cpumask: cpumask_t,
    pub pdev: *mut platform_device,
    pub pmu_cluster: *mut *mut cluster_pmu  __percpu,
    pub clusters: list_head,
}

//
// The cache is made up of one or more clusters, each cluster has its own PMU.
// Each cluster is associated with one or more CPUs.
// This structure represents one of the hardware PMUs.
//
// Events can be envisioned as a 2-dimensional array. Each column represents
// a group of events. There are 8 groups. Only one entry from each
// group can be in use at a time.
//
// Events are specified as 0xCCG, where CC is 2 hex digits specifying
// the code (array row) and G specifies the group (column).
//
// In addition there is a cycle counter event specified by L2CYCLE_CTR_RAW_CODE
// which is outside the above scheme.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cluster_pmu {
    pub next: list_head,
    pub events: [*mut perf_event; MAX_L2_CTRS],
    pub l2cache_pmu: *mut l2cache_pmu,
    pub MAX_L2_CTRS): DECLARE_BITMAP(used_counters,,
    pub 1): DECLARE_BITMAP(used_groups, L2_EVT_GROUP_MAX +,
    pub irq: c_int,
    pub cluster_id: c_int,
// The CPU that is used for collecting events on this cluster
    pub on_cpu: c_int,
// All the CPUs associated with this cluster
    pub cluster_cpus: cpumask_t,
    pub pmu_lock: spinlock_t,
}

    static u32 l2_cycle_ctr_idx;
    static u32 l2_counter_present_mask;
#[no_mangle]
pub unsafe extern "C" fn idx_to_reg_bit(idx: u32) -> u32 {
    static inline u32 idx_to_reg_bit(u32 idx)
    {
    if (idx == l2_cycle_ctr_idx)
    return BIT(L2CYCLE_CTR_BIT);
    return BIT(idx);
    }
    static inline struct cluster_pmu *get_cluster_pmu(
    struct l2cache_pmu *l2cache_pmu, int cpu)
    {
    return *per_cpu_ptr(l2cache_pmu.pmu_cluster, cpu);
    }
#[no_mangle]
unsafe extern "C" fn cluster_pmu_reset() {
    static void cluster_pmu_reset(void)
    {
// Reset all counters
    kryo_l2_set_indirect_reg(L2PMCR, L2PMCR_RESET_ALL);
    kryo_l2_set_indirect_reg(L2PMCNTENCLR, l2_counter_present_mask);
    kryo_l2_set_indirect_reg(L2PMINTENCLR, l2_counter_present_mask);
    kryo_l2_set_indirect_reg(L2PMOVSCLR, l2_counter_present_mask);
    }
#[no_mangle]
pub unsafe extern "C" fn cluster_pmu_enable() {
    static inline void cluster_pmu_enable(void)
    {
    kryo_l2_set_indirect_reg(L2PMCR, L2PMCR_COUNTERS_ENABLE);
    }
#[no_mangle]
pub unsafe extern "C" fn cluster_pmu_disable() {
    static inline void cluster_pmu_disable(void)
    {
    kryo_l2_set_indirect_reg(L2PMCR, L2PMCR_COUNTERS_DISABLE);
    }
#[no_mangle]
pub unsafe extern "C" fn cluster_pmu_counter_set_value(idx: u32, value: u64) {
    static inline void cluster_pmu_counter_set_value(u32 idx, u64 value)
    {
    if (idx == l2_cycle_ctr_idx)
    kryo_l2_set_indirect_reg(L2PMCCNTR, value);
    else
    kryo_l2_set_indirect_reg(reg_idx(IA_L2PMXEVCNTR, idx), value);
    }
#[no_mangle]
pub unsafe extern "C" fn cluster_pmu_counter_get_value(idx: u32) -> u64 {
    static inline u64 cluster_pmu_counter_get_value(u32 idx)
    {
    u64 value;
    if (idx == l2_cycle_ctr_idx)
    value = kryo_l2_get_indirect_reg(L2PMCCNTR);
    else
    value = kryo_l2_get_indirect_reg(reg_idx(IA_L2PMXEVCNTR, idx));
    return value;
    }
#[no_mangle]
pub unsafe extern "C" fn cluster_pmu_counter_enable(idx: u32) {
    static inline void cluster_pmu_counter_enable(u32 idx)
    {
    kryo_l2_set_indirect_reg(L2PMCNTENSET, idx_to_reg_bit(idx));
    }
#[no_mangle]
pub unsafe extern "C" fn cluster_pmu_counter_disable(idx: u32) {
    static inline void cluster_pmu_counter_disable(u32 idx)
    {
    kryo_l2_set_indirect_reg(L2PMCNTENCLR, idx_to_reg_bit(idx));
    }
#[no_mangle]
pub unsafe extern "C" fn cluster_pmu_counter_enable_interrupt(idx: u32) {
    static inline void cluster_pmu_counter_enable_interrupt(u32 idx)
    {
    kryo_l2_set_indirect_reg(L2PMINTENSET, idx_to_reg_bit(idx));
    }
#[no_mangle]
pub unsafe extern "C" fn cluster_pmu_counter_disable_interrupt(idx: u32) {
    static inline void cluster_pmu_counter_disable_interrupt(u32 idx)
    {
    kryo_l2_set_indirect_reg(L2PMINTENCLR, idx_to_reg_bit(idx));
    }
#[no_mangle]
pub unsafe extern "C" fn cluster_pmu_set_evccntcr(val: u32) {
    static inline void cluster_pmu_set_evccntcr(u32 val)
    {
    kryo_l2_set_indirect_reg(L2PMCCNTCR, val);
    }
#[no_mangle]
pub unsafe extern "C" fn cluster_pmu_set_evcntcr(ctr: u32, val: u32) {
    static inline void cluster_pmu_set_evcntcr(u32 ctr, u32 val)
    {
    kryo_l2_set_indirect_reg(reg_idx(IA_L2PMXEVCNTCR, ctr), val);
    }
#[no_mangle]
pub unsafe extern "C" fn cluster_pmu_set_evtyper(ctr: u32, val: u32) {
    static inline void cluster_pmu_set_evtyper(u32 ctr, u32 val)
    {
    kryo_l2_set_indirect_reg(reg_idx(IA_L2PMXEVTYPER, ctr), val);
    }
    static void cluster_pmu_set_resr(struct cluster_pmu *cluster,
    u32 event_group, u32 event_cc)
    {
    u64 field;
    u64 resr_val;
    u32 shift;
    unsigned long flags;
    shift = L2PMRESR_GROUP_BITS * event_group;
    field = ((u64)(event_cc & L2PMRESR_GROUP_MASK) << shift);
    spin_lock_irqsave(&cluster.pmu_lock, flags);
    resr_val = kryo_l2_get_indirect_reg(L2PMRESR);
    resr_val &= ~(L2PMRESR_GROUP_MASK << shift);
    resr_val |= field;
    resr_val |= L2PMRESR_EN;
    kryo_l2_set_indirect_reg(L2PMRESR, resr_val);
    spin_unlock_irqrestore(&cluster.pmu_lock, flags);
    }
//
// Hardware allows filtering of events based on the originating
// CPU. Turn this off by setting filter bits to allow events from
// all CPUS, subunits and ID independent events in this cluster.
//
#[no_mangle]
pub unsafe extern "C" fn cluster_pmu_set_evfilter_sys_mode(ctr: u32) {
    static inline void cluster_pmu_set_evfilter_sys_mode(u32 ctr)
    {
    u32 val =  L2PMXEVFILTER_SUFILTER_ALL |
    L2PMXEVFILTER_ORGFILTER_IDINDEP |
    L2PMXEVFILTER_ORGFILTER_ALL;
    kryo_l2_set_indirect_reg(reg_idx(IA_L2PMXEVFILTER, ctr), val);
    }
#[no_mangle]
pub unsafe extern "C" fn cluster_pmu_getreset_ovsr() -> u32 {
    static inline u32 cluster_pmu_getreset_ovsr(void)
    {
    let mut result: u32 = kryo_l2_get_indirect_reg(L2PMOVSSET);
    kryo_l2_set_indirect_reg(L2PMOVSCLR, result);
    return result;
    }
#[no_mangle]
pub unsafe extern "C" fn cluster_pmu_has_overflowed(ovsr: u32) -> bool {
    static inline bool cluster_pmu_has_overflowed(u32 ovsr)
    {
    return !!(ovsr & l2_counter_present_mask);
    }
#[no_mangle]
pub unsafe extern "C" fn cluster_pmu_counter_has_overflowed(ovsr: u32, idx: u32) -> bool {
    static inline bool cluster_pmu_counter_has_overflowed(u32 ovsr, u32 idx)
    {
    return !!(ovsr & idx_to_reg_bit(idx));
    }
#[no_mangle]
unsafe extern "C" fn l2_cache_event_update(event: *mut perf_event) {
    static void l2_cache_event_update(struct perf_event *event)
    {
    struct hw_perf_event *hwc = &event.hw;
    u64 delta, prev, now;
    let mut idx: u32 = hwc.idx;
    do {
    prev = local64_read(&hwc.prev_count);
    now = cluster_pmu_counter_get_value(idx);
    } while (local64_cmpxchg(&hwc.prev_count, prev, now) != prev);
//
// The cycle counter is 64-bit, but all other counters are
// 32-bit, and we must handle 32-bit overflow explicitly.
//
    delta = now - prev;
    if (idx != l2_cycle_ctr_idx)
    delta &= 0xffffffff;
    local64_add(delta, &event.count);
    }
    static void l2_cache_cluster_set_period(struct cluster_pmu *cluster,
    struct hw_perf_event *hwc)
    {
    let mut idx: u32 = hwc.idx;
    u64 new;
//
// We limit the max period to half the max counter value so
// that even in the case of extreme interrupt latency the
// counter will (hopefully) not wrap past its initial value.
//
    if (idx == l2_cycle_ctr_idx)
    new = L2_CYCLE_COUNTER_RELOAD;
    else
    new = L2_COUNTER_RELOAD;
    local64_set(&hwc.prev_count, new);
    cluster_pmu_counter_set_value(idx, new);
    }
    static int l2_cache_get_event_idx(struct cluster_pmu *cluster,
    struct perf_event *event)
    {
    struct hw_perf_event *hwc = &event.hw;
    int idx;
    let mut num_ctrs: c_int = cluster.l2cache_pmu.num_counters - 1;
    unsigned int group;
    if (hwc.config_base == L2CYCLE_CTR_RAW_CODE) {
    if (test_and_set_bit(l2_cycle_ctr_idx, cluster.used_counters))
    return -EAGAIN;
    return l2_cycle_ctr_idx;
    }
    idx = find_first_zero_bit(cluster.used_counters, num_ctrs);
    if (idx == num_ctrs)
// The counters are all in use.
    return -EAGAIN;
//
// Check for column exclusion: event column already in use by another
// event. This is for events which are not in the same group.
// Conflicting events in the same group are detected in event_init.
//
    group = L2_EVT_GROUP(hwc.config_base);
    if (test_bit(group, cluster.used_groups))
    return -EAGAIN;
    set_bit(idx, cluster.used_counters);
    set_bit(group, cluster.used_groups);
    return idx;
    }
    static void l2_cache_clear_event_idx(struct cluster_pmu *cluster,
    struct perf_event *event)
    {
    struct hw_perf_event *hwc = &event.hw;
    let mut idx: c_int = hwc.idx;
    clear_bit(idx, cluster.used_counters);
    if (hwc.config_base != L2CYCLE_CTR_RAW_CODE)
    clear_bit(L2_EVT_GROUP(hwc.config_base), cluster.used_groups);
    }
#[no_mangle]
unsafe extern "C" fn l2_cache_handle_irq(irq_num: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t l2_cache_handle_irq(int irq_num, void *data)
    {
    struct cluster_pmu *cluster = data;
    let mut num_counters: c_int = cluster.l2cache_pmu.num_counters;
    u32 ovsr;
    int idx;
    ovsr = cluster_pmu_getreset_ovsr();
    if (!cluster_pmu_has_overflowed(ovsr))
    return IRQ_NONE;
    for_each_set_bit(idx, cluster.used_counters, num_counters) {
    struct perf_event *event = cluster.events[idx];
    struct hw_perf_event *hwc;
    if (WARN_ON_ONCE(!event))
    continue;
    if (!cluster_pmu_counter_has_overflowed(ovsr, idx))
    continue;
    l2_cache_event_update(event);
    hwc = &event.hw;
    l2_cache_cluster_set_period(cluster, hwc);
    }
    return IRQ_HANDLED;
    }
//
// Implementation of abstract pmu functionality required by
// the core perf events code.
//
#[no_mangle]
unsafe extern "C" fn l2_cache_pmu_enable(pmu: *mut pmu) {
    static void l2_cache_pmu_enable(struct pmu *pmu)
    {
//
// Although there is only one PMU (per socket) controlling multiple
// physical PMUs (per cluster), because we do not support per-task mode
// each event is associated with a CPU. Each event has pmu_enable
// called on its CPU, so here it is only necessary to enable the
// counters for the current CPU.
//
    cluster_pmu_enable();
    }
#[no_mangle]
unsafe extern "C" fn l2_cache_pmu_disable(pmu: *mut pmu) {
    static void l2_cache_pmu_disable(struct pmu *pmu)
    {
    cluster_pmu_disable();
    }
#[no_mangle]
unsafe extern "C" fn l2_cache_event_init(event: *mut perf_event) -> c_int {
    static int l2_cache_event_init(struct perf_event *event)
    {
    struct hw_perf_event *hwc = &event.hw;
    struct cluster_pmu *cluster;
    struct perf_event *sibling;
    struct l2cache_pmu *l2cache_pmu;
    if (event.attr.type != event.pmu.type)
    return -ENOENT;
    l2cache_pmu = to_l2cache_pmu(event.pmu);
    if (hwc.sample_period) {
    dev_dbg_ratelimited(&l2cache_pmu.pdev.dev,
    "Sampling not supported\n");
    return -EOPNOTSUPP;
    }
    if (event.cpu < 0) {
    dev_dbg_ratelimited(&l2cache_pmu.pdev.dev,
    "Per-task mode not supported\n");
    return -EOPNOTSUPP;
    }
    if (((L2_EVT_GROUP(event.attr.config) > L2_EVT_GROUP_MAX) ||
    ((event.attr.config & ~L2_EVT_MASK) != 0)) &&
    (event.attr.config != L2CYCLE_CTR_RAW_CODE)) {
    dev_dbg_ratelimited(&l2cache_pmu.pdev.dev,
    "Invalid config %llx\n",
    event.attr.config);
    return -EINVAL;
    }
// Don't allow groups with mixed PMUs, except for s/w events
    if (event.group_leader.pmu != event.pmu &&
    !is_software_event(event.group_leader)) {
    dev_dbg_ratelimited(&l2cache_pmu.pdev.dev,
    "Can't create mixed PMU group\n");
    return -EINVAL;
    }
    for_each_sibling_event(sibling, event.group_leader) {
    if (sibling.pmu != event.pmu &&
    !is_software_event(sibling)) {
    dev_dbg_ratelimited(&l2cache_pmu.pdev.dev,
    "Can't create mixed PMU group\n");
    return -EINVAL;
    }
    }
    cluster = get_cluster_pmu(l2cache_pmu, event.cpu);
    if (!cluster) {
// CPU has not been initialised
    dev_dbg_ratelimited(&l2cache_pmu.pdev.dev,
    "CPU%d not associated with L2 cluster\n", event.cpu);
    return -EINVAL;
    }
// Ensure all events in a group are on the same cpu
    if ((event.group_leader != event) &&
    (cluster.on_cpu != event.group_leader.cpu)) {
    dev_dbg_ratelimited(&l2cache_pmu.pdev.dev,
    "Can't create group on CPUs %d and %d",
    event.cpu, event.group_leader.cpu);
    return -EINVAL;
    }
    if ((event != event.group_leader) &&
    !is_software_event(event.group_leader) &&
    (L2_EVT_GROUP(event.group_leader.attr.config) ==
    L2_EVT_GROUP(event.attr.config))) {
    dev_dbg_ratelimited(&l2cache_pmu.pdev.dev,
    "Column exclusion: conflicting events %llx %llx\n",
    event.group_leader.attr.config,
    event.attr.config);
    return -EINVAL;
    }
    for_each_sibling_event(sibling, event.group_leader) {
    if ((sibling != event) &&
    !is_software_event(sibling) &&
    (L2_EVT_GROUP(sibling.attr.config) ==
    L2_EVT_GROUP(event.attr.config))) {
    dev_dbg_ratelimited(&l2cache_pmu.pdev.dev,
    "Column exclusion: conflicting events %llx %llx\n",
    sibling.attr.config,
    event.attr.config);
    return -EINVAL;
    }
    }
    hwc.idx = -1;
    hwc.config_base = event.attr.config;
//
// Ensure all events are on the same cpu so all events are in the
// same cpu context, to avoid races on pmu_enable etc.
//
    event.cpu = cluster.on_cpu;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn l2_cache_event_start(event: *mut perf_event, flags: c_int) {
    static void l2_cache_event_start(struct perf_event *event, int flags)
    {
    struct cluster_pmu *cluster;
    struct hw_perf_event *hwc = &event.hw;
    let mut idx: c_int = hwc.idx;
    u32 config;
    u32 event_cc, event_group;
    hwc.state = 0;
    cluster = get_cluster_pmu(to_l2cache_pmu(event.pmu), event.cpu);
    l2_cache_cluster_set_period(cluster, hwc);
    if (hwc.config_base == L2CYCLE_CTR_RAW_CODE) {
    cluster_pmu_set_evccntcr(0);
    } else {
    config = hwc.config_base;
    event_cc    = L2_EVT_CODE(config);
    event_group = L2_EVT_GROUP(config);
    cluster_pmu_set_evcntcr(idx, 0);
    cluster_pmu_set_evtyper(idx, event_group);
    cluster_pmu_set_resr(cluster, event_group, event_cc);
    cluster_pmu_set_evfilter_sys_mode(idx);
    }
    cluster_pmu_counter_enable_interrupt(idx);
    cluster_pmu_counter_enable(idx);
    }
#[no_mangle]
unsafe extern "C" fn l2_cache_event_stop(event: *mut perf_event, flags: c_int) {
    static void l2_cache_event_stop(struct perf_event *event, int flags)
    {
    struct hw_perf_event *hwc = &event.hw;
    let mut idx: c_int = hwc.idx;
    if (hwc.state & PERF_HES_STOPPED)
    return;
    cluster_pmu_counter_disable_interrupt(idx);
    cluster_pmu_counter_disable(idx);
    if (flags & PERF_EF_UPDATE)
    l2_cache_event_update(event);
    hwc.state |= PERF_HES_STOPPED | PERF_HES_UPTODATE;
    }
#[no_mangle]
unsafe extern "C" fn l2_cache_event_add(event: *mut perf_event, flags: c_int) -> c_int {
    static int l2_cache_event_add(struct perf_event *event, int flags)
    {
    struct hw_perf_event *hwc = &event.hw;
    int idx;
    let mut err: c_int = 0;
    struct cluster_pmu *cluster;
    cluster = get_cluster_pmu(to_l2cache_pmu(event.pmu), event.cpu);
    idx = l2_cache_get_event_idx(cluster, event);
    if (idx < 0)
    return idx;
    hwc.idx = idx;
    hwc.state = PERF_HES_STOPPED | PERF_HES_UPTODATE;
    cluster.events[idx] = event;
    local64_set(&hwc.prev_count, 0);
    if (flags & PERF_EF_START)
    l2_cache_event_start(event, flags);
// Propagate changes to the userspace mapping.
    perf_event_update_userpage(event);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn l2_cache_event_del(event: *mut perf_event, flags: c_int) {
    static void l2_cache_event_del(struct perf_event *event, int flags)
    {
    struct hw_perf_event *hwc = &event.hw;
    struct cluster_pmu *cluster;
    let mut idx: c_int = hwc.idx;
    cluster = get_cluster_pmu(to_l2cache_pmu(event.pmu), event.cpu);
    l2_cache_event_stop(event, flags | PERF_EF_UPDATE);
    cluster.events[idx] = core::ptr::null_mut();
    l2_cache_clear_event_idx(cluster, event);
    perf_event_update_userpage(event);
    }
#[no_mangle]
unsafe extern "C" fn l2_cache_event_read(event: *mut perf_event) {
    static void l2_cache_event_read(struct perf_event *event)
    {
    l2_cache_event_update(event);
    }
    static ssize_t l2_cache_pmu_cpumask_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct l2cache_pmu *l2cache_pmu = to_l2cache_pmu(dev_get_drvdata(dev));
    return sysfs_emit(buf, "%*pbl\n", cpumask_pr_args(&l2cache_pmu.cpumask));
    }
    static struct device_attribute l2_cache_pmu_cpumask_attr =
    __ATTR(cpumask, S_IRUGO, l2_cache_pmu_cpumask_show, core::ptr::null_mut());
    static struct attribute *l2_cache_pmu_cpumask_attrs[] = {
    &l2_cache_pmu_cpumask_attr.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group l2_cache_pmu_cpumask_group = {
    .attrs = l2_cache_pmu_cpumask_attrs,
    };
// CCG format for perf RAW codes.
    PMU_FORMAT_ATTR(l2_code,   "config:4-11");
    PMU_FORMAT_ATTR(l2_group,  "config:0-3");
    PMU_FORMAT_ATTR(event,     "config:0-11");
    static struct attribute *l2_cache_pmu_formats[] = {
    &format_attr_l2_code.attr,
    &format_attr_l2_group.attr,
    &format_attr_event.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group l2_cache_pmu_format_group = {
    .name = "format",
    .attrs = l2_cache_pmu_formats,
    };
    static ssize_t l2cache_pmu_event_show(struct device *dev,
    struct device_attribute *attr, char *page)
    {
    struct perf_pmu_events_attr *pmu_attr;
    pmu_attr = container_of(attr, struct perf_pmu_events_attr, attr);
    return sysfs_emit(page, "event=0x%02llx\n", pmu_attr.id);
    }

    PMU_EVENT_ATTR_ID(_name, l2cache_pmu_event_show, _id)
    static struct attribute *l2_cache_pmu_events[] = {
    L2CACHE_EVENT_ATTR(cycles, L2_EVENT_CYCLES),
    L2CACHE_EVENT_ATTR(dcache-ops, L2_EVENT_DCACHE_OPS),
    L2CACHE_EVENT_ATTR(icache-ops, L2_EVENT_ICACHE_OPS),
    L2CACHE_EVENT_ATTR(tlbi, L2_EVENT_TLBI),
    L2CACHE_EVENT_ATTR(barriers, L2_EVENT_BARRIERS),
    L2CACHE_EVENT_ATTR(total-reads, L2_EVENT_TOTAL_READS),
    L2CACHE_EVENT_ATTR(total-writes, L2_EVENT_TOTAL_WRITES),
    L2CACHE_EVENT_ATTR(total-requests, L2_EVENT_TOTAL_REQUESTS),
    L2CACHE_EVENT_ATTR(ldrex, L2_EVENT_LDREX),
    L2CACHE_EVENT_ATTR(strex, L2_EVENT_STREX),
    L2CACHE_EVENT_ATTR(clrex, L2_EVENT_CLREX),
    core::ptr::null_mut()
    };
    static const struct attribute_group l2_cache_pmu_events_group = {
    .name = "events",
    .attrs = l2_cache_pmu_events,
    };
    static const struct attribute_group *l2_cache_pmu_attr_grps[] = {
    &l2_cache_pmu_format_group,
    &l2_cache_pmu_cpumask_group,
    &l2_cache_pmu_events_group,
    core::ptr::null_mut(),
    };
//
// Generic device handlers
//
    static const struct acpi_device_id l2_cache_pmu_acpi_match[] = {
    { "QCOM8130", },
    { }
    };
#[no_mangle]
unsafe extern "C" fn get_num_counters() -> c_int {
    static int get_num_counters(void)
    {
    int val;
    val = kryo_l2_get_indirect_reg(L2PMCR);
//
// Read number of counters from L2PMCR and add 1
// for the cycle counter.
//
    return ((val >> L2PMCR_NUM_EV_SHIFT) & L2PMCR_NUM_EV_MASK) + 1;
    }
    static struct cluster_pmu *l2_cache_associate_cpu_with_cluster(
    struct l2cache_pmu *l2cache_pmu, int cpu)
    {
    u64 mpidr;
    int cpu_cluster_id;
    struct cluster_pmu *cluster;
//
// This assumes that the cluster_id is in MPIDR[aff1] for
// single-threaded cores, and MPIDR[aff2] for multi-threaded
// cores. This logic will have to be updated if this changes.
//
    mpidr = read_cpuid_mpidr();
    if (mpidr & MPIDR_MT_BITMASK)
    cpu_cluster_id = MPIDR_AFFINITY_LEVEL(mpidr, 2);
    else
    cpu_cluster_id = MPIDR_AFFINITY_LEVEL(mpidr, 1);
    list_for_each_entry(cluster, &l2cache_pmu.clusters, next) {
    if (cluster.cluster_id != cpu_cluster_id)
    continue;
    dev_info(&l2cache_pmu.pdev.dev,
    "CPU%d associated with cluster %d\n", cpu,
    cluster.cluster_id);
    cpumask_set_cpu(cpu, &cluster.cluster_cpus);
// per_cpu_ptr(l2cache_pmu->pmu_cluster, cpu) = cluster;
    return cluster;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn l2cache_pmu_online_cpu(cpu: c_uint, node: *mut hlist_node) -> c_int {
    static int l2cache_pmu_online_cpu(unsigned int cpu, struct hlist_node *node)
    {
    struct cluster_pmu *cluster;
    struct l2cache_pmu *l2cache_pmu;
    l2cache_pmu = hlist_entry_safe(node, struct l2cache_pmu, node);
    cluster = get_cluster_pmu(l2cache_pmu, cpu);
    if (!cluster) {
// First time this CPU has come online
    cluster = l2_cache_associate_cpu_with_cluster(l2cache_pmu, cpu);
    if (!cluster) {
// Only if broken firmware doesn't list every cluster
    WARN_ONCE(1, "No L2 cache cluster for CPU%d\n", cpu);
    return 0;
    }
    }
// If another CPU is managing this cluster, we're done
    if (cluster.on_cpu != -1)
    return 0;
//
// All CPUs on this cluster were down, use this one.
// Reset to put it into sane state.
//
    cluster.on_cpu = cpu;
    cpumask_set_cpu(cpu, &l2cache_pmu.cpumask);
    cluster_pmu_reset();
    WARN_ON(irq_set_affinity(cluster.irq, cpumask_of(cpu)));
    enable_irq(cluster.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn l2cache_pmu_offline_cpu(cpu: c_uint, node: *mut hlist_node) -> c_int {
    static int l2cache_pmu_offline_cpu(unsigned int cpu, struct hlist_node *node)
    {
    struct l2cache_pmu *l2cache_pmu;
    struct cluster_pmu *cluster;
    unsigned int target;
    l2cache_pmu = hlist_entry_safe(node, struct l2cache_pmu, node);
    cluster = get_cluster_pmu(l2cache_pmu, cpu);
    if (!cluster)
    return 0;
// If this CPU is not managing the cluster, we're done
    if (cluster.on_cpu != cpu)
    return 0;
// Give up ownership of cluster
    cpumask_clear_cpu(cpu, &l2cache_pmu.cpumask);
    cluster.on_cpu = -1;
// Any other CPU for this cluster which is still online
    target = cpumask_any_and_but(&cluster.cluster_cpus,
    cpu_online_mask, cpu);
    if (target >= nr_cpu_ids) {
    disable_irq(cluster.irq);
    return 0;
    }
    perf_pmu_migrate_context(&l2cache_pmu.pmu, cpu, target);
    cluster.on_cpu = target;
    cpumask_set_cpu(target, &l2cache_pmu.cpumask);
    WARN_ON(irq_set_affinity(cluster.irq, cpumask_of(target)));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn l2_cache_pmu_probe_cluster(dev: *mut device, data: *mut c_void) -> c_int {
    static int l2_cache_pmu_probe_cluster(struct device *dev, void *data)
    {
    struct platform_device *pdev = to_platform_device(dev.parent);
    struct platform_device *sdev = to_platform_device(dev);
    struct l2cache_pmu *l2cache_pmu = data;
    struct cluster_pmu *cluster;
    u64 fw_cluster_id;
    int err;
    int irq;
    err = acpi_dev_uid_to_integer(ACPI_COMPANION(dev), &fw_cluster_id);
    if (err) {
    dev_err(&pdev.dev, "unable to read ACPI uid\n");
    return err;
    }
    cluster = devm_kzalloc(&pdev.dev, sizeof(*cluster), GFP_KERNEL);
    if (!cluster)
    return -ENOMEM;
    INIT_LIST_HEAD(&cluster.next);
    cluster.cluster_id = fw_cluster_id;
    irq = platform_get_irq(sdev, 0);
    if (irq < 0)
    return irq;
    cluster.irq = irq;
    cluster.l2cache_pmu = l2cache_pmu;
    cluster.on_cpu = -1;
    err = devm_request_irq(&pdev.dev, irq, l2_cache_handle_irq,
    IRQF_NOBALANCING | IRQF_NO_THREAD |
    IRQF_NO_AUTOEN,
    "l2-cache-pmu", cluster);
    if (err)
    return err;
    dev_info(&pdev.dev,
    "Registered L2 cache PMU cluster %lld\n", fw_cluster_id);
    spin_lock_init(&cluster.pmu_lock);
    list_add(&cluster.next, &l2cache_pmu.clusters);
    l2cache_pmu.num_pmus++;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn l2_cache_pmu_probe(pdev: *mut platform_device) -> c_int {
    static int l2_cache_pmu_probe(struct platform_device *pdev)
    {
    int err;
    struct l2cache_pmu *l2cache_pmu;
    l2cache_pmu =
    devm_kzalloc(&pdev.dev, sizeof(*l2cache_pmu), GFP_KERNEL);
    if (!l2cache_pmu)
    return -ENOMEM;
    INIT_LIST_HEAD(&l2cache_pmu.clusters);
    platform_set_drvdata(pdev, l2cache_pmu);
    l2cache_pmu.pmu = (struct pmu) {
// suffix is instance id for future use with multiple sockets
    .name		= "l2cache_0",
    .parent		= &pdev.dev,
    .task_ctx_nr    = perf_invalid_context,
    .pmu_enable	= l2_cache_pmu_enable,
    .pmu_disable	= l2_cache_pmu_disable,
    .event_init	= l2_cache_event_init,
    .add		= l2_cache_event_add,
    .del		= l2_cache_event_del,
    .start		= l2_cache_event_start,
    .stop		= l2_cache_event_stop,
    .read		= l2_cache_event_read,
    .attr_groups	= l2_cache_pmu_attr_grps,
    .capabilities	= PERF_PMU_CAP_NO_EXCLUDE,
    };
    l2cache_pmu.num_counters = get_num_counters();
    l2cache_pmu.pdev = pdev;
    l2cache_pmu.pmu_cluster = devm_alloc_percpu(&pdev.dev,
    struct cluster_pmu *);
    if (!l2cache_pmu.pmu_cluster)
    return -ENOMEM;
    l2_cycle_ctr_idx = l2cache_pmu.num_counters - 1;
    l2_counter_present_mask = GENMASK(l2cache_pmu.num_counters - 2, 0) |
    BIT(L2CYCLE_CTR_BIT);
    cpumask_clear(&l2cache_pmu.cpumask);
// Read cluster info and initialize each cluster
    err = device_for_each_child(&pdev.dev, l2cache_pmu,
    l2_cache_pmu_probe_cluster);
    if (err)
    return err;
    if (l2cache_pmu.num_pmus == 0) {
    dev_err(&pdev.dev, "No hardware L2 cache PMUs found\n");
    return -ENODEV;
    }
    err = cpuhp_state_add_instance(CPUHP_AP_PERF_ARM_QCOM_L2_ONLINE,
    &l2cache_pmu.node);
    if (err) {
    dev_err(&pdev.dev, "Error %d registering hotplug", err);
    return err;
    }
    err = perf_pmu_register(&l2cache_pmu.pmu, l2cache_pmu.pmu.name, -1);
    if (err) {
    dev_err(&pdev.dev, "Error %d registering L2 cache PMU\n", err);
    goto out_unregister;
    }
    dev_info(&pdev.dev, "Registered L2 cache PMU using %d HW PMUs\n",
    l2cache_pmu.num_pmus);
    return err;
    out_unregister:
    cpuhp_state_remove_instance(CPUHP_AP_PERF_ARM_QCOM_L2_ONLINE,
    &l2cache_pmu.node);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn l2_cache_pmu_remove(pdev: *mut platform_device) {
    static void l2_cache_pmu_remove(struct platform_device *pdev)
    {
    struct l2cache_pmu *l2cache_pmu =
    to_l2cache_pmu(platform_get_drvdata(pdev));
    perf_pmu_unregister(&l2cache_pmu.pmu);
    cpuhp_state_remove_instance(CPUHP_AP_PERF_ARM_QCOM_L2_ONLINE,
    &l2cache_pmu.node);
    }
    static struct platform_driver l2_cache_pmu_driver = {
    .driver = {
    .name = "qcom-l2cache-pmu",
    .acpi_match_table = ACPI_PTR(l2_cache_pmu_acpi_match),
    .suppress_bind_attrs = true,
    },
    .probe = l2_cache_pmu_probe,
    .remove = l2_cache_pmu_remove,
    };
#[no_mangle]
unsafe extern "C" fn register_l2_cache_pmu_driver() -> int __init {
    static int __init register_l2_cache_pmu_driver(void)
    {
    int err;
    err = cpuhp_setup_state_multi(CPUHP_AP_PERF_ARM_QCOM_L2_ONLINE,
    "AP_PERF_ARM_QCOM_L2_ONLINE",
    l2cache_pmu_online_cpu,
    l2cache_pmu_offline_cpu);
    if (err)
    return err;
    return platform_driver_register(&l2_cache_pmu_driver);
    }
    device_initcall(register_l2_cache_pmu_driver);
