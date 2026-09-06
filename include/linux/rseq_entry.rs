//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rseq_entry.h
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
// Must be outside the CONFIG_RSEQ guard to resolve the stubs

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rseq_stats {
    pub exit: c_ulong,
    pub signal: c_ulong,
    pub slowpath: c_ulong,
    pub fastpath: c_ulong,
    pub ids: c_ulong,
    pub cs: c_ulong,
    pub clear: c_ulong,
    pub fixup: c_ulong,
    pub s_granted: c_ulong,
    pub s_expired: c_ulong,
    pub s_revoked: c_ulong,
    pub s_yielded: c_ulong,
    pub s_aborted: c_ulong,
}

//
// Slow path has interrupts and preemption enabled, but the fast path
// runs with interrupts disabled so there is no point in having the
// preemption checks implied in __this_cpu_inc() for every operation.
//

extern "C" {
    pub fn __rseq_trace_update(t: *mut task_struct);
}

// Macro flag: #define rseq_inline

extern "C" {
    pub fn static_branch_likely(_arg: &rseq_slice_extension_key) -> return;
}
extern "C" {
    pub fn __rseq_arm_slice_extension_timer() -> bool;
}
extern "C" {
    pub fn __rseq_arm_slice_extension_timer() -> return;
}
//
// Open coded, so it can be invoked within a user access region.
//
// This clears the user space state of the time slice extensions field only when
// the task has registered the optimized RSEQ_ABI V2. Some legacy registrations,
// e.g. TCMalloc, have conflicting non-ABI fields in struct RSEQ, which would be
// overwritten by an unconditional write.
//

// If not enabled or not a return from interrupt, nothing to do.
//
// Quick check conditions where a grant is not possible or
// needs to be revoked.
//
// 1) Any TIF bit which needs to do extra work aside of
// rescheduling prevents a grant.
//
// 2) A previous rescheduling request resulted in a slice
// extension grant.
//
// Clear user control unconditionally. No point for checking
// Grant the slice extention
// Store expiry time for arming the timer on the way out
//
// This is racy against a remote CPU setting TIF_NEED_RESCHED in
// several ways:
//
// 1)
// CPU0			CPU1
// clear_tsk()
// set_tsk()
// clear_preempt()
// Raise scheduler IPI on CPU0
// --> IPI
// fold_need_resched() -> Folds correctly
// 2)
// CPU0			CPU1
// set_tsk()
// clear_tsk()
// clear_preempt()
// Raise scheduler IPI on CPU0
// --> IPI
// fold_need_resched() <- NOOP as TIF_NEED_RESCHED is false
//
// #1 is not any different from a regular remote reschedule as it
// sets the previously not set bit and then raises the IPI which
// folds it into the preempt counter
//
// #2 is obviously incorrect from a scheduler POV, but it's not
// differently incorrect than the code below clearing the
// reschedule request with the safety net of the timer.
//
// The important part is that the clearing is protected against the
// scheduler IPI and also against any other interrupt which might
// end up waking up a task and setting the bits in the middle of
// the operation:
//
// clear_tsk()
// ---> Interrupt
// wakeup_on_this_cpu()
// set_tsk()
// set_preempt()
// clear_preempt()
//
// which would be inconsistent state.
//

extern "C" {
    pub fn rseq_debug_update_user_cs(t: *mut task_struct, regs: *mut pt_regs, csaddr: c_ulong) -> bool;
}
//
// Check whether there is a valid critical section and whether the
// instruction pointer in @regs is inside the critical section.
//
// - If the critical section is invalid, terminate the task.
//
// - If valid and the instruction pointer is inside, set it to the abort IP.
//
// - If valid and the instruction pointer is outside, clear the critical
// section address.
//
// Returns true, if the section was valid and either fixup or clear was
// done, false otherwise.
//
// In the failure case task::rseq_event::fatal is set when a invalid
// section was found. It's clear when the failure was an unresolved page
// fault.
//
// If inlined into the exit to user path with interrupts disabled, the
// caller has to protect against page faults with pagefault_disable().
//
// In preemptible task context this would be counterproductive as the page
// faults could not be fully resolved. As a consequence unresolved page
// faults in task context are fatal too.
//

//
// The debug version is put out of line, but kept here so the code stays
// together.
//
// @csaddr has already been checked by the caller to be in user space
//
// Evaluate the user pile and exit if one of the conditions
// is not fulfilled.
//
// If outside, just clear the critical section.
// Check for overflow and wraparound
// If not inside, clear it.
// Ensure it's "valid"
// Validate that the abort IP is not in the critical section
//
// Check version and flags for 0. No point in emitting
// deprecated warnings before dying. That could be done in
// the slow path eventually, but *shrug*.
//
// abort_ip - 4 is >= 0. See abort_ip check above
// rseq_event.user_irq is only valid if CONFIG_GENERIC_IRQ_ENTRY=y
// If not in interrupt from user context, let it die

