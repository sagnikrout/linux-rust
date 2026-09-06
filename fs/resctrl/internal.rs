//! Automatically rewritten from C Header to Rust Module
//! Source: fs/resctrl/internal.h
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

pub const CQM_LIMBOCHECK_INTERVAL: c_int = 1000;
//
// cpumask_any_housekeeping() - Choose any CPU in @mask, preferring those that
// aren't marked nohz_full
// @mask:	The mask to pick a CPU from.
// @exclude_cpu:The CPU to avoid picking.
//
// Returns a CPU from @mask, but not @exclude_cpu. If there are housekeeping
// CPUs that don't use nohz_full, these are preferred. Pass
// RESCTRL_PICK_ANY_CPU to avoid excluding any CPUs.
//
// When a CPU is excluded, returns >= nr_cpu_ids if no CPUs are available.
//
// Try to find a CPU that isn't nohz_full to use in preference
extern "C" {
    pub fn cpumask_any_but(_arg: mask, _arg: exclude_cpu) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdt_fs_context {
    pub kfc: kernfs_fs_context,
    pub enable_cdpl2: bool,
    pub enable_cdpl3: bool,
    pub enable_mba_mbps: bool,
    pub enable_debug: bool,
}

extern "C" {
    pub fn container_of(_arg: kfc, rdt_fs_context: struct, _arg: kfc) -> return;
}
//
// struct mon_evt - Properties of a monitor event
// @evtid:		event id
// @rid:		resource id for this event
// @name:		name of the event
// @evt_cfg:		Event configuration value that represents the
// memory transactions (e.g., READS_TO_LOCAL_MEM,
// READS_TO_REMOTE_MEM) being tracked by @evtid.
// Only valid if @evtid is an MBM event.
// @configurable:	true if the event is configurable
// @any_cpu:		true if the event can be read from any CPU
// @is_floating_point:	event values are displayed in floating point format
// @binary_bits:	number of fixed-point binary bits from architecture,
// only valid if @is_floating_point is true
// @enabled:		true if the event is enabled
// @arch_priv:		Architecture private data for this event.
// The @arch_priv provided by the architecture via
// resctrl_enable_mon_event().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mon_evt {
    pub evtid: resctrl_event_id,
    pub rid: resctrl_res_level,
    pub name: *mut c_char,
    pub evt_cfg: u32,
    pub configurable: bool,
    pub any_cpu: bool,
    pub is_floating_point: bool,
    pub binary_bits: c_uint,
    pub enabled: bool,
    pub arch_priv: *mut c_void,
}

// Limit for mon_evt::binary_bits
pub const MAX_BINARY_BITS: c_int = 27;
//
// struct mon_data - Monitoring details for each event file.
// @list:            Member of the global @mon_data_kn_priv_list list.
// @rid:             Resource id associated with the event file.
// @evt:             Event structure associated with the event file.
// @sum:             Set for RDT_RESOURCE_L3 when event must be summed
// across multiple domains.
// @domid:           When @sum is zero this is the domain to which
// the event file belongs. When @sum is one this
// is the id of the L3 cache that all domains to be
// summed share.
//
// Pointed to by the kernfs kn->priv field of monitoring event files.
// Readers and writers must hold rdtgroup_mutex.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mon_data {
    pub list: list_head,
    pub rid: resctrl_res_level,
    pub evt: *mut mon_evt,
    pub domid: c_int,
    pub sum: bool,
}

//
// struct rmid_read - Data passed across smp_call*() to read event count.
// @rgrp:  Resource group for which the counter is being read. If it is a parent
// resource group then its event count is summed with the count from all
// its child resource groups.
// @r:	   Resource describing the properties of the event being read.
// @hdr:   Header of domain that the counter should be read from. If NULL then
// sum all domains in @r sharing L3 @ci.id
// @evt:   Which monitor event to read.
// @first: Initialize MBM counter when true.
// @ci:    Cacheinfo for L3. Only set when @hdr is NULL. Used when summing
// domains.
// @is_mbm_cntr: true if "mbm_event" counter assignment mode is enabled and it
// is an MBM event.
// @err:   Error encountered when reading counter.
// @val:   Returned value of event counter. If @rgrp is a parent resource
// group, @val includes the sum of event counts from its child
// resource groups.  If @hdr is NULL, @val includes the sum of all
// domains in @r sharing @ci.id, (summed across child resource groups
// if @rgrp is a parent resource group).
// @arch_mon_ctx: Hardware monitor allocated for this read request (MPAM only).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmid_read {
    pub rgrp: *mut rdtgroup,
    pub r: *mut rdt_resource,
    pub hdr: *mut rdt_domain_hdr,
    pub evt: *mut mon_evt,
    pub first: bool,
    pub ci: *mut cacheinfo,
    pub is_mbm_cntr: bool,
    pub err: c_int,
    pub val: u64,
    pub arch_mon_ctx: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdt_group_type {
    RDTCTRL_GROUP = 0,
    RDTMON_GROUP,
    RDT_NUM_GROUP,
}

