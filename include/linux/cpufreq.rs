//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/cpufreq.h
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
// linux/include/linux/cpufreq.h
//
// Copyright (C) 2001 Russell King
// (C) 2002 - 2003 Dominik Brodowski <linux@brodo.de>
//

//
// CPUFREQ INTERFACE
//
// Frequency values here are CPU kHz
//

pub const CPUFREQ_NAME_LEN: c_int = 16;
// Print length for names. Extra 1 space for accommodating '\n' in prints

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpufreq_table_sorting {
    CPUFREQ_TABLE_UNSORTED,
    CPUFREQ_TABLE_SORTED_ASCENDING,
    CPUFREQ_TABLE_SORTED_DESCENDING
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpufreq_cpuinfo {
    pub max_freq: c_uint,
    pub min_freq: c_uint,
// in 10^(-9) s = nanoseconds
    pub transition_latency: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpufreq_policy {
// CPUs sharing clock, require sw coordination
    pub /: *mut *mut cpumask_var_t cpus; / Online CPUs only,
    pub /: *mut *mut cpumask_var_t related_cpus; / Online + Offline CPUs,
    pub /: *mut *mut cpumask_var_t real_cpus; / Related and present,
    pub CPUs: *mut *mut unsigned int shared_type; / ACPI: ANY or ALL affected,
    pub /: *mut *mut unsigned int cpu; / cpu managing this policy, must be online,
    pub clk: *mut clk,
    pub /: *mut *mut cpufreq_cpuinfo cpuinfo;/ see above,
    pub /: *mut *mut unsigned int min; / in kHz,
    pub /: *mut *mut unsigned int max; / in kHz,
    pub cpufreq: *mut *mut unsigned int cur; / in kHz, only needed if,
// governors are used
    pub /: *mut *mut unsigned int suspend_freq; / freq to set during suspend,
    pub /: *mut *mut unsigned int policy; / see above,
    pub /: *mut *mut unsigned int last_policy; / policy before unplug,
    pub /: *mut *mut *mut cpufreq_governor governor; / see below,
    pub governor_data: *mut c_void,
    pub /: *mut *mut char last_governor[CPUFREQ_NAME_LEN]; / last governor used,
    pub be: *mut *mut work_update; / if update_policy() needs to,
// called, but you're in IRQ context
    pub constraints: freq_constraints,
    pub min_freq_req: freq_qos_request,
    pub max_freq_req: freq_qos_request,
    pub boost_freq_req: freq_qos_request,
    pub freq_table: *mut cpufreq_frequency_table,
    pub freq_table_sorted: cpufreq_table_sorting,
    pub policy_list: list_head,
    pub kobj: kobject,
    pub kobj_unregister: completion,
//
// The rules for this semaphore:
// - Any routine that wants to read from the policy structure will
// do a down_read on this semaphore.
// - Any routine that will write to the policy structure and/or may take away
// the policy altogether (eg. CPU hotplug), will hold this lock in write
// mode before doing so.
//
    pub rwsem: rw_semaphore,
//
// Fast switch flags:
// - fast_switch_possible should be set by the driver if it can
// guarantee that frequency can be changed on any CPU sharing the
// policy and that the change will affect all of the policy CPUs then.
// - fast_switch_enabled is to be set by governors that support fast
// frequency switching with the help of cpufreq_enable_fast_switch().
//
    pub fast_switch_possible: bool,
    pub fast_switch_enabled: bool,
//
// Set if the CPUFREQ_GOV_STRICT_TARGET flag is set for the current
// governor.
//
    pub strict_target: bool,
//
// Set if inefficient frequencies were found in the frequency table.
// This indicates if the relation flag CPUFREQ_RELATION_E can be
// honored.
//
    pub efficiencies_available: bool,
//
// Preferred average time interval between consecutive invocations of
// the driver to set the frequency for this policy.  To be set by the
// scaling driver (0, which is the default, means no preference).
//
    pub transition_delay_us: c_uint,
//
// Remote DVFS flag (Not added to the driver structure as we don't want
// to access another structure from scheduler hotpath).
//
// Should be set if CPUs can do DVFS on behalf of other CPUs from
// different cpufreq policies.
//
    pub dvfs_possible_from_any_cpu: bool,
// Per policy boost enabled flag.
    pub boost_enabled: bool,
// Per policy boost supported flag.
    pub boost_supported: bool,
// Pending policy->min/max update for the driver
    pub update_limits: bool,
// Cached frequency lookup from cpufreq_driver_resolve_freq.
    pub cached_target_freq: c_uint,
    pub cached_resolved_idx: c_uint,
// Synchronization for frequency transitions
    pub /: *mut *mut bool transition_ongoing; / Tracks transition status,
    pub transition_lock: spinlock_t,
    pub transition_wait: wait_queue_head_t,
    pub /: *mut *mut *mut task_transition_task; / Task which is doing the transition,
// cpufreq-stats
    pub stats: *mut cpufreq_stats,
// For cpufreq driver's internal use
    pub driver_data: *mut c_void,
// Pointer to the cooling device if used for thermal mitigation
    pub cdev: *mut thermal_cooling_device,
    pub nb_min: notifier_block,
    pub nb_max: notifier_block,
}

//
// Used for passing new cpufreq policy data to the cpufreq driver's ->verify()
// callback for sanitization.  That callback is only expected to modify the min
// and max values, if necessary, and specifically it must not update the
// frequency table.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpufreq_policy_data {
    pub cpuinfo: cpufreq_cpuinfo,
    pub freq_table: *mut cpufreq_frequency_table,
    pub cpu: c_uint,
    pub /: *mut *mut unsigned int min; / in kHz,
    pub /: *mut *mut unsigned int max; / in kHz,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpufreq_freqs {
    pub policy: *mut cpufreq_policy,
    pub old: c_uint,
    pub new: c_uint,
    pub /: *mut *mut u8 flags; / flags of cpufreq_driver, see below.,
}

// Only for ACPI

extern "C" {
    pub fn cpufreq_cpu_put(policy: *mut cpufreq_policy);
}

// Scope based cleanup macro for cpufreq_policy kobject reference counting
extern "C" {
    pub fn cpumask_empty(_arg: policy->cpus) -> return;
}

extern "C" {
    pub fn cpufreq_get(cpu: c_uint) -> c_uint;
}
extern "C" {
    pub fn cpufreq_quick_get(cpu: c_uint) -> c_uint;
}
extern "C" {
    pub fn cpufreq_quick_get_max(cpu: c_uint) -> c_uint;
}
extern "C" {
    pub fn cpufreq_get_hw_max_freq(cpu: c_uint) -> c_uint;
}
extern "C" {
    pub fn disable_cpufreq();
}
extern "C" {
    pub fn get_cpu_idle_time(cpu: c_uint, wall: *mut u64, io_busy: c_int) -> u64;
}
extern "C" {
    pub fn refresh_frequency_limits(policy: *mut cpufreq_policy);
}
extern "C" {
    pub fn cpufreq_update_policy(cpu: c_uint);
}
extern "C" {
    pub fn cpufreq_update_limits(cpu: c_uint);
}
extern "C" {
    pub fn have_governor_per_policy() -> bool;
}
extern "C" {
    pub fn cpufreq_supports_freq_invariance() -> bool;
}
extern "C" {
    pub fn cpufreq_enable_fast_switch(policy: *mut cpufreq_policy);
}
extern "C" {
    pub fn cpufreq_disable_fast_switch(policy: *mut cpufreq_policy);
}
extern "C" {
    pub fn has_target_index() -> bool;
}
extern "C" {
    pub fn READ_ONCE(_arg: per_cpu(cpufreq_pressure, _arg: cpu)) -> return;
}

extern "C" {
    pub fn cpufreq_stats_create_table(policy: *mut cpufreq_policy);
}
extern "C" {
    pub fn cpufreq_stats_free_table(policy: *mut cpufreq_policy);
}

//
// CPUFREQ DRIVER INTERFACE
//

// relation flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct freq_attr {
    pub attr: attribute,
    pub ): *mut *mut *mut ssize_t (show)(struct cpufreq_policy , char,
    pub count): *const *const *const *const ssize_t (store)(struct cpufreq_policy , char , size_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpufreq_driver {
    pub name: [c_char; CPUFREQ_NAME_LEN],
    pub flags: u16,
    pub driver_data: *mut c_void,
// needed by all drivers
    pub policy): *mut *mut int (init)(struct cpufreq_policy,
    pub policy): *mut *mut int (verify)(struct cpufreq_policy_data,
// define one out of two
    pub policy): *mut *mut int (setpolicy)(struct cpufreq_policy,
    pub /: *mut *mut unsigned int relation); / Deprecated,
    pub index): c_uint,
    pub target_freq): c_uint,
