//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/uaccess.h
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
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//
// Derived from MIPS:
// Copyright (C) 1996, 1997, 1998, 1999, 2000, 03, 04 by Ralf Baechle
// Copyright (C) 1999, 2000 Silicon Graphics, Inc.
// Copyright (C) 2007  Maciej W. Rozycki
// Copyright (C) 2014, Imagination Technologies Ltd.
//

pub const __LSW: c_int = 0;
pub const __MSW: c_int = 1;

pub const __UA_LIMIT: c_uint = 0x80000000UL;

//
// get_user: - Get a simple variable from user space.
// @x:	 Variable to store result.
// @ptr: Source address, in user space.
//
// Context: User context only. This function may sleep if pagefaults are
// enabled.
//
// This macro copies a single simple variable from user space to kernel
// space.  It supports simple types like char and int, but not larger
// data types like structures or arrays.
//
// @ptr must have pointer-to-simple-variable type, and the result of
// dereferencing @ptr must be assignable to @x without a cast.
//
// Returns zero on success, or -EFAULT on error.
// On error, the variable @x is set to zero.
//

//
// put_user: - Write a simple value into user space.
// @x:	 Value to copy to user space.
// @ptr: Destination address, in user space.
//
// Context: User context only. This function may sleep if pagefaults are
// enabled.
//
// This macro copies a single simple value from kernel space to user
// space.  It supports simple types like char and int, but not larger
// data types like structures or arrays.
//
// @ptr must have pointer-to-simple-variable type, and @x must be assignable
// to the result of dereferencing @ptr.
//
// Returns zero on success, or -EFAULT on error.
//

//
// __get_user: - Get a simple variable from user space, with less checking.
// @x:	 Variable to store result.
// @ptr: Source address, in user space.
//
// Context: User context only. This function may sleep if pagefaults are
// enabled.
//
// This macro copies a single simple variable from user space to kernel
// space.  It supports simple types like char and int, but not larger
// data types like structures or arrays.
//
// @ptr must have pointer-to-simple-variable type, and the result of
// dereferencing @ptr must be assignable to @x without a cast.
//
// Caller must check the pointer with access_ok() before calling this
// function.
//
// Returns zero on success, or -EFAULT on error.
// On error, the variable @x is set to zero.
//

//
// __put_user: - Write a simple value into user space, with less checking.
// @x:	 Value to copy to user space.
// @ptr: Destination address, in user space.
//
// Context: User context only. This function may sleep if pagefaults are
// enabled.
//
// This macro copies a single simple value from kernel space to user
// space.  It supports simple types like char and int, but not larger
// data types like structures or arrays.
//
// @ptr must have pointer-to-simple-variable type, and @x must be assignable
// to the result of dereferencing @ptr.
//
// Caller must check the pointer with access_ok() before calling this
// function.
//
// Returns zero on success, or -EFAULT on error.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __large_struct {

    pub \: case 1: __get_data_asm(val, "ld.b", ptr); break;,
    pub \: case 2: __get_data_asm(val, "ld.h", ptr); break;,
    pub \: case 4: __get_data_asm(val, "ld.w", ptr); break;,
    pub \: case 8: __get_data_asm_8(val, ptr); break;,
    pub \: default: BUILD_BUG(); break;,

    pub \: long __gu_tmp;,
    pub \: : "m" (__m(ptr)));,
    pub \: *mut *mut (val) = (__typeof__((ptr))) __gu_tmp;,

    pub \: u32 __lo, __hi;,
    pub \: *mut *mut *mut u32 __user __ptr = (u32 __user )(ptr);,
    pub \: : "m" (__ptr[__LSW]), "m" (__ptr[__MSW]));,
    pub \: __hi = 0;,
    pub \: ((((u64)__hi << 32) | __lo)));,

    pub \: case 1: __put_data_asm("st.b", ptr); break;,
    pub \: case 2: __put_data_asm("st.h", ptr); break;,
    pub \: case 4: __put_data_asm("st.w", ptr); break;,
    pub \: case 8: __put_data_asm_8(ptr); break;,
    pub \: default: BUILD_BUG(); break;,

    pub \: : "Jr" (__pu_val));,

    pub \: *mut *mut *mut u32 __user __ptr = (u32 __user )(ptr);,
    pub \: u64 __x = (__typeof__((__pu_val)-(__pu_val)))(__pu_val);,
    pub \: : "rJ" (__x), "rJ" (__x >> 32));,

    pub \: int __gu_err = 0;,
    pub \: *mut *mut ( type )(src));,
    pub \: __func__, read_csr_excode());,
    pub \: __func__, __builtin_return_address(0));,
    pub \: goto err_label;,

    pub \: type __pu_val;,
    pub \: int __pu_err = 0;,
    pub \: *mut *mut *mut __pu_val = ( type )(src);,
    pub \: *mut *mut __put_kernel_common(((type )(dst)), sizeof(type));,
    pub \: __func__, read_csr_excode());,
    pub \: __func__, __builtin_return_address(0));,
    pub \: goto err_label;,
    pub n): *const *const *const extern unsigned long __copy_user(void to, void from, __kernel_size_t,
    pub n): *const *const return __copy_user(to, ( void )from,,
    pub n): *mut *mut return __copy_user(( void )to, from,,
// Macro flag: #define INLINE_COPY_USER
//
// __clear_user: - Zero a block of memory in user space, with less checking.
// @addr: Destination address, in user space.
// @size: Number of bytes to zero.
//
// Zero a block of memory in user space.  Caller must check
// the specified block with access_ok() before calling this function.
//
// Returns number of bytes that could not be cleared.
// On success, this will be zero.
//
    pub size): *mut *mut extern unsigned long __clear_user(void __user addr, __kernel_size_t,

    pub \: *mut *mut void __user __cl_addr = (addr);,
    pub \: unsigned long __cl_size = (n);,
    pub \: __cl_size = __clear_user(__cl_addr, __cl_size);,
    pub \: __cl_size;,
    pub n): *const *const *const extern long strncpy_from_user(char to, char __user from, long,
    pub n): *const *const extern long strnlen_user(char __user str, long,
