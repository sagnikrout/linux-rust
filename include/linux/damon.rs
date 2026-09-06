//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/damon.h
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
// DAMON api
//

// Minimal region size.  Every damon_region is aligned by this.

// Maximum number of monitoring probes.

// Max priority score for DAMON-based operation schemes

//
// struct damon_addr_range - Represents an address region of [@start, @end).
// @start:	Start address of the region (inclusive).
// @end:	End address of the region (exclusive).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_addr_range {
    pub start: c_ulong,
    pub end: c_ulong,
}

//
// struct damon_size_range - Represents size for filter to operate on [@min, @max].
// @min:	Min size (inclusive).
// @max:	Max size (inclusive).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_size_range {
    pub min: c_ulong,
    pub max: c_ulong,
}

//
// struct damon_region - Represents a monitoring target region.
// @ar:			The address range of the region.
// @sampling_addr:	Address of the sample for the next access check.
// @nr_accesses:	Access frequency of this region.
// @probe_hits:		Number of probe-positive region samples.
// @age:		Age of this region.
//
// For any use case, @ar should be non-zero positive size.  damon_set_regions()
// does the validation.
//
// @nr_accesses is reset to zero for every &damon_attrs->aggr_interval and be
// increased for every &damon_attrs->sample_interval if an access to the region
// during the last sampling interval is found.  The update of this field should
// not be done with direct access but with the helper function,
// damon_update_region_access_rate().
//
// @age is initially zero, increased for each aggregation interval, and reset
// to zero again if the access frequency is significantly changed.  If two
// regions are merged into a new region, both @nr_accesses and @age of the new
// region are set as region size-weighted average of those of the two regions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_region {
    pub ar: damon_addr_range,
    pub sampling_addr: c_ulong,
    pub nr_accesses: c_uint,
    pub probe_hits: [c_uchar; DAMON_MAX_PROBES],
    pub age: c_uint,
// private: internal use only.
// List head for siblings.
    pub list: list_head,
// for age calculation.
    pub last_nr_accesses: c_uint,
    pub last_probe_hits: [c_uchar; DAMON_MAX_PROBES],
}

//
// struct damon_target - Represents a monitoring target.
// @pid:		The PID of the virtual address space to monitor.
// @obsolete:		Whether the commit destination target is obsolete.
//
// Each monitoring context could have multiple targets.  For example, a context
// for virtual memory address spaces could have multiple target processes.  The
// @pid should be set for appropriate &struct damon_operations including the
// virtual address spaces monitoring operations.
//
// @obsolete is used only for damon_commit_targets() source targets, to specify
// the matching destination targets are obsolete.  Read damon_commit_targets()
// to see how it is handled.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_target {
    pub pid: *mut pid,
    pub obsolete: bool,
// private:
// Number of monitoring target regions of this target.
    pub nr_regions: c_uint,
// Head of the monitoring target regions of this target.
    pub regions_list: list_head,
// List head for siblings.
    pub list: list_head,
}

//
// enum damos_action - Represents an action of a Data Access Monitoring-based
// Operation Scheme.
//
// @DAMOS_WILLNEED:	Call ``madvise()`` for the region with MADV_WILLNEED.
// @DAMOS_COLD:		Call ``madvise()`` for the region with MADV_COLD.
// @DAMOS_PAGEOUT:	Reclaim the region.
// @DAMOS_HUGEPAGE:	Call ``madvise()`` for the region with MADV_HUGEPAGE.
// @DAMOS_NOHUGEPAGE:	Call ``madvise()`` for the region with MADV_NOHUGEPAGE.
// @DAMOS_COLLAPSE:	Call ``madvise()`` for the region with MADV_COLLAPSE.
// @DAMOS_LRU_PRIO:	Prioritize the region on its LRU lists.
// @DAMOS_LRU_DEPRIO:	Deprioritize the region on its LRU lists.
// @DAMOS_MIGRATE_HOT:  Migrate the regions prioritizing warmer regions.
// @DAMOS_MIGRATE_COLD:	Migrate the regions prioritizing colder regions.
// @DAMOS_STAT:		Do nothing but count the stat.
// @NR_DAMOS_ACTIONS:	Total number of DAMOS actions
//
// The support of each action is up to running &struct damon_operations.
// Refer to 'Operation Action' section of Documentation/mm/damon/design.rst for
// status of the supports.
//
// Note that DAMOS_PAGEOUT doesn't trigger demotions.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum damos_action {
    DAMOS_WILLNEED,
    DAMOS_COLD,
    DAMOS_PAGEOUT,
    DAMOS_HUGEPAGE,
    DAMOS_NOHUGEPAGE,
    DAMOS_COLLAPSE,
    DAMOS_LRU_PRIO,
    DAMOS_LRU_DEPRIO,
    DAMOS_MIGRATE_HOT,
    DAMOS_MIGRATE_COLD,
    DAMOS_STAT,		/* Do nothing but only record the stat */
    NR_DAMOS_ACTIONS,
}

