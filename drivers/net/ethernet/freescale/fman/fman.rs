//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/freescale/fman/fman.h
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0-or-later
//
// Copyright 2008 - 2015 Freescale Semiconductor Inc.
// Copyright 2020 NXP
//

// FM Frame descriptor macros
// Frame queue Context Override
pub const FM_FD_CMD_FCO: c_uint = 0x80000000;
pub const FM_FD_CMD_RPD: c_uint = 0x40000000  /* Read Prepended Data */;
pub const FM_FD_CMD_UPD: c_uint = 0x20000000  /* Update Prepended Data */;
pub const FM_FD_CMD_DTC: c_uint = 0x10000000  /* Do L4 Checksum */;
// TX-Port: Unsupported Format
pub const FM_FD_ERR_UNSUPPORTED_FORMAT: c_uint = 0x04000000;
// TX Port: Length Error
pub const FM_FD_ERR_LENGTH: c_uint = 0x02000000;
pub const FM_FD_ERR_DMA: c_uint = 0x01000000  /* DMA Data error */;
// IPR frame (not error)
pub const FM_FD_IPR: c_uint = 0x00000001;
// IPR non-consistent-sp

// IPR error

// IPR timeout

// TX Port: Length Error

// Rx FIFO overflow, FCS error, code error, running disparity error
// (SGMII and TBI modes), FIFO parity error. PHY Sequence error,
// PHY error control character detected.
//
pub const FM_FD_ERR_PHYSICAL: c_uint = 0x00080000;
// Frame too long OR Frame size exceeds max_length_frame
pub const FM_FD_ERR_SIZE: c_uint = 0x00040000;
// classification discard
pub const FM_FD_ERR_CLS_DISCARD: c_uint = 0x00020000;
// Extract Out of Frame
pub const FM_FD_ERR_EXTRACTION: c_uint = 0x00008000;
// No Scheme Selected
pub const FM_FD_ERR_NO_SCHEME: c_uint = 0x00004000;
// Keysize Overflow
pub const FM_FD_ERR_KEYSIZE_OVERFLOW: c_uint = 0x00002000;
// Frame color is red
pub const FM_FD_ERR_COLOR_RED: c_uint = 0x00000800;
// Frame color is yellow
pub const FM_FD_ERR_COLOR_YELLOW: c_uint = 0x00000400;
// Parser Time out Exceed
pub const FM_FD_ERR_PRS_TIMEOUT: c_uint = 0x00000080;
// Invalid Soft Parser instruction
pub const FM_FD_ERR_PRS_ILL_INSTRUCT: c_uint = 0x00000040;
// Header error was identified during parsing
pub const FM_FD_ERR_PRS_HDR_ERR: c_uint = 0x00000020;
// Frame parsed beyind 256 first bytes
pub const FM_FD_ERR_BLOCK_LIMIT_EXCEEDED: c_uint = 0x00000008;
// non Frame-Manager error
pub const FM_FD_RX_STATUS_ERR_NON_FM: c_uint = 0x00400000;
// FMan driver defines
pub const FMAN_BMI_FIFO_UNITS: c_uint = 0x100;
pub const OFFSET_UNITS: c_int = 16;
// BMan defines

