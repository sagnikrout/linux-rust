//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kernel/kexec_image.c
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
// RISC-V Kexec image loader
//

#[no_mangle]
unsafe extern "C" fn image_probe(kernel_buf: *const c_char, kernel_len: c_ulong) -> c_int {
    static int image_probe(const char *kernel_buf, unsigned long kernel_len)
    {
    const struct riscv_image_header *h = (const struct riscv_image_header *)kernel_buf;
    if (!h || kernel_len < sizeof(*h))
    return -EINVAL;
// According to Documentation/arch/riscv/boot-image-header.rst,
// use "magic2" field to check when version >= 0.2.
//
    if (h.version >= RISCV_HEADER_VERSION &&
    memcmp(&h.magic2, RISCV_IMAGE_MAGIC2, sizeof(h.magic2)))
    return -EINVAL;
    return 0;
    }
    static void *image_load(struct kimage *image,
    char *kernel, unsigned long kernel_len,
    char *initrd, unsigned long initrd_len,
    char *cmdline, unsigned long cmdline_len)
    {
    struct riscv_image_header *h;
    u64 flags;
    bool be_image, be_kernel;
    let mut kbuf: kexec_buf = {};
    int ret;
// Check Image header
    h = (struct riscv_image_header *)kernel;
    if (!h.image_size) {
    ret = -EINVAL;
    goto out;
    }
// Check endianness
    flags = le64_to_cpu(h.flags);
    be_image = riscv_image_flag_field(flags, RISCV_IMAGE_FLAG_BE);
    be_kernel = IS_ENABLED(CONFIG_CPU_BIG_ENDIAN);
    if (be_image != be_kernel) {
    ret = -EINVAL;
    goto out;
    }
// Load the kernel image
    kbuf.image = image;
    kbuf.buf_min = 0;
    kbuf.buf_max = ULONG_MAX;
    kbuf.top_down = false;
    kbuf.buffer = kernel;
    kbuf.bufsz = kernel_len;
    kbuf.mem = KEXEC_BUF_MEM_UNKNOWN;
    kbuf.memsz = le64_to_cpu(h.image_size);
    kbuf.buf_align = le64_to_cpu(h.text_offset);
    ret = kexec_add_buffer(&kbuf);
    if (ret) {
    pr_err("Error add kernel image ret=%d\n", ret);
    goto out;
    }
    image.start = kbuf.mem;
    pr_info("Loaded kernel at 0x%lx bufsz=0x%lx memsz=0x%lx\n",
    kbuf.mem, kbuf.bufsz, kbuf.memsz);
    ret = load_extra_segments(image, kbuf.mem, kbuf.memsz,
    initrd, initrd_len, cmdline, cmdline_len);
    out:
    return ret ? ERR_PTR(ret) : core::ptr::null_mut();
    }
    const struct kexec_file_ops image_kexec_ops = {
    .probe = image_probe,
    .load = image_load,
    };
