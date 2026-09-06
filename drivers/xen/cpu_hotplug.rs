//! Automatically rewritten from C to Rust
//! Source: drivers/xen/cpu_hotplug.c
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

#[no_mangle]
unsafe extern "C" fn enable_hotplug_cpu(cpu: c_int) {
    static void enable_hotplug_cpu(int cpu)
    {
    if (!cpu_present(cpu))
    xen_arch_register_cpu(cpu);
    set_cpu_present(cpu, true);
    }
#[no_mangle]
unsafe extern "C" fn disable_hotplug_cpu(cpu: c_int) {
    static void disable_hotplug_cpu(int cpu)
    {
    if (!cpu_is_hotpluggable(cpu))
    return;
    lock_device_hotplug();
    if (cpu_online(cpu))
    device_offline(get_cpu_device(cpu));
    if (!cpu_online(cpu) && cpu_present(cpu)) {
    xen_arch_unregister_cpu(cpu);
    set_cpu_present(cpu, false);
    }
    unlock_device_hotplug();
    }
#[no_mangle]
unsafe extern "C" fn vcpu_online(cpu: c_uint) -> c_int {
    static int vcpu_online(unsigned int cpu)
    {
    int err;
    char dir[16], state[16];
    sprintf(dir, "cpu/%u", cpu);
    err = xenbus_scanf(XBT_NIL, dir, "availability", "%15s", state);
    if (err != 1) {
    if (!xen_initial_domain())
    pr_err("Unable to read cpu state\n");
    return err;
    }
    if (strcmp(state, "online") == 0)
    return 1;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strcmp(state, 0: "offline") ==) -> else {
    else if (strcmp(state, "offline") == 0)
    return 0;
    pr_err("unknown state(%s) on CPU%d\n", state, cpu);
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn vcpu_hotplug(cpu: c_uint) {
    static void vcpu_hotplug(unsigned int cpu)
    {
    if (cpu >= nr_cpu_ids || !cpu_possible(cpu))
    return;
    switch (vcpu_online(cpu)) {
    case 1:
    enable_hotplug_cpu(cpu);
    break;
    case 0:
    disable_hotplug_cpu(cpu);
    break;
    default:
    break;
    }
    }
    static void handle_vcpu_hotplug_event(struct xenbus_watch *watch,
    const char *path, const char *token)
    {
    unsigned int cpu;
    char *cpustr;
    cpustr = strstr(path, "cpu/");
    if (cpustr != core::ptr::null_mut()) {
    sscanf(cpustr, "cpu/%u", &cpu);
    vcpu_hotplug(cpu);
    }
    }
    static int setup_cpu_watcher(struct notifier_block *notifier,
    unsigned long event, void *data)
    {
    int cpu;
    static struct xenbus_watch cpu_watch = {
    .node = "cpu",
    .callback = handle_vcpu_hotplug_event};
    (void)register_xenbus_watch(&cpu_watch);
    for_each_possible_cpu(cpu) {
    if (vcpu_online(cpu) == 0)
    disable_hotplug_cpu(cpu);
    }
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn setup_vcpu_hotplug_event() -> int __init {
    static int __init setup_vcpu_hotplug_event(void)
    {
    static struct notifier_block xsn_cpu = {
    .notifier_call = setup_cpu_watcher };

    if (!xen_pv_domain() && !xen_pvh_domain())

    if (!xen_domain())

    return -ENODEV;
    register_xenstore_notifier(&xsn_cpu);
    return 0;
    }
    late_initcall(setup_vcpu_hotplug_event);