// General defines
pub const MAX_NUM_OF_MACS: c_int = 10;
// Enum for defining port types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fman_port_type {
    FMAN_PORT_TYPE_TX = 0,	/* TX Port */
    FMAN_PORT_TYPE_RX,	/* RX Port */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fman_rev_info {
    pub /: *mut *mut u8 major; / Major revision,
    pub /: *mut *mut u8 minor; / Minor revision,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fman_exceptions {
    FMAN_EX_DMA_BUS_ERROR = 0,	/* DMA bus error. */
    FMAN_EX_DMA_READ_ECC,		/* Read Buffer ECC error */
    FMAN_EX_DMA_SYSTEM_WRITE_ECC,	/* Write Buffer ECC err on sys side */
    FMAN_EX_DMA_FM_WRITE_ECC,	/* Write Buffer ECC error on FM side */
    FMAN_EX_DMA_SINGLE_PORT_ECC,	/* Single Port ECC error on FM side */
    FMAN_EX_FPM_STALL_ON_TASKS,	/* Stall of tasks on FPM */
    FMAN_EX_FPM_SINGLE_ECC,		/* Single ECC on FPM. */
    FMAN_EX_FPM_DOUBLE_ECC,		/* Double ECC error on FPM ram access */
    FMAN_EX_QMI_SINGLE_ECC,	/* Single ECC on QMI. */
    FMAN_EX_QMI_DOUBLE_ECC,	/* Double bit ECC occurred on QMI */
    FMAN_EX_QMI_DEQ_FROM_UNKNOWN_PORTID,/* DeQ from unknown port id */
    FMAN_EX_BMI_LIST_RAM_ECC,	/* Linked List RAM ECC error */
    FMAN_EX_BMI_STORAGE_PROFILE_ECC,/* storage profile */
    FMAN_EX_BMI_STATISTICS_RAM_ECC,/* Statistics RAM ECC Err Enable */
    FMAN_EX_BMI_DISPATCH_RAM_ECC,	/* Dispatch RAM ECC Error Enable */
    FMAN_EX_IRAM_ECC,		/* Double bit ECC occurred on IRAM */
    FMAN_EX_MURAM_ECC		/* Double bit ECC occurred on MURAM */
}

// Parse results memory layout
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fman_prs_result {
    pub /: *mut *mut u8 lpid; / Logical port id,
    pub /: *mut *mut u8 shimr; / Shim header result,
    pub /: *mut *mut __be16 l2r; / Layer 2 result,
    pub /: *mut *mut __be16 l3r; / Layer 3 result,
    pub /: *mut *mut u8 l4r; / Layer 4 result,
    pub /: *mut *mut u8 cplan; / Classification plan id,
    pub /: *mut *mut __be16 nxthdr; / Next Header,
    pub /: *mut *mut __be16 cksum; / Running-sum,
// Flags&fragment-offset field of the last IP-header
    pub flags_frag_off: __be16,
// Routing type field of a IPV6 routing extension header
    pub route_type: u8,
// Routing Extension Header Present; last bit is IP valid
    pub rhp_ip_valid: u8,
    pub /: *mut *mut u8 shim_off[2]; / Shim offset,
    pub /: *mut *mut u8 ip_pid_off; / IP PID (last IP-proto) offset,
    pub /: *mut *mut u8 eth_off; / ETH offset,
    pub /: *mut *mut u8 llc_snap_off; / LLC_SNAP offset,
    pub /: *mut *mut u8 vlan_off[2]; / VLAN offset,
    pub /: *mut *mut u8 etype_off; / ETYPE offset,
    pub /: *mut *mut u8 pppoe_off; / PPP offset,
    pub /: *mut *mut u8 mpls_off[2]; / MPLS offset,
    pub /: *mut *mut u8 ip_off[2]; / IP offset,
    pub /: *mut *mut u8 gre_off; / GRE offset,
    pub /: *mut *mut u8 l4_off; / Layer 4 offset,
    pub /: *mut *mut u8 nxthdr_off; / Parser end point,
}

// A structure for defining buffer prefix area content.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fman_buffer_prefix_content {
// Number of bytes to be left at the beginning of the external
// buffer; Note that the private-area will start from the base
// of the buffer address.
//
    pub priv_data_size: u16,
// true to pass the parse result to/from the FM;
// User may use FM_PORT_GetBufferPrsResult() in
// order to get the parser-result from a buffer.
//
    pub pass_prs_result: bool,
// true to pass the timeStamp to/from the FM User
    pub pass_time_stamp: bool,
// true to pass the KG hash result to/from the FM User may
// use FM_PORT_GetBufferHashResult() in order to get the
// parser-result from a buffer.
//
    pub pass_hash_result: bool,
// Add all other Internal-Context information: AD,
// hash-result, key, etc.
//
    pub data_align: u16,
}

// A structure of information about each of the external
// buffer pools used by a port or storage-profile.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fman_ext_pool_params {
    pub /: *mut *mut u8 id; / External buffer pool id,
    pub /: *mut *mut u16 size; / External buffer pool buffer size,
}

