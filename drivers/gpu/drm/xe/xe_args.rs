//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_args.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2024 Intel Corporation
//

//
// Why don't the following macros have the XE prefix?
//
// Once we find more potential users outside of the Xe driver, we plan to move
// all of the following macros unchanged to linux/args.h.
//
// CALL_ARGS - Invoke a macro, but allow parameters to be expanded beforehand.
// @f: name of the macro to invoke
// @args: arguments for the macro
//
// This macro allows calling macros which names might generated or we want to
// make sure it's arguments will be correctly expanded.
//
// Example:
//
// #define foo	X,Y,Z,Q
// #define bar	COUNT_ARGS(foo)
// #define buz	CALL_ARGS(COUNT_ARGS, foo)
//
// With above definitions bar expands to 1 while buz expands to 4.
//

//
// DROP_FIRST_ARG - Returns all arguments except the first one.
// @args: arguments
//
// This helper macro allows manipulation the argument list before passing it
// to the next level macro.
//
// Example:
//
// #define foo	X,Y,Z,Q
// #define bar	CALL_ARGS(COUNT_ARGS, DROP_FIRST_ARG(foo))
//
// With above definitions bar expands to 3.
//

//
// FIRST_ARG - Returns the first argument.
// @args: arguments
//
// This helper macro allows manipulation the argument list before passing it
// to the next level macro.
//
// Example:
//
// #define foo	X,Y,Z,Q
// #define bar	FIRST_ARG(foo)
//
// With above definitions bar expands to X.
//

//
// LAST_ARG - Returns the last argument.
// @args: arguments
//
// This helper macro allows manipulation the argument list before passing it
// to the next level macro.
//
// Like COUNT_ARGS() this macro works up to 12 arguments.
//
// Example:
//
// #define foo	X,Y,Z,Q
// #define bar	LAST_ARG(foo)
//
// With above definitions bar expands to Q.
//

//
// PICK_ARG - Returns the n-th argument.
// @n: argument number to be returned
// @args: arguments
//
// This helper macro allows manipulation the argument list before passing it
// to the next level macro.
//
// Like COUNT_ARGS() this macro supports n up to 12.
// Specialized macros PICK_ARG1() to PICK_ARG12() are also available.
//
// Example:
//
// #define foo	X,Y,Z,Q
// #define bar	PICK_ARG(2, foo)
// #define buz	PICK_ARG3(foo)
//
// With above definitions bar expands to Y and buz expands to Z.
//

//
// IF_ARGS() - Make selection based on optional argument list.
// @then: token to return if arguments are present
// @else: token to return if arguments are empty
// @...: arguments to check (optional)
//
// This macro allows to select a token based on the presence of the argument list.
//
// Example:
//
// #define foo	X, Y
// #define bar	IF_ARGS(Z, Q, foo)
// #define buz	IF_ARGS(Z, Q, DROP_FIRST_ARG(FIRST_ARG(foo)))
//
// With above definitions bar expands to Z while buz expands to Q.
//

//
// ARGS_SEP_COMMA - Definition of a comma character.
//
// This definition can be used in cases where any intermediate macro expects
// fixed number of arguments, but we want to pass more arguments which can
// be properly evaluated only by the next level macro.
//
// Example:
//
// #define foo(f)	f(X) f(Y) f(Z) f(Q)
// #define bar	DROP_FIRST_ARG(foo(ARGS_SEP_COMMA __stringify))
// #define buz	CALL_ARGS(COUNT_ARGS, DROP_FIRST_ARG(foo(ARGS_SEP_COMMA)))
//
// With above definitions bar expands to
// "X", "Y", "Z", "Q"
// and buz expands to 4.
//

