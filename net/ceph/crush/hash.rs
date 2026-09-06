//! Automatically rewritten from C to Rust
//! Source: net/ceph/crush/hash.c
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
// Robert Jenkins' function for mixing 32-bit values
// https://burtleburtle.net/bob/hash/evahash.html
// a, b = random bits, c = input and output
//

    a = a-b;  a = a-c;  a = a^(c>>13);	\
    b = b-c;  b = b-a;  b = b^(a<<8);	\
    c = c-a;  c = c-b;  c = c^(b>>13);	\
    a = a-b;  a = a-c;  a = a^(c>>12);	\
    b = b-c;  b = b-a;  b = b^(a<<16);	\
    c = c-a;  c = c-b;  c = c^(b>>5);	\
    a = a-b;  a = a-c;  a = a^(c>>3);	\
    b = b-c;  b = b-a;  b = b^(a<<10);	\
    c = c-a;  c = c-b;  c = c^(b>>15);	\
    } while (0)
pub const crush_hash_seed: c_int = 1315423911;
#[no_mangle]
unsafe extern "C" fn crush_hash32_rjenkins1(a: __u32) -> __u32 {
    static __u32 crush_hash32_rjenkins1(__u32 a)
    {
    let mut hash: __u32 = crush_hash_seed ^ a;
    let mut b: __u32 = a;
    let mut x: __u32 = 231232;
    let mut y: __u32 = 1232;
    crush_hashmix(b, x, hash);
    crush_hashmix(y, a, hash);
    return hash;
    }
#[no_mangle]
unsafe extern "C" fn crush_hash32_rjenkins1_2(a: __u32, b: __u32) -> __u32 {
    static __u32 crush_hash32_rjenkins1_2(__u32 a, __u32 b)
    {
    let mut hash: __u32 = crush_hash_seed ^ a ^ b;
    let mut x: __u32 = 231232;
    let mut y: __u32 = 1232;
    crush_hashmix(a, b, hash);
    crush_hashmix(x, a, hash);
    crush_hashmix(b, y, hash);
    return hash;
    }
#[no_mangle]
unsafe extern "C" fn crush_hash32_rjenkins1_3(a: __u32, b: __u32, c: __u32) -> __u32 {
    static __u32 crush_hash32_rjenkins1_3(__u32 a, __u32 b, __u32 c)
    {
    let mut hash: __u32 = crush_hash_seed ^ a ^ b ^ c;
    let mut x: __u32 = 231232;
    let mut y: __u32 = 1232;
    crush_hashmix(a, b, hash);
    crush_hashmix(c, x, hash);
    crush_hashmix(y, a, hash);
    crush_hashmix(b, x, hash);
    crush_hashmix(y, c, hash);
    return hash;
    }
#[no_mangle]
unsafe extern "C" fn crush_hash32_rjenkins1_4(a: __u32, b: __u32, c: __u32, d: __u32) -> __u32 {
    static __u32 crush_hash32_rjenkins1_4(__u32 a, __u32 b, __u32 c, __u32 d)
    {
    let mut hash: __u32 = crush_hash_seed ^ a ^ b ^ c ^ d;
    let mut x: __u32 = 231232;
    let mut y: __u32 = 1232;
    crush_hashmix(a, b, hash);
    crush_hashmix(c, d, hash);
    crush_hashmix(a, x, hash);
    crush_hashmix(y, b, hash);
    crush_hashmix(c, x, hash);
    crush_hashmix(y, d, hash);
    return hash;
    }
    static __u32 crush_hash32_rjenkins1_5(__u32 a, __u32 b, __u32 c, __u32 d,
    __u32 e)
    {
    let mut hash: __u32 = crush_hash_seed ^ a ^ b ^ c ^ d ^ e;
    let mut x: __u32 = 231232;
    let mut y: __u32 = 1232;
    crush_hashmix(a, b, hash);
    crush_hashmix(c, d, hash);
    crush_hashmix(e, x, hash);
    crush_hashmix(y, a, hash);
    crush_hashmix(b, x, hash);
    crush_hashmix(y, c, hash);
    crush_hashmix(d, x, hash);
    crush_hashmix(y, e, hash);
    return hash;
    }
#[no_mangle]
pub unsafe extern "C" fn crush_hash32(type: c_int, a: __u32) -> __u32 {
    __u32 crush_hash32(int type, __u32 a)
    {
    switch (type) {
    case CRUSH_HASH_RJENKINS1:
    return crush_hash32_rjenkins1(a);
    default:
    return 0;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn crush_hash32_2(type: c_int, a: __u32, b: __u32) -> __u32 {
    __u32 crush_hash32_2(int type, __u32 a, __u32 b)
    {
    switch (type) {
    case CRUSH_HASH_RJENKINS1:
    return crush_hash32_rjenkins1_2(a, b);
    default:
    return 0;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn crush_hash32_3(type: c_int, a: __u32, b: __u32, c: __u32) -> __u32 {
    __u32 crush_hash32_3(int type, __u32 a, __u32 b, __u32 c)
    {
    switch (type) {
    case CRUSH_HASH_RJENKINS1:
    return crush_hash32_rjenkins1_3(a, b, c);
    default:
    return 0;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn crush_hash32_4(type: c_int, a: __u32, b: __u32, c: __u32, d: __u32) -> __u32 {
    __u32 crush_hash32_4(int type, __u32 a, __u32 b, __u32 c, __u32 d)
    {
    switch (type) {
    case CRUSH_HASH_RJENKINS1:
    return crush_hash32_rjenkins1_4(a, b, c, d);
    default:
    return 0;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn crush_hash32_5(type: c_int, a: __u32, b: __u32, c: __u32, d: __u32, e: __u32) -> __u32 {
    __u32 crush_hash32_5(int type, __u32 a, __u32 b, __u32 c, __u32 d, __u32 e)
    {
    switch (type) {
    case CRUSH_HASH_RJENKINS1:
    return crush_hash32_rjenkins1_5(a, b, c, d, e);
    default:
    return 0;
    }
    }
    const char *crush_hash_name(int type)
    {
    switch (type) {
    case CRUSH_HASH_RJENKINS1:
    return "rjenkins1";
    default:
    return "unknown";
    }
    }
