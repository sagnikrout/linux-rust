//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/gaudi2/gaudi2P.h
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
// Copyright 2020-2022 HabanaLabs, Ltd.
// All Rights Reserved.
//

pub const NUMBER_OF_PDMA_QUEUES: c_int = 2;
pub const NUMBER_OF_EDMA_QUEUES: c_int = 8;
pub const NUMBER_OF_MME_QUEUES: c_int = 4;
pub const NUMBER_OF_TPC_QUEUES: c_int = 25;
pub const NUMBER_OF_NIC_QUEUES: c_int = 24;
pub const NUMBER_OF_ROT_QUEUES: c_int = 2;
pub const NUMBER_OF_CPU_QUEUES: c_int = 1;

// Map all arcs dccm + arc schedulers acp blocks

pub const NUM_OF_USER_NIC_UMR_BLOCKS: c_int = 15;

// Within the user mapped array, decoder entries start post all the ARC related
// entries
//

pub const GAUDI2_MAX_PENDING_CS: c_int = 64;

pub const GAUDI2_HBM_NUM: c_int = 6;

pub const QMAN_STREAMS: c_int = 4;
pub const NUM_OF_MME_SBTE_PORTS: c_int = 5;
pub const NUM_OF_MME_WB_PORTS: c_int = 2;

// DRAM Memory Map
pub const CPU_FW_IMAGE_SIZE: c_uint = 0x10000000	/* 256MB */;

pub const PMMU_PAGE_TABLES_SIZE: c_uint = 0x10000000      /* 256MB */;

pub const NUMBER_OF_PCIE_DEC: c_int = 2;
pub const PCIE_DEC_SHIFT: c_int = 8;
pub const SRAM_USER_BASE_OFFSET: c_int = 0;
// cluster binning
pub const MAX_FAULTY_HBMS: c_int = 1;
pub const GAUDI2_XBAR_EDGE_FULL_MASK: c_uint = 0xF;
pub const GAUDI2_EDMA_FULL_MASK: c_uint = 0xFF;
pub const GAUDI2_DRAM_FULL_MASK: c_uint = 0x3F;
// Host virtual address space.
pub const VA_HOST_SPACE_PAGE_START: c_uint = 0xFFF0000000000000ull;
pub const VA_HOST_SPACE_PAGE_END: c_uint = 0xFFF0800000000000ull /* 140TB */;
pub const VA_HOST_SPACE_HPAGE_START: c_uint = 0xFFF0800000000000ull;
pub const VA_HOST_SPACE_HPAGE_END: c_uint = 0xFFF1000000000000ull /* 140TB */;
// 140TB

// 140TB

//
// HBM virtual address space
// Gaudi2 has 6 HBM devices, each supporting 16GB total of 96GB at most.
// No core separation is supported so we can have one chunk of virtual address
// space just above the physical ones.
// The virtual address space starts immediately after the end of the physical
// address space which is determined at run-time.
//
pub const VA_HBM_SPACE_END: c_uint = 0x1002000000000000ull;

pub const HW_CAP_CPU_Q_SHIFT: c_int = 5;

pub const HW_CAP_DMMU_SHIFT: c_int = 9;

pub const HW_CAP_EDMA_SHIFT: c_int = 27;

pub const HW_CAP_MME_SHIFT: c_int = 35;

pub const HW_CAP_ROT_SHIFT: c_int = 39;

pub const HW_CAP_HBM_SCRAMBLER_SHIFT: c_int = 41;

// Range Registers
pub const RR_TYPE_SHORT: c_int = 0;
pub const RR_TYPE_LONG: c_int = 1;
pub const RR_TYPE_SHORT_PRIV: c_int = 2;
pub const RR_TYPE_LONG_PRIV: c_int = 3;
pub const NUM_SHORT_LBW_RR: c_int = 14;
pub const NUM_LONG_LBW_RR: c_int = 4;
pub const NUM_SHORT_HBW_RR: c_int = 6;
pub const NUM_LONG_HBW_RR: c_int = 4;
// RAZWI initiator coordinates- X- 5 bits, Y- 4 bits
pub const RAZWI_INITIATOR_X_SHIFT: c_int = 0;
pub const RAZWI_INITIATOR_X_MASK: c_uint = 0x1F;
pub const RAZWI_INITIATOR_Y_SHIFT: c_int = 5;
pub const RAZWI_INITIATOR_Y_MASK: c_uint = 0xF;

