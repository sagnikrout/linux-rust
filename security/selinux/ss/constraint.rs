//! Automatically rewritten from C Header to Rust Module
//! Source: security/selinux/ss/constraint.h
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
// A constraint is a condition that must be satisfied in
// order for one or more permissions to be granted.
// Constraints are used to impose additional restrictions
// beyond the type-based rules in `te' or the role-based
// transition rules in `rbac'.  Constraints are typically
// used to prevent a process from transitioning to a new user
// identity or role unless it is in a privileged type.
// Constraints are likewise typically used to prevent a
// process from labeling an object with a different user
// identity.
//
// Author : Stephen Smalley, <stephen.smalley.work@gmail.com>
//

pub const CEXPR_MAXDEPTH: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct constraint_expr {

    pub /: *mut *mut u32 expr_type; / expression type,

    pub /: *mut *mut u32 attr; / attribute,

    pub /: *mut *mut u32 op; / operator,
    pub /: *mut *mut ebitmap names; / names,
// internally unused, only forwarded via policydb_write()
    pub type_names: *mut type_set,
    pub /: *mut *mut *mut constraint_expr next; / next expression,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct constraint_node {
    pub /: *mut *mut u32 permissions; / constrained permissions,
    pub /: *mut *mut *mut constraint_expr expr; / constraint on permissions,
    pub /: *mut *mut *mut constraint_node next; / next constraint,
}
