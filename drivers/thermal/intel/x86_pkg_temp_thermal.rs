//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/intel/x86_pkg_temp_thermal.c
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
// x86_pkg_temp_thermal driver
// Copyright (c) 2013, Intel Corporation.
//

//
// Rate control delay: Idea is to introduce denounce effect
// This should be long enough to avoid reduce events, when
// threshold is set to a temperature, which is constantly
// violated, but at the short enough to take any action.
// The action can be remove threshold or change it to next
// interesting setting. Based on experiments, in around
// every 5 seconds under load will give us a significant
// temperature change.
//
pub const PKG_TEMP_THERMAL_NOTIFY_DELAY: c_int = 5000;
    let mut notify_delay_ms: static int = PKG_TEMP_THERMAL_NOTIFY_DELAY;
    module_param(notify_delay_ms, int, 0644);
    MODULE_PARM_DESC(notify_delay_ms,
    "User space notification delay in milli seconds.");
// Number of trip points in thermal zone. Currently it can't
// be more than 2. MSR can allow setting and getting notifications
// for only 2 thresholds. This define enforces this, if there
// is some wrong values returned by cpuid for number of thresholds.
//
pub const MAX_NUMBER_OF_TRIPS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zone_device {
    pub cpu: c_int,
    pub work_scheduled: bool,
    pub msr_pkg_therm: u64,
    pub work: delayed_work,
    pub tzone: *mut thermal_zone_device,
    pub cpumask: cpumask,
}

    static struct thermal_zone_params pkg_temp_tz_params = {
    .no_hwmon	= true,
    };
// Keep track of how many zone pointers we allocated in init()
    static int max_id __read_mostly;
// Array of zone pointers
    static struct zone_device **zones;
// Serializes interrupt notification, work and hotplug
    static DEFINE_RAW_SPINLOCK(pkg_temp_lock);
// Protects zone operation in the work function against hotplug removal
    static DEFINE_MUTEX(thermal_zone_mutex);
// The dynamically assigned cpu hotplug state for module_exit()
    static enum cpuhp_state pkg_thermal_hp_state __read_mostly;
// Debug counters to show using debugfs
    static struct dentry *debugfs;
    static unsigned int pkg_interrupt_cnt;
    static unsigned int pkg_work_cnt;
