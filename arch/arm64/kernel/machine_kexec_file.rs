//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/machine_kexec_file.c
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
// kexec_file for arm64
//
// Copyright (C) 2018 Linaro Limited
// Author: AKASHI Takahiro <takahiro.akashi@linaro.org>
//
// Most code is derived from arm64 port of kexec-tools
//

    const struct kexec_file_ops * const kexec_file_loaders[] = {
    &kexec_image_ops,
    core::ptr::null_mut()
    };
#[no_mangle]
pub unsafe extern "C" fn arch_kimage_file_post_load_cleanup(image: *mut kimage) -> c_int {
    int arch_kimage_file_post_load_cleanup(struct kimage *image)
    {
    kvfree(image.arch.dtb);
    image.arch.dtb = core::ptr::null_mut();
    vfree(image.elf_headers);
    image.elf_headers = core::ptr::null_mut();
    image.elf_headers_sz = 0;
    return kexec_image_post_load_cleanup_default(image);
    }

#[no_mangle]
pub unsafe extern "C" fn arch_get_system_nr_ranges() -> c_uint {
    unsigned int arch_get_system_nr_ranges(void)
    {
    unsigned int nr_ranges = 2 + crashk_cma_cnt; /* for exclusion of crashkernel region */
    phys_addr_t start, end;
    u64 i;
    for_each_mem_range(i, &start, &end)
    nr_ranges++;
    return nr_ranges;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_crash_populate_cmem(cmem: *mut crash_mem) -> c_int {
    int arch_crash_populate_cmem(struct crash_mem *cmem)
    {
    phys_addr_t start, end;
    u64 i;
    for_each_mem_range(i, &start, &end) {
    cmem.ranges[cmem.nr_ranges].start = start;
    cmem.ranges[cmem.nr_ranges].end = end - 1;
    cmem.nr_ranges++;
    }
    return 0;
    }

//
// Tries to add the initrd and DTB to the image. If it is not possible to find
// valid locations, this function will undo changes to the image and return non
// zero.
//
    int load_other_segments(struct kimage *image,
    unsigned long kernel_load_addr,
    unsigned long kernel_size,
    char *initrd, unsigned long initrd_len,
    char *cmdline)
    {
    let mut kbuf: kexec_buf = {};
    void *dtb = core::ptr::null_mut();
    unsigned long initrd_load_addr = 0, dtb_len,
    orig_segments = image.nr_segments;
    let mut ret: c_int = 0;
    kbuf.image = image;
// not allocate anything below the kernel
    kbuf.buf_min = kernel_load_addr + kernel_size;

// load elf core header
    void *headers;
    unsigned long headers_sz;
    if (image.type == KEXEC_TYPE_CRASH) {
    ret = crash_prepare_headers(true, &headers, &headers_sz, core::ptr::null_mut());
    if (ret) {
    pr_err("Preparing elf core header failed\n");
    goto out_err;
    }
    kbuf.buffer = headers;
    kbuf.bufsz = headers_sz;
    kbuf.mem = KEXEC_BUF_MEM_UNKNOWN;
    kbuf.memsz = headers_sz;
    kbuf.buf_align = SZ_64K; /* largest supported page size */
    kbuf.buf_max = ULONG_MAX;
    kbuf.top_down = true;
    ret = kexec_add_buffer(&kbuf);
    if (ret) {
    vfree(headers);
    goto out_err;
    }
    image.elf_headers = headers;
    image.elf_load_addr = kbuf.mem;
    image.elf_headers_sz = headers_sz;
    kexec_dprintk("Loaded elf core header at 0x%lx bufsz=0x%lx memsz=0x%lx\n",
    image.elf_load_addr, kbuf.bufsz, kbuf.memsz);
    ret = crash_load_dm_crypt_keys(image);
    if (ret)
    goto out_err;
    }

// load initrd
    if (initrd) {
    kbuf.buffer = initrd;
    kbuf.bufsz = initrd_len;
    kbuf.mem = KEXEC_BUF_MEM_UNKNOWN;
    kbuf.memsz = initrd_len;
    kbuf.buf_align = 0;
// within 1GB-aligned window of up to 32GB in size
    kbuf.buf_max = round_down(kernel_load_addr, SZ_1G)
    + (unsigned long)SZ_1G * 32;
    kbuf.top_down = false;
    ret = kexec_add_buffer(&kbuf);
    if (ret)
    goto out_err;
    initrd_load_addr = kbuf.mem;
    kexec_dprintk("Loaded initrd at 0x%lx bufsz=0x%lx memsz=0x%lx\n",
    initrd_load_addr, kbuf.bufsz, kbuf.memsz);
    }
// load dtb
    dtb = of_kexec_alloc_and_setup_fdt(image, initrd_load_addr,
    initrd_len, cmdline, 0);
    if (!dtb) {
    pr_err("Preparing for new dtb failed\n");
    ret = -EINVAL;
    goto out_err;
    }
// trim it
    fdt_pack(dtb);
    dtb_len = fdt_totalsize(dtb);
    kbuf.buffer = dtb;
    kbuf.bufsz = dtb_len;
    kbuf.mem = KEXEC_BUF_MEM_UNKNOWN;
    kbuf.memsz = dtb_len;
// not across 2MB boundary
    kbuf.buf_align = SZ_2M;
    kbuf.buf_max = ULONG_MAX;
    kbuf.top_down = true;
    ret = kexec_add_buffer(&kbuf);
    if (ret)
    goto out_err;
    image.arch.dtb = dtb;
    image.arch.dtb_mem = kbuf.mem;
    kexec_dprintk("Loaded dtb at 0x%lx bufsz=0x%lx memsz=0x%lx\n",
    kbuf.mem, kbuf.bufsz, kbuf.memsz);
    return 0;
    out_err:
    image.nr_segments = orig_segments;
    kvfree(dtb);
    return ret;
    }
