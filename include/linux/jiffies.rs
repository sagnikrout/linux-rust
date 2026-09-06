//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/jiffies.h
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
// The following defines establish the engineering parameters of the PLL
// model. The HZ variable establishes the timer interrupt frequency, 100 Hz
// for the SunOS kernel, 256 Hz for the Ultrix kernel and 1024 Hz for the
// OSF/1 kernel. The SHIFT_HZ define expresses the same value as the
// nearest power of two in order to avoid hardware multiply operations.
//

// Suppose we want to divide two numbers NOM and DEN: NOM/DEN, then we can
// improve accuracy by shifting LSH bits, hence calculating:
// (NOM << LSH) / DEN
// This however means trouble for large NOM, because (NOM << LSH) may no
// longer fit in 32 bits. The following way of calculating this gives us
// some slack, under the following conditions:
// - (NOM / DEN) fits in (32 - LSH) bits.
// - (NOM % DEN) fits in (32 - LSH) bits.
//

// LATCH is used in the interval timer and ftape setup.

extern "C" {
    pub fn register_refined_jiffies(clock_tick_rate: c_long);
}
// TICK_USEC is the time between ticks in usec

// USER_TICK_USEC is the time between ticks in usec assuming fake USER_HZ

//
// The 64-bit value is not atomic on 32-bit systems - you MUST NOT read it
// without sampling the sequence number in jiffies_lock.
// get_jiffies_64() will do this for you as appropriate.
//
// jiffies and jiffies_64 are at the same address for little-endian systems
// and for 64-bit big-endian systems.
// On 32-bit big-endian systems, jiffies is the lower 32 bits of jiffies_64
// (i.e., at address @jiffies_64 + 4).
// See arch/ARCH/kernel/vmlinux.lds.S
//

extern "C" {
    pub fn get_jiffies_64() -> u64;
}

//
// get_jiffies_64 - read the 64-bit non-atomic jiffies_64 value
//
// When BITS_PER_LONG < 64, this uses sequence number sampling using
// jiffies_lock to protect the 64-bit read.
//
// Return: current 64-bit jiffies value
//

//
// DOC: General information about time_* inlines
//
// These inlines deal with timer wrapping correctly. You are strongly encouraged
// to use them:
//
// #. Because people otherwise forget
// #. Because if the timer wrap changes in future you won't have to alter your
// driver code.
//
// time_after - returns true if the time a is after time b.
// @a: first comparable as unsigned long
// @b: second comparable as unsigned long
//
// Do this with "<0" and ">=0" to only test the sign of the result. A
// good compiler would generate better code (and a really good compiler
// wouldn't care). Gcc is currently neither.
//
// Return: %true is time a is after time b, otherwise %false.
//

//
// time_before - returns true if the time a is before time b.
// @a: first comparable as unsigned long
// @b: second comparable as unsigned long
//
// Return: %true is time a is before time b, otherwise %false.
//

//
// time_after_eq - returns true if the time a is after or the same as time b.
// @a: first comparable as unsigned long
// @b: second comparable as unsigned long
//
// Return: %true is time a is after or the same as time b, otherwise %false.
//

//
// time_before_eq - returns true if the time a is before or the same as time b.
// @a: first comparable as unsigned long
// @b: second comparable as unsigned long
//
// Return: %true is time a is before or the same as time b, otherwise %false.
//

//
// time_in_range - Calculate whether a is in the range of [b, c].
// @a: time to test
// @b: beginning of the range
// @c: end of the range
//
// Return: %true is time a is in the range [b, c], otherwise %false.
//

//
// time_in_range_open - Calculate whether a is in the range of [b, c).
// @a: time to test
// @b: beginning of the range
// @c: end of the range
//
// Return: %true is time a is in the range [b, c), otherwise %false.
//

// Same as above, but does so with platform independent 64bit types.
// These must be used when utilizing jiffies_64 (i.e. return value of
// get_jiffies_64()).
//
// time_after64 - returns true if the time a is after time b.
// @a: first comparable as __u64
// @b: second comparable as __u64
//
// This must be used when utilizing jiffies_64 (i.e. return value of
// get_jiffies_64()).
//
// Return: %true is time a is after time b, otherwise %false.
//