//
// enum damos_quota_goal_metric - Represents the metric to be used as the goal
//
// @DAMOS_QUOTA_USER_INPUT:	User-input value.
// @DAMOS_QUOTA_SOME_MEM_PSI_US:	System level some memory PSI in us.
// @DAMOS_QUOTA_NODE_MEM_USED_BP:	MemUsed ratio of a node.
// @DAMOS_QUOTA_NODE_MEM_FREE_BP:	MemFree ratio of a node.
// @DAMOS_QUOTA_NODE_MEMCG_USED_BP:	MemUsed ratio of a node for a cgroup.
// @DAMOS_QUOTA_NODE_MEMCG_FREE_BP:	MemFree ratio of a node for a cgroup.
// @DAMOS_QUOTA_ACTIVE_MEM_BP:		Active to total LRU memory ratio.
// @DAMOS_QUOTA_INACTIVE_MEM_BP:	Inactive to total LRU memory ratio.
// @DAMOS_QUOTA_NODE_ELIGIBLE_MEM_BP:	Scheme-eligible memory ratio of a
// node in basis points (0-10000).
// @NR_DAMOS_QUOTA_GOAL_METRICS:	Number of DAMOS quota goal metrics.
//
// Metrics equal to larger than @NR_DAMOS_QUOTA_GOAL_METRICS are unsupported.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum damos_quota_goal_metric {
    DAMOS_QUOTA_USER_INPUT,
    DAMOS_QUOTA_SOME_MEM_PSI_US,
    DAMOS_QUOTA_NODE_MEM_USED_BP,
    DAMOS_QUOTA_NODE_MEM_FREE_BP,
    DAMOS_QUOTA_NODE_MEMCG_USED_BP,
    DAMOS_QUOTA_NODE_MEMCG_FREE_BP,
    DAMOS_QUOTA_ACTIVE_MEM_BP,
    DAMOS_QUOTA_INACTIVE_MEM_BP,
    DAMOS_QUOTA_NODE_ELIGIBLE_MEM_BP,
    NR_DAMOS_QUOTA_GOAL_METRICS,
}

//
// struct damos_quota_goal - DAMOS scheme quota auto-tuning goal.
// @metric:		Metric to be used for representing the goal.
// @target_value:	Target value of @metric to achieve with the tuning.
// @current_value:	Current value of @metric.
// @nid:		Node id.
// @memcg_id:		Memcg id.
//
// Data structure for getting the current score of the quota tuning goal.  The
// score is calculated by how close @current_value and @target_value are.  Then
// the score is entered to DAMON's internal feedback loop mechanism to get the
// auto-tuned quota.
//
// If @metric is DAMOS_QUOTA_USER_INPUT, @current_value should be manually
// entered by the user, probably inside the kdamond callbacks.  Otherwise,
// DAMON sets @current_value with self-measured value of @metric.
//
// If @metric is DAMOS_QUOTA_NODE_MEM_{USED,FREE}_BP, @nid represents the node
// id of the target node to account the used/free memory.
//
// If @metric is DAMOS_QUOTA_NODE_MEMCG_{USED,FREE}_BP, @nid and @memcg_id
// represents the node id and the cgroup to account the used memory for.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damos_quota_goal {
    pub metric: damos_quota_goal_metric,
    pub target_value: c_ulong,
    pub current_value: c_ulong,
// metric-dependent fields
    pub nid: c_int,
    pub memcg_id: u64,
}

// private:
// Last measured total PSI
// private:
// List head for siblings.
//
// enum damos_quota_goal_tuner - Goal-based quota tuning logic.
// @DAMOS_QUOTA_GOAL_TUNER_CONSIST:	Aim long term consistent quota.
// @DAMOS_QUOTA_GOAL_TUNER_TEMPORAL:	Aim zero quota asap.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum damos_quota_goal_tuner {
    DAMOS_QUOTA_GOAL_TUNER_CONSIST,
    DAMOS_QUOTA_GOAL_TUNER_TEMPORAL,
}