// decoders have separate mask
pub const HW_CAP_DEC_SHIFT: c_int = 0;

// TPCs have separate mask
pub const HW_CAP_TPC_SHIFT: c_int = 0;

// nics have separate mask
pub const HW_CAP_NIC_SHIFT: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gaudi2_reserved_sob_id {
    GAUDI2_RESERVED_SOB_CS_COMPLETION_FIRST,
    GAUDI2_RESERVED_SOB_CS_COMPLETION_LAST =
    GAUDI2_RESERVED_SOB_CS_COMPLETION_FIRST + GAUDI2_MAX_PENDING_CS - 1,
    GAUDI2_RESERVED_SOB_KDMA_COMPLETION,
    GAUDI2_RESERVED_SOB_DEC_NRM_FIRST,
    GAUDI2_RESERVED_SOB_DEC_NRM_LAST =
    GAUDI2_RESERVED_SOB_DEC_NRM_FIRST + NUMBER_OF_DEC - 1,
    GAUDI2_RESERVED_SOB_DEC_ABNRM_FIRST,
    GAUDI2_RESERVED_SOB_DEC_ABNRM_LAST =
    GAUDI2_RESERVED_SOB_DEC_ABNRM_FIRST + NUMBER_OF_DEC - 1,
    GAUDI2_RESERVED_SOB_NUMBER
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gaudi2_reserved_mon_id {
    GAUDI2_RESERVED_MON_CS_COMPLETION_FIRST,
    GAUDI2_RESERVED_MON_CS_COMPLETION_LAST =
    GAUDI2_RESERVED_MON_CS_COMPLETION_FIRST + GAUDI2_MAX_PENDING_CS - 1,
    GAUDI2_RESERVED_MON_KDMA_COMPLETION,
    GAUDI2_RESERVED_MON_DEC_NRM_FIRST,
    GAUDI2_RESERVED_MON_DEC_NRM_LAST =
    GAUDI2_RESERVED_MON_DEC_NRM_FIRST + 3 * NUMBER_OF_DEC - 1,
    GAUDI2_RESERVED_MON_DEC_ABNRM_FIRST,
    GAUDI2_RESERVED_MON_DEC_ABNRM_LAST =
    GAUDI2_RESERVED_MON_DEC_ABNRM_FIRST + 3 * NUMBER_OF_DEC - 1,
    GAUDI2_RESERVED_MON_NUMBER
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gaudi2_reserved_cq_id {
    GAUDI2_RESERVED_CQ_CS_COMPLETION,
    GAUDI2_RESERVED_CQ_KDMA_COMPLETION,
    GAUDI2_RESERVED_CQ_NUMBER
}

//
// Gaudi2 subtitute TPCs Numbering
// At most- two faulty TPCs are allowed
// First replacement to a faulty TPC will be TPC24, second- TPC23
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum substitude_tpc {
    FAULTY_TPC_SUBTS_1_TPC_24,
    FAULTY_TPC_SUBTS_2_TPC_23,
    MAX_FAULTY_TPCS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gaudi2_dma_core_id {
    DMA_CORE_ID_PDMA0, /* Dcore 0 */
    DMA_CORE_ID_PDMA1, /* Dcore 0 */
    DMA_CORE_ID_EDMA0, /* Dcore 0 */
    DMA_CORE_ID_EDMA1, /* Dcore 0 */
    DMA_CORE_ID_EDMA2, /* Dcore 1 */
    DMA_CORE_ID_EDMA3, /* Dcore 1 */
    DMA_CORE_ID_EDMA4, /* Dcore 2 */
    DMA_CORE_ID_EDMA5, /* Dcore 2 */
    DMA_CORE_ID_EDMA6, /* Dcore 3 */
    DMA_CORE_ID_EDMA7, /* Dcore 3 */
    DMA_CORE_ID_KDMA, /* Dcore 0 */
    DMA_CORE_ID_SIZE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gaudi2_rotator_id {
    ROTATOR_ID_0,
    ROTATOR_ID_1,
    ROTATOR_ID_SIZE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gaudi2_mme_id {
    MME_ID_DCORE0,
    MME_ID_DCORE1,
    MME_ID_DCORE2,
    MME_ID_DCORE3,
    MME_ID_SIZE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gaudi2_tpc_id {
    TPC_ID_DCORE0_TPC0,
    TPC_ID_DCORE0_TPC1,
    TPC_ID_DCORE0_TPC2,
    TPC_ID_DCORE0_TPC3,
    TPC_ID_DCORE0_TPC4,
    TPC_ID_DCORE0_TPC5,
    TPC_ID_DCORE1_TPC0,
    TPC_ID_DCORE1_TPC1,
    TPC_ID_DCORE1_TPC2,
    TPC_ID_DCORE1_TPC3,
    TPC_ID_DCORE1_TPC4,
    TPC_ID_DCORE1_TPC5,
    TPC_ID_DCORE2_TPC0,
    TPC_ID_DCORE2_TPC1,
    TPC_ID_DCORE2_TPC2,
    TPC_ID_DCORE2_TPC3,
    TPC_ID_DCORE2_TPC4,
    TPC_ID_DCORE2_TPC5,
    TPC_ID_DCORE3_TPC0,
    TPC_ID_DCORE3_TPC1,
    TPC_ID_DCORE3_TPC2,
    TPC_ID_DCORE3_TPC3,
    TPC_ID_DCORE3_TPC4,
    TPC_ID_DCORE3_TPC5,
// the PCI TPC is placed last (mapped liked HW)
    TPC_ID_DCORE0_TPC6,
    TPC_ID_SIZE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gaudi2_dec_id {
    DEC_ID_DCORE0_DEC0,
    DEC_ID_DCORE0_DEC1,
    DEC_ID_DCORE1_DEC0,
    DEC_ID_DCORE1_DEC1,
    DEC_ID_DCORE2_DEC0,
    DEC_ID_DCORE2_DEC1,
    DEC_ID_DCORE3_DEC0,
    DEC_ID_DCORE3_DEC1,
    DEC_ID_PCIE_VDEC0,
    DEC_ID_PCIE_VDEC1,
    DEC_ID_SIZE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gaudi2_hbm_id {
    HBM_ID0,
    HBM_ID1,
    HBM_ID2,
    HBM_ID3,
    HBM_ID4,
    HBM_ID5,
    HBM_ID_SIZE,
}

// specific EDMA enumeration
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gaudi2_edma_id {
    EDMA_ID_DCORE0_INSTANCE0,
    EDMA_ID_DCORE0_INSTANCE1,
    EDMA_ID_DCORE1_INSTANCE0,
    EDMA_ID_DCORE1_INSTANCE1,
    EDMA_ID_DCORE2_INSTANCE0,
    EDMA_ID_DCORE2_INSTANCE1,
    EDMA_ID_DCORE3_INSTANCE0,
    EDMA_ID_DCORE3_INSTANCE1,
    EDMA_ID_SIZE,
}

// User interrupt count is aligned with HW CQ count.
// We have 64 CQ's per dcore, CQ0 in dcore 0 is reserved for legacy mode
//
pub const GAUDI2_NUM_USER_INTERRUPTS: c_int = 64;
pub const GAUDI2_NUM_RESERVED_INTERRUPTS: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gaudi2_irq_num {
    GAUDI2_IRQ_NUM_EVENT_QUEUE = GAUDI2_EVENT_QUEUE_MSIX_IDX,
    GAUDI2_IRQ_NUM_DCORE0_DEC0_NRM,
    GAUDI2_IRQ_NUM_DCORE0_DEC0_ABNRM,
    GAUDI2_IRQ_NUM_DCORE0_DEC1_NRM,
    GAUDI2_IRQ_NUM_DCORE0_DEC1_ABNRM,
    GAUDI2_IRQ_NUM_DCORE1_DEC0_NRM,
    GAUDI2_IRQ_NUM_DCORE1_DEC0_ABNRM,
    GAUDI2_IRQ_NUM_DCORE1_DEC1_NRM,
    GAUDI2_IRQ_NUM_DCORE1_DEC1_ABNRM,
    GAUDI2_IRQ_NUM_DCORE2_DEC0_NRM,
    GAUDI2_IRQ_NUM_DCORE2_DEC0_ABNRM,
    GAUDI2_IRQ_NUM_DCORE2_DEC1_NRM,
    GAUDI2_IRQ_NUM_DCORE2_DEC1_ABNRM,
    GAUDI2_IRQ_NUM_DCORE3_DEC0_NRM,
    GAUDI2_IRQ_NUM_DCORE3_DEC0_ABNRM,
    GAUDI2_IRQ_NUM_DCORE3_DEC1_NRM,
    GAUDI2_IRQ_NUM_DCORE3_DEC1_ABNRM,
    GAUDI2_IRQ_NUM_SHARED_DEC0_NRM,
    GAUDI2_IRQ_NUM_SHARED_DEC0_ABNRM,
    GAUDI2_IRQ_NUM_SHARED_DEC1_NRM,
    GAUDI2_IRQ_NUM_SHARED_DEC1_ABNRM,
    GAUDI2_IRQ_NUM_DEC_LAST = GAUDI2_IRQ_NUM_SHARED_DEC1_ABNRM,
    GAUDI2_IRQ_NUM_COMPLETION,
    GAUDI2_IRQ_NUM_NIC_PORT_FIRST,
    GAUDI2_IRQ_NUM_NIC_PORT_LAST = (GAUDI2_IRQ_NUM_NIC_PORT_FIRST + NIC_NUMBER_OF_PORTS - 1),
    GAUDI2_IRQ_NUM_TPC_ASSERT,
    GAUDI2_IRQ_NUM_EQ_ERROR,
    GAUDI2_IRQ_NUM_USER_FIRST,
    GAUDI2_IRQ_NUM_USER_LAST = (GAUDI2_IRQ_NUM_USER_FIRST + GAUDI2_NUM_USER_INTERRUPTS - 1),
    GAUDI2_IRQ_NUM_RESERVED_FIRST,
    GAUDI2_IRQ_NUM_RESERVED_LAST = (GAUDI2_MSIX_ENTRIES - GAUDI2_NUM_RESERVED_INTERRUPTS - 1),
    GAUDI2_IRQ_NUM_UNEXPECTED_ERROR = RESERVED_MSIX_UNEXPECTED_USER_ERROR_INTERRUPT,
    GAUDI2_IRQ_NUM_LAST = (GAUDI2_MSIX_ENTRIES - 1)
}

//
// struct dup_block_ctx - context to initialize unit instances across multiple
// blocks where block can be either a dcore of duplicated
// common module. this code relies on constant offsets
// of blocks and unit instances in a block.
// @instance_cfg_fn: instance specific configuration function.
// @data: private configuration data.
// @base: base address of the first instance in the first block.
// @block_off: subsequent blocks address spacing.
// @instance_off: subsequent block's instances address spacing.
// @enabled_mask: mask of enabled instances (1- enabled, 0- disabled).
// @blocks: number of blocks.
// @instances: unit instances per block.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dup_block_ctx {
    pub data): *mut *mut *mut void (instance_cfg_fn)(struct hl_device hdev, u64 base, void,
    pub data: *mut c_void,
    pub base: u64,
    pub block_off: u64,
    pub instance_off: u64,
    pub enabled_mask: u64,
    pub blocks: c_uint,
    pub instances: c_uint,
}

//
// struct gaudi2_queues_test_info - Holds the address of a the messages used for testing the
// device queues.
// @dma_addr: the address used by the HW for accessing the message.
// @kern_addr: The address used by the driver for accessing the message.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gaudi2_queues_test_info {
    pub dma_addr: dma_addr_t,
    pub kern_addr: *mut c_void,
}

//
// struct gaudi2_device - ASIC specific manage structure.
// @cpucp_info_get: get information on device from CPU-CP
// @mapped_blocks: array that holds the base address and size of all blocks
// the user can map.
// @lfsr_rand_seeds: array of MME ACC random seeds to set.
// @hw_queues_lock: protects the H/W queues from concurrent access.
// @scratchpad_kernel_address: general purpose PAGE_SIZE contiguous memory,
// this memory region should be write-only.
// currently used for HBW QMAN writes which is
// redundant.
// @scratchpad_bus_address: scratchpad bus address
// @virt_msix_db_cpu_addr: host memory page for the virtual MSI-X doorbell.
// @virt_msix_db_dma_addr: bus address of the page for the virtual MSI-X doorbell.
// @dram_bar_cur_addr: current address of DRAM PCI bar.
// @hw_cap_initialized: This field contains a bit per H/W engine. When that
// engine is initialized, that bit is set by the driver to
// signal we can use this engine in later code paths.
// Each bit is cleared upon reset of its corresponding H/W
// engine.
// @active_hw_arc: This field contains a bit per ARC of an H/W engine with
// exception of TPC and NIC engines. Once an engine arc is
// initialized, its respective bit is set. Driver can uniquely
// identify each initialized ARC and use this information in
// later code paths. Each respective bit is cleared upon reset
// of its corresponding ARC of the H/W engine.
// @dec_hw_cap_initialized: This field contains a bit per decoder H/W engine.
// When that engine is initialized, that bit is set by
// the driver to signal we can use this engine in later
// code paths.
// Each bit is cleared upon reset of its corresponding H/W
// engine.
// @tpc_hw_cap_initialized: This field contains a bit per TPC H/W engine.
// When that engine is initialized, that bit is set by
// the driver to signal we can use this engine in later
// code paths.
// Each bit is cleared upon reset of its corresponding H/W
// engine.
// @active_tpc_arc: This field contains a bit per ARC of the TPC engines.
// Once an engine arc is initialized, its respective bit is
// set. Each respective bit is cleared upon reset of its
// corresponding ARC of the TPC engine.
// @nic_hw_cap_initialized: This field contains a bit per nic H/W engine.
// @active_nic_arc: This field contains a bit per ARC of the NIC engines.
// Once an engine arc is initialized, its respective bit is
// set. Each respective bit is cleared upon reset of its
// corresponding ARC of the NIC engine.
// @hw_events: array that holds all H/W events that are defined valid.
// @events_stat: array that holds histogram of all received events.
// @events_stat_aggregate: same as events_stat but doesn't get cleared on reset.
// @num_of_valid_hw_events: used to hold the number of valid H/W events.
// @nic_ports: array that holds all NIC ports manage structures.
// @nic_macros: array that holds all NIC macro manage structures.
// @core_info: core info to be used by the Ethernet driver.
// @aux_ops: functions for core <-> aux drivers communication.
// @flush_db_fifo: flag to force flush DB FIFO after a write.
// @hbm_cfg: HBM subsystem settings
// @hw_queues_lock_mutex: used by simulator instead of hw_queues_lock.
// @queues_test_info: information used by the driver when testing the HW queues.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gaudi2_device {
    pub hdev): *mut *mut int (cpucp_info_get)(struct hl_device,
    pub mapped_blocks: [user_mapped_block; NUM_USER_MAPPED_BLOCKS],
    pub lfsr_rand_seeds: [c_int; MME_NUM_OF_LFSR_SEEDS],
    pub hw_queues_lock: spinlock_t,
    pub scratchpad_kernel_address: *mut c_void,
    pub scratchpad_bus_address: dma_addr_t,
    pub virt_msix_db_cpu_addr: *mut c_void,
    pub virt_msix_db_dma_addr: dma_addr_t,
    pub dram_bar_cur_addr: u64,
    pub hw_cap_initialized: u64,
    pub active_hw_arc: u64,
    pub dec_hw_cap_initialized: u64,
    pub tpc_hw_cap_initialized: u64,
    pub active_tpc_arc: u64,
    pub nic_hw_cap_initialized: u64,
    pub active_nic_arc: u64,
    pub hw_events: [u32; GAUDI2_EVENT_SIZE],
    pub events_stat: [u32; GAUDI2_EVENT_SIZE],
    pub events_stat_aggregate: [u32; GAUDI2_EVENT_SIZE],
    pub num_of_valid_hw_events: u32,
// Queue testing
    pub queues_test_info: [gaudi2_queues_test_info; GAUDI2_NUM_TESTED_QS],
}

//
// Types of the Gaudi2 IP blocks, used by special blocks iterator.
// Required for scenarios where only particular block types can be
// addressed (e.g., special PLDM images).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gaudi2_block_types {
    GAUDI2_BLOCK_TYPE_PLL,
    GAUDI2_BLOCK_TYPE_RTR,
    GAUDI2_BLOCK_TYPE_CPU,
    GAUDI2_BLOCK_TYPE_HIF,
    GAUDI2_BLOCK_TYPE_HBM,
    GAUDI2_BLOCK_TYPE_NIC,
    GAUDI2_BLOCK_TYPE_PCIE,
    GAUDI2_BLOCK_TYPE_PCIE_PMA,
    GAUDI2_BLOCK_TYPE_PDMA,
    GAUDI2_BLOCK_TYPE_EDMA,
    GAUDI2_BLOCK_TYPE_PMMU,
    GAUDI2_BLOCK_TYPE_PSOC,
    GAUDI2_BLOCK_TYPE_ROT,
    GAUDI2_BLOCK_TYPE_ARC_FARM,
    GAUDI2_BLOCK_TYPE_DEC,
    GAUDI2_BLOCK_TYPE_MME,
    GAUDI2_BLOCK_TYPE_EU_BIST,
    GAUDI2_BLOCK_TYPE_SYNC_MNGR,
    GAUDI2_BLOCK_TYPE_STLB,
    GAUDI2_BLOCK_TYPE_TPC,
    GAUDI2_BLOCK_TYPE_HMMU,
    GAUDI2_BLOCK_TYPE_SRAM,
    GAUDI2_BLOCK_TYPE_XBAR,
    GAUDI2_BLOCK_TYPE_KDMA,
    GAUDI2_BLOCK_TYPE_XDMA,
    GAUDI2_BLOCK_TYPE_XFT,
    GAUDI2_BLOCK_TYPE_MAX
}

extern "C" {
    pub fn gaudi2_iterate_tpcs(hdev: *mut hl_device, ctx: *mut iterate_module_ctx);
}
extern "C" {
    pub fn gaudi2_coresight_init(hdev: *mut hl_device) -> c_int;
}
extern "C" {
    pub fn gaudi2_debug_coresight(hdev: *mut hl_device, ctx: *mut hl_ctx, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn gaudi2_halt_coresight(hdev: *mut hl_device, ctx: *mut hl_ctx);
}
extern "C" {
    pub fn gaudi2_init_blocks(hdev: *mut hl_device, cfg_ctx: *mut dup_block_ctx);
}
extern "C" {
    pub fn gaudi2_is_hmmu_enabled(hdev: *mut hl_device, dcore_id: c_int, hmmu_id: c_int) -> bool;
}
extern "C" {
    pub fn gaudi2_init_security(hdev: *mut hl_device) -> c_int;
}
extern "C" {
    pub fn gaudi2_ack_protection_bits_errors(hdev: *mut hl_device);
}
extern "C" {
    pub fn gaudi2_send_device_activity(hdev: *mut hl_device, open: bool) -> c_int;
}
