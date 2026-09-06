//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/resctrl.h
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

// CLOSID, RMID value used by the default control group
pub const RESCTRL_RESERVED_CLOSID: c_int = 0;
pub const RESCTRL_RESERVED_RMID: c_int = 0;

// max value for struct rdt_domain's mbps_val

// Walk all possible resources, with variants for only controls or monitors.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum resctrl_res_level {
    RDT_RESOURCE_L3,
    RDT_RESOURCE_L2,
    RDT_RESOURCE_MBA,
    RDT_RESOURCE_SMBA,
    RDT_RESOURCE_PERF_PKG,
    RDT_RESOURCE_LAST = RDT_RESOURCE_PERF_PKG
}

//
// enum resctrl_conf_type - The type of configuration.
// @CDP_NONE:	No prioritisation, both code and data are controlled or monitored.
// @CDP_CODE:	Configuration applies to instruction fetches.
// @CDP_DATA:	Configuration applies to reads and writes.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum resctrl_conf_type {
    CDP_NONE,
    CDP_CODE,
    CDP_DATA,
    CDP_LAST = CDP_DATA
}

//
// struct pseudo_lock_region - pseudo-lock region information
// @s:			Resctrl schema for the resource to which this
// pseudo-locked region belongs
// @closid:		The closid that this pseudo-locked region uses
// @d:			RDT domain to which this pseudo-locked region
// belongs
// @cbm:		bitmask of the pseudo-locked region
// @lock_thread_wq:	waitqueue used to wait on the pseudo-locking thread
// completion
// @thread_done:	variable used by waitqueue to test if pseudo-locking
// thread completed
// @cpu:		core associated with the cache on which the setup code
// will be run
// @line_size:		size of the cache lines
// @size:		size of pseudo-locked region in bytes
// @kmem:		the kernel memory associated with pseudo-locked region
// @minor:		minor number of character device associated with this
// region
// @debugfs_dir:	pointer to this region's directory in the debugfs
// filesystem
// @pm_reqs:		Power management QoS requests related to this region
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pseudo_lock_region {
    pub s: *mut resctrl_schema,
    pub closid: u32,
    pub d: *mut rdt_ctrl_domain,
    pub cbm: u32,
    pub lock_thread_wq: wait_queue_head_t,
    pub thread_done: c_int,
    pub cpu: c_int,
    pub line_size: c_uint,
    pub size: c_uint,
    pub kmem: *mut c_void,
    pub minor: c_uint,
    pub debugfs_dir: *mut dentry,
    pub pm_reqs: list_head,
}

//
// struct resctrl_staged_config - parsed configuration to be applied
// @new_ctrl:		new ctrl value to be loaded
// @have_new_ctrl:	whether the user provided new_ctrl is valid
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct resctrl_staged_config {
    pub new_ctrl: u32,
    pub have_new_ctrl: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum resctrl_domain_type {
    RESCTRL_CTRL_DOMAIN,
    RESCTRL_MON_DOMAIN,
}

//
// struct rdt_domain_hdr - common header for different domain types
// @list:		all instances of this resource
// @id:			unique id for this instance
// @type:		type of this instance
// @rid:		resource id for this instance
// @cpu_mask:		which CPUs share this resource
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdt_domain_hdr {
    pub list: list_head,
    pub id: c_int,
    pub type: resctrl_domain_type,
    pub rid: resctrl_res_level,
    pub cpu_mask: cpumask,
}

//
// struct rdt_ctrl_domain - group of CPUs sharing a resctrl control resource
// @hdr:		common header for different domain types
// @plr:		pseudo-locked region (if any) associated with domain
// @staged_config:	parsed configuration to be applied
// @mbps_val:		When mba_sc is enabled, this holds the array of user
// specified control values for mba_sc in MBps, indexed
// by closid
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdt_ctrl_domain {
    pub hdr: rdt_domain_hdr,
    pub plr: *mut pseudo_lock_region,
    pub staged_config: [resctrl_staged_config; CDP_NUM_TYPES],
    pub mbps_val: *mut u32,
}

//
// struct mbm_cntr_cfg - Assignable counter configuration.
// @evtid:		MBM event to which the counter is assigned. Only valid
// if @rdtgroup is not NULL.
// @rdtgrp:		resctrl group assigned to the counter. NULL if the
// counter is free.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbm_cntr_cfg {
    pub evtid: resctrl_event_id,
    pub rdtgrp: *mut rdtgroup,
}