//
// time_before64 - returns true if the time a is before time b.
// @a: first comparable as __u64
// @b: second comparable as __u64
//
// This must be used when utilizing jiffies_64 (i.e. return value of
// get_jiffies_64()).
//
// Return: %true is time a is before time b, otherwise %false.
//

//
// time_after_eq64 - returns true if the time a is after or the same as time b.
// @a: first comparable as __u64
// @b: second comparable as __u64
//
// This must be used when utilizing jiffies_64 (i.e. return value of
// get_jiffies_64()).
//
// Return: %true is time a is after or the same as time b, otherwise %false.
//

//
// time_before_eq64 - returns true if the time a is before or the same as time b.
// @a: first comparable as __u64
// @b: second comparable as __u64
//
// This must be used when utilizing jiffies_64 (i.e. return value of
// get_jiffies_64()).
//
// Return: %true is time a is before or the same as time b, otherwise %false.
//

//
// time_in_range64 - Calculate whether a is in the range of [b, c].
// @a: time to test
// @b: beginning of the range
// @c: end of the range
//
// Return: %true is time a is in the range [b, c], otherwise %false.
//

//
// These eight macros compare jiffies[_64] and 'a' for convenience.
//
// time_is_before_jiffies - return true if a is before jiffies
// @a: time (unsigned long) to compare to jiffies
//
// Return: %true is time a is before jiffies, otherwise %false.
//

//
// time_is_before_jiffies64 - return true if a is before jiffies_64
// @a: time (__u64) to compare to jiffies_64
//
// Return: %true is time a is before jiffies_64, otherwise %false.
//

//
// time_is_after_jiffies - return true if a is after jiffies
// @a: time (unsigned long) to compare to jiffies
//
// Return: %true is time a is after jiffies, otherwise %false.
//

//
// time_is_after_jiffies64 - return true if a is after jiffies_64
// @a: time (__u64) to compare to jiffies_64
//
// Return: %true is time a is after jiffies_64, otherwise %false.
//

//
// time_is_before_eq_jiffies - return true if a is before or equal to jiffies
// @a: time (unsigned long) to compare to jiffies
//
// Return: %true is time a is before or the same as jiffies, otherwise %false.
//

//
// time_is_before_eq_jiffies64 - return true if a is before or equal to jiffies_64
// @a: time (__u64) to compare to jiffies_64
//
// Return: %true is time a is before or the same jiffies_64, otherwise %false.
//

//
// time_is_after_eq_jiffies - return true if a is after or equal to jiffies
// @a: time (unsigned long) to compare to jiffies
//
// Return: %true is time a is after or the same as jiffies, otherwise %false.
//

//
// time_is_after_eq_jiffies64 - return true if a is after or equal to jiffies_64
// @a: time (__u64) to compare to jiffies_64
//
// Return: %true is time a is after or the same as jiffies_64, otherwise %false.
//

//
// Have the 32-bit jiffies value wrap 5 minutes after boot
// so jiffies wrap bugs show up earlier.
//

//
// Change timeval to jiffies, trying to avoid the
// most obvious overflows..
//
// And some not so obvious.
//
// Note that we don't want to return LONG_MAX, because
// for various timeout reasons we often end up having
// to wait "jiffies+1" in order to guarantee that we wait
// at _least_ "jiffies" - so "jiffies+1" had better still
// be positive.
//

