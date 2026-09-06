//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/pm8001/pm8001_sas.h
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


//
// PMC-Sierra PM8001/8081/8088/8089 SAS/SATA based host adapters driver
//
// Copyright (c) 2008-2009 USI Co., Ltd.
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions, and the following disclaimer,
// without modification.
// 2. Redistributions in binary form must reproduce at minimum a disclaimer
// substantially similar to the "NO WARRANTY" disclaimer below
// ("Disclaimer") and any redistribution must be conditioned upon
// including a substantially similar Disclaimer requirement for further
// binary redistribution.
// 3. Neither the names of the above-listed copyright holders nor the names
// of any contributors may be used to endorse or promote products derived
// from this software without specific prior written permission.
//
// Alternatively, this software may be distributed under the terms of the
// GNU General Public License ("GPL") version 2 as published by the Free
// Software Foundation.
//
// NO WARRANTY
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// HOLDERS OR CONTRIBUTORS BE LIABLE FOR SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
// OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
// STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING
// IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGES.
//

pub const PM8001_FAIL_LOGGING: c_uint = 0x01 /* Error message logging */;
pub const PM8001_INIT_LOGGING: c_uint = 0x02 /* driver init logging */;
pub const PM8001_DISC_LOGGING: c_uint = 0x04 /* discovery layer logging */;
pub const PM8001_IO_LOGGING: c_uint = 0x08 /* I/O path logging */;
pub const PM8001_EH_LOGGING: c_uint = 0x10 /* libsas EH function logging*/;
pub const PM8001_IOCTL_LOGGING: c_uint = 0x20 /* IOCTL message logging */;
pub const PM8001_MSG_LOGGING: c_uint = 0x40 /* misc message logging */;
pub const PM8001_DEV_LOGGING: c_uint = 0x80 /* development message logging */;
pub const PM8001_DEVIO_LOGGING: c_uint = 0x100 /* development io message logging */;
pub const PM8001_IOERR_LOGGING: c_uint = 0x200 /* development io err message logging */;
pub const PM8001_EVENT_LOGGING: c_uint = 0x400 /* HW event logging */;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm8001_ioctl_payload {
    pub signature: u32,
    pub major_function: u16,
    pub minor_function: u16,
    pub status: u16,
    pub offset: u16,
    pub id: u16,
    pub wr_length: u32,
    pub rd_length: u32,
    pub func_specific: *mut u8,
}

pub const MPI_FATAL_ERROR_TABLE_OFFSET_MASK: c_uint = 0xFFFFFF;

pub const MPI_FATAL_EDUMP_TABLE_LO_OFFSET: c_uint = 0x00     /* HNFBUFL */;
pub const MPI_FATAL_EDUMP_TABLE_HI_OFFSET: c_uint = 0x04     /* HNFBUFH */;
pub const MPI_FATAL_EDUMP_TABLE_LENGTH: c_uint = 0x08     /* HNFBLEN */;
pub const MPI_FATAL_EDUMP_TABLE_HANDSHAKE: c_uint = 0x0C     /* FDDHSHK */;
pub const MPI_FATAL_EDUMP_TABLE_STATUS: c_uint = 0x10     /* FDDTSTAT */;
pub const MPI_FATAL_EDUMP_TABLE_ACCUM_LEN: c_uint = 0x14     /* ACCDDLEN */;
pub const MPI_FATAL_EDUMP_TABLE_TOTAL_LEN: c_uint = 0x18	    /* TOTALLEN */;
pub const MPI_FATAL_EDUMP_TABLE_SIGNATURE: c_uint = 0x1C     /* SIGNITURE */;
pub const MPI_FATAL_EDUMP_HANDSHAKE_RDY: c_uint = 0x1;
pub const MPI_FATAL_EDUMP_HANDSHAKE_BUSY: c_uint = 0x0;
pub const MPI_FATAL_EDUMP_TABLE_STAT_RSVD: c_uint = 0x0;
pub const MPI_FATAL_EDUMP_TABLE_STAT_DMA_FAILED: c_uint = 0x1;
pub const MPI_FATAL_EDUMP_TABLE_STAT_NF_SUCCESS_MORE_DATA: c_uint = 0x2;
pub const MPI_FATAL_EDUMP_TABLE_STAT_NF_SUCCESS_DONE: c_uint = 0x3;
pub const TYPE_GSM_SPACE: c_int = 1;
pub const TYPE_QUEUE: c_int = 2;
pub const TYPE_FATAL: c_int = 3;
pub const TYPE_NON_FATAL: c_int = 4;
pub const TYPE_INBOUND: c_int = 1;
pub const TYPE_OUTBOUND: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct forensic_data {
    pub data_type: u32,
    pub direct_len: u32,
    pub direct_offset: u32,
    pub direct_data: *mut c_void,
    pub gsm_buf: },
    pub queue_type: u16,
    pub queue_index: u16,
    pub direct_len: u32,
    pub direct_data: *mut c_void,
    pub queue_buf: },
    pub direct_len: u32,
    pub direct_offset: u32,
    pub read_len: u32,
    pub direct_data: *mut c_void,
    pub data_buf: },
}