//
// ->fast_switch() replacement for drivers that use an internal
// representation of performance levels and can pass hints other than
// the target performance level to the hardware. This can only be set
// if ->fast_switch is set too, because in those cases (under specific
// conditions) scale invariance can be disabled, which causes the
// schedutil governor to fall back to the latter.
//
    pub capacity): c_ulong,
//
// Only for drivers with target_index() and CPUFREQ_ASYNC_NOTIFICATION
// unset.
//
// get_intermediate should return a stable intermediate frequency
// platform wants to switch to and target_intermediate() should set CPU
// to that frequency, before jumping to the frequency corresponding
// to 'index'. Core will take care of sending notifications and driver
// doesn't have to handle them in target_intermediate() or
// target_index().
//
// Drivers can return '0' from get_intermediate() in case they don't
// wish to switch to intermediate frequency for some target frequency.
// In that case core will directly call ->target_index().
//
    pub index): c_uint,
    pub index): c_uint,
// should be defined, if possible, return 0 on error
    pub cpu): *mut *mut unsigned int (get)(unsigned int,
// Called to update policy limits on firmware notifications.
    pub policy): *mut *mut void (update_limits)(struct cpufreq_policy,
// optional
    pub limit): *mut *mut int (bios_limit)(int cpu, unsigned int,
    pub policy): *mut *mut int (online)(struct cpufreq_policy,
    pub policy): *mut *mut int (offline)(struct cpufreq_policy,
    pub policy): *mut *mut void (exit)(struct cpufreq_policy,
    pub policy): *mut *mut int (suspend)(struct cpufreq_policy,
    pub policy): *mut *mut int (resume)(struct cpufreq_policy,
// Will be called after the driver is fully initialized
    pub policy): *mut *mut void (ready)(struct cpufreq_policy,
    pub attr: *mut freq_attr,
// platform specific boost support code
    pub boost_enabled: bool,
    pub state): *mut *mut *mut int (set_boost)(struct cpufreq_policy policy, int,
//
// Set by drivers that want to register with the energy model after the
// policy is properly initialized, but before the governor is started.
//
    pub policy): *mut *mut void (register_em)(struct cpufreq_policy,
}