//
// We want to do realistic conversions of time so we need to use the same
// values the update wall clock code uses as the jiffies size.  This value
// is: TICK_NSEC (which is defined in timex.h).  This
// is a constant and is in nanoseconds.  We will use scaled math
// with a set of scales defined here as SEC_JIFFIE_SC,  USEC_JIFFIE_SC and
// NSEC_JIFFIE_SC.  Note that these defines contain nothing but
// constants and so are computed at compile time.  SHIFT_HZ (computed in
// timex.h) adjusts the scaling for different HZ values.
// Scaled math???  What is that?
//
// Scaled math is a way to do integer math on values that would,
// otherwise, either overflow, underflow, or cause undesired div
// instructions to appear in the execution path.  In short, we "scale"
// up the operands so they take more bits (more precision, less
// underflow), do the desired operation and then "scale" the result back
// by the same amount.  If we do the scaling by shifting we avoid the
// costly mpy and the dastardly div instructions.
// Suppose, for example, we want to convert from seconds to jiffies
// where jiffies is defined in nanoseconds as NSEC_PER_JIFFIE.  The
// simple math is: jiff = (sec * NSEC_PER_SEC) / NSEC_PER_JIFFIE; We
// observe that (NSEC_PER_SEC / NSEC_PER_JIFFIE) is a constant which we
// might calculate at compile time, however, the result will only have
// about 3-4 bits of precision (less for smaller values of HZ).
//
// So, we scale as follows:
// jiff = (sec) * (NSEC_PER_SEC / NSEC_PER_JIFFIE);
// jiff = ((sec) * ((NSEC_PER_SEC * SCALE)/ NSEC_PER_JIFFIE)) / SCALE;
// Then we make SCALE a power of two so:
// jiff = ((sec) * ((NSEC_PER_SEC << SCALE)/ NSEC_PER_JIFFIE)) >> SCALE;
// Now we define:
// #define SEC_CONV = ((NSEC_PER_SEC << SCALE)/ NSEC_PER_JIFFIE))
// jiff = (sec * SEC_CONV) >> SCALE;
//
// Often the math we use will expand beyond 32-bits so we tell C how to
// do this and pass the 64-bit result of the mpy through the ">> SCALE"
// which should take the result back to 32-bits.  We want this expansion
// to capture as much precision as possible.  At the same time we don't
// want to overflow so we pick the SCALE to avoid this.  In this file,
// that means using a different scale for each range of HZ values (as
// defined in timex.h).
//
// For those who want to know, gcc will give a 64-bit result from a "*"
// operator if the result is a long long AND at least one of the
// operands is cast to long long (usually just prior to the "*" so as
// not to confuse it into thinking it really has a 64-bit operand,
// which, buy the way, it can do, but it takes more code and at least 2
// mpys).
// We also need to be aware that one second in nanoseconds is only a
// couple of bits away from overflowing a 32-bit word, so we MUST use
// 64-bits to get the full range time in nanoseconds.
//
// Here are the scales we will use.  One for seconds, nanoseconds and
// microseconds.
//
// Within the limits of cpp we do a rough cut at the SEC_JIFFIE_SC and
// check if the sign bit is set.  If not, we bump the shift count by 1.
// (Gets an extra bit of precision where we can use it.)
// We know it is set for HZ = 1024 and HZ = 100 not for 1000.
// Haven't tested others.
// Limits of cpp (for #if expressions) only long (no long long), but
// then we only need the most signicant bit.
//

//
// The maximum jiffy value is (MAX_INT >> 1).  Here we translate that
// into seconds.  The 64-bit case will overflow if we are not careful,
// so use the messy SH_DIV macro to do it.  Still all constants.
//

//
// Convert various time units to each other:
//

//
// jiffies_to_msecs - Convert jiffies to milliseconds
// @j: jiffies value
//
// This inline version takes care of HZ in {100,250,1000}.
//
// Return: milliseconds value
//

extern "C" {
    pub fn jiffies_to_msecs(j: c_ulong) -> c_uint;
}

//
// jiffies_to_usecs - Convert jiffies to microseconds
// @j: jiffies value
//
// Return: microseconds value
//
// Hz usually doesn't go much further MSEC_PER_SEC.
// jiffies_to_usecs() and usecs_to_jiffies() depend on that.
//

extern "C" {
    pub fn jiffies_to_usecs(j: c_ulong) -> c_uint;
}

//
// jiffies_to_nsecs - Convert jiffies to nanoseconds
// @j: jiffies value
//
// Return: nanoseconds value
//
extern "C" {
    pub fn jiffies64_to_nsecs(j: u64) -> u64;
}
extern "C" {
    pub fn jiffies64_to_msecs(j: u64) -> u64;
}
extern "C" {
    pub fn __msecs_to_jiffies(m: c_uint) -> c_ulong;
}

