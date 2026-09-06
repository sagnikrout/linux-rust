//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/irqdesc.h
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
// Core internal functions to deal with irq descriptors
//
// struct irqstat - interrupt statistics
// @cnt:	real-time interrupt count
// @ref:	snapshot of interrupt count
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irqstat {
    pub cnt: c_uint,

    pub ref: c_uint,

}

//
// struct irq_redirect - interrupt redirection metadata
// @work:	Harg irq_work item for handler execution on a different CPU
// @target_cpu:	CPU to run irq handler on in case the current CPU is not part
// of the irq affinity mask
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_redirect {
    pub work: irq_work,
    pub target_cpu: c_uint,
}

//
// struct irq_desc - interrupt descriptor
// @irq_common_data:	per irq and chip data passed down to chip functions
// @kstat_irqs:		irq stats per cpu
// @handle_irq:		highlevel irq-events handler
// @action:		the irq action chain
// @status_use_accessors: status information
// @core_internal_state__do_not_mess_with_it: core internal status information
// @depth:		disable-depth, for nested irq_disable() calls
// @wake_depth:		enable depth, for multiple irq_set_irq_wake() callers
// @tot_count:		stats field for non-percpu irqs
// @last_unhandled:	aging timer for unhandled count
// @irq_count:		stats field to detect stalled irqs
// @irqs_unhandled:	stats field for spurious unhandled interrupts
// @threads_handled:	stats field for deferred spurious detection of threaded handlers
// @threads_handled_last: comparator field for deferred spurious detection of threaded handlers
// @lock:		locking for SMP
// @redirect:		Facility for redirecting interrupts via irq_work
// @affinity_hint:	hint to user space for preferred irq affinity
// @affinity_notify:	context for notification of affinity changes
// @pending_mask:	pending rebalanced interrupts
// @threads_oneshot:	bitfield to handle shared oneshot threads
// @threads_active:	number of irqaction threads currently running
// @wait_for_threads:	wait queue for sync_irq to wait for threaded handlers
// @nr_actions:		number of installed actions on this descriptor
// @no_suspend_depth:	number of irqactions on a irq descriptor with
// IRQF_NO_SUSPEND set
// @force_resume_depth:	number of irqactions on a irq descriptor with
// IRQF_FORCE_RESUME set
// @refcnt:		Reference count mainly for /proc/interrupts
// @rcu:		rcu head for delayed free
// @kobj:		kobject used to represent this struct in sysfs
// @request_mutex:	mutex to protect request/free before locking desc->lock
// @dir:		/proc/irq/ procfs entry
// @debugfs_file:	dentry for the debugfs file
// @name:		flow handler name for /proc/interrupts output
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_desc {
    pub irq_common_data: irq_common_data,
    pub irq_data: irq_data,
    pub kstat_irqs: *mut irqstat __percpu,
    pub handle_irq: irq_flow_handler_t,
    pub /: *mut *mut *mut irqaction action; / IRQ action list,
    pub status_use_accessors: c_uint,
    pub core_internal_state__do_not_mess_with_it: c_uint,
    pub /: *mut *mut unsigned int depth; / nested irq disables,
    pub /: *mut *mut unsigned int wake_depth; / nested wake enables,
    pub tot_count: c_ulong,
    pub /: *mut *mut unsigned long last_unhandled; / Aging timer for unhandled count,
    pub /: *mut *mut unsigned int irq_count; / For detecting broken IRQs,
    pub irqs_unhandled: c_uint,
    pub threads_handled: core::sync::atomic::AtomicI32,
    pub threads_handled_last: c_int,
    pub lock: raw_spinlock_t,
    pub percpu_enabled: *mut cpumask,

    pub redirect: irq_redirect,
    pub affinity_hint: *const cpumask,
    pub affinity_notify: *mut irq_affinity_notify,

    pub pending_mask: cpumask_var_t,

    pub threads_oneshot: c_ulong,
    pub threads_active: core::sync::atomic::AtomicI32,
    pub wait_for_threads: wait_queue_head_t,

    pub nr_actions: c_uint,
    pub no_suspend_depth: c_uint,
    pub cond_suspend_depth: c_uint,
    pub force_resume_depth: c_uint,

    pub dir: *mut proc_dir_entry,

    pub debugfs_file: *mut dentry,
    pub dev_name: *const c_char,

    pub refcnt: rcuref_t,

    pub rcu: rcu_head,
    pub kobj: kobject,

    pub request_mutex: mutex,
    pub parent_irq: c_int,
    pub owner: *mut module,
    pub name: *const c_char,

    pub resend_node: hlist_node,

    pub ____cacheline_internodealigned_in_smp: },

    pub irq_lock_sparse(void): extern void,
    pub irq_unlock_sparse(void): extern void,
    pub irq_desc: [extern struct irq_desc; NR_IRQS],
    pub cpu): return per_cpu(desc->kstat_irqs->cnt,,
    pub irq_common_data): return container_of(data->common, struct irq_desc,,
    pub desc->irq_data.irq: return,
    pub &desc->irq_data: return,
    pub desc->irq_data.chip: return,
    pub desc->irq_data.chip_data: return,
    pub desc->irq_common_data.handler_data: return,
//
// Architectures call this to let the generic IRQ layer
// handle an interrupt.
//
    pub desc): *mut int handle_irq_desc(struct irq_desc,
    pub irq): int generic_handle_irq(unsigned int,
    pub irq): int generic_handle_irq_safe(unsigned int,

//
// Convert a HW interrupt number to a logical one using a IRQ domain,
// and handle the result interrupt number. Return -EINVAL if
// conversion failed.
//
    pub hwirq): *mut *mut int generic_handle_domain_irq(struct irq_domain domain, irq_hw_number_t,
    pub hwirq): *mut *mut int generic_handle_domain_irq_safe(struct irq_domain domain, irq_hw_number_t,
    pub hwirq): *mut *mut int generic_handle_domain_nmi(struct irq_domain domain, irq_hw_number_t,
    pub hwirq): *mut *mut bool generic_handle_demux_domain_irq(struct irq_domain domain, irq_hw_number_t,

// Test to see if a driver has successfully requested an irq
    pub NULL: return desc && desc->action !=,
//
// irq_set_handler_locked - Set irq handler from a locked region
// @data:	Pointer to the irq_data structure which identifies the irq
// @handler:	Flow control handler function for this interrupt
//
// Sets the handler in the irq descriptor associated to @data.
//
// Must be called with irq_desc locked and valid parameters. Typical
// call site is the irq_set_type() callback.
//
    pub irq_data_to_desc(data): *mut *mut irq_desc desc =,
    pub handler: desc->handle_irq =,
//
// irq_set_chip_handler_name_locked - Set chip, handler and name from a locked region
// @data:	Pointer to the irq_data structure for which the chip is set
// @chip:	Pointer to the new irq chip
// @handler:	Flow control handler function for this interrupt
// @name:	Name of the interrupt
//
// Replace the irq chip at the proper hierarchy level in @data and
// sets the handler and name in the associated irq descriptor.
//
// Must be called with irq_desc locked and valid parameters.
//
    pub irq_data_to_desc(data): *mut *mut irq_desc desc =,
    pub handler: desc->handle_irq =,
    pub name: desc->name =,
    pub )chip: *mut data->chip = (struct irq_chip,
    pub bitmask): bool irq_check_status_bit(unsigned int irq, unsigned int,
    pub IRQ_NO_BALANCING_MASK): return irq_check_status_bit(irq,,
    pub IRQ_PER_CPU): return irq_check_status_bit(irq,,
    pub IRQ_PER_CPU_DEVID): return irq_check_status_bit(irq,,
    pub request_class): *mut lock_class_key,
    pub request_class): __irq_set_lockdep_class(irq, lock_class,,
