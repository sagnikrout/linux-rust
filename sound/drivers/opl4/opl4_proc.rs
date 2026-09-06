//! Automatically rewritten from C to Rust
//! Source: sound/drivers/opl4/opl4_proc.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Functions for the OPL4 proc file
// Copyright (c) 2003 by Clemens Ladisch <clemens@ladisch.de>
//

    static int snd_opl4_mem_proc_open(struct snd_info_entry *entry,
    unsigned short mode, void **file_private_data)
    {
    struct snd_opl4 *opl4 = entry.private_data;
    guard(mutex)(&opl4.access_mutex);
    if (opl4.memory_access)
    return -EBUSY;
    opl4.memory_access++;
    return 0;
    }
    static int snd_opl4_mem_proc_release(struct snd_info_entry *entry,
    unsigned short mode, void *file_private_data)
    {
    struct snd_opl4 *opl4 = entry.private_data;
    guard(mutex)(&opl4.access_mutex);
    opl4.memory_access--;
    return 0;
    }
    static ssize_t snd_opl4_mem_proc_read(struct snd_info_entry *entry,
    void *file_private_data,
    struct file *file, char __user *_buf,
    size_t count, loff_t pos)
    {
    struct snd_opl4 *opl4 = entry.private_data;
    char* buf;
    buf = vmalloc(count);
    if (!buf)
    return -ENOMEM;
    snd_opl4_read_memory(opl4, buf, pos, count);
    if (copy_to_user(_buf, buf, count)) {
    vfree(buf);
    return -EFAULT;
    }
    vfree(buf);
    return count;
    }
    static ssize_t snd_opl4_mem_proc_write(struct snd_info_entry *entry,
    void *file_private_data,
    struct file *file,
    const char __user *_buf,
    size_t count, loff_t pos)
    {
    struct snd_opl4 *opl4 = entry.private_data;
    char *buf;
    buf = vmalloc(count);
    if (!buf)
    return -ENOMEM;
    if (copy_from_user(buf, _buf, count)) {
    vfree(buf);
    return -EFAULT;
    }
    snd_opl4_write_memory(opl4, buf, pos, count);
    vfree(buf);
    return count;
    }
    static const struct snd_info_entry_ops snd_opl4_mem_proc_ops = {
    .open = snd_opl4_mem_proc_open,
    .release = snd_opl4_mem_proc_release,
    .read = snd_opl4_mem_proc_read,
    .write = snd_opl4_mem_proc_write,
    };
#[no_mangle]
pub unsafe extern "C" fn snd_opl4_create_proc(opl4: *mut snd_opl4) -> c_int {
    int snd_opl4_create_proc(struct snd_opl4 *opl4)
    {
    struct snd_info_entry *entry;
    entry = snd_info_create_card_entry(opl4.card, "opl4-mem", opl4.card.proc_root);
    if (entry) {
    if (opl4.hardware < OPL3_HW_OPL4_ML) {
// OPL4 can access 4 MB external ROM/SRAM
    entry.mode |= 0200;
    entry.size = 4 * 1024 * 1024;
    } else {
// OPL4-ML has 1 MB internal ROM
    entry.size = 1 * 1024 * 1024;
    }
    entry.content = SNDRV_INFO_CONTENT_DATA;
    entry.c.ops = &snd_opl4_mem_proc_ops;
    entry.module = THIS_MODULE;
    entry.private_data = opl4;
    }
    opl4.proc_entry = entry;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn snd_opl4_free_proc(opl4: *mut snd_opl4) {
    void snd_opl4_free_proc(struct snd_opl4 *opl4)
    {
    snd_info_free_entry(opl4.proc_entry);
    }