//
// enum rdtgrp_mode - Mode of a RDT resource group
// @RDT_MODE_SHAREABLE: This resource group allows sharing of its allocations
// @RDT_MODE_EXCLUSIVE: No sharing of this resource group's allocations allowed
// @RDT_MODE_PSEUDO_LOCKSETUP: Resource group will be used for Pseudo-Locking
// @RDT_MODE_PSEUDO_LOCKED: No sharing of this resource group's allocations
// allowed AND the allocations are Cache Pseudo-Locked
// @RDT_NUM_MODES: Total number of modes
//
// The mode of a resource group enables control over the allowed overlap
// between allocations associated with different resource groups (classes
// of service). User is able to modify the mode of a resource group by
// writing to the "mode" resctrl file associated with the resource group.
//
// The "shareable", "exclusive", and "pseudo-locksetup" modes are set by
// writing the appropriate text to the "mode" file. A resource group enters
// "pseudo-locked" mode after the schemata is written while the resource
// group is in "pseudo-locksetup" mode.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdtgrp_mode {
    RDT_MODE_SHAREABLE = 0,
    RDT_MODE_EXCLUSIVE,
    RDT_MODE_PSEUDO_LOCKSETUP,
    RDT_MODE_PSEUDO_LOCKED,

// Must be last
    RDT_NUM_MODES,
}

//
// struct mongroup - store mon group's data in resctrl fs.
// @mon_data_kn:		kernfs node for the mon_data directory
// @parent:			parent rdtgrp
// @crdtgrp_list:		child rdtgroup node list
// @rmid:			rmid for this rdtgroup
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mongroup {
    pub mon_data_kn: *mut kernfs_node,
    pub parent: *mut rdtgroup,
    pub crdtgrp_list: list_head,
    pub rmid: u32,
}

//
// struct rdtgroup - store rdtgroup's data in resctrl file system.
// @kn:				kernfs node
// @rdtgroup_list:		linked list for all rdtgroups
// @closid:			closid for this rdtgroup
// @cpu_mask:			CPUs assigned to this rdtgroup
// @flags:			status bits
// @waitcount:			how many cpus expect to find this
// group when they acquire rdtgroup_mutex
// @type:			indicates type of this rdtgroup - either
// monitor only or ctrl_mon group
// @mon:			mongroup related data
// @mode:			mode of resource group
// @mba_mbps_event:		input monitoring event id when mba_sc is enabled
// @plr:			pseudo-locked region
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdtgroup {
    pub kn: *mut kernfs_node,
    pub rdtgroup_list: list_head,
    pub closid: u32,
    pub cpu_mask: cpumask,
    pub flags: c_int,
    pub waitcount: core::sync::atomic::AtomicI32,
    pub type: rdt_group_type,
    pub mon: mongroup,
    pub mode: rdtgrp_mode,
    pub mba_mbps_event: resctrl_event_id,
    pub plr: *mut pseudo_lock_region,
}

// rdtgroup.flags
pub const RDT_DELETED: c_int = 1;
// rftype.flags
pub const RFTYPE_FLAGS_CPUS_LIST: c_int = 1;
//
// Define the file type flags for base and info directories.
//

// List of all resource groups
//
// struct rftype - describe each file in the resctrl file system
// @name:	File name
// @mode:	Access mode
// @kf_ops:	File operations
// @flags:	File specific RFTYPE_FLAGS_* flags
// @fflags:	File specific RFTYPE_* flags
// @seq_show:	Show content of the file
// @write:	Write to the file
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rftype {
    pub name: *mut c_char,
    pub mode: umode_t,
    pub kf_ops: *const kernfs_ops,
    pub flags: c_ulong,
    pub fflags: c_ulong,
    pub v): *mut *mut seq_file sf, void,
//
// write() is the generic write callback which maps directly to
// kernfs write operation and overrides all other operations.
// Maximum write size is determined by ->max_write_len.
//
    pub off): *mut *mut char buf, size_t nbytes, loff_t,
}

//
// struct mbm_state - status for each MBM counter in each domain
// @prev_bw_bytes: Previous bytes value read for bandwidth calculation
// @prev_bw:	The most recent bandwidth in MBps
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbm_state {
    pub prev_bw_bytes: u64,
    pub prev_bw: u32,
}

