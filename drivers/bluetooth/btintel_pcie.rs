//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/bluetooth/btintel_pcie.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Bluetooth support for Intel PCIe devices
//
// Copyright (C) 2024  Intel Corporation
//
// Control and Status Register(BTINTEL_PCIE_CSR)

// BTINTEL_PCIE_CSR Function Control Register

// Value for BTINTEL_PCIE_CSR_BOOT_STAGE register

// Registers for MSI-X

// IOSF Debug Register

pub const BTINTEL_PCIE_DBG_IDX_BIT_MASK: c_uint = 0x0F;

pub const BTINTEL_PCIE_DBG_OFFSET_BIT_MASK: c_uint = 0xFFFFFF;
// The DRAM buffer count, each buffer size, and
// fragment buffer size
//
pub const BTINTEL_PCIE_DBGC_BUFFER_COUNT: c_int = 16;

pub const BTINTEL_PCIE_DBGC_FRAG_VERSION: c_int = 1;

// Magic number(4), version(4), size of payload length(4)
pub const BTINTEL_PCIE_DBGC_FRAG_HEADER_SIZE: c_int = 12;
// Num of alloc Dbg buff (4) + (LSB(4), MSB(4), Size(4)) for each buffer
pub const BTINTEL_PCIE_DBGC_FRAG_PAYLOAD_SIZE: c_int = 196;
// Causes for the FH register interrupts
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msix_fh_int_causes {
    BTINTEL_PCIE_MSIX_FH_INT_CAUSES_0	= BIT(0),	/* cause 0 */
    BTINTEL_PCIE_MSIX_FH_INT_CAUSES_1	= BIT(1),	/* cause 1 */
}

// Causes for the HW register interrupts
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msix_hw_int_causes {
    BTINTEL_PCIE_MSIX_HW_INT_CAUSES_GP0	= BIT(0),	/* cause 32 */
    BTINTEL_PCIE_MSIX_HW_INT_CAUSES_GP1	= BIT(1),	/* cause 33 */
    BTINTEL_PCIE_MSIX_HW_INT_CAUSES_HWEXP	= BIT(3),	/* cause 35 */
    BTINTEL_PCIE_MSIX_HW_INT_CAUSES_FWTRIG	= BIT(5),	/* cause 37 */
}

// PCIe device states
// Host-Device interface is active
// Host-Device interface is inactive(as reflected by IPC_SLEEP_CONTROL_CSR_AD)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btintel_pcie_tlv_type {
    BTINTEL_CNVI_BT,
    BTINTEL_WRITE_PTR,
    BTINTEL_WRAP_CTR,
    BTINTEL_TRIGGER_REASON,
    BTINTEL_FW_SHA,
    BTINTEL_CNVR_TOP,
    BTINTEL_CNVI_TOP,
    BTINTEL_DUMP_TIME,
    BTINTEL_FW_BUILD,
    BTINTEL_VENDOR,
    BTINTEL_DRIVER,
    BTINTEL_EVENT_TYPE,
    BTINTEL_EVENT_ID
}

