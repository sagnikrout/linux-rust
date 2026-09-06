//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/ccree/cc_driver.h
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
// Copyright (C) 2012-2019 ARM Limited (or its affiliates).
// \file cc_driver.h
// ARM CryptoCell Linux Crypto Driver
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cc_hw_rev {
    CC_HW_REV_630 = 630,
    CC_HW_REV_710 = 710,
    CC_HW_REV_712 = 712,
    CC_HW_REV_713 = 713
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cc_std_body {
    CC_STD_NIST = 0x1,
    CC_STD_OSCCA = 0x2,
    CC_STD_ALL = 0x3
}

pub const CC_PINS_FULL: c_uint = 0x0;
pub const CC_PINS_SLIM: c_uint = 0x9F;
// Maximum DMA mask supported by IP
pub const DMA_BIT_MASK_LEN: c_int = 48;

// Register name mangling macro

// TEE FIPS status interrupt

pub const CC_CRA_PRIO: c_int = 400;

pub const MAX_REQUEST_QUEUE_SIZE: c_int = 4096;
pub const MAX_MLLI_BUFF_SIZE: c_int = 2080;
// Definitions for HW descriptors DIN/DOUT fields
pub const NS_BIT: c_int = 1;
pub const AXI_ID: c_int = 0;
// AXI_ID is not actually the AXI ID of the transaction but the value of AXI_ID
// field in the HW descriptor. The DMA engine +8 that value.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cc_cpp_req {
    pub is_cpp: bool,
    pub alg: cc_cpp_alg,
    pub slot: u8,
}

pub const CC_MAX_IVGEN_DMA_ADDRESSES: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cc_crypto_req {
    pub err): *mut *mut *mut *mut void (user_cb)(struct device dev, void req, int,
    pub user_arg: *mut c_void,
    pub /: *mut *mut completion seq_compl; / request completion,
    pub cpp: cc_cpp_req,
}

//
// struct cc_drvdata - driver private data context
// @cc_base:	virt address of the CC registers
// @irq:	bitmap indicating source of last interrupt
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cc_drvdata {
    pub cc_base: *mut void __iomem,
    pub irq: c_int,
    pub /: *mut *mut completion hw_queue_avail; / wait for HW queue availability,
    pub plat_dev: *mut platform_device,
    pub mlli_sram_addr: u32,
    pub mlli_buffs_pool: *mut dma_pool,
    pub alg_list: list_head,
    pub hash_handle: *mut c_void,
    pub aead_handle: *mut c_void,
    pub request_mgr_handle: *mut c_void,
    pub fips_handle: *mut c_void,
    pub /: *mut *mut u32 sram_free_offset; / offset to non-allocated area in SRAM,
    pub /: *mut *mut *mut dentry dir; / for debugfs,
    pub clk: *mut clk,
    pub coherent: bool,
    pub hw_rev_name: *mut c_char,
    pub hw_rev: cc_hw_rev,
    pub axim_mon_offset: u32,
    pub sig_offset: u32,
    pub ver_offset: u32,
    pub std_bodies: c_int,
    pub sec_disabled: bool,
    pub comp_mask: u32,
    pub cache_params: u32,
    pub ace_const: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cc_crypto_alg {
    pub entry: list_head,
    pub cipher_mode: c_int,
    pub /: *mut *mut int flow_mode; / Note: currently, refers to the cipher mode only.,
    pub auth_mode: c_int,
    pub drvdata: *mut cc_drvdata,
    pub skcipher_alg: skcipher_alg,
    pub aead_alg: aead_alg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cc_alg_template {
    pub name: [c_char; CRYPTO_MAX_ALG_NAME],
    pub driver_name: [c_char; CRYPTO_MAX_ALG_NAME],
    pub blocksize: c_uint,
    pub skcipher: skcipher_alg,
    pub aead: aead_alg,
    pub template_u: },
    pub cipher_mode: c_int,
    pub /: *mut *mut int flow_mode; / Note: currently, refers to the cipher mode only.,
    pub auth_mode: c_int,
    pub min_hw_rev: u32,
    pub std_body: cc_std_body,
    pub sec_func: bool,
    pub data_unit: c_uint,
    pub drvdata: *mut cc_drvdata,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct async_gen_req_ctx {
    pub iv_dma_addr: dma_addr_t,
    pub iv: *mut u8,
    pub op_type: drv_crypto_direction,
}

extern "C" {
    pub fn __dump_byte_array(name: *const c_char, buf: *const u8, len: usize);
}
extern "C" {
    pub fn cc_wait_for_reset_completion(drvdata: *mut cc_drvdata) -> bool;
}
extern "C" {
    pub fn init_cc_regs(drvdata: *mut cc_drvdata) -> c_int;
}
extern "C" {
    pub fn fini_cc_regs(drvdata: *mut cc_drvdata);
}
extern "C" {
    pub fn cc_get_default_hash_len(drvdata: *mut cc_drvdata) -> c_uint;
}
extern "C" {
    pub fn ioread32(reg: drvdata->cc_base +) -> return;
}