extern "C" {
    pub fn rcu_dereference_check(_arg: kn->name, _arg: lockdep_is_held(&rdtgroup_mutex)) -> return;
}
extern "C" {
    pub fn rdt_last_cmd_clear();
}
extern "C" {
    pub fn rdt_last_cmd_puts(s: *const c_char);
}
extern "C" {
    pub fn rdt_last_cmd_printf(fmt: *const c_char, ...);
}
extern "C" {
    pub fn rdtgroup_kn_unlock(kn: *mut kernfs_node);
}
extern "C" {
    pub fn info_kn_lock(kn: *mut kernfs_node) -> bool;
}
extern "C" {
    pub fn info_kn_unlock(kn: *mut kernfs_node);
}
extern "C" {
    pub fn rdtgroup_kn_mode_restrict(r: *mut rdtgroup, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn rdtgroup_mode_by_closid(closid: c_int) -> rdtgrp_mode;
}
extern "C" {
    pub fn rdtgroup_tasks_assigned(r: *mut rdtgroup) -> c_int;
}
extern "C" {
    pub fn closids_supported() -> c_int;
}
extern "C" {
    pub fn closid_free(closid: c_int);
}
extern "C" {
    pub fn setup_rmid_lru_list() -> c_int;
}
extern "C" {
    pub fn free_rmid_lru_list();
}
extern "C" {
    pub fn alloc_rmid(closid: u32) -> c_int;
}
extern "C" {
    pub fn free_rmid(closid: u32, rmid: u32);
}
extern "C" {
    pub fn resctrl_l3_mon_resource_init() -> c_int;
}
extern "C" {
    pub fn resctrl_l3_mon_resource_exit();
}
extern "C" {
    pub fn mon_event_count(info: *mut c_void);
}
extern "C" {
    pub fn rdtgroup_mondata_show(m: *mut seq_file, arg: *mut c_void) -> c_int;
}
extern "C" {
    pub fn mbm_handle_overflow(work: *mut work_struct);
}
extern "C" {
    pub fn is_mba_sc(r: *mut rdt_resource) -> bool;
}
extern "C" {
    pub fn cqm_handle_limbo(work: *mut work_struct);
}
extern "C" {
    pub fn has_busy_rmid(d: *mut rdt_l3_mon_domain) -> bool;
}
extern "C" {
    pub fn __check_limbo(d: *mut rdt_l3_mon_domain, force_free: bool);
}
extern "C" {
    pub fn resctrl_file_fflags_init(config: *const c_char, fflags: c_ulong);
}
extern "C" {
    pub fn resctrl_file_mode_init(config: *const c_char, mode: umode_t);
}
extern "C" {
    pub fn rdt_staged_configs_clear();
}
extern "C" {
    pub fn closid_allocated(closid: c_uint) -> bool;
}
extern "C" {
    pub fn closid_alloc_fixed(closid: u32) -> bool;
}
extern "C" {
    pub fn resctrl_find_cleanest_closid() -> c_int;
}
extern "C" {
    pub fn resctrl_mbm_assign_mode_show(of: *mut kernfs_open_file, s: *mut seq_file, v: *mut c_void) -> c_int;
}
extern "C" {
    pub fn resctrl_num_mbm_cntrs_show(of: *mut kernfs_open_file, s: *mut seq_file, v: *mut c_void) -> c_int;
}
extern "C" {
    pub fn rdtgroup_assign_cntrs(rdtgrp: *mut rdtgroup);
}
extern "C" {
    pub fn rdtgroup_unassign_cntrs(rdtgrp: *mut rdtgroup);
}
extern "C" {
    pub fn event_filter_show(of: *mut kernfs_open_file, seq: *mut seq_file, v: *mut c_void) -> c_int;
}
extern "C" {
    pub fn mbm_L3_assignments_show(of: *mut kernfs_open_file, s: *mut seq_file, v: *mut c_void) -> c_int;
}
extern "C" {
    pub fn resctrl_io_alloc_show(of: *mut kernfs_open_file, seq: *mut seq_file, v: *mut c_void) -> c_int;
}
extern "C" {
    pub fn rdtgroup_init_cat(s: *mut resctrl_schema, closid: u32) -> c_int;
}
extern "C" {
    pub fn resctrl_peer_type(my_type: resctrl_conf_type) -> resctrl_conf_type;
}
extern "C" {
    pub fn resctrl_io_alloc_closid(r: *mut rdt_resource) -> u32;
}

extern "C" {
    pub fn rdtgroup_locksetup_enter(rdtgrp: *mut rdtgroup) -> c_int;
}
extern "C" {
    pub fn rdtgroup_locksetup_exit(rdtgrp: *mut rdtgroup) -> c_int;
}
extern "C" {
    pub fn rdtgroup_cbm_overlaps_pseudo_locked(d: *mut rdt_ctrl_domain, cbm: c_ulong) -> bool;
}
extern "C" {
    pub fn rdtgroup_pseudo_locked_in_hierarchy(d: *mut rdt_ctrl_domain) -> bool;
}
extern "C" {
    pub fn rdt_pseudo_lock_init() -> c_int;
}
extern "C" {
    pub fn rdt_pseudo_lock_release();
}
extern "C" {
    pub fn rdtgroup_pseudo_lock_create(rdtgrp: *mut rdtgroup) -> c_int;
}
extern "C" {
    pub fn rdtgroup_pseudo_lock_remove(rdtgrp: *mut rdtgroup);
}

