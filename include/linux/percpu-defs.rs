//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/percpu-defs.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// linux/percpu-defs.h - basic definitions for percpu areas
//
// DO NOT INCLUDE DIRECTLY OUTSIDE PERCPU IMPLEMENTATION PROPER.
//
// This file is separate from linux/percpu.h to avoid cyclic inclusion
// dependency from arch header files.  Only to be included from
// asm/percpu.h.
//
// This file includes macros necessary to declare percpu sections and
// variables, and definitions of percpu accessors and operations.  It
// should provide enough percpu features to arch header files even when
// they can only include asm/percpu.h to avoid cyclic inclusion dependency.
//

//
// Base implementations of per-CPU variable declarations and definitions, where
// the section in which the variable is to be placed is provided by the
// 'sec' argument.  This may be used to affect the parameters governing the
// variable's storage.
//
// NOTE!  The sections for the DECLARE and for the DEFINE must match, lest
// linkage errors occur due the compiler generating the wrong code to access
// that section.
//

//
// alpha modules require percpu variables to be defined as
// weak to force the compiler to generate GOT based external
// references for them.  This is necessary because percpu sections
// will be located outside of the usually addressable area.
//
// This definition puts the following two extra restrictions when
// defining percpu variables.
//
// 1. The symbol must be globally unique, even the static ones.
// 2. Static percpu variables cannot be defined inside a function.
//
// Archs which need weak percpu definitions should set
// CONFIG_ARCH_MODULE_NEEDS_WEAK_PER_CPU when necessary.
//

//
// __pcpu_scope_* dummy variable is used to enforce scope.  It
// receives the static modifier when it's used in front of
// DEFINE_PER_CPU() and will trigger build failure if
// DECLARE_PER_CPU() is used for the same variable.
//
// __pcpu_unique_* dummy variable is used to enforce symbol uniqueness
// such that hidden weak symbol collision, which will cause unrelated
// variables to share the same address, can be detected during build.
//

//
// Normal declaration and definition macros.
//

//
// Variant on the per-CPU variable declaration/definition theme used for
// ordinary per-CPU variables.
//

//
// Declaration/definition used for per-CPU variables that are frequently
// accessed and should be in a single cacheline.
//
// For use only by architecture and core code.  Only use scalar or pointer
// types to maximize density.
//

//
// Declaration/definition used for per-CPU variables that must be cacheline
// aligned under SMP conditions so that, whilst a particular instance of the
// data corresponds to a particular CPU, inefficiencies due to direct access by
// other CPUs are reduced by preventing the data from unnecessarily spanning
// cachelines.
//
// An example of this would be statistical data, where each CPU's set of data
// is updated by that CPU alone, but the data from across all CPUs is collated
// by a CPU processing a read from a proc file.
//

//
// Declaration/definition used for per-CPU variables that must be page aligned.
//

//
// Declaration/definition used for per-CPU variables that must be read mostly.
//

//
// Declaration/definition used for per-CPU variables that should be accessed
// as decrypted when memory encryption is enabled in the guest.
//

//
// Intermodule exports for per-CPU variables.  sparse forgets about
// address space across EXPORT_SYMBOL(), change EXPORT_SYMBOL() to
// noop if __CHECKER__.
//

// Macro flag: #define EXPORT_PER_CPU_SYMBOL(var)
// Macro flag: #define EXPORT_PER_CPU_SYMBOL_GPL(var)

//
// Accessors and operations.
//
// __verify_pcpu_ptr() verifies @ptr is a percpu pointer without evaluating
// @ptr and is invoked once before a percpu area is accessed by all
// accessors and operations.  This is performed in the generic part of
// percpu and arch overrides don't need to worry about it; however, if an
// arch wants to implement an arch-specific percpu accessor or operation,
// it may use __verify_pcpu_ptr() to verify the parameters.
//
// + 0 is required in order to convert the pointer type from a
// potential array type to a pointer to a single item of the array.
//

//
// Add an offset to a pointer.  Use RELOC_HIDE() to prevent the compiler
// from making incorrect assumptions about the pointer value.
//

//
// Must be an lvalue. Since @var must be a simple identifier,
// we force a syntax error here if it isn't.
//

//
// The weird & is necessary because sparse considers (void)(var) to be
// a direct dereference of percpu variable (var).
//

//
// Branching function to split up a function into a set of functions that
// are called for different scalar sizes of the objects handled.
//
extern "C" {
    pub fn __bad_size_call_parameter();
}

extern "C" {
    pub fn __this_cpu_preempt_check(op: *const c_char);
}

//
// this_cpu operations (C) 2008-2013 Christoph Lameter <cl@gentwo.org>
//
// Optimized manipulation for memory allocated through the per cpu
// allocator or for addresses of per cpu variables.
//
// These operation guarantee exclusivity of access for other operations
// on the *same* processor. The assumption is that per cpu data is only
// accessed by a single processor instance (the current one).
//
// The arch code can provide optimized implementation by defining macros
// for certain scalar sizes. F.e. provide this_cpu_add_2() to provide per
// cpu atomic operations for 2 byte sized RMW actions. If arch code does
// not provide operations for a scalar size then the fallback in the
// generic code will be used.
//
// cmpxchg_double replaces two adjacent scalars at once.  The first two
// parameters are per cpu variables which have to be of the same size.  A
// truth value is returned to indicate success or failure (since a double
// register result is difficult to handle).  There is very limited hardware
// support for these operations, so only certain sizes may work.
//
// Operations for contexts where we do not want to do any checks for
// preemptions.  Unless strictly necessary, always use [__]this_cpu_*()
// instead.
//
// If there is no other protection through preempt disable and/or disabling
// interrupts then one of these RMW operations can show unexpected behavior
// because the execution thread was rescheduled on another processor or an
// interrupt occurred and the same percpu variable was modified from the
// interrupt context.
//

//
// Operations for contexts that are safe from preemption/interrupts.  These
// operations verify that preemption is disabled.
//

//
// Operations with implied preemption/interrupt protection.  These
// operations can be used without worrying about preemption or interrupt.
//

