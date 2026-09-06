//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/cgroup-defs.h
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
// linux/cgroup-defs.h - basic definitions for cgroup
//
// This file provides basic type and interface.  Include this file directly
// only if necessary to avoid cyclic dependencies.
//

pub const MAX_CGROUP_TYPE_NAMELEN: c_int = 32;
pub const MAX_CGROUP_ROOT_NAMELEN: c_int = 64;
pub const MAX_CFTYPE_NAME: c_int = 64;
// define the enumeration of all cgroup subsystems

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cgroup_subsys_id {

    CGROUP_SUBSYS_COUNT,
}

// bits in struct cgroup_subsys_state flags field
// bits in struct cgroup flags field
// Control Group requires release notifications to userspace
//
// Clone the parent's configuration when creating a new child
// cpuset cgroup.  For historical reasons, this option can be
// specified at mount time and thus is implemented here.
//
// Control group has to be frozen.
// Cgroup is frozen.
// cgroup_root->flags
//
// Consider namespaces as delegation boundaries.  If this flag is
// set, controller specific interface files in a namespace root
// aren't writeable from inside the namespace.
//
// Reduce latencies on dynamic cgroup modifications such as task
// migrations and controller on/offs by disabling percpu operation on
// cgroup_threadgroup_rwsem. This makes hot path operations such as
// forks and exits into the slow path and more expensive.
//
// Alleviate the contention between fork, exec, exit operations and
// writing to cgroup.procs by taking a per threadgroup rwsem instead of
// the global cgroup_threadgroup_rwsem. Fork and other operations
// from threads in different thread groups no longer contend with
// writing to cgroup.procs.
//
// The static usage pattern of creating a cgroup, enabling controllers,
// and then seeding it with CLONE_INTO_CGROUP doesn't require write
// locking cgroup_threadgroup_rwsem and thus doesn't benefit from
// favordynmod.
//
// Enable cpuset controller in v1 cgroup to use v2 behavior.
//
// Enable legacy local memory.events.
//
// Enable recursive subtree protection
//
// Enable hugetlb accounting for the memory controller.
//
// Enable legacy local pids.events.
//
// cftype->flags
// internal flags, do not use outside cgroup core proper
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cgroup_attach_lock_mode {
// Default
    CGRP_ATTACH_LOCK_GLOBAL,

// When pid=0 && threadgroup=false, see comments in cgroup_procs_write_start
    CGRP_ATTACH_LOCK_NONE,

// When favordynmods is on, see comments above CGRP_ROOT_FAVOR_DYNMODS
    CGRP_ATTACH_LOCK_PER_THREADGROUP,
}

//
// cgroup_file is the handle for a file instance created in a cgroup which
// is used, for example, to generate file changed notifications.  This can
// be obtained by setting cftype->file_offset.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgroup_file {
// do not access any fields from outside cgroup core
    pub kn: *mut kernfs_node,
    pub notified_at: c_ulong,
    pub notify_timer: timer_list,
    pub lock: spinlock_t,
}

//
// Per-subsystem/per-cgroup state maintained by the system.  This is the
// fundamental structural building block that controllers deal with.
//
// Fields marked with "PI:" are public and immutable and may be accessed
// directly without synchronization.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgroup_subsys_state {
// PI: the cgroup that this css is attached to
    pub cgroup: *mut cgroup,
// PI: the cgroup subsystem that this css is attached to
    pub ss: *mut cgroup_subsys,
// reference count - access via css_[try]get() and css_put()
    pub refcnt: percpu_ref,
//
// Depending on the context, this field is initialized
// via css_rstat_init() at different places:
//
// when css is associated with cgroup::self
// when css->cgroup is the root cgroup
// performed in cgroup_init()
// when css->cgroup is not the root cgroup
// performed in cgroup_create()
// when css is associated with a subsystem
// when css->cgroup is the root cgroup
// performed in cgroup_init_subsys() in the non-early path
// when css->cgroup is not the root cgroup
// performed in css_create()
//
    pub rstat_cpu: *mut css_rstat_cpu __percpu,
