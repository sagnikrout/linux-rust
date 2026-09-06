//! Automatically rewritten from C to Rust
//! Source: arch/x86/events/amd/uncore.c
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
// Copyright (C) 2013 Advanced Micro Devices, Inc.
//
// Author: Jacob Shin <jacob.shin@amd.com>
//

pub const NUM_COUNTERS_NB: c_int = 4;
pub const NUM_COUNTERS_L2: c_int = 4;
pub const NUM_COUNTERS_L3: c_int = 6;
pub const NUM_COUNTERS_MAX: c_int = 64;
pub const RDPMC_BASE_NB: c_int = 6;
pub const RDPMC_BASE_LLC: c_int = 10;
pub const COUNTER_SHIFT: c_int = 16;
pub const UNCORE_NAME_LEN: c_int = 16;
pub const UNCORE_GROUP_MAX: c_int = 256;

    static int pmu_version;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_uncore_ctx {
    pub refcnt: c_int,
    pub cpu: c_int,
    pub events: *mut perf_event,
    pub active_mask: [c_ulong; BITS_TO_LONGS(NUM_COUNTERS_MAX)],
    pub nr_active: c_int,
    pub hrtimer: hrtimer,
    pub hrtimer_duration: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_uncore_pmu {
    pub name: [c_char; UNCORE_NAME_LEN],
    pub num_counters: c_int,
    pub rdpmc_base: c_int,
    pub msr_base: u32,
    pub group: c_int,
    pub active_mask: cpumask_t,
    pub pmu: pmu,
    pub ctx: *mut *mut amd_uncore_ctx  __percpu,
}

    enum {
    UNCORE_TYPE_DF,
    UNCORE_TYPE_L3,
    UNCORE_TYPE_UMC,
    UNCORE_TYPE_MAX
    };
    union amd_uncore_info {
    struct {
    u64	aux_data:32;	/* auxiliary data */
    u64	num_pmcs:8;	/* number of counters */
    u64	gid:8;		/* group id */
    u64	cid:8;		/* context id */
    } split;
    u64		full;
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_uncore {
    pub info: *mut union amd_uncore_info __percpu,
    pub pmus: *mut amd_uncore_pmu,
    pub num_pmus: c_uint,
    pub init_done: bool,
    pub cpu): *mut *mut *mut void (scan)(struct amd_uncore uncore, unsigned int,
    pub cpu): *mut *mut *mut int (init)(struct amd_uncore uncore, unsigned int,
    pub cpu): *mut *mut *mut void (move)(struct amd_uncore uncore, unsigned int,
    pub cpu): *mut *mut *mut void (free)(struct amd_uncore uncore, unsigned int,
}

    static struct amd_uncore uncores[UNCORE_TYPE_MAX];
// Interval for hrtimer, defaults to 60000 milliseconds
    let mut update_interval: static unsigned int = 60 * MSEC_PER_SEC;
    module_param(update_interval, uint, 0444);
    static struct amd_uncore_pmu *event_to_amd_uncore_pmu(struct perf_event *event)
    {
    return container_of(event.pmu, struct amd_uncore_pmu, pmu);
    }
#[no_mangle]
unsafe extern "C" fn amd_uncore_hrtimer(hrtimer: *mut hrtimer) -> enum hrtimer_restart {
    static enum hrtimer_restart amd_uncore_hrtimer(struct hrtimer *hrtimer)
    {
    struct amd_uncore_ctx *ctx;
    struct perf_event *event;
    int bit;
    ctx = container_of(hrtimer, struct amd_uncore_ctx, hrtimer);
    if (!ctx.nr_active || ctx.cpu != smp_processor_id())
    return HRTIMER_NORESTART;
    for_each_set_bit(bit, ctx.active_mask, NUM_COUNTERS_MAX) {
    event = ctx.events[bit];
    event.pmu.read(event);
    }
    hrtimer_forward_now(hrtimer, ns_to_ktime(ctx.hrtimer_duration));
    return HRTIMER_RESTART;
    }
#[no_mangle]
unsafe extern "C" fn amd_uncore_start_hrtimer(ctx: *mut amd_uncore_ctx) {
    static void amd_uncore_start_hrtimer(struct amd_uncore_ctx *ctx)
    {
    hrtimer_start(&ctx.hrtimer, ns_to_ktime(ctx.hrtimer_duration),
    HRTIMER_MODE_REL_PINNED_HARD);
    }
#[no_mangle]
unsafe extern "C" fn amd_uncore_cancel_hrtimer(ctx: *mut amd_uncore_ctx) {
    static void amd_uncore_cancel_hrtimer(struct amd_uncore_ctx *ctx)
    {
    hrtimer_cancel(&ctx.hrtimer);
    }
#[no_mangle]
unsafe extern "C" fn amd_uncore_init_hrtimer(ctx: *mut amd_uncore_ctx) {
    static void amd_uncore_init_hrtimer(struct amd_uncore_ctx *ctx)
    {
    hrtimer_setup(&ctx.hrtimer, amd_uncore_hrtimer, CLOCK_MONOTONIC, HRTIMER_MODE_REL_HARD);
    }
#[no_mangle]
unsafe extern "C" fn amd_uncore_read(event: *mut perf_event) {
    static void amd_uncore_read(struct perf_event *event)
    {
    struct hw_perf_event *hwc = &event.hw;
    u64 prev, new;
    s64 delta;
//
// since we do not enable counter overflow interrupts,
// we do not have to worry about prev_count changing on us
//
    prev = local64_read(&hwc.prev_count);
//
// Some uncore PMUs do not have RDPMC assignments. In such cases,
// read counts directly from the corresponding PERF_CTR.
//
    if (hwc.event_base_rdpmc < 0)
    rdmsrq(hwc.event_base, new);
    else
    new = rdpmc(hwc.event_base_rdpmc);
    local64_set(&hwc.prev_count, new);
    delta = (new << COUNTER_SHIFT) - (prev << COUNTER_SHIFT);
    delta >>= COUNTER_SHIFT;
    local64_add(delta, &event.count);
    }
#[no_mangle]
unsafe extern "C" fn amd_uncore_start(event: *mut perf_event, flags: c_int) {
    static void amd_uncore_start(struct perf_event *event, int flags)
    {
    struct amd_uncore_pmu *pmu = event_to_amd_uncore_pmu(event);
    struct amd_uncore_ctx *ctx = *per_cpu_ptr(pmu.ctx, event.cpu);
    struct hw_perf_event *hwc = &event.hw;
    if (!ctx.nr_active++)
    amd_uncore_start_hrtimer(ctx);
    if (flags & PERF_EF_RELOAD)
    wrmsrq(hwc.event_base, (u64)local64_read(&hwc.prev_count));
    hwc.state = 0;
    __set_bit(hwc.idx, ctx.active_mask);
    wrmsrq(hwc.config_base, (hwc.config | ARCH_PERFMON_EVENTSEL_ENABLE));
    perf_event_update_userpage(event);
    }
#[no_mangle]
unsafe extern "C" fn amd_uncore_stop(event: *mut perf_event, flags: c_int) {
    static void amd_uncore_stop(struct perf_event *event, int flags)
    {
    struct amd_uncore_pmu *pmu = event_to_amd_uncore_pmu(event);
    struct amd_uncore_ctx *ctx = *per_cpu_ptr(pmu.ctx, event.cpu);
    struct hw_perf_event *hwc = &event.hw;
    wrmsrq(hwc.config_base, hwc.config);
    hwc.state |= PERF_HES_STOPPED;
    if ((flags & PERF_EF_UPDATE) && !(hwc.state & PERF_HES_UPTODATE)) {
    event.pmu.read(event);
    hwc.state |= PERF_HES_UPTODATE;
    }
    if (!--ctx.nr_active)
    amd_uncore_cancel_hrtimer(ctx);
    __clear_bit(hwc.idx, ctx.active_mask);
    }
#[no_mangle]
unsafe extern "C" fn amd_uncore_add(event: *mut perf_event, flags: c_int) -> c_int {
    static int amd_uncore_add(struct perf_event *event, int flags)
    {
    int i;
    struct amd_uncore_pmu *pmu = event_to_amd_uncore_pmu(event);
    struct amd_uncore_ctx *ctx = *per_cpu_ptr(pmu.ctx, event.cpu);
    struct hw_perf_event *hwc = &event.hw;
// are we already assigned?
    if (hwc.idx != -1 && ctx.events[hwc.idx] == event)
    goto out;
    for (i = 0; i < pmu.num_counters; i++) {
    if (ctx.events[i] == event) {
    hwc.idx = i;
    goto out;
    }
    }
// if not, take the first available counter
    hwc.idx = -1;
    for (i = 0; i < pmu.num_counters; i++) {
    struct perf_event *tmp = core::ptr::null_mut();
    if (try_cmpxchg(&ctx.events[i], &tmp, event)) {
    hwc.idx = i;
    break;
    }
    }
    out:
    if (hwc.idx == -1)
    return -EBUSY;
    hwc.config_base = pmu.msr_base + (2 * hwc.idx);
    hwc.event_base = pmu.msr_base + 1 + (2 * hwc.idx);
    hwc.event_base_rdpmc = pmu.rdpmc_base + hwc.idx;
    hwc.state = PERF_HES_UPTODATE | PERF_HES_STOPPED;
    if (pmu.rdpmc_base < 0)
    hwc.event_base_rdpmc = -1;
    if (flags & PERF_EF_START)
    event.pmu.start(event, PERF_EF_RELOAD);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn amd_uncore_del(event: *mut perf_event, flags: c_int) {
    static void amd_uncore_del(struct perf_event *event, int flags)
    {
    int i;
    struct amd_uncore_pmu *pmu = event_to_amd_uncore_pmu(event);
    struct amd_uncore_ctx *ctx = *per_cpu_ptr(pmu.ctx, event.cpu);
    struct hw_perf_event *hwc = &event.hw;
    event.pmu.stop(event, PERF_EF_UPDATE);
    for (i = 0; i < pmu.num_counters; i++) {
    struct perf_event *tmp = event;
    if (try_cmpxchg(&ctx.events[i], &tmp, core::ptr::null_mut()))
    break;
    }
    hwc.idx = -1;
    }
#[no_mangle]
unsafe extern "C" fn amd_uncore_group_valid(event: *mut perf_event) -> bool {
    static bool amd_uncore_group_valid(struct perf_event *event)
    {
    struct amd_uncore_pmu *pmu = event_to_amd_uncore_pmu(event);
    struct perf_event *leader = event.group_leader;
    struct perf_event *sibling;
    let mut counters: c_int = 0;
    if (leader.pmu == event.pmu)
    counters++;
    for_each_sibling_event(sibling, leader) {
    if (sibling.pmu == event.pmu &&
    sibling.state > PERF_EVENT_STATE_OFF)
    counters++;
    }
//
// When pmu->event_init() is called, the event is yet to be linked to
// its leader's sibling list, so it is counted separately
//
    return (counters + 1) <= pmu.num_counters;
    }
#[no_mangle]
unsafe extern "C" fn amd_uncore_event_init(event: *mut perf_event) -> c_int {
    static int amd_uncore_event_init(struct perf_event *event)
    {
    struct amd_uncore_pmu *pmu;
    struct amd_uncore_ctx *ctx;
    struct hw_perf_event *hwc = &event.hw;
    if (event.attr.type != event.pmu.type)
    return -ENOENT;
    if (event.cpu < 0)
    return -EINVAL;
    pmu = event_to_amd_uncore_pmu(event);
    ctx = *per_cpu_ptr(pmu.ctx, event.cpu);
    if (!ctx)
    return -ENODEV;
//
// Ensure that all events in a group can be scheduled together so that
// a failure can be reported at perf_event_open() time rather than
// silently at pmu->add() time when no free counter is found
//
    if (event.group_leader != event && !amd_uncore_group_valid(event))
    return -EINVAL;
//
// NB and Last level cache counters (MSRs) are shared across all cores
// that share the same NB / Last level cache.  On family 16h and below,
// Interrupts can be directed to a single target core, however, event
// counts generated by processes running on other cores cannot be masked
// out. So we do not support sampling and per-thread events via
// CAP_NO_INTERRUPT, and we do not enable counter overflow interrupts:
//
    hwc.config = event.attr.config;
    hwc.idx = -1;
//
// since request can come in to any of the shared cores, we will remap
// to a single common cpu.
//
    event.cpu = ctx.cpu;
    return 0;
    }
    static umode_t
    amd_f17h_uncore_is_visible(struct kobject *kobj, struct attribute *attr, int i)
    {
    return boot_cpu_data.x86 >= 0x17 && boot_cpu_data.x86 < 0x19 ?
    attr.mode : 0;
    }
    static umode_t
    amd_f19h_uncore_is_visible(struct kobject *kobj, struct attribute *attr, int i)
    {
    return boot_cpu_data.x86 >= 0x19 ? attr.mode : 0;
    }
    static ssize_t amd_uncore_attr_show_cpumask(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct pmu *ptr = dev_get_drvdata(dev);
    struct amd_uncore_pmu *pmu = container_of(ptr, struct amd_uncore_pmu, pmu);
    return sysfs_emit(buf, "%*pbl\n", cpumask_pr_args(&pmu.active_mask));
    }
    static DEVICE_ATTR(cpumask, S_IRUGO, amd_uncore_attr_show_cpumask, core::ptr::null_mut());
    static struct attribute *amd_uncore_attrs[] = {
    &dev_attr_cpumask.attr,
    core::ptr::null_mut(),
    };
    static struct attribute_group amd_uncore_attr_group = {
    .attrs = amd_uncore_attrs,
    };

    static ssize_t __uncore_##_var##_show(struct device *dev,		\
    struct device_attribute *attr,		\
    char *page)				\
    {									\
    BUILD_BUG_ON(sizeof(_format) >= PAGE_SIZE);			\
    return sprintf(page, _format "\n");				\
    }									\
    static struct device_attribute format_attr_##_var =			\
    __ATTR(_name, 0444, __uncore_##_var##_show, core::ptr::null_mut())
    DEFINE_UNCORE_FORMAT_ATTR(event12,	event,		"config:0-7,32-35");
    DEFINE_UNCORE_FORMAT_ATTR(event14,	event,		"config:0-7,32-35,59-60"); /* F17h+ DF */
    DEFINE_UNCORE_FORMAT_ATTR(event14v2,	event,		"config:0-7,32-37");	   /* PerfMonV2 DF */
    DEFINE_UNCORE_FORMAT_ATTR(event8,	event,		"config:0-7");		   /* F17h+ L3, PerfMonV2 UMC */
    DEFINE_UNCORE_FORMAT_ATTR(umask8,	umask,		"config:8-15");
    DEFINE_UNCORE_FORMAT_ATTR(umask12,	umask,		"config:8-15,24-27");	   /* PerfMonV2 DF */
    DEFINE_UNCORE_FORMAT_ATTR(coreid,	coreid,		"config:42-44");	   /* F19h L3 */
    DEFINE_UNCORE_FORMAT_ATTR(slicemask,	slicemask,	"config:48-51");	   /* F17h L3 */
    DEFINE_UNCORE_FORMAT_ATTR(threadmask8,	threadmask,	"config:56-63");	   /* F17h L3 */
    DEFINE_UNCORE_FORMAT_ATTR(threadmask2,	threadmask,	"config:56-57");	   /* F19h L3 */
    DEFINE_UNCORE_FORMAT_ATTR(enallslices,	enallslices,	"config:46");		   /* F19h L3 */
    DEFINE_UNCORE_FORMAT_ATTR(enallcores,	enallcores,	"config:47");		   /* F19h L3 */
    DEFINE_UNCORE_FORMAT_ATTR(sliceid,	sliceid,	"config:48-50");	   /* F19h L3 */
    DEFINE_UNCORE_FORMAT_ATTR(rdwrmask,	rdwrmask,	"config:8-9");		   /* PerfMonV2 UMC */
// Common DF and NB attributes
    static struct attribute *amd_uncore_df_format_attr[] = {
    &format_attr_event12.attr,	/* event */
    &format_attr_umask8.attr,	/* umask */
    core::ptr::null_mut(),
    };
// Common L2 and L3 attributes
    static struct attribute *amd_uncore_l3_format_attr[] = {
    &format_attr_event12.attr,	/* event */
    &format_attr_umask8.attr,	/* umask */
    core::ptr::null_mut(),				/* threadmask */
    core::ptr::null_mut(),
    };
// Common UMC attributes
    static struct attribute *amd_uncore_umc_format_attr[] = {
    &format_attr_event8.attr,       /* event */
    &format_attr_rdwrmask.attr,     /* rdwrmask */
    core::ptr::null_mut(),
    };
// F17h unique L3 attributes
    static struct attribute *amd_f17h_uncore_l3_format_attr[] = {
    &format_attr_slicemask.attr,	/* slicemask */
    core::ptr::null_mut(),
    };
// F19h unique L3 attributes
    static struct attribute *amd_f19h_uncore_l3_format_attr[] = {
    &format_attr_coreid.attr,	/* coreid */
    &format_attr_enallslices.attr,	/* enallslices */
    &format_attr_enallcores.attr,	/* enallcores */
    &format_attr_sliceid.attr,	/* sliceid */
    core::ptr::null_mut(),
    };
    static struct attribute_group amd_uncore_df_format_group = {
    .name = "format",
    .attrs = amd_uncore_df_format_attr,
    };
    static struct attribute_group amd_uncore_l3_format_group = {
    .name = "format",
    .attrs = amd_uncore_l3_format_attr,
    };
    static struct attribute_group amd_f17h_uncore_l3_format_group = {
    .name = "format",
    .attrs = amd_f17h_uncore_l3_format_attr,
    .is_visible = amd_f17h_uncore_is_visible,
    };
    static struct attribute_group amd_f19h_uncore_l3_format_group = {
    .name = "format",
    .attrs = amd_f19h_uncore_l3_format_attr,
    .is_visible = amd_f19h_uncore_is_visible,
    };
    static struct attribute_group amd_uncore_umc_format_group = {
    .name = "format",
    .attrs = amd_uncore_umc_format_attr,
    };
    static const struct attribute_group *amd_uncore_df_attr_groups[] = {
    &amd_uncore_attr_group,
    &amd_uncore_df_format_group,
    core::ptr::null_mut(),
    };
    static const struct attribute_group *amd_uncore_l3_attr_groups[] = {
    &amd_uncore_attr_group,
    &amd_uncore_l3_format_group,
    core::ptr::null_mut(),
    };
    static const struct attribute_group *amd_uncore_l3_attr_update[] = {
    &amd_f17h_uncore_l3_format_group,
    &amd_f19h_uncore_l3_format_group,
    core::ptr::null_mut(),
    };
    static const struct attribute_group *amd_uncore_umc_attr_groups[] = {
    &amd_uncore_attr_group,
    &amd_uncore_umc_format_group,
    core::ptr::null_mut(),
    };
    static __always_inline
#[no_mangle]
pub unsafe extern "C" fn amd_uncore_ctx_cid(uncore: *mut amd_uncore, cpu: c_uint) -> c_int {
    int amd_uncore_ctx_cid(struct amd_uncore *uncore, unsigned int cpu)
    {
    union amd_uncore_info *info = per_cpu_ptr(uncore.info, cpu);
    return info.split.cid;
    }
    static __always_inline
#[no_mangle]
pub unsafe extern "C" fn amd_uncore_ctx_gid(uncore: *mut amd_uncore, cpu: c_uint) -> c_int {
    int amd_uncore_ctx_gid(struct amd_uncore *uncore, unsigned int cpu)
    {
    union amd_uncore_info *info = per_cpu_ptr(uncore.info, cpu);
    return info.split.gid;
    }
    static __always_inline
#[no_mangle]
pub unsafe extern "C" fn amd_uncore_ctx_num_pmcs(uncore: *mut amd_uncore, cpu: c_uint) -> c_int {
    int amd_uncore_ctx_num_pmcs(struct amd_uncore *uncore, unsigned int cpu)
    {
    union amd_uncore_info *info = per_cpu_ptr(uncore.info, cpu);
    return info.split.num_pmcs;
    }
#[no_mangle]
unsafe extern "C" fn amd_uncore_ctx_free(uncore: *mut amd_uncore, cpu: c_uint) {
    static void amd_uncore_ctx_free(struct amd_uncore *uncore, unsigned int cpu)
    {
    struct amd_uncore_pmu *pmu;
    struct amd_uncore_ctx *ctx;
    int i;
    if (!uncore.init_done)
    return;
    for (i = 0; i < uncore.num_pmus; i++) {
    pmu = &uncore.pmus[i];
    ctx = *per_cpu_ptr(pmu.ctx, cpu);
    if (!ctx)
    continue;
    if (cpu == ctx.cpu)
    cpumask_clear_cpu(cpu, &pmu.active_mask);
    if (!--ctx.refcnt) {
    kfree(ctx.events);
    kfree(ctx);
    }
// per_cpu_ptr(pmu->ctx, cpu) = NULL;
    }
    }
#[no_mangle]
unsafe extern "C" fn amd_uncore_ctx_init(uncore: *mut amd_uncore, cpu: c_uint) -> c_int {
    static int amd_uncore_ctx_init(struct amd_uncore *uncore, unsigned int cpu)
    {
    struct amd_uncore_ctx *curr, *prev;
    struct amd_uncore_pmu *pmu;
    int node, cid, gid, i, j;
    if (!uncore.init_done || !uncore.num_pmus)
    return 0;
    cid = amd_uncore_ctx_cid(uncore, cpu);
    gid = amd_uncore_ctx_gid(uncore, cpu);
    for (i = 0; i < uncore.num_pmus; i++) {
    pmu = &uncore.pmus[i];
// per_cpu_ptr(pmu->ctx, cpu) = NULL;
    curr = core::ptr::null_mut();
// Check for group exclusivity
    if (gid != pmu.group)
    continue;
// Find a sibling context
    for_each_online_cpu(j) {
    if (cpu == j)
    continue;
    prev = *per_cpu_ptr(pmu.ctx, j);
    if (!prev)
    continue;
    if (cid == amd_uncore_ctx_cid(uncore, j)) {
    curr = prev;
    break;
    }
    }
// Allocate context if sibling does not exist
    if (!curr) {
    node = cpu_to_node(cpu);
    curr = kzalloc_node(sizeof(*curr), GFP_KERNEL, node);
    if (!curr)
    goto fail;
    curr.cpu = cpu;
    curr.events = kzalloc_node(sizeof(*curr.events) *
    pmu.num_counters,
    GFP_KERNEL, node);
    if (!curr.events) {
    kfree(curr);
    goto fail;
    }
    amd_uncore_init_hrtimer(curr);
    curr.hrtimer_duration = (u64)update_interval * NSEC_PER_MSEC;
    cpumask_set_cpu(cpu, &pmu.active_mask);
    }
    curr.refcnt++;
// per_cpu_ptr(pmu->ctx, cpu) = curr;
    }
    return 0;
    fail:
    amd_uncore_ctx_free(uncore, cpu);
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn amd_uncore_ctx_move(uncore: *mut amd_uncore, cpu: c_uint) {
    static void amd_uncore_ctx_move(struct amd_uncore *uncore, unsigned int cpu)
    {
    struct amd_uncore_ctx *curr, *next;
    struct amd_uncore_pmu *pmu;
    int i, j;
    if (!uncore.init_done)
    return;
    for (i = 0; i < uncore.num_pmus; i++) {
    pmu = &uncore.pmus[i];
    curr = *per_cpu_ptr(pmu.ctx, cpu);
    if (!curr)
    continue;
// Migrate to a shared sibling if possible
    for_each_online_cpu(j) {
    next = *per_cpu_ptr(pmu.ctx, j);
    if (!next || cpu == j)
    continue;
    if (curr == next) {
    perf_pmu_migrate_context(&pmu.pmu, cpu, j);
    cpumask_clear_cpu(cpu, &pmu.active_mask);
    cpumask_set_cpu(j, &pmu.active_mask);
    next.cpu = j;
    break;
    }
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn amd_uncore_cpu_starting(cpu: c_uint) -> c_int {
    static int amd_uncore_cpu_starting(unsigned int cpu)
    {
    struct amd_uncore *uncore;
    int i;
    for (i = 0; i < UNCORE_TYPE_MAX; i++) {
    uncore = &uncores[i];
    uncore.scan(uncore, cpu);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn amd_uncore_cpu_online(cpu: c_uint) -> c_int {
    static int amd_uncore_cpu_online(unsigned int cpu)
    {
    struct amd_uncore *uncore;
    int i;
    for (i = 0; i < UNCORE_TYPE_MAX; i++) {
    uncore = &uncores[i];
    if (uncore.init(uncore, cpu))
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn amd_uncore_cpu_down_prepare(cpu: c_uint) -> c_int {
    static int amd_uncore_cpu_down_prepare(unsigned int cpu)
    {
    struct amd_uncore *uncore;
    int i;
    for (i = 0; i < UNCORE_TYPE_MAX; i++) {
    uncore = &uncores[i];
    uncore.move(uncore, cpu);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn amd_uncore_cpu_dead(cpu: c_uint) -> c_int {
    static int amd_uncore_cpu_dead(unsigned int cpu)
    {
    struct amd_uncore *uncore;
    int i;
    for (i = 0; i < UNCORE_TYPE_MAX; i++) {
    uncore = &uncores[i];
    uncore.free(uncore, cpu);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn amd_uncore_df_event_init(event: *mut perf_event) -> c_int {
    static int amd_uncore_df_event_init(struct perf_event *event)
    {
    struct hw_perf_event *hwc = &event.hw;
    let mut ret: c_int = amd_uncore_event_init(event);
    hwc.config = event.attr.config &
    (pmu_version >= 2 ? AMD64_PERFMON_V2_RAW_EVENT_MASK_NB :
    AMD64_RAW_EVENT_MASK_NB);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn amd_uncore_df_add(event: *mut perf_event, flags: c_int) -> c_int {
    static int amd_uncore_df_add(struct perf_event *event, int flags)
    {
    let mut ret: c_int = amd_uncore_add(event, flags & ~PERF_EF_START);
    struct hw_perf_event *hwc = &event.hw;
    if (ret)
    return ret;
//
// The first four DF counters are accessible via RDPMC index 6 to 9
// followed by the L3 counters from index 10 to 15. For processors
// with more than four DF counters, the DF RDPMC assignments become
// discontiguous as the additional counters are accessible starting
// from index 16.
//
    if (hwc.idx >= NUM_COUNTERS_NB)
    hwc.event_base_rdpmc += NUM_COUNTERS_L3;
// Delayed start after rdpmc base update
    if (flags & PERF_EF_START)
    amd_uncore_start(event, PERF_EF_RELOAD);
    return 0;
    }
    static
#[no_mangle]
pub unsafe extern "C" fn amd_uncore_df_ctx_scan(uncore: *mut amd_uncore, cpu: c_uint) {
    void amd_uncore_df_ctx_scan(struct amd_uncore *uncore, unsigned int cpu)
    {
    union cpuid_0x80000022_ebx ebx;
    union amd_uncore_info info;
    if (!boot_cpu_has(X86_FEATURE_PERFCTR_NB))
    return;
    info.split.aux_data = 0;
    info.split.num_pmcs = NUM_COUNTERS_NB;
    info.split.gid = 0;
    info.split.cid = topology_amd_node_id(cpu);
    if (pmu_version >= 2) {
    ebx.full = cpuid_ebx(EXT_PERFMON_DEBUG_FEATURES);
    info.split.num_pmcs = ebx.split.num_df_pmc;
    }
// per_cpu_ptr(uncore->info, cpu) = info;
    }
    static
#[no_mangle]
pub unsafe extern "C" fn amd_uncore_df_ctx_init(uncore: *mut amd_uncore, cpu: c_uint) -> c_int {
    int amd_uncore_df_ctx_init(struct amd_uncore *uncore, unsigned int cpu)
    {
    struct attribute **df_attr = amd_uncore_df_format_attr;
    struct amd_uncore_pmu *pmu;
    int num_counters;
// Run just once
    if (uncore.init_done)
    return amd_uncore_ctx_init(uncore, cpu);
    num_counters = amd_uncore_ctx_num_pmcs(uncore, cpu);
    if (!num_counters)
    goto done;
// No grouping, single instance for a system
    uncore.pmus = kzalloc_obj(*uncore.pmus);
    if (!uncore.pmus)
    goto done;
//
// For Family 17h and above, the Northbridge counters are repurposed
// as Data Fabric counters. The PMUs are exported based on family as
// either NB or DF.
//
    pmu = &uncore.pmus[0];
    strscpy(pmu.name, boot_cpu_data.x86 >= 0x17 ? "amd_df" : "amd_nb",
    sizeof(pmu.name));
    pmu.num_counters = num_counters;
    pmu.msr_base = MSR_F15H_NB_PERF_CTL;
    pmu.rdpmc_base = RDPMC_BASE_NB;
    pmu.group = amd_uncore_ctx_gid(uncore, cpu);
    if (pmu_version >= 2) {
// df_attr++ = &format_attr_event14v2.attr;
// df_attr++ = &format_attr_umask12.attr;
    } else if (boot_cpu_data.x86 >= 0x17) {
// df_attr = &format_attr_event14.attr;
    }
    pmu.ctx = alloc_percpu(struct amd_uncore_ctx *);
    if (!pmu.ctx)
    goto done;
    pmu.pmu = (struct pmu) {
    .task_ctx_nr	= perf_invalid_context,
    .attr_groups	= amd_uncore_df_attr_groups,
    .name		= pmu.name,
    .event_init	= amd_uncore_df_event_init,
    .add		= amd_uncore_df_add,
    .del		= amd_uncore_del,
    .start		= amd_uncore_start,
    .stop		= amd_uncore_stop,
    .read		= amd_uncore_read,
    .capabilities	= PERF_PMU_CAP_NO_EXCLUDE | PERF_PMU_CAP_NO_INTERRUPT,
    .module		= THIS_MODULE,
    };
    if (perf_pmu_register(&pmu.pmu, pmu.pmu.name, -1)) {
    free_percpu(pmu.ctx);
    pmu.ctx = core::ptr::null_mut();
    goto done;
    }
    pr_info("%d %s%s counters detected\n", pmu.num_counters,
    boot_cpu_data.x86_vendor == X86_VENDOR_HYGON ?  "HYGON " : "",
    pmu.pmu.name);
    uncore.num_pmus = 1;
    done:
    uncore.init_done = true;
    return amd_uncore_ctx_init(uncore, cpu);
    }
#[no_mangle]
unsafe extern "C" fn amd_uncore_l3_event_init(event: *mut perf_event) -> c_int {
    static int amd_uncore_l3_event_init(struct perf_event *event)
    {
    let mut ret: c_int = amd_uncore_event_init(event);
    struct hw_perf_event *hwc = &event.hw;
    let mut config: u64 = event.attr.config;
    u64 mask;
    hwc.config = config & AMD64_RAW_EVENT_MASK_NB;
//
// SliceMask and ThreadMask need to be set for certain L3 events.
// For other events, the two fields do not affect the count.
//
    if (ret || boot_cpu_data.x86 < 0x17)
    return ret;
    mask = config & (AMD64_L3_F19H_THREAD_MASK | AMD64_L3_SLICEID_MASK |
    AMD64_L3_EN_ALL_CORES | AMD64_L3_EN_ALL_SLICES |
    AMD64_L3_COREID_MASK);
    if (boot_cpu_data.x86 <= 0x18)
    mask = ((config & AMD64_L3_SLICE_MASK) ? : AMD64_L3_SLICE_MASK) |
    ((config & AMD64_L3_THREAD_MASK) ? : AMD64_L3_THREAD_MASK);
//
// If the user doesn't specify a ThreadMask, they're not trying to
// count core 0, so we enable all cores & threads.
// We'll also assume that they want to count slice 0 if they specify
// a ThreadMask and leave SliceId and EnAllSlices unpopulated.
//
#[no_mangle]
pub unsafe extern "C" fn if(AMD64_L3_F19H_THREAD_MASK): !(config &) -> else {
    else if (!(config & AMD64_L3_F19H_THREAD_MASK))
    mask = AMD64_L3_F19H_THREAD_MASK | AMD64_L3_EN_ALL_SLICES |
    AMD64_L3_EN_ALL_CORES;
    hwc.config |= mask;
    return 0;
    }
    static
#[no_mangle]
pub unsafe extern "C" fn amd_uncore_l3_ctx_scan(uncore: *mut amd_uncore, cpu: c_uint) {
    void amd_uncore_l3_ctx_scan(struct amd_uncore *uncore, unsigned int cpu)
    {
    union amd_uncore_info info;
    if (!boot_cpu_has(X86_FEATURE_PERFCTR_LLC))
    return;
    info.split.aux_data = 0;
    info.split.num_pmcs = NUM_COUNTERS_L2;
    info.split.gid = 0;
    info.split.cid = per_cpu_llc_id(cpu);
    if (boot_cpu_data.x86 >= 0x17)
    info.split.num_pmcs = NUM_COUNTERS_L3;
// per_cpu_ptr(uncore->info, cpu) = info;
    }
    static
#[no_mangle]
pub unsafe extern "C" fn amd_uncore_l3_ctx_init(uncore: *mut amd_uncore, cpu: c_uint) -> c_int {
    int amd_uncore_l3_ctx_init(struct amd_uncore *uncore, unsigned int cpu)
    {
    struct attribute **l3_attr = amd_uncore_l3_format_attr;
    struct amd_uncore_pmu *pmu;
    int num_counters;
// Run just once
    if (uncore.init_done)
    return amd_uncore_ctx_init(uncore, cpu);
    num_counters = amd_uncore_ctx_num_pmcs(uncore, cpu);
    if (!num_counters)
    goto done;
// No grouping, single instance for a system
    uncore.pmus = kzalloc_obj(*uncore.pmus);
    if (!uncore.pmus)
    goto done;
//
// For Family 17h and above, L3 cache counters are available instead
// of L2 cache counters. The PMUs are exported based on family as
// either L2 or L3.
//
    pmu = &uncore.pmus[0];
    strscpy(pmu.name, boot_cpu_data.x86 >= 0x17 ? "amd_l3" : "amd_l2",
    sizeof(pmu.name));
    pmu.num_counters = num_counters;
    pmu.msr_base = MSR_F16H_L2I_PERF_CTL;
    pmu.rdpmc_base = RDPMC_BASE_LLC;
    pmu.group = amd_uncore_ctx_gid(uncore, cpu);
    if (boot_cpu_data.x86 >= 0x17) {
// l3_attr++ = &format_attr_event8.attr;
// l3_attr++ = &format_attr_umask8.attr;
// l3_attr++ = boot_cpu_data.x86 >= 0x19 ?
    &format_attr_threadmask2.attr :
    &format_attr_threadmask8.attr;
    }
    pmu.ctx = alloc_percpu(struct amd_uncore_ctx *);
    if (!pmu.ctx)
    goto done;
    pmu.pmu = (struct pmu) {
    .task_ctx_nr	= perf_invalid_context,
    .attr_groups	= amd_uncore_l3_attr_groups,
    .attr_update	= amd_uncore_l3_attr_update,
    .name		= pmu.name,
    .event_init	= amd_uncore_l3_event_init,
    .add		= amd_uncore_add,
    .del		= amd_uncore_del,
    .start		= amd_uncore_start,
    .stop		= amd_uncore_stop,
    .read		= amd_uncore_read,
    .capabilities	= PERF_PMU_CAP_NO_EXCLUDE | PERF_PMU_CAP_NO_INTERRUPT,
    .module		= THIS_MODULE,
    };
    if (perf_pmu_register(&pmu.pmu, pmu.pmu.name, -1)) {
    free_percpu(pmu.ctx);
    pmu.ctx = core::ptr::null_mut();
    goto done;
    }
    pr_info("%d %s%s counters detected\n", pmu.num_counters,
    boot_cpu_data.x86_vendor == X86_VENDOR_HYGON ?  "HYGON " : "",
    pmu.pmu.name);
    uncore.num_pmus = 1;
    done:
    uncore.init_done = true;
    return amd_uncore_ctx_init(uncore, cpu);
    }
#[no_mangle]
unsafe extern "C" fn amd_uncore_umc_event_init(event: *mut perf_event) -> c_int {
    static int amd_uncore_umc_event_init(struct perf_event *event)
    {
    struct hw_perf_event *hwc = &event.hw;
    let mut ret: c_int = amd_uncore_event_init(event);
    if (ret)
    return ret;
    hwc.config = event.attr.config & AMD64_PERFMON_V2_RAW_EVENT_MASK_UMC;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn amd_uncore_umc_start(event: *mut perf_event, flags: c_int) {
    static void amd_uncore_umc_start(struct perf_event *event, int flags)
    {
    struct amd_uncore_pmu *pmu = event_to_amd_uncore_pmu(event);
    struct amd_uncore_ctx *ctx = *per_cpu_ptr(pmu.ctx, event.cpu);
    struct hw_perf_event *hwc = &event.hw;
    if (!ctx.nr_active++)
    amd_uncore_start_hrtimer(ctx);
    if (flags & PERF_EF_RELOAD)
    wrmsrq(hwc.event_base, (u64)local64_read(&hwc.prev_count));
    hwc.state = 0;
    __set_bit(hwc.idx, ctx.active_mask);
    wrmsrq(hwc.config_base, (hwc.config | AMD64_PERFMON_V2_ENABLE_UMC));
    perf_event_update_userpage(event);
    }
#[no_mangle]
unsafe extern "C" fn amd_uncore_umc_read(event: *mut perf_event) {
    static void amd_uncore_umc_read(struct perf_event *event)
    {
    struct hw_perf_event *hwc = &event.hw;
    u64 prev, new, shift;
    s64 delta;
    shift = COUNTER_SHIFT + 1;
    prev = local64_read(&hwc.prev_count);
//
// UMC counters do not have RDPMC assignments. Read counts directly
// from the corresponding PERF_CTR.
//
    rdmsrq(hwc.event_base, new);
//
// Unlike the other uncore counters, UMC counters saturate and set the
// Overflow bit (bit 48) on overflow. Since they do not roll over,
// proactively reset the corresponding PERF_CTR when bit 47 is set so
// that the counter never gets a chance to saturate.
//
    if (new & BIT_ULL(63 - COUNTER_SHIFT)) {
    wrmsrq(hwc.event_base, 0);
    local64_set(&hwc.prev_count, 0);
    } else {
    local64_set(&hwc.prev_count, new);
    }
    delta = (new << shift) - (prev << shift);
    delta >>= shift;
    local64_add(delta, &event.count);
    }
    static
#[no_mangle]
pub unsafe extern "C" fn amd_uncore_umc_ctx_scan(uncore: *mut amd_uncore, cpu: c_uint) {
    void amd_uncore_umc_ctx_scan(struct amd_uncore *uncore, unsigned int cpu)
    {
    union cpuid_0x80000022_ebx ebx;
    union amd_uncore_info info;
    unsigned int eax, ecx, edx;
    if (pmu_version < 2)
    return;
    cpuid(EXT_PERFMON_DEBUG_FEATURES, &eax, &ebx.full, &ecx, &edx);
    info.split.aux_data = ecx;	/* stash active mask */
    info.split.num_pmcs = ebx.split.num_umc_pmc;
    info.split.gid = topology_amd_node_id(cpu);
    info.split.cid = topology_amd_node_id(cpu);
// per_cpu_ptr(uncore->info, cpu) = info;
    }
    static
#[no_mangle]
pub unsafe extern "C" fn amd_uncore_umc_ctx_init(uncore: *mut amd_uncore, cpu: c_uint) -> c_int {
    int amd_uncore_umc_ctx_init(struct amd_uncore *uncore, unsigned int cpu)
    {
    DECLARE_BITMAP(gmask, UNCORE_GROUP_MAX) = { 0 };
    u8 group_num_pmus[UNCORE_GROUP_MAX] = { 0 };
    u8 group_num_pmcs[UNCORE_GROUP_MAX] = { 0 };
    union amd_uncore_info info;
    struct amd_uncore_pmu *pmu;
    int gid, i;
    let mut index: u16 = 0;
    if (pmu_version < 2)
    return 0;
// Run just once
    if (uncore.init_done)
    return amd_uncore_ctx_init(uncore, cpu);
// Find unique groups
    for_each_online_cpu(i) {
    info = *per_cpu_ptr(uncore.info, i);
    gid = info.split.gid;
    if (test_bit(gid, gmask))
    continue;
    __set_bit(gid, gmask);
    group_num_pmus[gid] = hweight32(info.split.aux_data);
    group_num_pmcs[gid] = info.split.num_pmcs;
    uncore.num_pmus += group_num_pmus[gid];
    }
    uncore.pmus = kzalloc(sizeof(*uncore.pmus) * uncore.num_pmus,
    GFP_KERNEL);
    if (!uncore.pmus) {
    uncore.num_pmus = 0;
    goto done;
    }
    for_each_set_bit(gid, gmask, UNCORE_GROUP_MAX) {
    for (i = 0; i < group_num_pmus[gid]; i++) {
    pmu = &uncore.pmus[index];
    snprintf(pmu.name, sizeof(pmu.name), "amd_umc_%hu", index);
    pmu.num_counters = group_num_pmcs[gid] / group_num_pmus[gid];
    pmu.msr_base = MSR_F19H_UMC_PERF_CTL + i * pmu.num_counters * 2;
    pmu.rdpmc_base = -1;
    pmu.group = gid;
    pmu.ctx = alloc_percpu(struct amd_uncore_ctx *);
    if (!pmu.ctx)
    goto done;
    pmu.pmu = (struct pmu) {
    .task_ctx_nr	= perf_invalid_context,
    .attr_groups	= amd_uncore_umc_attr_groups,
    .name		= pmu.name,
    .event_init	= amd_uncore_umc_event_init,
    .add		= amd_uncore_add,
    .del		= amd_uncore_del,
    .start		= amd_uncore_umc_start,
    .stop		= amd_uncore_stop,
    .read		= amd_uncore_umc_read,
    .capabilities	= PERF_PMU_CAP_NO_EXCLUDE | PERF_PMU_CAP_NO_INTERRUPT,
    .module		= THIS_MODULE,
    };
    if (perf_pmu_register(&pmu.pmu, pmu.pmu.name, -1)) {
    free_percpu(pmu.ctx);
    pmu.ctx = core::ptr::null_mut();
    goto done;
    }
    pr_info("%d %s counters detected\n", pmu.num_counters,
    pmu.pmu.name);
    index++;
    }
    }
    done:
    uncore.num_pmus = index;
    uncore.init_done = true;
    return amd_uncore_ctx_init(uncore, cpu);
    }
    static struct amd_uncore uncores[UNCORE_TYPE_MAX] = {
// UNCORE_TYPE_DF
    {
    .scan = amd_uncore_df_ctx_scan,
    .init = amd_uncore_df_ctx_init,
    .move = amd_uncore_ctx_move,
    .free = amd_uncore_ctx_free,
    },
// UNCORE_TYPE_L3
    {
    .scan = amd_uncore_l3_ctx_scan,
    .init = amd_uncore_l3_ctx_init,
    .move = amd_uncore_ctx_move,
    .free = amd_uncore_ctx_free,
    },
// UNCORE_TYPE_UMC
    {
    .scan = amd_uncore_umc_ctx_scan,
    .init = amd_uncore_umc_ctx_init,
    .move = amd_uncore_ctx_move,
    .free = amd_uncore_ctx_free,
    },
    };
#[no_mangle]
unsafe extern "C" fn amd_uncore_init() -> int __init {
    static int __init amd_uncore_init(void)
    {
    struct amd_uncore *uncore;
    let mut ret: c_int = -ENODEV;
    int i;
    if (boot_cpu_data.x86_vendor != X86_VENDOR_AMD &&
    boot_cpu_data.x86_vendor != X86_VENDOR_HYGON)
    return -ENODEV;
    if (!boot_cpu_has(X86_FEATURE_TOPOEXT))
    return -ENODEV;
    if (boot_cpu_has(X86_FEATURE_PERFMON_V2))
    pmu_version = 2;
    for (i = 0; i < UNCORE_TYPE_MAX; i++) {
    uncore = &uncores[i];
    BUG_ON(!uncore.scan);
    BUG_ON(!uncore.init);
    BUG_ON(!uncore.move);
    BUG_ON(!uncore.free);
    uncore.info = alloc_percpu(union amd_uncore_info);
    if (!uncore.info) {
    ret = -ENOMEM;
    goto fail;
    }
    };
//
// Install callbacks. Core will call them for each online cpu.
//
    ret = cpuhp_setup_state(CPUHP_PERF_X86_AMD_UNCORE_PREP,
    "perf/x86/amd/uncore:prepare",
    core::ptr::null_mut(), amd_uncore_cpu_dead);
    if (ret)
    goto fail;
    ret = cpuhp_setup_state(CPUHP_AP_PERF_X86_AMD_UNCORE_STARTING,
    "perf/x86/amd/uncore:starting",
    amd_uncore_cpu_starting, core::ptr::null_mut());
    if (ret)
    goto fail_prep;
    ret = cpuhp_setup_state(CPUHP_AP_PERF_X86_AMD_UNCORE_ONLINE,
    "perf/x86/amd/uncore:online",
    amd_uncore_cpu_online,
    amd_uncore_cpu_down_prepare);
    if (ret)
    goto fail_start;
    return 0;
    fail_start:
    cpuhp_remove_state(CPUHP_AP_PERF_X86_AMD_UNCORE_STARTING);
    fail_prep:
    cpuhp_remove_state(CPUHP_PERF_X86_AMD_UNCORE_PREP);
    fail:
    for (i = 0; i < UNCORE_TYPE_MAX; i++) {
    uncore = &uncores[i];
    if (uncore.info) {
    free_percpu(uncore.info);
    uncore.info = core::ptr::null_mut();
    }
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn amd_uncore_exit() -> void __exit {
    static void __exit amd_uncore_exit(void)
    {
    struct amd_uncore *uncore;
    struct amd_uncore_pmu *pmu;
    int i, j;
    cpuhp_remove_state(CPUHP_AP_PERF_X86_AMD_UNCORE_ONLINE);
    cpuhp_remove_state(CPUHP_AP_PERF_X86_AMD_UNCORE_STARTING);
    cpuhp_remove_state(CPUHP_PERF_X86_AMD_UNCORE_PREP);
    for (i = 0; i < UNCORE_TYPE_MAX; i++) {
    uncore = &uncores[i];
    if (!uncore.info)
    continue;
    free_percpu(uncore.info);
    uncore.info = core::ptr::null_mut();
    for (j = 0; j < uncore.num_pmus; j++) {
    pmu = &uncore.pmus[j];
    if (!pmu.ctx)
    continue;
    perf_pmu_unregister(&pmu.pmu);
    free_percpu(pmu.ctx);
    pmu.ctx = core::ptr::null_mut();
    }
    kfree(uncore.pmus);
    uncore.pmus = core::ptr::null_mut();
    }
    }
    module_init(amd_uncore_init);
    module_exit(amd_uncore_exit);
    MODULE_DESCRIPTION("AMD Uncore Driver");
    MODULE_LICENSE("GPL v2");
