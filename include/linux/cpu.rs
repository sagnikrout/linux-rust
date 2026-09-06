//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/cpu.h
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
//
// include/linux/cpu.h - generic cpu definition
//
// This is mainly for topological representation. We define the
// basic 'struct cpu' here, which can be embedded in per-arch
// definitions of processors.
//
// Basic handling of the devices is done in drivers/base/cpu.c
//
// CPUs are exported via sysfs in the devices/system/cpu
// directory.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu {
    pub /: *mut *mut int node_id; / The node which contains the CPU,
    pub /: *mut *mut int hotpluggable; / creates sysfs control file if hotpluggable,
    pub dev: device,
}

extern "C" {
    pub fn boot_cpu_init();
}
extern "C" {
    pub fn boot_cpu_hotplug_init();
}
extern "C" {
    pub fn cpu_init();
}
extern "C" {
    pub fn trap_init();
}
extern "C" {
    pub fn register_cpu(cpu: *mut cpu, num: c_int) -> c_int;
}
extern "C" {
    pub fn cpu_is_hotpluggable(cpu: unsigned) -> bool;
}
extern "C" {
    pub fn arch_match_cpu_phys_id(cpu: c_int, phys_id: u64) -> bool;
}
extern "C" {
    pub fn cpu_add_dev_attr(attr: *mut device_attribute) -> c_int;
}
extern "C" {
    pub fn cpu_remove_dev_attr(attr: *mut device_attribute);
}
extern "C" {
    pub fn cpu_add_dev_attr_group(attrs: *mut attribute_group) -> c_int;
}
extern "C" {
    pub fn cpu_remove_dev_attr_group(attrs: *mut attribute_group);
}
extern "C" {
    pub fn cpu_show_srbds(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> isize;
}
extern "C" {
    pub fn cpu_show_ghostwrite(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> isize;
}
extern "C" {
    pub fn cpu_show_tsa(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> isize;
}
extern "C" {
    pub fn cpu_show_vmscape(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> isize;
}
extern "C" {
    pub fn arch_cpu_is_hotpluggable(cpu: c_int) -> bool;
}
extern "C" {
    pub fn arch_register_cpu(cpu: c_int) -> c_int;
}
extern "C" {
    pub fn arch_unregister_cpu(cpu: c_int);
}

extern "C" {
    pub fn unregister_cpu(cpu: *mut cpu);
}
extern "C" {
    pub fn arch_cpu_probe(: *const c_char, _arg: usize) -> isize;
}
extern "C" {
    pub fn arch_cpu_release(: *const c_char, _arg: usize) -> isize;
}

//
// These states are not related to the core CPU hotplug mechanism. They are
// used by various (sub)architectures to track internal state
//
pub const CPU_ONLINE: c_uint = 0x0002 /* CPU is up */;
pub const CPU_UP_PREPARE: c_uint = 0x0003 /* CPU coming up */;
pub const CPU_DEAD: c_uint = 0x0007 /* CPU dead */;
pub const CPU_DEAD_FROZEN: c_uint = 0x0008 /* CPU timed out on unplug */;
pub const CPU_POST_DEAD: c_uint = 0x0009 /* CPU successfully unplugged */;
pub const CPU_BROKEN: c_uint = 0x000B /* CPU did not die properly */;

extern "C" {
    pub fn add_cpu(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn cpu_device_up(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn notify_cpu_starting(cpu: c_uint);
}
extern "C" {
    pub fn cpu_maps_update_begin();
}
extern "C" {
    pub fn cpu_maps_update_done();
}
extern "C" {
    pub fn bringup_hibernate_cpu(sleep_cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn bringup_nonboot_cpus(max_cpus: c_uint);
}
extern "C" {
    pub fn arch_cpu_rescan_dead_smt_siblings() -> c_int;
}

pub const cpuhp_tasks_frozen: c_int = 0;

extern "C" {
    pub fn freeze_secondary_cpus(primary: c_int) -> c_int;
}
extern "C" {
    pub fn thaw_secondary_cpus();
}
extern "C" {
    pub fn freeze_secondary_cpus(_arg: cpu) -> return;
}

extern "C" {
    pub fn cpu_startup_entry(state: cpuhp_state) -> void __noreturn;
}
extern "C" {
    pub fn cpu_idle_poll_ctrl(enable: bool);
}
extern "C" {
    pub fn cpu_in_idle(pc: c_ulong) -> bool;
}
extern "C" {
    pub fn arch_cpu_idle();
}
extern "C" {
    pub fn arch_cpu_idle_prepare();
}
extern "C" {
    pub fn arch_cpu_idle_enter();
}
extern "C" {
    pub fn arch_cpu_idle_exit();
}
extern "C" {
    pub fn arch_tick_broadcast_enter();
}
extern "C" {
    pub fn arch_tick_broadcast_exit();
}
extern "C" {
    pub fn arch_cpu_idle_dead() -> void __noreturn;
}

extern "C" {
    pub fn arch_cpu_finalize_init();
}

extern "C" {
    pub fn play_idle_precise(duration_ns: u64, latency_ns: u64);
}

extern "C" {
    pub fn cpuhp_report_idle_dead();
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpu_attack_vectors {
    CPU_MITIGATE_USER_KERNEL,
    CPU_MITIGATE_USER_USER,
    CPU_MITIGATE_GUEST_HOST,
    CPU_MITIGATE_GUEST_GUEST,
    NR_CPU_ATTACK_VECTORS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smt_mitigations {
    SMT_MITIGATIONS_OFF,
    SMT_MITIGATIONS_AUTO,
    SMT_MITIGATIONS_ON,
}

extern "C" {
    pub fn cpu_mitigations_off() -> bool;
}
extern "C" {
    pub fn cpu_mitigations_auto_nosmt() -> bool;
}
extern "C" {
    pub fn cpu_attack_vector_mitigated(v: cpu_attack_vectors) -> bool;
}

extern "C" {
    pub fn arch_prctl_get_branch_landing_pad_state(t: *mut task_struct, state: *mut unsigned long __user) -> c_int;
}
extern "C" {
    pub fn arch_prctl_set_branch_landing_pad_state(t: *mut task_struct, state: c_ulong) -> c_int;
}
extern "C" {
    pub fn arch_prctl_lock_branch_landing_pad_state(t: *mut task_struct) -> c_int;
}