//
// siblings list anchored at the parent's ->children
//
// linkage is protected by cgroup_mutex or RCU
//
    pub sibling: list_head,
    pub children: list_head,
//
// PI: Subsys-unique ID.  0 is unused and root is always 1.  The
// matching css can be looked up using css_from_id().
//
    pub id: c_int,
    pub flags: c_uint,
//
// Monotonically increasing unique serial number which defines a
// uniform order among all csses.  It's guaranteed that all
// ->children lists are in the ascending order of ->serial_nr and
// used to allow interrupting and resuming iterations.
//
    pub serial_nr: u64,
//
// Incremented by online self and children.  Used to guarantee that
// parents are not offlined before their children.
//
    pub online_cnt: core::sync::atomic::AtomicI32,
// percpu_ref killing and RCU release
    pub destroy_work: work_struct,
    pub destroy_rwork: rcu_work,
//
// PI: the parent css.	Placed here for cache proximity to following
// fields of the containing structure.
//
    pub parent: *mut cgroup_subsys_state,
//
// Keep track of total numbers of visible descendant CSSes.
// The total number of dying CSSes is tracked in
// css->cgroup->nr_dying_subsys[ssid].
// Protected by cgroup_mutex.
//
    pub nr_descendants: c_int,
//
// Hierarchical populated state. For cgroup->self, nr_populated_csets
// counts populated csets linked via cgrp_cset_link.
// nr_populated_children counts immediate-child csses whose own
// populated state is nonzero. Protected by css_set_lock.
//
    pub nr_populated_csets: c_int,
    pub nr_populated_children: c_int,
// deferred kill_css_finish() queued by css_update_populated()
    pub kill_finish_work: work_struct,
//
// A singly-linked list of css structures to be rstat flushed.
// This is a scratch field to be used exclusively by
// css_rstat_flush().
//
// Protected by rstat_base_lock when css is cgroup::self.
// Protected by css->ss->rstat_ss_lock otherwise.
//
    pub rstat_flush_next: *mut cgroup_subsys_state,
}

//
// A css_set is a structure holding pointers to a set of
// cgroup_subsys_state objects. This saves space in the task struct
// object and speeds up fork()/exit(), since a single inc/dec and a
// list_add()/del() can bump the reference count on the entire cgroup
// set for a task.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct css_set {
//
// Set of subsystem states, one for each subsystem. This array is
// immutable after creation apart from the init_css_set during
// subsystem registration (at boot time).
//
    pub subsys: [*mut cgroup_subsys_state; CGROUP_SUBSYS_COUNT],
// reference count
    pub refcount: refcount_t,
//
// For a domain cgroup, the following points to self.  If threaded,
// to the matching cset of the nearest domain ancestor.  The
// dom_cset provides access to the domain cgroup and its csses to
// which domain level resource consumptions should be charged.
//
    pub dom_cset: *mut css_set,
// the default cgroup associated with this css_set
    pub dfl_cgrp: *mut cgroup,
// internal task count, protected by css_set_lock
    pub nr_tasks: c_int,
//
// Lists running through all tasks using this cgroup group.
// mg_tasks lists tasks which belong to this cset but are in the
// process of being migrated out or in.  Protected by
// css_set_lock, but, during migration, once tasks are moved to
// mg_tasks, it can be read safely while holding cgroup_mutex.
//
    pub tasks: list_head,
    pub mg_tasks: list_head,
    pub dying_tasks: list_head,
// all css_task_iters currently walking this cset
    pub task_iters: list_head,
//
// On the default hierarchy, ->subsys[ssid] may point to a css
// attached to an ancestor instead of the cgroup this css_set is
// associated with.  The following node is anchored at
// ->subsys[ssid]->cgroup->e_csets[ssid] and provides a way to
// iterate through all css's attached to a given cgroup.
//
    pub e_cset_node: [list_head; CGROUP_SUBSYS_COUNT],
