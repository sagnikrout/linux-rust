//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/engine/gr/fuc/hubgk208.fuc5.h
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
// 0x0000: hub_mmio_list_head
// 0x0004: hub_mmio_list_tail
// 0x0008: gpc_count
// 0x000c: rop_count
// 0x0010: cmd_queue
// 0x0058: ctx_current
// 0x0100: chan_data
// 0x0100: chan_mmio_count
// 0x0104: chan_mmio_address
// 0x0200: xfer_data
// 0x0300: hub_mmio_list_base
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
// 0x030e: init
// 0x0420: init_gpc
// 0x044f: init_gpc_wait
// 0x0492: wait
// 0x0498: main
// 0x0523: chsw_prev_no_next
// 0x053f: chsw_no_prev
// 0x054f: chsw_done
// 0x056c: main_not_ctx_switch
// 0x057b: main_not_ctx_chan
// 0x05aa: main_not_ctx_save
// 0x05b8: main_done
// 0x05ca: ih
// 0x060d: ih_no_fifo
// 0x061d: ih_no_ctxsw
// 0x0677: ih_no_fwmthd
// 0x068b: ih_no_other
// 0x06ad: ctx_4170s
// 0x06bc: ctx_4170w
// 0x06ce: ctx_redswitch
// 0x06e5: ctx_redswitch_delay
// 0x06fe: ctx_86c
// 0x071d: ctx_mem
// 0x0726: ctx_mem_wait
// 0x0735: ctx_load
// 0x0821: ctx_chan
// 0x0833: ctx_mmio_exec
// 0x0841: ctx_mmio_loop
// 0x0852: ctx_mmio_pull
// 0x0865: ctx_mmio_done
// 0x0881: ctx_xfer
// 0x088c: ctx_xfer_idle
// 0x08a0: ctx_xfer_pre
// 0x08a9: ctx_xfer_pre_load
// 0x08c1: ctx_xfer_exec
// 0x093d: ctx_xfer_post
// 0x0967: ctx_xfer_no_post_mmio
// 0x0967: ctx_xfer_done
