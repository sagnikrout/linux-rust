//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/topology.h
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
// to preserve the visibility of NUMA_NO_NODE definition,
// moved to there from here.  May be used independent of
// CONFIG_NUMA.
//

// Mappings between logical cpu number and node number

//
// override generic percpu implementation of cpu_to_node
//
extern "C" {
    pub fn __cpu_to_node(cpu: c_int) -> c_int;
}

extern "C" {
    pub fn early_cpu_to_node(cpu: c_int) -> c_int;
}

// Same function but used if called before per_cpu areas are setup
extern "C" {
    pub fn early_per_cpu(_arg: x86_cpu_to_node_map, _arg: cpu) -> return;
}

// Mappings between node number and cpus on that node.

// Returns a pointer to the cpumask of CPUs on Node 'node'.

extern "C" {
    pub fn setup_node_to_cpumask_map();
}

extern "C" {
    pub fn __node_distance(_arg: c_int, _arg: c_int) -> c_int;
}

//
// indicate override:
//

// Topology information
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum x86_topology_domains {
    TOPO_SMT_DOMAIN,
    TOPO_CORE_DOMAIN,
    TOPO_MODULE_DOMAIN,
    TOPO_TILE_DOMAIN,
    TOPO_DIE_DOMAIN,
    TOPO_DIEGRP_DOMAIN,
    TOPO_PKG_DOMAIN,
    TOPO_MAX_DOMAIN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amd_cpu_type {
    AMD_CPU_TYPE_PERFORMANCE	= 0,
    AMD_CPU_TYPE_EFFICIENCY		= 1,
    AMD_CPU_TYPE_LOW_POWER		= 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct x86_topology_system {
    pub dom_shifts: [c_uint; TOPO_MAX_DOMAIN],
    pub dom_size: [c_uint; TOPO_MAX_DOMAIN],
}

extern "C" {
    pub fn topology_get_logical_id(apicid: u32, at_level: x86_topology_domains) -> c_int;
}

//
// topology_is_primary_thread - Check whether CPU is the primary SMT thread
// @cpu:	CPU to check
//
extern "C" {
    pub fn cpumask_test_cpu(_arg: cpu, _arg: cpu_primary_thread_mask) -> return;
}

extern "C" {
    pub fn topology_get_primary_thread(cpu: c_uint) -> c_int;
}

extern "C" {
    pub fn x86_pci_root_bus_node(bus: c_int) -> c_int;
}
extern "C" {
    pub fn x86_pci_root_bus_resources(bus: c_int, resources: *mut list_head);
}

// Interface to set priority of a cpu
extern "C" {
    pub fn sched_set_itmt_core_prio(prio: c_int, core_cpu: c_int);
}
// Interface to notify scheduler that system supports ITMT
extern "C" {
    pub fn sched_set_itmt_support() -> c_int;
}
// Interface to notify scheduler that system revokes ITMT support
extern "C" {
    pub fn sched_clear_itmt_support();
}

extern "C" {
    pub fn per_cpu(_arg: arch_freq_scale, _arg: cpu) -> return;
}

extern "C" {
    pub fn arch_enable_hybrid_capacity_scale() -> bool;
}
extern "C" {
    pub fn arch_scale_cpu_capacity(cpu: c_int) -> c_ulong;
}

extern "C" {
    pub fn arch_set_max_freq_ratio(turbo_disabled: bool);
}
extern "C" {
    pub fn freq_invariance_set_perf_ratio(ratio: u64, turbo_disabled: bool);
}

extern "C" {
    pub fn arch_scale_freq_tick();
}

extern "C" {
    pub fn arch_sched_node_distance(from: c_int, to: c_int) -> c_int;
}
