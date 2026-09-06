//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/goya/goyaP.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright 2016-2022 HabanaLabs, Ltd.
// All Rights Reserved.
//

pub const NUMBER_OF_CMPLT_QUEUES: c_int = 5;
pub const NUMBER_OF_EXT_HW_QUEUES: c_int = 5;
pub const NUMBER_OF_CPU_HW_QUEUES: c_int = 1;
pub const NUMBER_OF_INT_HW_QUEUES: c_int = 9;

//
// Number of MSIX interrupts IDS:
// Each completion queue has 1 ID
// The event queue has 1 ID
//

pub const TPC_ENABLED_MASK: c_uint = 0xFF;

pub const DRAM_PHYS_DEFAULT_SIZE: c_uint = 0x100000000ull	/* 4GB */;

pub const GOYA_MAX_PENDING_CS: c_int = 64;

// DRAM Memory Map
pub const CPU_FW_IMAGE_SIZE: c_uint = 0x10000000	/* 256MB */;
pub const MMU_PAGE_TABLES_SIZE: c_uint = 0x0FC00000	/* 252MB */;
pub const MMU_DRAM_DEFAULT_PAGE_SIZE: c_uint = 0x00200000	/* 2MB */;
pub const MMU_CACHE_MNG_SIZE: c_uint = 0x00001000	/* 4KB */;

pub const DRAM_BASE_ADDR_USER: c_uint = 0x20000000;

//
// SRAM Memory Map for Driver
//
// Driver occupies DRIVER_SRAM_SIZE bytes from the start of SRAM. It is used for
// MME/TPC QMANs
//
pub const MME_QMAN_BASE_OFFSET: c_uint = 0x000000	/* Must be 0 */;
pub const MME_QMAN_LENGTH: c_int = 64;
pub const TPC_QMAN_LENGTH: c_int = 64;

// Virtual address space
pub const VA_HOST_SPACE_START: c_uint = 0x1000000000000ull	/* 256TB */;
pub const VA_HOST_SPACE_END: c_uint = 0x3FF8000000000ull	/* 1PB - 1TB */;

pub const VA_DDR_SPACE_START: c_uint = 0x800000000ull		/* 32GB */;
pub const VA_DDR_SPACE_END: c_uint = 0x2000000000ull		/* 128GB */;

pub const VA_CPU_ACCESSIBLE_MEM_ADDR: c_uint = 0x8000000000ull;

