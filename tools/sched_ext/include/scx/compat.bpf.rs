//! Automatically rewritten from C Header to Rust Module
//! Source: tools/sched_ext/include/scx/compat.bpf.h
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
// Copyright (c) 2024 Meta Platforms, Inc. and affiliates.
// Copyright (c) 2024 Tejun Heo <tj@kernel.org>
// Copyright (c) 2024 David Vernet <dvernet@meta.com>
//

// v6.12: 819513666966 ("sched_ext: Add cgroup support")

//
// v6.13: The verb `dispatch` was too overloaded and confusing. kfuncs are
// renamed to unload the verb.
//
// scx_bpf_dispatch_from_dsq() and friends were added during v6.12 by
// 4c30f5ce4f7a ("sched_ext: Implement scx_bpf_dispatch[_vtime]_from_dsq()").
//
// v7.1: scx_bpf_dsq_move_to_local___v2() to add @enq_flags.
//

//
// v6.15: 950ad93df2fc ("bpf: add kfunc for populating cpumask bits")
//
// Compat macro will be dropped on v6.19 release.
//

//
// v6.19: Introduce lockless peek API for user DSQs.
// v7.1:  Fix scx_bpf_dsq_peek() spuriously returning NULL on non-empty
// FIFO DSQs (2f2ea7709266).
//
// The kfunc exists from v6.19 but can return NULL for a non-empty FIFO DSQ
// before the v7.1 fix. Require kernel version >= 7.1.0 before calling it;
// otherwise fall through to the bpf_iter_scx_dsq fallback below.
//
extern "C" {
    pub fn scx_bpf_dsq_peek(_arg: dsq_id) -> return;
}
//
// v7.1: scx_bpf_sub_dispatch() for sub-sched dispatch. Preserve until
// we drop the compat layer for older kernels that lack the kfunc.
//
extern "C" {
    pub fn scx_bpf_sub_dispatch___compat(_arg: cgroup_id) -> return;
}
//
// v7.3: scx_bpf_cid_override() for explicit cid and shard mapping. Ignore if
// missing.
//
// __COMPAT_is_enq_cpu_selected - Test if SCX_ENQ_CPU_SELECTED is on
// in a compatible way. We will preserve this __COMPAT helper until v6.16.
//
// @enq_flags: enqueue flags from ops.enqueue()
//
// Return: True if SCX_ENQ_CPU_SELECTED is turned on in @enq_flags
//

//
// This is the case that a BPF code compiled against vmlinux.h
// where the enum SCX_ENQ_CPU_SELECTED exists.
//
// We should temporarily suspend the macro expansion of
// 'SCX_ENQ_CPU_SELECTED'. This avoids 'SCX_ENQ_CPU_SELECTED' being
// rewritten to '__SCX_ENQ_CPU_SELECTED' when 'SCX_ENQ_CPU_SELECTED'
// is defined in 'scripts/gen_enums.py'.
//

//
// When the kernel did not have SCX_ENQ_CPU_SELECTED,
// select_task_rq_scx() has never been skipped. Thus, this case
// should be considered that the CPU has already been selected.
//
// Once done, resume the macro expansion of 'SCX_ENQ_CPU_SELECTED'.
//

//
// This is the case that a BPF code compiled against vmlinux.h
// where the enum SCX_ENQ_CPU_SELECTED does NOT exist.
//

//
// v6.15: Introduce event counters.
//
// Preserve the following macro until v6.17.
//

//
// v6.15: Introduce NUMA-aware kfuncs to operate with per-node idle
// cpumasks.
//
// Preserve the following __COMPAT_scx_*_node macros until v6.17.
//

