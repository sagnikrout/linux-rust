//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/cpumask_types.h
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

// Don't assign or return these: may not be this big!
//
// cpumask_bits - get the bits in a cpumask
// @maskp: the struct cpumask
//
// You should only assume nr_cpu_ids bits of this mask are valid.  This is
// a macro so it's const-correct.
//

//
// cpumask_var_t: struct cpumask for stack usage.
//
// Oh, the wicked games we play!  In order to make kernel coding a
// little more difficult, we typedef cpumask_var_t to an array or a
// pointer: doing &mask on an array is a noop, so it still works.
//
// i.e.
// cpumask_var_t tmpmask;
// if (!alloc_cpumask_var(&tmpmask, GFP_KERNEL))
// return -ENOMEM;
//
// ... use 'tmpmask' like a normal struct cpumask * ...
//
// free_cpumask_var(tmpmask);
//
// However, one notable exception is there. alloc_cpumask_var() allocates
// only nr_cpumask_bits bits (in the other hand, real cpumask_t always has
// NR_CPUS bits). Therefore you don't have to dereference cpumask_var_t.
//
// cpumask_var_t tmpmask;
// if (!alloc_cpumask_var(&tmpmask, GFP_KERNEL))
// return -ENOMEM;
//
// var = *tmpmask;
//
// This code makes NR_CPUS length memcopy and brings to a memory corruption.
// cpumask_copy() provide safe copy functionality.
//
// Note that there is another evil here: If you define a cpumask_var_t
// as a percpu variable then the way to obtain the address of the cpumask
// structure differently influences what this_cpu_* operation needs to be
// used. Please use this_cpu_cpumask_var_t in those cases. The direct use
// of this_cpu_ptr() or this_cpu_read() will lead to failures when the
// other type of cpumask_var_t implementation is configured.
//
// Please also note that __cpumask_var_read_mostly can be used to declare
// a cpumask_var_t variable itself (not its content) as read mostly.
//

