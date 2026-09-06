//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/riscv/vector/v_exec_initval_nolibc.c
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
//
// Get values of vector registers as soon as the program starts to test if
// is properly cleaning the values before starting a new program. Vector
// registers are caller saved, so no function calls may happen before reading
// the values. To further ensure consistency, this file is compiled without
// libc and without auto-vectorization.
//
// To be "clean" all values must be all zeroes.
//

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    let mut value: c_char = 0;
    unsigned long vl;
    if (argc > 2 && strcmp(argv[2], "x"))
    asm volatile (
// 0 | zimm[10:0] | rs1 | 1 1 1 | rd |1010111| vsetvli
// vsetvli	t4, x0, e8, m1, d1
    ".4byte		0b00000000000000000111111011010111\n\t"
    "mv		%[vl], t4\n\t"
    : [vl] "=r" (vl) : : "t4"
    );
    else
    asm volatile (
    ".option push\n\t"
    ".option arch, +v\n\t"
    "vsetvli	%[vl], x0, e8, m1, ta, ma\n\t"
    ".option pop\n\t"
    : [vl] "=r" (vl)
    );

    for (int i = 0; i < vl; i++) {						\
    asm volatile (							\
    ".option push\n\t"					\
    ".option arch, +v\n\t"					\
    "vmv.x.s %0, " __stringify(register) "\n\t"		\
    "vsrl.vi " __stringify(register) ", " __stringify(register) ", 8\n\t" \
    ".option pop\n\t"					\
    : "=r" (value));					\
    if (value != 0x00) {						\
    printf("Register " __stringify(register)		\
    " values not clean! value: %u\n", value);	\
    exit(-1);						\
    }								\
    }									\
    })
    CHECK_VECTOR_REGISTER(v0);
    CHECK_VECTOR_REGISTER(v1);
    CHECK_VECTOR_REGISTER(v2);
    CHECK_VECTOR_REGISTER(v3);
    CHECK_VECTOR_REGISTER(v4);
    CHECK_VECTOR_REGISTER(v5);
    CHECK_VECTOR_REGISTER(v6);
    CHECK_VECTOR_REGISTER(v7);
    CHECK_VECTOR_REGISTER(v8);
    CHECK_VECTOR_REGISTER(v9);
    CHECK_VECTOR_REGISTER(v10);
    CHECK_VECTOR_REGISTER(v11);
    CHECK_VECTOR_REGISTER(v12);
    CHECK_VECTOR_REGISTER(v13);
    CHECK_VECTOR_REGISTER(v14);
    CHECK_VECTOR_REGISTER(v15);
    CHECK_VECTOR_REGISTER(v16);
    CHECK_VECTOR_REGISTER(v17);
    CHECK_VECTOR_REGISTER(v18);
    CHECK_VECTOR_REGISTER(v19);
    CHECK_VECTOR_REGISTER(v20);
    CHECK_VECTOR_REGISTER(v21);
    CHECK_VECTOR_REGISTER(v22);
    CHECK_VECTOR_REGISTER(v23);
    CHECK_VECTOR_REGISTER(v24);
    CHECK_VECTOR_REGISTER(v25);
    CHECK_VECTOR_REGISTER(v26);
    CHECK_VECTOR_REGISTER(v27);
    CHECK_VECTOR_REGISTER(v28);
    CHECK_VECTOR_REGISTER(v29);
    CHECK_VECTOR_REGISTER(v30);
    CHECK_VECTOR_REGISTER(v31);

    return 0;
    }
