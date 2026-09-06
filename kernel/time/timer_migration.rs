//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/time/timer_migration.h
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
// Per group capacity. Must be a power of 2!
pub const TMIGR_CHILDREN_PER_GROUP: c_int = 8;
//
// struct tmigr_hierarchy - a hierarchy associated to a given CPU capacity.
// Homogeneous systems have only one hierarchy.
// Heterogenous have one hierarchy per CPU capacity.
// @cpumask:	CPUs belonging to this hierarchy
// @root:	The current root of the hierarchy
// @capacity:	CPU capacity associated to this hierarchy
// @node:	Node in the global hierarchy list
// @level_list:	Per level lists of tmigr groups
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmigr_hierarchy {
    pub cpumask: *mut cpumask,
    pub root: *mut tmigr_group,
    pub capacity: c_ulong,
    pub node: list_head,
    pub level_list: [list_head; ],
}

//
// struct tmigr_event - a timer event associated to a CPU
// @nextevt:	The node to enqueue an event in the parent group queue
// @cpu:	The CPU to which this event belongs
// @ignore:	Hint whether the event could be ignored; it is set when
// CPU or group is active;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmigr_event {
    pub nextevt: timerqueue_node,
    pub cpu: c_uint,
    pub ignore: bool,
}

//
// struct tmigr_group - timer migration hierarchy group
// @lock:		Lock protecting the event information and group hierarchy
// information during setup
// @parent:		Pointer to the parent group. Pointer is updated when a
// new hierarchy level is added because of a CPU coming
// online the first time. Once it is set, the pointer will
// not be removed or updated. When accessing parent pointer
// lock less to decide whether to abort a propagation or
// not, it is not a problem. The worst outcome is an
// unnecessary/early CPU wake up. But do not access parent
// pointer several times in the same 'action' (like
// activation, deactivation, check for remote expiry,...)
// without holding the lock as it is not ensured that value
// will not change.
// @groupevt:		Next event of the group which is only used when the
// group is !active. The group event is then queued into
// the parent timer queue.
// Ignore bit of @groupevt is set when the group is active.
// @next_expiry:	Base monotonic expiry time of the next event of the
// group; It is used for the racy lockless check whether a
// remote expiry is required; it is always reliable
// @events:		Timer queue for child events queued in the group
// @migr_state:		State of the group (see union tmigr_state)
// @level:		Hierarchy level of the group; Required during setup
// @numa_node:		Required for setup only to make sure CPU and low level
// group information is NUMA local. It is set to NUMA node
// as long as the group level is per NUMA node (level <
// tmigr_crossnode_level); otherwise it is set to
// NUMA_NO_NODE
// @num_children:	Counter of group children to make sure the group is only
// filled with TMIGR_CHILDREN_PER_GROUP; Required for setup
// only
// @groupmask:		mask of the group in the parent group; is set during
// setup and will never change; can be read lockless
// @list:		List head that is added to the per level
// tmigr_level_list; is required during setup when a
// new group needs to be connected to the existing
// hierarchy groups
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmigr_group {
    pub lock: raw_spinlock_t,
    pub parent: *mut tmigr_group,
    pub groupevt: tmigr_event,
    pub next_expiry: u64,
    pub events: timerqueue_head,
    pub migr_state: core::sync::atomic::AtomicI32,
    pub level: c_uint,
    pub numa_node: c_int,
    pub num_children: c_uint,
    pub groupmask: u8,
    pub list: list_head,
}

//
// struct tmigr_cpu - timer migration per CPU group
// @lock:		Lock protecting the tmigr_cpu group information
// @available:		Indicates whether the CPU is available for handling
// global timers. In the deactivate path it is required to
// know whether the migrator in the top level group is to
// be set offline, while a timer is pending. Then another
// available CPU needs to be notified to take over the
// migrator role. Furthermore the information is required
// in the CPU hotplug path as the CPU is able to go idle
// before the timer migration hierarchy hotplug callback is
// reached.  During this phase, the CPU has to handle the
// global timers on its own and must not act as a migrator.
//
// @idle:		Indicates whether the CPU is idle in the timer migration
// hierarchy
// @remote:		Is set when timers of the CPU are expired remotely
// @tmgroup:		Pointer to the parent group
// @groupmask:		mask of tmigr_cpu in the parent group
// @wakeup:		Stores the first timer when the timer migration
// hierarchy is completely idle and remote expiry was done;
// is returned to timer code in the idle path and is only
// used in idle path.
// @cpuevt:		CPU event which could be enqueued into the parent group
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmigr_cpu {
    pub lock: raw_spinlock_t,
    pub available: bool,
    pub idle: bool,
    pub remote: bool,
    pub tmgroup: *mut tmigr_group,
    pub groupmask: u8,
    pub wakeup: u64,
    pub cpuevt: tmigr_event,
}

//
// union tmigr_state - state of tmigr_group
// @state:	Combined version of the state - only used for atomic
// read/cmpxchg function
// &anon struct: Split version of the state - only use the struct members to
// update information to stay independent of endianness
// @active:	Contains each mask bit of the active children
// @migrator:	Contains mask of the child which is migrator
// @seq:	Sequence counter needs to be increased when an update
// to the tmigr_state is done. It prevents a race when
// updates in the child groups are propagated in changed
// order. Detailed information about the scenario is
// given in the documentation at the begin of
// timer_migration.c.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union tmigr_state {
    pub state: u32,
    pub active: u8,
    pub migrator: u8,
    pub seq: u16,
    pub __packed: },
}

extern "C" {
    pub fn tmigr_handle_remote();
}
extern "C" {
    pub fn tmigr_requires_handle_remote() -> bool;
}
extern "C" {
    pub fn tmigr_cpu_activate();
}
extern "C" {
    pub fn tmigr_cpu_deactivate(nextevt: u64) -> u64;
}
extern "C" {
    pub fn tmigr_cpu_new_timer(nextevt: u64) -> u64;
}
extern "C" {
    pub fn tmigr_quick_check(nextevt: u64) -> u64;
}

