//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/machine_kexec_reloc.c
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

    int arch_kexec_do_relocs(int r_type, void *loc, unsigned long val,
    unsigned long addr)
    {
    switch (r_type) {
    case R_390_NONE:
    break;
    case R_390_8:		/* Direct 8 bit.   */
// (u8 *)loc = val;
    break;
    case R_390_12:		/* Direct 12 bit.  */
// (u16 *)loc &= 0xf000;
// (u16 *)loc |= val & 0xfff;
    break;
    case R_390_16:		/* Direct 16 bit.  */
// (u16 *)loc = val;
    break;
    case R_390_20:		/* Direct 20 bit.  */
// (u32 *)loc &= 0xf00000ff;
// (u32 *)loc |= (val & 0xfff) << 16;	/* DL
// (u32 *)loc |= (val & 0xff000) >> 4;	/* DH
    break;
    case R_390_32:		/* Direct 32 bit.  */
// (u32 *)loc = val;
    break;
    case R_390_64:		/* Direct 64 bit.  */
    case R_390_GLOB_DAT:
    case R_390_JMP_SLOT:
// (u64 *)loc = val;
    break;
    case R_390_PC16:	/* PC relative 16 bit.	*/
// (u16 *)loc = (val - addr);
    break;
    case R_390_PC16DBL:	/* PC relative 16 bit shifted by 1.  */
// (u16 *)loc = (val - addr) >> 1;
    break;
    case R_390_PC32DBL:	/* PC relative 32 bit shifted by 1.  */
// (u32 *)loc = (val - addr) >> 1;
    break;
    case R_390_PC32:	/* PC relative 32 bit.	*/
// (u32 *)loc = (val - addr);
    break;
    case R_390_PC64:	/* PC relative 64 bit.	*/
// (u64 *)loc = (val - addr);
    break;
    case R_390_RELATIVE:
// (unsigned long *) loc = val;
    break;
    default:
    return 1;
    }
    return 0;
    }
