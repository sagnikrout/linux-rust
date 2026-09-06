//! Automatically rewritten from C to Rust
//! Source: lib/crc/gen_crc64table.c
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
// This host program runs at kernel build time and generates the lookup tables
// used by the generic CRC64 code.
//
// Copyright 2018 SUSE Linux.
// Author: Coly Li <colyli@suse.de>
//

pub const CRC64_ECMA182_POLY: c_uint = 0x42F0E1EBA9EA3693ULL;
pub const CRC64_NVME_POLY: c_uint = 0x9A6C9329AC4BC9B5ULL;
    static uint64_t crc64_table[256] = {0};
    static uint64_t crc64_nvme_table[256] = {0};
#[no_mangle]
unsafe extern "C" fn generate_reflected_crc64_table(table[256]: u64, poly: u64) {
    static void generate_reflected_crc64_table(uint64_t table[256], uint64_t poly)
    {
    uint64_t i, j, c, crc;
    for (i = 0; i < 256; i++) {
    crc = 0ULL;
    c = i;
    for (j = 0; j < 8; j++) {
    if ((crc ^ (c >> j)) & 1)
    crc = (crc >> 1) ^ poly;
    else
    crc >>= 1;
    }
    table[i] = crc;
    }
    }
#[no_mangle]
unsafe extern "C" fn generate_crc64_table(table[256]: u64, poly: u64) {
    static void generate_crc64_table(uint64_t table[256], uint64_t poly)
    {
    uint64_t i, j, c, crc;
    for (i = 0; i < 256; i++) {
    crc = 0;
    c = i << 56;
    for (j = 0; j < 8; j++) {
    if ((crc ^ c) & 0x8000000000000000ULL)
    crc = (crc << 1) ^ poly;
    else
    crc <<= 1;
    c <<= 1;
    }
    table[i] = crc;
    }
    }
#[no_mangle]
unsafe extern "C" fn output_table(table[256]: u64) {
    static void output_table(uint64_t table[256])
    {
    int i;
    for (i = 0; i < 256; i++) {
    printf("\t0x%016" PRIx64 "ULL", table[i]);
    if (i & 0x1)
    printf(",\n");
    else
    printf(", ");
    }
    printf("};\n");
    }
#[no_mangle]
unsafe extern "C" fn print_crc64_tables() {
    static void print_crc64_tables(void)
    {
    printf("/* this file is generated - do not edit */\n\n");
    printf("#include <linux/types.h>\n");
    printf("#include <linux/cache.h>\n\n");
    printf("static const u64 ____cacheline_aligned crc64table[256] = {\n");
    output_table(crc64_table);
    printf("\nstatic const u64 ____cacheline_aligned crc64nvmetable[256] = {\n");
    output_table(crc64_nvme_table);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    generate_crc64_table(crc64_table, CRC64_ECMA182_POLY);
    generate_reflected_crc64_table(crc64_nvme_table, CRC64_NVME_POLY);
    print_crc64_tables();
    return 0;
    }