//
// struct damos_quota - Controls the aggressiveness of the given scheme.
// @reset_interval:	Charge reset interval in milliseconds.
// @ms:			Maximum milliseconds that the scheme can use.
// @sz:			Maximum bytes of memory that the action can be applied.
// @goal_tuner:		Goal-based @esz tuning algorithm to use.
// @esz:		Effective size quota in bytes.
// @fail_charge_num:	Failed regions charge rate numerator.
// @fail_charge_denom:	Failed regions charge rate denominator.
//
// @weight_sz:		Weight of the region's size for prioritization.
// @weight_nr_accesses:	Weight of the region's nr_accesses for prioritization.
// @weight_age:		Weight of the region's age for prioritization.
//
// To avoid consuming too much CPU time or IO resources for applying the
// &struct damos->action to large memory, DAMON allows users to set time and/or
// size quotas.  The quotas can be set by writing non-zero values to &ms and
// &sz, respectively.  If the time quota is set, DAMON tries to use only up to
// &ms milliseconds within &reset_interval for applying the action.  If the
// size quota is set, DAMON tries to apply the action only up to &sz bytes
// within &reset_interval.
//
// To convince the different types of quotas and goals, DAMON internally
// converts those into one single size quota called "effective quota".  DAMON
// internally uses it as the only one real quota.  The conversion is made as
// follows.
//
// The time quota is transformed to a size quota using estimated throughput of
// the scheme's action.  DAMON then compares it against &sz and uses smaller
// one as the effective quota.
//
// If goals is not empty, DAMON calculates yet another size quota based on the
// goals using its internal feedback loop algorithm, for every @reset_interval.
// Then, if the new size quota is smaller than the effective quota, it uses the
// new size quota as the effective quota.
//
// The resulting effective size quota in bytes is set to @esz.
//
// For DAMOS action applying failed amount of regions, charging those same to
// those that the action has successfully applied may be unfair.  For the
// reason, 'the size * @fail_charge_num / @fail_charge_denom' is charged.
//
// For selecting regions within the quota, DAMON prioritizes current scheme's
// target memory regions using the &struct damon_operations->get_scheme_score.
// You could customize the prioritization logic by setting &weight_sz,
// &weight_nr_accesses, and &weight_age, because monitoring operations are
// encouraged to respect those.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damos_quota {
    pub reset_interval: c_ulong,
    pub ms: c_ulong,
    pub sz: c_ulong,
    pub goal_tuner: damos_quota_goal_tuner,
    pub esz: c_ulong,
    pub fail_charge_num: c_uint,
    pub fail_charge_denom: c_uint,
    pub weight_sz: c_uint,
    pub weight_nr_accesses: c_uint,
    pub weight_age: c_uint,
// private:
// Head of quota tuning goals (&damos_quota_goal) list.
    pub goals: list_head,
// For throughput estimation
    pub total_charged_sz: c_ulong,
    pub total_charged_ns: c_ulong,
// For charging the quota
    pub charged_sz: c_ulong,
    pub charged_from: c_ulong,
    pub charge_target_from: *mut damon_target,
    pub charge_addr_from: c_ulong,
// For prioritization
    pub min_score: c_uint,
// For feedback loop
    pub esz_bp: c_ulong,
}

//
// enum damos_wmark_metric - Represents the watermark metric.
//
// @DAMOS_WMARK_NONE:		Ignore the watermarks of the given scheme.
// @DAMOS_WMARK_FREE_MEM_RATE:	Free memory rate of the system in [0,1000].
// @NR_DAMOS_WMARK_METRICS:	Total number of DAMOS watermark metrics
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum damos_wmark_metric {
    DAMOS_WMARK_NONE,
    DAMOS_WMARK_FREE_MEM_RATE,
    NR_DAMOS_WMARK_METRICS,
}

//
// struct damos_watermarks - Controls when a given scheme should be activated.
// @metric:	Metric for the watermarks.
// @interval:	Watermarks check time interval in microseconds.
// @high:	High watermark.
// @mid:	Middle watermark.
// @low:	Low watermark.
//
// If &metric is &DAMOS_WMARK_NONE, the scheme is always active.  Being active
// means DAMON does monitoring and applying the action of the scheme to
// appropriate memory regions.  Else, DAMON checks &metric of the system for at
// least every &interval microseconds and works as below.
//
// If &metric is higher than &high, the scheme is inactivated.  If &metric is
// between &mid and &low, the scheme is activated.  If &metric is lower than
// &low, the scheme is inactivated.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damos_watermarks {
    pub metric: damos_wmark_metric,
    pub interval: c_ulong,
    pub high: c_ulong,
    pub mid: c_ulong,
    pub low: c_ulong,
// private:
    pub activated: bool,
}

//
// struct damos_stat - Statistics on a given scheme.
// @nr_tried:	Total number of regions that the scheme is tried to be applied.
// @sz_tried:	Total size of regions that the scheme is tried to be applied.
// @nr_applied:	Total number of regions that the scheme is applied.
// @sz_applied:	Total size of regions that the scheme is applied.
// @sz_ops_filter_passed:
// Total bytes that passed ops layer-handled DAMOS filters.
// @qt_exceeds: Total number of times the quota of the scheme has exceeded.
// @nr_snapshots:
// Total number of DAMON snapshots that the scheme has tried.
//
// "Tried an action to a region" in this context means the DAMOS core logic
// determined the region as eligible to apply the action.  The access pattern
// (&struct damos_access_pattern), quotas (&struct damos_quota), watermarks
// (&struct damos_watermarks) and filters (&struct damos_filter) that handled
// on core logic can affect this.  The core logic asks the operation set
// (&struct damon_operations) to apply the action to the region.
//
// "Applied an action to a region" in this context means the operation set
// (&struct damon_operations) successfully applied the action to the region, at
// least to a part of the region.  The filters (&struct damos_filter) that
// handled on operation set layer and type of the action and pages of the
// region can affect this.  For example, if a filter is set to exclude
// anonymous pages and the region has only anonymous pages, the region will be
// failed at applying the action.  If the action is &DAMOS_PAGEOUT and all
// pages of the region are already paged out, the region will be failed at
// applying the action.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damos_stat {
    pub nr_tried: c_ulong,
    pub sz_tried: c_ulong,
    pub nr_applied: c_ulong,
    pub sz_applied: c_ulong,
    pub sz_ops_filter_passed: c_ulong,
    pub qt_exceeds: c_ulong,
    pub nr_snapshots: c_ulong,
}

