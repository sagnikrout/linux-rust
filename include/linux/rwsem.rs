//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rwsem.h
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
// rwsem.h: R/W semaphores, public interface
//
// Written by David Howells (dhowells@redhat.com).
// Derived from asm-i386/semaphore.h
//

//
// For an uncontended rwsem, count and owner are the only fields a task
// needs to touch when acquiring the rwsem. So they are put next to each
// other to increase the chance that they will share the same cacheline.
//
// In a contended rwsem, the owner is likely the most frequently accessed
// field in the structure as the optimistic waiter that holds the osq lock
// will spin on owner. For an embedded rwsem, other hot fields in the
// containing structure should be moved further away from the rwsem to
// reduce the chance that they will share the same cacheline causing
// cacheline bouncing problem.
//
// Write owner or one of the read owners as well flags regarding
// the current state of the rwsem. Can be used as a speculative
// check to see if the write owner is running on the cpu.
//

extern "C" {
    pub fn __guarded_by(_arg: &wait_lock) -> *mut rwsem_waiter first_waiter;
}

// Common initializer macros and functions

// Macro flag: #define __RWSEM_OPT_INIT(lockname)

//
// This is the same regardless of which rwsem implementation that is being used.
// It is just a heuristic meant to be called by somebody already holding the
// rwsem to see if somebody from an incompatible type is wanting access to the
// lock.
//
extern "C" {
    pub fn data_race(NULL: sem->first_waiter !=) -> return;
}

//
// Return just the real task structure pointer of the owner
//
// Return true if the rwsem is owned by a reader.
//
extern "C" {
    pub fn is_rwsem_reader_owned(sem: *mut rw_semaphore) -> bool;
}

extern "C" {
    pub fn rw_base_is_locked(_arg: &sem->rwbase) -> return;
}
extern "C" {
    pub fn rw_base_is_contended(_arg: &sem->rwbase) -> return;
}

//
// The functions below are the same for all rwsem implementations including
// the RT specific variant.
//
// lock for reading
//
extern "C" {
    pub fn down_read(__acquires_shared(sem: *mut *mut rw_semaphore sem));
}
extern "C" {
    pub fn down_read_interruptible(__cond_acquires_shared(0: *mut *mut rw_semaphore sem), _arg: sem) -> int __must_check;
}
extern "C" {
    pub fn down_read_killable(__cond_acquires_shared(0: *mut *mut rw_semaphore sem), _arg: sem) -> int __must_check;
}
//
// trylock for reading -- returns 1 if successful, 0 if contention
//
extern "C" {
    pub fn down_read_trylock(__cond_acquires_shared(true: *mut *mut rw_semaphore sem), _arg: sem) -> c_int;
}
//
// lock for writing
//
extern "C" {
    pub fn down_write(__acquires(sem: *mut *mut rw_semaphore sem));
}
extern "C" {
    pub fn down_write_killable(__cond_acquires(0: *mut *mut rw_semaphore sem), _arg: sem) -> int __must_check;
}
//
// trylock for writing -- returns 1 if successful, 0 if contention
//
extern "C" {
    pub fn down_write_trylock(__cond_acquires(true: *mut *mut rw_semaphore sem), _arg: sem) -> c_int;
}
//
// release a read lock
//
extern "C" {
    pub fn up_read(__releases_shared(sem: *mut *mut rw_semaphore sem));
}
//
// release a write lock
//
extern "C" {
    pub fn up_write(__releases(sem: *mut *mut rw_semaphore sem));
}

//
// downgrade write lock to read lock
//
extern "C" {
    pub fn downgrade_write(__acquires_shared(sem: *mut *mut rw_semaphore sem) __releases(sem));
}

//
// nested locking. NOTE: rwsems are not allowed to recurse
// (which occurs if the same task tries to acquire the same
// lock instance multiple times), but multiple locks of the
// same lock class might be taken, if the order of the locks
// is always the same. This ordering rule can be expressed
// to lockdep via the _nested() APIs, but enumerating the
// subclasses that are used. (If the nesting relationship is
// static then another method for expressing nested locking is
// the explicit definition of lock class keys and the use of
// lockdep_set_class() at lock initialization time.
// See Documentation/locking/lockdep-design.rst for more details.)
//
extern "C" {
    pub fn down_read_nested(sem: *mut rw_semaphore, __acquires_shared(sem: int subclass));
}
extern "C" {
    pub fn down_read_killable_nested(sem: *mut rw_semaphore, __cond_acquires_shared(0: int subclass), _arg: sem) -> int __must_check;
}
extern "C" {
    pub fn down_write_nested(sem: *mut rw_semaphore, __acquires(sem: int subclass));
}
extern "C" {
    pub fn down_write_killable_nested(sem: *mut rw_semaphore, __cond_acquires(0: int subclass), _arg: sem) -> c_int;
}
extern "C" {
    pub fn _down_write_nest_lock(sem: *mut rw_semaphore, __acquires(sem: *mut *mut lockdep_map nest_lock));
}

//
// Take/release a lock when not the owner will release it.
//
// [ This API should be avoided as much as possible - the
// proper abstraction for this case is completions. ]
//
extern "C" {
    pub fn down_read_non_owner(__acquires_shared(sem: *mut *mut rw_semaphore sem));
}
extern "C" {
    pub fn up_read_non_owner(__releases_shared(sem: *mut *mut rw_semaphore sem));
}

