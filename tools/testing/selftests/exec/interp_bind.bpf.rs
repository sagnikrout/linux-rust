//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/exec/interp_bind.bpf.c
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
// binfmt_misc_ops handler for the selftest's bound-interpreter case: one
// handler, one entry, an interpreter per guest architecture - each bound to
// a file when the entry was registered rather than to a path resolved at
// exec time. The load program names the one it wants; a name the entry did
// not bind fails the exec, which the harness checks too.
//

    char _license[] SEC("license") = "GPL";
pub const EI_CLASS: c_int = 4;
pub const ELFCLASS64: c_int = 2;
pub const E_MACHINE_OFF: c_int = 18;
pub const EM_ARM: c_int = 40;
pub const EM_AARCH64: c_int = 183;
pub const EM_RISCV: c_int = 243;
    extern int bpf_binprm_select_interp(struct linux_binprm *bprm,
    const char *name, size_t name__sz) __ksym;
// The guest architecture of a 64-bit ELF, or zero if it is not one.
#[no_mangle]
unsafe extern "C" fn elf_machine(bprm: *mut linux_binprm) -> __u16 {
    static __u16 elf_machine(struct linux_binprm *bprm)
    {
    if (bprm.buf[0] != 0x7f || bprm.buf[1] != 'E' ||
    bprm.buf[2] != 'L' || bprm.buf[3] != 'F' ||
    bprm.buf[EI_CLASS] != ELFCLASS64)
    return 0;
// Little-endian 16-bit field, read byte-wise for the verifier.
    return (__u8)bprm.buf[E_MACHINE_OFF] |
    ((__u16)(__u8)bprm.buf[E_MACHINE_OFF + 1] << 8);
    }
    SEC("struct_ops.s/match")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: interp_bind_match, bprm: *mut linux_binprm) -> bool {
    bool BPF_PROG(interp_bind_match, struct linux_binprm *bprm)
    {
    let mut machine: __u16 = elf_machine(bprm);
    return machine == EM_AARCH64 || machine == EM_RISCV ||
    machine == EM_ARM;
    }
    SEC("struct_ops.s/load")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: interp_bind_load, bprm: *mut linux_binprm) -> c_int {
    int BPF_PROG(interp_bind_load, struct linux_binprm *bprm)
    {
//
// Names, not paths: each one selects a file the entry pre-opened, so
// nothing is resolved here or later, in any namespace. The buffers
// are on the stack because the verifier rejects .rodata for a sized
// memory argument.
//
    char first[] = "first";
    char second[] = "second";
    char unbound[] = "unbound";
    switch (elf_machine(bprm)) {
    case EM_AARCH64:
    return bpf_binprm_select_interp(bprm, first, sizeof(first));
    case EM_RISCV:
    return bpf_binprm_select_interp(bprm, second, sizeof(second));
    }
// The entry bound nothing under this name: -ENOENT fails the exec.
    return bpf_binprm_select_interp(bprm, unbound, sizeof(unbound));
    }
    SEC(".struct_ops.link")
    struct binfmt_misc_ops interp_bind = {
    .match	= (void *)interp_bind_match,
    .load	= (void *)interp_bind_load,
    .name	= "interp_bind",
    };
