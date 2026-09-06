//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/intel/therm_throt.c
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
// Thermal throttle event support code (such as syslog messaging and rate
// limiting) that was factored out from x86_64 (mce_intel.c) and i386 (p4.c).
//
// This allows consistent reporting of CPU thermal throttle events.
//
// Maintains a counter in /sys that keeps track of the number of thermal
// events, such that the user knows how bad the thermal problem might be
// (since the logging to syslog is rate limited).
//
// Author: Dmitriy Zavin (dmitriyz@google.com)
//
// Credits: Adapted from Zwane Mwaikambo's original code in mce_intel.c.
// Inspired by Ross Biro's and Al Borchers' counter code.
//

// How long to wait between reporting thermal events

pub const THERMAL_THROTTLING_EVENT: c_int = 0;
pub const POWER_LIMIT_EVENT: c_int = 1;
//
// struct _thermal_state - Represent the current thermal event state
// @next_check:			Stores the next timestamp, when it is allowed
// to log the next warning message.
// @last_interrupt_time:	Stores the timestamp for the last threshold
// high event.
// @therm_work:			Delayed workqueue structure
// @count:			Stores the current running count for thermal
// or power threshold interrupts.
// @last_count:			Stores the previous running count for thermal
// or power threshold interrupts.
// @max_time_ms:		This shows the maximum amount of time CPU was
// in throttled state for a single thermal
// threshold high to low state.
// @total_time_ms:		This is a cumulative time during which CPU was
// in the throttled state.
// @rate_control_active:	Set when a throttling message is logged.
// This is used for the purpose of rate-control.
// @new_event:			Stores the last high/low status of the
// THERM_STATUS_PROCHOT or
// THERM_STATUS_POWER_LIMIT.
// @level:			Stores whether this _thermal_state instance is
// for a CORE level or for PACKAGE level.
// @sample_index:		Index for storing the next sample in the buffer
// temp_samples[].
// @sample_count:		Total number of samples collected in the buffer
// temp_samples[].
// @average:			The last moving average of temperature samples
// @baseline_temp:		Temperature at which thermal threshold high
// interrupt was generated.
// @temp_samples:		Storage for temperature samples to calculate
// moving average.
//
// This structure is used to represent data related to thermal state for a CPU.
// There is a separate storage for core and package level for each CPU.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _thermal_state {
    pub next_check: u64,
    pub last_interrupt_time: u64,
    pub therm_work: delayed_work,
    pub count: c_ulong,
    pub last_count: c_ulong,
    pub max_time_ms: c_ulong,
    pub total_time_ms: c_ulong,
    pub rate_control_active: bool,
    pub new_event: bool,
    pub level: u8,
    pub sample_index: u8,
    pub sample_count: u8,
    pub average: u8,
    pub baseline_temp: u8,
    pub temp_samples: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct thermal_state {
    pub core_throttle: _thermal_state,
    pub core_power_limit: _thermal_state,
    pub package_throttle: _thermal_state,
    pub package_power_limit: _thermal_state,
    pub core_thresh0: _thermal_state,
    pub core_thresh1: _thermal_state,
    pub pkg_thresh0: _thermal_state,
    pub pkg_thresh1: _thermal_state,
}

// Callback to handle core threshold interrupts
    int (*platform_thermal_notify)(__u64 msr_val);
    EXPORT_SYMBOL(platform_thermal_notify);
// Callback to handle core package threshold_interrupts
    int (*platform_thermal_package_notify)(__u64 msr_val);
    EXPORT_SYMBOL_GPL(platform_thermal_package_notify);
