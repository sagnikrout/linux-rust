//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/overflow.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT

//
// We need to compute the minimum and maximum values representable in a given
// type. These macros may also be useful elsewhere. It would seem more obvious
// to do something like:
//
// #define type_min(T) (T)(is_signed_type(T) ? (T)1 << (8*sizeof(T)-1) : 0)
// #define type_max(T) (T)(is_signed_type(T) ? ((T)1 << (8*sizeof(T)-1)) - 1 : ~(T)0)
//
// Unfortunately, the middle expressions, strictly speaking, have
// undefined behaviour, and at least some versions of gcc warn about
// the type_max expression (but not if -fsanitize=undefined is in
// effect; in that case, the warning is deferred to runtime...).
//
// The slightly excessive casting in type_min is to make sure the
// macros also produce sensible values for the exotic type _Bool. [The
// overflow checkers only almost work for _Bool, but that's
// a-feature-not-a-bug, since people shouldn't be doing arithmetic on
// _Bools. Besides, the gcc builtins don't allow _Bool* as third
// argument.]
//
// Idea stolen from
// https://mail-index.netbsd.org/tech-misc/2007/02/05/0000.html -
// credit to Christian Biere.
//

//
// Allows for effectively applying __must_check to a macro so we can have
// both the type-agnostic benefits of the macros while also being able to
// enforce that the return value is, in fact, checked.
//
extern "C" {
    pub fn unlikely(_arg: overflow) -> return;
}
//
// check_add_overflow() - Calculate addition with overflow checking
// @a: first addend
// @b: second addend
// @d: pointer to store sum
//
// Returns true on wrap-around, false otherwise.
//
// *@d holds the results of the attempted addition, regardless of whether
// wrap-around occurred.
//

//
// wrapping_add() - Intentionally perform a wrapping addition
// @type: type for result of calculation
// @a: first addend
// @b: second addend
//
// Return the potentially wrapped-around addition without
// tripping any wrap-around sanitizers that may be enabled.
//

//
// wrapping_assign_add() - Intentionally perform a wrapping increment assignment
// @var: variable to be incremented
// @offset: amount to add
//
// Increments @var by @offset with wrap-around. Returns the resulting
// value of @var. Will not trip any wrap-around sanitizers.
//
// Returns the new value of @var.
//

// __ptr = wrapping_add(typeof(var), *__ptr, offset);	\
//
// check_sub_overflow() - Calculate subtraction with overflow checking
// @a: minuend; value to subtract from
// @b: subtrahend; value to subtract from @a
// @d: pointer to store difference
//
// Returns true on wrap-around, false otherwise.
//
// *@d holds the results of the attempted subtraction, regardless of whether
// wrap-around occurred.
//

//
// wrapping_sub() - Intentionally perform a wrapping subtraction
// @type: type for result of calculation
// @a: minuend; value to subtract from
// @b: subtrahend; value to subtract from @a
//
// Return the potentially wrapped-around subtraction without
// tripping any wrap-around sanitizers that may be enabled.
//

//
// wrapping_assign_sub() - Intentionally perform a wrapping decrement assign
// @var: variable to be decremented
// @offset: amount to subtract
//
// Decrements @var by @offset with wrap-around. Returns the resulting
// value of @var. Will not trip any wrap-around sanitizers.
//
// Returns the new value of @var.
//

// __ptr = wrapping_sub(typeof(var), *__ptr, offset);	\
//
// check_mul_overflow() - Calculate multiplication with overflow checking
// @a: first factor
// @b: second factor
// @d: pointer to store product
//
// Returns true on wrap-around, false otherwise.
//
// *@d holds the results of the attempted multiplication, regardless of whether
// wrap-around occurred.
//

//
// wrapping_mul() - Intentionally perform a wrapping multiplication
// @type: type for result of calculation
// @a: first factor
// @b: second factor
//
// Return the potentially wrapped-around multiplication without
// tripping any wrap-around sanitizers that may be enabled.
//

//
// check_shl_overflow() - Calculate a left-shifted value and check overflow
// @a: Value to be shifted
// @s: How many bits left to shift
// @d: Pointer to where to store the result
//
// Computes *@d = (@a << @s)
//
// Returns true if '*@d' cannot hold the result or when '@a << @s' doesn't
// make sense. Example conditions:
//
// - '@a << @s' causes bits to be lost when stored in *@d.
// - '@s' is garbage (e.g. negative) or so large that the result of
// '@a << @s' is guaranteed to be 0.
// - '@a' is negative.
// - '@a << @s' sets the sign bit, if any, in '*@d'.
//
// '*@d' will hold the results of the attempted shift, but is not
// considered "safe for use" if true is returned.
//