// all threaded csets whose ->dom_cset points to this cset
    pub threaded_csets: list_head,
    pub threaded_csets_node: list_head,
//
// List running through all cgroup groups in the same hash
// slot. Protected by css_set_lock
//
    pub hlist: hlist_node,
//
// List of cgrp_cset_links pointing at cgroups referenced from this
// css_set.  Protected by css_set_lock.
//
    pub cgrp_links: list_head,
//
// List of csets participating in the on-going migration either as
// source or destination.  Protected by cgroup_mutex.
//
    pub mg_src_preload_node: list_head,
    pub mg_dst_preload_node: list_head,
    pub mg_node: list_head,
//
// If this cset is acting as the source of migration the following
// two fields are set.  mg_src_cgrp and mg_dst_cgrp are
// respectively the source and destination cgroups of the on-going
// migration.  mg_dst_cset is the destination cset the target tasks
// on this cset should be migrated to.  Protected by cgroup_mutex.
//
    pub mg_src_cgrp: *mut cgroup,
    pub mg_dst_cgrp: *mut cgroup,
    pub mg_dst_cset: *mut css_set,
// dead and being drained, ignore for migration
    pub dead: bool,
// For RCU-protected deletion
    pub rcu_head: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgroup_base_stat {
    pub cputime: task_cputime,

    pub forceidle_sum: u64,

    pub ntime: u64,
}

//
// rstat - cgroup scalable recursive statistics.  Accounting is done
// per-cpu in css_rstat_cpu which is then lazily propagated up the
// hierarchy on reads.
//
// When a stat gets updated, the css_rstat_cpu and its ancestors are
// linked into the updated tree.  On the following read, propagation only
// considers and consumes the updated tree.  This makes reading O(the
// number of descendants which have been active since last read) instead of
// O(the total number of descendants).
//
// This is important because there can be a lot of (draining) cgroups which
// aren't active and stat may be read frequently.  The combination can
// become very expensive.  By propagating selectively, increasing reading
// frequency decreases the cost of each read.
//
// This struct hosts both the fields which implement the above -
// updated_children and updated_next.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct css_rstat_cpu {
//
// Child cgroups with stat updates on this cpu since the last read
// are linked on the parent's ->updated_children through
// ->updated_next. updated_children is terminated by its container css.
//
    pub updated_children: *mut cgroup_subsys_state,
    pub /: *mut *mut *mut cgroup_subsys_state updated_next; / NULL if not on the list,
    pub /: *mut *mut llist_node lnode; / lockless list for update,
    pub /: *mut *mut *mut cgroup_subsys_state owner; / back pointer,
}

//
// This struct hosts the fields which track basic resource statistics on
// top of it - bsync, bstat and last_bstat.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgroup_rstat_base_cpu {
//
// ->bsync protects ->bstat.  These are the only fields which get
// updated in the hot path.
//
    pub bsync: u64_stats_sync,
    pub bstat: cgroup_base_stat,
//
// Snapshots at the last reading.  These are used to calculate the
// deltas to propagate to the global counters.
//
    pub last_bstat: cgroup_base_stat,
//
// This field is used to record the cumulative per-cpu time of
// the cgroup and its descendants. Currently it can be read via
// eBPF/drgn etc, and we are still trying to determine how to
// expose it in the cgroupfs interface.
//
    pub subtree_bstat: cgroup_base_stat,
