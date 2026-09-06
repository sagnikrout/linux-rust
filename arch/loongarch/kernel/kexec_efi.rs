//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/kernel/kexec_efi.c
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
// Load EFI vmlinux file for the kexec_file_load syscall.
//
// Author: Youling Tang <tangyouling@kylinos.cn>
// Copyright (C) 2025 KylinSoft Corporation.
//

#[no_mangle]
unsafe extern "C" fn efi_kexec_probe(kernel_buf: *const c_char, kernel_len: c_ulong) -> c_int {
    static int efi_kexec_probe(const char *kernel_buf, unsigned long kernel_len)
    {
    const struct loongarch_image_header *h = (const struct loongarch_image_header *)kernel_buf;
    if (!h || (kernel_len < sizeof(*h))) {
    kexec_dprintk("No LoongArch image header.\n");
    return -EINVAL;
    }
    if (!loongarch_header_check_dos_sig(h)) {
    kexec_dprintk("No LoongArch PE image header.\n");
    return -EINVAL;
    }
    return 0;
    }
    static void *efi_kexec_load(struct kimage *image,
    char *kernel, unsigned long kernel_len,
    char *initrd, unsigned long initrd_len,
    char *cmdline, unsigned long cmdline_len)
    {
    int ret;
    unsigned long text_offset, kernel_segment_number;
    let mut kbuf: kexec_buf = {};
    struct kexec_segment *kernel_segment;
    struct loongarch_image_header *h;
    h = (struct loongarch_image_header *)kernel;
    if (!h.kernel_asize)
    return ERR_PTR(-EINVAL);
//
// Load the kernel
// FIXME: Non-relocatable kernel rejected for kexec_file (require CONFIG_RELOCATABLE)
//
    kbuf.image = image;
    kbuf.buf_max = ULONG_MAX;
    kbuf.top_down = false;
    kbuf.buffer = kernel;
    kbuf.bufsz = kernel_len;
    kbuf.mem = KEXEC_BUF_MEM_UNKNOWN;
    kbuf.memsz = le64_to_cpu(h.kernel_asize);
    text_offset = le64_to_cpu(h.text_offset);
    kbuf.buf_min = text_offset;
    kbuf.buf_align = SZ_2M;
    kernel_segment_number = image.nr_segments;
//
// The location of the kernel segment may make it impossible to
// satisfy the other segment requirements, so we try repeatedly
// to find a location that will work.
//
    while ((ret = kexec_add_buffer(&kbuf)) == 0) {
// Try to load additional data
    kernel_segment = &image.segment[kernel_segment_number];
    ret = load_other_segments(image, kernel_segment.mem,
    kernel_segment.memsz, initrd,
    initrd_len, cmdline, cmdline_len);
    if (!ret)
    break;
//
// We couldn't find space for the other segments; erase the
// kernel segment and try the next available hole.
//
    image.nr_segments -= 1;
    kbuf.buf_min = kernel_segment.mem + kernel_segment.memsz;
    kbuf.mem = KEXEC_BUF_MEM_UNKNOWN;
    }
    if (ret < 0) {
    pr_err("Could not find any suitable kernel location!");
    return ERR_PTR(ret);
    }
    kernel_segment = &image.segment[kernel_segment_number];
// Make sure the second kernel jumps to the correct "kernel_entry"
    image.start = kernel_segment.mem + h.kernel_entry - text_offset;
    kexec_dprintk("Loaded kernel at 0x%lx bufsz=0x%lx memsz=0x%lx\n",
    kernel_segment.mem, kbuf.bufsz, kernel_segment.memsz);
    return core::ptr::null_mut();
    }
    const struct kexec_file_ops kexec_efi_ops = {
    .probe = efi_kexec_probe,
    .load = efi_kexec_load,
    };
