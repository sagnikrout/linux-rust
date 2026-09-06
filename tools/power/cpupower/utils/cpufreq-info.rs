//! Automatically rewritten from C to Rust
//! Source: tools/power/cpupower/utils/cpufreq-info.c
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
// (C) 2004-2009  Dominik Brodowski <linux@dominikbrodowski.de>
//

pub const LINE_LEN: c_int = 10;
#[no_mangle]
unsafe extern "C" fn count_cpus() -> c_uint {
    static unsigned int count_cpus(void)
    {
    FILE *fp;
    char value[LINE_LEN];
    let mut ret: c_uint = 0;
    let mut cpunr: c_uint = 0;
    fp = fopen("/proc/stat", "r");
    if (!fp) {
    printf(_("Couldn't count the number of CPUs (%s: %s), assuming 1\n"), "/proc/stat", strerror(errno));
    return 1;
    }
    while (!feof(fp)) {
    if (!fgets(value, LINE_LEN, fp))
    continue;
    value[LINE_LEN - 1] = '\0';
    if (strlen(value) < (LINE_LEN - 2))
    continue;
    if (strstr(value, "cpu "))
    continue;
    if (sscanf(value, "cpu%d ", &cpunr) != 1)
    continue;
    if (cpunr > ret)
    ret = cpunr;
    }
    fclose(fp);
// cpu count starts from 0, on error return 1 (UP)
    return ret + 1;
    }
#[no_mangle]
unsafe extern "C" fn proc_cpufreq_output() {
    static void proc_cpufreq_output(void)
    {
    unsigned int cpu, nr_cpus;
    struct cpufreq_policy *policy;
    let mut min_pctg: c_uint = 0;
    let mut max_pctg: c_uint = 0;
    unsigned long min, max;
    printf(_("          minimum CPU frequency  -  maximum CPU frequency  -  governor\n"));
    nr_cpus = count_cpus();
    for (cpu = 0; cpu < nr_cpus; cpu++) {
    policy = cpufreq_get_policy(cpu);
    if (!policy)
    continue;
    if (cpufreq_get_hardware_limits(cpu, &min, &max)) {
    max = 0;
    } else {
    min_pctg = (policy.min * 100) / max;
    max_pctg = (policy.max * 100) / max;
    }
    printf("CPU%3d    %9lu kHz (%3d %%)  -  %9lu kHz (%3d %%)  -  %s\n",
    cpu , policy.min, max ? min_pctg : 0, policy.max,
    max ? max_pctg : 0, policy.governor);
    cpufreq_put_policy(policy);
    }
    }
    static int no_rounding;
#[no_mangle]
unsafe extern "C" fn print_duration(duration: c_ulong) {
    static void print_duration(unsigned long duration)
    {
    unsigned long tmp;
    if (no_rounding) {
    if (duration > 1000000)
    printf("%u.%06u ms", ((unsigned int) duration/1000000),
    ((unsigned int) duration%1000000));
#[no_mangle]
pub unsafe extern "C" fn if(100000: duration >) -> else {
    else if (duration > 100000)
    printf("%u us", ((unsigned int) duration/1000));
#[no_mangle]
pub unsafe extern "C" fn if(1000: duration >) -> else {
    else if (duration > 1000)
    printf("%u.%03u us", ((unsigned int) duration/1000),
    ((unsigned int) duration%1000));
    else
    printf("%lu ns", duration);
    } else {
    if (duration > 1000000) {
    tmp = duration%10000;
    if (tmp >= 5000)
    duration += 10000;
    printf("%u.%02u ms", ((unsigned int) duration/1000000),
    ((unsigned int) (duration%1000000)/10000));
    } else if (duration > 100000) {
    tmp = duration%1000;
    if (tmp >= 500)
    duration += 1000;
    printf("%u us", ((unsigned int) duration / 1000));
    } else if (duration > 1000) {
    tmp = duration%100;
    if (tmp >= 50)
    duration += 100;
    printf("%u.%01u us", ((unsigned int) duration/1000),
    ((unsigned int) (duration%1000)/100));
    } else
    printf("%lu ns", duration);
    }
    }
#[no_mangle]
unsafe extern "C" fn get_boost_mode_x86(cpu: c_uint) -> c_int {
    static int get_boost_mode_x86(unsigned int cpu)
    {
    int support, active, b_states = 0, ret, pstate_no, i;
// ToDo: Make this more global
    unsigned long pstates[MAX_HW_PSTATES] = {0,};
    ret = cpufreq_has_x86_boost_support(cpu, &support, &active, &b_states);
    if (ret) {
    printf(_("Error while evaluating Boost Capabilities"
    " on CPU %d -- are you root?\n"), cpu);
    return ret;
    }
// P state changes via MSR are identified via cpuid 80000007
    on Intel and AMD, but we assume boost capable machines can do that
    if (cpuid_eax(0x80000000) >= 0x80000007
    && (cpuid_edx(0x80000007) & (1 << 7)))
//
    printf(_("  boost state support:\n"));
    printf(_("    Supported: %s\n"), support ? _("yes") : _("no"));
    printf(_("    Active: %s\n"), active ? _("yes") : _("no"));
    if (cpupower_cpu_info.vendor == X86_VENDOR_AMD &&
    cpupower_cpu_info.caps & CPUPOWER_CAP_AMD_PSTATE) {
    return 0;
    } else if ((cpupower_cpu_info.vendor == X86_VENDOR_AMD &&
    cpupower_cpu_info.family >= 0x10) ||
    cpupower_cpu_info.vendor == X86_VENDOR_HYGON) {
    ret = decode_pstates(cpu, b_states, pstates, &pstate_no);
    if (ret)
    return ret;
    printf(_("    Boost States: %d\n"), b_states);
    printf(_("    Total States: %d\n"), pstate_no);
    for (i = 0; i < pstate_no; i++) {
    if (!pstates[i])
    continue;
    if (i < b_states)
    printf(_("    Pstate-Pb%d: %luMHz (boost state)"
    "\n"), i, pstates[i]);
    else
    printf(_("    Pstate-P%d:  %luMHz\n"),
    i - b_states, pstates[i]);
    }
    } else if (cpupower_cpu_info.caps & CPUPOWER_CAP_HAS_TURBO_RATIO) {
    double bclk;
    let mut intel_turbo_ratio: c_ulonglong = 0;
    unsigned int ratio;
// Any way to autodetect this ?
    if (cpupower_cpu_info.caps & CPUPOWER_CAP_IS_SNB)
    bclk = 100.00;
    else
    bclk = 133.33;
    intel_turbo_ratio = msr_intel_get_turbo_ratio(cpu);
    dprint ("    Ratio: 0x%llx - bclk: %f\n",
    intel_turbo_ratio, bclk);
    ratio = (intel_turbo_ratio >> 24) & 0xFF;
    if (ratio)
    printf(_("    %.0f MHz max turbo 4 active cores\n"),
    ratio * bclk);
    ratio = (intel_turbo_ratio >> 16) & 0xFF;
    if (ratio)
    printf(_("    %.0f MHz max turbo 3 active cores\n"),
    ratio * bclk);
    ratio = (intel_turbo_ratio >> 8) & 0xFF;
    if (ratio)
    printf(_("    %.0f MHz max turbo 2 active cores\n"),
    ratio * bclk);
    ratio = (intel_turbo_ratio >> 0) & 0xFF;
    if (ratio)
    printf(_("    %.0f MHz max turbo 1 active cores\n"),
    ratio * bclk);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_boost_mode_generic(cpu: c_uint) -> c_int {
    static int get_boost_mode_generic(unsigned int cpu)
    {
    bool active;
    if (!cpufreq_has_generic_boost_support(&active)) {
    printf(_("  boost state support:\n"));
    printf(_("    Active: %s\n"), active ? _("yes") : _("no"));
    }
    return 0;
    }
// --boost / -b
#[no_mangle]
unsafe extern "C" fn get_boost_mode(cpu: c_uint) -> c_int {
    static int get_boost_mode(unsigned int cpu)
    {
    struct cpufreq_available_frequencies *freqs;
    if (cpupower_cpu_info.vendor == X86_VENDOR_AMD ||
    cpupower_cpu_info.vendor == X86_VENDOR_HYGON ||
    cpupower_cpu_info.vendor == X86_VENDOR_INTEL)
    return get_boost_mode_x86(cpu);
    else
    get_boost_mode_generic(cpu);
    freqs = cpufreq_get_boost_frequencies(cpu);
    if (freqs) {
    printf(_("  boost frequency steps: "));
    while (freqs.next) {
    print_speed(freqs.frequency, no_rounding);
    printf(", ");
    freqs = freqs.next;
    }
    print_speed(freqs.frequency, no_rounding);
    printf("\n");
    cpufreq_put_available_frequencies(freqs);
    }
    return 0;
    }
// --freq / -f
#[no_mangle]
unsafe extern "C" fn get_freq_kernel(cpu: c_uint, human: c_uint) -> c_int {
    static int get_freq_kernel(unsigned int cpu, unsigned int human)
    {
    let mut freq: c_ulong = cpufreq_get_freq_kernel(cpu);
    printf(_("  current CPU frequency: "));
    if (!freq) {
    printf(_(" Unable to call to kernel\n"));
    return -EINVAL;
    }
    if (human) {
    print_speed(freq, no_rounding);
    } else
    printf("%lu", freq);
    printf(_(" (asserted by call to kernel)\n"));
    return 0;
    }
// --hwfreq / -w
#[no_mangle]
unsafe extern "C" fn get_freq_hardware(cpu: c_uint, human: c_uint) -> c_int {
    static int get_freq_hardware(unsigned int cpu, unsigned int human)
    {
    unsigned long freq;
    freq = cpufreq_get_freq_hardware(cpu);
    if (!(cpupower_cpu_info.caps & CPUPOWER_CAP_APERF) && !freq)
    return -EINVAL;
    printf(_("  current CPU frequency: "));
    if (!freq) {
    printf("Unable to call hardware\n");
    return -EINVAL;
    }
    if (human) {
    print_speed(freq, no_rounding);
    } else
    printf("%lu", freq);
    printf(_(" (asserted by call to hardware)\n"));
    return 0;
    }
// --hwlimits / -l
#[no_mangle]
unsafe extern "C" fn get_hardware_limits(cpu: c_uint, human: c_uint) -> c_int {
    static int get_hardware_limits(unsigned int cpu, unsigned int human)
    {
    unsigned long min, max;
    if (cpufreq_get_hardware_limits(cpu, &min, &max)) {
    printf(_("Not Available\n"));
    return -EINVAL;
    }
    if (human) {
    printf(_("  hardware limits: "));
    print_speed(min, no_rounding);
    printf(" - ");
    print_speed(max, no_rounding);
    printf("\n");
    } else {
    printf("%lu %lu\n", min, max);
    }
    return 0;
    }
// --driver / -d
#[no_mangle]
unsafe extern "C" fn get_driver(cpu: c_uint) -> c_int {
    static int get_driver(unsigned int cpu)
    {
    char *driver = cpufreq_get_driver(cpu);
    if (!driver) {
    printf(_("  no or unknown cpufreq driver is active on this CPU\n"));
    return -EINVAL;
    }
    printf("  driver: %s\n", driver);
    cpufreq_put_driver(driver);
    return 0;
    }
// --policy / -p
#[no_mangle]
unsafe extern "C" fn get_policy(cpu: c_uint) -> c_int {
    static int get_policy(unsigned int cpu)
    {
    struct cpufreq_policy *policy = cpufreq_get_policy(cpu);
    if (!policy) {
    printf(_("  Unable to determine current policy\n"));
    return -EINVAL;
    }
    printf(_("  current policy: frequency should be within "));
    print_speed(policy.min, no_rounding);
    printf(_(" and "));
    print_speed(policy.max, no_rounding);
    printf(".\n                  ");
    printf(_("The governor \"%s\" may decide which speed to use\n"
    "                  within this range.\n"),
    policy.governor);
    cpufreq_put_policy(policy);
    return 0;
    }
// --governors / -g
#[no_mangle]
unsafe extern "C" fn get_available_governors(cpu: c_uint) -> c_int {
    static int get_available_governors(unsigned int cpu)
    {
    struct cpufreq_available_governors *governors =
    cpufreq_get_available_governors(cpu);
    printf(_("  available cpufreq governors: "));
    if (!governors) {
    printf(_("Not Available\n"));
    return -EINVAL;
    }
    while (governors.next) {
    printf("%s ", governors.governor);
    governors = governors.next;
    }
    printf("%s\n", governors.governor);
    cpufreq_put_available_governors(governors);
    return 0;
    }
// --affected-cpus  / -a
#[no_mangle]
unsafe extern "C" fn get_affected_cpus(cpu: c_uint) -> c_int {
    static int get_affected_cpus(unsigned int cpu)
    {
    struct cpufreq_affected_cpus *cpus = cpufreq_get_affected_cpus(cpu);
    printf(_("  CPUs which need to have their frequency coordinated by software: "));
    if (!cpus) {
    printf(_("Not Available\n"));
    return -EINVAL;
    }
    while (cpus.next) {
    printf("%d ", cpus.cpu);
    cpus = cpus.next;
    }
    printf("%d\n", cpus.cpu);
    cpufreq_put_affected_cpus(cpus);
    return 0;
    }
// --related-cpus  / -r
#[no_mangle]
unsafe extern "C" fn get_related_cpus(cpu: c_uint) -> c_int {
    static int get_related_cpus(unsigned int cpu)
    {
    struct cpufreq_affected_cpus *cpus = cpufreq_get_related_cpus(cpu);
    printf(_("  CPUs which run at the same hardware frequency: "));
    if (!cpus) {
    printf(_("Not Available\n"));
    return -EINVAL;
    }
    while (cpus.next) {
    printf("%d ", cpus.cpu);
    cpus = cpus.next;
    }
    printf("%d\n", cpus.cpu);
    cpufreq_put_related_cpus(cpus);
    return 0;
    }
// --stats / -s
#[no_mangle]
unsafe extern "C" fn get_freq_stats(cpu: c_uint, human: c_uint) -> c_int {
    static int get_freq_stats(unsigned int cpu, unsigned int human)
    {
    let mut total_trans: c_ulong = cpufreq_get_transitions(cpu);
    unsigned long long total_time;
    struct cpufreq_stats *stats = cpufreq_get_stats(cpu, &total_time);
    while (stats) {
    if (human) {
    print_speed(stats.frequency, no_rounding);
    printf(":%.2f%%",
    (100.0 * stats.time_in_state) / total_time);
    } else
    printf("%lu:%llu",
    stats.frequency, stats.time_in_state);
    stats = stats.next;
    if (stats)
    printf(", ");
    }
    cpufreq_put_stats(stats);
    if (total_trans)
    printf("  (%lu)\n", total_trans);
    return 0;
    }
// --epp / -z
#[no_mangle]
unsafe extern "C" fn get_epp(cpu: c_uint, interactive: bool) -> c_int {
    static int get_epp(unsigned int cpu, bool interactive)
    {
    char *epp;
    epp = cpufreq_get_energy_performance_preference(cpu);
    if (!epp)
    return -EINVAL;
    if (interactive)
    printf(_("  energy performance preference: %s\n"), epp);
    cpufreq_put_energy_performance_preference(epp);
    return 0;
    }
// --latency / -y
#[no_mangle]
unsafe extern "C" fn get_latency(cpu: c_uint, human: c_uint) -> c_int {
    static int get_latency(unsigned int cpu, unsigned int human)
    {
    let mut latency: c_ulong = cpufreq_get_transition_latency(cpu);
    if (!get_epp(cpu, false))
    return -EINVAL;
    printf(_("  maximum transition latency: "));
    if (!latency || latency == UINT_MAX) {
    printf(_(" Cannot determine or is not supported.\n"));
    return -EINVAL;
    }
    if (human) {
    print_duration(latency);
    printf("\n");
    } else
    printf("%lu\n", latency);
    return 0;
    }
// --performance / -c
#[no_mangle]
unsafe extern "C" fn get_perf_cap(cpu: c_uint) -> c_int {
    static int get_perf_cap(unsigned int cpu)
    {
    if (cpupower_cpu_info.vendor == X86_VENDOR_AMD &&
    cpupower_cpu_info.caps & CPUPOWER_CAP_AMD_PSTATE)
    amd_pstate_show_perf_and_freq(cpu, no_rounding);
    else
    cppc_show_perf_and_freq(cpu, no_rounding);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn debug_output_one(cpu: c_uint) {
    static void debug_output_one(unsigned int cpu)
    {
    struct cpufreq_available_frequencies *freqs;
    get_driver(cpu);
    get_related_cpus(cpu);
    get_affected_cpus(cpu);
    get_latency(cpu, 1);
    get_epp(cpu, true);
    get_hardware_limits(cpu, 1);
    freqs = cpufreq_get_available_frequencies(cpu);
    if (freqs) {
    printf(_("  available frequency steps:  "));
    while (freqs.next) {
    print_speed(freqs.frequency, no_rounding);
    printf(", ");
    freqs = freqs.next;
    }
    print_speed(freqs.frequency, no_rounding);
    printf("\n");
    cpufreq_put_available_frequencies(freqs);
    }
    get_available_governors(cpu);
    get_policy(cpu);
    get_freq_hardware(cpu, 1);
    get_freq_kernel(cpu, 1);
    get_boost_mode(cpu);
    get_perf_cap(cpu);
    }
    static struct option info_opts[] = {
    {"debug",	 no_argument,		 core::ptr::null_mut(),	 'e'},
    {"boost",	 no_argument,		 core::ptr::null_mut(),	 'b'},
    {"freq",	 no_argument,		 core::ptr::null_mut(),	 'f'},
    {"hwfreq",	 no_argument,		 core::ptr::null_mut(),	 'w'},
    {"hwlimits",	 no_argument,		 core::ptr::null_mut(),	 'l'},
    {"driver",	 no_argument,		 core::ptr::null_mut(),	 'd'},
    {"policy",	 no_argument,		 core::ptr::null_mut(),	 'p'},
    {"governors",	 no_argument,		 core::ptr::null_mut(),	 'g'},
    {"related-cpus",  no_argument,	 core::ptr::null_mut(),	 'r'},
    {"affected-cpus", no_argument,	 core::ptr::null_mut(),	 'a'},
    {"stats",	 no_argument,		 core::ptr::null_mut(),	 's'},
    {"latency",	 no_argument,		 core::ptr::null_mut(),	 'y'},
    {"proc",	 no_argument,		 core::ptr::null_mut(),	 'o'},
    {"human",	 no_argument,		 core::ptr::null_mut(),	 'm'},
    {"no-rounding", no_argument,	 core::ptr::null_mut(),	 'n'},
    {"performance", no_argument,	 core::ptr::null_mut(),	 'c'},
    {"epp",		 no_argument,		 core::ptr::null_mut(),	 'z'},
    { },
    };
#[no_mangle]
pub unsafe extern "C" fn cmd_freq_info(argc: c_int, argv: *mut c_char) -> c_int {
    int cmd_freq_info(int argc, char **argv)
    {
    let mut ret: c_int = 0, cont = 1;
    let mut cpu: c_uint = 0;
    let mut human: c_uint = 0;
    let mut output_param: c_int = 0;
    do {
    ret = getopt_long(argc, argv, "oefwldpgrasmybncz", info_opts,
    core::ptr::null_mut());
    switch (ret) {
    case '?':
    output_param = '?';
    cont = 0;
    break;
    case -1:
    cont = 0;
    break;
    case 'b':
    case 'o':
    case 'a':
    case 'r':
    case 'g':
    case 'p':
    case 'd':
    case 'l':
    case 'w':
    case 'f':
    case 'e':
    case 's':
    case 'y':
    case 'c':
    case 'z':
    if (output_param) {
    output_param = -1;
    cont = 0;
    break;
    }
    output_param = ret;
    break;
    case 'm':
    if (human) {
    output_param = -1;
    cont = 0;
    break;
    }
    human = 1;
    break;
    case 'n':
    no_rounding = 1;
    break;
    default:
    fprintf(stderr, "invalid or unknown argument\n");
    return EXIT_FAILURE;
    }
    } while (cont);
    switch (output_param) {
    case 'o':
    if (!bitmask_isallclear(cpus_chosen)) {
    printf(_("The argument passed to this tool can't be "
    "combined with passing a --cpu argument\n"));
    return -EINVAL;
    }
    break;
    case 0:
    output_param = 'e';
    }
    ret = 0;
// Default is: show output of base_cpu only
    if (bitmask_isallclear(cpus_chosen))
    bitmask_setbit(cpus_chosen, base_cpu);
    switch (output_param) {
    case -1:
    printf(_("You can't specify more than one --cpu parameter and/or\n"
    "more than one output-specific argument\n"));
    return -EINVAL;
    case '?':
    printf(_("invalid or unknown argument\n"));
    return -EINVAL;
    case 'o':
    proc_cpufreq_output();
    return EXIT_SUCCESS;
    }
    for (cpu = bitmask_first(cpus_chosen);
    cpu <= bitmask_last(cpus_chosen); cpu++) {
    if (!bitmask_isbitset(cpus_chosen, cpu))
    continue;
    printf(_("analyzing CPU %d:\n"), cpu);
    if (sysfs_is_cpu_online(cpu) != 1) {
    printf(_(" *is offline\n"));
    printf("\n");
    continue;
    }
    switch (output_param) {
    case 'b':
    get_boost_mode(cpu);
    break;
    case 'e':
    debug_output_one(cpu);
    break;
    case 'a':
    ret = get_affected_cpus(cpu);
    break;
    case 'r':
    ret = get_related_cpus(cpu);
    break;
    case 'g':
    ret = get_available_governors(cpu);
    break;
    case 'p':
    ret = get_policy(cpu);
    break;
    case 'd':
    ret = get_driver(cpu);
    break;
    case 'l':
    ret = get_hardware_limits(cpu, human);
    break;
    case 'w':
    ret = get_freq_hardware(cpu, human);
    break;
    case 'f':
    ret = get_freq_kernel(cpu, human);
    break;
    case 's':
    ret = get_freq_stats(cpu, human);
    break;
    case 'y':
    ret = get_latency(cpu, human);
    break;
    case 'c':
    ret = get_perf_cap(cpu);
    break;
    case 'z':
    ret = get_epp(cpu, true);
    break;
    }
    if (ret)
    return ret;
    }
    return ret;
    }
