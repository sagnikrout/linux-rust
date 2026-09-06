//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/uaccess.h
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
// User space memory access functions
//

extern "C" {
    pub fn __get_user_1() -> c_int;
}
extern "C" {
    pub fn __get_user_2() -> c_int;
}
extern "C" {
    pub fn __get_user_4() -> c_int;
}
extern "C" {
    pub fn __get_user_8() -> c_int;
}
extern "C" {
    pub fn __get_user_nocheck_1() -> c_int;
}
extern "C" {
    pub fn __get_user_nocheck_2() -> c_int;
}
extern "C" {
    pub fn __get_user_nocheck_4() -> c_int;
}
extern "C" {
    pub fn __get_user_nocheck_8() -> c_int;
}
extern "C" {
    pub fn __get_user_bad() -> c_int;
}

//
// This is the smallest unsigned integer type that can fit a value
// (up to 'long long')
//

//
// This is used for both get_user() and __get_user() to expand to
// the proper special function call that has odd calling conventions
// due to returning both a value and an error, and that depends on
// the size of the pointer passed in.
//
// Careful: we have to cast the result to the type of the pointer
// for sign reasons.
//
// The use of _ASM_DX as the register specifier is a bit of a
// simplification, as gcc only cares about it as the starting point
// and not size: for a 64-bit value it will use %ecx:%edx on 32 bits
// (%ecx being the next register in gcc's x86 register sequence), and
// %rdx on 64 bits.
//
// Clang/LLVM cares about the size of the register, but still wants
// the base register for something that ends up being a pair.
//

//
// get_user - Get a simple variable from user space.
// @x:   Variable to store result.
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
// Return: zero on success, or -EFAULT on error.
// On error, the variable @x is set to zero.
//

//
// __get_user - Get a simple variable from user space, with less checking.
// @x:   Variable to store result.
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
// Return: zero on success, or -EFAULT on error.
// On error, the variable @x is set to zero.
//

extern "C" {
    pub fn __put_user_bad();
}
//
// Strange magic calling convention: pointer in %ecx,
// value in %eax(:%edx), return value in %ecx. clobbers %rbx
//
extern "C" {
    pub fn __put_user_1();
}
extern "C" {
    pub fn __put_user_2();
}
extern "C" {
    pub fn __put_user_4();
}
extern "C" {
    pub fn __put_user_8();
}
extern "C" {
    pub fn __put_user_nocheck_1();
}
extern "C" {
    pub fn __put_user_nocheck_2();
}
extern "C" {
    pub fn __put_user_nocheck_4();
}
extern "C" {
    pub fn __put_user_nocheck_8();
}
//
// ptr must be evaluated and assigned to the temporary __ptr_pu before
// the assignment of x to __val_pu, to avoid any function calls
// involved in the ptr expression (possibly implicitly generated due
// to KASAN) from clobbering %ax.
//

//
// put_user - Write a simple value into user space.
// @x:   Value to copy to user space.
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
// Return: zero on success, or -EFAULT on error.
//

//
// __put_user - Write a simple value into user space, with less checking.
// @x:   Value to copy to user space.
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
// Return: zero on success, or -EFAULT on error.
//

// _old = __old;						\

// _old = __old;						\

// _old = __old;						\

//
// Unlike the normal CMPXCHG, use output GPR for both success/fail and error.
// There are only six GPRs available and four (EAX, EBX, ECX, and EDX) are
// hardcoded by CMPXCHG8B, leaving only ESI and EDI.  If the compiler uses
// both ESI and EDI for the memory operand, compilation will fail if the error
// is an input+output as there will be no register available for input.
//

// _old = __old;						\

// FIXME: this hack is definitely wrong -AK
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __large_struct {

//
// Tell gcc we read from memory instead of writing: this is because
// we do not write to any memory gcc knows about, so there are no
// aliasing issues.
//

    pub n): *const *const *const copy_from_user_nmi(void to, void __user from, unsigned long,
    pub count): *const *const *const strncpy_from_user(char dst, char __user src, long,
    pub n): *const *const extern __must_check long strnlen_user(char __user str, long,

    pub len): *const *const *const copy_mc_to_kernel(void to, void from, unsigned,

    pub len): *const *const *const copy_mc_to_user(void __user to, void from, unsigned,

//
// movsl can be slow when source and dest are not both 8-byte aligned
//

    pub mask: c_int,
    pub movsl_mask: } ____cacheline_aligned_in_smp,

pub const ARCH_HAS_NONTEMPORAL_UACCESS: c_int = 1;
//
// The "unsafe" user accesses aren't really "unsafe", but the naming
// is a big fat warning: you have to not only do the access_ok()
// checking before using them, but you have to surround them with the
// user_access_begin/end() pair.
//
    pub 0: return,
    pub 1: return,

    pub \: *mut *mut __inttype((ptr)) __gu_val;,
    pub \: *mut *mut __get_user_size(__gu_val, (ptr), sizeof((ptr)), err_label);,
    pub \: *mut *mut (x) = ( __typeof__((ptr)))__gu_val;,

    pub \: int __gu_err;,
    pub \: *mut *mut __inttype((ptr)) __gu_val;,
    pub \: *mut *mut __get_user_size(__gu_val, (ptr), sizeof((ptr)), __gu_err);,
    pub \: *mut *mut (x) = ( __typeof__((ptr)))__gu_val;,
    pub \: if (unlikely(__gu_err)) goto err_label;,

    pub __try_cmpxchg_user_wrong_size(void): extern void,

//
// Force the pointer to u<size> to match the size expected by the asm helper.
// clang/LLVM compiles all cases and only discards the unused paths after
// processing errors, which breaks i386 if the pointer is an 8-byte value.
//

    pub \: bool __ret;,
    pub \: __chk_user_ptr(_ptr);,
    pub \: (_nval), _label);,
    pub \: break;,
    pub \: (_nval), _label);,
    pub \: break;,
    pub \: (_nval), _label);,
    pub \: break;,
    pub \: (_nval), _label);,
    pub \: break;,
    pub \: default: __try_cmpxchg_user_wrong_size();,
    pub }): __ret;,
// "Returns" 0 on success, 1 on failure, -EFAULT if the access faults.

    pub \: int __ret = -EFAULT;,
    pub \: __uaccess_begin_nospec();,
    pub \: __ret = !unsafe_try_cmpxchg_user(_ptr, _oldp, _nval, _label);,
    pub \: __uaccess_end();,
    pub \: __ret;,
//
// We want the unsafe accessors to always be inlined and use
// the error labels - thus the macro games.
//

    pub \: *mut *mut *mut *mut unsafe_put_user((type )(src),(type __user )(dst),label);,
    pub \: dst += sizeof(type);,
    pub \: src += sizeof(type);,
    pub \: len -= sizeof(type);,

    pub \: *mut *mut char __user __ucu_dst = (_dst);,
    pub \: *const *const char __ucu_src = (_src);,
    pub \: size_t __ucu_len = (_len);,
    pub \: unsafe_copy_loop(__ucu_dst, __ucu_src, __ucu_len, u64, label);,
    pub \: unsafe_copy_loop(__ucu_dst, __ucu_src, __ucu_len, u32, label);,
    pub \: unsafe_copy_loop(__ucu_dst, __ucu_src, __ucu_len, u16, label);,
    pub \: unsafe_copy_loop(__ucu_dst, __ucu_src, __ucu_len, u8, label);,

    pub \: int __kr_err;,
    pub \: sizeof(type), __kr_err);,
    pub \: goto err_label;,