// _d = (_a_full << _to_shift);					\

//
// overflows_type - helper for checking the overflows between value, variables,
// or data type
//
// @n: source constant value or variable to be checked
// @T: destination variable or data type proposed to store @x
//
// Compares the @x expression for whether or not it can safely fit in
// the storage of the type in @T. @x and @T can have different types.
// If @x is a constant expression, this will also resolve to a constant
// expression.
//
// Returns: true if overflow can occur, false otherwise.
//

//
// range_overflows() - Check if a range is out of bounds
// @start: Start of the range.
// @size:  Size of the range.
// @max:   Exclusive upper boundary.
//
// A strict check to determine if the range [@start, @start + @size) is
// invalid with respect to the allowable range [0, @max). Any range
// starting at or beyond @max is considered an overflow, even if @size is 0.
//
// Returns: true if the range is out of bounds.
//

//
// range_overflows_t() - Check if a range is out of bounds
// @type:  Data type to use.
// @start: Start of the range.
// @size:  Size of the range.
// @max:   Exclusive upper boundary.
//
// Same as range_overflows() but forcing the parameters to @type.
//
// Returns: true if the range is out of bounds.
//

//
// range_end_overflows() - Check if a range's endpoint is out of bounds
// @start: Start of the range.
// @size:  Size of the range.
// @max:   Exclusive upper boundary.
//
// Checks only if the endpoint of a range (@start + @size) exceeds @max.
// Unlike range_overflows(), a zero-sized range at the boundary (@start == @max)
// is not considered an overflow. Useful for iterator-style checks.
//
// Returns: true if the endpoint exceeds the boundary.
//

//
// range_end_overflows_t() - Check if a range's endpoint is out of bounds
// @type:  Data type to use.
// @start: Start of the range.
// @size:  Size of the range.
// @max:   Exclusive upper boundary.
//
// Same as range_end_overflows() but forcing the parameters to @type.
//
// Returns: true if the endpoint exceeds the boundary.
//

//
// castable_to_type - like __same_type(), but also allows for casted literals
//
// @n: variable or constant value
// @T: variable or data type
//
// Unlike the __same_type() macro, this allows a constant value as the
// first argument. If this value would not overflow into an assignment
// of the second argument's type, it returns true. Otherwise, this falls
// back to __same_type().
//

//
// size_mul() - Calculate size_t multiplication with saturation at SIZE_MAX
// @factor1: first factor
// @factor2: second factor
//
// Returns: calculate @factor1 * @factor2, both promoted to size_t,
// with any overflow causing the return value to be SIZE_MAX. The
// lvalue must be size_t to avoid implicit type conversion.
//
// size_add() - Calculate size_t addition with saturation at SIZE_MAX
// @addend1: first addend
// @addend2: second addend
//
// Returns: calculate @addend1 + @addend2, both promoted to size_t,
// with any overflow causing the return value to be SIZE_MAX. The
// lvalue must be size_t to avoid implicit type conversion.
//
// size_sub() - Calculate size_t subtraction with saturation at SIZE_MAX
// @minuend: value to subtract from
// @subtrahend: value to subtract from @minuend
//
// Returns: calculate @minuend - @subtrahend, both promoted to size_t,
// with any overflow causing the return value to be SIZE_MAX. For
// composition with the size_add() and size_mul() helpers, neither
// argument may be SIZE_MAX (or the result with be forced to SIZE_MAX).
// The lvalue must be size_t to avoid implicit type conversion.
//
// array_size() - Calculate size of 2-dimensional array.
// @a: dimension one
// @b: dimension two
//
// Calculates size of 2-dimensional array: @a * @b.
//
// Returns: number of bytes needed to represent the array or SIZE_MAX on
// overflow.
//

//
// array3_size() - Calculate size of 3-dimensional array.
// @a: dimension one
// @b: dimension two
// @c: dimension three
//
// Calculates size of 3-dimensional array: @a * @b * @c.
//
// Returns: number of bytes needed to represent the array or SIZE_MAX on
// overflow.
//

//
// flex_array_size() - Calculate size of a flexible array member
// within an enclosing structure.
// @p: Pointer to the structure.
// @member: Name of the flexible array member.
// @count: Number of elements in the array.
//
// Calculates size of a flexible array of @count number of @member
// elements, at the end of structure @p.
//
// Return: number of bytes needed or SIZE_MAX on overflow.
//

