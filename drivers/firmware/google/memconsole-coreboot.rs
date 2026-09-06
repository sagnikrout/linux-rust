//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/google/memconsole-coreboot.c
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
// memconsole-coreboot.c
//
// Memory based BIOS console accessed through coreboot table.
//
// Copyright 2017 Google Inc.
//

pub const CB_TAG_CBMEM_CONSOLE: c_uint = 0x17;
// CBMEM firmware console log descriptor.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cbmem_cons {
    pub size_dont_access_after_boot: u32,
    pub cursor: u32,
    pub body: [u8; ],
    pub __packed: },

    pub cbmem_console: *mut static struct cbmem_cons,
    pub cbmem_console_size: static u32,
//
// The cbmem_console structure is read again on every access because it may
// change at any time if runtime firmware logs new messages. This may rarely
// lead to race conditions where the firmware overwrites the beginning of the
// ring buffer with more lines after we have already read |cursor|. It should be
// rare and harmless enough that we don't spend extra effort working around it.
//
#[no_mangle]
unsafe extern "C" fn memconsole_coreboot_read(buf: *mut c_char, pos: loff_t, count: usize) -> isize {
    static ssize_t memconsole_coreboot_read(char *buf, loff_t pos, size_t count)
    {
    pub CURSOR_MASK: u32 cursor = cbmem_console->cursor &,
    pub ~CURSOR_MASK: u32 flags = cbmem_console->cursor &,
    pub cbmem_console_size: u32 size =,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct seg {
    pub /: *mut *mut u32 phys; / physical offset from start of mem buffer,
    pub /: *mut *mut u32 len; / length of segment,
    pub }: } seg[2] = { {0}, {0},
    pub 0: size_t done =,
    pub i: c_int,
    if (flags & OVERFLOW) {
    if (cursor > size)	/* Shouldn't really happen, but... */
    pub 0: cursor =,
    pub cursor}: seg[0] = (struct seg){.phys = cursor, .len = size -,
    pub cursor}: seg[1] = (struct seg){.phys = 0, .len =,
    } else {
    pub size)}: seg[0] = (struct seg){.phys = 0, .len = min(cursor,,
    }
    pub {: for (i = 0; i < ARRAY_SIZE(seg) && count > done; i++),
    done += memory_read_from_buffer(buf + done, count - done, &pos,
    pub seg[i].len): cbmem_console->body + seg[i].phys,,
    pub seg[i].len: pos -=,
    }
    pub done: return,
    }
#[no_mangle]
unsafe extern "C" fn memconsole_probe(dev: *mut coreboot_device) -> c_int {
    static int memconsole_probe(struct coreboot_device *dev)
    {
    pub tmp_cbmc: *mut cbmem_cons,
    tmp_cbmc = memremap(dev.cbmem_ref.cbmem_addr,
    pub MEMREMAP_WB): *mut *mut sizeof(tmp_cbmc),,
    if (!tmp_cbmc)
    pub -ENOMEM: return,
// Read size only once to prevent overrun attack through /dev/mem.
    pub tmp_cbmc->size_dont_access_after_boot: cbmem_console_size =,
    cbmem_console = devm_memremap(&dev.dev, dev.cbmem_ref.cbmem_addr,
    cbmem_console_size + sizeof(*cbmem_console),
    if (IS_ERR(cbmem_console))
    pub PTR_ERR(cbmem_console): return,
    pub memconsole_sysfs_init(): return,
    }
#[no_mangle]
unsafe extern "C" fn memconsole_remove(dev: *mut coreboot_device) {
    static void memconsole_remove(struct coreboot_device *dev)
    {
    }
    static const struct coreboot_device_id memconsole_ids[] = {
    { .tag = CB_TAG_CBMEM_CONSOLE },
    { /* sentinel */ }
}

    MODULE_DEVICE_TABLE(coreboot, memconsole_ids);
    static struct coreboot_driver memconsole_driver = {
    .probe = memconsole_probe,
    .remove = memconsole_remove,
    .drv = {
    .name = "memconsole",
    },
    .id_table = memconsole_ids,
    };
    module_coreboot_driver(memconsole_driver);
    MODULE_AUTHOR("Google, Inc.");
    MODULE_DESCRIPTION("Memory based BIOS console accessed through coreboot table");
    MODULE_LICENSE("GPL");
