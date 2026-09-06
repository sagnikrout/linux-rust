//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/byteorder/generic.h
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
// linux/byteorder/generic.h
// Generic Byte-reordering support
//
// The "... p" macros, like le64_to_cpup, can be used with pointers
// to unaligned data, but there will be a performance penalty on
// some architectures.  Use get_unaligned for unaligned data.
//
// Francois-Rene Rideau <fare@tunes.org> 19970707
// gathered all the good ideas from all asm-foo/byteorder.h into one file,
// cleaned them up.
// I hope it is compliant with non-GCC compilers.
// I decided to put __BYTEORDER_HAS_U64__ in byteorder.h,
// because I wasn't sure it would be ok to put it in types.h
// Upgraded it to 2.1.43
// Francois-Rene Rideau <fare@tunes.org> 19971012
// Upgraded it to 2.1.57
// to please Linus T., replaced huge #ifdef's between little/big endian
// by nestedly #include'd files.
// Francois-Rene Rideau <fare@tunes.org> 19971205
// Made it to 2.1.71; now a facelift:
// Put files under include/linux/byteorder
// Split swab from generic support.
//
// TODO:
// = Regular kernel maintainers could also replace all these manual
// byteswap macros that remain, disseminated among drivers,
// after some grep or the sources...
// = Linus might want to rename all these macros and files to fit his taste,
// to fit his personal naming scheme.
// = it seems that a few drivers would also appreciate
// nybble swapping support...
// = every architecture could add their byteswap macro in asm/byteorder.h
// see how some architectures already do (i386, alpha, ppc, etc)
// = cpu_to_beXX and beXX_to_cpu might some day need to be well
// distinguished throughout the kernel. This is not the case currently,
// since little endian, big endian, and pdp endian machines needn't it.
// But this might be the case for, say, a port of Linux to 20/21 bit
// architectures (and F21 Linux addict around?).
//
// The following macros are to be defined by <asm/byteorder.h>:
//
// Conversion of long and short int between network and host format
// ntohl(__u32 x)
// ntohs(__u16 x)
// htonl(__u32 x)
// htons(__u16 x)
// It seems that some programs (which? where? or perhaps a standard? POSIX?)
// might like the above to be functions, not macros (why?).
// if that's true, then detect them, and take measures.
// Anyway, the measure is: define only ___ntohl as a macro instead,
// and in a separate file, have
// unsigned long inline ntohl(x){return ___ntohl(x);}
//
// The same for constant arguments
// __constant_ntohl(__u32 x)
// __constant_ntohs(__u16 x)
// __constant_htonl(__u32 x)
// __constant_htons(__u16 x)
//
// Conversion of XX-bit integers (16- 32- or 64-)
// between native CPU format and little/big endian format
// 64-bit stuff only defined for proper architectures
// cpu_to_[bl]eXX(__uXX x)
// [bl]eXX_to_cpu(__uXX x)
//
// The same, but takes a pointer to the value to convert
// cpu_to_[bl]eXXp(__uXX x)
// [bl]eXX_to_cpup(__uXX x)
//
// The same, but change in situ
// cpu_to_[bl]eXXs(__uXX x)
// [bl]eXX_to_cpus(__uXX x)
//
// See asm-foo/byteorder.h for examples of how to provide
// architecture-optimized versions
//

//
// They have to be macros in order to do the constant folding
// correctly - if the argument passed into a inline function
// it is no longer constant according to gcc..
//

// var = cpu_to_le16(le16_to_cpu(*var) + val);
// var = cpu_to_le32(le32_to_cpu(*var) + val);
// var = cpu_to_le64(le64_to_cpu(*var) + val);
// XXX: this stuff can be optimized
// var = cpu_to_be16(be16_to_cpu(*var) + val);
// var = cpu_to_be32(be32_to_cpu(*var) + val);
// var = cpu_to_be64(be64_to_cpu(*var) + val);
