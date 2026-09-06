//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/be2iscsi/be_main.h
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
// Copyright 2017 Broadcom. All Rights Reserved.
// The term "Broadcom" refers to Broadcom Limited and/or its subsidiaries.
//
// Contact Information:
// linux-drivers@broadcom.com
//

pub const BE_VENDOR_ID: c_uint = 0x19A2;
pub const ELX_VENDOR_ID: c_uint = 0x10DF;
// DEVICE ID's for BE2
pub const BE_DEVICE_ID1: c_uint = 0x212;
pub const OC_DEVICE_ID1: c_uint = 0x702;
pub const OC_DEVICE_ID2: c_uint = 0x703;
// DEVICE ID's for BE3
pub const BE_DEVICE_ID2: c_uint = 0x222;
pub const OC_DEVICE_ID3: c_uint = 0x712;
// DEVICE ID for SKH
pub const OC_SKH_ID1: c_uint = 0x722;
pub const BE2_IO_DEPTH: c_int = 1024;
pub const BE2_MAX_SESSIONS: c_int = 256;
pub const BE2_TMFS: c_int = 16;
pub const BE2_NOPOUT_REQ: c_int = 16;
pub const BE2_SGE: c_int = 32;
pub const BE2_DEFPDU_HDR_SZ: c_int = 64;
pub const BE2_DEFPDU_DATA_SZ: c_int = 8192;
pub const BE2_MAX_NUM_CQ_PROC: c_int = 512;

pub const BEISCSI_MAX_NUM_CPUS: c_int = 7;
pub const BEISCSI_VER_STRLEN: c_int = 32;
pub const BEISCSI_SGLIST_ELEMENTS: c_int = 30;
//
// BE_INVLDT_CMD_TBL_SZ is 128 which is total number commands that can
// be invalidated at a time, consider it before changing the value of
// BEISCSI_CMD_PER_LUN.
//

pub const BEISCSI_MAX_FRAGS_INIT: c_int = 192;
pub const BE_SENSE_INFO_SIZE: c_int = 258;
pub const BE_ISCSI_PDU_HEADER_SIZE: c_int = 64;
pub const BE_MIN_MEM_SIZE: c_int = 16384;
pub const MAX_CMD_SZ: c_int = 65536;
pub const IIOC_SCSI_DATA: c_uint = 0x05	/* Write Operation */;
//
// hardware needs the async PDU buffers to be posted in multiples of 8
// So have atleast 8 of them by default
//

// Memory BAR register
pub const PCICFG_MEMBAR_CTRL_INT_CTRL_OFFSET: c_uint = 0xfc;
//
// Host Interrupt Enable, if set interrupts are enabled although "PCI Interrupt
// Disable" may still globally block interrupts in addition to individual
// interrupt masks; a mechanism for the device driver to block all interrupts
// atomically without having to arbitrate for the PCI Interrupt Disable bit
// with the OS.
//

// ISR0 Register offset
pub const CEV_ISR0_OFFSET: c_uint = 0xC18;
pub const CEV_ISR_SIZE: c_int = 4;
//
// Macros for reading/writing a protection domain or CSR registers
// in BladeEngine.
//
pub const DB_TXULP0_OFFSET: c_uint = 0x40;
pub const DB_RXULP0_OFFSET: c_uint = 0xA0;
// Event Q door bell

pub const DB_EQ_RING_ID_LOW_MASK: c_uint = 0x1FF	/* bits 0 - 8 */;
// Clear the interrupt for this eq

// Must be 1

// Higher Order EQ_ID bit
pub const DB_EQ_RING_ID_HIGH_MASK: c_uint = 0x1F /* bits 11 - 15 */;
pub const DB_EQ_HIGH_SET_SHIFT: c_int = 11;
pub const DB_EQ_HIGH_FEILD_SHIFT: c_int = 9;
// Number of event entries processed

// Rearm bit

// Compl Q door bell
pub const DB_CQ_OFFSET: c_uint = 0x120;
pub const DB_CQ_RING_ID_LOW_MASK: c_uint = 0x3FF	/* bits 0 - 9 */;
// Higher Order CQ_ID bit
pub const DB_CQ_RING_ID_HIGH_MASK: c_uint = 0x1F /* bits 11 - 15 */;
pub const DB_CQ_HIGH_SET_SHIFT: c_int = 11;
pub const DB_CQ_HIGH_FEILD_SHIFT: c_int = 10;
// Number of event entries processed

// Rearm bit