//
// HZ is equal to or smaller than 1000, and 1000 is a nice round
// multiple of HZ, divide with the factor between them, but round
// upwards:
//

//
// HZ is larger than 1000, and HZ is a nice round multiple of 1000 -
// simply multiply with the factor between them.
//
// But first make sure the multiplication result cannot overflow:
//

//
// Generic case - multiply, round and divide. But first check that if
// we are doing a net multiplication, that we wouldn't overflow:
//

//
// msecs_to_jiffies: - convert milliseconds to jiffies
// @m:	time in milliseconds
//
// conversion is done as follows:
//
// - negative values mean 'infinite timeout' (MAX_JIFFY_OFFSET)
//
// - 'too large' values [that would result in larger than
// MAX_JIFFY_OFFSET values] mean 'infinite timeout' too.
//
// - all other values are converted to jiffies by either multiplying
// the input value by a factor or dividing it with a factor and
// handling any 32-bit overflows.
// for the details see _msecs_to_jiffies()
//
// msecs_to_jiffies() checks for the passed in value being a constant
// via __builtin_constant_p() allowing gcc to eliminate most of the
// code. __msecs_to_jiffies() is called if the value passed does not
// allow constant folding and the actual conversion must be done at
// runtime.
// The HZ range specific helpers _msecs_to_jiffies() are called both
// directly here and from __msecs_to_jiffies() in the case where
// constant folding is not possible.
//
// Return: jiffies value
//
extern "C" {
    pub fn _msecs_to_jiffies(_arg: m) -> return;
}
extern "C" {
    pub fn __msecs_to_jiffies(_arg: m) -> return;
}
//
// secs_to_jiffies: - convert seconds to jiffies
// @_secs: time in seconds
//
// Conversion is done by simple multiplication with HZ
//
// secs_to_jiffies() is defined as a macro rather than a static inline
// function so it can be used in static initializers.
//
// Return: jiffies value
//

extern "C" {
    pub fn __usecs_to_jiffies(u: c_uint) -> c_ulong;
}

//
// usecs_to_jiffies: - convert microseconds to jiffies
// @u:	time in microseconds
//
// conversion is done as follows:
//
// - 'too large' values [that would result in larger than
// MAX_JIFFY_OFFSET values] mean 'infinite timeout' too.
//
// - all other values are converted to jiffies by either multiplying
// the input value by a factor or dividing it with a factor and
// handling any 32-bit overflows as for msecs_to_jiffies.
//
// usecs_to_jiffies() checks for the passed in value being a constant
// via __builtin_constant_p() allowing gcc to eliminate most of the
// code. __usecs_to_jiffies() is called if the value passed does not
// allow constant folding and the actual conversion must be done at
// runtime.
// The HZ range specific helpers _usecs_to_jiffies() are called both
// directly here and from __msecs_to_jiffies() in the case where
// constant folding is not possible.
//
// Return: jiffies value
//
extern "C" {
    pub fn _usecs_to_jiffies(_arg: u) -> return;
}
extern "C" {
    pub fn __usecs_to_jiffies(_arg: u) -> return;
}
extern "C" {
    pub fn timespec64_to_jiffies(value: *const timespec64) -> c_ulong;
}
extern "C" {
    pub fn jiffies_to_clock_t(x: c_ulong) -> clock_t;
}
extern "C" {
    pub fn jiffies_to_clock_t(_arg: max(0L, _arg: delta)) -> return;
}
extern "C" {
    pub fn jiffies_to_msecs(_arg: max(0L, _arg: delta)) -> return;
}
extern "C" {
    pub fn clock_t_to_jiffies(x: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn jiffies_64_to_clock_t(x: u64) -> u64;
}
extern "C" {
    pub fn nsec_to_clock_t(x: u64) -> u64;
}
extern "C" {
    pub fn nsecs_to_jiffies64(n: u64) -> u64;
}
extern "C" {
    pub fn nsecs_to_jiffies(n: u64) -> c_ulong;
}
pub const TIMESTAMP_SIZE: c_int = 30;
