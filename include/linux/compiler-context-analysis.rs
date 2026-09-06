//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/compiler-context-analysis.h
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
// Macros and attributes for compiler-based static context analysis.
//

//
// These attributes define new context lock (Clang: capability) types.
// Internal only.
//

//
// The below are used to annotate code being checked. Internal only.
//

//
// The "assert_capability" attribute is a bit confusingly named. It does not
// generate a check. Instead, it tells the analysis to *assume* the capability
// is held. This is used for augmenting runtime assertions, that can then help
// with patterns beyond the compiler's static reasoning abilities.
//

//
// __guarded_by() - struct member and globals attribute, declares variable
// only accessible within active context
// @...: context lock instance pointer(s)
//
// Declares that the struct member or global variable is only accessible within
// the context entered by the given context lock(s). Read operations on the data
// require shared access to at least one of the context locks, while write
// operations require exclusive access to all listed context locks.
//
// .. code-block:: c
//
// struct some_state {
// spinlock_t lock;
// long counter __guarded_by(&lock);
// };
//
// struct some_state {
// spinlock_t lock1, lock2;
// long counter __guarded_by(&lock1, &lock2);
// };
//

//
// __pt_guarded_by() - struct member and globals attribute, declares pointed-to
// data only accessible within active context
// @...: context lock instance pointer(s)
//
// Declares that the data pointed to by the struct member pointer or global
// pointer is only accessible within the context entered by the given context
// lock(s). Read operations on the data require shared access to at least one
// of the context locks, while write operations require exclusive access to all
// listed context locks.
//
// .. code-block:: c
//
// struct some_state {
// spinlock_t lock;
// long *counter __pt_guarded_by(&lock);
// };
//
// struct some_state {
// spinlock_t lock1, lock2;
// long *counter __pt_guarded_by(&lock1, &lock2);
// };
//

//
// context_lock_struct() - declare or define a context lock struct
// @name: struct name
//
// Helper to declare or define a struct type that is also a context lock.
//
// .. code-block:: c
//
// context_lock_struct(my_handle) {
// int foo;
// long bar;
// };
//
// struct some_state {
// ...
// };
// // ... declared elsewhere ...
// context_lock_struct(some_state);
//
// Note: The implementation defines several helper functions that can acquire
// and release the context lock.
//

//
// disable_context_analysis() - disables context analysis
//
// Disables context analysis. Must be paired with a later
// enable_context_analysis().
//

//
// enable_context_analysis() - re-enables context analysis
//
// Re-enables context analysis. Must be paired with a prior
// disable_context_analysis().
//

//
// __no_context_analysis - function attribute, disables context analysis
//
// Function attribute denoting that context analysis is disabled for the
// whole function. Prefer use of `context_unsafe()` where possible.
//

//
// context_unsafe() - disable context checking for contained code
//
// Disables context checking for contained statements or expression.
//
// .. code-block:: c
//
// struct some_data {
// spinlock_t lock;
// int counter __guarded_by(&lock);
// };
//
// int foo(struct some_data *d)
// {
// // ...
// // other code that is still checked ...
// // ...
// return context_unsafe(d->counter);
// }
//

//
// __context_unsafe() - function attribute, disable context checking
// @comment: comment explaining why opt-out is safe
//
// Function attribute denoting that context analysis is disabled for the
// whole function. Forces adding an inline comment as argument.
//

//
// context_unsafe_alias() - helper to insert a context lock "alias barrier"
// @p: pointer aliasing a context lock or object containing context locks
//
// No-op function that acts as a "context lock alias barrier", where the
// analysis rightfully detects that we're switching aliases, but the switch is
// considered safe but beyond the analysis reasoning abilities.
//
// This should be inserted before the first use of such an alias.
//
// Implementation Note: The compiler ignores aliases that may be reassigned but
// their value cannot be determined (e.g. when passing a non-const pointer to an
// alias as a function argument).
//

//
// token_context_lock() - declare an abstract global context lock instance
// @name: token context lock name
//
// Helper that declares an abstract global context lock instance @name, but not
// backed by a real data structure (linker error if accidentally referenced).
// The type name is `__ctx_lock_@name`.
//

//
// token_context_lock_instance() - declare another instance of a global context lock
// @ctx: token context lock previously declared with token_context_lock()
// @name: name of additional global context lock instance
//
// Helper that declares an additional instance @name of the same token context
// lock class @ctx. This is helpful where multiple related token contexts are
// declared, to allow using the same underlying type (`__ctx_lock_@ctx`) as
// function arguments.
//