//
// Snapshots at the last reading. These are used to calculate the
// deltas to propagate to the per-cpu subtree_bstat.
//
    pub last_subtree_bstat: cgroup_base_stat,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgroup_freezer_state {
// Should the cgroup and its descendants be frozen.
    pub freeze: bool,
// Should the cgroup actually be frozen?
    pub e_freeze: bool,
// Fields below are protected by css_set_lock
// Number of frozen descendant cgroups
    pub nr_frozen_descendants: c_int,
//
// Number of tasks, which are counted as frozen:
// frozen, SIGSTOPped, and PTRACEd.
//
    pub nr_frozen_tasks: c_int,
// Freeze time data consistency protection
    pub freeze_seq: seqcount_spinlock_t,
//
// Most recent time the cgroup was requested to freeze.
// Accesses guarded by freeze_seq counter. Writes serialized
// by css_set_lock.
//
    pub freeze_start_nsec: u64,
//
// Total duration the cgroup has spent freezing.
// Accesses guarded by freeze_seq counter. Writes serialized
// by css_set_lock.
//
    pub frozen_nsec: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgroup {
// self css with NULL ->ss, points back to this cgroup
    pub self: cgroup_subsys_state,
    pub /: *mut *mut unsigned long flags; / "unsigned long" so bitops work,
//
// The depth this cgroup is at.  The root is at depth zero and each
// step down the hierarchy increments the level.  This along with
// ancestors[] can determine whether a given cgroup is a
// descendant of another without traversing the hierarchy.
//
    pub level: c_int,
// Maximum allowed descent tree depth
    pub max_depth: c_int,
//
// Keep track of total numbers of visible and dying descent cgroups.
// Dying cgroups are cgroups which were deleted by a user,
// but are still existing because someone else is holding a reference.
// max_descendants is a maximum allowed number of descent cgroups.
//
// nr_descendants and nr_dying_descendants are protected
// by cgroup_mutex and css_set_lock. It's fine to read them holding
// any of cgroup_mutex and css_set_lock; for writing both locks
// should be held.
//
    pub nr_descendants: c_int,
    pub nr_dying_descendants: c_int,
    pub max_descendants: c_int,
//
// Domain/threaded split of self.nr_populated_children: each counts
// immediate-child cgroups whose subtree is populated and sums to
// self.nr_populated_children. Kept as separate fields to allow readers
// like cgroup_can_be_thread_root() unlocked access. Protected by
// css_set_lock; updated by css_update_populated().
//
    pub nr_populated_domain_children: c_int,
    pub nr_populated_threaded_children: c_int,
    pub /: *mut *mut int nr_threaded_children; / # of live threaded child cgroups,
//
// Sequence number for cgroup.kill. Incremented with both cgroup_mutex
// and css_set_lock held. Readers hold either one.
//
    pub kill_seq: c_uint,
    pub /: *mut *mut *mut kernfs_node kn; / cgroup kernfs entry,
    pub /: *mut *mut cgroup_file procs_file; / handle for "cgroup.procs",
    pub /: *mut *mut cgroup_file events_file; / handle for "cgroup.events",
// handles for "{cpu,memory,io,irq}.pressure"
    pub psi_files: [cgroup_file; NR_PSI_RESOURCES],
//
// The bitmask of subsystems enabled on the child cgroups.
// ->subtree_control is the one configured through
// "cgroup.subtree_control" while ->subtree_ss_mask is the effective
// one which may have more subsystems enabled.  Controller knobs
// are made available iff it's enabled in ->subtree_control.
//
    pub subtree_control: u32,
    pub subtree_ss_mask: u32,
    pub old_subtree_control: u32,
    pub old_subtree_ss_mask: u32,
// Private pointers for each registered subsystem
    pub subsys: [*mut cgroup_subsys_state __rcu; CGROUP_SUBSYS_COUNT],
//
// Keep track of total number of dying CSSes at and below this cgroup.
// Protected by cgroup_mutex.
//
    pub nr_dying_subsys: [c_int; CGROUP_SUBSYS_COUNT],
    pub root: *mut cgroup_root,
//
// List of cgrp_cset_links pointing at css_sets with tasks in this
// cgroup.  Protected by css_set_lock.
//
    pub cset_links: list_head,
//
// On the default hierarchy, a css_set for a cgroup with some
// susbsys disabled will point to css's which are associated with
// the closest ancestor which has the subsys enabled.  The
// following lists all css_sets which point to this cgroup's css
// for the given subsystem.
//
    pub e_csets: [list_head; CGROUP_SUBSYS_COUNT],
//
// If !threaded, self.  If threaded, it points to the nearest
// domain ancestor.  Inside a threaded subtree, cgroups are exempt
// from process granularity and no-internal-task constraint.
// Domain level resource consumptions which aren't tied to a
// specific task are charged to the dom_cgrp.
//
    pub dom_cgrp: *mut cgroup,
    pub /: *mut *mut *mut cgroup old_dom_cgrp; / used while enabling threaded,
//
// Depending on the context, this field is initialized via
// css_rstat_init() at different places:
//
// when cgroup is the root cgroup
// performed in cgroup_setup_root()
// otherwise
// performed in cgroup_create()
//
    pub rstat_base_cpu: *mut cgroup_rstat_base_cpu __percpu,
//
// Add padding to keep the read mostly rstat per-cpu pointer on a
// different cacheline than the following *bstat fields which can have
// frequent updates.
//
// cgroup basic resource statistics
    pub last_bstat: cgroup_base_stat,
    pub bstat: cgroup_base_stat,
    pub /: *mut *mut prev_cputime prev_cputime; / for printing out cputime,
//
// list of pidlists, up to two for each namespace (one for procs, one
// for tasks); created on demand.
//
    pub pidlists: list_head,
    pub pidlist_mutex: mutex,
// used to wait for offlining of csses
    pub offline_waitq: wait_queue_head_t,
// used to schedule release agent
    pub release_agent_work: work_struct,
// used to track pressure stalls
    pub psi: *mut psi_group,
// used to store eBPF programs
    pub bpf: cgroup_bpf,
// Used to store internal freezer state
    pub freezer: cgroup_freezer_state,

    pub bpf_cgrp_storage: *mut bpf_local_storage __rcu,

    pub scx_sched: *mut scx_sched __rcu,

// All ancestors including self
    pub ancestors): *mut *mut DECLARE_FLEX_ARRAY(struct cgroup ,,
    pub _root_ancestor: *mut cgroup,
    pub _low_ancestors): *mut *mut DECLARE_FLEX_ARRAY(struct cgroup ,,
}

//
// A cgroup_root represents the root of a cgroup hierarchy, and may be
// associated with a kernfs_root to form an active hierarchy.  This is
// internal to cgroup core.  Don't access directly from controllers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgroup_root {
    pub kf_root: *mut kernfs_root,
// The bitmask of subsystems attached to this hierarchy
    pub subsys_mask: c_uint,
// Unique id for this hierarchy.
    pub hierarchy_id: c_int,
// A list running through the active hierarchies
    pub root_list: list_head,
    pub /: *mut *mut rcu_head rcu; / Must be near the top,
// Number of cgroups in the hierarchy, used only for /proc/cgroups
    pub nr_cgrps: core::sync::atomic::AtomicI32,
// Hierarchy-specific flags
    pub flags: c_uint,
// The path to use for release notifications.
    pub release_agent_path: [c_char; PATH_MAX],
// The name for this hierarchy - may be empty
    pub name: [c_char; MAX_CGROUP_ROOT_NAMELEN],
//
// The root cgroup. The containing cgroup_root will be destroyed on its
// release. This must be embedded last due to flexible array at the end
// of struct cgroup.
//
    pub cgrp: cgroup,
}

