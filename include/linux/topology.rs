//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/topology.h
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


//
// include/linux/topology.h
//
// Written by: Matthew Dobson, IBM Corporation
//
// Copyright (C) 2002, IBM Corp.
//
// All rights reserved.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY OR FITNESS FOR A PARTICULAR PURPOSE, GOOD TITLE or
// NON INFRINGEMENT.  See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 675 Mass Ave, Cambridge, MA 02139, USA.
//
// Send feedback to <colpatch@us.ibm.com>
//

extern "C" {
    pub fn arch_update_cpu_topology() -> c_int;
}
// Conform to ACPI 2.0 SLIT distance definitions
pub const LOCAL_DISTANCE: c_int = 10;
pub const REMOTE_DISTANCE: c_int = 20;
pub const DISTANCE_BITS: c_int = 8;

//
// If the distance between nodes in a system is larger than RECLAIM_DISTANCE
// (in whatever arch specific measurement units returned by node_distance())
// and node_reclaim_mode is enabled then the VM will only call node_reclaim()
// on nodes within this distance.
//
pub const RECLAIM_DISTANCE: c_int = 30;

//
// The following tunable allows platforms to override the default node
// reclaim distance (RECLAIM_DISTANCE) if remote memory accesses are
// sufficiently fast that the default value actually hurts
// performance.
//
// AMD EPYC machines use this because even though the 2-hop distance
// is 32 (3.2x slower than a local memory access) performance actually
// *improves* if allowed to reclaim memory and load balance tasks
// between NUMA nodes 2-hops apart.
//

// Returns the number of the current Node.
extern "C" {
    pub fn raw_cpu_read(_arg: numa_node) -> return;
}

extern "C" {
    pub fn per_cpu(_arg: numa_node, _arg: cpu) -> return;
}

// Returns the number of the current Node.

extern "C" {
    pub fn cpu_to_node(_arg: raw_smp_processor_id()) -> return;
}

//
// N.B., Do NOT reference the '_numa_mem_' per cpu variable directly.
// It will not be defined when CONFIG_HAVE_MEMORYLESS_NODES is not defined.
// Use the accessor functions set_numa_mem(), numa_mem_id() and cpu_to_mem().
//

// Returns the number of the nearest Node with memory
extern "C" {
    pub fn raw_cpu_read(_arg: _numa_mem_) -> return;
}

extern "C" {
    pub fn per_cpu(_arg: _numa_mem_, _arg: cpu) -> return;
}

// Returns the number of the nearest Node with memory
extern "C" {
    pub fn numa_node_id() -> return;
}

extern "C" {
    pub fn cpu_to_node(_arg: cpu) -> return;
}

// Macro flag: #define TOPOLOGY_DIE_SYSFS

// Macro flag: #define TOPOLOGY_CLUSTER_SYSFS

// Macro flag: #define TOPOLOGY_BOOK_SYSFS

// Macro flag: #define TOPOLOGY_DRAWER_SYSFS

//
// Defining cpu_smt_mask as cpumask_of that CPU helps to get
// rid of lot of ifdeffery all around the codebase in case of
// CONFIG_SCHED_SMT=n. It just means there are no other siblings, which
// is what is expected.
//

extern "C" {
    pub fn topology_sibling_cpumask(_arg: cpu) -> return;
}

extern "C" {
    pub fn cpumask_of(_arg: cpu) -> return;
}

//
// When disabling SMT, the primary thread of the SMT will remain
// enabled/active. Architectures that have a special primary thread
// (e.g. x86) need to override this function. Otherwise the first
// thread in the SMT can be made the primary thread.
//
// The sibling cpumask of an offline CPU always contains the CPU
// itself on architectures using the implementation of
// CONFIG_GENERIC_ARCH_TOPOLOGY for building their topology.
// Other architectures not using CONFIG_GENERIC_ARCH_TOPOLOGY for
// building their topology have to check whether to use this default
// implementation or to override it.
//

extern "C" {
    pub fn cpumask_of_node(_arg: cpu_to_node(cpu)) -> return;
}

extern "C" {
    pub fn sched_numa_find_nth_cpu(cpus: *const cpumask, cpu: c_int, node: c_int) -> c_int;
}

extern "C" {
    pub fn cpumask_nth_and(_arg: cpu, _arg: cpus, _arg: cpu_online_mask) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}

//
// for_each_node_numadist() - iterate over nodes in increasing distance
// order, starting from a given node
// @node: the iteration variable and the starting node.
// @unvisited: a nodemask to keep track of the unvisited nodes.
//
// This macro iterates over NUMA node IDs in increasing distance from the
// starting @node and yields MAX_NUMNODES when all the nodes have been
// visited.
//
// Note that by the time the loop completes, the @unvisited nodemask will
// be fully cleared, unless the loop exits early.
//
// The difference between for_each_node() and for_each_node_numadist() is
// that the former allows to iterate over nodes in numerical order, whereas
// the latter iterates over nodes in increasing order of distance.
//
// This complexity of this iterator is O(N^2), where N represents the
// number of nodes, as each iteration involves scanning all nodes to
// find the one with the shortest distance.
//
// Requires rcu_lock to be held.
//

//
// for_each_numa_hop_mask - iterate over cpumasks of increasing NUMA distance
// from a given node.
// @mask: the iteration variable.
// @node: the NUMA node to start the search from.
//
// Requires rcu_lock to be held.
//
// Yields cpu_online_mask for @node == NUMA_NO_NODE.
//

extern "C" {
    pub fn per_cpu(_arg: cpu_scale, _arg: cpu) -> return;
}
extern "C" {
    pub fn topology_set_cpu_scale(cpu: c_uint, capacity: c_ulong);
}
