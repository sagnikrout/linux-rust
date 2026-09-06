//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/btf_endian.c
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
// Copyright (c) 2020 Facebook
// Macro flag: #define _GNU_SOURCE

#[no_mangle]
pub unsafe extern "C" fn test_btf_endian() {

    let mut endian: enum btf_endianness = BTF_LITTLE_ENDIAN;

    let mut endian: enum btf_endianness = BTF_BIG_ENDIAN;

    let mut swap_endian: enum btf_endianness = 1 - endian;
    struct btf *btf = core::ptr::null_mut(), *swap_btf = core::ptr::null_mut();
    const void *raw_data, *swap_raw_data;
    const struct btf_type *t;
    const struct btf_header *hdr;
    __u32 raw_sz, swap_raw_sz;
    int var_id;
// Load BTF in native endianness
    btf = btf__parse_elf("btf_dump_test_case_syntax.bpf.o", core::ptr::null_mut());
    if (!ASSERT_OK_PTR(btf, "parse_native_btf"))
    goto err_out;
    ASSERT_EQ(btf__endianness(btf), endian, "endian");
    btf__set_endianness(btf, swap_endian);
    ASSERT_EQ(btf__endianness(btf), swap_endian, "endian");
// Get raw BTF data in non-native endianness...
    raw_data = btf__raw_data(btf, &raw_sz);
    if (!ASSERT_OK_PTR(raw_data, "raw_data_inverted"))
    goto err_out;
// ...and open it as a new BTF instance
    swap_btf = btf__new(raw_data, raw_sz);
    if (!ASSERT_OK_PTR(swap_btf, "parse_swap_btf"))
    goto err_out;
    ASSERT_EQ(btf__endianness(swap_btf), swap_endian, "endian");
    ASSERT_EQ(btf__type_cnt(swap_btf), btf__type_cnt(btf), "nr_types");
    swap_raw_data = btf__raw_data(swap_btf, &swap_raw_sz);
    if (!ASSERT_OK_PTR(swap_raw_data, "swap_raw_data"))
    goto err_out;
// both raw data should be identical (with non-native endianness)
    ASSERT_OK(memcmp(raw_data, swap_raw_data, raw_sz), "mem_identical");
// make sure that at least BTF header data is really swapped
    hdr = swap_raw_data;
    ASSERT_EQ(bswap_16(hdr.magic), BTF_MAGIC, "btf_magic_swapped");
    ASSERT_EQ(raw_sz, swap_raw_sz, "raw_sizes");
// swap it back to native endianness
    btf__set_endianness(swap_btf, endian);
    swap_raw_data = btf__raw_data(swap_btf, &swap_raw_sz);
    if (!ASSERT_OK_PTR(swap_raw_data, "swap_raw_data"))
    goto err_out;
// now header should have native BTF_MAGIC
    hdr = swap_raw_data;
    ASSERT_EQ(hdr.magic, BTF_MAGIC, "btf_magic_native");
    ASSERT_EQ(raw_sz, swap_raw_sz, "raw_sizes");
// now modify original BTF
    var_id = btf__add_var(btf, "some_var", BTF_VAR_GLOBAL_ALLOCATED, 1);
    ASSERT_GT(var_id, 0, "var_id");
    btf__free(swap_btf);
    swap_btf = core::ptr::null_mut();
    btf__set_endianness(btf, swap_endian);
    raw_data = btf__raw_data(btf, &raw_sz);
    if (!ASSERT_OK_PTR(raw_data, "raw_data_inverted"))
    goto err_out;
// and re-open swapped raw data again
    swap_btf = btf__new(raw_data, raw_sz);
    if (!ASSERT_OK_PTR(swap_btf, "parse_swap_btf"))
    goto err_out;
    ASSERT_EQ(btf__endianness(swap_btf), swap_endian, "endian");
    ASSERT_EQ(btf__type_cnt(swap_btf), btf__type_cnt(btf), "nr_types");
// the type should appear as if it was stored in native endianness
    t = btf__type_by_id(swap_btf, var_id);
    ASSERT_STREQ(btf__str_by_offset(swap_btf, t.name_off), "some_var", "var_name");
    ASSERT_EQ(btf_var(t).linkage, BTF_VAR_GLOBAL_ALLOCATED, "var_linkage");
    ASSERT_EQ(t.type, 1, "var_type");
    err_out:
    btf__free(btf);
    btf__free(swap_btf);
    }
