//! Automatically rewritten from C Header to Rust Module
//! Source: tools/sched_ext/include/scx/cid.bpf.h
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
// BPF-side helpers for cids and cmasks. See kernel/sched/ext/cid.h for the
// authoritative layout and semantics. The BPF-side helpers use the cmask_
// naming (no scx_ prefix); cmask is the SCX bitmap type so the prefix is
// redundant in BPF code. Atomics use __sync_val_compare_and_swap and every
// helper is inline (no .c counterpart).
//
// Included by scx/common.bpf.h; don't include directly.
//
// Copyright (c) 2026 Meta Platforms, Inc. and affiliates.
// Copyright (c) 2026 Tejun Heo <tj@kernel.org>
//

//
// Storage cap for bounded loops over bits[]. Sized to cover NR_CPUS=8192 with
// one extra word for head-misalignment. Increase if deployment targets larger
// NR_CPUS.
//

pub const CMASK_MAX_WORDS: c_int = 129;

//
// Mirrors SCX_CMASK_NR_WORDS in kernel/sched/ext/types.h. The u64 cast keeps
// the +63 from wrapping when @nr_cids is near U32_MAX, so cmask_reframe()
// bounds-checking the result against alloc_words catches the overflow instead
// of seeing a small value.
//

//
// __cmask_init - Initialize @m with explicit storage capacity
// @m: cmask to initialize
// @base: first cid of the active range
// @nr_cids: number of cids in the active range
// @alloc_cids: storage capacity in cids, at least @nr_cids
//
// Use when storage is sized larger than the initial active range. All of
// bits[] is zeroed.
//
// cmask_init - Initialize @m on tight storage
// @m: cmask to initialize
// @base: first cid of the active range
// @nr_cids: number of cids in the active range
//
// All of bits[] is zeroed.
//
// cmask_reframe - Reshape @m's active range without resizing storage
// @m: cmask to reframe
// @base: new active range base
// @nr_cids: new active range length, must fit within @m->alloc_words
//
// Body bits within the new range become garbage - only the head and tail
// words are zeroed to keep the padding invariant.
//
// x86 BPF JIT rejects BPF_OR | BPF_FETCH and BPF_AND | BPF_FETCH on arena
// pointers (see bpf_jit_supports_insn() in arch/x86/net/bpf_jit_comp.c). Only
// BPF_CMPXCHG / BPF_XCHG / BPF_ADD with FETCH are allowed. Implement
// test_and_{set,clear} and the atomic set/clear via a cmpxchg loop.
//
// CMASK_CAS_TRIES is sized so exhausting it means seconds of real spinning
// on one word - past any plausible contention. Abort hard.
//

// __cmask_word(cid, m) |= BIT_U64(cid & 63);
// __cmask_word(cid, m) &= ~BIT_U64(cid & 63);
// w |= bit;
// w &= ~bit;
//
// BPF_-prefixed to avoid colliding with the kernel's anonymous CMASK_OP_
// enum in ext/cid.c, which is exported via BTF and reachable through
// vmlinux.h.
//
// cmask_and/or/copy only modify @dst bits that lie in the intersection of
// [@dst->base, @dst->base + @dst->nr_cids) and [@src->base,
// @src->base + @src->nr_cids). Bits in @dst outside that window
// keep their prior values - in particular, cmask_copy() does NOT zero @dst
// bits that lie outside @src's range.
//
// True iff @a and @b have identical bits over their (assumed equal) range.
// Callers are expected to pass same-shape cmasks; differing shapes always
// compare unequal.
//
// cmask_next_set - find the first set bit at or after @cid
// @m: cmask to search
// @cid: starting cid (clamped to @m->base if below)
//
// Returns the smallest set cid in [@cid, @m->base + @m->nr_cids), or
// @m->base + @m->nr_cids if none (the out-of-range sentinel matches the
// termination condition used by cmask_for_each()).
//
extern "C" {
    pub fn cmask_next_set(_arg: m, _arg: m->base) -> return;
}

//
// True iff every bit set in @a is also set in @b. Matches the kernel-side
// scx_cmask_subset(): ranges don't need to nest, and set bits of @a outside
// @b's range fail the test.
//
// set bits of @a outside @b's range can't be in @b
//
// Walk the words the range intersection spans. Plain word tests
// suffice: the scans above guarantee @a has no set bit outside @b's
// range and padding bits are kept clear by all cmask helpers.
//
// Population count over [base, base + nr_cids). Padding bits in the head/tail
// words are guaranteed zero by the mutating helpers, so a flat popcount over
// the words the range spans is correct.
//
// True if @a and @b share any set bit. Walk only the intersection of their
// ranges, matching the semantics of cmask_and().
//
// Find the next cid set in both @a and @b at or after @start, bounded by the
// intersection of the two ranges. Return a->base + a->nr_cids if none found.
//
// Building block for cmask_next_and_set_wrap(). Callers that want a bounded
// scan without wrap call this directly.
//
// Find the next set cid in @m at or after @start, wrapping to @m->base if no
// set bit is found in [start, m->base + m->nr_cids). Return m->base +
// m->nr_cids if @m is empty.
//
// Callers do round-robin distribution by passing (last_cid + 1) as @start.
//
// Find the next cid set in both @a and @b at or after @start, wrapping to
// @a->base if none found in the forward half. Return a->base + a->nr_cids
// if the intersection is empty.
//
// Callers do round-robin distribution by passing (last_cid + 1) as @start.
//
// Like cmask_next_and_set() but over the intersection of THREE masks. Return
// a->base + a->nr_cids if no cid is set in all three at or after @start.
//
// Round-robin variant of cmask_next_and2_set(): wrap to @a->base if the
// three-way intersection has no cid in the forward half. Return a->base +
// a->nr_cids if empty.
//
// cmask_from_cpumask - translate a kernel cpumask to a cid-space cmask
// @m: cmask to fill. Zeroed first; only bits within [@m->base, @m->base +
// @m->nr_cids) are updated - cpus mapping to cids outside that range
// are ignored.
// @cpumask: kernel cpumask to translate
//
// For each cpu in @cpumask, set the cpu's cid in @m. Caller must ensure
// @cpumask stays stable across the call (e.g. RCU read lock for
// task->cpus_ptr).
//
