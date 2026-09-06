//! Automatically rewritten from C Header to Rust Module
//! Source: tools/sched_ext/scx_qmap.h
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
// Shared definitions between scx_qmap.bpf.c and scx_qmap.c.
//
// The scheduler keeps all state in a single BPF arena map. struct
// qmap_arena is the one object that lives at the base of the arena and is
// mmap'd into userspace so the loader can read counters directly.
//
// Copyright (c) 2026 Meta Platforms, Inc. and affiliates.
// Copyright (c) 2026 Tejun Heo <tj@kernel.org>
//

pub const MAX_SUB_SCHEDS: c_int = 8;

//
// cpu_ctxs[] is sized to a fixed cap so the layout is shared between BPF and
// userspace. Keep this in sync with NR_CPUS used by the BPF side.
//
pub const SCX_QMAP_MAX_CPUS: c_int = 1024;
//
// An owner id identifies who holds a cid: a child slot in [0, MAX_SUB_SCHEDS),
// CID_SELF for this node, CID_NONE for a cid not currently held, or CID_SHARED
// for a cid in the round-robin pool (its live holder is rr_slots[rr_pos]). Used
// by the partition's cid_owner[].
//

// -C cid-override test modes. Selects cid_override_mode in scx_qmap.bpf.c.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qmap_cid_override {
    QMAP_CID_OVR_OFF	= 0,	/* disabled */
    QMAP_CID_OVR_SHUFFLE	= 1,	/* valid reversed cpu->cid mapping */
    QMAP_CID_OVR_BAD_DUP	= 2,	/* invalid: duplicate cid assignment */
    QMAP_CID_OVR_BAD_RANGE	= 3,	/* invalid: out-of-range cid */
    QMAP_CID_OVR_BAD_MONO	= 4,	/* invalid: non-monotonic shard_start */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_ctx {
    pub /: *mut *mut u64 dsp_idx; / dispatch index,
    pub /: *mut *mut u64 dsp_cnt; / remaining count,
    pub avg_weight: u32,
    pub cpuperf_target: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmap_fifo {
    pub head: *mut task_ctx __arena,
    pub tail: *mut task_ctx __arena,
    pub idx: i32,
}

// -J fault-injection modes. Selects inject_mode in struct qmap_arena.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qmap_inject {
    QMAP_INJ_OFF		= 0,
    QMAP_INJ_WRONG_CID	= 1,	/* dispatch to a cid we don't hold */
    QMAP_INJ_INIT_FAIL	= 2,	/* fail init_task for "qmfail*" comms */
    QMAP_INJ_CGRP_INIT_FAIL	= 3,	/* fail cpuctl_init for "qmfail*" cgroups */
}

//
// scx_cmask's are embedded in struct qmap_arena with inline backing storage.
// The bpf side uses &field.mask with the normal cmask_* helpers. Userspace
// doesn't have access to the type definition and sees same-sized opaque words.
// _Static_assert()'s in .bpf.c ensure that they are in sync.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmap_cmask {

    pub mask: scx_cmask,
    pub 2]: u64 words[QMAP_CMASK_WORDS +,
}

// Opaque to userspace; defined in scx_qmap.bpf.c.
// per-direct-child state for the sub-scheduler
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sub_sched_ctx {
    pub cgroup_id: u64,
    pub /: *mut *mut u32 weight; / cpu.weight, seeded at attach, then set_weight,
    pub nr_dsps: u64,
    pub /: *mut *mut qmap_cmask granted_cids; / cids granted excl to this child,
    pub /: *mut *mut qmap_cmask prev_granted; / last grant, for delta calculation,
}

