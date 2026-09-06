//! Automatically rewritten from C Header to Rust Module
//! Source: include/trace/syscall.h
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
// A syscall entry in the ftrace syscalls array.
//
// @name: name of the syscall
// @syscall_nr: number of the syscall
// @nb_args: number of parameters it takes
// @user_arg_is_str: set if the arg for @user_arg_size is a string
// @user_arg_size: holds @arg that has size of the user space to read
// @user_mask: mask of @args that will read user space
// @types: list of types as strings
// @args: list of args as strings (args[i] matches types[i])
// @enter_fields: list of fields for syscall_enter trace event
// @enter_event: associated syscall_enter trace event
// @exit_event: associated syscall_exit trace event
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct syscall_metadata {
    pub name: *const c_char,
    pub syscall_nr: c_int,
    pub nb_args:7: u8,
    pub user_arg_is_str:1: u8,
    pub user_arg_size: i8,
    pub user_mask: c_short,
    pub types: *const c_char,
    pub args: *const c_char,
    pub enter_fields: list_head,
    pub enter_event: *mut trace_event_call,
    pub exit_event: *mut trace_event_call,
}