//
// enum damos_filter_type - Type of memory for &struct damos_filter
// @DAMOS_FILTER_TYPE_ANON:	Anonymous pages.
// @DAMOS_FILTER_TYPE_ACTIVE:	Active pages.
// @DAMOS_FILTER_TYPE_MEMCG:	Specific memcg's pages.
// @DAMOS_FILTER_TYPE_YOUNG:	Recently accessed pages.
// @DAMOS_FILTER_TYPE_HUGEPAGE_SIZE:	Page is part of a hugepage.
// @DAMOS_FILTER_TYPE_UNMAPPED:	Unmapped pages.
// @DAMOS_FILTER_TYPE_ADDR:	Address range.
// @DAMOS_FILTER_TYPE_TARGET:	Data Access Monitoring target.
// @NR_DAMOS_FILTER_TYPES:	Number of filter types.
//
// All types except &DAMOS_FILTER_TYPE_ADDR and &DAMOS_FILTER_TYPE_TARGET
// are handled by the underlying &struct damon_operations as a part of scheme
// action trying, and therefore accounted as 'tried'.  In contrast,
// &DAMOS_FILTER_TYPE_ADDR and &DAMOS_FILTER_TYPE_TARGET filters are handled
// by the core layer before trying of the action, and therefore not accounted
// as 'tried'.
//
// Support for the operations-handled filters depends on the running
// &struct damon_operations.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum damos_filter_type {
    DAMOS_FILTER_TYPE_ANON,
    DAMOS_FILTER_TYPE_ACTIVE,
    DAMOS_FILTER_TYPE_MEMCG,
    DAMOS_FILTER_TYPE_YOUNG,
    DAMOS_FILTER_TYPE_HUGEPAGE_SIZE,
    DAMOS_FILTER_TYPE_UNMAPPED,
    DAMOS_FILTER_TYPE_ADDR,
    DAMOS_FILTER_TYPE_TARGET,
    NR_DAMOS_FILTER_TYPES,
}

//
// struct damos_filter - DAMOS action target memory filter.
// @type:	Type of the target memory.
// @matching:	Whether this is for @type-matching memory.
// @allow:	Whether to include or exclude the @matching memory.
// @memcg_id:	Memcg id of the question if @type is DAMOS_FILTER_MEMCG.
// @addr_range:	Address range if @type is DAMOS_FILTER_TYPE_ADDR.
// @target_idx:	Index of the &struct damon_target of
// &damon_ctx->adaptive_targets if @type is
// DAMOS_FILTER_TYPE_TARGET.
// @sz_range:	Size range if @type is DAMOS_FILTER_TYPE_HUGEPAGE_SIZE.
//
// Before applying the &damos->action to a memory region, DAMOS checks if each
// byte of the region matches to this given condition and avoid applying the
// action if so.  Support of each filter type depends on the running &struct
// damon_operations and the type.  Refer to &enum damos_filter_type for more
// details.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damos_filter {
    pub type: damos_filter_type,
    pub matching: bool,
    pub allow: bool,
    pub memcg_id: u64,
    pub addr_range: damon_addr_range,
    pub target_idx: c_int,
    pub sz_range: damon_size_range,
}

// private:
// List head for siblings.
//
// struct damos_walk_control - Control damos_walk().
//
// @walk_fn:	Function to be called back for each region.
// @data:	Data that will be passed to walk functions.
//
// Control damos_walk(), which requests specific kdamond to invoke the given
// function to each region that eligible to apply actions of the kdamond's
// schemes.  Refer to damos_walk() for more details.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damos_walk_control {
    pub sz_filter_passed): *mut *mut damos s, unsigned long,
    pub data: *mut c_void,
// private: internal use only
// informs if the kdamond finished handling of the walk request
    pub completion: completion,
// informs if the walk is canceled.
    pub canceled: bool,
}