//
// struct rdt_l3_mon_domain - group of CPUs sharing RDT_RESOURCE_L3 monitoring
// @hdr:		common header for different domain types
// @ci_id:		cache info id for this domain
// @rmid_busy_llc:	bitmap of which limbo RMIDs are above threshold
// @mbm_states:		Per-event pointer to the MBM event's saved state.
// An MBM event's state is an array of struct mbm_state
// indexed by RMID on x86 or combined CLOSID, RMID on Arm.
// @mbm_over:		worker to periodically read MBM h/w counters
// @cqm_limbo:		worker to periodically read CQM h/w counters
// @mbm_work_cpu:	worker CPU for MBM h/w counters
// @cqm_work_cpu:	worker CPU for CQM h/w counters
// @cntr_cfg:		array of assignable counters' configuration (indexed
// by counter ID)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdt_l3_mon_domain {
    pub hdr: rdt_domain_hdr,
    pub ci_id: c_uint,
    pub rmid_busy_llc: *mut c_ulong,
    pub mbm_states: [*mut mbm_state; QOS_NUM_L3_MBM_EVENTS],
    pub mbm_over: delayed_work,
    pub cqm_limbo: delayed_work,
    pub mbm_work_cpu: c_int,
    pub cqm_work_cpu: c_int,
    pub cntr_cfg: *mut mbm_cntr_cfg,
}

//
// struct resctrl_cache - Cache allocation related data
// @cbm_len:		Length of the cache bit mask
// @min_cbm_bits:	Minimum number of consecutive bits to be set.
// The value 0 means the architecture can support
// zero CBM.
// @shareable_bits:	Bitmask of shareable resource with other
// executing entities
// @arch_has_sparse_bitmasks:	True if a bitmask like f00f is valid.
// @arch_has_per_cpu_cfg:	True if QOS_CFG register for this cache
// level has CPU scope.
// @io_alloc_capable:	True if portion of the cache can be configured
// for I/O traffic.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct resctrl_cache {
    pub cbm_len: c_uint,
    pub min_cbm_bits: c_uint,
    pub shareable_bits: c_uint,
    pub arch_has_sparse_bitmasks: bool,
    pub arch_has_per_cpu_cfg: bool,
    pub io_alloc_capable: bool,
}

//
// enum membw_throttle_mode - System's memory bandwidth throttling mode
// @THREAD_THROTTLE_UNDEFINED:	Not relevant to the system
// @THREAD_THROTTLE_MAX:	Memory bandwidth is throttled at the core
// always using smallest bandwidth percentage
// assigned to threads, aka "max throttling"
// @THREAD_THROTTLE_PER_THREAD:	Memory bandwidth is throttled at the thread
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum membw_throttle_mode {
    THREAD_THROTTLE_UNDEFINED = 0,
    THREAD_THROTTLE_MAX,
    THREAD_THROTTLE_PER_THREAD,
}

