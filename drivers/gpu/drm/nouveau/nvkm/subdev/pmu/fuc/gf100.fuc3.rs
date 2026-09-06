//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/pmu/fuc/gf100.fuc3.h
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
// 0x0000: proc_kern
// 0x0058: proc_list_head
// 0x0268: proc_list_tail
// 0x0268: time_prev
// 0x026c: time_next
// 0x0270: fifo_queue
// 0x02f0: rfifo_queue
// 0x0370: memx_func_head
// 0x037c: memx_func_next
// 0x03c4: memx_func_tail
// 0x03c4: memx_ts_start
// 0x03c8: memx_ts_end
// 0x03cc: memx_data_head
// 0x0bcc: memx_data_tail
// 0x0bcc: memx_train_head
// 0x0ccc: memx_train_tail
// 0x0ccc: i2c_scl_map
// 0x0cf4: i2c_sda_map
// 0x0d1c: i2c_ctrl
// 0x0004: rd32
// 0x0023: rd32_wait
// 0x0040: wr32
// 0x006b: wr32_wait
// 0x007e: nsec
// 0x008b: nsec_loop
// 0x00a3: wait
// 0x00b0: wait_loop
// 0x00d4: wait_done
// 0x00da: intr_watchdog
// 0x00f8: intr_watchdog_next_time
// 0x0107: intr_watchdog_next_time_set
// 0x010a: intr_watchdog_next_proc
// 0x0119: intr
// 0x017d: intr_skip_watchdog
// 0x01bd: intr_subintr_skip_fifo
// 0x01c9: intr_skip_subintr
// 0x01f9: ticks_from_ns
// 0x0221: ticks_from_ns_quit
// 0x022a: ticks_from_us
// 0x0244: ticks_from_us_quit
// 0x024a: ticks_to_us
// 0x0256: timer
// 0x02ac: timer_reset
// 0x02ba: timer_enable
// 0x02c8: timer_done
// 0x02d1: send_proc
// 0x030b: send_done
// 0x0311: find
// 0x0319: find_loop
// 0x032f: find_done
// 0x0336: send
// 0x033f: recv
// 0x038c: recv_done
// 0x0392: init
// 0x03fa: init_proc
// 0x040b: mulu32_32_64
// 0x045c: host_send
// 0x04a5: host_send_done
// 0x04a7: host_recv
// 0x04b5: host_recv_wait
// 0x050a: host_init
// 0x0549: memx_func_enter
// 0x05b5: memx_func_enter_wait
// 0x05d3: memx_func_leave
// 0x05ee: memx_func_leave_wait
// 0x065b: memx_func_wait_vblank
// 0x066d: memx_func_wait_vblank_head1
// 0x0673: memx_func_wait_vblank_head0
// 0x0676: memx_func_wait_vblank_0
// 0x0686: memx_func_wait_vblank_1
// 0x0696: memx_func_wait_vblank_fini
// 0x069b: memx_func_wr32
// 0x06b7: memx_func_wait
// 0x06d4: memx_func_delay
// 0x06df: memx_func_train
// 0x06e1: memx_exec
// 0x06eb: memx_exec_next
// 0x0727: memx_info
// 0x072d: memx_info_data
// 0x0738: memx_info_train
// 0x0740: memx_info_send
// 0x0746: memx_recv
// 0x0754: memx_init
// 0x0756: perf_recv
// 0x0758: perf_init
// 0x075a: i2c_drive_scl
// 0x076e: i2c_drive_scl_lo
// 0x077c: i2c_drive_sda
// 0x0790: i2c_drive_sda_lo
// 0x079e: i2c_sense_scl
// 0x07b4: i2c_sense_scl_done
// 0x07b6: i2c_sense_sda
// 0x07cc: i2c_sense_sda_done
// 0x07ce: i2c_raise_scl
// 0x07db: i2c_raise_scl_wait
// 0x07ef: i2c_raise_scl_done
// 0x07f3: i2c_start
// 0x0804: i2c_start_rep
// 0x0831: i2c_start_send
// 0x084d: i2c_start_out
// 0x084f: i2c_stop
// 0x0882: i2c_bitw
// 0x08c1: i2c_bitw_out
// 0x08c3: i2c_bitr
// 0x0908: i2c_bitr_done
// 0x090a: i2c_get_byte
// 0x0910: i2c_get_byte_next
// 0x095a: i2c_get_byte_done
// 0x095c: i2c_put_byte
// 0x095f: i2c_put_byte_next
// 0x09b5: i2c_put_byte_done
// 0x09b7: i2c_addr
// 0x09fc: i2c_addr_done
// 0x09fe: i2c_acquire_addr
// 0x0a0d: i2c_acquire
// 0x0a1c: i2c_release
// 0x0a2b: i2c_recv
// 0x0b31: i2c_recv_not_rd08
// 0x0b71: i2c_recv_not_wr08
// 0x0b71: i2c_recv_done
// 0x0b86: i2c_recv_exit
// 0x0b88: i2c_init
// 0x0b8a: test_recv
// 0x0bb1: test_init
// 0x0bbb: idle_recv
// 0x0bbd: idle
// 0x0bd9: idle_loop
// 0x0bdf: idle_proc
// 0x0bdf: idle_proc_exec
// 0x0bf3: idle_proc_next
