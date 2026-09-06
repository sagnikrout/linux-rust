//! Automatically rewritten from C to Rust
//! Source: drivers/cpufreq/speedstep-centrino.c
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
// cpufreq driver for Enhanced SpeedStep, as found in Intel's Pentium
// M (part of the Centrino chipset).
//
// Since the original Pentium M, most new Intel CPUs support Enhanced
// SpeedStep.
//
// Despite the "SpeedStep" in the name, this is almost entirely unlike
// traditional SpeedStep.
//
// Modelled on speedstep.c
//
// Copyright (C) 2003 Jeremy Fitzhardinge <jeremy@goop.org>
//

    struct cpu_id
    {
    __u8	x86;            /* CPU family */
    __u8	x86_model;	/* model */
    __u8	x86_stepping;	/* stepping */
    };
    enum {
    CPU_BANIAS,
    CPU_DOTHAN_A1,
    CPU_DOTHAN_A2,
    CPU_DOTHAN_B0,
    CPU_MP4HT_D0,
    CPU_MP4HT_E0,
    };
    static const struct cpu_id cpu_ids[] = {
    [CPU_BANIAS]	= { 6,  9, 5 },
    [CPU_DOTHAN_A1]	= { 6, 13, 1 },
    [CPU_DOTHAN_A2]	= { 6, 13, 2 },
    [CPU_DOTHAN_B0]	= { 6, 13, 6 },
    [CPU_MP4HT_D0]	= {15,  3, 4 },
    [CPU_MP4HT_E0]	= {15,  4, 1 },
    };

    struct cpu_model
    {
    const struct cpu_id *cpu_id;
    const char	*model_name;
    unsigned	max_freq; /* max clock in kHz */
    struct cpufreq_frequency_table *op_points; /* clock/voltage pairs */
    };
    static int centrino_verify_cpu_id(const struct cpuinfo_x86 *c,
    const struct cpu_id *x);
// Operating points for current CPU
    static DEFINE_PER_CPU(struct cpu_model *, centrino_model);
    static DEFINE_PER_CPU(const struct cpu_id *, centrino_cpu);
    static struct cpufreq_driver centrino_driver;

// Computes the correct form for IA32_PERF_CTL MSR for a particular
    frequency/voltage operating point; frequency in MHz, volts in mV.
    This is stored as "driver_data" in the structure. */

    {								\
    .frequency = (mhz) * 1000,				\
    .driver_data = (((mhz)/100) << 8) | ((mv - 700) / 16)		\
    }
//
// These voltage tables were derived from the Intel Pentium M
// datasheet, document 25261202.pdf, Table 5.  I have verified they
// are consistent with my IBM ThinkPad X31, which has a 1.3GHz Pentium
// M.
//
// Ultra Low Voltage Intel Pentium M processor 900MHz (Banias)
    static struct cpufreq_frequency_table banias_900[] =
    {
    OP(600,  844),
    OP(800,  988),
    OP(900, 1004),
    { .frequency = CPUFREQ_TABLE_END }
    };
// Ultra Low Voltage Intel Pentium M processor 1000MHz (Banias)
    static struct cpufreq_frequency_table banias_1000[] =
    {
    OP(600,   844),
    OP(800,   972),
    OP(900,   988),
    OP(1000, 1004),
    { .frequency = CPUFREQ_TABLE_END }
    };
// Low Voltage Intel Pentium M processor 1.10GHz (Banias)
    static struct cpufreq_frequency_table banias_1100[] =
    {
    OP( 600,  956),
    OP( 800, 1020),
    OP( 900, 1100),
    OP(1000, 1164),
    OP(1100, 1180),
    { .frequency = CPUFREQ_TABLE_END }
    };
// Low Voltage Intel Pentium M processor 1.20GHz (Banias)
    static struct cpufreq_frequency_table banias_1200[] =
    {
    OP( 600,  956),
    OP( 800, 1004),
    OP( 900, 1020),
    OP(1000, 1100),
    OP(1100, 1164),
    OP(1200, 1180),
    { .frequency = CPUFREQ_TABLE_END }
    };
