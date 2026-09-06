//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ipa/ipa_cmd.h
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
// Copyright (c) 2012-2018, The Linux Foundation. All rights reserved.
// Copyright (C) 2019-2024 Linaro Ltd.
//

//
// enum ipa_cmd_opcode:	IPA immediate commands
//
// @IPA_CMD_IP_V4_FILTER_INIT:	Initialize IPv4 filter table
// @IPA_CMD_IP_V6_FILTER_INIT:	Initialize IPv6 filter table
// @IPA_CMD_IP_V4_ROUTING_INIT:	Initialize IPv4 routing table
// @IPA_CMD_IP_V6_ROUTING_INIT:	Initialize IPv6 routing table
// @IPA_CMD_HDR_INIT_LOCAL:	Initialize IPA-local header memory
// @IPA_CMD_REGISTER_WRITE:	Register write performed by IPA
// @IPA_CMD_IP_PACKET_INIT:	Set up next packet's destination endpoint
// @IPA_CMD_DMA_SHARED_MEM:	DMA command performed by IPA
// @IPA_CMD_IP_PACKET_TAG_STATUS: Have next packet generate tag * status
// @IPA_CMD_NONE:		Special (invalid) "not a command" value
//
// All immediate commands are issued using the AP command TX endpoint.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipa_cmd_opcode {
    IPA_CMD_NONE			= 0x0,
    IPA_CMD_IP_V4_FILTER_INIT	= 0x3,
    IPA_CMD_IP_V6_FILTER_INIT	= 0x4,
    IPA_CMD_IP_V4_ROUTING_INIT	= 0x7,
    IPA_CMD_IP_V6_ROUTING_INIT	= 0x8,
    IPA_CMD_HDR_INIT_LOCAL		= 0x9,
    IPA_CMD_REGISTER_WRITE		= 0xc,
    IPA_CMD_IP_PACKET_INIT		= 0x10,
    IPA_CMD_DMA_SHARED_MEM		= 0x13,
    IPA_CMD_IP_PACKET_TAG_STATUS	= 0x14,
}

//
// ipa_cmd_table_init_valid() - Validate a memory region holding a table
// @ipa:	- IPA pointer
// @mem:	- IPA memory region descriptor
// @route:	- Whether the region holds a route or filter table
//
// Return:	true if region is valid, false otherwise
//
// ipa_cmd_pool_init() - initialize command channel pools
// @channel:	AP->IPA command TX GSI channel pointer
// @tre_count:	Number of pool elements to allocate
//
// Return:	0 if successful, or a negative error code
//
extern "C" {
    pub fn ipa_cmd_pool_init(channel: *mut gsi_channel, tre_count: u32) -> c_int;
}
//
// ipa_cmd_pool_exit() - Inverse of ipa_cmd_pool_init()
// @channel:	AP->IPA command TX GSI channel pointer
//
extern "C" {
    pub fn ipa_cmd_pool_exit(channel: *mut gsi_channel);
}
//
// ipa_cmd_table_init_add() - Add table init command to a transaction
// @trans:	GSI transaction
// @opcode:	IPA immediate command opcode
// @size:	Size of non-hashed routing table memory
// @offset:	Offset in IPA shared memory of non-hashed routing table memory
// @addr:	DMA address of non-hashed table data to write
// @hash_size:	Size of hashed routing table memory
// @hash_offset: Offset in IPA shared memory of hashed routing table memory
// @hash_addr:	DMA address of hashed table data to write
//
// If hash_size is 0, hash_offset and hash_addr are ignored.
//
// ipa_cmd_hdr_init_local_add() - Add a header init command to a transaction
// @trans:	GSI transaction
// @offset:	Offset of header memory in IPA local space
// @size:	Size of header memory
// @addr:	DMA address of buffer to be written from
//
// Defines and fills the location in IPA memory to use for headers.
//
// ipa_cmd_register_write_add() - Add a register write command to a transaction
// @trans:	GSI transaction
// @offset:	Offset of register to be written
// @value:	Value to be written
// @mask:	Mask of bits in register to update with bits from value
// @clear_full: Pipeline clear option; true means full pipeline clear
//
// ipa_cmd_dma_shared_mem_add() - Add a DMA memory command to a transaction
// @trans:	GSI transaction
// @offset:	Offset of IPA memory to be read or written
// @size:	Number of bytes of memory to be transferred
// @addr:	DMA address of buffer to be read into or written from
// @toward_ipa:	true means write to IPA memory; false means read
//
// ipa_cmd_pipeline_clear_add() - Add pipeline clear commands to a transaction
// @trans:	GSI transaction
//
extern "C" {
    pub fn ipa_cmd_pipeline_clear_add(trans: *mut gsi_trans);
}
//
// ipa_cmd_pipeline_clear_count() - # commands required to clear pipeline
//
// Return:	The number of elements to allocate in a transaction
// to hold commands to clear the pipeline
//
extern "C" {
    pub fn ipa_cmd_pipeline_clear_count() -> u32;
}
//
// ipa_cmd_pipeline_clear_wait() - Wait pipeline clear to complete
// @ipa:	- IPA pointer
//
extern "C" {
    pub fn ipa_cmd_pipeline_clear_wait(ipa: *mut ipa);
}
//
// ipa_cmd_trans_alloc() - Allocate a transaction for the command TX endpoint
// @ipa:	IPA pointer
// @tre_count:	Number of elements in the transaction
//
// Return:	A GSI transaction structure, or a null pointer if all
// available transactions are in use
//
// ipa_cmd_init() - Initialize IPA immediate commands
// @ipa:	- IPA pointer
//
// Return:	0 if successful, or a negative error code
//
// There is no need for a matching ipa_cmd_exit() function.
//
extern "C" {
    pub fn ipa_cmd_init(ipa: *mut ipa) -> c_int;
}