//
// struct_size() - Calculate size of structure with trailing flexible array.
// @p: Pointer to the structure.
// @member: Name of the array member.
// @count: Number of elements in the array.
//
// Calculates size of memory needed for structure of @p followed by an
// array of @count number of @member elements.
//
// Return: number of bytes needed or SIZE_MAX on overflow.
//

//
// struct_size_t() - Calculate size of structure with trailing flexible array
// @type: structure type name.
// @member: Name of the array member.
// @count: Number of elements in the array.
//
// Calculates size of memory needed for structure @type followed by an
// array of @count number of @member elements. Prefer using struct_size()
// when possible instead, to keep calculations associated with a specific
// instance variable of type @type.
//
// Return: number of bytes needed or SIZE_MAX on overflow.
//

//
// struct_offset() - Calculate the offset of a member within a struct
// @p: Pointer to the struct
// @member: Name of the member to get the offset of
//
// Calculates the offset of a particular @member of the structure pointed
// to by @p.
//
// Return: number of bytes to the location of @member.
//

//
// __DEFINE_FLEX() - helper macro for DEFINE_FLEX() family.
// Enables caller macro to pass arbitrary trailing expressions
//
// @type: structure type name, including "struct" keyword.
// @name: Name for a variable to define.
// @member: Name of the array member.
// @count: Number of elements in the array; must be compile-time const.
// @trailer: Trailing expressions for attributes and/or initializers.
//

//
// _DEFINE_FLEX() - helper macro for DEFINE_FLEX() family.
// Enables caller macro to pass (different) initializer.
//
// @type: structure type name, including "struct" keyword.
// @name: Name for a variable to define.
// @member: Name of the array member.
// @count: Number of elements in the array; must be compile-time const.
// @initializer: Initializer expression (e.g., pass `= { }` at minimum).
//

//
// DEFINE_RAW_FLEX() - Define an on-stack instance of structure with a trailing
// flexible array member, when it does not have a __counted_by annotation.
//
// @type: structure type name, including "struct" keyword.
// @name: Name for a variable to define.
// @member: Name of the array member.
// @count: Number of elements in the array; must be compile-time const.
//
// Define a zeroed, on-stack, instance of @type structure with a trailing
// flexible array member.
// Use __struct_size(@name) to get compile-time size of it afterwards.
// Use __member_size(@name->member) to get compile-time size of @name members.
// Use STACK_FLEX_ARRAY_SIZE(@name, @member) to get compile-time number of
// elements in array @member.
//

//
// DEFINE_FLEX() - Define an on-stack instance of structure with a trailing
// flexible array member.
//
// @TYPE: structure type name, including "struct" keyword.
// @NAME: Name for a variable to define.
// @MEMBER: Name of the array member.
// @COUNTER: Name of the __counted_by member.
// @COUNT: Number of elements in the array; must be compile-time const.
//
// Define a zeroed, on-stack, instance of @TYPE structure with a trailing
// flexible array member.
// Use __struct_size(@NAME) to get compile-time size of it afterwards.
// Use __member_size(@NAME->member) to get compile-time size of @NAME members.
// Use STACK_FLEX_ARRAY_SIZE(@name, @member) to get compile-time number of
// elements in array @member.
//

//
// STACK_FLEX_ARRAY_SIZE() - helper macro for DEFINE_FLEX() family.
// Returns the number of elements in @array.
//
// @name: Name for a variable defined in DEFINE_RAW_FLEX()/DEFINE_FLEX().
// @array: Name of the array member.
//

//
// typeof_flex_counter() - Return the type of the counter variable of a given
// flexible array member annotated by __counted_by().
// @FAM: Instance of flexible array member within a given struct.
//
// Returns: "size_t" if no annotation exists.
//

//
// overflows_flex_counter_type() - Check if the counter associated with the
// given flexible array member can represent
// a value.
// @TYPE: Type of the struct that contains the @FAM.
// @FAM: Member name of the FAM within @TYPE.
// @COUNT: Value to check against the __counted_by annotated @FAM's counter.
//
// Returns: true if @COUNT can be represented in the @FAM's counter. When
// @FAM is not annotated with __counted_by(), always returns true.
//

//
// __set_flex_counter() - Set the counter associated with the given flexible
// array member that has been annoated by __counted_by().
// @FAM: Instance of flexible array member within a given struct.
// @COUNT: Value to store to the __counted_by annotated @FAM_PTR's counter.
//
// This is a no-op if no annotation exists. Count needs to be checked with
// overflows_flex_counter_type() before using this function.
//

// _Generic(__flex_counter(FAM),				\
