//! Automatically rewritten from C Header to Rust Module
//! Source: tools/sched_ext/include/scx/compat.h
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

extern "C" {
    pub fn __attribute__(_arg: (weak)) -> *mut btf __COMPAT_vmlinux_btf;
}
//
// Recover the true value of a 64-bit enum enumerator whose kernel BTF entry
// was truncated to its low 32 bits.
//
// Kernels whose BTF was generated without BTF_KIND_ENUM64 support encode
// 64-bit enums as 8-byte BTF_KIND_ENUM entries whose enumerator values only
// carry the low 32 bits. This happens with pahole < 1.24, which predates
// ENUM64, and with pahole passing --skip_encoding_btf_enum64 (e.g. Google's
// Container-Optimized OS / GKE kernels deliberately pass it for backward
// compatibility with older BTF consumers). The high bits
// can't be recovered from kernel BTF, so substitute the value from the
// vmlinux.h this tree was built against, cross-checked against the low 32
// bits the kernel did provide.
//
// Note that this is a best-effort recovery, not a ground truth. The
// substitution assumes the running kernel agrees with this tree's vmlinux.h
// on the high 32 bits, but only the low 32 bits can actually be verified.
// The cross-check is vacuous for enumerators whose value has no low bits
// set (e.g. SCX_DSQ_FLAG_BUILTIN, __SCX_ENQ_INTERNAL_MASK,
// SCX_ENQ_CLEAR_OPSS, SCX_ECODE_*): their lo32 is 0 and matches anything,
// so those substitutions rest entirely on the high bits never moving. An
// enumerator missing from the table (a kernel newer than this tree's
// vmlinux.h, or a stale autogen table) can't be recovered at all. If a
// substitution is ever wrong, the scheduler operates on bogus values (e.g.
// dispatching to nonexistent DSQ ids or silently dropping flags) and can
// wildly malfunction, which is why the mismatch and table-miss paths refuse
// instead of guessing.
//
// v = lo32;
// v = e->val;
//
// Unknown enumerator (likely a stale autogen table). Fail
// pessimistically to avoid returning an invalid value.
//
// Try to recover a 64-bit enum from an 8-byte
// BTF_KIND_ENUM that was encoded without ENUM64
// support (old pahole or
// --skip_encoding_btf_enum64). Only scx_
// types are covered by the substitution table;
// non-scx types fall through to the raw value
// so this generic utility keeps working for
// them.
//
// v = e[i].val;
// v = btf_enum64_value(&e[i]);

//
// Open the sched_ext_ops skeleton.
//
// struct sched_ext_ops can change over time. Two complementary mechanisms
// keep BPF schedulers built against newer headers running on older kernels:
//
// 1. Load-time fix-up (SCX_OPS_OPEN()). For each optional ops callback or field
// added to struct sched_ext_ops, an explicit stanza below probes the
// running kernel's BTF via __COMPAT_struct_has_field() and, if the field
// is missing, clears it in the in-memory struct_ops (with a warning to
// stderr) before load. Handles additive changes - a new stanza must be
// added here for each new optional field.
//
// 2. Multi-variant struct_ops via compat.bpf.h::SCX_OPS_DEFINE(). That
// macro can be expanded to emit several variants of struct sched_ext_ops,
// and SCX_OPS_LOAD()/ATTACH() can pick the right one based on what the
// kernel supports. Needed when an existing operation has to change
// incompatibly (e.g. a callback signature changes); the load-time
// fix-up above only handles purely additive changes.
//
// ec7e3b0463e1 ("implement-ops") in https://github.com/sched-ext/sched_ext is
// the current minimum required kernel version.
//
// COMPAT:
// - v6.17: ops.cgroup_set_bandwidth()
// - v6.19: ops.cgroup_set_idle()
// - v7.1:  ops.sub_attach(), ops.sub_detach(), ops.sub_cgroup_id
// - v7.3:  ops.rescue_bandwidth_ppt, ops.rescue_quantum_us
//

//
// Open a cid-form (struct sched_ext_ops_cid) skeleton. The cid form postdates
// every op the load-time fix-ups above handle, so none of them apply.
//

//
// Associate non-struct_ops BPF programs with the scheduler's struct_ops map so
// that scx_prog_sched() can determine which scheduler a BPF program belongs
// to. Requires libbpf >= 1.7.
//

// See SCX_OPS_OPEN() above for backward-compatibility handling.

//
// New versions of bpftool now emit additional link placeholders for BPF maps,
// and set up BPF skeleton in such a way that libbpf will auto-attach BPF maps
// automatically, assuming libbpf is recent enough (v1.5+). Old libbpf will do
// nothing with those links and won't attempt to auto-attach maps.
//
// To maintain compatibility with older libbpf while avoiding trying to attach
// twice, disable the autoattach feature on newer libbpf.
//

