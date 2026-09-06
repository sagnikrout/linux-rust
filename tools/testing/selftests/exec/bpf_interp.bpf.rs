//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/exec/bpf_interp.bpf.c
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
// binfmt_misc_ops handler for the selftest's fixed-interpreter case: match a
// 64-bit aarch64 ELF header from the prefetched buffer and route it to a fixed
// interpreter chosen by the program. This is the portable, self-contained
// equivalent of routing a foreign binary to an emulator: it matches
// programmatically and computes the interpreter, but points at a test binary
// the harness installs rather than a system emulator.
//

    char _license[] SEC("license") = "GPL";
pub const EI_CLASS: c_int = 4;
pub const ELFCLASS64: c_int = 2;
pub const EM_AARCH64: c_int = 183;
    extern int bpf_binprm_set_interp(struct linux_binprm *bprm, const char *path,
    size_t path__sz) __ksym;
//
// A magic-style decision needs nothing beyond the prefetched bprm->buf,
// even though the match program could read the file.
//
    SEC("struct_ops.s/match")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: bpf_interp_match, bprm: *mut linux_binprm) -> bool {
    bool BPF_PROG(bpf_interp_match, struct linux_binprm *bprm)
    {
    __u16 machine;
    if (bprm.buf[0] != 0x7f || bprm.buf[1] != 'E' ||
    bprm.buf[2] != 'L' || bprm.buf[3] != 'F' ||
    bprm.buf[EI_CLASS] != ELFCLASS64)
    return false;
// e_machine is a 16-bit little-endian field at offset 18.
    machine = (__u8)bprm.buf[18] | ((__u16)(__u8)bprm.buf[19] << 8);
    let mut machine: return = = EM_AARCH64;
    }
    SEC("struct_ops.s/load")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: bpf_interp_load, bprm: *mut linux_binprm) -> c_int {
    int BPF_PROG(bpf_interp_load, struct linux_binprm *bprm)
    {
//
// Keep the path on the (writable) stack: bpf_binprm_set_interp() takes
// a sized memory arg and the verifier rejects a read-only .rodata
// buffer for it. The harness installs the interpreter at this path.
//
    char interp[] = "/tmp/binfmt_bpf_interp";
// @path__sz includes the terminating NUL; 0 commits the selection.
    return bpf_binprm_set_interp(bprm, interp, sizeof(interp));
    }
    SEC(".struct_ops.link")
    struct binfmt_misc_ops bpf_interp = {
    .match = (void *)bpf_interp_match,
    .load = (void *)bpf_interp_load,
    .name = "bpf_interp",
    };