// flags
//
// Set by drivers that need to update internal upper and lower boundaries along
// with the target frequency and so the core and governors should also invoke
// the driver if the target frequency does not change, but the policy min or max
// may have changed.
//

// loops_per_jiffy or other kernel "constants" aren't affected by frequency transitions

//
// Set by drivers that want the core to automatically register the cpufreq
// driver as a thermal cooling device.
//

//
// This should be set by platforms having multiple clock-domains, i.e.
// supporting multiple policies. With this sysfs directories of governor would
// be created in cpu/cpu<num>/cpufreq/ directory and so they can use the same
// governor with different tunables for different clusters.
//

//
// Driver will do POSTCHANGE notifications from outside of their ->target()
// routine and so must set cpufreq_driver->flags with this flag, so that core
// can handle them specially.
//

//
// Set by drivers which want cpufreq core to check if CPU is running at a
// frequency present in freq-table exposed by the driver. For these drivers if
// CPU is found running at an out of table freq, we will try to set it to a freq
// from the table. And if that fails, we will stop further boot process by
// issuing a BUG_ON().
//

//
// Set by drivers to disallow use of governors with "dynamic_switching" flag
// set.
//

extern "C" {
    pub fn cpufreq_register_driver(driver_data: *mut cpufreq_driver) -> c_int;
}
extern "C" {
    pub fn cpufreq_unregister_driver(driver_data: *mut cpufreq_driver);
}
extern "C" {
    pub fn cpufreq_driver_test_flags(flags: u16) -> bool;
}

extern "C" {
    pub fn cpufreq_suspend();
}
extern "C" {
    pub fn cpufreq_resume();
}
extern "C" {
    pub fn cpufreq_generic_suspend(policy: *mut cpufreq_policy) -> c_int;
}