// causes for the MBOX interrupts
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msix_mbox_int_causes {
    BTINTEL_PCIE_CSR_MBOX_STATUS_MBOX1 = BIT(0), /* cause MBOX1 */
    BTINTEL_PCIE_CSR_MBOX_STATUS_MBOX2 = BIT(1), /* cause MBOX2 */
    BTINTEL_PCIE_CSR_MBOX_STATUS_MBOX3 = BIT(2), /* cause MBOX3 */
    BTINTEL_PCIE_CSR_MBOX_STATUS_MBOX4 = BIT(3), /* cause MBOX4 */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btintel_pcie_reset_type {
    BTINTEL_PCIE_IOSF_PRR_FLR = 0,
    BTINTEL_PCIE_IOSF_PRR_PLDR = 1,
}

// Minimum and Maximum number of MSI-X Vector
// Intel Bluetooth PCIe support only 1 vector
//
pub const BTINTEL_PCIE_MSIX_VEC_MAX: c_int = 1;
pub const BTINTEL_PCIE_MSIX_VEC_MIN: c_int = 1;
// Default poll time for MAC access during init
pub const BTINTEL_DEFAULT_MAC_ACCESS_TIMEOUT_US: c_int = 200000;
// Default interrupt timeout in msec
pub const BTINTEL_DEFAULT_INTR_TIMEOUT_MS: c_int = 3000;
pub const BTINTEL_PCIE_DX_TRANSITION_MAX_RETRIES: c_int = 3;
// The number of descriptors in TX queues
pub const BTINTEL_PCIE_TX_DESCS_COUNT: c_int = 32;
// The number of descriptors in RX queues
pub const BTINTEL_PCIE_RX_DESCS_COUNT: c_int = 64;
// Number of Queue for TX and RX
// It indicates the index of the IA(Index Array)
//
// The size of DMA buffer for TX and RX in bytes
pub const BTINTEL_PCIE_BUFFER_SIZE: c_int = 4096;
pub const BTINTEL_PCIE_TX_WAIT_TIMEOUT_MS: c_int = 500;
// Doorbell vector for TFD
pub const BTINTEL_PCIE_TX_DB_VEC: c_int = 0;
// Doorbell vector for FRBD
pub const BTINTEL_PCIE_RX_DB_VEC: c_int = 513;
// RBD buffer size mapping
pub const BTINTEL_PCIE_RBD_SIZE_4K: c_uint = 0x04;
//
// Struct for Context Information (v2)
//
// All members are write-only for host and read-only for device.
//
// @version: Version of context information
// @size: Size of context information
// @config: Config with which host wants peripheral to execute
// Subset of capability register published by device
// @addr_tr_hia: Address of TR Head Index Array
// @addr_tr_tia: Address of TR Tail Index Array
// @addr_cr_hia: Address of CR Head Index Array
// @addr_cr_tia: Address of CR Tail Index Array
// @num_tr_ia: Number of entries in TR Index Arrays
// @num_cr_ia: Number of entries in CR Index Arrays
// @rbd_siz: RBD Size { 0x4=4K }
// @addr_tfdq: Address of TFD Queue(tx)
// @addr_urbdq0: Address of URBD Queue(tx)
// @num_tfdq: Number of TFD in TFD Queue(tx)
// @num_urbdq0: Number of URBD in URBD Queue(tx)
// @tfdq_db_vec: Queue number of TFD
// @urbdq0_db_vec: Queue number of URBD
// @addr_frbdq: Address of FRBD Queue(rx)
// @addr_urbdq1: Address of URBD Queue(rx)
// @num_frbdq: Number of FRBD in FRBD Queue(rx)
// @frbdq_db_vec: Queue number of FRBD
// @num_urbdq1: Number of URBD in URBD Queue(rx)
// @urbdq_db_vec: Queue number of URBDQ1
// @tr_msi_vec: Transfer Ring MSI-X Vector
// @cr_msi_vec: Completion Ring MSI-X Vector
// @dbgc_addr: DBGC first fragment address
// @dbgc_size: DBGC buffer size
// @early_enable: Enarly debug enable
// @dbg_output_mode: Debug output mode
// Bit[4] DBGC O/P { 0=SRAM, 1=DRAM(not relevant for NPK) }
// Bit[5] DBGC I/P { 0=BDBG, 1=DBGI }
// Bits[6:7] DBGI O/P(relevant if bit[5] = 1)
// 0=BT DBGC, 1=WiFi DBGC, 2=NPK }
// @dbg_preset: Debug preset
// @ext_addr: Address of context information extension
// @ext_size: Size of context information part
//
// Total 38 DWords
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctx_info {
    pub version: u16,
    pub size: u16,
    pub config: u32,
    pub reserved_dw02: u32,
    pub reserved_dw03: u32,
    pub addr_tr_hia: u64,
    pub addr_tr_tia: u64,
    pub addr_cr_hia: u64,
    pub addr_cr_tia: u64,
    pub num_tr_ia: u16,
    pub num_cr_ia: u16,
    pub addr_tfdq: u64,
    pub addr_urbdq0: u64,
    pub num_tfdq: u16,
    pub num_urbdq0: u16,
    pub tfdq_db_vec: u16,
    pub urbdq0_db_vec: u16,
    pub addr_frbdq: u64,
    pub addr_urbdq1: u64,
    pub num_frbdq: u16,
    pub frbdq_db_vec: u16,
    pub num_urbdq1: u16,
    pub urbdq_db_vec: u16,
    pub tr_msi_vec: u16,
    pub cr_msi_vec: u16,
    pub reserved_dw27: u32,
    pub dbgc_addr: u64,
    pub dbgc_size: u32,
    pub ext_addr: u64,
    pub ext_size: u32,
    pub test_param: u32,
    pub reserved_dw36: u32,
    pub reserved_dw37: u32,
    pub __packed: },
// Transfer Descriptor for TX
// @type: Not in use. Set to 0x0
// @size: Size of data in the buffer
// @addr: DMA Address of buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tfd {
    pub type: u8,
    pub size: u16,
    pub reserved: u8,
    pub addr: u64,
    pub reserved1: u32,
    pub __packed: },
