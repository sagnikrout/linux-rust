//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/keembay/ocs-aes.h
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
// Intel Keem Bay OCS AES Crypto Driver.
//
// Copyright (C) 2018-2020 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocs_cipher {
    OCS_AES = 0,
    OCS_SM4 = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocs_mode {
    OCS_MODE_ECB = 0,
    OCS_MODE_CBC = 1,
    OCS_MODE_CTR = 2,
    OCS_MODE_CCM = 6,
    OCS_MODE_GCM = 7,
    OCS_MODE_CTS = 9,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocs_instruction {
    OCS_ENCRYPT = 0,
    OCS_DECRYPT = 1,
    OCS_EXPAND  = 2,
    OCS_BYPASS  = 3,
}

//
// struct ocs_aes_dev - AES device context.
// @list:			List head for insertion into device list hold
// by driver.
// @dev:			OCS AES device.
// @irq:			IRQ number.
// @base_reg:			IO base address of OCS AES.
// @irq_copy_completion:	Completion to indicate IRQ has been triggered.
// @dma_err_mask:		Error reported by OCS DMA interrupts.
// @engine:			Crypto engine for the device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocs_aes_dev {
    pub list: list_head,
    pub dev: *mut device,
    pub irq: c_int,
    pub base_reg: *mut void __iomem,
    pub irq_completion: completion,
    pub dma_err_mask: u32,
    pub engine: *mut crypto_engine,
}

//
// struct ocs_dll_desc - Descriptor of an OCS DMA Linked List.
// @vaddr:	Virtual address of the linked list head.
// @dma_addr:	DMA address of the linked list head.
// @size:	Size (in bytes) of the linked list.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocs_dll_desc {
    pub vaddr: *mut c_void,
    pub dma_addr: dma_addr_t,
    pub size: usize,
}

//
// ocs_aes_bypass_op() - Use OCS DMA to copy data.
// @aes_dev:            The OCS AES device to use.
// @dst_dma_list:	The OCS DMA list mapping the memory where input data
// will be copied to.
// @src_dma_list:	The OCS DMA list mapping input data.
// @src_size:		The amount of data to copy.
//
extern "C" {
    pub fn ocs_aes_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
