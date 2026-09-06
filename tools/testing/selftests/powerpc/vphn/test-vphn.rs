//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/vphn/test-vphn.c
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

    static struct test {
    char *descr;
    long input[VPHN_REGISTER_COUNT];
    u32 expected[VPHN_ASSOC_BUFSIZE];
    } all_tests[] = {
    {
    "vphn: no data",
    {
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    },
    {
    0x00000000
    }
    },
    {
    "vphn: 1 x 16-bit value",
    {
    0x8001ffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    },
    {
    0x00000001,
    0x00000001
    }
    },
    {
    "vphn: 2 x 16-bit values",
    {
    0x80018002ffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    },
    {
    0x00000002,
    0x00000001,
    0x00000002
    }
    },
    {
    "vphn: 3 x 16-bit values",
    {
    0x800180028003ffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    },
    {
    0x00000003,
    0x00000001,
    0x00000002,
    0x00000003
    }
    },
    {
    "vphn: 4 x 16-bit values",
    {
    0x8001800280038004,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    },
    {
    0x00000004,
    0x00000001,
    0x00000002,
    0x00000003,
    0x00000004
    }
    },
    {
// Parsing the next 16-bit value out of the next 64-bit input
// value.
//
    "vphn: 5 x 16-bit values",
    {
    0x8001800280038004,
    0x8005ffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    },
    {
    0x00000005,
    0x00000001,
    0x00000002,
    0x00000003,
    0x00000004,
    0x00000005
    }
    },
    {
// Parse at most 6 x 64-bit input values
    "vphn: 24 x 16-bit values",
    {
    0x8001800280038004,
    0x8005800680078008,
    0x8009800a800b800c,
    0x800d800e800f8010,
    0x8011801280138014,
    0x8015801680178018
    },
    {
    0x00000018,
    0x00000001,
    0x00000002,
    0x00000003,
    0x00000004,
    0x00000005,
    0x00000006,
    0x00000007,
    0x00000008,
    0x00000009,
    0x0000000a,
    0x0000000b,
    0x0000000c,
    0x0000000d,
    0x0000000e,
    0x0000000f,
    0x00000010,
    0x00000011,
    0x00000012,
    0x00000013,
    0x00000014,
    0x00000015,
    0x00000016,
    0x00000017,
    0x00000018
    }
    },
    {
    "vphn: 1 x 32-bit value",
    {
    0x00000001ffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff
    },
    {
    0x00000001,
    0x00000001
    }
    },
    {
    "vphn: 2 x 32-bit values",
    {
    0x0000000100000002,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff
    },
    {
    0x00000002,
    0x00000001,
    0x00000002
    }
    },
    {
// Parsing the next 32-bit value out of the next 64-bit input
// value.
//
    "vphn: 3 x 32-bit values",
    {
    0x0000000100000002,
    0x00000003ffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff
    },
    {
    0x00000003,
    0x00000001,
    0x00000002,
    0x00000003
    }
    },
    {
// Parse at most 6 x 64-bit input values
    "vphn: 12 x 32-bit values",
    {
    0x0000000100000002,
    0x0000000300000004,
    0x0000000500000006,
    0x0000000700000008,
    0x000000090000000a,
    0x0000000b0000000c
    },
    {
    0x0000000c,
    0x00000001,
    0x00000002,
    0x00000003,
    0x00000004,
    0x00000005,
    0x00000006,
    0x00000007,
    0x00000008,
    0x00000009,
    0x0000000a,
    0x0000000b,
    0x0000000c
    }
    },
    {
    "vphn: 16-bit value followed by 32-bit value",
    {
    0x800100000002ffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff
    },
    {
    0x00000002,
    0x00000001,
    0x00000002
    }
    },
    {
    "vphn: 32-bit value followed by 16-bit value",
    {
    0x000000018002ffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff
    },
    {
    0x00000002,
    0x00000001,
    0x00000002
    }
    },
    {
// Parse a 32-bit value split across two consecutives 64-bit
// input values.
//
    "vphn: 16-bit value followed by 2 x 32-bit values",
    {
    0x8001000000020000,
    0x0003ffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff
    },
    {
    0x00000003,
    0x00000001,
    0x00000002,
    0x00000003,
    0x00000004,
    0x00000005
    }
    },
    {
// The lower bits in 0x0001ffff don't get mixed up with the
// 0xffff terminator.
//
    "vphn: 32-bit value has all ones in 16 lower bits",
    {
    0x0001ffff80028003,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff
    },
    {
    0x00000003,
    0x0001ffff,
    0x00000002,
    0x00000003
    }
    },
    {
// The following input doesn't follow the specification.
//
    "vphn: last 32-bit value is truncated",
    {
    0x0000000100000002,
    0x0000000300000004,
    0x0000000500000006,
    0x0000000700000008,
    0x000000090000000a,
    0x0000000b800c2bad
    },
    {
    0x0000000c,
    0x00000001,
    0x00000002,
    0x00000003,
    0x00000004,
    0x00000005,
    0x00000006,
    0x00000007,
    0x00000008,
    0x00000009,
    0x0000000a,
    0x0000000b,
    0x0000000c
    }
    },
    {
    "vphn: garbage after terminator",
    {
    0xffff2bad2bad2bad,
    0x2bad2bad2bad2bad,
    0x2bad2bad2bad2bad,
    0x2bad2bad2bad2bad,
    0x2bad2bad2bad2bad,
    0x2bad2bad2bad2bad
    },
    {
    0x00000000
    }
    },
    {
    core::ptr::null_mut()
    }
    };
#[no_mangle]
unsafe extern "C" fn test_one(test: *mut test) -> c_int {
    static int test_one(struct test *test)
    {
    __be32 output[VPHN_ASSOC_BUFSIZE] = { 0 };
    int i, len;
    vphn_unpack_associativity(test.input, output);
    len = be32_to_cpu(output[0]);
    if (len != test.expected[0]) {
    printf("expected %d elements, got %d\n", test.expected[0],
    len);
    return 1;
    }
    for (i = 1; i < len; i++) {
    let mut val: u32 = be32_to_cpu(output[i]);
    if (val != test.expected[i]) {
    printf("element #%d is 0x%x, should be 0x%x\n", i, val,
    test.expected[i]);
    return 1;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_vphn() -> c_int {
    static int test_vphn(void)
    {
    static struct test *test;
    for (test = all_tests; test.descr; test++) {
    int ret;
    ret = test_one(test);
    test_finish(test.descr, ret);
    if (ret)
    return ret;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    return test_harness(test_vphn, "test-vphn");
    }