// A structure for informing the driver about the external
// buffer pools allocated in the BM and used by a port or a
// storage-profile.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fman_ext_pools {
    pub /: *mut *mut u8 num_of_pools_used; / Number of pools use by this port,
    pub ext_buf_pool: [fman_ext_pool_params; FMAN_PORT_MAX_EXT_POOLS_NUM],
// Parameters for each port
}

// A structure for defining BM pool depletion criteria
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fman_buf_pool_depletion {
// select mode in which pause frames will be sent after a
// number of pools (all together!) are depleted
//
    pub pools_grp_mode_enable: bool,
// the number of depleted pools that will invoke pause
// frames transmission.
//
    pub num_of_pools: u8,
// For each pool, true if it should be considered for
// depletion (Note - this pool must be used by this port!).
//
    pub pools_to_consider: [bool; BM_MAX_NUM_OF_POOLS],
// select mode in which pause frames will be sent
// after a single-pool is depleted;
//
    pub single_pool_mode_enable: bool,
// For each pool, true if it should be considered
// for depletion (Note - this pool must be used by this port!)
//
    pub pools_to_consider_for_single_mode: [bool; BM_MAX_NUM_OF_POOLS],
}

// Enum for inter-module interrupts registration
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fman_event_modules {
    FMAN_MOD_MAC = 0,		/* MAC event */
    FMAN_MOD_FMAN_CTRL,	/* FMAN Controller */
    FMAN_MOD_DUMMY_LAST
}

// Enum for interrupts types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fman_intr_type {
    FMAN_INTR_TYPE_ERR,
    FMAN_INTR_TYPE_NORMAL
}