//
// v6.18: Add a helper to retrieve the current task running on a CPU.
//
// The kernel tree dropped this helper and scx_bpf_cpu_rq(), but schedulers in
// this tree still support pre-v6.18 kernels where scx_bpf_cpu_curr() doesn't
// resolve and the scx_bpf_cpu_rq() fallback still exists. Keep it until
// pre-v6.18 kernels fall out of the support window.
//
extern "C" {
    pub fn scx_bpf_cpu_curr(_arg: cpu) -> return;
}
//
// v6.19: To work around BPF maximum parameter limit, the following kfuncs are
// replaced with variants that pack scalar arguments in a struct. Wrappers are
// provided to maintain source compatibility.
//
// v6.13: scx_bpf_dsq_insert_vtime() renaming is also handled here. See the
// block on dispatch renaming above for more details.
//
// The kernel will carry the compat variants until v6.23 to maintain binary
// compatibility. After v6.23 release, remove the compat handling and move the
// wrappers to common.bpf.h.
//
// scx_bpf_select_cpu_and - Pick an idle CPU usable by task @p
// @p: task_struct to select a CPU for
// @prev_cpu: CPU @p was on previously
// @wake_flags: %SCX_WAKE_* flags
// @cpus_allowed: cpumask of allowed CPUs
// @flags: %SCX_PICK_IDLE* flags
//
// Inline wrapper that packs scalar arguments into a struct and calls
// __scx_bpf_select_cpu_and(). See __scx_bpf_select_cpu_and() for details.
//
extern "C" {
    pub fn __scx_bpf_select_cpu_and(_arg: p, _arg: cpus_allowed, _arg: &args) -> return;
}
//
// scx_bpf_select_cpu_and() is now an inline wrapper. Use this instead of
// bpf_ksym_exists(scx_bpf_select_cpu_and) to test availability.
//

//
// scx_bpf_dsq_insert_vtime - Insert a task into the vtime priority queue of a DSQ
// @p: task_struct to insert
// @dsq_id: DSQ to insert into
// @slice: duration @p can run for in nsecs, 0 to keep the current value
// @vtime: @p's ordering inside the vtime-sorted queue of the target DSQ
// @enq_flags: SCX_ENQ_
//
// Inline wrapper that packs scalar arguments into a struct and calls
// __scx_bpf_dsq_insert_vtime(). See __scx_bpf_dsq_insert_vtime() for details.
//
extern "C" {
    pub fn __scx_bpf_dsq_insert_vtime(_arg: p, _arg: &args) -> return;
}
//
// v6.19: scx_bpf_dsq_insert() now returns bool instead of void. Move
// scx_bpf_dsq_insert() decl to common.bpf.h and drop compat helper after v6.22.
// The extra ___compat suffix is to work around libbpf not ignoring __SUFFIX on
// kernel side. The entire suffix can be dropped later.
//
// v6.13: scx_bpf_dsq_insert() renaming is also handled here. See the block on
// dispatch renaming above for more details.
//
extern "C" {
    pub fn scx_bpf_dsq_insert___v2___compat(_arg: p, _arg: dsq_id, _arg: slice, _arg: enq_flags) -> return;
}
//
// v6.19: scx_bpf_task_set_slice() and scx_bpf_task_set_dsq_vtime() added to for
// sub-sched authority checks. Drop the wrappers and move the decls to
// common.bpf.h after v6.22.
//
// v7.1: New scx_bpf_dsq_reenq() that allows re-enqueues on more DSQs. This
// will eventually deprecate scx_bpf_reenqueue_local().
//
extern "C" {
    pub fn bpf_ksym_exists(_arg: scx_bpf_dsq_reenq___compat) -> return;
}
//
// v6.19: The new void variant can be called from anywhere while the older v1
// variant can only be called from ops.cpu_release(). The double ___ prefixes on
// the v2 variant need to be removed once libbpf is updated to ignore ___ prefix
// on kernel side. Drop the wrapper and move the decl to common.bpf.h after
// v6.22.
//
extern "C" {
    pub fn bpf_ksym_exists(_arg: scx_bpf_reenqueue_local___v2___compat) -> return;
}
//
// The generic reenq kfunc and the v2 reenqueue-local variant can both be
// called from anywhere; v1 cannot. Test each ksym in its own branch with a
// distinct call: combining them with || would fold into a bitwise OR of the
// two ksym addresses, which the verifier rejects.
//
// Define sched_ext_ops. See compat.h::SCX_OPS_OPEN() for how backward
// compatibility is handled (this macro can be expanded to emit multiple
// variants for incompatible op changes; SCX_OPS_OPEN() handles purely
// additive changes at load time).
//

//
// Define a cid-form sched_ext_ops. Programs targeting this struct_ops type
// use cid-form callback signatures (select_cid, set_cmask, cid_online/offline,
// dispatch with cid arg, etc.) and may only call the cid-form scx_bpf_
// kfuncs (kick_cid, task_cid, this_cid, ...).
//