// bit31-26 - mask bar
pub const SCRATCH_PAD0_BAR_MASK: c_uint = 0xFC000000;
// bit25-0  - offset mask
pub const SCRATCH_PAD0_OFFSET_MASK: c_uint = 0x03FFFFFF;
// if AAP error state
pub const SCRATCH_PAD0_AAPERR_MASK: c_uint = 0xFFFFFFFF;
// Inbound doorbell bit7
pub const SPCv_MSGU_CFG_TABLE_NONFATAL_DUMP: c_uint = 0x80;
// Inbound doorbell bit7 SPCV
pub const SPCV_MSGU_CFG_TABLE_TRANSFER_DEBUG_INFO: c_uint = 0x80;
pub const MAIN_MERRDCTO_MERRDCES: c_uint = 0xA0/* DWORD 0x28) */;
//
// enum fatal_error_reporter: Indicates the originator of the fatal error
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fatal_error_reporter {
    REPORTER_DRIVER,
    REPORTER_FIRMWARE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm8001_dispatch {
    pub name: *mut c_char,
    pub pm8001_ha): *mut *mut int (chip_init)(struct pm8001_hba_info,
    pub pm8001_ha): *mut *mut void (chip_post_init)(struct pm8001_hba_info,
    pub pm8001_ha): *mut *mut int (chip_soft_rst)(struct pm8001_hba_info,
    pub pm8001_ha): *mut *mut void (chip_rst)(struct pm8001_hba_info,
    pub pm8001_ha): *mut *mut int (chip_ioremap)(struct pm8001_hba_info,
    pub pm8001_ha): *mut *mut void (chip_iounmap)(struct pm8001_hba_info,
    pub vec): *mut *mut *mut irqreturn_t (isr)(struct pm8001_hba_info pm8001_ha, u8,
    pub pm8001_ha): *mut *mut u32 (is_our_interrupt)(struct pm8001_hba_info,
    pub vec): *mut *mut *mut int (isr_process_oq)(struct pm8001_hba_info pm8001_ha, u8,
    pub vec): *mut *mut *mut void (interrupt_enable)(struct pm8001_hba_info pm8001_ha, u8,
    pub vec): *mut *mut *mut void (interrupt_disable)(struct pm8001_hba_info pm8001_ha, u8,
    pub prd): *mut *mut *mut void (make_prd)(struct scatterlist scatter, int nr, void,
    pub ccb): *mut pm8001_ccb_info,
    pub ccb): *mut pm8001_ccb_info,
    pub ccb): *mut pm8001_ccb_info,
    pub phy_id): *mut *mut *mut int (phy_start_req)(struct pm8001_hba_info pm8001_ha, u8,
    pub phy_id): *mut *mut *mut int (phy_stop_req)(struct pm8001_hba_info pm8001_ha, u8,
    pub flag): *mut *mut pm8001_device pm8001_dev, u32,
    pub device_id): *mut *mut *mut int (dereg_dev_req)(struct pm8001_hba_info pm8001_ha, u32,
    pub phy_op): u32 phy_id, u32,
    pub ccb): *mut pm8001_ccb_info,
    pub tmf): *mut *mut pm8001_ccb_info ccb, sas_tmf_task,
    pub payload): *mut *mut *mut int (get_nvmd_req)(struct pm8001_hba_info pm8001_ha, void,
    pub payload): *mut *mut *mut int (set_nvmd_req)(struct pm8001_hba_info pm8001_ha, void,
    pub payload): *mut c_void,
    pub state): *mut *mut pm8001_device pm8001_dev, u32,
    pub state): u32,
    pub state): u32,
    pub pm8001_ha): *mut *mut int (sas_re_init_req)(struct pm8001_hba_info,
    pub pm8001_ha): *mut *mut int (fatal_errors)(struct pm8001_hba_info,
    pub param1): u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm8001_chip_info {
    pub encrypt: u32,
    pub n_phy: u32,
    pub dispatch: *const pm8001_dispatch,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm8001_port {
    pub sas_port: asd_sas_port,
    pub port_attached: u8,
    pub wide_port_phymap: u16,
    pub port_state: u8,
    pub port_id: u8,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm8001_phy {
    pub pm8001_ha: *mut pm8001_hba_info,
    pub port: *mut pm8001_port,
    pub sas_phy: asd_sas_phy,
    pub identify: sas_identify,
    pub sdev: *mut scsi_device,
    pub dev_sas_addr: u64,
    pub phy_type: u32,
    pub enable_completion: *mut completion,
    pub frame_rcvd_size: u32,
    pub frame_rcvd: [u8; 32],
    pub phy_attached: u8,
    pub phy_state: u8,
    pub minimum_linkrate: sas_linkrate,
    pub maximum_linkrate: sas_linkrate,
    pub reset_completion: *mut completion,
    pub port_reset_status: bool,
    pub reset_success: bool,
}

// port reset status
pub const PORT_RESET_SUCCESS: c_uint = 0x00;
pub const PORT_RESET_TMO: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm8001_device {
    pub dev_type: sas_device_type,
    pub sas_device: *mut domain_device,
    pub attached_phy: u32,
    pub id: u32,
    pub dcompletion: *mut completion,
    pub setds_completion: *mut completion,
    pub device_id: u32,
    pub running_req: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm8001_prd_imt {
    pub len: __le32,
    pub e: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm8001_prd {
    pub /: *mut *mut __le64 addr; / 64-bit buffer address,
    pub /: *mut *mut pm8001_prd_imt im_len; / 64-bit length,
// C attribute field omitted
//
// CCB(Command Control Block)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm8001_ccb_info {
    pub task: *mut sas_task,
    pub n_elem: u32,
    pub ccb_tag: u32,
    pub ccb_dma_handle: dma_addr_t,
    pub device: *mut pm8001_device,
    pub buf_prd: *mut pm8001_prd,
    pub fw_control_context: *mut fw_control_ex,
    pub open_retry: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi_mem {
    pub virt_ptr: *mut c_void,
    pub phys_addr: dma_addr_t,
    pub phys_addr_hi: u32,
    pub phys_addr_lo: u32,
    pub total_len: u32,
    pub num_elements: u32,
    pub element_size: u32,
    pub alignment: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpi_mem_req {
// The number of element in the  mpiMemory array
    pub count: u32,
// The array of structures that define memroy regions
    pub region: [mpi_mem; USI_MAX_MEMCNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct encrypt {
    pub cipher_mode: u32,
    pub sec_mode: u32,
    pub status: u32,
    pub flag: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sas_phy_attribute_table {
    pub phystart1_16: [u32; 16],
    pub outbound_hw_event_pid1_16: [u32; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union main_cfg_table {
    pub signature: u32,
    pub interface_rev: u32,
    pub firmware_rev: u32,
    pub max_out_io: u32,
    pub max_sgl: u32,
    pub ctrl_cap_flag: u32,
    pub gst_offset: u32,
    pub inbound_queue_offset: u32,
    pub outbound_queue_offset: u32,
    pub inbound_q_nppd_hppd: u32,
    pub outbound_hw_event_pid0_3: u32,
    pub outbound_hw_event_pid4_7: u32,
    pub outbound_ncq_event_pid0_3: u32,
    pub outbound_ncq_event_pid4_7: u32,
    pub outbound_tgt_ITNexus_event_pid0_3: u32,
    pub outbound_tgt_ITNexus_event_pid4_7: u32,
    pub outbound_tgt_ssp_event_pid0_3: u32,
    pub outbound_tgt_ssp_event_pid4_7: u32,
    pub outbound_tgt_smp_event_pid0_3: u32,
    pub outbound_tgt_smp_event_pid4_7: u32,
    pub upper_event_log_addr: u32,
    pub lower_event_log_addr: u32,
    pub event_log_size: u32,
    pub event_log_option: u32,
    pub upper_iop_event_log_addr: u32,
    pub lower_iop_event_log_addr: u32,
    pub iop_event_log_size: u32,
    pub iop_event_log_option: u32,
    pub fatal_err_interrupt: u32,
    pub fatal_err_dump_offset0: u32,
    pub fatal_err_dump_length0: u32,
    pub fatal_err_dump_offset1: u32,
    pub fatal_err_dump_length1: u32,
    pub hda_mode_flag: u32,
    pub anolog_setup_table_offset: u32,
    pub rsvd: [u32; 4],
    pub pm8001_tbl: },
    pub signature: u32,
    pub interface_rev: u32,
    pub firmware_rev: u32,
    pub max_out_io: u32,
    pub max_sgl: u32,
    pub ctrl_cap_flag: u32,
    pub gst_offset: u32,
    pub inbound_queue_offset: u32,
    pub outbound_queue_offset: u32,
    pub inbound_q_nppd_hppd: u32,
    pub rsvd: [u32; 8],
    pub crc_core_dump: u32,
    pub rsvd1: u32,
    pub upper_event_log_addr: u32,
    pub lower_event_log_addr: u32,
    pub event_log_size: u32,
    pub event_log_severity: u32,
    pub upper_pcs_event_log_addr: u32,
    pub lower_pcs_event_log_addr: u32,
    pub pcs_event_log_size: u32,
    pub pcs_event_log_severity: u32,
    pub fatal_err_interrupt: u32,
    pub fatal_err_dump_offset0: u32,
    pub fatal_err_dump_length0: u32,
    pub fatal_err_dump_offset1: u32,
    pub fatal_err_dump_length1: u32,
    pub gpio_led_mapping: u32,
    pub analog_setup_table_offset: u32,
    pub int_vec_table_offset: u32,
    pub phy_attr_table_offset: u32,
    pub port_recovery_timer: u32,
    pub interrupt_reassertion_delay: u32,
    pub /: *mut *mut u32 fatal_n_non_fatal_dump; / 0x28,
    pub ila_version: u32,
    pub inc_fw_version: u32,
    pub pm80xx_tbl: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union general_status_table {
    pub gst_len_mpistate: u32,
    pub iq_freeze_state0: u32,
    pub iq_freeze_state1: u32,
    pub msgu_tcnt: u32,
    pub iop_tcnt: u32,
    pub rsvd: u32,
    pub phy_state: [u32; 8],
    pub gpio_input_val: u32,
    pub rsvd1: [u32; 2],
    pub recover_err_info: [u32; 8],
    pub pm8001_tbl: },
    pub gst_len_mpistate: u32,
    pub iq_freeze_state0: u32,
    pub iq_freeze_state1: u32,
    pub msgu_tcnt: u32,
    pub iop_tcnt: u32,
    pub rsvd: [u32; 9],
    pub gpio_input_val: u32,
    pub rsvd1: [u32; 2],
    pub recover_err_info: [u32; 8],
    pub pm80xx_tbl: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inbound_queue_table {
    pub element_pri_size_cnt: u32,
    pub upper_base_addr: u32,
    pub lower_base_addr: u32,
    pub ci_upper_base_addr: u32,
    pub ci_lower_base_addr: u32,
    pub pi_pci_bar: u32,
    pub pi_offset: u32,
    pub total_length: u32,
    pub base_virt: *mut c_void,
    pub ci_virt: *mut c_void,
    pub reserved: u32,
    pub consumer_index: __le32,
    pub producer_idx: u32,
    pub iq_lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct outbound_queue_table {
    pub element_size_cnt: u32,
    pub upper_base_addr: u32,
    pub lower_base_addr: u32,
    pub base_virt: *mut c_void,
    pub pi_upper_base_addr: u32,
    pub pi_lower_base_addr: u32,
    pub ci_pci_bar: u32,
    pub ci_offset: u32,
    pub total_length: u32,
    pub pi_virt: *mut c_void,
    pub interrup_vec_cnt_delay: u32,
    pub dinterrup_to_pci_offset: u32,
    pub producer_index: __le32,
    pub consumer_idx: u32,
    pub oq_lock: spinlock_t,
    pub lock_flags: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm8001_hba_memspace {
    pub memvirtaddr: *mut void __iomem,
    pub membase: u64,
    pub memsize: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isr_param {
    pub drv_inst: *mut pm8001_hba_info,
    pub irq_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm8001_hba_info {
    pub name: [c_char; PM8001_NAME_LENGTH],
    pub list: list_head,
    pub flags: c_ulong,
    pub /: *mut *mut spinlock_t lock;/ host-wide lock,
    pub bitmap_lock: spinlock_t,
    pub /: *mut *mut *mut pci_dev pdev;/ our device,
    pub dev: *mut device,
    pub io_mem: [pm8001_hba_memspace; 6],
    pub memoryMap: mpi_mem_req,
    pub /: *mut *mut encrypt encrypt_info; / support encryption,
    pub forensic_info: forensic_data,
    pub fatal_bar_loc: u32,
    pub forensic_last_offset: u32,
    pub fatal_forensic_shift_offset: u32,
    pub forensic_fatal_step: u32,
    pub forensic_preserved_accumulated_transfer: u32,
    pub evtlog_ib_offset: u32,
    pub evtlog_ob_offset: u32,
    pub Addr*/: *mut *mut *mut void __iomem msg_unit_tbl_addr;/Message Unit Table,
    pub Addr*/: *mut *mut *mut void __iomem main_cfg_tbl_addr;/Main Config Table,
    pub Addr*/: *mut *mut *mut void __iomem general_stat_tbl_addr;/General Status Table,
    pub Addr*/: *mut *mut *mut void __iomem inbnd_q_tbl_addr;/Inbound Queue Config Table,
    pub Addr*/: *mut *mut *mut void __iomem outbnd_q_tbl_addr;/Outbound Queue Config Table,
    pub pspa_q_tbl_addr: *mut void __iomem,
// MPI SAS PHY attributes Queue Config Table Addr
    pub /: *mut *mut *mut void __iomem ivt_tbl_addr; /MPI IVT Table Addr,
    pub /: *mut *mut *mut void __iomem fatal_tbl_addr; /MPI IVT Table Addr,
    pub main_cfg_tbl: main_cfg_table,
    pub gs_tbl: general_status_table,
    pub inbnd_q_tbl: [inbound_queue_table; PM8001_MAX_INB_NUM],
    pub outbnd_q_tbl: [outbound_queue_table; PM8001_MAX_OUTB_NUM],
    pub phy_attr_table: sas_phy_attribute_table,
// MPI SAS PHY attributes
    pub sas_addr: [u8; SAS_ADDR_SIZE],
    pub /: *mut *mut *mut sas_ha_sas;/ SCSI/SAS glue,
    pub shost: *mut Scsi_Host,
    pub chip_id: u32,
    pub chip: *const pm8001_chip_info,
    pub nvmd_completion: *mut completion,
    pub rsvd_tags: *mut c_ulong,
    pub phy: [pm8001_phy; PM8001_MAX_PHYS],
    pub port: [pm8001_port; PM8001_MAX_PHYS],
    pub id: u32,
    pub irq: u32,
    pub /: *mut *mut u32 iomb_size; / SPC and SPCV IOMB size,
    pub devices: *mut pm8001_device,
    pub ccb_info: *mut pm8001_ccb_info,
    pub ccb_count: u32,
    pub use_msix: bool,
    pub remove()*/: *mut *mut int number_of_intr;/will be used in,
    pub tasklet: [tasklet_struct; PM8001_MAX_MSIX_VEC],
    pub logging_level: u32,
    pub link_rate: u32,
    pub fw_status: u32,
    pub smp_exp_mode: u32,
    pub controller_fatal_error: bool,
    pub fw_image: *const firmware,
    pub irq_vector: [isr_param; PM8001_MAX_MSIX_VEC],
    pub non_fatal_count: u32,
    pub non_fatal_read_length: u32,
    pub max_q_num: u32,
    pub ib_offset: u32,
    pub ob_offset: u32,
    pub ci_offset: u32,
    pub pi_offset: u32,
    pub max_memcnt: u32,
    pub iop_log_start: u32,
    pub iop_log_end: u32,
    pub iop_log_count: u32,
    pub iop_log_lock: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm8001_work {
    pub work: work_struct,
    pub pm8001_ha: *mut pm8001_hba_info,
    pub data: *mut c_void,
    pub handler: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm8001_fw_image_header {
    pub vender_id: [u8; 8],
    pub product_id: u8,
    pub hardware_rev: u8,
    pub dest_partition: u8,
    pub reserved: u8,
    pub fw_rev: [u8; 4],
    pub image_length: __be32,
    pub image_crc: __be32,
    pub startup_entry: __be32,
// C attribute field omitted
//
// FW Flash Update status values
//
pub const FLASH_UPDATE_COMPLETE_PENDING_REBOOT: c_uint = 0x00;
pub const FLASH_UPDATE_IN_PROGRESS: c_uint = 0x01;
pub const FLASH_UPDATE_HDR_ERR: c_uint = 0x02;
pub const FLASH_UPDATE_OFFSET_ERR: c_uint = 0x03;
pub const FLASH_UPDATE_CRC_ERR: c_uint = 0x04;
pub const FLASH_UPDATE_LENGTH_ERR: c_uint = 0x05;
pub const FLASH_UPDATE_HW_ERR: c_uint = 0x06;
pub const FLASH_UPDATE_DNLD_NOT_SUPPORTED: c_uint = 0x10;
pub const FLASH_UPDATE_DISABLED: c_uint = 0x11;
// Device states
pub const DS_OPERATIONAL: c_uint = 0x01;
pub const DS_PORT_IN_RESET: c_uint = 0x02;
pub const DS_IN_RECOVERY: c_uint = 0x03;
pub const DS_IN_ERROR: c_uint = 0x04;
pub const DS_NON_OPERATIONAL: c_uint = 0x07;
//
// brief param structure for firmware flash update.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_flash_updata_info {
    pub cur_image_offset: u32,
    pub cur_image_len: u32,
    pub total_image_len: u32,
    pub sgl: pm8001_prd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_control_info {
    pub (status)*/: *mut *mut u32 retcode;/ret code,
    pub phase*/: *mut *mut u32 phase;/ret code,
    pub current: *mut *mut u32 phaseCmplt;/percent complete for the,
    pub number*/: *mut *mut u32 version;/Hex encoded firmware version,
    pub /: *mut *mut u32 offset;/Used for downloading firmware,
    pub buffer*/: *mut *mut u32 len; /len of,
    pub size: *mut *mut u32 size;/ Used in OS VPD and Trace get,
    pub bit: *mut *mut u32 reserved;/ padding required for 64,
    pub /: *mut *mut u8 buffer[];/ Start of buffer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_control_ex {
    pub fw_control: *mut fw_control_info,
    pub be: *mut *mut *mut void buffer;/ keep buffer pointer to,
    pub /: *mut *mut *mut void virtAddr;/ keep virtual address of the data,
    pub the: *mut *mut *mut void usrAddr;/ keep virtual address of,
    pub phys_addr: dma_addr_t,
    pub /: *mut *mut u32 len; / len of buffer,
    pub /: *mut *mut *mut void payload; / pointer to IOCTL Payload,
    pub in: *mut *mut u8 inProgress;/if 1 - the IOCTL request is,
    pub param1: *mut c_void,
    pub param2: *mut c_void,
    pub param3: *mut c_void,
}

// pm8001 workqueue
// function prototype
extern "C" {
    pub fn pm8001_tag_alloc(pm8001_ha: *mut pm8001_hba_info, tag_out: *mut u32) -> c_int;
}
extern "C" {
    pub fn pm8001_get_ncq_tag(task: *mut sas_task, tag: *mut u32) -> u32;
}
extern "C" {
    pub fn pm8001_scan_start(shost: *mut Scsi_Host);
}
extern "C" {
    pub fn pm8001_scan_finished(shost: *mut Scsi_Host, time: c_ulong) -> c_int;
}
extern "C" {
    pub fn pm8001_queue_command(task: *mut sas_task, gfp_flags: gfp_t) -> c_int;
}
extern "C" {
    pub fn pm8001_abort_task(task: *mut sas_task) -> c_int;
}
extern "C" {
    pub fn pm8001_clear_task_set(dev: *mut domain_device, lun: *mut u8) -> c_int;
}
extern "C" {
    pub fn pm8001_dev_found(dev: *mut domain_device) -> c_int;
}
extern "C" {
    pub fn pm8001_dev_gone(dev: *mut domain_device);
}
extern "C" {
    pub fn pm8001_lu_reset(dev: *mut domain_device, lun: *mut u8) -> c_int;
}
extern "C" {
    pub fn pm8001_I_T_nexus_reset(dev: *mut domain_device) -> c_int;
}
extern "C" {
    pub fn pm8001_I_T_nexus_event_handler(dev: *mut domain_device) -> c_int;
}
extern "C" {
    pub fn pm8001_query_task(task: *mut sas_task) -> c_int;
}
extern "C" {
    pub fn pm8001_port_formed(sas_phy: *mut asd_sas_phy);
}
extern "C" {
    pub fn pm8001_chip_iounmap(pm8001_ha: *mut pm8001_hba_info);
}
extern "C" {
    pub fn pm8001_chip_set_nvmd_req(pm8001_ha: *mut pm8001_hba_info, payload: *mut c_void) -> c_int;
}
extern "C" {
    pub fn pm8001_chip_get_nvmd_req(pm8001_ha: *mut pm8001_hba_info, payload: *mut c_void) -> c_int;
}
extern "C" {
    pub fn pm8001_chip_dereg_dev_req(pm8001_ha: *mut pm8001_hba_info, device_id: u32) -> c_int;
}
extern "C" {
    pub fn pm8001_chip_make_sg(scatter: *mut scatterlist, nr: c_int, prd: *mut c_void);
}
extern "C" {
    pub fn pm8001_work_fn(work: *mut work_struct);
}
extern "C" {
    pub fn pm8001_get_lrate_mode(phy: *mut pm8001_phy, link_rate: u8);
}
extern "C" {
    pub fn pm8001_get_attached_sas_addr(phy: *mut pm8001_phy, sas_addr: *mut u8);
}
extern "C" {
    pub fn pm8001_bytes_dmaed(pm8001_ha: *mut pm8001_hba_info, i: c_int);
}
extern "C" {
    pub fn pm8001_mpi_reg_resp(pm8001_ha: *mut pm8001_hba_info, piomb: *mut c_void) -> c_int;
}
extern "C" {
    pub fn pm8001_mpi_dereg_resp(pm8001_ha: *mut pm8001_hba_info, piomb: *mut c_void) -> c_int;
}
extern "C" {
    pub fn pm8001_mpi_general_event(pm8001_ha: *mut pm8001_hba_info, piomb: *mut c_void) -> c_int;
}
extern "C" {
    pub fn pm8001_mpi_task_abort_resp(pm8001_ha: *mut pm8001_hba_info, piomb: *mut c_void) -> c_int;
}
extern "C" {
    pub fn pm8001_tag_free(pm8001_ha: *mut pm8001_hba_info, tag: u32);
}
extern "C" {
    pub fn pm80xx_set_thermal_config(pm8001_ha: *mut pm8001_hba_info) -> c_int;
}
extern "C" {
    pub fn pm8001_bar4_shift(pm8001_ha: *mut pm8001_hba_info, shiftValue: u32) -> c_int;
}
extern "C" {
    pub fn pm80xx_bar4_shift(pm8001_ha: *mut pm8001_hba_info, shiftValue: u32) -> c_int;
}
extern "C" {
    pub fn pm8001_get_gsm_dump(cdev: *mut device, _arg: u32, buf: *mut c_char) -> isize;
}
extern "C" {
    pub fn pm80xx_fatal_errors(pm8001_ha: *mut pm8001_hba_info) -> c_int;
}
extern "C" {
    pub fn pm8001_free_dev(pm8001_dev: *mut pm8001_device);
}
// ctl shared API

//
// Allocate a new tag and return the corresponding ccb after initializing it.
//
// Free the tag of an initialized ccb.
//
// Cleanup the ccb to make sure that a manual scan of the adapter
// ccb_info array can detect ccb's that are in use.
// C.f. pm8001_open_reject_retry()
//
extern "C" {
    pub fn pm8001_setds_completion(dev: *mut domain_device);
}
extern "C" {
    pub fn pm8001_tmf_aborted(task: *mut sas_task);
}
extern "C" {
    pub fn pm80xx_get_local_phy_id(dev: *mut domain_device) -> u32;
}
