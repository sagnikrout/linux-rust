//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/engine/gr/fuc/gpcgk208.fuc5.h
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
// 0x0000: gpc_mmio_list_head
// 0x0004: gpc_mmio_list_tail
// 0x0004: tpc_mmio_list_head
// 0x0008: tpc_mmio_list_tail
// 0x0008: unk_mmio_list_head
// 0x000c: unk_mmio_list_tail
// 0x0010: gpc_id
// 0x0014: tpc_count
// 0x0018: tpc_mask
// 0x001c: unk_count
// 0x0020: unk_mask
// 0x0024: cmd_queue
// 0x0004: queue_put
// 0x001a: queue_put_next
// 0x0037: queue_get
// 0x0063: queue_get_done
// 0x0065: nv_rd32
// 0x0073: nv_rd32_wait
// 0x008f: nv_wr32
// 0x00a9: nv_wr32_wait
// 0x00b8: wait_donez
// 0x00cf: wait_donez_ne
// 0x00ec: wait_doneo
// 0x0103: wait_doneo_e
// 0x0120: mmctx_size
// 0x0122: nv_mmctx_size_loop
// 0x013d: mmctx_xfer
// 0x015f: mmctx_base_disabled
// 0x017a: mmctx_multi_disabled
// 0x0195: mmctx_exec_loop
// 0x0195: mmctx_wait_free
// 0x01bf: mmctx_fini_wait
// 0x01d8: mmctx_stop
// 0x01ed: mmctx_stop_wait
// 0x01fa: mmctx_done
// 0x020a: strand_wait
// 0x0216: strand_pre
// 0x0227: strand_post
// 0x0238: strand_set
// 0x0268: strand_ctx_init
// 0x02c7: ctx_init_strand_loop
// 0x02f8: error
// 0x0314: init
// 0x0386: init_unk_loop
// 0x039b: init_unk_next
// 0x03a7: init_unk_done
// 0x0448: wait
// 0x044e: main
// 0x0477: main_not_ctx_xfer
// 0x0484: ih
// 0x04c3: ih_no_fifo
// 0x04e5: hub_barrier_done
// 0x04f9: ctx_redswitch
// 0x0506: ctx_redswitch_delay
// 0x051f: ctx_xfer
// 0x052f: ctx_xfer_not_load
// 0x05b9: ctx_xfer_post
// 0x05bd: ctx_xfer_done
