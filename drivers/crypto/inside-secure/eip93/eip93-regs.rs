//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/inside-secure/eip93/eip93-regs.h
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
// Copyright (C) 2019 - 2021
//
// Richard van Schagen <vschagen@icloud.com>
// Christian Marangi <ansuelsmth@gmail.com>
//
pub const EIP93_REG_PE_CTRL_STAT: c_uint = 0x0;

pub const EIP93_PE_CTRL_PE_EXT_ERR_PROCESSING: c_uint = 0x8;
pub const EIP93_PE_CTRL_PE_EXT_ERR_BLOCK_SIZE_ERR: c_uint = 0x7;
pub const EIP93_PE_CTRL_PE_EXT_ERR_INVALID_PK_LENGTH: c_uint = 0x6;
pub const EIP93_PE_CTRL_PE_EXT_ERR_ZERO_LENGTH: c_uint = 0x5;
pub const EIP93_PE_CTRL_PE_EXT_ERR_SPI: c_uint = 0x4;
pub const EIP93_PE_CTRL_PE_EXT_ERR_INVALID_CRYPTO_ALGO: c_uint = 0x3;
pub const EIP93_PE_CTRL_PE_EXT_ERR_INVALID_CRYPTO_OP: c_uint = 0x2;
pub const EIP93_PE_CTRL_PE_EXT_ERR_DESC_OWNER: c_uint = 0x1;
pub const EIP93_PE_CTRL_PE_EXT_ERR_BUS: c_uint = 0x0;

pub const EIP93_PE_CTRL_PE_READY: c_uint = 0x2;
pub const EIP93_PE_CTRL_HOST_READY: c_uint = 0x1;
pub const EIP93_REG_PE_SOURCE_ADDR: c_uint = 0x4;
pub const EIP93_REG_PE_DEST_ADDR: c_uint = 0x8;
pub const EIP93_REG_PE_SA_ADDR: c_uint = 0xc;
pub const EIP93_REG_PE_ADDR: c_uint = 0x10 /* STATE_ADDR */;
//
// Special implementation for user ID
// user_id in eip93_descriptor is used to identify the
// descriptor and is opaque and can be used by the driver
// in custom way.
//
// The usage of this should be to put an address to the crypto
// request struct from the kernel but this can't work in 64bit
// world.
//
// Also it's required to put some flags to identify the last
// descriptor.
//
// To handle this, split the u32 in 2 part:
// - 31:16 descriptor flags
// - 15:0 IDR to connect the crypto request address
//
pub const EIP93_REG_PE_USER_ID: c_uint = 0x18;

pub const EIP93_REG_PE_LENGTH: c_uint = 0x1c;

pub const EIP93_PE_LENGTH_PE_READY: c_uint = 0x2;
pub const EIP93_PE_LENGTH_HOST_READY: c_uint = 0x1;

// PACKET ENGINE RING configuration registers
pub const EIP93_REG_PE_CDR_BASE: c_uint = 0x80;
pub const EIP93_REG_PE_RDR_BASE: c_uint = 0x84;
pub const EIP93_REG_PE_RING_CONFIG: c_uint = 0x88;

// Absent in later revision of eip93
// #define   EIP93_PE_RING_OFFSET		GENMASK(23, 15)

pub const EIP93_REG_PE_RING_THRESH: c_uint = 0x8c;

pub const EIP93_REG_PE_CD_COUNT: c_uint = 0x90;

//
// In the same register, writing a value in GENMASK(7, 0) will
// increment the descriptor count and start DMA action.
//

pub const EIP93_REG_PE_RD_COUNT: c_uint = 0x94;

//
// In the same register, writing a value in GENMASK(7, 0) will
// increment the descriptor count and start DMA action.
//

pub const EIP93_REG_PE_RING_RW_PNTR: c_uint = 0x98 /* RING_PNTR */;
// PACKET ENGINE  configuration registers
pub const EIP93_REG_PE_CONFIG: c_uint = 0x100;

pub const EIP93_REG_PE_STATUS: c_uint = 0x104;
pub const EIP93_REG_PE_BUF_THRESH: c_uint = 0x10c;

pub const EIP93_REG_PE_INBUF_COUNT: c_uint = 0x110;
pub const EIP93_REG_PE_OUTBUF_COUNT: c_uint = 0x114;
pub const EIP93_REG_PE_BUF_RW_PNTR: c_uint = 0x118 /* BUF_PNTR */;
// PACKET ENGINE endian config
pub const EIP93_REG_PE_ENDIAN_CONFIG: c_uint = 0x1cc;
pub const EIP93_AIROHA_REG_PE_ENDIAN_CONFIG: c_uint = 0x1d0;

//
// Byte goes 2 and 2 and are referenced by ID
// Split GENMASK(7, 0) in 4 part, one for each byte.
// Example LITTLE ENDIAN: Example BIG ENDIAN
// GENMASK(7, 6) 0x3	  GENMASK(7, 6) 0x0
// GENMASK(5, 4) 0x2	  GENMASK(7, 6) 0x1
// GENMASK(3, 2) 0x1	  GENMASK(3, 2) 0x2
// GENMASK(1, 0) 0x0	  GENMASK(1, 0) 0x3
//
pub const EIP93_PE_ENDIAN_BYTE0: c_uint = 0x0;
pub const EIP93_PE_ENDIAN_BYTE1: c_uint = 0x1;
pub const EIP93_PE_ENDIAN_BYTE2: c_uint = 0x2;
pub const EIP93_PE_ENDIAN_BYTE3: c_uint = 0x3;
// EIP93 CLOCK control registers
pub const EIP93_REG_PE_CLOCK_CTRL: c_uint = 0x1e8;

