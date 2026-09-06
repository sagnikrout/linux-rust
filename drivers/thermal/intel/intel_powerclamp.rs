//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/intel/intel_powerclamp.c
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
// intel_powerclamp.c - package c-state idle injection
//
// Copyright (c) 2012-2023, Intel Corporation.
//
// Authors:
// Arjan van de Ven <arjan@linux.intel.com>
// Jacob Pan <jacob.jun.pan@linux.intel.com>
//
// TODO:
// 1. better handle wakeup from external interrupts, currently a fixed
// compensation is added to clamping duration when excessive amount
// of wakeups are observed during idle time. the reason is that in
// case of external interrupts without need for ack, clamping down
// cpu in non-irq context does not reduce irq. for majority of the
// cases, clamping down cpu does help reduce irq as well, we should
// be able to differentiate the two cases and give a quantitative
// solution for the irqs that we can control. perhaps based on
// get_cpu_iowait_time_us()
//
// 2. synchronization with other hw blocks
//

// For each undisturbed clamping period (no extra wake ups during idle time),
// we increment the confidence counter for the given target ratio.
// CONFIDENCE_OK defines the level where runtime calibration results are
// valid.
//

// Default idle injection duration, driver adjust sleep time to meet target
// idle ratio. Similar to frequency modulation.
//

    static struct dentry *debug_dir;
    static bool poll_pkg_cstate_enable;
// Idle ratio observed using package C-state counters
    static unsigned int current_ratio;
// Skip the idle injection till set to true
    static bool should_skip;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct powerclamp_data {
    pub cpu: c_uint,
    pub count: c_uint,
    pub guard: c_uint,
    pub window_size_now: c_uint,
    pub target_ratio: c_uint,
    pub clamping: bool,
}

    static struct powerclamp_data powerclamp_data;
    static struct thermal_cooling_device *cooling_dev;
    static DEFINE_MUTEX(powerclamp_lock);
// This duration is in microseconds
    static unsigned int duration;
    static unsigned int pkg_cstate_ratio_cur;
    static unsigned int window_size;
