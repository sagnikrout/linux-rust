//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/mvsas/mv_sas.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Marvell 88SE64xx/88SE94xx main function head file
//
// Copyright 2007 Red Hat, Inc.
// Copyright 2008 Marvell. <kewei@marvell.com>
// Copyright 2009-2011 Marvell. <yuxiangl@marvell.com>
//

pub const MVS_ID_NOT_MAPPED: c_uint = 0x7f;
pub const WIDE_PORT_MAX_PHY: c_int = 4;

pub const MV_MAX_U32: c_uint = 0xffffffff;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dev_status {
    MVS_DEV_NORMAL = 0x0,
    MVS_DEV_EH	= 0x1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dev_reset {
    MVS_SOFT_RESET	= 0,
    MVS_HARD_RESET	= 1,
    MVS_PHY_TUNE	= 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvs_dispatch {
    pub name: *mut c_char,
    pub mvi): *mut *mut int (chip_init)(struct mvs_info,
    pub mvi): *mut *mut int (spi_init)(struct mvs_info,
    pub mvi): *mut *mut int (chip_ioremap)(struct mvs_info,
    pub mvi): *mut *mut void (chip_iounmap)(struct mvs_info,
    pub stat): *mut *mut *mut irqreturn_t (isr)(struct mvs_info mvi, int irq, u32,
    pub irq): *mut *mut *mut u32 (isr_status)(struct mvs_info mvi, int,
    pub mvi): *mut *mut void (interrupt_enable)(struct mvs_info,
    pub mvi): *mut *mut void (interrupt_disable)(struct mvs_info,
    pub port): *mut *mut *mut u32 (read_phy_ctl)(struct mvs_info mvi, u32,
    pub val): *mut *mut *mut void (write_phy_ctl)(struct mvs_info mvi, u32 port, u32,
    pub port): *mut *mut *mut u32 (read_port_cfg_data)(struct mvs_info mvi, u32,
    pub val): *mut *mut *mut void (write_port_cfg_data)(struct mvs_info mvi, u32 port, u32,
    pub addr): *mut *mut *mut void (write_port_cfg_addr)(struct mvs_info mvi, u32 port, u32,
    pub port): *mut *mut *mut u32 (read_port_vsr_data)(struct mvs_info mvi, u32,
    pub val): *mut *mut *mut void (write_port_vsr_data)(struct mvs_info mvi, u32 port, u32,
    pub addr): *mut *mut *mut void (write_port_vsr_addr)(struct mvs_info mvi, u32 port, u32,
    pub port): *mut *mut *mut u32 (read_port_irq_stat)(struct mvs_info mvi, u32,
    pub val): *mut *mut *mut void (write_port_irq_stat)(struct mvs_info mvi, u32 port, u32,
    pub port): *mut *mut *mut u32 (read_port_irq_mask)(struct mvs_info mvi, u32,
    pub val): *mut *mut *mut void (write_port_irq_mask)(struct mvs_info mvi, u32 port, u32,
    pub slot_idx): *mut *mut *mut void (command_active)(struct mvs_info mvi, u32,
    pub clear_all): *mut *mut *mut void (clear_srs_irq)(struct mvs_info mvi, u8 reg_set, u8,
    pub tfs): u32,
    pub tx): *mut *mut *mut void (start_delivery)(struct mvs_info mvi, u32,
    pub mvi): *mut *mut u32 (rx_update)(struct mvs_info,
    pub mvi): *mut *mut void (int_full)(struct mvs_info,
    pub tfs): *mut *mut *mut u8 (assign_reg_set)(struct mvs_info mvi, u8,
    pub tfs): *mut *mut *mut void (free_reg_set)(struct mvs_info mvi, u8,
    pub (*prd_size)(void): *mut u32,
    pub (*prd_count)(void): *mut u32,
    pub prd): *mut *mut *mut void (make_prd)(struct scatterlist scatter, int nr, void,
    pub i): *mut *mut *mut void (detect_porttype)(struct mvs_info mvi, int,
    pub i): *mut *mut *mut int (oob_done)(struct mvs_info mvi, int,
    pub id): *mut sas_identify_frame,
    pub i): *mut *mut *mut void (phy_work_around)(struct mvs_info mvi, int,
    pub rates): *mut sas_phy_linkrates,
    pub (*phy_max_link_rate)(void): *mut u32,
    pub phy_id): *mut *mut *mut void (phy_disable)(struct mvs_info mvi, u32,
    pub phy_id): *mut *mut *mut void (phy_enable)(struct mvs_info mvi, u32,
    pub hard): *mut *mut *mut void (phy_reset)(struct mvs_info mvi, u32 phy_id, int,
    pub phy_id): *mut *mut *mut void (stp_reset)(struct mvs_info mvi, u32,
    pub mvi): *mut *mut void (clear_active_cmds)(struct mvs_info,
    pub mvi): *mut *mut u32 (spi_read_data)(struct mvs_info,
    pub data): *mut *mut *mut void (spi_write_data)(struct mvs_info mvi, u32,
    pub cmd): *mut *mut *mut int (spi_issuecmd)(struct mvs_info mvi, u32,
    pub timeout): *mut *mut *mut int (spi_waitdataready)(struct mvs_info mvi, u32,
    pub prd): *mut int buf_len, int from, void,
    pub time): *mut *mut *mut void (tune_interrupt)(struct mvs_info mvi, u32,
    pub mvi): *mut *mut void (non_spec_ncq_error)(struct mvs_info,
    pub write_data): *mut u8 reg_index, u8 reg_count, u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvs_chip_info {
    pub n_host: u32,
    pub n_phy: u32,
    pub fis_offs: u32,
    pub fis_count: u32,
    pub srs_sz: u32,
    pub sg_width: u32,
    pub slot_width: u32,
    pub dispatch: *const mvs_dispatch,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvs_err_info {
    pub flags: __le32,
    pub flags2: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvs_cmd_hdr {
    pub /: *mut *mut __le32 flags; / PRD tbl len; SAS, SATA ctl,
    pub /: *mut *mut __le32 lens; / cmd, max resp frame len,
    pub /: *mut *mut __le32 tags; / targ port xfer tag; tag,
    pub /: *mut *mut __le32 data_len; / data xfer len,
    pub /: *mut *mut __le64 cmd_tbl; / command table address,
    pub /: *mut *mut __le64 open_frame; / open addr frame address,
    pub /: *mut *mut __le64 status_buf; / status buffer address,
    pub /: *mut *mut __le64 prd_tbl; / PRD tbl address,
    pub reserved: [__le32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvs_port {
    pub sas_port: asd_sas_port,
    pub port_attached: u8,
    pub wide_port_phymap: u8,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvs_phy {
    pub mvi: *mut mvs_info,
    pub port: *mut mvs_port,
    pub sas_phy: asd_sas_phy,
    pub identify: sas_identify,
    pub sdev: *mut scsi_device,
    pub timer: timer_list,
    pub dev_sas_addr: u64,
    pub att_dev_sas_addr: u64,
    pub att_dev_info: u32,
    pub dev_info: u32,
    pub phy_type: u32,
    pub phy_status: u32,
    pub irq_status: u32,
    pub frame_rcvd_size: u32,
    pub frame_rcvd: [u8; 32],
    pub phy_attached: u8,
    pub phy_mode: u8,
    pub reserved: [u8; 2],
    pub phy_event: u32,
    pub minimum_linkrate: sas_linkrate,
    pub maximum_linkrate: sas_linkrate,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvs_device {
    pub dev_entry: list_head,
    pub dev_type: sas_device_type,
    pub mvi_info: *mut mvs_info,
    pub sas_device: *mut domain_device,
    pub attached_phy: u32,
    pub device_id: u32,
    pub running_req: u32,
    pub taskfileset: u8,
    pub dev_status: u8,
    pub reserved: u16,
}

// Generate  PHY tunning parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_tuning {
// 1 bit,  transmitter emphasis enable
    pub trans_emp_en:1: u8,
// 4 bits, transmitter emphasis amplitude
    pub trans_emp_amp:4: u8,
// 3 bits, reserved space
    pub Reserved_2bit_1:3: u8,
// 5 bits, transmitter amplitude
    pub trans_amp:5: u8,
// 2 bits, transmitter amplitude adjust
    pub trans_amp_adj:2: u8,
// 1 bit, reserved space
    pub resv_2bit_2:1: u8,
// 2 bytes, reserved space
    pub reserved: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ffe_control {
// 4 bits,  FFE Capacitor Select  (value range 0~F)
    pub ffe_cap_sel:4: u8,
// 3 bits,  FFE Resistor Select (value range 0~7)
    pub ffe_rss_sel:3: u8,
// 1 bit reserve
    pub reserved:1: u8,
}

//
// HBA_Info_Page is saved in Flash/NVRAM, total 256 bytes.
// The data area is valid only Signature="MRVL".
// If any member fills with 0xFF, the member is invalid.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hba_info_page {
// Dword 0
// 4 bytes, structure signature,should be "MRVL" at first initial
    pub signature: [u8; 4],
// Dword 1-13
    pub reserved1: [u32; 13],
// Dword 14-29
// 64 bytes, SAS address for each port
    pub sas_addr: [u64; 8],
// Dword 30-31
// 8 bytes for vanir 8 port PHY FFE seeting
// BIT 0~3 : FFE Capacitor select(value range 0~F)
// BIT 4~6 : FFE Resistor select(value range 0~7)
// BIT 7: reserve.
//
    pub ffe_ctl: [ffe_control; 8],
// Dword 32 -43
    pub reserved2: [u32; 12],
// Dword 44-45
// 8 bytes,  0:  1.5G, 1: 3.0G, should be 0x01 at first initial
    pub phy_rate: [u8; 8],
// Dword 46-53
// 32 bytes, PHY tuning parameters for each PHY
    pub phy_tuning: [phy_tuning; 8],
// Dword 54-63
    pub reserved3: [u32; 10],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvs_slot_info {
    pub entry: list_head,
    pub task: *mut sas_task,
    pub tdata: *mut c_void,
}

// DMA buffer for storing cmd tbl, open addr frame, status buffer,
// and PRD table
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvs_info {
    pub flags: c_ulong,
// host-wide lock
    pub lock: spinlock_t,
// our device
    pub pdev: *mut pci_dev,
    pub dev: *mut device,
// enhanced mode registers
    pub regs: *mut void __iomem,
// peripheral or soc registers
    pub regs_ex: *mut void __iomem,
    pub sas_addr: [u8; SAS_ADDR_SIZE],
// SCSI/SAS glue
    pub sas: *mut sas_ha_struct,
    pub shost: *mut Scsi_Host,
// TX (delivery) DMA ring
    pub tx: *mut __le32,
    pub tx_dma: dma_addr_t,
// cached next-producer idx
    pub tx_prod: u32,
// RX (completion) DMA ring
    pub rx: *mut __le32,
    pub rx_dma: dma_addr_t,
// RX consumer idx
    pub rx_cons: u32,
// RX'd FIS area
    pub rx_fis: *mut __le32,
    pub rx_fis_dma: dma_addr_t,
// DMA command header slots
    pub slot: *mut mvs_cmd_hdr,
    pub slot_dma: dma_addr_t,
    pub chip_id: u32,
    pub chip: *const mvs_chip_info,
    pub rsvd_tags: *mut c_ulong,
// further per-slot information
    pub phy: [mvs_phy; MVS_MAX_PHYS],
    pub port: [mvs_port; MVS_MAX_PHYS],
    pub id: u32,
    pub sata_reg_set: u64,
    pub hba_list: *mut list_head,
    pub soc_entry: list_head,
    pub wq_list: list_head,
    pub instance: c_ulong,
    pub flashid: u16,
    pub flashsize: u32,
    pub flashsectSize: u32,
    pub addon: *mut c_void,
    pub hba_info_param: hba_info_page,
    pub devices: [mvs_device; MVS_MAX_DEVICES],
    pub bulk_buffer: *mut c_void,
    pub bulk_buffer_dma: dma_addr_t,
    pub bulk_buffer1: *mut c_void,
    pub bulk_buffer_dma1: dma_addr_t,
pub const TRASH_BUCKET_SIZE: c_uint = 0x20000;
    pub dma_pool: *mut c_void,
    pub slot_info: [mvs_slot_info; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvs_prv_info {
    pub n_host: u8,
    pub n_phy: u8,
    pub scan_finished: u8,
    pub reserve: u8,
    pub mvi: [*mut mvs_info; 2],
    pub mv_tasklet: tasklet_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvs_wq {
    pub work_q: delayed_work,
    pub mvi: *mut mvs_info,
    pub data: *mut c_void,
    pub handler: c_int,
    pub entry: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvs_task_exec_info {
    pub task: *mut sas_task,
    pub hdr: *mut mvs_cmd_hdr,
    pub port: *mut mvs_port,
    pub tag: u32,
    pub n_elem: c_int,
}

// function prototype
extern "C" {
    pub fn mvs_get_sas_addr(buf: *mut c_void, buflen: u32);
}
extern "C" {
    pub fn mvs_iounmap(regs: *mut void __iomem);
}
extern "C" {
    pub fn mvs_ioremap(mvi: *mut mvs_info, bar: c_int, bar_ex: c_int) -> c_int;
}
extern "C" {
    pub fn mvs_scan_start(shost: *mut Scsi_Host);
}
extern "C" {
    pub fn mvs_scan_finished(shost: *mut Scsi_Host, time: c_ulong) -> c_int;
}
extern "C" {
    pub fn mvs_queue_command(task: *mut sas_task, gfp_flags: gfp_t) -> c_int;
}
extern "C" {
    pub fn mvs_abort_task(task: *mut sas_task) -> c_int;
}
extern "C" {
    pub fn mvs_port_formed(sas_phy: *mut asd_sas_phy);
}
extern "C" {
    pub fn mvs_port_deformed(sas_phy: *mut asd_sas_phy);
}
extern "C" {
    pub fn mvs_dev_found(dev: *mut domain_device) -> c_int;
}
extern "C" {
    pub fn mvs_dev_gone(dev: *mut domain_device);
}
extern "C" {
    pub fn mvs_lu_reset(dev: *mut domain_device, lun: *mut u8) -> c_int;
}
extern "C" {
    pub fn mvs_slot_complete(mvi: *mut mvs_info, rx_desc: u32, flags: u32) -> c_int;
}
extern "C" {
    pub fn mvs_I_T_nexus_reset(dev: *mut domain_device) -> c_int;
}
extern "C" {
    pub fn mvs_query_task(task: *mut sas_task) -> c_int;
}
extern "C" {
    pub fn mvs_int_port(mvi: *mut mvs_info, phy_no: c_int, events: u32);
}
extern "C" {
    pub fn mvs_update_phyinfo(mvi: *mut mvs_info, i: c_int, get_st: c_int);
}
extern "C" {
    pub fn mvs_int_rx(mvi: *mut mvs_info, self_clear: bool) -> c_int;
}
