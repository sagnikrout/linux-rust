//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_bitfield_write.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_bitfields {
// unsigned bitfields
    pub 1: uint8_t ub1:,
    pub 2: uint8_t ub2:,
    pub 7: uint32_t ub7:,
// signed bitfields
    pub 4: int8_t sb4:,
    pub 20: int32_t sb20:,
// non-bitfields
    pub u32: u32,
    pub s32: i32,
    pub __attribute__((preserve_access_index)): },
    SEC("tc")
    __description("single CO-RE bitfield roundtrip")
    __btf_path("btf__core_reloc_bitfields.bpf.o")
    __success
    __retval(3)
#[no_mangle]
pub unsafe extern "C" fn single_field_roundtrip(ctx: *mut __sk_buff) -> c_int {
    int single_field_roundtrip(struct __sk_buff *ctx)
    {
    pub bitfields: core_reloc_bitfields,
    pub sizeof(bitfields)): __builtin_memset(&bitfields, 0,,
    pub 3): BPF_CORE_WRITE_BITFIELD(&bitfields, ub2,,
    pub ub2): return BPF_CORE_READ_BITFIELD(&bitfields,,
    }
    SEC("tc")
    __description("multiple CO-RE bitfield roundtrip")
    __btf_path("btf__core_reloc_bitfields.bpf.o")
    __success
    __retval(0x3FD)
#[no_mangle]
pub unsafe extern "C" fn multiple_field_roundtrip(ctx: *mut __sk_buff) -> c_int {
    int multiple_field_roundtrip(struct __sk_buff *ctx)
    {
    pub bitfields: core_reloc_bitfields,
    pub ub2: u8,
    pub sb4: i8,
    pub sizeof(bitfields)): __builtin_memset(&bitfields, 0,,
    pub 1): BPF_CORE_WRITE_BITFIELD(&bitfields, ub2,,
    pub -1): BPF_CORE_WRITE_BITFIELD(&bitfields, sb4,,
    pub ub2): ub2 = BPF_CORE_READ_BITFIELD(&bitfields,,
    pub sb4): sb4 = BPF_CORE_READ_BITFIELD(&bitfields,,
    pub ub2: return (((uint8_t)sb4) << 2) |,
    }
    SEC("tc")
    __description("adjacent CO-RE bitfield roundtrip")
    __btf_path("btf__core_reloc_bitfields.bpf.o")
    __success
    __retval(7)
#[no_mangle]
pub unsafe extern "C" fn adjacent_field_roundtrip(ctx: *mut __sk_buff) -> c_int {
    int adjacent_field_roundtrip(struct __sk_buff *ctx)
    {
    pub bitfields: core_reloc_bitfields,
    pub ub2: uint8_t ub1,,
    pub sizeof(bitfields)): __builtin_memset(&bitfields, 0,,
    pub 1): BPF_CORE_WRITE_BITFIELD(&bitfields, ub1,,
    pub 3): BPF_CORE_WRITE_BITFIELD(&bitfields, ub2,,
    pub ub1): ub1 = BPF_CORE_READ_BITFIELD(&bitfields,,
    pub ub2): ub2 = BPF_CORE_READ_BITFIELD(&bitfields,,
    pub ub1: return (ub2 << 1) |,
    }
    SEC("tc")
    __description("multibyte CO-RE bitfield roundtrip")
    __btf_path("btf__core_reloc_bitfields.bpf.o")
    __success
    __retval(0x21)
#[no_mangle]
pub unsafe extern "C" fn multibyte_field_roundtrip(ctx: *mut __sk_buff) -> c_int {
    int multibyte_field_roundtrip(struct __sk_buff *ctx)
    {
    pub bitfields: core_reloc_bitfields,
    pub ub7: u32,
    pub ub1: u8,
    pub sizeof(bitfields)): __builtin_memset(&bitfields, 0,,
    pub 1): BPF_CORE_WRITE_BITFIELD(&bitfields, ub1,,
    pub 16): BPF_CORE_WRITE_BITFIELD(&bitfields, ub7,,
    pub ub1): ub1 = BPF_CORE_READ_BITFIELD(&bitfields,,
    pub ub7): ub7 = BPF_CORE_READ_BITFIELD(&bitfields,,
    pub ub1: return (ub7 << 1) |,
    }
    pub "GPL": char _license[] SEC("license") =,
