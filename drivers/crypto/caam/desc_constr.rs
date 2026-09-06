//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/caam/desc_constr.h
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
// caam descriptor construction helper functions
//
// Copyright 2008-2012 Freescale Semiconductor, Inc.
// Copyright 2019, 2025 NXP
//

//
// The CAAM QI hardware constructs a job descriptor which points
// to shared descriptor (as pointed by context_a of FQ to CAAM).
// When the job descriptor is executed by deco, the whole job
// descriptor together with shared descriptor gets loaded in
// deco buffer which is 64 words long (each 32-bit).
//
// The job descriptor constructed by QI hardware has layout:
//
// HEADER		(1 word)
// Shdesc ptr	(1 or 2 words)
// SEQ_OUT_PTR	(1 word)
// Out ptr		(1 or 2 words)
// Out length	(1 word)
// SEQ_IN_PTR	(1 word)
// In ptr		(1 or 2 words)
// In length	(1 word)
//
// The shdesc ptr is used to fetch shared descriptor contents
// into deco buffer.
//
// Apart from shdesc contents, the total number of words that
// get loaded in deco buffer are '8' or '11'. The remaining words
// in deco buffer can be used for storing shared descriptor.
//

// Macro flag: #define PRINT_POS

//
// HW fetches 4 S/G table entries at a time, irrespective of how many entries
// are in the table. It's SW's responsibility to make sure these accesses
// do not have side effects.
//
extern "C" {
    pub fn ALIGN(_arg: sg_nents, _arg: 4) -> return;
}
// desc = cpu_to_caam32((options | HDR_ONE) + 1);
// offset = cpu_to_caam_dma(ptr);
// Avoid gcc warning: memcpy with data == NULL
// cmd = cpu_to_caam32(command);

// Only 32-bit alignment is guaranteed in descriptor buffer
// offset = cpu_to_caam32(lower_32_bits(data));
// (++offset) = cpu_to_caam32(upper_32_bits(data));
// offset = cpu_to_caam32(upper_32_bits(data));
// (++offset) = cpu_to_caam32(lower_32_bits(data));
// Write command without affecting header, and return pointer to next word
// desc = cpu_to_caam32(command);
// Write length after pointer, rather than inside command

// jump_cmd = cpu_to_caam32(caam32_to_cpu(*jump_cmd) |
// move_cmd = cpu_to_caam32(val);

// The following options do not require pointer

//
// Determine whether to store length internally or externally depending on
// the size of its type
//

//
// 2nd variant for commands whose specified immediate length differs
// from length of immediate data provided, e.g., split keys
//

//
// ee - endianness
// size - size of immediate type in bytes
//

//
// Append math command. Only the last part of destination and source need to
// be specified
//

// Exactly one source is IMM. Data is passed in as u32 value

// Exactly one source is IMM. Data is passed in as u64 value

//
// struct alginfo - Container for algorithm details
// @algtype: algorithm selector; for valid values, see documentation of the
// functions where it is used.
// @keylen: length of the provided algorithm key, in bytes
// @keylen_pad: padded length of the provided algorithm key, in bytes
// @key_dma: dma (bus) address where algorithm key resides
// @protected_key_dma: dma (bus) address where protected key resides
// @key_virt: virtual address where algorithm key resides
// @key_inline: true - key can be inlined in the descriptor; false - key is
// referenced by the descriptor
// @plain_keylen: size of the key to be loaded by the CAAM
// @key_cmd_opt: optional parameters for KEY command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct alginfo {
    pub algtype: u32,
    pub keylen: c_uint,
    pub keylen_pad: c_uint,
    pub key_dma: dma_addr_t,
    pub protected_key_dma: dma_addr_t,
    pub key_virt: *const c_void,
    pub key_inline: bool,
    pub plain_keylen: u32,
    pub key_cmd_opt: u32,
}

//
// desc_inline_query() - Provide indications on which data items can be inlined
// and which shall be referenced in a shared descriptor.
// @sd_base_len: Shared descriptor base length - bytes consumed by the commands,
// excluding the data items to be inlined (or corresponding
// pointer if an item is not inlined). Each cnstr_* function that
// generates descriptors should have a define mentioning
// corresponding length.
// @jd_len: Maximum length of the job descriptor(s) that will be used
// together with the shared descriptor.
// @data_len: Array of lengths of the data items trying to be inlined
// @inl_mask: 32bit mask with bit x = 1 if data item x can be inlined, 0
// otherwise.
// @count: Number of data items (size of @data_len array); must be <= 32
//
// Return: 0 if data can be inlined / referenced, negative value if not. If 0,
// check @inl_mask for details.
//
// inl_mask = 0;
// inl_mask |= (1 << i);
//
// append_proto_dkp - Derived Key Protocol (DKP): key -> split key
// @desc: pointer to buffer used for descriptor construction
// @adata: pointer to authentication transform definitions.
// keylen should be the length of initial key, while keylen_pad
// the length of the derived (split) key.
// Valid algorithm values - one of OP_ALG_ALGSEL_{MD5, SHA1, SHA224,
// SHA256, SHA384, SHA512}.
//
// Quick & dirty translation from OP_ALG_ALGSEL_{MD5, SHA*}
// to OP_PCLID_DKP_{MD5, SHA*}
//
// Reserve space in descriptor buffer for the derived key
