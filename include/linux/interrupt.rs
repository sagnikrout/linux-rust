//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/interrupt.h
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
// interrupt.h

//
// These correspond to the IORESOURCE_IRQ_* defines in
// linux/ioport.h to select the interrupt line behaviour.  When
// requesting an interrupt without specifying a IRQF_TRIGGER, the
// setting should be assumed to be "as already configured", which
// may be as per machine or firmware initialisation.
//
pub const IRQF_TRIGGER_NONE: c_uint = 0x00000000;
pub const IRQF_TRIGGER_RISING: c_uint = 0x00000001;
pub const IRQF_TRIGGER_FALLING: c_uint = 0x00000002;
pub const IRQF_TRIGGER_HIGH: c_uint = 0x00000004;
pub const IRQF_TRIGGER_LOW: c_uint = 0x00000008;

pub const IRQF_TRIGGER_PROBE: c_uint = 0x00000010;
//
// These flags used only by the kernel as part of the
// irq handling routines.
//
// IRQF_SHARED - allow sharing the irq among several devices
// IRQF_PROBE_SHARED - set by callers when they expect sharing mismatches to occur
// IRQF_TIMER - Flag to mark this interrupt as timer interrupt
// IRQF_PERCPU - Interrupt is per cpu
// IRQF_NOBALANCING - Flag to exclude this interrupt from irq balancing
// IRQF_IRQPOLL - Interrupt is used for polling (only the interrupt that is
// registered first in a shared interrupt is considered for
// performance reasons)
// IRQF_ONESHOT - Interrupt is not reenabled after the hardirq handler finished.
// Used by threaded interrupts which need to keep the
// irq line disabled until the threaded handler has been run.
// IRQF_NO_SUSPEND - Do not disable this IRQ during suspend.  Does not guarantee
// that this interrupt will wake the system from a suspended
// state.  See Documentation/power/suspend-and-interrupts.rst
// IRQF_FORCE_RESUME - Force enable it on resume even if IRQF_NO_SUSPEND is set
// IRQF_NO_THREAD - Interrupt cannot be threaded
// IRQF_EARLY_RESUME - Resume IRQ early during syscore instead of at device
// resume time.
// IRQF_COND_SUSPEND - If the IRQ is shared with a NO_SUSPEND user, execute this
// interrupt handler after suspending interrupts. For system
// wakeup devices users need to implement wakeup detection in
// their interrupt handlers.
// IRQF_NO_AUTOEN - Don't enable IRQ or NMI automatically when users request it.
// Users will enable it explicitly by enable_irq() or enable_nmi()
// later.
// IRQF_NO_DEBUG - Exclude from runnaway detection for IPI and similar handlers,
// depends on IRQF_PERCPU.
// IRQF_COND_ONESHOT - Agree to do IRQF_ONESHOT if already set for a shared
// interrupt.
//
pub const IRQF_SHARED: c_uint = 0x00000080;
pub const IRQF_PROBE_SHARED: c_uint = 0x00000100;
pub const __IRQF_TIMER: c_uint = 0x00000200;
pub const IRQF_PERCPU: c_uint = 0x00000400;
pub const IRQF_NOBALANCING: c_uint = 0x00000800;
pub const IRQF_IRQPOLL: c_uint = 0x00001000;
pub const IRQF_ONESHOT: c_uint = 0x00002000;
pub const IRQF_NO_SUSPEND: c_uint = 0x00004000;
pub const IRQF_FORCE_RESUME: c_uint = 0x00008000;
pub const IRQF_NO_THREAD: c_uint = 0x00010000;
pub const IRQF_EARLY_RESUME: c_uint = 0x00020000;
pub const IRQF_COND_SUSPEND: c_uint = 0x00040000;
pub const IRQF_NO_AUTOEN: c_uint = 0x00080000;
pub const IRQF_NO_DEBUG: c_uint = 0x00100000;
pub const IRQF_COND_ONESHOT: c_uint = 0x00200000;

//
// These values can be returned by request_any_context_irq() and
// describe the context the interrupt will be run in.
//
// IRQC_IS_HARDIRQ - interrupt runs in hardirq context
// IRQC_IS_NESTED - interrupt runs in a nested threaded context
//
extern "C" {
    pub fn irqreturn_t(_arg: *mut irq_handler_t)(int, : *mut c_void) -> typedef;
}
//
// struct irqaction - per interrupt action descriptor
// @handler:	interrupt handler function
// @name:	name of the device
// @dev_id:	cookie to identify the device
// @percpu_dev_id:	cookie to identify the device
// @affinity:	CPUs this irqaction is allowed to run on
// @next:	pointer to the next irqaction for shared interrupts
// @irq:	interrupt number
// @flags:	flags (see IRQF_* above)
// @thread_fn:	interrupt handler function for threaded interrupts
// @thread:	thread pointer for threaded interrupts
// @secondary:	pointer to secondary irqaction (force threading)
// @thread_flags:	flags related to @thread
// @thread_mask:	bitmask for keeping track of @thread activity
// @dir:	pointer to the proc/irq/NN/name entry
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irqaction {
    pub handler: irq_handler_t,
    pub dev_id: *mut c_void,
    pub percpu_dev_id: *mut void __percpu,
}