//
// CPUFREQ NOTIFIER INTERFACE
//

// Transition notifiers

// Policy Notifiers

extern "C" {
    pub fn cpufreq_register_notifier(nb: *mut notifier_block, list: c_uint) -> c_int;
}
extern "C" {
    pub fn cpufreq_unregister_notifier(nb: *mut notifier_block, list: c_uint) -> c_int;
}

//
// cpufreq_scale - "old * mult / div" calculation for large values (32-bit-arch
// safe)
// @old:   old value
// @div:   divisor
// @mult:  multiplier
//
// new = old * mult / div
//

//
// CPUFREQ GOVERNORS
//

//
// If (cpufreq_driver->target) exists, the ->governor decides what frequency
// within the limits is used. If (cpufreq_driver->setpolicy> exists, these
// two generic policies are available:
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpufreq_governor {
    pub name: [c_char; CPUFREQ_NAME_LEN],
    pub policy): *mut *mut int (init)(struct cpufreq_policy,
    pub policy): *mut *mut void (exit)(struct cpufreq_policy,
    pub policy): *mut *mut int (start)(struct cpufreq_policy,
    pub policy): *mut *mut void (stop)(struct cpufreq_policy,
    pub policy): *mut *mut void (limits)(struct cpufreq_policy,
    pub buf): *mut c_char,
    pub freq): c_uint,
    pub governor_list: list_head,
    pub owner: *mut module,
    pub flags: u8,
}

// Governor flags
// For governors which change frequency dynamically by themselves

// For governors wanting the target frequency to be set exactly

// Pass a target to the cpufreq driver
extern "C" {
    pub fn cpufreq_driver_has_adjust_perf() -> bool;
}
extern "C" {
    pub fn cpufreq_policy_transition_delay_us(policy: *mut cpufreq_policy) -> c_uint;
}
extern "C" {
    pub fn cpufreq_register_governor(governor: *mut cpufreq_governor) -> c_int;
}
extern "C" {
    pub fn cpufreq_unregister_governor(governor: *mut cpufreq_governor);
}
extern "C" {
    pub fn cpufreq_start_governor(policy: *mut cpufreq_policy) -> c_int;
}
extern "C" {
    pub fn cpufreq_stop_governor(policy: *mut cpufreq_policy);
}

extern "C" {
    pub fn sugov_is_governor(policy: *mut cpufreq_policy) -> bool;
}

// Governor attribute set
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gov_attr_set {
    pub kobj: kobject,
    pub policy_list: list_head,
    pub update_lock: mutex,
    pub usage_count: c_int,
}

// sysfs ops for cpufreq governors
extern "C" {
    pub fn container_of(_arg: kobj, gov_attr_set: struct, _arg: kobj) -> return;
}
extern "C" {
    pub fn gov_attr_set_init(attr_set: *mut gov_attr_set, list_node: *mut list_head);
}
extern "C" {
    pub fn gov_attr_set_get(attr_set: *mut gov_attr_set, list_node: *mut list_head);
}
extern "C" {
    pub fn gov_attr_set_put(attr_set: *mut gov_attr_set, list_node: *mut list_head) -> c_uint;
}
// Governor sysfs attribute
#[repr(C)]
#[derive(Copy, Clone)]
pub struct governor_attr {
    pub attr: attribute,
    pub buf): *mut *mut *mut ssize_t (show)(struct gov_attr_set attr_set, char,
    pub count): usize,
}

//
// FREQUENCY TABLE HELPERS
//
// Special Values of .frequency field

// Special Values of .flags field

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpufreq_frequency_table {
    pub flags: c_uint,
    pub /: *mut *mut unsigned int driver_data; / driver specific data, not used by core,
    pub ascending: *mut *mut unsigned int frequency; / kHz - doesn't need to be in,
// order
}

//
// cpufreq_for_each_entry -	iterate over a cpufreq_frequency_table
// @pos:	the cpufreq_frequency_table * to use as a loop cursor.
// @table:	the cpufreq_frequency_table * to iterate over.
//

