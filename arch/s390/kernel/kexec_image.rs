//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/kexec_image.c
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
// Image loader for kexec_file_load system call.
//
// Copyright IBM Corp. 2018
//
// Author(s): Philipp Rudo <prudo@linux.vnet.ibm.com>
//

    static int kexec_file_add_kernel_image(struct kimage *image,
    struct s390_load_data *data)
    {
    let mut buf: kexec_buf = {};
    buf.image = image;
    buf.buffer = image.kernel_buf;
    buf.bufsz = image.kernel_buf_len;
    buf.mem = 0;

    if (image.type == KEXEC_TYPE_CRASH)
    buf.mem += crashk_res.start;

    buf.memsz = buf.bufsz;
    data.kernel_buf = image.kernel_buf;
    data.kernel_mem = buf.mem;
    data.parm = image.kernel_buf + PARMAREA;
    data.memsz += buf.memsz;
    ipl_report_add_component(data.report, &buf,
    IPL_RB_COMPONENT_FLAG_SIGNED |
    IPL_RB_COMPONENT_FLAG_VERIFIED,
    IPL_RB_CERT_UNKNOWN);
    return kexec_add_buffer(&buf);
    }
    static void *s390_image_load(struct kimage *image,
    char *kernel, unsigned long kernel_len,
    char *initrd, unsigned long initrd_len,
    char *cmdline, unsigned long cmdline_len)
    {
    return kexec_file_add_components(image, kexec_file_add_kernel_image);
    }
#[no_mangle]
unsafe extern "C" fn s390_image_probe(buf: *const c_char, len: c_ulong) -> c_int {
    static int s390_image_probe(const char *buf, unsigned long len)
    {
// Can't reliably tell if an image is valid.  Therefore give the
// user whatever he wants.
//
    return 0;
    }
    const struct kexec_file_ops s390_kexec_image_ops = {
    .probe = s390_image_probe,
    .load = s390_image_load,

    .verify_sig = s390_verify_sig,

    };