// URB Descriptor for TX
// @tfd_index: Index of TFD in TFDQ + 1
// @num_txq: Queue index of TFD Queue
// @cmpl_count: Completion count. Always 0x01
// @immediate_cmpl: Immediate completion flag: Always 0x01
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct urbd0 {
    pub __packed: },
// FRB Descriptor for RX
// @tag: RX buffer tag (index of RX buffer queue)
// @addr: Address of buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct frbd {
    pub reserved2: u32,
    pub addr: u64,
    pub __packed: },
// URB Descriptor for RX
// @frbd_tag: Tag from FRBD
// @status: Status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct urbd1 {
    pub __packed: },
// RFH header in RX packet
// @packet_len: Length of the data in the buffer
// @rxq: RX Queue number
// @cmd_id: Command ID. Not in Use
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rfh_hdr {
    pub __packed: },
// Internal data buffer
// @data: pointer to the data buffer
// @p_addr: physical address of data buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct data_buf {
    pub data: *mut u8,
    pub data_p_addr: dma_addr_t,
}

// Index Array
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ia {
    pub tr_hia_p_addr: dma_addr_t,
    pub tr_hia: *mut u16,
    pub tr_tia_p_addr: dma_addr_t,
    pub tr_tia: *mut u16,
    pub cr_hia_p_addr: dma_addr_t,
    pub cr_hia: *mut u16,
    pub cr_tia_p_addr: dma_addr_t,
    pub cr_tia: *mut u16,
}

// Structure for TX Queue
// @count: Number of descriptors
// @tfds: Array of TFD
// @urbd0s: Array of URBD0
// @buf: Array of data_buf structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct txq {
    pub count: u16,
    pub tfds_p_addr: dma_addr_t,
    pub tfds: *mut tfd,
    pub urbd0s_p_addr: dma_addr_t,
    pub urbd0s: *mut urbd0,
    pub buf_p_addr: dma_addr_t,
    pub buf_v_addr: *mut c_void,
    pub bufs: *mut data_buf,
}

// Structure for RX Queue
// @count: Number of descriptors
// @frbds: Array of FRBD
// @urbd1s: Array of URBD1
// @buf: Array of data_buf structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxq {
    pub count: u16,
    pub frbds_p_addr: dma_addr_t,
    pub frbds: *mut frbd,
    pub urbd1s_p_addr: dma_addr_t,
    pub urbd1s: *mut urbd1,
    pub buf_p_addr: dma_addr_t,
    pub buf_v_addr: *mut c_void,
    pub bufs: *mut data_buf,
}

// Structure for DRAM Buffer
// @count: Number of descriptors
// @buf: Array of data_buf structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btintel_pcie_dbgc {
    pub count: u16,
    pub frag_v_addr: *mut c_void,
    pub frag_p_addr: dma_addr_t,
    pub frag_size: u16,
    pub buf_p_addr: dma_addr_t,
    pub buf_v_addr: *mut c_void,
    pub bufs: *mut data_buf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btintel_pcie_dump_header {
    pub driver_name: *const c_char,
    pub cnvi_top: u32,
    pub cnvr_top: u32,
    pub fw_timestamp: u16,
    pub fw_build_type: u8,
    pub fw_build_num: u32,
    pub fw_git_sha1: u32,
    pub cnvi_bt: u32,
    pub write_ptr: u32,
    pub wrap_ctr: u32,
    pub trigger_reason: u16,
    pub state: c_int,
    pub event_type: u8,
    pub event_id: u16,
}

