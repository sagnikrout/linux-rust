//! Automatically rewritten from C to Rust
//! Source: samples/bpf/tracex1_user.c
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

#[no_mangle]
pub unsafe extern "C" fn main(ac: c_int, argv: *mut c_char) -> c_int {
    int main(int ac, char **argv)
    {
    struct bpf_link *link = core::ptr::null_mut();
    struct bpf_program *prog;
    struct bpf_object *obj;
    char filename[256];
    FILE *f;
    snprintf(filename, sizeof(filename), "%s.bpf.o", argv[0]);
    obj = bpf_object__open_file(filename, core::ptr::null_mut());
    if (libbpf_get_error(obj)) {
    fprintf(stderr, "ERROR: opening BPF object file failed\n");
    return 0;
    }
    prog = bpf_object__find_program_by_name(obj, "bpf_prog1");
    if (!prog) {
    fprintf(stderr, "ERROR: finding a prog in obj file failed\n");
    goto cleanup;
    }
// load BPF program
    if (bpf_object__load(obj)) {
    fprintf(stderr, "ERROR: loading BPF object file failed\n");
    goto cleanup;
    }
    link = bpf_program__attach(prog);
    if (libbpf_get_error(link)) {
    fprintf(stderr, "ERROR: bpf_program__attach failed\n");
    link = core::ptr::null_mut();
    goto cleanup;
    }
    f = popen("taskset 1 ping -c5 localhost", "r");
    (void) f;
    read_trace_pipe();
    cleanup:
    bpf_link__destroy(link);
    bpf_object__close(obj);
    return 0;
    }