#[no_mangle]
unsafe extern "C" fn pkg_temp_debugfs_init() {
    static void pkg_temp_debugfs_init(void)
    {
    debugfs = debugfs_create_dir("pkg_temp_thermal", core::ptr::null_mut());
    debugfs_create_u32("pkg_thres_interrupt", S_IRUGO, debugfs,
    &pkg_interrupt_cnt);
    debugfs_create_u32("pkg_thres_work", S_IRUGO, debugfs,
    &pkg_work_cnt);
    }
//
// Protection:
//
// - cpu hotplug: Read serialized by cpu hotplug lock
// Write must hold pkg_temp_lock
//
// - Other callsites: Must hold pkg_temp_lock
//
    static struct zone_device *pkg_temp_thermal_get_dev(unsigned int cpu)
    {
    let mut id: c_int = topology_logical_die_id(cpu);
    if (id >= 0 && id < max_id)
    return zones[id];
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn sys_get_curr_temp(tzd: *mut thermal_zone_device, temp: *mut c_int) -> c_int {
    static int sys_get_curr_temp(struct thermal_zone_device *tzd, int *temp)
    {
    struct zone_device *zonedev = thermal_zone_device_priv(tzd);
    int val, ret;
    ret = intel_tcc_get_temp(zonedev.cpu, &val, true);
    if (ret < 0)
    return ret;
// temp = val * 1000;
    pr_debug("sys_get_curr_temp %d\n", *temp);
    return 0;
    }
    static int
    sys_set_trip_temp(struct thermal_zone_device *tzd,
    const struct thermal_trip *trip, int temp)
    {
    struct zone_device *zonedev = thermal_zone_device_priv(tzd);
    let mut trip_index: c_uint = THERMAL_TRIP_PRIV_TO_INT(trip.priv);
    u32 mask, shift, intr;
    int tj_max, val, ret;
    struct msr v;
    if (temp == THERMAL_TEMP_INVALID)
    temp = 0;
    tj_max = intel_tcc_get_tjmax(zonedev.cpu);
    if (tj_max < 0)
    return tj_max;
    tj_max *= 1000;
    val = (tj_max - temp)/1000;
    if (trip_index >= MAX_NUMBER_OF_TRIPS || val < 0 || val > 0x7f)
    return -EINVAL;
    ret = rdmsrq_on_cpu(zonedev.cpu, MSR_IA32_PACKAGE_THERM_INTERRUPT, &v.q);
    if (ret < 0)
    return ret;
    if (trip_index) {
    mask = THERM_MASK_THRESHOLD1;
    shift = THERM_SHIFT_THRESHOLD1;
    intr = THERM_INT_THRESHOLD1_ENABLE;
    } else {
    mask = THERM_MASK_THRESHOLD0;
    shift = THERM_SHIFT_THRESHOLD0;
    intr = THERM_INT_THRESHOLD0_ENABLE;
    }
    v.l &= ~mask;
//
// When users space sets a trip temperature == 0, which is indication
// that, it is no longer interested in receiving notifications.
//
    if (!temp) {
    v.l &= ~intr;
    } else {
    v.l |= val << shift;
    v.l |= intr;
    }
    return wrmsrq_on_cpu(zonedev.cpu, MSR_IA32_PACKAGE_THERM_INTERRUPT, v.q);
    }
// Thermal zone callback registry
    static const struct thermal_zone_device_ops tzone_ops = {
    .get_temp = sys_get_curr_temp,
    .set_trip_temp = sys_set_trip_temp,
    };
#[no_mangle]
unsafe extern "C" fn pkg_thermal_rate_control() -> bool {
    static bool pkg_thermal_rate_control(void)
    {
    return true;
    }
// Enable threshold interrupt on local package/cpu
#[no_mangle]
pub unsafe extern "C" fn enable_pkg_thres_interrupt() {
    static inline void enable_pkg_thres_interrupt(void)
    {
    u8 thres_0, thres_1;
    struct msr val;
    rdmsrq(MSR_IA32_PACKAGE_THERM_INTERRUPT, val.q);
// only enable/disable if it had valid threshold value
    thres_0 = (val.l & THERM_MASK_THRESHOLD0) >> THERM_SHIFT_THRESHOLD0;
    thres_1 = (val.l & THERM_MASK_THRESHOLD1) >> THERM_SHIFT_THRESHOLD1;
    if (thres_0)
    val.l |= THERM_INT_THRESHOLD0_ENABLE;
    if (thres_1)
    val.l |= THERM_INT_THRESHOLD1_ENABLE;
    wrmsrq(MSR_IA32_PACKAGE_THERM_INTERRUPT, val.q);
    }
// Disable threshold interrupt on local package/cpu
#[no_mangle]
pub unsafe extern "C" fn disable_pkg_thres_interrupt() {
    static inline void disable_pkg_thres_interrupt(void)
    {
    struct msr val;
    rdmsrq(MSR_IA32_PACKAGE_THERM_INTERRUPT, val.q);
    val.l &= ~(THERM_INT_THRESHOLD0_ENABLE | THERM_INT_THRESHOLD1_ENABLE);
    wrmsrq(MSR_IA32_PACKAGE_THERM_INTERRUPT, val.q);
    }
#[no_mangle]
unsafe extern "C" fn pkg_temp_thermal_threshold_work_fn(work: *mut work_struct) {
    static void pkg_temp_thermal_threshold_work_fn(struct work_struct *work)
    {
    struct thermal_zone_device *tzone = core::ptr::null_mut();
    let mut cpu: c_int = smp_processor_id();
    struct zone_device *zonedev;
    mutex_lock(&thermal_zone_mutex);
    raw_spin_lock_irq(&pkg_temp_lock);
    ++pkg_work_cnt;
    zonedev = pkg_temp_thermal_get_dev(cpu);
    if (!zonedev) {
    raw_spin_unlock_irq(&pkg_temp_lock);
    mutex_unlock(&thermal_zone_mutex);
    return;
    }
    zonedev.work_scheduled = false;
    thermal_clear_package_intr_status(PACKAGE_LEVEL, THERM_LOG_THRESHOLD0 | THERM_LOG_THRESHOLD1);
    tzone = zonedev.tzone;
    enable_pkg_thres_interrupt();
    raw_spin_unlock_irq(&pkg_temp_lock);
//
// If tzone is not NULL, then thermal_zone_mutex will prevent the
// concurrent removal in the cpu offline callback.
//
    if (tzone)
    thermal_zone_device_update(tzone, THERMAL_EVENT_UNSPECIFIED);
    mutex_unlock(&thermal_zone_mutex);
    }
#[no_mangle]
unsafe extern "C" fn pkg_thermal_schedule_work(cpu: c_int, work: *mut delayed_work) {
    static void pkg_thermal_schedule_work(int cpu, struct delayed_work *work)
    {
    let mut ms: c_ulong = msecs_to_jiffies(notify_delay_ms);
    schedule_delayed_work_on(cpu, work, ms);
    }
#[no_mangle]
unsafe extern "C" fn pkg_thermal_notify(msr_val: u64) -> c_int {
    static int pkg_thermal_notify(u64 msr_val)
    {
    let mut cpu: c_int = smp_processor_id();
    struct zone_device *zonedev;
    unsigned long flags;
    raw_spin_lock_irqsave(&pkg_temp_lock, flags);
    ++pkg_interrupt_cnt;
    disable_pkg_thres_interrupt();
// Work is per package, so scheduling it once is enough.
    zonedev = pkg_temp_thermal_get_dev(cpu);
    if (zonedev && !zonedev.work_scheduled) {
    zonedev.work_scheduled = true;
    pkg_thermal_schedule_work(zonedev.cpu, &zonedev.work);
    }
    raw_spin_unlock_irqrestore(&pkg_temp_lock, flags);
    return 0;
    }
    static int pkg_temp_thermal_trips_init(int cpu, int tj_max,
    struct thermal_trip *trips, int num_trips)
    {
    unsigned long thres_reg_value;
    u32 mask, shift;
    struct msr val;
    int ret, i;
    for (i = 0; i < num_trips; i++) {
    if (i) {
    mask = THERM_MASK_THRESHOLD1;
    shift = THERM_SHIFT_THRESHOLD1;
    } else {
    mask = THERM_MASK_THRESHOLD0;
    shift = THERM_SHIFT_THRESHOLD0;
    }
    ret = rdmsrq_on_cpu(cpu, MSR_IA32_PACKAGE_THERM_INTERRUPT, &val.q);
    if (ret < 0)
    return ret;
    thres_reg_value = (val.l & mask) >> shift;
    trips[i].temperature = thres_reg_value ?
    tj_max - thres_reg_value * 1000 : THERMAL_TEMP_INVALID;
    trips[i].type = THERMAL_TRIP_PASSIVE;
    trips[i].flags |= THERMAL_TRIP_FLAG_RW_TEMP;
    trips[i].priv = THERMAL_INT_TO_TRIP_PRIV(i);
    pr_debug("%s: cpu=%d, trip=%d, temp=%d\n",
    __func__, cpu, i, trips[i].temperature);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pkg_temp_thermal_device_add(cpu: c_uint) -> c_int {
    static int pkg_temp_thermal_device_add(unsigned int cpu)
    {
    struct thermal_trip trips[MAX_NUMBER_OF_TRIPS] = { 0 };
    let mut id: c_int = topology_logical_die_id(cpu);
    u32 eax, ebx, ecx, edx;
    struct zone_device *zonedev;
    int thres_count, err;
    int tj_max;
    if (id >= max_id)
    return -ENOMEM;
    cpuid(6, &eax, &ebx, &ecx, &edx);
    thres_count = ebx & 0x07;
    if (!thres_count)
    return -ENODEV;
    thres_count = clamp_val(thres_count, 0, MAX_NUMBER_OF_TRIPS);
    tj_max = intel_tcc_get_tjmax(cpu);
    if (tj_max < 0)
    return tj_max;
    tj_max *= 1000;
    zonedev = kzalloc_obj(*zonedev);
    if (!zonedev)
    return -ENOMEM;
    err = pkg_temp_thermal_trips_init(cpu, tj_max, trips, thres_count);
    if (err)
    goto out_kfree_zonedev;
    INIT_DELAYED_WORK(&zonedev.work, pkg_temp_thermal_threshold_work_fn);
    zonedev.cpu = cpu;
    zonedev.tzone = thermal_zone_device_register_with_trips("x86_pkg_temp",
    trips, thres_count,
    zonedev, &tzone_ops, &pkg_temp_tz_params, 0, 0);
    if (IS_ERR(zonedev.tzone)) {
    err = PTR_ERR(zonedev.tzone);
    goto out_kfree_zonedev;
    }
    err = thermal_zone_device_enable(zonedev.tzone);
    if (err)
    goto out_unregister_tz;
// Store MSR value for package thermal interrupt, to restore at exit
    rdmsrq(MSR_IA32_PACKAGE_THERM_INTERRUPT, zonedev.msr_pkg_therm);
    cpumask_set_cpu(cpu, &zonedev.cpumask);
    raw_spin_lock_irq(&pkg_temp_lock);
    zones[id] = zonedev;
    raw_spin_unlock_irq(&pkg_temp_lock);
    return 0;
    out_unregister_tz:
    thermal_zone_device_unregister(zonedev.tzone);
    out_kfree_zonedev:
    kfree(zonedev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn pkg_thermal_cpu_offline(cpu: c_uint) -> c_int {
    static int pkg_thermal_cpu_offline(unsigned int cpu)
    {
    struct zone_device *zonedev = pkg_temp_thermal_get_dev(cpu);
    bool lastcpu, was_target;
    int target;
    if (!zonedev)
    return 0;
    target = cpumask_any_but(&zonedev.cpumask, cpu);
    cpumask_clear_cpu(cpu, &zonedev.cpumask);
    lastcpu = target >= nr_cpu_ids;
//
// Remove the sysfs files, if this is the last cpu in the package
// before doing further cleanups.
//
    if (lastcpu) {
    struct thermal_zone_device *tzone = zonedev.tzone;
//
// We must protect against a work function calling
// thermal_zone_update, after/while unregister. We null out
// the pointer under the zone mutex, so the worker function
// won't try to call.
//
    mutex_lock(&thermal_zone_mutex);
    zonedev.tzone = core::ptr::null_mut();
    mutex_unlock(&thermal_zone_mutex);
    thermal_zone_device_unregister(tzone);
    }
// Protect against work and interrupts
    raw_spin_lock_irq(&pkg_temp_lock);
//
// Check whether this cpu was the current target and store the new
// one. When we drop the lock, then the interrupt notify function
// will see the new target.
//
    was_target = zonedev.cpu == cpu;
    zonedev.cpu = target;
//
// If this is the last CPU in the package remove the package
// reference from the array and restore the interrupt MSR. When we
// drop the lock neither the interrupt notify function nor the
// worker will see the package anymore.
//
    if (lastcpu) {
    zones[topology_logical_die_id(cpu)] = core::ptr::null_mut();
// After this point nothing touches the MSR anymore.
    wrmsrq(MSR_IA32_PACKAGE_THERM_INTERRUPT,
    zonedev.msr_pkg_therm);
    }
//
// Check whether there is work scheduled and whether the work is
// targeted at the outgoing CPU.
//
    if (zonedev.work_scheduled && was_target) {
//
// To cancel the work we need to drop the lock, otherwise
// we might deadlock if the work needs to be flushed.
//
    raw_spin_unlock_irq(&pkg_temp_lock);
    cancel_delayed_work_sync(&zonedev.work);
    raw_spin_lock_irq(&pkg_temp_lock);
//
// If this is not the last cpu in the package and the work
// did not run after we dropped the lock above, then we
// need to reschedule the work, otherwise the interrupt
// stays disabled forever.
//
    if (!lastcpu && zonedev.work_scheduled)
    pkg_thermal_schedule_work(target, &zonedev.work);
    }
    raw_spin_unlock_irq(&pkg_temp_lock);
// Final cleanup if this is the last cpu
    if (lastcpu)
    kfree(zonedev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pkg_thermal_cpu_online(cpu: c_uint) -> c_int {
    static int pkg_thermal_cpu_online(unsigned int cpu)
    {
    struct zone_device *zonedev = pkg_temp_thermal_get_dev(cpu);
    struct cpuinfo_x86 *c = &cpu_data(cpu);
// Paranoia check
    if (!cpu_has(c, X86_FEATURE_DTHERM) || !cpu_has(c, X86_FEATURE_PTS))
    return -ENODEV;
// If the package exists, nothing to do
    if (zonedev) {
    cpumask_set_cpu(cpu, &zonedev.cpumask);
    return 0;
    }
    return pkg_temp_thermal_device_add(cpu);
    }
    static const struct x86_cpu_id __initconst pkg_temp_thermal_ids[] = {
    X86_MATCH_VENDOR_FEATURE(INTEL, X86_FEATURE_PTS, core::ptr::null_mut()),
    {}
    };
    MODULE_DEVICE_TABLE(x86cpu, pkg_temp_thermal_ids);
#[no_mangle]
unsafe extern "C" fn pkg_temp_thermal_init() -> int __init {
    static int __init pkg_temp_thermal_init(void)
    {
    int ret;
    if (!x86_match_cpu(pkg_temp_thermal_ids))
    return -ENODEV;
    max_id = topology_max_packages() * topology_max_dies_per_package();
    zones = kzalloc_objs(struct zone_device *, max_id);
    if (!zones)
    return -ENOMEM;
    ret = cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, "thermal/x86_pkg:online",
    pkg_thermal_cpu_online,	pkg_thermal_cpu_offline);
    if (ret < 0)
    goto err;
// Store the state for module exit
    pkg_thermal_hp_state = ret;
    platform_thermal_package_notify = pkg_thermal_notify;
    platform_thermal_package_rate_control = pkg_thermal_rate_control;
// Don't care if it fails
    pkg_temp_debugfs_init();
    return 0;
    err:
    kfree(zones);
    return ret;
    }
    module_init(pkg_temp_thermal_init)
#[no_mangle]
unsafe extern "C" fn pkg_temp_thermal_exit() -> void __exit {
    static void __exit pkg_temp_thermal_exit(void)
    {
    platform_thermal_package_notify = core::ptr::null_mut();
    platform_thermal_package_rate_control = core::ptr::null_mut();
    cpuhp_remove_state(pkg_thermal_hp_state);
    debugfs_remove_recursive(debugfs);
    kfree(zones);
    }
    module_exit(pkg_temp_thermal_exit)
    MODULE_IMPORT_NS("INTEL_TCC");
    MODULE_DESCRIPTION("X86 PKG TEMP Thermal Driver");
    MODULE_AUTHOR("Srinivas Pandruvada <srinivas.pandruvada@linux.intel.com>");
    MODULE_LICENSE("GPL v2");
