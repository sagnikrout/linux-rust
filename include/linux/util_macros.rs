//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/util_macros.h
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
// for_each_if - helper for handling conditionals in various for_each macros
// @condition: The condition to check
//
// Typical use::
//
// #define for_each_foo_bar(x, y) \'
// list_for_each_entry(x, y->list, head) \'
// for_each_if(x->something == SOMETHING)
//
// The for_each_if() macro makes the use of for_each_foo_bar() less error
// prone.
//

//
// find_closest - locate the closest element in a sorted array
// @x: The reference value.
// @a: The array in which to look for the closest element. Must be sorted
// in ascending order.
// @as: Size of 'a'.
//
// Returns the index of the element closest to 'x'.
// Note: If using an array of negative numbers (or mixed positive numbers),
// then be sure that 'x' is of a signed-type to get good results.
//

//
// find_closest_descending - locate the closest element in a sorted array
// @x: The reference value.
// @a: The array in which to look for the closest element. Must be sorted
// in descending order.
// @as: Size of 'a'.
//
// Similar to find_closest() but 'a' is expected to be sorted in descending
// order. The iteration is done in reverse order, so that the comparison
// of '__fc_right' & '__fc_left' also works for unsigned numbers.
//

//
// PTR_IF - evaluate to @ptr if @cond is true, or to NULL otherwise.
// @cond: A conditional, usually in a form of IS_ENABLED(CONFIG_FOO)
// @ptr: A pointer to assign if @cond is true.
//
// PTR_IF(IS_ENABLED(CONFIG_FOO), ptr) evaluates to @ptr if CONFIG_FOO is set
// to 'y' or 'm', or to NULL otherwise. The @ptr argument must be a pointer.
//
// The macro can be very useful to help compiler dropping dead code.
//
// For instance, consider the following::
//
// #ifdef CONFIG_FOO_SUSPEND
// static int foo_suspend(struct device *dev)
// {
// ...
// }
// #endif
//
// static struct pm_ops foo_ops = {
// #ifdef CONFIG_FOO_SUSPEND
// .suspend = foo_suspend,
// #endif
// };
//
// While this works, the foo_suspend() macro is compiled conditionally,
// only when CONFIG_FOO_SUSPEND is set. This is problematic, as there could
// be a build bug in this function, we wouldn't have a way to know unless
// the configuration option is set.
//
// An alternative is to declare foo_suspend() always, but mark it
// as __maybe_unused. This works, but the __maybe_unused attribute
// is required to instruct the compiler that the function may not
// be referenced anywhere, and is safe to remove without making
// a fuss about it. This makes the programmer responsible for tagging
// the functions that can be garbage-collected.
//
// With the macro it is possible to write the following::
//
// static int foo_suspend(struct device *dev)
// {
// ...
// }
//
// static struct pm_ops foo_ops = {
// .suspend = PTR_IF(IS_ENABLED(CONFIG_FOO_SUSPEND), foo_suspend),
// };
//
// The foo_suspend() function will now be automatically dropped by the
// compiler, and it does not require any specific attribute.
//

//
// u64_to_user_ptr - cast a pointer passed as u64 from user space to void __user
// @x: The u64 value from user space, usually via IOCTL
//
// u64_to_user_ptr() simply casts a pointer passed as u64 from user space to void
// __user * correctly. Using this lets us get rid of all the tiresome casts.
//

//
// is_insidevar - check if the @ptr points inside the @var memory range.
// @ptr:	the pointer to a memory address.
// @var:	the variable which address and size identify the memory range.
//
// Evaluates to true if the address in @ptr lies within the memory
// range allocated to @var.
//