// Enum for inter-module interrupts registration
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fman_inter_module_event {
    FMAN_EV_ERR_MAC0 = 0,	/* MAC 0 error event */
    FMAN_EV_ERR_MAC1,		/* MAC 1 error event */
    FMAN_EV_ERR_MAC2,		/* MAC 2 error event */
    FMAN_EV_ERR_MAC3,		/* MAC 3 error event */
    FMAN_EV_ERR_MAC4,		/* MAC 4 error event */
    FMAN_EV_ERR_MAC5,		/* MAC 5 error event */
    FMAN_EV_ERR_MAC6,		/* MAC 6 error event */
    FMAN_EV_ERR_MAC7,		/* MAC 7 error event */
    FMAN_EV_ERR_MAC8,		/* MAC 8 error event */
    FMAN_EV_ERR_MAC9,		/* MAC 9 error event */
    FMAN_EV_MAC0,		/* MAC 0 event (Magic packet detection) */
    FMAN_EV_MAC1,		/* MAC 1 event (Magic packet detection) */
    FMAN_EV_MAC2,		/* MAC 2 (Magic packet detection) */
    FMAN_EV_MAC3,		/* MAC 3 (Magic packet detection) */
    FMAN_EV_MAC4,		/* MAC 4 (Magic packet detection) */
    FMAN_EV_MAC5,		/* MAC 5 (Magic packet detection) */
    FMAN_EV_MAC6,		/* MAC 6 (Magic packet detection) */
    FMAN_EV_MAC7,		/* MAC 7 (Magic packet detection) */
    FMAN_EV_MAC8,		/* MAC 8 event (Magic packet detection) */
    FMAN_EV_MAC9,		/* MAC 9 event (Magic packet detection) */
    FMAN_EV_FMAN_CTRL_0,	/* Fman controller event 0 */
    FMAN_EV_FMAN_CTRL_1,	/* Fman controller event 1 */
    FMAN_EV_FMAN_CTRL_2,	/* Fman controller event 2 */
    FMAN_EV_FMAN_CTRL_3,	/* Fman controller event 3 */
    FMAN_EV_CNT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fman_intr_src {
    pub src_arg): *mut *mut void (isr_cb)(void,
    pub src_handle: *mut c_void,
}

// fman_exceptions_cb
// fman         - Pointer to FMan
// exception    - The exception.
//
// Exceptions user callback routine, will be called upon an exception
// passing the exception identification.
//
// Return: irq status
//
// fman_bus_error_cb
// fman         - Pointer to FMan
// port_id      - Port id
// addr         - Address that caused the error
// tnum         - Owner of error
// liodn        - Logical IO device number
//
// Bus error user callback routine, will be called upon bus error,
// passing parameters describing the errors and the owner.
//
// Return: IRQ status
//
// Structure that holds information received from device tree
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fman_dts_params {
    pub /: *mut *mut *mut void __iomem base_addr; / FMan virtual address,
    pub /: *mut *mut *mut resource res; / FMan memory resource,
    pub /: *mut *mut u8 id; / FMan ID,
    pub /: *mut *mut int err_irq; / FMan Error IRQ,
    pub /: *mut *mut u16 clk_freq; / FMan clock freq (In Mhz),
    pub /: *mut *mut u32 qman_channel_base; / QMan channels base,
    pub /: *mut *mut u32 num_of_qman_channels; / Number of QMan channels,
    pub /: *mut *mut resource muram_res; / MURAM resource,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fman {
    pub dev: *mut device,
    pub base_addr: *mut void __iomem,
    pub intr_mng: [fman_intr_src; FMAN_EV_CNT],
    pub fpm_regs: *mut fman_fpm_regs __iomem,
    pub bmi_regs: *mut fman_bmi_regs __iomem,
    pub qmi_regs: *mut fman_qmi_regs __iomem,
    pub dma_regs: *mut fman_dma_regs __iomem,
    pub hwp_regs: *mut fman_hwp_regs __iomem,
    pub kg_regs: *mut fman_kg_regs __iomem,
    pub exception_cb: *mut fman_exceptions_cb,
    pub bus_error_cb: *mut fman_bus_error_cb,
// Spinlock for FMan use
    pub spinlock: spinlock_t,
    pub state: *mut fman_state_struct,
    pub cfg: *mut fman_cfg,
    pub muram: *mut muram_info,
    pub keygen: *mut fman_keygen,
// cam section in muram
    pub cam_offset: c_ulong,
    pub cam_size: usize,
// Fifo in MURAM
    pub fifo_offset: c_ulong,
    pub fifo_size: usize,
    pub liodn_base: [u32; 64],
    pub liodn_offset: [u32; 64],
    pub dts_params: fman_dts_params,
}

// Structure for port-FM communication during fman_port_init.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fman_port_init_params {
    pub /: *mut *mut u8 port_id; / port Id,
    pub /: *mut *mut fman_port_type port_type; / Port type,
    pub /: *mut *mut u16 port_speed; / Port speed,
    pub /: *mut *mut u16 liodn_offset; / Port's requested resource,
    pub /: *mut *mut u8 num_of_tasks; / Port's requested resource,
    pub /: *mut *mut u8 num_of_extra_tasks; / Port's requested resource,
    pub /: *mut *mut u8 num_of_open_dmas; / Port's requested resource,
    pub /: *mut *mut u8 num_of_extra_open_dmas; / Port's requested resource,
    pub /: *mut *mut u32 size_of_fifo; / Port's requested resource,
    pub /: *mut *mut u32 extra_size_of_fifo; / Port's requested resource,
    pub /: *mut *mut u8 deq_pipeline_depth; / Port's requested resource,
    pub /: *mut *mut u16 max_frame_length; / Port's max frame length.,
    pub liodn_base: u16,
// LIODN base for this port, to be used together with LIODN offset.
}

extern "C" {
    pub fn fman_get_revision(fman: *mut fman, rev_info: *mut fman_rev_info);
}
extern "C" {
    pub fn fman_reset_mac(fman: *mut fman, mac_id: u8) -> c_int;
}
extern "C" {
    pub fn fman_get_clock_freq(fman: *mut fman) -> u16;
}
extern "C" {
    pub fn fman_get_bmi_max_fifo_size(fman: *mut fman) -> u32;
}
extern "C" {
    pub fn fman_set_mac_max_frame(fman: *mut fman, mac_id: u8, mfl: u16) -> c_int;
}
extern "C" {
    pub fn fman_get_qman_channel_id(fman: *mut fman, port_id: u32) -> u32;
}
extern "C" {
    pub fn fman_get_max_frm() -> u16;
}
extern "C" {
    pub fn fman_get_rx_extra_headroom() -> c_int;
}

extern "C" {
    pub fn fman_has_errata_a050385() -> bool;
}

