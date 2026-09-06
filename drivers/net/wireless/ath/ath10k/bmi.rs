//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath10k/bmi.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2005-2011 Atheros Communications Inc.
// Copyright (c) 2011-2015,2017 Qualcomm Atheros, Inc.
//

//
// Bootloader Messaging Interface (BMI)
//
// BMI is a very simple messaging interface used during initialization
// to read memory, write memory, execute code, and to define an
// application entry PC.
//
// It is used to download an application to QCA988x, to provide
// patches to code that is already resident on QCA988x, and generally
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
// "BMI Command Credit", using AR8K Counter #4.  As soon as the
// Target has completed a command, it issues another BMI Command
// Credit (so the Host can issue the next command).
//
// BMI handles all required Target-side cache flushing.
//
// Maximum data size used for BMI transfers
pub const BMI_MAX_DATA_SIZE: c_int = 256;
// len = cmd + addr + length

// Maximum data size used for large BMI transfers
pub const BMI_MAX_LARGE_DATA_SIZE: c_int = 2048;
// len = cmd + addr + length

// BMI Commands
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bmi_cmd_id {
    BMI_NO_COMMAND          = 0,
    BMI_DONE                = 1,
    BMI_READ_MEMORY         = 2,
    BMI_WRITE_MEMORY        = 3,
    BMI_EXECUTE             = 4,
    BMI_SET_APP_START       = 5,
    BMI_READ_SOC_REGISTER   = 6,
    BMI_READ_SOC_WORD       = 6,
    BMI_WRITE_SOC_REGISTER  = 7,
    BMI_WRITE_SOC_WORD      = 7,
    BMI_GET_TARGET_ID       = 8,
    BMI_GET_TARGET_INFO     = 8,
    BMI_ROMPATCH_INSTALL    = 9,
    BMI_ROMPATCH_UNINSTALL  = 10,
    BMI_ROMPATCH_ACTIVATE   = 11,
    BMI_ROMPATCH_DEACTIVATE = 12,
    BMI_LZ_STREAM_START     = 13, /* should be followed by LZ_DATA */
    BMI_LZ_DATA             = 14,
    BMI_NVRAM_PROCESS       = 15,
}

pub const BMI_NVRAM_SEG_NAME_SZ: c_int = 16;
pub const BMI_PARAM_GET_EEPROM_BOARD_ID: c_uint = 0x10;
pub const BMI_PARAM_GET_FLASH_BOARD_ID: c_uint = 0x8000;
pub const BMI_PARAM_FLASH_SECTION_ALL: c_uint = 0x10000;
// Dual-band Extended Board ID
pub const BMI_PARAM_GET_EXT_BOARD_ID: c_uint = 0x40000;
pub const ATH10K_BMI_EXT_BOARD_ID_SUPPORT: c_uint = 0x40000;
pub const ATH10K_BMI_BOARD_ID_FROM_OTP_MASK: c_uint = 0x7c00;
pub const ATH10K_BMI_BOARD_ID_FROM_OTP_LSB: c_int = 10;
pub const ATH10K_BMI_CHIP_ID_FROM_OTP_MASK: c_uint = 0x18000;
pub const ATH10K_BMI_CHIP_ID_FROM_OTP_LSB: c_int = 15;
pub const ATH10K_BMI_BOARD_ID_STATUS_MASK: c_uint = 0xff;
pub const ATH10K_BMI_EBOARD_ID_STATUS_MASK: c_uint = 0xff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmi_cmd {
    pub /: *mut *mut __le32 id; / enum bmi_cmd_id,
    pub done: },
    pub addr: __le32,
    pub len: __le32,
    pub read_mem: },
    pub addr: __le32,
    pub len: __le32,
    pub payload: [u8; ],
    pub write_mem: },
    pub addr: __le32,
    pub param: __le32,
    pub execute: },
    pub addr: __le32,
    pub set_app_start: },
    pub addr: __le32,
    pub read_soc_reg: },
    pub addr: __le32,
    pub value: __le32,
    pub write_soc_reg: },
    pub get_target_info: },
    pub rom_addr: __le32,
    pub /: *mut *mut __le32 ram_addr; / or value,
    pub size: __le32,
    pub /: *mut *mut __le32 activate; / 0=install, but dont activate,
    pub rompatch_install: },
    pub patch_id: __le32,
    pub rompatch_uninstall: },
    pub count: __le32,
    pub /: *mut *mut __le32 patch_ids[]; / length of @count,
    pub rompatch_activate: },
    pub count: __le32,
    pub /: *mut *mut __le32 patch_ids[]; / length of @count,
    pub rompatch_deactivate: },
    pub addr: __le32,
    pub lz_start: },
    pub /: *mut *mut __le32 len; / max BMI_MAX_DATA_SIZE,
    pub /: *mut *mut u8 payload[]; / length of @len,
    pub lz_data: },
    pub name: [u8; BMI_NVRAM_SEG_NAME_SZ],
    pub nvram_process: },
    pub payload: [u8; BMI_MAX_CMDBUF_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union bmi_resp {
    pub payload): DECLARE_FLEX_ARRAY(u8,,
    pub read_mem: },
    pub result: __le32,
    pub execute: },
    pub value: __le32,
    pub read_soc_reg: },
    pub len: __le32,
    pub version: __le32,
    pub type: __le32,
    pub get_target_info: },
    pub patch_id: __le32,
    pub rompatch_install: },
    pub patch_id: __le32,
    pub rompatch_uninstall: },