// Intel Pentium M processor 1.30GHz (Banias)
    static struct cpufreq_frequency_table banias_1300[] =
    {
    OP( 600,  956),
    OP( 800, 1260),
    OP(1000, 1292),
    OP(1200, 1356),
    OP(1300, 1388),
    { .frequency = CPUFREQ_TABLE_END }
    };
// Intel Pentium M processor 1.40GHz (Banias)
    static struct cpufreq_frequency_table banias_1400[] =
    {
    OP( 600,  956),
    OP( 800, 1180),
    OP(1000, 1308),
    OP(1200, 1436),
    OP(1400, 1484),
    { .frequency = CPUFREQ_TABLE_END }
    };
// Intel Pentium M processor 1.50GHz (Banias)
    static struct cpufreq_frequency_table banias_1500[] =
    {
    OP( 600,  956),
    OP( 800, 1116),
    OP(1000, 1228),
    OP(1200, 1356),
    OP(1400, 1452),
    OP(1500, 1484),
    { .frequency = CPUFREQ_TABLE_END }
    };
// Intel Pentium M processor 1.60GHz (Banias)
    static struct cpufreq_frequency_table banias_1600[] =
    {
    OP( 600,  956),
    OP( 800, 1036),
    OP(1000, 1164),
    OP(1200, 1276),
    OP(1400, 1420),
    OP(1600, 1484),
    { .frequency = CPUFREQ_TABLE_END }
    };
// Intel Pentium M processor 1.70GHz (Banias)
    static struct cpufreq_frequency_table banias_1700[] =
    {
    OP( 600,  956),
    OP( 800, 1004),
    OP(1000, 1116),
    OP(1200, 1228),
    OP(1400, 1308),
    OP(1700, 1484),
    { .frequency = CPUFREQ_TABLE_END }
    };

    {	.cpu_id		= cpuid,	\
    .model_name	= "Intel(R) Pentium(R) M processor " name "MHz", \
    .max_freq	= (max)*1000,	\
    .op_points	= banias_##max,	\
    }

// CPU models, their operating frequency range, and freq/voltage
    operating points */
    static struct cpu_model models[] =
    {
    _BANIAS(&cpu_ids[CPU_BANIAS], 900, " 900"),
    BANIAS(1000),
    BANIAS(1100),
    BANIAS(1200),
    BANIAS(1300),
    BANIAS(1400),
    BANIAS(1500),
    BANIAS(1600),
    BANIAS(1700),
// NULL model_name is a wildcard
    { &cpu_ids[CPU_DOTHAN_A1], core::ptr::null_mut(), 0, core::ptr::null_mut() },
    { &cpu_ids[CPU_DOTHAN_A2], core::ptr::null_mut(), 0, core::ptr::null_mut() },
    { &cpu_ids[CPU_DOTHAN_B0], core::ptr::null_mut(), 0, core::ptr::null_mut() },
    { &cpu_ids[CPU_MP4HT_D0], core::ptr::null_mut(), 0, core::ptr::null_mut() },
    { &cpu_ids[CPU_MP4HT_E0], core::ptr::null_mut(), 0, core::ptr::null_mut() },
    { core::ptr::null_mut(), }
    };