pub const HW_CAP_PLL: c_uint = 0x00000001;
pub const HW_CAP_DDR_0: c_uint = 0x00000002;
pub const HW_CAP_DDR_1: c_uint = 0x00000004;
pub const HW_CAP_MME: c_uint = 0x00000008;
pub const HW_CAP_CPU: c_uint = 0x00000010;
pub const HW_CAP_DMA: c_uint = 0x00000020;
pub const HW_CAP_MSIX: c_uint = 0x00000040;
pub const HW_CAP_CPU_Q: c_uint = 0x00000080;
pub const HW_CAP_MMU: c_uint = 0x00000100;
pub const HW_CAP_TPC_MBIST: c_uint = 0x00000200;
pub const HW_CAP_GOLDEN: c_uint = 0x00000400;
pub const HW_CAP_TPC: c_uint = 0x00000800;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct goya_work_freq {
    pub hdev: *mut hl_device,
    pub work_freq: delayed_work,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct goya_device {
// TODO: remove hw_queues_lock after moving to scheduler code
    pub hw_queues_lock: spinlock_t,
    pub goya_work: *mut goya_work_freq,
    pub mme_clk: u64,
    pub tpc_clk: u64,
    pub ic_clk: u64,
    pub ddr_bar_cur_addr: u64,
    pub events_stat: [u32; GOYA_ASYNC_EVENT_ID_SIZE],
    pub events_stat_aggregate: [u32; GOYA_ASYNC_EVENT_ID_SIZE],
    pub hw_cap_initialized: u32,
    pub device_cpu_mmu_mappings_done: u8,
    pub curr_pll_profile: hl_pll_frequency,
    pub pm_mng_profile: hl_pm_mng_profile,
}

extern "C" {
    pub fn goya_set_fixed_properties(hdev: *mut hl_device) -> c_int;
}
extern "C" {
    pub fn goya_mmu_init(hdev: *mut hl_device) -> c_int;
}
extern "C" {
    pub fn goya_init_dma_qmans(hdev: *mut hl_device);
}
extern "C" {
    pub fn goya_init_mme_qmans(hdev: *mut hl_device);
}
extern "C" {
    pub fn goya_init_tpc_qmans(hdev: *mut hl_device);
}
extern "C" {
    pub fn goya_init_cpu_queues(hdev: *mut hl_device) -> c_int;
}
extern "C" {
    pub fn goya_init_security(hdev: *mut hl_device);
}
extern "C" {
    pub fn goya_ack_protection_bits_errors(hdev: *mut hl_device);
}
extern "C" {
    pub fn goya_late_init(hdev: *mut hl_device) -> c_int;
}
extern "C" {
    pub fn goya_late_fini(hdev: *mut hl_device);
}
extern "C" {
    pub fn goya_ring_doorbell(hdev: *mut hl_device, hw_queue_id: u32, pi: u32);
}
extern "C" {
    pub fn goya_pqe_write(hdev: *mut hl_device, pqe: *mut __le64, bd: *mut hl_bd);
}
extern "C" {
    pub fn goya_update_eq_ci(hdev: *mut hl_device, val: u32);
}
extern "C" {
    pub fn goya_restore_phase_topology(hdev: *mut hl_device);
}
extern "C" {
    pub fn goya_context_switch(hdev: *mut hl_device, asid: u32) -> c_int;
}
extern "C" {
    pub fn goya_debugfs_led_set(hdev: *mut hl_device, led: u8, state: u8);
}
extern "C" {
    pub fn goya_test_queue(hdev: *mut hl_device, hw_queue_id: u32) -> c_int;
}
extern "C" {
    pub fn goya_test_queues(hdev: *mut hl_device) -> c_int;
}
extern "C" {
    pub fn goya_test_cpu_queue(hdev: *mut hl_device) -> c_int;
}
extern "C" {
    pub fn goya_get_temperature(hdev: *mut hl_device, sensor_index: c_int, attr: u32) -> c_long;
}
extern "C" {
    pub fn goya_get_voltage(hdev: *mut hl_device, sensor_index: c_int, attr: u32) -> c_long;
}
extern "C" {
    pub fn goya_get_current(hdev: *mut hl_device, sensor_index: c_int, attr: u32) -> c_long;
}
extern "C" {
    pub fn goya_get_fan_speed(hdev: *mut hl_device, sensor_index: c_int, attr: u32) -> c_long;
}
extern "C" {
    pub fn goya_get_pwm_info(hdev: *mut hl_device, sensor_index: c_int, attr: u32) -> c_long;
}
extern "C" {
    pub fn goya_get_max_power(hdev: *mut hl_device) -> u64;
}
extern "C" {
    pub fn goya_set_max_power(hdev: *mut hl_device, value: u64);
}
extern "C" {
    pub fn goya_set_pll_profile(hdev: *mut hl_device, freq: hl_pll_frequency);
}
extern "C" {
    pub fn goya_cpucp_info_get(hdev: *mut hl_device) -> c_int;
}
extern "C" {
    pub fn goya_debug_coresight(hdev: *mut hl_device, ctx: *mut hl_ctx, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn goya_halt_coresight(hdev: *mut hl_device, ctx: *mut hl_ctx);
}
extern "C" {
    pub fn goya_suspend(hdev: *mut hl_device) -> c_int;
}
extern "C" {
    pub fn goya_resume(hdev: *mut hl_device) -> c_int;
}
extern "C" {
    pub fn goya_handle_eqe(hdev: *mut hl_device, eq_entry: *mut hl_eq_entry);
}
extern "C" {
    pub fn goya_cs_parser(hdev: *mut hl_device, parser: *mut hl_cs_parser) -> c_int;
}
extern "C" {
    pub fn goya_scrub_device_mem(hdev: *mut hl_device) -> c_int;
}
extern "C" {
    pub fn goya_get_dma_desc_list_size(hdev: *mut hl_device, sgt: *mut sg_table) -> u32;
}
extern "C" {
    pub fn goya_send_heartbeat(hdev: *mut hl_device) -> c_int;
}
extern "C" {
    pub fn goya_mmu_remove_device_cpu_mappings(hdev: *mut hl_device);
}
extern "C" {
    pub fn goya_get_queue_id_for_cq(hdev: *mut hl_device, cq_idx: u32) -> u32;
}
extern "C" {
    pub fn goya_get_device_time(hdev: *mut hl_device) -> u64;
}
extern "C" {
    pub fn goya_set_frequency(hdev: *mut hl_device, freq: hl_pll_frequency) -> c_int;
}