//
// compute_partition() builds the following from this node's held caps, and
// apply_partition()/rr_advance() execute it. Userspace only reads for the
// hierarchy display.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmap_partition {
    pub /: *mut *mut u32 nr_excl; / number of excl-held (delegatable) cids,
    pub /: *mut *mut s32 cid_owner[SCX_QMAP_MAX_CPUS]; / per cid: owner id, or CID_NONE,
    pub /: *mut *mut s32 shared_cids[MAX_PARTS]; / the round-robin cid pool,
    pub /: *mut *mut u32 nr_shared; / number of shared_cids entries,
    pub /: *mut *mut u64 rr_slots[MAX_PARTS]; / rotation order: holder cgroup_id, 0 = self,
    pub /: *mut *mut u32 nr_rr; / number of rr_slots entries,
    pub /: *mut *mut u32 rr_pos; / current rotation index,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qmap_arena {
// userspace-visible stats
    pub nr_reenqueued_cid0: u64 nr_enqueued, nr_dispatched, nr_reenqueued,,
    pub nr_ddsp_from_enq: u64 nr_dequeued,,
    pub nr_core_sched_execed: u64,
    pub nr_expedited_remote: u64 nr_expedited_local,,
    pub nr_expedited_from_timer: u64 nr_expedited_lost,,
    pub nr_highpri_queued: u64,
    pub test_error_cnt: u32,
    pub cpuperf_max: u32 cpuperf_min, cpuperf_avg,,
    pub cpuperf_target_max: u32 cpuperf_target_min, cpuperf_target_avg,,
// kernel-side runtime state
    pub core_sched_head_seqs: [u64; 5],
    pub core_sched_tail_seqs: [u64; 5],
    pub cpu_ctxs: [cpu_ctx; SCX_QMAP_MAX_CPUS],
// cid-override test input, populated by the loader before attach
    pub cid_override_cpu_to_cid: [__s32; SCX_QMAP_MAX_CPUS],
    pub cid_override_shard_start: [__s32; SCX_QMAP_MAX_CPUS],
// task_ctx slab; allocated and threaded by qmap_init()
    pub task_ctxs: *mut task_ctx __arena,
    pub task_free_head: *mut task_ctx __arena,
// five priority FIFOs, each a doubly-linked list through task_ctx
    pub fifos: [qmap_fifo; 5],
//
// Hierarchical sub-scheduling state. See the design comment at the top
// of scx_qmap.bpf.c.
//
    pub /: *mut *mut u32 nr_cids; / cid count, cached at init,
// bpf-owned partition: read by userspace for display
    pub part: qmap_partition,
    pub /: *mut *mut sub_sched_ctx sub_sched_ctxs[MAX_SUB_SCHEDS]; / per-child context,
    pub /: *mut *mut u64 nr_sub_scheds; / number of attached children,
// bpf-internal per-cid state
    pub /: *mut *mut u8 cid_shared[SCX_QMAP_MAX_CPUS]; / per cid: 1 if held shared (ENQ_IMMED-only),
// allocated cid-time, charged per owner by account_alloc()
    pub /: *mut *mut u64 alloc_ns[MAX_SUB_SCHEDS]; / per child slot,
    pub self_alloc_ns: u64,
    pub /: *mut *mut u64 alloc_ts; / last accounting timestamp,
    pub /: *mut *mut u64 alloc_window_ns; / total accounted time, the alloc denominator,
// bpf-internal cmasks (embedded, see struct qmap_cmask)
    pub /: *mut *mut qmap_cmask self_cids; / cids this node runs its own tasks on,
    pub /: *mut *mut qmap_cmask idle_cids; / idle state of all cids regardless of delegation,
    pub /: *mut *mut qmap_cmask rr_cids; / the shared pool, as a mask for grant/revoke,
// scratch cmasks
    pub /: *mut *mut qmap_cmask to_revoke_cids; / delta cids to revoke,
    pub /: *mut *mut qmap_cmask to_grant_cids; / delta cids to grant,
    pub /: *mut *mut qmap_cmask prev_rr_cids; / previous shared pool, to clear stale grants,
    pub /: *mut *mut qmap_cmask held_excl; / cids held excl (ENQ): delegatable,
    pub /: *mut *mut qmap_cmask held_shared; / cids held shared (ENQ_IMMED only): self-local,
// bpf -> userspace: stats
    pub /: *mut *mut u64 nr_reenq_cap; / SCX_TASK_REENQ_CAP bounces,
    pub /: *mut *mut u64 nr_reenq_immed; / SCX_TASK_REENQ_IMMED bounces,
    pub /: *mut *mut u64 nr_inject_attempts; / fault-injection: dispatches to an unheld cid,
    pub /: *mut *mut u64 nr_rescue_dsp; / SCX_ENQ_RESCUE dispatch attempts,
    pub /: *mut *mut *mut u32 inject_mode; / fault-injection mode (QMAP_INJ_),
}
