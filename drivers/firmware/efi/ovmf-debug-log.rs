//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/efi/ovmf-debug-log.c
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

pub const OVMF_DEBUG_LOG_MAGIC1: c_uint = 0x3167646d666d766f  // "ovmfmdg1";
pub const OVMF_DEBUG_LOG_MAGIC2: c_uint = 0x3267646d666d766f  // "ovmfmdg2";
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ovmf_debug_log_header {
    pub magic1: u64,
    pub magic2: u64,
    pub hdr_size: u64,
    pub log_size: u64,
    pub spinlock: u64 lock; // edk2,
    pub head_off: u64,
    pub tail_off: u64,
    pub truncated: u64,
    pub fw_version: [u8; 128],
}

    static struct ovmf_debug_log_header *hdr;
    static u8 *logbuf;
    static u64 logbufsize;
    static ssize_t ovmf_log_read(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *attr, char *buf,
    loff_t offset, size_t count)
    {
    u64 start, end;
    start = hdr.head_off + offset;
    if (hdr.head_off > hdr.tail_off && start >= hdr.log_size)
    start -= hdr.log_size;
    end = start + count;
    if (start > hdr.tail_off) {
    if (end > hdr.log_size)
    end = hdr.log_size;
    } else {
    if (end > hdr.tail_off)
    end = hdr.tail_off;
    }
    if (start > logbufsize || end > logbufsize)
    return 0;
    if (start >= end)
    return 0;
    memcpy(buf, logbuf + start, end - start);
    return end - start;
    }
    static struct bin_attribute ovmf_log_bin_attr = {
    .attr = {
    .name = "ovmf_debug_log",
    .mode = 0444,
    },
    .read = ovmf_log_read,
    };
#[no_mangle]
pub unsafe extern "C" fn ovmf_log_probe(ovmf_debug_log_table: c_ulong) -> int __init {
    int __init ovmf_log_probe(unsigned long ovmf_debug_log_table)
    {
    let mut ret: c_int = -EINVAL;
    u64 size;
// map + verify header
    hdr = memremap(ovmf_debug_log_table, sizeof(*hdr), MEMREMAP_WB);
    if (!hdr) {
    pr_err("OVMF debug log: header map failed\n");
    return -EINVAL;
    }
    if (hdr.magic1 != OVMF_DEBUG_LOG_MAGIC1 ||
    hdr.magic2 != OVMF_DEBUG_LOG_MAGIC2) {
    printk(KERN_ERR "OVMF debug log: magic mismatch\n");
    goto err_unmap;
    }
    size = hdr.hdr_size + hdr.log_size;
    pr_info("OVMF debug log: firmware version: \"%s\"\n", hdr.fw_version);
    pr_info("OVMF debug log: buffer size: %lluk\n", size / 1024);
// map complete log buffer
    memunmap(hdr);
    hdr = memremap(ovmf_debug_log_table, size, MEMREMAP_WB);
    if (!hdr) {
    pr_err("OVMF debug log: buffer map failed\n");
    return -EINVAL;
    }
    logbuf = (void *)hdr + hdr.hdr_size;
    logbufsize = hdr.log_size;
    ovmf_log_bin_attr.size = size;
    ret = sysfs_create_bin_file(efi_kobj, &ovmf_log_bin_attr);
    if (ret != 0) {
    pr_err("OVMF debug log: sysfs register failed\n");
    goto err_unmap;
    }
    return 0;
    err_unmap:
    memunmap(hdr);
    return ret;
    }