extern "C" {
    pub fn no_action(cpl: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
//
// If a (PCI) device interrupt is not connected we set dev->irq to
// IRQ_NOTCONNECTED. This causes request_irq() to fail with -ENOTCONN, so we
// can distinguish that case from other error returns.
//
// 0x80000000 is guaranteed to be outside the available range of interrupts
// and easy to distinguish from other possible incorrect values.
//

//
// request_irq - Add a handler for an interrupt line
// @irq:	The interrupt line to allocate
// @handler:	Function to be called when the IRQ occurs.
// Primary handler for threaded interrupts
// If NULL, the default primary handler is installed
// @flags:	Handling flags
// @name:	Name of the device generating this interrupt
// @dev:	A cookie passed to the handler function
//
// This call allocates an interrupt and establishes a handler; see
// the documentation for request_threaded_irq() for details.
//
extern "C" {
    pub fn request_threaded_irq(_arg: irq, _arg: handler, _arg: NULL, IRQF_COND_ONESHOT: flags |, _arg: name, _arg: dev) -> return;
}
extern "C" {
    pub fn free_percpu_irq(int: unsigned, : *mut void __percpu);
}
extern "C" {
    pub fn free_percpu_nmi(irq: c_uint, percpu_dev_id: *mut void __percpu);
}
extern "C" {
    pub fn devm_free_irq(dev: *mut device, irq: c_uint, dev_id: *mut c_void);
}
extern "C" {
    pub fn irq_has_action(irq: c_uint) -> bool;
}
extern "C" {
    pub fn disable_irq_nosync(irq: c_uint);
}
extern "C" {
    pub fn disable_hardirq(irq: c_uint) -> bool;
}
extern "C" {
    pub fn disable_irq(irq: c_uint);
}
extern "C" {
    pub fn disable_percpu_irq(irq: c_uint);
}
extern "C" {
    pub fn enable_irq(irq: c_uint);
}
extern "C" {
    pub fn enable_percpu_irq(irq: c_uint, type: c_uint);
}
extern "C" {
    pub fn irq_percpu_is_enabled(irq: c_uint) -> bool;
}
extern "C" {
    pub fn irq_wake_thread(irq: c_uint, dev_id: *mut c_void);
}
extern "C" {
    pub fn disable_nmi_nosync(irq: c_uint);
}
extern "C" {
    pub fn disable_percpu_nmi(irq: c_uint);
}
extern "C" {
    pub fn enable_nmi(irq: c_uint);
}
extern "C" {
    pub fn enable_percpu_nmi(irq: c_uint, type: c_uint);
}
extern "C" {
    pub fn prepare_percpu_nmi(irq: c_uint) -> c_int;
}
extern "C" {
    pub fn teardown_percpu_nmi(irq: c_uint);
}
extern "C" {
    pub fn irq_inject_interrupt(irq: c_uint) -> c_int;
}
// The following three functions are for the core kernel use only.
extern "C" {
    pub fn suspend_device_irqs();
}
extern "C" {
    pub fn resume_device_irqs();
}
extern "C" {
    pub fn rearm_wake_irq(irq: c_uint);
}
//
// struct irq_affinity_notify - context for notification of IRQ affinity changes
// @irq:		Interrupt to which notification applies
// @kref:		Reference count, for internal use
// @work:		Work item, for internal use
// @notify:		Function to be called on change.  This will be
// called in process context.
// @release:		Function to be called on release.  This will be
// called in process context.  Once registered, the
// structure must only be freed when this function is
// called or later.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_affinity_notify {
    pub irq: c_uint,
    pub kref: kref,
    pub work: work_struct,
    pub mask): *const *const *const void (notify)(struct irq_affinity_notify , cpumask_t,
    pub ref): *mut *mut void (release)(struct kref,
}

pub const IRQ_AFFINITY_MAX_SETS: c_int = 4;
//
// struct irq_affinity - Description for automatic irq affinity assignments
// @pre_vectors:	Don't apply affinity to @pre_vectors at beginning of
// the MSI(-X) vector space
// @post_vectors:	Don't apply affinity to @post_vectors at end of
// the MSI(-X) vector space
// @nr_sets:		The number of interrupt sets for which affinity
// spreading is required
// @set_size:		Array holding the size of each interrupt set
// @calc_sets:		Callback for calculating the number and size
// of interrupt sets
// @priv:		Private data for usage by @calc_sets, usually a
// pointer to driver/device specific data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_affinity {
    pub pre_vectors: c_uint,
    pub post_vectors: c_uint,
    pub nr_sets: c_uint,
    pub set_size: [c_uint; IRQ_AFFINITY_MAX_SETS],
    pub nvecs): *mut *mut *mut void (calc_sets)(struct irq_affinity , unsigned int,
    pub priv: *mut c_void,
}

