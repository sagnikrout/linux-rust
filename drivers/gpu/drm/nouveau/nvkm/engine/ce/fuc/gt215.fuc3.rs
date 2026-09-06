//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/engine/ce/fuc/gt215.fuc3.h
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
// 0x0004: ctx_dma
// 0x0004: ctx_dma_query
// 0x0008: ctx_dma_src
// 0x000c: ctx_dma_dst
// 0x0010: ctx_query_address_high
// 0x0014: ctx_query_address_low
// 0x0018: ctx_query_counter
// 0x001c: ctx_src_address_high
// 0x0020: ctx_src_address_low
// 0x0024: ctx_src_pitch
// 0x0028: ctx_src_tile_mode
// 0x002c: ctx_src_xsize
// 0x0030: ctx_src_ysize
// 0x0034: ctx_src_zsize
// 0x0038: ctx_src_zoff
// 0x003c: ctx_src_xoff
// 0x0040: ctx_src_yoff
// 0x0044: ctx_src_cpp
// 0x0048: ctx_dst_address_high
// 0x004c: ctx_dst_address_low
// 0x0050: ctx_dst_pitch
// 0x0054: ctx_dst_tile_mode
// 0x0058: ctx_dst_xsize
// 0x005c: ctx_dst_ysize
// 0x0060: ctx_dst_zsize
// 0x0064: ctx_dst_zoff
// 0x0068: ctx_dst_xoff
// 0x006c: ctx_dst_yoff
// 0x0070: ctx_dst_cpp
// 0x0074: ctx_format
// 0x0078: ctx_swz_const0
// 0x007c: ctx_swz_const1
// 0x0080: ctx_xcnt
// 0x0084: ctx_ycnt
// 0x0100: dispatch_table
// 0x0128: dispatch_dma
// 0x0000: main
// 0x002f: spin
// 0x0035: ih
// 0x0041: ih_no_chsw
// 0x004a: ih_no_cmd
// 0x0052: swctx
// 0x006b: swctx_load
// 0x006e: swctx_done
// 0x0072: chsw
// 0x0093: chsw_no_unload
// 0x00a8: chsw_load_ctx_dma
// 0x00bb: chsw_finish_load
// 0x00c3: dispatch
// 0x00dc: dispatch_loop
// 0x0100: dispatch_valid_mthd
// 0x0127: dispatch_cmd
// 0x0132: dispatch_invalid_bitfield
// 0x0135: dispatch_illegal_mthd
// 0x0138: dispatch_error
// 0x0148: hostirq_wait
// 0x0154: dispatch_done
// 0x0160: cmd_nop
// 0x0162: cmd_pm_trigger
// 0x0170: cmd_dma
// 0x0189: cmd_exec_set_format
// 0x01b4: ncomp_loop
// 0x01bc: bpc_loop
// 0x01ce: cmp_c0
// 0x01da: cmp_c1
// 0x01e9: cmp_zero
// 0x01ed: bpc_next
// 0x0216: dst_xcnt
// 0x0276: cmd_exec_set_surface_tiled
// 0x0291: xtile64
// 0x029d: xtileok
// 0x0382: cmd_exec_set_surface_linear
// 0x03ab: cmd_exec_wait
// 0x03b6: loop
// 0x03c5: cmd_exec_query
// 0x0438: query_counter
// 0x0492: cmd_exec
// 0x04a7: cmd_exec_no_format
// 0x04c2: cmd_exec_init_src_surface
// 0x04d4: src_tiled
// 0x04db: cmd_exec_init_dst_surface
// 0x04ee: dst_tiled
// 0x04f5: cmd_exec_kick
// 0x0519: cmd_exec_done
// 0x051b: cmd_wrcache_flush
