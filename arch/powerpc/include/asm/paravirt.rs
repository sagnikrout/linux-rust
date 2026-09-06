//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/paravirt.h
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


// SPDX-License-Identifier: GPL-2.0-or-later

extern "C" {
    pub fn static_branch_unlikely(_arg: &shared_processor) -> return;
}

extern "C" {
    pub fn pseries_paravirt_steal_clock(cpu: c_int) -> u64;
}
extern "C" {
    pub fn pseries_paravirt_steal_clock(_arg: cpu) -> return;
}

// If bit 0 is set, the cpu has been ceded, conferred, or preempted
extern "C" {
    pub fn be32_to_cpu(_arg: yield_count) -> return;
}
//
// Spinlock code confers and prods, so don't trace the hcalls because the
// tracing code takes spinlocks which can cause recursion deadlocks.
//
// These calls are made while the lock is not held: the lock slowpath yields if
// it can not acquire the lock, and unlock slow path might prod if a waiter has
// yielded). So this may not be a problem for simple spin locks because the
// tracing does not technically recurse on the lock, but we avoid it anyway.
//
// However the queued spin lock contended path is more strictly ordered: the
// H_CONFER hcall is made after the task has queued itself on the lock, so then
// recursing on that lock will cause the task to then queue up again behind the
// first instance (or worse: queued spinlocks use tricks that assume a context
// never waits on more than one spinlock, so such recursion may cause random
// corruption in the lock code).
//
// This is the yield_count.  An "odd" value (low bit on) means that
// the processor is yielded (either because of an OS yield or a
// hypervisor preempt).  An even value implies that the processor is
// currently executing.
//

extern "C" {
    pub fn ___bad_yield_to_preempted();
}
extern "C" {
    pub fn ___bad_yield_to_any();
}
extern "C" {
    pub fn ___bad_prod_cpu();
}

//
// The dispatch/yield bit alone is an imperfect indicator of
// whether the hypervisor has dispatched @cpu to run on a physical
// processor. When it is clear, @cpu is definitely not preempted.
// But when it is set, it means only that it *might* be, subject to
// other conditions. So we check other properties of the VM and
// @cpu first, resorting to the yield count last.
//
// Hypervisor preemption isn't possible in dedicated processor
// mode by definition.
//
// If the hypervisor has dispatched the target CPU on a physical
// processor, then the target CPU is definitely not preempted.
//
// if the target CPU is not dispatched and the guest OS
// has not marked the CPU idle, then it is hypervisor preempted.
//

//
// The result of vcpu_is_preempted() is used in a
// speculative way, and is always subject to invalidation
// by events internal and external to Linux. While we can
// be called in preemptable context (in the Linux sense),
// we're not accessing per-cpu resources in a way that can
// race destructively with Linux scheduler preemption and
// migration, and callers can tolerate the potential for
// error introduced by sampling the CPU index without
// pinning the task to it. So it is permissible to use
// raw_smp_processor_id() here to defeat the preempt debug
// warnings that can arise from using smp_processor_id()
// in arbitrary contexts.
//
// The PowerVM hypervisor dispatches VMs on a whole core
// basis. So we know that a thread sibling of the executing CPU
// cannot have been preempted by the hypervisor, even if it
// has called H_CONFER, which will set the yield bit.
//
// The specific target CPU was marked by guest OS as idle, but
// then also check all other cpus in the core for PowerVM
// because it does core scheduling and one of the vcpu
// of the core getting preempted by hypervisor implies
// other vcpus can also be considered preempted.
//

//
// None of the threads in target CPU's core are running but none of
// them were preempted too. Hence assume the target CPU to be
// non-preempted.
//