//
// struct cftype: handler definitions for cgroup control files
//
// When reading/writing to a file:
// - the cgroup to use is file->f_path.dentry->d_parent->d_fsdata
// - the 'cftype' of the file is file->f_path.dentry->d_fsdata
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cftype {
//
// Name of the subsystem is prepended in cgroup_file_name().
// Zero length string indicates end of cftype array.
//
    pub name: [c_char; MAX_CFTYPE_NAME],
    pub private: c_ulong,
//
// The maximum length of string, excluding trailing nul, that can
// be passed to write.  If < PAGE_SIZE-1, PAGE_SIZE-1 is assumed.
//
    pub max_write_len: usize,
// CFTYPE_* flags
    pub flags: c_uint,
//
// If non-zero, should contain the offset from the start of css to
// a struct cgroup_file field.  cgroup will record the handle of
// the created file into it.  The recorded handle can be used as
// long as the containing css remains accessible.
//
    pub file_offset: c_uint,
//
// Fields used for internal bookkeeping.  Initialized automatically
// during registration.
//
    pub /: *mut *mut *mut cgroup_subsys ss; / NULL for cgroup core files,
    pub /: *mut *mut list_head node; / anchored at ss->cfts,
    pub kf_ops: *mut kernfs_ops,
    pub of): *mut *mut int (open)(struct kernfs_open_file,
    pub of): *mut *mut void (release)(struct kernfs_open_file,
//
// read_u64() is a shortcut for the common case of returning a
// single integer. Use it in place of read()
//
    pub cft): *mut *mut *mut u64 (read_u64)(struct cgroup_subsys_state css, struct cftype,
//
// read_s64() is a signed version of read_u64()
//
    pub cft): *mut *mut *mut s64 (read_s64)(struct cgroup_subsys_state css, struct cftype,
// generic seq_file read interface
    pub v): *mut *mut *mut int (seq_show)(struct seq_file sf, void,
// optional ops, implement all or none
    pub ppos): *mut *mut *mut *mut void (seq_start)(struct seq_file sf, loff_t,
    pub ppos): *mut *mut *mut *mut *mut void (seq_next)(struct seq_file sf, void v, loff_t,
    pub v): *mut *mut *mut void (seq_stop)(struct seq_file sf, void,
//
// write_u64() is a shortcut for the common case of accepting
// a single integer (as parsed by simple_strtoull) from
// userspace. Use in place of write(); return 0 or error.
//
    pub val): u64,
