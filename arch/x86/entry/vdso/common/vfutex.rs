//! Automatically rewritten from C to Rust
//! Source: arch/x86/entry/vdso/common/vfutex.c
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
// Assembly template for the try unlock functions. The basic functionality is:
//
// mov		esi, %eax	Move the TID into EAX
// xor		%ecx, %ecx	Clear ECX
// lock_cmpxchgl	%ecx, (%rdi)	Attempt the TID -> 0 transition
// .Lcs_start:					Start of the critical section
// jnz		.Lcs_end	If cmpxchl failed jump to the end
// .Lcs_success:				Start of the success section
// movq		%rcx, (%rdx)	Set the pending op pointer to 0
// .Lcs_end:					End of the critical section
//
// .Lcs_start and .Lcs_end establish the critical section range. .Lcs_success is
// technically not required, but there for illustration, debugging and testing.
//
// When CONFIG_COMPAT is enabled then the 64-bit VDSO provides two functions.
// One for the regular 64-bit sized pending operation pointer and one for a
// 32-bit sized pointer to support gaming emulators.
//
// The 32-bit VDSO provides only the one for 32-bit sized pointers.
//

    ({									\
    asm volatile (							\
    "						\n"	\
    "	lock cmpxchgl	%k[zero], %a[lock]	\n"	\
    "						\n"	\
    LABEL(prefix, start)					\
    "						\n"	\
    JNZ_END(prefix)						\
    "						\n"	\
    LABEL(prefix, success)					\
    "						\n"	\
    clear_pop					\
    "						\n"	\
    LABEL(prefix, end)					\
    : [tid]   "+&a" (__tid)					\
    : [lock]  "D"   (__lock),				\
    [pop]   "d"   (__pop),				\
    [zero]  "r"   (0UL)					\
    : "memory"						\
    );								\
    __tid;								\
    })

#[no_mangle]
pub unsafe extern "C" fn __vdso_futex_robust_list64_try_unlock(lock: *mut __u32, tid: __u32, pop: *mut __u64) -> __u32 {
    __u32 __vdso_futex_robust_list64_try_unlock(__u32 *lock, __u32 tid, __u64 *pop)
    {
    return futex_robust_try_unlock(__futex_list64, CLEAR_POPQ, lock, tid, pop);
    }

#[no_mangle]
pub unsafe extern "C" fn __vdso_futex_robust_list32_try_unlock(lock: *mut __u32, tid: __u32, pop: *mut __u32) -> __u32 {
    __u32 __vdso_futex_robust_list32_try_unlock(__u32 *lock, __u32 tid, __u32 *pop)
    {
    return futex_robust_try_unlock(__futex_list32, CLEAR_POPL, lock, tid, pop);
    }
