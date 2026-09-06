//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/cpu/umwait.c
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

pub const UMWAIT_C02_ENABLE: c_int = 0;

    (((max_time) & MSR_IA32_UMWAIT_CONTROL_TIME_MASK) |		\
    ((c02_disable) & MSR_IA32_UMWAIT_CONTROL_C02_DISABLE))
//
// Cache IA32_UMWAIT_CONTROL MSR. This is a systemwide control. By default,
// umwait max time is 100000 in TSC-quanta and C0.2 is enabled
//
    let mut umwait_control_cached: static u32 = UMWAIT_CTRL_VAL(100000, UMWAIT_C02_ENABLE);
//
// Cache the original IA32_UMWAIT_CONTROL MSR value which is configured by
// hardware or BIOS before kernel boot.
//
    static u32 orig_umwait_control_cached __ro_after_init;
//
// Serialize access to umwait_control_cached and IA32_UMWAIT_CONTROL MSR in
// the sysfs write functions.
//
    static DEFINE_MUTEX(umwait_lock);
#[no_mangle]
unsafe extern "C" fn umwait_update_control_msr(unused: *mut *mut c_void) {
    static void umwait_update_control_msr(void * unused)
    {
    lockdep_assert_irqs_disabled();
    wrmsrq(MSR_IA32_UMWAIT_CONTROL, READ_ONCE(umwait_control_cached));
    }
//
// The CPU hotplug callback sets the control MSR to the global control
// value.
//
// Disable interrupts so the read of umwait_control_cached and the WRMSR
// are protected against a concurrent sysfs write. Otherwise the sysfs
// write could update the cached value after it had been read on this CPU
// and issue the IPI before the old value had been written. The IPI would
// interrupt, write the new value and after return from IPI the previous
// value would be written by this CPU.
//
// With interrupts disabled the upcoming CPU either sees the new control
// value or the IPI is updating this CPU to the new control value after
// interrupts have been reenabled.
//
#[no_mangle]
unsafe extern "C" fn umwait_cpu_online(cpu: c_uint) -> c_int {
    static int umwait_cpu_online(unsigned int cpu)
    {
    local_irq_disable();
    umwait_update_control_msr(core::ptr::null_mut());
    local_irq_enable();
    return 0;
    }
//
// The CPU hotplug callback sets the control MSR to the original control
// value.
//
#[no_mangle]
unsafe extern "C" fn umwait_cpu_offline(cpu: c_uint) -> c_int {
    static int umwait_cpu_offline(unsigned int cpu)
    {
//
// This code is protected by the CPU hotplug already and
// orig_umwait_control_cached is never changed after it caches
// the original control MSR value in umwait_init(). So there
// is no race condition here.
//
    wrmsrq(MSR_IA32_UMWAIT_CONTROL, orig_umwait_control_cached);
    return 0;
    }
//
// On resume, restore IA32_UMWAIT_CONTROL MSR on the boot processor which
// is the only active CPU at this time. The MSR is set up on the APs via the
// CPU hotplug callback.
//
// This function is invoked on resume from suspend and hibernation. On
// resume from suspend the restore should be not required, but we neither
// trust the firmware nor does it matter if the same value is written
// again.
//
#[no_mangle]
unsafe extern "C" fn umwait_syscore_resume(data: *mut c_void) {
    static void umwait_syscore_resume(void *data)
    {
    umwait_update_control_msr(core::ptr::null_mut());
    }
    static const struct syscore_ops umwait_syscore_ops = {
    .resume	= umwait_syscore_resume,
    };
    static struct syscore umwait_syscore = {
    .ops = &umwait_syscore_ops,
    };
// sysfs interface
//
// When bit 0 in IA32_UMWAIT_CONTROL MSR is 1, C0.2 is disabled.
// Otherwise, C0.2 is enabled.
//
#[no_mangle]
pub unsafe extern "C" fn umwait_ctrl_c02_enabled(ctrl: u32) -> bool {
    static inline bool umwait_ctrl_c02_enabled(u32 ctrl)
    {
    return !(ctrl & MSR_IA32_UMWAIT_CONTROL_C02_DISABLE);
    }
#[no_mangle]
pub unsafe extern "C" fn umwait_ctrl_max_time(ctrl: u32) -> u32 {
    static inline u32 umwait_ctrl_max_time(u32 ctrl)
    {
    return ctrl & MSR_IA32_UMWAIT_CONTROL_TIME_MASK;
    }
#[no_mangle]
pub unsafe extern "C" fn umwait_update_control(maxtime: u32, c02_enable: bool) {
    static inline void umwait_update_control(u32 maxtime, bool c02_enable)
    {
    let mut ctrl: u32 = maxtime & MSR_IA32_UMWAIT_CONTROL_TIME_MASK;
    if (!c02_enable)
    ctrl |= MSR_IA32_UMWAIT_CONTROL_C02_DISABLE;
    WRITE_ONCE(umwait_control_cached, ctrl);
// Propagate to all CPUs
    on_each_cpu(umwait_update_control_msr, core::ptr::null_mut(), 1);
    }
    static ssize_t
    enable_c02_show(struct device *dev, struct device_attribute *attr, char *buf)
    {
    let mut ctrl: u32 = READ_ONCE(umwait_control_cached);
    return sprintf(buf, "%d\n", umwait_ctrl_c02_enabled(ctrl));
    }
    static ssize_t enable_c02_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    bool c02_enable;
    u32 ctrl;
    int ret;
    ret = kstrtobool(buf, &c02_enable);
    if (ret)
    return ret;
    mutex_lock(&umwait_lock);
    ctrl = READ_ONCE(umwait_control_cached);
    if (c02_enable != umwait_ctrl_c02_enabled(ctrl))
    umwait_update_control(ctrl, c02_enable);
    mutex_unlock(&umwait_lock);
    return count;
    }
    static DEVICE_ATTR_RW(enable_c02);
    static ssize_t
    max_time_show(struct device *kobj, struct device_attribute *attr, char *buf)
    {
    let mut ctrl: u32 = READ_ONCE(umwait_control_cached);
    return sprintf(buf, "%u\n", umwait_ctrl_max_time(ctrl));
    }
    static ssize_t max_time_store(struct device *kobj,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    u32 max_time, ctrl;
    int ret;
    ret = kstrtou32(buf, 0, &max_time);
    if (ret)
    return ret;
// bits[1:0] must be zero
    if (max_time & ~MSR_IA32_UMWAIT_CONTROL_TIME_MASK)
    return -EINVAL;
    mutex_lock(&umwait_lock);
    ctrl = READ_ONCE(umwait_control_cached);
    if (max_time != umwait_ctrl_max_time(ctrl))
    umwait_update_control(max_time, umwait_ctrl_c02_enabled(ctrl));
    mutex_unlock(&umwait_lock);
    return count;
    }
    static DEVICE_ATTR_RW(max_time);
    static struct attribute *umwait_attrs[] = {
    &dev_attr_enable_c02.attr,
    &dev_attr_max_time.attr,
    core::ptr::null_mut()
    };
    static struct attribute_group umwait_attr_group = {
    .attrs = umwait_attrs,
    .name = "umwait_control",
    };
#[no_mangle]
unsafe extern "C" fn umwait_init() -> int __init {
    static int __init umwait_init(void)
    {
    struct device *dev;
    int ret;
    if (!boot_cpu_has(X86_FEATURE_WAITPKG))
    return -ENODEV;
//
// Cache the original control MSR value before the control MSR is
// changed. This is the only place where orig_umwait_control_cached
// is modified.
//
    rdmsrq(MSR_IA32_UMWAIT_CONTROL, orig_umwait_control_cached);
    ret = cpuhp_setup_state(CPUHP_AP_ONLINE_DYN, "umwait:online",
    umwait_cpu_online, umwait_cpu_offline);
    if (ret < 0) {
//
// On failure, the control MSR on all CPUs has the
// original control value.
//
    return ret;
    }
    register_syscore(&umwait_syscore);
//
// Add umwait control interface. Ignore failure, so at least the
// default values are set up in case the machine manages to boot.
//
    dev = bus_get_dev_root(&cpu_subsys);
    if (dev) {
    ret = sysfs_create_group(&dev.kobj, &umwait_attr_group);
    put_device(dev);
    }
    return ret;
    }
    device_initcall(umwait_init);
