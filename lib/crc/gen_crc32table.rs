//! Automatically rewritten from C to Rust
//! Source: lib/crc/gen_crc32table.c
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

    static uint32_t crc32table_le[256];
    static uint32_t crc32table_be[256];
    static uint32_t crc32ctable_le[256];
//
// crc32init_le() - allocate and initialize LE table data
//
// crc is the crc of the byte i; other entries are filled in based on the
// fact that crctable[i^j] = crctable[i] ^ crctable[j].
//
#[no_mangle]
unsafe extern "C" fn crc32init_le_generic(polynomial: u32, tab[256]: u32) {
    static void crc32init_le_generic(const uint32_t polynomial, uint32_t tab[256])
    {
    unsigned i, j;
    let mut crc: u32 = 1;
    tab[0] = 0;
    for (i = 128; i; i >>= 1) {
    crc = (crc >> 1) ^ ((crc & 1) ? polynomial : 0);
    for (j = 0; j < 256; j += 2 * i)
    tab[i + j] = crc ^ tab[j];
    }
    }
#[no_mangle]
unsafe extern "C" fn crc32init_le() {
    static void crc32init_le(void)
    {
    crc32init_le_generic(CRC32_POLY_LE, crc32table_le);
    }
#[no_mangle]
unsafe extern "C" fn crc32cinit_le() {
    static void crc32cinit_le(void)
    {
    crc32init_le_generic(CRC32C_POLY_LE, crc32ctable_le);
    }
//
// crc32init_be() - allocate and initialize BE table data
//
#[no_mangle]
unsafe extern "C" fn crc32init_be() {
    static void crc32init_be(void)
    {
    unsigned i, j;
    let mut crc: u32 = 0x80000000;
    crc32table_be[0] = 0;
    for (i = 1; i < 256; i <<= 1) {
    crc = (crc << 1) ^ ((crc & 0x80000000) ? CRC32_POLY_BE : 0);
    for (j = 0; j < i; j++)
    crc32table_be[i + j] = crc ^ crc32table_be[j];
    }
    }
#[no_mangle]
unsafe extern "C" fn output_table(table[256]: u32) {
    static void output_table(const uint32_t table[256])
    {
    int i;
    for (i = 0; i < 256; i += 4) {
    printf("\t0x%08x, 0x%08x, 0x%08x, 0x%08x,\n",
    table[i], table[i + 1], table[i + 2], table[i + 3]);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut *mut c_char) -> c_int {
    int main(int argc, char** argv)
    {
    printf("/* this file is generated - do not edit */\n\n");
    crc32init_le();
    printf("static const u32 ____cacheline_aligned crc32table_le[256] = {\n");
    output_table(crc32table_le);
    printf("};\n");
    crc32init_be();
    printf("static const u32 ____cacheline_aligned crc32table_be[256] = {\n");
    output_table(crc32table_be);
    printf("};\n");
    crc32cinit_le();
    printf("static const u32 ____cacheline_aligned crc32ctable_le[256] = {\n");
    output_table(crc32ctable_le);
    printf("};\n");
    return 0;
    }
