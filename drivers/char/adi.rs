//! Automatically rewritten from C to Rust
//! Source: drivers/char/adi.c
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
// Privileged ADI driver for sparc64
//
// Author: Tom Hromatka <tom.hromatka@oracle.com>
//

#[no_mangle]
unsafe extern "C" fn read_mcd_tag(addr: c_ulong) -> c_int {
    static int read_mcd_tag(unsigned long addr)
    {
    long err;
    int ver;
    __asm__ __volatile__(
    "1:	ldxa [%[addr]] %[asi], %[ver]\n"
    "	mov 0, %[err]\n"
    "2:\n"
    "	.section .fixup,#alloc,#execinstr\n"
    "	.align 4\n"
    "3:	sethi %%hi(2b), %%g1\n"
    "	jmpl  %%g1 + %%lo(2b), %%g0\n"
    "	mov %[invalid], %[err]\n"
    "	.previous\n"
    "	.section __ex_table, \"a\"\n"
    "	.align 4\n"
    "	.word  1b, 3b\n"
    "	.previous\n"
    : [ver] "=r" (ver), [err] "=r" (err)
    : [addr] "r"  (addr), [invalid] "i" (EFAULT),
    [asi] "i" (ASI_MCD_REAL)
    : "memory", "g1"
    );
    if (err)
    return -EFAULT;
    else
    return ver;
    }
    static ssize_t adi_read(struct file *file, char __user *buf,
    size_t count, loff_t *offp)
    {
    size_t ver_buf_sz, bytes_read = 0;
    let mut ver_buf_idx: c_int = 0;
    loff_t offset;
    u8 *ver_buf;
    ssize_t ret;
    ver_buf_sz = min_t(size_t, count, MAX_BUF_SZ);
    ver_buf = kmalloc(ver_buf_sz, GFP_KERNEL);
    if (!ver_buf)
    return -ENOMEM;
    offset = (*offp) * adi_blksize();
    while (bytes_read < count) {
    ret = read_mcd_tag(offset);
    if (ret < 0)
    goto out;
    ver_buf[ver_buf_idx] = (u8)ret;
    ver_buf_idx++;
    offset += adi_blksize();
    if (ver_buf_idx >= ver_buf_sz) {
    if (copy_to_user(buf + bytes_read, ver_buf,
    ver_buf_sz)) {
    ret = -EFAULT;
    goto out;
    }
    bytes_read += ver_buf_sz;
    ver_buf_idx = 0;
    ver_buf_sz = min_t(size_t, count - bytes_read,
    MAX_BUF_SZ);
    }
    }
    (*offp) += bytes_read;
    ret = bytes_read;
    out:
    kfree(ver_buf);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn set_mcd_tag(addr: c_ulong, ver: u8) -> c_int {
    static int set_mcd_tag(unsigned long addr, u8 ver)
    {
    long err;
    __asm__ __volatile__(
    "1:	stxa %[ver], [%[addr]] %[asi]\n"
    "	mov 0, %[err]\n"
    "2:\n"
    "	.section .fixup,#alloc,#execinstr\n"
    "	.align 4\n"
    "3:	sethi %%hi(2b), %%g1\n"
    "	jmpl %%g1 + %%lo(2b), %%g0\n"
    "	mov %[invalid], %[err]\n"
    "	.previous\n"
    "	.section __ex_table, \"a\"\n"
    "	.align 4\n"
    "	.word 1b, 3b\n"
    "	.previous\n"
    : [err] "=r" (err)
    : [ver] "r" (ver), [addr] "r"  (addr),
    [invalid] "i" (EFAULT), [asi] "i" (ASI_MCD_REAL)
    : "memory", "g1"
    );
    if (err)
    return -EFAULT;
    else
    return ver;
    }
    static ssize_t adi_write(struct file *file, const char __user *buf,
    size_t count, loff_t *offp)
    {
    size_t ver_buf_sz, bytes_written = 0;
    loff_t offset;
    u8 *ver_buf;
    ssize_t ret;
    int i;
    if (count == 0)
    return -EINVAL;
    ver_buf_sz = min_t(size_t, count, MAX_BUF_SZ);
    ver_buf = kmalloc(ver_buf_sz, GFP_KERNEL);
    if (!ver_buf)
    return -ENOMEM;
    offset = (*offp) * adi_blksize();
    do {
    if (copy_from_user(ver_buf, &buf[bytes_written],
    ver_buf_sz)) {
    ret = -EFAULT;
    goto out;
    }
    for (i = 0; i < ver_buf_sz; i++) {
    ret = set_mcd_tag(offset, ver_buf[i]);
    if (ret < 0)
    goto out;
    offset += adi_blksize();
    }
    bytes_written += ver_buf_sz;
    ver_buf_sz = min_t(size_t, count - bytes_written, MAX_BUF_SZ);
    } while (bytes_written < count);
    (*offp) += bytes_written;
    ret = bytes_written;
    out:
    __asm__ __volatile__("membar #Sync");
    kfree(ver_buf);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn adi_llseek(file: *mut file, offset: loff_t, whence: c_int) -> loff_t {
    static loff_t adi_llseek(struct file *file, loff_t offset, int whence)
    {
    let mut ret: loff_t = -EINVAL;
    switch (whence) {
    case SEEK_END:
    case SEEK_DATA:
    case SEEK_HOLE:
// unsupported
    return -EINVAL;
    case SEEK_CUR:
    if (offset == 0)
    return file.f_pos;
    offset += file.f_pos;
    break;
    case SEEK_SET:
    break;
    }
    if (offset != file.f_pos) {
    file.f_pos = offset;
    ret = offset;
    }
    return ret;
    }
    static const struct file_operations adi_fops = {
    .owner		= THIS_MODULE,
    .llseek		= adi_llseek,
    .read		= adi_read,
    .write		= adi_write,
    .fop_flags	= FOP_UNSIGNED_OFFSET,
    };
    static struct miscdevice adi_miscdev = {
    .minor = MISC_DYNAMIC_MINOR,
    .name = KBUILD_MODNAME,
    .fops = &adi_fops,
    };
#[no_mangle]
unsafe extern "C" fn adi_init() -> int __init {
    static int __init adi_init(void)
    {
    if (!adi_capable())
    return -EPERM;
    return misc_register(&adi_miscdev);
    }
#[no_mangle]
unsafe extern "C" fn adi_exit() -> void __exit {
    static void __exit adi_exit(void)
    {
    misc_deregister(&adi_miscdev);
    }
    module_init(adi_init);
    module_exit(adi_exit);
    MODULE_AUTHOR("Tom Hromatka <tom.hromatka@oracle.com>");
    MODULE_DESCRIPTION("Privileged interface to ADI");
    MODULE_VERSION("1.0");
    MODULE_LICENSE("GPL v2");
