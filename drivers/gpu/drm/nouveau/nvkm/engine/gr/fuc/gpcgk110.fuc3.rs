//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/engine/gr/fuc/gpcgk110.fuc3.h
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
// 0x001c: queue_put_next
// 0x0039: queue_get
// 0x0066: queue_get_done
// 0x0068: nv_rd32
// 0x007a: nv_rd32_wait
// 0x009d: nv_wr32
// 0x00be: nv_wr32_wait
// 0x00d0: wait_donez
// 0x00ed: wait_donez_ne
// 0x0110: wait_doneo
// 0x012d: wait_doneo_e
// 0x0150: mmctx_size
// 0x0152: nv_mmctx_size_loop
// 0x016f: mmctx_xfer
// 0x0197: mmctx_base_disabled
// 0x01b8: mmctx_multi_disabled
// 0x01d6: mmctx_exec_loop
// 0x01d6: mmctx_wait_free
// 0x0207: mmctx_fini_wait
// 0x0223: mmctx_stop
// 0x023b: mmctx_stop_wait
// 0x024b: mmctx_done
// 0x025e: strand_wait
// 0x026a: strand_pre
// 0x027f: strand_post
// 0x0294: strand_set
// 0x02d3: strand_ctx_init
// 0x034a: ctx_init_strand_loop
// 0x037e: error
// 0x03a1: init
// 0x0433: init_unk_loop
// 0x0448: init_unk_next
// 0x0454: init_unk_done
// 0x0508: wait
// 0x050e: main
// 0x0538: main_not_ctx_xfer
// 0x0545: ih
// 0x0595: ih_no_fifo
// 0x05bb: hub_barrier_done
// 0x05d3: ctx_redswitch
// 0x05e5: ctx_redswitch_delay
// 0x0601: ctx_xfer
// 0x0614: ctx_xfer_not_load
// 0x06b0: ctx_xfer_post
// 0x06b4: ctx_xfer_done