//
// struct damos_access_pattern - Target access pattern of the given scheme.
// @min_sz_region:	Minimum size of target regions.
// @max_sz_region:	Maximum size of target regions.
// @min_nr_accesses:	Minimum ``->nr_accesses`` of target regions.
// @max_nr_accesses:	Maximum ``->nr_accesses`` of target regions.
// @min_age_region:	Minimum age of target regions.
// @max_age_region:	Maximum age of target regions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damos_access_pattern {
    pub min_sz_region: c_ulong,
    pub max_sz_region: c_ulong,
    pub min_nr_accesses: c_uint,
    pub max_nr_accesses: c_uint,
    pub min_age_region: c_uint,
    pub max_age_region: c_uint,
}

//
// struct damos_migrate_dests - Migration destination nodes and their weights.
// @node_id_arr:	Array of migration destination node ids.
// @weight_arr:		Array of migration weights for @node_id_arr.
// @nr_dests:		Length of the @node_id_arr and @weight_arr arrays.
//
// @node_id_arr is an array of the ids of migration destination nodes.
// @weight_arr is an array of the weights for those.  The weights in
// @weight_arr are for nodes in @node_id_arr of same array index.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damos_migrate_dests {
    pub node_id_arr: *mut c_uint,
    pub weight_arr: *mut c_uint,
    pub nr_dests: usize,
}

//
// struct damos - Represents a Data Access Monitoring-based Operation Scheme.
// @pattern:		Access pattern of target regions.
// @action:		&damos_action to be applied to the target regions.
// @apply_interval_us:	The time between applying the @action.
// @quota:		Control the aggressiveness of this scheme.
// @wmarks:		Watermarks for automated (in)activation of this scheme.
// @migrate_dests:	Destination nodes if @action is "migrate_{hot,cold}".
// @target_nid:		Destination node if @action is "migrate_{hot,cold}".
// @stat:		Statistics of this scheme.
// @max_nr_snapshots:	Upper limit of nr_snapshots stat.
//
// For each @apply_interval_us, DAMON finds regions which fit in the
// &pattern and applies &action to those. To avoid consuming too much
// CPU time or IO resources for the &action, &quota is used.
//
// If @apply_interval_us is zero, &damon_attrs->aggr_interval is used instead.
//
// To do the work only when needed, schemes can be activated for specific
// system situations using &wmarks.  If all schemes that registered to the
// monitoring context are inactive, DAMON stops monitoring either, and just
// repeatedly checks the watermarks.
//
// @migrate_dests specifies multiple migration target nodes with different
// weights for migrate_hot or migrate_cold actions.  @target_nid is ignored if
// this is set.
//
// @target_nid is used to set the migration target node for migrate_hot or
// migrate_cold actions, and @migrate_dests is unset.
//
// Before applying the &action to a memory region, &struct damon_operations
// implementation could check pages of the region and skip &action to respect
// &struct damos_filter.
//
// After applying the &action to each region, &stat is updated.
//
// If &max_nr_snapshots is set as non-zero and &stat.nr_snapshots be same to or
// greater than it, the scheme is deactivated.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damos {
    pub pattern: damos_access_pattern,
    pub action: damos_action,
    pub apply_interval_us: c_ulong,
    pub quota: damos_quota,
    pub wmarks: damos_watermarks,
    pub target_nid: c_int,
    pub migrate_dests: damos_migrate_dests,
}

// private: internal use only
//
// number of sample intervals that should be passed before applying
// @action
//
// informs if ongoing DAMOS walk for this scheme is finished
//
// If the current region in the filtering stage is allowed by core
// layer-handled filters.  If true, operations layer allows it, too.
//
// whether to reject core/ops filters umatched regions
// Additional set of &struct damos_filter for &action.
// ops layer handling &struct damos_filter objects list.
//
// Last @action applied ops-managing entity.
//
// The minimum entity that @action can be applied depends on the
// underlying &struct damon_operations.  Since it may not be aligned
// with the core layer abstract, namely &struct damon_region, &struct
// damon_operations could apply @action to same entity multiple times.
// Large folios that underlying on multiple &struct damon region
// objects could be such examples.  The &struct damon_operations can
// use @last_applied to avoid that.  DAMOS core logic unsets
// @last_applied when each regions walking for applying the scheme is
// finished.
//
// List head for siblings.
//
// enum damon_ops_id - Identifier for each monitoring operations implementation
//
// @DAMON_OPS_VADDR:	Monitoring operations for virtual address spaces
// @DAMON_OPS_FVADDR:	Monitoring operations for only fixed ranges of virtual
// address spaces
// @DAMON_OPS_PADDR:	Monitoring operations for the physical address space
// @NR_DAMON_OPS:	Number of monitoring operations implementations
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum damon_ops_id {
    DAMON_OPS_VADDR,
    DAMON_OPS_FVADDR,
    DAMON_OPS_PADDR,
    NR_DAMON_OPS,
}

