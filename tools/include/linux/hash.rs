//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/linux/hash.h
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


// Fast hashing routine for ints,  longs and pointers.

//
// The "GOLDEN_RATIO_PRIME" is used in ifs/btrfs/brtfs_inode.h and
// fs/inode.c.  It's not actually prime any more (the previous primes
// were actively bad for hashing), but the name remains.
//

//
// This hash multiplies the input by a large odd number and takes the
// high bits.  Since multiplication propagates changes to the most
// significant end only, it is essential that the high bits of the
// product be used for the hash value.
//
// Chuck Lever verified the effectiveness of this technique:
// http://www.citi.umich.edu/techreports/reports/citi-tr-00-1.pdf
//
// Although a random odd number will do, it turns out that the golden
// ratio phi = (sqrt(5)-1)/2, or its negative, has particularly nice
// properties.  (See Knuth vol 3, section 6.4, exercise 9.)
//
// These are the negative, (1 - phi) = phi**2 = (3 - sqrt(5))/2,
// which is very slightly easier to multiply by and makes no
// difference to the hash distribution.
//
pub const GOLDEN_RATIO_32: c_uint = 0x61C88647;
pub const GOLDEN_RATIO_64: c_uint = 0x61C8864680B583EBull;

// This header may use the GOLDEN_RATIO_xx constants

//
// The _generic versions exist only so lib/test_hash.c can compare
// the arch-optimized versions with the generic.
//
// Note that if you change these, any <asm/hash.h> that aren't updated
// to match need to have their HAVE_ARCH_* define values updated so the
// self-test will not false-positive.
//

// High bits are more random, so use them.
extern "C" {
    pub fn __hash_32(bits: val) >> (32 -) -> return;
}

// 64x64-bit multiply is efficient on all 64-bit processors

// Hash 64 bits using only 32x32-bit multiply.
extern "C" {
    pub fn hash_32(32): (u32)val ^ __hash_32(val >>, _arg: bits) -> return;
}

extern "C" {
    pub fn hash_long(long)ptr: (unsigned, _arg: bits) -> return;
}
// This really should be called fold32_ptr; it does no hashing to speak of.

