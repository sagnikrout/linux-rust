//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/engine/gr/fuc/hubgf100.fuc3.h
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
// 0x039b: init
// 0x04f4: init_gpc
// 0x051c: init_gpc_wait
// 0x0564: wait
// 0x056a: main
// 0x060c: chsw_prev_no_next
// 0x062c: chsw_no_prev
// 0x063c: chsw_done
// 0x0660: main_not_ctx_switch
// 0x0670: main_not_ctx_chan
// 0x06a5: main_not_ctx_save
// 0x06b3: main_done
// 0x06c8: ih
// 0x071c: ih_no_fifo
// 0x072d: ih_no_ctxsw
// 0x079d: ih_no_fwmthd
// 0x07b5: ih_no_other
// 0x07db: ctx_4160s
// 0x07eb: ctx_4160s_wait
// 0x0800: ctx_4160c
// 0x0811: ctx_4170s
// 0x0823: ctx_4170w
// 0x0838: ctx_redswitch
// 0x0854: ctx_redswitch_delay
// 0x0870: ctx_86c
// 0x0898: ctx_mem
// 0x08a4: ctx_mem_wait
// 0x08b6: ctx_load
// 0x09d4: ctx_chan
// 0x09ef: ctx_mmio_exec
// 0x0a00: ctx_mmio_loop
// 0x0a12: ctx_mmio_pull
// 0x0a24: ctx_mmio_done
// 0x0a44: ctx_xfer
// 0x0a53: ctx_xfer_idle
// 0x0a6a: ctx_xfer_pre
// 0x0a78: ctx_xfer_pre_load
// 0x0a91: ctx_xfer_exec
// 0x0b20: ctx_xfer_post
// 0x0b4b: ctx_xfer_no_post_mmio
// 0x0b4f: ctx_xfer_done