// Callback support of rate control, return true, if
// callback has rate control
    bool (*platform_thermal_package_rate_control)(void);
    EXPORT_SYMBOL_GPL(platform_thermal_package_rate_control);
    static DEFINE_PER_CPU(struct thermal_state, thermal_state);
    let mut therm_throt_en: static atomic_t = ATOMIC_INIT(0);
    static u32 lvtthmr_init __read_mostly;

    static DEVICE_ATTR(_name, 0444,					\
    therm_throt_device_show_##_name,		\
    core::ptr::null_mut())				\

    \
    static ssize_t therm_throt_device_show_##event##_##name(		\
    struct device *dev,				\
    struct device_attribute *attr,			\
    char *buf)					\
    {									\
    unsigned int cpu = dev.id;					\
    ssize_t ret;							\
    \
    preempt_disable();	/* CPU hotplug */			\
    if (cpu_online(cpu)) {						\
    ret = sysfs_emit(buf, "%lu\n",				\
    per_cpu(thermal_state, cpu).event.name);	\
    } else								\
    ret = 0;						\
    preempt_enable();						\
    \
    return ret;							\
    }
    define_therm_throt_device_show_func(core_throttle, count);
    define_therm_throt_device_one_ro(core_throttle_count);
    define_therm_throt_device_show_func(core_power_limit, count);
    define_therm_throt_device_one_ro(core_power_limit_count);
    define_therm_throt_device_show_func(package_throttle, count);
    define_therm_throt_device_one_ro(package_throttle_count);
    define_therm_throt_device_show_func(package_power_limit, count);
    define_therm_throt_device_one_ro(package_power_limit_count);
    define_therm_throt_device_show_func(core_throttle, max_time_ms);
    define_therm_throt_device_one_ro(core_throttle_max_time_ms);
    define_therm_throt_device_show_func(package_throttle, max_time_ms);
    define_therm_throt_device_one_ro(package_throttle_max_time_ms);
    define_therm_throt_device_show_func(core_throttle, total_time_ms);
    define_therm_throt_device_one_ro(core_throttle_total_time_ms);
    define_therm_throt_device_show_func(package_throttle, total_time_ms);
    define_therm_throt_device_one_ro(package_throttle_total_time_ms);
    static struct attribute *thermal_throttle_attrs[] = {
    &dev_attr_core_throttle_count.attr,
    &dev_attr_core_throttle_max_time_ms.attr,
    &dev_attr_core_throttle_total_time_ms.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group thermal_attr_group = {
    .attrs	= thermal_throttle_attrs,
    .name	= "thermal_throttle"
    };

    static u64 therm_intr_core_clear_mask;
    static u64 therm_intr_pkg_clear_mask;
#[no_mangle]
unsafe extern "C" fn thermal_intr_init_core_clear_mask() {
    static void thermal_intr_init_core_clear_mask(void)
    {
    if (therm_intr_core_clear_mask)
    return;
//
// Reference: Intel SDM  Volume 4
// "Table 2-2. IA-32 Architectural MSRs", MSR 0x19C
// IA32_THERM_STATUS.
//
// Bit 1, 3, 5: CPUID.01H:EDX[22] = 1. This driver will not
// enable interrupts, when 0 as it checks for X86_FEATURE_ACPI.
//
    therm_intr_core_clear_mask = (BIT(1) | BIT(3) | BIT(5));
//
// Bit 7 and 9: Thermal Threshold #1 and #2 log
// If CPUID.01H:ECX[8] = 1
//
    if (boot_cpu_has(X86_FEATURE_TM2))
    therm_intr_core_clear_mask |= (BIT(7) | BIT(9));
// Bit 11: Power Limitation log (R/WC0) If CPUID.06H:EAX[4] = 1
    if (boot_cpu_has(X86_FEATURE_PLN))
    therm_intr_core_clear_mask |= BIT(11);
//
// Bit 13: Current Limit log (R/WC0) If CPUID.06H:EAX[7] = 1
// Bit 15: Cross Domain Limit log (R/WC0) If CPUID.06H:EAX[7] = 1
//
    if (boot_cpu_has(X86_FEATURE_HWP))
    therm_intr_core_clear_mask |= (BIT(13) | BIT(15));
    }
#[no_mangle]
unsafe extern "C" fn thermal_intr_init_pkg_clear_mask() {
    static void thermal_intr_init_pkg_clear_mask(void)
    {
    if (therm_intr_pkg_clear_mask)
    return;
//
// Reference: Intel SDM  Volume 4
// "Table 2-2. IA-32 Architectural MSRs", MSR 0x1B1
// IA32_PACKAGE_THERM_STATUS.
//
// All bits except BITs 25 and 26 depend on CPUID.06H: EAX[6] = 1
    if (boot_cpu_has(X86_FEATURE_PTS))
    therm_intr_pkg_clear_mask = (BIT(1) | BIT(3) | BIT(5) | BIT(7) | BIT(9) | BIT(11));
//
// Intel SDM Volume 1: Thermal and Power Management Leaf
// Bit 26: CPUID.06H: EAX[19] = 1
//
    if (boot_cpu_has(X86_FEATURE_HFI))
    therm_intr_pkg_clear_mask |= BIT(26);
//
// Intel SDM Volume 1: Thermal and Power Management Leaf
// Bit 25: CPUID.06H: EAX[24] = 1
//
    if (boot_cpu_has(X86_FEATURE_DPTI))
    therm_intr_pkg_clear_mask |= BIT(25);
    }
//
// Clear the bits in package thermal status register for bit = 1
// in bitmask
//
#[no_mangle]
pub unsafe extern "C" fn thermal_clear_package_intr_status(level: c_int, bit_mask: u64) {
    void thermal_clear_package_intr_status(int level, u64 bit_mask)
    {
    u64 msr_val;
    int msr;
    if (level == CORE_LEVEL) {
    msr  = MSR_IA32_THERM_STATUS;
    msr_val = therm_intr_core_clear_mask;
    } else {
    msr  = MSR_IA32_PACKAGE_THERM_STATUS;
    msr_val = therm_intr_pkg_clear_mask;
    }
    msr_val &= ~bit_mask;
    wrmsrq(msr, msr_val);
    }
    EXPORT_SYMBOL_GPL(thermal_clear_package_intr_status);
#[no_mangle]
unsafe extern "C" fn get_therm_status(level: c_int, proc_hot: *mut bool, temp: *mut u8) {
    static void get_therm_status(int level, bool *proc_hot, u8 *temp)
    {
    int msr;
    u64 msr_val;
    if (level == CORE_LEVEL)
    msr = MSR_IA32_THERM_STATUS;
    else
    msr = MSR_IA32_PACKAGE_THERM_STATUS;
    rdmsrq(msr, msr_val);
    if (msr_val & THERM_STATUS_PROCHOT_LOG)
// proc_hot = true;
    else
// proc_hot = false;
// temp = (msr_val >> 16) & 0x7F;
    }
#[no_mangle]
unsafe extern "C" fn throttle_active_work(work: *mut work_struct) -> void __maybe_unused {
    static void __maybe_unused throttle_active_work(struct work_struct *work)
    {
    struct _thermal_state *state = container_of(to_delayed_work(work),
    struct _thermal_state, therm_work);
    unsigned int i, avg, this_cpu = smp_processor_id();
    let mut now: u64 = get_jiffies_64();
    bool hot;
    u8 temp;
    get_therm_status(state.level, &hot, &temp);
// temperature value is offset from the max so lesser means hotter
    if (!hot && temp > state.baseline_temp) {
    if (state.rate_control_active)
    pr_info("CPU%d: %s temperature/speed normal (total events = %lu)\n",
    this_cpu,
    state.level == CORE_LEVEL ? "Core" : "Package",
    state.count);
    state.rate_control_active = false;
    return;
    }
    if (time_before64(now, state.next_check) &&
    state.rate_control_active)
    goto re_arm;
    state.next_check = now + CHECK_INTERVAL;
    if (state.count != state.last_count) {
// There was one new thermal interrupt
    state.last_count = state.count;
    state.average = 0;
    state.sample_count = 0;
    state.sample_index = 0;
    }
    state.temp_samples[state.sample_index] = temp;
    state.sample_count++;
    state.sample_index = (state.sample_index + 1) % ARRAY_SIZE(state.temp_samples);
    if (state.sample_count < ARRAY_SIZE(state.temp_samples))
    goto re_arm;
    avg = 0;
    for (i = 0; i < ARRAY_SIZE(state.temp_samples); ++i)
    avg += state.temp_samples[i];
    avg /= ARRAY_SIZE(state.temp_samples);
    if (state.average > avg) {
    pr_warn("CPU%d: %s temperature is above threshold, cpu clock is throttled (total events = %lu)\n",
    this_cpu,
    state.level == CORE_LEVEL ? "Core" : "Package",
    state.count);
    state.rate_control_active = true;
    }
    state.average = avg;
    re_arm:
    thermal_clear_package_intr_status(state.level, THERM_STATUS_PROCHOT_LOG);
    schedule_delayed_work_on(this_cpu, &state.therm_work, THERM_THROT_POLL_INTERVAL);
    }
//
// therm_throt_process - Process thermal throttling event from interrupt
// @curr: Whether the condition is current or not (boolean), since the
// thermal interrupt normally gets called both when the thermal
// event begins and once the event has ended.
//
// This function is called by the thermal interrupt after the
// IRQ has been acknowledged.
//
// It will take care of rate limiting and printing messages to the syslog.
//
#[no_mangle]
unsafe extern "C" fn therm_throt_process(new_event: bool, event: c_int, level: c_int) {
    static void therm_throt_process(bool new_event, int event, int level)
    {
    struct _thermal_state *state;
    let mut this_cpu: c_uint = smp_processor_id();
    bool old_event;
    u64 now;
    struct thermal_state *pstate = &per_cpu(thermal_state, this_cpu);
    now = get_jiffies_64();
    if (level == CORE_LEVEL) {
    if (event == THERMAL_THROTTLING_EVENT)
    state = &pstate.core_throttle;
#[no_mangle]
pub unsafe extern "C" fn if(POWER_LIMIT_EVENT: event ==) -> else {
    else if (event == POWER_LIMIT_EVENT)
    state = &pstate.core_power_limit;
    else
    return;
    } else if (level == PACKAGE_LEVEL) {
    if (event == THERMAL_THROTTLING_EVENT)
    state = &pstate.package_throttle;
#[no_mangle]
pub unsafe extern "C" fn if(POWER_LIMIT_EVENT: event ==) -> else {
    else if (event == POWER_LIMIT_EVENT)
    state = &pstate.package_power_limit;
    else
    return;
    } else
    return;
    old_event = state.new_event;
    state.new_event = new_event;
    if (new_event)
    state.count++;
    if (event != THERMAL_THROTTLING_EVENT)
    return;
    if (new_event && !state.last_interrupt_time) {
    bool hot;
    u8 temp;
    get_therm_status(state.level, &hot, &temp);
//
// Ignore short temperature spike as the system is not close
// to PROCHOT. 10C offset is large enough to ignore. It is
// already dropped from the high threshold temperature.
//
    if (temp > 10)
    return;
    state.baseline_temp = temp;
    state.last_interrupt_time = now;
    schedule_delayed_work_on(this_cpu, &state.therm_work, THERM_THROT_POLL_INTERVAL);
    } else if (old_event && state.last_interrupt_time) {
    unsigned long throttle_time;
    throttle_time = jiffies_delta_to_msecs(now - state.last_interrupt_time);
    if (throttle_time > state.max_time_ms)
    state.max_time_ms = throttle_time;
    state.total_time_ms += throttle_time;
    state.last_interrupt_time = 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn thresh_event_valid(level: c_int, event: c_int) -> c_int {
    static int thresh_event_valid(int level, int event)
    {
    struct _thermal_state *state;
    let mut this_cpu: c_uint = smp_processor_id();
    struct thermal_state *pstate = &per_cpu(thermal_state, this_cpu);
    let mut now: u64 = get_jiffies_64();
    if (level == PACKAGE_LEVEL)
    state = (event == 0) ? &pstate.pkg_thresh0 :
    &pstate.pkg_thresh1;
    else
    state = (event == 0) ? &pstate.core_thresh0 :
    &pstate.core_thresh1;
    if (time_before64(now, state.next_check))
    return 0;
    state.next_check = now + CHECK_INTERVAL;
    return 1;
    }
    static bool int_pln_enable;
#[no_mangle]
unsafe extern "C" fn int_pln_enable_setup(s: *mut c_char) -> int __init {
    static int __init int_pln_enable_setup(char *s)
    {
    int_pln_enable = true;
    return 1;
    }
    __setup("int_pln_enable", int_pln_enable_setup);

// Add/Remove thermal_throttle interface for CPU device:
#[no_mangle]
unsafe extern "C" fn thermal_throttle_add_dev(dev: *mut device, cpu: c_uint) -> c_int {
    static int thermal_throttle_add_dev(struct device *dev, unsigned int cpu)
    {
    int err;
    struct cpuinfo_x86 *c = &cpu_data(cpu);
    err = sysfs_create_group(&dev.kobj, &thermal_attr_group);
    if (err)
    return err;
    if (cpu_has(c, X86_FEATURE_PLN) && int_pln_enable) {
    err = sysfs_add_file_to_group(&dev.kobj,
    &dev_attr_core_power_limit_count.attr,
    thermal_attr_group.name);
    if (err)
    goto del_group;
    }
    if (cpu_has(c, X86_FEATURE_PTS)) {
    err = sysfs_add_file_to_group(&dev.kobj,
    &dev_attr_package_throttle_count.attr,
    thermal_attr_group.name);
    if (err)
    goto del_group;
    err = sysfs_add_file_to_group(&dev.kobj,
    &dev_attr_package_throttle_max_time_ms.attr,
    thermal_attr_group.name);
    if (err)
    goto del_group;
    err = sysfs_add_file_to_group(&dev.kobj,
    &dev_attr_package_throttle_total_time_ms.attr,
    thermal_attr_group.name);
    if (err)
    goto del_group;
    if (cpu_has(c, X86_FEATURE_PLN) && int_pln_enable) {
    err = sysfs_add_file_to_group(&dev.kobj,
    &dev_attr_package_power_limit_count.attr,
    thermal_attr_group.name);
    if (err)
    goto del_group;
    }
    }
    return 0;
    del_group:
    sysfs_remove_group(&dev.kobj, &thermal_attr_group);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn thermal_throttle_remove_dev(dev: *mut device) {
    static void thermal_throttle_remove_dev(struct device *dev)
    {
    sysfs_remove_group(&dev.kobj, &thermal_attr_group);
    }
#[no_mangle]
unsafe extern "C" fn check_directed_thermal_pkg_intr_ack() -> c_int {
    static int check_directed_thermal_pkg_intr_ack(void)
    {
    let mut count: c_uint = 15000;
    u64 msr_val;
//
// Hardware acknowledges the directed interrupt setup in 10ms or less.
// Wait 15ms to be safe.
//
    do {
    rdmsrq(MSR_IA32_PACKAGE_THERM_STATUS, msr_val);
    udelay(1);
    } while (!(msr_val & PACKAGE_THERM_STATUS_DPTI_ACK) && --count);
    if (!count)
    return -ETIMEDOUT;
    thermal_clear_package_intr_status(PACKAGE_LEVEL,
    PACKAGE_THERM_STATUS_DPTI_ACK);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn config_directed_thermal_pkg_intr(info: *mut c_void) {
    static void config_directed_thermal_pkg_intr(void *info)
    {
    let mut enable: bool = *((bool *)info);
    u64 msr_val;
    rdmsrq(MSR_IA32_THERM_INTERRUPT, msr_val);
    if (enable)
    msr_val |= THERM_INT_DPTI_ENABLE;
    else
    msr_val &= ~THERM_INT_DPTI_ENABLE;
    wrmsrq(MSR_IA32_THERM_INTERRUPT, msr_val);
    }
//
// Accessed from CPU hotplug callbacks and from code that runs while CPU
// hotplug is inactive: the init and cleanup paths as well as syscore callbacks.
// No extra locking needed.
//
    static unsigned int *directed_intr_handler_cpus;
#[no_mangle]
unsafe extern "C" fn directed_thermal_pkg_intr_supported() -> bool {
    static bool directed_thermal_pkg_intr_supported(void)
    {
    if (!boot_cpu_has(X86_FEATURE_DPTI))
    return false;
    if (!directed_intr_handler_cpus)
    return false;
    return true;
    }
//
// Must be called with cpu_hotplug_lock held to prevent CPUs from going offline
// while iterating through packages and interrupts must be enabled to avoid
// deadlocks in SMP function calls. The syscore shutdown callback also calls
// this function, but runs with CPU hotplug disabled (and interrupts enabled).
//
#[no_mangle]
unsafe extern "C" fn disable_directed_thermal_pkg_intr_all() {
    static void disable_directed_thermal_pkg_intr_all(void)
    {
    let mut enable: bool = false;
    int i;
    if (!directed_thermal_pkg_intr_supported())
    return;
    for (i = 0; i < topology_max_packages(); i++) {
    if (directed_intr_handler_cpus[i] == nr_cpu_ids)
    continue;
    smp_call_function_single(directed_intr_handler_cpus[i],
    config_directed_thermal_pkg_intr,
    &enable, true);
    }
    }
#[no_mangle]
unsafe extern "C" fn enable_directed_thermal_pkg_intr(cpu: c_uint) -> c_int {
    static int enable_directed_thermal_pkg_intr(unsigned int cpu)
    {
    let mut enable: bool = true;
    u16 pkg_id;
    if (!directed_thermal_pkg_intr_supported())
    return 0;
    pkg_id = topology_logical_package_id(cpu);
    if (pkg_id >= topology_max_packages())
    return -EINVAL;
// Another CPU in this package already handles the directed interrupt.
    if (directed_intr_handler_cpus[pkg_id] != nr_cpu_ids)
    return 0;
    thermal_clear_package_intr_status(PACKAGE_LEVEL,
    PACKAGE_THERM_STATUS_DPTI_ACK);
    config_directed_thermal_pkg_intr(&enable);
    if (!check_directed_thermal_pkg_intr_ack()) {
    directed_intr_handler_cpus[pkg_id] = cpu;
    return 0;
    }
//
// A failure indicates faulty hardware. Roll back completely so that
// no other CPU tries. This is especially important during boot as all
// CPUs may come online and would otherwise keep trying.
//
    enable = false;
    config_directed_thermal_pkg_intr(&enable);
    return -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn disable_directed_thermal_pkg_intr(cpu: c_uint) {
    static void disable_directed_thermal_pkg_intr(unsigned int cpu)
    {
    unsigned int new_cpu;
    bool enable;
    u16 pkg_id;
    if (!directed_thermal_pkg_intr_supported())
    return;
    pkg_id = topology_logical_package_id(cpu);
    if (pkg_id >= topology_max_packages())
    return;
// Not the CPU handling the directed interrupt.
    if (directed_intr_handler_cpus[pkg_id] != cpu)
    return;
//
// The package-level interrupt must remain directed after this CPU goes
// offline.
//
    new_cpu = cpumask_any_but(topology_core_cpumask(cpu), cpu);
    if (new_cpu < nr_cpu_ids) {
    enable = true;
    thermal_clear_package_intr_status(PACKAGE_LEVEL,
    PACKAGE_THERM_STATUS_DPTI_ACK);
//
// We are here via CPU hotplug. Since we are holding the
// cpu_hotplug_lock, @new_cpu cannot go offline and interrupts
// are enabled, so the SMP function call is safe.
//
// The syscore suspend callback runs with interrupts disabled,
// but it does not reach this path because all the secondary
// CPUs are offline.
//
    smp_call_function_single(new_cpu, config_directed_thermal_pkg_intr,
    &enable, true);
    }
//
// If hardware does not acknowledge the directed interrupt setup on
// @new_cpu, disable the redirection. Since no other CPU is configured
// to receive the package-level interrupt, all CPUs in the package will
// receive it.
//
    enable = false;
    if (new_cpu < nr_cpu_ids && check_directed_thermal_pkg_intr_ack()) {
    smp_call_function_single(new_cpu, config_directed_thermal_pkg_intr,
    &enable, true);
    pr_warn_once("Failed to redirect package thermal interrupt from CPU%u to CPU%u; reverting to broadcast.\n",
    cpu, new_cpu);
    new_cpu = nr_cpu_ids;
    }
//
// Clear the directed interrupt on @cpu. Hardware acknowledgment can be
// ignored since @cpu is going offline.
//
    config_directed_thermal_pkg_intr(&enable);
    directed_intr_handler_cpus[pkg_id] = (new_cpu < nr_cpu_ids) ? new_cpu : nr_cpu_ids;
    }
//
// CPU0 may be handling the directed interrupt, but the CPU hotplug callbacks
// are not called for CPU0 during suspend and resume.
//
#[no_mangle]
unsafe extern "C" fn directed_pkg_intr_syscore_resume(data: *mut c_void) {
    static void directed_pkg_intr_syscore_resume(void *data)
    {
//
// We can't do anything to handle errors. If direction fails for CPU0,
// another CPU will take over or disable direction entirely during CPU
// hotplug.
//
    enable_directed_thermal_pkg_intr(0);
    }
#[no_mangle]
unsafe extern "C" fn directed_pkg_intr_syscore_suspend(data: *mut c_void) -> c_int {
    static int directed_pkg_intr_syscore_suspend(void *data)
    {
    disable_directed_thermal_pkg_intr(0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn directed_pkg_intr_syscore_shutdown(data: *mut c_void) {
    static void directed_pkg_intr_syscore_shutdown(void *data)
    {
    disable_directed_thermal_pkg_intr_all();
    }
    static const struct syscore_ops directed_pkg_intr_pm_ops = {
    .resume = directed_pkg_intr_syscore_resume,
    .suspend = directed_pkg_intr_syscore_suspend,
    .shutdown = directed_pkg_intr_syscore_shutdown,
    };
    static struct syscore directed_pkg_intr_pm = {
    .ops = &directed_pkg_intr_pm_ops,
    };
#[no_mangle]
unsafe extern "C" fn init_directed_pkg_intr() -> __init void {
    static __init void init_directed_pkg_intr(void)
    {
    int i;
    if (!boot_cpu_has(X86_FEATURE_DPTI))
    return;
    directed_intr_handler_cpus = kmalloc_array(topology_max_packages(),
    sizeof(*directed_intr_handler_cpus),
    GFP_KERNEL);
    if (!directed_intr_handler_cpus)
    return;
    for (i = 0; i < topology_max_packages(); i++)
    directed_intr_handler_cpus[i] = nr_cpu_ids;
    register_syscore(&directed_pkg_intr_pm);
    }
#[no_mangle]
unsafe extern "C" fn cleanup_directed_pkg_thermal_intr() {
    static void cleanup_directed_pkg_thermal_intr(void)
    {
    if (!directed_thermal_pkg_intr_supported())
    return;
    unregister_syscore(&directed_pkg_intr_pm);
    disable_directed_thermal_pkg_intr_all();
    kfree(directed_intr_handler_cpus);
    directed_intr_handler_cpus = core::ptr::null_mut();
    }
// Get notified when a cpu comes on/off. Be hotplug friendly.
#[no_mangle]
unsafe extern "C" fn thermal_throttle_online(cpu: c_uint) -> c_int {
    static int thermal_throttle_online(unsigned int cpu)
    {
    struct thermal_state *state = &per_cpu(thermal_state, cpu);
    struct device *dev = get_cpu_device(cpu);
    int err;
    u32 l;
    err = thermal_throttle_add_dev(dev, cpu);
    if (err)
    return err;
    state.package_throttle.level = PACKAGE_LEVEL;
    state.core_throttle.level = CORE_LEVEL;
    INIT_DELAYED_WORK(&state.package_throttle.therm_work, throttle_active_work);
    INIT_DELAYED_WORK(&state.core_throttle.therm_work, throttle_active_work);
//
// The first CPU coming online will enable the HFI. Usually this causes
// hardware to issue an HFI thermal interrupt. Such interrupt will reach
// the CPU once we enable the thermal vector in the local APIC.
//
    intel_hfi_online(cpu);
    if (enable_directed_thermal_pkg_intr(cpu)) {
    pr_info_once("Failed to direct package thermal interrupts. All CPUs will receive it.\n");
    cleanup_directed_pkg_thermal_intr();
    }
// Unmask the thermal vector after the above workqueues are initialized.
    l = apic_read(APIC_LVTTHMR);
    apic_write(APIC_LVTTHMR, l & ~APIC_LVT_MASKED);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn thermal_throttle_offline(cpu: c_uint) -> c_int {
    static int thermal_throttle_offline(unsigned int cpu)
    {
    struct thermal_state *state = &per_cpu(thermal_state, cpu);
    struct device *dev = get_cpu_device(cpu);
    u32 l;
// Mask the thermal vector before draining evtl. pending work
    l = apic_read(APIC_LVTTHMR);
    apic_write(APIC_LVTTHMR, l | APIC_LVT_MASKED);
    disable_directed_thermal_pkg_intr(cpu);
    intel_hfi_offline(cpu);
    cancel_delayed_work_sync(&state.package_throttle.therm_work);
    cancel_delayed_work_sync(&state.core_throttle.therm_work);
    state.package_throttle.rate_control_active = false;
    state.core_throttle.rate_control_active = false;
    thermal_throttle_remove_dev(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn thermal_throttle_init_device() -> __init int {
    static __init int thermal_throttle_init_device(void)
    {
    int ret;
    if (!atomic_read(&therm_throt_en))
    return 0;
    init_directed_pkg_intr();
    intel_hfi_init();
    ret = cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, "x86/therm:online",
    thermal_throttle_online,
    thermal_throttle_offline);
    if (ret >= 0)
    return 0;
    cleanup_directed_pkg_thermal_intr();
    return ret;
    }
    device_initcall(thermal_throttle_init_device);

#[no_mangle]
unsafe extern "C" fn notify_package_thresholds(msr_val: __u64) {
    static void notify_package_thresholds(__u64 msr_val)
    {
    let mut notify_thres_0: bool = false;
    let mut notify_thres_1: bool = false;
    if (!platform_thermal_package_notify)
    return;
// lower threshold check
    if (msr_val & THERM_LOG_THRESHOLD0)
    notify_thres_0 = true;
// higher threshold check
    if (msr_val & THERM_LOG_THRESHOLD1)
    notify_thres_1 = true;
    if (!notify_thres_0 && !notify_thres_1)
    return;
    if (platform_thermal_package_rate_control &&
    platform_thermal_package_rate_control()) {
// Rate control is implemented in callback
    platform_thermal_package_notify(msr_val);
    return;
    }
// lower threshold reached
    if (notify_thres_0 && thresh_event_valid(PACKAGE_LEVEL, 0))
    platform_thermal_package_notify(msr_val);
// higher threshold reached
    if (notify_thres_1 && thresh_event_valid(PACKAGE_LEVEL, 1))
    platform_thermal_package_notify(msr_val);
    }
#[no_mangle]
unsafe extern "C" fn notify_thresholds(msr_val: __u64) {
    static void notify_thresholds(__u64 msr_val)
    {
// check whether the interrupt handler is defined;
// otherwise simply return
//
    if (!platform_thermal_notify)
    return;
// lower threshold reached
    if ((msr_val & THERM_LOG_THRESHOLD0) &&
    thresh_event_valid(CORE_LEVEL, 0))
    platform_thermal_notify(msr_val);
// higher threshold reached
    if ((msr_val & THERM_LOG_THRESHOLD1) &&
    thresh_event_valid(CORE_LEVEL, 1))
    platform_thermal_notify(msr_val);
    }
#[no_mangle]
pub unsafe extern "C" fn notify_hwp_interrupt() -> void __weak {
    void __weak notify_hwp_interrupt(void)
    {
    wrmsrq_safe(MSR_HWP_STATUS, 0);
    }
// Thermal transition interrupt handler
#[no_mangle]
pub unsafe extern "C" fn intel_thermal_interrupt() {
    void intel_thermal_interrupt(void)
    {
    __u64 msr_val;
    if (cpu_feature_enabled(X86_FEATURE_HWP))
    notify_hwp_interrupt();
    rdmsrq(MSR_IA32_THERM_STATUS, msr_val);
// Check for violation of core thermal thresholds
    notify_thresholds(msr_val);
    therm_throt_process(msr_val & THERM_STATUS_PROCHOT,
    THERMAL_THROTTLING_EVENT,
    CORE_LEVEL);
    if (this_cpu_has(X86_FEATURE_PLN) && int_pln_enable)
    therm_throt_process(msr_val & THERM_STATUS_POWER_LIMIT,
    POWER_LIMIT_EVENT,
    CORE_LEVEL);
    if (this_cpu_has(X86_FEATURE_PTS)) {
    rdmsrq(MSR_IA32_PACKAGE_THERM_STATUS, msr_val);
// check violations of package thermal thresholds
    notify_package_thresholds(msr_val);
    therm_throt_process(msr_val & PACKAGE_THERM_STATUS_PROCHOT,
    THERMAL_THROTTLING_EVENT,
    PACKAGE_LEVEL);
    if (this_cpu_has(X86_FEATURE_PLN) && int_pln_enable)
    therm_throt_process(msr_val &
    PACKAGE_THERM_STATUS_POWER_LIMIT,
    POWER_LIMIT_EVENT,
    PACKAGE_LEVEL);
    if (this_cpu_has(X86_FEATURE_HFI))
    intel_hfi_process_event(msr_val &
    PACKAGE_THERM_STATUS_HFI_UPDATED);
    }
    }
// Thermal monitoring depends on APIC, ACPI and clock modulation
#[no_mangle]
unsafe extern "C" fn intel_thermal_supported(c: *mut cpuinfo_x86) -> c_int {
    static int intel_thermal_supported(struct cpuinfo_x86 *c)
    {
    if (!boot_cpu_has(X86_FEATURE_APIC))
    return 0;
    if (!cpu_has(c, X86_FEATURE_ACPI) || !cpu_has(c, X86_FEATURE_ACC))
    return 0;
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn x86_thermal_enabled() -> bool {
    bool x86_thermal_enabled(void)
    {
    return atomic_read(&therm_throt_en);
    }
#[no_mangle]
pub unsafe extern "C" fn therm_lvt_init() -> void __init {
    void __init therm_lvt_init(void)
    {
//
// This function is only called on boot CPU. Save the init thermal
// LVT value on BSP and use that value to restore APs' thermal LVT
// entry BIOS programmed later
//
    if (intel_thermal_supported(&boot_cpu_data))
    lvtthmr_init = apic_read(APIC_LVTTHMR);
    }
#[no_mangle]
pub unsafe extern "C" fn intel_init_thermal(c: *mut cpuinfo_x86) {
    void intel_init_thermal(struct cpuinfo_x86 *c)
    {
    let mut cpu: c_uint = smp_processor_id();
    struct msr val;
    let mut tm2: c_int = 0;
    if (!intel_thermal_supported(c))
    return;
//
// First check if its enabled already, in which case there might
// be some SMM goo which handles it, so we can't even put a handler
// since it might be delivered via SMI already:
//
    rdmsrq(MSR_IA32_MISC_ENABLE, val.q);
    val.h = lvtthmr_init;
//
// The initial value of thermal LVT entries on all APs always reads
// 0x10000 because APs are woken up by BSP issuing INIT-SIPI-SIPI
// sequence to them and LVT registers are reset to 0s except for
// the mask bits which are set to 1s when APs receive INIT IPI.
// If BIOS takes over the thermal interrupt and sets its interrupt
// delivery mode to SMI (not fixed), it restores the value that the
// BIOS has programmed on AP based on BSP's info we saved since BIOS
// is always setting the same value for all threads/cores.
//
    if ((val.h & APIC_DM_FIXED_MASK) != APIC_DM_FIXED)
    apic_write(APIC_LVTTHMR, lvtthmr_init);
    if ((val.l & MSR_IA32_MISC_ENABLE_TM1) && (val.h & APIC_DM_SMI)) {
    if (system_state == SYSTEM_BOOTING)
    pr_debug("CPU%d: Thermal monitoring handled by SMI\n", cpu);
    return;
    }
// early Pentium M models use different method for enabling TM2
    if (cpu_has(c, X86_FEATURE_TM2)) {
    if (c.x86 == 6 && (c.x86_model == 9 || c.x86_model == 13)) {
    rdmsrq(MSR_THERM2_CTL, val.q);
    if (val.l & MSR_THERM2_CTL_TM_SELECT)
    tm2 = 1;
    } else if (val.l & MSR_IA32_MISC_ENABLE_TM2)
    tm2 = 1;
    }
// We'll mask the thermal vector in the lapic till we're ready:
    val.h = THERMAL_APIC_VECTOR | APIC_DM_FIXED | APIC_LVT_MASKED;
    apic_write(APIC_LVTTHMR, val.h);
    thermal_intr_init_core_clear_mask();
    thermal_intr_init_pkg_clear_mask();
    rdmsrq(MSR_IA32_THERM_INTERRUPT, val.q);
    if (cpu_has(c, X86_FEATURE_PLN) && !int_pln_enable) {
    val.l |= THERM_INT_LOW_ENABLE | THERM_INT_HIGH_ENABLE;
    val.l &= ~THERM_INT_PLN_ENABLE;
    } else if (cpu_has(c, X86_FEATURE_PLN) && int_pln_enable)
    val.l |= THERM_INT_LOW_ENABLE | THERM_INT_HIGH_ENABLE |
    THERM_INT_PLN_ENABLE;
    else
    val.l |= THERM_INT_LOW_ENABLE | THERM_INT_HIGH_ENABLE;
    wrmsrq(MSR_IA32_THERM_INTERRUPT, val.q);
    if (cpu_has(c, X86_FEATURE_PTS)) {
    rdmsrq(MSR_IA32_PACKAGE_THERM_INTERRUPT, val.q);
    if (cpu_has(c, X86_FEATURE_PLN) && !int_pln_enable) {
    val.l |= PACKAGE_THERM_INT_LOW_ENABLE |
    PACKAGE_THERM_INT_HIGH_ENABLE;
    val.l &= ~PACKAGE_THERM_INT_PLN_ENABLE;
    } else if (cpu_has(c, X86_FEATURE_PLN) && int_pln_enable)
    val.l |= PACKAGE_THERM_INT_LOW_ENABLE |
    PACKAGE_THERM_INT_HIGH_ENABLE |
    PACKAGE_THERM_INT_PLN_ENABLE;
    else
    val.l |= PACKAGE_THERM_INT_LOW_ENABLE |
    PACKAGE_THERM_INT_HIGH_ENABLE;
    wrmsrq(MSR_IA32_PACKAGE_THERM_INTERRUPT, val.q);
    if (cpu_has(c, X86_FEATURE_HFI)) {
    rdmsrq(MSR_IA32_PACKAGE_THERM_INTERRUPT, val.q);
    wrmsrq(MSR_IA32_PACKAGE_THERM_INTERRUPT,
    val.q | PACKAGE_THERM_INT_HFI_ENABLE);
    }
    }
    rdmsrq(MSR_IA32_MISC_ENABLE, val.q);
    wrmsrq(MSR_IA32_MISC_ENABLE, val.q | MSR_IA32_MISC_ENABLE_TM1);
    pr_info_once("CPU0: Thermal monitoring enabled (%s)\n",
    tm2 ? "TM2" : "TM1");
// enable thermal throttle processing
    atomic_set(&therm_throt_en, 1);
    }
