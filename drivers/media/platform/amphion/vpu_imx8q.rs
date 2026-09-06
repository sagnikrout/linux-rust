//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/amphion/vpu_imx8q.h
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
// Copyright 2020-2021 NXP
//
pub const SCB_XREG_SLV_BASE: c_uint = 0x00000000;
pub const SCB_SCB_BLK_CTRL: c_uint = 0x00070000;
pub const SCB_BLK_CTRL_XMEM_RESET_SET: c_uint = 0x00000090;
pub const SCB_BLK_CTRL_CACHE_RESET_SET: c_uint = 0x000000A0;
pub const SCB_BLK_CTRL_CACHE_RESET_CLR: c_uint = 0x000000A4;
pub const SCB_BLK_CTRL_SCB_CLK_ENABLE_SET: c_uint = 0x00000100;
pub const XMEM_CONTROL: c_uint = 0x00041000;
pub const MC_CACHE_0_BASE: c_uint = 0x00060000;
pub const MC_CACHE_1_BASE: c_uint = 0x00068000;
pub const DEC_MFD_XREG_SLV_BASE: c_uint = 0x00180000;
pub const ENC_MFD_XREG_SLV_0_BASE: c_uint = 0x00800000;
pub const ENC_MFD_XREG_SLV_1_BASE: c_uint = 0x00A00000;
pub const MFD_HIF: c_uint = 0x0001C000;
pub const MFD_HIF_MSD_REG_INTERRUPT_STATUS: c_uint = 0x00000018;
pub const MFD_SIF: c_uint = 0x0001D000;
pub const MFD_SIF_CTRL_STATUS: c_uint = 0x000000F0;
pub const MFD_SIF_INTR_STATUS: c_uint = 0x000000F4;
pub const MFD_MCX: c_uint = 0x00020800;
pub const MFD_MCX_OFF: c_uint = 0x00000020;
pub const MFD_PIX_IF: c_uint = 0x00020000;
pub const MFD_BLK_CTRL: c_uint = 0x00030000;
pub const MFD_BLK_CTRL_MFD_SYS_RESET_SET: c_uint = 0x00000000;
pub const MFD_BLK_CTRL_MFD_SYS_RESET_CLR: c_uint = 0x00000004;
pub const MFD_BLK_CTRL_MFD_SYS_CLOCK_ENABLE_SET: c_uint = 0x00000100;
pub const MFD_BLK_CTRL_MFD_SYS_CLOCK_ENABLE_CLR: c_uint = 0x00000104;
pub const VID_API_NUM_STREAMS: c_int = 8;
pub const VID_API_MAX_BUF_PER_STR: c_int = 3;
pub const VID_API_MAX_NUM_MVC_VIEWS: c_int = 4;
pub const MEDIAIP_MAX_NUM_MALONES: c_int = 2;
pub const MEDIAIP_MAX_NUM_MALONE_IRQ_PINS: c_int = 2;
pub const MEDIAIP_MAX_NUM_WINDSORS: c_int = 1;
pub const MEDIAIP_MAX_NUM_WINDSOR_IRQ_PINS: c_int = 2;
pub const MEDIAIP_MAX_NUM_CMD_IRQ_PINS: c_int = 2;
pub const MEDIAIP_MAX_NUM_MSG_IRQ_PINS: c_int = 1;
pub const MEDIAIP_MAX_NUM_TIMER_IRQ_PINS: c_int = 4;
pub const MEDIAIP_MAX_NUM_TIMER_IRQ_SLOTS: c_int = 4;
pub const WINDSOR_PAL_IRQ_PIN_L: c_uint = 0x4;
pub const WINDSOR_PAL_IRQ_PIN_H: c_uint = 0x5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_rpc_system_config {
    pub cfg_cookie: u32,
    pub num_malones: u32,
    pub malone_base_addr: [u32; MEDIAIP_MAX_NUM_MALONES],
    pub hif_offset: [u32; MEDIAIP_MAX_NUM_MALONES],
    pub malone_irq_pin: [u32; MEDIAIP_MAX_NUM_MALONES][MEDIAIP_MAX_NUM_MALONE_IRQ_PINS],
    pub malone_irq_target: [u32; MEDIAIP_MAX_NUM_MALONES][MEDIAIP_MAX_NUM_MALONE_IRQ_PINS],
    pub num_windsors: u32,
    pub windsor_base_addr: [u32; MEDIAIP_MAX_NUM_WINDSORS],
    pub windsor_irq_pin: [u32; MEDIAIP_MAX_NUM_WINDSORS][MEDIAIP_MAX_NUM_WINDSOR_IRQ_PINS],
    pub windsor_irq_target: [u32; MEDIAIP_MAX_NUM_WINDSORS][MEDIAIP_MAX_NUM_WINDSOR_IRQ_PINS],
    pub cmd_irq_pin: [u32; MEDIAIP_MAX_NUM_CMD_IRQ_PINS],
    pub cmd_irq_target: [u32; MEDIAIP_MAX_NUM_CMD_IRQ_PINS],
    pub msg_irq_pin: [u32; MEDIAIP_MAX_NUM_MSG_IRQ_PINS],
    pub msg_irq_target: [u32; MEDIAIP_MAX_NUM_MSG_IRQ_PINS],
    pub sys_clk_freq: u32,
    pub num_timers: u32,
    pub timer_base_addr: u32,
    pub timer_irq_pin: [u32; MEDIAIP_MAX_NUM_TIMER_IRQ_PINS],
    pub timer_irq_target: [u32; MEDIAIP_MAX_NUM_TIMER_IRQ_PINS],
    pub timer_slots: [u32; MEDIAIP_MAX_NUM_TIMER_IRQ_SLOTS],
    pub gic_base_addr: u32,
    pub uart_base_addr: u32,
    pub dpv_base_addr: u32,
    pub dpv_irq_pin: u32,
    pub dpv_irq_target: u32,
    pub pixif_base_addr: u32,
    pub pal_trace_level: u32,
    pub pal_trace_destination: u32,
    pub pal_trace_level1: u32,
    pub pal_trace_destination1: u32,
    pub heap_base: u32,
    pub heap_size: u32,
    pub cache_base_addr: [u32; 2],
}

extern "C" {
    pub fn vpu_imx8q_setup_dec(vpu: *mut vpu_dev) -> c_int;
}
extern "C" {
    pub fn vpu_imx8q_setup_enc(vpu: *mut vpu_dev) -> c_int;
}
extern "C" {
    pub fn vpu_imx8q_setup(vpu: *mut vpu_dev) -> c_int;
}
extern "C" {
    pub fn vpu_imx8q_reset(vpu: *mut vpu_dev) -> c_int;
}
extern "C" {
    pub fn vpu_imx8q_set_system_cfg_common(config: *mut vpu_rpc_system_config, regs: u32, core_id: u32) -> c_int;
}
extern "C" {
    pub fn vpu_imx8q_boot_core(core: *mut vpu_core) -> c_int;
}
extern "C" {
    pub fn vpu_imx8q_get_power_state(core: *mut vpu_core) -> c_int;
}
extern "C" {
    pub fn vpu_imx8q_on_firmware_loaded(core: *mut vpu_core) -> c_int;
}
extern "C" {
    pub fn vpu_imx8q_check_memory_region(base: dma_addr_t, addr: dma_addr_t, size: u32) -> c_int;
}
extern "C" {
    pub fn vpu_imx8q_check_codec(type: vpu_core_type) -> bool;
}
extern "C" {
    pub fn vpu_imx8q_check_fmt(type: vpu_core_type, pixelfmt: u32) -> bool;
}
