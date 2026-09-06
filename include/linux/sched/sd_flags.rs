//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sched/sd_flags.h
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
// sched-domains (multiprocessor balancing) flag declarations.
//

//
// Hierarchical metaflags
//
// SHARED_CHILD: These flags are meant to be set from the base domain upwards.
// If a domain has this flag set, all of its children should have it set. This
// is usually because the flag describes some shared resource (all CPUs in that
// domain share the same resource), or because they are tied to a scheduling
// behaviour that we want to disable at some point in the hierarchy for
// scalability reasons.
//
// In those cases it doesn't make sense to have the flag set for a domain but
// not have it in (some of) its children: sched domains ALWAYS span their child
// domains, so operations done with parent domains will cover CPUs in the lower
// child domains.
//
// SHARED_PARENT: These flags are meant to be set from the highest domain
// downwards. If a domain has this flag set, all of its parents should have it
// set. This is usually for topology properties that start to appear above a
// certain level (e.g. domain starts spanning CPUs outside of the base CPU's
// socket).
//
pub const SDF_SHARED_CHILD: c_uint = 0x1;
pub const SDF_SHARED_PARENT: c_uint = 0x2;
//
// Behavioural metaflags
//
// NEEDS_GROUPS: These flags are only relevant if the domain they are set on has
// more than one group. This is usually for balancing flags (load balancing
// involves equalizing a metric between groups), or for flags describing some
// shared resource (which would be shared between groups).
//
pub const SDF_NEEDS_GROUPS: c_uint = 0x4;
//
// Balance when about to become idle
//
// SHARED_CHILD: Set from the base domain up to cpuset.sched_relax_domain_level.
// NEEDS_GROUPS: Load balancing flag.
//
// Balance on exec
//
// SHARED_CHILD: Set from the base domain up to the NUMA reclaim level.
// NEEDS_GROUPS: Load balancing flag.
//
// Balance on fork, clone
//
// SHARED_CHILD: Set from the base domain up to the NUMA reclaim level.
// NEEDS_GROUPS: Load balancing flag.
//
// Balance on wakeup
//
// SHARED_CHILD: Set from the base domain up to cpuset.sched_relax_domain_level.
// NEEDS_GROUPS: Load balancing flag.
//
// Consider waking task on waking CPU.
//
// SHARED_CHILD: Set from the base domain up to the NUMA reclaim level.
//
// Domain members have different CPU capacities
//
// SHARED_PARENT: Set from the topmost domain down to the first domain where
// asymmetry is detected.
// NEEDS_GROUPS: Per-CPU capacity is asymmetric between groups.
//
// Domain members have different CPU capacities spanning all unique CPU
// capacity values.
//
// SHARED_PARENT: Set from the topmost domain down to the first domain where
// all available CPU capacities are visible
// NEEDS_GROUPS: Per-CPU capacity is asymmetric between groups.
//
// Domain members share CPU capacity (i.e. SMT)
//
// SHARED_CHILD: Set from the base domain up until spanned CPUs no longer share
// CPU capacity.
// NEEDS_GROUPS: Capacity is shared between groups.
//
// Domain members share CPU cluster (LLC tags or L2 cache)
//
// NEEDS_GROUPS: Clusters are shared between groups.
//
// Domain members share CPU Last Level Caches
//
// SHARED_CHILD: Set from the base domain up until spanned CPUs no longer share
// the same cache(s).
// NEEDS_GROUPS: Caches are shared between groups.
//
// Only a single load balancing instance
//
// SHARED_PARENT: Set for all NUMA levels above NODE. Could be set from a
// different level upwards, but it doesn't change that if a
// domain has this flag set, then all of its parents need to have
// it too (otherwise the serialization doesn't make sense).
// NEEDS_GROUPS: No point in preserving domain if it has a single group.
//
// Place busy tasks earlier in the domain
//
// NEEDS_GROUPS: Load balancing flag.
//
// Prefer to place tasks in a sibling domain
//
// Set up until domains start spanning NUMA nodes.
//
// NEEDS_GROUPS: Load balancing flag.
//
// Cross-node balancing
//
// SHARED_PARENT: Set for all NUMA levels above NODE.
// NEEDS_GROUPS: No point in preserving domain if it has a single group.
//
