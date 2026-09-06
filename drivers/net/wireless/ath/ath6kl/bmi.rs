//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath6kl/bmi.h
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


//
// Copyright (c) 2004-2011 Atheros Communications Inc.
// Copyright (c) 2011 Qualcomm Atheros, Inc.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//
// Bootloader Messaging Interface (BMI)
//
// BMI is a very simple messaging interface used during initialization
// to read memory, write memory, execute code, and to define an
// application entry PC.
//
// It is used to download an application to ATH6KL, to provide
// patches to code that is already resident on ATH6KL, and generally
// to examine and modify state.  The Host has an opportunity to use
// BMI only once during bootup.  Once the Host issues a BMI_DONE
// command, this opportunity ends.
//
// The Host writes BMI requests to mailbox0, and reads BMI responses
// from mailbox0.   BMI requests all begin with a command
// (see below for specific commands), and are followed by
// command-specific data.
//
// Flow control:
// The Host can only issue a command once the Target gives it a
// "BMI Command Credit", using ATH6KL Counter #4.  As soon as the
// Target has completed a command, it issues another BMI Command
// Credit (so the Host can issue the next command).
//
// BMI handles all required Target-side cache flushing.
//
// BMI Commands
pub const BMI_NO_COMMAND: c_int = 0;
pub const BMI_DONE: c_int = 1;
//
// Semantics: Host is done using BMI
// Request format:
// u32 command (BMI_DONE)
// Response format: none
//
pub const BMI_READ_MEMORY: c_int = 2;
//
// Semantics: Host reads ATH6KL memory
// Request format:
// u32 command (BMI_READ_MEMORY)
// u32 address
// u32 length, at most BMI_DATASZ_MAX
// Response format:
// u8 data[length]
//
pub const BMI_WRITE_MEMORY: c_int = 3;
//
// Semantics: Host writes ATH6KL memory
// Request format:
// u32 command (BMI_WRITE_MEMORY)
// u32 address
// u32 length, at most BMI_DATASZ_MAX
// u8 data[length]
// Response format: none
//
pub const BMI_EXECUTE: c_int = 4;
//
// Semantics: Causes ATH6KL to execute code
// Request format:
// u32 command (BMI_EXECUTE)
// u32 address
// u32 parameter
// Response format:
// u32 return value
//
pub const BMI_SET_APP_START: c_int = 5;
//
// Semantics: Set Target application starting address
// Request format:
// u32 command (BMI_SET_APP_START)
// u32 address
// Response format: none
//
pub const BMI_READ_SOC_REGISTER: c_int = 6;
//
// Semantics: Read a 32-bit Target SOC register.
// Request format:
// u32 command (BMI_READ_REGISTER)
// u32 address
// Response format:
// u32 value
//
pub const BMI_WRITE_SOC_REGISTER: c_int = 7;
//
// Semantics: Write a 32-bit Target SOC register.
// Request format:
// u32 command (BMI_WRITE_REGISTER)
// u32 address
// u32 value
//
// Response format: none
//
pub const BMI_GET_TARGET_ID: c_int = 8;
pub const BMI_GET_TARGET_INFO: c_int = 8;
//
// Semantics: Fetch the 4-byte Target information
// Request format:
// u32 command (BMI_GET_TARGET_ID/INFO)
// Response format1 (old firmware):
// u32 TargetVersionID
// Response format2 (newer firmware):
// u32 TARGET_VERSION_SENTINAL
// struct bmi_target_info;
//
pub const TARGET_VERSION_SENTINAL: c_uint = 0xffffffff;
pub const TARGET_TYPE_AR6003: c_int = 3;
pub const TARGET_TYPE_AR6004: c_int = 5;
pub const BMI_ROMPATCH_INSTALL: c_int = 9;
//
// Semantics: Install a ROM Patch.
// Request format:
// u32 command (BMI_ROMPATCH_INSTALL)
// u32 Target ROM Address
// u32 Target RAM Address or Value (depending on Target Type)
// u32 Size, in bytes
// u32 Activate? 1-->activate;
// 0-->install but do not activate
// Response format:
// u32 PatchID
//
pub const BMI_ROMPATCH_UNINSTALL: c_int = 10;
//
// Semantics: Uninstall a previously-installed ROM Patch,
// automatically deactivating, if necessary.
// Request format:
// u32 command (BMI_ROMPATCH_UNINSTALL)
// u32 PatchID
//
// Response format: none
//
pub const BMI_ROMPATCH_ACTIVATE: c_int = 11;
//
// Semantics: Activate a list of previously-installed ROM Patches.
// Request format:
// u32 command (BMI_ROMPATCH_ACTIVATE)
// u32 rompatch_count
// u32 PatchID[rompatch_count]
//
// Response format: none
//
pub const BMI_ROMPATCH_DEACTIVATE: c_int = 12;
//
// Semantics: Deactivate a list of active ROM Patches.
// Request format:
// u32 command (BMI_ROMPATCH_DEACTIVATE)
// u32 rompatch_count
// u32 PatchID[rompatch_count]
//
// Response format: none
//
pub const BMI_LZ_STREAM_START: c_int = 13;
//
// Semantics: Begin an LZ-compressed stream of input
// which is to be uncompressed by the Target to an
// output buffer at address.  The output buffer must
// be sufficiently large to hold the uncompressed
// output from the compressed input stream.  This BMI
// command should be followed by a series of 1 or more
// BMI_LZ_DATA commands.
// u32 command (BMI_LZ_STREAM_START)
// u32 address
// Note: Not supported on all versions of ROM firmware.
//
pub const BMI_LZ_DATA: c_int = 14;
//
// Semantics: Host writes ATH6KL memory with LZ-compressed
// data which is uncompressed by the Target.  This command
// must be preceded by a BMI_LZ_STREAM_START command. A series
// of BMI_LZ_DATA commands are considered part of a single
// input stream until another BMI_LZ_STREAM_START is issued.
// Request format:
// u32 command (BMI_LZ_DATA)
// u32 length (of compressed data),
// at most BMI_DATASZ_MAX
// u8 CompressedData[length]
// Response format: none
// Note: Not supported on all versions of ROM firmware.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath6kl_bmi_target_info {
    pub /: *mut *mut __le32 byte_count; / size of this structure,
    pub /: *mut *mut __le32 version; / target version id,
    pub /: *mut *mut __le32 type; / target type,
    pub __packed: },

    pub \: u32 addr;,
    pub \: __le32 v;,
    pub \: addr = ath6kl_get_hi_item_addr(ar, HI_ITEM(item));,
    pub \: v = cpu_to_le32(val);,
    pub \: *mut *mut ath6kl_bmi_write(ar, addr, (u8 ) &v, sizeof(v));,

    pub \: *mut *mut u32 addr, check_type = val;,
    pub \: __le32 tmp;,
    pub \: int ret;,
    pub \: (void) (check_type == val);,
    pub \: addr = ath6kl_get_hi_item_addr(ar, HI_ITEM(item));,
    pub \: *mut *mut ret = ath6kl_bmi_read(ar, addr, (u8 ) &tmp, 4);,
// val = le32_to_cpu(tmp);			\
    pub \: ret;,
    pub ar): *mut int ath6kl_bmi_init(struct ath6kl,
    pub ar): *mut void ath6kl_bmi_cleanup(struct ath6kl,
    pub ar): *mut void ath6kl_bmi_reset(struct ath6kl,
    pub ar): *mut int ath6kl_bmi_done(struct ath6kl,
    pub targ_info): *mut ath6kl_bmi_target_info,
    pub len): *mut *mut *mut int ath6kl_bmi_read(struct ath6kl ar, u32 addr, u8 buf, u32,
    pub len): *mut *mut *mut int ath6kl_bmi_write(struct ath6kl ar, u32 addr, u8 buf, u32,
    pub param): *mut u32 addr, u32,
    pub addr): u32,
    pub param): *mut *mut int ath6kl_bmi_reg_read(struct ath6kl ar, u32 addr, u32,
    pub param): *mut *mut int ath6kl_bmi_reg_write(struct ath6kl ar, u32 addr, u32,
    pub len): *mut *mut u8 buf, u32,
    pub addr): u32,
    pub len): *mut *mut u32 addr, u8 buf, u32,