// EIP93 Device Option and Revision Register
pub const EIP93_REG_PE_OPTION_1: c_uint = 0x1f4;

pub const EIP93_REG_PE_OPTION_0: c_uint = 0x1f8;
pub const EIP93_REG_PE_REVISION: c_uint = 0x1fc;

// EIP93 Interrupt Control Register
pub const EIP93_REG_INT_UNMASK_STAT: c_uint = 0x200;
pub const EIP93_REG_INT_MASK_STAT: c_uint = 0x204;
pub const EIP93_REG_INT_CLR: c_uint = 0x204;
pub const EIP93_REG_INT_MASK: c_uint = 0x208 /* INT_EN */;
// Each int reg have the same bitmap

pub const EIP93_REG_INT_CFG: c_uint = 0x20c;

pub const EIP93_REG_MASK_ENABLE: c_uint = 0x210;
pub const EIP93_REG_MASK_DISABLE: c_uint = 0x214;
// EIP93 SA Record register
pub const EIP93_REG_SA_CMD_0: c_uint = 0x400;

pub const EIP93_SA_CMD_OPCODE_BASIC_OUT_PRNG: c_uint = 0x7;
pub const EIP93_SA_CMD_OPCODE_BASIC_OUT_HASH: c_uint = 0x3;
pub const EIP93_SA_CMD_OPCODE_BASIC_OUT_ENC_HASH: c_uint = 0x1;
pub const EIP93_SA_CMD_OPCODE_BASIC_OUT_ENC: c_uint = 0x0;
pub const EIP93_SA_CMD_OPCODE_BASIC_IN_HASH: c_uint = 0x3;
pub const EIP93_SA_CMD_OPCODE_BASIC_IN_HASH_DEC: c_uint = 0x1;
pub const EIP93_SA_CMD_OPCODE_BASIC_IN_DEC: c_uint = 0x0;
pub const EIP93_SA_CMD_OPCODE_PROTOCOL_OUT_ESP: c_uint = 0x0;
pub const EIP93_SA_CMD_OPCODE_PROTOCOL_OUT_SSL: c_uint = 0x4;
pub const EIP93_SA_CMD_OPCODE_PROTOCOL_OUT_TLS: c_uint = 0x5;
pub const EIP93_SA_CMD_OPCODE_PROTOCOL_OUT_SRTP: c_uint = 0x7;
pub const EIP93_SA_CMD_OPCODE_PROTOCOL_IN_ESP: c_uint = 0x0;
pub const EIP93_SA_CMD_OPCODE_PROTOCOL_IN_SSL: c_uint = 0x2;
pub const EIP93_SA_CMD_OPCODE_PROTOCOL_IN_TLS: c_uint = 0x3;
pub const EIP93_SA_CMD_OPCODE_PROTOCOL_IN_SRTP: c_uint = 0x7;
pub const EIP93_SA_CMD_OPCODE_EXT_OUT_DTSL: c_uint = 0x1;
pub const EIP93_SA_CMD_OPCODE_EXT_OUT_SSL: c_uint = 0x4;
pub const EIP93_SA_CMD_OPCODE_EXT_OUT_TLSV10: c_uint = 0x5;
pub const EIP93_SA_CMD_OPCODE_EXT_OUT_TLSV11: c_uint = 0x6;
pub const EIP93_SA_CMD_OPCODE_EXT_IN_DTSL: c_uint = 0x1;
pub const EIP93_SA_CMD_OPCODE_EXT_IN_SSL: c_uint = 0x4;
pub const EIP93_SA_CMD_OPCODE_EXT_IN_TLSV10: c_uint = 0x5;
pub const EIP93_SA_CMD_OPCODE_EXT_IN_TLSV11: c_uint = 0x6;
pub const EIP93_REG_SA_CMD_1: c_uint = 0x404;

// This mask can be either used for ARC4 or AES

// This mask can be either used for ARC4 or AES

// AES or DES operations

// ARC4 operations

// State save register
pub const EIP93_REG_STATE_IV_0: c_uint = 0x500;
pub const EIP93_REG_STATE_IV_1: c_uint = 0x504;
pub const EIP93_REG_PE_ARC4STATE: c_uint = 0x700;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sa_record {
    pub sa_cmd0_word: u32,
    pub sa_cmd1_word: u32,
    pub sa_key: [u32; 8],
    pub sa_i_digest: [u8; 32],
    pub sa_o_digest: [u8; 32],
    pub sa_spi: u32,
    pub sa_seqnum: [u32; 2],
    pub sa_seqmum_mask: [u32; 2],
    pub sa_nonce: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sa_state {
    pub state_iv: [u32; 4],
    pub state_byte_cnt: [u32; 2],
    pub state_i_digest: [u8; 32],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eip93_descriptor {
    pub pe_ctrl_stat_word: u32,
    pub src_addr: u32,
    pub dst_addr: u32,
    pub sa_addr: u32,
    pub state_addr: u32,
    pub arc4_addr: u32,
    pub user_id: u32,
    pub pe_length_word: u32,
    pub __packed: },