//
// struct irq_affinity_desc - Interrupt affinity descriptor
// @mask:	cpumask to hold the affinity assignment
// @is_managed: 1 if the interrupt is managed internally
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_affinity_desc {
    pub mask: cpumask,
    pub 1: unsigned int is_managed :,
}

extern "C" {
    pub fn irq_set_affinity(irq: c_uint, cpumask: *const cpumask) -> c_int;
}
extern "C" {
    pub fn irq_force_affinity(irq: c_uint, cpumask: *const cpumask) -> c_int;
}
extern "C" {
    pub fn irq_can_set_affinity(irq: c_uint) -> c_int;
}
extern "C" {
    pub fn irq_select_affinity(irq: c_uint) -> c_int;
}
//
// irq_update_affinity_hint - Update the affinity hint
// @irq:	Interrupt to update
// @m:		cpumask pointer (NULL to clear the hint)
//
// Updates the affinity hint, but does not change the affinity of the interrupt.
//
extern "C" {
    pub fn __irq_apply_affinity_hint(_arg: irq, _arg: m, _arg: false) -> return;
}
//
// irq_set_affinity_and_hint - Update the affinity hint and apply the provided
// cpumask to the interrupt
// @irq:	Interrupt to update
// @m:		cpumask pointer (NULL to clear the hint)
//
// Updates the affinity hint and if @m is not NULL it applies it as the
// affinity of that interrupt.
//
extern "C" {
    pub fn __irq_apply_affinity_hint(_arg: irq, _arg: m, _arg: true) -> return;
}
//
// Deprecated. Use irq_update_affinity_hint() or irq_set_affinity_and_hint()
// instead.
//
extern "C" {
    pub fn irq_set_affinity_and_hint(_arg: irq, _arg: m) -> return;
}

//
// Special lockdep variants of irq disabling/enabling.
// These should be used for locking constructs that
// know that a particular irq context which is disabled,
// and which is the only irq-context user of a lock,
// that it's safe to take the lock in the irq-disabled
// section without disabling hardirqs.
//
// On !CONFIG_LOCKDEP they are equivalent to the normal
// irq disable/enable methods.
//

// IRQ wakeup (PM) control:
extern "C" {
    pub fn irq_set_irq_wake(irq: c_uint, on: c_uint) -> c_int;
}
extern "C" {
    pub fn irq_set_irq_wake(_arg: irq, _arg: 1) -> return;
}
extern "C" {
    pub fn irq_set_irq_wake(_arg: irq, _arg: 0) -> return;
}
//
// irq_get_irqchip_state/irq_set_irqchip_state specific flags
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum irqchip_irq_state {
    IRQCHIP_STATE_PENDING,		/* Is interrupt pending? */
    IRQCHIP_STATE_ACTIVE,		/* Is interrupt in progress? */
    IRQCHIP_STATE_MASKED,		/* Is interrupt masked? */
    IRQCHIP_STATE_LINE_LEVEL,	/* Is IRQ line high? */
}

// Some architectures might implement lazy enabling/disabling of
// interrupts. In some cases, such as stop_machine, we might want
// to ensure that after a local_irq_disable(), interrupts have
// really been disabled in hardware. Such architectures need to
// implement the following hook.
//

// PLEASE, avoid to allocate new softirqs, if you need not _really_ high
//
// The following vectors can be safely ignored after ksoftirqd is parked:
//
// _ RCU:
// 1) rcutree_migrate_callbacks() migrates the queue.
// 2) rcutree_report_cpu_dead() reports the final quiescent states.
//
// _ IRQ_POLL: irq_poll_cpu_dead() migrates the queue
//
// _ (HR)TIMER_SOFTIRQ: (hr)timers_dead_cpu() migrates the queue
//

// map softirq index to softirq name. update 'softirq_to_name' in
// kernel/softirq.c when adding a new softirq.
//
// softirq mask and active fields moved to irq_cpustat_t in
// asm/hardirq.h to get better cache usage.  KAO
//
extern "C" {
    pub fn do_softirq() -> asmlinkage void;
}
extern "C" {
    pub fn __do_softirq() -> asmlinkage void;
}

extern "C" {
    pub fn do_softirq_post_smp_call_flush(was_pending: c_uint);
}

