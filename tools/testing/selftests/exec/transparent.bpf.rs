//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/exec/transparent.bpf.c
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
// binfmt_misc_ops handler for the transparent-mode case: match a synthetic
// riscv ELF header and run the asserting interpreter transparently - the
// argument vector untouched, the binary in AT_EXECFD and mm->exe_file
// labeled with the binary.
//

    char _license[] SEC("license") = "GPL";
pub const EI_CLASS: c_int = 4;
pub const ELFCLASS64: c_int = 2;
pub const EM_RISCV: c_int = 243;
    extern int bpf_binprm_set_interp(struct linux_binprm *bprm, const char *path,
    size_t path__sz) __ksym;
    extern int bpf_binprm_set_flags(struct linux_binprm *bprm,
    enum bpf_binprm_flags flags) __ksym;
    SEC("struct_ops.s/match")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: transparent_match, bprm: *mut linux_binprm) -> bool {
    bool BPF_PROG(transparent_match, struct linux_binprm *bprm)
    {
    __u16 machine;
    if (bprm.buf[0] != 0x7f || bprm.buf[1] != 'E' ||
    bprm.buf[2] != 'L' || bprm.buf[3] != 'F' ||
    bprm.buf[EI_CLASS] != ELFCLASS64)
    return false;
// e_machine is a 16-bit little-endian field at offset 18.
    machine = (__u8)bprm.buf[18] | ((__u16)(__u8)bprm.buf[19] << 8);
    let mut machine: return = = EM_RISCV;
    }
    SEC("struct_ops.s/load")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: transparent_load, bprm: *mut linux_binprm) -> c_int {
    int BPF_PROG(transparent_load, struct linux_binprm *bprm)
    {
    char interp[] = "/tmp/binfmt_transparent_interp";
    int err;
    err = bpf_binprm_set_flags(bprm, BPF_BINPRM_TRANSPARENT);
    if (err)
    return err;
// @path__sz includes the terminating NUL; 0 commits the selection.
    return bpf_binprm_set_interp(bprm, interp, sizeof(interp));
    }
    SEC(".struct_ops.link")
    struct binfmt_misc_ops transparent = {
    .match = (void *)transparent_match,
    .load = (void *)transparent_load,
    .name = "transparent",
    };
