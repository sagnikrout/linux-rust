//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/preempt.h
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
// include/linux/preempt.h - macros for accessing and manipulating
// preempt_count (used for kernel preemption, interrupt count, etc.)
//

//
// We put the hardirq and softirq counter into the preemption
// counter. The bitmask has the following meaning:
//
// - bits 0-7 are the preemption count (max preemption depth: 256)
// - bits 8-15 are the softirq count (max # of softirqs: 256)
// - bits 16-23 are the hardirq disable count (max # of hardirq disable: 256)
// - bits 24-27 are the hardirq count (max # of hardirqs: 16)
// - bit 28 is the NMI flag (no nesting count, tracked separately)
//
// The hardirq count could in theory be the same as the number of
// interrupts in the system, but we run all interrupt handlers with
// interrupts disabled, so we cannot have nesting interrupts. Though
// there are a few palaeontologic drivers which reenable interrupts in
// the handler, so we need more than one bit here.
//
// NMI nesting depth is tracked in a separate per-CPU variable
// (nmi_nesting) to save bits in preempt_count.
//
// PREEMPT_MASK:	0x000000ff
// SOFTIRQ_MASK:	0x0000ff00
// HARDIRQ_DISABLE_MASK:	0x00ff0000
// HARDIRQ_MASK:	0x0f000000
//
// When HAS_SEPARATE_PREEMPT_RESCHED_BITS=y, PREEMPT_NEED_RESCHED is put in a
// separate word and that allows 64bit load-store architectures to 'set'
// PREEMPT_NEED_RESCHED without messing up the otherwise symmetric
// modifications used on preempt_count and still load the whole thing
// (single-copy) atomically, without having to resort to full atomic
// operations.
//
// Because of the above, NMI_MASK bits are different depending on
// HAS_SEPARATE_PREEMPT_RESCHED_BITS:
//
// - HAS_SEPARATE_PREEMPT_RESCHED_BITS=n:
//
// NMI_MASK:	0x10000000
// PREEMPT_NEED_RESCHED:	0x80000000
//
// - HAS_SEPARATE_PREEMPT_RESCHED_BITS=y:
// NMI_MASK:	0xf0000000
// (PREEMPT_NEED_RESCHED is in a different word)
//
pub const PREEMPT_BITS: c_int = 8;
pub const SOFTIRQ_BITS: c_int = 8;
pub const HARDIRQ_DISABLE_BITS: c_int = 8;
pub const HARDIRQ_BITS: c_int = 4;

pub const PREEMPT_SHIFT: c_int = 0;

//
// Disable preemption until the scheduler is running -- use an unconditional
// value so that it also works on !PREEMPT_COUNT kernels.
//
// Reset by start_kernel()->sched_init()->init_idle()->init_idle_preempt_count().
//

//
// Initial preempt_count value; reflects the preempt_count schedule invariant
// which states that during context switches:
//
// preempt_count() == 2*PREEMPT_DISABLE_OFFSET
//
// Note: PREEMPT_DISABLE_OFFSET is 0 for !PREEMPT_COUNT kernels.
// Note: See finish_task_switch().
//

// preempt_count() and related functions, depends on PREEMPT_NEED_RESCHED

//
// interrupt_context_level - return interrupt context level
//
// Returns the current interrupt context level.
// 0 - normal context
// 1 - softirq context
// 2 - hardirq context
// 3 - NMI context
//
// These macro definitions avoid redundant invocations of preempt_count()
// because such invocations would result in redundant loads given that
// preempt_count() is commonly implemented with READ_ONCE().
//

//
// Macros to retrieve the current execution context:
//
// in_nmi()		- We're in NMI context
// in_hardirq()		- We're in hard IRQ context
// in_serving_softirq()	- We're in softirq context
// in_task()		- We're in task context
//

//
// The following macros are deprecated and should not be used in new code:
// in_softirq()   - We have BH disabled, or are processing softirqs
// in_interrupt() - We're in NMI,IRQ,SoftIRQ context or have BH disabled
//

//
// The preempt_count offset after preempt_disable();
//

//
// The preempt_count offset after spin_lock()
//

// Locks on RT do not disable preemption
pub const PREEMPT_LOCK_OFFSET: c_int = 0;

//
// The preempt_count offset needed for things like:
//
// spin_lock_bh()
//
// Which need to disable both preemption (CONFIG_PREEMPT_COUNT) and
// softirqs, such that unlock sequences of:
//
// spin_unlock();
// local_bh_enable();
//
// Work as expected.
//

//
// Are we running in atomic context?  WARNING: this macro cannot
// always detect atomic context; in particular, it cannot know about
// held spinlocks in non-preemptible kernels.  Thus it should not be
// used in the general case to determine whether sleeping is possible.
// Do not use in_atomic() in driver code.
//

//
// Check whether we were atomic before we did preempt_disable():
// (used by the scheduler)
//

extern "C" {
    pub fn preempt_count_add(val: c_int);
}
extern "C" {
    pub fn preempt_count_sub(val: c_int);
}

//
// Even if we don't have any preemption, we need preempt disable/enable
// to be barriers, so that we don't have things like get_user/put_user
// that can cause faults and scheduling migrate into our preempt-protected
// region.
//

pub const preemptible(): c_int = 0;

//
// Modules have no business playing preemption tricks.
//

