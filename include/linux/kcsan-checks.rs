//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kcsan-checks.h
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
// KCSAN access checks and modifiers. These can be used to explicitly check
// uninstrumented accesses, or change KCSAN checking behaviour of accesses.
//
// Copyright (C) 2019, Google LLC.
//
// Note: Only include what is already included by compiler.h.

// Access types -- if KCSAN_ACCESS_WRITE is not set, the access is a read.

// The following are special, and never due to compiler instrumentation.

//
// __kcsan_*: Always calls into the runtime when KCSAN is enabled. This may be used
// even in compilation units that selectively disable KCSAN, but must use KCSAN
// to validate access to an address. Never use these in header files!
//

//
// __kcsan_check_access - check generic access for races
//
// @ptr: address of access
// @size: size of access
// @type: access type modifier
//
extern "C" {
    pub fn __kcsan_check_access(ptr: *const volatile void, size: usize, type: c_int);
}
//
// See definition of __tsan_atomic_signal_fence() in kernel/kcsan/core.c.
// Note: The mappings are arbitrary, and do not reflect any real mappings of C11
// memory orders to the LKMM memory orders and vice-versa!
//

//
// __kcsan_mb - full memory barrier instrumentation
//
extern "C" {
    pub fn __kcsan_mb();
}
//
// __kcsan_wmb - write memory barrier instrumentation
//
extern "C" {
    pub fn __kcsan_wmb();
}
//
// __kcsan_rmb - read memory barrier instrumentation
//
extern "C" {
    pub fn __kcsan_rmb();
}
//
// __kcsan_release - release barrier instrumentation
//
extern "C" {
    pub fn __kcsan_release();
}
//
// kcsan_disable_current - disable KCSAN for the current context
//
// Supports nesting.
//
extern "C" {
    pub fn kcsan_disable_current();
}
//
// kcsan_enable_current - re-enable KCSAN for the current context
//
// Supports nesting.
//
extern "C" {
    pub fn kcsan_enable_current();
}
//
// kcsan_nestable_atomic_begin - begin nestable atomic region
//
// Accesses within the atomic region may appear to race with other accesses but
// should be considered atomic.
//
extern "C" {
    pub fn kcsan_nestable_atomic_begin();
}
//
// kcsan_nestable_atomic_end - end nestable atomic region
//
extern "C" {
    pub fn kcsan_nestable_atomic_end();
}
//
// kcsan_flat_atomic_begin - begin flat atomic region
//
// Accesses within the atomic region may appear to race with other accesses but
// should be considered atomic.
//
extern "C" {
    pub fn kcsan_flat_atomic_begin();
}
//
// kcsan_flat_atomic_end - end flat atomic region
//
extern "C" {
    pub fn kcsan_flat_atomic_end();
}
//
// kcsan_atomic_next - consider following accesses as atomic
//
// Force treating the next n memory accesses for the current context as atomic
// operations.
//
// @n: number of following memory accesses to treat as atomic.
//
extern "C" {
    pub fn kcsan_atomic_next(n: c_int);
}
//
// kcsan_set_access_mask - set access mask
//
// Set the access mask for all accesses for the current context if non-zero.
// Only value changes to bits set in the mask will be reported.
//
// @mask: bitmask
//
extern "C" {
    pub fn kcsan_set_access_mask(mask: c_ulong);
}
// Scoped access information.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kcsan_scoped_access {
    pub /: *mut *mut list_head list; / scoped_accesses list,
//
// Not an entry in scoped_accesses list; stack depth from where
// the access was initialized.
//
    pub stack_depth: c_int,
}

// Access information.
// Location where scoped access was set up.
//
// Automatically call kcsan_end_scoped_access() when kcsan_scoped_access goes
// out of scope; relies on attribute "cleanup", which is supported by all
// compilers that support KCSAN.
//

