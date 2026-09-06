//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/pmu/fuc/gf119.fuc4.h
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
// 0x0004: rd32
// 0x001d: rd32_wait
// 0x0034: wr32
// 0x0056: wr32_wait
// 0x0066: nsec
// 0x0070: nsec_loop
// 0x0085: wait
// 0x008f: wait_loop
// 0x00b0: wait_done
// 0x00b6: intr_watchdog
// 0x00d4: intr_watchdog_next_time
// 0x00e3: intr_watchdog_next_time_set
// 0x00e6: intr_watchdog_next_proc
// 0x00f5: intr
// 0x014d: intr_skip_watchdog
// 0x0184: intr_subintr_skip_fifo
// 0x018d: intr_skip_subintr
// 0x01ba: ticks_from_ns
// 0x01e2: ticks_from_ns_quit
// 0x01eb: ticks_from_us
// 0x0205: ticks_from_us_quit
// 0x020b: ticks_to_us
// 0x0217: timer
// 0x0261: timer_reset
// 0x026c: timer_enable
// 0x0277: timer_done
// 0x0280: send_proc
// 0x02ba: send_done
// 0x02c0: find
// 0x02c8: find_loop
// 0x02de: find_done
// 0x02e5: send
// 0x02ee: recv
// 0x033b: recv_done
// 0x0341: init
// 0x039a: init_proc
// 0x03ab: mulu32_32_64
// 0x03fc: host_send
// 0x043c: host_send_done
// 0x043e: host_recv
// 0x044c: host_recv_wait
// 0x0495: host_init
// 0x04cb: memx_func_enter
// 0x0534: memx_func_enter_wait
// 0x054c: memx_func_leave
// 0x0561: memx_func_leave_wait
// 0x05cb: memx_func_wait_vblank
// 0x05d0: memx_func_wr32
// 0x05ec: memx_func_wait
// 0x0606: memx_func_delay
// 0x0611: memx_func_train
// 0x0613: memx_exec
// 0x061d: memx_exec_next
// 0x0656: memx_info
// 0x065c: memx_info_data
// 0x0667: memx_info_train
// 0x066f: memx_info_send
// 0x0675: memx_recv
// 0x0683: memx_init
// 0x0685: perf_recv
// 0x0687: perf_init
// 0x0689: i2c_drive_scl
// 0x069a: i2c_drive_scl_lo
// 0x06a5: i2c_drive_sda
// 0x06b6: i2c_drive_sda_lo
// 0x06c1: i2c_sense_scl
// 0x06d4: i2c_sense_scl_done
// 0x06d6: i2c_sense_sda
// 0x06e9: i2c_sense_sda_done
// 0x06eb: i2c_raise_scl
// 0x06f8: i2c_raise_scl_wait
// 0x070c: i2c_raise_scl_done
// 0x0710: i2c_start
// 0x0721: i2c_start_rep
// 0x074e: i2c_start_send
// 0x076a: i2c_start_out
// 0x076c: i2c_stop
// 0x079f: i2c_bitw
// 0x07de: i2c_bitw_out
// 0x07e0: i2c_bitr
// 0x0825: i2c_bitr_done
// 0x0827: i2c_get_byte
// 0x082d: i2c_get_byte_next
// 0x0877: i2c_get_byte_done
// 0x0879: i2c_put_byte
// 0x087c: i2c_put_byte_next
// 0x08d2: i2c_put_byte_done
// 0x08d4: i2c_addr
// 0x0919: i2c_addr_done
// 0x091b: i2c_acquire_addr
// 0x0927: i2c_acquire
// 0x0936: i2c_release
// 0x0945: i2c_recv
// 0x0a4b: i2c_recv_not_rd08
// 0x0a8b: i2c_recv_not_wr08
// 0x0a8b: i2c_recv_done
// 0x0aa0: i2c_recv_exit
// 0x0aa2: i2c_init
// 0x0aa4: test_recv
// 0x0ac5: test_init
// 0x0acf: idle_recv
// 0x0ad1: idle
// 0x0ae7: idle_loop
// 0x0aed: idle_proc
// 0x0aed: idle_proc_exec
// 0x0b01: idle_proc_next