//
// This only ensures that abort_ip is in the user address space and
// validates that it is preceded by the signature.
//
// No other sanity checks are done here, that's what the debug code is for.
//
extern "C" {
    pub fn rseq_debug_update_user_cs(_arg: t, _arg: regs, _arg: csaddr) -> return;
}
//
// No sanity checks. If user space screwed it up, it can
// keep the pieces. That's what debug code is for.
//
// If outside, just clear the critical section.
//
// Two requirements for @abort_ip:
// - Must be in user space as x86 IRET would happily return to
// the kernel.
// - The four bytes preceding the instruction at @abort_ip must
// contain the signature.
//
// The latter protects against the following attack vector:
//
// An attacker with limited abilities to write, creates a critical
// section descriptor, sets the abort IP to a library function or
// some other ROP gadget and stores the address of the descriptor
// in TLS::rseq::rseq_cs. An RSEQ abort would then evade ROP
// protection.
//
// The address is guaranteed to be >= 0 and < TASK_SIZE
// Invalidate the critical section
// Update the instruction pointer
//
// Updates CPU ID, Node ID and MM CID and reads the critical section
// address, when @csaddr != NULL. This allows to put the ID update and the
// read under the same uaccess region to spare a separate begin/end.
//
// As this is either invoked from a C wrapper with @csaddr = NULL or from
// the fast path code with a valid pointer, a clever compiler should be
// able to optimize the read out. Spares a duplicate implementation.
//
// Returns true, if the operation was successful, false otherwise.
//
// In the failure case task::rseq_event::fatal is set when invalid data
// was found on debug kernels. It's clear when the failure was an unresolved page
// fault.
//
// If inlined into the exit to user path with interrupts disabled, the
// caller has to protect against page faults with pagefault_disable().
//
// In preemptible task context this would be counterproductive as the page
// faults could not be fully resolved. As a consequence unresolved page
// faults in task context are fatal too.
//
// Validate the R/O fields for debug and optimized mode
// RSEQ ABI V2 only operations
// Cache the new values
//
// Update user space with new IDs and conditionally check whether the task
// is in a critical section.
//
// On architectures which utilize the generic entry code this
// allows to skip the critical section when the entry was not from
// a user space interrupt, unless debug mode is enabled.
//
// Sigh, this really needs to do work
extern "C" {
    pub fn rseq_update_user_cs(_arg: t, _arg: regs, _arg: csaddr) -> return;
}
//
// If you want to use this then convert your architecture to the generic
// entry code. I'm tired of building workarounds for people who can't be
// bothered to make the maintenance of generic infrastructure less
// burdensome. Just sucking everything into the architecture code and
// thereby making others chase the horrible hacks and keep them working is
// neither acceptable nor sustainable.
//

//
// This is inlined into the exit path because:
//
// 1) It's a one time comparison in the fast path when there is no event to
// handle
//
// 2) The access to the user space rseq memory (TLS) is unlikely to fault
// so the straight inline operation is:
//
// - Four 32-bit stores only if CPU ID/ MM CID need to be updated
// - One 64-bit load to retrieve the critical section address
//
// 3) In the unlikely case that the critical section address is != NULL:
//
// - One 64-bit load to retrieve the start IP
// - One 64-bit load to retrieve the offset for calculating the end
// - One 64-bit load to retrieve the abort IP
// - One 64-bit load to retrieve the signature
// - One store to clear the critical section address
//
// The non-debug case implements only the minimal required checking. It
// provides protection against a rogue abort IP in kernel space, which
// would be exploitable at least on x86, and also against a rogue CS
// descriptor by checking the signature at the abort IP. Any fallout from
// invalid critical section descriptors is a user space problem. The debug
// case provides the full set of checks and terminates the task if a
// condition is not met.
//
// In case of a fault or an invalid value, this sets TIF_NOTIFY_RESUME and
// tells the caller to loop back into exit_to_user_mode_loop(). The rseq
// slow path there will handle the failure.
//
// Page faults need to be disabled as this is called with
// interrupts disabled
//
// This optimization is only valid when the task registered for the
// optimized RSEQ_ABI_V2 variant. Some legacy users rely on the original
// RSEQ implementation behaviour which unconditionally updated the IDs.
// rseq_sched_switch_event() ensures that legacy registrations always
// have both sched_switch and ids_changed set, which is compatible with
// the historical TIF_NOTIFY_RESUME behaviour.
//
// If IDs have not changed rseq_event::user_irq must be true
// See rseq_sched_switch_event().
//
// RSEQ ABI V2 only operations
extern "C" {
    pub fn rseq_update_usr(_arg: t, _arg: regs, _arg: &ids) -> return;
}
//
// If the task did not go through schedule or got the flag enforced
// by the rseq syscall or execve, then nothing to do here.
//
// CPU ID and MM CID can only change when going through a context
// switch.
//
// rseq_sched_switch_event() sets the rseq_event::sched_switch bit
// only when rseq_event::has_rseq is true. That conditional is
// required to avoid setting the TIF bit if RSEQ is not registered
// for a task. rseq_event::sched_switch is cleared when RSEQ is
// unregistered by a task so it's sufficient to check for the
// sched_switch bit alone.
//
// A sane compiler requires three instructions for the nothing to do
// case including clearing the events, but your mileage might vary.
//
// Clear state so next entry starts from a clean slate
// Required to allow conversion to GENERIC_ENTRY w/o GENERIC_TIF_BITS

//
// Arm the slice extension timer if nothing to do anymore and the
// task really goes out to user space.
//
extern "C" {
    pub fn rseq_arm_slice_extension_timer() -> return;
}

// Needed to remove the store for the !lockdep case
//
// Ensure that event (especially user_irq) is cleared when the
// interrupt did not result in a schedule and therefore the
// rseq processing could not clear it.
//
extern "C" {
    pub fn __rseq_debug_syscall_return(regs: *mut pt_regs);
}