pub const MEM_DESCR_OFFSET: c_int = 8;
pub const BEISCSI_DEFQ_HDR: c_int = 1;
pub const BEISCSI_DEFQ_DATA: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum be_mem_enum {
    HWI_MEM_ADDN_CONTEXT,
    HWI_MEM_WRB,
    HWI_MEM_WRBH,
    HWI_MEM_SGLH,
    HWI_MEM_SGE,
    HWI_MEM_TEMPLATE_HDR_ULP0,
    HWI_MEM_ASYNC_HEADER_BUF_ULP0,	/* 6 */
    HWI_MEM_ASYNC_DATA_BUF_ULP0,
    HWI_MEM_ASYNC_HEADER_RING_ULP0,
    HWI_MEM_ASYNC_DATA_RING_ULP0,
    HWI_MEM_ASYNC_HEADER_HANDLE_ULP0,
    HWI_MEM_ASYNC_DATA_HANDLE_ULP0,	/* 11 */
    HWI_MEM_ASYNC_PDU_CONTEXT_ULP0,
    HWI_MEM_TEMPLATE_HDR_ULP1,
    HWI_MEM_ASYNC_HEADER_BUF_ULP1,	/* 14 */
    HWI_MEM_ASYNC_DATA_BUF_ULP1,
    HWI_MEM_ASYNC_HEADER_RING_ULP1,
    HWI_MEM_ASYNC_DATA_RING_ULP1,
    HWI_MEM_ASYNC_HEADER_HANDLE_ULP1,
    HWI_MEM_ASYNC_DATA_HANDLE_ULP1,	/* 19 */
    HWI_MEM_ASYNC_PDU_CONTEXT_ULP1,
    ISCSI_MEM_GLOBAL_HEADER,
    SE_MEM_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_bus_address32 {
    pub address_lo: c_uint,
    pub address_hi: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_bus_address64 {
    pub address: c_ulonglong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_bus_address {
    pub a32: be_bus_address32,
    pub a64: be_bus_address64,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mem_array {
    pub /: *mut *mut be_bus_address bus_address; / Bus address of location,
    pub /: *mut *mut *mut void virtual_address; / virtual address to the location,
    pub /: *mut *mut unsigned int size; / Size required by memory block,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_mem_descriptor {
    pub /: *mut *mut unsigned int size_in_bytes; / Size required by memory block,
    pub num_elements: c_uint,
    pub mem_array: *mut mem_array,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgl_handle {
    pub sgl_index: c_uint,
    pub type: c_uint,
    pub cid: c_uint,
    pub task: *mut iscsi_task,
    pub pfrag: *mut iscsi_sge,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hba_parameters {
    pub ios_per_ctrl: c_uint,
    pub cxns_per_ctrl: c_uint,
    pub icds_per_ctrl: c_uint,
    pub num_sge_per_io: c_uint,
    pub defpdu_hdr_sz: c_uint,
    pub defpdu_data_sz: c_uint,
    pub num_cq_entries: c_uint,
    pub num_eq_entries: c_uint,
    pub wrbs_per_cxn: c_uint,
    pub hwi_ws_sz: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwi_wrb_context {
    pub wrb_lock: spinlock_t,
    pub pwrb_handle_base: *mut wrb_handle,
    pub pwrb_handle_basestd: *mut wrb_handle,
    pub plast_wrb: *mut iscsi_wrb,
    pub alloc_index: c_ushort,
    pub free_index: c_ushort,
    pub wrb_handles_available: c_ushort,
    pub cid: c_ushort,
    pub /: *mut *mut uint8_t ulp_num; / ULP to which CID binded,
    pub doorbell_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ulp_cid_info {
    pub cid_array: *mut c_ushort,
    pub avlbl_cids: c_ushort,
    pub cid_alloc: c_ushort,
    pub cid_free: c_ushort,
}

pub const BEISCSI_ULP0: c_int = 0;
pub const BEISCSI_ULP1: c_int = 1;
pub const BEISCSI_ULP_COUNT: c_int = 2;
pub const BEISCSI_ULP0_LOADED: c_uint = 0x01;
pub const BEISCSI_ULP1_LOADED: c_uint = 0x02;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct beiscsi_hba {
    pub params: hba_parameters,
    pub phwi_ctrlr: *mut hwi_controller,
    pub mem_req: [c_uint; SE_MEM_MAX],
// PCI BAR mapped addresses
    pub /: *mut *mut *mut u8 __iomem csr_va; / CSR,
    pub /: *mut *mut *mut u8 __iomem db_va; / Door Bell,
    pub /: *mut *mut *mut u8 __iomem pci_va; / PCI Config,
// PCI representation of our HBA
    pub pcidev: *mut pci_dev,
    pub num_cpus: c_uint,
    pub nxt_cqid: c_uint,
    pub msi_name: [*mut c_char; MAX_CPUS],
    pub init_mem: *mut be_mem_descriptor,
    pub io_sgl_alloc_index: c_ushort,
    pub io_sgl_free_index: c_ushort,
    pub io_sgl_hndl_avbl: c_ushort,
    pub io_sgl_hndl_base: *mut sgl_handle,
    pub eh_sgl_alloc_index: c_ushort,
    pub eh_sgl_free_index: c_ushort,
    pub eh_sgl_hndl_avbl: c_ushort,
    pub eh_sgl_hndl_base: *mut sgl_handle,
    pub io_sgl_lock: spinlock_t,
    pub mgmt_sgl_lock: spinlock_t,
    pub async_pdu_lock: spinlock_t,
    pub hba_queue: list_head,
pub const BE_MAX_SESSION: c_int = 2048;
pub const BE_INVALID_CID: c_uint = 0xffff;
    pub cid_to_cri_map: [c_ushort; BE_MAX_SESSION],
    pub cid_array_info: [*mut ulp_cid_info; BEISCSI_ULP_COUNT],
    pub ep_array: *mut iscsi_endpoint,
    pub conn_table: *mut beiscsi_conn,
    pub shost: *mut Scsi_Host,
    pub ipv4_iface: *mut iscsi_iface,
    pub ipv6_iface: *mut iscsi_iface,
//
// group together since they are used most frequently
// for cid to cri conversion
//
pub const BEISCSI_PHYS_PORT_MAX: c_int = 4;
    pub phys_port: c_uint,
// valid values of phys_port id are 0, 1, 2, 3
    pub eqid_count: c_uint,
    pub cqid_count: c_uint,
    pub iscsi_cid_start: [c_uint; BEISCSI_ULP_COUNT],    pub iscsi_cid_count: [c_uint; BEISCSI_ULP_COUNT],
    pub iscsi_icd_count: [c_uint; BEISCSI_ULP_COUNT],
    pub iscsi_icd_start: [c_uint; BEISCSI_ULP_COUNT],
    pub iscsi_chain_start: [c_uint; BEISCSI_ULP_COUNT],
    pub iscsi_chain_count: [c_uint; BEISCSI_ULP_COUNT],
    pub iscsi_features: c_ushort,
    pub dual_ulp_aware: u16,
    pub ulp_supported: c_ulong,
    pub fw_config: },
    pub state: c_ulong,
pub const BEISCSI_HBA_ONLINE: c_int = 0;
pub const BEISCSI_HBA_LINK_UP: c_int = 1;
pub const BEISCSI_HBA_BOOT_FOUND: c_int = 2;
pub const BEISCSI_HBA_BOOT_WORK: c_int = 3;
pub const BEISCSI_HBA_UER_SUPP: c_int = 4;
pub const BEISCSI_HBA_PCI_ERR: c_int = 5;
pub const BEISCSI_HBA_FW_TIMEOUT: c_int = 6;
pub const BEISCSI_HBA_IN_UE: c_int = 7;
pub const BEISCSI_HBA_IN_TPE: c_int = 8;
// error bits

    pub optic_state: u8,
    pub eqd_update: delayed_work,
// update EQ delay timer every 1000ms
pub const BEISCSI_EQD_UPDATE_INTERVAL: c_int = 1000;
    pub hw_check: timer_list,
// check for UE every 1000ms
pub const BEISCSI_UE_DETECT_INTERVAL: c_int = 1000;
    pub ue2rp: u32,
    pub recover_port: delayed_work,
    pub sess_work: work_struct,
    pub mac_addr_set: bool,
    pub mac_address: [u8; ETH_ALEN],
    pub port_name: u8,
    pub port_speed: u8,
    pub fw_ver_str: [c_char; BEISCSI_VER_STRLEN],
    pub /: *mut *mut *mut workqueue_wq; / The actuak work queue,
    pub ctrl: be_ctrl_info,
    pub generation: c_uint,
    pub interface_handle: c_uint,
    pub aic_obj: [be_aic_obj; MAX_CPUS],
    pub attr_log_enable: c_uint,
    pub writedir): u32,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct boot_struct {
    pub retry: c_int,
    pub tag: c_uint,
    pub s_handle: c_uint,
    pub nonemb_cmd: be_dma_mem,
    pub action: },
    pub boot_sess: mgmt_session_info,
    pub boot_kset: *mut iscsi_boot_kset,
    pub boot_struct: },
    pub boot_work: work_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct beiscsi_session {
    pub bhs_pool: *mut dma_pool,
}

//
// struct beiscsi_conn - iscsi connection structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct beiscsi_conn {
    pub conn: *mut iscsi_conn,
    pub phba: *mut beiscsi_hba,
    pub exp_statsn: u32,
    pub doorbell_offset: u32,
    pub beiscsi_conn_cid: u32,
    pub ep: *mut beiscsi_endpoint,
    pub login_in_progress: c_ushort,
    pub plogin_wrb_handle: *mut wrb_handle,
    pub plogin_sgl_handle: *mut sgl_handle,
    pub beiscsi_sess: *mut beiscsi_session,
    pub task: *mut iscsi_task,
}

// This structure is used by the chip
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pdu_data_out {
    pub dw: [u32; 12],
}

//
// Pseudo amap definition in which each bit of the actual structure is defined
// as a byte: used to calculate offset/shift/mask of each field
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amap_pdu_data_out {
    pub /: *mut *mut u8 opcode[6]; / opcode,
    pub /: *mut *mut u8 rsvd0[2]; / should be 0,
    pub rsvd1: [u8; 7],
    pub /: *mut *mut u8 final_bit; / F bit,
    pub rsvd2: [u8; 16],
    pub /: *mut *mut u8 ahs_length[8]; / no AHS,
    pub data_len_hi: [u8; 8],
    pub /: *mut *mut u8 data_len_lo[16]; / DataSegmentLength,
    pub lun: [u8; 64],
    pub /: *mut *mut u8 itt[32]; / ITT; initiator task tag,
    pub /: *mut *mut u8 ttt[32]; / TTT; valid for R2T or 0xffffffff,
    pub rsvd3: [u8; 32],
    pub exp_stat_sn: [u8; 32],
    pub rsvd4: [u8; 32],
    pub data_sn: [u8; 32],
    pub buffer_offset: [u8; 32],
    pub rsvd5: [u8; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_cmd_bhs {
    pub iscsi_hdr: iscsi_scsi_req,
    pub pad1: [c_uchar; 16],
    pub iscsi_data_pdu: pdu_data_out,
    pub pdu_data_out)]: sizeof(struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct beiscsi_io_task {
    pub pwrb_handle: *mut wrb_handle,
    pub psgl_handle: *mut sgl_handle,
    pub conn: *mut beiscsi_conn,
    pub scsi_cmnd: *mut scsi_cmnd,
    pub num_sg: c_int,
    pub pwrb_context: *mut hwi_wrb_context,
    pub libiscsi_itt: itt_t,
    pub cmd_bhs: *mut be_cmd_bhs,
    pub bhs_pa: be_bus_address,
    pub bhs_len: c_ushort,
    pub mtask_addr: dma_addr_t,
    pub mtask_data_count: u32,
    pub wrb_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_nonio_bhs {
    pub iscsi_hdr: iscsi_hdr,
    pub pad1: [c_uchar; 16],
    pub iscsi_data_pdu: pdu_data_out,
    pub pdu_data_out)]: sizeof(struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_status_bhs {
    pub iscsi_hdr: iscsi_scsi_req,
    pub pad1: [c_uchar; 16],
//
// The plus 2 below is to hold the sense info length that gets
// DMA'ed by RxULP
//
    pub sense_info: [c_uchar; BE_SENSE_INFO_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_sge {
    pub dw: [u32; 4],
}

//
// Pseudo amap definition in which each bit of the actual structure is defined
// as a byte: used to calculate offset/shift/mask of each field
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amap_iscsi_sge {
    pub addr_hi: [u8; 32],
    pub addr_lo: [u8; 32],
    pub /: *mut *mut u8 sge_offset[22]; / DWORD 2,
    pub /: *mut *mut u8 rsvd0[9]; / DWORD 2,
    pub /: *mut *mut u8 last_sge; / DWORD 2,
    pub /: *mut *mut u8 len[17]; / DWORD 3,
    pub /: *mut *mut u8 rsvd1[15]; / DWORD 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct beiscsi_offload_params {
    pub dw: [u32; 6],
}

pub const OFFLD_PARAMS_ERL: c_uint = 0x00000003;
pub const OFFLD_PARAMS_DDE: c_uint = 0x00000004;
pub const OFFLD_PARAMS_HDE: c_uint = 0x00000008;
pub const OFFLD_PARAMS_IR2T: c_uint = 0x00000010;
pub const OFFLD_PARAMS_IMD: c_uint = 0x00000020;
pub const OFFLD_PARAMS_DATA_SEQ_INORDER: c_uint = 0x00000040;
pub const OFFLD_PARAMS_PDU_SEQ_INORDER: c_uint = 0x00000080;
pub const OFFLD_PARAMS_MAX_R2T: c_uint = 0x00FFFF00;
//
// Pseudo amap definition in which each bit of the actual structure is defined
// as a byte: used to calculate offset/shift/mask of each field
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amap_beiscsi_offload_params {
    pub max_burst_length: [u8; 32],
    pub max_send_data_segment_length: [u8; 32],
    pub first_burst_length: [u8; 32],
    pub erl: [u8; 2],
    pub dde: [u8; 1],
    pub hde: [u8; 1],
    pub ir2t: [u8; 1],
    pub imd: [u8; 1],
    pub data_seq_inorder: [u8; 1],
    pub pdu_seq_inorder: [u8; 1],
    pub max_r2t: [u8; 16],
    pub pad: [u8; 8],
    pub exp_statsn: [u8; 32],
    pub max_recv_data_segment_length: [u8; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hd_async_handle {
    pub link: list_head,
    pub pa: be_bus_address,
    pub pbuffer: *mut c_void,
    pub buffer_len: u32,
    pub index: u16,
    pub cri: u16,
    pub is_header: u8,
    pub is_final: u8,
    pub in_use: u8,
}

//
// This has list of async PDUs that are waiting to be processed.
// Buffers live in this list for a brief duration before they get
// processed and posted back to hardware.
// Note that we don't really need one cri_wait_queue per async_entry.
// We need one cri_wait_queue per CRI. Its easier to manage if this
// is tagged along with the async_entry.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hd_async_entry {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cri_wait_queue {
    pub hdr_len: c_ushort,
    pub bytes_received: c_uint,
    pub bytes_needed: c_uint,
    pub list: list_head,
    pub wq: },
// handles posted to FW resides here
    pub header: *mut hd_async_handle,
    pub data: *mut hd_async_handle,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hd_async_buf_context {
    pub pa_base: be_bus_address,
    pub va_base: *mut c_void,
    pub ring_base: *mut c_void,
    pub handle_base: *mut hd_async_handle,
    pub buffer_size: u32,
    pub pi: u16,
}

//
// hd_async_context is declared for each ULP supporting iSCSI function.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hd_async_context {
    pub async_header: hd_async_buf_context,
    pub async_data: hd_async_buf_context,
    pub num_entries: u16,
//
// When unsol PDU is in, it needs to be chained till all the bytes are
// received and then processing is done. hd_async_entry is created
// based on the cid_count for each ULP. When unsol PDU comes in based
// on the conn_id it needs to be added to the correct async_entry wq.
// Below defined cid_to_async_cri_map is used to reterive the
// async_cri_map for a particular connection.
//
// This array is initialized after beiscsi_create_wrb_rings returns.
//
// - this method takes more memory space, fixed to 2K
// - any support for connections greater than this the array size needs
// to be incremented
//
    pub cid_to_async_cri_map: [c_ushort; BE_MAX_SESSION],
//
// This is a variable size array. Don`t add anything after this field!!
//
    pub async_entry: *mut hd_async_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i_t_dpdu_cqe {
    pub dw: [u32; 4],
    pub __packed: },
//
// Pseudo amap definition in which each bit of the actual structure is defined
// as a byte: used to calculate offset/shift/mask of each field
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amap_i_t_dpdu_cqe {
    pub db_addr_hi: [u8; 32],
    pub db_addr_lo: [u8; 32],
    pub code: [u8; 6],
    pub cid: [u8; 10],
    pub dpl: [u8; 16],
    pub index: [u8; 16],
    pub num_cons: [u8; 10],
    pub rsvd0: [u8; 4],
    pub final: u8,
    pub valid: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amap_i_t_dpdu_cqe_v2 {
    pub /: *mut *mut u8 db_addr_hi[32]; / DWORD 0,
    pub /: *mut *mut u8 db_addr_lo[32]; / DWORD 1,
    pub /: *mut *mut u8 code[6]; / DWORD 2,
    pub 2*/: *mut *mut u8 num_cons; / DWORD,
    pub /: *mut *mut u8 rsvd0[8]; / DWORD 2,
    pub /: *mut *mut u8 dpl[17]; / DWORD 2,
    pub /: *mut *mut u8 index[16]; / DWORD 3,
    pub /: *mut *mut u8 cid[13]; / DWORD 3,
    pub /: *mut *mut u8 rsvd1; / DWORD 3,
    pub /: *mut *mut u8 final; / DWORD 3,
    pub /: *mut *mut u8 valid; / DWORD 3,
    pub __packed: },
pub const CQE_VALID_MASK: c_uint = 0x80000000;
pub const CQE_CODE_MASK: c_uint = 0x0000003F;
pub const CQE_CID_MASK: c_uint = 0x0000FFC0;
pub const EQE_VALID_MASK: c_uint = 0x00000001;
pub const EQE_MAJORCODE_MASK: c_uint = 0x0000000E;
pub const EQE_RESID_MASK: c_uint = 0xFFFF0000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_eq_entry {
    pub dw: [u32; 1],
    pub __packed: },
//
// Pseudo amap definition in which each bit of the actual structure is defined
// as a byte: used to calculate offset/shift/mask of each field
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amap_eq_entry {
    pub /: *mut *mut u8 valid; / DWORD 0,
    pub /: *mut *mut u8 major_code[3]; / DWORD 0,
    pub /: *mut *mut u8 minor_code[12]; / DWORD 0,
    pub /: *mut *mut u8 resource_id[16]; / DWORD 0,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cq_db {
    pub dw: [u32; 1],
    pub __packed: },
//
// Pseudo amap definition in which each bit of the actual structure is defined
// as a byte: used to calculate offset/shift/mask of each field
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amap_cq_db {
    pub qid: [u8; 10],
    pub event: [u8; 1],
    pub rsvd0: [u8; 5],
    pub num_popped: [u8; 13],
    pub rearm: [u8; 1],
    pub rsvd1: [u8; 2],
    pub __packed: },
    pub phba): *mut void beiscsi_process_eq(struct beiscsi_hba,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_wrb {
    pub dw: [u32; 16],
    pub __packed: },
pub const WRB_TYPE_MASK: c_uint = 0xF0000000;
pub const SKH_WRB_TYPE_OFFSET: c_int = 27;
pub const BE_WRB_TYPE_OFFSET: c_int = 28;

//
// Pseudo amap definition in which each bit of the actual structure is defined
// as a byte: used to calculate offset/shift/mask of each field
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amap_iscsi_wrb {
    pub /: *mut *mut u8 lun[14]; / DWORD 0,
    pub /: *mut *mut u8 lt; / DWORD 0,
    pub /: *mut *mut u8 invld; / DWORD 0,
    pub /: *mut *mut u8 wrb_idx[8]; / DWORD 0,
    pub /: *mut *mut u8 dsp; / DWORD 0,
    pub /: *mut *mut u8 dmsg; / DWORD 0,
    pub /: *mut *mut u8 undr_run; / DWORD 0,
    pub /: *mut *mut u8 over_run; / DWORD 0,
    pub /: *mut *mut u8 type[4]; / DWORD 0,
    pub /: *mut *mut u8 ptr2nextwrb[8]; / DWORD 1,
    pub /: *mut *mut u8 r2t_exp_dtl[24]; / DWORD 1,
    pub /: *mut *mut u8 sgl_icd_idx[12]; / DWORD 2,
    pub /: *mut *mut u8 rsvd0[20]; / DWORD 2,
    pub /: *mut *mut u8 exp_data_sn[32]; / DWORD 3,
    pub /: *mut *mut u8 iscsi_bhs_addr_hi[32]; / DWORD 4,
    pub /: *mut *mut u8 iscsi_bhs_addr_lo[32]; / DWORD 5,
    pub /: *mut *mut u8 cmdsn_itt[32]; / DWORD 6,
    pub /: *mut *mut u8 dif_ref_tag[32]; / DWORD 7,
    pub /: *mut *mut u8 sge0_addr_hi[32]; / DWORD 8,
    pub /: *mut *mut u8 sge0_addr_lo[32]; / DWORD 9,
    pub /: *mut *mut u8 sge0_offset[22]; / DWORD 10,
    pub /: *mut *mut u8 pbs; / DWORD 10,
    pub /: *mut *mut u8 dif_mode[2]; / DWORD 10,
    pub /: *mut *mut u8 rsvd1[6]; / DWORD 10,
    pub /: *mut *mut u8 sge0_last; / DWORD 10,
    pub /: *mut *mut u8 sge0_len[17]; / DWORD 11,
    pub /: *mut *mut u8 dif_meta_tag[14]; / DWORD 11,
    pub /: *mut *mut u8 sge0_in_ddr; / DWORD 11,
    pub /: *mut *mut u8 sge1_addr_hi[32]; / DWORD 12,
    pub /: *mut *mut u8 sge1_addr_lo[32]; / DWORD 13,
    pub /: *mut *mut u8 sge1_r2t_offset[22]; / DWORD 14,
    pub /: *mut *mut u8 rsvd2[9]; / DWORD 14,
    pub /: *mut *mut u8 sge1_last; / DWORD 14,
    pub /: *mut *mut u8 sge1_len[17]; / DWORD 15,
    pub /: *mut *mut u8 ref_sgl_icd_idx[12]; / DWORD 15,
    pub /: *mut *mut u8 rsvd3[2]; / DWORD 15,
    pub /: *mut *mut u8 sge1_in_ddr; / DWORD 15,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amap_iscsi_wrb_v2 {
    pub /: *mut *mut u8 r2t_exp_dtl[25]; / DWORD 0,
    pub 0*/: *mut *mut u8 rsvd0[2]; / DWORD,
    pub /: *mut *mut u8 type[5]; / DWORD 0,
    pub /: *mut *mut u8 ptr2nextwrb[8]; / DWORD 1,
    pub /: *mut *mut u8 wrb_idx[8]; / DWORD 1,
    pub /: *mut *mut u8 lun[16]; / DWORD 1,
    pub /: *mut *mut u8 sgl_idx[16]; / DWORD 2,
    pub /: *mut *mut u8 ref_sgl_icd_idx[16]; / DWORD 2,
    pub /: *mut *mut u8 exp_data_sn[32]; / DWORD 3,
    pub /: *mut *mut u8 iscsi_bhs_addr_hi[32]; / DWORD 4,
    pub /: *mut *mut u8 iscsi_bhs_addr_lo[32]; / DWORD 5,
    pub /: *mut *mut u8 cq_id[16]; / DWORD 6,
    pub /: *mut *mut u8 rsvd1[16]; / DWORD 6,
    pub /: *mut *mut u8 cmdsn_itt[32]; / DWORD 7,
    pub /: *mut *mut u8 sge0_addr_hi[32]; / DWORD 8,
    pub /: *mut *mut u8 sge0_addr_lo[32]; / DWORD 9,
    pub /: *mut *mut u8 sge0_offset[24]; / DWORD 10,
    pub /: *mut *mut u8 rsvd2[7]; / DWORD 10,
    pub /: *mut *mut u8 sge0_last; / DWORD 10,
    pub /: *mut *mut u8 sge0_len[17]; / DWORD 11,
    pub /: *mut *mut u8 rsvd3[7]; / DWORD 11,
    pub /: *mut *mut u8 diff_enbl; / DWORD 11,
    pub /: *mut *mut u8 u_run; / DWORD 11,
    pub /: *mut *mut u8 o_run; / DWORD 11,
    pub /: *mut *mut u8 invld; / DWORD 11,
    pub /: *mut *mut u8 dsp; / DWORD 11,
    pub /: *mut *mut u8 dmsg; / DWORD 11,
    pub /: *mut *mut u8 rsvd4; / DWORD 11,
    pub /: *mut *mut u8 lt; / DWORD 11,
    pub /: *mut *mut u8 sge1_addr_hi[32]; / DWORD 12,
    pub /: *mut *mut u8 sge1_addr_lo[32]; / DWORD 13,
    pub /: *mut *mut u8 sge1_r2t_offset[24]; / DWORD 14,
    pub /: *mut *mut u8 rsvd5[7]; / DWORD 14,
    pub /: *mut *mut u8 sge1_last; / DWORD 14,
    pub /: *mut *mut u8 sge1_len[17]; / DWORD 15,
    pub /: *mut *mut u8 rsvd6[15]; / DWORD 15,
    pub __packed: },
    pub pcontext): *mut hwi_wrb_context,
    pub psgl_handle): *mut *mut free_mgmt_sgl_handle(struct beiscsi_hba phba, struct sgl_handle,
    pub task): *mut iscsi_task,
    pub rearm): c_uchar,
    pub budget): *mut *mut unsigned int beiscsi_process_cq(struct be_eq_obj pbe_eq, int,
    pub phba): *mut void beiscsi_process_mcc_cq(struct beiscsi_hba,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pdu_nop_out {
    pub dw: [u32; 12],
}

//
// Pseudo amap definition in which each bit of the actual structure is defined
// as a byte: used to calculate offset/shift/mask of each field
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amap_pdu_nop_out {
    pub /: *mut *mut u8 opcode[6]; / opcode 0x00,
    pub /: *mut *mut u8 i_bit; / I Bit,
    pub /: *mut *mut u8 x_bit; / reserved; should be 0,
    pub fp_bit_filler1: [u8; 7],
    pub /: *mut *mut u8 f_bit; / always 1,
    pub reserved1: [u8; 16],
    pub /: *mut *mut u8 ahs_length[8]; / no AHS,
    pub data_len_hi: [u8; 8],
    pub /: *mut *mut u8 data_len_lo[16]; / DataSegmentLength,
    pub lun: [u8; 64],
    pub /: *mut *mut u8 itt[32]; / initiator id for ping or 0xffffffff,
    pub /: *mut *mut u8 ttt[32]; / target id for ping or 0xffffffff,
    pub cmd_sn: [u8; 32],
    pub exp_stat_sn: [u8; 32],
    pub reserved5: [u8; 128],
}

pub const PDUBASE_OPCODE_MASK: c_uint = 0x0000003F;
pub const PDUBASE_DATALENHI_MASK: c_uint = 0x0000FF00;
pub const PDUBASE_DATALENLO_MASK: c_uint = 0xFFFF0000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pdu_base {
    pub dw: [u32; 16],
    pub __packed: },
//
// Pseudo amap definition in which each bit of the actual structure is defined
// as a byte: used to calculate offset/shift/mask of each field
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amap_pdu_base {
    pub opcode: [u8; 6],
    pub /: *mut *mut u8 i_bit; / immediate bit,
    pub /: *mut *mut u8 x_bit; / reserved, always 0,
    pub /: *mut *mut u8 reserved1[24]; / opcode-specific fields,
    pub /: *mut *mut u8 ahs_length[8]; / length units is 4 byte words,
    pub data_len_hi: [u8; 8],
    pub /: *mut *mut u8 data_len_lo[16]; / DatasegmentLength,
    pub /: *mut *mut u8 lun[64]; / lun or opcode-specific fields,
    pub /: *mut *mut u8 itt[32]; / initiator task tag,
    pub reserved4: [u8; 224],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_target_context_update_wrb {
    pub dw: [u32; 16],
    pub __packed: },
//
// Pseudo amap definition in which each bit of the actual structure is defined
// as a byte: used to calculate offset/shift/mask of each field
//
pub const BE_TGT_CTX_UPDT_CMD: c_uint = 0x07;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amap_iscsi_target_context_update_wrb {
    pub /: *mut *mut u8 lun[14]; / DWORD 0,
    pub /: *mut *mut u8 lt; / DWORD 0,
    pub /: *mut *mut u8 invld; / DWORD 0,
    pub /: *mut *mut u8 wrb_idx[8]; / DWORD 0,
    pub /: *mut *mut u8 dsp; / DWORD 0,
    pub /: *mut *mut u8 dmsg; / DWORD 0,
    pub /: *mut *mut u8 undr_run; / DWORD 0,
    pub /: *mut *mut u8 over_run; / DWORD 0,
    pub /: *mut *mut u8 type[4]; / DWORD 0,
    pub /: *mut *mut u8 ptr2nextwrb[8]; / DWORD 1,
    pub /: *mut *mut u8 max_burst_length[19]; / DWORD 1,
    pub /: *mut *mut u8 rsvd0[5]; / DWORD 1,
    pub /: *mut *mut u8 rsvd1[15]; / DWORD 2,
    pub /: *mut *mut u8 max_send_data_segment_length[17]; / DWORD 2,
    pub /: *mut *mut u8 first_burst_length[14]; / DWORD 3,
    pub /: *mut *mut u8 rsvd2[2]; / DWORD 3,
    pub /: *mut *mut u8 tx_wrbindex_drv_msg[8]; / DWORD 3,
    pub /: *mut *mut u8 rsvd3[5]; / DWORD 3,
    pub /: *mut *mut u8 session_state[3]; / DWORD 3,
    pub /: *mut *mut u8 rsvd4[16]; / DWORD 4,
    pub /: *mut *mut u8 tx_jumbo; / DWORD 4,
    pub /: *mut *mut u8 hde; / DWORD 4,
    pub /: *mut *mut u8 dde; / DWORD 4,
    pub /: *mut *mut u8 erl[2]; / DWORD 4,
    pub /: *mut *mut u8 domain_id[5]; / DWORD 4,
    pub /: *mut *mut u8 mode; / DWORD 4,
    pub /: *mut *mut u8 imd; / DWORD 4,
    pub /: *mut *mut u8 ir2t; / DWORD 4,
    pub /: *mut *mut u8 notpredblq[2]; / DWORD 4,
    pub /: *mut *mut u8 compltonack; / DWORD 4,
    pub /: *mut *mut u8 stat_sn[32]; / DWORD 5,
    pub /: *mut *mut u8 pad_buffer_addr_hi[32]; / DWORD 6,
    pub /: *mut *mut u8 pad_buffer_addr_lo[32]; / DWORD 7,
    pub /: *mut *mut u8 pad_addr_hi[32]; / DWORD 8,
    pub /: *mut *mut u8 pad_addr_lo[32]; / DWORD 9,
    pub /: *mut *mut u8 rsvd5[32]; / DWORD 10,
    pub /: *mut *mut u8 rsvd6[32]; / DWORD 11,
    pub /: *mut *mut u8 rsvd7[32]; / DWORD 12,
    pub /: *mut *mut u8 rsvd8[32]; / DWORD 13,
    pub /: *mut *mut u8 rsvd9[32]; / DWORD 14,
    pub /: *mut *mut u8 rsvd10[32]; / DWORD 15,
    pub __packed: },

pub const BEISCSI_MAX_CXNS: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amap_iscsi_target_context_update_wrb_v2 {
    pub /: *mut *mut u8 max_burst_length[24]; / DWORD 0,
    pub /: *mut *mut u8 rsvd0[3]; / DWORD 0,
    pub /: *mut *mut u8 type[5]; / DWORD 0,
    pub /: *mut *mut u8 ptr2nextwrb[8]; / DWORD 1,
    pub /: *mut *mut u8 wrb_idx[8]; / DWORD 1,
    pub /: *mut *mut u8 rsvd1[16]; / DWORD 1,
    pub /: *mut *mut u8 max_send_data_segment_length[24]; / DWORD 2,
    pub /: *mut *mut u8 rsvd2[8]; / DWORD 2,
    pub /: *mut *mut u8 first_burst_length[24]; / DWORD 3,
    pub /: *mut *mut u8 rsvd3[8]; / DOWRD 3,
    pub /: *mut *mut u8 max_r2t[16]; / DWORD 4,
    pub /: *mut *mut u8 rsvd4; / DWORD 4,
    pub /: *mut *mut u8 hde; / DWORD 4,
    pub /: *mut *mut u8 dde; / DWORD 4,
    pub /: *mut *mut u8 erl[2]; / DWORD 4,
    pub /: *mut *mut u8 rsvd5[6]; / DWORD 4,
    pub /: *mut *mut u8 imd; / DWORD 4,
    pub /: *mut *mut u8 ir2t; / DWORD 4,
    pub /: *mut *mut u8 rsvd6[3]; / DWORD 4,
    pub /: *mut *mut u8 stat_sn[32]; / DWORD 5,
    pub /: *mut *mut u8 rsvd7[32]; / DWORD 6,
    pub /: *mut *mut u8 rsvd8[32]; / DWORD 7,
    pub /: *mut *mut u8 max_recv_dataseg_len[24]; / DWORD 8,
    pub /: *mut *mut u8 rsvd9[8]; / DWORD 8,
    pub /: *mut *mut u8 rsvd10[32]; / DWORD 9,
    pub /: *mut *mut u8 rsvd11[32]; / DWORD 10,
    pub /: *mut *mut u8 max_cxns[16]; / DWORD 11,
    pub 11*/: *mut *mut u8 rsvd12[11]; / DWORD,
    pub /: *mut *mut u8 invld; / DWORD 11,
    pub 11*/: *mut *mut u8 rsvd13;/ DWORD,
    pub /: *mut *mut u8 dmsg; / DWORD 11,
    pub /: *mut *mut u8 data_seq_inorder; / DWORD 11,
    pub /: *mut *mut u8 pdu_seq_inorder; / DWORD 11,
    pub /: *mut *mut u8 rsvd14[32]; /DWORD 12,
    pub /: *mut *mut u8 rsvd15[32]; / DWORD 13,
    pub /: *mut *mut u8 rsvd16[32]; / DWORD 14,
    pub /: *mut *mut u8 rsvd17[32]; / DWORD 15,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_ring {
    pub /: *mut *mut u32 pages; / queue size in pages,
    pub /: *mut *mut u32 id; / queue id assigned by beklib,
    pub /: *mut *mut u32 num; / number of elements in queue,
    pub /: *mut *mut u32 cidx; / consumer index,
    pub /: *mut *mut u32 pidx; / producer index -- not used by most rings,
    pub /: *mut *mut u32 item_size; / size in bytes of one object,
    pub /: *mut *mut u8 ulp_num; / ULP to which CID binded,
    pub register_set: u16,
    pub doorbell_format: u16,
    pub doorbell_offset: u32,
    pub This: *mut *mut *mut void va; / The virtual address of the ring.,
// should be last to allow 32 & 64 bit debugger
// extensions to work.
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwi_controller {
    pub wrb_context: *mut hwi_wrb_context,
    pub default_pdu_hdr: [be_ring; BEISCSI_ULP_COUNT],
    pub default_pdu_data: [be_ring; BEISCSI_ULP_COUNT],
    pub phwi_ctxt: *mut hwi_context_memory,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hwh_type_enum {
    HWH_TYPE_IO = 1,
    HWH_TYPE_LOGOUT = 2,
    HWH_TYPE_TMF = 3,
    HWH_TYPE_NOP = 4,
    HWH_TYPE_IO_RD = 5,
    HWH_TYPE_LOGIN = 11,
    HWH_TYPE_INVALID = 0xFFFFFFFF
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wrb_handle {
    pub wrb_index: c_ushort,
    pub pio_handle: *mut iscsi_task,
    pub pwrb: *mut iscsi_wrb,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwi_context_memory {
    pub be_eq: [be_eq_obj; MAX_CPUS],
    pub 1]: be_queue_info be_cq[MAX_CPUS -,
    pub be_wrbq: *mut be_queue_info,
//
// Create array of ULP number for below entries as DEFQ
// will be created for both ULP if iSCSI Protocol is
// loaded on both ULP.
//
    pub be_def_hdrq: [be_queue_info; BEISCSI_ULP_COUNT],
    pub be_def_dataq: [be_queue_info; BEISCSI_ULP_COUNT],
    pub pasync_ctx: [*mut hd_async_context; BEISCSI_ULP_COUNT],
}

extern "C" {
    pub fn beiscsi_start_boot_work(phba: *mut beiscsi_hba, s_handle: c_uint);
}
// Logging related definitions
pub const BEISCSI_LOG_INIT: c_uint = 0x0001	/* Initialization events */;
pub const BEISCSI_LOG_MBOX: c_uint = 0x0002	/* Mailbox Events */;
pub const BEISCSI_LOG_MISC: c_uint = 0x0004	/* Miscllaneous Events */;
pub const BEISCSI_LOG_EH: c_uint = 0x0008	/* Error Handler */;
pub const BEISCSI_LOG_IO: c_uint = 0x0010	/* IO Code Path */;
pub const BEISCSI_LOG_CONFIG: c_uint = 0x0020	/* CONFIG Code Path */;
pub const BEISCSI_LOG_ISCSI: c_uint = 0x0040	/* SCSI/iSCSI Protocol related Logs */;

