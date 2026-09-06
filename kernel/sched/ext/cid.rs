//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/sched/ext/cid.h
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
// Topological CPU IDs (cids)
// --------------------------
//
// Raw cpu numbers are clumsy for sharding work and communication across
// topology units, especially from BPF: the space can be sparse, numerical
// closeness doesn't imply topological closeness (x86 hyperthreading often puts
// SMT siblings far apart), and a range of cpu ids doesn't mean anything.
// Sub-scheds make this acute - cpu allocation, revocation and other state are
// constantly communicated across sub-scheds, and passing whole cpumasks scales
// poorly with cpu count. cpumasks are also awkward in BPF: a variable-length
// kernel type sized for the maximum NR_CPUS (4k), with verbose helper sequences
// for every op.
//
// cids give every cpu a dense, topology-ordered id. CPUs sharing a core, LLC or
// NUMA node get contiguous cid ranges, so a topology unit becomes a (start,
// length) slice of cid space. Communication can pass a slice instead of a
// cpumask, and BPF code can process, for example, a u64 word's worth of cids at
// a time.
//
// The mapping is built once at root scheduler enable time by walking the
// topology of online cpus only. Going by online cpus is out of necessity:
// depending on the arch, topology info isn't reliably available for offline
// cpus. The expected usage model is restarting the scheduler on hotplug events
// so the mapping is rebuilt against the new online set. A scheduler that wants
// to handle hotplug without a restart can provide its own cid and shard mapping
// through the override interface.
//
// Copyright (c) 2026 Meta Platforms, Inc. and affiliates.
// Copyright (c) 2026 Tejun Heo <tj@kernel.org>
//

//
// Cid space (total is always num_possible_cpus()) is laid out with
// topology-annotated cids first, then no-topo cids at the tail. The
// topology-annotated block covers the cpus that were online when scx_cid_init()
// ran and remains valid even after those cpus go offline. The tail block covers
// possible-but-not-online cpus and carries all-(-1) topo info (see
// scx_cid_topo); callers detect it via the -1 sentinels.
//
// See the comment above the table definitions in cid.c for the
// memory-ordering and visibility contract.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scx_cid_tables {
    pub nr_shards: u32,
    pub /: *mut *mut *mut s16 cid_to_cpu; / [num_possible_cpus()],
    pub /: *mut *mut *mut s16 cpu_to_cid; / [nr_cpu_ids],
    pub /: *mut *mut *mut s32 cid_to_shard; / [num_possible_cpus()],
    pub /: *mut *mut *mut s32 shard_node; / [num_possible_cpus()],
    pub /: *mut *mut *mut scx_cid_shard shard_ranges; / [num_possible_cpus()],
    pub /: *mut *mut *mut scx_cid_topo topo; / [num_possible_cpus()],
    pub rcu: rcu_head,
}

