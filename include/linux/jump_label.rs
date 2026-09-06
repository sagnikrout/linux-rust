//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/jump_label.h
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
// Jump label support
//
// Copyright (C) 2009-2012 Jason Baron <jbaron@redhat.com>
// Copyright (C) 2011-2012 Red Hat, Inc., Peter Zijlstra
//
// DEPRECATED API:
//
// The use of 'struct static_key' directly, is now DEPRECATED. In addition
// static_key_{true,false}() is also DEPRECATED. IE DO NOT use the following:
//
// struct static_key false = STATIC_KEY_INIT_FALSE;
// struct static_key true = STATIC_KEY_INIT_TRUE;
// static_key_true()
// static_key_false()
//
// The updated API replacements are:
//
// DEFINE_STATIC_KEY_TRUE(key);
// DEFINE_STATIC_KEY_FALSE(key);
// DEFINE_STATIC_KEY_ARRAY_TRUE(keys, count);
// DEFINE_STATIC_KEY_ARRAY_FALSE(keys, count);
// static_branch_likely()
// static_branch_unlikely()
//
// Jump labels provide an interface to generate dynamic branches using
// self-modifying code. Assuming toolchain and architecture support, if we
// define a "key" that is initially false via "DEFINE_STATIC_KEY_FALSE(key)",
// an "if (static_branch_unlikely(&key))" statement is an unconditional branch
// (which defaults to false - and the true block is placed out of line).
// Similarly, we can define an initially true key via
// "DEFINE_STATIC_KEY_TRUE(key)", and use it in the same
// "if (static_branch_unlikely(&key))", in which case we will generate an
// unconditional branch to the out-of-line true branch. Keys that are
// initially true or false can be using in both static_branch_unlikely()
// and static_branch_likely() statements.
//
// At runtime we can change the branch target by setting the key
// to true via a call to static_branch_enable(), or false using
// static_branch_disable(). If the direction of the branch is switched by
// these calls then we run-time modify the branch target via a
// no-op -> jump or jump -> no-op conversion. For example, for an
// initially false key that is used in an "if (static_branch_unlikely(&key))"
// statement, setting the key to true requires us to patch in a jump
// to the out-of-line of true branch.
//
// In addition to static_branch_{enable,disable}, we can also reference count
// the key or branch direction via static_branch_{inc,dec}. Thus,
// static_branch_inc() can be thought of as a 'make more true' and
// static_branch_dec() as a 'make more false'.
//
// Since this relies on modifying code, the branch modifying functions
// must be considered absolute slow paths (machine wide synchronization etc.).
// OTOH, since the affected branches are unconditional, their runtime overhead
// will be absolutely minimal, esp. in the default (off) case where the total
// effect is a single NOP of appropriate size. The on case will patch in a jump
// to the out-of-line block.
//
// When the control is directly exposed to userspace, it is prudent to delay the
// decrement to avoid high frequency code modifications which can (and do)
// cause significant performance degradation. Struct static_key_deferred and
// static_key_slow_dec_deferred() provide for this.
//
// Lacking toolchain and or architecture support, static keys fall back to a
// simple conditional branch.
//
// Additional babbling in: Documentation/staging/static-keys.rst
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct static_key {
    pub enabled: core::sync::atomic::AtomicI32,

//
// bit 0 => 1 if key is initially true
// 0 if initially false
// bit 1 => 1 if points to struct static_key_mod
// 0 if points to struct jump_entry
//
    pub type: c_ulong,
    pub entries: *mut jump_entry,
    pub next: *mut static_key_mod,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jump_entry {
    pub code: i32,
    pub target: i32,
    pub KASLR: long key; // key may be far away from the core kernel under,
}

extern "C" {
    pub fn arch_jump_entry_size(_arg: entry) -> return;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum jump_label_type {
    JUMP_LABEL_NOP = 0,
    JUMP_LABEL_JMP,
}

extern "C" {
    pub fn arch_static_branch(_arg: key, _arg: false) -> return;
}
extern "C" {
    pub fn jump_label_init();
}
extern "C" {
    pub fn jump_label_init_ro();
}
extern "C" {
    pub fn jump_label_lock();
}
extern "C" {
    pub fn jump_label_unlock();
}
extern "C" {
    pub fn arch_jump_label_transform_apply();
}
extern "C" {
    pub fn jump_label_text_reserved(start: *mut c_void, end: *mut c_void) -> c_int;
}
extern "C" {
    pub fn static_key_slow_inc(key: *mut static_key) -> bool;
}
extern "C" {
    pub fn static_key_fast_inc_not_disabled(key: *mut static_key) -> bool;
}
extern "C" {
    pub fn static_key_slow_dec(key: *mut static_key);
}
extern "C" {
    pub fn static_key_slow_inc_cpuslocked(key: *mut static_key) -> bool;
}
extern "C" {
    pub fn static_key_slow_dec_cpuslocked(key: *mut static_key);
}
extern "C" {
    pub fn static_key_count(key: *mut static_key) -> c_int;
}
extern "C" {
    pub fn static_key_enable(key: *mut static_key);
}
extern "C" {
    pub fn static_key_disable(key: *mut static_key);
}
extern "C" {
    pub fn static_key_enable_cpuslocked(key: *mut static_key);
}
extern "C" {
    pub fn static_key_disable_cpuslocked(key: *mut static_key);
}
extern "C" {
    pub fn jump_label_init_type(entry: *mut jump_entry) -> jump_label_type;
}

extern "C" {
    pub fn raw_atomic_read(_arg: &key->enabled) -> return;
}
//
// Prevent key->enabled getting negative to follow the same semantics
// as for CONFIG_JUMP_LABEL=y, see kernel/jump_label.c comment.
//

// --------------------------------------------------------------------------
//
// Two type wrappers around static_key, such that we can use compile time
// type differentiation to emit the right code.
//
// All the below code is macros in order to play type games.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct static_key_true {
    pub key: static_key,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct static_key_false {
    pub key: static_key,
}

extern "C" {
    pub fn ____wrong_branch_error() -> bool;
}

//
// Combine the right initial value (type) with the right branch order
// to generate the desired result.
//
// type\branch|	likely (1)	      |	unlikely (0)
// -----------+-----------------------+------------------
// |                       |
// true (1)  |	   ...		      |	   ...
// |    NOP		      |	   JMP L
// |    <br-stmts>	      |	1: ...
// |	L: ...		      |
// |			      |
// |			      |	L: <br-stmts>
// |			      |	   jmp 1b
// |                       |
// -----------+-----------------------+------------------
// |                       |
// false (0) |	   ...		      |	   ...
// |    JMP L	      |	   NOP
// |    <br-stmts>	      |	1: ...
// |	L: ...		      |
// |			      |
// |			      |	L: <br-stmts>
// |			      |	   jmp 1b
// |                       |
// -----------+-----------------------+------------------
//
// The initial value is encoded in the LSB of static_key::entries,
// type: 0 = false, 1 = true.
//
// The branch type is encoded in the LSB of jump_entry::key,
// branch: 0 = unlikely, 1 = likely.
//
// This gives the following logic table:
//
// enabled	type	branch	  instuction
// -----------------------------+-----------
// 0	0	0	| NOP
// 0	0	1	| JMP
// 0	1	0	| NOP
// 0	1	1	| JMP
//
// 1	0	0	| JMP
// 1	0	1	| NOP
// 1	1	0	| JMP
// 1	1	1	| NOP
//
// Which gives the following functions:
//
// dynamic: instruction = enabled ^ branch
// static:  instruction = type ^ branch
//
// See jump_label_type() / jump_label_init_type().
//

//
// Advanced usage; refcount, branch is enabled when: count != 0
//

//
// Normal usage; boolean enable/disable.
//

