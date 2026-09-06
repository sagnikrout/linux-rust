//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/minmax.h
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
// min()/max()/clamp() macros must accomplish several things:
//
// - Avoid multiple evaluations of the arguments (so side-effects like
// "x++" happen only once) when non-constant.
// - Perform signed v unsigned type-checking (to generate compile
// errors instead of nasty runtime surprises).
// - Unsigned char/short are always promoted to signed int and can be
// compared against signed or unsigned arguments.
// - Unsigned arguments can be compared against non-negative signed constants.
// - Comparison of a signed argument against an unsigned constant fails
// even if the constant is below __INT_MAX__ and could be cast to int.
//

//
// __sign_use for integer expressions:
// bit #0 set if ok for unsigned comparisons
// bit #1 set if ok for signed comparisons
//
// In particular, statically non-negative signed integer expressions
// are ok for both.
//
// NOTE! Unsigned types smaller than 'int' are implicitly converted to 'int'
// in expressions, and are accepted for signed conversions for now.
// This is debatable.
//
// Note that 'x' is the original expression, and 'ux' is the unique variable
// that contains the value.
//
// We use 'ux' for pure type checking, and 'x' for when we need to look at the
// value (but without evaluating it for side effects!
// Careful to only ever evaluate it with sizeof() or __builtin_constant_p() etc).
//
// Pointers end up being checked by the normal C type rules at the actual
// comparison, and these expressions only need to be careful to not cause
// warnings for pointer use.
//

//
// Check whether a signed value is always non-negative.
//
// A cast is needed to avoid any warnings from values that aren't signed
// integer types (in which case the result doesn't matter).
//
// On 64-bit any integer or pointer type can safely be cast to 'long long'.
// But on 32-bit we need to avoid warnings about casting pointers to integers
// of different sizes without truncating 64-bit values so 'long' or 'long long'
// must be used depending on the size of the value.
//
// This does not work for 128-bit signed integers since the cast would truncate
// them, but we do not use s128 types in the kernel (we do use 'u128',
// but they are handled by the !is_signed_type() case).
//

//
// min - return minimum of two values of the same or compatible types
// @x: first value
// @y: second value
//

//
// max - return maximum of two values of the same or compatible types
// @x: first value
// @y: second value
//

//
// umin - return minimum of two non-negative values
// Signed types are zero extended to match a larger unsigned type.
// @x: first value
// @y: second value
//

//
// umax - return maximum of two non-negative values
// @x: first value
// @y: second value
//

//
// min3 - return minimum of three values
// @x: first value
// @y: second value
// @z: third value
//

//
// max3 - return maximum of three values
// @x: first value
// @y: second value
// @z: third value
//

//
// min_t - return minimum of two values, using the specified type
// @type: data type to use
// @x: first value
// @y: second value
//

//
// max_t - return maximum of two values, using the specified type
// @type: data type to use
// @x: first value
// @y: second value
//

//
// min_not_zero - return the minimum that is _not_ zero, unless both are zero
// @x: value1
// @y: value2
//

//
// clamp - return a value clamped to a given range with typechecking
// @val: current value
// @lo: lowest allowable value
// @hi: highest allowable value
//
// This macro checks @val/@lo/@hi to make sure they have compatible
// signedness.
//

//
// clamp_t - return a value clamped to a given range using a given type
// @type: the type of variable to use
// @val: current value
// @lo: minimum allowable value
// @hi: maximum allowable value
//
// This macro does no typechecking and uses temporary variables of type
// @type to make all the comparisons.
//

//
// clamp_val - return a value clamped to a given range using val's type
// @val: current value
// @lo: minimum allowable value
// @hi: maximum allowable value
//
// This macro does no typechecking and uses temporary variables of whatever
// type the input argument @val is.  This is useful when @val is an unsigned
// type and @lo and @hi are literals that will otherwise be assigned a signed
// integer type.
//

//
// Do not check the array parameter using __must_be_array().
// In the following legit use-case where the "array" passed is a simple pointer,
// __must_be_array() will return a failure.
// --- 8< ---
// int *buff
// ...
// min = min_array(buff, nb_items);
// --- 8< ---
//
// The first typeof(&(array)[0]) is needed in order to support arrays of both
// 'int *buff' and 'int buff[N]' types.
//
// The array can be an array of const items.
// typeof() keeps the const qualifier. Use __unqual_scalar_typeof() in order
// to discard the const qualifier for the __element variable.
//

//
// min_array - return minimum of values present in an array
// @array: array
// @len: array length
//
// Note that @len must not be zero (empty array).
//

//
// max_array - return maximum of values present in an array
// @array: array
// @len: array length
//
// Note that @len must not be zero (empty array).
//

//
// in_range - Determine if a value lies within a range.
// @val: Value to test.
// @start: First value in range.
// @len: Number of values in range.
//
// This is more efficient than "if (start <= val && val < (start + len))".
// It also gives a different answer if @start + @len overflows the size of
// the type by a sufficient amount to encompass @val.  Decide for yourself
// which behaviour you want, or prove that start + len never overflow.
// Do not blindly replace one form with the other.
//

//
// swap - swap values of @a and @b
// @a: first value
// @b: second value
//

//
// Use these carefully: no type checking, and uses the arguments
// multiple times. Use for obvious constants only.
//

