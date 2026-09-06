//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pm_domain.h
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
// pm_domain.h - Definitions and headers related to device power domains.
//
// Copyright (C) 2011 Rafael J. Wysocki <rjw@sisk.pl>, Renesas Electronics Corp.
//

//
// Flags to control the behaviour when attaching a device to its PM domains.
//
// PD_FLAG_NO_DEV_LINK:		As the default behaviour creates a device-link
// for every PM domain that gets attached, this
// flag can be used to skip that.
//
// PD_FLAG_DEV_LINK_ON:		Add the DL_FLAG_RPM_ACTIVE to power-on the
// supplier and its PM domain when creating the
// device-links.
//
// PD_FLAG_REQUIRED_OPP:	Assign required_devs for the required OPPs. The
// index of the required OPP must correspond to the
// index in the array of the pd_names. If pd_names
// isn't specified, the index just follows the
// index for the attached PM domain.
//
// PD_FLAG_ATTACH_POWER_ON:	Power on the domain during attach.
//
// PD_FLAG_DETACH_POWER_OFF:	Power off the domain during detach.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_pm_domain_attach_data {
    pub pd_names: *const *const c_char,
    pub num_pd_names: u32,
    pub pd_flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_pm_domain_list {
    pub pd_devs: *mut device,
    pub pd_links: *mut device_link,
    pub opp_tokens: *mut u32,
    pub num_pds: u32,
}