//
// struct damon_operations - Monitoring operations for given use cases.
//
// @id:				Identifier of this operations set.
// @init:			Initialize operations-related data structures.
// @update:			Update operations-related data structures.
// @prepare_access_checks:	Prepare next access check of target regions.
// @check_accesses:		Check the accesses to target regions.
// @apply_probes:		Apply probes for each region.
// @get_scheme_score:		Get the score of a region for a scheme.
// @apply_scheme:		Apply a DAMON-based operation scheme.
// @target_valid:		Determine if the target is valid.
// @cleanup_target:		Clean up each target before deallocation.
//
// DAMON can be extended for various address spaces and usages.  For this,
// users should register the low level operations for their target address
// space and usecase via the &damon_ctx.ops.  Then, the monitoring thread
// (&damon_ctx.kdamond) calls @init and @prepare_access_checks before starting
// the monitoring, @update after each &damon_attrs.ops_update_interval, and
// @check_accesses, @target_valid and @prepare_access_checks after each
// &damon_attrs.sample_interval.
//
// Each &struct damon_operations instance having valid @id can be registered
// via damon_register_ops() and selected by damon_select_ops() later.
// @init should initialize operations-related data structures.  For example,
// this could be used to construct proper monitoring target regions and link
// those to @damon_ctx.adaptive_targets.
// @update should update the operations-related data structures.  For example,
// this could be used to update monitoring target regions for current status.
// @prepare_access_checks should manipulate the monitoring regions to be
// prepared for the next access check.
// @check_accesses should check the accesses to each region that made after the
// last preparation and update the number of observed accesses of each region.
// It should also return max number of observed accesses that made as a result
// of its update.  The value will be used for regions adjustment threshold.
// @apply_probes should apply the data attribute probes to each region and
// accordingly update the probe hits counter of the region.  It should also
// set &damon_region->sampling_addr of each region if ``set_samples`` is true.
// It should also return maximum probe hits weighted sum of regions if
// ``return_max_wsum`` is true.
// @get_scheme_score should return the priority score of a region for a scheme
// as an integer in [0, &DAMOS_MAX_SCORE].
// @apply_scheme is called from @kdamond when a region for user provided
// DAMON-based operation scheme is found.  It should apply the scheme's action
// to the region and return bytes of the region that the action is successfully
// applied.  It should also report how many bytes of the region has passed
// filters (&struct damos_filter) that handled by itself.
// @target_valid should check whether the target is still valid for the
// monitoring.
// @cleanup_target is called before the target will be deallocated.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_operations {
    pub id: damon_ops_id,
    pub context): *mut *mut void (init)(struct damon_ctx,
    pub context): *mut *mut void (update)(struct damon_ctx,
    pub context): *mut *mut void (prepare_access_checks)(struct damon_ctx,
    pub context): *mut *mut unsigned int (check_accesses)(struct damon_ctx,
    pub return_max_wsum): bool set_samples, bool,
    pub scheme): *mut *mut damon_region r, damos,
    pub sz_filter_passed): *mut *mut damos scheme, unsigned long,
    pub t): *mut *mut bool (target_valid)(struct damon_target,
    pub t): *mut *mut void (cleanup_target)(struct damon_target,
}

//
// struct damon_call_control - Control damon_call().
//
// @fn:			Function to be called back.
// @data:		Data that will be passed to @fn.
// @repeat:		Repeat invocations.
// @return_code:	Return code from @fn invocation.
// @dealloc_on_cancel:	If @repeat is true, de-allocate when canceled.
//
// Control damon_call(), which requests specific kdamond to invoke a given
// function.  Refer to damon_call() for more details.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_call_control {
    pub data): *mut *mut int (fn)(void,
    pub data: *mut c_void,
    pub repeat: bool,
    pub return_code: c_int,
    pub dealloc_on_cancel: bool,
// private: internal use only
// informs if the kdamond finished handling of the request
    pub completion: completion,
// informs if the kdamond canceled @fn infocation
    pub canceled: bool,
// List head for siblings.
    pub list: list_head,
}

//
// struct damon_intervals_goal - Monitoring intervals auto-tuning goal.
//
// @access_bp:		Access events observation ratio to achieve in bp.
// @aggrs:		Number of aggregations to achieve @access_bp within.
// @min_sample_us:	Minimum resulting sampling interval in microseconds.
// @max_sample_us:	Maximum resulting sampling interval in microseconds.
//
// DAMON automatically tunes &damon_attrs->sample_interval and
// &damon_attrs->aggr_interval aiming the ratio in bp (1/10,000) of
// DAMON-observed access events to theoretical maximum amount within @aggrs
// aggregations be same to @access_bp.  The logic increases
// &damon_attrs->aggr_interval and &damon_attrs->sampling_interval in same
// ratio if the current access events observation ratio is lower than the
// target for each @aggrs aggregations, and vice versa.
//
// If @aggrs is zero, the tuning is disabled and hence this struct is ignored.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_intervals_goal {
    pub access_bp: c_ulong,
    pub aggrs: c_ulong,
    pub min_sample_us: c_ulong,
    pub max_sample_us: c_ulong,
}