//
// write_s64() is a signed version of write_u64()
//
    pub val): i64,
//
// write() is the generic write callback which maps directly to
// kernfs write operation and overrides all other operations.
// Maximum write size is determined by ->max_write_len.  Use
// of_css/cft() to access the associated css and cft.
//
    pub off): *mut *mut char buf, size_t nbytes, loff_t,
    pub pt): *mut poll_table_struct,
    pub lockdep_key: lock_class_key,
}

//
// Control Group subsystem type.
// See Documentation/admin-guide/cgroup-v1/cgroups.rst for details
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgroup_subsys {
    pub parent_css): *mut *mut *mut cgroup_subsys_state (css_alloc)(cgroup_subsys_state,
    pub css): *mut *mut int (css_online)(struct cgroup_subsys_state,
    pub css): *mut *mut void (css_offline)(struct cgroup_subsys_state,
    pub css): *mut *mut void (css_released)(struct cgroup_subsys_state,
    pub css): *mut *mut void (css_free)(struct cgroup_subsys_state,
    pub css): *mut *mut void (css_reset)(struct cgroup_subsys_state,
    pub css): *mut *mut void (css_killed)(struct cgroup_subsys_state,
    pub cpu): *mut *mut *mut void (css_rstat_flush)(struct cgroup_subsys_state css, int,
    pub css): *mut cgroup_subsys_state,
    pub css): *mut cgroup_subsys_state,
    pub tset): *mut *mut int (can_attach)(struct cgroup_taskset,
    pub tset): *mut *mut void (cancel_attach)(struct cgroup_taskset,
    pub tset): *mut *mut void (attach)(struct cgroup_taskset,
    pub cset): *mut css_set,
    pub cset): *mut *mut *mut void (cancel_fork)(struct task_struct task, struct css_set,
    pub task): *mut *mut void (fork)(struct task_struct,
    pub task): *mut *mut void (exit)(struct task_struct,
    pub task): *mut *mut void (release)(struct task_struct,
    pub root_css): *mut *mut void (bind)(struct cgroup_subsys_state,
    pub early_init:1: bool,
//
// If %true, the controller, on the default hierarchy, doesn't show
// up in "cgroup.controllers" or "cgroup.subtree_control", is
// implicitly enabled on all cgroups on the default hierarchy, and
// bypasses the "no internal process" constraint.  This is for
// utility type controllers which is transparent to userland.
//
// An implicit controller can be stolen from the default hierarchy
// anytime and thus must be okay with offline csses from previous
// hierarchies coexisting with csses for the current one.
//
    pub implicit_on_dfl:1: bool,
//
// If %true, the controller, supports threaded mode on the default
// hierarchy.  In a threaded subtree, both process granularity and
// no-internal-process constraint are ignored and a threaded
// controllers should be able to handle that.
//
// Note that as an implicit controller is automatically enabled on
// all cgroups on the default hierarchy, it should also be
// threaded.  implicit && !threaded is not supported.
//
    pub threaded:1: bool,
// the following two fields are initialized automatically during boot
    pub id: c_int,
    pub name: *const c_char,
// optional, initialized automatically during boot if not set
    pub legacy_name: *const c_char,
// link to parent, protected by cgroup_lock()
    pub root: *mut cgroup_root,
// idr for css->id
    pub css_idr: idr,
//
// List of cftypes.  Each entry is the first entry of an array
// terminated by zero length name.
//
    pub cfts: list_head,
//
// Base cftypes which are automatically registered.  The two can
// point to the same array.
//
    pub /: *mut *mut *mut cftype dfl_cftypes; / for the default hierarchy,
    pub /: *mut *mut *mut cftype legacy_cftypes; / for the legacy hierarchies,
//
// A subsystem may depend on other subsystems.  When such subsystem
// is enabled on a cgroup, the depended-upon subsystems are enabled
// together if available.  Subsystems enabled due to dependency are
// not visible to userland until explicitly enabled.  The following
// specifies the mask of subsystems that this one depends on.
//
    pub depends_on: c_uint,
    pub rstat_ss_lock: spinlock_t,
    pub /: *mut *mut *mut llist_head __percpu lhead; / lockless update list head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgroup_of_peak {
    pub value: c_ulong,
    pub list: list_head,
}

//
// cgroup_threadgroup_change_begin - threadgroup exclusion for cgroups
// @tsk: target task
//
// Allows cgroup operations to synchronize against threadgroup changes
// using a global percpu_rw_semaphore and a per threadgroup rw_semaphore when
// favordynmods is on. See the comment above CGRP_ROOT_FAVOR_DYNMODS definition.
//
// cgroup_threadgroup_change_end - threadgroup exclusion for cgroups
// @tsk: target task
//
// Counterpart of cgroup_threadgroup_change_begin().
//

pub const CGROUP_SUBSYS_COUNT: c_int = 0;

//
// sock_cgroup_data is embedded at sock->sk_cgrp_data and contains
// per-socket cgroup information except for memcg association.
//
// On legacy hierarchies, net_prio and net_cls controllers directly
// set attributes on each sock which can then be tested by the network
// layer. On the default hierarchy, each sock is associated with the
// cgroup it was created in and the networking layer can match the
// cgroup directly.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sock_cgroup_data {
    pub /: *mut *mut *mut cgroup cgroup; / v2,

    pub /: *mut *mut u32 classid; / v1,

    pub /: *mut *mut u16 prioidx; / v1,

}

extern "C" {
    pub fn READ_ONCE(_arg: skcd->prioidx) -> return;
}

extern "C" {
    pub fn READ_ONCE(_arg: skcd->classid) -> return;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sock_cgroup_data {
}