//
// struct resctrl_membw - Memory bandwidth allocation related data
// @min_bw:		Minimum memory bandwidth percentage user can request
// @max_bw:		Maximum memory bandwidth value, used as the reset value
// @bw_gran:		Granularity at which the memory bandwidth is allocated
// @delay_linear:	True if memory B/W delay is in linear scale
// @arch_needs_linear:	True if we can't configure non-linear resources
// @throttle_mode:	Bandwidth throttling mode when threads request
// different memory bandwidths
// @mba_sc:		True if MBA software controller(mba_sc) is enabled
// @mb_map:		Mapping of memory B/W percentage to memory B/W delay
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct resctrl_membw {
    pub min_bw: u32,
    pub max_bw: u32,
    pub bw_gran: u32,
    pub delay_linear: u32,
    pub arch_needs_linear: bool,
    pub throttle_mode: membw_throttle_mode,
    pub mba_sc: bool,
    pub mb_map: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum resctrl_scope {
    RESCTRL_L2_CACHE = 2,
    RESCTRL_L3_CACHE = 3,
    RESCTRL_L3_NODE,
    RESCTRL_PACKAGE,
}

//
// enum resctrl_schema_fmt - The format user-space provides for a schema.
// @RESCTRL_SCHEMA_BITMAP:	The schema is a bitmap in hex.
// @RESCTRL_SCHEMA_RANGE:	The schema is a decimal number.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum resctrl_schema_fmt {
    RESCTRL_SCHEMA_BITMAP,
    RESCTRL_SCHEMA_RANGE,
}

//
// struct resctrl_mon - Monitoring related data of a resctrl resource.
// @num_rmid:			Number of RMIDs available.
// @mbm_cfg_mask:		Memory transactions that can be tracked when
// bandwidth monitoring events can be configured.
// @num_mbm_cntrs:		Number of assignable counters.
// @mbm_cntr_assignable:	Is system capable of supporting counter assignment?
// @mbm_assign_on_mkdir:	True if counters should automatically be assigned to MBM
// events of monitor groups created via mkdir.
// @mbm_cntr_configurable:	True if assignable counters are configurable.
// @mbm_cntr_assign_fixed:	True if the counter assignment mode is fixed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct resctrl_mon {
    pub num_rmid: u32,
    pub mbm_cfg_mask: c_uint,
    pub num_mbm_cntrs: c_int,
    pub mbm_cntr_assignable: bool,
    pub mbm_assign_on_mkdir: bool,
    pub mbm_cntr_configurable: bool,
    pub mbm_cntr_assign_fixed: bool,
}

//
// struct rdt_resource - attributes of a resctrl resource
// @rid:		The index of the resource
// @alloc_capable:	Is allocation available on this machine
// @mon_capable:	Is monitor feature available on this machine
// @ctrl_scope:		Scope of this resource for control functions
// @mon_scope:		Scope of this resource for monitor functions
// @cache:		Cache allocation related data
// @membw:		If the component has bandwidth controls, their properties.
// @mon:		Monitoring related data.
// @ctrl_domains:	RCU list of all control domains for this resource
// @mon_domains:	RCU list of all monitor domains for this resource
// @name:		Name to use in "schemata" file.
// @schema_fmt:		Which format string and parser is used for this schema.
// @cdp_capable:	Is the CDP feature available on this resource
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdt_resource {
    pub rid: resctrl_res_level,
    pub alloc_capable: bool,
    pub mon_capable: bool,
    pub ctrl_scope: resctrl_scope,
    pub mon_scope: resctrl_scope,
    pub cache: resctrl_cache,
    pub membw: resctrl_membw,
    pub mon: resctrl_mon,
    pub ctrl_domains: list_head,
    pub mon_domains: list_head,
    pub name: *mut c_char,
    pub schema_fmt: resctrl_schema_fmt,
    pub cdp_capable: bool,
}