//
// Common keywords for static context analysis.
//
// __must_hold() - function attribute, caller must hold exclusive context lock
//
// Function attribute declaring that the caller must hold the given context
// lock instance(s) exclusively.
//

//
// __must_not_hold() - function attribute, caller must not hold context lock
//
// Function attribute declaring that the caller must not hold the given context
// lock instance(s).
//

//
// __acquires() - function attribute, function acquires context lock exclusively
//
// Function attribute declaring that the function acquires the given context
// lock instance(s) exclusively, but does not release them.
//

//
// Clang's analysis does not care precisely about the value, only that it is
// either zero or non-zero. So the __cond_acquires() interface might be
// misleading if we say that @ret is the value returned if acquired. Instead,
// provide symbolic variants which we translate.
//

//
// __cond_acquires() - function attribute, function conditionally
// acquires a context lock exclusively
// @ret: abstract value returned by function if context lock acquired
// @x: context lock instance pointer
//
// Function attribute declaring that the function conditionally acquires the
// given context lock instance @x exclusively, but does not release it. The
// function return value @ret denotes when the context lock is acquired.
//
// @ret may be one of: true, false, nonzero, 0, nonnull, NULL.
//

//
// __releases() - function attribute, function releases a context lock exclusively
//
// Function attribute declaring that the function releases the given context
// lock instance(s) exclusively. The associated context(s) must be active on
// entry.
//

//
// Clang's analysis does not care precisely about the value, only that it is
// either zero or non-zero. So the __cond_acquires() interface might be
// misleading if we say that @ret is the value returned if acquired. Instead,
// provide symbolic variants which we translate.
//

//
// __cond_releases() - function attribute, function conditionally
// releases a context lock exclusively
// @ret: abstract value returned by function if context lock releases
// @x: context lock instance pointer
//
// Function attribute declaring that the function conditionally releases the
// given context lock instance @x exclusively. The associated context(s) must
// be active on entry. The function return value @ret denotes when the context
// lock is released.
//
// @ret may be one of: true, false, nonzero, 0, nonnull, NULL.
//
// NOTE: clang does not have a native attribute for this; instead implement
// it as an unconditional release and a conditional acquire for the
// inverted condition -- which is semantically equivalent.
//

//
// __acquire() - function to acquire context lock exclusively
// @x: context lock instance pointer
//
// No-op function that acquires the given context lock instance @x exclusively.
//

//
// __release() - function to release context lock exclusively
// @x: context lock instance pointer
//
// No-op function that releases the given context lock instance @x.
//

//
// __must_hold_shared() - function attribute, caller must hold shared context lock
//
// Function attribute declaring that the caller must hold the given context
// lock instance(s) with shared access.
//

//
// __acquires_shared() - function attribute, function acquires context lock shared
//
// Function attribute declaring that the function acquires the given
// context lock instance(s) with shared access, but does not release them.
//

//
// __cond_acquires_shared() - function attribute, function conditionally
// acquires a context lock shared
// @ret: abstract value returned by function if context lock acquired
// @x: context lock instance pointer
//
// Function attribute declaring that the function conditionally acquires the
// given context lock instance @x with shared access, but does not release it.
// The function return value @ret denotes when the context lock is acquired.
//
// @ret may be one of: true, false, nonzero, 0, nonnull, NULL.
//

//
// __releases_shared() - function attribute, function releases a
// context lock shared
//
// Function attribute declaring that the function releases the given context
// lock instance(s) with shared access. The associated context(s) must be
// active on entry.
//

//
// __acquire_shared() - function to acquire context lock shared
// @x: context lock instance pointer
//
// No-op function that acquires the given context lock instance @x with shared
// access.
//

//
// __release_shared() - function to release context lock shared
// @x: context lock instance pointer
//
// No-op function that releases the given context lock instance @x with shared
// access.
//

//
// __acquire_ret() - helper to acquire context lock of return value
// @call: call expression
// @ret_expr: acquire expression that uses __ret
//

//
// __acquire_shared_ret() - helper to acquire context lock shared of return value
// @call: call expression
// @ret_expr: acquire shared expression that uses __ret
//

//
// Attributes to mark functions returning acquired context locks.
//
// This is purely cosmetic to help readability, and should be used with the
// above macros as follows:
//
// struct foo { spinlock_t lock; ... };
// ...
// #define myfunc(...) __acquire_ret(_myfunc(__VA_ARGS__), &__ret->lock)
// struct foo *_myfunc(int bar) __acquires_ret;
// ...
//

