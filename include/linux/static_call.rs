//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/static_call.h
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
// Static call support
//
// Static calls use code patching to hard-code function pointers into direct
// branch instructions. They give the flexibility of function pointers, but
// with improved performance. This is especially important for cases where
// retpolines would otherwise be used, as retpolines can significantly impact
// performance.
//
// API overview:
//
// DECLARE_STATIC_CALL(name, func);
// DEFINE_STATIC_CALL(name, func);
// DEFINE_STATIC_CALL_NULL(name, typename);
// DEFINE_STATIC_CALL_RET0(name, typename);
//
// __static_call_return0;
//
// static_call(name)(args...);
// static_call_cond(name)(args...);
// static_call_update(name, func);
// static_call_query(name);
//
// EXPORT_STATIC_CALL{,_TRAMP}{,_GPL}()
//
// Usage example:
//
// # Start with the following functions (with identical prototypes):
// int func_a(int arg1, int arg2);
// int func_b(int arg1, int arg2);
//
// # Define a 'my_name' reference, associated with func_a() by default
// DEFINE_STATIC_CALL(my_name, func_a);
//
// # Call func_a()
// static_call(my_name)(arg1, arg2);
//
// # Update 'my_name' to point to func_b()
// static_call_update(my_name, &func_b);
//
// # Call func_b()
// static_call(my_name)(arg1, arg2);
//
// Implementation details:
//
// This requires some arch-specific code (CONFIG_HAVE_STATIC_CALL).
// Otherwise basic indirect calls are used (with function pointers).
//
// Each static_call() site calls into a trampoline associated with the name.
// The trampoline has a direct branch to the default function.  Updates to a
// name will modify the trampoline's branch destination.
//
// If the arch has CONFIG_HAVE_STATIC_CALL_INLINE, then the call sites
// themselves will be patched at runtime to call the functions directly,
// rather than calling through the trampoline.  This requires objtool or a
// compiler plugin to detect all the static_call() sites and annotate them
// in the .static_call_sites section.
//
// Notes on NULL function pointers:
//
// Static_call()s support NULL functions, with many of the caveats that
// regular function pointers have.
//
// Clearly calling a NULL function pointer is 'BAD', so too for
// static_call()s (although when HAVE_STATIC_CALL it might not be immediately
// fatal). A NULL static_call can be the result of:
//
// DECLARE_STATIC_CALL_NULL(my_static_call, void (*)(int));
//
// which is equivalent to declaring a NULL function pointer with just a
// typename:
//
// void (*my_func_ptr)(int arg1) = NULL;
//
// or using static_call_update() with a NULL function. In both cases the
// HAVE_STATIC_CALL implementation will patch the trampoline with a RET
// instruction, instead of an immediate tail-call JMP. HAVE_STATIC_CALL_INLINE
// architectures can patch the trampoline call to a NOP.
//
// In all cases, any argument evaluation is unconditional. Unlike a regular
// conditional function pointer call:
//
// if (my_func_ptr)
// my_func_ptr(arg1)
//
// where the argument evaludation also depends on the pointer value.
//
// When calling a static_call that can be NULL, use:
//
// static_call_cond(name)(arg1);
//
// which will include the required value tests to avoid NULL-pointer
// dereferences.
//
// To query which function is currently set to be called, use:
//
// func = static_call_query(name);
//
// DEFINE_STATIC_CALL_RET0 / __static_call_return0:
//
// Just like how DEFINE_STATIC_CALL_NULL() / static_call_cond() optimize the
// conditional void function call, DEFINE_STATIC_CALL_RET0
// __static_call_return0 optimize the do nothing return 0 function.
//
// This feature is strictly UB per the C standard (since it casts a function
// pointer to a different signature) and relies on the architecture ABI to
// make things work. In particular it relies on Caller Stack-cleanup and the
// whole return register being clobbered for short return values. All normal
// CDECL style ABIs conform.
//
// In particular the x86_64 implementation replaces the 5 byte CALL
// instruction at the callsite with a 5 byte clear of the RAX register,
// completely eliding any function call overhead.
//
// Notably argument setup is unconditional.
//
// EXPORT_STATIC_CALL() vs EXPORT_STATIC_CALL_TRAMP():
//
// The difference is that the _TRAMP variant tries to only export the
// trampoline with the result that a module can use static_call{,_cond}() but
// not static_call_update().
//

//
// Either @site or @tramp can be NULL.
//
extern "C" {
    pub fn arch_static_call_transform(site: *mut c_void, tramp: *mut c_void, func: *mut c_void, tail: bool);
}

extern "C" {
    pub fn static_call_init() -> int __init;
}
extern "C" {
    pub fn static_call_force_reinit();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct static_call_mod {
    pub next: *mut static_call_mod,
    pub /: *mut *mut *mut module mod; / for vmlinux, mod == NULL,
    pub sites: *mut static_call_site,
}

// For finding the key associated with a trampoline
#[repr(C)]
#[derive(Copy, Clone)]
pub struct static_call_tramp_key {
    pub tramp: i32,
    pub key: i32,
}

extern "C" {
    pub fn __static_call_update(key: *mut static_call_key, tramp: *mut c_void, func: *mut c_void);
}
extern "C" {
    pub fn static_call_mod_init(mod: *mut module) -> c_int;
}
extern "C" {
    pub fn static_call_text_reserved(start: *mut c_void, end: *mut c_void) -> c_int;
}
extern "C" {
    pub fn __static_call_return0() -> c_long;
}

// Leave the key unexported, so modules can't change static call targets:

pub const static_call_initialized: c_int = 0;

extern "C" {
    pub fn __static_call_return0() -> c_long;
}

// Leave the key unexported, so modules can't change static call targets:

pub const static_call_initialized: c_int = 0;

//
// This horrific hack takes care of two things:
//
// - it ensures the compiler will only load the function pointer ONCE,
// which avoids a reload race.
//
// - it ensures the argument evaluation is unconditional, similar
// to the HAVE_STATIC_CALL variant.
//
// Sadly current GCC/Clang (10 for both) do not optimize this properly
// and will emit an indirect call for the NULL case :-(
//