#[no_mangle]
unsafe extern "C" fn centrino_cpu_init_table(policy: *mut cpufreq_policy) -> c_int {
    static int centrino_cpu_init_table(struct cpufreq_policy *policy)
    {
    struct cpuinfo_x86 *cpu = &cpu_data(policy.cpu);
    struct cpu_model *model;
    for(model = models; model.cpu_id != core::ptr::null_mut(); model++)
    if (centrino_verify_cpu_id(cpu, model.cpu_id) &&
    (model.model_name == core::ptr::null_mut() ||
    strcmp(cpu.x86_model_id, model.model_name) == 0))
    break;
    if (model.cpu_id == core::ptr::null_mut()) {
// No match at all
    pr_debug("no support for CPU model \"%s\": "
    "send /proc/cpuinfo to " MAINTAINER "\n",
    cpu.x86_model_id);
    return -ENOENT;
    }
    if (model.op_points == core::ptr::null_mut()) {
// Matched a non-match
    pr_debug("no table support for CPU model \"%s\"\n",
    cpu.x86_model_id);
    pr_debug("try using the acpi-cpufreq driver\n");
    return -ENOENT;
    }
    per_cpu(centrino_model, policy.cpu) = model;
    pr_debug("found \"%s\": max frequency: %dkHz\n",
    model.model_name, model.max_freq);
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn centrino_cpu_init_table(policy: *mut cpufreq_policy) -> c_int {
    static inline int centrino_cpu_init_table(struct cpufreq_policy *policy)
    {
    return -ENODEV;
    }

    static int centrino_verify_cpu_id(const struct cpuinfo_x86 *c,
    const struct cpu_id *x)
    {
    if ((c.x86 == x.x86) &&
    (c.x86_model == x.x86_model) &&
    (c.x86_stepping == x.x86_stepping))
    return 1;
    return 0;
    }
// To be called only after centrino_model is initialized
#[no_mangle]
unsafe extern "C" fn extract_clock(msr: unsigned, cpu: c_uint, failsafe: c_int) -> unsigned {
    static unsigned extract_clock(unsigned msr, unsigned int cpu, int failsafe)
    {
    int i;
//
// Extract clock in kHz from PERF_CTL value
// for centrino, as some DSDTs are buggy.
// Ideally, this can be done using the acpi_data structure.
//
    if ((per_cpu(centrino_cpu, cpu) == &cpu_ids[CPU_BANIAS]) ||
    (per_cpu(centrino_cpu, cpu) == &cpu_ids[CPU_DOTHAN_A1]) ||
    (per_cpu(centrino_cpu, cpu) == &cpu_ids[CPU_DOTHAN_B0])) {
    msr = (msr >> 8) & 0xff;
    return msr * 100000;
    }
    if ((!per_cpu(centrino_model, cpu)) ||
    (!per_cpu(centrino_model, cpu).op_points))
    return 0;
    msr &= 0xffff;
    for (i = 0;
    per_cpu(centrino_model, cpu).op_points[i].frequency
    != CPUFREQ_TABLE_END;
    i++) {
    if (msr == per_cpu(centrino_model, cpu).op_points[i].driver_data)
    return per_cpu(centrino_model, cpu).
    op_points[i].frequency;
    }
    if (failsafe)
    return per_cpu(centrino_model, cpu).op_points[i-1].frequency;
    else
    return 0;
    }
// Return the current CPU frequency in kHz
#[no_mangle]
unsafe extern "C" fn get_cur_freq(cpu: c_uint) -> c_uint {
    static unsigned int get_cur_freq(unsigned int cpu)
    {
    struct msr val;
    unsigned clock_freq;
    rdmsrq_on_cpu(cpu, MSR_IA32_PERF_STATUS, &val.q);
    clock_freq = extract_clock(val.l, cpu, 0);
    if (unlikely(clock_freq == 0)) {
//
// On some CPUs, we can see transient MSR values (which are
// not present in _PSS), while CPU is doing some automatic
// P-state transition (like TM2). Get the last freq set
// in PERF_CTL.
//
    rdmsrq_on_cpu(cpu, MSR_IA32_PERF_CTL, &val.q);
    clock_freq = extract_clock(val.l, cpu, 1);
    }
    return clock_freq;
    }
#[no_mangle]
unsafe extern "C" fn centrino_cpu_init(policy: *mut cpufreq_policy) -> c_int {
    static int centrino_cpu_init(struct cpufreq_policy *policy)
    {
    struct cpuinfo_x86 *cpu = &cpu_data(policy.cpu);
    u64 q;
    int i;
// Only Intel makes Enhanced Speedstep-capable CPUs
    if (cpu.x86_vendor != X86_VENDOR_INTEL ||
    !cpu_has(cpu, X86_FEATURE_EST))
    return -ENODEV;
    if (cpu_has(cpu, X86_FEATURE_CONSTANT_TSC))
    centrino_driver.flags |= CPUFREQ_CONST_LOOPS;
    if (policy.cpu != 0)
    return -ENODEV;
    for (i = 0; i < N_IDS; i++)
    if (centrino_verify_cpu_id(cpu, &cpu_ids[i]))
    break;
    if (i != N_IDS)
    per_cpu(centrino_cpu, policy.cpu) = &cpu_ids[i];
    if (!per_cpu(centrino_cpu, policy.cpu)) {
    pr_debug("found unsupported CPU with "
    "Enhanced SpeedStep: send /proc/cpuinfo to "
    MAINTAINER "\n");
    return -ENODEV;
    }
    if (centrino_cpu_init_table(policy))
    return -ENODEV;
// Check to see if Enhanced SpeedStep is enabled, and try to
    enable it if not. */
    rdmsrq(MSR_IA32_MISC_ENABLE, q);
    if (!(q & MSR_IA32_MISC_ENABLE_ENHANCED_SPEEDSTEP)) {
    q |= MSR_IA32_MISC_ENABLE_ENHANCED_SPEEDSTEP;
    pr_debug("trying to enable Enhanced SpeedStep (%x)\n", (u32)q);
    wrmsrq(MSR_IA32_MISC_ENABLE, q);
// check to see if it stuck
    rdmsrq(MSR_IA32_MISC_ENABLE, q);
    if (!(q & MSR_IA32_MISC_ENABLE_ENHANCED_SPEEDSTEP)) {
    pr_info("couldn't enable Enhanced SpeedStep\n");
    return -ENODEV;
    }
    }
    policy.cpuinfo.transition_latency = 10000;
// 10uS transition latency
    policy.freq_table = per_cpu(centrino_model, policy.cpu).op_points;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn centrino_cpu_exit(policy: *mut cpufreq_policy) {
    static void centrino_cpu_exit(struct cpufreq_policy *policy)
    {
    let mut cpu: c_uint = policy.cpu;
    if (per_cpu(centrino_model, cpu))
    per_cpu(centrino_model, cpu) = core::ptr::null_mut();
    }
//
// centrino_target - set a new CPUFreq policy
// @policy: new policy
// @index: index of target frequency
//
// Sets a new CPUFreq policy.
//
#[no_mangle]
unsafe extern "C" fn centrino_target(policy: *mut cpufreq_policy, index: c_uint) -> c_int {
    static int centrino_target(struct cpufreq_policy *policy, unsigned int index)
    {
    unsigned int	msr, cpu = policy.cpu;
    let mut oldmsr: msr = { .q = 0 };
    let mut retval: c_int = 0;
    unsigned int		j, first_cpu;
    struct cpufreq_frequency_table *op_points;
    cpumask_var_t covered_cpus;
    if (unlikely(!zalloc_cpumask_var(&covered_cpus, GFP_KERNEL)))
    return -ENOMEM;
    if (unlikely(per_cpu(centrino_model, cpu) == core::ptr::null_mut())) {
    retval = -ENODEV;
    goto out;
    }
    first_cpu = 1;
    op_points = &per_cpu(centrino_model, cpu).op_points[index];
    for_each_cpu(j, policy.cpus) {
    int good_cpu;
//
// Support for SMP systems.
// Make sure we are running on CPU that wants to change freq
//
    if (policy.shared_type == CPUFREQ_SHARED_TYPE_ANY)
    good_cpu = cpumask_any_and(policy.cpus,
    cpu_online_mask);
    else
    good_cpu = j;
    if (good_cpu >= nr_cpu_ids) {
    pr_debug("couldn't limit to CPUs in this domain\n");
    retval = -EAGAIN;
    if (first_cpu) {
// We haven't started the transition yet.
    goto out;
    }
    break;
    }
    msr = op_points.driver_data;
    if (first_cpu) {
    rdmsrq_on_cpu(good_cpu, MSR_IA32_PERF_CTL, &oldmsr.q);
    if (msr == (oldmsr.l & 0xffff)) {
    pr_debug("no change needed - msr was and needs "
    "to be %x\n", oldmsr.l);
    retval = 0;
    goto out;
    }
    first_cpu = 0;
// all but 16 LSB are reserved, treat them with care
    oldmsr.l &= ~0xffff;
    msr &= 0xffff;
    oldmsr.l |= msr;
    }
    wrmsrq_on_cpu(good_cpu, MSR_IA32_PERF_CTL, oldmsr.q);
    if (policy.shared_type == CPUFREQ_SHARED_TYPE_ANY)
    break;
    cpumask_set_cpu(j, covered_cpus);
    }
    if (unlikely(retval)) {
//
// We have failed halfway through the frequency change.
// We have sent callbacks to policy->cpus and
// MSRs have already been written on coverd_cpus.
// Best effort undo..
//
    for_each_cpu(j, covered_cpus)
    wrmsrq_on_cpu(j, MSR_IA32_PERF_CTL, oldmsr.q);
    }
    retval = 0;
    out:
    free_cpumask_var(covered_cpus);
    return retval;
    }
    static struct cpufreq_driver centrino_driver = {
    .name		= "centrino", /* should be speedstep-centrino,
    but there's a 16 char limit */
    .init		= centrino_cpu_init,
    .exit		= centrino_cpu_exit,
    .verify		= cpufreq_generic_frequency_table_verify,
    .target_index	= centrino_target,
    .get		= get_cur_freq,
    };
//
// This doesn't replace the detailed checks above because
// the generic CPU IDs don't have a way to match for steppings
// or ASCII model IDs.
//
    static const struct x86_cpu_id centrino_ids[] = {
    X86_MATCH_VFM_FEATURE(IFM( 6,  9), X86_FEATURE_EST, core::ptr::null_mut()),
    X86_MATCH_VFM_FEATURE(IFM( 6, 13), X86_FEATURE_EST, core::ptr::null_mut()),
    X86_MATCH_VFM_FEATURE(IFM(15,  3), X86_FEATURE_EST, core::ptr::null_mut()),
    X86_MATCH_VFM_FEATURE(IFM(15,  4), X86_FEATURE_EST, core::ptr::null_mut()),
    {}
    };
//
// centrino_init - initializes the Enhanced SpeedStep CPUFreq driver
//
// Initializes the Enhanced SpeedStep support. Returns -ENODEV on
// unsupported devices, -ENOENT if there's no voltage table for this
// particular CPU model, -EINVAL on problems during initiatization,
// and zero on success.
//
// This is quite picky.  Not only does the CPU have to advertise the
// "est" flag in the cpuid capability flags, we look for a specific
// CPU model and stepping, and we need to have the exact model name in
// our voltage tables.  That is, be paranoid about not releasing
// someone's valuable magic smoke.
//
#[no_mangle]
unsafe extern "C" fn centrino_init() -> int __init {
    static int __init centrino_init(void)
    {
    if (!x86_match_cpu(centrino_ids))
    return -ENODEV;
    return cpufreq_register_driver(&centrino_driver);
    }
#[no_mangle]
unsafe extern "C" fn centrino_exit() -> void __exit {
    static void __exit centrino_exit(void)
    {
    cpufreq_unregister_driver(&centrino_driver);
    }
    MODULE_AUTHOR ("Jeremy Fitzhardinge <jeremy@goop.org>");
    MODULE_DESCRIPTION ("Enhanced SpeedStep driver for Intel Pentium M processors.");
    MODULE_LICENSE ("GPL");
    late_initcall(centrino_init);
    module_exit(centrino_exit);
