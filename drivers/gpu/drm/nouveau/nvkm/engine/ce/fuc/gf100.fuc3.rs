//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/engine/ce/fuc/gf100.fuc3.h
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
// 0x0000: ctx_object
// 0x0004: ctx_query_address_high
// 0x0008: ctx_query_address_low
// 0x000c: ctx_query_counter
// 0x0010: ctx_src_address_high
// 0x0014: ctx_src_address_low
// 0x0018: ctx_src_pitch
// 0x001c: ctx_src_tile_mode
// 0x0020: ctx_src_xsize
// 0x0024: ctx_src_ysize
// 0x0028: ctx_src_zsize
// 0x002c: ctx_src_zoff
// 0x0030: ctx_src_xoff
// 0x0034: ctx_src_yoff
// 0x0038: ctx_src_cpp
// 0x003c: ctx_dst_address_high
// 0x0040: ctx_dst_address_low
// 0x0044: ctx_dst_pitch
// 0x0048: ctx_dst_tile_mode
// 0x004c: ctx_dst_xsize
// 0x0050: ctx_dst_ysize
// 0x0054: ctx_dst_zsize
// 0x0058: ctx_dst_zoff
// 0x005c: ctx_dst_xoff
// 0x0060: ctx_dst_yoff
// 0x0064: ctx_dst_cpp
// 0x0068: ctx_format
// 0x006c: ctx_swz_const0
// 0x0070: ctx_swz_const1
// 0x0074: ctx_xcnt
// 0x0078: ctx_ycnt
// 0x0100: dispatch_table
// 0x0000: main
// 0x002f: spin
// 0x0035: ih
// 0x0041: ih_no_chsw
// 0x004b: ih_no_cmd
// 0x0053: swctx
// 0x00c3: swctx_load
// 0x00c6: swctx_done
// 0x00ca: chsw
// 0x00eb: chsw_no_unload
// 0x00fa: chsw_finish_load
// 0x0102: dispatch
// 0x011b: dispatch_loop
// 0x013f: dispatch_valid_mthd
// 0x0166: dispatch_cmd
// 0x0171: dispatch_invalid_bitfield
// 0x0174: dispatch_illegal_mthd
// 0x0177: dispatch_error
// 0x0187: hostirq_wait
// 0x0193: dispatch_done
// 0x019f: cmd_nop
// 0x01a1: cmd_pm_trigger
// 0x01af: cmd_exec_set_format
// 0x01da: ncomp_loop
// 0x01e2: bpc_loop
// 0x01f4: cmp_c0
// 0x0200: cmp_c1
// 0x020f: cmp_zero
// 0x0213: bpc_next
// 0x023c: dst_xcnt
// 0x029c: cmd_exec_set_surface_tiled
// 0x02b7: xtile64
// 0x02c3: xtileok
// 0x03a8: cmd_exec_set_surface_linear
// 0x03d1: cmd_exec_wait
// 0x03dc: loop
// 0x03eb: cmd_exec_query
// 0x045e: query_counter
// 0x04b8: cmd_exec
// 0x04cd: cmd_exec_no_format
// 0x04e8: cmd_exec_init_src_surface
// 0x04fa: src_tiled
// 0x0501: cmd_exec_init_dst_surface
// 0x0514: dst_tiled
// 0x051b: cmd_exec_kick
// 0x053f: cmd_exec_done
// 0x0541: cmd_wrcache_flush