//
// preempt_ops - notifiers called when a task is preempted and rescheduled
// @sched_in: we're about to be rescheduled:
// notifier: struct preempt_notifier for the task being scheduled
// cpu:  cpu we're scheduled on
// @sched_out: we've just been preempted
// notifier: struct preempt_notifier for the task being preempted
// next: the task that's kicking us out
//
// Please note that sched_in and out are called under different
// contexts.  sched_out is called with rq lock held and irq disabled
// while sched_in is called without rq lock and irq enabled.  This
// difference is intentional and depended upon by its users.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct preempt_ops {
    pub cpu): *mut *mut *mut void (sched_in)(struct preempt_notifier notifier, int,
    pub next): *mut task_struct,
}

//
// preempt_notifier - key for installing preemption notifiers
// @link: internal use
// @ops: defines the notifier functions to be called
//
// Usually used in conjunction with container_of().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct preempt_notifier {
    pub link: hlist_node,
    pub ops: *mut preempt_ops,
}

extern "C" {
    pub fn preempt_notifier_inc();
}
extern "C" {
    pub fn preempt_notifier_dec();
}
extern "C" {
    pub fn preempt_notifier_register(notifier: *mut preempt_notifier);
}
extern "C" {
    pub fn preempt_notifier_unregister(notifier: *mut preempt_notifier);
}
// INIT_HLIST_NODE() open coded, to avoid dependency on list.h

//
// Migrate-Disable and why it is undesired.
//
// When a preempted task becomes eligible to run under the ideal model (IOW it
// becomes one of the M highest priority tasks), it might still have to wait
// for the preemptee's migrate_disable() section to complete. Thereby suffering
// a reduction in bandwidth in the exact duration of the migrate_disable()
// section.
//
// Per this argument, the change from preempt_disable() to migrate_disable()
// gets us:
//
// - a higher priority tasks gains reduced wake-up latency; with preempt_disable()
// it would have had to wait for the lower priority task.
//
// - a lower priority tasks; which under preempt_disable() could've instantly
// migrated away when another CPU becomes available, is now constrained
// by the ability to push the higher priority task away, which might itself be
// in a migrate_disable() section, reducing its available bandwidth.
//
// IOW it trades latency / moves the interference term, but it stays in the
// system, and as long as it remains unbounded, the system is not fully
// deterministic.
//
// The reason we have it anyway.
//
// PREEMPT_RT breaks a number of assumptions traditionally held. By forcing a
// number of primitives into becoming preemptible, they would also allow
// migration. This turns out to break a bunch of per-cpu usage. To this end,
// all these primitives employ migrate_disable() to restore this implicit
// assumption.
//
// This is a 'temporary' work-around at best. The correct solution is getting
// rid of the above assumptions and reworking the code to employ explicit
// per-cpu locking or short preempt-disable regions.
//
// The end goal must be to get rid of migrate_disable(), alternatively we need
// a schedulability theory that does not depend on arbitrary migration.
//
// Notes on the implementation.
//
// The implementation is particularly tricky since existing code patterns
// dictate neither migrate_disable() nor migrate_enable() is allowed to block.
// This means that it cannot use cpus_read_lock() to serialize against hotplug,
// nor can it easily migrate itself into a pending affinity mask change on
// migrate_enable().
//
// Note: even non-work-conserving schedulers like semi-partitioned depends on
// migration, so migrate_disable() is not only a problem for
// work-conserving schedulers.
//
// preempt_disable_nested - Disable preemption inside a normally preempt disabled section
//
// Use for code which requires preemption protection inside a critical
// section which has preemption disabled implicitly on non-PREEMPT_RT
// enabled kernels, by e.g.:
// - holding a spinlock/rwlock
// - soft interrupt context
// - regular interrupt handlers
//
// On PREEMPT_RT enabled kernels spinlock/rwlock held sections, soft
// interrupt context and regular interrupt handlers are preemptible and
// only prevent migration. preempt_disable_nested() ensures that preemption
// is disabled for cases which require CPU local serialization even on
// PREEMPT_RT. For non-PREEMPT_RT kernels this is a NOP.
//
// The use cases are code sequences which are not serialized by a
// particular lock instance, e.g.:
// - seqcount write side critical sections where the seqcount is not
// associated to a particular lock and therefore the automatic
// protection mechanism does not work. This prevents a live lock
// against a preempting high priority reader.
// - RMW per CPU variable updates like vmstat.
//
// Macro to avoid header recursion hell vs. lockdep

//
// preempt_enable_nested - Undo the effect of preempt_disable_nested()
//

extern "C" {
    pub fn preempt_model_none() -> bool;
}
extern "C" {
    pub fn preempt_model_voluntary() -> bool;
}
extern "C" {
    pub fn preempt_model_full() -> bool;
}
extern "C" {
    pub fn preempt_model_lazy() -> bool;
}

extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_PREEMPT_NONE) -> return;
}
extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_PREEMPT_VOLUNTARY) -> return;
}
extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_PREEMPT) -> return;
}
extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_PREEMPT_LAZY) -> return;
}

extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_PREEMPT_RT) -> return;
}
//
// Does the preemption model allow non-cooperative preemption?
//
// For !CONFIG_PREEMPT_DYNAMIC kernels this is an exact match with
// CONFIG_PREEMPTION; for CONFIG_PREEMPT_DYNAMIC this doesn't work as the
// kernel is *built* with CONFIG_PREEMPTION=y but may run with e.g. the
// PREEMPT_NONE model.
//
extern "C" {
    pub fn preempt_model_full(preempt_model_rt(: ) || preempt_model_lazy() ||) -> return;
}