//
// cpufreq_for_each_entry_idx -	iterate over a cpufreq_frequency_table
// with index
// @pos:	the cpufreq_frequency_table * to use as a loop cursor.
// @table:	the cpufreq_frequency_table * to iterate over.
// @idx:	the table entry currently being processed
//

//
// cpufreq_for_each_valid_entry -     iterate over a cpufreq_frequency_table
// excluding CPUFREQ_ENTRY_INVALID frequencies.
// @pos:        the cpufreq_frequency_table * to use as a loop cursor.
// @table:      the cpufreq_frequency_table * to iterate over.
//

//
// cpufreq_for_each_valid_entry_idx -     iterate with index over a cpufreq
// frequency_table excluding CPUFREQ_ENTRY_INVALID frequencies.
// @pos:	the cpufreq_frequency_table * to use as a loop cursor.
// @table:	the cpufreq_frequency_table * to iterate over.
// @idx:	the table entry currently being processed
//

//
// cpufreq_for_each_efficient_entry_idx - iterate with index over a cpufreq
// frequency_table excluding CPUFREQ_ENTRY_INVALID and
// CPUFREQ_INEFFICIENT_FREQ frequencies.
// @pos: the &struct cpufreq_frequency_table to use as a loop cursor.
// @table: the &struct cpufreq_frequency_table to iterate over.
// @idx: the table entry currently being processed.
// @efficiencies: set to true to only iterate over efficient frequencies.
//

extern "C" {
    pub fn cpufreq_frequency_table_cpuinfo(policy: *mut cpufreq_policy) -> c_int;
}
extern "C" {
    pub fn cpufreq_frequency_table_verify(policy: *mut cpufreq_policy_data) -> c_int;
}
extern "C" {
    pub fn cpufreq_generic_frequency_table_verify(policy: *mut cpufreq_policy_data) -> c_int;
}
extern "C" {
    pub fn cpufreq_show_cpus(mask: *const cpumask, buf: *mut c_char) -> isize;
}

extern "C" {
    pub fn cpufreq_boost_enabled() -> bool;
}
extern "C" {
    pub fn cpufreq_boost_set_sw(policy: *mut cpufreq_policy, state: c_int) -> c_int;
}
// Find lowest freq at or above target in a table in ascending order
// Find lowest freq at or above target in a table in descending order
// No freq found above target_freq
// Works only on sorted freq-tables
extern "C" {
    pub fn find_index_l(_arg: policy, _arg: target_freq, _arg: policy->min, _arg: policy->max, _arg: efficiencies) -> return;
}
// Find highest freq at or below target in a table in ascending order
// No freq found below target_freq
// Find highest freq at or below target in a table in descending order
// Works only on sorted freq-tables
extern "C" {
    pub fn find_index_h(_arg: policy, _arg: target_freq, _arg: policy->min, _arg: policy->max, _arg: efficiencies) -> return;
}
// Find closest freq to target in a table in ascending order
// No freq found below target_freq
// Choose the closest freq
// Find closest freq to target in a table in descending order
// No freq found above target_freq
// Choose the closest freq
// Works only on sorted freq-tables
extern "C" {
    pub fn find_index_c(_arg: policy, _arg: target_freq, _arg: policy->min, _arg: policy->max, _arg: efficiencies) -> return;
}
// cpufreq_table_index_unsorted() has no use for this flag anyway
// Limit frequency index to honor min and max
//
// cpufreq_table_set_inefficient() - Mark a frequency as inefficient
// @policy:	the &struct cpufreq_policy containing the inefficient frequency
// @frequency:	the inefficient frequency
//
// The &struct cpufreq_policy must use a sorted frequency table
//
// Return:	%0 on success or a negative errno code
//
// Not supported
extern "C" {
    pub fn __free(of_cpu_device_node_get(cpu: device_node) =) -> *mut device_node cpu_np;
}

extern "C" {
    pub fn arch_freq_get_on_cpu(cpu: c_int) -> c_int;
}

// the following are really really optional
extern "C" {
    pub fn cpufreq_table_validate_and_sort(policy: *mut cpufreq_policy) -> c_int;
}
extern "C" {
    pub fn cpufreq_generic_get(cpu: c_uint) -> c_uint;
}
extern "C" {
    pub fn cpufreq_ready_for_eas(cpu_mask: *const cpumask) -> bool;
}