extern "C" {
    pub fn open_softirq(nr: c_int, (*action)(void): *mut c_void);
}
extern "C" {
    pub fn softirq_init();
}
extern "C" {
    pub fn __raise_softirq_irqoff(nr: c_uint);
}
extern "C" {
    pub fn raise_softirq_irqoff(nr: c_uint);
}
extern "C" {
    pub fn raise_softirq(nr: c_uint);
}
//
// With forced-threaded interrupts enabled a raised softirq is deferred to
// ksoftirqd unless it can be handled within the threaded interrupt. This
// affects timer_list timers and hrtimers which are explicitly marked with
// HRTIMER_MODE_SOFT.
// With PREEMPT_RT enabled more hrtimers are moved to softirq for processing
// which includes all timers which are not explicitly marked HRTIMER_MODE_HARD.
// Userspace controlled timers (like the clock_nanosleep() interface) is divided
// into two categories: Tasks with elevated scheduling policy including
// SCHED_{FIFO|RR|DL} and the remaining scheduling policy. The tasks with the
// elevated scheduling policy are woken up directly from the HARDIRQ while all
// other wake ups are delayed to softirq and so to ksoftirqd.
//
// The ksoftirqd runs at SCHED_OTHER policy at which it should remain since it
// handles the softirq in an overloaded situation (not handled everything
// within its last run).
// If the timers are handled at SCHED_OTHER priority then they competes with all
// other SCHED_OTHER tasks for CPU resources are possibly delayed.
// Moving timers softirqs to a low priority SCHED_FIFO thread instead ensures
// that timer are performed before scheduling any SCHED_OTHER thread.
//
extern "C" {
    pub fn raise_ktimers_thread(nr: c_uint);
}
extern "C" {
    pub fn __this_cpu_read(_arg: pending_timer_softirq) -> return;
}
extern "C" {
    pub fn local_timers_pending_force_th() -> return;
}
extern "C" {
    pub fn local_softirq_pending() -> return;
}
extern "C" {
    pub fn this_cpu_read(_arg: ksoftirqd) -> return;
}
// Tasklets --- multithreaded analogue of BHs.
// If tasklet_schedule() is called, then tasklet is guaranteed
// If the tasklet is already scheduled, but its execution is still not
// If this tasklet is already running on another CPU (or schedule is called
// Tasklet is strictly serialized wrt itself, but not
//

extern "C" {
    pub fn tasklet_unlock(t: *mut tasklet_struct);
}
extern "C" {
    pub fn tasklet_unlock_wait(t: *mut tasklet_struct);
}
extern "C" {
    pub fn tasklet_unlock_spin_wait(t: *mut tasklet_struct);
}

extern "C" {
    pub fn __tasklet_schedule(t: *mut tasklet_struct);
}
extern "C" {
    pub fn __tasklet_hi_schedule(t: *mut tasklet_struct);
}
//
// Do not use in new code. Disabling tasklets from atomic contexts is
// error prone and should be avoided.
//
extern "C" {
    pub fn tasklet_kill(t: *mut tasklet_struct);
}
//
// Autoprobing for irqs:
//
// probe_irq_on() and probe_irq_off() provide robust primitives
// for accurate IRQ probing during kernel initialization.  They are
// reasonably simple to use, are not "fooled" by spurious interrupts,
// and, unlike other attempts at IRQ probing, they do not get hung on
// stuck interrupts (such as unused PS2 mouse interfaces on ASUS boards).
//
// For reasonably foolproof probing, use them as follows:
//
// 1. clear and/or mask the device's internal interrupt.
// 2. sti();
// 3. irqs = probe_irq_on();      // "take over" all unassigned idle IRQs
// 4. enable the device and cause it to trigger an interrupt.
// 5. wait for the device to interrupt, using non-intrusive polling or a delay.
// 6. irq = probe_irq_off(irqs);  // get IRQ number, 0=none, negative=multiple
// 7. service the device to clear its pending interrupt.
// 8. loop again if paranoia is required.
//
// probe_irq_on() returns a mask of allocated irq's.
//
// probe_irq_off() takes the mask as a parameter,
// and returns the irq number which occurred,
// or zero if none occurred, or a negative irq number
// if more than one irq occurred.
//

// Initialize /proc/irq/
extern "C" {
    pub fn init_irq_proc();
}

extern "C" {
    pub fn show_interrupts(p: *mut seq_file, v: *mut c_void) -> c_int;
}
extern "C" {
    pub fn arch_show_interrupts(p: *mut seq_file, prec: c_int) -> c_int;
}
extern "C" {
    pub fn irq_proc_emit_counts(p: *mut seq_file, cnts: *mut unsigned int __percpu);
}
extern "C" {
    pub fn early_irq_init() -> c_int;
}
extern "C" {
    pub fn arch_probe_nr_irqs() -> c_int;
}
extern "C" {
    pub fn arch_early_irq_init() -> c_int;
}
//
// We want to know which function is an entrypoint of a hardirq or a softirq.
//

