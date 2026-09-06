//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/acpi/cppc.c
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
// cppc.c: CPPC Interface for x86
// Copyright (c) 2016, Intel Corporation.
//

pub const CPPC_HIGHEST_PERF_PERFORMANCE: c_int = 196;
pub const CPPC_HIGHEST_PERF_PREFCORE: c_int = 166;
    enum amd_pref_core {
    AMD_PREF_CORE_UNKNOWN = 0,
    AMD_PREF_CORE_SUPPORTED,
    AMD_PREF_CORE_UNSUPPORTED,
    };
    static enum amd_pref_core amd_pref_core_detected;
    static u64 boost_numerator;
// Refer to drivers/acpi/cppc_acpi.c for the description of functions
#[no_mangle]
pub unsafe extern "C" fn cpc_supported_by_cpu() -> bool {
    bool cpc_supported_by_cpu(void)
    {
    switch (boot_cpu_data.x86_vendor) {
    case X86_VENDOR_AMD:
    case X86_VENDOR_HYGON:
    if (boot_cpu_data.x86 == 0x19 && ((boot_cpu_data.x86_model <= 0x0f) ||
    (boot_cpu_data.x86_model >= 0x20 && boot_cpu_data.x86_model <= 0x2f)))
    return true;
    else if (boot_cpu_data.x86 == 0x17 &&
    boot_cpu_data.x86_model >= 0x30 && boot_cpu_data.x86_model <= 0x7f)
    return true;
    return boot_cpu_has(X86_FEATURE_CPPC);
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn cpc_ffh_supported() -> bool {
    bool cpc_ffh_supported(void)
    {
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn cpc_read_ffh(cpunum: c_int, reg: *mut cpc_reg, val: *mut u64) -> c_int {
    int cpc_read_ffh(int cpunum, struct cpc_reg *reg, u64 *val)
    {
    int err;
    err = rdmsrq_safe_on_cpu(cpunum, reg.address, val);
    if (!err) {
    u64 mask = GENMASK_ULL(reg.bit_offset + reg.bit_width - 1,
    reg.bit_offset);
// val &= mask;
// val >>= reg->bit_offset;
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn cpc_write_ffh(cpunum: c_int, reg: *mut cpc_reg, val: u64) -> c_int {
    int cpc_write_ffh(int cpunum, struct cpc_reg *reg, u64 val)
    {
    u64 rd_val;
    int err;
    err = rdmsrq_safe_on_cpu(cpunum, reg.address, &rd_val);
    if (!err) {
    u64 mask = GENMASK_ULL(reg.bit_offset + reg.bit_width - 1,
    reg.bit_offset);
    val <<= reg.bit_offset;
    val &= mask;
    rd_val &= ~mask;
    rd_val |= val;
    err = wrmsrq_safe_on_cpu(cpunum, reg.address, rd_val);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn amd_set_max_freq_ratio() {
    static void amd_set_max_freq_ratio(void)
    {
    struct cppc_perf_caps perf_caps;
    u64 numerator, nominal_perf;
    u64 perf_ratio;
    int rc;
    rc = cppc_get_perf_caps(0, &perf_caps);
    if (rc) {
    pr_debug("Could not retrieve perf counters (%d)\n", rc);
    return;
    }
    rc = amd_get_boost_ratio_numerator(0, &numerator);
    if (rc) {
    pr_debug("Could not retrieve highest performance (%d)\n", rc);
    return;
    }
    nominal_perf = perf_caps.nominal_perf;
    if (!nominal_perf) {
    pr_debug("Could not retrieve nominal performance\n");
    return;
    }
// midpoint between max_boost and max_P
    perf_ratio = (div_u64(numerator * SCHED_CAPACITY_SCALE, nominal_perf) + SCHED_CAPACITY_SCALE) >> 1;
    freq_invariance_set_perf_ratio(perf_ratio, false);
    }
    static DEFINE_MUTEX(freq_invariance_lock);
#[no_mangle]
pub unsafe extern "C" fn init_freq_invariance_cppc() {
    static inline void init_freq_invariance_cppc(void)
    {
    static bool init_done;
    if (!cpu_feature_enabled(X86_FEATURE_APERFMPERF))
    return;
    if (boot_cpu_data.x86_vendor != X86_VENDOR_AMD)
    return;
    mutex_lock(&freq_invariance_lock);
    if (!init_done)
    amd_set_max_freq_ratio();
    init_done = true;
    mutex_unlock(&freq_invariance_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn acpi_processor_init_invariance_cppc() {
    void acpi_processor_init_invariance_cppc(void)
    {
    init_freq_invariance_cppc();
    }
//
// Get the highest performance register value.
// @cpu: CPU from which to get highest performance.
// @highest_perf: Return address for highest performance value.
//
// Return: 0 for success, negative error code otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn amd_get_highest_perf(cpu: c_uint, highest_perf: *mut u32) -> c_int {
    int amd_get_highest_perf(unsigned int cpu, u32 *highest_perf)
    {
    u64 val;
    int ret;
    if (cpu_feature_enabled(X86_FEATURE_CPPC)) {
    ret = rdmsrq_safe_on_cpu(cpu, MSR_AMD_CPPC_CAP1, &val);
    if (ret)
    goto out;
    val = FIELD_GET(AMD_CPPC_HIGHEST_PERF_MASK, val);
    } else {
    ret = cppc_get_highest_perf(cpu, &val);
    if (ret)
    goto out;
    }
    WRITE_ONCE(*highest_perf, (u32)val);
    out:
    return ret;
    }
    EXPORT_SYMBOL_GPL(amd_get_highest_perf);
//
// amd_detect_prefcore: Detect if CPUs in the system support preferred cores
// @detected: Output variable for the result of the detection.
//
// Determine whether CPUs in the system support preferred cores. On systems
// that support preferred cores, different highest perf values will be found
// on different cores. On other systems, the highest perf value will be the
// same on all cores.
//
// The result of the detection will be stored in the 'detected' parameter.
//
// Return: 0 for success, negative error code otherwise
//
#[no_mangle]
pub unsafe extern "C" fn amd_detect_prefcore(detected: *mut bool) -> c_int {
    int amd_detect_prefcore(bool *detected)
    {
    int cpu, count = 0;
    u64 highest_perf[2] = {0};
    if (WARN_ON(!detected))
    return -EINVAL;
    switch (amd_pref_core_detected) {
    case AMD_PREF_CORE_SUPPORTED:
// detected = true;
    return 0;
    case AMD_PREF_CORE_UNSUPPORTED:
// detected = false;
    return 0;
    default:
    break;
    }
    for_each_online_cpu(cpu) {
    u32 tmp;
    int ret;
    ret = amd_get_highest_perf(cpu, &tmp);
    if (ret)
    return ret;
    if (!count || (count == 1 && tmp != highest_perf[0]))
    highest_perf[count++] = tmp;
    if (count == 2)
    break;
    }
// detected = (count == 2);
    boost_numerator = highest_perf[0];
    amd_pref_core_detected = *detected ? AMD_PREF_CORE_SUPPORTED :
    AMD_PREF_CORE_UNSUPPORTED;
    pr_debug("AMD CPPC preferred core is %ssupported (highest perf: 0x%llx)\n",
// detected ? "" : "un", highest_perf[0]);
    return 0;
    }
    EXPORT_SYMBOL_GPL(amd_detect_prefcore);
//
// amd_get_boost_ratio_numerator: Get the numerator to use for boost ratio calculation
// @cpu: CPU to get numerator for.
// @numerator: Output variable for numerator.
//
// Determine the numerator to use for calculating the boost ratio on
// a CPU. On systems that support preferred cores, this will be a hardcoded
// value. On other systems this will the highest performance register value.
//
// If booting the system with amd-pstate enabled but preferred cores disabled then
// the correct boost numerator will be returned to match hardware capabilities
// even if the preferred cores scheduling hints are not enabled.
//
// Return: 0 for success, negative error code otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn amd_get_boost_ratio_numerator(cpu: c_uint, numerator: *mut u64) -> c_int {
    int amd_get_boost_ratio_numerator(unsigned int cpu, u64 *numerator)
    {
    bool prefcore;
    int ret;
    u32 tmp;
    ret = amd_detect_prefcore(&prefcore);
    if (ret)
    return ret;
// without preferred cores, return the highest perf register value
    if (!prefcore) {
// numerator = boost_numerator;
    return 0;
    }
//
// For AMD CPUs with Family ID 19H and Model ID range 0x70 to 0x7f,
// the highest performance level is set to 196.
// https://bugzilla.kernel.org/show_bug.cgi?id=218759
//
    if (cpu_feature_enabled(X86_FEATURE_ZEN4)) {
    switch (boot_cpu_data.x86_model) {
    case 0x70 ... 0x7f:
// numerator = CPPC_HIGHEST_PERF_PERFORMANCE;
    return 0;
    default:
    break;
    }
    }
// detect if running on heterogeneous design
    if (cpu_feature_enabled(X86_FEATURE_AMD_HTR_CORES)) {
    switch (cpu_data(cpu).topo.cpu_type) {
    case TOPO_CPU_TYPE_UNKNOWN:
    case TOPO_CPU_TYPE_ANY:
    pr_warn("Undefined core type found for cpu %d\n", cpu);
    break;
    case TOPO_CPU_TYPE_PERFORMANCE:
// use the max scale for performance cores
// numerator = CPPC_HIGHEST_PERF_PERFORMANCE;
    return 0;
    case TOPO_CPU_TYPE_LOW_POWER:
    case TOPO_CPU_TYPE_EFFICIENCY:
// use the highest perf value for efficiency and low-power cores
    ret = amd_get_highest_perf(cpu, &tmp);
    if (ret)
    return ret;
// numerator = tmp;
    return 0;
    }
    }
// numerator = CPPC_HIGHEST_PERF_PREFCORE;
    return 0;
    }
    EXPORT_SYMBOL_GPL(amd_get_boost_ratio_numerator);