// struct btintel_pcie_data
// @pdev: pci device
// @hdev: hdev device
// @flags: driver state
// @irq_lock: spinlock for MSI-X
// @hci_rx_lock: spinlock for HCI RX flow
// @base_addr: pci base address (from BAR)
// @msix_entries: array of MSI-X entries
// @msix_enabled: true if MSI-X is enabled;
// @alloc_vecs: number of interrupt vectors allocated
// @def_irq: default irq for all causes
// @fh_init_mask: initial unmasked rxq causes
// @hw_init_mask: initial unmaksed hw causes
// @boot_stage_cache: cached value of boot stage register
// @img_resp_cache: cached value of image response register
// @cnvi: CNVi register value
// @cnvr: CNVr register value
// @gp0_received: condition for gp0 interrupt
// @gp0_wait_q: wait_q for gp0 interrupt
// @tx_wait_done: condition for tx interrupt
// @tx_wait_q: wait_q for tx interrupt
// @workqueue: workqueue for RX work
// @rx_skb_q: SKB queue for RX packet
// @rx_work: RX work struct to process the RX packet in @rx_skb_q
// @dump_workqueue: dedicated ordered workqueue serializing the coredump,
// hardware exception, and firmware-trigger dump workers
// @coredump_work: work struct for DRAM trace coredump collection
// @hwexp_work: work struct for hardware exception event read
// @fwtrigger_work: work struct for firmware-triggered diagnostic event read
// @dma_pool: DMA pool for descriptors, index array and ci
// @dma_p_addr: DMA address for pool
// @dma_v_addr: address of pool
// @ci_p_addr: DMA address for CI struct
// @ci: CI struct
// @ia: Index Array struct
// @txq: TX Queue struct
// @rxq: RX Queue struct
// @alive_intr_ctxt: Alive interrupt context
// @pm_sx_event: PM event on which system got suspended
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btintel_pcie_data {
    pub pdev: *mut pci_dev,
    pub hdev: *mut hci_dev,
    pub flags: c_ulong,
// lock used in MSI-X interrupt
    pub irq_lock: spinlock_t,
// lock to serialize rx events
    pub hci_rx_lock: spinlock_t,
    pub base_addr: *mut void __iomem,
    pub msix_entries: [msix_entry; BTINTEL_PCIE_MSIX_VEC_MAX],
    pub msix_enabled: bool,
    pub alloc_vecs: u32,
    pub def_irq: u32,
    pub fh_init_mask: u32,
    pub hw_init_mask: u32,
    pub boot_stage_cache: u32,
    pub img_resp_cache: u32,
    pub cnvi: u32,
    pub cnvr: u32,
    pub gp0_received: bool,
    pub gp0_wait_q: wait_queue_head_t,
    pub tx_wait_done: bool,
    pub tx_wait_q: wait_queue_head_t,
    pub workqueue: *mut workqueue_struct,
    pub rx_skb_q: sk_buff_head,
    pub rx_work: work_struct,
    pub reset_work: work_struct,
    pub dump_workqueue: *mut workqueue_struct,
    pub coredump_work: work_struct,
    pub hwexp_work: work_struct,
    pub fwtrigger_work: work_struct,
    pub dma_pool: *mut dma_pool,
    pub dma_p_addr: dma_addr_t,
    pub dma_v_addr: *mut c_void,
    pub ci_p_addr: dma_addr_t,
    pub ci: *mut ctx_info,
    pub ia: ia,
    pub txq: txq,
    pub rxq: rxq,
    pub alive_intr_ctxt: u32,
    pub reset_type: btintel_pcie_reset_type,
    pub dbgc: btintel_pcie_dbgc,
    pub dmp_hdr: btintel_pcie_dump_header,
    pub pm_sx_event: u8,
    pub debug_evt_addr: u32,
    pub debug_evt_size: u32,
}

extern "C" {
    pub fn ioread32(offset: data->base_addr +) -> return;
}
extern "C" {
    pub fn btintel_pcie_rd_reg32(_arg: data, _arg: BTINTEL_PCIE_PRPH_DEV_RD_REG) -> return;
}
