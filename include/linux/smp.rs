//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/smp.h
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
// Generic SMP support
// Alan Cox. <alan@redhat.com>
//

extern "C" {
    pub fn void(info: *mut *mut smp_call_func_t)(void) -> typedef;
}
extern "C" {
    pub fn bool(cpu: *mut *mut smp_cond_func_t)(int, info: *mut c_void) -> typedef;
}
//
// structure shares (partial) layout with struct irq_work
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __call_single_data {
    pub node: __call_single_node,
    pub func: smp_call_func_t,
    pub info: *mut c_void,
}

// Use __aligned() to avoid to use 2 cache lines for 1 csd

// (_csd) = CSD_INIT((_func), (_info));	\
//
// Enqueue a llist_node on the call_single_queue; be very careful, read
// flush_smp_call_function_queue() in detail.
//
extern "C" {
    pub fn __smp_call_single_queue(cpu: c_int, node: *mut llist_node);
}
// total number of cpus in this system (may exceed NR_CPUS)
extern "C" {
    pub fn smp_call_function_single(cpuid: c_int, func: smp_call_func_t, info: *mut c_void, wait: bool) -> c_int;
}
extern "C" {
    pub fn smp_call_function_single_async(cpu: c_int, csd: *mut call_single_data_t) -> c_int;
}
//
// Cpus stopping functions in panic. All have default weak definitions.
// Architecture-dependent code may override them.
//
extern "C" {
    pub fn panic_smp_self_stop() -> void __noreturn;
}
extern "C" {
    pub fn nmi_panic_self_stop(regs: *mut pt_regs) -> void __noreturn;
}
extern "C" {
    pub fn crash_smp_send_stop();
}
extern "C" {
    pub fn panic_smp_redirect_cpu(target_cpu: c_int, msg: *mut c_void) -> c_int;
}
//
// Call a function on all processors
//
// on_each_cpu_mask() - Run a function on processors specified by
// cpumask, which may include the local processor.
// @mask: The set of cpus to run on (only runs on online subset).
// @func: The function to run. This must be fast and non-blocking.
// @info: An arbitrary pointer to pass to the function.
// @wait: If true, wait (atomically) until function has completed
// on other CPUs.
//
// If @wait is true, then returns once @func has returned.
//
// You must not call this function with disabled interrupts or from a
// hardware interrupt handler or from a bottom half handler.  The
// exception is that it may be used during early boot while
// early_boot_irqs_disabled is set.
//
// Call a function on each processor for which the supplied function
// cond_func returns a positive value. This may include the local
// processor.  May be used during early boot while early_boot_irqs_disabled is
// set. Use local_irq_save/restore() instead of local_irq_disable/enable().
//
// Architecture specific boot CPU setup.  Defined as empty weak function in
// init/main.c. Architectures can override it.
//
extern "C" {
    pub fn smp_prepare_boot_cpu() -> void __init;
}

//
// main cross-CPU interfaces, handles INIT, TLB flush, STOP, etc.
// (defined in asm header):
//
// stops all CPUs but the current one:
//
extern "C" {
    pub fn smp_send_stop();
}
//
// sends a 'reschedule' event to another CPU:
//
extern "C" {
    pub fn arch_smp_send_reschedule(cpu: c_int);
}
//
// scheduler_ipi() is inline so can't be passed as callback reason, but the
// callsite IP should be sufficient for root-causing IPIs sent from here.
//

//
// Prepare machine for booting other CPUs.
//
extern "C" {
    pub fn smp_prepare_cpus(max_cpus: c_uint);
}
//
// Bring a CPU up
//
extern "C" {
    pub fn __cpu_up(cpunum: c_uint, tidle: *mut task_struct) -> c_int;
}
//
// Final polishing of CPUs
//
extern "C" {
    pub fn smp_cpus_done(max_cpus: c_uint);
}
//
// Call a function on all other processors
//
extern "C" {
    pub fn smp_call_function(func: smp_call_func_t, info: *mut c_void, wait: c_int);
}
extern "C" {
    pub fn kick_all_cpus_sync();
}
extern "C" {
    pub fn wake_up_all_idle_cpus();
}
extern "C" {
    pub fn cpus_peek_for_pending_ipi(mask: *const cpumask) -> bool;
}
//
// Generic and arch helpers
//
extern "C" {
    pub fn call_function_init() -> void __init;
}
extern "C" {
    pub fn generic_smp_call_function_single_interrupt();
}

extern "C" {
    pub fn setup_nr_cpu_ids() -> void __init;
}
extern "C" {
    pub fn smp_init() -> void __init;
}

//
// These macros fold the SMP functionality into a single CPU system
//
pub const raw_smp_processor_id(): c_int = 0;

extern "C" {
    pub fn smp_call_function_single(_arg: 0, _arg: func, _arg: info, _arg: wait) -> return;
}
pub const setup_max_cpus: c_int = 0;

extern "C" {
    pub fn up_late_init() -> void __init;
}

extern "C" {
    pub fn smp_task_ipi_mask_alloc(task: *mut task_struct) -> c_int;
}
extern "C" {
    pub fn smp_task_ipi_mask_free(task: *mut task_struct);
}

//
// raw_smp_processor_id() - get the current (unstable) CPU id
//
// raw_smp_processor_id() is arch-specific/arch-defined and
// may be a macro or a static inline function.
//
// For when you know what you are doing and need an unstable
// CPU id.
//
// Allow the architecture to differentiate between a stable and unstable read.
// For example, x86 uses an IRQ-safe asm-volatile read for the unstable but a
// regular asm read for the stable.
//

extern "C" {
    pub fn debug_smp_processor_id() -> c_uint;
}

//
// smp_processor_id() - get the current (stable) CPU id
//
// This is the normal accessor to the CPU id and should be used
// whenever possible.
//
// The CPU id is stable when:
//
// - IRQs are disabled;
// - preemption is disabled;
// - the task is CPU affine.
//
// When CONFIG_DEBUG_PREEMPT=y, we verify these assumptions and WARN
// when smp_processor_id() is used when the CPU id is not stable.
//

//
// Callback to arch code if there's nosmp or maxcpus=0 on the
// boot command line:
//
extern "C" {
    pub fn arch_disable_smp_support();
}
extern "C" {
    pub fn arch_thaw_secondary_cpus_begin();
}
extern "C" {
    pub fn arch_thaw_secondary_cpus_end();
}
extern "C" {
    pub fn smp_setup_processor_id();
}
// SMP core functions
extern "C" {
    pub fn smpcfd_prepare_cpu(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn smpcfd_dead_cpu(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn smpcfd_dying_cpu(cpu: c_uint) -> c_int;
}

extern "C" {
    pub fn csd_lock_is_stuck() -> bool;
}

