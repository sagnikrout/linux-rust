//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/pmu/fuc/gk208.fuc5.h
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
// 0x0018: rd32_wait
// 0x002d: wr32
// 0x0049: wr32_wait
// 0x0058: nsec
// 0x0061: nsec_loop
// 0x0074: wait
// 0x007d: wait_loop
// 0x009a: wait_done
// 0x00a0: intr_watchdog
// 0x00bd: intr_watchdog_next_time
// 0x00cb: intr_watchdog_next_time_set
// 0x00ce: intr_watchdog_next_proc
// 0x00dd: intr
// 0x0130: intr_skip_watchdog
// 0x0160: intr_subintr_skip_fifo
// 0x0168: intr_skip_subintr
// 0x0193: ticks_from_ns
// 0x01b3: ticks_from_ns_quit
// 0x01bb: ticks_from_us
// 0x01d0: ticks_from_us_quit
// 0x01d6: ticks_to_us
// 0x01de: timer
// 0x0222: timer_reset
// 0x022c: timer_enable
// 0x0235: timer_done
// 0x023e: send_proc
// 0x0277: send_done
// 0x027d: find
// 0x0284: find_loop
// 0x0299: find_done
// 0x029f: send
// 0x02a8: recv
// 0x02f3: recv_done
// 0x02f9: init
// 0x0341: init_proc
// 0x0352: mulu32_32_64
// 0x03a1: host_send
// 0x03dd: host_send_done
// 0x03df: host_recv
// 0x03e9: host_recv_wait
// 0x042c: host_init
// 0x045c: memx_func_enter
// 0x04b6: memx_func_enter_wait
// 0x04cc: memx_func_leave
// 0x04de: memx_func_leave_wait
// 0x053c: memx_func_wait_vblank
// 0x0541: memx_func_wr32
// 0x055e: memx_func_wait
// 0x0578: memx_func_delay
// 0x0584: memx_func_train
// 0x0586: memx_exec
// 0x058e: memx_exec_next
// 0x05c5: memx_info
// 0x05cb: memx_info_data
// 0x05d4: memx_info_train
// 0x05da: memx_info_send
// 0x05e0: memx_recv
// 0x05ee: memx_init
// 0x05f0: perf_recv
// 0x05f2: perf_init
// 0x05f4: i2c_drive_scl
// 0x0604: i2c_drive_scl_lo
// 0x060e: i2c_drive_sda
// 0x061e: i2c_drive_sda_lo
// 0x0628: i2c_sense_scl
// 0x063a: i2c_sense_scl_done
// 0x063c: i2c_sense_sda
// 0x064e: i2c_sense_sda_done
// 0x0650: i2c_raise_scl
// 0x065b: i2c_raise_scl_wait
// 0x066f: i2c_raise_scl_done
// 0x0673: i2c_start
// 0x0684: i2c_start_rep
// 0x06af: i2c_start_send
// 0x06c9: i2c_start_out
// 0x06cb: i2c_stop
// 0x06fa: i2c_bitw
// 0x0738: i2c_bitw_out
// 0x073a: i2c_bitr
// 0x077d: i2c_bitr_done
// 0x077f: i2c_get_byte
// 0x0783: i2c_get_byte_next
// 0x07cc: i2c_get_byte_done
// 0x07ce: i2c_put_byte
// 0x07d0: i2c_put_byte_next
// 0x0826: i2c_put_byte_done
// 0x0828: i2c_addr
// 0x086d: i2c_addr_done
// 0x086f: i2c_acquire_addr
// 0x087b: i2c_acquire
// 0x088c: i2c_release
// 0x089d: i2c_recv
// 0x099f: i2c_recv_not_rd08
// 0x09dd: i2c_recv_not_wr08
// 0x09dd: i2c_recv_done
// 0x09f1: i2c_recv_exit
// 0x09f3: i2c_init
// 0x09f5: test_recv
// 0x0a11: test_init
// 0x0a1a: idle_recv
// 0x0a1c: idle
// 0x0a30: idle_loop
// 0x0a35: idle_proc
// 0x0a35: idle_proc_exec
// 0x0a48: idle_proc_next
