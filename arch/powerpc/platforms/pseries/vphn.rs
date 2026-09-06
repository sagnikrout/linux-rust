//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/pseries/vphn.c
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
// The associativity domain numbers are returned from the hypervisor as a
// stream of mixed 16-bit and 32-bit fields. The stream is terminated by the
// special value of "all ones" (aka. 0xffff) and its size may not exceed 48
// bytes.
//
// --- 16-bit fields -->
// _________________________
// |  0  |  1  |  2  |  3  |   be_packed[0]
// ------+-----+-----+------
// _________________________
// |  4  |  5  |  6  |  7  |   be_packed[1]
// -------------------------
// ...
// _________________________
// | 20  | 21  | 22  | 23  |   be_packed[5]
// -------------------------
//
// Convert to the sequence they would appear in the ibm,associativity property.
//
#[no_mangle]
unsafe extern "C" fn vphn_unpack_associativity(packed: *const c_long, unpacked: *mut __be32) -> c_int {
    static int vphn_unpack_associativity(const long *packed, __be32 *unpacked)
    {
    __be64 be_packed[VPHN_REGISTER_COUNT];
    int i, nr_assoc_doms = 0;
    const __be16 *field = (const __be16 *) be_packed;
    let mut last: u16 = 0;
    let mut is_32bit: bool = false;

// Let's fix the values returned by plpar_hcall9()
    for (i = 0; i < VPHN_REGISTER_COUNT; i++)
    be_packed[i] = cpu_to_be64(packed[i]);
    for (i = 1; i < VPHN_ASSOC_BUFSIZE; i++) {
    let mut new: u16 = be16_to_cpup(field++);
    if (is_32bit) {
//
// Let's concatenate the 16 bits of this field to the
// 15 lower bits of the previous field
//
    unpacked[++nr_assoc_doms] =
    cpu_to_be32(last << 16 | new);
    is_32bit = false;
    } else if (new == VPHN_FIELD_UNUSED)
// This is the list terminator
    break;
#[no_mangle]
pub unsafe extern "C" fn if(VPHN_FIELD_MSB: new &) -> else {
// Data is in the lower 15 bits of this field
    unpacked[++nr_assoc_doms] =
    cpu_to_be32(new & VPHN_FIELD_MASK);
    } else {
//
// Data is in the lower 15 bits of this field
// concatenated with the next 16 bit field
//
    last = new;
    is_32bit = true;
    }
    }
// The first cell contains the length of the property
    unpacked[0] = cpu_to_be32(nr_assoc_doms);
    return nr_assoc_doms;
    }
// NOTE: This file is included by a selftest and built in userspace.

#[no_mangle]
pub unsafe extern "C" fn hcall_vphn(cpu: c_ulong, flags: u64, associativity: *mut __be32) -> c_long {
    long hcall_vphn(unsigned long cpu, u64 flags, __be32 *associativity)
    {
    long rc;
    long retbuf[PLPAR_HCALL9_BUFSIZE] = {0};
    rc = plpar_hcall9(H_HOME_NODE_ASSOCIATIVITY, retbuf, flags, cpu);
    if (rc == H_SUCCESS)
    vphn_unpack_associativity(retbuf, associativity);
    return rc;
    }
