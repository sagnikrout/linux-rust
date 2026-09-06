//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/engine/sec/fuc/g98.fuc0s.h
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


// SPDX-License-Identifier: MIT
// 0x0000: ctx_dma
// 0x0000: ctx_dma_query
// 0x0004: ctx_dma_src
// 0x0008: ctx_dma_dst
// 0x000c: ctx_query_address_high
// 0x0010: ctx_query_address_low
// 0x0014: ctx_query_counter
// 0x0018: ctx_cond_address_high
// 0x001c: ctx_cond_address_low
// 0x0020: ctx_cond_off
// 0x0024: ctx_src_address_high
// 0x0028: ctx_src_address_low
// 0x002c: ctx_dst_address_high
// 0x0030: ctx_dst_address_low
// 0x0034: ctx_mode
// 0x0040: ctx_key
// 0x0050: ctx_iv
// 0x0080: swap
// 0x00a0: common_cmd_dtable
// 0x00e0: engine_cmd_dtable
// 0x0150: sec_dtable
// 0x002f: spin
// 0x0035: ih
// 0x0075: ctxload
// 0x008c: ctxload_dma_loop
// 0x009f: dummyload
// 0x00a5: noctx
// 0x0103: dma_cmd
// 0x0123: dtable_cmd
// 0x015b: cmd_setctx
// 0x0161: invalid_bitfield
// 0x0164: dispatch_error
// 0x0164: illegal_mthd
// 0x0174: im_loop
// 0x0180: cmddone
// 0x018a: nocmd
// 0x0192: cmd_query_get
// 0x019c: ptimer_retry
// 0x01d7: cmd_cond_mode
// 0x01f2: return
// 0x01f4: cmd_cond_mode_queryful
// 0x022c: cmd_cond_mode_double
// 0x0260: cmd_wrcache_flush
// 0x0271: sec_cmd_mode
// 0x0283: sec_cmd_mode_return
// 0x0285: sec_cmd_length
// 0x0321: sec_copy_prep
// 0x032f: sec_store_prep
// 0x0339: sec_ecb_e_prep
// 0x034b: sec_ecb_d_prep
// 0x0361: sec_cbc_e_prep
// 0x0377: sec_cbc_d_prep
// 0x0395: sec_pcbc_e_prep
// 0x03af: sec_pcbc_d_prep
// 0x03cd: sec_cfb_e_prep
// 0x03e3: sec_cfb_d_prep
// 0x03f9: sec_ofb_prep
// 0x040f: sec_ctr_prep
// 0x0429: sec_cbc_mac_prep
// 0x043b: sec_cmac_finish_complete_prep
// 0x045d: sec_cmac_finish_partial_prep
// 0x0483: sec_do_in
// 0x0490: sec_do_in_loop
// 0x04b1: sec_do_out
// 0x04be: sec_do_out_loop
// 0x04db: sec_do_inout
// 0x04e5: sec_do_inout_loop
