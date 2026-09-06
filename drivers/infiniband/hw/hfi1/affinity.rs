//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/hfi1/affinity.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright(c) 2015 - 2020 Intel Corporation.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irq_type {
    IRQ_SDMA,
    IRQ_RCVCTXT,
    IRQ_NETDEVCTXT,
    IRQ_GENERAL,
    IRQ_OTHER
}

// Can be used for both memory and cpu
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum affinity_flags {
    AFF_AUTO,
    AFF_NUMA_LOCAL,
    AFF_DEV_LOCAL,
    AFF_IRQ_LOCAL
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_mask_set {
    pub mask: cpumask,
    pub used: cpumask,
    pub gen: c_uint,
}

// Initialize non-HT cpu cores mask
extern "C" {
    pub fn init_real_cpu_mask();
}
// Initialize driver affinity data
extern "C" {
    pub fn hfi1_dev_affinity_init(dd: *mut hfi1_devdata) -> c_int;
}
//
// Set IRQ affinity to a CPU. The function will determine the
// CPU and set the affinity to it.
//
// Remove the IRQ's CPU affinity. This function also updates
// any internal CPU tracking data
//
// Determine a CPU affinity for a user process, if the process does not
// have an affinity set yet.
//
extern "C" {
    pub fn hfi1_get_proc_affinity(node: c_int) -> c_int;
}
// Release a CPU used by a user process.
extern "C" {
    pub fn hfi1_put_proc_affinity(cpu: c_int);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi1_affinity_node {
    pub node: c_int,
    pub comp_vect_affinity: *mut u16 __percpu,
    pub def_intr: cpu_mask_set,
    pub rcv_intr: cpu_mask_set,
    pub general_intr_mask: cpumask,
    pub comp_vect_mask: cpumask,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi1_affinity_node_list {
    pub list: list_head,
    pub real_cpu_mask: cpumask,
    pub proc: cpu_mask_set,
    pub num_core_siblings: c_int,
    pub num_possible_nodes: c_int,
    pub num_online_nodes: c_int,
    pub num_online_cpus: c_int,
    pub /: *mut *mut mutex lock; / protects affinity nodes,
}

extern "C" {
    pub fn node_affinity_init() -> c_int;
}
extern "C" {
    pub fn node_affinity_destroy_all();
}
extern "C" {
    pub fn hfi1_dev_affinity_clean_up(dd: *mut hfi1_devdata);
}
extern "C" {
    pub fn hfi1_comp_vect_mappings_lookup(rdi: *mut rvt_dev_info, comp_vect: c_int) -> c_int;
}
extern "C" {
    pub fn hfi1_comp_vectors_set_up(dd: *mut hfi1_devdata) -> c_int;
}
extern "C" {
    pub fn hfi1_comp_vectors_clean_up(dd: *mut hfi1_devdata);
}
