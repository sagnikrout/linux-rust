//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/kernel/head_booke.h
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
// Macros used for common Book-e exception handling
//

//
// Macro used to get to thread save registers.
// Note that entries 0-3 are used for the prolog code, and the remaining
// entries are available for specific exception use in the event a handler
// requires more than 4 scratch registers.
//

// Macro flag: #define BOOKE_CLEAR_BTB(reg)

// if from user, start at top of this thread's kernel stack */       \

// To handle the additional exception priority levels on Book-E
// processors we allocate a stack per additional priority level.
//
// On 44x/e500 we have critical and machine check
//
// Additionally we reserve a SPRG for each priority level so we can free up a
// GPR to use as the base for indirect access to the exception stacks.  This
// is necessary since the MMU is always on, for Book-E parts, and the stacks
// are offset from KERNELBASE.
//
// There is some space optimization to be had here if desired.  However
// to allow for a common kernel with support for debug exceptions either
// going to critical or their own debug level we aren't currently
// providing configurations that micro-optimize space usage.
//

// only on e500mc

//
// Exception prolog for critical/machine check exceptions.  This is a
// little different from the normal exception prolog above since a
// critical/machine check exception can potentially occur at any point
// during normal exception processing. Thus we cannot use the same SPRG
// registers as the normal prolog above. Instead we use a portion of the
// critical/machine check exception stack at low physical addresses.
//

// COMING FROM USER MODE */					     \
// COMING FROM PRIV MODE */					     \

//
// Guest Doorbell -- this is a bit odd in that uses GSRR0/1 despite
// being delivered to the host.  This exception can only happen
// inside a KVM guest -- so we just handle up to the DO_KVM rather
// than try to fit this into one of the existing prolog macros.
//

//
// Exception vectors.
//

// Check for a single step debug exception while in an exception
// handler before state has been saved.  This is to catch the case
// where an instruction that we are trying to single step causes
// an exception (eg ITLB/DTLB miss) and thus the first instruction of
// the exception handler generates a single step debug exception.
//
// If we get a debug trap on the first instruction of an exception handler,
// we reset the MSR_DE in the _exception handler's_ MSR (the debug trap is
// a critical exception, so we are using SPRN_CSRR1 to manipulate the MSR).
// The exception handler was handling a non-critical interrupt, so it will
// save (and later restore) the MSR via SPRN_CSRR1, which will still have
// the MSR_DE bit set.
//

// \
// If there is a single step or branch-taken exception in an	      \
// exception entry sequence, it was probably meant to apply to	      \
// the code where the exception occurred (since exception entry	      \
// doesn't turn off DE automatically).  We simulate the effect	      \
// of turning off DE on entry to an exception handler by turning      \
// off DE in the DSRR1 value and clearing the debug status.	      \
// \
// here it looks like we got an inappropriate debug exception. */     \
// restore state and get out */					      \
// continue normal handling for a debug exception... */		      \

// \
// If there is a single step or branch-taken exception in an	      \
// exception entry sequence, it was probably meant to apply to	      \
// the code where the exception occurred (since exception entry	      \
// doesn't turn off DE automatically).  We simulate the effect	      \
// of turning off DE on entry to an exception handler by turning      \
// off DE in the CSRR1 value and clearing the debug status.	      \
// \
// here it looks like we got an inappropriate debug exception. */     \
// restore state and get out */					      \
// continue normal handling for a critical exception... */	      \

//
// Instruction TLB Error interrupt handlers may call InstructionStorage
// directly without clearing ESR, so the ESR at this point may be left over
// from a prior interrupt.
//
// In any case, do_page_fault for BOOK3E does not use ESR and always expects
// dsisr to be 0. ESR_DST from a prior store in particular would confuse fault
// handling.
//