// 0 = nothing executed
// otherwise = NVRAM segment return value
//
    pub result: __le32,
    pub nvram_process: },
    pub payload: [u8; BMI_MAX_CMDBUF_SIZE],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmi_target_info {
    pub version: u32,
    pub type: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmi_segmented_file_header {
    pub magic_num: __le32,
    pub file_flags: __le32,
    pub data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmi_segmented_metadata {
    pub addr: __le32,
    pub length: __le32,
    pub data: [u8; ],
}

pub const BMI_SGMTFILE_MAGIC_NUM: c_uint = 0x544d4753 /* "SGMT" */;
pub const BMI_SGMTFILE_FLAG_COMPRESS: c_int = 1;
// Special values for bmi_segmented_metadata.length (all have high bit set)
// end of segmented data
pub const BMI_SGMTFILE_DONE: c_uint = 0xffffffff;
// Board Data segment
pub const BMI_SGMTFILE_BDDATA: c_uint = 0xfffffffe;
// set beginning address
pub const BMI_SGMTFILE_BEGINADDR: c_uint = 0xfffffffd;
// immediate function execution
pub const BMI_SGMTFILE_EXEC: c_uint = 0xfffffffc;
// in jiffies

pub const BMI_CE_NUM_TO_TARG: c_int = 0;
pub const BMI_CE_NUM_TO_HOST: c_int = 1;
extern "C" {
    pub fn ath10k_bmi_start(ar: *mut ath10k);
}
extern "C" {
    pub fn ath10k_bmi_done(ar: *mut ath10k) -> c_int;
}

// val = __le32_to_cpu(tmp);			\

extern "C" {
    pub fn ath10k_bmi_execute(ar: *mut ath10k, address: u32, param: u32, result: *mut u32) -> c_int;
}
extern "C" {
    pub fn ath10k_bmi_lz_stream_start(ar: *mut ath10k, address: u32) -> c_int;
}
extern "C" {
    pub fn ath10k_bmi_lz_data(ar: *mut ath10k, buffer: *const c_void, length: u32) -> c_int;
}
extern "C" {
    pub fn ath10k_bmi_read_soc_reg(ar: *mut ath10k, address: u32, reg_val: *mut u32) -> c_int;
}
extern "C" {
    pub fn ath10k_bmi_write_soc_reg(ar: *mut ath10k, address: u32, reg_val: u32) -> c_int;
}
extern "C" {
    pub fn ath10k_bmi_set_start(ar: *mut ath10k, address: u32) -> c_int;
}