//
// kcsan_begin_scoped_access - begin scoped access
//
// Begin scoped access and initialize @sa, which will cause KCSAN to
// continuously check the memory range in the current thread until
// kcsan_end_scoped_access() is called for @sa.
//
// Scoped accesses are implemented by appending @sa to an internal list for the
// current execution context, and then checked on every call into the KCSAN
// runtime.
//
// @ptr: address of access
// @size: size of access
// @type: access type modifier
// @sa: struct kcsan_scoped_access to use for the scope of the access
//
// kcsan_end_scoped_access - end scoped access
//
// End a scoped access, which will stop KCSAN checking the memory range.
// Requires that kcsan_begin_scoped_access() was previously called once for @sa.
//
// @sa: a previously initialized struct kcsan_scoped_access
//
extern "C" {
    pub fn kcsan_end_scoped_access(sa: *mut kcsan_scoped_access);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kcsan_scoped_access {

    pub }: *mut *mut kcsan_scoped_access sa) { return sa;,

//
// Only calls into the runtime when the particular compilation unit has KCSAN
// instrumentation enabled. May be used in header files.
//

//
// Only use these to disable KCSAN for accesses in the current compilation unit;
// calls into libraries may still perform KCSAN checks.
//

//
// Normal barrier instrumentation is not done via explicit calls, but by mapping
// to a repurposed __atomic_signal_fence(), which normally does not generate any
// real instructions, but is still intercepted by fsanitize=thread. This means,
// like any other compile-time instrumentation, barrier instrumentation can be
// disabled with the __no_kcsan function attribute.
//
// Also see definition of __tsan_atomic_signal_fence() in kernel/kcsan/core.c.
//
// These are all macros, like <asm/barrier.h>, since some architectures use them
// in non-static inline functions.
//

    pub \: barrier();,
    pub \: __atomic_signal_fence(__KCSAN_BARRIER_TO_SIGNAL_FENCE_##name);,
    pub \: barrier();,

//
// __kcsan_check_read - check regular read access for races
//
// @ptr: address of access
// @size: size of access
//

//
// __kcsan_check_write - check regular write access for races
//
// @ptr: address of access
// @size: size of access
//

//
// __kcsan_check_read_write - check regular read-write access for races
//
// @ptr: address of access
// @size: size of access
//

//
// kcsan_check_read - check regular read access for races
//
// @ptr: address of access
// @size: size of access
//

//
// kcsan_check_write - check regular write access for races
//
// @ptr: address of access
// @size: size of access
//

//
// kcsan_check_read_write - check regular read-write access for races
//
// @ptr: address of access
// @size: size of access
//

//
// Check for atomic accesses: if atomic accesses are not ignored, this simply
// aliases to kcsan_check_access(), otherwise becomes a no-op.
//

//
// ASSERT_EXCLUSIVE_WRITER - assert no concurrent writes to @var
//
// Assert that there are no concurrent writes to @var; other readers are
// allowed. This assertion can be used to specify properties of concurrent code,
// where violation cannot be detected as a normal data race.
//
// For example, if we only have a single writer, but multiple concurrent
// readers, to avoid data races, all these accesses must be marked; even
// concurrent marked writes racing with the single writer are bugs.
// Unfortunately, due to being marked, they are no longer data races. For cases
// like these, we can use the macro as follows:
//
// .. code-block:: c
//
// void writer(void) {
// spin_lock(&update_foo_lock);
// ASSERT_EXCLUSIVE_WRITER(shared_foo);
// WRITE_ONCE(shared_foo, ...);
// spin_unlock(&update_foo_lock);
// }
// void reader(void) {
// // update_foo_lock does not need to be held!
// ... = READ_ONCE(shared_foo);
// }
//
// Note: ASSERT_EXCLUSIVE_WRITER_SCOPED(), if applicable, performs more thorough
// checking if a clear scope where no concurrent writes are expected exists.
//
// @var: variable to assert on
//

//
// Helper macros for implementation of for ASSERT_EXCLUSIVE_*_SCOPED(). @id is
// expected to be unique for the scope in which instances of kcsan_scoped_access
// are declared.
//

    pub \: __kcsan_cleanup_scoped;,
//
// ASSERT_EXCLUSIVE_WRITER_SCOPED - assert no concurrent writes to @var in scope
//
// Scoped variant of ASSERT_EXCLUSIVE_WRITER().
//
// Assert that there are no concurrent writes to @var for the duration of the
// scope in which it is introduced. This provides a better way to fully cover
// the enclosing scope, compared to multiple ASSERT_EXCLUSIVE_WRITER(), and
// increases the likelihood for KCSAN to detect racing accesses.
//
// For example, it allows finding race-condition bugs that only occur due to
// state changes within the scope itself:
//
// .. code-block:: c
//
// void writer(void) {
// spin_lock(&update_foo_lock);
// {
// ASSERT_EXCLUSIVE_WRITER_SCOPED(shared_foo);
// WRITE_ONCE(shared_foo, 42);
// ...
// // shared_foo should still be 42 here!
// }
// spin_unlock(&update_foo_lock);
// }
// void buggy(void) {
// if (READ_ONCE(shared_foo) == 42)
// WRITE_ONCE(shared_foo, 1); // bug!
// }
//
// @var: variable to assert on
//

//
// ASSERT_EXCLUSIVE_ACCESS - assert no concurrent accesses to @var
//
// Assert that there are no concurrent accesses to @var (no readers nor
// writers). This assertion can be used to specify properties of concurrent
// code, where violation cannot be detected as a normal data race.
//
// For example, where exclusive access is expected after determining no other
// users of an object are left, but the object is not actually freed. We can
// check that this property actually holds as follows:
//
// .. code-block:: c
//
// if (refcount_dec_and_test(&obj->refcnt)) {
// ASSERT_EXCLUSIVE_ACCESS(*obj);
// do_some_cleanup(obj);
// release_for_reuse(obj);
// }
//
// Note:
//
// 1. ASSERT_EXCLUSIVE_ACCESS_SCOPED(), if applicable, performs more thorough
// checking if a clear scope where no concurrent accesses are expected exists.
//
// 2. For cases where the object is freed, `KASAN <kasan.html>`_ is a better
// fit to detect use-after-free bugs.
//
// @var: variable to assert on
//

//
// ASSERT_EXCLUSIVE_ACCESS_SCOPED - assert no concurrent accesses to @var in scope
//
// Scoped variant of ASSERT_EXCLUSIVE_ACCESS().
//
// Assert that there are no concurrent accesses to @var (no readers nor writers)
// for the entire duration of the scope in which it is introduced. This provides
// a better way to fully cover the enclosing scope, compared to multiple
// ASSERT_EXCLUSIVE_ACCESS(), and increases the likelihood for KCSAN to detect
// racing accesses.
//
// @var: variable to assert on
//

//
// ASSERT_EXCLUSIVE_BITS - assert no concurrent writes to subset of bits in @var
//
// Bit-granular variant of ASSERT_EXCLUSIVE_WRITER().
//
// Assert that there are no concurrent writes to a subset of bits in @var;
// concurrent readers are permitted. This assertion captures more detailed
// bit-level properties, compared to the other (word granularity) assertions.
// Only the bits set in @mask are checked for concurrent modifications, while
// ignoring the remaining bits, i.e. concurrent writes (or reads) to ~mask bits
// are ignored.
//
// Use this for variables, where some bits must not be modified concurrently,
// yet other bits are expected to be modified concurrently.
//
// For example, variables where, after initialization, some bits are read-only,
// but other bits may still be modified concurrently. A reader may wish to
// assert that this is true as follows:
//
// .. code-block:: c
//
// ASSERT_EXCLUSIVE_BITS(flags, READ_ONLY_MASK);
// foo = (READ_ONCE(flags) & READ_ONLY_MASK) >> READ_ONLY_SHIFT;
//
// Note: The access that immediately follows ASSERT_EXCLUSIVE_BITS() is assumed
// to access the masked bits only, and KCSAN optimistically assumes it is
// therefore safe, even in the presence of data races, and marking it with
// READ_ONCE() is optional from KCSAN's point-of-view. We caution, however, that
// it may still be advisable to do so, since we cannot reason about all compiler
// optimizations when it comes to bit manipulations (on the reader and writer
// side). If you are sure nothing can go wrong, we can write the above simply
// as:
//
// .. code-block:: c
//
// ASSERT_EXCLUSIVE_BITS(flags, READ_ONLY_MASK);
// foo = (flags & READ_ONLY_MASK) >> READ_ONLY_SHIFT;
//
// Another example, where this may be used, is when certain bits of @var may
// only be modified when holding the appropriate lock, but other bits may still
// be modified concurrently. Writers, where other bits may change concurrently,
// could use the assertion as follows:
//
// .. code-block:: c
//
// spin_lock(&foo_lock);
// ASSERT_EXCLUSIVE_BITS(flags, FOO_MASK);
// old_flags = flags;
// new_flags = (old_flags & ~FOO_MASK) | (new_foo << FOO_SHIFT);
// if (cmpxchg(&flags, old_flags, new_flags) != old_flags) { ... }
// spin_unlock(&foo_lock);
//
// @var: variable to assert on
// @mask: only check for modifications to bits set in @mask
//

    pub \: kcsan_set_access_mask(mask);,
    pub KCSAN_ACCESS_ASSERT);\: __kcsan_check_access(&(var), sizeof(var),,
    pub \: kcsan_set_access_mask(0);,
    pub \: kcsan_atomic_next(1);,
