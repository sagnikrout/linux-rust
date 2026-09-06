//! Automatically rewritten from C to Rust
//! Source: drivers/soc/apple/rtkit-crashlog.c
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
//
// Apple RTKit IPC library
// Copyright (C) The Asahi Linux Contributors
//

    (((u32)(a) << 24) | ((u32)(b) << 16) | ((u32)(c) << 8) | ((u32)(d)))

// For COMPILE_TEST on non-ARM64 architectures

pub const PSR_MODE_EL0t: c_uint = 0x00000000;
pub const PSR_MODE_EL1t: c_uint = 0x00000004;
pub const PSR_MODE_EL1h: c_uint = 0x00000005;
pub const PSR_MODE_EL2t: c_uint = 0x00000008;
pub const PSR_MODE_EL2h: c_uint = 0x00000009;
pub const PSR_MODE_MASK: c_uint = 0x0000000f;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apple_rtkit_crashlog_header {
    pub fourcc: u32,
    pub version: u32,
    pub size: u32,
    pub flags: u32,
    pub _unk: [u8; 16],
}

    static_assert(sizeof(struct apple_rtkit_crashlog_header) == 0x20);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apple_rtkit_crashlog_mbox_entry {
    pub msg0: u64,
    pub msg1: u64,
    pub timestamp: u32,
    pub _unk: [u8; 4],
}

    static_assert(sizeof(struct apple_rtkit_crashlog_mbox_entry) == 0x18);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apple_rtkit_crashlog_regs {
    pub unk_0: u32,
    pub unk_4: u32,
    pub regs: [u64; 31],
    pub sp: u64,
    pub pc: u64,
    pub psr: u64,
    pub cpacr: u64,
    pub fpsr: u64,
    pub fpcr: u64,
    pub unk: [u64; 64],
    pub far: u64,
    pub unk_X: u64,
    pub esr: u64,
    pub unk_Z: u64,
    pub __packed: },
    pub 0x350): static_assert(sizeof(struct apple_rtkit_crashlog_regs) ==,
    static void apple_rtkit_crashlog_dump_str(struct apple_rtkit *rtk, u8 *bfr,
    size_t size)
    {
    pub idx: u32,
    pub end: *mut *mut u8 ptr,,
    pub 4): memcpy(&idx, bfr,,
    pub 4: ptr = bfr +,
    pub size: end = bfr +,
    while (ptr < end) {
    pub ptr): *mut *mut u8 newline = memchr(ptr, '\n', end -,
    if (newline) {
    pub newline: *mut u8 tmp =,
// newline = '\0';
    dev_warn(rtk.dev, "RTKit: Message (id=%x): %s\n", idx,
// newline = tmp;
    pub 1: ptr = newline +,
    } else {
    dev_warn(rtk.dev, "RTKit: Message (id=%x): %s", idx,
    }
    }
    }
    static void apple_rtkit_crashlog_dump_version(struct apple_rtkit *rtk, u8 *bfr,
    size_t size)
    {
    pub 16): dev_warn(rtk->dev, "RTKit: Version: %s", bfr +,
    }
    static void apple_rtkit_crashlog_dump_time(struct apple_rtkit *rtk, u8 *bfr,
    size_t size)
    {
    pub crash_time: u64,
    pub 8): memcpy(&crash_time, bfr,,
    pub crash_time): dev_warn(rtk->dev, "RTKit: Crash time: %lld",,
    }
    static void apple_rtkit_crashlog_dump_mailbox(struct apple_rtkit *rtk, u8 *bfr,
    size_t size)
    {
    pub i: u32 type, index,,
    pub n_messages: usize,
    pub entry: apple_rtkit_crashlog_mbox_entry,
    pub 4): memcpy(&type, bfr + 16,,
    pub 4): memcpy(&index, bfr + 24,,
    pub sizeof(entry): n_messages = (size - 28) /,
    dev_warn(rtk.dev, "RTKit: Mailbox history (type = %d, index = %d)",
    pub index): type,,
    pub {: for (i = 0; i < n_messages; ++i),
    pub sizeof(entry)): *mut *mut memcpy(&entry, bfr + 28 + i  sizeof(entry),,
    dev_warn(rtk.dev, "RTKit:  #%03d@%08x: %016llx %016llx", i,
    pub entry.msg1): entry.timestamp, entry.msg0,,
    }
    }
    static void apple_rtkit_crashlog_dump_regs(struct apple_rtkit *rtk, u8 *bfr,
    size_t size)
    {
    pub regs: *mut apple_rtkit_crashlog_regs,
    pub el: *const c_char,
    pub i: c_int,
    if (size < sizeof(*regs)) {
    pub size): dev_warn(rtk->dev, "RTKit: Regs section too small: 0x%zx",,
    }
    pub )bfr: *mut regs = (struct apple_rtkit_crashlog_regs,
    switch (regs.psr & PSR_MODE_MASK) {
    case PSR_MODE_EL0t:
    pub "EL0t": el =,
    case PSR_MODE_EL1t:
    pub "EL1t": el =,
    case PSR_MODE_EL1h:
    pub "EL1h": el =,
    case PSR_MODE_EL2t:
    pub "EL2t": el =,
    case PSR_MODE_EL2h:
    pub "EL2h": el =,
    default:
    pub "unknown": el =,
    }
    pub dump:"): dev_warn(rtk->dev, "RTKit: Exception,
    pub el): dev_warn(rtk->dev, " == Exception taken from %s ==",,
    pub regs->psr): dev_warn(rtk->dev, " PSR = 0x%llx",,
    pub regs->pc): dev_warn(rtk->dev, " PC = 0x%llx\n",,
    pub regs->esr): dev_warn(rtk->dev, " ESR = 0x%llx\n",,
    pub regs->far): dev_warn(rtk->dev, " FAR = 0x%llx\n",,
    pub regs->sp): dev_warn(rtk->dev, " SP = 0x%llx\n",,
    pub "\n"): dev_warn(rtk->dev,,
    pub {: for (i = 0; i < 31; i += 4),
    if (i < 28)
    dev_warn(rtk.dev,
    "  x%02d-x%02d = %016llx %016llx %016llx %016llx\n",
    i, i + 3,
    regs.regs[i], regs.regs[i + 1],
    pub 3]): regs->regs[i + 2], regs->regs[i +,
    else
    dev_warn(rtk.dev,
    "  x%02d-x%02d = %016llx %016llx %016llx\n", i, i + 3,
    pub 2]): regs->regs[i], regs->regs[i + 1], regs->regs[i +,
    }
    pub "\n"): dev_warn(rtk->dev,,
    }
#[no_mangle]
pub unsafe extern "C" fn apple_rtkit_crashlog_dump(rtk: *mut apple_rtkit, bfr: *mut u8, size: usize) {
    void apple_rtkit_crashlog_dump(struct apple_rtkit *rtk, u8 *bfr, size_t size)
    {
    pub offset: usize,
    pub section_size: u32 section_fourcc,,
    pub header: apple_rtkit_crashlog_header,
    pub sizeof(header)): memcpy(&header, bfr,,
    if (header.fourcc != APPLE_RTKIT_CRASHLOG_HEADER) {
    dev_warn(rtk.dev, "RTKit: Expected crashlog header but got %x",
    }
    if (header.size > size) {
    dev_warn(rtk.dev, "RTKit: Crashlog size (%x) is too large",
    }
    pub header.size: size =,
    pub sizeof(header): offset =,
    while (offset < size) {
    pub 4): memcpy(&section_fourcc, bfr + offset,,
    pub 4): memcpy(&section_size, bfr + offset + 12,,
    switch (section_fourcc) {
    case APPLE_RTKIT_CRASHLOG_HEADER:
    pub reached"): dev_dbg(rtk->dev, "RTKit: End of crashlog,
    case APPLE_RTKIT_CRASHLOG_STR:
    apple_rtkit_crashlog_dump_str(rtk, bfr + offset + 16,
    case APPLE_RTKIT_CRASHLOG_VERSION:
    apple_rtkit_crashlog_dump_version(
    pub section_size): rtk, bfr + offset + 16,,
    case APPLE_RTKIT_CRASHLOG_MBOX:
    apple_rtkit_crashlog_dump_mailbox(
    pub section_size): rtk, bfr + offset + 16,,
    case APPLE_RTKIT_CRASHLOG_TIME:
    apple_rtkit_crashlog_dump_time(rtk, bfr + offset + 16,
    case APPLE_RTKIT_CRASHLOG_REGS:
    apple_rtkit_crashlog_dump_regs(rtk, bfr + offset + 16,
    default:
    dev_warn(rtk.dev,
    "RTKit: Unknown crashlog section: %x",
    }
    pub section_size: offset +=,
    }
    dev_warn(rtk.dev,
    pub present"): "RTKit: End of crashlog reached but no footer,
    }
