//! Automatically rewritten from C to Rust
//! Source: lib/raid/raid6/mktables.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 2002-2007 H. Peter Anvin - All Rights Reserved
//
// Make RAID-6 tables.  This is a host user space program to be run at compile
// time.
//

#[no_mangle]
unsafe extern "C" fn gfmul(a: u8, b: u8) -> u8 {
    static uint8_t gfmul(uint8_t a, uint8_t b)
    {
    let mut v: u8 = 0;
    while (b) {
    if (b & 1)
    v ^= a;
    a = (a << 1) ^ (a & 0x80 ? 0x1d : 0);
    b >>= 1;
    }
    return v;
    }
#[no_mangle]
unsafe extern "C" fn gfpow(a: u8, b: c_int) -> u8 {
    static uint8_t gfpow(uint8_t a, int b)
    {
    let mut v: u8 = 1;
    b %= 255;
    if (b < 0)
    b += 255;
    while (b) {
    if (b & 1)
    v = gfmul(v, a);
    a = gfmul(a, a);
    b >>= 1;
    }
    return v;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    int i, j, k;
    uint8_t v;
    uint8_t exptbl[256], invtbl[256];
    printf("#include <linux/export.h>\n");
    printf("#include \"algos.h\"\n");
// Compute multiplication table
    printf("\nconst u8  __attribute__((aligned(256)))\n"
    "raid6_gfmul[256][256] =\n"
    "{\n");
    for (i = 0; i < 256; i++) {
    printf("\t{\n");
    for (j = 0; j < 256; j += 8) {
    printf("\t\t");
    for (k = 0; k < 8; k++)
    printf("0x%02x,%c", gfmul(i, j + k),
    (k == 7) ? '\n' : ' ');
    }
    printf("\t},\n");
    }
    printf("};\n");
    printf("EXPORT_SYMBOL(raid6_gfmul);\n");
// Compute vector multiplication table
    printf("\nconst u8  __attribute__((aligned(256)))\n"
    "raid6_vgfmul[256][32] =\n"
    "{\n");
    for (i = 0; i < 256; i++) {
    printf("\t{\n");
    for (j = 0; j < 16; j += 8) {
    printf("\t\t");
    for (k = 0; k < 8; k++)
    printf("0x%02x,%c", gfmul(i, j + k),
    (k == 7) ? '\n' : ' ');
    }
    for (j = 0; j < 16; j += 8) {
    printf("\t\t");
    for (k = 0; k < 8; k++)
    printf("0x%02x,%c", gfmul(i, (j + k) << 4),
    (k == 7) ? '\n' : ' ');
    }
    printf("\t},\n");
    }
    printf("};\n");
    printf("EXPORT_SYMBOL(raid6_vgfmul);\n");
// Compute power-of-2 table (exponent)
    v = 1;
    printf("\nconst u8 __attribute__((aligned(256)))\n"
    "raid6_gfexp[256] =\n" "{\n");
    for (i = 0; i < 256; i += 8) {
    printf("\t");
    for (j = 0; j < 8; j++) {
    exptbl[i + j] = v;
    printf("0x%02x,%c", v, (j == 7) ? '\n' : ' ');
    v = gfmul(v, 2);
    if (v == 1)
    v = 0;	/* For entry 255, not a real entry */
    }
    }
    printf("};\n");
    printf("EXPORT_SYMBOL(raid6_gfexp);\n");
// Compute log-of-2 table
    printf("\nconst u8 __attribute__((aligned(256)))\n"
    "raid6_gflog[256] =\n" "{\n");
    for (i = 0; i < 256; i += 8) {
    printf("\t");
    for (j = 0; j < 8; j++) {
    v = 255;
    for (k = 0; k < 256; k++)
    if (exptbl[k] == (i + j)) {
    v = k;
    break;
    }
    printf("0x%02x,%c", v, (j == 7) ? '\n' : ' ');
    }
    }
    printf("};\n");
    printf("EXPORT_SYMBOL(raid6_gflog);\n");
// Compute inverse table x^-1 == x^254
    printf("\nconst u8 __attribute__((aligned(256)))\n"
    "raid6_gfinv[256] =\n" "{\n");
    for (i = 0; i < 256; i += 8) {
    printf("\t");
    for (j = 0; j < 8; j++) {
    invtbl[i + j] = v = gfpow(i + j, 254);
    printf("0x%02x,%c", v, (j == 7) ? '\n' : ' ');
    }
    }
    printf("};\n");
    printf("EXPORT_SYMBOL(raid6_gfinv);\n");
// Compute inv(2^x + 1) (exponent-xor-inverse) table
    printf("\nconst u8 __attribute__((aligned(256)))\n"
    "raid6_gfexi[256] =\n" "{\n");
    for (i = 0; i < 256; i += 8) {
    printf("\t");
    for (j = 0; j < 8; j++)
    printf("0x%02x,%c", invtbl[exptbl[i + j] ^ 1],
    (j == 7) ? '\n' : ' ');
    }
    printf("};\n");
    printf("EXPORT_SYMBOL(raid6_gfexi);\n");
    return 0;
    }