extern "C" {
    pub fn scx_cmask_clear(m: *mut scx_cmask);
}
extern "C" {
    pub fn scx_cmask_fill(m: *mut scx_cmask);
}
extern "C" {
    pub fn scx_cmask_and(dst: *mut scx_cmask, src: *const scx_cmask);
}
extern "C" {
    pub fn scx_cmask_or(dst: *mut scx_cmask, src: *const scx_cmask);
}
extern "C" {
    pub fn scx_cmask_copy(dst: *mut scx_cmask, src: *const scx_cmask);
}
extern "C" {
    pub fn scx_cmask_andnot(dst: *mut scx_cmask, src: *const scx_cmask);
}
extern "C" {
    pub fn scx_cmask_subset(sub: *const scx_cmask, super: *const scx_cmask) -> bool;
}
extern "C" {
    pub fn scx_cmask_intersects(a: *const scx_cmask, b: *const scx_cmask) -> bool;
}
extern "C" {
    pub fn scx_cmask_empty(m: *const scx_cmask) -> bool;
}
extern "C" {
    pub fn scx_cid_init(sch: *mut scx_sched) -> i32;
}
extern "C" {
    pub fn scx_cid_publish_tables();
}
extern "C" {
    pub fn scx_cid_retire_tables();
}
extern "C" {
    pub fn scx_cid_kfunc_init() -> c_int;
}
//
// cid_valid - Verify a cid value, to be used on ops input args
// @sch: scx_sched to abort on error
// @cid: cid which came from a BPF ops
//
// Return true if @cid is in [0, num_possible_cpus()). On failure, trigger
// scx_error() and return false.
//
// __scx_cid_to_cpu - Unchecked cid->cpu table lookup
// @cid: cid to look up. Must be in [0, num_possible_cpus()).
//
// Intended for callsites that have already validated @cid and that run on a
// live scheduler, which guarantees the tables are published and stable.
//
// __scx_cpu_to_cid - Unchecked cpu->cid table lookup
// @cpu: cpu to look up. Must be a valid possible cpu id.
//
// Same usage constraints as __scx_cid_to_cpu().
//
// scx_cid_to_cpu - Translate @cid to its cpu
// @sch: scx_sched for error reporting
// @cid: cid to look up
//
// Return the cpu for @cid or a negative errno on failure. Invalid cid triggers
// scx_error() on @sch. The mapping is stable while the scheduler is live.
//
// Return -EINVAL without triggering scx_error() if no tables have been
// published yet, which a prog-facing kfunc can observe while racing the root
// scheduler enable.
//
// scx_cpu_to_cid - Translate @cpu to its cid
// @sch: scx_sched for error reporting
// @cpu: cpu to look up
//
// Return the cid for @cpu or a negative errno on failure. Invalid cpu triggers
// scx_error() on @sch. Same usage rules as scx_cid_to_cpu().
//
// scx_is_cid_type - Test whether the active scheduler hierarchy is cid-form
//
extern "C" {
    pub fn static_branch_unlikely(_arg: &__scx_is_cid_type) -> return;
}
extern "C" {
    pub fn likely(m->nr_cids: cid >= m->base && cid < m->base +) -> return;
}
// Word in bits[] covering @cid. @cid must satisfy __scx_cmask_contains().
//
// __scx_cmask_init - Initialize @m with explicit storage capacity
// @m: cmask to initialize
// @base: first cid of the active range
// @nr_cids: number of cids in the active range
// @alloc_cids: storage capacity in cids, at least @nr_cids
//
// Use when storage is sized larger than the initial active range. All of
// bits[] is zeroed.
//
// scx_cmask_init - Initialize @m on tight storage
// @m: cmask to initialize
// @base: first cid of the active range
// @nr_cids: number of cids in the active range
//
// All of bits[] is zeroed.
//
// scx_cmask_reframe - Reshape @m's active range without resizing storage
// @m: cmask to reframe
// @base: new active range base
// @nr_cids: new active range length, must fit within @m->alloc_words
//
// Body bits within the new range become garbage - only the head and tail
// words are zeroed to keep the padding invariant.
//
// __scx_cmask_word(cid, m) |= BIT_U64(cid & 63);
//
// scx_cmask_test - test whether @cid is set in @m
// @cid: cid to test
// @m: cmask to test
//
// Return %false if @cid is outside @m's active range. Otherwise return the
// bit's value. Read via READ_ONCE so callers can race set/clear writers.
//
extern "C" {
    pub fn READ_ONCE(_arg: *mut __scx_cmask_word(cid, 63: m)) & BIT_U64(cid &) -> return;
}
//
// Words of bits[] the active range spans, 0 if empty. Tighter than the storage
// SCX_CMASK_NR_WORDS() sizes for the worst-case base alignment.
//
// scx_cmask_for_each_cid - iterate set cids in @m
// @cid: s32 loop var that receives each set cid in turn
// @m: cmask to iterate
//
// Visits set bits within @m's active range in ascending order. Scans only the
// words the active range spans, where head and tail padding is kept zero, so
// no per-cid range check is needed.
//

//
// scx_cpu_arg() wraps a cpu arg being handed to an SCX op. For cid-form
// schedulers it resolves to the matching cid; for cpu-form it passes @cpu
// through. scx_cpu_ret() is the inverse for a cpu/cid returned from an op
// (currently only ops.select_cpu); it validates the BPF-supplied cid and
// triggers scx_error() on @sch if invalid.
//
extern "C" {
    pub fn __scx_cpu_to_cid(_arg: cpu) -> return;
}
extern "C" {
    pub fn scx_cid_to_cpu(_arg: sch, _arg: cpu_or_cid) -> return;
}
extern "C" {
    pub fn scx_cmask_ref_or(ref: *const scx_cmask_ref, src: *const scx_cmask);
}
extern "C" {
    pub fn scx_cmask_ref_copy(ref: *const scx_cmask_ref, src: *const scx_cmask);
}
