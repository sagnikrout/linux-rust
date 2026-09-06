//! Automatically rewritten from C to Rust
//! Source: arch/x86/boot/compressed/mkpiggy.c
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


// SPDX-License-Identifier: GPL-2.0-only
// -----------------------------------------------------------------------
//
// Copyright (C) 2009 Intel Corporation. All rights reserved.
//
// H. Peter Anvin <hpa@linux.intel.com>
//
// -----------------------------------------------------------------------
//
// Outputs a small assembly wrapper with the appropriate symbols defined.
//

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    uint32_t olen;
    long ilen;
    FILE *f = core::ptr::null_mut();
    let mut retval: c_int = 1;
    if (argc < 2) {
    fprintf(stderr, "Usage: %s compressed_file\n", argv[0]);
    goto bail;
    }
// Get the information for the compressed kernel image first
    f = fopen(argv[1], "r");
    if (!f) {
    perror(argv[1]);
    goto bail;
    }
    if (fseek(f, -4L, SEEK_END)) {
    perror(argv[1]);
    }
    if (fread(&olen, sizeof(olen), 1, f) != 1) {
    perror(argv[1]);
    goto bail;
    }
    ilen = ftell(f);
    olen = get_unaligned_le32(&olen);
    printf(".section \".rodata..compressed\",\"a\",@progbits\n");
    printf(".globl z_input_len\n");
    printf("z_input_len = %lu\n", ilen);
    printf(".globl z_output_len\n");
    printf("z_output_len = %lu\n", (unsigned long)olen);
    printf(".globl input_data, input_data_end\n");
    printf("input_data:\n");
    printf(".incbin \"%s\"\n", argv[1]);
    printf("input_data_end:\n");
    printf(".section \".rodata\",\"a\",@progbits\n");
    printf(".globl input_len\n");
    printf("input_len:\n\t.long %lu\n", ilen);
    printf(".globl output_len\n");
    printf("output_len:\n\t.long %lu\n", (unsigned long)olen);
    retval = 0;
    bail:
    if (f)
    fclose(f);
    return retval;
    }
