//! Automatically rewritten from C to Rust
//! Source: arch/x86/events/msr.c
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

    enum perf_msr_id {
    PERF_MSR_TSC			= 0,
    PERF_MSR_APERF			= 1,
    PERF_MSR_MPERF			= 2,
    PERF_MSR_PPERF			= 3,
    PERF_MSR_SMI			= 4,
    PERF_MSR_PTSC			= 5,
    PERF_MSR_IRPERF			= 6,
    PERF_MSR_THERM			= 7,
    PERF_MSR_EVENT_MAX,
    };
#[no_mangle]
unsafe extern "C" fn test_aperfmperf(idx: c_int, data: *mut c_void) -> bool {
    static bool test_aperfmperf(int idx, void *data)
    {
    return boot_cpu_has(X86_FEATURE_APERFMPERF);
    }
#[no_mangle]
unsafe extern "C" fn test_ptsc(idx: c_int, data: *mut c_void) -> bool {
    static bool test_ptsc(int idx, void *data)
    {
    return boot_cpu_has(X86_FEATURE_PTSC);
    }
#[no_mangle]
unsafe extern "C" fn test_irperf(idx: c_int, data: *mut c_void) -> bool {
    static bool test_irperf(int idx, void *data)
    {
    return boot_cpu_has(X86_FEATURE_IRPERF);
    }
#[no_mangle]
unsafe extern "C" fn test_therm_status(idx: c_int, data: *mut c_void) -> bool {
    static bool test_therm_status(int idx, void *data)
    {
    return boot_cpu_has(X86_FEATURE_DTHERM);
    }
#[no_mangle]
unsafe extern "C" fn test_intel(idx: c_int, data: *mut c_void) -> bool {
    static bool test_intel(int idx, void *data)
    {
    if (boot_cpu_data.x86_vendor != X86_VENDOR_INTEL)
    return false;
// Rely on perf_msr_probe() to check the availability
    return true;
    }
    PMU_EVENT_ATTR_STRING(tsc,				attr_tsc,		"event=0x00"	);
    PMU_EVENT_ATTR_STRING(aperf,				attr_aperf,		"event=0x01"	);
    PMU_EVENT_ATTR_STRING(mperf,				attr_mperf,		"event=0x02"	);
    PMU_EVENT_ATTR_STRING(pperf,				attr_pperf,		"event=0x03"	);
    PMU_EVENT_ATTR_STRING(smi,				attr_smi,		"event=0x04"	);
    PMU_EVENT_ATTR_STRING(ptsc,				attr_ptsc,		"event=0x05"	);
    PMU_EVENT_ATTR_STRING(irperf,				attr_irperf,		"event=0x06"	);
    PMU_EVENT_ATTR_STRING(cpu_thermal_margin,		attr_therm,		"event=0x07"	);
    PMU_EVENT_ATTR_STRING(cpu_thermal_margin.snapshot,	attr_therm_snap,	"1"		);
    PMU_EVENT_ATTR_STRING(cpu_thermal_margin.unit,		attr_therm_unit,	"C"		);
    static unsigned long msr_mask;
    PMU_EVENT_GROUP(events, aperf);
    PMU_EVENT_GROUP(events, mperf);
    PMU_EVENT_GROUP(events, pperf);
    PMU_EVENT_GROUP(events, smi);
    PMU_EVENT_GROUP(events, ptsc);
    PMU_EVENT_GROUP(events, irperf);
    static struct attribute *attrs_therm[] = {
    &attr_therm.attr.attr,
    &attr_therm_snap.attr.attr,
    &attr_therm_unit.attr.attr,
    core::ptr::null_mut(),
    };
    static struct attribute_group group_therm = {
    .name  = "events",
    .attrs = attrs_therm,
    };
    static struct perf_msr msr[] = {
    [PERF_MSR_TSC]		= { .no_check = true,								},
    [PERF_MSR_APERF]	= { MSR_IA32_APERF,		&group_aperf,		test_aperfmperf,	},
    [PERF_MSR_MPERF]	= { MSR_IA32_MPERF,		&group_mperf,		test_aperfmperf,	},
    [PERF_MSR_PPERF]	= { MSR_PPERF,			&group_pperf,		test_intel,		},
    [PERF_MSR_SMI]		= { MSR_SMI_COUNT,		&group_smi,		test_intel,		},
    [PERF_MSR_PTSC]		= { MSR_F15H_PTSC,		&group_ptsc,		test_ptsc,		},
    [PERF_MSR_IRPERF]	= { MSR_F17H_IRPERF,		&group_irperf,		test_irperf,		},
    [PERF_MSR_THERM]	= { MSR_IA32_THERM_STATUS,	&group_therm,		test_therm_status,	},
    };
    static struct attribute *events_attrs[] = {
    &attr_tsc.attr.attr,
    core::ptr::null_mut(),
    };
    static struct attribute_group events_attr_group = {
    .name = "events",
    .attrs = events_attrs,
    };
    PMU_FORMAT_ATTR(event, "config:0-63");
    static struct attribute *format_attrs[] = {
    &format_attr_event.attr,
    core::ptr::null_mut(),
    };
    static struct attribute_group format_attr_group = {
    .name = "format",
    .attrs = format_attrs,
    };
    static const struct attribute_group *attr_groups[] = {
    &events_attr_group,
    &format_attr_group,
    core::ptr::null_mut(),
    };
    static const struct attribute_group *attr_update[] = {
    &group_aperf,
    &group_mperf,
    &group_pperf,
    &group_smi,
    &group_ptsc,
    &group_irperf,
    &group_therm,
    core::ptr::null_mut(),
    };
#[no_mangle]
unsafe extern "C" fn msr_event_init(event: *mut perf_event) -> c_int {
    static int msr_event_init(struct perf_event *event)
    {
    let mut cfg: u64 = event.attr.config;
    if (event.attr.type != event.pmu.type)
    return -ENOENT;
// unsupported modes and filters
    if (event.attr.sample_period) /* no sampling */
    return -EINVAL;
    if (cfg >= PERF_MSR_EVENT_MAX)
    return -EINVAL;
    cfg = array_index_nospec((unsigned long)cfg, PERF_MSR_EVENT_MAX);
    if (!(msr_mask & (1 << cfg)))
    return -EINVAL;
    event.hw.idx		= -1;
    event.hw.event_base	= msr[cfg].msr;
    event.hw.config	= cfg;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn msr_read_counter(event: *mut perf_event) -> u64 {
    static inline u64 msr_read_counter(struct perf_event *event)
    {
    u64 now;
    if (event.hw.event_base)
    rdmsrq(event.hw.event_base, now);
    else
    now = rdtsc_ordered();
    return now;
    }
#[no_mangle]
unsafe extern "C" fn msr_event_update(event: *mut perf_event) {
    static void msr_event_update(struct perf_event *event)
    {
    u64 prev, now;
    s64 delta;
// Careful, an NMI might modify the previous event value:
    prev = local64_read(&event.hw.prev_count);
    do {
    now = msr_read_counter(event);
    } while (!local64_try_cmpxchg(&event.hw.prev_count, &prev, now));
    delta = now - prev;
    if (unlikely(event.hw.event_base == MSR_SMI_COUNT)) {
    delta = sign_extend64(delta, 31);
    local64_add(delta, &event.count);
    } else if (unlikely(event.hw.event_base == MSR_IA32_THERM_STATUS)) {
// If valid, extract digital readout, otherwise set to -1:
    now = now & (1ULL << 31) ? (now >> 16) & 0x3f :  -1;
    local64_set(&event.count, now);
    } else {
    local64_add(delta, &event.count);
    }
    }
#[no_mangle]
unsafe extern "C" fn msr_event_start(event: *mut perf_event, flags: c_int) {
    static void msr_event_start(struct perf_event *event, int flags)
    {
    let mut now: u64 = msr_read_counter(event);
    local64_set(&event.hw.prev_count, now);
    }
#[no_mangle]
unsafe extern "C" fn msr_event_stop(event: *mut perf_event, flags: c_int) {
    static void msr_event_stop(struct perf_event *event, int flags)
    {
    msr_event_update(event);
    }
#[no_mangle]
unsafe extern "C" fn msr_event_del(event: *mut perf_event, flags: c_int) {
    static void msr_event_del(struct perf_event *event, int flags)
    {
    msr_event_stop(event, PERF_EF_UPDATE);
    }
#[no_mangle]
unsafe extern "C" fn msr_event_add(event: *mut perf_event, flags: c_int) -> c_int {
    static int msr_event_add(struct perf_event *event, int flags)
    {
    if (flags & PERF_EF_START)
    msr_event_start(event, flags);
    return 0;
    }
    static struct pmu pmu_msr = {
    .task_ctx_nr	= perf_sw_context,
    .attr_groups	= attr_groups,
    .event_init	= msr_event_init,
    .add		= msr_event_add,
    .del		= msr_event_del,
    .start		= msr_event_start,
    .stop		= msr_event_stop,
    .read		= msr_event_update,
    .capabilities	= PERF_PMU_CAP_NO_INTERRUPT | PERF_PMU_CAP_NO_EXCLUDE,
    .attr_update	= attr_update,
    };
#[no_mangle]
unsafe extern "C" fn msr_init() -> int __init {
    static int __init msr_init(void)
    {
    if (!boot_cpu_has(X86_FEATURE_TSC)) {
    pr_cont("no MSR PMU driver.\n");
    return 0;
    }
    msr_mask = perf_msr_probe(msr, PERF_MSR_EVENT_MAX, true, core::ptr::null_mut());
    perf_pmu_register(&pmu_msr, "msr", -1);
    return 0;
    }
    device_initcall(msr_init);
