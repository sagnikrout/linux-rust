//! Automatically rewritten from C Header to Rust Module
//! Source: security/selinux/ss/conditional.h
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
// Authors: Karl MacMillan <kmacmillan@tresys.com>
// Frank Mayer <mayerf@tresys.com>
// Copyright (C) 2003 - 2004 Tresys Technology, LLC
//

pub const COND_EXPR_MAXDEPTH: c_int = 10;
//
// A conditional expression is a list of operators and operands
// in reverse polish notation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cond_expr_node {

    pub expr_type: u32,
    pub boolean: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cond_expr {
    pub nodes: *mut cond_expr_node,
    pub len: u32,
}

//
// Each cond_node contains a list of rules to be enabled/disabled
// depending on the current value of the conditional expression. This
// struct is for that list.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cond_av_list {
    pub nodes: *mut avtab_node,
    pub len: u32,
}

//
// A cond node represents a conditional block in a policy. It
// contains a conditional expression, the current state of the expression,
// two lists of rules to enable/disable depending on the value of the
// expression (the true list corresponds to if and the false list corresponds
// to else)..
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cond_node {
    pub cur_state: c_int,
    pub expr: cond_expr,
    pub true_list: cond_av_list,
    pub false_list: cond_av_list,
}

extern "C" {
    pub fn cond_policydb_init(p: *mut policydb);
}
extern "C" {
    pub fn cond_policydb_destroy(p: *mut policydb);
}
extern "C" {
    pub fn cond_init_bool_indexes(p: *mut policydb) -> c_int;
}
extern "C" {
    pub fn cond_destroy_bool(key: *mut c_void, datum: *mut c_void, p: *mut c_void) -> c_int;
}
extern "C" {
    pub fn cond_index_bool(key: *mut c_void, datum: *mut c_void, datap: *mut c_void) -> c_int;
}
extern "C" {
    pub fn cond_read_bool(p: *mut policydb, s: *mut symtab, fp: *mut policy_file) -> c_int;
}
extern "C" {
    pub fn cond_read_list(p: *mut policydb, fp: *mut policy_file) -> c_int;
}
extern "C" {
    pub fn cond_write_bool(key: *mut c_void, datum: *mut c_void, ptr: *mut c_void) -> c_int;
}
extern "C" {
    pub fn cond_write_list(p: *mut policydb, fp: *mut policy_file) -> c_int;
}
extern "C" {
    pub fn evaluate_cond_nodes(p: *mut policydb);
}
extern "C" {
    pub fn cond_policydb_destroy_dup(p: *mut policydb);
}
extern "C" {
    pub fn cond_policydb_dup(new: *mut policydb, orig: *const policydb) -> c_int;
}