//
// Flags to control the behaviour of a genpd.
//
// These flags may be set in the struct generic_pm_domain's flags field by a
// genpd backend driver. The flags must be set before it calls pm_genpd_init(),
// which initializes a genpd.
//
// GENPD_FLAG_PM_CLK:		Instructs genpd to use the PM clk framework,
// while powering on/off attached devices.
//
// GENPD_FLAG_IRQ_SAFE:		This informs genpd that its backend callbacks,
// ->power_on|off(), doesn't sleep. Hence, these
// can be invoked from within atomic context, which
// enables genpd to power on/off the PM domain,
// even when pm_runtime_is_irq_safe() returns true,
// for any of its attached devices. Note that, a
// genpd having this flag set, requires its
// masterdomains to also have it set.
//
// GENPD_FLAG_ALWAYS_ON:	Instructs genpd to always keep the PM domain
// powered on.
//
// GENPD_FLAG_ACTIVE_WAKEUP:	Instructs genpd to keep the PM domain powered
// on, in case any of its attached devices is used
// in the wakeup path to serve system wakeups.
//
// GENPD_FLAG_CPU_DOMAIN:	Instructs genpd that it should expect to get
// devices attached, which may belong to CPUs or
// possibly have subdomains with CPUs attached.
// This flag enables the genpd backend driver to
// deploy idle power management support for CPUs
// and groups of CPUs. Note that, the backend
// driver must then comply with the so called,
// last-man-standing algorithm, for the CPUs in the
// PM domain.
//
// GENPD_FLAG_RPM_ALWAYS_ON:	Instructs genpd to always keep the PM domain
// powered on except for system suspend.
//
// GENPD_FLAG_MIN_RESIDENCY:	Enable the genpd governor to consider its
// components' next wakeup when determining the
// optimal idle state.
//
// GENPD_FLAG_OPP_TABLE_FW:	The genpd provider supports performance states,
// but its corresponding OPP tables are not
// described in DT, but are given directly by FW.
//
// GENPD_FLAG_DEV_NAME_FW:	Instructs genpd to generate an unique device name
// using ida. It is used by genpd providers which
// get their genpd-names directly from FW.
//
// GENPD_FLAG_NO_SYNC_STATE:	The ->sync_state() support is implemented in a
// genpd provider specific way, likely through a
// parent device node. This flag makes genpd to
// skip its internal support for this.
//
// GENPD_FLAG_NO_STAY_ON:	For genpd OF providers a powered-on PM domain at
// initialization is prevented from being
// powered-off until the ->sync_state() callback is
// invoked. This flag informs genpd to allow a
// power-off without waiting for ->sync_state().
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gpd_status {
    GENPD_STATE_ON = 0,	/* PM domain is on */
    GENPD_STATE_OFF,	/* PM domain is off */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum genpd_notication {
    GENPD_NOTIFY_PRE_OFF = 0,
    GENPD_NOTIFY_OFF,
    GENPD_NOTIFY_PRE_ON,
    GENPD_NOTIFY_ON,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum genpd_sync_state {
    GENPD_SYNC_STATE_OFF = 0,
    GENPD_SYNC_STATE_SIMPLE,
    GENPD_SYNC_STATE_ONECELL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_power_governor {
    pub domain): *mut *mut bool (system_power_down_ok)(struct dev_pm_domain,
    pub domain): *mut *mut bool (power_down_ok)(struct dev_pm_domain,
    pub dev): *mut *mut bool (suspend_ok)(struct device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpd_dev_ops {
    pub dev): *mut *mut int (start)(struct device,
    pub dev): *mut *mut int (stop)(struct device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct genpd_governor_data {
    pub max_off_time_ns: i64,
    pub max_off_time_changed: bool,
    pub next_wakeup: ktime_t,
    pub next_hrtimer: ktime_t,
    pub last_enter: ktime_t,
    pub reflect_residency: bool,
    pub cached_power_down_ok: bool,
    pub cached_power_down_state_idx: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct genpd_power_state {
    pub name: *const c_char,
    pub power_off_latency_ns: i64,
    pub power_on_latency_ns: i64,
    pub residency_ns: i64,
    pub usage: u64,
    pub rejected: u64,
    pub above: u64,
    pub below: u64,
    pub usage_s2idle: u64,
    pub fwnode: *mut fwnode_handle,
    pub idle_time: u64,
    pub data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct generic_pm_domain {
    pub dev: device,
    pub /: *mut *mut dev_pm_domain domain; / PM domain operations,
    pub /: *mut *mut list_head gpd_list_node; / Node in the global PM domains list,
    pub /: *mut *mut list_head parent_links; / Links with PM domain as a parent,
    pub /: *mut *mut list_head child_links; / Links with PM domain as a child,
    pub /: *mut *mut list_head dev_list; / List of devices,
    pub gov: *mut dev_power_governor,
    pub /: *mut *mut *mut genpd_governor_data gd; / Data used by a genpd governor.,
    pub power_off_work: work_struct,
    pub /: *mut *mut *mut fwnode_handle provider; / Identity of the domain provider,
    pub has_provider: bool,
    pub name: *const c_char,
    pub /: *mut *mut atomic_t sd_count; / Number of subdomains with power "on",
    pub /: *mut *mut gpd_status status; / Current state of the domain,
    pub /: *mut *mut unsigned int device_count; / Number of devices,
    pub /: *mut *mut unsigned int device_id; / unique device id,
    pub /: *mut *mut unsigned int suspended_count; / System suspend device counter,
    pub /: *mut *mut unsigned int prepared_count; / Suspend counter of prepared devices,
    pub /: *mut *mut unsigned int performance_state; / Aggregated max performance state,
    pub /: *mut *mut cpumask_var_t cpus; / A cpumask of the attached CPUs,
    pub /: *mut *mut bool synced_poweroff; / A consumer needs a synced poweroff,
    pub /: *mut *mut bool stay_on; / Stay powered-on during boot.,
    pub /: *mut *mut genpd_sync_state sync_state; / How sync_state is managed.,
    pub domain): *mut *mut int (power_off)(struct generic_pm_domain,
    pub domain): *mut *mut int (power_on)(struct generic_pm_domain,
    pub /: *mut *mut raw_notifier_head power_notifiers; / Power on/off notifiers,
    pub /: *mut *mut *mut opp_table opp_table; / OPP table of the genpd,
    pub state): c_uint,
    pub dev_ops: gpd_dev_ops,
    pub enable): *mut *mut device dev, bool,
    pub dev): *mut device,
    pub dev): *mut device,
    pub dev): *mut device,
    pub /: *mut *mut unsigned int flags; / Bit field of configs for genpd,
    pub states: *mut genpd_power_state,
    pub state_count): c_uint,
    pub /: *mut *mut unsigned int state_count; / number of states,
    pub /: *mut *mut unsigned int state_idx; / state that genpd will go to when off,
    pub on_time: u64,
    pub accounting_time: u64,
    pub lock_ops: *const genpd_lock_ops,
    pub mlock: mutex,
    pub slock: spinlock_t,
    pub lock_flags: c_ulong,
}

extern "C" {
    pub fn container_of(_arg: pd, generic_pm_domain: struct, _arg: domain) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpd_link {
    pub parent: *mut generic_pm_domain,
    pub parent_node: list_head,
    pub child: *mut generic_pm_domain,
    pub child_node: list_head,
// Sub-domain's per-master domain performance state
    pub performance_state: c_uint,
    pub prev_performance_state: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpd_timing_data {
    pub suspend_latency_ns: i64,
    pub resume_latency_ns: i64,
    pub effective_constraint_ns: i64,
    pub next_wakeup: ktime_t,
    pub constraint_changed: bool,
    pub cached_suspend_ok: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm_domain_data {
    pub list_node: list_head,
    pub dev: *mut device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct generic_pm_domain_data {
    pub base: pm_domain_data,
    pub td: *mut gpd_timing_data,
    pub nb: notifier_block,
    pub power_nb: *mut notifier_block,
    pub cpu: c_int,
    pub performance_state: c_uint,
    pub default_pstate: c_uint,
    pub rpm_pstate: c_uint,
    pub opp_token: c_uint,
    pub hw_mode: bool,
    pub rpm_always_on: bool,
    pub data: *mut c_void,
}

extern "C" {
    pub fn container_of(_arg: pdd, generic_pm_domain_data: struct, _arg: base) -> return;
}
extern "C" {
    pub fn to_gpd_data(_arg: dev->power.subsys_data->domain_data) -> return;
}
extern "C" {
    pub fn pm_genpd_add_device(genpd: *mut generic_pm_domain, dev: *mut device) -> c_int;
}
extern "C" {
    pub fn pm_genpd_remove_device(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn pm_genpd_remove(genpd: *mut generic_pm_domain) -> c_int;
}
extern "C" {
    pub fn dev_pm_genpd_set_performance_state(dev: *mut device, state: c_uint) -> c_int;
}
extern "C" {
    pub fn dev_pm_genpd_add_notifier(dev: *mut device, nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn dev_pm_genpd_remove_notifier(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn dev_pm_genpd_set_next_wakeup(dev: *mut device, next: ktime_t);
}
extern "C" {
    pub fn dev_pm_genpd_get_next_hrtimer(dev: *mut device) -> ktime_t;
}
extern "C" {
    pub fn dev_pm_genpd_synced_poweroff(dev: *mut device);
}
extern "C" {
    pub fn dev_pm_genpd_set_hwmode(dev: *mut device, enable: bool) -> c_int;
}
extern "C" {
    pub fn dev_pm_genpd_get_hwmode(dev: *mut device) -> bool;
}
extern "C" {
    pub fn dev_pm_genpd_rpm_always_on(dev: *mut device, on: bool) -> c_int;
}
extern "C" {
    pub fn dev_pm_genpd_is_on(dev: *mut device) -> bool;
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENOSYS) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}

extern "C" {
    pub fn dev_pm_genpd_suspend(dev: *mut device);
}
extern "C" {
    pub fn dev_pm_genpd_resume(dev: *mut device);
}

// OF PM domain providers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct genpd_onecell_data {
    pub domains: *mut generic_pm_domain,
    pub num_domains: c_uint,
    pub xlate: genpd_xlate_t,
}

extern "C" {
    pub fn of_genpd_del_provider(np: *mut device_node);
}
extern "C" {
    pub fn of_genpd_add_device(args: *const of_phandle_args, dev: *mut device) -> c_int;
}
extern "C" {
    pub fn of_genpd_sync_state(np: *mut device_node);
}
extern "C" {
    pub fn genpd_dev_pm_attach(dev: *mut device) -> c_int;
}

extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}

extern "C" {
    pub fn dev_pm_domain_attach(dev: *mut device, flags: u32) -> c_int;
}
extern "C" {
    pub fn dev_pm_domain_detach(dev: *mut device, power_off: bool);
}
extern "C" {
    pub fn dev_pm_domain_detach_list(list: *mut dev_pm_domain_list);
}
extern "C" {
    pub fn dev_pm_domain_start(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn dev_pm_domain_set(dev: *mut device, pd: *mut dev_pm_domain);
}
extern "C" {
    pub fn dev_pm_domain_set_performance_state(dev: *mut device, state: c_uint) -> c_int;
}

