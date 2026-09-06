//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/kernel/kexec_elf.c
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
// Load ELF vmlinux file for the kexec_file_load syscall.
//
// Author: Youling Tang <tangyouling@kylinos.cn>
// Copyright (C) 2025 KylinSoft Corporation.
//

    static int _elf_kexec_load(struct kimage *image,
    struct elfhdr *ehdr, struct kexec_elf_info *elf_info,
    struct kexec_buf *kbuf, unsigned long *text_offset)
    {
    int i, ret = -1;
// Read in the PT_LOAD segments.
    for (i = 0; i < ehdr.e_phnum; i++) {
    size_t size;
    const struct elf_phdr *phdr;
    phdr = &elf_info.proghdrs[i];
    if (phdr.p_type != PT_LOAD)
    continue;
    size = phdr.p_filesz;
    if (size > phdr.p_memsz)
    size = phdr.p_memsz;
    kbuf.buffer = (void *)elf_info.buffer + phdr.p_offset;
    kbuf.bufsz = size;
    kbuf.buf_align = phdr.p_align;
// text_offset = __pa(phdr->p_paddr);
    kbuf.buf_min = *text_offset;
    kbuf.memsz = ALIGN(phdr.p_memsz, SZ_64K);
    kbuf.mem = KEXEC_BUF_MEM_UNKNOWN;
    ret = kexec_add_buffer(kbuf);
    if (ret < 0)
    break;
    }
    return ret;
    }
    static void *elf_kexec_load(struct kimage *image,
    char *kernel, unsigned long kernel_len,
    char *initrd, unsigned long initrd_len,
    char *cmdline, unsigned long cmdline_len)
    {
    int ret;
    unsigned long text_offset, kernel_segment_number;
    struct elfhdr ehdr;
    let mut kbuf: kexec_buf = {};
    struct kexec_elf_info elf_info;
    struct kexec_segment *kernel_segment;
    ret = kexec_build_elf_info(kernel, kernel_len, &ehdr, &elf_info);
    if (ret < 0)
    return ERR_PTR(ret);
//
// Load the kernel
// FIXME: Non-relocatable kernel rejected for kexec_file (require CONFIG_RELOCATABLE)
//
    kbuf.image = image;
    kbuf.buf_max = ULONG_MAX;
    kbuf.top_down = false;
    kernel_segment_number = image.nr_segments;
    ret = _elf_kexec_load(image, &ehdr, &elf_info, &kbuf, &text_offset);
    if (ret < 0)
    goto out;
// Load additional data
    kernel_segment = &image.segment[kernel_segment_number];
    ret = load_other_segments(image, kernel_segment.mem, kernel_segment.memsz,
    initrd, initrd_len, cmdline, cmdline_len);
    if (ret < 0)
    goto out;
// Make sure the second kernel jumps to the correct "kernel_entry".
    image.start = kernel_segment.mem + __pa(ehdr.e_entry) - text_offset;
    kexec_dprintk("Loaded kernel at 0x%lx bufsz=0x%lx memsz=0x%lx\n",
    kernel_segment.mem, kbuf.bufsz, kernel_segment.memsz);
    out:
    kexec_free_elf_info(&elf_info);
    return ret ? ERR_PTR(ret) : core::ptr::null_mut();
    }
    const struct kexec_file_ops kexec_elf_ops = {
    .probe = elf_kexec_probe,
    .load  = elf_kexec_load,
    };