//
// Get the resource that exists at this level. If the level is not supported
// a dummy/not-capable resource can be returned. Levels >= RDT_NUM_RESOURCES
// will return NULL.
//
// struct resctrl_schema - configuration abilities of a resource presented to
// user-space
// @list:	Member of resctrl_schema_all.
// @name:	The name to use in the "schemata" file.
// @fmt_str:	Format string to show domain value.
// @conf_type:	Whether this schema is specific to code/data.
// @res:	The resource structure exported by the architecture to describe
// the hardware that is configured by this schema.
// @num_closid:	The number of closid that can be used with this schema. When
// features like CDP are enabled, this will be lower than the
// hardware supports for the resource.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct resctrl_schema {
    pub list: list_head,
    pub name: [c_char; 8],
    pub fmt_str: *const c_char,
    pub conf_type: resctrl_conf_type,
    pub res: *mut rdt_resource,
    pub num_closid: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct resctrl_cpu_defaults {
    pub closid: u32,
    pub rmid: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct resctrl_mon_config_info {
    pub r: *mut rdt_resource,
    pub d: *mut rdt_l3_mon_domain,
    pub evtid: u32,
    pub mon_config: u32,
}

//
// resctrl_arch_sync_cpu_closid_rmid() - Refresh this CPU's CLOSID and RMID.
// Call via IPI.
// @info:	If non-NULL, a pointer to a struct resctrl_cpu_defaults
// specifying the new CLOSID and RMID for tasks in the default
// resctrl ctrl and mon group when running on this CPU.  If NULL,
// this CPU is not re-assigned to a different default group.
//
// Propagates reassignment of CPUs and/or tasks to different resctrl groups
// when requested by the resctrl core code.
//
// This function records the per-cpu defaults specified by @info (if any),
// and then reconfigures the CPU's hardware CLOSID and RMID for subsequent
// execution based on @current, in the same way as during a task switch.
//
extern "C" {
    pub fn resctrl_arch_sync_cpu_closid_rmid(info: *mut c_void);
}
//
// resctrl_get_default_ctrl() - Return the default control value for this
// resource.
// @r:		The resource whose default control type is queried.
//
extern "C" {
    pub fn WARN_ON_ONCE(_arg: 1) -> return;
}
// The number of closid supported by this resource regardless of CDP
extern "C" {
    pub fn resctrl_arch_get_num_closid(r: *mut rdt_resource) -> u32;
}
extern "C" {
    pub fn resctrl_arch_system_num_rmid_idx() -> u32;
}
extern "C" {
    pub fn resctrl_arch_update_domains(r: *mut rdt_resource, closid: u32) -> c_int;
}
extern "C" {
    pub fn resctrl_is_mon_event_enabled(eventid: resctrl_event_id) -> bool;
}
extern "C" {
    pub fn resctrl_arch_is_evt_configurable(evt: resctrl_event_id) -> bool;
}
extern "C" {
    pub fn resctrl_get_mon_evt_cfg(eventid: resctrl_event_id) -> u32;
}
// Iterate over all memory bandwidth events

// Iterate over memory bandwidth arrays in domain structures

//
// resctrl_arch_mon_event_config_write() - Write the config for an event.
// @config_info: struct resctrl_mon_config_info describing the resource, domain
// and event.
//
// Reads resource, domain and eventid from @config_info and writes the
// event config_info->mon_config into hardware.
//
// Called via IPI to reach a CPU that is a member of the specified domain.
//
extern "C" {
    pub fn resctrl_arch_mon_event_config_write(config_info: *mut c_void);
}
//
// resctrl_arch_mon_event_config_read() - Read the config for an event.
// @config_info: struct resctrl_mon_config_info describing the resource, domain
// and event.
//
// Reads resource, domain and eventid from @config_info and reads the
// hardware config value into config_info->mon_config.
//
// Called via IPI to reach a CPU that is a member of the specified domain.
//
extern "C" {
    pub fn resctrl_arch_mon_event_config_read(config_info: *mut c_void);
}
// For use by arch code to remap resctrl's smaller CDP CLOSID range
extern "C" {
    pub fn resctrl_arch_get_cdp_enabled(l: resctrl_res_level) -> bool;
}
extern "C" {
    pub fn resctrl_arch_set_cdp_enabled(l: resctrl_res_level, enable: bool) -> c_int;
}
//
// resctrl_arch_mbm_cntr_assign_enabled() - Check if MBM counter assignment
// mode is enabled.
// @r:		Pointer to the resource structure.
//
// Return:
// true if the assignment mode is enabled, false otherwise.
//
extern "C" {
    pub fn resctrl_arch_mbm_cntr_assign_enabled(r: *mut rdt_resource) -> bool;
}
//
// resctrl_arch_mbm_cntr_assign_set() - Configure the MBM counter assignment mode.
// @r:		Pointer to the resource structure.
// @enable:	Set to true to enable, false to disable the assignment mode.
//
// Return:
// 0 on success, < 0 on error.
//
extern "C" {
    pub fn resctrl_arch_mbm_cntr_assign_set(r: *mut rdt_resource, enable: bool) -> c_int;
}
//
// Update the ctrl_val and apply this config right now.
// Must be called on one of the domain's CPUs.
//
extern "C" {
    pub fn resctrl_online_ctrl_domain(r: *mut rdt_resource, d: *mut rdt_ctrl_domain) -> c_int;
}
extern "C" {
    pub fn resctrl_online_mon_domain(r: *mut rdt_resource, hdr: *mut rdt_domain_hdr) -> c_int;
}
extern "C" {
    pub fn resctrl_offline_ctrl_domain(r: *mut rdt_resource, d: *mut rdt_ctrl_domain);
}
extern "C" {
    pub fn resctrl_offline_mon_domain(r: *mut rdt_resource, hdr: *mut rdt_domain_hdr);
}
extern "C" {
    pub fn resctrl_online_cpu(cpu: c_uint);
}
extern "C" {
    pub fn resctrl_offline_cpu(cpu: c_uint);
}
//
// Architecture hook called at beginning of first file system mount attempt.
// No locks are held.
//
extern "C" {
    pub fn resctrl_arch_pre_mount();
}
//
// resctrl_arch_rmid_read() - Read the eventid counter corresponding to rmid
// for this resource and domain.
// @r:			resource that the counter should be read from.
// @hdr:		Header of domain that the counter should be read from.
// @closid:		closid that matches the rmid. Depending on the architecture, the
// counter may match traffic of both @closid and @rmid, or @rmid
// only.
// @rmid:		rmid of the counter to read.
// @eventid:		eventid to read, e.g. L3 occupancy.
// @arch_priv:		Architecture private data for this event.
// The @arch_priv provided by the architecture via
// resctrl_enable_mon_event().
// @val:		result of the counter read in bytes.
// @arch_mon_ctx:	An architecture specific value from
// resctrl_arch_mon_ctx_alloc(), for MPAM this identifies
// the hardware monitor allocated for this read request.
//
// Some architectures need to sleep when first programming some of the counters.
// (specifically: arm64's MPAM cache occupancy counters can return 'not ready'
// for a short period of time). Call from a non-migrateable process context on
// a CPU that belongs to domain @d. e.g. use smp_call_on_cpu() or
// schedule_work_on(). This function can be called with interrupts masked,
// e.g. using smp_call_function_any(), but may consistently return an error.
//
// Return:
// 0 on success, or -EIO, -EINVAL etc on error.
//
// resctrl_arch_rmid_read_context_check()  - warn about invalid contexts
//
// When built with CONFIG_DEBUG_ATOMIC_SLEEP generate a warning when
// resctrl_arch_rmid_read() is called with preemption disabled.
//
// The contract with resctrl_arch_rmid_read() is that if interrupts
// are unmasked, it can sleep. This allows NOHZ_FULL systems to use an
// IPI, (and fail if the call needed to sleep), while most of the time
// the work is scheduled, allowing the call to sleep.
//
// resctrl_find_domain() - Search for a domain id in a resource domain list.
// @h:		The domain list to search.
// @id:		The domain id to search for.
// @pos:	A pointer to position in the list id should be inserted.
//
// Search the domain list to find the domain id. If the domain id is
// found, return the domain. NULL otherwise.  If the domain id is not
// found (and NULL returned) then the first domain with id bigger than
// the input id can be returned to the caller via @pos.
//
// resctrl_arch_reset_rmid() - Reset any private state associated with rmid
// and eventid.
// @r:		The domain's resource.
// @d:		The rmid's domain.
// @closid:	closid that matches the rmid. Depending on the architecture, the
// counter may match traffic of both @closid and @rmid, or @rmid only.
// @rmid:	The rmid whose counter values should be reset.
// @eventid:	The eventid whose counter values should be reset.
//
// This can be called from any CPU.
//
// resctrl_arch_reset_rmid_all() - Reset all private state associated with
// all rmids and eventids.
// @r:		The resctrl resource.
// @d:		The domain for which all architectural counter state will
// be cleared.
//
// This can be called from any CPU.
//
extern "C" {
    pub fn resctrl_arch_reset_rmid_all(r: *mut rdt_resource, d: *mut rdt_l3_mon_domain);
}
//
// resctrl_arch_reset_all_ctrls() - Reset the control for each CLOSID to its
// default.
// @r:		The resctrl resource to reset.
//
// This can be called from any CPU.
//
extern "C" {
    pub fn resctrl_arch_reset_all_ctrls(r: *mut rdt_resource);
}
//
// resctrl_arch_config_cntr() - Configure the counter with its new RMID
// and event details.
// @r:			Resource structure.
// @d:			The domain in which counter with ID @cntr_id should be configured.
// @evtid:		Monitoring event type (e.g., QOS_L3_MBM_TOTAL_EVENT_ID
// or QOS_L3_MBM_LOCAL_EVENT_ID).
// @rmid:		RMID.
// @closid:		CLOSID.
// @cntr_id:		Counter ID to configure.
// @assign:		True to assign the counter or update an existing assignment,
// false to unassign the counter.
//
// This can be called from any CPU.
//
// resctrl_arch_cntr_read() - Read the event data corresponding to the counter ID
// assigned to the RMID, event pair for this resource
// and domain.
// @r:		Resource that the counter should be read from.
// @d:		Domain that the counter should be read from.
// @closid:	CLOSID that matches the RMID.
// @rmid:	The RMID to which @cntr_id is assigned.
// @cntr_id:	The counter to read.
// @eventid:	The MBM event to which @cntr_id is assigned.
// @val:	Result of the counter read in bytes.
//
// Called on a CPU that belongs to domain @d when "mbm_event" mode is enabled.
// Called from a non-migrateable process context via smp_call_on_cpu() unless all
// CPUs are nohz_full, in which case it is called via IPI (smp_call_function_any()).
//
// Return:
// 0 on success, or -EIO, -EINVAL etc on error.
//
// resctrl_arch_reset_cntr() - Reset any private state associated with counter ID.
// @r:		The domain's resource.
// @d:		The counter ID's domain.
// @closid:	CLOSID that matches the RMID.
// @rmid:	The RMID to which @cntr_id is assigned.
// @cntr_id:	The counter to reset.
// @eventid:	The MBM event to which @cntr_id is assigned.
//
// This can be called from any CPU.
//
// resctrl_arch_io_alloc_enable() - Enable/disable io_alloc feature.
// @r:		The resctrl resource.
// @enable:	Enable (true) or disable (false) io_alloc on resource @r.
//
// This can be called from any CPU.
//
// Return:
// 0 on success, <0 on error.
//
extern "C" {
    pub fn resctrl_arch_io_alloc_enable(r: *mut rdt_resource, enable: bool) -> c_int;
}
//
// resctrl_arch_get_io_alloc_enabled() - Get io_alloc feature state.
// @r:		The resctrl resource.
//
// Return:
// true if io_alloc is enabled or false if disabled.
//
extern "C" {
    pub fn resctrl_arch_get_io_alloc_enabled(r: *mut rdt_resource) -> bool;
}
extern "C" {
    pub fn resctrl_init() -> c_int;
}
extern "C" {
    pub fn resctrl_exit();
}

extern "C" {
    pub fn resctrl_arch_get_prefetch_disable_bits() -> u64;
}
extern "C" {
    pub fn resctrl_arch_pseudo_lock_fn(_plr: *mut c_void) -> c_int;
}
extern "C" {
    pub fn resctrl_arch_measure_cycles_lat_fn(_plr: *mut c_void) -> c_int;
}
extern "C" {
    pub fn resctrl_arch_measure_l2_residency(_plr: *mut c_void) -> c_int;
}
extern "C" {
    pub fn resctrl_arch_measure_l3_residency(_plr: *mut c_void) -> c_int;
}