//
// enum damon_filter_type - Type of &struct damon_filter
//
// @DAMON_FILTER_TYPE_ANON:	Anonymous pages.
// @DAMON_FILTER_TYPE_MEMCG:	Specific memcg's pages.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum damon_filter_type {
    DAMON_FILTER_TYPE_ANON,
    DAMON_FILTER_TYPE_MEMCG,
}

//
// struct damon_filter - DAMON region filter for &struct damon_probe.
//
// @type:	Type of the region.
// @matching:	Whether this filter is for the type-matching ones.
// @allow:	Whether the @type-@matching ones should pass this filter.
// @memcg_id:	Memcg id of the question if @type is DAMON_FILTER_MEMCG.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_filter {
    pub type: damon_filter_type,
    pub matching: bool,
    pub allow: bool,
    pub memcg_id: u64,
}

// private:
// Siblings list.
//
// struct damon_probe - Data region attribute probe.
//
// @weight:	Relative priority of the attribute for this probe.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_probe {
    pub weight: c_uint,
// private:
// Filters for assessing if a given region is for this probe.
    pub filters: list_head,
// Siblings list.
    pub list: list_head,
}

//
// struct damon_attrs - Monitoring attributes for accuracy/overhead control.
//
// @sample_interval:		The time between access samplings.
// @aggr_interval:		The time between monitor results aggregations.
// @ops_update_interval:	The time between monitoring operations updates.
// @intervals_goal:		Intervals auto-tuning goal.
// @min_nr_regions:		The minimum number of adaptive monitoring
// regions.
// @max_nr_regions:		The maximum number of adaptive monitoring
// regions.
//
// For each @sample_interval, DAMON checks whether each region is accessed or
// not during the last @sample_interval.  If such access is found, DAMON
// aggregates the information by increasing &damon_region->nr_accesses for
// @aggr_interval time.  For each @aggr_interval, the count is reset.  DAMON
// also checks whether the target memory regions need update (e.g., by
// ``mmap()`` calls from the application, in case of virtual memory monitoring)
// and applies the changes for each @ops_update_interval.  All time intervals
// are in micro-seconds.  Please refer to &struct damon_operations and &struct
// damon_call_control for more detail.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_attrs {
    pub sample_interval: c_ulong,
    pub aggr_interval: c_ulong,
    pub ops_update_interval: c_ulong,
    pub intervals_goal: damon_intervals_goal,
    pub min_nr_regions: c_ulong,
    pub max_nr_regions: c_ulong,
// private: internal use only
//
// @aggr_interval to @sample_interval ratio.
// Core-external components call damon_set_attrs() with &damon_attrs
// that this field is unset.  In the case, damon_set_attrs() sets this
// field of resulting &damon_attrs.  Core-internal components such as
// kdamond_tune_intervals() calls damon_set_attrs() with &damon_attrs
// that this field is set.  In the case, damon_set_attrs() just keep
// it.
//
    pub aggr_samples: c_ulong,
}

//
// struct damon_ctx - Represents a context for each monitoring.  This is the
// main interface that allows users to set the attributes and get the results
// of the monitoring.
//
// @attrs:		Monitoring attributes for accuracy/overhead control.
//
// For each monitoring context, one kernel thread for the monitoring, namely
// kdamond, is created.  The pid of kdamond can be retrieved using
// damon_kdamond_pid().
//
// Once started, kdamond runs until explicitly required to be terminated or
// every monitoring target is invalid.  The validity of the targets is checked
// via the &damon_operations.target_valid of @ops.  The termination can also be
// explicitly requested by calling damon_stop().  To know if a kdamond is
// running, damon_is_running() can be used.
//
// While the kdamond is running, all accesses to &struct damon_ctx from a
// thread other than the kdamond should be made using safe DAMON APIs,
// including damon_call() and damos_walk().
//
// @addr_unit:	Scale factor for core to ops address conversion.
// @min_region_sz:	Minimum region size.
// @pause:	Pause kdamond main loop.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_ctx {
    pub attrs: damon_attrs,
// private: internal use only
// number of sample intervals that passed since this context started
    pub passed_sample_intervals: c_ulong,
//
// number of sample intervals that should be passed before next
// aggregation
//
    pub next_aggregation_sis: c_ulong,
//
// number of sample intervals that should be passed before next ops
// update
//
    pub next_ops_update_sis: c_ulong,
//
// number of sample intervals that should be passed before next
// intervals tuning
//
    pub next_intervals_tune_sis: c_ulong,
// for waiting until the execution of the kdamond_fn is started
    pub kdamond_started: completion,
// for scheme quotas prioritization
    pub regions_score_histogram: *mut c_ulong,
// lists of &struct damon_call_control
    pub call_controls: list_head,
    pub call_controls_obsolete: bool,
    pub call_controls_lock: mutex,
    pub walk_control: *mut damos_walk_control,
    pub walk_control_obsolete: bool,
    pub walk_control_lock: mutex,