#[no_mangle]
unsafe extern "C" fn duration_set(arg: *const c_char, kp: *const kernel_param) -> c_int {
    static int duration_set(const char *arg, const struct kernel_param *kp)
    {
    let mut ret: c_int = 0;
    unsigned long new_duration;
    ret = kstrtoul(arg, 10, &new_duration);
    if (ret)
    goto exit;
    if (new_duration > 25 || new_duration < 6) {
    pr_err("Out of recommended range %lu, between 6-25ms\n",
    new_duration);
    ret = -EINVAL;
    goto exit;
    }
    mutex_lock(&powerclamp_lock);
    duration = clamp(new_duration, 6ul, 25ul) * 1000;
    mutex_unlock(&powerclamp_lock);
    exit:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn duration_get(buf: *mut c_char, kp: *const kernel_param) -> c_int {
    static int duration_get(char *buf, const struct kernel_param *kp)
    {
    int ret;
    mutex_lock(&powerclamp_lock);
    ret = sysfs_emit(buf, "%d\n", duration / 1000);
    mutex_unlock(&powerclamp_lock);
    return ret;
    }
    static const struct kernel_param_ops duration_ops = {
    .set = duration_set,
    .get = duration_get,
    };
    module_param_cb(duration, &duration_ops, core::ptr::null_mut(), 0644);
    MODULE_PARM_DESC(duration, "forced idle time for each attempt in msec.");
pub const DEFAULT_MAX_IDLE: c_int = 50;
pub const MAX_ALL_CPU_IDLE: c_int = 75;
    let mut max_idle: static u8 = DEFAULT_MAX_IDLE;
    static cpumask_var_t idle_injection_cpu_mask;
#[no_mangle]
unsafe extern "C" fn allocate_copy_idle_injection_mask(copy_mask: *const cpumask) -> c_int {
    static int allocate_copy_idle_injection_mask(const struct cpumask *copy_mask)
    {
    if (cpumask_available(idle_injection_cpu_mask))
    goto copy_mask;
// This mask is allocated only one time and freed during module exit
    if (!alloc_cpumask_var(&idle_injection_cpu_mask, GFP_KERNEL))
    return -ENOMEM;
    copy_mask:
    cpumask_copy(idle_injection_cpu_mask, copy_mask);
    return 0;
    }
// Return true if the cpumask and idle percent combination is invalid
#[no_mangle]
unsafe extern "C" fn check_invalid(mask: cpumask_var_t, idle: u8) -> bool {
    static bool check_invalid(cpumask_var_t mask, u8 idle)
    {
    if (cpumask_equal(cpu_present_mask, mask) && idle > MAX_ALL_CPU_IDLE)
    return true;
    return false;
    }
#[no_mangle]
unsafe extern "C" fn cpumask_set(arg: *const c_char, kp: *const kernel_param) -> c_int {
    static int cpumask_set(const char *arg, const struct kernel_param *kp)
    {
    cpumask_var_t new_mask;
    int ret;
    mutex_lock(&powerclamp_lock);
// Can't set mask when cooling device is in use
    if (powerclamp_data.clamping) {
    ret = -EAGAIN;
    goto skip_cpumask_set;
    }
    ret = alloc_cpumask_var(&new_mask, GFP_KERNEL);
    if (!ret)
    goto skip_cpumask_set;
    ret = bitmap_parse(arg, strlen(arg), cpumask_bits(new_mask),
    nr_cpumask_bits);
    if (ret)
    goto free_cpumask_set;
    if (cpumask_empty(new_mask) || check_invalid(new_mask, max_idle)) {
    ret = -EINVAL;
    goto free_cpumask_set;
    }
//
// When module parameters are passed from kernel command line
// during insmod, the module parameter callback is called
// before powerclamp_init(), so we can't assume that some
// cpumask can be allocated and copied before here. Also
// in this case this cpumask is used as the default mask.
//
    ret = allocate_copy_idle_injection_mask(new_mask);
    free_cpumask_set:
    free_cpumask_var(new_mask);
    skip_cpumask_set:
    mutex_unlock(&powerclamp_lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cpumask_get(buf: *mut c_char, kp: *const kernel_param) -> c_int {
    static int cpumask_get(char *buf, const struct kernel_param *kp)
    {
    if (!cpumask_available(idle_injection_cpu_mask))
    return -ENODEV;
    return sysfs_emit(buf, "%*pb\n", cpumask_pr_args(idle_injection_cpu_mask));
    }
    static const struct kernel_param_ops cpumask_ops = {
    .set = cpumask_set,
    .get = cpumask_get,
    };
    module_param_cb(cpumask, &cpumask_ops, core::ptr::null_mut(), 0644);
    MODULE_PARM_DESC(cpumask, "Mask of CPUs to use for idle injection.");
#[no_mangle]
unsafe extern "C" fn max_idle_set(arg: *const c_char, kp: *const kernel_param) -> c_int {
    static int max_idle_set(const char *arg, const struct kernel_param *kp)
    {
    u8 new_max_idle;
    let mut ret: c_int = 0;
    mutex_lock(&powerclamp_lock);
// Can't set mask when cooling device is in use
    if (powerclamp_data.clamping) {
    ret = -EAGAIN;
    goto skip_limit_set;
    }
    ret = kstrtou8(arg, 10, &new_max_idle);
    if (ret)
    goto skip_limit_set;
    if (new_max_idle > MAX_TARGET_RATIO) {
    ret = -EINVAL;
    goto skip_limit_set;
    }
    if (!cpumask_available(idle_injection_cpu_mask)) {
    ret = allocate_copy_idle_injection_mask(cpu_present_mask);
    if (ret)
    goto skip_limit_set;
    }
    if (check_invalid(idle_injection_cpu_mask, new_max_idle)) {
    ret = -EINVAL;
    goto skip_limit_set;
    }
    max_idle = new_max_idle;
    skip_limit_set:
    mutex_unlock(&powerclamp_lock);
    return ret;
    }
    static const struct kernel_param_ops max_idle_ops = {
    .set = max_idle_set,
    .get = param_get_byte,
    };
    module_param_cb(max_idle, &max_idle_ops, &max_idle, 0644);
    MODULE_PARM_DESC(max_idle, "maximum injected idle time to the total CPU time ratio in percent range:1-100");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct powerclamp_calibration_data {
    pub counter: *mut *mut unsigned long confidence; / used for calibration, basically a,
// gets incremented each time a clamping
// period is completed without extra wakeups
// once that counter is reached given level,
// compensation is deemed usable.
//
    pub when: *mut *mut unsigned long steady_comp; / steady state compensation used,
// no extra wakeups occurred.
//
    pub idle: *mut *mut unsigned long dynamic_comp; / compensate excessive wakeup from,
// mostly from external interrupts.
//
}

    static struct powerclamp_calibration_data cal_data[MAX_TARGET_RATIO];
#[no_mangle]
unsafe extern "C" fn window_size_set(arg: *const c_char, kp: *const kernel_param) -> c_int {
    static int window_size_set(const char *arg, const struct kernel_param *kp)
    {
    let mut ret: c_int = 0;
    unsigned long new_window_size;
    ret = kstrtoul(arg, 10, &new_window_size);
    if (ret)
    goto exit_win;
    if (new_window_size > 10 || new_window_size < 2) {
    pr_err("Out of recommended window size %lu, between 2-10\n",
    new_window_size);
    ret = -EINVAL;
    }
    window_size = clamp(new_window_size, 2ul, 10ul);
    smp_mb();
    exit_win:
    return ret;
    }
    static const struct kernel_param_ops window_size_ops = {
    .set = window_size_set,
    .get = param_get_int,
    };
    module_param_cb(window_size, &window_size_ops, &window_size, 0644);
    MODULE_PARM_DESC(window_size, "sliding window in number of clamping cycles\n"
    "\tpowerclamp controls idle ratio within this window. larger\n"
    "\twindow size results in slower response time but more smooth\n"
    "\tclamping results. default to 2.");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkg_cstate_info {
    pub skip: bool,
    pub msr_index: c_int,
    pub cstate_id: c_int,
}

    .msr_index = MSR_PKG_C##id##_RESIDENCY, \
    .cstate_id = id				\
    }
    static struct pkg_cstate_info pkg_cstates[] = {
    PKG_CSTATE_INIT(2),
    PKG_CSTATE_INIT(3),
    PKG_CSTATE_INIT(6),
    PKG_CSTATE_INIT(7),
    PKG_CSTATE_INIT(8),
    PKG_CSTATE_INIT(9),
    PKG_CSTATE_INIT(10),
    {core::ptr::null_mut()},
    };
#[no_mangle]
unsafe extern "C" fn has_pkg_state_counter() -> bool {
    static bool has_pkg_state_counter(void)
    {
    u64 val;
    struct pkg_cstate_info *info = pkg_cstates;
// check if any one of the counter msrs exists
    while (info.msr_index) {
    if (!rdmsrq_safe(info.msr_index, &val))
    return true;
    info++;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn pkg_state_counter() -> u64 {
    static u64 pkg_state_counter(void)
    {
    u64 val;
    let mut count: u64 = 0;
    struct pkg_cstate_info *info = pkg_cstates;
    while (info.msr_index) {
    if (!info.skip) {
    if (!rdmsrq_safe(info.msr_index, &val))
    count += val;
    else
    info.skip = true;
    }
    info++;
    }
    return count;
    }
#[no_mangle]
unsafe extern "C" fn get_compensation(ratio: c_int) -> c_uint {
    static unsigned int get_compensation(int ratio)
    {
    let mut comp: c_uint = 0;
    if (!poll_pkg_cstate_enable)
    return 0;
// we only use compensation if all adjacent ones are good
    if (ratio == 1 &&
    cal_data[ratio].confidence >= CONFIDENCE_OK &&
    cal_data[ratio + 1].confidence >= CONFIDENCE_OK &&
    cal_data[ratio + 2].confidence >= CONFIDENCE_OK) {
    comp = (cal_data[ratio].steady_comp +
    cal_data[ratio + 1].steady_comp +
    cal_data[ratio + 2].steady_comp) / 3;
    } else if (ratio == MAX_TARGET_RATIO - 1 &&
    cal_data[ratio].confidence >= CONFIDENCE_OK &&
    cal_data[ratio - 1].confidence >= CONFIDENCE_OK &&
    cal_data[ratio - 2].confidence >= CONFIDENCE_OK) {
    comp = (cal_data[ratio].steady_comp +
    cal_data[ratio - 1].steady_comp +
    cal_data[ratio - 2].steady_comp) / 3;
    } else if (cal_data[ratio].confidence >= CONFIDENCE_OK &&
    cal_data[ratio - 1].confidence >= CONFIDENCE_OK &&
    cal_data[ratio + 1].confidence >= CONFIDENCE_OK) {
    comp = (cal_data[ratio].steady_comp +
    cal_data[ratio - 1].steady_comp +
    cal_data[ratio + 1].steady_comp) / 3;
    }
// do not exceed limit
    if (comp + ratio >= MAX_TARGET_RATIO)
    comp = MAX_TARGET_RATIO - ratio - 1;
    return comp;
    }
#[no_mangle]
unsafe extern "C" fn adjust_compensation(target_ratio: c_int, win: c_uint) {
    static void adjust_compensation(int target_ratio, unsigned int win)
    {
    int delta;
    struct powerclamp_calibration_data *d = &cal_data[target_ratio];
//
// adjust compensations if confidence level has not been reached.
//
    if (d.confidence >= CONFIDENCE_OK)
    return;
    delta = powerclamp_data.target_ratio - current_ratio;
// filter out bad data
    if (delta >= 0 && delta <= (1+target_ratio/10)) {
    if (d.steady_comp)
    d.steady_comp =
    roundup(delta+d.steady_comp, 2)/2;
    else
    d.steady_comp = delta;
    d.confidence++;
    }
    }
    static bool powerclamp_adjust_controls(unsigned int target_ratio,
    unsigned int guard, unsigned int win)
    {
    static u64 msr_last, tsc_last;
    u64 msr_now, tsc_now;
    u64 val64;
// check result for the last window
    msr_now = pkg_state_counter();
    tsc_now = rdtsc();
// calculate pkg cstate vs tsc ratio
    if (!msr_last || !tsc_last)
    current_ratio = 1;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: tsc_now-tsc_last) -> else {
    val64 = 100*(msr_now-msr_last);
    do_div(val64, (tsc_now-tsc_last));
    current_ratio = val64;
    }
// update record
    msr_last = msr_now;
    tsc_last = tsc_now;
    adjust_compensation(target_ratio, win);
// if we are above target+guard, skip
    return powerclamp_data.target_ratio + guard <= current_ratio;
    }
//
// This function calculates runtime from the current target ratio.
// This function gets called under powerclamp_lock.
//
#[no_mangle]
unsafe extern "C" fn get_run_time() -> c_uint {
    static unsigned int get_run_time(void)
    {
    unsigned int compensated_ratio;
    unsigned int runtime;
//
// make sure user selected ratio does not take effect until
// the next round. adjust target_ratio if user has changed
// target such that we can converge quickly.
//
    powerclamp_data.guard = 1 + powerclamp_data.target_ratio / 20;
    powerclamp_data.window_size_now = window_size;
//
// systems may have different ability to enter package level
// c-states, thus we need to compensate the injected idle ratio
// to achieve the actual target reported by the HW.
//
    compensated_ratio = powerclamp_data.target_ratio +
    get_compensation(powerclamp_data.target_ratio);
    if (compensated_ratio <= 0)
    compensated_ratio = 1;
    runtime = duration * 100 / compensated_ratio - duration;
    return runtime;
    }
//
// 1 HZ polling while clamping is active, useful for userspace
// to monitor actual idle ratio.
//
    static void poll_pkg_cstate(struct work_struct *dummy);
    static DECLARE_DELAYED_WORK(poll_pkg_cstate_work, poll_pkg_cstate);
#[no_mangle]
unsafe extern "C" fn poll_pkg_cstate(dummy: *mut work_struct) {
    static void poll_pkg_cstate(struct work_struct *dummy)
    {
    static u64 msr_last;
    static u64 tsc_last;
    u64 msr_now;
    u64 tsc_now;
    u64 val64;
    msr_now = pkg_state_counter();
    tsc_now = rdtsc();
// calculate pkg cstate vs tsc ratio
    if (!msr_last || !tsc_last)
    pkg_cstate_ratio_cur = 1;
    else {
    if (tsc_now - tsc_last) {
    val64 = 100 * (msr_now - msr_last);
    do_div(val64, (tsc_now - tsc_last));
    pkg_cstate_ratio_cur = val64;
    }
    }
// update record
    msr_last = msr_now;
    tsc_last = tsc_now;
    mutex_lock(&powerclamp_lock);
    if (powerclamp_data.clamping)
    schedule_delayed_work(&poll_pkg_cstate_work, HZ);
    mutex_unlock(&powerclamp_lock);
    }
    static struct idle_inject_device *ii_dev;
//
// This function is called from idle injection core on timer expiry
// for the run duration. This allows powerclamp to readjust or skip
// injecting idle for this cycle.
//
#[no_mangle]
unsafe extern "C" fn idle_inject_update() -> bool {
    static bool idle_inject_update(void)
    {
    let mut update: bool = false;
// We can't sleep in this callback
    if (!mutex_trylock(&powerclamp_lock))
    return true;
    if (!(powerclamp_data.count % powerclamp_data.window_size_now)) {
    should_skip = powerclamp_adjust_controls(powerclamp_data.target_ratio,
    powerclamp_data.guard,
    powerclamp_data.window_size_now);
    update = true;
    }
    if (update) {
    let mut runtime: c_uint = get_run_time();
    idle_inject_set_duration(ii_dev, runtime, duration);
    }
    powerclamp_data.count++;
    mutex_unlock(&powerclamp_lock);
    if (should_skip)
    return false;
    return true;
    }
// This function starts idle injection by calling idle_inject_start()
#[no_mangle]
unsafe extern "C" fn trigger_idle_injection() {
    static void trigger_idle_injection(void)
    {
    let mut runtime: c_uint = get_run_time();
    idle_inject_set_duration(ii_dev, runtime, duration);
    idle_inject_start(ii_dev);
    powerclamp_data.clamping = true;
    }
//
// This function is called from start_power_clamp() to register
// CPUS with powercap idle injection register and set default
// idle duration and latency.
//
#[no_mangle]
unsafe extern "C" fn powerclamp_idle_injection_register() -> c_int {
    static int powerclamp_idle_injection_register(void)
    {
    poll_pkg_cstate_enable = false;
    if (cpumask_equal(cpu_present_mask, idle_injection_cpu_mask)) {
    ii_dev = idle_inject_register_full(idle_injection_cpu_mask, idle_inject_update);
    if (topology_max_packages() == 1 && topology_max_dies_per_package() == 1)
    poll_pkg_cstate_enable = true;
    } else {
    ii_dev = idle_inject_register(idle_injection_cpu_mask);
    }
    if (!ii_dev) {
    pr_err("powerclamp: idle_inject_register failed\n");
    return -EAGAIN;
    }
    idle_inject_set_duration(ii_dev, TICK_USEC, duration);
    idle_inject_set_latency(ii_dev, UINT_MAX);
    return 0;
    }
//
// This function is called from end_power_clamp() to stop idle injection
// and unregister CPUS from powercap idle injection core.
//
#[no_mangle]
unsafe extern "C" fn remove_idle_injection() {
    static void remove_idle_injection(void)
    {
    if (!powerclamp_data.clamping)
    return;
    powerclamp_data.clamping = false;
    idle_inject_stop(ii_dev);
    }
//
// This function is called when user change the cooling device
// state from zero to some other value.
//
#[no_mangle]
unsafe extern "C" fn start_power_clamp() -> c_int {
    static int start_power_clamp(void)
    {
    int ret;
    ret = powerclamp_idle_injection_register();
    if (!ret) {
    trigger_idle_injection();
    if (poll_pkg_cstate_enable)
    schedule_delayed_work(&poll_pkg_cstate_work, 0);
    }
    return ret;
    }
//
// This function is called when user change the cooling device
// state from non zero value zero.
//
#[no_mangle]
unsafe extern "C" fn end_power_clamp() {
    static void end_power_clamp(void)
    {
    if (powerclamp_data.clamping) {
    remove_idle_injection();
    idle_inject_unregister(ii_dev);
    }
    }
    static int powerclamp_get_max_state(struct thermal_cooling_device *cdev,
    unsigned long *state)
    {
// state = MAX_TARGET_RATIO;
    return 0;
    }
    static int powerclamp_get_cur_state(struct thermal_cooling_device *cdev,
    unsigned long *state)
    {
    mutex_lock(&powerclamp_lock);
// state = powerclamp_data.target_ratio;
    mutex_unlock(&powerclamp_lock);
    return 0;
    }
    static int powerclamp_set_cur_state(struct thermal_cooling_device *cdev,
    unsigned long new_target_ratio)
    {
    let mut ret: c_int = 0;
    mutex_lock(&powerclamp_lock);
    new_target_ratio = clamp(new_target_ratio, 0UL,
    (unsigned long) (max_idle - 1));
    if (powerclamp_data.target_ratio == new_target_ratio)
    goto exit_set;
    if (!powerclamp_data.target_ratio && new_target_ratio > 0) {
    pr_info("Start idle injection to reduce power\n");
    powerclamp_data.target_ratio = new_target_ratio;
    ret = start_power_clamp();
    if (ret)
    powerclamp_data.target_ratio = 0;
    goto exit_set;
    } else	if (powerclamp_data.target_ratio > 0 && new_target_ratio == 0) {
    pr_info("Stop forced idle injection\n");
    end_power_clamp();
    powerclamp_data.target_ratio = 0;
    } else	/* adjust currently running */ {
    unsigned int runtime;
    powerclamp_data.target_ratio = new_target_ratio;
    runtime = get_run_time();
    idle_inject_set_duration(ii_dev, runtime, duration);
    }
    exit_set:
    mutex_unlock(&powerclamp_lock);
    return ret;
    }
// bind to generic thermal layer as cooling device
    static const struct thermal_cooling_device_ops powerclamp_cooling_ops = {
    .get_max_state = powerclamp_get_max_state,
    .get_cur_state = powerclamp_get_cur_state,
    .set_cur_state = powerclamp_set_cur_state,
    };
    static const struct x86_cpu_id __initconst intel_powerclamp_ids[] = {
    X86_MATCH_VENDOR_FEATURE(INTEL, X86_FEATURE_MWAIT, core::ptr::null_mut()),
    {}
    };
    MODULE_DEVICE_TABLE(x86cpu, intel_powerclamp_ids);
#[no_mangle]
unsafe extern "C" fn powerclamp_probe() -> int __init {
    static int __init powerclamp_probe(void)
    {
    if (!x86_match_cpu(intel_powerclamp_ids)) {
    pr_err("CPU does not support MWAIT\n");
    return -ENODEV;
    }
// The goal for idle time alignment is to achieve package cstate.
    if (!has_pkg_state_counter()) {
    pr_info("No package C-state available\n");
    return -ENODEV;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn powerclamp_debug_show(m: *mut seq_file, unused: *mut c_void) -> c_int {
    static int powerclamp_debug_show(struct seq_file *m, void *unused)
    {
    let mut i: c_int = 0;
    seq_printf(m, "pct confidence steady dynamic (compensation)\n");
    for (i = 0; i < MAX_TARGET_RATIO; i++) {
    seq_printf(m, "%d\t%lu\t%lu\t%lu\n",
    i,
    cal_data[i].confidence,
    cal_data[i].steady_comp,
    cal_data[i].dynamic_comp);
    }
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(powerclamp_debug);
#[no_mangle]
pub unsafe extern "C" fn powerclamp_create_debug_files() {
    static inline void powerclamp_create_debug_files(void)
    {
    debug_dir = debugfs_create_dir("intel_powerclamp", core::ptr::null_mut());
    debugfs_create_file("powerclamp_calib", S_IRUGO, debug_dir, cal_data,
    &powerclamp_debug_fops);
    }
#[no_mangle]
unsafe extern "C" fn powerclamp_init() -> int __init {
    static int __init powerclamp_init(void)
    {
    int retval;
// probe cpu features and ids here
    retval = powerclamp_probe();
    if (retval)
    return retval;
    mutex_lock(&powerclamp_lock);
    if (!cpumask_available(idle_injection_cpu_mask))
    retval = allocate_copy_idle_injection_mask(cpu_present_mask);
    mutex_unlock(&powerclamp_lock);
    if (retval)
    return retval;
// set default limit, maybe adjusted during runtime based on feedback
    window_size = 2;
    cooling_dev = thermal_cooling_device_register("intel_powerclamp", core::ptr::null_mut(),
    &powerclamp_cooling_ops);
    if (IS_ERR(cooling_dev))
    return -ENODEV;
    if (!duration)
    duration = jiffies_to_usecs(DEFAULT_DURATION_JIFFIES);
    powerclamp_create_debug_files();
    return 0;
    }
    module_init(powerclamp_init);
#[no_mangle]
unsafe extern "C" fn powerclamp_exit() -> void __exit {
    static void __exit powerclamp_exit(void)
    {
    mutex_lock(&powerclamp_lock);
    end_power_clamp();
    mutex_unlock(&powerclamp_lock);
    thermal_cooling_device_unregister(cooling_dev);
    cancel_delayed_work_sync(&poll_pkg_cstate_work);
    debugfs_remove_recursive(debug_dir);
    if (cpumask_available(idle_injection_cpu_mask))
    free_cpumask_var(idle_injection_cpu_mask);
    }
    module_exit(powerclamp_exit);
    MODULE_IMPORT_NS("IDLE_INJECT");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Arjan van de Ven <arjan@linux.intel.com>");
    MODULE_AUTHOR("Jacob Pan <jacob.jun.pan@linux.intel.com>");
    MODULE_DESCRIPTION("Package Level C-state Idle Injection for Intel CPUs");
