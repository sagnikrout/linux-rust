//! Automatically rewritten from C to Rust
//! Source: tools/power/x86/turbostat/turbostat.c
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
// turbostat -- show CPU frequency and C-state residency
// on modern Intel and AMD processors.
//
// Copyright (c) 2010 - 2026 Intel Corporation
// Len Brown <len.brown@intel.com>
//
// Macro flag: #define _GNU_SOURCE

// copied from arch/x86/include/asm/cpu_device_id.h
pub const VFM_MODEL_BIT: c_int = 0;
pub const VFM_FAMILY_BIT: c_int = 8;
pub const VFM_VENDOR_BIT: c_int = 16;
pub const VFM_RSVD_BIT: c_int = 24;

    ((_model) << VFM_MODEL_BIT) |		\
    ((_family) << VFM_FAMILY_BIT) |		\
    ((_vendor) << VFM_VENDOR_BIT)		\
    )
// end copied section
pub const CPUID_LEAF_MODEL_ID: c_uint = 0x1A;
pub const CPUID_LEAF_MODEL_ID_CORE_TYPE_SHIFT: c_int = 24;
pub const X86_VENDOR_INTEL: c_int = 0;

//
// This list matches the column headers, except
// 1. built-in only, the sysfs counters are not here -- we learn of those at run-time
// 2. Core and CPU are moved to the end, we can't have strings that contain them
// matching on them for --show and --hide.
//
// buffer size used by sscanf() for added column names
// Usually truncated to 7 characters, but also handles 18 columns for raw 64-bit counters
//
pub const NAME_BYTES: c_int = 20;
pub const PATH_BYTES: c_int = 128;
pub const PERF_NAME_BYTES: c_int = 128;
pub const MAX_NOFILE: c_uint = 0x8000;

pub const PERF_DEV_NAME_BYTES: c_int = 32;
pub const PERF_EVT_NAME_BYTES: c_int = 32;
pub const INTEL_ECORE_TYPE: c_uint = 0x20;
pub const INTEL_PCORE_TYPE: c_uint = 0x40;

    enum counter_scope { SCOPE_CPU, SCOPE_CORE, SCOPE_PACKAGE };
    enum counter_type { COUNTER_ITEMS, COUNTER_CYCLES, COUNTER_SECONDS, COUNTER_USEC, COUNTER_K2M };
    enum counter_format { FORMAT_RAW, FORMAT_DELTA, FORMAT_PERCENT, FORMAT_AVERAGE };
    enum counter_source { COUNTER_SOURCE_NONE, COUNTER_SOURCE_PERF, COUNTER_SOURCE_MSR };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_counter_info {
    pub next: *mut perf_counter_info,
// How to open the counter / What counter it is.
    pub device: [c_char; PERF_DEV_NAME_BYTES],
    pub event: [c_char; PERF_EVT_NAME_BYTES],
// How to show/format the counter.
    pub name: [c_char; PERF_NAME_BYTES],
    pub width: c_uint,
    pub scope: enum counter_scope,
    pub type: enum counter_type,
    pub format: enum counter_format,
    pub scale: double,
// For reading the counter.
    pub fd_perf_per_domain: *mut c_int,
    pub num_domains: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sysfs_path {
    pub path: [c_char; PATH_BYTES],
    pub id: c_int,
    pub next: *mut sysfs_path,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msr_counter {
    pub msr_num: c_uint,
    pub name: [c_char; NAME_BYTES],
    pub sp: *mut sysfs_path,
    pub width: c_uint,
    pub type: enum counter_type,
    pub format: enum counter_format,
    pub next: *mut msr_counter,
    pub flags: c_uint,

}

    static int use_android_msr_path;
    struct msr_counter bic[] = {
    { 0x0, "usec", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "Time_Of_Day_Seconds", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "Package", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "Node", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "Avg_MHz", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "Busy%", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "Bzy_MHz", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "TSC_MHz", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "IRQ", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "SMI", core::ptr::null_mut(), 32, 0, FORMAT_DELTA, core::ptr::null_mut(), 0 },
    { 0x0, "cpuidle", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "CPU%c1", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "CPU%c3", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "CPU%c6", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "CPU%c7", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "ThreadC", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "CoreTmp", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "CoreCnt", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "PkgTmp", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "GFX%rc6", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "GFXMHz", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "Pkg%pc2", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "Pkg%pc3", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "Pkg%pc6", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "Pkg%pc7", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "Pkg%pc8", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "Pkg%pc9", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "Pk%pc10", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "CPU%LPI", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "SYS%LPI", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "PkgWatt", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "CorWatt", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "GFXWatt", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "PkgCnt", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "RAMWatt", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "PKG_%", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "RAM_%", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "Pkg_J", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "Cor_J", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "GFX_J", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "RAM_J", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "Mod%c6", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "Totl%C0", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "Any%C0", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "GFX%C0", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "CPUGFX%", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "Module", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "Core", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "CPU", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "APIC", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "X2APIC", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "Die", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "L3", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "GFXAMHz", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "IPC", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "CoreThr", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "UncMHz", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "SAM%mc6", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "SAMMHz", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "SAMAMHz", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "Die%c6", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "SysWatt", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "Sys_J", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "NMI", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "CPU%c1e", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "pct_idle", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "LLCMRPS", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "LLC%hit", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "L2MRPS", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    { 0x0, "L2%hit", core::ptr::null_mut(), 0, 0, 0, core::ptr::null_mut(), 0 },
    };
// n.b. bic_names must match the order in bic[], above
    enum bic_names {
    BIC_USEC,
    BIC_TOD,
    BIC_Package,
    BIC_Node,
    BIC_Avg_MHz,
    BIC_Busy,
    BIC_Bzy_MHz,
    BIC_TSC_MHz,
    BIC_IRQ,
    BIC_SMI,
    BIC_cpuidle,
    BIC_CPU_c1,
    BIC_CPU_c3,
    BIC_CPU_c6,
    BIC_CPU_c7,
    BIC_ThreadC,
    BIC_CoreTmp,
    BIC_CoreCnt,
    BIC_PkgTmp,
    BIC_GFX_rc6,
    BIC_GFXMHz,
    BIC_Pkgpc2,
    BIC_Pkgpc3,
    BIC_Pkgpc6,
    BIC_Pkgpc7,
    BIC_Pkgpc8,
    BIC_Pkgpc9,
    BIC_Pkgpc10,
    BIC_CPU_LPI,
    BIC_SYS_LPI,
    BIC_PkgWatt,
    BIC_CorWatt,
    BIC_GFXWatt,
    BIC_PkgCnt,
    BIC_RAMWatt,
    BIC_PKG__,
    BIC_RAM__,
    BIC_Pkg_J,
    BIC_Cor_J,
    BIC_GFX_J,
    BIC_RAM_J,
    BIC_Mod_c6,
    BIC_Totl_c0,
    BIC_Any_c0,
    BIC_GFX_c0,
    BIC_CPUGFX,
    BIC_Module,
    BIC_Core,
    BIC_CPU,
    BIC_APIC,
    BIC_X2APIC,
    BIC_Die,
    BIC_L3,
    BIC_GFXACTMHz,
    BIC_IPC,
    BIC_CORE_THROT_CNT,
    BIC_UNCORE_MHZ,
    BIC_SAM_mc6,
    BIC_SAMMHz,
    BIC_SAMACTMHz,
    BIC_Diec6,
    BIC_SysWatt,
    BIC_Sys_J,
    BIC_NMI,
    BIC_CPU_c1e,
    BIC_pct_idle,
    BIC_LLC_MRPS,
    BIC_LLC_HIT,
    BIC_L2_MRPS,
    BIC_L2_HIT,
    MAX_BIC
    };
#[no_mangle]
pub unsafe extern "C" fn print_bic_set(s: *mut c_char, set: *mut cpu_set_t) {
    void print_bic_set(char *s, cpu_set_t *set)
    {
    int i;
    assert(MAX_BIC < CPU_SETSIZE);
    printf("%s:", s);
    for (i = 0; i < MAX_BIC; ++i) {
    if (CPU_ISSET(i, set))
    printf(" %s", bic[i].name);
    }
    putchar('\n');
    }
    static cpu_set_t bic_group_topology;
    static cpu_set_t bic_group_thermal_pwr;
    static cpu_set_t bic_group_frequency;
    static cpu_set_t bic_group_hw_idle;
    static cpu_set_t bic_group_sw_idle;
    static cpu_set_t bic_group_idle;
    static cpu_set_t bic_group_cache;
    static cpu_set_t bic_group_other;
    static cpu_set_t bic_group_disabled_by_default;
    static cpu_set_t bic_enabled;
    static cpu_set_t bic_present;
// modify

// test

#[no_mangle]
unsafe extern "C" fn bic_set_all(set: *mut cpu_set_t) {
    static void bic_set_all(cpu_set_t *set)
    {
    int i;
    assert(MAX_BIC < CPU_SETSIZE);
    for (i = 0; i < MAX_BIC; ++i)
    SET_BIC(i, set);
    }
//
// bic_clear_bits()
// clear all the bits from "clr" in "dst"
//
#[no_mangle]
unsafe extern "C" fn bic_clear_bits(dst: *mut cpu_set_t, clr: *mut cpu_set_t) {
    static void bic_clear_bits(cpu_set_t *dst, cpu_set_t *clr)
    {
    int i;
    assert(MAX_BIC < CPU_SETSIZE);
    for (i = 0; i < MAX_BIC; ++i)
    if (CPU_ISSET(i, clr))
    CLR_BIC(i, dst);
    }
#[no_mangle]
unsafe extern "C" fn bic_groups_init() {
    static void bic_groups_init(void)
    {
    BIC_INIT(&bic_group_topology);
    SET_BIC(BIC_Package, &bic_group_topology);
    SET_BIC(BIC_Node, &bic_group_topology);
    SET_BIC(BIC_CoreCnt, &bic_group_topology);
    SET_BIC(BIC_PkgCnt, &bic_group_topology);
    SET_BIC(BIC_Module, &bic_group_topology);
    SET_BIC(BIC_Core, &bic_group_topology);
    SET_BIC(BIC_CPU, &bic_group_topology);
    SET_BIC(BIC_Die, &bic_group_topology);
    SET_BIC(BIC_L3, &bic_group_topology);
    BIC_INIT(&bic_group_thermal_pwr);
    SET_BIC(BIC_CoreTmp, &bic_group_thermal_pwr);
    SET_BIC(BIC_PkgTmp, &bic_group_thermal_pwr);
    SET_BIC(BIC_PkgWatt, &bic_group_thermal_pwr);
    SET_BIC(BIC_CorWatt, &bic_group_thermal_pwr);
    SET_BIC(BIC_GFXWatt, &bic_group_thermal_pwr);
    SET_BIC(BIC_RAMWatt, &bic_group_thermal_pwr);
    SET_BIC(BIC_PKG__, &bic_group_thermal_pwr);
    SET_BIC(BIC_RAM__, &bic_group_thermal_pwr);
    SET_BIC(BIC_SysWatt, &bic_group_thermal_pwr);
    BIC_INIT(&bic_group_frequency);
    SET_BIC(BIC_Avg_MHz, &bic_group_frequency);
    SET_BIC(BIC_Busy, &bic_group_frequency);
    SET_BIC(BIC_Bzy_MHz, &bic_group_frequency);
    SET_BIC(BIC_TSC_MHz, &bic_group_frequency);
    SET_BIC(BIC_GFXMHz, &bic_group_frequency);
    SET_BIC(BIC_GFXACTMHz, &bic_group_frequency);
    SET_BIC(BIC_SAMMHz, &bic_group_frequency);
    SET_BIC(BIC_SAMACTMHz, &bic_group_frequency);
    SET_BIC(BIC_UNCORE_MHZ, &bic_group_frequency);
    BIC_INIT(&bic_group_hw_idle);
    SET_BIC(BIC_Busy, &bic_group_hw_idle);
    SET_BIC(BIC_CPU_c1, &bic_group_hw_idle);
    SET_BIC(BIC_CPU_c3, &bic_group_hw_idle);
    SET_BIC(BIC_CPU_c6, &bic_group_hw_idle);
    SET_BIC(BIC_CPU_c7, &bic_group_hw_idle);
    SET_BIC(BIC_GFX_rc6, &bic_group_hw_idle);
    SET_BIC(BIC_Pkgpc2, &bic_group_hw_idle);
    SET_BIC(BIC_Pkgpc3, &bic_group_hw_idle);
    SET_BIC(BIC_Pkgpc6, &bic_group_hw_idle);
    SET_BIC(BIC_Pkgpc7, &bic_group_hw_idle);
    SET_BIC(BIC_Pkgpc8, &bic_group_hw_idle);
    SET_BIC(BIC_Pkgpc9, &bic_group_hw_idle);
    SET_BIC(BIC_Pkgpc10, &bic_group_hw_idle);
    SET_BIC(BIC_CPU_LPI, &bic_group_hw_idle);
    SET_BIC(BIC_SYS_LPI, &bic_group_hw_idle);
    SET_BIC(BIC_Mod_c6, &bic_group_hw_idle);
    SET_BIC(BIC_Totl_c0, &bic_group_hw_idle);
    SET_BIC(BIC_Any_c0, &bic_group_hw_idle);
    SET_BIC(BIC_GFX_c0, &bic_group_hw_idle);
    SET_BIC(BIC_CPUGFX, &bic_group_hw_idle);
    SET_BIC(BIC_SAM_mc6, &bic_group_hw_idle);
    SET_BIC(BIC_Diec6, &bic_group_hw_idle);
    BIC_INIT(&bic_group_sw_idle);
    SET_BIC(BIC_Busy, &bic_group_sw_idle);
    SET_BIC(BIC_cpuidle, &bic_group_sw_idle);
    SET_BIC(BIC_pct_idle, &bic_group_sw_idle);
    BIC_INIT(&bic_group_idle);
    CPU_OR(&bic_group_idle, &bic_group_idle, &bic_group_hw_idle);
    SET_BIC(BIC_pct_idle, &bic_group_idle);
    BIC_INIT(&bic_group_cache);
    SET_BIC(BIC_LLC_MRPS, &bic_group_cache);
    SET_BIC(BIC_LLC_HIT, &bic_group_cache);
    SET_BIC(BIC_L2_MRPS, &bic_group_cache);
    SET_BIC(BIC_L2_HIT, &bic_group_cache);
    BIC_INIT(&bic_group_other);
    SET_BIC(BIC_IRQ, &bic_group_other);
    SET_BIC(BIC_NMI, &bic_group_other);
    SET_BIC(BIC_SMI, &bic_group_other);
    SET_BIC(BIC_ThreadC, &bic_group_other);
    SET_BIC(BIC_CoreTmp, &bic_group_other);
    SET_BIC(BIC_IPC, &bic_group_other);
    BIC_INIT(&bic_group_disabled_by_default);
    SET_BIC(BIC_USEC, &bic_group_disabled_by_default);
    SET_BIC(BIC_TOD, &bic_group_disabled_by_default);
    SET_BIC(BIC_cpuidle, &bic_group_disabled_by_default);
    SET_BIC(BIC_APIC, &bic_group_disabled_by_default);
    SET_BIC(BIC_X2APIC, &bic_group_disabled_by_default);
    BIC_INIT(&bic_enabled);
    bic_set_all(&bic_enabled);
    bic_clear_bits(&bic_enabled, &bic_group_disabled_by_default);
    BIC_INIT(&bic_present);
    SET_BIC(BIC_USEC, &bic_present);
    SET_BIC(BIC_TOD, &bic_present);
    SET_BIC(BIC_cpuidle, &bic_present);
    SET_BIC(BIC_APIC, &bic_present);
    SET_BIC(BIC_X2APIC, &bic_present);
    SET_BIC(BIC_pct_idle, &bic_present);
    }
//
// MSR_PKG_CST_CONFIG_CONTROL decoding for pkg_cstate_limit:
// If you change the values, note they are used both in comparisons
// (>= PCL__7) and to index pkg_cstate_limit_strings[].
//

    char *proc_stat = "/proc/stat";
    FILE *outf;
    int *fd_percpu;
    int *fd_instr_count_percpu;
    int *fd_llc_percpu;
    int *fd_l2_percpu;
    let mut interval_tv: timeval = { 5, 0 };
    let mut interval_ts: timespec = { 5, 0 };
    unsigned int num_iterations;
    unsigned int header_iterations;
    unsigned int debug;
    unsigned int quiet;
    unsigned int shown;
    unsigned int sums_need_wide_columns;
    unsigned int rapl_joules;
    unsigned int valid_rapl_msrs;
    unsigned int summary_only;
    unsigned int list_header_only;
    unsigned int dump_only;
    unsigned int force_load;
    unsigned int cpuid_has_aperf_mperf;
    unsigned int cpuid_has_hv;
    unsigned int has_aperf_access;
    unsigned int has_epb;
    unsigned int has_turbo;
    unsigned int is_hybrid;
    unsigned int units = 1000000;	/* MHz etc */
    unsigned int genuine_intel;
    unsigned int authentic_amd;
    unsigned int hygon_genuine;
    unsigned int max_level, max_extended_level;
    unsigned int has_invariant_tsc;
    let mut aperf_mperf_multiplier: c_uint = 1;
    double bclk;
    double base_hz;
    unsigned int has_base_hz;
    let mut tsc_tweak: double = 1.0;
    unsigned int show_pkg_only;
    unsigned int show_core_only;
    char *output_buffer, *outp;
    unsigned int do_dts;
    unsigned int do_ptm;
    unsigned int do_ipc;
    unsigned long long cpuidle_cur_cpu_lpi_us;
    unsigned long long cpuidle_cur_sys_lpi_us;
    unsigned int tj_max;
    unsigned int tj_max_override;
    double rapl_power_units, rapl_time_units;
    double rapl_dram_energy_units, rapl_energy_units, rapl_psys_energy_units;
    double rapl_joule_counter_range;
    unsigned int crystal_hz;
    unsigned long long tsc_hz;
    int master_cpu;
    unsigned int has_hwp;		/* IA32_PM_ENABLE, IA32_HWP_CAPABILITIES */
// IA32_HWP_REQUEST, IA32_HWP_STATUS
    unsigned int has_hwp_notify;	/* IA32_HWP_INTERRUPT */
    unsigned int has_hwp_activity_window;	/* IA32_HWP_REQUEST[bits 41:32] */
    unsigned int has_hwp_epp;	/* IA32_HWP_REQUEST[bits 31:24] */
    unsigned int has_hwp_pkg;	/* IA32_HWP_REQUEST_PKG */
    let mut first_counter_read: c_uint = 1;
    static struct timeval procsysfs_tv_begin;
    int ignore_stdin;
    bool no_msr;
    bool no_perf;
    enum gfx_sysfs_idx {
    GFX_rc6,
    GFX_MHz,
    GFX_ACTMHz,
    SAM_mc6,
    SAM_MHz,
    SAM_ACTMHz,
    GFX_MAX
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfx_sysfs_info {
    pub fp: *mut FILE,
    pub val: c_uint,
    pub val_ull: c_ulonglong,
}

    static struct gfx_sysfs_info gfx_info[GFX_MAX];
    int get_msr(int cpu, off_t offset, unsigned long long *msr);
    int add_counter(unsigned int msr_num, char *path, char *name,
    unsigned int width, enum counter_scope scope, enum counter_type type, enum counter_format format, int flags, int package_num);
// Model specific support Start
// List of features that may diverge among different platforms
#[repr(C)]
#[derive(Copy, Clone)]
pub struct platform_features {
    pub /: *mut *mut bool has_msr_misc_feature_control; / MSR_MISC_FEATURE_CONTROL,
    pub /: *mut *mut bool has_msr_misc_pwr_mgmt; / MSR_MISC_PWR_MGMT,
    pub /: *mut *mut bool has_nhm_msrs; / MSR_PLATFORM_INFO, MSR_IA32_TEMPERATURE_TARGET, MSR_SMI_COUNT, MSR_PKG_CST_CONFIG_CONTROL, MSR_IA32_POWER_CTL, TRL MSRs,
    pub /: *mut *mut bool has_config_tdp; / MSR_CONFIG_TDP_NOMINAL/LEVEL_1/LEVEL_2/CONTROL, MSR_TURBO_ACTIVATION_RATIO,
    pub /: *mut *mut int bclk_freq; / CPU base clock,
    pub /: *mut *mut int crystal_freq; / Crystal clock to use when not available from CPUID.15,
    pub /: *mut *mut int supported_cstates; / Core cstates and Package cstates supported,
    pub /: *mut *mut int cst_limit; / MSR_PKG_CST_CONFIG_CONTROL,
    pub /: *mut *mut bool has_cst_auto_convension; / AUTOMATIC_CSTATE_CONVERSION bit in MSR_PKG_CST_CONFIG_CONTROL,
    pub /: *mut *mut bool has_irtl_msrs; / MSR_PKGC3/PKGC6/PKGC7/PKGC8/PKGC9/PKGC10_IRTL,
    pub /: *mut *mut bool has_msr_core_c1_res; / MSR_CORE_C1_RES,
    pub /: *mut *mut bool has_msr_module_c6_res_ms; / MSR_MODULE_C6_RES_MS,
    pub /: *mut *mut bool has_msr_c6_demotion_policy_config; / MSR_CC6_DEMOTION_POLICY_CONFIG/MSR_MC6_DEMOTION_POLICY_CONFIG,
    pub /: *mut *mut bool has_msr_atom_pkg_c6_residency; / MSR_ATOM_PKG_C6_RESIDENCY,
    pub /: *mut *mut bool has_msr_knl_core_c6_residency; / MSR_KNL_CORE_C6_RESIDENCY,
    pub /: *mut *mut bool has_ext_cst_msrs; / MSR_PKG_WEIGHTED_CORE_C0_RES/MSR_PKG_ANY_CORE_C0_RES/MSR_PKG_ANY_GFXE_C0_RES/MSR_PKG_BOTH_CORE_GFXE_C0_RES,
    pub /: *mut *mut bool has_cst_prewake_bit; / Cstate prewake bit in MSR_IA32_POWER_CTL,
    pub /: *mut *mut int trl_msrs; / MSR_TURBO_RATIO_LIMIT/LIMIT1/LIMIT2/SECONDARY, Atom TRL MSRs,
    pub /: *mut *mut int plr_msrs; / MSR_CORE/GFX/RING_PERF_LIMIT_REASONS,
    pub /: *mut *mut int plat_rapl_msrs; / RAPL PKG/DRAM/CORE/GFX MSRs, AMD RAPL MSRs,
    pub /: *mut *mut bool has_per_core_rapl; / Indicates cores energy collection is per-core, not per-package. AMD specific for now,
    pub /: *mut *mut bool has_rapl_divisor; / Divisor for Energy unit raw value from MSR_RAPL_POWER_UNIT,
    pub /: *mut *mut bool has_fixed_rapl_unit; / Fixed Energy Unit used for DRAM RAPL Domain,
    pub /: *mut *mut bool has_fixed_rapl_psys_unit; / Fixed Energy Unit used for PSYS RAPL Domain,
    pub /: *mut *mut int rapl_quirk_tdp; / Hardcoded TDP value when cannot be retrieved from hardware,
    pub /: *mut *mut int tcc_offset_bits; / TCC Offset bits in MSR_IA32_TEMPERATURE_TARGET,
    pub /: *mut *mut bool enable_tsc_tweak; / Use CPU Base freq instead of TSC freq for aperf/mperf counter,
    pub /: *mut *mut bool need_perf_multiplier; / mperf/aperf multiplier,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct platform_data {
    pub vfm: c_uint,
    pub features: *const platform_features,
}

// For BCLK
    enum bclk_freq {
    BCLK_100MHZ = 1,
    BCLK_133MHZ,
    BCLK_SLV,
    };
pub const SLM_BCLK_FREQS: c_int = 5;
    double slm_freq_table[SLM_BCLK_FREQS] = { 83.3, 100.0, 133.3, 116.7, 80.0 };
#[no_mangle]
pub unsafe extern "C" fn slm_bclk() -> double {
    double slm_bclk(void)
    {
    let mut msr: c_ulonglong = 3;
    unsigned int i;
    double freq;
    if (get_msr(master_cpu, MSR_FSB_FREQ, &msr))
    fprintf(outf, "SLM BCLK: unknown\n");
    i = msr & 0xf;
    if (i >= SLM_BCLK_FREQS) {
    fprintf(outf, "SLM BCLK[%d] invalid\n", i);
    i = 3;
    }
    freq = slm_freq_table[i];
    if (!quiet)
    fprintf(outf, "SLM BCLK: %.1f Mhz\n", freq);
    return freq;
    }
// For Package cstate limit
    enum package_cstate_limit {
    CST_LIMIT_NHM = 1,
    CST_LIMIT_SNB,
    CST_LIMIT_HSW,
    CST_LIMIT_SKX,
    CST_LIMIT_ICX,
    CST_LIMIT_SLV,
    CST_LIMIT_AMT,
    CST_LIMIT_KNL,
    CST_LIMIT_GMT,
    };
// For Turbo Ratio Limit MSRs
    enum turbo_ratio_limit_msrs {
    TRL_BASE = BIT(0),
    TRL_LIMIT1 = BIT(1),
    TRL_LIMIT2 = BIT(2),
    TRL_ATOM = BIT(3),
    TRL_KNL = BIT(4),
    TRL_CORECOUNT = BIT(5),
    };
// For Perf Limit Reason MSRs
    enum perf_limit_reason_msrs {
    PLR_CORE = BIT(0),
    PLR_GFX = BIT(1),
    PLR_RING = BIT(2),
    };
// For RAPL MSRs
    enum rapl_msrs {
    RAPL_PKG_POWER_LIMIT = BIT(0),	/* 0x610 MSR_PKG_POWER_LIMIT */
    RAPL_PKG_ENERGY_STATUS = BIT(1),	/* 0x611 MSR_PKG_ENERGY_STATUS */
    RAPL_PKG_PERF_STATUS = BIT(2),	/* 0x613 MSR_PKG_PERF_STATUS */
    RAPL_PKG_POWER_INFO = BIT(3),	/* 0x614 MSR_PKG_POWER_INFO */
    RAPL_DRAM_POWER_LIMIT = BIT(4),	/* 0x618 MSR_DRAM_POWER_LIMIT */
    RAPL_DRAM_ENERGY_STATUS = BIT(5),	/* 0x619 MSR_DRAM_ENERGY_STATUS */
    RAPL_DRAM_PERF_STATUS = BIT(6),	/* 0x61b MSR_DRAM_PERF_STATUS */
    RAPL_DRAM_POWER_INFO = BIT(7),	/* 0x61c MSR_DRAM_POWER_INFO */
    RAPL_CORE_POWER_LIMIT = BIT(8),	/* 0x638 MSR_PP0_POWER_LIMIT */
    RAPL_CORE_ENERGY_STATUS = BIT(9),	/* 0x639 MSR_PP0_ENERGY_STATUS */
    RAPL_CORE_POLICY = BIT(10),	/* 0x63a MSR_PP0_POLICY */
    RAPL_GFX_POWER_LIMIT = BIT(11),	/* 0x640 MSR_PP1_POWER_LIMIT */
    RAPL_GFX_ENERGY_STATUS = BIT(12),	/* 0x641 MSR_PP1_ENERGY_STATUS */
    RAPL_GFX_POLICY = BIT(13),	/* 0x642 MSR_PP1_POLICY */
    RAPL_AMD_PWR_UNIT = BIT(14),	/* 0xc0010299 MSR_AMD_RAPL_POWER_UNIT */
    RAPL_AMD_CORE_ENERGY_STAT = BIT(15),	/* 0xc001029a MSR_AMD_CORE_ENERGY_STATUS */
    RAPL_AMD_PKG_ENERGY_STAT = BIT(16),	/* 0xc001029b MSR_AMD_PKG_ENERGY_STATUS */
    RAPL_PLATFORM_ENERGY_LIMIT = BIT(17),	/* 0x64c MSR_PLATFORM_ENERGY_LIMIT */
    RAPL_PLATFORM_ENERGY_STATUS = BIT(18),	/* 0x64d MSR_PLATFORM_ENERGY_STATUS */
    };

// For Cstates
    enum cstates {
    CC1 = BIT(0),
    CC3 = BIT(1),
    CC6 = BIT(2),
    CC7 = BIT(3),
    PC2 = BIT(4),
    PC3 = BIT(5),
    PC6 = BIT(6),
    PC7 = BIT(7),
    PC8 = BIT(8),
    PC9 = BIT(9),
    PC10 = BIT(10),
    };
    static const struct platform_features nhm_features = {
    .has_msr_misc_pwr_mgmt = 1,
    .has_nhm_msrs = 1,
    .bclk_freq = BCLK_133MHZ,
    .supported_cstates = CC1 | CC3 | CC6 | PC3 | PC6,
    .cst_limit = CST_LIMIT_NHM,
    .trl_msrs = TRL_BASE,
    };
    static const struct platform_features nhx_features = {
    .has_msr_misc_pwr_mgmt = 1,
    .has_nhm_msrs = 1,
    .bclk_freq = BCLK_133MHZ,
    .supported_cstates = CC1 | CC3 | CC6 | PC3 | PC6,
    .cst_limit = CST_LIMIT_NHM,
    };
    static const struct platform_features snb_features = {
    .has_msr_misc_feature_control = 1,
    .has_msr_misc_pwr_mgmt = 1,
    .has_nhm_msrs = 1,
    .bclk_freq = BCLK_100MHZ,
    .supported_cstates = CC1 | CC3 | CC6 | CC7 | PC2 | PC3 | PC6 | PC7,
    .cst_limit = CST_LIMIT_SNB,
    .has_irtl_msrs = 1,
    .trl_msrs = TRL_BASE,
    .plat_rapl_msrs = RAPL_PKG | RAPL_CORE_ALL | RAPL_GFX | RAPL_PKG_POWER_INFO,
    };
    static const struct platform_features snx_features = {
    .has_msr_misc_feature_control = 1,
    .has_msr_misc_pwr_mgmt = 1,
    .has_nhm_msrs = 1,
    .bclk_freq = BCLK_100MHZ,
    .supported_cstates = CC1 | CC3 | CC6 | CC7 | PC2 | PC3 | PC6 | PC7,
    .cst_limit = CST_LIMIT_SNB,
    .has_irtl_msrs = 1,
    .trl_msrs = TRL_BASE,
    .plat_rapl_msrs = RAPL_PKG_ALL | RAPL_CORE_ALL | RAPL_DRAM_ALL,
    };
    static const struct platform_features ivb_features = {
    .has_msr_misc_feature_control = 1,
    .has_msr_misc_pwr_mgmt = 1,
    .has_nhm_msrs = 1,
    .has_config_tdp = 1,
    .bclk_freq = BCLK_100MHZ,
    .supported_cstates = CC1 | CC3 | CC6 | CC7 | PC2 | PC3 | PC6 | PC7,
    .cst_limit = CST_LIMIT_SNB,
    .has_irtl_msrs = 1,
    .trl_msrs = TRL_BASE,
    .plat_rapl_msrs = RAPL_PKG | RAPL_CORE_ALL | RAPL_GFX | RAPL_PKG_POWER_INFO,
    };
    static const struct platform_features ivx_features = {
    .has_msr_misc_feature_control = 1,
    .has_msr_misc_pwr_mgmt = 1,
    .has_nhm_msrs = 1,
    .bclk_freq = BCLK_100MHZ,
    .supported_cstates = CC1 | CC3 | CC6 | CC7 | PC2 | PC3 | PC6 | PC7,
    .cst_limit = CST_LIMIT_SNB,
    .has_irtl_msrs = 1,
    .trl_msrs = TRL_BASE | TRL_LIMIT1,
    .plat_rapl_msrs = RAPL_PKG_ALL | RAPL_CORE_ALL | RAPL_DRAM_ALL,
    };
    static const struct platform_features hsw_features = {
    .has_msr_misc_feature_control = 1,
    .has_msr_misc_pwr_mgmt = 1,
    .has_nhm_msrs = 1,
    .has_config_tdp = 1,
    .bclk_freq = BCLK_100MHZ,
    .supported_cstates = CC1 | CC3 | CC6 | CC7 | PC2 | PC3 | PC6 | PC7,
    .cst_limit = CST_LIMIT_HSW,
    .has_irtl_msrs = 1,
    .trl_msrs = TRL_BASE,
    .plr_msrs = PLR_CORE | PLR_GFX | PLR_RING,
    .plat_rapl_msrs = RAPL_PKG | RAPL_CORE_ALL | RAPL_GFX | RAPL_PKG_POWER_INFO,
    };
    static const struct platform_features hsx_features = {
    .has_msr_misc_feature_control = 1,
    .has_msr_misc_pwr_mgmt = 1,
    .has_nhm_msrs = 1,
    .has_config_tdp = 1,
    .bclk_freq = BCLK_100MHZ,
    .supported_cstates = CC1 | CC3 | CC6 | CC7 | PC2 | PC3 | PC6 | PC7,
    .cst_limit = CST_LIMIT_HSW,
    .has_irtl_msrs = 1,
    .trl_msrs = TRL_BASE | TRL_LIMIT1 | TRL_LIMIT2,
    .plr_msrs = PLR_CORE | PLR_RING,
    .plat_rapl_msrs = RAPL_PKG_ALL | RAPL_DRAM_ALL,
    .has_fixed_rapl_unit = 1,
    };
    static const struct platform_features hswl_features = {
    .has_msr_misc_feature_control = 1,
    .has_msr_misc_pwr_mgmt = 1,
    .has_nhm_msrs = 1,
    .has_config_tdp = 1,
    .bclk_freq = BCLK_100MHZ,
    .supported_cstates = CC1 | CC3 | CC6 | CC7 | PC2 | PC3 | PC6 | PC7 | PC8 | PC9 | PC10,
    .cst_limit = CST_LIMIT_HSW,
    .has_irtl_msrs = 1,
    .trl_msrs = TRL_BASE,
    .plr_msrs = PLR_CORE | PLR_GFX | PLR_RING,
    .plat_rapl_msrs = RAPL_PKG | RAPL_CORE_ALL | RAPL_GFX | RAPL_PKG_POWER_INFO,
    };
    static const struct platform_features hswg_features = {
    .has_msr_misc_feature_control = 1,
    .has_msr_misc_pwr_mgmt = 1,
    .has_nhm_msrs = 1,
    .has_config_tdp = 1,
    .bclk_freq = BCLK_100MHZ,
    .supported_cstates = CC1 | CC3 | CC6 | CC7 | PC2 | PC3 | PC6 | PC7,
    .cst_limit = CST_LIMIT_HSW,
    .has_irtl_msrs = 1,
    .trl_msrs = TRL_BASE,
    .plr_msrs = PLR_CORE | PLR_GFX | PLR_RING,
    .plat_rapl_msrs = RAPL_PKG | RAPL_CORE_ALL | RAPL_GFX | RAPL_PKG_POWER_INFO,
    };
    static const struct platform_features bdw_features = {
    .has_msr_misc_feature_control = 1,
    .has_msr_misc_pwr_mgmt = 1,
    .has_nhm_msrs = 1,
    .has_config_tdp = 1,
    .bclk_freq = BCLK_100MHZ,
    .supported_cstates = CC1 | CC3 | CC6 | CC7 | PC2 | PC3 | PC6 | PC7 | PC8 | PC9 | PC10,
    .cst_limit = CST_LIMIT_HSW,
    .has_irtl_msrs = 1,
    .trl_msrs = TRL_BASE,
    .plat_rapl_msrs = RAPL_PKG | RAPL_CORE_ALL | RAPL_GFX | RAPL_PKG_POWER_INFO,
    };
    static const struct platform_features bdwg_features = {
    .has_msr_misc_feature_control = 1,
    .has_msr_misc_pwr_mgmt = 1,
    .has_nhm_msrs = 1,
    .has_config_tdp = 1,
    .bclk_freq = BCLK_100MHZ,
    .supported_cstates = CC1 | CC3 | CC6 | CC7 | PC2 | PC3 | PC6 | PC7,
    .cst_limit = CST_LIMIT_HSW,
    .has_irtl_msrs = 1,
    .trl_msrs = TRL_BASE,
    .plat_rapl_msrs = RAPL_PKG | RAPL_CORE_ALL | RAPL_GFX | RAPL_PKG_POWER_INFO,
    };
    static const struct platform_features bdx_features = {
    .has_msr_misc_feature_control = 1,
    .has_msr_misc_pwr_mgmt = 1,
    .has_nhm_msrs = 1,
    .has_config_tdp = 1,
    .bclk_freq = BCLK_100MHZ,
    .supported_cstates = CC1 | CC3 | CC6 | PC2 | PC3 | PC6,
    .cst_limit = CST_LIMIT_HSW,
    .has_irtl_msrs = 1,
    .has_cst_auto_convension = 1,
    .trl_msrs = TRL_BASE,
    .plat_rapl_msrs = RAPL_PKG_ALL | RAPL_DRAM_ALL,
    .has_fixed_rapl_unit = 1,
    };
    static const struct platform_features skl_features = {
    .has_msr_misc_feature_control = 1,
    .has_msr_misc_pwr_mgmt = 1,
    .has_nhm_msrs = 1,
    .has_config_tdp = 1,
    .bclk_freq = BCLK_100MHZ,
    .crystal_freq = 24000000,
    .supported_cstates = CC1 | CC3 | CC6 | CC7 | PC2 | PC3 | PC6 | PC7 | PC8 | PC9 | PC10,
    .cst_limit = CST_LIMIT_HSW,
    .has_irtl_msrs = 1,
    .has_ext_cst_msrs = 1,
    .trl_msrs = TRL_BASE,
    .tcc_offset_bits = 6,
    .plat_rapl_msrs = RAPL_PKG_ALL | RAPL_CORE_ALL | RAPL_DRAM | RAPL_DRAM_PERF_STATUS | RAPL_GFX | RAPL_PSYS,
    .enable_tsc_tweak = 1,
    };
    static const struct platform_features cnl_features = {
    .has_msr_misc_feature_control = 1,
    .has_msr_misc_pwr_mgmt = 1,
    .has_nhm_msrs = 1,
    .has_config_tdp = 1,
    .bclk_freq = BCLK_100MHZ,
    .supported_cstates = CC1 | CC6 | CC7 | PC2 | PC3 | PC6 | PC7 | PC8 | PC9 | PC10,
    .cst_limit = CST_LIMIT_HSW,
    .has_irtl_msrs = 1,
    .has_msr_core_c1_res = 1,
    .has_ext_cst_msrs = 1,
    .trl_msrs = TRL_BASE,
    .tcc_offset_bits = 6,
    .plat_rapl_msrs = RAPL_PKG_ALL | RAPL_CORE_ALL | RAPL_DRAM | RAPL_DRAM_PERF_STATUS | RAPL_GFX | RAPL_PSYS,
    .enable_tsc_tweak = 1,
    };
// Copied from cnl_features, with PC7/PC9 removed
    static const struct platform_features adl_features = {
    .has_msr_misc_feature_control	= cnl_features.has_msr_misc_feature_control,
    .has_msr_misc_pwr_mgmt		= cnl_features.has_msr_misc_pwr_mgmt,
    .has_nhm_msrs			= cnl_features.has_nhm_msrs,
    .has_config_tdp			= cnl_features.has_config_tdp,
    .bclk_freq			= cnl_features.bclk_freq,
    .supported_cstates		= CC1 | CC6 | CC7 | PC2 | PC3 | PC6 | PC8 | PC10,
    .cst_limit			= cnl_features.cst_limit,
    .has_irtl_msrs			= cnl_features.has_irtl_msrs,
    .has_msr_core_c1_res		= cnl_features.has_msr_core_c1_res,
    .has_ext_cst_msrs		= cnl_features.has_ext_cst_msrs,
    .trl_msrs			= cnl_features.trl_msrs,
    .tcc_offset_bits		= cnl_features.tcc_offset_bits,
    .plat_rapl_msrs			= cnl_features.plat_rapl_msrs,
    .enable_tsc_tweak		= cnl_features.enable_tsc_tweak,
    };
// Copied from adl_features, with PC3/PC8 removed
    static const struct platform_features lnl_features = {
    .has_msr_misc_feature_control	= adl_features.has_msr_misc_feature_control,
    .has_msr_misc_pwr_mgmt		= adl_features.has_msr_misc_pwr_mgmt,
    .has_nhm_msrs			= adl_features.has_nhm_msrs,
    .has_config_tdp			= adl_features.has_config_tdp,
    .bclk_freq			= adl_features.bclk_freq,
    .supported_cstates		= CC1 | CC6 | CC7 | PC2 | PC6 | PC10,
    .cst_limit			= adl_features.cst_limit,
    .has_irtl_msrs			= adl_features.has_irtl_msrs,
    .has_msr_core_c1_res		= adl_features.has_msr_core_c1_res,
    .has_ext_cst_msrs		= adl_features.has_ext_cst_msrs,
    .trl_msrs			= adl_features.trl_msrs,
    .tcc_offset_bits		= adl_features.tcc_offset_bits,
    .plat_rapl_msrs			= adl_features.plat_rapl_msrs,
    .enable_tsc_tweak		= adl_features.enable_tsc_tweak,
    };
    static const struct platform_features skx_features = {
    .has_msr_misc_feature_control = 1,
    .has_msr_misc_pwr_mgmt = 1,
    .has_nhm_msrs = 1,
    .has_config_tdp = 1,
    .bclk_freq = BCLK_100MHZ,
    .supported_cstates = CC1 | CC6 | PC2 | PC6,
    .cst_limit = CST_LIMIT_SKX,
    .has_irtl_msrs = 1,
    .has_cst_auto_convension = 1,
    .trl_msrs = TRL_BASE | TRL_CORECOUNT,
    .plat_rapl_msrs = RAPL_PKG_ALL | RAPL_DRAM_ALL,
    .has_fixed_rapl_unit = 1,
    };
    static const struct platform_features icx_features = {
    .has_msr_misc_feature_control = 1,
    .has_msr_misc_pwr_mgmt = 1,
    .has_nhm_msrs = 1,
    .has_config_tdp = 1,
    .bclk_freq = BCLK_100MHZ,
    .supported_cstates = CC1 | CC6 | PC2 | PC6,
    .cst_limit = CST_LIMIT_ICX,
    .has_msr_core_c1_res = 1,
    .has_irtl_msrs = 1,
    .has_cst_prewake_bit = 1,
    .trl_msrs = TRL_BASE | TRL_CORECOUNT,
    .plat_rapl_msrs = RAPL_PKG_ALL | RAPL_DRAM_ALL | RAPL_PSYS,
    .has_fixed_rapl_unit = 1,
    };
    static const struct platform_features spr_features = {
    .has_msr_misc_feature_control = 1,
    .has_msr_misc_pwr_mgmt = 1,
    .has_nhm_msrs = 1,
    .has_config_tdp = 1,
    .bclk_freq = BCLK_100MHZ,
    .supported_cstates = CC1 | CC6 | PC2 | PC6,
    .cst_limit = CST_LIMIT_SKX,
    .has_msr_core_c1_res = 1,
    .has_irtl_msrs = 1,
    .has_cst_prewake_bit = 1,
    .has_fixed_rapl_psys_unit = 1,
    .trl_msrs = TRL_BASE | TRL_CORECOUNT,
    .plat_rapl_msrs = RAPL_PKG_ALL | RAPL_DRAM_ALL | RAPL_PSYS,
    };
    static const struct platform_features dmr_features = {
    .has_msr_misc_feature_control	= spr_features.has_msr_misc_feature_control,
    .has_msr_misc_pwr_mgmt		= spr_features.has_msr_misc_pwr_mgmt,
    .has_nhm_msrs			= spr_features.has_nhm_msrs,
    .bclk_freq			= spr_features.bclk_freq,
    .supported_cstates		= spr_features.supported_cstates,
    .cst_limit			= spr_features.cst_limit,
    .has_msr_core_c1_res		= spr_features.has_msr_core_c1_res,
    .has_cst_prewake_bit		= spr_features.has_cst_prewake_bit,
    .has_fixed_rapl_psys_unit	= spr_features.has_fixed_rapl_psys_unit,
    .trl_msrs			= spr_features.trl_msrs,
    .has_msr_module_c6_res_ms	= 1,	/* DMR has Dual-Core-Module and MC6 MSR */
    .plat_rapl_msrs			= 0,	/* DMR does not have RAPL MSRs */
    .plr_msrs			= 0,	/* DMR does not have PLR  MSRs */
    .has_irtl_msrs			= 0,	/* DMR does not have IRTL MSRs */
    .has_config_tdp			= 0,	/* DMR does not have CTDP MSRs */
    };
    static const struct platform_features srf_features = {
    .has_msr_misc_feature_control = 1,
    .has_msr_misc_pwr_mgmt = 1,
    .has_nhm_msrs = 1,
    .has_config_tdp = 1,
    .bclk_freq = BCLK_100MHZ,
    .supported_cstates = CC1 | CC6 | PC2 | PC6,
    .cst_limit = CST_LIMIT_SKX,
    .has_msr_core_c1_res = 1,
    .has_msr_module_c6_res_ms = 1,
    .has_irtl_msrs = 1,
    .has_cst_prewake_bit = 1,
    .trl_msrs = TRL_BASE | TRL_CORECOUNT,
    .plat_rapl_msrs = RAPL_PKG_ALL | RAPL_DRAM_ALL | RAPL_PSYS,
    };
    static const struct platform_features grr_features = {
    .has_msr_misc_feature_control = 1,
    .has_msr_misc_pwr_mgmt = 1,
    .has_nhm_msrs = 1,
    .has_config_tdp = 1,
    .bclk_freq = BCLK_100MHZ,
    .supported_cstates = CC1 | CC6,
    .cst_limit = CST_LIMIT_SKX,
    .has_msr_core_c1_res = 1,
    .has_msr_module_c6_res_ms = 1,
    .has_irtl_msrs = 1,
    .has_cst_prewake_bit = 1,
    .trl_msrs = TRL_BASE | TRL_CORECOUNT,
    .plat_rapl_msrs = RAPL_PKG_ALL | RAPL_DRAM_ALL | RAPL_PSYS,
    };
    static const struct platform_features slv_features = {
    .has_nhm_msrs = 1,
    .bclk_freq = BCLK_SLV,
    .supported_cstates = CC1 | CC6 | PC6,
    .cst_limit = CST_LIMIT_SLV,
    .has_msr_core_c1_res = 1,
    .has_msr_module_c6_res_ms = 1,
    .has_msr_c6_demotion_policy_config = 1,
    .has_msr_atom_pkg_c6_residency = 1,
    .trl_msrs = TRL_ATOM,
    .plat_rapl_msrs = RAPL_PKG | RAPL_CORE,
    .has_rapl_divisor = 1,
    .rapl_quirk_tdp = 30,
    };
    static const struct platform_features slvd_features = {
    .has_msr_misc_pwr_mgmt = 1,
    .has_nhm_msrs = 1,
    .bclk_freq = BCLK_SLV,
    .supported_cstates = CC1 | CC6 | PC3 | PC6,
    .cst_limit = CST_LIMIT_SLV,
    .has_msr_atom_pkg_c6_residency = 1,
    .trl_msrs = TRL_BASE,
    .plat_rapl_msrs = RAPL_PKG | RAPL_CORE,
    .rapl_quirk_tdp = 30,
    };
    static const struct platform_features amt_features = {
    .has_nhm_msrs = 1,
    .bclk_freq = BCLK_133MHZ,
    .supported_cstates = CC1 | CC3 | CC6 | PC3 | PC6,
    .cst_limit = CST_LIMIT_AMT,
    .trl_msrs = TRL_BASE,
    };
    static const struct platform_features gmt_features = {
    .has_msr_misc_pwr_mgmt = 1,
    .has_nhm_msrs = 1,
    .bclk_freq = BCLK_100MHZ,
    .crystal_freq = 19200000,
    .supported_cstates = CC1 | CC3 | CC6 | CC7 | PC2 | PC3 | PC6 | PC7 | PC8 | PC9 | PC10,
    .cst_limit = CST_LIMIT_GMT,
    .has_irtl_msrs = 1,
    .trl_msrs = TRL_BASE | TRL_CORECOUNT,
    .plat_rapl_msrs = RAPL_PKG | RAPL_PKG_POWER_INFO,
    };
    static const struct platform_features gmtd_features = {
    .has_msr_misc_pwr_mgmt = 1,
    .has_nhm_msrs = 1,
    .bclk_freq = BCLK_100MHZ,
    .crystal_freq = 25000000,
    .supported_cstates = CC1 | CC6 | PC2 | PC6,
    .cst_limit = CST_LIMIT_GMT,
    .has_irtl_msrs = 1,
    .has_msr_core_c1_res = 1,
    .trl_msrs = TRL_BASE | TRL_CORECOUNT,
    .plat_rapl_msrs = RAPL_PKG_ALL | RAPL_DRAM_ALL | RAPL_CORE_ENERGY_STATUS,
    };
    static const struct platform_features gmtp_features = {
    .has_msr_misc_pwr_mgmt = 1,
    .has_nhm_msrs = 1,
    .bclk_freq = BCLK_100MHZ,
    .crystal_freq = 19200000,
    .supported_cstates = CC1 | CC3 | CC6 | CC7 | PC2 | PC3 | PC6 | PC7 | PC8 | PC9 | PC10,
    .cst_limit = CST_LIMIT_GMT,
    .has_irtl_msrs = 1,
    .trl_msrs = TRL_BASE,
    .plat_rapl_msrs = RAPL_PKG | RAPL_PKG_POWER_INFO,
    };
    static const struct platform_features tmt_features = {
    .has_msr_misc_pwr_mgmt = 1,
    .has_nhm_msrs = 1,
    .bclk_freq = BCLK_100MHZ,
    .supported_cstates = CC1 | CC6 | CC7 | PC2 | PC3 | PC6 | PC7 | PC8 | PC9 | PC10,
    .cst_limit = CST_LIMIT_GMT,
    .has_irtl_msrs = 1,
    .trl_msrs = TRL_BASE,
    .plat_rapl_msrs = RAPL_PKG_ALL | RAPL_CORE_ALL | RAPL_DRAM | RAPL_DRAM_PERF_STATUS | RAPL_GFX,
    .enable_tsc_tweak = 1,
    };
    static const struct platform_features tmtd_features = {
    .has_msr_misc_pwr_mgmt = 1,
    .has_nhm_msrs = 1,
    .bclk_freq = BCLK_100MHZ,
    .supported_cstates = CC1 | CC6,
    .cst_limit = CST_LIMIT_GMT,
    .has_irtl_msrs = 1,
    .trl_msrs = TRL_BASE | TRL_CORECOUNT,
    .plat_rapl_msrs = RAPL_PKG_ALL,
    };
    static const struct platform_features knl_features = {
    .has_msr_misc_pwr_mgmt = 1,
    .has_nhm_msrs = 1,
    .has_config_tdp = 1,
    .bclk_freq = BCLK_100MHZ,
    .supported_cstates = CC1 | CC6 | PC3 | PC6,
    .cst_limit = CST_LIMIT_KNL,
    .has_msr_knl_core_c6_residency = 1,
    .trl_msrs = TRL_KNL,
    .plat_rapl_msrs = RAPL_PKG_ALL | RAPL_DRAM_ALL,
    .has_fixed_rapl_unit = 1,
    .need_perf_multiplier = 1,
    };
    static const struct platform_features default_features = {
    };
    static const struct platform_features amd_features_with_rapl = {
    .plat_rapl_msrs = RAPL_AMD_F17H,
    .has_per_core_rapl = 1,
    .rapl_quirk_tdp = 280,	/* This is the max stock TDP of HEDT/Server Fam17h+ chips */
    };
    static const struct platform_data turbostat_pdata[] = {
    { INTEL_NEHALEM, &nhm_features },
    { INTEL_NEHALEM_G, &nhm_features },
    { INTEL_NEHALEM_EP, &nhm_features },
    { INTEL_NEHALEM_EX, &nhx_features },
    { INTEL_WESTMERE, &nhm_features },
    { INTEL_WESTMERE_EP, &nhm_features },
    { INTEL_WESTMERE_EX, &nhx_features },
    { INTEL_SANDYBRIDGE, &snb_features },
    { INTEL_SANDYBRIDGE_X, &snx_features },
    { INTEL_IVYBRIDGE, &ivb_features },
    { INTEL_IVYBRIDGE_X, &ivx_features },
    { INTEL_HASWELL, &hsw_features },
    { INTEL_HASWELL_X, &hsx_features },
    { INTEL_HASWELL_L, &hswl_features },
    { INTEL_HASWELL_G, &hswg_features },
    { INTEL_BROADWELL, &bdw_features },
    { INTEL_BROADWELL_G, &bdwg_features },
    { INTEL_BROADWELL_X, &bdx_features },
    { INTEL_BROADWELL_D, &bdx_features },
    { INTEL_SKYLAKE_L, &skl_features },
    { INTEL_SKYLAKE, &skl_features },
    { INTEL_SKYLAKE_X, &skx_features },
    { INTEL_KABYLAKE_L, &skl_features },
    { INTEL_KABYLAKE, &skl_features },
    { INTEL_COMETLAKE, &skl_features },
    { INTEL_COMETLAKE_L, &skl_features },
    { INTEL_CANNONLAKE_L, &cnl_features },
    { INTEL_ICELAKE_X, &icx_features },
    { INTEL_ICELAKE_D, &icx_features },
    { INTEL_ICELAKE_L, &cnl_features },
    { INTEL_ICELAKE_NNPI, &cnl_features },
    { INTEL_ROCKETLAKE, &cnl_features },
    { INTEL_TIGERLAKE_L, &cnl_features },
    { INTEL_TIGERLAKE, &cnl_features },
    { INTEL_SAPPHIRERAPIDS_X, &spr_features },
    { INTEL_EMERALDRAPIDS_X, &spr_features },
    { INTEL_GRANITERAPIDS_X, &spr_features },
    { INTEL_GRANITERAPIDS_D, &spr_features },
    { INTEL_DIAMONDRAPIDS_X, &dmr_features },
    { INTEL_LAKEFIELD, &cnl_features },
    { INTEL_ALDERLAKE, &adl_features },
    { INTEL_ALDERLAKE_L, &adl_features },
    { INTEL_RAPTORLAKE, &adl_features },
    { INTEL_RAPTORLAKE_P, &adl_features },
    { INTEL_RAPTORLAKE_S, &adl_features },
    { INTEL_BARTLETTLAKE, &adl_features },
    { INTEL_METEORLAKE, &adl_features },
    { INTEL_METEORLAKE_L, &adl_features },
    { INTEL_ARROWLAKE_H, &adl_features },
    { INTEL_ARROWLAKE_U, &adl_features },
    { INTEL_ARROWLAKE, &adl_features },
    { INTEL_LUNARLAKE_M, &lnl_features },
    { INTEL_PANTHERLAKE_L, &lnl_features },
    { INTEL_NOVALAKE, &lnl_features },
    { INTEL_NOVALAKE_L, &lnl_features },
    { INTEL_WILDCATLAKE_L, &lnl_features },
    { INTEL_ATOM_SILVERMONT, &slv_features },
    { INTEL_ATOM_SILVERMONT_D, &slvd_features },
    { INTEL_ATOM_AIRMONT, &amt_features },
    { INTEL_ATOM_GOLDMONT, &gmt_features },
    { INTEL_ATOM_GOLDMONT_D, &gmtd_features },
    { INTEL_ATOM_GOLDMONT_PLUS, &gmtp_features },
    { INTEL_ATOM_TREMONT_D, &tmtd_features },
    { INTEL_ATOM_TREMONT, &tmt_features },
    { INTEL_ATOM_TREMONT_L, &tmt_features },
    { INTEL_ATOM_GRACEMONT, &adl_features },
    { INTEL_ATOM_CRESTMONT_X, &srf_features },
    { INTEL_ATOM_CRESTMONT, &grr_features },
    { INTEL_ATOM_DARKMONT_X, &srf_features },
    { INTEL_XEON_PHI_KNL, &knl_features },
    { INTEL_XEON_PHI_KNM, &knl_features },
//
// Missing support for
// INTEL_ICELAKE
// INTEL_ATOM_SILVERMONT_MID
// INTEL_ATOM_SILVERMONT_MID2
// INTEL_ATOM_AIRMONT_NP
//
    { 0, core::ptr::null_mut() },
    };
    struct {
    unsigned int uniform;
    unsigned int pcore;
    unsigned int ecore;
    unsigned int lcore;
    } perf_pmu_types;
//
// Events are enumerated in https://github.com/intel/perfmon
// and tools/perf/pmu-events/arch/x86/.../cache.json
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_l2_events {
    pub /: *mut *mut unsigned long long refs; / L2_REQUEST.ALL,
    pub /: *mut *mut unsigned long long hits; / L2_REQUEST.HIT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_model_support {
    pub vfm: c_uint,
    pub first: perf_l2_events,
    pub second: perf_l2_events,
    pub third: perf_l2_events,
    pub perf_model_support: *mut },
// Perf Cache Events

//
// Enumerate up to three perf CPU PMU's in a system.
// The first, second, and third columns are populated without skipping, describing
// pcore, ecore, lcore PMUs, in order, if present.  (The associated PMU "type" field is
// read from sysfs in all cases.)  Eg.
//
// non-hybrid:
// GNR: pcore, {}, {}
// ADL-N: ecore, {}, {}
// hybrid:
// MTL: pcore, ecore, {}%
// ARL-H: pcore, ecore, lcore
// LNL: ecore, ecore%%, {}
//
// % MTL physical lcore share architecture and PMU with ecore, and are thus not enumerated separately.
// %% LNL physical lcore is enumerated by perf as ecore
//
    static struct perf_model_support turbostat_perf_model_support[] = {
    { INTEL_SAPPHIRERAPIDS_X, { PCE(0x00, 0xFF), PCE(0x00, 0xDF)}, {}, {} },
    { INTEL_EMERALDRAPIDS_X, { PCE(0x00, 0xFF), PCE(0x00, 0xDF)}, {}, {} },
    { INTEL_GRANITERAPIDS_X, { PCE(0x00, 0xFF), PCE(0x00, 0xDF)}, {}, {} },
    { INTEL_GRANITERAPIDS_D, { PCE(0x00, 0xFF), PCE(0x00, 0xDF)}, {}, {} },
    { INTEL_DIAMONDRAPIDS_X, { PCE(0x00, 0xFF), PCE(0x00, 0x5F)}, {}, {} },
    { INTEL_ATOM_GRACEMONT, { PCE(0x00, 0x00), PCE(0x00, 0x02)}, {}, {} },	/* ADL-N */
    { INTEL_ATOM_CRESTMONT_X, { PCE(0x00, 0x00), PCE(0x00, 0x02)}, {}, {} },	/* SRF */
    { INTEL_ATOM_CRESTMONT, { PCE(0x00, 0x00), PCE(0x00, 0x02)}, {}, {} },	/* GRR */
    { INTEL_ATOM_DARKMONT_X, { PCE(0x01, 0xFF), PCE(0x01, 0xBF)}, {}, {} },	/* CWF */
    { INTEL_ALDERLAKE, { PCE(0x00, 0xFF), PCE(0x00, 0xDF)}, { PCE(0x00, 0x00), PCE(0x00, 0x02)}, {} },
    { INTEL_ALDERLAKE, { PCE(0x00, 0xFF), PCE(0x00, 0xDF)}, { PCE(0x00, 0x00), PCE(0x00, 0x02)}, {} },
    { INTEL_ALDERLAKE_L, { PCE(0x00, 0xFF), PCE(0x00, 0xDF)}, { PCE(0x00, 0x00), PCE(0x00, 0x02)}, {} },
    { INTEL_RAPTORLAKE, { PCE(0x00, 0xFF), PCE(0x00, 0xDF)}, { PCE(0x00, 0x00), PCE(0x00, 0x02)}, {} },
    { INTEL_RAPTORLAKE_P, { PCE(0x00, 0xFF), PCE(0x00, 0xDF)}, { PCE(0x00, 0x00), PCE(0x00, 0x02)}, {} },
    { INTEL_RAPTORLAKE_S, { PCE(0x00, 0xFF), PCE(0x00, 0xDF)}, { PCE(0x00, 0x00), PCE(0x00, 0x02)}, {} },
    { INTEL_METEORLAKE_L, { PCE(0x00, 0xFF), PCE(0x00, 0xDF)}, { PCE(0x00, 0x00), PCE(0x00, 0x02)}, {} },
    { INTEL_METEORLAKE, { PCE(0x00, 0xFF), PCE(0x00, 0xDF)}, { PCE(0x00, 0x00), PCE(0x00, 0x02)}, {} },
    { INTEL_ARROWLAKE_U, { PCE(0x00, 0xFF), PCE(0x00, 0xDF)}, { PCE(0x00, 0x00), PCE(0x00, 0x02)}, {} },
    { INTEL_LUNARLAKE_M, { PCE(0x00, 0xFF), PCE(0x00, 0x5F)}, { PCE(0x00, 0x07), PCE(0x00, 0x02)}, {} },
    { INTEL_ARROWLAKE_H, { PCE(0x00, 0xFF), PCE(0x00, 0x5F)}, { PCE(0x00, 0x07), PCE(0x00, 0x02)}, { PCE(0x00, 0x00), PCE(0x00, 0x02)} },
    { INTEL_ARROWLAKE, { PCE(0x00, 0xFF), PCE(0x00, 0x5F)}, { PCE(0x00, 0x07), PCE(0x00, 0x02)}, {} },
    { INTEL_PANTHERLAKE_L, { PCE(0x00, 0xFF), PCE(0x00, 0x5F)}, { PCE(0x01, 0xFF), PCE(0x01, 0xBF)}, {} },
    { INTEL_WILDCATLAKE_L, { PCE(0x00, 0xFF), PCE(0x00, 0x5F)}, { PCE(0x01, 0xFF), PCE(0x01, 0xBF)}, {} },
    { INTEL_NOVALAKE, { PCE(0x00, 0xFF), PCE(0x00, 0x5F)}, { PCE(0x01, 0xFF), PCE(0x01, 0xBF)}, {} },
    { INTEL_NOVALAKE_L, { PCE(0x00, 0xFF), PCE(0x00, 0x5F)}, { PCE(0x01, 0xFF), PCE(0x01, 0xBF)}, {} },
    { 0, {}, {}, {} }
}

    static const struct platform_features *platform;
#[no_mangle]
pub unsafe extern "C" fn probe_platform_features(family: c_uint, model: c_uint) {
    void probe_platform_features(unsigned int family, unsigned int model)
    {
    int i;
    if (authentic_amd || hygon_genuine) {
// fallback to default features on unsupported models
    force_load++;
    if (max_extended_level >= 0x80000007) {
    unsigned int eax, ebx, ecx, edx;
    __cpuid(0x80000007, eax, ebx, ecx, edx);
// RAPL (Fam 17h+)
    if ((edx & (1 << 14)) && family >= 0x17)
    platform = &amd_features_with_rapl;
    }
    goto end;
    }
    if (!genuine_intel)
    goto end;
    for (i = 0; turbostat_pdata[i].features; i++) {
    if (VFM_FAMILY(turbostat_pdata[i].vfm) == family && VFM_MODEL(turbostat_pdata[i].vfm) == model) {
    platform = turbostat_pdata[i].features;
    return;
    }
    }
    end:
    if (force_load && !platform) {
    fprintf(outf, "Forced to run on unsupported platform!\n");
    platform = &default_features;
    }
    if (platform)
    return;
    fprintf(stderr, "Unsupported platform detected.\n\tSee RUN THE LATEST VERSION on turbostat(8)\n");
    exit(1);
    }
#[no_mangle]
pub unsafe extern "C" fn init_perf_model_support(family: c_uint, model: c_uint) {
    void init_perf_model_support(unsigned int family, unsigned int model)
    {
    int i;
    if (!genuine_intel)
    return;
    for (i = 0; turbostat_perf_model_support[i].vfm; i++) {
    if (VFM_FAMILY(turbostat_perf_model_support[i].vfm) == family && VFM_MODEL(turbostat_perf_model_support[i].vfm) == model) {
    perf_model_support = &turbostat_perf_model_support[i];
    return;
    }
    }
    }
// Model specific support End
pub const TJMAX_DEFAULT: c_int = 100;
// MSRs that are not yet in the kernel-provided header.
pub const MSR_RAPL_PWR_UNIT: c_uint = 0xc0010299;
pub const MSR_CORE_ENERGY_STAT: c_uint = 0xc001029a;
pub const MSR_PKG_ENERGY_STAT: c_uint = 0xc001029b;

    int backwards_count;
    char *progname;

    cpu_set_t *cpu_present_set, *cpu_possible_set, *cpu_effective_set, *cpu_allowed_set, *cpu_affinity_set, *cpu_subset;
    cpu_set_t *perf_pcore_set, *perf_ecore_set, *perf_lcore_set;
    size_t cpu_present_setsize, cpu_possible_setsize, cpu_effective_setsize, cpu_allowed_setsize, cpu_affinity_setsize, cpu_subset_size;
pub const MAX_ADDED_THREAD_COUNTERS: c_int = 24;
pub const MAX_ADDED_CORE_COUNTERS: c_int = 8;
pub const MAX_ADDED_PACKAGE_COUNTERS: c_int = 16;
pub const PMT_MAX_ADDED_THREAD_COUNTERS: c_int = 24;
pub const PMT_MAX_ADDED_CORE_COUNTERS: c_int = 8;
pub const PMT_MAX_ADDED_PACKAGE_COUNTERS: c_int = 16;
pub const BITMASK_SIZE: c_int = 32;

// Indexes used to map data read from perf and MSRs into global variables
    enum rapl_rci_index {
    RAPL_RCI_INDEX_ENERGY_PKG = 0,
    RAPL_RCI_INDEX_ENERGY_CORES = 1,
    RAPL_RCI_INDEX_DRAM = 2,
    RAPL_RCI_INDEX_GFX = 3,
    RAPL_RCI_INDEX_PKG_PERF_STATUS = 4,
    RAPL_RCI_INDEX_DRAM_PERF_STATUS = 5,
    RAPL_RCI_INDEX_CORE_ENERGY = 6,
    RAPL_RCI_INDEX_ENERGY_PLATFORM = 7,
    NUM_RAPL_COUNTERS,
    };
    enum rapl_unit {
    RAPL_UNIT_INVALID,
    RAPL_UNIT_JOULES,
    RAPL_UNIT_WATTS,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rapl_counter_info_t {
    pub data: [c_ulonglong; NUM_RAPL_COUNTERS],
    pub source: [enum counter_source; NUM_RAPL_COUNTERS],
    pub flags: [c_ulonglong; NUM_RAPL_COUNTERS],
    pub scale: [double; NUM_RAPL_COUNTERS],
    pub unit: [enum rapl_unit; NUM_RAPL_COUNTERS],
    pub msr: [c_ulonglong; NUM_RAPL_COUNTERS],
    pub msr_mask: [c_ulonglong; NUM_RAPL_COUNTERS],
    pub msr_shift: [c_int; NUM_RAPL_COUNTERS],
    pub fd_perf: c_int,
}

// struct rapl_counter_info_t for each RAPL domain
    struct rapl_counter_info_t *rapl_counter_info_perdomain;
    unsigned int rapl_counter_info_perdomain_size;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rapl_counter_arch_info {
    pub /: *mut *mut int feature_mask; / Mask for testing if the counter is supported on host,
    pub perf_subsys: *const c_char,
    pub perf_name: *const c_char,
    pub msr: c_ulonglong,
    pub msr_mask: c_ulonglong,
    pub /: *mut *mut int msr_shift; / Positive mean shift right, negative mean shift left,
    pub /: *mut *mut *mut double platform_rapl_msr_scale; / Scale applied to values read by MSR (platform dependent, filled at runtime),
    pub /: *mut *mut unsigned int rci_index; / Maps data from perf counters to global variables,
    pub bic_number: c_uint,
    pub /: *mut *mut double compat_scale; / Some counters require constant scaling to be in the same range as other, similar ones,
    pub flags: c_ulonglong,
}

    static const struct rapl_counter_arch_info rapl_counter_arch_infos[] = {
    {
    .feature_mask = RAPL_PKG,
    .perf_subsys = "power",
    .perf_name = "energy-pkg",
    .msr = MSR_PKG_ENERGY_STATUS,
    .msr_mask = 0xFFFFFFFFFFFFFFFF,
    .msr_shift = 0,
    .platform_rapl_msr_scale = &rapl_energy_units,
    .rci_index = RAPL_RCI_INDEX_ENERGY_PKG,
    .bic_number = BIC_PkgWatt,
    .compat_scale = 1.0,
    .flags = RAPL_COUNTER_FLAG_USE_MSR_SUM,
    },
    {
    .feature_mask = RAPL_PKG,
    .perf_subsys = "power",
    .perf_name = "energy-pkg",
    .msr = MSR_PKG_ENERGY_STATUS,
    .msr_mask = 0xFFFFFFFFFFFFFFFF,
    .msr_shift = 0,
    .platform_rapl_msr_scale = &rapl_energy_units,
    .rci_index = RAPL_RCI_INDEX_ENERGY_PKG,
    .bic_number = BIC_Pkg_J,
    .compat_scale = 1.0,
    .flags = RAPL_COUNTER_FLAG_USE_MSR_SUM,
    },
    {
    .feature_mask = RAPL_AMD_F17H,
    .perf_subsys = "power",
    .perf_name = "energy-pkg",
    .msr = MSR_PKG_ENERGY_STAT,
    .msr_mask = 0xFFFFFFFFFFFFFFFF,
    .msr_shift = 0,
    .platform_rapl_msr_scale = &rapl_energy_units,
    .rci_index = RAPL_RCI_INDEX_ENERGY_PKG,
    .bic_number = BIC_PkgWatt,
    .compat_scale = 1.0,
    .flags = RAPL_COUNTER_FLAG_USE_MSR_SUM,
    },
    {
    .feature_mask = RAPL_AMD_F17H,
    .perf_subsys = "power",
    .perf_name = "energy-pkg",
    .msr = MSR_PKG_ENERGY_STAT,
    .msr_mask = 0xFFFFFFFFFFFFFFFF,
    .msr_shift = 0,
    .platform_rapl_msr_scale = &rapl_energy_units,
    .rci_index = RAPL_RCI_INDEX_ENERGY_PKG,
    .bic_number = BIC_Pkg_J,
    .compat_scale = 1.0,
    .flags = RAPL_COUNTER_FLAG_USE_MSR_SUM,
    },
    {
    .feature_mask = RAPL_CORE_ENERGY_STATUS,
    .perf_subsys = "power",
    .perf_name = "energy-cores",
    .msr = MSR_PP0_ENERGY_STATUS,
    .msr_mask = 0xFFFFFFFFFFFFFFFF,
    .msr_shift = 0,
    .platform_rapl_msr_scale = &rapl_energy_units,
    .rci_index = RAPL_RCI_INDEX_ENERGY_CORES,
    .bic_number = BIC_CorWatt,
    .compat_scale = 1.0,
    .flags = RAPL_COUNTER_FLAG_USE_MSR_SUM,
    },
    {
    .feature_mask = RAPL_CORE_ENERGY_STATUS,
    .perf_subsys = "power",
    .perf_name = "energy-cores",
    .msr = MSR_PP0_ENERGY_STATUS,
    .msr_mask = 0xFFFFFFFFFFFFFFFF,
    .msr_shift = 0,
    .platform_rapl_msr_scale = &rapl_energy_units,
    .rci_index = RAPL_RCI_INDEX_ENERGY_CORES,
    .bic_number = BIC_Cor_J,
    .compat_scale = 1.0,
    .flags = RAPL_COUNTER_FLAG_USE_MSR_SUM,
    },
    {
    .feature_mask = RAPL_DRAM,
    .perf_subsys = "power",
    .perf_name = "energy-ram",
    .msr = MSR_DRAM_ENERGY_STATUS,
    .msr_mask = 0xFFFFFFFFFFFFFFFF,
    .msr_shift = 0,
    .platform_rapl_msr_scale = &rapl_dram_energy_units,
    .rci_index = RAPL_RCI_INDEX_DRAM,
    .bic_number = BIC_RAMWatt,
    .compat_scale = 1.0,
    .flags = RAPL_COUNTER_FLAG_USE_MSR_SUM,
    },
    {
    .feature_mask = RAPL_DRAM,
    .perf_subsys = "power",
    .perf_name = "energy-ram",
    .msr = MSR_DRAM_ENERGY_STATUS,
    .msr_mask = 0xFFFFFFFFFFFFFFFF,
    .msr_shift = 0,
    .platform_rapl_msr_scale = &rapl_dram_energy_units,
    .rci_index = RAPL_RCI_INDEX_DRAM,
    .bic_number = BIC_RAM_J,
    .compat_scale = 1.0,
    .flags = RAPL_COUNTER_FLAG_USE_MSR_SUM,
    },
    {
    .feature_mask = RAPL_GFX,
    .perf_subsys = "power",
    .perf_name = "energy-gpu",
    .msr = MSR_PP1_ENERGY_STATUS,
    .msr_mask = 0xFFFFFFFFFFFFFFFF,
    .msr_shift = 0,
    .platform_rapl_msr_scale = &rapl_energy_units,
    .rci_index = RAPL_RCI_INDEX_GFX,
    .bic_number = BIC_GFXWatt,
    .compat_scale = 1.0,
    .flags = RAPL_COUNTER_FLAG_USE_MSR_SUM,
    },
    {
    .feature_mask = RAPL_GFX,
    .perf_subsys = "power",
    .perf_name = "energy-gpu",
    .msr = MSR_PP1_ENERGY_STATUS,
    .msr_mask = 0xFFFFFFFFFFFFFFFF,
    .msr_shift = 0,
    .platform_rapl_msr_scale = &rapl_energy_units,
    .rci_index = RAPL_RCI_INDEX_GFX,
    .bic_number = BIC_GFX_J,
    .compat_scale = 1.0,
    .flags = RAPL_COUNTER_FLAG_USE_MSR_SUM,
    },
    {
    .feature_mask = RAPL_PKG_PERF_STATUS,
    .perf_subsys = core::ptr::null_mut(),
    .perf_name = core::ptr::null_mut(),
    .msr = MSR_PKG_PERF_STATUS,
    .msr_mask = 0xFFFFFFFFFFFFFFFF,
    .msr_shift = 0,
    .platform_rapl_msr_scale = &rapl_time_units,
    .rci_index = RAPL_RCI_INDEX_PKG_PERF_STATUS,
    .bic_number = BIC_PKG__,
    .compat_scale = 100.0,
    .flags = RAPL_COUNTER_FLAG_USE_MSR_SUM,
    },
    {
    .feature_mask = RAPL_DRAM_PERF_STATUS,
    .perf_subsys = core::ptr::null_mut(),
    .perf_name = core::ptr::null_mut(),
    .msr = MSR_DRAM_PERF_STATUS,
    .msr_mask = 0xFFFFFFFFFFFFFFFF,
    .msr_shift = 0,
    .platform_rapl_msr_scale = &rapl_time_units,
    .rci_index = RAPL_RCI_INDEX_DRAM_PERF_STATUS,
    .bic_number = BIC_RAM__,
    .compat_scale = 100.0,
    .flags = RAPL_COUNTER_FLAG_USE_MSR_SUM,
    },
    {
    .feature_mask = RAPL_AMD_F17H,
    .perf_subsys = core::ptr::null_mut(),
    .perf_name = core::ptr::null_mut(),
    .msr = MSR_CORE_ENERGY_STAT,
    .msr_mask = 0xFFFFFFFF,
    .msr_shift = 0,
    .platform_rapl_msr_scale = &rapl_energy_units,
    .rci_index = RAPL_RCI_INDEX_CORE_ENERGY,
    .bic_number = BIC_CorWatt,
    .compat_scale = 1.0,
    .flags = 0,
    },
    {
    .feature_mask = RAPL_AMD_F17H,
    .perf_subsys = core::ptr::null_mut(),
    .perf_name = core::ptr::null_mut(),
    .msr = MSR_CORE_ENERGY_STAT,
    .msr_mask = 0xFFFFFFFF,
    .msr_shift = 0,
    .platform_rapl_msr_scale = &rapl_energy_units,
    .rci_index = RAPL_RCI_INDEX_CORE_ENERGY,
    .bic_number = BIC_Cor_J,
    .compat_scale = 1.0,
    .flags = 0,
    },
    {
    .feature_mask = RAPL_PSYS,
    .perf_subsys = "power",
    .perf_name = "energy-psys",
    .msr = MSR_PLATFORM_ENERGY_STATUS,
    .msr_mask = 0x00000000FFFFFFFF,
    .msr_shift = 0,
    .platform_rapl_msr_scale = &rapl_psys_energy_units,
    .rci_index = RAPL_RCI_INDEX_ENERGY_PLATFORM,
    .bic_number = BIC_SysWatt,
    .compat_scale = 1.0,
    .flags = RAPL_COUNTER_FLAG_PLATFORM_COUNTER | RAPL_COUNTER_FLAG_USE_MSR_SUM,
    },
    {
    .feature_mask = RAPL_PSYS,
    .perf_subsys = "power",
    .perf_name = "energy-psys",
    .msr = MSR_PLATFORM_ENERGY_STATUS,
    .msr_mask = 0x00000000FFFFFFFF,
    .msr_shift = 0,
    .platform_rapl_msr_scale = &rapl_psys_energy_units,
    .rci_index = RAPL_RCI_INDEX_ENERGY_PLATFORM,
    .bic_number = BIC_Sys_J,
    .compat_scale = 1.0,
    .flags = RAPL_COUNTER_FLAG_PLATFORM_COUNTER | RAPL_COUNTER_FLAG_USE_MSR_SUM,
    },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rapl_counter {
    pub raw_value: c_ulonglong,
    pub unit: enum rapl_unit,
    pub scale: double,
}

// Indexes used to map data read from perf and MSRs into global variables
    enum ccstate_rci_index {
    CCSTATE_RCI_INDEX_C1_RESIDENCY = 0,
    CCSTATE_RCI_INDEX_C3_RESIDENCY = 1,
    CCSTATE_RCI_INDEX_C6_RESIDENCY = 2,
    CCSTATE_RCI_INDEX_C7_RESIDENCY = 3,
    PCSTATE_RCI_INDEX_C2_RESIDENCY = 4,
    PCSTATE_RCI_INDEX_C3_RESIDENCY = 5,
    PCSTATE_RCI_INDEX_C6_RESIDENCY = 6,
    PCSTATE_RCI_INDEX_C7_RESIDENCY = 7,
    PCSTATE_RCI_INDEX_C8_RESIDENCY = 8,
    PCSTATE_RCI_INDEX_C9_RESIDENCY = 9,
    PCSTATE_RCI_INDEX_C10_RESIDENCY = 10,
    NUM_CSTATE_COUNTERS,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cstate_counter_info_t {
    pub data: [c_ulonglong; NUM_CSTATE_COUNTERS],
    pub source: [enum counter_source; NUM_CSTATE_COUNTERS],
    pub msr: [c_ulonglong; NUM_CSTATE_COUNTERS],
    pub fd_perf_core: c_int,
    pub fd_perf_pkg: c_int,
}

    struct cstate_counter_info_t *ccstate_counter_info;
    unsigned int ccstate_counter_info_size;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cstate_counter_arch_info {
    pub /: *mut *mut int feature_mask; / Mask for testing if the counter is supported on host,
    pub perf_subsys: *const c_char,
    pub perf_name: *const c_char,
    pub msr: c_ulonglong,
    pub /: *mut *mut unsigned int rci_index; / Maps data from perf counters to global variables,
    pub bic_number: c_uint,
    pub flags: c_ulonglong,
    pub pkg_cstate_limit: c_int,
}

    static struct cstate_counter_arch_info ccstate_counter_arch_infos[] = {
    {
    .feature_mask = CC1,
    .perf_subsys = "cstate_core",
    .perf_name = "c1-residency",
    .msr = MSR_CORE_C1_RES,
    .rci_index = CCSTATE_RCI_INDEX_C1_RESIDENCY,
    .bic_number = BIC_CPU_c1,
    .flags = CSTATE_COUNTER_FLAG_COLLECT_PER_THREAD,
    .pkg_cstate_limit = 0,
    },
    {
    .feature_mask = CC3,
    .perf_subsys = "cstate_core",
    .perf_name = "c3-residency",
    .msr = MSR_CORE_C3_RESIDENCY,
    .rci_index = CCSTATE_RCI_INDEX_C3_RESIDENCY,
    .bic_number = BIC_CPU_c3,
    .flags = CSTATE_COUNTER_FLAG_COLLECT_PER_CORE | CSTATE_COUNTER_FLAG_SOFT_C1_DEPENDENCY,
    .pkg_cstate_limit = 0,
    },
    {
    .feature_mask = CC6,
    .perf_subsys = "cstate_core",
    .perf_name = "c6-residency",
    .msr = MSR_CORE_C6_RESIDENCY,
    .rci_index = CCSTATE_RCI_INDEX_C6_RESIDENCY,
    .bic_number = BIC_CPU_c6,
    .flags = CSTATE_COUNTER_FLAG_COLLECT_PER_CORE | CSTATE_COUNTER_FLAG_SOFT_C1_DEPENDENCY,
    .pkg_cstate_limit = 0,
    },
    {
    .feature_mask = CC7,
    .perf_subsys = "cstate_core",
    .perf_name = "c7-residency",
    .msr = MSR_CORE_C7_RESIDENCY,
    .rci_index = CCSTATE_RCI_INDEX_C7_RESIDENCY,
    .bic_number = BIC_CPU_c7,
    .flags = CSTATE_COUNTER_FLAG_COLLECT_PER_CORE | CSTATE_COUNTER_FLAG_SOFT_C1_DEPENDENCY,
    .pkg_cstate_limit = 0,
    },
    {
    .feature_mask = PC2,
    .perf_subsys = "cstate_pkg",
    .perf_name = "c2-residency",
    .msr = MSR_PKG_C2_RESIDENCY,
    .rci_index = PCSTATE_RCI_INDEX_C2_RESIDENCY,
    .bic_number = BIC_Pkgpc2,
    .flags = 0,
    .pkg_cstate_limit = PCL__2,
    },
    {
    .feature_mask = PC3,
    .perf_subsys = "cstate_pkg",
    .perf_name = "c3-residency",
    .msr = MSR_PKG_C3_RESIDENCY,
    .rci_index = PCSTATE_RCI_INDEX_C3_RESIDENCY,
    .bic_number = BIC_Pkgpc3,
    .flags = 0,
    .pkg_cstate_limit = PCL__3,
    },
    {
    .feature_mask = PC6,
    .perf_subsys = "cstate_pkg",
    .perf_name = "c6-residency",
    .msr = MSR_PKG_C6_RESIDENCY,
    .rci_index = PCSTATE_RCI_INDEX_C6_RESIDENCY,
    .bic_number = BIC_Pkgpc6,
    .flags = 0,
    .pkg_cstate_limit = PCL__6,
    },
    {
    .feature_mask = PC7,
    .perf_subsys = "cstate_pkg",
    .perf_name = "c7-residency",
    .msr = MSR_PKG_C7_RESIDENCY,
    .rci_index = PCSTATE_RCI_INDEX_C7_RESIDENCY,
    .bic_number = BIC_Pkgpc7,
    .flags = 0,
    .pkg_cstate_limit = PCL__7,
    },
    {
    .feature_mask = PC8,
    .perf_subsys = "cstate_pkg",
    .perf_name = "c8-residency",
    .msr = MSR_PKG_C8_RESIDENCY,
    .rci_index = PCSTATE_RCI_INDEX_C8_RESIDENCY,
    .bic_number = BIC_Pkgpc8,
    .flags = 0,
    .pkg_cstate_limit = PCL__8,
    },
    {
    .feature_mask = PC9,
    .perf_subsys = "cstate_pkg",
    .perf_name = "c9-residency",
    .msr = MSR_PKG_C9_RESIDENCY,
    .rci_index = PCSTATE_RCI_INDEX_C9_RESIDENCY,
    .bic_number = BIC_Pkgpc9,
    .flags = 0,
    .pkg_cstate_limit = PCL__9,
    },
    {
    .feature_mask = PC10,
    .perf_subsys = "cstate_pkg",
    .perf_name = "c10-residency",
    .msr = MSR_PKG_C10_RESIDENCY,
    .rci_index = PCSTATE_RCI_INDEX_C10_RESIDENCY,
    .bic_number = BIC_Pkgpc10,
    .flags = 0,
    .pkg_cstate_limit = PCL_10,
    },
    };
// Indexes used to map data read from perf and MSRs into global variables
    enum msr_rci_index {
    MSR_RCI_INDEX_APERF = 0,
    MSR_RCI_INDEX_MPERF = 1,
    MSR_RCI_INDEX_SMI = 2,
    NUM_MSR_COUNTERS,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msr_counter_info_t {
    pub data: [c_ulonglong; NUM_MSR_COUNTERS],
    pub source: [enum counter_source; NUM_MSR_COUNTERS],
    pub msr: [c_ulonglong; NUM_MSR_COUNTERS],
    pub msr_mask: [c_ulonglong; NUM_MSR_COUNTERS],
    pub fd_perf: c_int,
}

    struct msr_counter_info_t *msr_counter_info;
    unsigned int msr_counter_info_size;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msr_counter_arch_info {
    pub perf_subsys: *const c_char,
    pub perf_name: *const c_char,
    pub msr: c_ulonglong,
    pub msr_mask: c_ulonglong,
    pub /: *mut *mut unsigned int rci_index; / Maps data from perf counters to global variables,
    pub needed: bool,
    pub present: bool,
}

    enum msr_arch_info_index {
    MSR_ARCH_INFO_APERF_INDEX = 0,
    MSR_ARCH_INFO_MPERF_INDEX = 1,
    MSR_ARCH_INFO_SMI_INDEX = 2,
    };
    static struct msr_counter_arch_info msr_counter_arch_infos[] = {
    [MSR_ARCH_INFO_APERF_INDEX] = {
    .perf_subsys = "msr",
    .perf_name = "aperf",
    .msr = MSR_IA32_APERF,
    .msr_mask = 0xFFFFFFFFFFFFFFFF,
    .rci_index = MSR_RCI_INDEX_APERF,
    },
    [MSR_ARCH_INFO_MPERF_INDEX] = {
    .perf_subsys = "msr",
    .perf_name = "mperf",
    .msr = MSR_IA32_MPERF,
    .msr_mask = 0xFFFFFFFFFFFFFFFF,
    .rci_index = MSR_RCI_INDEX_MPERF,
    },
    [MSR_ARCH_INFO_SMI_INDEX] = {
    .perf_subsys = "msr",
    .perf_name = "smi",
    .msr = MSR_SMI_COUNT,
    .msr_mask = 0xFFFFFFFF,
    .rci_index = MSR_RCI_INDEX_SMI,
    },
    };
// Can be redefined when compiling, useful for testing.

pub const PMT_COUNTER_MTL_DC6_OFFSET: c_int = 120;
pub const PMT_COUNTER_MTL_DC6_LSB: c_int = 0;
pub const PMT_COUNTER_MTL_DC6_MSB: c_int = 63;
pub const PMT_MTL_DC6_GUID: c_uint = 0x1a067102;
pub const PMT_MTL_DC6_SEQ: c_int = 0;
pub const PMT_COUNTER_CWF_MC1E_OFFSET_BASE: c_int = 20936;
pub const PMT_COUNTER_CWF_MC1E_OFFSET_INCREMENT: c_int = 24;
pub const PMT_COUNTER_CWF_MC1E_NUM_MODULES_PER_FILE: c_int = 12;
pub const PMT_COUNTER_CWF_CPUS_PER_MODULE: c_int = 4;
pub const PMT_COUNTER_CWF_MC1E_LSB: c_int = 0;
pub const PMT_COUNTER_CWF_MC1E_MSB: c_int = 63;
pub const PMT_CWF_MC1E_GUID: c_uint = 0x14421519;
    let mut tcore_clock_freq_hz: c_ulonglong = 800000000;
pub const PMT_COUNTER_NAME_SIZE_BYTES: c_int = 16;
pub const PMT_COUNTER_TYPE_NAME_SIZE_BYTES: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmt_mmio {
    pub next: *mut pmt_mmio,
    pub guid: c_uint,
    pub size: c_uint,
// Base pointer to the mmaped memory.
    pub mmio_base: *mut c_void,
//
// Offset to be applied to the mmio_base
// to get the beginning of the PMT counters for given GUID.
//
    pub pmt_offset: c_ulong,
    pub pmt_mmios: *mut },
    enum pmt_datatype {
    PMT_TYPE_RAW,
    PMT_TYPE_XTAL_TIME,
    PMT_TYPE_TCORE_CLOCK,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmt_domain_info {
//
// Pointer to the MMIO obtained by applying a counter offset
// to the mmio_base of the mmaped region for the given GUID.
//
// This is where to read the raw value of the counter from.
//
    pub pcounter: *mut c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmt_counter {
    pub next: *mut pmt_counter,
// PMT metadata
    pub name: [c_char; PMT_COUNTER_NAME_SIZE_BYTES],
    pub type: enum pmt_datatype,
    pub scope: enum counter_scope,
    pub lsb: c_uint,
    pub msb: c_uint,
// BIC-like metadata
    pub format: enum counter_format,
    pub num_domains: c_uint,
    pub domains: *mut pmt_domain_info,
}

//
// PMT telemetry directory iterator.
// Used to iterate telemetry files in sysfs in correct order.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmt_diriter_t {
    pub dir: *mut DIR,
    pub namelist: *mut dirent,
    pub num_names: c_uint,
    pub current_name_idx: c_uint,
}

#[no_mangle]
pub unsafe extern "C" fn pmt_telemdir_filter(e: *const dirent) -> c_int {
    int pmt_telemdir_filter(const struct dirent *e)
    {
    unsigned int dummy;
    return sscanf(e.d_name, "telem%u", &dummy);
    }
#[no_mangle]
pub unsafe extern "C" fn pmt_telemdir_sort(a: *const dirent, b: *const dirent) -> c_int {
    int pmt_telemdir_sort(const struct dirent **a, const struct dirent **b)
    {
    let mut aidx: c_uint = 0, bidx = 0;
    sscanf((*a).d_name, "telem%u", &aidx);
    sscanf((*b).d_name, "telem%u", &bidx);
    return (aidx > bidx) ? 1 : (aidx < bidx) ? -1 : 0;
    }
    const struct dirent *pmt_diriter_next(struct pmt_diriter_t *iter)
    {
    const struct dirent *ret = core::ptr::null_mut();
    if (!iter.dir)
    return core::ptr::null_mut();
    if (iter.current_name_idx >= iter.num_names)
    return core::ptr::null_mut();
    ret = iter.namelist[iter.current_name_idx];
    ++iter.current_name_idx;
    return ret;
    }
    const struct dirent *pmt_diriter_begin(struct pmt_diriter_t *iter, const char *pmt_root_path)
    {
    let mut num_names: c_int = iter.num_names;
    if (!iter.dir) {
    iter.dir = opendir(pmt_root_path);
    if (iter.dir == core::ptr::null_mut())
    return core::ptr::null_mut();
    num_names = scandir(pmt_root_path, &iter.namelist, pmt_telemdir_filter, pmt_telemdir_sort);
    if (num_names == -1)
    return core::ptr::null_mut();
    }
    iter.current_name_idx = 0;
    iter.num_names = num_names;
    return pmt_diriter_next(iter);
    }
#[no_mangle]
pub unsafe extern "C" fn pmt_diriter_init(iter: *mut pmt_diriter_t) {
    void pmt_diriter_init(struct pmt_diriter_t *iter)
    {
    memset(iter, 0, sizeof(*iter));
    }
#[no_mangle]
pub unsafe extern "C" fn pmt_diriter_remove(iter: *mut pmt_diriter_t) {
    void pmt_diriter_remove(struct pmt_diriter_t *iter)
    {
    if (iter.namelist) {
    for (unsigned int i = 0; i < iter.num_names; i++) {
    free(iter.namelist[i]);
    iter.namelist[i] = core::ptr::null_mut();
    }
    }
    free(iter.namelist);
    iter.namelist = core::ptr::null_mut();
    iter.num_names = 0;
    iter.current_name_idx = 0;
    closedir(iter.dir);
    iter.dir = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn pmt_counter_get_width(p: *const pmt_counter) -> c_uint {
    unsigned int pmt_counter_get_width(const struct pmt_counter *p)
    {
    return (p.msb - p.lsb) + 1;
    }
#[no_mangle]
pub unsafe extern "C" fn pmt_counter_resize_(pcounter: *mut pmt_counter, new_size: c_uint) {
    void pmt_counter_resize_(struct pmt_counter *pcounter, unsigned int new_size)
    {
    struct pmt_domain_info *new_mem;
    new_mem = (struct pmt_domain_info *)reallocarray(pcounter.domains, new_size, sizeof(*pcounter.domains));
    if (!new_mem) {
    fprintf(stderr, "%s: failed to allocate memory for PMT counters\n", __func__);
    exit(1);
    }
// Zero initialize just allocated memory.
    let mut num_new_domains: usize = new_size - pcounter.num_domains;
    memset(&new_mem[pcounter.num_domains], 0, num_new_domains * sizeof(*pcounter.domains));
    pcounter.num_domains = new_size;
    pcounter.domains = new_mem;
    }
#[no_mangle]
pub unsafe extern "C" fn pmt_counter_resize(pcounter: *mut pmt_counter, new_size: c_uint) {
    void pmt_counter_resize(struct pmt_counter *pcounter, unsigned int new_size)
    {
//
// Allocate more memory ahead of time.
//
// Always allocate space for at least 8 elements
// and double the size when growing.
//
    if (new_size < 8)
    new_size = 8;
    new_size = MAX(new_size, pcounter.num_domains * 2);
    pmt_counter_resize_(pcounter, new_size);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct llc_stats {
    pub references: c_ulonglong,
    pub misses: c_ulonglong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2_stats {
    pub references: c_ulonglong,
    pub hits: c_ulonglong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct thread_data {
    pub tv_begin: timeval,
    pub tv_end: timeval,
    pub tv_delta: timeval,
    pub tsc: c_ulonglong,
    pub aperf: c_ulonglong,
    pub mperf: c_ulonglong,
    pub c1: c_ulonglong,
    pub instr_count: c_ulonglong,
    pub irq_count: c_ulonglong,
    pub nmi_count: c_ulonglong,
    pub smi_count: c_uint,
    pub llc: llc_stats,
    pub l2: l2_stats,
    pub cpu_id: c_uint,
    pub apic_id: c_uint,
    pub x2apic_id: c_uint,
    pub flags: c_uint,
    pub is_atom: bool,
    pub counter: [c_ulonglong; MAX_ADDED_THREAD_COUNTERS],
    pub perf_counter: [c_ulonglong; MAX_ADDED_THREAD_COUNTERS],
    pub pmt_counter: [c_ulonglong; PMT_MAX_ADDED_THREAD_COUNTERS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_data {
    pub first_cpu: c_int,
    pub c3: c_ulonglong,
    pub c6: c_ulonglong,
    pub c7: c_ulonglong,
    pub /: *mut *mut unsigned long long mc6_us; / duplicate as per-core for now, even though per module,
    pub core_temp_c: c_uint,
    pub /: *mut *mut rapl_counter core_energy; / MSR_CORE_ENERGY_STAT,
    pub core_throt_cnt: c_ulonglong,
    pub counter: [c_ulonglong; MAX_ADDED_CORE_COUNTERS],
    pub perf_counter: [c_ulonglong; MAX_ADDED_CORE_COUNTERS],
    pub pmt_counter: [c_ulonglong; PMT_MAX_ADDED_CORE_COUNTERS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkg_data {
    pub first_cpu: c_int,
    pub pc2: c_ulonglong,
    pub pc3: c_ulonglong,
    pub pc6: c_ulonglong,
    pub pc7: c_ulonglong,
    pub pc8: c_ulonglong,
    pub pc9: c_ulonglong,
    pub pc10: c_ulonglong,
    pub cpu_lpi: c_longlong,
    pub sys_lpi: c_longlong,
    pub pkg_wtd_core_c0: c_ulonglong,
    pub pkg_any_core_c0: c_ulonglong,
    pub pkg_any_gfxe_c0: c_ulonglong,
    pub pkg_both_core_gfxe_c0: c_ulonglong,
    pub gfx_rc6_ms: c_longlong,
    pub gfx_mhz: c_uint,
    pub gfx_act_mhz: c_uint,
    pub sam_mc6_ms: c_longlong,
    pub sam_mhz: c_uint,
    pub sam_act_mhz: c_uint,
    pub /: *mut *mut rapl_counter energy_pkg; / MSR_PKG_ENERGY_STATUS,
    pub /: *mut *mut rapl_counter energy_dram; / MSR_DRAM_ENERGY_STATUS,
    pub /: *mut *mut rapl_counter energy_cores; / MSR_PP0_ENERGY_STATUS,
    pub /: *mut *mut rapl_counter energy_gfx; / MSR_PP1_ENERGY_STATUS,
    pub /: *mut *mut rapl_counter rapl_pkg_perf_status; / MSR_PKG_PERF_STATUS,
    pub /: *mut *mut rapl_counter rapl_dram_perf_status; / MSR_DRAM_PERF_STATUS,
    pub pkg_temp_c: c_uint,
    pub uncore_mhz: c_uint,
    pub die_c6: c_ulonglong,
    pub counter: [c_ulonglong; MAX_ADDED_PACKAGE_COUNTERS],
    pub perf_counter: [c_ulonglong; MAX_ADDED_PACKAGE_COUNTERS],
    pub pmt_counter: [c_ulonglong; PMT_MAX_ADDED_PACKAGE_COUNTERS],
}

//
// The accumulated sum of MSR is defined as a monotonic
// increasing MSR, it will be accumulated periodically,
// despite its register's bit width.
//
    enum {
    IDX_PKG_ENERGY,
    IDX_DRAM_ENERGY,
    IDX_PP0_ENERGY,
    IDX_PP1_ENERGY,
    IDX_PKG_PERF,
    IDX_DRAM_PERF,
    IDX_PSYS_ENERGY,
    IDX_COUNT,
    };
    int get_msr_sum(int cpu, off_t offset, unsigned long long *msr);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msr_sum_array {
// get_msr_sum() = sum + (get_msr() - last)
    struct {
// The accumulated MSR value is updated by the timer
    pub sum: c_ulonglong,
// The MSR footprint recorded in last timer
    pub last: c_ulonglong,
    pub entries: [}; IDX_COUNT],
}

// The percpu MSR sum array.
    struct msr_sum_array *per_cpu_msr_sum;
#[no_mangle]
pub unsafe extern "C" fn idx_to_offset(idx: c_int) -> off_t {
    off_t idx_to_offset(int idx)
    {
    off_t offset;
    switch (idx) {
    case IDX_PKG_ENERGY:
    if (platform.plat_rapl_msrs & RAPL_AMD_F17H)
    offset = MSR_PKG_ENERGY_STAT;
    else
    offset = MSR_PKG_ENERGY_STATUS;
    break;
    case IDX_DRAM_ENERGY:
    offset = MSR_DRAM_ENERGY_STATUS;
    break;
    case IDX_PP0_ENERGY:
    offset = MSR_PP0_ENERGY_STATUS;
    break;
    case IDX_PP1_ENERGY:
    offset = MSR_PP1_ENERGY_STATUS;
    break;
    case IDX_PKG_PERF:
    offset = MSR_PKG_PERF_STATUS;
    break;
    case IDX_DRAM_PERF:
    offset = MSR_DRAM_PERF_STATUS;
    break;
    case IDX_PSYS_ENERGY:
    offset = MSR_PLATFORM_ENERGY_STATUS;
    break;
    default:
    offset = -1;
    }
    return offset;
    }
#[no_mangle]
pub unsafe extern "C" fn offset_to_idx(offset: off_t) -> c_int {
    int offset_to_idx(off_t offset)
    {
    int idx;
    switch (offset) {
    case MSR_PKG_ENERGY_STATUS:
    case MSR_PKG_ENERGY_STAT:
    idx = IDX_PKG_ENERGY;
    break;
    case MSR_DRAM_ENERGY_STATUS:
    idx = IDX_DRAM_ENERGY;
    break;
    case MSR_PP0_ENERGY_STATUS:
    idx = IDX_PP0_ENERGY;
    break;
    case MSR_PP1_ENERGY_STATUS:
    idx = IDX_PP1_ENERGY;
    break;
    case MSR_PKG_PERF_STATUS:
    idx = IDX_PKG_PERF;
    break;
    case MSR_DRAM_PERF_STATUS:
    idx = IDX_DRAM_PERF;
    break;
    case MSR_PLATFORM_ENERGY_STATUS:
    idx = IDX_PSYS_ENERGY;
    break;
    default:
    idx = -1;
    }
    return idx;
    }
#[no_mangle]
pub unsafe extern "C" fn idx_valid(idx: c_int) -> c_int {
    int idx_valid(int idx)
    {
    switch (idx) {
    case IDX_PKG_ENERGY:
    return valid_rapl_msrs & (RAPL_PKG | RAPL_AMD_F17H);
    case IDX_DRAM_ENERGY:
    return valid_rapl_msrs & RAPL_DRAM;
    case IDX_PP0_ENERGY:
    return valid_rapl_msrs & RAPL_CORE_ENERGY_STATUS;
    case IDX_PP1_ENERGY:
    return valid_rapl_msrs & RAPL_GFX;
    case IDX_PKG_PERF:
    return valid_rapl_msrs & RAPL_PKG_PERF_STATUS;
    case IDX_DRAM_PERF:
    return valid_rapl_msrs & RAPL_DRAM_PERF_STATUS;
    case IDX_PSYS_ENERGY:
    return valid_rapl_msrs & RAPL_PSYS;
    default:
    return 0;
    }
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sys_counters {
// MSR added counters
    pub added_thread_counters: c_uint,
    pub added_core_counters: c_uint,
    pub added_package_counters: c_uint,
    pub tp: *mut msr_counter,
    pub cp: *mut msr_counter,
    pub pp: *mut msr_counter,
// perf added counters
    pub added_thread_perf_counters: c_uint,
    pub added_core_perf_counters: c_uint,
    pub added_package_perf_counters: c_uint,
    pub perf_tp: *mut perf_counter_info,
    pub perf_cp: *mut perf_counter_info,
    pub perf_pp: *mut perf_counter_info,
    pub pmt_tp: *mut pmt_counter,
    pub pmt_cp: *mut pmt_counter,
    pub pmt_pp: *mut pmt_counter,
    pub sys: },
#[no_mangle]
unsafe extern "C" fn free_msr_counters_(pp: *mut msr_counter) -> usize {
    static size_t free_msr_counters_(struct msr_counter **pp)
    {
    pub NULL: *mut *mut msr_counter p =,
    pub 0: size_t num_freed =,
    while (*pp) {
    pub pp: *mut p =,
    if (p.msr_num != 0) {
// pp = p->next;
    }
    pub &p->next: pp =,
    }
    pub num_freed: return,
    }
//
// Free all added counters accessed via msr.
//
#[no_mangle]
unsafe extern "C" fn free_sys_msr_counters() {
    static void free_sys_msr_counters(void)
    {
// Thread counters
    pub free_msr_counters_(&sys.tp): sys.added_thread_counters -=,
// Core counters
    pub free_msr_counters_(&sys.cp): sys.added_core_counters -=,
// Package counters
    pub free_msr_counters_(&sys.pp): sys.added_package_counters -=,
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct counters {
    pub threads: *mut thread_data,
    pub cores: *mut core_data,
    pub packages: *mut pkg_data,
    pub odd: } average, even,,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct platform_counters {
    pub /: *mut *mut rapl_counter energy_psys; / MSR_PLATFORM_ENERGY_STATUS,
    pub platform_counters_even: } platform_counters_odd,,

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_topology {
    pub cpu_id: c_int,
    pub /: *mut *mut int core_id; / unique within a package,
    pub module_id: c_int,
    pub package_id: c_int,
    pub die_id: c_int,
    pub l3_id: c_int,
    pub physical_node_id: c_int,
    pub /: *mut *mut int logical_node_id; / 0-based count within the package,
    pub /: *mut *mut int ht_id; / unique within a core,
    pub 1]: int ht_sibling_cpu_id[MAX_HT_ID +,
    pub type: c_int,
    pub /: *mut *mut *mut cpu_set_t put_ids; / Processing Unit/Thread IDs,
    pub cpus: *mut },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct topo_params {
    pub num_packages: c_int,
    pub num_die: c_int,
    pub num_cpus: c_int,
    pub /: *mut *mut int num_cores; / system wide,
    pub allowed_packages: c_int,
    pub allowed_cpus: c_int,
    pub allowed_cores: c_int,
    pub max_cpu_num: c_int,
    pub /: *mut *mut int max_core_id; / within a package,
    pub /: *mut *mut int min_module_id; / system wide,
    pub /: *mut *mut int max_module_id; / system wide,
    pub max_package_id: c_int,
    pub max_die_id: c_int,
    pub max_l3_id: c_int,
    pub max_node_num: c_int,
    pub nodes_per_pkg: c_int,
    pub cores_per_pkg: c_int,
    pub threads_per_core: c_int,
    pub topo: },
    pub tv_delta: timeval tv_even, tv_odd,,
    pub /: *mut *mut *mut int irq_column_2_cpu; / /proc/interrupts column numbers,
    pub /: *mut *mut *mut int irqs_per_cpu; / indexed by cpu_num,
    pub /: *mut *mut *mut int nmi_per_cpu; / indexed by cpu_num,
    pub startup): void setup_all_buffers(bool,
    pub sys_lpi_file: *mut c_char,
    pub "/sys/devices/system/cpu/cpuidle/low_power_idle_system_residency_us": *mut *mut char sys_lpi_file_sysfs =,
    pub "/sys/kernel/debug/pmc_core/slp_s0_residency_usec": *mut *mut char sys_lpi_file_debugfs =,
#[no_mangle]
pub unsafe extern "C" fn cpu_is_not_present(cpu: c_int) -> c_int {
    int cpu_is_not_present(int cpu)
    {
    if (cpu < 0)
    pub 1: return,
    pub cpu_present_set): return !CPU_ISSET_S(cpu, cpu_present_setsize,,
    }
#[no_mangle]
pub unsafe extern "C" fn cpu_is_not_allowed(cpu: c_int) -> c_int {
    int cpu_is_not_allowed(int cpu)
    {
    if (cpu < 0)
    pub 1: return,
    pub cpu_allowed_set): return !CPU_ISSET_S(cpu, cpu_allowed_setsize,,
    }

//
// run func(thread, core, package) in topology order
// skip non-present cpus
//

#[no_mangle]
pub unsafe extern "C" fn has_allowed_lower_ht_sibling(cpu: c_int) -> c_int {
    int has_allowed_lower_ht_sibling(int cpu)
    {
    pub i: c_int,
    pub {: for (i = 0; i <= cpus[cpu].ht_id; ++i),
    pub cpus: [int sibling_cpu_id =; cpu].ht_sibling_cpu_id[i],
    if (sibling_cpu_id == cpu)
    pub 0: return,
    if (!cpu_is_not_allowed(sibling_cpu_id))
    pub 1: return,
    }
    pub 0: return,
    }
    int for_all_cpus(int (func) (struct thread_data *, struct core_data *, struct pkg_data *),
    struct thread_data *thread_base, struct core_data *core_base, struct pkg_data *pkg_base)
    {
    pub retval: int cpu,,
    pub 0: retval =,
    pub {: for (cpu = 0; cpu <= topo.max_cpu_num; ++cpu),
    pub t: *mut thread_data,
    pub c: *mut core_data,
    pub p: *mut pkg_data,
    pub cpus[cpu].package_id: int pkg_id =,
    if (cpu_is_not_allowed(cpu))
    if (has_allowed_lower_ht_sibling(cpu))	/* skip HT sibling */
    pub &thread_base[cpu]: t =,
    pub pkg_id)]: c = &core_base[GLOBAL_CORE_ID(cpus[cpu].core_id,,
    pub &pkg_base[pkg_id]: p =,
    pub p): retval |= func(t, c,,
// Handle other HT siblings now
    pub i: c_int,
    pub {: for (i = 0; i <= MAX_HT_ID; ++i),
    pub cpus: [int sibling_cpu_id =; cpu].ht_sibling_cpu_id[i],
    if (sibling_cpu_id < 0)
    if (sibling_cpu_id == cpu)
    if (cpu_is_not_allowed(sibling_cpu_id))
    pub &thread_base[sibling_cpu_id]: t =,
    pub p): retval |= func(t, c,,
    }
    }
    pub retval: return,
    }
#[no_mangle]
pub unsafe extern "C" fn is_cpu_first_thread_in_core(t: *mut thread_data, c: *mut core_data) -> c_int {
    int is_cpu_first_thread_in_core(struct thread_data *t, struct core_data *c)
    {
    pub 0): return ((int)t->cpu_id == c->first_cpu || c->first_cpu <,
    }
#[no_mangle]
pub unsafe extern "C" fn is_cpu_first_core_in_package(t: *mut thread_data, p: *mut pkg_data) -> c_int {
    int is_cpu_first_core_in_package(struct thread_data *t, struct pkg_data *p)
    {
    pub 0): return ((int)t->cpu_id == p->first_cpu || p->first_cpu <,
    }
#[no_mangle]
pub unsafe extern "C" fn is_cpu_first_thread_in_package(t: *mut thread_data, c: *mut core_data, p: *mut pkg_data) -> c_int {
    int is_cpu_first_thread_in_package(struct thread_data *t, struct core_data *c, struct pkg_data *p)
    {
    pub p): return is_cpu_first_thread_in_core(t, c) && is_cpu_first_core_in_package(t,,
    }
#[no_mangle]
pub unsafe extern "C" fn cpu_migrate(cpu: c_int) -> c_int {
    int cpu_migrate(int cpu)
    {
    pub cpu_affinity_set): CPU_ZERO_S(cpu_affinity_setsize,,
    pub cpu_affinity_set): CPU_SET_S(cpu, cpu_affinity_setsize,,
    if (sched_setaffinity(0, cpu_affinity_setsize, cpu_affinity_set) == -1)
    pub -1: return,
    else
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn get_msr_fd(cpu: c_int) -> c_int {
    int get_msr_fd(int cpu)
    {
    pub pathname: [c_char; 32],
    pub fd: c_int,
    pub fd_percpu: [fd =; cpu],
    if (fd)
    pub fd: return,
    pub cpu): sprintf(pathname, use_android_msr_path ? "/dev/msr%d" : "/dev/cpu/%d/msr",,
    pub O_RDONLY): fd = open(pathname,,
    if (fd < 0)
    err(-1, "%s open failed, try chown or chmod +r %s, "
    pub "/dev/cpu/*/msr"): *mut *mut "or run with --no-msr, or run as root", pathname, use_android_msr_path ? "/dev/msr" :,
    pub fd: fd_percpu[cpu] =,
    pub fd: return,
    }
#[no_mangle]
unsafe extern "C" fn bic_disable_msr_access() {
    static void bic_disable_msr_access(void)
    {
    pub &bic_enabled): CLR_BIC(BIC_Mod_c6,,
    pub &bic_enabled): CLR_BIC(BIC_CoreTmp,,
    pub &bic_enabled): CLR_BIC(BIC_Totl_c0,,
    pub &bic_enabled): CLR_BIC(BIC_Any_c0,,
    pub &bic_enabled): CLR_BIC(BIC_GFX_c0,,
    pub &bic_enabled): CLR_BIC(BIC_CPUGFX,,
    pub &bic_enabled): CLR_BIC(BIC_PkgTmp,,
    }
#[no_mangle]
unsafe extern "C" fn bic_disable_perf_access() {
    static void bic_disable_perf_access(void)
    {
    pub &bic_enabled): CLR_BIC(BIC_IPC,,
    pub &bic_enabled): CLR_BIC(BIC_LLC_MRPS,,
    pub &bic_enabled): CLR_BIC(BIC_LLC_HIT,,
    pub &bic_enabled): CLR_BIC(BIC_L2_MRPS,,
    pub &bic_enabled): CLR_BIC(BIC_L2_HIT,,
    }
#[no_mangle]
unsafe extern "C" fn perf_event_open(hw_event: *mut perf_event_attr, pid: pid_t, cpu: c_int, group_fd: c_int, flags: c_ulong) -> c_long {
    static long perf_event_open(struct perf_event_attr *hw_event, pid_t pid, int cpu, int group_fd, unsigned long flags)
    {
    pub flags): return syscall(__NR_perf_event_open, hw_event, pid, cpu, group_fd,,
    }
#[no_mangle]
unsafe extern "C" fn open_perf_counter(cpu: c_int, type: c_uint, config: c_uint, group_fd: c_int, read_format: __u64) -> c_long {
    static long open_perf_counter(int cpu, unsigned int type, unsigned int config, int group_fd, __u64 read_format)
    {
    pub attr: perf_event_attr,
    pub -1: pid_t pid =,
    pub 0: unsigned long flags =,
    pub perf_event_attr)): memset(&attr, 0, sizeof(struct,
    pub type: attr.type =,
    pub perf_event_attr): attr.size = sizeof(struct,
    pub config: attr.config =,
    pub 0: attr.disabled =,
    pub PERF_SAMPLE_IDENTIFIER: attr.sample_type =,
    pub read_format: attr.read_format =,
    pub flags): int fd = perf_event_open(&attr, pid, cpu, group_fd,,
    pub fd: return,
    }
#[no_mangle]
pub unsafe extern "C" fn get_instr_count_fd(cpu: c_int) -> c_int {
    int get_instr_count_fd(int cpu)
    {
    if (fd_instr_count_percpu[cpu])
    pub fd_instr_count_percpu: [return; cpu],
    pub 0): fd_instr_count_percpu[cpu] = open_perf_counter(cpu, PERF_TYPE_HARDWARE, PERF_COUNT_HW_INSTRUCTIONS, -1,,
    pub fd_instr_count_percpu: [return; cpu],
    }
#[no_mangle]
pub unsafe extern "C" fn get_msr(cpu: c_int, offset: off_t, msr: *mut c_ulonglong) -> c_int {
    int get_msr(int cpu, off_t offset, unsigned long long *msr)
    {
    pub retval: isize,
    pub offset): *mut *mut retval = pread(get_msr_fd(cpu), msr, sizeof(msr),,
    if (retval != sizeof *msr)
    pub long)offset): err(-1, "cpu%d: msr offset 0x%llx read failed", cpu, (unsigned long,
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn add_msr_counter(cpu: c_int, offset: off_t) -> c_int {
    int add_msr_counter(int cpu, off_t offset)
    {
    pub retval: isize,
    pub value: c_ulonglong,
    if (no_msr)
    pub -1: return,
    if (!offset)
    pub -1: return,
    pub offset): retval = pread(get_msr_fd(cpu), &value, sizeof(value),,
// if the read failed, the probe fails
    if (retval != sizeof(value))
    pub -1: return,
    if (value == 0)
    pub 0: return,
    pub 1: return,
    }
#[no_mangle]
pub unsafe extern "C" fn add_rapl_msr_counter(cpu: c_int, cai: *const rapl_counter_arch_info) -> c_int {
    int add_rapl_msr_counter(int cpu, const struct rapl_counter_arch_info *cai)
    {
    pub ret: c_int,
    if (!(valid_rapl_msrs & cai.feature_mask))
    pub -1: return,
    pub cai->msr): ret = add_msr_counter(cpu,,
    if (ret < 0)
    pub -1: return,
    switch (cai.rci_index) {
    case RAPL_RCI_INDEX_ENERGY_PKG:
    case RAPL_RCI_INDEX_ENERGY_CORES:
    case RAPL_RCI_INDEX_DRAM:
    case RAPL_RCI_INDEX_GFX:
    case RAPL_RCI_INDEX_ENERGY_PLATFORM:
    if (ret == 0)
    pub 1: return,
    }
// PKG,DRAM_PERF_STATUS MSRs, can return any value
    pub 1: return,
    }
// Convert CPU ID to domain ID for given added perf counter.
#[no_mangle]
pub unsafe extern "C" fn cpu_to_domain(pc: *const perf_counter_info, cpu: c_int) -> c_uint {
    unsigned int cpu_to_domain(const struct perf_counter_info *pc, int cpu)
    {
    switch (pc.scope) {
    case SCOPE_CPU:
    pub cpu: return,
    case SCOPE_CORE:
    pub cpus[cpu].core_id: return,
    case SCOPE_PACKAGE:
    pub cpus[cpu].package_id: return,
    }
    }
pub const MAX_DEFERRED: c_int = 16;
    pub deferred_add_names: [*mut c_char; MAX_DEFERRED],
    pub deferred_skip_names: [*mut c_char; MAX_DEFERRED],
    pub deferred_add_index: c_int,
    pub deferred_skip_index: c_int,
    pub deferred_add_consumed: c_uint,
    pub deferred_skip_consumed: c_uint,
//
// HIDE_LIST - hide this list of counters, show the rest [default]
// SHOW_LIST - show this list of counters, hide the rest
//
    pub HIDE_LIST: enum show_hide_mode { SHOW_LIST, HIDE_LIST } global_show_hide_mode =,
#[no_mangle]
pub unsafe extern "C" fn help() {
    void help(void)
    {
    fprintf(outf,
    "Usage: turbostat [OPTIONS][(--interval seconds) | COMMAND ...]\n"
    "\n"
    "Turbostat forks the specified COMMAND and prints statistics\n"
    "when COMMAND completes.\n"
    "If no COMMAND is specified, turbostat wakes every 5-seconds\n"
    "to print statistics, until interrupted.\n"
    "  -a, --add counter\n"
    "		add a counter\n"
    "		  eg. --add msr0x10,u64,cpu,delta,MY_TSC\n"
    "		  eg. --add perf/cstate_pkg/c2-residency,package,delta,percent,perfPC2\n"
    "		  eg. --add pmt,name=XTAL,type=raw,domain=package0,offset=0,lsb=0,msb=63,guid=0x1a067102\n"
    "  -c, --cpu cpu-set\n"
    "		limit output to summary plus cpu-set:\n"
    "		  {core | package | j,k,l..m,n-p }\n"
    "  -d, --debug\n"
    "		displays usec, Time_Of_Day_Seconds and more debugging\n"
    "		debug messages are printed to stderr\n"
    "  -D, --Dump\n"
    "		displays the raw counter values\n"
    "  -e, --enable [all | column]\n"
    "		shows all or the specified disabled column\n"
    "  -f, --force\n"
    "		force load turbostat with minimum default features on unsupported platforms.\n"
    "  -H, --hide [column | column,column,...]\n"
    "		hide the specified column(s)\n"
    "  -i, --interval sec.subsec\n"
    "		override default 5-second measurement interval\n"
    "  -J, --Joules\n"
    "		displays energy in Joules instead of Watts\n"
    "  -l, --list\n"
    "		list column headers only\n"
    "  -M, --no-msr\n"
    "		disable all uses of the MSR driver\n"
    "  -P, --no-perf\n"
    "		disable all uses of the perf API\n"
    "  -n, --num_iterations num\n"
    "		number of the measurement iterations\n"
    "  -N, --header_iterations num\n"
    "		print header every num iterations\n"
    "  -o, --out file\n"
    "		create or truncate \"file\" for all output\n"
    "  -q, --quiet\n"
    "		skip decoding system configuration header\n"
    "  -s, --show [column | column,column,...]\n"
    "		show only the specified column(s)\n"
    "  -S, --Summary\n"
    "		limits output to 1-line system summary per interval\n"
    "  -T, --TCC temperature\n"
    "		sets the Thermal Control Circuit temperature in\n"
    "		  degrees Celsius\n"
    "  -h, --help\n"
    pub turbostat\"\n"): " print this help message\n -v, --version\n\t\tprint version information\n\nFor more help, run \"man,
    }
//
// bic_lookup
// for all the strings in comma separate name_list,
// set the approprate bit in return value.
//
#[no_mangle]
pub unsafe extern "C" fn bic_lookup(ret_set: *mut cpu_set_t, name_list: *mut c_char, mode: enum show_hide_mode) {
    void bic_lookup(cpu_set_t *ret_set, char *name_list, enum show_hide_mode mode)
    {
    pub i: c_uint,
    while (name_list) {
    pub comma: *mut c_char,
    pub ','): comma = strchr(name_list,,
    if (comma)
// comma = '\0';
    pub {: for (i = 0; i < MAX_BIC; ++i),
    if (!strcmp(name_list, bic[i].name)) {
    pub ret_set): SET_BIC(i,,
    }
    if (!strcmp(name_list, "all")) {
    } else if (!strcmp(name_list, "topology")) {
    pub &bic_group_topology): CPU_OR(ret_set, ret_set,,
    } else if (!strcmp(name_list, "power")) {
    pub &bic_group_thermal_pwr): CPU_OR(ret_set, ret_set,,
    } else if (!strcmp(name_list, "idle")) {
    pub &bic_group_idle): CPU_OR(ret_set, ret_set,,
    } else if (!strcmp(name_list, "cache")) {
    pub &bic_group_cache): CPU_OR(ret_set, ret_set,,
    } else if (!strcmp(name_list, "llc")) {
    pub &bic_group_cache): CPU_OR(ret_set, ret_set,,
    } else if (!strcmp(name_list, "swidle")) {
    pub &bic_group_sw_idle): CPU_OR(ret_set, ret_set,,
    } else if (!strcmp(name_list, "sysfs")) {	/* legacy compatibility */
    pub &bic_group_sw_idle): CPU_OR(ret_set, ret_set,,
    } else if (!strcmp(name_list, "hwidle")) {
    pub &bic_group_hw_idle): CPU_OR(ret_set, ret_set,,
    } else if (!strcmp(name_list, "frequency")) {
    pub &bic_group_frequency): CPU_OR(ret_set, ret_set,,
    } else if (!strcmp(name_list, "other")) {
    pub &bic_group_other): CPU_OR(ret_set, ret_set,,
    }
    }
    if (i == MAX_BIC) {
    if (mode == SHOW_LIST) {
    pub name_list: deferred_add_names[deferred_add_index++] =,
    if (deferred_add_index >= MAX_DEFERRED) {
    pub name_list): fprintf(stderr, "More than max %d un-recognized --add options '%s'\n", MAX_DEFERRED,,
    }
    } else {
    pub name_list: deferred_skip_names[deferred_skip_index++] =,
    if (debug)
    pub name_list): fprintf(stderr, "deferred \"%s\"\n",,
    if (deferred_skip_index >= MAX_DEFERRED) {
    pub name_list): fprintf(stderr, "More than max %d un-recognized --skip options '%s'\n", MAX_DEFERRED,,
    }
    }
    }
    pub comma: name_list =,
    if (name_list)
    }
    }
//
// print_name()
// Print column header name for raw 64-bit counter in 16 columns (at least 8-char plus a tab)
// Otherwise, allow the name + tab to fit within 8-coumn tab-stop.
// In both cases, left justififed, just like other turbostat columns,
// to allow the column values to consume the tab.
//
// Yes, 32-bit counters can overflow 8-columns, and
// 64-bit counters can overflow 16-columns, but that is uncommon.
//
#[no_mangle]
pub unsafe extern "C" fn print_name(width: c_int, printed: *mut c_int, delim: *mut c_char, name: *mut c_char, type: enum counter_type, format: enum counter_format) -> c_int {
    static inline int print_name(int width, int *printed, char *delim, char *name, enum counter_type type, enum counter_format format)
    {
    pub "": *mut *mut *mut char sep = (printed)++ ? delim :,
    if (format == FORMAT_RAW && width >= 64)
    pub name): return sprintf(outp, "%s%-8s", sep,,
    else
    pub name): return sprintf(outp, "%s%s", sep,,
    }
#[no_mangle]
pub unsafe extern "C" fn print_hex_value(width: c_int, printed: *mut c_int, delim: *mut c_char, value: c_ulonglong) -> c_int {
    static inline int print_hex_value(int width, int *printed, char *delim, unsigned long long value)
    {
    pub "": *mut *mut *mut char sep = (printed)++ ? delim :,
    if (width <= 32)
    pub value): return sprintf(outp, "%s%08llx", sep,,
    else
    pub value): return sprintf(outp, "%s%016llx", sep,,
    }
#[no_mangle]
pub unsafe extern "C" fn print_decimal_value(width: c_int, printed: *mut c_int, delim: *mut c_char, value: c_ulonglong) -> c_int {
    static inline int print_decimal_value(int width, int *printed, char *delim, unsigned long long value)
    {
    pub "": *mut *mut *mut char sep = (printed)++ ? delim :,
    pub value): return sprintf(outp, "%s%lld", sep,,
    }
#[no_mangle]
pub unsafe extern "C" fn print_float_value(printed: *mut c_int, delim: *mut c_char, value: double) -> c_int {
    static inline int print_float_value(int *printed, char *delim, double value)
    {
    pub "": *mut *mut *mut char sep = (printed)++ ? delim :,
    pub value): return sprintf(outp, "%s%0.2f", sep,,
    }
#[no_mangle]
pub unsafe extern "C" fn print_header(delim: *mut c_char) {
    void print_header(char *delim)
    {
    pub mp: *mut msr_counter,
    pub pp: *mut perf_counter_info,
    pub ppmt: *mut pmt_counter,
    pub 0: int printed =,
    if (DO_BIC(BIC_USEC))
    pub "")): outp += sprintf(outp, "%susec", (printed++ ? delim :,
    if (DO_BIC(BIC_TOD))
    pub "")): outp += sprintf(outp, "%sTime_Of_Day_Seconds", (printed++ ? delim :,
    if (DO_BIC(BIC_Package))
    pub "")): outp += sprintf(outp, "%sPackage", (printed++ ? delim :,
    if (DO_BIC(BIC_Die))
    pub "")): outp += sprintf(outp, "%sDie", (printed++ ? delim :,
    if (DO_BIC(BIC_L3))
    pub "")): outp += sprintf(outp, "%sL3", (printed++ ? delim :,
    if (DO_BIC(BIC_Node))
    pub "")): outp += sprintf(outp, "%sNode", (printed++ ? delim :,
    if (DO_BIC(BIC_Module))
    pub "")): outp += sprintf(outp, "%sModule", (printed++ ? delim :,
    if (DO_BIC(BIC_Core))
    pub "")): outp += sprintf(outp, "%sCore", (printed++ ? delim :,
    if (DO_BIC(BIC_CPU))
    pub "")): outp += sprintf(outp, "%sCPU", (printed++ ? delim :,
    if (DO_BIC(BIC_APIC))
    pub "")): outp += sprintf(outp, "%sAPIC", (printed++ ? delim :,
    if (DO_BIC(BIC_X2APIC))
    pub "")): outp += sprintf(outp, "%sX2APIC", (printed++ ? delim :,
    if (DO_BIC(BIC_Avg_MHz))
    pub "")): outp += sprintf(outp, "%sAvg_MHz", (printed++ ? delim :,
    if (DO_BIC(BIC_Busy))
    pub "")): outp += sprintf(outp, "%sBusy%%", (printed++ ? delim :,
    if (DO_BIC(BIC_Bzy_MHz))
    pub "")): outp += sprintf(outp, "%sBzy_MHz", (printed++ ? delim :,
    if (DO_BIC(BIC_TSC_MHz))
    pub "")): outp += sprintf(outp, "%sTSC_MHz", (printed++ ? delim :,
    if (DO_BIC(BIC_IPC))
    pub "")): outp += sprintf(outp, "%sIPC", (printed++ ? delim :,
    if (DO_BIC(BIC_IRQ)) {
    if (sums_need_wide_columns)
    pub "")): outp += sprintf(outp, "%s IRQ", (printed++ ? delim :,
    else
    pub "")): outp += sprintf(outp, "%sIRQ", (printed++ ? delim :,
    }
    if (DO_BIC(BIC_NMI)) {
    if (sums_need_wide_columns)
    pub "")): outp += sprintf(outp, "%s NMI", (printed++ ? delim :,
    else
    pub "")): outp += sprintf(outp, "%sNMI", (printed++ ? delim :,
    }
    if (DO_BIC(BIC_SMI))
    pub "")): outp += sprintf(outp, "%sSMI", (printed++ ? delim :,
    if (DO_BIC(BIC_LLC_MRPS))
    pub "")): outp += sprintf(outp, "%sLLCMRPS", (printed++ ? delim :,
    if (DO_BIC(BIC_LLC_HIT))
    pub "")): outp += sprintf(outp, "%sLLC%%hit", (printed++ ? delim :,
    if (DO_BIC(BIC_L2_MRPS))
    pub "")): outp += sprintf(outp, "%sL2MRPS", (printed++ ? delim :,
    if (DO_BIC(BIC_L2_HIT))
    pub "")): outp += sprintf(outp, "%sL2%%hit", (printed++ ? delim :,
    pub mp->next): for (mp = sys.tp; mp; mp =,
    pub mp->format): outp += print_name(mp->width, &printed, delim, mp->name, mp->type,,
    pub pp->next): for (pp = sys.perf_tp; pp; pp =,
    pub pp->format): outp += print_name(pp->width, &printed, delim, pp->name, pp->type,,
    pub sys.pmt_tp: ppmt =,
    while (ppmt) {
    switch (ppmt.type) {
    case PMT_TYPE_RAW:
    pub ppmt->format): outp += print_name(pmt_counter_get_width(ppmt), &printed, delim, ppmt->name, COUNTER_ITEMS,,
    case PMT_TYPE_XTAL_TIME:
    case PMT_TYPE_TCORE_CLOCK:
    pub ppmt->format): outp += print_name(32, &printed, delim, ppmt->name, COUNTER_ITEMS,,
    }
    pub ppmt->next: ppmt =,
    }
    if (DO_BIC(BIC_CPU_c1))
    pub "")): outp += sprintf(outp, "%sCPU%%c1", (printed++ ? delim :,
    if (DO_BIC(BIC_CPU_c3))
    pub "")): outp += sprintf(outp, "%sCPU%%c3", (printed++ ? delim :,
    if (DO_BIC(BIC_CPU_c6))
    pub "")): outp += sprintf(outp, "%sCPU%%c6", (printed++ ? delim :,
    if (DO_BIC(BIC_CPU_c7))
    pub "")): outp += sprintf(outp, "%sCPU%%c7", (printed++ ? delim :,
    if (DO_BIC(BIC_Mod_c6))
    pub "")): outp += sprintf(outp, "%sMod%%c6", (printed++ ? delim :,
    if (DO_BIC(BIC_CoreTmp))
    pub "")): outp += sprintf(outp, "%sCoreTmp", (printed++ ? delim :,
    if (DO_BIC(BIC_CORE_THROT_CNT))
    pub "")): outp += sprintf(outp, "%sCoreThr", (printed++ ? delim :,
    if (valid_rapl_msrs && !rapl_joules) {
    if (DO_BIC(BIC_CorWatt) && platform.has_per_core_rapl)
    pub "")): outp += sprintf(outp, "%sCorWatt", (printed++ ? delim :,
    } else if (valid_rapl_msrs && rapl_joules) {
    if (DO_BIC(BIC_Cor_J) && platform.has_per_core_rapl)
    pub "")): outp += sprintf(outp, "%sCor_J", (printed++ ? delim :,
    }
    pub mp->next): for (mp = sys.cp; mp; mp =,
    pub mp->format): outp += print_name(mp->width, &printed, delim, mp->name, mp->type,,
    pub pp->next): for (pp = sys.perf_cp; pp; pp =,
    pub pp->format): outp += print_name(pp->width, &printed, delim, pp->name, pp->type,,
    pub sys.pmt_cp: ppmt =,
    while (ppmt) {
    switch (ppmt.type) {
    case PMT_TYPE_RAW:
    pub ppmt->format): outp += print_name(pmt_counter_get_width(ppmt), &printed, delim, ppmt->name, COUNTER_ITEMS,,
    case PMT_TYPE_XTAL_TIME:
    case PMT_TYPE_TCORE_CLOCK:
    pub ppmt->format): outp += print_name(32, &printed, delim, ppmt->name, COUNTER_ITEMS,,
    }
    pub ppmt->next: ppmt =,
    }
    if (DO_BIC(BIC_PkgTmp))
    pub "")): outp += sprintf(outp, "%sPkgTmp", (printed++ ? delim :,
    if (DO_BIC(BIC_GFX_rc6))
    pub "")): outp += sprintf(outp, "%sGFX%%rc6", (printed++ ? delim :,
    if (DO_BIC(BIC_GFXMHz))
    pub "")): outp += sprintf(outp, "%sGFXMHz", (printed++ ? delim :,
    if (DO_BIC(BIC_GFXACTMHz))
    pub "")): outp += sprintf(outp, "%sGFXAMHz", (printed++ ? delim :,
    if (DO_BIC(BIC_SAM_mc6))
    pub "")): outp += sprintf(outp, "%sSAM%%mc6", (printed++ ? delim :,
    if (DO_BIC(BIC_SAMMHz))
    pub "")): outp += sprintf(outp, "%sSAMMHz", (printed++ ? delim :,
    if (DO_BIC(BIC_SAMACTMHz))
    pub "")): outp += sprintf(outp, "%sSAMAMHz", (printed++ ? delim :,
    if (DO_BIC(BIC_Totl_c0))
    pub "")): outp += sprintf(outp, "%sTotl%%C0", (printed++ ? delim :,
    if (DO_BIC(BIC_Any_c0))
    pub "")): outp += sprintf(outp, "%sAny%%C0", (printed++ ? delim :,
    if (DO_BIC(BIC_GFX_c0))
    pub "")): outp += sprintf(outp, "%sGFX%%C0", (printed++ ? delim :,
    if (DO_BIC(BIC_CPUGFX))
    pub "")): outp += sprintf(outp, "%sCPUGFX%%", (printed++ ? delim :,
    if (DO_BIC(BIC_Pkgpc2))
    pub "")): outp += sprintf(outp, "%sPkg%%pc2", (printed++ ? delim :,
    if (DO_BIC(BIC_Pkgpc3))
    pub "")): outp += sprintf(outp, "%sPkg%%pc3", (printed++ ? delim :,
    if (DO_BIC(BIC_Pkgpc6))
    pub "")): outp += sprintf(outp, "%sPkg%%pc6", (printed++ ? delim :,
    if (DO_BIC(BIC_Pkgpc7))
    pub "")): outp += sprintf(outp, "%sPkg%%pc7", (printed++ ? delim :,
    if (DO_BIC(BIC_Pkgpc8))
    pub "")): outp += sprintf(outp, "%sPkg%%pc8", (printed++ ? delim :,
    if (DO_BIC(BIC_Pkgpc9))
    pub "")): outp += sprintf(outp, "%sPkg%%pc9", (printed++ ? delim :,
    if (DO_BIC(BIC_Pkgpc10))
    pub "")): outp += sprintf(outp, "%sPk%%pc10", (printed++ ? delim :,
    if (DO_BIC(BIC_Diec6))
    pub "")): outp += sprintf(outp, "%sDie%%c6", (printed++ ? delim :,
    if (DO_BIC(BIC_CPU_LPI))
    pub "")): outp += sprintf(outp, "%sCPU%%LPI", (printed++ ? delim :,
    if (DO_BIC(BIC_SYS_LPI))
    pub "")): outp += sprintf(outp, "%sSYS%%LPI", (printed++ ? delim :,
    if (!rapl_joules) {
    if (DO_BIC(BIC_PkgWatt))
    pub "")): outp += sprintf(outp, "%sPkgWatt", (printed++ ? delim :,
    if (DO_BIC(BIC_CorWatt) && !platform.has_per_core_rapl)
    pub "")): outp += sprintf(outp, "%sCorWatt", (printed++ ? delim :,
    if (DO_BIC(BIC_GFXWatt))
    pub "")): outp += sprintf(outp, "%sGFXWatt", (printed++ ? delim :,
    if (DO_BIC(BIC_RAMWatt))
    pub "")): outp += sprintf(outp, "%sRAMWatt", (printed++ ? delim :,
    if (DO_BIC(BIC_PKG__))
    pub "")): outp += sprintf(outp, "%sPKG_%%", (printed++ ? delim :,
    if (DO_BIC(BIC_RAM__))
    pub "")): outp += sprintf(outp, "%sRAM_%%", (printed++ ? delim :,
    } else {
    if (DO_BIC(BIC_Pkg_J))
    pub "")): outp += sprintf(outp, "%sPkg_J", (printed++ ? delim :,
    if (DO_BIC(BIC_Cor_J) && !platform.has_per_core_rapl)
    pub "")): outp += sprintf(outp, "%sCor_J", (printed++ ? delim :,
    if (DO_BIC(BIC_GFX_J))
    pub "")): outp += sprintf(outp, "%sGFX_J", (printed++ ? delim :,
    if (DO_BIC(BIC_RAM_J))
    pub "")): outp += sprintf(outp, "%sRAM_J", (printed++ ? delim :,
    if (DO_BIC(BIC_PKG__))
    pub "")): outp += sprintf(outp, "%sPKG_%%", (printed++ ? delim :,
    if (DO_BIC(BIC_RAM__))
    pub "")): outp += sprintf(outp, "%sRAM_%%", (printed++ ? delim :,
    }
    if (DO_BIC(BIC_UNCORE_MHZ))
    pub "")): outp += sprintf(outp, "%sUncMHz", (printed++ ? delim :,
    pub mp->next): for (mp = sys.pp; mp; mp =,
    pub mp->format): outp += print_name(mp->width, &printed, delim, mp->name, mp->type,,
    pub pp->next): for (pp = sys.perf_pp; pp; pp =,
    pub pp->format): outp += print_name(pp->width, &printed, delim, pp->name, pp->type,,
    pub sys.pmt_pp: ppmt =,
    while (ppmt) {
    switch (ppmt.type) {
    case PMT_TYPE_RAW:
    pub ppmt->format): outp += print_name(pmt_counter_get_width(ppmt), &printed, delim, ppmt->name, COUNTER_ITEMS,,
    case PMT_TYPE_XTAL_TIME:
    case PMT_TYPE_TCORE_CLOCK:
    pub ppmt->format): outp += print_name(32, &printed, delim, ppmt->name, COUNTER_ITEMS,,
    }
    pub ppmt->next: ppmt =,
    }
    if (DO_BIC(BIC_SysWatt))
    pub "")): outp += sprintf(outp, "%sSysWatt", (printed++ ? delim :,
    if (DO_BIC(BIC_Sys_J))
    pub "")): outp += sprintf(outp, "%sSys_J", (printed++ ? delim :,
    pub "\n"): outp += sprintf(outp,,
    }
//
// pct(numerator, denominator)
//
// Return sanity checked percentage (100.0 * numerator/denominotor)
//
// n < 0: nan
// d <= 0: nan
// n/d > 1.1: nan
//
#[no_mangle]
pub unsafe extern "C" fn pct(numerator: double, denominator: double) -> double {
    double pct(double numerator, double denominator)
    {
    pub retval: double,
    if (numerator < 0)
    pub nan(""): return,
    if (denominator <= 0)
    pub nan(""): return,
    pub denominator: *mut *mut retval = 100.0  numerator /,
    if (retval > 110.0)
    pub nan(""): return,
    pub retval: return,
    }
#[no_mangle]
pub unsafe extern "C" fn dump_counters(_arg: PER_THREAD_PARAMS) -> c_int {
    int dump_counters(PER_THREAD_PARAMS)
    {
    pub i: c_int,
    pub mp: *mut msr_counter,
    pub &platform_counters_even: *mut *mut platform_counters pplat_cnt = p == odd.packages ? &platform_counters_odd :,
    pub p): outp += sprintf(outp, "t %p, c %p, p %p\n", t, c,,
    if (t) {
    pub t->flags): outp += sprintf(outp, "CPU: %d flags 0x%x\n", t->cpu_id,,
    pub t->tsc): outp += sprintf(outp, "TSC: %016llX\n",,
    pub t->aperf): outp += sprintf(outp, "aperf: %016llX\n",,
    pub t->mperf): outp += sprintf(outp, "mperf: %016llX\n",,
    pub t->c1): outp += sprintf(outp, "c1: %016llX\n",,
    if (DO_BIC(BIC_IPC))
    pub t->instr_count): outp += sprintf(outp, "IPC: %lld\n",,
    if (DO_BIC(BIC_IRQ))
    pub t->irq_count): outp += sprintf(outp, "IRQ: %lld\n",,
    if (DO_BIC(BIC_NMI))
    pub t->nmi_count): outp += sprintf(outp, "IRQ: %lld\n",,
    if (DO_BIC(BIC_SMI))
    pub t->smi_count): outp += sprintf(outp, "SMI: %d\n",,
    pub t->llc.references): outp += sprintf(outp, "LLC refs: %lld",,
    pub t->llc.misses): outp += sprintf(outp, "LLC miss: %lld",,
    pub t->llc.references)): outp += sprintf(outp, "LLC Hit%%: %.2f", pct((t->llc.references - t->llc.misses),,
    pub t->l2.references): outp += sprintf(outp, "L2 refs: %lld",,
    pub t->l2.hits): outp += sprintf(outp, "L2 hits: %lld",,
    pub t->l2.references)): outp += sprintf(outp, "L2 Hit%%: %.2f", pct(t->l2.hits,,
    pub {: for (i = 0, mp = sys.tp; mp; i++, mp = mp->next),
    pub mp->sp->path): outp += sprintf(outp, "tADDED [%d] %8s msr0x%x: %08llX %s\n", i, mp->name, mp->msr_num, t->counter[i],,
    }
    }
    if (c && is_cpu_first_thread_in_core(t, c)) {
    pub cpus[t->cpu_id].core_id): outp += sprintf(outp, "core: 0x%x\n",,
    pub c->c3): outp += sprintf(outp, "c3: %016llX\n",,
    pub c->c6): outp += sprintf(outp, "c6: %016llX\n",,
    pub c->c7): outp += sprintf(outp, "c7: %016llX\n",,
    pub c->core_temp_c): outp += sprintf(outp, "DTS: %dC\n",,
    pub c->core_throt_cnt): outp += sprintf(outp, "cpu_throt_count: %016llX\n",,
    pub c->core_energy.scale: *const *const unsigned long long energy_value = c->core_energy.raw_value,
    pub c->core_energy.scale: double energy_scale =,
    if (c.core_energy.unit == RAPL_UNIT_JOULES)
    pub energy_scale): outp += sprintf(outp, "Joules: %0llX (scale: %lf)\n", energy_value,,
    pub {: for (i = 0, mp = sys.cp; mp; i++, mp = mp->next),
    pub mp->sp->path): outp += sprintf(outp, "cADDED [%d] %8s msr0x%x: %08llX %s\n", i, mp->name, mp->msr_num, c->counter[i],,
    }
    pub c->mc6_us): outp += sprintf(outp, "mc6_us: %016llX\n",,
    }
    if (p && is_cpu_first_core_in_package(t, p)) {
    pub p->pkg_wtd_core_c0): outp += sprintf(outp, "Weighted cores: %016llX\n",,
    pub p->pkg_any_core_c0): outp += sprintf(outp, "Any cores: %016llX\n",,
    pub p->pkg_any_gfxe_c0): outp += sprintf(outp, "Any GFX: %016llX\n",,
    pub p->pkg_both_core_gfxe_c0): outp += sprintf(outp, "CPU + GFX: %016llX\n",,
    pub p->pc2): outp += sprintf(outp, "pc2: %016llX\n",,
    if (DO_BIC(BIC_Pkgpc3))
    pub p->pc3): outp += sprintf(outp, "pc3: %016llX\n",,
    if (DO_BIC(BIC_Pkgpc6))
    pub p->pc6): outp += sprintf(outp, "pc6: %016llX\n",,
    if (DO_BIC(BIC_Pkgpc7))
    pub p->pc7): outp += sprintf(outp, "pc7: %016llX\n",,
    pub p->pc8): outp += sprintf(outp, "pc8: %016llX\n",,
    pub p->pc9): outp += sprintf(outp, "pc9: %016llX\n",,
    pub p->pc10): outp += sprintf(outp, "pc10: %016llX\n",,
    pub p->cpu_lpi): outp += sprintf(outp, "cpu_lpi: %016llX\n",,
    pub p->sys_lpi): outp += sprintf(outp, "sys_lpi: %016llX\n",,
    pub p->energy_pkg.raw_value): outp += sprintf(outp, "Joules PKG: %0llX\n",,
    pub p->energy_cores.raw_value): outp += sprintf(outp, "Joules COR: %0llX\n",,
    pub p->energy_gfx.raw_value): outp += sprintf(outp, "Joules GFX: %0llX\n",,
    pub p->energy_dram.raw_value): outp += sprintf(outp, "Joules RAM: %0llX\n",,
    pub pplat_cnt->energy_psys.raw_value): outp += sprintf(outp, "Joules PSYS: %0llX\n",,
    pub p->rapl_pkg_perf_status.raw_value): outp += sprintf(outp, "Throttle PKG: %0llX\n",,
    pub p->rapl_dram_perf_status.raw_value): outp += sprintf(outp, "Throttle RAM: %0llX\n",,
    pub p->pkg_temp_c): outp += sprintf(outp, "PTM: %dC\n",,
    pub {: for (i = 0, mp = sys.pp; mp; i++, mp = mp->next),
    pub mp->sp->path): outp += sprintf(outp, "pADDED [%d] %8s msr0x%x: %08llX %s\n", i, mp->name, mp->msr_num, p->counter[i],,
    }
    }
    pub "\n"): outp += sprintf(outp,,
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn rapl_counter_get_value(c: *const rapl_counter, desired_unit: enum rapl_unit, interval: double) -> double {
    double rapl_counter_get_value(const struct rapl_counter *c, enum rapl_unit desired_unit, double interval)
    {
    pub RAPL_UNIT_INVALID): assert(desired_unit !=,
//
// For now we don't expect anything other than joules,
// so just simplify the logic.
//
    pub RAPL_UNIT_JOULES): assert(c->unit ==,
    pub c->scale: *const *const double scaled = c->raw_value,
    if (desired_unit == RAPL_UNIT_WATTS)
    pub interval: return scaled /,
    pub scaled: return,
    }
#[no_mangle]
pub unsafe extern "C" fn get_perf_llc_stats(cpu: c_int, llc: *mut llc_stats) {
    void get_perf_llc_stats(int cpu, struct llc_stats *llc)
    {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct read_format {
    pub num_read: c_ulonglong,
    pub llc: llc_stats,
    pub r: },
    pub sizeof(r): ssize_t expected_read_size =,
    pub actual_read_size: isize,
    pub expected_read_size): actual_read_size = read(fd_llc_percpu[cpu], &r,,
    if (actual_read_size == -1)
    pub expected_read_size): err(-1, "%s(cpu%d,) %d,,%ld", __func__, cpu, fd_llc_percpu[cpu],,
    pub r.llc.references: llc->references =,
    pub r.llc.misses: llc->misses =,
    if (actual_read_size != expected_read_size)
    pub actual_read_size): warn("%s: failed to read perf_data (req %zu act %zu)", __func__, expected_read_size,,
    }
#[no_mangle]
pub unsafe extern "C" fn get_perf_l2_stats(cpu: c_int, l2: *mut l2_stats) {
    void get_perf_l2_stats(int cpu, struct l2_stats *l2)
    {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct read_format {
    pub num_read: c_ulonglong,
    pub l2: l2_stats,
    pub r: },
    pub sizeof(r): ssize_t expected_read_size =,
    pub actual_read_size: isize,
    pub expected_read_size): actual_read_size = read(fd_l2_percpu[cpu], &r,,
    if (actual_read_size == -1)
    pub expected_read_size): err(-1, "%s(cpu%d,) %d,,%ld", __func__, cpu, fd_l2_percpu[cpu],,
    pub r.l2.references: l2->references =,
    pub r.l2.hits: l2->hits =,
    if (actual_read_size != expected_read_size)
    pub actual_read_size): warn("%s: cpu%d: failed to read(%d) perf_data (req %zu act %zu)", __func__, cpu, fd_l2_percpu[cpu], expected_read_size,,
    }
//
// column formatting convention & formats
//
#[no_mangle]
pub unsafe extern "C" fn format_counters(_arg: PER_THREAD_PARAMS) -> c_int {
    int format_counters(PER_THREAD_PARAMS)
    {
    pub count: static int,
    pub NULL: *mut *mut platform_counters pplat_cnt =,
    pub tsc: double interval_float,,
    pub "%s%.2f": *mut *mut char fmt8 =,
    pub i: c_int,
    pub mp: *mut msr_counter,
    pub pp: *mut perf_counter_info,
    pub ppmt: *mut pmt_counter,
    pub "\t": *mut *mut char delim =,
    pub 0: int printed =,
    if (t == average.threads) {
    pub &platform_counters_even: pplat_cnt = count & 1 ? &platform_counters_odd :,
    }
// if showing only 1st thread in core and this isn't one, bail out
    if (show_core_only && !is_cpu_first_thread_in_core(t, c))
    pub 0: return,
// if showing only 1st thread in pkg and this isn't one, bail out
    if (show_pkg_only && !is_cpu_first_core_in_package(t, p))
    pub 0: return,
// if not summary line and --cpu is used
    if ((t != average.threads) && (cpu_subset && !CPU_ISSET_S(t.cpu_id, cpu_subset_size, cpu_subset)))
    pub 0: return,
    if (DO_BIC(BIC_USEC)) {
// on each row, print how many usec each timestamp took to gather
    pub tv: timeval,
    pub &tv): timersub(&t->tv_end, &t->tv_begin,,
    pub tv.tv_usec): *mut *mut outp += sprintf(outp, "%5ld\t", tv.tv_sec  1000000 +,
    }
// Time_Of_Day_Seconds: on each row, print sec.usec last timestamp taken
    if (DO_BIC(BIC_TOD))
    pub t->tv_end.tv_usec): outp += sprintf(outp, "%10ld.%06ld\t", t->tv_end.tv_sec,,
    pub 1000000.0: interval_float = t->tv_delta.tv_sec + t->tv_delta.tv_usec /,
    pub tsc_tweak: *mut *mut tsc = t->tsc,
// topo columns, print blanks on 1st (average) line
    if (t == average.threads) {
    if (DO_BIC(BIC_Package))
    pub "")): outp += sprintf(outp, "%s-", (printed++ ? delim :,
    if (DO_BIC(BIC_Die))
    pub "")): outp += sprintf(outp, "%s-", (printed++ ? delim :,
    if (DO_BIC(BIC_L3))
    pub "")): outp += sprintf(outp, "%s-", (printed++ ? delim :,
    if (DO_BIC(BIC_Node))
    pub "")): outp += sprintf(outp, "%s-", (printed++ ? delim :,
    if (DO_BIC(BIC_Module))
    pub "")): outp += sprintf(outp, "%s-", (printed++ ? delim :,
    if (DO_BIC(BIC_Core))
    pub "")): outp += sprintf(outp, "%s-", (printed++ ? delim :,
    if (DO_BIC(BIC_CPU))
    pub "")): outp += sprintf(outp, "%s-", (printed++ ? delim :,
    if (DO_BIC(BIC_APIC))
    pub "")): outp += sprintf(outp, "%s-", (printed++ ? delim :,
    if (DO_BIC(BIC_X2APIC))
    pub "")): outp += sprintf(outp, "%s-", (printed++ ? delim :,
    } else {
    if (DO_BIC(BIC_Package)) {
    if (p)
    pub cpus[t->cpu_id].package_id): outp += sprintf(outp, "%s%d", (printed++ ? delim : ""),,
    else
    pub "")): outp += sprintf(outp, "%s-", (printed++ ? delim :,
    }
    if (DO_BIC(BIC_Die)) {
    if (c)
    pub cpus[t->cpu_id].die_id): outp += sprintf(outp, "%s%d", (printed++ ? delim : ""),,
    else
    pub "")): outp += sprintf(outp, "%s-", (printed++ ? delim :,
    }
    if (DO_BIC(BIC_L3)) {
    if (c)
    pub cpus[t->cpu_id].l3_id): outp += sprintf(outp, "%s%d", (printed++ ? delim : ""),,
    else
    pub "")): outp += sprintf(outp, "%s-", (printed++ ? delim :,
    }
    if (DO_BIC(BIC_Node)) {
    if (t)
    pub cpus[t->cpu_id].physical_node_id): outp += sprintf(outp, "%s%d", (printed++ ? delim : ""),,
    else
    pub "")): outp += sprintf(outp, "%s-", (printed++ ? delim :,
    }
    if (DO_BIC(BIC_Module)) {
    if (c)
    pub cpus[t->cpu_id].module_id): outp += sprintf(outp, "%s0x%x", (printed++ ? delim : ""),,
    else
    pub "")): outp += sprintf(outp, "%s-", (printed++ ? delim :,
    }
    if (DO_BIC(BIC_Core)) {
    if (c)
    pub cpus[t->cpu_id].core_id): outp += sprintf(outp, "%s0x%x", (printed++ ? delim : ""),,
    else
    pub "")): outp += sprintf(outp, "%s-", (printed++ ? delim :,
    }
    if (DO_BIC(BIC_CPU))
    pub t->cpu_id): outp += sprintf(outp, "%s%d", (printed++ ? delim : ""),,
    if (DO_BIC(BIC_APIC))
    pub t->apic_id): outp += sprintf(outp, "%s0x%x", (printed++ ? delim : ""),,
    if (DO_BIC(BIC_X2APIC))
    pub t->x2apic_id): outp += sprintf(outp, "%s0x%x", (printed++ ? delim : ""),,
    }
    if (DO_BIC(BIC_Avg_MHz))
    pub interval_float): *mut *mut outp += sprintf(outp, "%s%.0f", (printed++ ? delim : ""), 1.0 / units  t->aperf /,
    if (DO_BIC(BIC_Busy))
    pub tsc)): outp += sprintf(outp, "%s%.2f", (printed++ ? delim : ""), pct(t->mperf,,
    if (DO_BIC(BIC_Bzy_MHz)) {
    if (has_base_hz)
    pub t->mperf): *mut *mut outp += sprintf(outp, "%s%.0f", (printed++ ? delim : ""), base_hz / units  t->aperf /,
    else
    pub interval_float): *mut *mut outp += sprintf(outp, "%s%.0f", (printed++ ? delim : ""), tsc / units  t->aperf / t->mperf /,
    }
    if (DO_BIC(BIC_TSC_MHz))
    pub interval_float): *mut *mut outp += sprintf(outp, "%s%.0f", (printed++ ? delim : ""), 1.0  t->tsc / units /,
    if (DO_BIC(BIC_IPC))
    pub t->aperf): *mut *mut outp += sprintf(outp, "%s%.2f", (printed++ ? delim : ""), 1.0  t->instr_count /,
// IRQ
    if (DO_BIC(BIC_IRQ)) {
    if (sums_need_wide_columns)
    pub t->irq_count): outp += sprintf(outp, "%s%8lld", (printed++ ? delim : ""),,
    else
    pub t->irq_count): outp += sprintf(outp, "%s%lld", (printed++ ? delim : ""),,
    }
// NMI
    if (DO_BIC(BIC_NMI)) {
    if (sums_need_wide_columns)
    pub t->nmi_count): outp += sprintf(outp, "%s%8lld", (printed++ ? delim : ""),,
    else
    pub t->nmi_count): outp += sprintf(outp, "%s%lld", (printed++ ? delim : ""),,
    }
// SMI
    if (DO_BIC(BIC_SMI))
    pub t->smi_count): outp += sprintf(outp, "%s%d", (printed++ ? delim : ""),,
// LLC Stats
    if (DO_BIC(BIC_LLC_MRPS))
    pub 1000000): outp += sprintf(outp, "%s%.0f", (printed++ ? delim : ""), t->llc.references / interval_float /,
    if (DO_BIC(BIC_LLC_HIT))
    pub t->llc.references)): outp += sprintf(outp, fmt8, (printed++ ? delim : ""), pct((t->llc.references - t->llc.misses),,
// L2 Stats
    if (DO_BIC(BIC_L2_MRPS))
    pub 1000000): outp += sprintf(outp, "%s%.0f", (printed++ ? delim : ""), t->l2.references / interval_float /,
    if (DO_BIC(BIC_L2_HIT))
    pub t->l2.references)): outp += sprintf(outp, fmt8, (printed++ ? delim : ""), pct(t->l2.hits,,
// Added Thread Counters
    pub {: for (i = 0, mp = sys.tp; mp; i++, mp = mp->next),
    if (mp.format == FORMAT_RAW)
    pub t->counter[i]): outp += print_hex_value(mp->width, &printed, delim,,
#[no_mangle]
pub unsafe extern "C" fn if(FORMAT_AVERAGE: mp->format == FORMAT_DELTA || mp->format ==) -> else {
    else if (mp.format == FORMAT_DELTA || mp.format == FORMAT_AVERAGE)
    pub t->counter[i]): outp += print_decimal_value(mp->width, &printed, delim,,
#[no_mangle]
pub unsafe extern "C" fn if(FORMAT_PERCENT: mp->format ==) -> else {
    if (mp.type == COUNTER_USEC)
    pub 10000): outp += print_float_value(&printed, delim, t->counter[i] / interval_float /,
    else
    pub tsc)): outp += print_float_value(&printed, delim, pct(t->counter[i],,
    }
    }
// Added perf Thread Counters
    pub {: for (i = 0, pp = sys.perf_tp; pp; ++i, pp = pp->next),
    if (pp.format == FORMAT_RAW)
    pub t->perf_counter[i]): outp += print_hex_value(pp->width, &printed, delim,,
#[no_mangle]
pub unsafe extern "C" fn if(FORMAT_AVERAGE: pp->format == FORMAT_DELTA || pp->format ==) -> else {
    else if (pp.format == FORMAT_DELTA || pp.format == FORMAT_AVERAGE)
    pub t->perf_counter[i]): outp += print_decimal_value(pp->width, &printed, delim,,
#[no_mangle]
pub unsafe extern "C" fn if(FORMAT_PERCENT: pp->format ==) -> else {
    if (pp.type == COUNTER_USEC)
    pub 10000): outp += print_float_value(&printed, delim, t->perf_counter[i] / interval_float /,
    else
    pub tsc)): outp += print_float_value(&printed, delim, pct(t->perf_counter[i],,
    }
    }
// Added PMT Thread Counters
    pub {: for (i = 0, ppmt = sys.pmt_tp; ppmt; i++, ppmt = ppmt->next),
    pub t->pmt_counter[i]: unsigned long value_raw =,
    pub value_converted: double,
    switch (ppmt.type) {
    case PMT_TYPE_RAW:
    pub t->pmt_counter[i]): outp += print_hex_value(pmt_counter_get_width(ppmt), &printed, delim,,
    case PMT_TYPE_XTAL_TIME:
    pub interval_float): value_converted = pct(value_raw / crystal_hz,,
    pub value_converted): outp += print_float_value(&printed, delim,,
    case PMT_TYPE_TCORE_CLOCK:
    pub interval_float): value_converted = pct(value_raw / tcore_clock_freq_hz,,
    pub value_converted): outp += print_float_value(&printed, delim,,
    }
    }
// C1
    if (DO_BIC(BIC_CPU_c1))
    pub tsc)): outp += sprintf(outp, "%s%.2f", (printed++ ? delim : ""), pct(t->c1,,
// print per-core data only for 1st thread in core
    if (!is_cpu_first_thread_in_core(t, c))
    pub done: goto,
    if (DO_BIC(BIC_CPU_c3))
    pub tsc)): outp += sprintf(outp, "%s%.2f", (printed++ ? delim : ""), pct(c->c3,,
    if (DO_BIC(BIC_CPU_c6))
    pub tsc)): outp += sprintf(outp, "%s%.2f", (printed++ ? delim : ""), pct(c->c6,,
    if (DO_BIC(BIC_CPU_c7))
    pub tsc)): outp += sprintf(outp, "%s%.2f", (printed++ ? delim : ""), pct(c->c7,,
// Mod%c6
    if (DO_BIC(BIC_Mod_c6))
    pub tsc)): outp += sprintf(outp, "%s%.2f", (printed++ ? delim : ""), pct(c->mc6_us,,
    if (DO_BIC(BIC_CoreTmp))
    pub c->core_temp_c): outp += sprintf(outp, "%s%d", (printed++ ? delim : ""),,
// Core throttle count
    if (DO_BIC(BIC_CORE_THROT_CNT))
    pub c->core_throt_cnt): outp += sprintf(outp, "%s%lld", (printed++ ? delim : ""),,
// Added Core Counters
    pub {: for (i = 0, mp = sys.cp; mp; i++, mp = mp->next),
    if (mp.format == FORMAT_RAW)
    pub c->counter[i]): outp += print_hex_value(mp->width, &printed, delim,,
#[no_mangle]
pub unsafe extern "C" fn if(FORMAT_AVERAGE: mp->format == FORMAT_DELTA || mp->format ==) -> else {
    else if (mp.format == FORMAT_DELTA || mp.format == FORMAT_AVERAGE)
    pub c->counter[i]): outp += print_decimal_value(mp->width, &printed, delim,,
#[no_mangle]
pub unsafe extern "C" fn if(FORMAT_PERCENT: mp->format ==) -> else {
    else if (mp.format == FORMAT_PERCENT)
    pub tsc)): outp += print_float_value(&printed, delim, pct(c->counter[i],,
    }
// Added perf Core counters
    pub {: for (i = 0, pp = sys.perf_cp; pp; i++, pp = pp->next),
    if (pp.format == FORMAT_RAW)
    pub c->perf_counter[i]): outp += print_hex_value(pp->width, &printed, delim,,
#[no_mangle]
pub unsafe extern "C" fn if(FORMAT_AVERAGE: pp->format == FORMAT_DELTA || pp->format ==) -> else {
    else if (pp.format == FORMAT_DELTA || pp.format == FORMAT_AVERAGE)
    pub c->perf_counter[i]): outp += print_decimal_value(pp->width, &printed, delim,,
#[no_mangle]
pub unsafe extern "C" fn if(FORMAT_PERCENT: pp->format ==) -> else {
    else if (pp.format == FORMAT_PERCENT)
    pub tsc)): outp += print_float_value(&printed, delim, pct(c->perf_counter[i],,
    }
// Added PMT Core counters
    pub {: for (i = 0, ppmt = sys.pmt_cp; ppmt; i++, ppmt = ppmt->next),
    pub c->pmt_counter[i]: unsigned long value_raw =,
    pub value_converted: double,
    switch (ppmt.type) {
    case PMT_TYPE_RAW:
    pub c->pmt_counter[i]): outp += print_hex_value(pmt_counter_get_width(ppmt), &printed, delim,,
    case PMT_TYPE_XTAL_TIME:
    pub interval_float): value_converted = pct(value_raw / crystal_hz,,
    pub value_converted): outp += print_float_value(&printed, delim,,
    case PMT_TYPE_TCORE_CLOCK:
    pub interval_float): value_converted = pct(value_raw / tcore_clock_freq_hz,,
    pub value_converted): outp += print_float_value(&printed, delim,,
    }
    }
    if (DO_BIC(BIC_CorWatt) && platform.has_per_core_rapl)
    pub interval_float)): outp += sprintf(outp, fmt8, (printed++ ? delim : ""), rapl_counter_get_value(&c->core_energy, RAPL_UNIT_WATTS,,
    if (DO_BIC(BIC_Cor_J) && platform.has_per_core_rapl)
    pub interval_float)): outp += sprintf(outp, fmt8, (printed++ ? delim : ""), rapl_counter_get_value(&c->core_energy, RAPL_UNIT_JOULES,,
// print per-package data only for 1st core in package
    if (!is_cpu_first_core_in_package(t, p))
    pub done: goto,
// PkgTmp
    if (DO_BIC(BIC_PkgTmp))
    pub p->pkg_temp_c): outp += sprintf(outp, "%s%d", (printed++ ? delim : ""),,
// GFXrc6
    if (DO_BIC(BIC_GFX_rc6)) {
    if (p.gfx_rc6_ms == -1) {	/* detect GFX counter reset */
    pub "")): *mut *mut *mut *mut *mut outp += sprintf(outp, "%s.", (printed++ ? delim :,
    } else {
    pub interval_float): outp += sprintf(outp, "%s%.2f", (printed++ ? delim : ""), p->gfx_rc6_ms / 10.0 /,
    }
    }
// GFXMHz
    if (DO_BIC(BIC_GFXMHz))
    pub p->gfx_mhz): outp += sprintf(outp, "%s%d", (printed++ ? delim : ""),,
// GFXACTMHz
    if (DO_BIC(BIC_GFXACTMHz))
    pub p->gfx_act_mhz): outp += sprintf(outp, "%s%d", (printed++ ? delim : ""),,
// SAMmc6
    if (DO_BIC(BIC_SAM_mc6)) {
    if (p.sam_mc6_ms == -1) {	/* detect GFX counter reset */
    pub "")): *mut *mut *mut *mut *mut outp += sprintf(outp, "%s.", (printed++ ? delim :,
    } else {
    pub interval_float): outp += sprintf(outp, "%s%.2f", (printed++ ? delim : ""), p->sam_mc6_ms / 10.0 /,
    }
    }
// SAMMHz
    if (DO_BIC(BIC_SAMMHz))
    pub p->sam_mhz): outp += sprintf(outp, "%s%d", (printed++ ? delim : ""),,
// SAMACTMHz
    if (DO_BIC(BIC_SAMACTMHz))
    pub p->sam_act_mhz): outp += sprintf(outp, "%s%d", (printed++ ? delim : ""),,
// Totl%C0, Any%C0 GFX%C0 CPUGFX%
    if (DO_BIC(BIC_Totl_c0))
    pub /: *mut *mut *mut outp += sprintf(outp, "%s%.2f", (printed++ ? delim : ""), 100  p->pkg_wtd_core_c0 / tsc); / can exceed 100%,
    if (DO_BIC(BIC_Any_c0))
    pub tsc)): outp += sprintf(outp, "%s%.2f", (printed++ ? delim : ""), pct(p->pkg_any_core_c0,,
    if (DO_BIC(BIC_GFX_c0))
    pub tsc)): outp += sprintf(outp, "%s%.2f", (printed++ ? delim : ""), pct(p->pkg_any_gfxe_c0,,
    if (DO_BIC(BIC_CPUGFX))
    pub tsc)): outp += sprintf(outp, "%s%.2f", (printed++ ? delim : ""), pct(p->pkg_both_core_gfxe_c0,,
    if (DO_BIC(BIC_Pkgpc2))
    pub tsc)): outp += sprintf(outp, "%s%.2f", (printed++ ? delim : ""), pct(p->pc2,,
    if (DO_BIC(BIC_Pkgpc3))
    pub tsc)): outp += sprintf(outp, "%s%.2f", (printed++ ? delim : ""), pct(p->pc3,,
    if (DO_BIC(BIC_Pkgpc6))
    pub tsc)): outp += sprintf(outp, "%s%.2f", (printed++ ? delim : ""), pct(p->pc6,,
    if (DO_BIC(BIC_Pkgpc7))
    pub tsc)): outp += sprintf(outp, "%s%.2f", (printed++ ? delim : ""), pct(p->pc7,,
    if (DO_BIC(BIC_Pkgpc8))
    pub tsc)): outp += sprintf(outp, "%s%.2f", (printed++ ? delim : ""), pct(p->pc8,,
    if (DO_BIC(BIC_Pkgpc9))
    pub tsc)): outp += sprintf(outp, "%s%.2f", (printed++ ? delim : ""), pct(p->pc9,,
    if (DO_BIC(BIC_Pkgpc10))
    pub tsc)): outp += sprintf(outp, "%s%.2f", (printed++ ? delim : ""), pct(p->pc10,,
    if (DO_BIC(BIC_Diec6))
    pub interval_float)): outp += sprintf(outp, "%s%.2f", (printed++ ? delim : ""), pct(p->die_c6 / crystal_hz,,
    if (DO_BIC(BIC_CPU_LPI)) {
    if (p.cpu_lpi >= 0)
    pub interval_float)): outp += sprintf(outp, "%s%.2f", (printed++ ? delim : ""), pct(p->cpu_lpi / 1000000.0,,
    else
    pub "")): outp += sprintf(outp, "%s(neg)", (printed++ ? delim :,
    }
    if (DO_BIC(BIC_SYS_LPI)) {
    if (p.sys_lpi >= 0)
    pub interval_float)): outp += sprintf(outp, "%s%.2f", (printed++ ? delim : ""), pct(p->sys_lpi / 1000000.0,,
    else
    pub "")): outp += sprintf(outp, "%s(neg)", (printed++ ? delim :,
    }
    if (DO_BIC(BIC_PkgWatt))
    pub interval_float)): outp += sprintf(outp, fmt8, (printed++ ? delim : ""), rapl_counter_get_value(&p->energy_pkg, RAPL_UNIT_WATTS,,
    if (DO_BIC(BIC_CorWatt) && !platform.has_per_core_rapl)
    pub interval_float)): outp += sprintf(outp, fmt8, (printed++ ? delim : ""), rapl_counter_get_value(&p->energy_cores, RAPL_UNIT_WATTS,,
    if (DO_BIC(BIC_GFXWatt))
    pub interval_float)): outp += sprintf(outp, fmt8, (printed++ ? delim : ""), rapl_counter_get_value(&p->energy_gfx, RAPL_UNIT_WATTS,,
    if (DO_BIC(BIC_RAMWatt))
    pub interval_float)): outp += sprintf(outp, fmt8, (printed++ ? delim : ""), rapl_counter_get_value(&p->energy_dram, RAPL_UNIT_WATTS,,
    if (DO_BIC(BIC_Pkg_J))
    pub interval_float)): outp += sprintf(outp, fmt8, (printed++ ? delim : ""), rapl_counter_get_value(&p->energy_pkg, RAPL_UNIT_JOULES,,
    if (DO_BIC(BIC_Cor_J) && !platform.has_per_core_rapl)
    pub interval_float)): outp += sprintf(outp, fmt8, (printed++ ? delim : ""), rapl_counter_get_value(&p->energy_cores, RAPL_UNIT_JOULES,,
    if (DO_BIC(BIC_GFX_J))
    pub interval_float)): outp += sprintf(outp, fmt8, (printed++ ? delim : ""), rapl_counter_get_value(&p->energy_gfx, RAPL_UNIT_JOULES,,
    if (DO_BIC(BIC_RAM_J))
    pub interval_float)): outp += sprintf(outp, fmt8, (printed++ ? delim : ""), rapl_counter_get_value(&p->energy_dram, RAPL_UNIT_JOULES,,
    if (DO_BIC(BIC_PKG__))
    pub interval_float)): outp += sprintf(outp, fmt8, (printed++ ? delim : ""), rapl_counter_get_value(&p->rapl_pkg_perf_status, RAPL_UNIT_WATTS,,
    if (DO_BIC(BIC_RAM__))
    pub interval_float)): outp += sprintf(outp, fmt8, (printed++ ? delim : ""), rapl_counter_get_value(&p->rapl_dram_perf_status, RAPL_UNIT_WATTS,,
// UncMHz
    if (DO_BIC(BIC_UNCORE_MHZ))
    pub p->uncore_mhz): outp += sprintf(outp, "%s%d", (printed++ ? delim : ""),,
// Added Package Counters
    pub {: for (i = 0, mp = sys.pp; mp; i++, mp = mp->next),
    if (mp.format == FORMAT_RAW)
    pub p->counter[i]): outp += print_hex_value(mp->width, &printed, delim,,
#[no_mangle]
pub unsafe extern "C" fn if(COUNTER_K2M: mp->type ==) -> else {
    else if (mp.type == COUNTER_K2M)
    pub 1000): outp += sprintf(outp, "%s%d", (printed++ ? delim : ""), (unsigned int)p->counter[i] /,
#[no_mangle]
pub unsafe extern "C" fn if(FORMAT_AVERAGE: mp->format == FORMAT_DELTA || mp->format ==) -> else {
    else if (mp.format == FORMAT_DELTA || mp.format == FORMAT_AVERAGE)
    pub p->counter[i]): outp += print_decimal_value(mp->width, &printed, delim,,
#[no_mangle]
pub unsafe extern "C" fn if(FORMAT_PERCENT: mp->format ==) -> else {
    else if (mp.format == FORMAT_PERCENT)
    pub tsc)): outp += print_float_value(&printed, delim, pct(p->counter[i],,
    }
// Added perf Package Counters
    pub {: for (i = 0, pp = sys.perf_pp; pp; i++, pp = pp->next),
    if (pp.format == FORMAT_RAW)
    pub p->perf_counter[i]): outp += print_hex_value(pp->width, &printed, delim,,
#[no_mangle]
pub unsafe extern "C" fn if(COUNTER_K2M: pp->type ==) -> else {
    else if (pp.type == COUNTER_K2M)
    pub 1000): outp += sprintf(outp, "%s%d", (printed++ ? delim : ""), (unsigned int)p->perf_counter[i] /,
#[no_mangle]
pub unsafe extern "C" fn if(FORMAT_AVERAGE: pp->format == FORMAT_DELTA || pp->format ==) -> else {
    else if (pp.format == FORMAT_DELTA || pp.format == FORMAT_AVERAGE)
    pub p->perf_counter[i]): outp += print_decimal_value(pp->width, &printed, delim,,
#[no_mangle]
pub unsafe extern "C" fn if(FORMAT_PERCENT: pp->format ==) -> else {
    else if (pp.format == FORMAT_PERCENT)
    pub tsc)): outp += print_float_value(&printed, delim, pct(p->perf_counter[i],,
    }
// Added PMT Package Counters
    pub {: for (i = 0, ppmt = sys.pmt_pp; ppmt; i++, ppmt = ppmt->next),
    pub p->pmt_counter[i]: unsigned long value_raw =,
    pub value_converted: double,
    switch (ppmt.type) {
    case PMT_TYPE_RAW:
    pub p->pmt_counter[i]): outp += print_hex_value(pmt_counter_get_width(ppmt), &printed, delim,,
    case PMT_TYPE_XTAL_TIME:
    pub interval_float): value_converted = pct(value_raw / crystal_hz,,
    pub value_converted): outp += print_float_value(&printed, delim,,
    case PMT_TYPE_TCORE_CLOCK:
    pub interval_float): value_converted = pct(value_raw / tcore_clock_freq_hz,,
    pub value_converted): outp += print_float_value(&printed, delim,,
    }
    }
    if (DO_BIC(BIC_SysWatt) && (t == average.threads))
    pub interval_float)): outp += sprintf(outp, fmt8, (printed++ ? delim : ""), rapl_counter_get_value(&pplat_cnt->energy_psys, RAPL_UNIT_WATTS,,
    if (DO_BIC(BIC_Sys_J) && (t == average.threads))
    pub interval_float)): outp += sprintf(outp, fmt8, (printed++ ? delim : ""), rapl_counter_get_value(&pplat_cnt->energy_psys, RAPL_UNIT_JOULES,,
    done:
    if (*(outp - 1) != '\n')
    pub "\n"): outp += sprintf(outp,,
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn flush_output_stdout() {
    void flush_output_stdout(void)
    {
    pub filep: *mut FILE,
    if (outf == stderr)
    pub stdout: filep =,
    else
    pub outf: filep =,
    pub filep): fputs(output_buffer,,
    pub output_buffer: outp =,
    }
#[no_mangle]
pub unsafe extern "C" fn flush_output_stderr() {
    void flush_output_stderr(void)
    {
    pub outf): fputs(output_buffer,,
    pub output_buffer: outp =,
    }
#[no_mangle]
pub unsafe extern "C" fn format_all_counters(_arg: PER_THREAD_PARAMS) {
    void format_all_counters(PER_THREAD_PARAMS)
    {
    pub count: static int,
    if ((!count || (header_iterations && !(count % header_iterations))) || !summary_only)
    pub average.packages): format_counters(average.threads, average.cores,,
    if (summary_only)
    pub p): for_all_cpus(format_counters, t, c,,
    }

    pub 32): old = ((((unsigned long long)new << 32) - ((unsigned long long)old << 32)) >>,
#[no_mangle]
pub unsafe extern "C" fn delta_package(new: *mut pkg_data, old: *mut pkg_data) -> c_int {
    int delta_package(struct pkg_data *new, struct pkg_data *old)
    {
    pub i: c_int,
    pub mp: *mut msr_counter,
    pub pp: *mut perf_counter_info,
    pub ppmt: *mut pmt_counter,
    if (DO_BIC(BIC_Totl_c0))
    pub old->pkg_wtd_core_c0: old->pkg_wtd_core_c0 = new->pkg_wtd_core_c0 -,
    if (DO_BIC(BIC_Any_c0))
    pub old->pkg_any_core_c0: old->pkg_any_core_c0 = new->pkg_any_core_c0 -,
    if (DO_BIC(BIC_GFX_c0))
    pub old->pkg_any_gfxe_c0: old->pkg_any_gfxe_c0 = new->pkg_any_gfxe_c0 -,
    if (DO_BIC(BIC_CPUGFX))
    pub old->pkg_both_core_gfxe_c0: old->pkg_both_core_gfxe_c0 = new->pkg_both_core_gfxe_c0 -,
    pub old->pc2: old->pc2 = new->pc2 -,
    if (DO_BIC(BIC_Pkgpc3))
    pub old->pc3: old->pc3 = new->pc3 -,
    if (DO_BIC(BIC_Pkgpc6))
    pub old->pc6: old->pc6 = new->pc6 -,
    if (DO_BIC(BIC_Pkgpc7))
    pub old->pc7: old->pc7 = new->pc7 -,
    pub old->pc8: old->pc8 = new->pc8 -,
    pub old->pc9: old->pc9 = new->pc9 -,
    pub old->pc10: old->pc10 = new->pc10 -,
    pub old->die_c6: old->die_c6 = new->die_c6 -,
    pub old->cpu_lpi: old->cpu_lpi = new->cpu_lpi -,
    pub old->sys_lpi: old->sys_lpi = new->sys_lpi -,
    pub new->pkg_temp_c: old->pkg_temp_c =,
// flag an error when rc6 counter resets/wraps
    if (old.gfx_rc6_ms > new.gfx_rc6_ms)
    pub -1: old->gfx_rc6_ms =,
    else
    pub old->gfx_rc6_ms: old->gfx_rc6_ms = new->gfx_rc6_ms -,
    pub new->uncore_mhz: old->uncore_mhz =,
    pub new->gfx_mhz: old->gfx_mhz =,
    pub new->gfx_act_mhz: old->gfx_act_mhz =,
// flag an error when mc6 counter resets/wraps
    if (old.sam_mc6_ms > new.sam_mc6_ms)
    pub -1: old->sam_mc6_ms =,
    else
    pub old->sam_mc6_ms: old->sam_mc6_ms = new->sam_mc6_ms -,
    pub new->sam_mhz: old->sam_mhz =,
    pub new->sam_act_mhz: old->sam_act_mhz =,
    pub old->energy_pkg.raw_value: old->energy_pkg.raw_value = new->energy_pkg.raw_value -,
    pub old->energy_cores.raw_value: old->energy_cores.raw_value = new->energy_cores.raw_value -,
    pub old->energy_gfx.raw_value: old->energy_gfx.raw_value = new->energy_gfx.raw_value -,
    pub old->energy_dram.raw_value: old->energy_dram.raw_value = new->energy_dram.raw_value -,
    pub old->rapl_pkg_perf_status.raw_value: old->rapl_pkg_perf_status.raw_value = new->rapl_pkg_perf_status.raw_value -,
    pub old->rapl_dram_perf_status.raw_value: old->rapl_dram_perf_status.raw_value = new->rapl_dram_perf_status.raw_value -,
    pub {: for (i = 0, mp = sys.pp; mp; i++, mp = mp->next),
    if (mp.format == FORMAT_RAW)
    pub new->counter[i]: old->counter[i] =,
#[no_mangle]
pub unsafe extern "C" fn if(FORMAT_AVERAGE: mp->format ==) -> else {
    else if (mp.format == FORMAT_AVERAGE)
    pub new->counter[i]: old->counter[i] =,
    else
    pub old->counter[i]: old->counter[i] = new->counter[i] -,
    }
    pub {: for (i = 0, pp = sys.perf_pp; pp; i++, pp = pp->next),
    if (pp.format == FORMAT_RAW)
    pub new->perf_counter[i]: old->perf_counter[i] =,
#[no_mangle]
pub unsafe extern "C" fn if(FORMAT_AVERAGE: pp->format ==) -> else {
    else if (pp.format == FORMAT_AVERAGE)
    pub new->perf_counter[i]: old->perf_counter[i] =,
    else
    pub old->perf_counter[i]: old->perf_counter[i] = new->perf_counter[i] -,
    }
    pub {: for (i = 0, ppmt = sys.pmt_pp; ppmt; i++, ppmt = ppmt->next),
    if (ppmt.format == FORMAT_RAW)
    pub new->pmt_counter[i]: old->pmt_counter[i] =,
    else
    pub old->pmt_counter[i]: old->pmt_counter[i] = new->pmt_counter[i] -,
    }
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn delta_core(new: *mut core_data, old: *mut core_data) {
    void delta_core(struct core_data *new, struct core_data *old)
    {
    pub i: c_int,
    pub mp: *mut msr_counter,
    pub pp: *mut perf_counter_info,
    pub ppmt: *mut pmt_counter,
    pub old->c3: old->c3 = new->c3 -,
    pub old->c6: old->c6 = new->c6 -,
    pub old->c7: old->c7 = new->c7 -,
    pub new->core_temp_c: old->core_temp_c =,
    pub old->core_throt_cnt: old->core_throt_cnt = new->core_throt_cnt -,
    pub old->mc6_us: old->mc6_us = new->mc6_us -,
    pub old->core_energy.raw_value): DELTA_WRAP32(new->core_energy.raw_value,,
    pub {: for (i = 0, mp = sys.cp; mp; i++, mp = mp->next),
    if (mp.format == FORMAT_RAW || mp.format == FORMAT_AVERAGE)
    pub new->counter[i]: old->counter[i] =,
    else
    pub old->counter[i]: old->counter[i] = new->counter[i] -,
    }
    pub {: for (i = 0, pp = sys.perf_cp; pp; i++, pp = pp->next),
    if (pp.format == FORMAT_RAW)
    pub new->perf_counter[i]: old->perf_counter[i] =,
    else
    pub old->perf_counter[i]: old->perf_counter[i] = new->perf_counter[i] -,
    }
    pub {: for (i = 0, ppmt = sys.pmt_cp; ppmt; i++, ppmt = ppmt->next),
    if (ppmt.format == FORMAT_RAW)
    pub new->pmt_counter[i]: old->pmt_counter[i] =,
    else
    pub old->pmt_counter[i]: old->pmt_counter[i] = new->pmt_counter[i] -,
    }
    }
#[no_mangle]
pub unsafe extern "C" fn soft_c1_residency_display(bic: c_int) -> c_int {
    int soft_c1_residency_display(int bic)
    {
    if (!DO_BIC(BIC_CPU_c1) || platform.has_msr_core_c1_res)
    pub 0: return,
    pub DO_BIC_READ(bic): return,
    }
//
// old = new - old
//
#[no_mangle]
pub unsafe extern "C" fn delta_thread(new: *mut thread_data, old: *mut thread_data, core_delta: *mut core_data) -> c_int {
    int delta_thread(struct thread_data *new, struct thread_data *old, struct core_data *core_delta)
    {
    pub i: c_int,
    pub mp: *mut msr_counter,
    pub pp: *mut perf_counter_info,
    pub ppmt: *mut pmt_counter,
// we run cpuid just the 1st time, copy the results
    if (DO_BIC(BIC_APIC))
    pub old->apic_id: new->apic_id =,
    if (DO_BIC(BIC_X2APIC))
    pub old->x2apic_id: new->x2apic_id =,
//
// the timestamps from start of measurement interval are in "old"
// the timestamp from end of measurement interval are in "new"
// over-write old w/ new so we can print end of interval values
//
    pub &old->tv_delta): timersub(&new->tv_begin, &old->tv_begin,,
    pub new->tv_begin: old->tv_begin =,
    pub new->tv_end: old->tv_end =,
    pub old->tsc: old->tsc = new->tsc -,
// check for TSC < 1 Mcycles over interval
    if (old.tsc < (1000 * 1000))
    errx(-3, "Insanely slow TSC rate, TSC stops in idle?\n"
    pub \"processor.max_cstate=1\""): "You can disable all c-states by booting with \"idle=poll\"\nor just the deep ones with,
    pub old->c1: old->c1 = new->c1 -,
    if (DO_BIC(BIC_Avg_MHz) || DO_BIC(BIC_Busy) || DO_BIC(BIC_Bzy_MHz) || DO_BIC(BIC_IPC)
    || soft_c1_residency_display(BIC_Avg_MHz)) {
    if ((new.aperf > old.aperf) && (new.mperf > old.mperf)) {
    pub old->aperf: old->aperf = new->aperf -,
    pub old->mperf: old->mperf = new->mperf -,
    } else {
    pub -1: return,
    }
    }
    if (platform.has_msr_core_c1_res) {
//
// Some models have a dedicated C1 residency MSR,
// which should be more accurate than the derivation below.
//
    } else {
//
// As counter collection is not atomic,
// it is possible for mperf's non-halted cycles + idle states
// to exceed TSC's all cycles: show c1 = 0% in that case.
//
    if ((old.mperf + core_delta.c3 + core_delta.c6 + core_delta.c7) > (old.tsc * tsc_tweak))
    pub 0: old->c1 =,
    else {
// normal case, derive c1
    pub core_delta->c7: *mut *mut old->c1 = (old->tsc  tsc_tweak) - old->mperf - core_delta->c3 - core_delta->c6 -,
    }
    }
    if (old.mperf == 0) {
    if (debug > 1)
    pub old->cpu_id): fprintf(outf, "cpu%d MPERF 0!\n",,
    pub /: *mut *mut old->mperf = 1; / divide by 0 protection,
    }
    if (DO_BIC(BIC_IPC))
    pub old->instr_count: old->instr_count = new->instr_count -,
    if (DO_BIC(BIC_IRQ))
    pub old->irq_count: old->irq_count = new->irq_count -,
    if (DO_BIC(BIC_NMI))
    pub old->nmi_count: old->nmi_count = new->nmi_count -,
    if (DO_BIC(BIC_SMI))
    pub old->smi_count: old->smi_count = new->smi_count -,
    if (DO_BIC(BIC_LLC_MRPS) || DO_BIC(BIC_LLC_HIT))
    pub old->llc.references: old->llc.references = new->llc.references -,
    if (DO_BIC(BIC_LLC_HIT))
    pub old->llc.misses: old->llc.misses = new->llc.misses -,
    if (DO_BIC(BIC_L2_MRPS) || DO_BIC(BIC_L2_HIT))
    pub old->l2.references: old->l2.references = new->l2.references -,
    if (DO_BIC(BIC_L2_HIT))
    pub old->l2.hits: old->l2.hits = new->l2.hits -,
    pub {: for (i = 0, mp = sys.tp; mp; i++, mp = mp->next),
    if (mp.format == FORMAT_RAW || mp.format == FORMAT_AVERAGE)
    pub new->counter[i]: old->counter[i] =,
    else
    pub old->counter[i]: old->counter[i] = new->counter[i] -,
    }
    pub {: for (i = 0, pp = sys.perf_tp; pp; i++, pp = pp->next),
    if (pp.format == FORMAT_RAW)
    pub new->perf_counter[i]: old->perf_counter[i] =,
    else
    pub old->perf_counter[i]: old->perf_counter[i] = new->perf_counter[i] -,
    }
    pub {: for (i = 0, ppmt = sys.pmt_tp; ppmt; i++, ppmt = ppmt->next),
    if (ppmt.format == FORMAT_RAW)
    pub new->pmt_counter[i]: old->pmt_counter[i] =,
    else
    pub old->pmt_counter[i]: old->pmt_counter[i] = new->pmt_counter[i] -,
    }
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn delta_cpu(t: *mut thread_data, c: *mut core_data, p: *mut pkg_data, t2: *mut thread_data, c2: *mut core_data, p2: *mut pkg_data) -> c_int {
    int delta_cpu(struct thread_data *t, struct core_data *c, struct pkg_data *p, struct thread_data *t2, struct core_data *c2, struct pkg_data *p2)
    {
    pub 0: int retval =,
// calculate core delta only for 1st thread in core
    if (is_cpu_first_thread_in_core(t, c))
    pub c2): delta_core(c,,
// always calculate thread delta
    pub /: *mut *mut retval = delta_thread(t, t2, c2); / c2 is core delta,
// calculate package delta only for 1st core in package
    if (is_cpu_first_core_in_package(t, p))
    pub p2): retval |= delta_package(p,,
    pub retval: return,
    }
#[no_mangle]
pub unsafe extern "C" fn delta_platform(new: *mut platform_counters, old: *mut platform_counters) {
    void delta_platform(struct platform_counters *new, struct platform_counters *old)
    {
    pub old->energy_psys.raw_value: old->energy_psys.raw_value = new->energy_psys.raw_value -,
    }
#[no_mangle]
pub unsafe extern "C" fn rapl_counter_clear(c: *mut rapl_counter) {
    void rapl_counter_clear(struct rapl_counter *c)
    {
    pub 0: c->raw_value =,
    pub 0.0: c->scale =,
    pub RAPL_UNIT_INVALID: c->unit =,
    }
#[no_mangle]
pub unsafe extern "C" fn clear_counters(_arg: PER_THREAD_PARAMS) {
    void clear_counters(PER_THREAD_PARAMS)
    {
    pub i: c_int,
    pub mp: *mut msr_counter,
    pub 0: t->tv_begin.tv_sec =,
    pub 0: t->tv_begin.tv_usec =,
    pub 0: t->tv_end.tv_sec =,
    pub 0: t->tv_end.tv_usec =,
    pub 0: t->tv_delta.tv_sec =,
    pub 0: t->tv_delta.tv_usec =,
    pub 0: t->tsc =,
    pub 0: t->aperf =,
    pub 0: t->mperf =,
    pub 0: t->c1 =,
    pub 0: t->instr_count =,
    pub 0: t->irq_count =,
    pub 0: t->nmi_count =,
    pub 0: t->smi_count =,
    pub 0: t->llc.references =,
    pub 0: t->llc.misses =,
    pub 0: t->l2.references =,
    pub 0: t->l2.hits =,
    pub 0: c->c3 =,
    pub 0: c->c6 =,
    pub 0: c->c7 =,
    pub 0: c->mc6_us =,
    pub 0: c->core_temp_c =,
    pub 0: c->core_throt_cnt =,
    pub 0: p->pkg_wtd_core_c0 =,
    pub 0: p->pkg_any_core_c0 =,
    pub 0: p->pkg_any_gfxe_c0 =,
    pub 0: p->pkg_both_core_gfxe_c0 =,
    pub 0: p->pc2 =,
    if (DO_BIC(BIC_Pkgpc3))
    pub 0: p->pc3 =,
    if (DO_BIC(BIC_Pkgpc6))
    pub 0: p->pc6 =,
    if (DO_BIC(BIC_Pkgpc7))
    pub 0: p->pc7 =,
    pub 0: p->pc8 =,
    pub 0: p->pc9 =,
    pub 0: p->pc10 =,
    pub 0: p->die_c6 =,
    pub 0: p->cpu_lpi =,
    pub 0: p->sys_lpi =,
    pub 0: p->pkg_temp_c =,
    pub 0: p->gfx_rc6_ms =,
    pub 0: p->uncore_mhz =,
    pub 0: p->gfx_mhz =,
    pub 0: p->gfx_act_mhz =,
    pub 0: p->sam_mc6_ms =,
    pub 0: p->sam_mhz =,
    pub 0: p->sam_act_mhz =,
    pub mp->next): for (i = 0, mp = sys.tp; mp; i++, mp =,
    pub 0: t->counter[i] =,
    pub mp->next): for (i = 0, mp = sys.cp; mp; i++, mp =,
    pub 0: c->counter[i] =,
    pub mp->next): for (i = 0, mp = sys.pp; mp; i++, mp =,
    pub 0: p->counter[i] =,
    pub sizeof(t->perf_counter)): memset(&t->perf_counter[0], 0,,
    pub sizeof(c->perf_counter)): memset(&c->perf_counter[0], 0,,
    pub sizeof(p->perf_counter)): memset(&p->perf_counter[0], 0,,
    pub ARRAY_SIZE(t->pmt_counter)): memset(&t->pmt_counter[0], 0,,
    pub ARRAY_SIZE(c->pmt_counter)): memset(&c->pmt_counter[0], 0,,
    pub ARRAY_SIZE(p->pmt_counter)): memset(&p->pmt_counter[0], 0,,
    }
#[no_mangle]
pub unsafe extern "C" fn rapl_counter_accumulate(dst: *mut rapl_counter, src: *const rapl_counter) {
    void rapl_counter_accumulate(struct rapl_counter *dst, const struct rapl_counter *src)
    {
// Copy unit and scale from src if dst is not initialized
    if (dst.unit == RAPL_UNIT_INVALID) {
    pub src->unit: dst->unit =,
    pub src->scale: dst->scale =,
    }
    pub src->unit): assert(dst->unit ==,
    pub src->scale): assert(dst->scale ==,
    pub src->raw_value: dst->raw_value +=,
    }
#[no_mangle]
pub unsafe extern "C" fn sum_counters(_arg: PER_THREAD_PARAMS) -> c_int {
    int sum_counters(PER_THREAD_PARAMS)
    {
    pub i: c_int,
    pub mp: *mut msr_counter,
    pub pp: *mut perf_counter_info,
    pub ppmt: *mut pmt_counter,
// copy un-changing apic_id's
    if (DO_BIC(BIC_APIC))
    pub t->apic_id: average.threads->apic_id =,
    if (DO_BIC(BIC_X2APIC))
    pub t->x2apic_id: average.threads->x2apic_id =,
// remember first tv_begin
    if (average.threads.tv_begin.tv_sec == 0)
    pub procsysfs_tv_begin: average.threads->tv_begin =,
// remember last tv_end
    pub t->tv_end: average.threads->tv_end =,
    pub t->tsc: average.threads->tsc +=,
    pub t->aperf: average.threads->aperf +=,
    pub t->mperf: average.threads->mperf +=,
    pub t->c1: average.threads->c1 +=,
    pub t->instr_count: average.threads->instr_count +=,
    pub t->irq_count: average.threads->irq_count +=,
    pub t->nmi_count: average.threads->nmi_count +=,
    pub t->smi_count: average.threads->smi_count +=,
    pub t->llc.references: average.threads->llc.references +=,
    pub t->llc.misses: average.threads->llc.misses +=,
    pub t->l2.references: average.threads->l2.references +=,
    pub t->l2.hits: average.threads->l2.hits +=,
    pub {: for (i = 0, mp = sys.tp; mp; i++, mp = mp->next),
    if (mp.format == FORMAT_RAW)
    pub t->counter[i]: average.threads->counter[i] +=,
    }
    pub {: for (i = 0, pp = sys.perf_tp; pp; i++, pp = pp->next),
    if (pp.format == FORMAT_RAW)
    pub t->perf_counter[i]: average.threads->perf_counter[i] +=,
    }
    pub {: for (i = 0, ppmt = sys.pmt_tp; ppmt; i++, ppmt = ppmt->next),
    pub t->pmt_counter[i]: average.threads->pmt_counter[i] +=,
    }
// sum per-core values only for 1st thread in core
    if (!is_cpu_first_thread_in_core(t, c))
    pub 0: return,
    pub c->c3: average.cores->c3 +=,
    pub c->c6: average.cores->c6 +=,
    pub c->c7: average.cores->c7 +=,
    pub c->mc6_us: average.cores->mc6_us +=,
    pub c->core_temp_c): average.cores->core_temp_c = MAX(average.cores->core_temp_c,,
    pub c->core_throt_cnt): average.cores->core_throt_cnt = MAX(average.cores->core_throt_cnt,,
    pub &c->core_energy): rapl_counter_accumulate(&average.cores->core_energy,,
    pub {: for (i = 0, mp = sys.cp; mp; i++, mp = mp->next),
    if (mp.format == FORMAT_RAW)
    pub c->counter[i]: average.cores->counter[i] +=,
    }
    pub {: for (i = 0, pp = sys.perf_cp; pp; i++, pp = pp->next),
    if (pp.format == FORMAT_RAW)
    pub c->perf_counter[i]: average.cores->perf_counter[i] +=,
    }
    pub {: for (i = 0, ppmt = sys.pmt_cp; ppmt; i++, ppmt = ppmt->next),
    pub c->pmt_counter[i]: average.cores->pmt_counter[i] +=,
    }
// sum per-pkg values only for 1st core in pkg
    if (!is_cpu_first_core_in_package(t, p))
    pub 0: return,
    if (DO_BIC(BIC_Totl_c0))
    pub p->pkg_wtd_core_c0: average.packages->pkg_wtd_core_c0 +=,
    if (DO_BIC(BIC_Any_c0))
    pub p->pkg_any_core_c0: average.packages->pkg_any_core_c0 +=,
    if (DO_BIC(BIC_GFX_c0))
    pub p->pkg_any_gfxe_c0: average.packages->pkg_any_gfxe_c0 +=,
    if (DO_BIC(BIC_CPUGFX))
    pub p->pkg_both_core_gfxe_c0: average.packages->pkg_both_core_gfxe_c0 +=,
    pub p->pc2: average.packages->pc2 +=,
    if (DO_BIC(BIC_Pkgpc3))
    pub p->pc3: average.packages->pc3 +=,
    if (DO_BIC(BIC_Pkgpc6))
    pub p->pc6: average.packages->pc6 +=,
    if (DO_BIC(BIC_Pkgpc7))
    pub p->pc7: average.packages->pc7 +=,
    pub p->pc8: average.packages->pc8 +=,
    pub p->pc9: average.packages->pc9 +=,
    pub p->pc10: average.packages->pc10 +=,
    pub p->die_c6: average.packages->die_c6 +=,
    pub p->cpu_lpi: average.packages->cpu_lpi =,
    pub p->sys_lpi: average.packages->sys_lpi =,
    pub &p->energy_pkg): rapl_counter_accumulate(&average.packages->energy_pkg,,
    pub &p->energy_dram): rapl_counter_accumulate(&average.packages->energy_dram,,
    pub &p->energy_cores): rapl_counter_accumulate(&average.packages->energy_cores,,
    pub &p->energy_gfx): rapl_counter_accumulate(&average.packages->energy_gfx,,
    pub p->gfx_rc6_ms: average.packages->gfx_rc6_ms =,
    pub p->uncore_mhz: average.packages->uncore_mhz =,
    pub p->gfx_mhz: average.packages->gfx_mhz =,
    pub p->gfx_act_mhz: average.packages->gfx_act_mhz =,
    pub p->sam_mc6_ms: average.packages->sam_mc6_ms =,
    pub p->sam_mhz: average.packages->sam_mhz =,
    pub p->sam_act_mhz: average.packages->sam_act_mhz =,
    pub p->pkg_temp_c): average.packages->pkg_temp_c = MAX(average.packages->pkg_temp_c,,
    pub &p->rapl_pkg_perf_status): rapl_counter_accumulate(&average.packages->rapl_pkg_perf_status,,
    pub &p->rapl_dram_perf_status): rapl_counter_accumulate(&average.packages->rapl_dram_perf_status,,
    pub {: for (i = 0, mp = sys.pp; mp; i++, mp = mp->next),
    if ((mp.format == FORMAT_RAW) && (topo.num_packages == 0))
    pub p->counter[i]: average.packages->counter[i] =,
    else
    pub p->counter[i]: average.packages->counter[i] +=,
    }
    pub {: for (i = 0, pp = sys.perf_pp; pp; i++, pp = pp->next),
    if ((pp.format == FORMAT_RAW) && (topo.num_packages == 0))
    pub p->perf_counter[i]: average.packages->perf_counter[i] =,
    else
    pub p->perf_counter[i]: average.packages->perf_counter[i] +=,
    }
    pub {: for (i = 0, ppmt = sys.pmt_pp; ppmt; i++, ppmt = ppmt->next),
    pub p->pmt_counter[i]: average.packages->pmt_counter[i] +=,
    }
    pub 0: return,
    }
//
// sum the counters for all cpus in the system
// compute the weighted average
//
#[no_mangle]
pub unsafe extern "C" fn compute_average(_arg: PER_THREAD_PARAMS) {
    void compute_average(PER_THREAD_PARAMS)
    {
    pub i: c_int,
    pub mp: *mut msr_counter,
    pub pp: *mut perf_counter_info,
    pub ppmt: *mut pmt_counter,
    pub average.packages): clear_counters(average.threads, average.cores,,
    pub p): for_all_cpus(sum_counters, t, c,,
// Use the global time delta for the average.
    pub tv_delta: average.threads->tv_delta =,
    pub topo.allowed_cpus: average.threads->tsc /=,
    pub topo.allowed_cpus: average.threads->aperf /=,
    pub topo.allowed_cpus: average.threads->mperf /=,
    pub topo.allowed_cpus: average.threads->instr_count /=,
    pub topo.allowed_cpus: average.threads->c1 /=,
    if (average.threads.irq_count > 9999999)
    pub 1: sums_need_wide_columns =,
    if (average.threads.nmi_count > 9999999)
    pub 1: sums_need_wide_columns =,
    pub topo.allowed_cores: average.cores->c3 /=,
    pub topo.allowed_cores: average.cores->c6 /=,
    pub topo.allowed_cores: average.cores->c7 /=,
    pub topo.allowed_cores: average.cores->mc6_us /=,
    if (DO_BIC(BIC_Totl_c0))
    pub topo.allowed_packages: average.packages->pkg_wtd_core_c0 /=,
    if (DO_BIC(BIC_Any_c0))
    pub topo.allowed_packages: average.packages->pkg_any_core_c0 /=,
    if (DO_BIC(BIC_GFX_c0))
    pub topo.allowed_packages: average.packages->pkg_any_gfxe_c0 /=,
    if (DO_BIC(BIC_CPUGFX))
    pub topo.allowed_packages: average.packages->pkg_both_core_gfxe_c0 /=,
    pub topo.allowed_packages: average.packages->pc2 /=,
    if (DO_BIC(BIC_Pkgpc3))
    pub topo.allowed_packages: average.packages->pc3 /=,
    if (DO_BIC(BIC_Pkgpc6))
    pub topo.allowed_packages: average.packages->pc6 /=,
    if (DO_BIC(BIC_Pkgpc7))
    pub topo.allowed_packages: average.packages->pc7 /=,
    pub topo.allowed_packages: average.packages->pc8 /=,
    pub topo.allowed_packages: average.packages->pc9 /=,
    pub topo.allowed_packages: average.packages->pc10 /=,
    pub topo.allowed_packages: average.packages->die_c6 /=,
    pub {: for (i = 0, mp = sys.tp; mp; i++, mp = mp->next),
    if (mp.format == FORMAT_RAW)
    if (mp.type == COUNTER_ITEMS) {
    if (average.threads.counter[i] > 9999999)
    pub 1: sums_need_wide_columns =,
    }
    pub topo.allowed_cpus: average.threads->counter[i] /=,
    }
    pub {: for (i = 0, mp = sys.cp; mp; i++, mp = mp->next),
    if (mp.format == FORMAT_RAW)
    if (mp.type == COUNTER_ITEMS) {
    if (average.cores.counter[i] > 9999999)
    pub 1: sums_need_wide_columns =,
    }
    pub topo.allowed_cores: average.cores->counter[i] /=,
    }
    pub {: for (i = 0, mp = sys.pp; mp; i++, mp = mp->next),
    if (mp.format == FORMAT_RAW)
    if (mp.type == COUNTER_ITEMS) {
    if (average.packages.counter[i] > 9999999)
    pub 1: sums_need_wide_columns =,
    }
    pub topo.allowed_packages: average.packages->counter[i] /=,
    }
    pub {: for (i = 0, pp = sys.perf_tp; pp; i++, pp = pp->next),
    if (pp.format == FORMAT_RAW)
    if (pp.type == COUNTER_ITEMS) {
    if (average.threads.perf_counter[i] > 9999999)
    pub 1: sums_need_wide_columns =,
    }
    pub topo.allowed_cpus: average.threads->perf_counter[i] /=,
    }
    pub {: for (i = 0, pp = sys.perf_cp; pp; i++, pp = pp->next),
    if (pp.format == FORMAT_RAW)
    if (pp.type == COUNTER_ITEMS) {
    if (average.cores.perf_counter[i] > 9999999)
    pub 1: sums_need_wide_columns =,
    }
    pub topo.allowed_cores: average.cores->perf_counter[i] /=,
    }
    pub {: for (i = 0, pp = sys.perf_pp; pp; i++, pp = pp->next),
    if (pp.format == FORMAT_RAW)
    if (pp.type == COUNTER_ITEMS) {
    if (average.packages.perf_counter[i] > 9999999)
    pub 1: sums_need_wide_columns =,
    }
    pub topo.allowed_packages: average.packages->perf_counter[i] /=,
    }
    pub {: for (i = 0, ppmt = sys.pmt_tp; ppmt; i++, ppmt = ppmt->next),
    pub topo.allowed_cpus: average.threads->pmt_counter[i] /=,
    }
    pub {: for (i = 0, ppmt = sys.pmt_cp; ppmt; i++, ppmt = ppmt->next),
    pub topo.allowed_cores: average.cores->pmt_counter[i] /=,
    }
    pub {: for (i = 0, ppmt = sys.pmt_pp; ppmt; i++, ppmt = ppmt->next),
    pub topo.allowed_packages: average.packages->pmt_counter[i] /=,
    }
    }
#[no_mangle]
unsafe extern "C" fn rdtsc() -> c_ulonglong {
    static unsigned long long rdtsc(void)
    {
    pub high: unsigned int low,,
    pub "=d"(high)): asm volatile ("rdtsc":"=a" (low),,
    pub 32: return low | ((unsigned long long)high) <<,
    }
//
// Open a file, and exit on failure
//
    FILE *fopen_or_die(const char *path, const char *mode)
    {
    pub mode): *mut *mut FILE filep = fopen(path,,
    if (!filep)
    pub path): err(1, "%s: open failed",,
    pub filep: return,
    }
//
// snapshot_sysfs_counter()
//
// return snapshot of given counter
//
#[no_mangle]
pub unsafe extern "C" fn snapshot_sysfs_counter(path: *mut c_char) -> c_ulonglong {
    unsigned long long snapshot_sysfs_counter(char *path)
    {
    pub fp: *mut FILE,
    pub retval: c_int,
    pub counter: c_ulonglong,
    pub "r"): fp = fopen_or_die(path,,
    pub &counter): retval = fscanf(fp, "%lld",,
    if (retval != 1)
    pub path): err(1, "snapshot_sysfs_counter(%s)",,
    pub counter: return,
    }
#[no_mangle]
pub unsafe extern "C" fn get_mp(cpu: c_int, mp: *mut msr_counter, counterp: *mut c_ulonglong, counter_path: *mut c_char) -> c_int {
    int get_mp(int cpu, struct msr_counter *mp, unsigned long long *counterp, char *counter_path)
    {
    if (mp.msr_num != 0) {
    if (get_msr(cpu, mp.msr_num, counterp))
    pub -1: return,
    } else {
    pub PATH_BYTES]: char path[128 +,
    if (mp.flags & SYSFS_PERCPU) {
    pub mp->sp->path): sprintf(path, "/sys/devices/system/cpu/cpu%d/%s", cpu,,
// counterp = snapshot_sysfs_counter(path);
    } else {
// counterp = snapshot_sysfs_counter(counter_path);
    }
    }
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn get_legacy_uncore_mhz(package: c_int) -> c_ulonglong {
    unsigned long long get_legacy_uncore_mhz(int package)
    {
    pub path: [c_char; 128],
    pub die: c_int,
    pub warn_once: static int,
//
// for this package, use the first die_id that exists
//
    pub {: for (die = 0; die <= topo.max_die_id; ++die),
    pub die): sprintf(path, "/sys/devices/system/cpu/intel_uncore_frequency/package_%02d_die_%02d/current_freq_khz", package,,
    if (access(path, R_OK) == 0)
    pub 1000): return (snapshot_sysfs_counter(path) /,
    }
    if (!warn_once) {
    pub path): warnx("BUG: %s: No %s", __func__,,
    pub 1: warn_once =,
    }
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn get_epb(cpu: c_int) -> c_int {
    int get_epb(int cpu)
    {
    pub PATH_BYTES]: char path[128 +,
    pub msr: c_ulonglong,
    pub -1: int ret, epb =,
    pub fp: *mut FILE,
    pub cpu): sprintf(path, "/sys/devices/system/cpu/cpu%d/power/energy_perf_bias",,
    pub "r"): fp = fopen(path,,
    if (!fp)
    pub msr_fallback: goto,
    pub &epb): ret = fscanf(fp, "%d",,
    if (ret != 1)
    pub path): err(1, "%s(%s)", __func__,,
    pub epb: return,
    msr_fallback:
    if (no_msr)
    pub -1: return,
    pub &msr): get_msr(cpu, MSR_IA32_ENERGY_PERF_BIAS,,
    pub 0xf: return msr &,
    }
#[no_mangle]
pub unsafe extern "C" fn get_apic_id(t: *mut thread_data) {
    void get_apic_id(struct thread_data *t)
    {
    pub edx: unsigned int eax, ebx, ecx,,
    if (DO_BIC(BIC_APIC)) {
    pub 0: eax = ebx = ecx = edx =,
    pub edx): __cpuid(1, eax, ebx, ecx,,
    pub 0xff: t->apic_id = (ebx >> 24) &,
    }
    if (!DO_BIC(BIC_X2APIC))
    if (authentic_amd || hygon_genuine) {
    pub topology_extensions: c_uint,
    if (max_extended_level < 0x8000001e)
    pub 0: eax = ebx = ecx = edx =,
    pub edx): __cpuid(0x80000001, eax, ebx, ecx,,
    pub 22): topology_extensions = ecx & (1 <<,
    if (topology_extensions == 0)
    pub 0: eax = ebx = ecx = edx =,
    pub edx): __cpuid(0x8000001e, eax, ebx, ecx,,
    pub eax: t->x2apic_id =,
    }
    if (!genuine_intel)
    if (max_level < 0xb)
    pub 0: ecx =,
    pub edx): __cpuid(0xb, eax, ebx, ecx,,
    pub edx: t->x2apic_id =,
    if (debug && (t.apic_id != (t.x2apic_id & 0xff)))
    pub t->x2apic_id): fprintf(outf, "cpu%d: BIOS BUG: apic 0x%x x2apic 0x%x\n", t->cpu_id, t->apic_id,,
    }
#[no_mangle]
pub unsafe extern "C" fn get_core_throt_cnt(cpu: c_int, cnt: *mut c_ulonglong) -> c_int {
    int get_core_throt_cnt(int cpu, unsigned long long *cnt)
    {
    pub PATH_BYTES]: char path[128 +,
    pub tmp: c_ulonglong,
    pub fp: *mut FILE,
    pub ret: c_int,
    pub cpu): sprintf(path, "/sys/devices/system/cpu/cpu%d/thermal_throttle/core_throttle_count",,
    pub "r"): fp = fopen(path,,
    if (!fp)
    pub -1: return,
    pub &tmp): ret = fscanf(fp, "%lld",,
    if (ret != 1)
    pub -1: return,
// cnt = tmp;
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn read_perf_counter_info(path: *const *const c_char, parse_format: *const *const c_char, value_ptr: *mut c_void) -> c_int {
    static int read_perf_counter_info(const char *const path, const char *const parse_format, void *value_ptr)
    {
    pub fdmt: c_int,
    pub bytes_read: c_int,
    pub buf: [c_char; 64],
    pub -1: int ret =,
    pub 0): fdmt = open(path, O_RDONLY,,
    if (fdmt == -1) {
    if (debug)
    pub path): fprintf(stderr, "Failed to parse perf counter info %s\n",,
    pub -1: ret =,
    pub cleanup_and_exit: goto,
    }
    pub 1): bytes_read = read(fdmt, buf, sizeof(buf) -,
    if (bytes_read <= 0 || bytes_read >= (int)sizeof(buf)) {
    if (debug)
    pub path): fprintf(stderr, "Failed to parse perf counter info %s\n",,
    pub -1: ret =,
    pub cleanup_and_exit: goto,
    }
    pub '\0': buf[bytes_read] =,
    if (sscanf(buf, parse_format, value_ptr) != 1) {
    if (debug)
    pub path): fprintf(stderr, "Failed to parse perf counter info %s\n",,
    pub -1: ret =,
    pub cleanup_and_exit: goto,
    }
    pub 0: ret =,
    cleanup_and_exit:
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn read_perf_counter_info_n(path: *const *const c_char, parse_format: *const *const c_char) -> c_uint {
    static unsigned int read_perf_counter_info_n(const char *const path, const char *const parse_format)
    {
    pub v: c_uint,
    pub status: c_int,
    pub &v): status = read_perf_counter_info(path, parse_format,,
    if (status)
    pub -1: v =,
    pub v: return,
    }
#[no_mangle]
unsafe extern "C" fn read_perf_type(subsys: *const c_char) -> c_uint {
    static unsigned int read_perf_type(const char *subsys)
    {
    pub "/sys/bus/event_source/devices/%s/type": *const *const char path_format =,
    pub "%u": *const *const char format =,
    pub path: [c_char; 128],
    pub subsys): snprintf(path, sizeof(path), path_format,,
    pub format): return read_perf_counter_info_n(path,,
    }
#[no_mangle]
unsafe extern "C" fn read_perf_config(subsys: *const c_char, event_name: *const c_char) -> c_uint {
    static unsigned int read_perf_config(const char *subsys, const char *event_name)
    {
    pub "/sys/bus/event_source/devices/%s/events/%s": *const *const char path_format =,
    pub NULL: *mut *mut FILE fconfig =,
    pub path: [c_char; 128],
    pub config_str: [c_char; 64],
    pub config: c_uint,
    pub umask: c_uint,
    pub false: bool has_config =,
    pub false: bool has_umask =,
    pub -1: unsigned int ret =,
    pub event_name): snprintf(path, sizeof(path), path_format, subsys,,
    pub "r"): fconfig = fopen(path,,
    if (!fconfig)
    pub -1: return,
    if (fgets(config_str, ARRAY_SIZE(config_str), fconfig) != config_str)
    pub cleanup_and_exit: goto,
    pub {: *mut *mut for (char pconfig_str = &config_str[0]; pconfig_str;),
    if (sscanf(pconfig_str, "event=%x", &config) == 1) {
    pub true: has_config =,
    pub next: goto,
    }
    if (sscanf(pconfig_str, "umask=%x", &umask) == 1) {
    pub true: has_umask =,
    pub next: goto,
    }
    next:
    pub ','): pconfig_str = strchr(pconfig_str,,
    if (pconfig_str) {
// pconfig_str = '\0';
    }
    }
    if (!has_umask)
    pub 0: umask =,
    if (has_config)
    pub config: ret = (umask << 8) |,
    cleanup_and_exit:
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn read_perf_rapl_unit(subsys: *const c_char, event_name: *const c_char) -> c_uint {
    static unsigned int read_perf_rapl_unit(const char *subsys, const char *event_name)
    {
    pub "/sys/bus/event_source/devices/%s/events/%s.unit": *const *const char path_format =,
    pub "%s": *const *const char format =,
    pub path: [c_char; 128],
    pub unit_buffer: [c_char; 16],
    pub event_name): snprintf(path, sizeof(path), path_format, subsys,,
    pub &unit_buffer): read_perf_counter_info(path, format,,
    if (strcmp("Joules", unit_buffer) == 0)
    pub RAPL_UNIT_JOULES: return,
    pub RAPL_UNIT_INVALID: return,
    }
#[no_mangle]
unsafe extern "C" fn read_perf_scale(subsys: *const c_char, event_name: *const c_char) -> double {
    static double read_perf_scale(const char *subsys, const char *event_name)
    {
    pub "/sys/bus/event_source/devices/%s/events/%s.scale": *const *const char path_format =,
    pub "%lf": *const *const char format =,
    pub path: [c_char; 128],
    pub scale: double,
    pub event_name): snprintf(path, sizeof(path), path_format, subsys,,
    if (read_perf_counter_info(path, format, &scale))
    pub 0.0: return,
    pub scale: return,
    }
#[no_mangle]
pub unsafe extern "C" fn rapl_counter_info_count_perf(rci: *const rapl_counter_info_t) -> usize {
    size_t rapl_counter_info_count_perf(const struct rapl_counter_info_t *rci)
    {
    pub 0: size_t ret =,
    pub ++i): for (int i = 0; i < NUM_RAPL_COUNTERS;,
    if (rci.source[i] == COUNTER_SOURCE_PERF)
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn cstate_counter_info_count_perf(cci: *const cstate_counter_info_t) -> usize {
    static size_t cstate_counter_info_count_perf(const struct cstate_counter_info_t *cci)
    {
    pub 0: size_t ret =,
    pub ++i): for (int i = 0; i < NUM_CSTATE_COUNTERS;,
    if (cci.source[i] == COUNTER_SOURCE_PERF)
    pub ret: return,
    }
#[no_mangle]
pub unsafe extern "C" fn write_rapl_counter(rc: *mut rapl_counter, rci: *mut rapl_counter_info_t, idx: c_uint) {
    void write_rapl_counter(struct rapl_counter *rc, struct rapl_counter_info_t *rci, unsigned int idx)
    {
    if (rci.source[idx] == COUNTER_SOURCE_NONE)
    pub rci->data[idx]: rc->raw_value =,
    pub rci->unit[idx]: rc->unit =,
    pub rci->scale[idx]: rc->scale =,
    }
#[no_mangle]
pub unsafe extern "C" fn get_rapl_counters(cpu: c_int, domain: c_uint, c: *mut core_data, p: *mut pkg_data) -> c_int {
    int get_rapl_counters(int cpu, unsigned int domain, struct core_data *c, struct pkg_data *p)
    {
    pub &platform_counters_even: *mut *mut platform_counters pplat_cnt = p == odd.packages ? &platform_counters_odd :,
    pub 1]: unsigned long long perf_data[NUM_RAPL_COUNTERS +,
    pub rci: *mut rapl_counter_info_t,
    if (debug >= 2)
    pub domain): fprintf(stderr, "%s: cpu%d domain%d\n", __func__, cpu,,
    pub rapl_counter_info_perdomain_size): assert(domain <,
    pub &rapl_counter_info_perdomain[domain]: rci =,
//
// If we have any perf counters to read, read them all now, in bulk
//
    if (rci.fd_perf != -1) {
    pub rapl_counter_info_count_perf(rci): size_t num_perf_counters =,
    pub long): *const *const ssize_t expected_read_size = (num_perf_counters + 1)  sizeof(unsigned long,
    pub sizeof(perf_data)): ssize_t actual_read_size = read(rci->fd_perf, &perf_data[0],,
    if (actual_read_size != expected_read_size)
    pub actual_read_size): err(-1, "%s: failed to read perf_data (%zu %zu)", __func__, expected_read_size,,
    }
    pub {: for (unsigned int i = 0, pi = 1; i < NUM_RAPL_COUNTERS; ++i),
    switch (rci.source[i]) {
    case COUNTER_SOURCE_NONE:
    pub 0: rci->data[i] =,
    case COUNTER_SOURCE_PERF:
    pub ARRAY_SIZE(perf_data)): assert(pi <,
    pub -1): assert(rci->fd_perf !=,
    if (debug >= 2)
    fprintf(stderr, "Reading rapl counter via perf at %u (%llu %e %lf)\n",
    pub rci->scale[i]): *mut *mut i, perf_data[pi], rci->scale[i], perf_data[pi],
    pub perf_data: [rci->data[i] =; pi],
    case COUNTER_SOURCE_MSR:
    if (debug >= 2)
    pub i): fprintf(stderr, "Reading rapl counter via msr at %u\n",,
    if (rci.flags[i] & RAPL_COUNTER_FLAG_USE_MSR_SUM) {
    if (get_msr_sum(cpu, rci.msr[i], &rci.data[i]))
    pub i: return -13 -,
    } else {
    if (get_msr(cpu, rci.msr[i], &rci.data[i]))
    pub i: return -13 -,
    }
    pub rci->msr_mask[i]: rci->data[i] &=,
    if (rci.msr_shift[i] >= 0)
    pub abs(rci->msr_shift[i]): rci->data[i] >>=,
    else
    pub abs(rci->msr_shift[i]): rci->data[i] <<=,
    }
    }
    pub 8): BUILD_BUG_ON(NUM_RAPL_COUNTERS !=,
    pub RAPL_RCI_INDEX_ENERGY_PKG): write_rapl_counter(&p->energy_pkg, rci,,
    pub RAPL_RCI_INDEX_ENERGY_CORES): write_rapl_counter(&p->energy_cores, rci,,
    pub RAPL_RCI_INDEX_DRAM): write_rapl_counter(&p->energy_dram, rci,,
    pub RAPL_RCI_INDEX_GFX): write_rapl_counter(&p->energy_gfx, rci,,
    pub RAPL_RCI_INDEX_PKG_PERF_STATUS): write_rapl_counter(&p->rapl_pkg_perf_status, rci,,
    pub RAPL_RCI_INDEX_DRAM_PERF_STATUS): write_rapl_counter(&p->rapl_dram_perf_status, rci,,
    pub RAPL_RCI_INDEX_CORE_ENERGY): write_rapl_counter(&c->core_energy, rci,,
    pub RAPL_RCI_INDEX_ENERGY_PLATFORM): write_rapl_counter(&pplat_cnt->energy_psys, rci,,
    pub 0: return,
    }
    char *find_sysfs_path_by_id(struct sysfs_path *sp, int id)
    {
    while (sp) {
    if (sp.id == id)
    pub (sp->path): return,
    pub sp->next: sp =,
    }
    if (debug)
    pub id): warnx("%s: id%d not found", __func__,,
    pub NULL: return,
    }
#[no_mangle]
pub unsafe extern "C" fn get_cstate_counters(cpu: c_uint, _arg: PER_THREAD_PARAMS) -> c_int {
    int get_cstate_counters(unsigned int cpu, PER_THREAD_PARAMS)
    {
//
// Overcommit memory a little bit here,
// but skip calculating exact sizes for the buffers.
//
    pub perf_data: [c_ulonglong; NUM_CSTATE_COUNTERS],
    pub 1]: unsigned long long perf_data_core[NUM_CSTATE_COUNTERS +,
    pub 1]: unsigned long long perf_data_pkg[NUM_CSTATE_COUNTERS +,
    pub cci: *mut cstate_counter_info_t,
    if (debug >= 2)
    pub cpu): fprintf(stderr, "%s: cpu%d\n", __func__,,
    pub ccstate_counter_info_size): assert(cpu <=,
    pub &ccstate_counter_info[cpu]: cci =,
//
// If we have any perf counters to read, read them all now, in bulk
//
    pub cstate_counter_info_count_perf(cci): size_t num_perf_counters =,
    pub long): *mut *mut ssize_t expected_read_size = num_perf_counters  sizeof(unsigned long,
    pub 0: ssize_t actual_read_size_core = 0, actual_read_size_pkg =,
    if (cci.fd_perf_core != -1) {
// Each descriptor read begins with number of counters read.
    pub long): expected_read_size += sizeof(unsigned long,
    pub sizeof(perf_data_core)): actual_read_size_core = read(cci->fd_perf_core, &perf_data_core[0],,
    if (actual_read_size_core <= 0)
    pub actual_read_size_core): err(-1, "%s: read perf %s: %ld", __func__, "core",,
    }
    if (cci.fd_perf_pkg != -1) {
// Each descriptor read begins with number of counters read.
    pub long): expected_read_size += sizeof(unsigned long,
    pub sizeof(perf_data_pkg)): actual_read_size_pkg = read(cci->fd_perf_pkg, &perf_data_pkg[0],,
    if (actual_read_size_pkg <= 0)
    pub actual_read_size_pkg): err(-1, "%s: read perf %s: %ld", __func__, "pkg",,
    }
    pub actual_read_size_pkg: ssize_t actual_read_size_total = actual_read_size_core +,
    if (actual_read_size_total != expected_read_size)
    pub actual_read_size_total): err(-1, "%s: failed to read perf_data (%zu %zu)", __func__, expected_read_size,,
//
// Copy ccstate and pcstate data into unified buffer.
//
// Skip first element from core and pkg buffers.
// Kernel puts there how many counters were read.
//
    pub perf_data_core: [size_t num_core_counters =; 0],
    pub perf_data_pkg: [size_t num_pkg_counters =; 0],
    pub num_pkg_counters): assert(num_perf_counters == num_core_counters +,
// Copy ccstate perf data
    pub long)): *mut *mut memcpy(&perf_data[0], &perf_data_core[1], num_core_counters  sizeof(unsigned long,
// Copy pcstate perf data
    pub long)): *mut *mut memcpy(&perf_data[num_core_counters], &perf_data_pkg[1], num_pkg_counters  sizeof(unsigned long,
    pub {: for (unsigned int i = 0, pi = 0; i < NUM_CSTATE_COUNTERS; ++i),
    switch (cci.source[i]) {
    case COUNTER_SOURCE_NONE:
    case COUNTER_SOURCE_PERF:
    pub ARRAY_SIZE(perf_data)): assert(pi <,
    pub -1): assert(cci->fd_perf_core != -1 || cci->fd_perf_pkg !=,
    if (debug >= 2)
    pub perf_data[pi]): fprintf(stderr, "cstate via %s %u: %llu\n", "perf", i,,
    pub perf_data: [cci->data[i] =; pi],
    case COUNTER_SOURCE_MSR:
    if (get_msr(cpu, cci.msr[i], &cci.data[i]))
    pub i: return -13 -,
    if (debug >= 2)
    pub cci->data[i]): fprintf(stderr, "cstate via %s0x%llx %u: %llu\n", "msr", cci->msr[i], i,,
    }
    }
//
// Helper to write the data only if the source of
// the counter for the current cpu is not none.
//
// Otherwise we would overwrite core data with 0 (default value),
// when invoked for the thread sibling.
//

    if (cci.source[index] != COUNTER_SOURCE_NONE)		\
    pub \: out_counter = cci->data[index];,
    } while (0)
    pub 11): BUILD_BUG_ON(NUM_CSTATE_COUNTERS !=,
    pub CCSTATE_RCI_INDEX_C1_RESIDENCY): PERF_COUNTER_WRITE_DATA(t->c1,,
    pub CCSTATE_RCI_INDEX_C3_RESIDENCY): PERF_COUNTER_WRITE_DATA(c->c3,,
    pub CCSTATE_RCI_INDEX_C6_RESIDENCY): PERF_COUNTER_WRITE_DATA(c->c6,,
    pub CCSTATE_RCI_INDEX_C7_RESIDENCY): PERF_COUNTER_WRITE_DATA(c->c7,,
    pub PCSTATE_RCI_INDEX_C2_RESIDENCY): PERF_COUNTER_WRITE_DATA(p->pc2,,
    pub PCSTATE_RCI_INDEX_C3_RESIDENCY): PERF_COUNTER_WRITE_DATA(p->pc3,,
    pub PCSTATE_RCI_INDEX_C6_RESIDENCY): PERF_COUNTER_WRITE_DATA(p->pc6,,
    pub PCSTATE_RCI_INDEX_C7_RESIDENCY): PERF_COUNTER_WRITE_DATA(p->pc7,,
    pub PCSTATE_RCI_INDEX_C8_RESIDENCY): PERF_COUNTER_WRITE_DATA(p->pc8,,
    pub PCSTATE_RCI_INDEX_C9_RESIDENCY): PERF_COUNTER_WRITE_DATA(p->pc9,,
    pub PCSTATE_RCI_INDEX_C10_RESIDENCY): PERF_COUNTER_WRITE_DATA(p->pc10,,

    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn msr_counter_info_count_perf(mci: *const msr_counter_info_t) -> usize {
    size_t msr_counter_info_count_perf(const struct msr_counter_info_t *mci)
    {
    pub 0: size_t ret =,
    pub ++i): for (int i = 0; i < NUM_MSR_COUNTERS;,
    if (mci.source[i] == COUNTER_SOURCE_PERF)
    pub ret: return,
    }
#[no_mangle]
pub unsafe extern "C" fn get_smi_aperf_mperf(cpu: c_uint, t: *mut thread_data) -> c_int {
    int get_smi_aperf_mperf(unsigned int cpu, struct thread_data *t)
    {
    pub 1]: unsigned long long perf_data[NUM_MSR_COUNTERS +,
    pub mci: *mut msr_counter_info_t,
    if (debug >= 2)
    pub cpu): fprintf(stderr, "%s: cpu%d\n", __func__,,
    pub msr_counter_info_size): assert(cpu <=,
    pub &msr_counter_info[cpu]: mci =,
    if (mci.fd_perf != -1) {
    pub msr_counter_info_count_perf(mci): size_t num_perf_counters =,
    pub long): *const *const ssize_t expected_read_size = (num_perf_counters + 1)  sizeof(unsigned long,
    pub sizeof(perf_data)): ssize_t actual_read_size = read(mci->fd_perf, &perf_data[0],,
    if (actual_read_size != expected_read_size)
    pub actual_read_size): err(-1, "%s: failed to read perf_data (%zu %zu)", __func__, expected_read_size,,
    }
    pub {: for (unsigned int i = 0, pi = 1; i < NUM_MSR_COUNTERS; ++i),
    switch (mci.source[i]) {
    case COUNTER_SOURCE_NONE:
    case COUNTER_SOURCE_PERF:
    pub ARRAY_SIZE(perf_data)): assert(pi <,
    pub -1): assert(mci->fd_perf !=,
    if (debug >= 2)
    pub perf_data[pi]): fprintf(stderr, "Reading msr counter via perf at %u: %llu\n", i,,
    pub perf_data: [mci->data[i] =; pi],
    case COUNTER_SOURCE_MSR:
    if (get_msr(cpu, mci.msr[i], &mci.data[i]))
    pub i: return -2 -,
    pub mci->msr_mask[i]: mci->data[i] &=,
    if (debug >= 2)
    pub mci->data[i]): fprintf(stderr, "Reading msr counter via msr at %u: %llu\n", i,,
    }
    }
    pub 3): BUILD_BUG_ON(NUM_MSR_COUNTERS !=,
    pub mci->data[MSR_RCI_INDEX_APERF]: t->aperf =,
    pub mci->data[MSR_RCI_INDEX_MPERF]: t->mperf =,
    pub mci->data[MSR_RCI_INDEX_SMI]: t->smi_count =,
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn perf_counter_info_read_values(pp: *mut perf_counter_info, cpu: c_int, out: *mut c_ulonglong, out_size: usize) -> c_int {
    int perf_counter_info_read_values(struct perf_counter_info *pp, int cpu, unsigned long long *out, size_t out_size)
    {
    pub domain: c_uint,
    pub value: c_ulonglong,
    pub fd_counter: c_int,
    pub {: for (size_t i = 0; pp; ++i, pp = pp->next),
    pub cpu): domain = cpu_to_domain(pp,,
    pub pp->num_domains): assert(domain <,
    pub pp->fd_perf_per_domain[domain]: fd_counter =,
    if (fd_counter == -1)
    if (read(fd_counter, &value, sizeof(value)) != sizeof(value))
    pub 1: return,
    pub out_size): assert(i <,
    pub pp->scale: *mut *mut out[i] = value,
    }
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn pmt_gen_value_mask(lsb: c_uint, msb: c_uint) -> c_ulong {
    unsigned long pmt_gen_value_mask(unsigned int lsb, unsigned int msb)
    {
    pub mask: c_ulong,
    if (msb == 63)
    pub 0xffffffffffffffff: mask =,
    else
    pub 1): mask = ((1 << (msb + 1)) -,
    pub 1: mask -= (1 << lsb) -,
    pub mask: return,
    }
#[no_mangle]
pub unsafe extern "C" fn pmt_read_counter(ppmt: *mut pmt_counter, domain_id: c_uint) -> c_ulong {
    unsigned long pmt_read_counter(struct pmt_counter *ppmt, unsigned int domain_id)
    {
    if (domain_id >= ppmt.num_domains)
    pub 0: return,
    pub ppmt->domains[domain_id].pcounter: *const *const unsigned long pmmio =,
    pub 0: *const *const unsigned long value = pmmio ? pmmio :,
    pub ppmt->msb): unsigned long value_mask = pmt_gen_value_mask(ppmt->lsb,,
    pub ppmt->lsb: unsigned long value_shift =,
    pub value_shift: return (value & value_mask) >>,
    }
// Rapl domain enumeration helpers
#[no_mangle]
pub unsafe extern "C" fn get_rapl_num_domains() -> c_int {
    static inline int get_rapl_num_domains(void)
    {
    if (!platform.has_per_core_rapl)
    pub topo.num_packages: return,
    pub 1: return GLOBAL_CORE_ID(topo.max_core_id, topo.num_packages) +,
    }
#[no_mangle]
pub unsafe extern "C" fn get_rapl_domain_id(cpu: c_int) -> c_int {
    static inline int get_rapl_domain_id(int cpu)
    {
    if (!platform.has_per_core_rapl)
    pub cpus[cpu].package_id: return,
    pub cpus[cpu].package_id): return GLOBAL_CORE_ID(cpus[cpu].core_id,,
    }
//
// get_counters(...)
// migrate to cpu
// acquire and record local counters for that cpu
//
#[no_mangle]
pub unsafe extern "C" fn get_counters(_arg: PER_THREAD_PARAMS) -> c_int {
    int get_counters(PER_THREAD_PARAMS)
    {
    pub t->cpu_id: int cpu =,
    pub msr: c_ulonglong,
    pub mp: *mut msr_counter,
    pub pp: *mut pmt_counter,
    pub i: c_int,
    pub status: c_int,
    if (cpu_migrate(cpu)) {
    pub cpu): fprintf(outf, "%s: Could not migrate to CPU %d\n", __func__,,
    pub -1: return,
    }
    pub )NULL): *mut gettimeofday(&t->tv_begin, (struct timezone,
    if (first_counter_read)
    pub /: *mut *mut t->tsc = rdtsc(); / we are running on local CPU of interest,
    pub t): get_smi_aperf_mperf(cpu,,
    if (DO_BIC(BIC_LLC_MRPS) || DO_BIC(BIC_LLC_HIT))
    pub &t->llc): get_perf_llc_stats(cpu,,
    if (DO_BIC(BIC_L2_MRPS) || DO_BIC(BIC_L2_HIT))
    pub &t->l2): get_perf_l2_stats(cpu,,
    if (DO_BIC(BIC_IPC))
    if (read(get_instr_count_fd(cpu), &t.instr_count, sizeof(long long)) != sizeof(long long))
    pub -4: return,
    if (DO_BIC(BIC_IRQ))
    pub irqs_per_cpu: [t->irq_count =; cpu],
    if (DO_BIC(BIC_NMI))
    pub nmi_per_cpu: [t->nmi_count =; cpu],
    pub p): get_cstate_counters(cpu, t, c,,
    pub {: for (i = 0, mp = sys.tp; mp; i++, mp = mp->next),
    if (get_mp(cpu, mp, &t.counter[i], mp.sp.path))
    pub -10: return,
    }
    if (perf_counter_info_read_values(sys.perf_tp, cpu, t.perf_counter, MAX_ADDED_THREAD_COUNTERS))
    pub -10: return,
    pub pp->next): for (i = 0, pp = sys.pmt_tp; pp; i++, pp =,
    pub t->cpu_id): t->pmt_counter[i] = pmt_read_counter(pp,,
// collect core counters only for 1st thread in core
    if (!is_cpu_first_thread_in_core(t, c))
    pub done: goto,
    if (platform.has_per_core_rapl) {
    pub p): status = get_rapl_counters(cpu, get_rapl_domain_id(cpu), c,,
    if (status != 0)
    pub status: return,
    }
    if (DO_BIC(BIC_CPU_c7) && t.is_atom) {
//
// For Atom CPUs that has core cstate deeper than c6,
// MSR_CORE_C6_RESIDENCY returns residency of cc6 and deeper.
// Minus CC7 (and deeper cstates) residency to get
// accturate cc6 residency.
//
    pub c->c7: c->c6 -=,
    }
    if (DO_BIC(BIC_Mod_c6))
    if (get_msr(cpu, MSR_MODULE_C6_RES_MS, &c.mc6_us))
    pub -8: return,
    if (DO_BIC(BIC_CoreTmp)) {
    if (get_msr(cpu, MSR_IA32_THERM_STATUS, &msr))
    pub -9: return,
    pub 0x7F): c->core_temp_c = tj_max - ((msr >> 16) &,
    }
    if (DO_BIC(BIC_CORE_THROT_CNT))
    pub &c->core_throt_cnt): get_core_throt_cnt(cpu,,
    pub {: for (i = 0, mp = sys.cp; mp; i++, mp = mp->next),
    if (get_mp(cpu, mp, &c.counter[i], mp.sp.path))
    pub -10: return,
    }
    if (perf_counter_info_read_values(sys.perf_cp, cpu, c.perf_counter, MAX_ADDED_CORE_COUNTERS))
    pub -10: return,
    pub pp->next): for (i = 0, pp = sys.pmt_cp; pp; i++, pp =,
    pub cpus[t->cpu_id].core_id): c->pmt_counter[i] = pmt_read_counter(pp,,
// collect package counters only for 1st core in package
    if (!is_cpu_first_core_in_package(t, p))
    pub done: goto,
    if (DO_BIC(BIC_Totl_c0)) {
    if (get_msr(cpu, MSR_PKG_WEIGHTED_CORE_C0_RES, &p.pkg_wtd_core_c0))
    pub -10: return,
    }
    if (DO_BIC(BIC_Any_c0)) {
    if (get_msr(cpu, MSR_PKG_ANY_CORE_C0_RES, &p.pkg_any_core_c0))
    pub -11: return,
    }
    if (DO_BIC(BIC_GFX_c0)) {
    if (get_msr(cpu, MSR_PKG_ANY_GFXE_C0_RES, &p.pkg_any_gfxe_c0))
    pub -12: return,
    }
    if (DO_BIC(BIC_CPUGFX)) {
    if (get_msr(cpu, MSR_PKG_BOTH_CORE_GFXE_C0_RES, &p.pkg_both_core_gfxe_c0))
    pub -13: return,
    }
    if (DO_BIC(BIC_CPU_LPI))
    pub cpuidle_cur_cpu_lpi_us: p->cpu_lpi =,
    if (DO_BIC(BIC_SYS_LPI))
    pub cpuidle_cur_sys_lpi_us: p->sys_lpi =,
    if (!platform.has_per_core_rapl) {
    pub p): status = get_rapl_counters(cpu, get_rapl_domain_id(cpu), c,,
    if (status != 0)
    pub status: return,
    }
    if (DO_BIC(BIC_PkgTmp)) {
    if (get_msr(cpu, MSR_IA32_PACKAGE_THERM_STATUS, &msr))
    pub -17: return,
    pub 0x7F): p->pkg_temp_c = tj_max - ((msr >> 16) &,
    }
    if (DO_BIC(BIC_UNCORE_MHZ))
    pub get_legacy_uncore_mhz(cpus[t->cpu_id].package_id): p->uncore_mhz =,
    if (DO_BIC(BIC_GFX_rc6))
    pub gfx_info[GFX_rc6].val_ull: p->gfx_rc6_ms =,
    if (DO_BIC(BIC_GFXMHz))
    pub gfx_info[GFX_MHz].val: p->gfx_mhz =,
    if (DO_BIC(BIC_GFXACTMHz))
    pub gfx_info[GFX_ACTMHz].val: p->gfx_act_mhz =,
    if (DO_BIC(BIC_SAM_mc6))
    pub gfx_info[SAM_mc6].val_ull: p->sam_mc6_ms =,
    if (DO_BIC(BIC_SAMMHz))
    pub gfx_info[SAM_MHz].val: p->sam_mhz =,
    if (DO_BIC(BIC_SAMACTMHz))
    pub gfx_info[SAM_ACTMHz].val: p->sam_act_mhz =,
    pub {: for (i = 0, mp = sys.pp; mp; i++, mp = mp->next),
    pub NULL: *mut *mut char path =,
    if (mp.msr_num == 0) {
    pub cpus[t->cpu_id].package_id): path = find_sysfs_path_by_id(mp->sp,,
    if (path == core::ptr::null_mut()) {
    pub cpus[t->cpu_id].package_id): warnx("%s: package_id %d not found", __func__,,
    pub -10: return,
    }
    }
    if (get_mp(cpu, mp, &p.counter[i], path))
    pub -10: return,
    }
    if (perf_counter_info_read_values(sys.perf_pp, cpu, p.perf_counter, MAX_ADDED_PACKAGE_COUNTERS))
    pub -10: return,
    pub pp->next): for (i = 0, pp = sys.pmt_pp; pp; i++, pp =,
    pub cpus[t->cpu_id].package_id): p->pmt_counter[i] = pmt_read_counter(pp,,
    done:
    pub )NULL): *mut gettimeofday(&t->tv_end, (struct timezone,
    pub 0: return,
    }
    pub PCLUKN: int pkg_cstate_limit =,
    char *pkg_cstate_limit_strings[] = { "unknown", "reserved", "pc0", "pc1", "pc2",
    "pc3", "pc4", "pc6", "pc6n", "pc6r", "pc7", "pc7s", "pc8", "pc9", "pc10", "unlimited"
}

    int nhm_pkg_cstate_limits[16] = { PCL__0, PCL__1, PCL__3, PCL__6, PCL__7, PCLRSV, PCLRSV, PCLUNL, PCLRSV, PCLRSV, PCLRSV, PCLRSV, PCLRSV, PCLRSV,
    PCLRSV, PCLRSV
    };
    int snb_pkg_cstate_limits[16] = { PCL__0, PCL__2, PCL_6N, PCL_6R, PCL__7, PCL_7S, PCLRSV, PCLUNL, PCLRSV, PCLRSV, PCLRSV, PCLRSV, PCLRSV, PCLRSV,
    PCLRSV, PCLRSV
    };
    int hsw_pkg_cstate_limits[16] = { PCL__0, PCL__2, PCL__3, PCL__6, PCL__7, PCL_7S, PCL__8, PCL__9, PCLUNL, PCLRSV, PCLRSV, PCLRSV, PCLRSV, PCLRSV,
    PCLRSV, PCLRSV
    };
    int slv_pkg_cstate_limits[16] = { PCL__0, PCL__1, PCLRSV, PCLRSV, PCL__4, PCLRSV, PCL__6, PCL__7, PCLRSV, PCLRSV, PCLRSV, PCLRSV, PCLRSV, PCLRSV,
    PCL__6, PCL__7
    };
    int amt_pkg_cstate_limits[16] = { PCLUNL, PCL__1, PCL__2, PCLRSV, PCLRSV, PCLRSV, PCL__6, PCL__7, PCLRSV, PCLRSV, PCLRSV, PCLRSV, PCLRSV, PCLRSV,
    PCLRSV, PCLRSV
    };
    int phi_pkg_cstate_limits[16] = { PCL__0, PCL__2, PCL_6N, PCL_6R, PCLRSV, PCLRSV, PCLRSV, PCLUNL, PCLRSV, PCLRSV, PCLRSV, PCLRSV, PCLRSV, PCLRSV,
    PCLRSV, PCLRSV
    };
    int glm_pkg_cstate_limits[16] = { PCLUNL, PCL__1, PCL__3, PCL__6, PCL__7, PCL_7S, PCL__8, PCL__9, PCL_10, PCLRSV, PCLRSV, PCLRSV, PCLRSV, PCLRSV,
    PCLRSV, PCLRSV
    };
    int skx_pkg_cstate_limits[16] = { PCL__0, PCL__2, PCL_6N, PCL_6R, PCLRSV, PCLRSV, PCLRSV, PCLUNL, PCLRSV, PCLRSV, PCLRSV, PCLRSV, PCLRSV, PCLRSV,
    PCLRSV, PCLRSV
    };
    int icx_pkg_cstate_limits[16] = { PCL__0, PCL__2, PCL__6, PCL__6, PCLRSV, PCLRSV, PCLRSV, PCLUNL, PCLRSV, PCLRSV, PCLRSV, PCLRSV, PCLRSV, PCLRSV,
    PCLRSV, PCLRSV
    };
#[no_mangle]
pub unsafe extern "C" fn probe_cst_limit() {
    void probe_cst_limit(void)
    {
    unsigned long long msr;
    int *pkg_cstate_limits;
    if (!platform.has_nhm_msrs || no_msr)
    return;
    switch (platform.cst_limit) {
    case CST_LIMIT_NHM:
    pkg_cstate_limits = nhm_pkg_cstate_limits;
    break;
    case CST_LIMIT_SNB:
    pkg_cstate_limits = snb_pkg_cstate_limits;
    break;
    case CST_LIMIT_HSW:
    pkg_cstate_limits = hsw_pkg_cstate_limits;
    break;
    case CST_LIMIT_SKX:
    pkg_cstate_limits = skx_pkg_cstate_limits;
    break;
    case CST_LIMIT_ICX:
    pkg_cstate_limits = icx_pkg_cstate_limits;
    break;
    case CST_LIMIT_SLV:
    pkg_cstate_limits = slv_pkg_cstate_limits;
    break;
    case CST_LIMIT_AMT:
    pkg_cstate_limits = amt_pkg_cstate_limits;
    break;
    case CST_LIMIT_KNL:
    pkg_cstate_limits = phi_pkg_cstate_limits;
    break;
    case CST_LIMIT_GMT:
    pkg_cstate_limits = glm_pkg_cstate_limits;
    break;
    default:
    return;
    }
    get_msr(master_cpu, MSR_PKG_CST_CONFIG_CONTROL, &msr);
    pkg_cstate_limit = pkg_cstate_limits[msr & 0xF];
    }
#[no_mangle]
unsafe extern "C" fn dump_platform_info() {
    static void dump_platform_info(void)
    {
    unsigned long long msr;
    unsigned int ratio;
    if (!platform.has_nhm_msrs || no_msr)
    return;
    get_msr(master_cpu, MSR_PLATFORM_INFO, &msr);
    fprintf(outf, "cpu%d: MSR_PLATFORM_INFO: 0x%08llx\n", master_cpu, msr);
    ratio = (msr >> 40) & 0xFF;
    fprintf(outf, "%d * %.1f = %.1f MHz max efficiency frequency\n", ratio, bclk, ratio * bclk);
    ratio = (msr >> 8) & 0xFF;
    fprintf(outf, "%d * %.1f = %.1f MHz base frequency\n", ratio, bclk, ratio * bclk);
    }
#[no_mangle]
unsafe extern "C" fn dump_power_ctl() {
    static void dump_power_ctl(void)
    {
    unsigned long long msr;
    if (!platform.has_nhm_msrs || no_msr)
    return;
    get_msr(master_cpu, MSR_IA32_POWER_CTL, &msr);
    fprintf(outf, "cpu%d: MSR_IA32_POWER_CTL: 0x%08llx (C1E auto-promotion: %sabled)\n", master_cpu, msr, msr & 0x2 ? "EN" : "DIS");
// C-state Pre-wake Disable (CSTATE_PREWAKE_DISABLE)
    if (platform.has_cst_prewake_bit)
    fprintf(outf, "C-state Pre-wake: %sabled\n", msr & 0x40000000 ? "DIS" : "EN");
    return;
    }
#[no_mangle]
unsafe extern "C" fn dump_turbo_ratio_limit2() {
    static void dump_turbo_ratio_limit2(void)
    {
    unsigned long long msr;
    unsigned int ratio;
    get_msr(master_cpu, MSR_TURBO_RATIO_LIMIT2, &msr);
    fprintf(outf, "cpu%d: MSR_TURBO_RATIO_LIMIT2: 0x%08llx\n", master_cpu, msr);
    ratio = (msr >> 8) & 0xFF;
    if (ratio)
    fprintf(outf, "%d * %.1f = %.1f MHz max turbo 18 active cores\n", ratio, bclk, ratio * bclk);
    ratio = (msr >> 0) & 0xFF;
    if (ratio)
    fprintf(outf, "%d * %.1f = %.1f MHz max turbo 17 active cores\n", ratio, bclk, ratio * bclk);
    return;
    }
#[no_mangle]
unsafe extern "C" fn dump_turbo_ratio_limit1() {
    static void dump_turbo_ratio_limit1(void)
    {
    unsigned long long msr;
    unsigned int ratio;
    get_msr(master_cpu, MSR_TURBO_RATIO_LIMIT1, &msr);
    fprintf(outf, "cpu%d: MSR_TURBO_RATIO_LIMIT1: 0x%08llx\n", master_cpu, msr);
    ratio = (msr >> 56) & 0xFF;
    if (ratio)
    fprintf(outf, "%d * %.1f = %.1f MHz max turbo 16 active cores\n", ratio, bclk, ratio * bclk);
    ratio = (msr >> 48) & 0xFF;
    if (ratio)
    fprintf(outf, "%d * %.1f = %.1f MHz max turbo 15 active cores\n", ratio, bclk, ratio * bclk);
    ratio = (msr >> 40) & 0xFF;
    if (ratio)
    fprintf(outf, "%d * %.1f = %.1f MHz max turbo 14 active cores\n", ratio, bclk, ratio * bclk);
    ratio = (msr >> 32) & 0xFF;
    if (ratio)
    fprintf(outf, "%d * %.1f = %.1f MHz max turbo 13 active cores\n", ratio, bclk, ratio * bclk);
    ratio = (msr >> 24) & 0xFF;
    if (ratio)
    fprintf(outf, "%d * %.1f = %.1f MHz max turbo 12 active cores\n", ratio, bclk, ratio * bclk);
    ratio = (msr >> 16) & 0xFF;
    if (ratio)
    fprintf(outf, "%d * %.1f = %.1f MHz max turbo 11 active cores\n", ratio, bclk, ratio * bclk);
    ratio = (msr >> 8) & 0xFF;
    if (ratio)
    fprintf(outf, "%d * %.1f = %.1f MHz max turbo 10 active cores\n", ratio, bclk, ratio * bclk);
    ratio = (msr >> 0) & 0xFF;
    if (ratio)
    fprintf(outf, "%d * %.1f = %.1f MHz max turbo 9 active cores\n", ratio, bclk, ratio * bclk);
    return;
    }
#[no_mangle]
unsafe extern "C" fn dump_turbo_ratio_limits(trl_msr_offset: c_int) {
    static void dump_turbo_ratio_limits(int trl_msr_offset)
    {
    unsigned long long msr, core_counts;
    int shift;
    get_msr(master_cpu, trl_msr_offset, &msr);
    fprintf(outf, "cpu%d: MSR_%sTURBO_RATIO_LIMIT: 0x%08llx\n", master_cpu, trl_msr_offset == MSR_SECONDARY_TURBO_RATIO_LIMIT ? "SECONDARY_" : "", msr);
    if (platform.trl_msrs & TRL_CORECOUNT) {
    get_msr(master_cpu, MSR_TURBO_RATIO_LIMIT1, &core_counts);
    fprintf(outf, "cpu%d: MSR_TURBO_RATIO_LIMIT1: 0x%08llx\n", master_cpu, core_counts);
    } else {
    core_counts = 0x0807060504030201;
    }
    for (shift = 56; shift >= 0; shift -= 8) {
    unsigned int ratio, group_size;
    ratio = (msr >> shift) & 0xFF;
    group_size = (core_counts >> shift) & 0xFF;
    if (ratio)
    fprintf(outf, "%d * %.1f = %.1f MHz max turbo %d active cores\n", ratio, bclk, ratio * bclk, group_size);
    }
    return;
    }
#[no_mangle]
unsafe extern "C" fn dump_atom_turbo_ratio_limits() {
    static void dump_atom_turbo_ratio_limits(void)
    {
    unsigned long long msr;
    unsigned int ratio;
    get_msr(master_cpu, MSR_ATOM_CORE_RATIOS, &msr);
    fprintf(outf, "cpu%d: MSR_ATOM_CORE_RATIOS: 0x%08llx\n", master_cpu, msr & 0xFFFFFFFF);
    ratio = (msr >> 0) & 0x3F;
    if (ratio)
    fprintf(outf, "%d * %.1f = %.1f MHz minimum operating frequency\n", ratio, bclk, ratio * bclk);
    ratio = (msr >> 8) & 0x3F;
    if (ratio)
    fprintf(outf, "%d * %.1f = %.1f MHz low frequency mode (LFM)\n", ratio, bclk, ratio * bclk);
    ratio = (msr >> 16) & 0x3F;
    if (ratio)
    fprintf(outf, "%d * %.1f = %.1f MHz base frequency\n", ratio, bclk, ratio * bclk);
    get_msr(master_cpu, MSR_ATOM_CORE_TURBO_RATIOS, &msr);
    fprintf(outf, "cpu%d: MSR_ATOM_CORE_TURBO_RATIOS: 0x%08llx\n", master_cpu, msr & 0xFFFFFFFF);
    ratio = (msr >> 24) & 0x3F;
    if (ratio)
    fprintf(outf, "%d * %.1f = %.1f MHz max turbo 4 active cores\n", ratio, bclk, ratio * bclk);
    ratio = (msr >> 16) & 0x3F;
    if (ratio)
    fprintf(outf, "%d * %.1f = %.1f MHz max turbo 3 active cores\n", ratio, bclk, ratio * bclk);
    ratio = (msr >> 8) & 0x3F;
    if (ratio)
    fprintf(outf, "%d * %.1f = %.1f MHz max turbo 2 active cores\n", ratio, bclk, ratio * bclk);
    ratio = (msr >> 0) & 0x3F;
    if (ratio)
    fprintf(outf, "%d * %.1f = %.1f MHz max turbo 1 active core\n", ratio, bclk, ratio * bclk);
    }
#[no_mangle]
unsafe extern "C" fn dump_knl_turbo_ratio_limits() {
    static void dump_knl_turbo_ratio_limits(void)
    {
    let mut buckets_no: c_uint = 7;
    unsigned long long msr;
    int delta_cores, delta_ratio;
    int i, b_nr;
    unsigned int cores[buckets_no];
    unsigned int ratio[buckets_no];
    get_msr(master_cpu, MSR_TURBO_RATIO_LIMIT, &msr);
    fprintf(outf, "cpu%d: MSR_TURBO_RATIO_LIMIT: 0x%08llx\n", master_cpu, msr);
//
// Turbo encoding in KNL is as follows:
// [0] -- Reserved
// [7:1] -- Base value of number of active cores of bucket 1.
// [15:8] -- Base value of freq ratio of bucket 1.
// [20:16] -- +ve delta of number of active cores of bucket 2.
// i.e. active cores of bucket 2 =
// active cores of bucket 1 + delta
// [23:21] -- Negative delta of freq ratio of bucket 2.
// i.e. freq ratio of bucket 2 =
// freq ratio of bucket 1 - delta
// [28:24]-- +ve delta of number of active cores of bucket 3.
// [31:29]-- -ve delta of freq ratio of bucket 3.
// [36:32]-- +ve delta of number of active cores of bucket 4.
// [39:37]-- -ve delta of freq ratio of bucket 4.
// [44:40]-- +ve delta of number of active cores of bucket 5.
// [47:45]-- -ve delta of freq ratio of bucket 5.
// [52:48]-- +ve delta of number of active cores of bucket 6.
// [55:53]-- -ve delta of freq ratio of bucket 6.
// [60:56]-- +ve delta of number of active cores of bucket 7.
// [63:61]-- -ve delta of freq ratio of bucket 7.
//
    b_nr = 0;
    cores[b_nr] = (msr & 0xFF) >> 1;
    ratio[b_nr] = (msr >> 8) & 0xFF;
    for (i = 16; i < 64; i += 8) {
    delta_cores = (msr >> i) & 0x1F;
    delta_ratio = (msr >> (i + 5)) & 0x7;
    cores[b_nr + 1] = cores[b_nr] + delta_cores;
    ratio[b_nr + 1] = ratio[b_nr] - delta_ratio;
    b_nr++;
    }
    for (i = buckets_no - 1; i >= 0; i--)
    if (i > 0 ? ratio[i] != ratio[i - 1] : 1)
    fprintf(outf, "%d * %.1f = %.1f MHz max turbo %d active cores\n", ratio[i], bclk, ratio[i] * bclk, cores[i]);
    }
#[no_mangle]
unsafe extern "C" fn dump_cst_cfg() {
    static void dump_cst_cfg(void)
    {
    unsigned long long msr;
    if (!platform.has_nhm_msrs || no_msr)
    return;
    get_msr(master_cpu, MSR_PKG_CST_CONFIG_CONTROL, &msr);
    fprintf(outf, "cpu%d: MSR_PKG_CST_CONFIG_CONTROL: 0x%08llx", master_cpu, msr);
    fprintf(outf, " (%s%s%s%s%slocked, pkg-cstate-limit=%d (%s)",
    (msr & SNB_C3_AUTO_UNDEMOTE) ? "UNdemote-C3, " : "",
    (msr & SNB_C1_AUTO_UNDEMOTE) ? "UNdemote-C1, " : "",
    (msr & NHM_C3_AUTO_DEMOTE) ? "demote-C3, " : "",
    (msr & NHM_C1_AUTO_DEMOTE) ? "demote-C1, " : "",
    (msr & (1 << 15)) ? "" : "UN", (unsigned int)msr & 0xF, pkg_cstate_limit_strings[pkg_cstate_limit]);

    if (platform.has_cst_auto_convension) {
    fprintf(outf, ", automatic c-state conversion=%s", (msr & AUTOMATIC_CSTATE_CONVERSION) ? "on" : "off");
    }
    fprintf(outf, ")\n");
    return;
    }
#[no_mangle]
unsafe extern "C" fn dump_config_tdp() {
    static void dump_config_tdp(void)
    {
    unsigned long long msr;
    get_msr(master_cpu, MSR_CONFIG_TDP_NOMINAL, &msr);
    fprintf(outf, "cpu%d: MSR_CONFIG_TDP_NOMINAL: 0x%08llx", master_cpu, msr);
    fprintf(outf, " (base_ratio=%d)\n", (unsigned int)msr & 0xFF);
    get_msr(master_cpu, MSR_CONFIG_TDP_LEVEL_1, &msr);
    fprintf(outf, "cpu%d: MSR_CONFIG_TDP_LEVEL_1: 0x%08llx (", master_cpu, msr);
    if (msr) {
    fprintf(outf, "PKG_MIN_PWR_LVL1=%d ", (unsigned int)(msr >> 48) & 0x7FFF);
    fprintf(outf, "PKG_MAX_PWR_LVL1=%d ", (unsigned int)(msr >> 32) & 0x7FFF);
    fprintf(outf, "LVL1_RATIO=%d ", (unsigned int)(msr >> 16) & 0xFF);
    fprintf(outf, "PKG_TDP_LVL1=%d", (unsigned int)(msr) & 0x7FFF);
    }
    fprintf(outf, ")\n");
    get_msr(master_cpu, MSR_CONFIG_TDP_LEVEL_2, &msr);
    fprintf(outf, "cpu%d: MSR_CONFIG_TDP_LEVEL_2: 0x%08llx (", master_cpu, msr);
    if (msr) {
    fprintf(outf, "PKG_MIN_PWR_LVL2=%d ", (unsigned int)(msr >> 48) & 0x7FFF);
    fprintf(outf, "PKG_MAX_PWR_LVL2=%d ", (unsigned int)(msr >> 32) & 0x7FFF);
    fprintf(outf, "LVL2_RATIO=%d ", (unsigned int)(msr >> 16) & 0xFF);
    fprintf(outf, "PKG_TDP_LVL2=%d", (unsigned int)(msr) & 0x7FFF);
    }
    fprintf(outf, ")\n");
    get_msr(master_cpu, MSR_CONFIG_TDP_CONTROL, &msr);
    fprintf(outf, "cpu%d: MSR_CONFIG_TDP_CONTROL: 0x%08llx (", master_cpu, msr);
    if ((msr) & 0x3)
    fprintf(outf, "TDP_LEVEL=%d ", (unsigned int)(msr) & 0x3);
    fprintf(outf, " lock=%d", (unsigned int)(msr >> 31) & 1);
    fprintf(outf, ")\n");
    get_msr(master_cpu, MSR_TURBO_ACTIVATION_RATIO, &msr);
    fprintf(outf, "cpu%d: MSR_TURBO_ACTIVATION_RATIO: 0x%08llx (", master_cpu, msr);
    fprintf(outf, "MAX_NON_TURBO_RATIO=%d", (unsigned int)(msr) & 0xFF);
    fprintf(outf, " lock=%d", (unsigned int)(msr >> 31) & 1);
    fprintf(outf, ")\n");
    }
    unsigned int irtl_time_units[] = { 1, 32, 1024, 32768, 1048576, 33554432, 0, 0 };
#[no_mangle]
pub unsafe extern "C" fn print_irtl() {
    void print_irtl(void)
    {
    unsigned long long msr;
    if (!platform.has_irtl_msrs || no_msr)
    return;
    if (platform.supported_cstates & PC3) {
    get_msr(master_cpu, MSR_PKGC3_IRTL, &msr);
    fprintf(outf, "cpu%d: MSR_PKGC3_IRTL: 0x%08llx (", master_cpu, msr);
    fprintf(outf, "%svalid, %lld ns)\n", msr & (1 << 15) ? "" : "NOT", (msr & 0x3FF) * irtl_time_units[(msr >> 10) & 0x3]);
    }
    if (platform.supported_cstates & PC6) {
    get_msr(master_cpu, MSR_PKGC6_IRTL, &msr);
    fprintf(outf, "cpu%d: MSR_PKGC6_IRTL: 0x%08llx (", master_cpu, msr);
    fprintf(outf, "%svalid, %lld ns)\n", msr & (1 << 15) ? "" : "NOT", (msr & 0x3FF) * irtl_time_units[(msr >> 10) & 0x3]);
    }
    if (platform.supported_cstates & PC7) {
    get_msr(master_cpu, MSR_PKGC7_IRTL, &msr);
    fprintf(outf, "cpu%d: MSR_PKGC7_IRTL: 0x%08llx (", master_cpu, msr);
    fprintf(outf, "%svalid, %lld ns)\n", msr & (1 << 15) ? "" : "NOT", (msr & 0x3FF) * irtl_time_units[(msr >> 10) & 0x3]);
    }
    if (platform.supported_cstates & PC8) {
    get_msr(master_cpu, MSR_PKGC8_IRTL, &msr);
    fprintf(outf, "cpu%d: MSR_PKGC8_IRTL: 0x%08llx (", master_cpu, msr);
    fprintf(outf, "%svalid, %lld ns)\n", msr & (1 << 15) ? "" : "NOT", (msr & 0x3FF) * irtl_time_units[(msr >> 10) & 0x3]);
    }
    if (platform.supported_cstates & PC9) {
    get_msr(master_cpu, MSR_PKGC9_IRTL, &msr);
    fprintf(outf, "cpu%d: MSR_PKGC9_IRTL: 0x%08llx (", master_cpu, msr);
    fprintf(outf, "%svalid, %lld ns)\n", msr & (1 << 15) ? "" : "NOT", (msr & 0x3FF) * irtl_time_units[(msr >> 10) & 0x3]);
    }
    if (platform.supported_cstates & PC10) {
    get_msr(master_cpu, MSR_PKGC10_IRTL, &msr);
    fprintf(outf, "cpu%d: MSR_PKGC10_IRTL: 0x%08llx (", master_cpu, msr);
    fprintf(outf, "%svalid, %lld ns)\n", msr & (1 << 15) ? "" : "NOT", (msr & 0x3FF) * irtl_time_units[(msr >> 10) & 0x3]);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn free_fd_percpu() {
    void free_fd_percpu(void)
    {
    int i;
    if (!fd_percpu)
    return;
    for (i = 0; i < topo.max_cpu_num + 1; ++i) {
    if (fd_percpu[i] != 0)
    close(fd_percpu[i]);
    }
    free(fd_percpu);
    fd_percpu = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn free_fd_instr_count_percpu() {
    void free_fd_instr_count_percpu(void)
    {
    if (!fd_instr_count_percpu)
    return;
    for (int i = 0; i < topo.max_cpu_num + 1; ++i) {
    if (fd_instr_count_percpu[i] != 0)
    close(fd_instr_count_percpu[i]);
    }
    free(fd_instr_count_percpu);
    fd_instr_count_percpu = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn free_fd_llc_percpu() {
    void free_fd_llc_percpu(void)
    {
    if (!fd_llc_percpu)
    return;
    for (int i = 0; i < topo.max_cpu_num + 1; ++i) {
    if (fd_llc_percpu[i] != 0)
    close(fd_llc_percpu[i]);
    }
    free(fd_llc_percpu);
    fd_llc_percpu = core::ptr::null_mut();
    BIC_NOT_PRESENT(BIC_LLC_MRPS);
    BIC_NOT_PRESENT(BIC_LLC_HIT);
    }
#[no_mangle]
pub unsafe extern "C" fn free_fd_l2_percpu() {
    void free_fd_l2_percpu(void)
    {
    if (!fd_l2_percpu)
    return;
    for (int i = 0; i < topo.max_cpu_num + 1; ++i) {
    if (fd_l2_percpu[i] != 0)
    close(fd_l2_percpu[i]);
    }
    free(fd_l2_percpu);
    fd_l2_percpu = core::ptr::null_mut();
    BIC_NOT_PRESENT(BIC_L2_MRPS);
    BIC_NOT_PRESENT(BIC_L2_HIT);
    }
#[no_mangle]
pub unsafe extern "C" fn free_fd_cstate() {
    void free_fd_cstate(void)
    {
    if (!ccstate_counter_info)
    return;
    let mut counter_info_num: c_int = ccstate_counter_info_size;
    for (int counter_id = 0; counter_id < counter_info_num; ++counter_id) {
    if (ccstate_counter_info[counter_id].fd_perf_core != -1)
    close(ccstate_counter_info[counter_id].fd_perf_core);
    if (ccstate_counter_info[counter_id].fd_perf_pkg != -1)
    close(ccstate_counter_info[counter_id].fd_perf_pkg);
    }
    free(ccstate_counter_info);
    ccstate_counter_info = core::ptr::null_mut();
    ccstate_counter_info_size = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn free_fd_msr() {
    void free_fd_msr(void)
    {
    if (!msr_counter_info)
    return;
    for (int cpu = 0; cpu < topo.max_cpu_num; ++cpu) {
    if (msr_counter_info[cpu].fd_perf != -1)
    close(msr_counter_info[cpu].fd_perf);
    }
    free(msr_counter_info);
    msr_counter_info = core::ptr::null_mut();
    msr_counter_info_size = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn free_fd_rapl_percpu() {
    void free_fd_rapl_percpu(void)
    {
    if (!rapl_counter_info_perdomain)
    return;
    let mut num_domains: c_int = rapl_counter_info_perdomain_size;
    for (int domain_id = 0; domain_id < num_domains; ++domain_id) {
    if (rapl_counter_info_perdomain[domain_id].fd_perf != -1)
    close(rapl_counter_info_perdomain[domain_id].fd_perf);
    }
    free(rapl_counter_info_perdomain);
    rapl_counter_info_perdomain = core::ptr::null_mut();
    rapl_counter_info_perdomain_size = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn free_fd_added_perf_counters_(pp: *mut perf_counter_info) {
    void free_fd_added_perf_counters_(struct perf_counter_info *pp)
    {
    if (!pp)
    return;
    if (!pp.fd_perf_per_domain)
    return;
    while (pp) {
    for (size_t domain = 0; domain < pp.num_domains; ++domain) {
    if (pp.fd_perf_per_domain[domain] != -1) {
    close(pp.fd_perf_per_domain[domain]);
    pp.fd_perf_per_domain[domain] = -1;
    }
    }
    free(pp.fd_perf_per_domain);
    pp.fd_perf_per_domain = core::ptr::null_mut();
    pp = pp.next;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn free_fd_added_perf_counters() {
    void free_fd_added_perf_counters(void)
    {
    free_fd_added_perf_counters_(sys.perf_tp);
    free_fd_added_perf_counters_(sys.perf_cp);
    free_fd_added_perf_counters_(sys.perf_pp);
    }
#[no_mangle]
pub unsafe extern "C" fn free_all_buffers() {
    void free_all_buffers(void)
    {
    int i;
    CPU_FREE(cpu_present_set);
    cpu_present_set = core::ptr::null_mut();
    cpu_present_setsize = 0;
    CPU_FREE(cpu_effective_set);
    cpu_effective_set = core::ptr::null_mut();
    cpu_effective_setsize = 0;
    CPU_FREE(cpu_allowed_set);
    cpu_allowed_set = core::ptr::null_mut();
    cpu_allowed_setsize = 0;
    CPU_FREE(cpu_affinity_set);
    cpu_affinity_set = core::ptr::null_mut();
    cpu_affinity_setsize = 0;
    if (perf_pcore_set) {
    CPU_FREE(perf_pcore_set);
    perf_pcore_set = core::ptr::null_mut();
    }
    if (perf_ecore_set) {
    CPU_FREE(perf_ecore_set);
    perf_ecore_set = core::ptr::null_mut();
    }
    if (perf_lcore_set) {
    CPU_FREE(perf_lcore_set);
    perf_lcore_set = core::ptr::null_mut();
    }
    free(even.threads);
    free(even.cores);
    free(even.packages);
    even.threads = core::ptr::null_mut();
    even.cores = core::ptr::null_mut();
    even.packages = core::ptr::null_mut();
    free(odd.threads);
    free(odd.cores);
    free(odd.packages);
    odd.threads = core::ptr::null_mut();
    odd.cores = core::ptr::null_mut();
    odd.packages = core::ptr::null_mut();
    free(output_buffer);
    output_buffer = core::ptr::null_mut();
    outp = core::ptr::null_mut();
    free_fd_percpu();
    free_fd_instr_count_percpu();
    free_fd_llc_percpu();
    free_fd_l2_percpu();
    free_fd_msr();
    free_fd_rapl_percpu();
    free_fd_cstate();
    free_fd_added_perf_counters();
    free(irq_column_2_cpu);
    free(irqs_per_cpu);
    free(nmi_per_cpu);
    for (i = 0; i <= topo.max_cpu_num; ++i) {
    if (cpus[i].put_ids)
    CPU_FREE(cpus[i].put_ids);
    }
    free(cpus);
    }
//
// Parse a file containing a single int.
// Return 0 if file can not be opened
// Exit if file can be opened, but can not be parsed
//
#[no_mangle]
pub unsafe extern "C" fn parse_int_file(fmt: *const c_char, ...) -> c_int {
    int parse_int_file(const char *fmt, ...)
    {
    va_list args;
    char path[PATH_MAX];
    FILE *filep;
    int value;
    va_start(args, fmt);
    vsnprintf(path, sizeof(path), fmt, args);
    va_end(args);
    filep = fopen(path, "r");
    if (!filep)
    return 0;
    if (fscanf(filep, "%d", &value) != 1)
    err(1, "%s: failed to parse number from file", path);
    fclose(filep);
    return value;
    }
//
// cpu_is_first_core_in_package(cpu)
// return 1 if given CPU is 1st core in package
//
#[no_mangle]
pub unsafe extern "C" fn cpu_is_first_core_in_package(cpu: c_int) -> c_int {
    int cpu_is_first_core_in_package(int cpu)
    {
    let mut cpu: return = = parse_int_file("/sys/devices/system/cpu/cpu%d/topology/core_siblings_list", cpu);
    }
#[no_mangle]
pub unsafe extern "C" fn get_package_id(cpu: c_int) -> c_int {
    int get_package_id(int cpu)
    {
    return parse_int_file("/sys/devices/system/cpu/cpu%d/topology/physical_package_id", cpu);
    }
#[no_mangle]
pub unsafe extern "C" fn get_die_id(cpu: c_int) -> c_int {
    int get_die_id(int cpu)
    {
    return parse_int_file("/sys/devices/system/cpu/cpu%d/topology/die_id", cpu);
    }
#[no_mangle]
pub unsafe extern "C" fn get_l3_id(cpu: c_int) -> c_int {
    int get_l3_id(int cpu)
    {
    return parse_int_file("/sys/devices/system/cpu/cpu%d/cache/index3/id", cpu);
    }
#[no_mangle]
pub unsafe extern "C" fn get_module_id(cpu: c_int) -> c_int {
    int get_module_id(int cpu)
    {
    return parse_int_file("/sys/devices/system/cpu/cpu%d/topology/cluster_id", cpu);
    }
#[no_mangle]
pub unsafe extern "C" fn get_core_id(cpu: c_int) -> c_int {
    int get_core_id(int cpu)
    {
    return parse_int_file("/sys/devices/system/cpu/cpu%d/topology/core_id", cpu);
    }
#[no_mangle]
pub unsafe extern "C" fn set_node_data() {
    void set_node_data(void)
    {
    int pkg, node, lnode, cpu, cpux;
    int cpu_count;
// initialize logical_node_id
    for (cpu = 0; cpu <= topo.max_cpu_num; ++cpu)
    cpus[cpu].logical_node_id = -1;
    cpu_count = 0;
    for (pkg = 0; pkg < topo.num_packages; pkg++) {
    lnode = 0;
    for (cpu = 0; cpu <= topo.max_cpu_num; ++cpu) {
    if (cpus[cpu].package_id != pkg)
    continue;
// find a cpu with an unset logical_node_id
    if (cpus[cpu].logical_node_id != -1)
    continue;
    cpus[cpu].logical_node_id = lnode;
    node = cpus[cpu].physical_node_id;
    cpu_count++;
//
// find all matching cpus on this pkg and set
// the logical_node_id
//
    for (cpux = cpu; cpux <= topo.max_cpu_num; cpux++) {
    if ((cpus[cpux].package_id == pkg) && (cpus[cpux].physical_node_id == node)) {
    cpus[cpux].logical_node_id = lnode;
    cpu_count++;
    }
    }
    lnode++;
    if (lnode > topo.nodes_per_pkg)
    topo.nodes_per_pkg = lnode;
    }
    if (cpu_count >= topo.max_cpu_num)
    break;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn get_physical_node_id(thiscpu: *mut cpu_topology) -> c_int {
    int get_physical_node_id(struct cpu_topology *thiscpu)
    {
    char path[80];
    FILE *filep;
    int i;
    let mut cpu: c_int = thiscpu.cpu_id;
    for (i = 0; i <= topo.max_cpu_num; i++) {
    sprintf(path, "/sys/devices/system/cpu/cpu%d/node%i/cpulist", cpu, i);
    filep = fopen(path, "r");
    if (!filep)
    continue;
    fclose(filep);
    return i;
    }
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn parse_cpu_str(cpu_str: *mut c_char, cpu_set: *mut cpu_set_t, cpu_set_size: c_int) -> c_int {
    static int parse_cpu_str(char *cpu_str, cpu_set_t *cpu_set, int cpu_set_size)
    {
    unsigned int start, end;
    char *next = cpu_str;
    while (next && *next) {
    if (*next == '-')	/* no negative cpu numbers */
    return 1;
    if (*next == '\0' || *next == '\n')
    break;
    start = strtoul(next, &next, 10);
    if (start >= CPU_SUBSET_MAXCPUS)
    return 1;
    CPU_SET_S(start, cpu_set_size, cpu_set);
    if (*next == '\0' || *next == '\n')
    break;
    if (*next == ',') {
    next += 1;
    continue;
    }
    if (*next == '-') {
    next += 1;	/* start range */
    } else if (*next == '.') {
    next += 1;
    if (*next == '.')
    next += 1;	/* start range */
    else
    return 1;
    }
    end = strtoul(next, &next, 10);
    if (end <= start)
    return 1;
    while (++start <= end) {
    if (start >= CPU_SUBSET_MAXCPUS)
    return 1;
    CPU_SET_S(start, cpu_set_size, cpu_set);
    }
    if (*next == ',')
    next += 1;
#[no_mangle]
pub unsafe extern "C" fn if('\n': *mut *mut *mut next != '\0' && next !=) -> else {
    else if (*next != '\0' && *next != '\n')
    return 1;
    }
    return 0;
    }
//
// run func(thread, core, package) in topology order
// skip non-present cpus
//
    int for_all_cpus_2(int (func) (struct thread_data *, struct core_data *,
    struct pkg_data *, struct thread_data *, struct core_data *,
    struct pkg_data *), struct thread_data *thread_base,
    struct core_data *core_base, struct pkg_data *pkg_base,
    struct thread_data *thread_base2, struct core_data *core_base2, struct pkg_data *pkg_base2)
    {
    int cpu, retval;
    retval = 0;
    for (cpu = 0; cpu <= topo.max_cpu_num; ++cpu) {
    struct thread_data *t, *t2;
    struct core_data *c, *c2;
    struct pkg_data *p, *p2;
    if (cpu_is_not_allowed(cpu))
    continue;
    if (has_allowed_lower_ht_sibling(cpu))	/* skip HT sibling */
    continue;
    t = &thread_base[cpu];
    t2 = &thread_base2[cpu];
    c = &core_base[GLOBAL_CORE_ID(cpus[cpu].core_id, cpus[cpu].package_id)];
    c2 = &core_base2[GLOBAL_CORE_ID(cpus[cpu].core_id, cpus[cpu].package_id)];
    p = &pkg_base[cpus[cpu].package_id];
    p2 = &pkg_base2[cpus[cpu].package_id];
    retval |= func(t, c, p, t2, c2, p2);
// Handle HT sibling now
    int i;
    for (i = 0; i <= MAX_HT_ID; ++i) {
    let mut sibling_cpu_id: c_int = cpus[cpu].ht_sibling_cpu_id[i];
    if (sibling_cpu_id < 0)
    break;
    if (sibling_cpu_id == cpu)
    continue;
    if (cpu_is_not_allowed(sibling_cpu_id))
    continue;
    t = &thread_base[sibling_cpu_id];
    t2 = &thread_base2[sibling_cpu_id];
    retval |= func(t, c, p, t2, c2, p2);
    }
    }
    return retval;
    }
//
// run func(cpu) on every cpu in /proc/stat
// return max_cpu number
//
#[no_mangle]
pub unsafe extern "C" fn for_all_proc_cpus((int): int (func)) -> c_int {
    int for_all_proc_cpus(int (func) (int))
    {
    FILE *fp;
    int cpu_num;
    int retval;
    fp = fopen_or_die(proc_stat, "r");
    retval = fscanf(fp, "cpu %*d %*d %*d %*d %*d %*d %*d %*d %*d %*d\n");
    if (retval != 0)
    err(1, "%s: failed to parse format", proc_stat);
    while (1) {
    retval = fscanf(fp, "cpu%u %*d %*d %*d %*d %*d %*d %*d %*d %*d %*d\n", &cpu_num);
    if (retval != 1)
    break;
    retval = func(cpu_num);
    if (retval) {
    fclose(fp);
    return (retval);
    }
    }
    fclose(fp);
    return 0;
    }

    static char cpu_effective_str[1024];
#[no_mangle]
unsafe extern "C" fn update_effective_str(startup: bool) -> c_int {
    static int update_effective_str(bool startup)
    {
    FILE *fp;
    char *pos;
    char buf[1024];
    int ret;
    if (cpu_effective_str[0] == '\0' && !startup)
    return 0;
    fp = fopen(PATH_EFFECTIVE_CPUS, "r");
    if (!fp)
    return 0;
    pos = fgets(buf, 1024, fp);
    if (!pos)
    err(1, "%s: file read failed", PATH_EFFECTIVE_CPUS);
    fclose(fp);
    ret = strncmp(cpu_effective_str, buf, 1024);
    if (!ret)
    return 0;
    strncpy(cpu_effective_str, buf, 1024);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn update_effective_set(startup: bool) {
    static void update_effective_set(bool startup)
    {
    update_effective_str(startup);
    if (parse_cpu_str(cpu_effective_str, cpu_effective_set, cpu_effective_setsize))
    err(1, "%s: cpu str malformat %s", PATH_EFFECTIVE_CPUS, cpu_effective_str);
    }
    void linux_perf_init(void);
    void msr_perf_init(void);
    void rapl_perf_init(void);
    void cstate_perf_init(void);
    void perf_llc_init(void);
    void perf_l2_init(void);
    void added_perf_counters_init(void);
    void pmt_init(void);
#[no_mangle]
pub unsafe extern "C" fn re_initialize() {
    void re_initialize(void)
    {
    free_all_buffers();
    setup_all_buffers(false);
    linux_perf_init();
    msr_perf_init();
    rapl_perf_init();
    cstate_perf_init();
    perf_llc_init();
    perf_l2_init();
    added_perf_counters_init();
    pmt_init();
    fprintf(outf, "turbostat: re-initialized with num_cpus %d, allowed_cpus %d\n", topo.num_cpus, topo.allowed_cpus);
    }
#[no_mangle]
pub unsafe extern "C" fn set_max_cpu_num() {
    void set_max_cpu_num(void)
    {
    FILE *filep;
    int current_cpu;
    unsigned long dummy;
    char pathname[64];
    current_cpu = sched_getcpu();
    if (current_cpu < 0)
    err(1, "cannot find calling cpu ID");
    sprintf(pathname, "/sys/devices/system/cpu/cpu%d/topology/thread_siblings", current_cpu);
    filep = fopen_or_die(pathname, "r");
    topo.max_cpu_num = 0;
    while (fscanf(filep, "%lx,", &dummy) == 1)
    topo.max_cpu_num += BITMASK_SIZE;
    fclose(filep);
    topo.max_cpu_num--;	/* 0 based */
    }
//
// count_cpus()
// remember the last one seen, it will be the max
//
#[no_mangle]
pub unsafe extern "C" fn count_cpus(cpu: c_int) -> c_int {
    int count_cpus(int cpu)
    {
    UNUSED(cpu);
    topo.num_cpus++;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mark_cpu_present(cpu: c_int) -> c_int {
    int mark_cpu_present(int cpu)
    {
    CPU_SET_S(cpu, cpu_present_setsize, cpu_present_set);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn clear_ht_id(cpu: c_int) -> c_int {
    int clear_ht_id(int cpu)
    {
    int i;
    cpus[cpu].ht_id = -1;
    for (i = 0; i <= MAX_HT_ID; ++i)
    cpus[cpu].ht_sibling_cpu_id[i] = -1;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn set_my_cpu_type() -> c_int {
    int set_my_cpu_type(void)
    {
    unsigned int eax, ebx, ecx, edx;
    unsigned int max_level;
    __cpuid(0, max_level, ebx, ecx, edx);
    if (max_level < CPUID_LEAF_MODEL_ID)
    return 0;
    __cpuid(CPUID_LEAF_MODEL_ID, eax, ebx, ecx, edx);
    return (eax >> CPUID_LEAF_MODEL_ID_CORE_TYPE_SHIFT);
    }
#[no_mangle]
pub unsafe extern "C" fn set_cpu_hybrid_type(cpu: c_int) -> c_int {
    int set_cpu_hybrid_type(int cpu)
    {
    if (cpu_migrate(cpu))
    return -1;
    let mut type: c_int = set_my_cpu_type();
    cpus[cpu].type = type;
    return 0;
    }
//
// snapshot_proc_interrupts()
//
// read and record summary of /proc/interrupts
//
// return 1 if config change requires a restart, else return 0
//
#[no_mangle]
pub unsafe extern "C" fn snapshot_proc_interrupts() -> c_int {
    int snapshot_proc_interrupts(void)
    {
    static FILE *fp;
    int column, retval;
    if (fp == core::ptr::null_mut())
    fp = fopen_or_die("/proc/interrupts", "r");
    else
    rewind(fp);
// read 1st line of /proc/interrupts to get cpu* name for each column
    for (column = 0; column < topo.num_cpus; ++column) {
    int cpu_number;
    retval = fscanf(fp, " CPU%d", &cpu_number);
    if (retval != 1)
    break;
    if (cpu_number > topo.max_cpu_num) {
    warn("/proc/interrupts: cpu%d: > %d", cpu_number, topo.max_cpu_num);
    return 1;
    }
    irq_column_2_cpu[column] = cpu_number;
    irqs_per_cpu[cpu_number] = 0;
    nmi_per_cpu[cpu_number] = 0;
    }
// read /proc/interrupt count lines and sum up irqs per cpu
    while (1) {
    int column;
    char buf[64];
    let mut this_row_is_nmi: c_int = 0;
    retval = fscanf(fp, " %s:", buf);	/* irq# "N:" */
    if (retval != 1)
    break;
    if (strncmp(buf, "NMI", strlen("NMI")) == 0)
    this_row_is_nmi = 1;
// read the count per cpu
    for (column = 0; column < topo.num_cpus; ++column) {
    int cpu_number, irq_count;
    retval = fscanf(fp, " %d", &irq_count);
    if (retval != 1)
    break;
    cpu_number = irq_column_2_cpu[column];
    irqs_per_cpu[cpu_number] += irq_count;
    if (this_row_is_nmi)
    nmi_per_cpu[cpu_number] += irq_count;
    }
    while (getc(fp) != '\n') ;	/* flush interrupt description */
    }
    return 0;
    }
//
// snapshot_graphics()
//
// record snapshot of specified graphics sysfs knob
//
// return 1 if config change requires a restart, else return 0
//
#[no_mangle]
pub unsafe extern "C" fn snapshot_graphics(idx: c_int) -> c_int {
    int snapshot_graphics(int idx)
    {
    int retval;
    rewind(gfx_info[idx].fp);
    fflush(gfx_info[idx].fp);
    switch (idx) {
    case GFX_rc6:
    case SAM_mc6:
    retval = fscanf(gfx_info[idx].fp, "%lld", &gfx_info[idx].val_ull);
    if (retval != 1)
    err(1, "rc6");
    return 0;
    case GFX_MHz:
    case GFX_ACTMHz:
    case SAM_MHz:
    case SAM_ACTMHz:
    retval = fscanf(gfx_info[idx].fp, "%d", &gfx_info[idx].val);
    if (retval != 1)
    err(1, "MHz");
    return 0;
    default:
    return -EINVAL;
    }
    }
//
// snapshot_cpu_lpi()
//
// record snapshot of
// /sys/devices/system/cpu/cpuidle/low_power_idle_cpu_residency_us
//
#[no_mangle]
pub unsafe extern "C" fn snapshot_cpu_lpi_us() -> c_int {
    int snapshot_cpu_lpi_us(void)
    {
    FILE *fp;
    int retval;
    fp = fopen_or_die("/sys/devices/system/cpu/cpuidle/low_power_idle_cpu_residency_us", "r");
    retval = fscanf(fp, "%lld", &cpuidle_cur_cpu_lpi_us);
    if (retval != 1) {
    fprintf(stderr, "Disabling Low Power Idle CPU output\n");
    BIC_NOT_PRESENT(BIC_CPU_LPI);
    fclose(fp);
    return -1;
    }
    fclose(fp);
    return 0;
    }
//
// snapshot_sys_lpi()
//
// record snapshot of sys_lpi_file
//
#[no_mangle]
pub unsafe extern "C" fn snapshot_sys_lpi_us() -> c_int {
    int snapshot_sys_lpi_us(void)
    {
    FILE *fp;
    int retval;
    fp = fopen_or_die(sys_lpi_file, "r");
    retval = fscanf(fp, "%lld", &cpuidle_cur_sys_lpi_us);
    if (retval != 1) {
    fprintf(stderr, "Disabling Low Power Idle System output\n");
    BIC_NOT_PRESENT(BIC_SYS_LPI);
    fclose(fp);
    return -1;
    }
    fclose(fp);
    return 0;
    }
//
// snapshot /proc and /sys files
//
// return 1 if configuration restart needed, else return 0
//
#[no_mangle]
pub unsafe extern "C" fn snapshot_proc_sysfs_files() -> c_int {
    int snapshot_proc_sysfs_files(void)
    {
    gettimeofday(&procsysfs_tv_begin, (struct timezone *)core::ptr::null_mut());
    if (DO_BIC(BIC_IRQ) || DO_BIC(BIC_NMI))
    if (snapshot_proc_interrupts())
    return 1;
    if (DO_BIC(BIC_GFX_rc6))
    snapshot_graphics(GFX_rc6);
    if (DO_BIC(BIC_GFXMHz))
    snapshot_graphics(GFX_MHz);
    if (DO_BIC(BIC_GFXACTMHz))
    snapshot_graphics(GFX_ACTMHz);
    if (DO_BIC(BIC_SAM_mc6))
    snapshot_graphics(SAM_mc6);
    if (DO_BIC(BIC_SAMMHz))
    snapshot_graphics(SAM_MHz);
    if (DO_BIC(BIC_SAMACTMHz))
    snapshot_graphics(SAM_ACTMHz);
    if (DO_BIC(BIC_CPU_LPI))
    snapshot_cpu_lpi_us();
    if (DO_BIC(BIC_SYS_LPI))
    snapshot_sys_lpi_us();
    return 0;
    }
    int exit_requested;
#[no_mangle]
unsafe extern "C" fn signal_handler(signal: c_int) {
    static void signal_handler(int signal)
    {
    switch (signal) {
    case SIGINT:
    exit_requested = 1;
    if (debug)
    fprintf(stderr, " SIGINT\n");
    break;
    case SIGUSR1:
    if (debug > 1)
    fprintf(stderr, "SIGUSR1\n");
    break;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn setup_signal_handler() {
    void setup_signal_handler(void)
    {
    struct sigaction sa;
    memset(&sa, 0, sizeof(sa));
    sa.sa_handler = &signal_handler;
    if (sigaction(SIGINT, &sa, core::ptr::null_mut()) < 0)
    err(1, "sigaction SIGINT");
    if (sigaction(SIGUSR1, &sa, core::ptr::null_mut()) < 0)
    err(1, "sigaction SIGUSR1");
    }
#[no_mangle]
pub unsafe extern "C" fn do_sleep() {
    void do_sleep(void)
    {
    struct timeval tout;
    struct timespec rest;
    fd_set readfds;
    int retval;
    FD_ZERO(&readfds);
    FD_SET(0, &readfds);
    if (ignore_stdin) {
    nanosleep(&interval_ts, core::ptr::null_mut());
    return;
    }
    tout = interval_tv;
    retval = select(1, &readfds, core::ptr::null_mut(), core::ptr::null_mut(), &tout);
    if (retval == 1) {
    switch (getc(stdin)) {
    case 'q':
    exit_requested = 1;
    break;
    case EOF:
//
// 'stdin' is a pipe closed on the other end. There
// won't be any further input.
//
    ignore_stdin = 1;
// Sleep the rest of the time
    rest.tv_sec = (tout.tv_sec + tout.tv_usec / 1000000);
    rest.tv_nsec = (tout.tv_usec % 1000000) * 1000;
    nanosleep(&rest, core::ptr::null_mut());
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn get_msr_sum(cpu: c_int, offset: off_t, msr: *mut c_ulonglong) -> c_int {
    int get_msr_sum(int cpu, off_t offset, unsigned long long *msr)
    {
    int ret, idx;
    unsigned long long msr_cur, msr_last;
    assert(!no_msr);
    if (!per_cpu_msr_sum)
    return 1;
    idx = offset_to_idx(offset);
    if (idx < 0)
    return idx;
// get_msr_sum() = sum + (get_msr() - last)
    ret = get_msr(cpu, offset, &msr_cur);
    if (ret)
    return ret;
    msr_last = per_cpu_msr_sum[cpu].entries[idx].last;
    DELTA_WRAP32(msr_cur, msr_last);
// msr = msr_last + per_cpu_msr_sum[cpu].entries[idx].sum;
    return 0;
    }
    timer_t timerid;
// Timer callback, update the sum of MSRs periodically.
#[no_mangle]
unsafe extern "C" fn update_msr_sum(_arg: PER_THREAD_PARAMS) -> c_int {
    static int update_msr_sum(PER_THREAD_PARAMS)
    {
    int i, ret;
    let mut cpu: c_int = t.cpu_id;
    UNUSED(c);
    UNUSED(p);
    assert(!no_msr);
    for (i = IDX_PKG_ENERGY; i < IDX_COUNT; i++) {
    unsigned long long msr_cur, msr_last;
    off_t offset;
    if (!idx_valid(i))
    continue;
    offset = idx_to_offset(i);
    if (offset < 0)
    continue;
    ret = get_msr(cpu, offset, &msr_cur);
    if (ret) {
    fprintf(outf, "Can not update msr(0x%llx)\n", (unsigned long long)offset);
    continue;
    }
    msr_last = per_cpu_msr_sum[cpu].entries[i].last;
    per_cpu_msr_sum[cpu].entries[i].last = msr_cur & 0xffffffff;
    DELTA_WRAP32(msr_cur, msr_last);
    per_cpu_msr_sum[cpu].entries[i].sum += msr_last;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn msr_record_handler(v: union sigval) {
    static void msr_record_handler(union sigval v)
    {
    UNUSED(v);
    for_all_cpus(update_msr_sum, EVEN_COUNTERS);
    }
#[no_mangle]
pub unsafe extern "C" fn msr_sum_record() {
    void msr_sum_record(void)
    {
    struct itimerspec its;
    struct sigevent sev;
    per_cpu_msr_sum = calloc(topo.max_cpu_num + 1, sizeof(struct msr_sum_array));
    if (!per_cpu_msr_sum) {
    fprintf(outf, "Can not allocate memory for long time MSR.\n");
    return;
    }
//
// Signal handler might be restricted, so use thread notifier instead.
//
    memset(&sev, 0, sizeof(struct sigevent));
    sev.sigev_notify = SIGEV_THREAD;
    sev.sigev_notify_function = msr_record_handler;
    sev.sigev_value.sival_ptr = &timerid;
    if (timer_create(CLOCK_REALTIME, &sev, &timerid) == -1) {
    fprintf(outf, "Can not create timer.\n");
    goto release_msr;
    }
    its.it_value.tv_sec = 0;
    its.it_value.tv_nsec = 1;
//
// A wraparound time has been calculated early.
// Some sources state that the peak power for a
// microprocessor is usually 1.5 times the TDP rating,
// use 2 * TDP for safety.
//
    its.it_interval.tv_sec = rapl_joule_counter_range / 2;
    its.it_interval.tv_nsec = 0;
    if (timer_settime(timerid, 0, &its, core::ptr::null_mut()) == -1) {
    fprintf(outf, "Can not set timer.\n");
    goto release_timer;
    }
    return;
    release_timer:
    timer_delete(timerid);
    release_msr:
    free(per_cpu_msr_sum);
    per_cpu_msr_sum = core::ptr::null_mut();
    }
//
// set_my_sched_priority(pri)
// return previous priority on success
// return value < -20 on failure
//
#[no_mangle]
pub unsafe extern "C" fn set_my_sched_priority(priority: c_int) -> c_int {
    int set_my_sched_priority(int priority)
    {
    int retval;
    int original_priority;
    errno = 0;
    original_priority = getpriority(PRIO_PROCESS, 0);
    if (errno && (original_priority == -1))
    return -21;
    retval = setpriority(PRIO_PROCESS, 0, priority);
    if (retval)
    return -21;
    errno = 0;
    retval = getpriority(PRIO_PROCESS, 0);
    if (retval != priority)
    return -21;
    return original_priority;
    }
#[no_mangle]
pub unsafe extern "C" fn turbostat_loop() {
    void turbostat_loop()
    {
    int retval;
    let mut restarted: c_int = 0;
    let mut done_iters: c_uint = 0;
    setup_signal_handler();
//
// elevate own priority for interval mode
//
// ignore on error - we probably don't have permission to set it, but
// it's not a big deal
//
    set_my_sched_priority(-20);
    restart:
    restarted++;
    snapshot_proc_sysfs_files();
    retval = for_all_cpus(get_counters, EVEN_COUNTERS);
    first_counter_read = 0;
    if (retval < -1) {
    exit(retval);
    } else if (retval == -1) {
    if (restarted > 10) {
    exit(retval);
    }
    re_initialize();
    goto restart;
    }
    restarted = 0;
    done_iters = 0;
    gettimeofday(&tv_even, (struct timezone *)core::ptr::null_mut());
    while (1) {
    if (for_all_proc_cpus(cpu_is_not_present)) {
    re_initialize();
    goto restart;
    }
    if (update_effective_str(false)) {
    re_initialize();
    goto restart;
    }
    do_sleep();
    if (snapshot_proc_sysfs_files())
    goto restart;
    retval = for_all_cpus(get_counters, ODD_COUNTERS);
    if (retval < -1) {
    exit(retval);
    } else if (retval == -1) {
    re_initialize();
    goto restart;
    }
    gettimeofday(&tv_odd, (struct timezone *)core::ptr::null_mut());
    timersub(&tv_odd, &tv_even, &tv_delta);
    if (for_all_cpus_2(delta_cpu, ODD_COUNTERS, EVEN_COUNTERS)) {
    re_initialize();
    goto restart;
    }
    delta_platform(&platform_counters_odd, &platform_counters_even);
    compute_average(EVEN_COUNTERS);
    format_all_counters(EVEN_COUNTERS);
    flush_output_stdout();
    if (exit_requested)
    break;
    if (num_iterations && ++done_iters >= num_iterations)
    break;
    do_sleep();
    if (snapshot_proc_sysfs_files())
    goto restart;
    retval = for_all_cpus(get_counters, EVEN_COUNTERS);
    if (retval < -1) {
    exit(retval);
    } else if (retval == -1) {
    re_initialize();
    goto restart;
    }
    gettimeofday(&tv_even, (struct timezone *)core::ptr::null_mut());
    timersub(&tv_even, &tv_odd, &tv_delta);
    if (for_all_cpus_2(delta_cpu, EVEN_COUNTERS, ODD_COUNTERS)) {
    re_initialize();
    goto restart;
    }
    delta_platform(&platform_counters_even, &platform_counters_odd);
    compute_average(ODD_COUNTERS);
    format_all_counters(ODD_COUNTERS);
    flush_output_stdout();
    if (exit_requested)
    break;
    if (num_iterations && ++done_iters >= num_iterations)
    break;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn probe_dev_msr() -> c_int {
    int probe_dev_msr(void)
    {
    struct stat sb;
    char pathname[32];
    sprintf(pathname, "/dev/msr%d", master_cpu);
    return !stat(pathname, &sb);
    }
#[no_mangle]
pub unsafe extern "C" fn probe_dev_cpu_msr() -> c_int {
    int probe_dev_cpu_msr(void)
    {
    struct stat sb;
    char pathname[32];
    sprintf(pathname, "/dev/cpu/%d/msr", master_cpu);
    return !stat(pathname, &sb);
    }
#[no_mangle]
pub unsafe extern "C" fn probe_msr_driver() -> c_int {
    int probe_msr_driver(void)
    {
    if (probe_dev_msr()) {
    use_android_msr_path = 1;
    return 1;
    }
    return probe_dev_cpu_msr();
    }
#[no_mangle]
pub unsafe extern "C" fn check_msr_driver() {
    void check_msr_driver(void)
    {
    if (probe_msr_driver())
    return;
    if (system("/sbin/modprobe msr > /dev/null 2>&1"))
    no_msr = 1;
    if (!probe_msr_driver())
    no_msr = 1;
    }
//
// check for CAP_SYS_RAWIO
// return 0 on success
// return 1 on fail
//
#[no_mangle]
pub unsafe extern "C" fn check_for_cap_sys_rawio() -> c_int {
    int check_for_cap_sys_rawio(void)
    {
    cap_t caps;
    cap_flag_value_t cap_flag_value;
    let mut ret: c_int = 0;
    caps = cap_get_proc();
    if (caps == core::ptr::null_mut()) {
//
// CONFIG_MULTIUSER=n kernels have no cap_get_proc()
// Allow them to continue and attempt to access MSRs
//
    if (errno == ENOSYS)
    return 0;
    return 1;
    }
    if (cap_get_flag(caps, CAP_SYS_RAWIO, CAP_EFFECTIVE, &cap_flag_value)) {
    ret = 1;
    goto free_and_exit;
    }
    if (cap_flag_value != CAP_SET) {
    ret = 1;
    goto free_and_exit;
    }
    free_and_exit:
    if (cap_free(caps) == -1)
    err(-6, "cap_free");
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn check_msr_permission() {
    void check_msr_permission(void)
    {
    let mut failed: c_int = 0;
    char pathname[32];
    if (no_msr)
    return;
// check for CAP_SYS_RAWIO
    failed += check_for_cap_sys_rawio();
// test file permissions
    sprintf(pathname, use_android_msr_path ? "/dev/msr%d" : "/dev/cpu/%d/msr", master_cpu);
    if (euidaccess(pathname, R_OK)) {
    failed++;
    }
    if (failed) {
    warnx("Failed to access %s. Some of the counters may not be available\n"
    "\tRun as root to enable them or use %s to disable the access explicitly", pathname, "--no-msr");
    no_msr = 1;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn probe_bclk() {
    void probe_bclk(void)
    {
    unsigned long long msr;
    unsigned int base_ratio;
    if (!platform.has_nhm_msrs || no_msr)
    return;
    if (platform.bclk_freq == BCLK_100MHZ)
    bclk = 100.00;
#[no_mangle]
pub unsafe extern "C" fn if(BCLK_133MHZ: platform->bclk_freq ==) -> else {
    else if (platform.bclk_freq == BCLK_133MHZ)
    bclk = 133.33;
#[no_mangle]
pub unsafe extern "C" fn if(BCLK_SLV: platform->bclk_freq ==) -> else {
    else if (platform.bclk_freq == BCLK_SLV)
    bclk = slm_bclk();
    else
    return;
    get_msr(master_cpu, MSR_PLATFORM_INFO, &msr);
    base_ratio = (msr >> 8) & 0xFF;
    base_hz = base_ratio * bclk * 1000000;
    has_base_hz = 1;
    if (platform.enable_tsc_tweak)
    tsc_tweak = base_hz / tsc_hz;
    }
#[no_mangle]
unsafe extern "C" fn remove_underbar(s: *mut c_char) {
    static void remove_underbar(char *s)
    {
    char *to = s;
    while (*s) {
    if (*s != '_')
// to++ = *s;
    s++;
    }
// to = 0;
    }
#[no_mangle]
unsafe extern "C" fn dump_turbo_ratio_info() {
    static void dump_turbo_ratio_info(void)
    {
    if (!has_turbo)
    return;
    if (!platform.has_nhm_msrs || no_msr)
    return;
    if (platform.trl_msrs & TRL_LIMIT2)
    dump_turbo_ratio_limit2();
    if (platform.trl_msrs & TRL_LIMIT1)
    dump_turbo_ratio_limit1();
    if (platform.trl_msrs & TRL_BASE) {
    dump_turbo_ratio_limits(MSR_TURBO_RATIO_LIMIT);
    if (is_hybrid)
    dump_turbo_ratio_limits(MSR_SECONDARY_TURBO_RATIO_LIMIT);
    }
    if (platform.trl_msrs & TRL_ATOM)
    dump_atom_turbo_ratio_limits();
    if (platform.trl_msrs & TRL_KNL)
    dump_knl_turbo_ratio_limits();
    if (platform.has_config_tdp)
    dump_config_tdp();
    }
#[no_mangle]
unsafe extern "C" fn read_sysfs_int(path: *mut c_char) -> c_int {
    static int read_sysfs_int(char *path)
    {
    FILE *input;
    let mut retval: c_int = -1;
    input = fopen(path, "r");
    if (input == core::ptr::null_mut()) {
    if (debug)
    fprintf(outf, "NSFOD %s\n", path);
    return (-1);
    }
    if (fscanf(input, "%d", &retval) != 1)
    err(1, "%s: failed to read int from file", path);
    fclose(input);
    return (retval);
    }
#[no_mangle]
unsafe extern "C" fn dump_sysfs_file(path: *mut c_char) {
    static void dump_sysfs_file(char *path)
    {
    FILE *input;
    char cpuidle_buf[64];
    input = fopen(path, "r");
    if (input == core::ptr::null_mut()) {
    if (debug)
    fprintf(outf, "NSFOD %s\n", path);
    return;
    }
    if (!fgets(cpuidle_buf, sizeof(cpuidle_buf), input))
    err(1, "%s: failed to read file", path);
    fclose(input);
    fprintf(outf, "%s: %s", strrchr(path, '/') + 1, cpuidle_buf);
    }
#[no_mangle]
unsafe extern "C" fn probe_intel_uncore_frequency_legacy() {
    static void probe_intel_uncore_frequency_legacy(void)
    {
    int i, j;
    char path[256];
    for (i = 0; i < topo.num_packages; ++i) {
    for (j = 0; j <= topo.max_die_id; ++j) {
    int k, l;
    char path_base[128];
    sprintf(path_base, "/sys/devices/system/cpu/intel_uncore_frequency/package_%02d_die_%02d", i, j);
    sprintf(path, "%s/current_freq_khz", path_base);
    if (access(path, R_OK))
    continue;
    BIC_PRESENT(BIC_UNCORE_MHZ);
    if (quiet)
    return;
    sprintf(path, "%s/min_freq_khz", path_base);
    k = read_sysfs_int(path);
    sprintf(path, "%s/max_freq_khz", path_base);
    l = read_sysfs_int(path);
    fprintf(outf, "Uncore Frequency package%d die%d: %d - %d MHz ", i, j, k / 1000, l / 1000);
    sprintf(path, "%s/initial_min_freq_khz", path_base);
    k = read_sysfs_int(path);
    sprintf(path, "%s/initial_max_freq_khz", path_base);
    l = read_sysfs_int(path);
    fprintf(outf, "(%d - %d MHz)", k / 1000, l / 1000);
    sprintf(path, "%s/current_freq_khz", path_base);
    k = read_sysfs_int(path);
    fprintf(outf, " %d MHz\n", k / 1000);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn probe_intel_uncore_frequency_cluster() {
    static void probe_intel_uncore_frequency_cluster(void)
    {
    int i, uncore_max_id;
    char path[256];
    char path_base[128];
    if (access("/sys/devices/system/cpu/intel_uncore_frequency/uncore00/current_freq_khz", R_OK))
    return;
    for (uncore_max_id = 0;; ++uncore_max_id) {
    sprintf(path_base, "/sys/devices/system/cpu/intel_uncore_frequency/uncore%02d", uncore_max_id);
// uncore## start at 00 and skips no numbers, so stop upon first missing
    if (access(path_base, R_OK)) {
    uncore_max_id -= 1;
    break;
    }
    }
    for (i = uncore_max_id; i >= 0; --i) {
    int k, l;
    int unc_pkg_id, domain_id, cluster_id;
    char name_buf[16];
    sprintf(path_base, "/sys/devices/system/cpu/intel_uncore_frequency/uncore%02d", i);
    if (access(path_base, R_OK))
    err(1, "%s: %s", __func__, path_base);
    sprintf(path, "%s/package_id", path_base);
    unc_pkg_id = read_sysfs_int(path);
    sprintf(path, "%s/domain_id", path_base);
    domain_id = read_sysfs_int(path);
    sprintf(path, "%s/fabric_cluster_id", path_base);
    cluster_id = read_sysfs_int(path);
    sprintf(path, "%s/current_freq_khz", path_base);
    sprintf(name_buf, "UMHz%d.%d", domain_id, cluster_id);
//
// Once add_couter() is called, that counter is always read
// and reported -- So it is effectively (enabled & present).
// Only call add_counter() here if legacy BIC_UNCORE_MHZ (UncMHz)
// is (enabled).  Since we are in this routine, we
// know we will not probe and set (present) the legacy counter.
//
// This allows "--show/--hide UncMHz" to be effective for
// the clustered MHz counters, as a group.
//
    if BIC_IS_ENABLED
    (BIC_UNCORE_MHZ)
    add_counter(0, path, name_buf, 0, SCOPE_PACKAGE, COUNTER_K2M, FORMAT_AVERAGE, 0, unc_pkg_id);
    if (quiet)
    continue;
    sprintf(path, "%s/min_freq_khz", path_base);
    k = read_sysfs_int(path);
    sprintf(path, "%s/max_freq_khz", path_base);
    l = read_sysfs_int(path);
    fprintf(outf, "Uncore Frequency package%d domain%d cluster%d: %d - %d MHz ", unc_pkg_id, domain_id, cluster_id, k / 1000, l / 1000);
    sprintf(path, "%s/initial_min_freq_khz", path_base);
    k = read_sysfs_int(path);
    sprintf(path, "%s/initial_max_freq_khz", path_base);
    l = read_sysfs_int(path);
    fprintf(outf, "(%d - %d MHz)", k / 1000, l / 1000);
    sprintf(path, "%s/current_freq_khz", path_base);
    k = read_sysfs_int(path);
    fprintf(outf, " %d MHz\n", k / 1000);
    }
    }
#[no_mangle]
unsafe extern "C" fn probe_intel_uncore_frequency() {
    static void probe_intel_uncore_frequency(void)
    {
    if (!genuine_intel)
    return;
    if (access("/sys/devices/system/cpu/intel_uncore_frequency/uncore00", R_OK) == 0)
    probe_intel_uncore_frequency_cluster();
    else
    probe_intel_uncore_frequency_legacy();
    }
#[no_mangle]
unsafe extern "C" fn set_graphics_fp(path: *mut c_char, idx: c_int) {
    static void set_graphics_fp(char *path, int idx)
    {
    if (!access(path, R_OK))
    gfx_info[idx].fp = fopen_or_die(path, "r");
    }
// Enlarge this if there are /sys/class/drm/card2 ...
pub const GFX_MAX_CARDS: c_int = 2;
#[no_mangle]
unsafe extern "C" fn probe_graphics() {
    static void probe_graphics(void)
    {
    char path[PATH_MAX];
    int i;
// Xe graphics sysfs knobs
    if (!access("/sys/class/drm/card0/device/tile0/gt0/gtidle/idle_residency_ms", R_OK)) {
    FILE *fp;
    char buf[8];
    bool gt0_is_gt;
    fp = fopen("/sys/class/drm/card0/device/tile0/gt0/gtidle/name", "r");
    if (!fp)
    goto next;
    if (!fread(buf, sizeof(char), 7, fp)) {
    fclose(fp);
    goto next;
    }
    fclose(fp);
    if (!strncmp(buf, "gt0-rc", strlen("gt0-rc")))
    gt0_is_gt = true;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncmp(buf, _arg: "gt0-mc", _arg: strlen("gt0-mc"))) -> else {
    else if (!strncmp(buf, "gt0-mc", strlen("gt0-mc")))
    gt0_is_gt = false;
    else
    goto next;
    set_graphics_fp("/sys/class/drm/card0/device/tile0/gt0/gtidle/idle_residency_ms", gt0_is_gt ? GFX_rc6 : SAM_mc6);
    set_graphics_fp("/sys/class/drm/card0/device/tile0/gt0/freq0/cur_freq", gt0_is_gt ? GFX_MHz : SAM_MHz);
    set_graphics_fp("/sys/class/drm/card0/device/tile0/gt0/freq0/act_freq", gt0_is_gt ? GFX_ACTMHz : SAM_ACTMHz);
    set_graphics_fp("/sys/class/drm/card0/device/tile0/gt1/gtidle/idle_residency_ms", gt0_is_gt ? SAM_mc6 : GFX_rc6);
    set_graphics_fp("/sys/class/drm/card0/device/tile0/gt1/freq0/cur_freq", gt0_is_gt ? SAM_MHz : GFX_MHz);
    set_graphics_fp("/sys/class/drm/card0/device/tile0/gt1/freq0/act_freq", gt0_is_gt ? SAM_ACTMHz : GFX_ACTMHz);
    goto end;
    }
    next:
// New i915 graphics sysfs knobs
    for (i = 0; i < GFX_MAX_CARDS; i++) {
    snprintf(path, PATH_MAX, "/sys/class/drm/card%d/gt/gt0/rc6_residency_ms", i);
    if (!access(path, R_OK))
    break;
    }
    if (i == GFX_MAX_CARDS)
    goto legacy_i915;
    snprintf(path, PATH_MAX, "/sys/class/drm/card%d/gt/gt0/rc6_residency_ms", i);
    set_graphics_fp(path, GFX_rc6);
    snprintf(path, PATH_MAX, "/sys/class/drm/card%d/gt/gt0/rps_cur_freq_mhz", i);
    set_graphics_fp(path, GFX_MHz);
    snprintf(path, PATH_MAX, "/sys/class/drm/card%d/gt/gt0/rps_act_freq_mhz", i);
    set_graphics_fp(path, GFX_ACTMHz);
    snprintf(path, PATH_MAX, "/sys/class/drm/card%d/gt/gt1/rc6_residency_ms", i);
    set_graphics_fp(path, SAM_mc6);
    snprintf(path, PATH_MAX, "/sys/class/drm/card%d/gt/gt1/rps_cur_freq_mhz", i);
    set_graphics_fp(path, SAM_MHz);
    snprintf(path, PATH_MAX, "/sys/class/drm/card%d/gt/gt1/rps_act_freq_mhz", i);
    set_graphics_fp(path, SAM_ACTMHz);
    goto end;
    legacy_i915:
// Fall back to traditional i915 graphics sysfs knobs
    set_graphics_fp("/sys/class/drm/card0/power/rc6_residency_ms", GFX_rc6);
    set_graphics_fp("/sys/class/drm/card0/gt_cur_freq_mhz", GFX_MHz);
    if (!gfx_info[GFX_MHz].fp)
    set_graphics_fp("/sys/class/graphics/fb0/device/drm/card0/gt_cur_freq_mhz", GFX_MHz);
    set_graphics_fp("/sys/class/drm/card0/gt_act_freq_mhz", GFX_ACTMHz);
    if (!gfx_info[GFX_ACTMHz].fp)
    set_graphics_fp("/sys/class/graphics/fb0/device/drm/card0/gt_act_freq_mhz", GFX_ACTMHz);
    end:
    if (gfx_info[GFX_rc6].fp)
    BIC_PRESENT(BIC_GFX_rc6);
    if (gfx_info[GFX_MHz].fp)
    BIC_PRESENT(BIC_GFXMHz);
    if (gfx_info[GFX_ACTMHz].fp)
    BIC_PRESENT(BIC_GFXACTMHz);
    if (gfx_info[SAM_mc6].fp)
    BIC_PRESENT(BIC_SAM_mc6);
    if (gfx_info[SAM_MHz].fp)
    BIC_PRESENT(BIC_SAMMHz);
    if (gfx_info[SAM_ACTMHz].fp)
    BIC_PRESENT(BIC_SAMACTMHz);
    }
#[no_mangle]
unsafe extern "C" fn dump_sysfs_cstate_config() {
    static void dump_sysfs_cstate_config(void)
    {
    char path[64];
    char name_buf[16];
    char desc[64];
    FILE *input;
    int state;
    char *sp;
    if (access("/sys/devices/system/cpu/cpuidle", R_OK)) {
    fprintf(outf, "cpuidle not loaded\n");
    return;
    }
    dump_sysfs_file("/sys/devices/system/cpu/cpuidle/current_driver");
    dump_sysfs_file("/sys/devices/system/cpu/cpuidle/current_governor");
    dump_sysfs_file("/sys/devices/system/cpu/cpuidle/current_governor_ro");
    for (state = 0; state < 10; ++state) {
    sprintf(path, "/sys/devices/system/cpu/cpu%d/cpuidle/state%d/name", master_cpu, state);
    input = fopen(path, "r");
    if (input == core::ptr::null_mut())
    continue;
    if (!fgets(name_buf, sizeof(name_buf), input))
    err(1, "%s: failed to read file", path);
// truncate "C1-HSW\n" to "C1", or truncate "C1\n" to "C1"
    sp = strchr(name_buf, '-');
    if (!sp)
    sp = strchrnul(name_buf, '\n');
// sp = '\0';
    fclose(input);
    remove_underbar(name_buf);
    sprintf(path, "/sys/devices/system/cpu/cpu%d/cpuidle/state%d/desc", master_cpu, state);
    input = fopen(path, "r");
    if (input == core::ptr::null_mut())
    continue;
    if (!fgets(desc, sizeof(desc), input))
    err(1, "%s: failed to read file", path);
    fprintf(outf, "cpu%d: %s: %s", master_cpu, name_buf, desc);
    fclose(input);
    }
    }
#[no_mangle]
unsafe extern "C" fn dump_sysfs_pstate_config() {
    static void dump_sysfs_pstate_config(void)
    {
    char path[64];
    char driver_buf[64];
    char governor_buf[64];
    FILE *input;
    int turbo;
    sprintf(path, "/sys/devices/system/cpu/cpu%d/cpufreq/scaling_driver", master_cpu);
    input = fopen(path, "r");
    if (input == core::ptr::null_mut()) {
    fprintf(outf, "NSFOD %s\n", path);
    return;
    }
    if (!fgets(driver_buf, sizeof(driver_buf), input))
    err(1, "%s: failed to read file", path);
    fclose(input);
    sprintf(path, "/sys/devices/system/cpu/cpu%d/cpufreq/scaling_governor", master_cpu);
    input = fopen(path, "r");
    if (input == core::ptr::null_mut()) {
    fprintf(outf, "NSFOD %s\n", path);
    return;
    }
    if (!fgets(governor_buf, sizeof(governor_buf), input))
    err(1, "%s: failed to read file", path);
    fclose(input);
    fprintf(outf, "cpu%d: cpufreq driver: %s", master_cpu, driver_buf);
    fprintf(outf, "cpu%d: cpufreq governor: %s", master_cpu, governor_buf);
    sprintf(path, "/sys/devices/system/cpu/cpufreq/boost");
    input = fopen(path, "r");
    if (input != core::ptr::null_mut()) {
    if (fscanf(input, "%d", &turbo) != 1)
    err(1, "%s: failed to parse number from file", path);
    fprintf(outf, "cpufreq boost: %d\n", turbo);
    fclose(input);
    }
    sprintf(path, "/sys/devices/system/cpu/intel_pstate/no_turbo");
    input = fopen(path, "r");
    if (input != core::ptr::null_mut()) {
    if (fscanf(input, "%d", &turbo) != 1)
    err(1, "%s: failed to parse number from file", path);
    fprintf(outf, "cpufreq intel_pstate no_turbo: %d\n", turbo);
    fclose(input);
    }
    }
//
// print_epb()
// Decode the ENERGY_PERF_BIAS MSR
//
#[no_mangle]
pub unsafe extern "C" fn print_epb(_arg: PER_THREAD_PARAMS) -> c_int {
    int print_epb(PER_THREAD_PARAMS)
    {
    char *epb_string;
    int cpu, epb;
    UNUSED(c);
    UNUSED(p);
    if (!has_epb)
    return 0;
    cpu = t.cpu_id;
// EPB is per-package
    if (!is_cpu_first_thread_in_package(t, c, p))
    return 0;
    if (cpu_migrate(cpu)) {
    fprintf(outf, "print_epb: Could not migrate to CPU %d\n", cpu);
    return -1;
    }
    epb = get_epb(cpu);
    if (epb < 0)
    return 0;
    switch (epb) {
    case ENERGY_PERF_BIAS_PERFORMANCE:
    epb_string = "performance";
    break;
    case ENERGY_PERF_BIAS_NORMAL:
    epb_string = "balanced";
    break;
    case ENERGY_PERF_BIAS_POWERSAVE:
    epb_string = "powersave";
    break;
    default:
    epb_string = "custom";
    break;
    }
    fprintf(outf, "cpu%d: EPB: %d (%s)\n", cpu, epb, epb_string);
    return 0;
    }
//
// print_hwp()
// Decode the MSR_HWP_CAPABILITIES
//
#[no_mangle]
pub unsafe extern "C" fn print_hwp(_arg: PER_THREAD_PARAMS) -> c_int {
    int print_hwp(PER_THREAD_PARAMS)
    {
    unsigned long long msr;
    int cpu;
    UNUSED(c);
    UNUSED(p);
    if (no_msr)
    return 0;
    if (!has_hwp)
    return 0;
    cpu = t.cpu_id;
// MSR_HWP_CAPABILITIES is per-package
    if (!is_cpu_first_thread_in_package(t, c, p))
    return 0;
    if (cpu_migrate(cpu)) {
    fprintf(outf, "print_hwp: Could not migrate to CPU %d\n", cpu);
    return -1;
    }
    if (get_msr(cpu, MSR_PM_ENABLE, &msr))
    return 0;
    fprintf(outf, "cpu%d: MSR_PM_ENABLE: 0x%08llx (%sHWP)\n", cpu, msr, (msr & (1 << 0)) ? "" : "No-");
// MSR_PM_ENABLE[1] == 1 if HWP is enabled and MSRs visible
    if ((msr & (1 << 0)) == 0)
    return 0;
    if (get_msr(cpu, MSR_HWP_CAPABILITIES, &msr))
    return 0;
    fprintf(outf, "cpu%d: MSR_HWP_CAPABILITIES: 0x%08llx "
    "(high %d guar %d eff %d low %d)\n",
    cpu, msr,
    (unsigned int)HWP_HIGHEST_PERF(msr),
    (unsigned int)HWP_GUARANTEED_PERF(msr), (unsigned int)HWP_MOSTEFFICIENT_PERF(msr), (unsigned int)HWP_LOWEST_PERF(msr));
    if (get_msr(cpu, MSR_HWP_REQUEST, &msr))
    return 0;
    fprintf(outf, "cpu%d: MSR_HWP_REQUEST: 0x%08llx "
    "(min %d max %d des %d epp 0x%x window 0x%x pkg 0x%x)\n",
    cpu, msr,
    (unsigned int)(((msr) >> 0) & 0xff),
    (unsigned int)(((msr) >> 8) & 0xff),
    (unsigned int)(((msr) >> 16) & 0xff),
    (unsigned int)(((msr) >> 24) & 0xff), (unsigned int)(((msr) >> 32) & 0xff3), (unsigned int)(((msr) >> 42) & 0x1));
    if (has_hwp_pkg) {
    if (get_msr(cpu, MSR_HWP_REQUEST_PKG, &msr))
    return 0;
    fprintf(outf, "cpu%d: MSR_HWP_REQUEST_PKG: 0x%08llx "
    "(min %d max %d des %d epp 0x%x window 0x%x)\n",
    cpu, msr,
    (unsigned int)(((msr) >> 0) & 0xff),
    (unsigned int)(((msr) >> 8) & 0xff),
    (unsigned int)(((msr) >> 16) & 0xff), (unsigned int)(((msr) >> 24) & 0xff), (unsigned int)(((msr) >> 32) & 0xff3));
    }
    if (has_hwp_notify) {
    if (get_msr(cpu, MSR_HWP_INTERRUPT, &msr))
    return 0;
    fprintf(outf, "cpu%d: MSR_HWP_INTERRUPT: 0x%08llx "
    "(%s_Guaranteed_Perf_Change, %s_Excursion_Min)\n", cpu, msr, ((msr) & 0x1) ? "EN" : "Dis", ((msr) & 0x2) ? "EN" : "Dis");
    }
    if (get_msr(cpu, MSR_HWP_STATUS, &msr))
    return 0;
    fprintf(outf, "cpu%d: MSR_HWP_STATUS: 0x%08llx "
    "(%sGuaranteed_Perf_Change, %sExcursion_Min)\n", cpu, msr, ((msr) & 0x1) ? "" : "No-", ((msr) & 0x4) ? "" : "No-");
    return 0;
    }
//
// print_perf_limit()
//
#[no_mangle]
pub unsafe extern "C" fn print_perf_limit(_arg: PER_THREAD_PARAMS) -> c_int {
    int print_perf_limit(PER_THREAD_PARAMS)
    {
    unsigned long long msr;
    int cpu;
    UNUSED(c);
    UNUSED(p);
    if (no_msr)
    return 0;
    cpu = t.cpu_id;
// per-package
    if (!is_cpu_first_thread_in_package(t, c, p))
    return 0;
    if (cpu_migrate(cpu)) {
    fprintf(outf, "print_perf_limit: Could not migrate to CPU %d\n", cpu);
    return -1;
    }
    if (platform.plr_msrs & PLR_CORE) {
    get_msr(cpu, MSR_CORE_PERF_LIMIT_REASONS, &msr);
    fprintf(outf, "cpu%d: MSR_CORE_PERF_LIMIT_REASONS, 0x%08llx", cpu, msr);
    fprintf(outf, " (Active: %s%s%s%s%s%s%s%s%s%s%s%s%s%s)",
    (msr & 1 << 15) ? "bit15, " : "",
    (msr & 1 << 14) ? "bit14, " : "",
    (msr & 1 << 13) ? "Transitions, " : "",
    (msr & 1 << 12) ? "MultiCoreTurbo, " : "",
    (msr & 1 << 11) ? "PkgPwrL2, " : "",
    (msr & 1 << 10) ? "PkgPwrL1, " : "",
    (msr & 1 << 9) ? "CorePwr, " : "",
    (msr & 1 << 8) ? "Amps, " : "",
    (msr & 1 << 6) ? "VR-Therm, " : "",
    (msr & 1 << 5) ? "Auto-HWP, " : "",
    (msr & 1 << 4) ? "Graphics, " : "",
    (msr & 1 << 2) ? "bit2, " : "", (msr & 1 << 1) ? "ThermStatus, " : "", (msr & 1 << 0) ? "PROCHOT, " : "");
    fprintf(outf, " (Logged: %s%s%s%s%s%s%s%s%s%s%s%s%s%s)\n",
    (msr & 1 << 31) ? "bit31, " : "",
    (msr & 1 << 30) ? "bit30, " : "",
    (msr & 1 << 29) ? "Transitions, " : "",
    (msr & 1 << 28) ? "MultiCoreTurbo, " : "",
    (msr & 1 << 27) ? "PkgPwrL2, " : "",
    (msr & 1 << 26) ? "PkgPwrL1, " : "",
    (msr & 1 << 25) ? "CorePwr, " : "",
    (msr & 1 << 24) ? "Amps, " : "",
    (msr & 1 << 22) ? "VR-Therm, " : "",
    (msr & 1 << 21) ? "Auto-HWP, " : "",
    (msr & 1 << 20) ? "Graphics, " : "",
    (msr & 1 << 18) ? "bit18, " : "", (msr & 1 << 17) ? "ThermStatus, " : "", (msr & 1 << 16) ? "PROCHOT, " : "");
    }
    if (platform.plr_msrs & PLR_GFX) {
    get_msr(cpu, MSR_GFX_PERF_LIMIT_REASONS, &msr);
    fprintf(outf, "cpu%d: MSR_GFX_PERF_LIMIT_REASONS, 0x%08llx", cpu, msr);
    fprintf(outf, " (Active: %s%s%s%s%s%s%s%s)",
    (msr & 1 << 0) ? "PROCHOT, " : "",
    (msr & 1 << 1) ? "ThermStatus, " : "",
    (msr & 1 << 4) ? "Graphics, " : "",
    (msr & 1 << 6) ? "VR-Therm, " : "",
    (msr & 1 << 8) ? "Amps, " : "",
    (msr & 1 << 9) ? "GFXPwr, " : "", (msr & 1 << 10) ? "PkgPwrL1, " : "", (msr & 1 << 11) ? "PkgPwrL2, " : "");
    fprintf(outf, " (Logged: %s%s%s%s%s%s%s%s)\n",
    (msr & 1 << 16) ? "PROCHOT, " : "",
    (msr & 1 << 17) ? "ThermStatus, " : "",
    (msr & 1 << 20) ? "Graphics, " : "",
    (msr & 1 << 22) ? "VR-Therm, " : "",
    (msr & 1 << 24) ? "Amps, " : "",
    (msr & 1 << 25) ? "GFXPwr, " : "", (msr & 1 << 26) ? "PkgPwrL1, " : "", (msr & 1 << 27) ? "PkgPwrL2, " : "");
    }
    if (platform.plr_msrs & PLR_RING) {
    get_msr(cpu, MSR_RING_PERF_LIMIT_REASONS, &msr);
    fprintf(outf, "cpu%d: MSR_RING_PERF_LIMIT_REASONS, 0x%08llx", cpu, msr);
    fprintf(outf, " (Active: %s%s%s%s%s%s)",
    (msr & 1 << 0) ? "PROCHOT, " : "",
    (msr & 1 << 1) ? "ThermStatus, " : "",
    (msr & 1 << 6) ? "VR-Therm, " : "",
    (msr & 1 << 8) ? "Amps, " : "", (msr & 1 << 10) ? "PkgPwrL1, " : "", (msr & 1 << 11) ? "PkgPwrL2, " : "");
    fprintf(outf, " (Logged: %s%s%s%s%s%s)\n",
    (msr & 1 << 16) ? "PROCHOT, " : "",
    (msr & 1 << 17) ? "ThermStatus, " : "",
    (msr & 1 << 22) ? "VR-Therm, " : "",
    (msr & 1 << 24) ? "Amps, " : "", (msr & 1 << 26) ? "PkgPwrL1, " : "", (msr & 1 << 27) ? "PkgPwrL2, " : "");
    }
    return 0;
    }
pub const RAPL_POWER_GRANULARITY: c_uint = 0x7FFF	/* 15 bit power granularity */;
pub const RAPL_TIME_GRANULARITY: c_uint = 0x3F	/* 6 bit time granularity */;
#[no_mangle]
pub unsafe extern "C" fn get_quirk_tdp() -> double {
    double get_quirk_tdp(void)
    {
    if (platform.rapl_quirk_tdp)
    return platform.rapl_quirk_tdp;
    return 135.0;
    }
#[no_mangle]
pub unsafe extern "C" fn get_tdp_intel() -> double {
    double get_tdp_intel(void)
    {
    unsigned long long msr;
    if (valid_rapl_msrs & RAPL_PKG_POWER_INFO)
    if (!get_msr(master_cpu, MSR_PKG_POWER_INFO, &msr))
    return ((msr >> 0) & RAPL_POWER_GRANULARITY) * rapl_power_units;
    return get_quirk_tdp();
    }
#[no_mangle]
pub unsafe extern "C" fn get_tdp_amd() -> double {
    double get_tdp_amd(void)
    {
    return get_quirk_tdp();
    }
#[no_mangle]
pub unsafe extern "C" fn rapl_probe_intel() {
    void rapl_probe_intel(void)
    {
    unsigned long long msr;
    unsigned int time_unit;
    double tdp;
    if (rapl_joules) {
    CLR_BIC(BIC_SysWatt, &bic_enabled);
    CLR_BIC(BIC_PkgWatt, &bic_enabled);
    CLR_BIC(BIC_CorWatt, &bic_enabled);
    CLR_BIC(BIC_RAMWatt, &bic_enabled);
    CLR_BIC(BIC_GFXWatt, &bic_enabled);
    } else {
    CLR_BIC(BIC_Sys_J, &bic_enabled);
    CLR_BIC(BIC_Pkg_J, &bic_enabled);
    CLR_BIC(BIC_Cor_J, &bic_enabled);
    CLR_BIC(BIC_RAM_J, &bic_enabled);
    CLR_BIC(BIC_GFX_J, &bic_enabled);
    }
    if (!valid_rapl_msrs || no_msr)
    return;
    if (!(valid_rapl_msrs & RAPL_PKG_PERF_STATUS))
    CLR_BIC(BIC_PKG__, &bic_enabled);
    if (!(valid_rapl_msrs & RAPL_DRAM_PERF_STATUS))
    CLR_BIC(BIC_RAM__, &bic_enabled);
// units on package 0, verify later other packages match
    if (get_msr(master_cpu, MSR_RAPL_POWER_UNIT, &msr))
    return;
    rapl_power_units = 1.0 / (1 << (msr & 0xF));
    if (platform.has_rapl_divisor)
    rapl_energy_units = 1.0 * (1 << (msr >> 8 & 0x1F)) / 1000000;
    else
    rapl_energy_units = 1.0 / (1 << (msr >> 8 & 0x1F));
    if (platform.has_fixed_rapl_unit)
    rapl_dram_energy_units = (15.3 / 1000000);
    else
    rapl_dram_energy_units = rapl_energy_units;
    if (platform.has_fixed_rapl_psys_unit)
    rapl_psys_energy_units = 1.0;
    else
    rapl_psys_energy_units = rapl_energy_units;
    time_unit = msr >> 16 & 0xF;
    if (time_unit == 0)
    time_unit = 0xA;
    rapl_time_units = 1.0 / (1 << (time_unit));
    tdp = get_tdp_intel();
    rapl_joule_counter_range = 0xFFFFFFFF * rapl_energy_units / tdp;
    if (!quiet)
    fprintf(outf, "RAPL: %.0f sec. Joule Counter Range, at %.0f Watts\n", rapl_joule_counter_range, tdp);
    }
#[no_mangle]
pub unsafe extern "C" fn rapl_probe_amd() {
    void rapl_probe_amd(void)
    {
    unsigned long long msr;
    double tdp;
    if (rapl_joules) {
    CLR_BIC(BIC_SysWatt, &bic_enabled);
    CLR_BIC(BIC_CorWatt, &bic_enabled);
    } else {
    CLR_BIC(BIC_Pkg_J, &bic_enabled);
    CLR_BIC(BIC_Cor_J, &bic_enabled);
    }
    if (!valid_rapl_msrs || no_msr)
    return;
    if (get_msr(master_cpu, MSR_RAPL_PWR_UNIT, &msr))
    return;
    rapl_time_units = ldexp(1.0, -(msr >> 16 & 0xf));
    rapl_energy_units = ldexp(1.0, -(msr >> 8 & 0x1f));
    rapl_power_units = ldexp(1.0, -(msr & 0xf));
    tdp = get_tdp_amd();
    rapl_joule_counter_range = 0xFFFFFFFF * rapl_energy_units / tdp;
    if (!quiet)
    fprintf(outf, "RAPL: %.0f sec. Joule Counter Range, at %.0f Watts\n", rapl_joule_counter_range, tdp);
    }
#[no_mangle]
pub unsafe extern "C" fn print_power_limit_msr(cpu: c_int, msr: c_ulonglong, label: *mut c_char) {
    void print_power_limit_msr(int cpu, unsigned long long msr, char *label)
    {
    fprintf(outf, "cpu%d: %s: %sabled (%0.3f Watts, %f sec, clamp %sabled)\n",
    cpu, label,
    ((msr >> 15) & 1) ? "EN" : "DIS",
    ((msr >> 0) & 0x7FFF) * rapl_power_units,
    (1.0 + (((msr >> 22) & 0x3) / 4.0)) * (1 << ((msr >> 17) & 0x1F)) * rapl_time_units, (((msr >> 16) & 1) ? "EN" : "DIS"));
    return;
    }
#[no_mangle]
unsafe extern "C" fn fread_int(path: *mut c_char, val: *mut c_int) -> c_int {
    static int fread_int(char *path, int *val)
    {
    FILE *filep;
    int ret;
    filep = fopen(path, "r");
    if (!filep)
    return -1;
    ret = fscanf(filep, "%d", val);
    fclose(filep);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn fread_ull(path: *mut c_char, val: *mut c_ulonglong) -> c_int {
    static int fread_ull(char *path, unsigned long long *val)
    {
    FILE *filep;
    int ret;
    filep = fopen(path, "r");
    if (!filep)
    return -1;
    ret = fscanf(filep, "%llu", val);
    fclose(filep);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn fread_str(path: *mut c_char, buf: *mut c_char, size: c_int) -> c_int {
    static int fread_str(char *path, char *buf, int size)
    {
    FILE *filep;
    int ret;
    char *cp;
    filep = fopen(path, "r");
    if (!filep)
    return -1;
    ret = fread(buf, 1, size, filep);
    fclose(filep);
// replace '\n' with '\0'
    cp = strchr(buf, '\n');
    if (cp != core::ptr::null_mut())
// cp = '\0';
    return ret;
    }

#[no_mangle]
unsafe extern "C" fn dump_one_domain(domain_path: *mut c_char) -> c_int {
    static int dump_one_domain(char *domain_path)
    {
    char path[PATH_MAX];
    char str[PATH_MAX];
    unsigned long long val;
    int constraint;
    int enable;
    int ret;
    snprintf(path, PATH_MAX, "%s/name", domain_path);
    ret = fread_str(path, str, PATH_MAX);
    if (ret <= 0)
    return -1;
    fprintf(outf, "%s: %s", domain_path + strlen(PATH_RAPL_SYSFS) + 1, str);
    snprintf(path, PATH_MAX, "%s/enabled", domain_path);
    ret = fread_int(path, &enable);
    if (ret <= 0)
    return -1;
    if (!enable) {
    fputs(" disabled\n", outf);
    return 0;
    }
    for (constraint = 0;; constraint++) {
    snprintf(path, PATH_MAX, "%s/constraint_%d_time_window_us", domain_path, constraint);
    ret = fread_ull(path, &val);
    if (ret <= 0)
    break;
    if (val > 1000000)
    fprintf(outf, " %0.1fs", (double)val / 1000000);
#[no_mangle]
pub unsafe extern "C" fn if(1000: val >) -> else {
    else if (val > 1000)
    fprintf(outf, " %0.1fms", (double)val / 1000);
    else
    fprintf(outf, " %0.1fus", (double)val);
    snprintf(path, PATH_MAX, "%s/constraint_%d_power_limit_uw", domain_path, constraint);
    ret = fread_ull(path, &val);
    if (ret > 0 && val)
    fprintf(outf, ":%lluW", val / 1000000);
    snprintf(path, PATH_MAX, "%s/constraint_%d_max_power_uw", domain_path, constraint);
    ret = fread_ull(path, &val);
    if (ret > 0 && val)
    fprintf(outf, ",max:%lluW", val / 1000000);
    }
    fputc('\n', outf);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn print_rapl_sysfs() -> c_int {
    static int print_rapl_sysfs(void)
    {
    DIR *dir, *cdir;
    struct dirent *entry, *centry;
    char path[PATH_MAX];
    char str[PATH_MAX];
    if ((dir = opendir(PATH_RAPL_SYSFS)) == core::ptr::null_mut()) {
    warn("open %s failed", PATH_RAPL_SYSFS);
    return 1;
    }
    while ((entry = readdir(dir)) != core::ptr::null_mut()) {
    if (strlen(entry.d_name) > 100)
    continue;
    if (strncmp(entry.d_name, "intel-rapl", strlen("intel-rapl")))
    continue;
    snprintf(path, PATH_MAX, "%s/%s/name", PATH_RAPL_SYSFS, entry.d_name);
// Parse top level domains first, including package and psys
    fread_str(path, str, PATH_MAX);
    if (strncmp(str, "package", strlen("package")) && strncmp(str, "psys", strlen("psys")))
    continue;
    snprintf(path, PATH_MAX, "%s/%s", PATH_RAPL_SYSFS, entry.d_name);
    if ((cdir = opendir(path)) == core::ptr::null_mut()) {
    perror("opendir() error");
    return 1;
    }
    dump_one_domain(path);
    while ((centry = readdir(cdir)) != core::ptr::null_mut()) {
    if (strncmp(centry.d_name, "intel-rapl", strlen("intel-rapl")))
    continue;
    snprintf(path, PATH_MAX, "%s/%s/%s", PATH_RAPL_SYSFS, entry.d_name, centry.d_name);
    dump_one_domain(path);
    }
    closedir(cdir);
    }
    closedir(dir);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn print_rapl(_arg: PER_THREAD_PARAMS) -> c_int {
    int print_rapl(PER_THREAD_PARAMS)
    {
    unsigned long long msr;
    const char *msr_name;
    int cpu;
    UNUSED(c);
    UNUSED(p);
    if (!valid_rapl_msrs)
    return 0;
// RAPL counters are per package, so print only for 1st thread/package
    if (!is_cpu_first_thread_in_package(t, c, p))
    return 0;
    cpu = t.cpu_id;
    if (cpu_migrate(cpu)) {
    fprintf(outf, "print_rapl: Could not migrate to CPU %d\n", cpu);
    return -1;
    }
    if (valid_rapl_msrs & RAPL_AMD_F17H) {
    msr_name = "MSR_RAPL_PWR_UNIT";
    if (get_msr(cpu, MSR_RAPL_PWR_UNIT, &msr))
    return -1;
    } else {
    msr_name = "MSR_RAPL_POWER_UNIT";
    if (get_msr(cpu, MSR_RAPL_POWER_UNIT, &msr))
    return -1;
    }
    fprintf(outf, "cpu%d: %s: 0x%08llx (%f Watts, %f Joules, %f sec.)\n", cpu, msr_name, msr, rapl_power_units, rapl_energy_units, rapl_time_units);
    if (valid_rapl_msrs & RAPL_PKG_POWER_INFO) {
    if (get_msr(cpu, MSR_PKG_POWER_INFO, &msr))
    return -5;
    fprintf(outf, "cpu%d: MSR_PKG_POWER_INFO: 0x%08llx (%.0f W TDP, RAPL %.0f - %.0f W, %f sec.)\n",
    cpu, msr,
    ((msr >> 0) & RAPL_POWER_GRANULARITY) * rapl_power_units,
    ((msr >> 16) & RAPL_POWER_GRANULARITY) * rapl_power_units,
    ((msr >> 32) & RAPL_POWER_GRANULARITY) * rapl_power_units, ((msr >> 48) & RAPL_TIME_GRANULARITY) * rapl_time_units);
    }
    if (valid_rapl_msrs & RAPL_PKG) {
    if (get_msr(cpu, MSR_PKG_POWER_LIMIT, &msr))
    return -9;
    fprintf(outf, "cpu%d: MSR_PKG_POWER_LIMIT: 0x%08llx (%slocked)\n", cpu, msr, (msr >> 63) & 1 ? "" : "UN");
    print_power_limit_msr(cpu, msr, "PKG Limit #1");
    fprintf(outf, "cpu%d: PKG Limit #2: %sabled (%0.3f Watts, %f* sec, clamp %sabled)\n",
    cpu,
    ((msr >> 47) & 1) ? "EN" : "DIS",
    ((msr >> 32) & 0x7FFF) * rapl_power_units,
    (1.0 + (((msr >> 54) & 0x3) / 4.0)) * (1 << ((msr >> 49) & 0x1F)) * rapl_time_units, ((msr >> 48) & 1) ? "EN" : "DIS");
    if (get_msr(cpu, MSR_VR_CURRENT_CONFIG, &msr))
    return -9;
    fprintf(outf, "cpu%d: MSR_VR_CURRENT_CONFIG: 0x%08llx\n", cpu, msr);
    fprintf(outf, "cpu%d: PKG Limit #4: %f Watts (%slocked)\n", cpu, ((msr >> 0) & 0x1FFF) * rapl_power_units, (msr >> 31) & 1 ? "" : "UN");
    }
    if (valid_rapl_msrs & RAPL_DRAM_POWER_INFO) {
    if (get_msr(cpu, MSR_DRAM_POWER_INFO, &msr))
    return -6;
    fprintf(outf, "cpu%d: MSR_DRAM_POWER_INFO,: 0x%08llx (%.0f W TDP, RAPL %.0f - %.0f W, %f sec.)\n",
    cpu, msr,
    ((msr >> 0) & RAPL_POWER_GRANULARITY) * rapl_power_units,
    ((msr >> 16) & RAPL_POWER_GRANULARITY) * rapl_power_units,
    ((msr >> 32) & RAPL_POWER_GRANULARITY) * rapl_power_units, ((msr >> 48) & RAPL_TIME_GRANULARITY) * rapl_time_units);
    }
    if (valid_rapl_msrs & RAPL_DRAM) {
    if (get_msr(cpu, MSR_DRAM_POWER_LIMIT, &msr))
    return -9;
    fprintf(outf, "cpu%d: MSR_DRAM_POWER_LIMIT: 0x%08llx (%slocked)\n", cpu, msr, (msr >> 31) & 1 ? "" : "UN");
    print_power_limit_msr(cpu, msr, "DRAM Limit");
    }
    if (valid_rapl_msrs & RAPL_CORE_POLICY) {
    if (get_msr(cpu, MSR_PP0_POLICY, &msr))
    return -7;
    fprintf(outf, "cpu%d: MSR_PP0_POLICY: %lld\n", cpu, msr & 0xF);
    }
    if (valid_rapl_msrs & RAPL_CORE_POWER_LIMIT) {
    if (get_msr(cpu, MSR_PP0_POWER_LIMIT, &msr))
    return -9;
    fprintf(outf, "cpu%d: MSR_PP0_POWER_LIMIT: 0x%08llx (%slocked)\n", cpu, msr, (msr >> 31) & 1 ? "" : "UN");
    print_power_limit_msr(cpu, msr, "Cores Limit");
    }
    if (valid_rapl_msrs & RAPL_GFX) {
    if (get_msr(cpu, MSR_PP1_POLICY, &msr))
    return -8;
    fprintf(outf, "cpu%d: MSR_PP1_POLICY: %lld\n", cpu, msr & 0xF);
    if (get_msr(cpu, MSR_PP1_POWER_LIMIT, &msr))
    return -9;
    fprintf(outf, "cpu%d: MSR_PP1_POWER_LIMIT: 0x%08llx (%slocked)\n", cpu, msr, (msr >> 31) & 1 ? "" : "UN");
    print_power_limit_msr(cpu, msr, "GFX Limit");
    }
    return 0;
    }
//
// probe_rapl_msrs
//
// initialize global valid_rapl_msrs to platform->plat_rapl_msrs
// only if PKG_ENERGY counter is enumerated and reads non-zero
//
#[no_mangle]
pub unsafe extern "C" fn probe_rapl_msrs() {
    void probe_rapl_msrs(void)
    {
    int ret;
    off_t offset;
    unsigned long long msr_value;
    if (no_msr)
    return;
    if ((platform.plat_rapl_msrs & (RAPL_PKG | RAPL_AMD_F17H)) == 0)
    return;
    offset = idx_to_offset(IDX_PKG_ENERGY);
    if (offset < 0)
    return;
    ret = get_msr(master_cpu, offset, &msr_value);
    if (ret) {
    if (debug)
    fprintf(outf, "Can not read RAPL_PKG_ENERGY MSR(0x%llx)\n", (unsigned long long)offset);
    return;
    }
    if (msr_value == 0) {
    if (debug)
    fprintf(outf, "RAPL_PKG_ENERGY MSR(0x%llx) == ZERO: disabling all RAPL MSRs\n", (unsigned long long)offset);
    return;
    }
    valid_rapl_msrs = platform.plat_rapl_msrs;	/* success */
    }
//
// probe_rapl()
//
// sets rapl_power_units, rapl_energy_units, rapl_time_units
//
#[no_mangle]
pub unsafe extern "C" fn probe_rapl() {
    void probe_rapl(void)
    {
    probe_rapl_msrs();
    if (genuine_intel)
    rapl_probe_intel();
    if (authentic_amd || hygon_genuine)
    rapl_probe_amd();
    if (quiet)
    return;
    print_rapl_sysfs();
    if (!valid_rapl_msrs || no_msr)
    return;
    for_all_cpus(print_rapl, ODD_COUNTERS);
    }
//
// MSR_IA32_TEMPERATURE_TARGET indicates the temperature where
// the Thermal Control Circuit (TCC) activates.
// This is usually equal to tjMax.
//
// Older processors do not have this MSR, so there we guess,
// but also allow cmdline over-ride with -T.
//
// Several MSR temperature values are in units of degrees-C
// below this value, including the Digital Thermal Sensor (DTS),
// Package Thermal Management Sensor (PTM), and thermal event thresholds.
//
#[no_mangle]
pub unsafe extern "C" fn set_temperature_target(_arg: PER_THREAD_PARAMS) -> c_int {
    int set_temperature_target(PER_THREAD_PARAMS)
    {
    unsigned long long msr;
    unsigned int tcc_default, tcc_offset;
    int cpu;
    UNUSED(c);
    UNUSED(p);
// tj_max is used only for dts or ptm
    if (!(do_dts || do_ptm))
    return 0;
// this is a per-package concept
    if (!is_cpu_first_thread_in_package(t, c, p))
    return 0;
    cpu = t.cpu_id;
    if (cpu_migrate(cpu)) {
    fprintf(outf, "Could not migrate to CPU %d\n", cpu);
    return -1;
    }
    if (tj_max_override != 0) {
    tj_max = tj_max_override;
    fprintf(outf, "cpu%d: Using cmdline TCC Target (%d C)\n", cpu, tj_max);
    return 0;
    }
// Temperature Target MSR is Nehalem and newer only
    if (!platform.has_nhm_msrs || no_msr)
    goto guess;
    if (get_msr(master_cpu, MSR_IA32_TEMPERATURE_TARGET, &msr))
    goto guess;
    tcc_default = (msr >> 16) & 0xFF;
    if (!quiet) {
    let mut bits: c_int = platform.tcc_offset_bits;
    let mut enabled: c_ulonglong = 0;
    if (bits && !get_msr(master_cpu, MSR_PLATFORM_INFO, &enabled))
    enabled = (enabled >> 30) & 1;
    if (bits && enabled) {
    tcc_offset = (msr >> 24) & GENMASK(bits - 1, 0);
    fprintf(outf, "cpu%d: MSR_IA32_TEMPERATURE_TARGET: 0x%08llx (%d C) (%d default - %d offset)\n",
    cpu, msr, tcc_default - tcc_offset, tcc_default, tcc_offset);
    } else {
    fprintf(outf, "cpu%d: MSR_IA32_TEMPERATURE_TARGET: 0x%08llx (%d C)\n", cpu, msr, tcc_default);
    }
    }
    if (!tcc_default)
    goto guess;
    tj_max = tcc_default;
    return 0;
    guess:
    tj_max = TJMAX_DEFAULT;
    fprintf(outf, "cpu%d: Guessing tjMax %d C, Please use -T to specify\n", cpu, tj_max);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn print_thermal(_arg: PER_THREAD_PARAMS) -> c_int {
    int print_thermal(PER_THREAD_PARAMS)
    {
    unsigned long long msr;
    unsigned int dts, dts2;
    int cpu;
    UNUSED(c);
    UNUSED(p);
    if (no_msr)
    return 0;
    if (!(do_dts || do_ptm))
    return 0;
    cpu = t.cpu_id;
// DTS is per-core, no need to print for each thread
    if (!is_cpu_first_thread_in_core(t, c))
    return 0;
    if (cpu_migrate(cpu)) {
    fprintf(outf, "print_thermal: Could not migrate to CPU %d\n", cpu);
    return -1;
    }
    if (do_ptm && is_cpu_first_core_in_package(t, p)) {
    if (get_msr(cpu, MSR_IA32_PACKAGE_THERM_STATUS, &msr))
    return 0;
    dts = (msr >> 16) & 0x7F;
    fprintf(outf, "cpu%d: MSR_IA32_PACKAGE_THERM_STATUS: 0x%08llx (%d C)\n", cpu, msr, tj_max - dts);
    if (get_msr(cpu, MSR_IA32_PACKAGE_THERM_INTERRUPT, &msr))
    return 0;
    dts = (msr >> 16) & 0x7F;
    dts2 = (msr >> 8) & 0x7F;
    fprintf(outf, "cpu%d: MSR_IA32_PACKAGE_THERM_INTERRUPT: 0x%08llx (%d C, %d C)\n", cpu, msr, tj_max - dts, tj_max - dts2);
    }
    if (do_dts && debug) {
    unsigned int resolution;
    if (get_msr(cpu, MSR_IA32_THERM_STATUS, &msr))
    return 0;
    dts = (msr >> 16) & 0x7F;
    resolution = (msr >> 27) & 0xF;
    fprintf(outf, "cpu%d: MSR_IA32_THERM_STATUS: 0x%08llx (%d C +/- %d)\n", cpu, msr, tj_max - dts, resolution);
    if (get_msr(cpu, MSR_IA32_THERM_INTERRUPT, &msr))
    return 0;
    dts = (msr >> 16) & 0x7F;
    dts2 = (msr >> 8) & 0x7F;
    fprintf(outf, "cpu%d: MSR_IA32_THERM_INTERRUPT: 0x%08llx (%d C, %d C)\n", cpu, msr, tj_max - dts, tj_max - dts2);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn probe_thermal() {
    void probe_thermal(void)
    {
    if (!access("/sys/devices/system/cpu/cpu0/thermal_throttle/core_throttle_count", R_OK))
    BIC_PRESENT(BIC_CORE_THROT_CNT);
    else
    BIC_NOT_PRESENT(BIC_CORE_THROT_CNT);
    for_all_cpus(set_temperature_target, ODD_COUNTERS);
    if (quiet)
    return;
    for_all_cpus(print_thermal, ODD_COUNTERS);
    }
#[no_mangle]
pub unsafe extern "C" fn get_cpu_type(_arg: PER_THREAD_PARAMS) -> c_int {
    int get_cpu_type(PER_THREAD_PARAMS)
    {
    unsigned int eax, ebx, ecx, edx;
    UNUSED(c);
    UNUSED(p);
    if (!genuine_intel)
    return 0;
    if (cpu_migrate(t.cpu_id)) {
    fprintf(outf, "Could not migrate to CPU %d\n", t.cpu_id);
    return -1;
    }
    if (max_level < 0x1a)
    return 0;
    __cpuid(0x1a, eax, ebx, ecx, edx);
    eax = (eax >> 24) & 0xFF;
    if (eax == 0x20)
    t.is_atom = true;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn decode_feature_control_msr() {
    void decode_feature_control_msr(void)
    {
    unsigned long long msr;
    if (no_msr)
    return;
    if (quiet)
    return;
    if (!get_msr(master_cpu, MSR_IA32_FEAT_CTL, &msr))
    fprintf(outf, "cpu%d: MSR_IA32_FEATURE_CONTROL: 0x%08llx (%sLocked %s)\n",
    master_cpu, msr, msr & FEAT_CTL_LOCKED ? "" : "UN-", msr & (1 << 18) ? "SGX" : "");
    }
#[no_mangle]
pub unsafe extern "C" fn decode_misc_enable_msr() {
    void decode_misc_enable_msr(void)
    {
    unsigned long long msr;
    if (no_msr)
    return;
    if (!genuine_intel)
    return;
    if (!get_msr(master_cpu, MSR_IA32_MISC_ENABLE, &msr))
    fprintf(outf, "cpu%d: MSR_IA32_MISC_ENABLE: 0x%08llx (%sTCC %sEIST %sMWAIT %sPREFETCH %sTURBO)\n",
    master_cpu, msr,
    msr & MSR_IA32_MISC_ENABLE_TM1 ? "" : "No-",
    msr & MSR_IA32_MISC_ENABLE_ENHANCED_SPEEDSTEP ? "" : "No-",
    msr & MSR_IA32_MISC_ENABLE_MWAIT ? "" : "No-",
    msr & MSR_IA32_MISC_ENABLE_PREFETCH_DISABLE ? "No-" : "", msr & MSR_IA32_MISC_ENABLE_TURBO_DISABLE ? "No-" : "");
    }
#[no_mangle]
pub unsafe extern "C" fn decode_misc_feature_control() {
    void decode_misc_feature_control(void)
    {
    unsigned long long msr;
    if (no_msr)
    return;
    if (!platform.has_msr_misc_feature_control)
    return;
    if (!get_msr(master_cpu, MSR_MISC_FEATURE_CONTROL, &msr))
    fprintf(outf,
    "cpu%d: MSR_MISC_FEATURE_CONTROL: 0x%08llx (%sL2-Prefetch %sL2-Prefetch-pair %sL1-Prefetch %sL1-IP-Prefetch)\n",
    master_cpu, msr, msr & (0 << 0) ? "No-" : "", msr & (1 << 0) ? "No-" : "", msr & (2 << 0) ? "No-" : "", msr & (3 << 0) ? "No-" : "");
    }
//
// Decode MSR_MISC_PWR_MGMT
//
// Decode the bits according to the Nehalem documentation
// bit[0] seems to continue to have same meaning going forward
// bit[1] less so...
//
#[no_mangle]
pub unsafe extern "C" fn decode_misc_pwr_mgmt_msr() {
    void decode_misc_pwr_mgmt_msr(void)
    {
    unsigned long long msr;
    if (no_msr)
    return;
    if (!platform.has_msr_misc_pwr_mgmt)
    return;
    if (!get_msr(master_cpu, MSR_MISC_PWR_MGMT, &msr))
    fprintf(outf, "cpu%d: MSR_MISC_PWR_MGMT: 0x%08llx (%sable-EIST_Coordination %sable-EPB %sable-OOB)\n",
    master_cpu, msr, msr & (1 << 0) ? "DIS" : "EN", msr & (1 << 1) ? "EN" : "DIS", msr & (1 << 8) ? "EN" : "DIS");
    }
//
// Decode MSR_CC6_DEMOTION_POLICY_CONFIG, MSR_MC6_DEMOTION_POLICY_CONFIG
//
// This MSRs are present on Silvermont processors,
// Intel Atom processor E3000 series (Baytrail), and friends.
//
#[no_mangle]
pub unsafe extern "C" fn decode_c6_demotion_policy_msr() {
    void decode_c6_demotion_policy_msr(void)
    {
    unsigned long long msr;
    if (no_msr)
    return;
    if (!platform.has_msr_c6_demotion_policy_config)
    return;
    if (!get_msr(master_cpu, MSR_CC6_DEMOTION_POLICY_CONFIG, &msr))
    fprintf(outf, "cpu%d: MSR_CC6_DEMOTION_POLICY_CONFIG: 0x%08llx (%sable-CC6-Demotion)\n", master_cpu, msr, msr & (1 << 0) ? "EN" : "DIS");
    if (!get_msr(master_cpu, MSR_MC6_DEMOTION_POLICY_CONFIG, &msr))
    fprintf(outf, "cpu%d: MSR_MC6_DEMOTION_POLICY_CONFIG: 0x%08llx (%sable-MC6-Demotion)\n", master_cpu, msr, msr & (1 << 0) ? "EN" : "DIS");
    }
#[no_mangle]
pub unsafe extern "C" fn print_dev_latency() {
    void print_dev_latency(void)
    {
    char *path = "/dev/cpu_dma_latency";
    int fd;
    int value;
    int retval;
    fd = open(path, O_RDONLY);
    if (fd < 0) {
    if (debug)
    warnx("Read %s failed", path);
    return;
    }
    retval = read(fd, (void *)&value, sizeof(int));
    if (retval != sizeof(int)) {
    warn("read failed %s", path);
    close(fd);
    return;
    }
    fprintf(outf, "/dev/cpu_dma_latency: %d usec (%s)\n", value, value == 2000000000 ? "default" : "constrained");
    close(fd);
    }
#[no_mangle]
unsafe extern "C" fn has_perf_instr_count_access() -> c_int {
    static int has_perf_instr_count_access(void)
    {
    int fd;
    if (no_perf)
    return 0;
    fd = open_perf_counter(master_cpu, PERF_TYPE_HARDWARE, PERF_COUNT_HW_INSTRUCTIONS, -1, 0);
    if (fd != -1)
    close(fd);
    if (fd == -1)
    warnx("Failed to access %s. Some of the counters may not be available\n"
    "\tRun as root to enable them or use %s to disable the access explicitly", "perf instructions retired counter",
    "'--hide IPC' or '--no-perf'");
    return (fd != -1);
    }
#[no_mangle]
pub unsafe extern "C" fn add_rapl_perf_counter(cpu: c_int, rci: *mut rapl_counter_info_t, cai: *const rapl_counter_arch_info, scale_: *mut double, unit_: *mut enum rapl_unit) -> c_int {
    int add_rapl_perf_counter(int cpu, struct rapl_counter_info_t *rci, const struct rapl_counter_arch_info *cai, double *scale_, enum rapl_unit *unit_)
    {
    let mut ret: c_int = -1;
    if (no_perf)
    return -1;
    if (!cai.perf_name)
    return -1;
    let mut scale: double = read_perf_scale(cai.perf_subsys, cai.perf_name);
    if (scale == 0.0)
    goto end;
    let mut unit: enum rapl_unit = read_perf_rapl_unit(cai.perf_subsys, cai.perf_name);
    if (unit == RAPL_UNIT_INVALID)
    goto end;
    let mut rapl_type: c_uint = read_perf_type(cai.perf_subsys);
    let mut rapl_energy_pkg_config: c_uint = read_perf_config(cai.perf_subsys, cai.perf_name);
    ret = open_perf_counter(cpu, rapl_type, rapl_energy_pkg_config, rci.fd_perf, PERF_FORMAT_GROUP);
    if (ret == -1)
    goto end;
// If it's the first counter opened, make it a group descriptor
    if (rci.fd_perf == -1)
    rci.fd_perf = ret;
// scale_ = scale;
// unit_ = unit;
    end:
    if (debug >= 2)
    fprintf(stderr, "%s: %d (cpu: %d)\n", __func__, ret, cpu);
    return ret;
    }
    char cpuset_buf[1024];
#[no_mangle]
pub unsafe extern "C" fn initialize_cpu_set_from_sysfs(cpu_set: *mut cpu_set_t, sysfs_path: *mut c_char, sysfs_file: *mut c_char) -> c_int {
    int initialize_cpu_set_from_sysfs(cpu_set_t *cpu_set, char *sysfs_path, char *sysfs_file)
    {
    FILE *fp;
    char path[128];
    if (snprintf(path, 128, "%s/%s", sysfs_path, sysfs_file) > 128)
    err(-1, "%s %s", sysfs_path, sysfs_file);
    fp = fopen(path, "r");
    if (!fp) {
    warn("open %s", path);
    return -1;
    }
    if (fread(cpuset_buf, sizeof(char), 1024, fp) == 0) {
    warn("read %s", sysfs_path);
    goto err;
    }
    if (parse_cpu_str(cpuset_buf, cpu_set, cpu_possible_setsize)) {
    warnx("%s: cpu str malformat %s\n", sysfs_path, cpu_effective_str);
    goto err;
    }
    return 0;
    err:
    fclose(fp);
    return -1;
    }
#[no_mangle]
pub unsafe extern "C" fn print_cpu_set(s: *mut c_char, set: *mut cpu_set_t) {
    void print_cpu_set(char *s, cpu_set_t *set)
    {
    int i;
    assert(MAX_BIC < CPU_SETSIZE);
    printf("%s:", s);
    for (i = 0; i <= topo.max_cpu_num; ++i)
    if (CPU_ISSET(i, set))
    printf(" %d", i);
    putchar('\n');
    }
#[no_mangle]
pub unsafe extern "C" fn linux_perf_init_hybrid_cpus() {
    void linux_perf_init_hybrid_cpus(void)
    {
    char *perf_cpu_pcore_path = "/sys/devices/cpu_core";
    char *perf_cpu_ecore_path = "/sys/devices/cpu_atom";
    char *perf_cpu_lcore_path = "/sys/devices/cpu_lowpower";
    char path[128];
    if (!access(perf_cpu_pcore_path, F_OK)) {
    perf_pcore_set = CPU_ALLOC((topo.max_cpu_num + 1));
    if (perf_pcore_set == core::ptr::null_mut())
    err(3, "CPU_ALLOC");
    CPU_ZERO_S(cpu_possible_setsize, perf_pcore_set);
    initialize_cpu_set_from_sysfs(perf_pcore_set, perf_cpu_pcore_path, "cpus");
    if (debug)
    print_cpu_set("perf pcores", perf_pcore_set);
    sprintf(path, "%s/%s", perf_cpu_pcore_path, "type");
    perf_pmu_types.pcore = snapshot_sysfs_counter(path);
    }
    if (!access(perf_cpu_ecore_path, F_OK)) {
    perf_ecore_set = CPU_ALLOC((topo.max_cpu_num + 1));
    if (perf_ecore_set == core::ptr::null_mut())
    err(3, "CPU_ALLOC");
    CPU_ZERO_S(cpu_possible_setsize, perf_ecore_set);
    initialize_cpu_set_from_sysfs(perf_ecore_set, perf_cpu_ecore_path, "cpus");
    if (debug)
    print_cpu_set("perf ecores", perf_ecore_set);
    sprintf(path, "%s/%s", perf_cpu_ecore_path, "type");
    perf_pmu_types.ecore = snapshot_sysfs_counter(path);
    }
    if (!access(perf_cpu_lcore_path, F_OK)) {
    perf_lcore_set = CPU_ALLOC((topo.max_cpu_num + 1));
    if (perf_lcore_set == core::ptr::null_mut())
    err(3, "CPU_ALLOC");
    CPU_ZERO_S(cpu_possible_setsize, perf_lcore_set);
    initialize_cpu_set_from_sysfs(perf_lcore_set, perf_cpu_lcore_path, "cpus");
    if (debug)
    print_cpu_set("perf lcores", perf_lcore_set);
    sprintf(path, "%s/%s", perf_cpu_lcore_path, "type");
    perf_pmu_types.lcore = snapshot_sysfs_counter(path);
    }
    }
//
// Linux-perf related initialization
//
#[no_mangle]
pub unsafe extern "C" fn linux_perf_init() {
    void linux_perf_init(void)
    {
    char path[128];
    char *perf_cpu_path = "/sys/devices/cpu";
    if (access("/proc/sys/kernel/perf_event_paranoid", F_OK))
    return;
    if (!access(perf_cpu_path, F_OK)) {
    sprintf(path, "%s/%s", perf_cpu_path, "type");
    perf_pmu_types.uniform = snapshot_sysfs_counter(path);
    } else {
    linux_perf_init_hybrid_cpus();
    }
    if (BIC_IS_ENABLED(BIC_IPC) && cpuid_has_aperf_mperf) {
    fd_instr_count_percpu = calloc(topo.max_cpu_num + 1, sizeof(int));
    if (fd_instr_count_percpu == core::ptr::null_mut())
    err(-1, "calloc fd_instr_count_percpu");
    }
    if (BIC_IS_ENABLED(BIC_LLC_MRPS) || BIC_IS_ENABLED(BIC_LLC_HIT)) {
    fd_llc_percpu = calloc(topo.max_cpu_num + 1, sizeof(int));
    if (fd_llc_percpu == core::ptr::null_mut())
    err(-1, "calloc fd_llc_percpu");
    }
    if (BIC_IS_ENABLED(BIC_L2_MRPS) || BIC_IS_ENABLED(BIC_L2_HIT)) {
    fd_l2_percpu = calloc(topo.max_cpu_num + 1, sizeof(int));
    if (fd_l2_percpu == core::ptr::null_mut())
    err(-1, "calloc fd_l2_percpu");
    }
    }
#[no_mangle]
pub unsafe extern "C" fn rapl_perf_init() {
    void rapl_perf_init(void)
    {
    let mut num_domains: c_uint = get_rapl_num_domains();
    bool *domain_visited = calloc(num_domains, sizeof(bool));
    rapl_counter_info_perdomain = calloc(num_domains, sizeof(*rapl_counter_info_perdomain));
    if (rapl_counter_info_perdomain == core::ptr::null_mut())
    err(-1, "calloc rapl_counter_info_percpu");
    rapl_counter_info_perdomain_size = num_domains;
//
// Initialize rapl_counter_info_percpu
//
    for (unsigned int domain_id = 0; domain_id < num_domains; ++domain_id) {
    struct rapl_counter_info_t *rci = &rapl_counter_info_perdomain[domain_id];
    rci.fd_perf = -1;
    for (size_t i = 0; i < NUM_RAPL_COUNTERS; ++i) {
    rci.data[i] = 0;
    rci.source[i] = COUNTER_SOURCE_NONE;
    }
    }
//
// Open/probe the counters
// If can't get it via perf, fallback to MSR
//
    for (size_t i = 0; i < ARRAY_SIZE(rapl_counter_arch_infos); ++i) {
    let mut cai: *const rapl_counter_arch_info const = &rapl_counter_arch_infos[i];
    let mut has_counter: bool = 0;
    double scale;
    enum rapl_unit unit;
    unsigned int next_domain;
    if (!BIC_IS_ENABLED(cai.bic_number))
    continue;
    memset(domain_visited, 0, num_domains * sizeof(*domain_visited));
    for (int cpu = 0; cpu < topo.max_cpu_num + 1; ++cpu) {
    if (cpu_is_not_allowed(cpu))
    continue;
// Skip already seen and handled RAPL domains
    next_domain = get_rapl_domain_id(cpu);
    assert(next_domain < num_domains);
    if (domain_visited[next_domain])
    continue;
    domain_visited[next_domain] = 1;
    if ((cai.flags & RAPL_COUNTER_FLAG_PLATFORM_COUNTER) && (cpu != master_cpu))
    continue;
    struct rapl_counter_info_t *rci = &rapl_counter_info_perdomain[next_domain];
//
// rapl_counter_arch_infos[] can have multiple entries describing the same
// counter, due to the difference from different platforms/Vendors.
// E.g. rapl_counter_arch_infos[0] and rapl_counter_arch_infos[1] share the
// same perf_subsys and perf_name, but with different MSR address.
// rapl_counter_arch_infos[0] is for Intel and rapl_counter_arch_infos[1]
// is for AMD.
// In this case, it is possible that multiple rapl_counter_arch_infos[]
// entries are probed just because their perf/msr is duplicate and valid.
//
// Thus need a check to avoid re-probe the same counters.
//
    if (rci.source[cai.rci_index] != COUNTER_SOURCE_NONE)
    break;
// Use perf API for this counter
    if (add_rapl_perf_counter(cpu, rci, cai, &scale, &unit) != -1) {
    rci.source[cai.rci_index] = COUNTER_SOURCE_PERF;
    rci.scale[cai.rci_index] = scale * cai.compat_scale;
    rci.unit[cai.rci_index] = unit;
    rci.flags[cai.rci_index] = cai.flags;
// Use MSR for this counter
    } else if (add_rapl_msr_counter(cpu, cai) >= 0) {
    rci.source[cai.rci_index] = COUNTER_SOURCE_MSR;
    rci.msr[cai.rci_index] = cai.msr;
    rci.msr_mask[cai.rci_index] = cai.msr_mask;
    rci.msr_shift[cai.rci_index] = cai.msr_shift;
    rci.unit[cai.rci_index] = RAPL_UNIT_JOULES;
    rci.scale[cai.rci_index] = *cai.platform_rapl_msr_scale * cai.compat_scale;
    rci.flags[cai.rci_index] = cai.flags;
    }
    if (rci.source[cai.rci_index] != COUNTER_SOURCE_NONE)
    has_counter = 1;
    }
// If any CPU has access to the counter, make it present
    if (has_counter)
    BIC_PRESENT(cai.bic_number);
    }
    free(domain_visited);
    }
// Assumes msr_counter_info is populated
#[no_mangle]
unsafe extern "C" fn has_amperf_access() -> c_int {
    static int has_amperf_access(void)
    {
    return cpuid_has_aperf_mperf && msr_counter_arch_infos[MSR_ARCH_INFO_APERF_INDEX].present && msr_counter_arch_infos[MSR_ARCH_INFO_MPERF_INDEX].present;
    }
    int *get_cstate_perf_group_fd(struct cstate_counter_info_t *cci, const char *group_name)
    {
    if (strcmp(group_name, "cstate_core") == 0)
    return &cci.fd_perf_core;
    if (strcmp(group_name, "cstate_pkg") == 0)
    return &cci.fd_perf_pkg;
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn add_cstate_perf_counter(cpu: c_int, cci: *mut cstate_counter_info_t, cai: *const cstate_counter_arch_info) -> c_int {
    int add_cstate_perf_counter(int cpu, struct cstate_counter_info_t *cci, const struct cstate_counter_arch_info *cai)
    {
    let mut ret: c_int = -1;
    if (no_perf)
    return -1;
    if (!cai.perf_name)
    return -1;
    int *pfd_group = get_cstate_perf_group_fd(cci, cai.perf_subsys);
    if (pfd_group == core::ptr::null_mut())
    goto end;
    let mut type: c_uint = read_perf_type(cai.perf_subsys);
    let mut config: c_uint = read_perf_config(cai.perf_subsys, cai.perf_name);
    ret = open_perf_counter(cpu, type, config, *pfd_group, PERF_FORMAT_GROUP);
    if (ret == -1)
    goto end;
// If it's the first counter opened, make it a group descriptor
    if (*pfd_group == -1)
// pfd_group = ret;
    end:
    if (debug >= 2)
    fprintf(stderr, "%s: %d (cpu: %d)\n", __func__, ret, cpu);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn add_msr_perf_counter(cpu: c_int, cci: *mut msr_counter_info_t, cai: *const msr_counter_arch_info) -> c_int {
    int add_msr_perf_counter(int cpu, struct msr_counter_info_t *cci, const struct msr_counter_arch_info *cai)
    {
    let mut ret: c_int = -1;
    if (no_perf)
    return -1;
    if (!cai.perf_name)
    return -1;
    let mut type: c_uint = read_perf_type(cai.perf_subsys);
    let mut config: c_uint = read_perf_config(cai.perf_subsys, cai.perf_name);
    ret = open_perf_counter(cpu, type, config, cci.fd_perf, PERF_FORMAT_GROUP);
    if (ret == -1)
    goto end;
// If it's the first counter opened, make it a group descriptor
    if (cci.fd_perf == -1)
    cci.fd_perf = ret;
    end:
    if (debug)
    fprintf(stderr, "%s: %s/%s: %d (cpu: %d)\n", __func__, cai.perf_subsys, cai.perf_name, ret, cpu);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn msr_perf_init_() {
    void msr_perf_init_(void)
    {
    let mut mci_num: c_int = topo.max_cpu_num + 1;
    msr_counter_info = calloc(mci_num, sizeof(*msr_counter_info));
    if (!msr_counter_info)
    err(1, "calloc msr_counter_info");
    msr_counter_info_size = mci_num;
    for (int cpu = 0; cpu < mci_num; ++cpu)
    msr_counter_info[cpu].fd_perf = -1;
    for (int cidx = 0; cidx < NUM_MSR_COUNTERS; ++cidx) {
    struct msr_counter_arch_info *cai = &msr_counter_arch_infos[cidx];
    cai.present = false;
    for (int cpu = 0; cpu < mci_num; ++cpu) {
    let mut cci: *mut msr_counter_info_t const = &msr_counter_info[cpu];
    if (cpu_is_not_allowed(cpu))
    continue;
    if (cai.needed) {
// Use perf API for this counter
    if (add_msr_perf_counter(cpu, cci, cai) != -1) {
    cci.source[cai.rci_index] = COUNTER_SOURCE_PERF;
    cai.present = true;
// User MSR for this counter
    } else if (add_msr_counter(cpu, cai.msr) >= 0) {
    cci.source[cai.rci_index] = COUNTER_SOURCE_MSR;
    cci.msr[cai.rci_index] = cai.msr;
    cci.msr_mask[cai.rci_index] = cai.msr_mask;
    cai.present = true;
    }
    }
    }
    }
    }
// Initialize data for reading perf counters from the MSR group.
#[no_mangle]
pub unsafe extern "C" fn msr_perf_init() {
    void msr_perf_init(void)
    {
    let mut need_amperf: bool = false, need_smi = false;
    let mut need_soft_c1: bool = (!platform.has_msr_core_c1_res) && (platform.supported_cstates & CC1);
    need_amperf = BIC_IS_ENABLED(BIC_Avg_MHz) || BIC_IS_ENABLED(BIC_Busy) || BIC_IS_ENABLED(BIC_Bzy_MHz)
    || BIC_IS_ENABLED(BIC_IPC) || need_soft_c1;
    if (BIC_IS_ENABLED(BIC_SMI))
    need_smi = true;
// Enable needed counters
    msr_counter_arch_infos[MSR_ARCH_INFO_APERF_INDEX].needed = need_amperf;
    msr_counter_arch_infos[MSR_ARCH_INFO_MPERF_INDEX].needed = need_amperf;
    msr_counter_arch_infos[MSR_ARCH_INFO_SMI_INDEX].needed = need_smi;
    msr_perf_init_();
    let mut has_amperf: bool = has_amperf_access();
    let mut has_smi: bool = msr_counter_arch_infos[MSR_ARCH_INFO_SMI_INDEX].present;
    has_aperf_access = has_amperf;
    if (has_amperf) {
    BIC_PRESENT(BIC_Avg_MHz);
    BIC_PRESENT(BIC_Busy);
    BIC_PRESENT(BIC_Bzy_MHz);
    BIC_PRESENT(BIC_SMI);
    }
    if (has_smi)
    BIC_PRESENT(BIC_SMI);
    }
#[no_mangle]
pub unsafe extern "C" fn cstate_perf_init_(soft_c1: bool) {
    void cstate_perf_init_(bool soft_c1)
    {
    bool has_counter;
    bool *cores_visited = core::ptr::null_mut(), *pkg_visited = core::ptr::null_mut();
    let mut cores_visited_elems: c_int = topo.max_core_id + 1;
    let mut pkg_visited_elems: c_int = topo.max_package_id + 1;
    let mut cci_num: c_int = topo.max_cpu_num + 1;
    ccstate_counter_info = calloc(cci_num, sizeof(*ccstate_counter_info));
    if (!ccstate_counter_info)
    err(1, "calloc ccstate_counter_arch_info");
    ccstate_counter_info_size = cci_num;
    cores_visited = calloc(cores_visited_elems, sizeof(*cores_visited));
    if (!cores_visited)
    err(1, "calloc cores_visited");
    pkg_visited = calloc(pkg_visited_elems, sizeof(*pkg_visited));
    if (!pkg_visited)
    err(1, "calloc pkg_visited");
// Initialize cstate_counter_info_percpu
    for (int cpu = 0; cpu < cci_num; ++cpu) {
    ccstate_counter_info[cpu].fd_perf_core = -1;
    ccstate_counter_info[cpu].fd_perf_pkg = -1;
    }
    for (int cidx = 0; cidx < NUM_CSTATE_COUNTERS; ++cidx) {
    has_counter = false;
    memset(cores_visited, 0, cores_visited_elems * sizeof(*cores_visited));
    memset(pkg_visited, 0, pkg_visited_elems * sizeof(*pkg_visited));
    const struct cstate_counter_arch_info *cai = &ccstate_counter_arch_infos[cidx];
    for (int cpu = 0; cpu < cci_num; ++cpu) {
    let mut cci: *mut cstate_counter_info_t const = &ccstate_counter_info[cpu];
    if (cpu_is_not_allowed(cpu))
    continue;
    let mut core_id: c_int = cpus[cpu].core_id;
    let mut pkg_id: c_int = cpus[cpu].package_id;
    assert(core_id < cores_visited_elems);
    assert(pkg_id < pkg_visited_elems);
    let mut per_thread: bool = cai.flags & CSTATE_COUNTER_FLAG_COLLECT_PER_THREAD;
    let mut per_core: bool = cai.flags & CSTATE_COUNTER_FLAG_COLLECT_PER_CORE;
    if (!per_thread && cores_visited[core_id])
    continue;
    if (!per_core && pkg_visited[pkg_id])
    continue;
    let mut counter_needed: bool = BIC_IS_ENABLED(cai.bic_number) || (soft_c1 && (cai.flags & CSTATE_COUNTER_FLAG_SOFT_C1_DEPENDENCY));
    let mut counter_supported: bool = (platform.supported_cstates & cai.feature_mask);
    if (counter_needed && counter_supported) {
// Use perf API for this counter
    if (add_cstate_perf_counter(cpu, cci, cai) != -1) {
    cci.source[cai.rci_index] = COUNTER_SOURCE_PERF;
// User MSR for this counter
    } else if (pkg_cstate_limit >= cai.pkg_cstate_limit && add_msr_counter(cpu, cai.msr) >= 0) {
    cci.source[cai.rci_index] = COUNTER_SOURCE_MSR;
    cci.msr[cai.rci_index] = cai.msr;
    }
    }
    if (cci.source[cai.rci_index] != COUNTER_SOURCE_NONE) {
    has_counter = true;
    cores_visited[core_id] = true;
    pkg_visited[pkg_id] = true;
    }
    }
// If any CPU has access to the counter, make it present
    if (has_counter)
    BIC_PRESENT(cai.bic_number);
    }
    free(cores_visited);
    free(pkg_visited);
    }
#[no_mangle]
pub unsafe extern "C" fn cstate_perf_init() {
    void cstate_perf_init(void)
    {
//
// If we don't have a C1 residency MSR, we calculate it "in software",
// but we need APERF, MPERF too.
//
    const bool soft_c1 = !platform.has_msr_core_c1_res && has_amperf_access()
    && platform.supported_cstates & CC1;
    if (soft_c1)
    BIC_PRESENT(BIC_CPU_c1);
    cstate_perf_init_(soft_c1);
    }
#[no_mangle]
pub unsafe extern "C" fn probe_cstates() {
    void probe_cstates(void)
    {
    probe_cst_limit();
    if (platform.has_msr_module_c6_res_ms)
    BIC_PRESENT(BIC_Mod_c6);
    if (platform.has_ext_cst_msrs && !no_msr) {
    BIC_PRESENT(BIC_Totl_c0);
    BIC_PRESENT(BIC_Any_c0);
    BIC_PRESENT(BIC_GFX_c0);
    BIC_PRESENT(BIC_CPUGFX);
    }
    if (quiet)
    return;
    dump_power_ctl();
    dump_cst_cfg();
    decode_c6_demotion_policy_msr();
    print_dev_latency();
    dump_sysfs_cstate_config();
    print_irtl();
    }
#[no_mangle]
pub unsafe extern "C" fn probe_lpi() {
    void probe_lpi(void)
    {
    if (!access("/sys/devices/system/cpu/cpuidle/low_power_idle_cpu_residency_us", R_OK))
    BIC_PRESENT(BIC_CPU_LPI);
    else
    BIC_NOT_PRESENT(BIC_CPU_LPI);
    if (!access(sys_lpi_file_sysfs, R_OK)) {
    sys_lpi_file = sys_lpi_file_sysfs;
    BIC_PRESENT(BIC_SYS_LPI);
    } else if (!access(sys_lpi_file_debugfs, R_OK)) {
    sys_lpi_file = sys_lpi_file_debugfs;
    BIC_PRESENT(BIC_SYS_LPI);
    } else {
    sys_lpi_file_sysfs = core::ptr::null_mut();
    BIC_NOT_PRESENT(BIC_SYS_LPI);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn probe_pstates() {
    void probe_pstates(void)
    {
    probe_bclk();
    if (quiet)
    return;
    dump_platform_info();
    dump_turbo_ratio_info();
    dump_sysfs_pstate_config();
    decode_misc_pwr_mgmt_msr();
    for_all_cpus(print_hwp, ODD_COUNTERS);
    for_all_cpus(print_epb, ODD_COUNTERS);
    for_all_cpus(print_perf_limit, ODD_COUNTERS);
    }
#[no_mangle]
pub unsafe extern "C" fn dump_word_chars(word: c_uint) {
    void dump_word_chars(unsigned int word)
    {
    int i;
    for (i = 0; i < 4; ++i)
    fprintf(outf, "%c", (word >> (i * 8)) & 0xFF);
    }
#[no_mangle]
pub unsafe extern "C" fn dump_cpuid_hypervisor() {
    void dump_cpuid_hypervisor(void)
    {
    let mut ebx: c_uint = 0;
    let mut ecx: c_uint = 0;
    let mut edx: c_uint = 0;
    __cpuid(0x40000000, max_extended_level, ebx, ecx, edx);
    fprintf(outf, "Hypervisor: ");
    dump_word_chars(ebx);
    dump_word_chars(ecx);
    dump_word_chars(edx);
    fprintf(outf, "\n");
    }
#[no_mangle]
pub unsafe extern "C" fn process_cpuid() {
    void process_cpuid()
    {
    unsigned int eax, ebx, ecx, edx;
    unsigned int fms, family, model, stepping, ecx_flags, edx_flags;
    let mut ucode_patch: c_ulonglong = 0;
    let mut ucode_patch_valid: bool = false;
    eax = ebx = ecx = edx = 0;
    __cpuid(0, max_level, ebx, ecx, edx);
    if (ebx == 0x756e6547 && ecx == 0x6c65746e && edx == 0x49656e69)
    genuine_intel = 1;
#[no_mangle]
pub unsafe extern "C" fn if(0x69746e65: ebx == 0x68747541 && ecx == 0x444d4163 && edx ==) -> else {
    else if (ebx == 0x68747541 && ecx == 0x444d4163 && edx == 0x69746e65)
    authentic_amd = 1;
#[no_mangle]
pub unsafe extern "C" fn if(0x6e65476e: ebx == 0x6f677948 && ecx == 0x656e6975 && edx ==) -> else {
    else if (ebx == 0x6f677948 && ecx == 0x656e6975 && edx == 0x6e65476e)
    hygon_genuine = 1;
    if (!quiet)
    fprintf(outf, "CPUID(0): %.4s%.4s%.4s 0x%x CPUID levels\n", (char *)&ebx, (char *)&edx, (char *)&ecx, max_level);
    __cpuid(1, fms, ebx, ecx, edx);
    family = (fms >> 8) & 0xf;
    model = (fms >> 4) & 0xf;
    stepping = fms & 0xf;
    if (family == 0xf)
    family += (fms >> 20) & 0xff;
    if (family >= 6)
    model += ((fms >> 16) & 0xf) << 4;
    ecx_flags = ecx;
    edx_flags = edx;
    cpuid_has_hv = ecx_flags & (1 << 31);
    if (!no_msr) {
    if (get_msr(sched_getcpu(), MSR_IA32_UCODE_REV, &ucode_patch)) {
    warnx("get_msr(UCODE)");
    } else {
    ucode_patch_valid = true;
    if (!authentic_amd && !hygon_genuine)
    ucode_patch >>= 32;
    }
    }
//
// check max extended function levels of CPUID.
// This is needed to check for invariant TSC.
// This check is valid for both Intel and AMD.
//
    ebx = ecx = edx = 0;
    __cpuid(0x80000000, max_extended_level, ebx, ecx, edx);
    if (!quiet) {
    fprintf(outf, "CPUID(1): family:model:stepping 0x%x:%x:%x (%d:%d:%d)", family, model, stepping, family, model, stepping);
    if (ucode_patch_valid)
    fprintf(outf, " microcode 0x%x", (unsigned int)ucode_patch);
    fputc('\n', outf);
    fprintf(outf, "CPUID(0x80000000): max_extended_levels: 0x%x\n", max_extended_level);
    fprintf(outf, "CPUID(1): %sSSE3 %sMONITOR %sSMX %sEIST %sTM2 %sHV %sTSC %sMSR %sACPI-TM %sHT %sTM\n",
    ecx_flags & (1 << 0) ? "" : "No-",
    ecx_flags & (1 << 3) ? "" : "No-",
    ecx_flags & (1 << 6) ? "" : "No-",
    ecx_flags & (1 << 7) ? "" : "No-",
    ecx_flags & (1 << 8) ? "" : "No-",
    cpuid_has_hv ? "" : "No-",
    edx_flags & (1 << 4) ? "" : "No-",
    edx_flags & (1 << 5) ? "" : "No-",
    edx_flags & (1 << 22) ? "" : "No-", edx_flags & (1 << 28) ? "" : "No-", edx_flags & (1 << 29) ? "" : "No-");
    }
    if (!quiet && cpuid_has_hv)
    dump_cpuid_hypervisor();
    probe_platform_features(family, model);
    init_perf_model_support(family, model);
    if (!(edx_flags & (1 << 5)))
    errx(1, "CPUID: no MSR");
    if (max_extended_level >= 0x80000007) {
//
// Non-Stop TSC is advertised by CPUID.EAX=0x80000007: EDX.bit8
// this check is valid for both Intel and AMD
//
    __cpuid(0x80000007, eax, ebx, ecx, edx);
    has_invariant_tsc = edx & (1 << 8);
    }
//
// APERF/MPERF is advertised by CPUID.EAX=0x6: ECX.bit0
// this check is valid for both Intel and AMD
//
    __cpuid(0x6, eax, ebx, ecx, edx);
    cpuid_has_aperf_mperf = ecx & (1 << 0);
    do_dts = eax & (1 << 0);
    if (do_dts)
    BIC_PRESENT(BIC_CoreTmp);
    has_turbo = eax & (1 << 1);
    do_ptm = eax & (1 << 6);
    if (do_ptm)
    BIC_PRESENT(BIC_PkgTmp);
    has_hwp = eax & (1 << 7);
    has_hwp_notify = eax & (1 << 8);
    has_hwp_activity_window = eax & (1 << 9);
    has_hwp_epp = eax & (1 << 10);
    has_hwp_pkg = eax & (1 << 11);
    has_epb = ecx & (1 << 3);
    if (!quiet)
    fprintf(outf, "CPUID(6): %sAPERF, %sTURBO, %sDTS, %sPTM, %sHWP, "
    "%sHWPnotify, %sHWPwindow, %sHWPepp, %sHWPpkg, %sEPB\n",
    cpuid_has_aperf_mperf ? "" : "No-",
    has_turbo ? "" : "No-",
    do_dts ? "" : "No-",
    do_ptm ? "" : "No-",
    has_hwp ? "" : "No-",
    has_hwp_notify ? "" : "No-",
    has_hwp_activity_window ? "" : "No-", has_hwp_epp ? "" : "No-", has_hwp_pkg ? "" : "No-", has_epb ? "" : "No-");
    if (!quiet)
    decode_misc_enable_msr();
    if (max_level >= 0x7) {
    int has_sgx;
    ecx = 0;
    __cpuid_count(0x7, 0, eax, ebx, ecx, edx);
    has_sgx = ebx & (1 << 2);
    is_hybrid = !!(edx & (1 << 15));
    if (!quiet)
    fprintf(outf, "CPUID(7): %sSGX %sHybrid\n", has_sgx ? "" : "No-", is_hybrid ? "" : "No-");
    if (has_sgx)
    decode_feature_control_msr();
    }
    if (max_level >= 0x15) {
    unsigned int eax_crystal;
    unsigned int ebx_tsc;
//
// CPUID 15H TSC/Crystal ratio, possibly Crystal Hz
//
    eax_crystal = ebx_tsc = crystal_hz = edx = 0;
    __cpuid(0x15, eax_crystal, ebx_tsc, crystal_hz, edx);
    if (ebx_tsc != 0) {
    if (!quiet && (ebx != 0))
    fprintf(outf, "CPUID(0x15): eax_crystal: %d ebx_tsc: %d ecx_crystal_hz: %d\n", eax_crystal, ebx_tsc, crystal_hz);
    if (crystal_hz == 0)
    crystal_hz = platform.crystal_freq;
    if (crystal_hz) {
    tsc_hz = (unsigned long long)crystal_hz *ebx_tsc / eax_crystal;
    if (!quiet)
    fprintf(outf, "TSC: %lld MHz (%d Hz * %d / %d / 1000000)\n", tsc_hz / 1000000, crystal_hz, ebx_tsc, eax_crystal);
    }
    }
    }
    if (max_level >= 0x16) {
    unsigned int base_mhz, max_mhz, bus_mhz, edx;
//
// CPUID 16H Base MHz, Max MHz, Bus MHz
//
    base_mhz = max_mhz = bus_mhz = edx = 0;
    __cpuid(0x16, base_mhz, max_mhz, bus_mhz, edx);
    bclk = bus_mhz;
    base_hz = base_mhz * 1000000;
    has_base_hz = 1;
    if (platform.enable_tsc_tweak)
    tsc_tweak = base_hz / tsc_hz;
    if (!quiet)
    fprintf(outf, "CPUID(0x16): base_mhz: %d max_mhz: %d bus_mhz: %d\n", base_mhz, max_mhz, bus_mhz);
    }
    if (cpuid_has_aperf_mperf)
    aperf_mperf_multiplier = platform.need_perf_multiplier ? 1024 : 1;
    BIC_PRESENT(BIC_IRQ);
    BIC_PRESENT(BIC_NMI);
    BIC_PRESENT(BIC_TSC_MHz);
    }
#[no_mangle]
unsafe extern "C" fn counter_info_init() {
    static void counter_info_init(void)
    {
    for (int i = 0; i < NUM_CSTATE_COUNTERS; ++i) {
    let mut cai: *mut cstate_counter_arch_info const = &ccstate_counter_arch_infos[i];
    if (platform.has_msr_knl_core_c6_residency && cai.msr == MSR_CORE_C6_RESIDENCY)
    cai.msr = MSR_KNL_CORE_C6_RESIDENCY;
    if (!platform.has_msr_core_c1_res && cai.msr == MSR_CORE_C1_RES)
    cai.msr = 0;
    if (platform.has_msr_atom_pkg_c6_residency && cai.msr == MSR_PKG_C6_RESIDENCY)
    cai.msr = MSR_ATOM_PKG_C6_RESIDENCY;
    }
    for (int i = 0; i < NUM_MSR_COUNTERS; ++i) {
    msr_counter_arch_infos[i].present = false;
    msr_counter_arch_infos[i].needed = false;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn probe_pm_features() {
    void probe_pm_features(void)
    {
    probe_pstates();
    probe_cstates();
    probe_lpi();
    probe_intel_uncore_frequency();
    probe_graphics();
    probe_rapl();
    probe_thermal();
    if (platform.has_nhm_msrs && !no_msr)
    BIC_PRESENT(BIC_SMI);
    if (!quiet)
    decode_misc_feature_control();
    }
//
// has_perf_llc_access()
//
// return 1 on success, else 0
//
#[no_mangle]
pub unsafe extern "C" fn has_perf_llc_access() -> c_int {
    int has_perf_llc_access(void)
    {
    int fd;
    if (no_perf)
    return 0;
    fd = open_perf_counter(master_cpu, PERF_TYPE_HARDWARE, PERF_COUNT_HW_CACHE_REFERENCES, -1, PERF_FORMAT_GROUP);
    if (fd != -1)
    close(fd);
    if (fd == -1)
    warnx("Failed to access %s. Some of the counters may not be available\n"
    "\tRun as root to enable them or use %s to disable the access explicitly", "perf LLC counters", "'--hide LLC' or '--no-perf'");
    return (fd != -1);
    }
#[no_mangle]
pub unsafe extern "C" fn perf_llc_init() {
    void perf_llc_init(void)
    {
    int cpu;
    int retval;
    if (no_perf)
    return;
    if (!(BIC_IS_ENABLED(BIC_LLC_MRPS) || BIC_IS_ENABLED(BIC_LLC_HIT)))
    return;
    assert(fd_llc_percpu != 0);
    for (cpu = 0; cpu <= topo.max_cpu_num; ++cpu) {
    if (cpu_is_not_allowed(cpu))
    continue;
    fd_llc_percpu[cpu] = open_perf_counter(cpu, PERF_TYPE_HARDWARE, PERF_COUNT_HW_CACHE_REFERENCES, -1, PERF_FORMAT_GROUP);
    if (fd_llc_percpu[cpu] == -1) {
    warnx("%s: perf REFS: failed to open counter on cpu%d", __func__, cpu);
    free_fd_llc_percpu();
    return;
    }
    retval = open_perf_counter(cpu, PERF_TYPE_HARDWARE, PERF_COUNT_HW_CACHE_MISSES, fd_llc_percpu[cpu], PERF_FORMAT_GROUP);
    if (retval == -1) {
    warnx("%s: perf MISS: failed to open counter on cpu%d", __func__, cpu);
    free_fd_llc_percpu();
    return;
    }
    }
    BIC_PRESENT(BIC_LLC_MRPS);
    BIC_PRESENT(BIC_LLC_HIT);
    }
#[no_mangle]
pub unsafe extern "C" fn perf_l2_init() {
    void perf_l2_init(void)
    {
    int cpu;
    int retval;
    if (no_perf)
    return;
    if (!(BIC_IS_ENABLED(BIC_L2_MRPS) || BIC_IS_ENABLED(BIC_L2_HIT)))
    return;
    if (perf_model_support == core::ptr::null_mut())
    return;
    assert(fd_l2_percpu != 0);
    for (cpu = 0; cpu <= topo.max_cpu_num; ++cpu) {
    if (cpu_is_not_allowed(cpu))
    continue;
    if (!is_hybrid) {
    fd_l2_percpu[cpu] = open_perf_counter(cpu, perf_pmu_types.uniform, perf_model_support.first.refs, -1, PERF_FORMAT_GROUP);
    if (fd_l2_percpu[cpu] == -1) {
    warnx("%s(cpu%d, 0x%x, 0x%llx) REFS", __func__, cpu, perf_pmu_types.uniform, perf_model_support.first.refs);
    free_fd_l2_percpu();
    return;
    }
    retval = open_perf_counter(cpu, perf_pmu_types.uniform, perf_model_support.first.hits, fd_l2_percpu[cpu], PERF_FORMAT_GROUP);
    if (retval == -1) {
    warnx("%s(cpu%d, 0x%x, 0x%llx) HITS", __func__, cpu, perf_pmu_types.uniform, perf_model_support.first.hits);
    free_fd_l2_percpu();
    return;
    }
    continue;
    }
    if (perf_pcore_set && CPU_ISSET_S(cpu, cpu_possible_setsize, perf_pcore_set)) {
    fd_l2_percpu[cpu] = open_perf_counter(cpu, perf_pmu_types.pcore, perf_model_support.first.refs, -1, PERF_FORMAT_GROUP);
    if (fd_l2_percpu[cpu] == -1) {
    warnx("%s(cpu%d, 0x%x, 0x%llx) REFS", __func__, cpu, perf_pmu_types.pcore, perf_model_support.first.refs);
    free_fd_l2_percpu();
    return;
    }
    retval = open_perf_counter(cpu, perf_pmu_types.pcore, perf_model_support.first.hits, fd_l2_percpu[cpu], PERF_FORMAT_GROUP);
    if (retval == -1) {
    warnx("%s(cpu%d, 0x%x, 0x%llx) HITS", __func__, cpu, perf_pmu_types.pcore, perf_model_support.first.hits);
    free_fd_l2_percpu();
    return;
    }
    } else if (perf_ecore_set && CPU_ISSET_S(cpu, cpu_possible_setsize, perf_ecore_set)) {
    fd_l2_percpu[cpu] = open_perf_counter(cpu, perf_pmu_types.ecore, perf_model_support.second.refs, -1, PERF_FORMAT_GROUP);
    if (fd_l2_percpu[cpu] == -1) {
    warnx("%s(cpu%d, 0x%x, 0x%llx) REFS", __func__, cpu, perf_pmu_types.ecore, perf_model_support.second.refs);
    free_fd_l2_percpu();
    return;
    }
    retval = open_perf_counter(cpu, perf_pmu_types.ecore, perf_model_support.second.hits, fd_l2_percpu[cpu], PERF_FORMAT_GROUP);
    if (retval == -1) {
    warnx("%s(cpu%d, 0x%x, 0x%llx) HITS", __func__, cpu, perf_pmu_types.ecore, perf_model_support.second.hits);
    free_fd_l2_percpu();
    return;
    }
    } else if (perf_lcore_set && CPU_ISSET_S(cpu, cpu_possible_setsize, perf_lcore_set)) {
    fd_l2_percpu[cpu] = open_perf_counter(cpu, perf_pmu_types.lcore, perf_model_support.third.refs, -1, PERF_FORMAT_GROUP);
    if (fd_l2_percpu[cpu] == -1) {
    warnx("%s(cpu%d, 0x%x, 0x%llx) REFS", __func__, cpu, perf_pmu_types.lcore, perf_model_support.third.refs);
    free_fd_l2_percpu();
    return;
    }
    retval = open_perf_counter(cpu, perf_pmu_types.lcore, perf_model_support.third.hits, fd_l2_percpu[cpu], PERF_FORMAT_GROUP);
    if (retval == -1) {
    warnx("%s(cpu%d, 0x%x, 0x%llx) HITS", __func__, cpu, perf_pmu_types.lcore, perf_model_support.third.hits);
    free_fd_l2_percpu();
    return;
    }
    } else
    err(-1, "%s: cpu%d: type %d", __func__, cpu, cpus[cpu].type);
    }
    BIC_PRESENT(BIC_L2_MRPS);
    BIC_PRESENT(BIC_L2_HIT);
    }
//
// in /dev/cpu/ return success for names that are numbers
// ie. filter out ".", "..", "microcode".
//
#[no_mangle]
pub unsafe extern "C" fn dir_filter(dirp: *const dirent) -> c_int {
    int dir_filter(const struct dirent *dirp)
    {
    if (isdigit(dirp.d_name[0]))
    return 1;
    else
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn set_thread_siblings(thiscpu: *mut cpu_topology) -> c_int {
    int set_thread_siblings(struct cpu_topology *thiscpu)
    {
    char path[80];
    let mut cpu: c_int = thiscpu.cpu_id;
    size_t size;
    let mut ht_id: c_int = 0;
    int i;
    thiscpu.put_ids = CPU_ALLOC((topo.max_cpu_num + 1));
    if (thiscpu.ht_id < 0)
    thiscpu.ht_id = 0;	/* first CPU in core */
    if (!thiscpu.put_ids)
    return -1;
    size = CPU_ALLOC_SIZE((topo.max_cpu_num + 1));
    CPU_ZERO_S(size, thiscpu.put_ids);
    sprintf(path, "/sys/devices/system/cpu/cpu%d/topology", cpu);
    initialize_cpu_set_from_sysfs(thiscpu.put_ids, path, "thread_siblings_list");
    for (i = 0; i <= topo.max_cpu_num; ++i)
    if (CPU_ISSET_S(i, size, thiscpu.put_ids)) {
    cpus[i].ht_id = ht_id;
    cpus[cpu].ht_sibling_cpu_id[ht_id] = i;
    ht_id += 1;
    }
    return (ht_id - 1);
    }
#[no_mangle]
pub unsafe extern "C" fn topology_probe(startup: bool) {
    void topology_probe(bool startup)
    {
    int i;
    let mut max_core_id: c_int = 0;
    let mut max_package_id: c_int = 0;
    let mut max_siblings: c_int = 0;
// Initialize num_cpus, max_cpu_num
    set_max_cpu_num();
    topo.num_cpus = 0;
    for_all_proc_cpus(count_cpus);
    if (!summary_only)
    BIC_PRESENT(BIC_CPU);
    if (debug > 1)
    fprintf(outf, "num_cpus %d max_cpu_num %d\n", topo.num_cpus, topo.max_cpu_num);
    cpus = calloc(1, (topo.max_cpu_num + 1) * sizeof(struct cpu_topology));
    if (cpus == core::ptr::null_mut())
    err(1, "calloc cpus");
//
// Allocate and initialize cpu_present_set
//
    cpu_present_set = CPU_ALLOC((topo.max_cpu_num + 1));
    if (cpu_present_set == core::ptr::null_mut())
    err(3, "CPU_ALLOC");
    cpu_present_setsize = CPU_ALLOC_SIZE((topo.max_cpu_num + 1));
    CPU_ZERO_S(cpu_present_setsize, cpu_present_set);
    for_all_proc_cpus(mark_cpu_present);
    if (debug)
    print_cpu_set("present set", cpu_present_set);
//
// Allocate and initialize cpu_possible_set
//
    cpu_possible_set = CPU_ALLOC((topo.max_cpu_num + 1));
    if (cpu_possible_set == core::ptr::null_mut())
    err(3, "CPU_ALLOC");
    cpu_possible_setsize = CPU_ALLOC_SIZE((topo.max_cpu_num + 1));
    CPU_ZERO_S(cpu_possible_setsize, cpu_possible_set);
    initialize_cpu_set_from_sysfs(cpu_possible_set, "/sys/devices/system/cpu", "possible");
    if (debug)
    print_cpu_set("possible set", cpu_possible_set);
//
// Allocate and initialize cpu_effective_set
//
    cpu_effective_set = CPU_ALLOC((topo.max_cpu_num + 1));
    if (cpu_effective_set == core::ptr::null_mut())
    err(3, "CPU_ALLOC");
    cpu_effective_setsize = CPU_ALLOC_SIZE((topo.max_cpu_num + 1));
    CPU_ZERO_S(cpu_effective_setsize, cpu_effective_set);
    update_effective_set(startup);
    if (debug)
    print_cpu_set("effective set", cpu_effective_set);
//
// Allocate and initialize cpu_allowed_set
//
    cpu_allowed_set = CPU_ALLOC((topo.max_cpu_num + 1));
    if (cpu_allowed_set == core::ptr::null_mut())
    err(3, "CPU_ALLOC");
    cpu_allowed_setsize = CPU_ALLOC_SIZE((topo.max_cpu_num + 1));
    CPU_ZERO_S(cpu_allowed_setsize, cpu_allowed_set);
//
// Validate and update cpu_allowed_set.
//
// Make sure all cpus in cpu_subset are also in cpu_present_set during startup.
// Give a warning when cpus in cpu_subset become unavailable at runtime.
// Give a warning when cpus are not effective because of cgroup setting.
//
// cpu_allowed_set is the intersection of cpu_present_set/cpu_effective_set/cpu_subset.
//
    for (i = 0; i < CPU_SUBSET_MAXCPUS; ++i) {
    if (cpu_subset && !CPU_ISSET_S(i, cpu_subset_size, cpu_subset))
    continue;
    if (!CPU_ISSET_S(i, cpu_present_setsize, cpu_present_set)) {
    if (cpu_subset) {
// cpus in cpu_subset must be in cpu_present_set during startup
    if (startup)
    err(1, "cpu%d not present", i);
    else
    fprintf(stderr, "cpu%d not present\n", i);
    }
    continue;
    }
    if (CPU_COUNT_S(cpu_effective_setsize, cpu_effective_set)) {
    if (!CPU_ISSET_S(i, cpu_effective_setsize, cpu_effective_set)) {
    fprintf(stderr, "cpu%d not effective\n", i);
    continue;
    }
    }
    CPU_SET_S(i, cpu_allowed_setsize, cpu_allowed_set);
    }
    if (debug)
    print_cpu_set("allowed set", cpu_allowed_set);
    if (!CPU_COUNT_S(cpu_allowed_setsize, cpu_allowed_set))
    err(-ENODEV, "No valid cpus found");
    sched_setaffinity(0, cpu_allowed_setsize, cpu_allowed_set);
//
// Allocate and initialize cpu_affinity_set
//
    cpu_affinity_set = CPU_ALLOC((topo.max_cpu_num + 1));
    if (cpu_affinity_set == core::ptr::null_mut())
    err(3, "CPU_ALLOC");
    cpu_affinity_setsize = CPU_ALLOC_SIZE((topo.max_cpu_num + 1));
    CPU_ZERO_S(cpu_affinity_setsize, cpu_affinity_set);
    for_all_proc_cpus(clear_ht_id);
    for_all_proc_cpus(set_cpu_hybrid_type);
//
// For online cpus
// find max_core_id, max_package_id, num_cores (per system)
//
    topo.min_module_id = 0x7FFFFFFF;
    for (i = 0; i <= topo.max_cpu_num; ++i) {
    int siblings;
    if (cpu_is_not_present(i)) {
    if (debug > 1)
    fprintf(outf, "cpu%d NOT PRESENT\n", i);
    continue;
    }
    cpus[i].cpu_id = i;
// get package information
    cpus[i].package_id = get_package_id(i);
    if (cpus[i].package_id > max_package_id)
    max_package_id = cpus[i].package_id;
// get die information
    cpus[i].die_id = get_die_id(i);
    if (cpus[i].die_id > topo.max_die_id)
    topo.max_die_id = cpus[i].die_id;
// get l3 information
    cpus[i].l3_id = get_l3_id(i);
    if (cpus[i].l3_id > topo.max_l3_id)
    topo.max_l3_id = cpus[i].l3_id;
// get numa node information
    cpus[i].physical_node_id = get_physical_node_id(&cpus[i]);
    if (cpus[i].physical_node_id > topo.max_node_num)
    topo.max_node_num = cpus[i].physical_node_id;
// get module information
    cpus[i].module_id = get_module_id(i);
    if (cpus[i].module_id > topo.max_module_id)
    topo.max_module_id = cpus[i].module_id;
    if (cpus[i].module_id < topo.min_module_id)
    topo.min_module_id = cpus[i].module_id;
// get core information
    cpus[i].core_id = get_core_id(i);
    if (cpus[i].core_id > max_core_id)
    max_core_id = cpus[i].core_id;
// get thread information
    siblings = set_thread_siblings(&cpus[i]);
    if (siblings > max_siblings)
    max_siblings = siblings;
    if (cpus[i].ht_id == 0)
    topo.num_cores++;
    }
    topo.max_core_id = max_core_id;	/* within a package */
    topo.max_package_id = max_package_id;
    topo.cores_per_pkg = max_core_id + 1;
    if (debug > 1)
    fprintf(outf, "max_core_id %d, sizing for %d cores per package\n", max_core_id, topo.cores_per_pkg);
    if (!summary_only)
    BIC_PRESENT(BIC_Core);
    if (debug > 1)
    fprintf(outf, "min_module_id %d max_module_id %d\n", topo.min_module_id, topo.max_module_id);
    if (!summary_only && (topo.min_module_id != topo.max_module_id))
    BIC_PRESENT(BIC_Module);
    topo.num_die = topo.max_die_id + 1;
    if (debug > 1)
    fprintf(outf, "max_die_id %d, sizing for %d die\n", topo.max_die_id, topo.num_die);
    if (!summary_only && topo.num_die > 1)
    BIC_PRESENT(BIC_Die);
    if (!summary_only && topo.max_l3_id > 0)
    BIC_PRESENT(BIC_L3);
    topo.num_packages = max_package_id + 1;
    if (debug > 1)
    fprintf(outf, "max_package_id %d, sizing for %d packages\n", max_package_id, topo.num_packages);
    if (!summary_only && topo.num_packages > 1)
    BIC_PRESENT(BIC_Package);
    set_node_data();
    if (debug > 1)
    fprintf(outf, "nodes_per_pkg %d\n", topo.nodes_per_pkg);
    if (!summary_only && topo.nodes_per_pkg > 1)
    BIC_PRESENT(BIC_Node);
    topo.threads_per_core = max_siblings;
    if (debug > 1)
    fprintf(outf, "max_siblings %d\n", max_siblings);
    if (debug < 1)
    return;
    for (i = 0; i <= topo.max_cpu_num; ++i) {
    int ht_id;
    if (cpu_is_not_present(i))
    continue;
    fprintf(outf,
    "cpu %d pkg %d die %d l3 %d node %d lnode %d module 0x%x core %d ht_id %d",
    i, cpus[i].package_id, cpus[i].die_id, cpus[i].l3_id,
    cpus[i].physical_node_id, cpus[i].logical_node_id, cpus[i].module_id, cpus[i].core_id, cpus[i].ht_id);
    fprintf(outf, " siblings");
    for (ht_id = 0; ht_id <= MAX_HT_ID; ++ht_id)
    fprintf(outf, " %d", cpus[i].ht_sibling_cpu_id[ht_id]);
    fprintf(outf, "\n");
    }
    }
#[no_mangle]
pub unsafe extern "C" fn allocate_counters_1(counters: *mut counters) {
    void allocate_counters_1(struct counters *counters)
    {
    counters.threads = calloc(1, sizeof(struct thread_data));
    if (counters.threads == core::ptr::null_mut())
    goto error;
    counters.cores = calloc(1, sizeof(struct core_data));
    if (counters.cores == core::ptr::null_mut())
    goto error;
    counters.packages = calloc(1, sizeof(struct pkg_data));
    if (counters.packages == core::ptr::null_mut())
    goto error;
    return;
    error:
    err(1, "calloc counters_1");
    }
#[no_mangle]
pub unsafe extern "C" fn allocate_counters(counters: *mut counters) {
    void allocate_counters(struct counters *counters)
    {
    int i;
    let mut num_cores: c_int = topo.cores_per_pkg * topo.num_packages;
    counters.threads = calloc(topo.max_cpu_num + 1, sizeof(struct thread_data));
    if (counters.threads == core::ptr::null_mut())
    goto error;
    for (i = 0; i < topo.max_cpu_num + 1; i++)
    (counters.threads)[i].cpu_id = -1;
    counters.cores = calloc(num_cores, sizeof(struct core_data));
    if (counters.cores == core::ptr::null_mut())
    goto error;
    for (i = 0; i < num_cores; i++)
    (counters.cores)[i].first_cpu = -1;
    counters.packages = calloc(topo.num_packages, sizeof(struct pkg_data));
    if (counters.packages == core::ptr::null_mut())
    goto error;
    for (i = 0; i < topo.num_packages; i++)
    (counters.packages)[i].first_cpu = -1;
    return;
    error:
    err(1, "calloc counters");
    }
//
// init_counter()
//
// set t->cpu_id, FIRST_THREAD_IN_CORE and FIRST_CORE_IN_PACKAGE
//
#[no_mangle]
pub unsafe extern "C" fn init_counter(thread_base: *mut thread_data, core_base: *mut core_data, pkg_base: *mut pkg_data, cpu_id: c_int) {
    void init_counter(struct thread_data *thread_base, struct core_data *core_base, struct pkg_data *pkg_base, int cpu_id)
    {
    let mut pkg_id: c_int = cpus[cpu_id].package_id;
    let mut node_id: c_int = cpus[cpu_id].logical_node_id;
    let mut core_id: c_int = cpus[cpu_id].core_id;
    struct thread_data *t;
    struct core_data *c;
// Workaround for systems where physical_node_id==-1
// and logical_node_id==(-1 - topo.num_cpus)
//
    if (node_id < 0)
    node_id = 0;
    t = &thread_base[cpu_id];
    c = &core_base[GLOBAL_CORE_ID(core_id, pkg_id)];
    t.cpu_id = cpu_id;
    if (!cpu_is_not_allowed(cpu_id)) {
    if (c.first_cpu < 0)
    c.first_cpu = t.cpu_id;
    if (pkg_base[pkg_id].first_cpu < 0)
    pkg_base[pkg_id].first_cpu = t.cpu_id;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn initialize_counters(cpu_id: c_int) -> c_int {
    int initialize_counters(int cpu_id)
    {
    init_counter(EVEN_COUNTERS, cpu_id);
    init_counter(ODD_COUNTERS, cpu_id);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn allocate_output_buffer() {
    void allocate_output_buffer()
    {
    output_buffer = calloc(1, (1 + topo.num_cpus) * 2048);
    outp = output_buffer;
    if (outp == core::ptr::null_mut())
    err(-1, "calloc output buffer");
    }
#[no_mangle]
pub unsafe extern "C" fn allocate_fd_percpu() {
    void allocate_fd_percpu(void)
    {
    fd_percpu = calloc(topo.max_cpu_num + 1, sizeof(int));
    if (fd_percpu == core::ptr::null_mut())
    err(-1, "calloc fd_percpu");
    }
#[no_mangle]
pub unsafe extern "C" fn allocate_irq_buffers() {
    void allocate_irq_buffers(void)
    {
    irq_column_2_cpu = calloc(topo.num_cpus, sizeof(int));
    if (irq_column_2_cpu == core::ptr::null_mut())
    err(-1, "calloc %d", topo.num_cpus);
    irqs_per_cpu = calloc(topo.max_cpu_num + 1, sizeof(int));
    if (irqs_per_cpu == core::ptr::null_mut())
    err(-1, "calloc %d IRQ", topo.max_cpu_num + 1);
    nmi_per_cpu = calloc(topo.max_cpu_num + 1, sizeof(int));
    if (nmi_per_cpu == core::ptr::null_mut())
    err(-1, "calloc %d NMI", topo.max_cpu_num + 1);
    }
#[no_mangle]
pub unsafe extern "C" fn update_topo(_arg: PER_THREAD_PARAMS) -> c_int {
    int update_topo(PER_THREAD_PARAMS)
    {
    topo.allowed_cpus++;
    if ((int)t.cpu_id == c.first_cpu)
    topo.allowed_cores++;
    if ((int)t.cpu_id == p.first_cpu)
    topo.allowed_packages++;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn topology_update() {
    void topology_update(void)
    {
    topo.allowed_cpus = 0;
    topo.allowed_cores = 0;
    topo.allowed_packages = 0;
    for_all_cpus(update_topo, ODD_COUNTERS);
    if (debug)
    fprintf(stderr, "allowed_cpus %d allowed_cores %d allowed_packages %d\n", topo.allowed_cpus, topo.allowed_cores, topo.allowed_packages);
    }
#[no_mangle]
pub unsafe extern "C" fn setup_all_buffers(startup: bool) {
    void setup_all_buffers(bool startup)
    {
    topology_probe(startup);
    allocate_irq_buffers();
    allocate_fd_percpu();
    allocate_counters_1(&average);
    allocate_counters(&even);
    allocate_counters(&odd);
    allocate_output_buffer();
    for_all_proc_cpus(initialize_counters);
    topology_update();
    }
#[no_mangle]
pub unsafe extern "C" fn set_master_cpu() {
    void set_master_cpu(void)
    {
    int i;
    for (i = 0; i < topo.max_cpu_num + 1; ++i) {
    if (cpu_is_not_allowed(i))
    continue;
    master_cpu = i;
    if (debug > 1)
    fprintf(outf, "master_cpu = %d\n", master_cpu);
    return;
    }
    err(-ENODEV, "No valid cpus found");
    }
#[no_mangle]
pub unsafe extern "C" fn has_added_counters() -> bool {
    bool has_added_counters(void)
    {
//
// It only makes sense to call this after the command line is parsed,
// otherwise sys structure is not populated.
//
    return sys.added_core_counters | sys.added_thread_counters | sys.added_package_counters;
    }
#[no_mangle]
pub unsafe extern "C" fn check_msr_access() {
    void check_msr_access(void)
    {
    check_msr_driver();
    check_msr_permission();
    if (no_msr)
    bic_disable_msr_access();
    }
#[no_mangle]
pub unsafe extern "C" fn check_perf_access() {
    void check_perf_access(void)
    {
    if (BIC_IS_ENABLED(BIC_IPC))
    if (!has_perf_instr_count_access())
    no_perf = 1;
    if (BIC_IS_ENABLED(BIC_LLC_MRPS) || BIC_IS_ENABLED(BIC_LLC_HIT))
    if (!has_perf_llc_access())
    no_perf = 1;
    if (no_perf)
    bic_disable_perf_access();
    }
#[no_mangle]
pub unsafe extern "C" fn perf_has_hybrid_devices() -> bool {
    bool perf_has_hybrid_devices(void)
    {
//
// 0: unknown
// 1: has separate perf device for p and e core
// -1: doesn't have separate perf device for p and e core
//
    static int cached;
    if (cached > 0)
    return true;
    if (cached < 0)
    return false;
    if (access("/sys/bus/event_source/devices/cpu_core", F_OK)) {
    cached = -1;
    return false;
    }
    if (access("/sys/bus/event_source/devices/cpu_atom", F_OK)) {
    cached = -1;
    return false;
    }
    cached = 1;
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn added_perf_counters_init_(pinfo: *mut perf_counter_info) -> c_int {
    int added_perf_counters_init_(struct perf_counter_info *pinfo)
    {
    let mut num_domains: usize = 0;
    unsigned int next_domain;
    bool *domain_visited;
    unsigned int perf_type, perf_config;
    double perf_scale;
    int fd_perf;
    if (!pinfo)
    return 0;
    let mut max_num_domains: usize = MAX(topo.max_cpu_num + 1, MAX(topo.max_core_id + 1, topo.max_package_id + 1));
    domain_visited = calloc(max_num_domains, sizeof(*domain_visited));
    while (pinfo) {
    switch (pinfo.scope) {
    case SCOPE_CPU:
    num_domains = topo.max_cpu_num + 1;
    break;
    case SCOPE_CORE:
    num_domains = topo.max_core_id + 1;
    break;
    case SCOPE_PACKAGE:
    num_domains = topo.max_package_id + 1;
    break;
    }
// Allocate buffer for file descriptor for each domain.
    pinfo.fd_perf_per_domain = calloc(num_domains, sizeof(*pinfo.fd_perf_per_domain));
    if (!pinfo.fd_perf_per_domain)
    errx(1, "%s: alloc %s", __func__, "fd_perf_per_domain");
    for (size_t i = 0; i < num_domains; ++i)
    pinfo.fd_perf_per_domain[i] = -1;
    pinfo.num_domains = num_domains;
    pinfo.scale = 1.0;
    memset(domain_visited, 0, max_num_domains * sizeof(*domain_visited));
    for (int cpu = 0; cpu < topo.max_cpu_num + 1; ++cpu) {
    next_domain = cpu_to_domain(pinfo, cpu);
    assert(next_domain < num_domains);
    if (cpu_is_not_allowed(cpu))
    continue;
    if (domain_visited[next_domain])
    continue;
//
// Intel hybrid platforms expose different perf devices for P and E cores.
// Instead of one, "/sys/bus/event_source/devices/cpu" device, there are
// "/sys/bus/event_source/devices/{cpu_core,cpu_atom}".
//
// This makes it more complicated to the user, because most of the counters
// are available on both and have to be handled manually, otherwise.
//
// Code below, allow user to use the old "cpu" name, which is translated accordingly.
//
    const char *perf_device = pinfo.device;
    if (strcmp(perf_device, "cpu") == 0 && perf_has_hybrid_devices()) {
    switch (cpus[cpu].type) {
    case INTEL_PCORE_TYPE:
    perf_device = "cpu_core";
    break;
    case INTEL_ECORE_TYPE:
    perf_device = "cpu_atom";
    break;
    default:	/* Don't change, we will probably fail and report a problem soon. */
    break;
    }
    }
    perf_type = read_perf_type(perf_device);
    if (perf_type == (unsigned int)-1) {
    warnx("%s: perf/%s/%s: failed to read %s", __func__, perf_device, pinfo.event, "type");
    continue;
    }
    perf_config = read_perf_config(perf_device, pinfo.event);
    if (perf_config == (unsigned int)-1) {
    warnx("%s: perf/%s/%s: failed to read %s", __func__, perf_device, pinfo.event, "config");
    continue;
    }
// Scale is not required, some counters just don't have it.
    perf_scale = read_perf_scale(perf_device, pinfo.event);
    if (perf_scale == 0.0)
    perf_scale = 1.0;
    fd_perf = open_perf_counter(cpu, perf_type, perf_config, -1, 0);
    if (fd_perf == -1) {
    warnx("%s: perf/%s/%s: failed to open counter on cpu%d", __func__, perf_device, pinfo.event, cpu);
    continue;
    }
    domain_visited[next_domain] = 1;
    pinfo.fd_perf_per_domain[next_domain] = fd_perf;
    pinfo.scale = perf_scale;
    if (debug)
    fprintf(stderr, "Add perf/%s/%s cpu%d: %d\n", perf_device, pinfo.event, cpu, pinfo.fd_perf_per_domain[next_domain]);
    }
    pinfo = pinfo.next;
    }
    free(domain_visited);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn added_perf_counters_init() {
    void added_perf_counters_init(void)
    {
    if (added_perf_counters_init_(sys.perf_tp))
    errx(1, "%s: %s", __func__, "thread");
    if (added_perf_counters_init_(sys.perf_cp))
    errx(1, "%s: %s", __func__, "core");
    if (added_perf_counters_init_(sys.perf_pp))
    errx(1, "%s: %s", __func__, "package");
    }
#[no_mangle]
pub unsafe extern "C" fn parse_telem_info_file(fd_dir: c_int, info_filename: *const c_char, format: *const c_char, output: *mut c_ulong) -> c_int {
    int parse_telem_info_file(int fd_dir, const char *info_filename, const char *format, unsigned long *output)
    {
    int fd_telem_info;
    FILE *file_telem_info;
    unsigned long value;
    fd_telem_info = openat(fd_dir, info_filename, O_RDONLY);
    if (fd_telem_info == -1)
    return -1;
    file_telem_info = fdopen(fd_telem_info, "r");
    if (file_telem_info == core::ptr::null_mut()) {
    close(fd_telem_info);
    return -1;
    }
    if (fscanf(file_telem_info, format, &value) != 1) {
    fclose(file_telem_info);
    return -1;
    }
    fclose(file_telem_info);
// output = value;
    return 0;
    }
    struct pmt_mmio *pmt_mmio_open(unsigned int target_guid)
    {
    struct pmt_diriter_t pmt_iter;
    const struct dirent *entry;
    struct stat st;
    int fd_telem_dir, fd_pmt;
    unsigned long guid, size, offset;
    size_t mmap_size;
    void *mmio;
    struct pmt_mmio *head = core::ptr::null_mut(), *last = core::ptr::null_mut();
    struct pmt_mmio *new_pmt = core::ptr::null_mut();
    if (stat(SYSFS_TELEM_PATH, &st) == -1)
    return core::ptr::null_mut();
    pmt_diriter_init(&pmt_iter);
    entry = pmt_diriter_begin(&pmt_iter, SYSFS_TELEM_PATH);
    if (!entry) {
    pmt_diriter_remove(&pmt_iter);
    return core::ptr::null_mut();
    }
    for (; entry != core::ptr::null_mut(); entry = pmt_diriter_next(&pmt_iter)) {
    if (fstatat(dirfd(pmt_iter.dir), entry.d_name, &st, 0) == -1)
    break;
    if (!S_ISDIR(st.st_mode))
    continue;
    fd_telem_dir = openat(dirfd(pmt_iter.dir), entry.d_name, O_RDONLY);
    if (fd_telem_dir == -1)
    break;
    if (parse_telem_info_file(fd_telem_dir, "guid", "%lx", &guid)) {
    close(fd_telem_dir);
    break;
    }
    if (parse_telem_info_file(fd_telem_dir, "size", "%lu", &size)) {
    close(fd_telem_dir);
    break;
    }
    if (guid != target_guid) {
    close(fd_telem_dir);
    continue;
    }
    if (parse_telem_info_file(fd_telem_dir, "offset", "%lu", &offset)) {
    close(fd_telem_dir);
    break;
    }
    assert(offset == 0);
    fd_pmt = openat(fd_telem_dir, "telem", O_RDONLY);
    if (fd_pmt == -1)
    goto loop_cleanup_and_break;
    mmap_size = ROUND_UP_TO_PAGE_SIZE(size);
    mmio = mmap(0, mmap_size, PROT_READ, MAP_SHARED, fd_pmt, 0);
    if (mmio != MAP_FAILED) {
    if (debug)
    fprintf(stderr, "%s: 0x%lx mmaped at: %p\n", __func__, guid, mmio);
    new_pmt = calloc(1, sizeof(*new_pmt));
    if (!new_pmt) {
    fprintf(stderr, "%s: Failed to allocate pmt_mmio\n", __func__);
    exit(1);
    }
//
// Create linked list of mmaped regions,
// but preserve the ordering from sysfs.
// Ordering is important for the user to
// use the seq=%u parameter when adding a counter.
//
    new_pmt.guid = guid;
    new_pmt.mmio_base = mmio;
    new_pmt.pmt_offset = offset;
    new_pmt.size = size;
    new_pmt.next = pmt_mmios;
    if (last)
    last.next = new_pmt;
    else
    head = new_pmt;
    last = new_pmt;
    }
    loop_cleanup_and_break:
    close(fd_pmt);
    close(fd_telem_dir);
    }
    pmt_diriter_remove(&pmt_iter);
//
// If we found something, stick just
// created linked list to the front.
//
    if (head)
    pmt_mmios = head;
    return head;
    }
    struct pmt_mmio *pmt_mmio_find(unsigned int guid)
    {
    struct pmt_mmio *pmmio = pmt_mmios;
    while (pmmio) {
    if (pmmio.guid == guid)
    return pmmio;
    pmmio = pmmio.next;
    }
    return core::ptr::null_mut();
    }
    void *pmt_get_counter_pointer(struct pmt_mmio *pmmio, unsigned long counter_offset)
    {
    char *ret;
// Get base of mmaped PMT file.
    ret = (char *)pmmio.mmio_base;
//
// Apply PMT MMIO offset to obtain beginning of the mmaped telemetry data.
// It's not guaranteed that the mmaped memory begins with the telemetry data
// - we might have to apply the offset first.
//
    ret += pmmio.pmt_offset;
// Apply the counter offset to get the address to the mmaped counter.
    ret += counter_offset;
    return ret;
    }
    struct pmt_mmio *pmt_add_guid(unsigned int guid, unsigned int seq)
    {
    struct pmt_mmio *ret;
    ret = pmt_mmio_find(guid);
    if (!ret)
    ret = pmt_mmio_open(guid);
    while (ret && seq) {
    ret = ret.next;
    --seq;
    }
    return ret;
    }
    enum pmt_open_mode {
    PMT_OPEN_TRY,		/* Open failure is not an error. */
    PMT_OPEN_REQUIRED,	/* Open failure is a fatal error. */
    };
    struct pmt_counter *pmt_find_counter(struct pmt_counter *pcounter, const char *name)
    {
    while (pcounter) {
    if (strcmp(pcounter.name, name) == 0)
    break;
    pcounter = pcounter.next;
    }
    return pcounter;
    }
    struct pmt_counter **pmt_get_scope_root(enum counter_scope scope)
    {
    switch (scope) {
    case SCOPE_CPU:
    return &sys.pmt_tp;
    case SCOPE_CORE:
    return &sys.pmt_cp;
    case SCOPE_PACKAGE:
    return &sys.pmt_pp;
    }
    __builtin_unreachable();
    }
#[no_mangle]
pub unsafe extern "C" fn pmt_counter_add_domain(pcounter: *mut pmt_counter, pmmio: *mut c_ulong, domain_id: c_uint) {
    void pmt_counter_add_domain(struct pmt_counter *pcounter, unsigned long *pmmio, unsigned int domain_id)
    {
// Make sure the new domain fits.
    if (domain_id >= pcounter.num_domains)
    pmt_counter_resize(pcounter, domain_id + 1);
    assert(pcounter.domains);
    assert(domain_id < pcounter.num_domains);
    pcounter.domains[domain_id].pcounter = pmmio;
    }
    int pmt_add_counter(unsigned int guid, unsigned int seq, const char *name, enum pmt_datatype type,
    unsigned int lsb, unsigned int msb, unsigned int offset, enum counter_scope scope,
    enum counter_format format, unsigned int domain_id, enum pmt_open_mode mode)
    {
    struct pmt_mmio *mmio;
    struct pmt_counter *pcounter;
    let mut pmt_root: *mut *mut pmt_counter const = pmt_get_scope_root(scope);
    let mut new_counter: bool = false;
    let mut conflict: c_int = 0;
    if (lsb > msb) {
    fprintf(stderr, "%s: %s: `%s` must be satisfied\n", __func__, "lsb <= msb", name);
    exit(1);
    }
    if (msb >= 64) {
    fprintf(stderr, "%s: %s: `%s` must be satisfied\n", __func__, "msb < 64", name);
    exit(1);
    }
    mmio = pmt_add_guid(guid, seq);
    if (!mmio) {
    if (mode != PMT_OPEN_TRY) {
    fprintf(stderr, "%s: failed to map PMT MMIO for guid %x, seq %u\n", __func__, guid, seq);
    exit(1);
    }
    return 1;
    }
    if (offset >= mmio.size) {
    if (mode != PMT_OPEN_TRY) {
    fprintf(stderr, "%s: offset %u outside of PMT MMIO size %u\n", __func__, offset, mmio.size);
    exit(1);
    }
    return 1;
    }
    pcounter = pmt_find_counter(*pmt_root, name);
    if (!pcounter) {
    pcounter = calloc(1, sizeof(*pcounter));
    new_counter = true;
    }
    if (new_counter) {
    strncpy(pcounter.name, name, ARRAY_SIZE(pcounter.name) - 1);
    pcounter.type = type;
    pcounter.scope = scope;
    pcounter.lsb = lsb;
    pcounter.msb = msb;
    pcounter.format = format;
    } else {
    conflict += pcounter.type != type;
    conflict += pcounter.scope != scope;
    conflict += pcounter.lsb != lsb;
    conflict += pcounter.msb != msb;
    conflict += pcounter.format != format;
    }
    if (conflict) {
    fprintf(stderr, "%s: conflicting parameters for the PMT counter with the same name %s\n", __func__, name);
    exit(1);
    }
    pmt_counter_add_domain(pcounter, pmt_get_counter_pointer(mmio, offset), domain_id);
    if (new_counter) {
    pcounter.next = *pmt_root;
// pmt_root = pcounter;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn pmt_init() {
    void pmt_init(void)
    {
    int cpu_num;
    unsigned long seq, offset, mod_num;
    if (BIC_IS_ENABLED(BIC_Diec6)) {
    pmt_add_counter(PMT_MTL_DC6_GUID, PMT_MTL_DC6_SEQ, "Die%c6", PMT_TYPE_XTAL_TIME,
    PMT_COUNTER_MTL_DC6_LSB, PMT_COUNTER_MTL_DC6_MSB, PMT_COUNTER_MTL_DC6_OFFSET, SCOPE_PACKAGE, FORMAT_DELTA, 0, PMT_OPEN_TRY);
    }
    if (BIC_IS_ENABLED(BIC_CPU_c1e)) {
    seq = 0;
    offset = PMT_COUNTER_CWF_MC1E_OFFSET_BASE;
    mod_num = 0;	/* Relative module number for current PMT file. */
// Open the counter for each CPU.
    for (cpu_num = 0; cpu_num < topo.max_cpu_num;) {
    if (cpu_is_not_allowed(cpu_num))
    goto next_loop_iter;
//
// Set the scope to CPU, even though CWF report the counter per module.
// CPUs inside the same module will read from the same location, instead of reporting zeros.
//
// CWF with newer firmware might require a PMT_TYPE_XTAL_TIME intead of PMT_TYPE_TCORE_CLOCK.
//
    pmt_add_counter(PMT_CWF_MC1E_GUID, seq, "CPU%c1e", PMT_TYPE_TCORE_CLOCK,
    PMT_COUNTER_CWF_MC1E_LSB, PMT_COUNTER_CWF_MC1E_MSB, offset, SCOPE_CPU, FORMAT_DELTA, cpu_num, PMT_OPEN_TRY);
//
// Rather complex logic for each time we go to the next loop iteration,
// so keep it as a label.
//
    next_loop_iter:
//
// Advance the cpu number and check if we should also advance offset to
// the next counter inside the PMT file.
//
// On Clearwater Forest platform, the counter is reported per module,
// so open the same counter for all of the CPUs inside the module.
// That way, reported table show the correct value for all of the CPUs inside the module,
// instead of zeros.
//
    ++cpu_num;
    if (cpu_num % PMT_COUNTER_CWF_CPUS_PER_MODULE == 0) {
    offset += PMT_COUNTER_CWF_MC1E_OFFSET_INCREMENT;
    ++mod_num;
    }
//
// There are PMT_COUNTER_CWF_MC1E_NUM_MODULES_PER_FILE in each PMT file.
//
// If that number is reached, seq must be incremented to advance to the next file in a sequence.
// Offset inside that file and a module counter has to be reset.
//
    if (mod_num == PMT_COUNTER_CWF_MC1E_NUM_MODULES_PER_FILE) {
    ++seq;
    offset = PMT_COUNTER_CWF_MC1E_OFFSET_BASE;
    mod_num = 0;
    }
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn turbostat_init() {
    void turbostat_init()
    {
    setup_all_buffers(true);
    set_master_cpu();
    check_msr_access();
    check_perf_access();
    process_cpuid();
    counter_info_init();
    probe_pm_features();
    msr_perf_init();
    linux_perf_init();
    rapl_perf_init();
    cstate_perf_init();
    perf_llc_init();
    perf_l2_init();
    added_perf_counters_init();
    pmt_init();
    for_all_cpus(get_cpu_type, ODD_COUNTERS);
    for_all_cpus(get_cpu_type, EVEN_COUNTERS);
    if (BIC_IS_ENABLED(BIC_IPC) && has_aperf_access && get_instr_count_fd(master_cpu) != -1)
    BIC_PRESENT(BIC_IPC);
//
// If TSC tweak is needed, but couldn't get it,
// disable more BICs, since it can't be reported accurately.
//
    if (platform.enable_tsc_tweak && !has_base_hz) {
    CLR_BIC(BIC_Busy, &bic_enabled);
    CLR_BIC(BIC_Bzy_MHz, &bic_enabled);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn affinitize_child() {
    void affinitize_child(void)
    {
// Prefer cpu_possible_set, if available
    if (sched_setaffinity(0, cpu_possible_setsize, cpu_possible_set)) {
    warn("sched_setaffinity cpu_possible_set");
// Otherwise, allow child to run on same cpu set as turbostat
    if (sched_setaffinity(0, cpu_allowed_setsize, cpu_allowed_set))
    warn("sched_setaffinity cpu_allowed_set");
    }
    }
#[no_mangle]
pub unsafe extern "C" fn fork_it(argv: *mut c_char) -> c_int {
    int fork_it(char **argv)
    {
    pid_t child_pid;
    int status;
    snapshot_proc_sysfs_files();
    status = for_all_cpus(get_counters, EVEN_COUNTERS);
    first_counter_read = 0;
    if (status)
    exit(status);
    gettimeofday(&tv_even, (struct timezone *)core::ptr::null_mut());
    child_pid = fork();
    if (!child_pid) {
// child
    affinitize_child();
    execvp(argv[0], argv);
    err(errno, "exec %s", argv[0]);
    } else {
// parent
    if (child_pid == -1)
    err(1, "fork");
    signal(SIGINT, SIG_IGN);
    signal(SIGQUIT, SIG_IGN);
    if (waitpid(child_pid, &status, 0) == -1)
    err(status, "waitpid");
    if (WIFEXITED(status))
    status = WEXITSTATUS(status);
    }
//
// n.b. fork_it() does not check for errors from for_all_cpus()
// because re-starting is problematic when forking
//
    snapshot_proc_sysfs_files();
    for_all_cpus(get_counters, ODD_COUNTERS);
    gettimeofday(&tv_odd, (struct timezone *)core::ptr::null_mut());
    timersub(&tv_odd, &tv_even, &tv_delta);
    if (for_all_cpus_2(delta_cpu, ODD_COUNTERS, EVEN_COUNTERS))
    fprintf(outf, "%s: Counter reset detected\n", progname);
    delta_platform(&platform_counters_odd, &platform_counters_even);
    compute_average(EVEN_COUNTERS);
    format_all_counters(EVEN_COUNTERS);
    fprintf(outf, "%.6f sec\n", tv_delta.tv_sec + tv_delta.tv_usec / 1000000.0);
    flush_output_stderr();
    return status;
    }
#[no_mangle]
pub unsafe extern "C" fn get_and_dump_counters() -> c_int {
    int get_and_dump_counters(void)
    {
    int status;
    snapshot_proc_sysfs_files();
    status = for_all_cpus(get_counters, ODD_COUNTERS);
    if (status)
    return status;
    status = for_all_cpus(dump_counters, ODD_COUNTERS);
    if (status)
    return status;
    flush_output_stdout();
    return status;
    }
#[no_mangle]
pub unsafe extern "C" fn print_version() {
    void print_version()
    {
    fprintf(outf, "turbostat version 2026.04.21 - Len Brown <lenb@kernel.org>\n");
    }
pub const COMMAND_LINE_SIZE: c_int = 2048;
#[no_mangle]
pub unsafe extern "C" fn print_bootcmd() {
    void print_bootcmd(void)
    {
    char bootcmd[COMMAND_LINE_SIZE];
    FILE *fp;
    int ret;
    memset(bootcmd, 0, COMMAND_LINE_SIZE);
    fp = fopen("/proc/cmdline", "r");
    if (!fp)
    return;
    ret = fread(bootcmd, sizeof(char), COMMAND_LINE_SIZE - 1, fp);
    if (ret) {
    bootcmd[ret] = '\0';
// the last character is already '\n'
    fprintf(outf, "Kernel command line: %s", bootcmd);
    }
    fclose(fp);
    }
    struct msr_counter *find_msrp_by_name(struct msr_counter *head, char *name)
    {
    struct msr_counter *mp;
    for (mp = head; mp; mp = mp.next) {
    if (debug)
    fprintf(stderr, "%s: %s %s\n", __func__, name, mp.name);
    if (!strcmp(name, mp.name))
    return mp;
    }
    return core::ptr::null_mut();
    }
    int add_counter(unsigned int msr_num, char *path, char *name,
    unsigned int width, enum counter_scope scope, enum counter_type type, enum counter_format format, int flags, int id)
    {
    struct msr_counter *msrp;
    if (no_msr && msr_num)
    errx(1, "Requested MSR counter 0x%x, but in --no-msr mode", msr_num);
    if (debug)
    fprintf(stderr, "%s(msr%d, %s, %s, width%d, scope%d, type%d, format%d, flags%x, id%d)\n",
    __func__, msr_num, path, name, width, scope, type, format, flags, id);
    switch (scope) {
    case SCOPE_CPU:
    msrp = find_msrp_by_name(sys.tp, name);
    if (msrp) {
    if (debug)
    fprintf(stderr, "%s: %s FOUND\n", __func__, name);
    break;
    }
    if (sys.added_thread_counters++ >= MAX_ADDED_THREAD_COUNTERS) {
    warnx("ignoring thread counter %s", name);
    return -1;
    }
    break;
    case SCOPE_CORE:
    msrp = find_msrp_by_name(sys.cp, name);
    if (msrp) {
    if (debug)
    fprintf(stderr, "%s: %s FOUND\n", __func__, name);
    break;
    }
    if (sys.added_core_counters++ >= MAX_ADDED_CORE_COUNTERS) {
    warnx("ignoring core counter %s", name);
    return -1;
    }
    break;
    case SCOPE_PACKAGE:
    msrp = find_msrp_by_name(sys.pp, name);
    if (msrp) {
    if (debug)
    fprintf(stderr, "%s: %s FOUND\n", __func__, name);
    break;
    }
    if (sys.added_package_counters++ >= MAX_ADDED_PACKAGE_COUNTERS) {
    warnx("ignoring package counter %s", name);
    return -1;
    }
    break;
    default:
    warnx("ignoring counter %s with unknown scope", name);
    return -1;
    }
    if (msrp == core::ptr::null_mut()) {
    msrp = calloc(1, sizeof(struct msr_counter));
    if (msrp == core::ptr::null_mut())
    err(-1, "calloc msr_counter");
    msrp.msr_num = msr_num;
    strncpy(msrp.name, name, NAME_BYTES - 1);
    msrp.width = width;
    msrp.type = type;
    msrp.format = format;
    msrp.flags = flags;
    switch (scope) {
    case SCOPE_CPU:
    msrp.next = sys.tp;
    sys.tp = msrp;
    break;
    case SCOPE_CORE:
    msrp.next = sys.cp;
    sys.cp = msrp;
    break;
    case SCOPE_PACKAGE:
    msrp.next = sys.pp;
    sys.pp = msrp;
    break;
    }
    }
    if (path) {
    struct sysfs_path *sp;
    sp = calloc(1, sizeof(struct sysfs_path));
    if (sp == core::ptr::null_mut()) {
    perror("calloc");
    exit(1);
    }
    strncpy(sp.path, path, PATH_BYTES - 1);
    sp.id = id;
    sp.next = msrp.sp;
    msrp.sp = sp;
    }
    return 0;
    }
//
// Initialize the fields used for identifying and opening the counter.
//
// Defer the initialization of any runtime buffers for actually reading
// the counters for when we initialize all perf counters, so we can later
// easily call re_initialize().
//
    struct perf_counter_info *make_perf_counter_info(const char *perf_device,
    const char *perf_event,
    const char *name,
    unsigned int width, enum counter_scope scope, enum counter_type type, enum counter_format format)
    {
    struct perf_counter_info *pinfo;
    pinfo = calloc(1, sizeof(*pinfo));
    if (!pinfo)
    errx(1, "%s: Failed to allocate %s/%s\n", __func__, perf_device, perf_event);
    strncpy(pinfo.device, perf_device, ARRAY_SIZE(pinfo.device) - 1);
    strncpy(pinfo.event, perf_event, ARRAY_SIZE(pinfo.event) - 1);
    strncpy(pinfo.name, name, ARRAY_SIZE(pinfo.name) - 1);
    pinfo.width = width;
    pinfo.scope = scope;
    pinfo.type = type;
    pinfo.format = format;
    return pinfo;
    }
    int add_perf_counter(const char *perf_device, const char *perf_event, const char *name_buffer, unsigned int width,
    enum counter_scope scope, enum counter_type type, enum counter_format format)
    {
    struct perf_counter_info *pinfo;
    switch (scope) {
    case SCOPE_CPU:
    if (sys.added_thread_perf_counters >= MAX_ADDED_THREAD_COUNTERS) {
    warnx("ignoring thread counter perf/%s/%s", perf_device, perf_event);
    return -1;
    }
    break;
    case SCOPE_CORE:
    if (sys.added_core_perf_counters >= MAX_ADDED_CORE_COUNTERS) {
    warnx("ignoring core counter perf/%s/%s", perf_device, perf_event);
    return -1;
    }
    break;
    case SCOPE_PACKAGE:
    if (sys.added_package_perf_counters >= MAX_ADDED_PACKAGE_COUNTERS) {
    warnx("ignoring package counter perf/%s/%s", perf_device, perf_event);
    return -1;
    }
    break;
    }
    pinfo = make_perf_counter_info(perf_device, perf_event, name_buffer, width, scope, type, format);
    if (!pinfo)
    return -1;
    switch (scope) {
    case SCOPE_CPU:
    pinfo.next = sys.perf_tp;
    sys.perf_tp = pinfo;
    ++sys.added_thread_perf_counters;
    break;
    case SCOPE_CORE:
    pinfo.next = sys.perf_cp;
    sys.perf_cp = pinfo;
    ++sys.added_core_perf_counters;
    break;
    case SCOPE_PACKAGE:
    pinfo.next = sys.perf_pp;
    sys.perf_pp = pinfo;
    ++sys.added_package_perf_counters;
    break;
    }
// FIXME: we might not have debug here yet
    if (debug)
    fprintf(stderr, "%s: %s/%s, name: %s, scope%d\n", __func__, pinfo.device, pinfo.event, pinfo.name, pinfo.scope);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn parse_add_command_msr(add_command: *mut c_char) {
    void parse_add_command_msr(char *add_command)
    {
    let mut msr_num: c_int = 0;
    char *path = core::ptr::null_mut();
    char perf_device[PERF_DEV_NAME_BYTES] = "";
    char perf_event[PERF_EVT_NAME_BYTES] = "";
    char name_buffer[PERF_NAME_BYTES] = "";
    let mut width: c_int = 64;
    let mut fail: c_int = 0;
    let mut scope: enum counter_scope = SCOPE_CPU;
    let mut type: enum counter_type = COUNTER_CYCLES;
    let mut format: enum counter_format = FORMAT_DELTA;
    while (add_command) {
    if (sscanf(add_command, "msr0x%x", &msr_num) == 1)
    goto next;
    if (sscanf(add_command, "msr%d", &msr_num) == 1)
    goto next;
    BUILD_BUG_ON(ARRAY_SIZE(perf_device) <= 31);
    BUILD_BUG_ON(ARRAY_SIZE(perf_event) <= 31);
    if (sscanf(add_command, "perf/%31[^/]/%31[^,]", &perf_device[0], &perf_event[0]) == 2)
    goto next;
    if (*add_command == '/') {
    path = add_command;
    goto next;
    }
    if (sscanf(add_command, "u%d", &width) == 1) {
    if ((width == 32) || (width == 64))
    goto next;
    width = 64;
    }
    if (!strncmp(add_command, "cpu", strlen("cpu"))) {
    scope = SCOPE_CPU;
    goto next;
    }
    if (!strncmp(add_command, "core", strlen("core"))) {
    scope = SCOPE_CORE;
    goto next;
    }
    if (!strncmp(add_command, "package", strlen("package"))) {
    scope = SCOPE_PACKAGE;
    goto next;
    }
    if (!strncmp(add_command, "cycles", strlen("cycles"))) {
    type = COUNTER_CYCLES;
    goto next;
    }
    if (!strncmp(add_command, "seconds", strlen("seconds"))) {
    type = COUNTER_SECONDS;
    goto next;
    }
    if (!strncmp(add_command, "usec", strlen("usec"))) {
    type = COUNTER_USEC;
    goto next;
    }
    if (!strncmp(add_command, "raw", strlen("raw"))) {
    format = FORMAT_RAW;
    goto next;
    }
    if (!strncmp(add_command, "average", strlen("average"))) {
    format = FORMAT_AVERAGE;
    goto next;
    }
    if (!strncmp(add_command, "delta", strlen("delta"))) {
    format = FORMAT_DELTA;
    goto next;
    }
    if (!strncmp(add_command, "percent", strlen("percent"))) {
    format = FORMAT_PERCENT;
    goto next;
    }
    BUILD_BUG_ON(ARRAY_SIZE(name_buffer) <= 18);
    if (sscanf(add_command, "%18s,%*s", name_buffer) == 1) {
    char *eos;
    eos = strchr(name_buffer, ',');
    if (eos)
// eos = '\0';
    goto next;
    }
    next:
    add_command = strchr(add_command, ',');
    if (add_command) {
// add_command = '\0';
    add_command++;
    }
    }
    if ((msr_num == 0) && (path == core::ptr::null_mut()) && (perf_device[0] == '\0' || perf_event[0] == '\0')) {
    fprintf(stderr, "--add: (msrDDD | msr0xXXX | /path_to_counter | perf/device/event) required\n");
    fail++;
    }
// Test for non-empty perf_device and perf_event
    let mut is_perf_counter: bool = perf_device[0] && perf_event[0];
// generate default column header
    if (*name_buffer == '\0') {
    if (is_perf_counter) {
    snprintf(name_buffer, ARRAY_SIZE(name_buffer), "perf/%s", perf_event);
    } else {
    if (width == 32)
    sprintf(name_buffer, "M0x%x%s", msr_num, format == FORMAT_PERCENT ? "%" : "");
    else
    sprintf(name_buffer, "M0X%x%s", msr_num, format == FORMAT_PERCENT ? "%" : "");
    }
    }
    if (is_perf_counter) {
    if (add_perf_counter(perf_device, perf_event, name_buffer, width, scope, type, format))
    fail++;
    } else {
    if (add_counter(msr_num, path, name_buffer, width, scope, type, format, 0, 0))
    fail++;
    }
    if (fail) {
    help();
    exit(1);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn starts_with(str: *const c_char, prefix: *const c_char) -> bool {
    bool starts_with(const char *str, const char *prefix)
    {
    return strncmp(prefix, str, strlen(prefix)) == 0;
    }
#[no_mangle]
pub unsafe extern "C" fn pmt_parse_from_path(target_path: *const c_char, out_guid: *mut c_uint, out_seq: *mut c_uint) -> c_int {
    int pmt_parse_from_path(const char *target_path, unsigned int *out_guid, unsigned int *out_seq)
    {
    struct pmt_diriter_t pmt_iter;
    const struct dirent *dirname;
    struct stat stat, target_stat;
    let mut fd_telem_dir: c_int = -1;
    int fd_target_dir;
    let mut seq: c_uint = 0;
    unsigned long guid, target_guid;
    let mut ret: c_int = -1;
    fd_target_dir = open(target_path, O_RDONLY | O_DIRECTORY);
    if (fd_target_dir == -1) {
    return -1;
    }
    if (fstat(fd_target_dir, &target_stat) == -1) {
    fprintf(stderr, "%s: Failed to stat the target: %s", __func__, strerror(errno));
    exit(1);
    }
    if (parse_telem_info_file(fd_target_dir, "guid", "%lx", &target_guid)) {
    fprintf(stderr, "%s: Failed to parse the target guid file: %s", __func__, strerror(errno));
    exit(1);
    }
    close(fd_target_dir);
    pmt_diriter_init(&pmt_iter);
    for (dirname = pmt_diriter_begin(&pmt_iter, SYSFS_TELEM_PATH); dirname != core::ptr::null_mut(); dirname = pmt_diriter_next(&pmt_iter)) {
    fd_telem_dir = openat(dirfd(pmt_iter.dir), dirname.d_name, O_RDONLY | O_DIRECTORY);
    if (fd_telem_dir == -1)
    continue;
    if (parse_telem_info_file(fd_telem_dir, "guid", "%lx", &guid)) {
    fprintf(stderr, "%s: Failed to parse the guid file: %s", __func__, strerror(errno));
    continue;
    }
    if (fstat(fd_telem_dir, &stat) == -1) {
    fprintf(stderr, "%s: Failed to stat %s directory: %s", __func__, dirname.d_name, strerror(errno));
    continue;
    }
//
// If reached the same directory as target, exit the loop.
// Seq has the correct value now.
//
    if (stat.st_dev == target_stat.st_dev && stat.st_ino == target_stat.st_ino) {
    ret = 0;
    break;
    }
//
// If reached directory with the same guid,
// but it's not the target directory yet,
// increment seq and continue the search.
//
    if (guid == target_guid)
    ++seq;
    close(fd_telem_dir);
    fd_telem_dir = -1;
    }
    pmt_diriter_remove(&pmt_iter);
    if (fd_telem_dir != -1)
    close(fd_telem_dir);
    if (!ret) {
// out_guid = target_guid;
// out_seq = seq;
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn parse_add_command_pmt(add_command: *mut c_char) {
    void parse_add_command_pmt(char *add_command)
    {
    char *name = core::ptr::null_mut();
    char *type_name = core::ptr::null_mut();
    char *format_name = core::ptr::null_mut();
    char *direct_path = core::ptr::null_mut();
    static const char direct_path_prefix[] = "path=";
    unsigned int offset;
    unsigned int lsb;
    unsigned int msb;
    unsigned int guid;
    unsigned int seq = 0;	/* By default, pick first file in a sequence with a given GUID. */
    unsigned int domain_id;
    let mut scope: enum counter_scope = 0;
    let mut type: enum pmt_datatype = PMT_TYPE_RAW;
    let mut format: enum counter_format = FORMAT_RAW;
    let mut has_offset: bool = false;
    let mut has_lsb: bool = false;
    let mut has_msb: bool = false;
    bool has_format = true;	/* Format has a default value. */
    let mut has_guid: bool = false;
    let mut has_scope: bool = false;
    bool has_type = true;	/* Type has a default value. */
// Consume the "pmt," prefix.
    add_command = strchr(add_command, ',');
    if (!add_command) {
    help();
    exit(1);
    }
    ++add_command;
    while (add_command) {
    if (starts_with(add_command, "name=")) {
    name = add_command + strlen("name=");
    goto next;
    }
    if (starts_with(add_command, "type=")) {
    type_name = add_command + strlen("type=");
    goto next;
    }
    if (starts_with(add_command, "domain=")) {
    let mut prefix_len: usize = strlen("domain=");
    if (sscanf(add_command + prefix_len, "cpu%u", &domain_id) == 1) {
    scope = SCOPE_CPU;
    has_scope = true;
    } else if (sscanf(add_command + prefix_len, "core%u", &domain_id) == 1) {
    scope = SCOPE_CORE;
    has_scope = true;
    } else if (sscanf(add_command + prefix_len, "package%u", &domain_id) == 1) {
    scope = SCOPE_PACKAGE;
    has_scope = true;
    }
    if (!has_scope) {
    printf("%s: invalid value for scope. Expected cpu%%u, core%%u or package%%u.\n", __func__);
    exit(1);
    }
    goto next;
    }
    if (starts_with(add_command, "format=")) {
    format_name = add_command + strlen("format=");
    goto next;
    }
    if (sscanf(add_command, "offset=%u", &offset) == 1) {
    has_offset = true;
    goto next;
    }
    if (sscanf(add_command, "lsb=%u", &lsb) == 1) {
    has_lsb = true;
    goto next;
    }
    if (sscanf(add_command, "msb=%u", &msb) == 1) {
    has_msb = true;
    goto next;
    }
    if (sscanf(add_command, "guid=%x", &guid) == 1) {
    has_guid = true;
    goto next;
    }
    if (sscanf(add_command, "seq=%x", &seq) == 1)
    goto next;
    if (strncmp(add_command, direct_path_prefix, strlen(direct_path_prefix)) == 0) {
    direct_path = add_command + strlen(direct_path_prefix);
    goto next;
    }
    next:
    add_command = strchr(add_command, ',');
    if (add_command) {
// add_command = '\0';
    add_command++;
    }
    }
    if (!name) {
    printf("%s: missing %s\n", __func__, "name");
    exit(1);
    }
    if (strlen(name) >= PMT_COUNTER_NAME_SIZE_BYTES) {
    printf("%s: name has to be at most %d characters long\n", __func__, PMT_COUNTER_NAME_SIZE_BYTES);
    exit(1);
    }
    if (format_name) {
    has_format = false;
    if (strcmp("raw", format_name) == 0) {
    format = FORMAT_RAW;
    has_format = true;
    }
    if (strcmp("average", format_name) == 0) {
    format = FORMAT_AVERAGE;
    has_format = true;
    }
    if (strcmp("delta", format_name) == 0) {
    format = FORMAT_DELTA;
    has_format = true;
    }
    if (!has_format) {
    fprintf(stderr, "%s: Invalid format %s. Expected raw, average or delta\n", __func__, format_name);
    exit(1);
    }
    }
    if (type_name) {
    has_type = false;
    if (strcmp("raw", type_name) == 0) {
    type = PMT_TYPE_RAW;
    has_type = true;
    }
    if (strcmp("txtal_time", type_name) == 0) {
    type = PMT_TYPE_XTAL_TIME;
    has_type = true;
    }
    if (strcmp("tcore_clock", type_name) == 0) {
    type = PMT_TYPE_TCORE_CLOCK;
    has_type = true;
    }
    if (!has_type) {
    printf("%s: invalid %s: %s\n", __func__, "type", type_name);
    exit(1);
    }
    }
    if (!has_offset) {
    printf("%s : missing %s\n", __func__, "offset");
    exit(1);
    }
    if (!has_lsb) {
    printf("%s: missing %s\n", __func__, "lsb");
    exit(1);
    }
    if (!has_msb) {
    printf("%s: missing %s\n", __func__, "msb");
    exit(1);
    }
    if (direct_path && has_guid) {
    printf("%s: path and guid+seq parameters are mutually exclusive\nnotice: passed guid=0x%x and path=%s\n", __func__, guid, direct_path);
    exit(1);
    }
    if (direct_path) {
    if (pmt_parse_from_path(direct_path, &guid, &seq)) {
    printf("%s: failed to parse PMT file from %s\n", __func__, direct_path);
    exit(1);
    }
// GUID was just infered from the direct path.
    has_guid = true;
    }
    if (!has_guid) {
    printf("%s: missing %s\n", __func__, "guid or path");
    exit(1);
    }
    if (!has_scope) {
    printf("%s: missing %s\n", __func__, "scope");
    exit(1);
    }
    if (lsb > msb) {
    printf("%s: lsb > msb doesn't make sense\n", __func__);
    exit(1);
    }
    pmt_add_counter(guid, seq, name, type, lsb, msb, offset, scope, format, domain_id, PMT_OPEN_REQUIRED);
    }
#[no_mangle]
pub unsafe extern "C" fn parse_add_command(add_command: *mut c_char) {
    void parse_add_command(char *add_command)
    {
    if (strncmp(add_command, "pmt", strlen("pmt")) == 0)
    return parse_add_command_pmt(add_command);
    return parse_add_command_msr(add_command);
    }
#[no_mangle]
pub unsafe extern "C" fn is_deferred_add(name: *mut c_char) -> c_int {
    int is_deferred_add(char *name)
    {
    int i;
    for (i = 0; i < deferred_add_index; ++i)
    if (!strcmp(name, deferred_add_names[i])) {
    deferred_add_consumed |= (1 << i);
    return 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn is_deferred_skip(name: *mut c_char) -> c_int {
    int is_deferred_skip(char *name)
    {
    int i;
    for (i = 0; i < deferred_skip_index; ++i)
    if (!strcmp(name, deferred_skip_names[i])) {
    deferred_skip_consumed |= (1 << i);
    return 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn verify_deferred_consumed() {
    void verify_deferred_consumed(void)
    {
    int i;
    let mut fail: c_int = 0;
    for (i = 0; i < deferred_add_index; ++i) {
    if (!(deferred_add_consumed & (1 << i))) {
    warnx("Counter '%s' can not be added.", deferred_add_names[i]);
    fail++;
    }
    }
    for (i = 0; i < deferred_skip_index; ++i) {
    if (!(deferred_skip_consumed & (1 << i))) {
    warnx("Counter '%s' can not be skipped.", deferred_skip_names[i]);
    fail++;
    }
    }
    if (fail)
    exit(-EINVAL);
    }
#[no_mangle]
pub unsafe extern "C" fn probe_cpuidle_residency() {
    void probe_cpuidle_residency(void)
    {
    char path[64];
    char name_buf[16];
    FILE *input;
    int state;
    let mut min_state: c_int = 1024, max_state = 0;
    char *sp;
    for (state = 10; state >= 0; --state) {
    sprintf(path, "/sys/devices/system/cpu/cpu%d/cpuidle/state%d/name", master_cpu, state);
    input = fopen(path, "r");
    if (input == core::ptr::null_mut())
    continue;
    if (!fgets(name_buf, sizeof(name_buf), input))
    err(1, "%s: failed to read file", path);
// truncate "C1-HSW\n" to "C1", or truncate "C1\n" to "C1"
    sp = strchr(name_buf, '-');
    if (!sp)
    sp = strchrnul(name_buf, '\n');
// sp = '%';
// (sp + 1) = '\0';
    remove_underbar(name_buf);
    fclose(input);
    sprintf(path, "cpuidle/state%d/time", state);
    if (!DO_BIC(BIC_pct_idle) && !is_deferred_add(name_buf))
    continue;
    if (is_deferred_skip(name_buf))
    continue;
    add_counter(0, path, name_buf, 32, SCOPE_CPU, COUNTER_USEC, FORMAT_PERCENT, SYSFS_PERCPU, 0);
    if (state > max_state)
    max_state = state;
    if (state < min_state)
    min_state = state;
    }
    }
#[no_mangle]
unsafe extern "C" fn cpuidle_counter_wanted(name: *mut c_char) -> bool {
    static bool cpuidle_counter_wanted(char *name)
    {
    if (is_deferred_skip(name))
    return false;
    return DO_BIC(BIC_cpuidle) || is_deferred_add(name);
    }
#[no_mangle]
pub unsafe extern "C" fn probe_cpuidle_counts() {
    void probe_cpuidle_counts(void)
    {
    char path[64];
    char name_buf[16];
    FILE *input;
    int state;
    let mut min_state: c_int = 1024, max_state = 0;
    char *sp;
    if (!DO_BIC(BIC_cpuidle) && !deferred_add_index)
    return;
    for (state = 10; state >= 0; --state) {
    sprintf(path, "/sys/devices/system/cpu/cpu%d/cpuidle/state%d/name", master_cpu, state);
    input = fopen(path, "r");
    if (input == core::ptr::null_mut())
    continue;
    if (!fgets(name_buf, sizeof(name_buf), input))
    err(1, "%s: failed to read file", path);
    fclose(input);
    remove_underbar(name_buf);
// truncate "C1-HSW\n" to "C1", or truncate "C1\n" to "C1"
    sp = strchr(name_buf, '-');
    if (!sp)
    sp = strchrnul(name_buf, '\n');
//
// The 'below' sysfs file always contains 0 for the deepest state (largest index),
// do not add it.
//
    if (state != max_state) {
//
// Add 'C1+' for C1, and so on. The 'below' sysfs file always contains 0 for
// the last state, so do not add it.
//
// sp = '+';
// (sp + 1) = '\0';
    if (cpuidle_counter_wanted(name_buf)) {
    sprintf(path, "cpuidle/state%d/below", state);
    add_counter(0, path, name_buf, 64, SCOPE_CPU, COUNTER_ITEMS, FORMAT_DELTA, SYSFS_PERCPU, 0);
    }
    }
// sp = '\0';
    if (cpuidle_counter_wanted(name_buf)) {
    sprintf(path, "cpuidle/state%d/usage", state);
    add_counter(0, path, name_buf, 64, SCOPE_CPU, COUNTER_ITEMS, FORMAT_DELTA, SYSFS_PERCPU, 0);
    }
//
// The 'above' sysfs file always contains 0 for the shallowest state (smallest
// index), do not add it.
//
    if (state != min_state) {
// sp = '-';
// (sp + 1) = '\0';
    if (cpuidle_counter_wanted(name_buf)) {
    sprintf(path, "cpuidle/state%d/above", state);
    add_counter(0, path, name_buf, 64, SCOPE_CPU, COUNTER_ITEMS, FORMAT_DELTA, SYSFS_PERCPU, 0);
    }
    }
    }
    }
//
// parse cpuset with following syntax
// 1,2,4..6,8-10 and set bits in cpu_subset
//
#[no_mangle]
pub unsafe extern "C" fn parse_cpu_command(optarg: *mut c_char) {
    void parse_cpu_command(char *optarg)
    {
    if (!strcmp(optarg, "core")) {
    if (cpu_subset)
    goto error;
    show_core_only++;
    return;
    }
    if (!strcmp(optarg, "package")) {
    if (cpu_subset)
    goto error;
    show_pkg_only++;
    return;
    }
    if (show_core_only || show_pkg_only)
    goto error;
    cpu_subset = CPU_ALLOC(CPU_SUBSET_MAXCPUS);
    if (cpu_subset == core::ptr::null_mut())
    err(3, "CPU_ALLOC");
    cpu_subset_size = CPU_ALLOC_SIZE(CPU_SUBSET_MAXCPUS);
    CPU_ZERO_S(cpu_subset_size, cpu_subset);
    if (parse_cpu_str(optarg, cpu_subset, cpu_subset_size))
    goto error;
    return;
    error:
    fprintf(stderr, "\"--cpu %s\" malformed\n", optarg);
    help();
    exit(-1);
    }
#[no_mangle]
pub unsafe extern "C" fn cmdline(argc: c_int, argv: *mut c_char) {
    void cmdline(int argc, char **argv)
    {
    int opt;
    let mut option_index: c_int = 0;
    static struct option long_options[] = {
    { "add", required_argument, 0, 'a' },
    { "cpu", required_argument, 0, 'c' },
    { "Dump", no_argument, 0, 'D' },
    { "debug", no_argument, 0, 'd' },	/* internal, not documented */
    { "enable", required_argument, 0, 'e' },
    { "force", no_argument, 0, 'f' },
    { "interval", required_argument, 0, 'i' },
    { "IPC", no_argument, 0, 'I' },
    { "num_iterations", required_argument, 0, 'n' },
    { "header_iterations", required_argument, 0, 'N' },
    { "help", no_argument, 0, 'h' },
    { "hide", required_argument, 0, 'H' },	// meh, -h taken by --help
    { "Joules", no_argument, 0, 'J' },
    { "list", no_argument, 0, 'l' },
    { "out", required_argument, 0, 'o' },
    { "quiet", no_argument, 0, 'q' },
    { "no-msr", no_argument, 0, 'M' },
    { "no-perf", no_argument, 0, 'P' },
    { "show", required_argument, 0, 's' },
    { "Summary", no_argument, 0, 'S' },
    { "TCC", required_argument, 0, 'T' },
    { "version", no_argument, 0, 'v' },
    { 0, 0, 0, 0 }
    };
    progname = argv[0];
//
// Parse some options early, because they may make other options invalid,
// like adding the MSR counter with --add and at the same time using --no-msr.
//
    while ((opt = getopt_long_only(argc, argv, "+:MP", long_options, &option_index)) != -1) {
    switch (opt) {
    case 'M':
    no_msr = 1;
    break;
    case 'P':
    no_perf = 1;
    break;
    default:
    break;
    }
    }
    optind = 0;
    while ((opt = getopt_long_only(argc, argv, "+C:c:Dde:hi:Jn:N:o:qMPST:v", long_options, &option_index)) != -1) {
    switch (opt) {
    case 'a':
    parse_add_command(optarg);
    break;
    case 'c':
    parse_cpu_command(optarg);
    break;
    case 'D':
    dump_only++;
//
// Force the no_perf early to prevent using it as a source.
// User asks for raw values, but perf returns them relative
// to the opening of the file descriptor.
//
    no_perf = 1;
    break;
    case 'e':
// --enable specified counter, without clearning existing list
    bic_lookup(&bic_enabled, optarg, SHOW_LIST);
    break;
    case 'f':
    force_load++;
    break;
    case 'd':
    debug++;
    bic_set_all(&bic_enabled);
    break;
    case 'H':
//
// --hide: do not show those specified
// multiple invocations simply clear more bits in enabled mask
//
    {
    cpu_set_t bic_group_hide;
    BIC_INIT(&bic_group_hide);
    bic_lookup(&bic_group_hide, optarg, HIDE_LIST);
    bic_clear_bits(&bic_enabled, &bic_group_hide);
    }
    break;
    case 'h':
    help();
    exit(1);
    case 'i':
    {
    let mut interval: double = strtod(optarg, core::ptr::null_mut());
    if (interval < 0.001) {
    fprintf(outf, "interval %f seconds is too small\n", interval);
    exit(2);
    }
    interval_tv.tv_sec = interval_ts.tv_sec = interval;
    interval_tv.tv_usec = (interval - interval_tv.tv_sec) * 1000000;
    interval_ts.tv_nsec = (interval - interval_ts.tv_sec) * 1000000000;
    }
    break;
    case 'J':
    rapl_joules++;
    break;
    case 'l':
    bic_set_all(&bic_enabled);
    list_header_only++;
    quiet++;
    break;
    case 'o':
    outf = fopen_or_die(optarg, "w");
    break;
    case 'q':
    quiet = 1;
    break;
    case 'M':
    case 'P':
// Parsed earlier
    break;
    case 'n':
    num_iterations = strtoul(optarg, core::ptr::null_mut(), 0);
    errno = 0;
    if (errno || num_iterations == 0)
    errx(-1, "invalid iteration count: %s", optarg);
    break;
    case 'N':
    header_iterations = strtoul(optarg, core::ptr::null_mut(), 0);
    errno = 0;
    if (errno || header_iterations == 0)
    errx(-1, "invalid header iteration count: %s", optarg);
    break;
    case 's':
//
// --show: show only those specified
// The 1st invocation will clear and replace the enabled mask
// subsequent invocations can add to it.
//
    if (shown == 0)
    BIC_INIT(&bic_enabled);
    bic_lookup(&bic_enabled, optarg, SHOW_LIST);
    shown = 1;
    break;
    case 'S':
    summary_only++;
    break;
    case 'T':
    tj_max_override = atoi(optarg);
    break;
    case 'v':
    print_version();
    exit(0);
    break;
    default:
    help();
    exit(1);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn set_rlimit() {
    void set_rlimit(void)
    {
    struct rlimit limit;
    if (getrlimit(RLIMIT_NOFILE, &limit) < 0)
    err(1, "Failed to get rlimit");
    if (limit.rlim_max < MAX_NOFILE)
    limit.rlim_max = MAX_NOFILE;
    if (limit.rlim_cur < MAX_NOFILE)
    limit.rlim_cur = MAX_NOFILE;
    if (setrlimit(RLIMIT_NOFILE, &limit) < 0)
    err(1, "Failed to set rlimit");
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    int fd, ret;
    bic_groups_init();
    fd = open("/sys/fs/cgroup/cgroup.procs", O_WRONLY);
    if (fd < 0)
    goto skip_cgroup_setting;
    ret = write(fd, "0\n", 2);
    if (ret == -1)
    perror("Can't update cgroup\n");
    close(fd);
    skip_cgroup_setting:
    outf = stderr;
    cmdline(argc, argv);
    if (!quiet) {
    print_version();
    print_bootcmd();
    }
    probe_cpuidle_residency();
    probe_cpuidle_counts();
    verify_deferred_consumed();
    if (!getuid())
    set_rlimit();
    turbostat_init();
    if (!no_msr)
    msr_sum_record();
// dump counters and exit
    if (dump_only)
    return get_and_dump_counters();
// list header and exit
    if (list_header_only) {
    print_header(",");
    flush_output_stdout();
    return 0;
    }
//
// if any params left, it must be a command to fork
//
    if (argc - optind)
    return fork_it(argv + optind);
    else
    turbostat_loop();
    return 0;
    }