//
// indicate if this may be corrupted.  Currentonly this is set only for
// damon_commit_ctx() failure.
//
    pub maybe_corrupted: bool,
// Working thread of the given DAMON context
    pub kdamond: *mut task_struct,
// Protects @kdamond field access
    pub kdamond_lock: mutex,
// public:
    pub addr_unit: c_ulong,
    pub min_region_sz: c_ulong,
    pub pause: bool,
// private:
// Set of monitoring operations for given use cases.
    pub ops: damon_operations,
// Head of monitoring targets (&damon_target) list.
    pub adaptive_targets: list_head,
// Head of probes (&damon_probe) list.
    pub probes: list_head,
// Head of schemes (&damos) list.
    pub schemes: list_head,
// @rnd_state:	Per-ctx PRNG state for damon_rand().
    pub rnd_state: rnd_state,
}

// Get a random number in [@l, @r) using @ctx's lockless PRNG.
extern "C" {
    pub fn container_of(_arg: r->list.next, damon_region: struct, _arg: list) -> return;
}
extern "C" {
    pub fn container_of(_arg: r->list.prev, damon_region: struct, _arg: list) -> return;
}
extern "C" {
    pub fn list_last_entry(_arg: &t->regions_list, damon_region: struct, _arg: list) -> return;
}
extern "C" {
    pub fn list_first_entry(_arg: &t->regions_list, damon_region: struct, _arg: list) -> return;
}

extern "C" {
    pub fn damon_add_filter(probe: *mut damon_probe, f: *mut damon_filter);
}
extern "C" {
    pub fn damon_destroy_filter(f: *mut damon_filter);
}
extern "C" {
    pub fn damon_add_probe(ctx: *mut damon_ctx, probe: *mut damon_probe);
}
extern "C" {
    pub fn damon_update_region_access_rate(r: *mut damon_region, accessed: bool);
}
extern "C" {
    pub fn damos_add_filter(s: *mut damos, f: *mut damos_filter);
}
extern "C" {
    pub fn damos_filter_for_ops(type: damos_filter_type) -> bool;
}
extern "C" {
    pub fn damos_destroy_filter(f: *mut damos_filter);
}
extern "C" {
    pub fn damos_add_quota_goal(q: *mut damos_quota, g: *mut damos_quota_goal);
}
extern "C" {
    pub fn damos_destroy_quota_goal(goal: *mut damos_quota_goal);
}
extern "C" {
    pub fn damon_add_scheme(ctx: *mut damon_ctx, s: *mut damos);
}
extern "C" {
    pub fn damon_destroy_scheme(s: *mut damos);
}
extern "C" {
    pub fn damos_commit_quota_goals(dst: *mut damos_quota, src: *mut damos_quota) -> c_int;
}
extern "C" {
    pub fn damon_add_target(ctx: *mut damon_ctx, t: *mut damon_target);
}
extern "C" {
    pub fn damon_targets_empty(ctx: *mut damon_ctx) -> bool;
}
extern "C" {
    pub fn damon_free_target(t: *mut damon_target);
}
extern "C" {
    pub fn damon_destroy_target(t: *mut damon_target, ctx: *mut damon_ctx);
}
extern "C" {
    pub fn damon_nr_regions(t: *mut damon_target) -> c_uint;
}
extern "C" {
    pub fn damon_destroy_ctx(ctx: *mut damon_ctx);
}
extern "C" {
    pub fn damon_set_attrs(ctx: *mut damon_ctx, attrs: *mut damon_attrs) -> c_int;
}
extern "C" {
    pub fn damon_commit_ctx(old_ctx: *mut damon_ctx, new_ctx: *mut damon_ctx) -> c_int;
}
extern "C" {
    pub fn damon_nr_running_ctxs() -> c_int;
}
extern "C" {
    pub fn damon_is_registered_ops(id: damon_ops_id) -> bool;
}
extern "C" {
    pub fn damon_register_ops(ops: *mut damon_operations) -> c_int;
}
extern "C" {
    pub fn damon_select_ops(ctx: *mut damon_ctx, id: damon_ops_id) -> c_int;
}
// Returns number of samples per aggregation interval
extern "C" {
    pub fn damon_initialized() -> bool;
}
extern "C" {
    pub fn damon_start(ctxs: *mut damon_ctx, nr_ctxs: c_int, exclusive: bool) -> c_int;
}
extern "C" {
    pub fn damon_stop(ctxs: *mut damon_ctx, nr_ctxs: c_int);
}
extern "C" {
    pub fn damon_is_running(ctx: *mut damon_ctx) -> bool;
}
extern "C" {
    pub fn damon_kdamond_pid(ctx: *mut damon_ctx) -> c_int;
}
extern "C" {
    pub fn damon_call(ctx: *mut damon_ctx, control: *mut damon_call_control) -> c_int;
}
extern "C" {
    pub fn damos_walk(ctx: *mut damon_ctx, control: *mut damos_walk_control) -> c_int;
}

