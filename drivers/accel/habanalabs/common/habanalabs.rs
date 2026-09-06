//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/common/habanalabs.h
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
// Copyright 2016-2023 HabanaLabs, Ltd.
// All Rights Reserved.
//

pub const PCI_VENDOR_ID_HABANALABS: c_uint = 0x1da3;
// Use upper bits of mmap offset to store habana driver specific information.
// bits[63:59] - Encode mmap type
// bits[45:0]  - mmap offset value
//
// NOTE: struct vm_area_struct.vm_pgoff uses offset in pages. Hence, these
// defines are w.r.t to PAGE_SIZE
//

pub const HL_PENDING_RESET_PER_SEC: c_int = 10;

pub const HL_PENDING_RESET_LONG_SEC: c_int = 60;
//
// In device fini, wait 10 minutes for user processes to be terminated after we kill them.
// This is needed to prevent situation of clearing resources while user processes are still alive.
//
pub const HL_WAIT_PROCESS_KILL_ON_DEVICE_FINI: c_int = 600;
pub const HL_HARD_RESET_MAX_TIMEOUT: c_int = 120;

pub const HL_COMMON_USER_CQ_INTERRUPT_ID: c_uint = 0xFFF;
pub const HL_COMMON_DEC_INTERRUPT_ID: c_uint = 0xFFE;
pub const HL_STATE_DUMP_HIST_LEN: c_int = 5;
pub const HL_DBGFS_CFG_ACCESS_HIST_LEN: c_int = 20;

// Default value for device reset trigger , an invalid value
pub const HL_RESET_TRIGGER_DEFAULT: c_uint = 0xFF;

// Memory

// MMU

pub const TIMESTAMP_FREE_NODES_NUM: c_int = 512;
//
// enum hl_mmu_page_table_location - mmu page table location
// @MMU_DR_PGT: page-table is located on device DRAM.
// @MMU_HR_PGT: page-table is located on host memory.
// @MMU_NUM_PGT_LOCATIONS: number of page-table locations currently supported.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hl_mmu_page_table_location {
    MMU_DR_PGT = 0,		/* device-dram-resident MMU PGT */
    MMU_HR_PGT,		/* host resident MMU PGT */
    MMU_NUM_PGT_LOCATIONS	/* num of PGT locations */
}

//
// HL_RSVD_SOBS 'sync stream' reserved sync objects per QMAN stream
// HL_RSVD_MONS 'sync stream' reserved monitors per QMAN stream
//
pub const HL_RSVD_SOBS: c_int = 2;
pub const HL_RSVD_MONS: c_int = 1;
//
// HL_COLLECTIVE_RSVD_MSTR_MONS 'collective' reserved monitors per QMAN stream
//
pub const HL_COLLECTIVE_RSVD_MSTR_MONS: c_int = 2;

pub const HL_PCI_NUM_BARS: c_int = 6;
// Completion queue entry relates to completed job
pub const HL_COMPLETION_MODE_JOB: c_int = 0;
// Completion queue entry relates to completed command submission
pub const HL_COMPLETION_MODE_CS: c_int = 1;
pub const HL_MAX_DCORES: c_int = 8;
// DMA alloc/free wrappers

//
// Reset Flags
//
// - HL_DRV_RESET_HARD
// If set do hard reset to all engines. If not set reset just
// compute/DMA engines.
//
// - HL_DRV_RESET_FROM_RESET_THR
// Set if the caller is the hard-reset thread
//
// - HL_DRV_RESET_HEARTBEAT
// Set if reset is due to heartbeat
//
// - HL_DRV_RESET_TDR
// Set if reset is due to TDR
//
// - HL_DRV_RESET_DEV_RELEASE
// Set if reset is due to device release
//
// - HL_DRV_RESET_BYPASS_REQ_TO_FW
// F/W will perform the reset. No need to ask it to reset the device. This is relevant
// only when running with secured f/w
//
// - HL_DRV_RESET_FW_FATAL_ERR
// Set if reset is due to a fatal error from FW
//
// - HL_DRV_RESET_DELAY
// Set if a delay should be added before the reset
//
// - HL_DRV_RESET_FROM_WD_THR
// Set if the caller is the device release watchdog thread
//

//
// Security
//
pub const HL_PB_SHARED: c_int = 1;
pub const HL_PB_NA: c_int = 0;
pub const HL_PB_SINGLE_INSTANCE: c_int = 1;
pub const HL_BLOCK_SIZE: c_uint = 0x1000;
pub const HL_BLOCK_GLBL_ERR_MASK: c_uint = 0xF40;
pub const HL_BLOCK_GLBL_ERR_ADDR: c_uint = 0xF44;
pub const HL_BLOCK_GLBL_ERR_CAUSE: c_uint = 0xF48;
pub const HL_BLOCK_GLBL_SEC_OFFS: c_uint = 0xF80;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hl_protection_levels {
    SECURED_LVL,
    PRIVILEGED_LVL,
    NON_SECURED_LVL
}

//
// struct iterate_module_ctx - HW module iterator
// @fn: function to apply to each HW module instance
// @data: optional internal data to the function iterator
// @rc: return code for optional use of iterator/iterator-caller
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iterate_module_ctx {
//
// callback for the HW module iterator
// @hdev: pointer to the habanalabs device structure
// @block: block (ASIC specific definition can be dcore/hdcore)
// @inst: HW module instance within the block
// @offset: current HW module instance offset from the 1-st HW module instance
// in the 1-st block
// @ctx: the iterator context.
//
    pub ctx): *mut iterate_module_ctx,
    pub data: *mut c_void,
    pub rc: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_block_glbl_sec {
    pub sec_array: [u32; HL_BLOCK_GLBL_SEC_LEN],
}

pub const HL_MAX_SOBS_PER_MONITOR: c_int = 8;
//
// struct hl_gen_wait_properties - properties for generating a wait CB
// @data: command buffer
// @q_idx: queue id is used to extract fence register address
// @size: offset in command buffer
// @sob_base: SOB base to use in this wait CB
// @sob_val: SOB value to wait for
// @mon_id: monitor to use in this wait CB
// @sob_mask: each bit represents a SOB offset from sob_base to be used
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_gen_wait_properties {
    pub data: *mut c_void,
    pub q_idx: u32,
    pub size: u32,
    pub sob_base: u16,
    pub sob_val: u16,
    pub mon_id: u16,
    pub sob_mask: u8,
}

//
// struct pgt_info - MMU hop page info.
// @node: hash linked-list node for the pgts on host (shadow pgts for device resident MMU and
// actual pgts for host resident MMU).
// @phys_addr: physical address of the pgt.
// @virt_addr: host virtual address of the pgt (see above device/host resident).
// @shadow_addr: shadow hop in the host for device resident MMU.
// @ctx: pointer to the owner ctx.
// @num_of_ptes: indicates how many ptes are used in the pgt. used only for dynamically
// allocated HOPs (all HOPs but HOP0)
//
// The MMU page tables hierarchy can be placed either on the device's DRAM (in which case shadow
// pgts will be stored on host memory) or on host memory (in which case no shadow is required).
//
// When a new level (hop) is needed during mapping this structure will be used to describe
// the newly allocated hop as well as to track number of PTEs in it.
// During unmapping, if no valid PTEs remained in the page of a newly allocated hop, it is
// freed with its pgt_info structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pgt_info {
    pub node: hlist_node,
    pub phys_addr: u64,
    pub virt_addr: u64,
    pub shadow_addr: u64,
    pub ctx: *mut hl_ctx,
    pub num_of_ptes: c_int,
}

//
// enum hl_pci_match_mode - pci match mode per region
// @PCI_ADDRESS_MATCH_MODE: address match mode
// @PCI_BAR_MATCH_MODE: bar match mode
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hl_pci_match_mode {
    PCI_ADDRESS_MATCH_MODE,
    PCI_BAR_MATCH_MODE
}

//
// enum hl_fw_component - F/W components to read version through registers.
// @FW_COMP_BOOT_FIT: boot fit.
// @FW_COMP_PREBOOT: preboot.
// @FW_COMP_LINUX: linux.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hl_fw_component {
    FW_COMP_BOOT_FIT,
    FW_COMP_PREBOOT,
    FW_COMP_LINUX,
}

//
// enum hl_fw_types - F/W types present in the system
// @FW_TYPE_NONE: no FW component indication
// @FW_TYPE_LINUX: Linux image for device CPU
// @FW_TYPE_BOOT_CPU: Boot image for device CPU
// @FW_TYPE_PREBOOT_CPU: Indicates pre-loaded CPUs are present in the system
// (preboot, ppboot etc...)
// @FW_TYPE_ALL_TYPES: Mask for all types
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hl_fw_types {
    FW_TYPE_NONE = 0x0,
    FW_TYPE_LINUX = 0x1,
    FW_TYPE_BOOT_CPU = 0x2,
    FW_TYPE_PREBOOT_CPU = 0x4,
    FW_TYPE_ALL_TYPES =
    (FW_TYPE_LINUX | FW_TYPE_BOOT_CPU | FW_TYPE_PREBOOT_CPU)
}

//
// enum hl_queue_type - Supported QUEUE types.
// @QUEUE_TYPE_NA: queue is not available.
// @QUEUE_TYPE_EXT: external queue which is a DMA channel that may access the
// host.
// @QUEUE_TYPE_INT: internal queue that performs DMA inside the device's
// memories and/or operates the compute engines.
// @QUEUE_TYPE_CPU: S/W queue for communication with the device's CPU.
// @QUEUE_TYPE_HW: queue of DMA and compute engines jobs, for which completion
// notifications are sent by H/W.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hl_queue_type {
    QUEUE_TYPE_NA,
    QUEUE_TYPE_EXT,
    QUEUE_TYPE_INT,
    QUEUE_TYPE_CPU,
    QUEUE_TYPE_HW
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hl_cs_type {
    CS_TYPE_DEFAULT,
    CS_TYPE_SIGNAL,
    CS_TYPE_WAIT,
    CS_TYPE_COLLECTIVE_WAIT,
    CS_RESERVE_SIGNALS,
    CS_UNRESERVE_SIGNALS,
    CS_TYPE_ENGINE_CORE,
    CS_TYPE_ENGINES,
    CS_TYPE_FLUSH_PCI_HBW_WRITES,
}

//
// struct hl_inbound_pci_region - inbound region descriptor
// @mode: pci match mode for this region
// @addr: region target address
// @size: region size in bytes
// @offset_in_bar: offset within bar (address match mode)
// @bar: bar id
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_inbound_pci_region {
    pub mode: hl_pci_match_mode,
    pub addr: u64,
    pub size: u64,
    pub offset_in_bar: u64,
    pub bar: u8,
}

//
// struct hl_outbound_pci_region - outbound region descriptor
// @addr: region target address
// @size: region size in bytes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_outbound_pci_region {
    pub addr: u64,
    pub size: u64,
}

//
// enum queue_cb_alloc_flags - Indicates queue support for CBs that
// allocated by Kernel or by User
// @CB_ALLOC_KERNEL: support only CBs that allocated by Kernel
// @CB_ALLOC_USER: support only CBs that allocated by User
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum queue_cb_alloc_flags {
    CB_ALLOC_KERNEL = 0x1,
    CB_ALLOC_USER   = 0x2
}

//
// struct hl_hw_sob - H/W SOB info.
// @hdev: habanalabs device structure.
// @kref: refcount of this SOB. The SOB will reset once the refcount is zero.
// @sob_id: id of this SOB.
// @sob_addr: the sob offset from the base address.
// @q_idx: the H/W queue that uses this SOB.
// @need_reset: reset indication set when switching to the other sob.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_hw_sob {
    pub hdev: *mut hl_device,
    pub kref: kref,
    pub sob_id: u32,
    pub sob_addr: u32,
    pub q_idx: u32,
    pub need_reset: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hl_collective_mode {
    HL_COLLECTIVE_NOT_SUPPORTED = 0x0,
    HL_COLLECTIVE_MASTER = 0x1,
    HL_COLLECTIVE_SLAVE = 0x2
}

//
// struct hw_queue_properties - queue information.
// @type: queue type.
// @cb_alloc_flags: bitmap which indicates if the hw queue supports CB
// that allocated by the Kernel driver and therefore,
// a CB handle can be provided for jobs on this queue.
// Otherwise, a CB address must be provided.
// @collective_mode: collective mode of current queue
// @q_dram_bd_address: PQ dram address, used when PQ need to reside in DRAM.
// @driver_only: true if only the driver is allowed to send a job to this queue,
// false otherwise.
// @binned: True if the queue is binned out and should not be used
// @supports_sync_stream: True if queue supports sync stream
// @dram_bd: True if the bd should be copied to dram, needed for PQ which has been allocated on dram
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_queue_properties {
    pub type: hl_queue_type,
    pub cb_alloc_flags: queue_cb_alloc_flags,
    pub collective_mode: hl_collective_mode,
    pub q_dram_bd_address: u64,
    pub driver_only: u8,
    pub binned: u8,
    pub supports_sync_stream: u8,
    pub dram_bd: u8,
}

//
// enum vm_type - virtual memory mapping request information.
// @VM_TYPE_USERPTR: mapping of user memory to device virtual address.
// @VM_TYPE_PHYS_PACK: mapping of DRAM memory to device virtual address.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vm_type {
    VM_TYPE_USERPTR = 0x1,
    VM_TYPE_PHYS_PACK = 0x2
}

//
// enum mmu_op_flags - mmu operation relevant information.
// @MMU_OP_USERPTR: operation on user memory (host resident).
// @MMU_OP_PHYS_PACK: operation on DRAM (device resident).
// @MMU_OP_CLEAR_MEMCACHE: operation has to clear memcache.
// @MMU_OP_SKIP_LOW_CACHE_INV: operation is allowed to skip parts of cache invalidation.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mmu_op_flags {
    MMU_OP_USERPTR = 0x1,
    MMU_OP_PHYS_PACK = 0x2,
    MMU_OP_CLEAR_MEMCACHE = 0x4,
    MMU_OP_SKIP_LOW_CACHE_INV = 0x8,
}

//
// enum hl_device_hw_state - H/W device state. use this to understand whether
// to do reset before hw_init or not
// @HL_DEVICE_HW_STATE_CLEAN: H/W state is clean. i.e. after hard reset
// @HL_DEVICE_HW_STATE_DIRTY: H/W state is dirty. i.e. we started to execute
// hw_init
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hl_device_hw_state {
    HL_DEVICE_HW_STATE_CLEAN = 0,
    HL_DEVICE_HW_STATE_DIRTY
}

pub const HL_MMU_VA_ALIGNMENT_NOT_NEEDED: c_int = 0;
//
// struct hl_mmu_properties - ASIC specific MMU address translation properties.
// @start_addr: virtual start address of the memory region.
// @end_addr: virtual end address of the memory region.
// @hop_shifts: array holds HOPs shifts.
// @hop_masks: array holds HOPs masks.
// @last_mask: mask to get the bit indicating this is the last hop.
// @pgt_size: size for page tables.
// @supported_pages_mask: bitmask for supported page size (relevant only for MMUs
// supporting multiple page size).
// @page_size: default page size used to allocate memory.
// @num_hops: The amount of hops supported by the translation table.
// @hop_table_size: HOP table size.
// @hop0_tables_total_size: total size for all HOP0 tables.
// @host_resident: Should the MMU page table reside in host memory or in the
// device DRAM.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_mmu_properties {
    pub start_addr: u64,
    pub end_addr: u64,
    pub hop_shifts: [u64; MMU_HOP_MAX],
    pub hop_masks: [u64; MMU_HOP_MAX],
    pub last_mask: u64,
    pub pgt_size: u64,
    pub supported_pages_mask: u64,
    pub page_size: u32,
    pub num_hops: u32,
    pub hop_table_size: u32,
    pub hop0_tables_total_size: u32,
    pub host_resident: u8,
}

//
// struct hl_hints_range - hint addresses reserved va range.
// @start_addr: start address of the va range.
// @end_addr: end address of the va range.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_hints_range {
    pub start_addr: u64,
    pub end_addr: u64,
}

//
// struct asic_fixed_properties - ASIC specific immutable properties.
// @hw_queues_props: H/W queues properties.
// @special_blocks: points to an array containing special blocks info.
// @skip_special_blocks_cfg: special blocks skip configs.
// @cpucp_info: received various information from CPU-CP regarding the H/W, e.g.
// available sensors.
// @uboot_ver: F/W U-boot version.
// @preboot_ver: F/W Preboot version.
// @dmmu: DRAM MMU address translation properties.
// @pmmu: PCI (host) MMU address translation properties.
// @pmmu_huge: PCI (host) MMU address translation properties for memory
// allocated with huge pages.
// @hints_dram_reserved_va_range: dram hint addresses reserved range.
// @hints_host_reserved_va_range: host hint addresses reserved range.
// @hints_host_hpage_reserved_va_range: host huge page hint addresses reserved range.
// @sram_base_address: SRAM physical start address.
// @sram_end_address: SRAM physical end address.
// @sram_user_base_address - SRAM physical start address for user access.
// @dram_base_address: DRAM physical start address.
// @dram_end_address: DRAM physical end address.
// @dram_user_base_address: DRAM physical start address for user access.
// @dram_size: DRAM total size.
// @dram_pci_bar_size: size of PCI bar towards DRAM.
// @max_power_default: max power of the device after reset.
// @dc_power_default: power consumed by the device in mode idle.
// @dram_size_for_default_page_mapping: DRAM size needed to map to avoid page
// fault.
// @pcie_dbi_base_address: Base address of the PCIE_DBI block.
// @pcie_aux_dbi_reg_addr: Address of the PCIE_AUX DBI register.
// @mmu_pgt_addr: base physical address in DRAM of MMU page tables.
// @mmu_dram_default_page_addr: DRAM default page physical address.
// @tpc_enabled_mask: which TPCs are enabled.
// @tpc_binning_mask: which TPCs are binned. 0 means usable and 1 means binned.
// @dram_enabled_mask: which DRAMs are enabled.
// @dram_binning_mask: which DRAMs are binned. 0 means usable, 1 means binned.
// @dram_hints_align_mask: dram va hint addresses alignment mask which is used
// for hints validity check.
// @cfg_base_address: config space base address.
// @mmu_cache_mng_addr: address of the MMU cache.
// @mmu_cache_mng_size: size of the MMU cache.
// @device_dma_offset_for_host_access: the offset to add to host DMA addresses
// to enable the device to access them.
// @host_base_address: host physical start address for host DMA from device
// @host_end_address: host physical end address for host DMA from device
// @max_freq_value: current max clk frequency.
// @engine_core_interrupt_reg_addr: interrupt register address for engine core to use
// in order to raise events toward FW.
// @clk_pll_index: clock PLL index that specify which PLL determines the clock
// we display to the user
// @mmu_pgt_size: MMU page tables total size.
// @mmu_pte_size: PTE size in MMU page tables.
// @dram_page_size: The DRAM physical page size.
// @cfg_size: configuration space size on SRAM.
// @sram_size: total size of SRAM.
// @max_asid: maximum number of open contexts (ASIDs).
// @num_of_events: number of possible internal H/W IRQs.
// @psoc_pci_pll_nr: PCI PLL NR value.
// @psoc_pci_pll_nf: PCI PLL NF value.
// @psoc_pci_pll_od: PCI PLL OD value.
// @psoc_pci_pll_div_factor: PCI PLL DIV FACTOR 1 value.
// @psoc_timestamp_frequency: frequency of the psoc timestamp clock.
// @high_pll: high PLL frequency used by the device.
// @cb_pool_cb_cnt: number of CBs in the CB pool.
// @cb_pool_cb_size: size of each CB in the CB pool.
// @decoder_enabled_mask: which decoders are enabled.
// @decoder_binning_mask: which decoders are binned, 0 means usable and 1 means binned.
// @rotator_enabled_mask: which rotators are enabled.
// @edma_enabled_mask: which EDMAs are enabled.
// @edma_binning_mask: which EDMAs are binned, 0 means usable and 1 means
// binned (at most one binned DMA).
// @max_pending_cs: maximum of concurrent pending command submissions
// @max_queues: maximum amount of queues in the system
// @fw_preboot_cpu_boot_dev_sts0: bitmap representation of preboot cpu
// capabilities reported by FW, bit description
// can be found in CPU_BOOT_DEV_STS0
// @fw_preboot_cpu_boot_dev_sts1: bitmap representation of preboot cpu
// capabilities reported by FW, bit description
// can be found in CPU_BOOT_DEV_STS1
// @fw_bootfit_cpu_boot_dev_sts0: bitmap representation of boot cpu security
// status reported by FW, bit description can be
// found in CPU_BOOT_DEV_STS0
// @fw_bootfit_cpu_boot_dev_sts1: bitmap representation of boot cpu security
// status reported by FW, bit description can be
// found in CPU_BOOT_DEV_STS1
// @fw_app_cpu_boot_dev_sts0: bitmap representation of application security
// status reported by FW, bit description can be
// found in CPU_BOOT_DEV_STS0
// @fw_app_cpu_boot_dev_sts1: bitmap representation of application security
// status reported by FW, bit description can be
// found in CPU_BOOT_DEV_STS1
// @max_dec: maximum number of decoders
// @hmmu_hif_enabled_mask: mask of HMMUs/HIFs that are not isolated (enabled)
// 1- enabled, 0- isolated.
// @faulty_dram_cluster_map: mask of faulty DRAM cluster.
// 1- faulty cluster, 0- good cluster.
// @xbar_edge_enabled_mask: mask of XBAR_EDGEs that are not isolated (enabled)
// 1- enabled, 0- isolated.
// @device_mem_alloc_default_page_size: may be different than dram_page_size only for ASICs for
// which the property supports_user_set_page_size is true
// (i.e. the DRAM supports multiple page sizes), otherwise
// it will shall  be equal to dram_page_size.
// @num_engine_cores: number of engine cpu cores.
// @max_num_of_engines: maximum number of all engines in the ASIC.
// @num_of_special_blocks: special_blocks array size.
// @glbl_err_max_cause_num: global err max cause number.
// @hbw_flush_reg: register to read to generate HBW flush. value of 0 means HBW flush is
// not supported.
// @reserved_fw_mem_size: size of dram memory reserved for FW.
// @fw_event_queue_size: queue size for events from CPU-CP.
// A value of 0 means using the default HL_EQ_SIZE_IN_BYTES value.
// @collective_first_sob: first sync object available for collective use
// @collective_first_mon: first monitor available for collective use
// @sync_stream_first_sob: first sync object available for sync stream use
// @sync_stream_first_mon: first monitor available for sync stream use
// @first_available_user_sob: first sob available for the user
// @first_available_user_mon: first monitor available for the user
// @first_available_user_interrupt: first available interrupt reserved for the user
// @first_available_cq: first available CQ for the user.
// @user_interrupt_count: number of user interrupts.
// @user_dec_intr_count: number of decoder interrupts exposed to user.
// @tpc_interrupt_id: interrupt id for TPC to use in order to raise events towards the host.
// @eq_interrupt_id: interrupt id for EQ, uses to synchronize EQ interrupts in hard-reset.
// @cache_line_size: device cache line size.
// @server_type: Server type that the ASIC is currently installed in.
// The value is according to enum hl_server_type in uapi file.
// @completion_queues_count: number of completion queues.
// @completion_mode: 0 - job based completion, 1 - cs based completion
// @mme_master_slave_mode: 0 - Each MME works independently, 1 - MME works
// in Master/Slave mode
// @fw_security_enabled: true if security measures are enabled in firmware,
// false otherwise
// @fw_cpu_boot_dev_sts0_valid: status bits are valid and can be fetched from
// BOOT_DEV_STS0
// @fw_cpu_boot_dev_sts1_valid: status bits are valid and can be fetched from
// BOOT_DEV_STS1
// @dram_supports_virtual_memory: is there an MMU towards the DRAM
// @hard_reset_done_by_fw: true if firmware is handling hard reset flow
// @num_functional_hbms: number of functional HBMs in each DCORE.
// @hints_range_reservation: device support hint addresses range reservation.
// @iatu_done_by_fw: true if iATU configuration is being done by FW.
// @dynamic_fw_load: is dynamic FW load is supported.
// @gic_interrupts_enable: true if FW is not blocking GIC controller,
// false otherwise.
// @use_get_power_for_reset_history: To support backward compatibility for Goya
// and Gaudi
// @supports_compute_reset: is a reset which is not a hard-reset supported by this asic.
// @allow_inference_soft_reset: true if the ASIC supports soft reset that is
// initiated by user or TDR. This is only true
// in inference ASICs, as there is no real-world
// use-case of doing soft-reset in training (due
// to the fact that training runs on multiple
// devices)
// @configurable_stop_on_err: is stop-on-error option configurable via debugfs.
// @set_max_power_on_device_init: true if need to set max power in F/W on device init.
// @supports_user_set_page_size: true if user can set the allocation page size.
// @dma_mask: the dma mask to be set for this device.
// @supports_advanced_cpucp_rc: true if new cpucp opcodes are supported.
// @supports_engine_modes: true if changing engines/engine_cores modes is supported.
// @support_dynamic_resereved_fw_size: true if we support dynamic reserved size for fw.
// @supports_nvme: indicates whether the asic supports NVMe P2P DMA.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct asic_fixed_properties {
    pub hw_queues_props: *mut hw_queue_properties,
    pub special_blocks: *mut hl_special_block_info,
    pub skip_special_blocks_cfg: hl_skip_blocks_cfg,
    pub cpucp_info: cpucp_info,
    pub uboot_ver: [c_char; VERSION_MAX_LEN],
    pub preboot_ver: [c_char; VERSION_MAX_LEN],
    pub dmmu: hl_mmu_properties,
    pub pmmu: hl_mmu_properties,
    pub pmmu_huge: hl_mmu_properties,
    pub hints_dram_reserved_va_range: hl_hints_range,
    pub hints_host_reserved_va_range: hl_hints_range,
    pub hints_host_hpage_reserved_va_range: hl_hints_range,
    pub sram_base_address: u64,
    pub sram_end_address: u64,
    pub sram_user_base_address: u64,
    pub dram_base_address: u64,
    pub dram_end_address: u64,
    pub dram_user_base_address: u64,
    pub dram_size: u64,
    pub dram_pci_bar_size: u64,
    pub max_power_default: u64,
    pub dc_power_default: u64,
    pub dram_size_for_default_page_mapping: u64,
    pub pcie_dbi_base_address: u64,
    pub pcie_aux_dbi_reg_addr: u64,
    pub mmu_pgt_addr: u64,
    pub mmu_dram_default_page_addr: u64,
    pub tpc_enabled_mask: u64,
    pub tpc_binning_mask: u64,
    pub dram_enabled_mask: u64,
    pub dram_binning_mask: u64,
    pub dram_hints_align_mask: u64,
    pub cfg_base_address: u64,
    pub mmu_cache_mng_addr: u64,
    pub mmu_cache_mng_size: u64,
    pub device_dma_offset_for_host_access: u64,
    pub host_base_address: u64,
    pub host_end_address: u64,
    pub max_freq_value: u64,
    pub engine_core_interrupt_reg_addr: u64,
    pub clk_pll_index: u32,
    pub mmu_pgt_size: u32,
    pub mmu_pte_size: u32,
    pub dram_page_size: u32,
    pub cfg_size: u32,
    pub sram_size: u32,
    pub max_asid: u32,
    pub num_of_events: u32,
    pub psoc_pci_pll_nr: u32,
    pub psoc_pci_pll_nf: u32,
    pub psoc_pci_pll_od: u32,
    pub psoc_pci_pll_div_factor: u32,
    pub psoc_timestamp_frequency: u32,
    pub high_pll: u32,
    pub cb_pool_cb_cnt: u32,
    pub cb_pool_cb_size: u32,
    pub decoder_enabled_mask: u32,
    pub decoder_binning_mask: u32,
    pub rotator_enabled_mask: u32,
    pub edma_enabled_mask: u32,
    pub edma_binning_mask: u32,
    pub max_pending_cs: u32,
    pub max_queues: u32,
    pub fw_preboot_cpu_boot_dev_sts0: u32,
    pub fw_preboot_cpu_boot_dev_sts1: u32,
    pub fw_bootfit_cpu_boot_dev_sts0: u32,
    pub fw_bootfit_cpu_boot_dev_sts1: u32,
    pub fw_app_cpu_boot_dev_sts0: u32,
    pub fw_app_cpu_boot_dev_sts1: u32,
    pub max_dec: u32,
    pub hmmu_hif_enabled_mask: u32,
    pub faulty_dram_cluster_map: u32,
    pub xbar_edge_enabled_mask: u32,
    pub device_mem_alloc_default_page_size: u32,
    pub num_engine_cores: u32,
    pub max_num_of_engines: u32,
    pub num_of_special_blocks: u32,
    pub glbl_err_max_cause_num: u32,
    pub hbw_flush_reg: u32,
    pub reserved_fw_mem_size: u32,
    pub fw_event_queue_size: u32,
    pub collective_first_sob: u16,
    pub collective_first_mon: u16,
    pub sync_stream_first_sob: u16,
    pub sync_stream_first_mon: u16,
    pub first_available_user_sob: [u16; HL_MAX_DCORES],
    pub first_available_user_mon: [u16; HL_MAX_DCORES],
    pub first_available_user_interrupt: u16,
    pub first_available_cq: [u16; HL_MAX_DCORES],
    pub user_interrupt_count: u16,
    pub user_dec_intr_count: u16,
    pub tpc_interrupt_id: u16,
    pub eq_interrupt_id: u16,
    pub cache_line_size: u16,
    pub server_type: u16,
    pub completion_queues_count: u8,
    pub completion_mode: u8,
    pub mme_master_slave_mode: u8,
    pub fw_security_enabled: u8,
    pub fw_cpu_boot_dev_sts0_valid: u8,
    pub fw_cpu_boot_dev_sts1_valid: u8,
    pub dram_supports_virtual_memory: u8,
    pub hard_reset_done_by_fw: u8,
    pub num_functional_hbms: u8,
    pub hints_range_reservation: u8,
    pub iatu_done_by_fw: u8,
    pub dynamic_fw_load: u8,
    pub gic_interrupts_enable: u8,
    pub use_get_power_for_reset_history: u8,
    pub supports_compute_reset: u8,
    pub allow_inference_soft_reset: u8,
    pub configurable_stop_on_err: u8,
    pub set_max_power_on_device_init: u8,
    pub supports_user_set_page_size: u8,
    pub dma_mask: u8,
    pub supports_advanced_cpucp_rc: u8,
    pub supports_engine_modes: u8,
    pub support_dynamic_resereved_fw_size: u8,
    pub supports_nvme: u8,
}

//
// struct hl_fence - software synchronization primitive
// @completion: fence is implemented using completion
// @refcount: refcount for this fence
// @cs_sequence: sequence of the corresponding command submission
// @stream_master_qid_map: streams masters QID bitmap to represent all streams
// masters QIDs that multi cs is waiting on
// @error: mark this fence with error
// @timestamp: timestamp upon completion
// @mcs_handling_done: indicates that corresponding command submission has
// finished msc handling, this does not mean it was part
// of the mcs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_fence {
    pub completion: completion,
    pub refcount: kref,
    pub cs_sequence: u64,
    pub stream_master_qid_map: u32,
    pub error: c_int,
    pub timestamp: ktime_t,
    pub mcs_handling_done: u8,
}

//
// struct hl_cs_compl - command submission completion object.
// @base_fence: hl fence object.
// @lock: spinlock to protect fence.
// @hdev: habanalabs device structure.
// @hw_sob: the H/W SOB used in this signal/wait CS.
// @encaps_sig_hdl: encaps signals handler.
// @cs_seq: command submission sequence number.
// @type: type of the CS - signal/wait.
// @sob_val: the SOB value that is used in this signal/wait CS.
// @sob_group: the SOB group that is used in this collective wait CS.
// @encaps_signals: indication whether it's a completion object of cs with
// encaps signals or not.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_cs_compl {
    pub base_fence: hl_fence,
    pub lock: spinlock_t,
    pub hdev: *mut hl_device,
    pub hw_sob: *mut hl_hw_sob,
    pub encaps_sig_hdl: *mut hl_cs_encaps_sig_handle,
    pub cs_seq: u64,
    pub type: hl_cs_type,
    pub sob_val: u16,
    pub sob_group: u16,
    pub encaps_signals: bool,
}

//
// Command Buffers
//
// struct hl_ts_buff - describes a timestamp buffer.
// @kernel_buff_address: Holds the internal buffer's kernel virtual address.
// @user_buff_address: Holds the user buffer's kernel virtual address.
// @kernel_buff_size: Holds the internal kernel buffer size.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_ts_buff {
    pub kernel_buff_address: *mut c_void,
    pub user_buff_address: *mut c_void,
    pub kernel_buff_size: u32,
}

//
// struct hl_mem_mgr - describes unified memory manager for mappable memory chunks.
// @dev: back pointer to the owning device
// @lock: protects handles
// @handles: an idr holding all active handles to the memory buffers in the system.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_mem_mgr {
    pub dev: *mut device,
    pub lock: spinlock_t,
    pub handles: idr,
}

//
// struct hl_mem_mgr_fini_stats - describes statistics returned during memory manager teardown.
// @n_busy_cb: the amount of CB handles that could not be removed
// @n_busy_ts: the amount of TS handles that could not be removed
// @n_busy_other: the amount of any other type of handles that could not be removed
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_mem_mgr_fini_stats {
    pub n_busy_cb: u32,
    pub n_busy_ts: u32,
    pub n_busy_other: u32,
}

//
// struct hl_mmap_mem_buf_behavior - describes unified memory manager buffer behavior
// @topic: string identifier used for logging
// @mem_id: memory type identifier, embedded in the handle and used to identify
// the memory type by handle.
// @alloc: callback executed on buffer allocation, shall allocate the memory,
// set it under buffer private, and set mappable size.
// @mmap: callback executed on mmap, must map the buffer to vma
// @release: callback executed on release, must free the resources used by the buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_mmap_mem_buf_behavior {
    pub topic: *const c_char,
    pub mem_id: u64,
    pub args): *mut *mut *mut int (alloc)(struct hl_mmap_mem_buf buf, gfp_t gfp, void,
    pub args): *mut *mut *mut *mut int (mmap)(struct hl_mmap_mem_buf buf, struct vm_area_struct vma, void,
    pub buf): *mut *mut void (release)(struct hl_mmap_mem_buf,
}

//
// struct hl_mmap_mem_buf - describes a single unified memory buffer
// @behavior: buffer behavior
// @mmg: back pointer to the unified memory manager
// @refcount: reference counter for buffer users
// @private: pointer to buffer behavior private data
// @mmap: atomic boolean indicating whether or not the buffer is mapped right now
// @real_mapped_size: the actual size of buffer mapped, after part of it may be released,
// may change at runtime.
// @mappable_size: the original mappable size of the buffer, does not change after
// the allocation.
// @handle: the buffer id in mmg handles store
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_mmap_mem_buf {
    pub behavior: *mut hl_mmap_mem_buf_behavior,
    pub mmg: *mut hl_mem_mgr,
    pub refcount: kref,
    pub private: *mut c_void,
    pub mmap: core::sync::atomic::AtomicI32,
    pub real_mapped_size: u64,
    pub mappable_size: u64,
    pub handle: u64,
}

//
// struct hl_cb - describes a Command Buffer.
// @hdev: pointer to device this CB belongs to.
// @ctx: pointer to the CB owner's context.
// @buf: back pointer to the parent mappable memory buffer
// @debugfs_list: node in debugfs list of command buffers.
// @pool_list: node in pool list of command buffers.
// @kernel_address: Holds the CB's kernel virtual address.
// @virtual_addr: Holds the CB's virtual address.
// @bus_address: Holds the CB's DMA address.
// @size: holds the CB's size.
// @roundup_size: holds the cb size after roundup to page size.
// @cs_cnt: holds number of CS that this CB participates in.
// @is_handle_destroyed: atomic boolean indicating whether or not the CB handle was destroyed.
// @is_pool: true if CB was acquired from the pool, false otherwise.
// @is_internal: internally allocated
// @is_mmu_mapped: true if the CB is mapped to the device's MMU.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_cb {
    pub hdev: *mut hl_device,
    pub ctx: *mut hl_ctx,
    pub buf: *mut hl_mmap_mem_buf,
    pub debugfs_list: list_head,
    pub pool_list: list_head,
    pub kernel_address: *mut c_void,
    pub virtual_addr: u64,
    pub bus_address: dma_addr_t,
    pub size: u32,
    pub roundup_size: u32,
    pub cs_cnt: core::sync::atomic::AtomicI32,
    pub is_handle_destroyed: core::sync::atomic::AtomicI32,
    pub is_pool: u8,
    pub is_internal: u8,
    pub is_mmu_mapped: u8,
}

//
// QUEUES
//
// Queue length of external and HW queues
pub const HL_QUEUE_LENGTH: c_int = 4096;

// HL_CQ_LENGTH is in units of struct hl_cq_entry

// Must be power of 2
pub const HL_EQ_LENGTH: c_int = 64;

// Host <-> CPU-CP shared memory size

//
// struct hl_sync_stream_properties -
// describes a H/W queue sync stream properties
// @hw_sob: array of the used H/W SOBs by this H/W queue.
// @next_sob_val: the next value to use for the currently used SOB.
// @base_sob_id: the base SOB id of the SOBs used by this queue.
// @base_mon_id: the base MON id of the MONs used by this queue.
// @collective_mstr_mon_id: the MON ids of the MONs used by this master queue
// in order to sync with all slave queues.
// @collective_slave_mon_id: the MON id used by this slave queue in order to
// sync with its master queue.
// @collective_sob_id: current SOB id used by this collective slave queue
// to signal its collective master queue upon completion.
// @curr_sob_offset: the id offset to the currently used SOB from the
// HL_RSVD_SOBS that are being used by this queue.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_sync_stream_properties {
    pub hw_sob: [hl_hw_sob; HL_RSVD_SOBS],
    pub next_sob_val: u16,
    pub base_sob_id: u16,
    pub base_mon_id: u16,
    pub collective_mstr_mon_id: [u16; HL_COLLECTIVE_RSVD_MSTR_MONS],
    pub collective_slave_mon_id: u16,
    pub collective_sob_id: u16,
    pub curr_sob_offset: u8,
}

//
// struct hl_encaps_signals_mgr - describes sync stream encapsulated signals
// handlers manager
// @lock: protects handles.
// @handles: an idr to hold all encapsulated signals handles.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_encaps_signals_mgr {
    pub lock: spinlock_t,
    pub handles: idr,
}

//
// struct hl_hw_queue - describes a H/W transport queue.
// @shadow_queue: pointer to a shadow queue that holds pointers to jobs.
// @sync_stream_prop: sync stream queue properties
// @queue_type: type of queue.
// @collective_mode: collective mode of current queue
// @kernel_address: holds the queue's kernel virtual address.
// @bus_address: holds the queue's DMA address.
// @pq_dram_address: hold the dram address when the PQ is allocated, used when dram_bd is true in
// queue properites.
// @pi: holds the queue's pi value.
// @ci: holds the queue's ci value, AS CALCULATED BY THE DRIVER (not real ci).
// @hw_queue_id: the id of the H/W queue.
// @cq_id: the id for the corresponding CQ for this H/W queue.
// @msi_vec: the IRQ number of the H/W queue.
// @int_queue_len: length of internal queue (number of entries).
// @valid: is the queue valid (we have array of 32 queues, not all of them
// exist).
// @supports_sync_stream: True if queue supports sync stream
// @dram_bd: True if the bd should be copied to dram, needed for PQ which has been allocated on dram
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_hw_queue {
    pub shadow_queue: *mut hl_cs_job,
    pub sync_stream_prop: hl_sync_stream_properties,
    pub queue_type: hl_queue_type,
    pub collective_mode: hl_collective_mode,
    pub kernel_address: *mut c_void,
    pub bus_address: dma_addr_t,
    pub pq_dram_address: u64,
    pub pi: u32,
    pub ci: core::sync::atomic::AtomicI32,
    pub hw_queue_id: u32,
    pub cq_id: u32,
    pub msi_vec: u32,
    pub int_queue_len: u16,
    pub valid: u8,
    pub supports_sync_stream: u8,
    pub dram_bd: u8,
}

//
// struct hl_cq - describes a completion queue
// @hdev: pointer to the device structure
// @kernel_address: holds the queue's kernel virtual address
// @bus_address: holds the queue's DMA address
// @cq_idx: completion queue index in array
// @hw_queue_id: the id of the matching H/W queue
// @ci: ci inside the queue
// @pi: pi inside the queue
// @free_slots_cnt: counter of free slots in queue
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_cq {
    pub hdev: *mut hl_device,
    pub kernel_address: *mut c_void,
    pub bus_address: dma_addr_t,
    pub cq_idx: u32,
    pub hw_queue_id: u32,
    pub ci: u32,
    pub pi: u32,
    pub free_slots_cnt: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hl_user_interrupt_type {
    HL_USR_INTERRUPT_CQ = 0,
    HL_USR_INTERRUPT_DECODER,
    HL_USR_INTERRUPT_TPC,
    HL_USR_INTERRUPT_UNEXPECTED
}

//
// struct hl_ts_free_jobs - holds user interrupt ts free nodes related data
// @free_nodes_pool: pool of nodes to be used for free timestamp jobs
// @free_nodes_length: number of nodes in free_nodes_pool
// @next_avail_free_node_idx: index of the next free node in the pool
//
// the free nodes pool must be protected by the user interrupt lock
// to avoid race between different interrupts which are using the same
// ts buffer with different offsets.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_ts_free_jobs {
    pub free_nodes_pool: *mut timestamp_reg_free_node,
    pub free_nodes_length: u32,
    pub next_avail_free_node_idx: u32,
}

//
// struct hl_user_interrupt - holds user interrupt information
// @hdev: pointer to the device structure
// @ts_free_jobs_data: timestamp free jobs related data
// @type: user interrupt type
// @wait_list_head: head to the list of user threads pending on this interrupt
// @ts_list_head: head to the list of timestamp records
// @wait_list_lock: protects wait_list_head
// @ts_list_lock: protects ts_list_head
// @timestamp: last timestamp taken upon interrupt
// @interrupt_id: msix interrupt id
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_user_interrupt {
    pub hdev: *mut hl_device,
    pub ts_free_jobs_data: hl_ts_free_jobs,
    pub type: hl_user_interrupt_type,
    pub wait_list_head: list_head,
    pub ts_list_head: list_head,
    pub wait_list_lock: spinlock_t,
    pub ts_list_lock: spinlock_t,
    pub timestamp: ktime_t,
    pub interrupt_id: u32,
}

//
// struct timestamp_reg_free_node - holds the timestamp registration free objects node
// @free_objects_node: node in the list free_obj_jobs
// @cq_cb: pointer to cq command buffer to be freed
// @buf: pointer to timestamp buffer to be freed
// @in_use: indicates whether the node still in use in workqueue thread.
// @dynamic_alloc: indicates whether the node was allocated dynamically in the interrupt handler
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timestamp_reg_free_node {
    pub free_objects_node: list_head,
    pub cq_cb: *mut hl_cb,
    pub buf: *mut hl_mmap_mem_buf,
    pub in_use: core::sync::atomic::AtomicI32,
    pub dynamic_alloc: u8,
}

// struct timestamp_reg_work_obj - holds the timestamp registration free objects job
// the job will be to pass over the free_obj_jobs list and put refcount to objects
// in each node of the list
// @free_obj: workqueue object to free timestamp registration node objects
// @hdev: pointer to the device structure
// @free_obj_head: list of free jobs nodes (node type timestamp_reg_free_node)
// @dynamic_alloc_free_obj_head: list of free jobs nodes which were dynamically allocated in the
// interrupt handler.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timestamp_reg_work_obj {
    pub free_obj: work_struct,
    pub hdev: *mut hl_device,
    pub free_obj_head: *mut list_head,
    pub dynamic_alloc_free_obj_head: *mut list_head,
}

// struct timestamp_reg_info - holds the timestamp registration related data.
// @buf: pointer to the timestamp buffer which include both user/kernel buffers.
// relevant only when doing timestamps records registration.
// @cq_cb: pointer to CQ counter CB.
// @interrupt: interrupt that the node hanged on it's wait list.
// @timestamp_kernel_addr: timestamp handle address, where to set timestamp
// relevant only when doing timestamps records
// registration.
// @in_use: indicates if the node already in use. relevant only when doing
// timestamps records registration, since in this case the driver
// will have it's own buffer which serve as a records pool instead of
// allocating records dynamically.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timestamp_reg_info {
    pub buf: *mut hl_mmap_mem_buf,
    pub cq_cb: *mut hl_cb,
    pub interrupt: *mut hl_user_interrupt,
    pub timestamp_kernel_addr: *mut u64,
    pub in_use: bool,
}

//
// struct hl_user_pending_interrupt - holds a context to a user thread
// pending on an interrupt
// @ts_reg_info: holds the timestamps registration nodes info
// @list_node: node in the list of user threads pending on an interrupt or timestamp
// @fence: hl fence object for interrupt completion
// @cq_target_value: CQ target value
// @cq_kernel_addr: CQ kernel address, to be used in the cq interrupt
// handler for target value comparison
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_user_pending_interrupt {
    pub ts_reg_info: timestamp_reg_info,
    pub list_node: list_head,
    pub fence: hl_fence,
    pub cq_target_value: u64,
    pub cq_kernel_addr: *mut u64,
}

//
// struct hl_eq - describes the event queue (single one per device)
// @hdev: pointer to the device structure
// @kernel_address: holds the queue's kernel virtual address
// @bus_address: holds the queue's DMA address
// @size: the event queue size
// @ci: ci inside the queue
// @prev_eqe_index: the index of the previous event queue entry. The index of
// the current entry's index must be +1 of the previous one.
// @check_eqe_index: do we need to check the index of the current entry vs. the
// previous one. This is for backward compatibility with older
// firmwares
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_eq {
    pub hdev: *mut hl_device,
    pub kernel_address: *mut c_void,
    pub bus_address: dma_addr_t,
    pub size: u32,
    pub ci: u32,
    pub prev_eqe_index: u32,
    pub check_eqe_index: bool,
}

//
// struct hl_dec - describes a decoder sw instance.
// @hdev: pointer to the device structure.
// @abnrm_intr_work: workqueue work item to run when decoder generates an error interrupt.
// @core_id: ID of the decoder.
// @base_addr: base address of the decoder.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_dec {
    pub hdev: *mut hl_device,
    pub abnrm_intr_work: work_struct,
    pub core_id: u32,
    pub base_addr: u32,
}

//
// enum hl_asic_type - supported ASIC types.
// @ASIC_INVALID: Invalid ASIC type.
// @ASIC_GOYA: Goya device (HL-1000).
// @ASIC_GAUDI: Gaudi device (HL-2000).
// @ASIC_GAUDI_SEC: Gaudi secured device (HL-2000).
// @ASIC_GAUDI2: Gaudi2 device.
// @ASIC_GAUDI2B: Gaudi2B device.
// @ASIC_GAUDI2C: Gaudi2C device.
// @ASIC_GAUDI2D: Gaudi2D device.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hl_asic_type {
    ASIC_INVALID,

    ASIC_GOYA,
    ASIC_GAUDI,
    ASIC_GAUDI_SEC,
    ASIC_GAUDI2,
    ASIC_GAUDI2B,
    ASIC_GAUDI2C,
    ASIC_GAUDI2D,
}

//
// enum hl_pm_mng_profile - power management profile.
// @PM_AUTO: internal clock is set by the Linux driver.
// @PM_MANUAL: internal clock is set by the user.
// @PM_LAST: last power management type.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hl_pm_mng_profile {
    PM_AUTO = 1,
    PM_MANUAL,
    PM_LAST
}

//
// enum hl_pll_frequency - PLL frequency.
// @PLL_HIGH: high frequency.
// @PLL_LOW: low frequency.
// @PLL_LAST: last frequency values that were configured by the user.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hl_pll_frequency {
    PLL_HIGH = 1,
    PLL_LOW,
    PLL_LAST
}

pub const PLL_REF_CLK: c_int = 50;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum div_select_defs {
    DIV_SEL_REF_CLK = 0,
    DIV_SEL_PLL_CLK = 1,
    DIV_SEL_DIVIDED_REF = 2,
    DIV_SEL_DIVIDED_PLL = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum debugfs_access_type {
    DEBUGFS_READ8,
    DEBUGFS_WRITE8,
    DEBUGFS_READ32,
    DEBUGFS_WRITE32,
    DEBUGFS_READ64,
    DEBUGFS_WRITE64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_region {
    PCI_REGION_CFG,
    PCI_REGION_SRAM,
    PCI_REGION_DRAM,
    PCI_REGION_SP_SRAM,
    PCI_REGION_NUMBER,
}

//
// struct pci_mem_region - describe memory region in a PCI bar
// @region_base: region base address
// @region_size: region size
// @bar_size: size of the BAR
// @offset_in_bar: region offset into the bar
// @bar_id: bar ID of the region
// @used: if used 1, otherwise 0
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_mem_region {
    pub region_base: u64,
    pub region_size: u64,
    pub bar_size: u64,
    pub offset_in_bar: u64,
    pub bar_id: u8,
    pub used: u8,
}

//
// struct static_fw_load_mgr - static FW load manager
// @preboot_version_max_off: max offset to preboot version
// @boot_fit_version_max_off: max offset to boot fit version
// @kmd_msg_to_cpu_reg: register address for KDM->CPU messages
// @cpu_cmd_status_to_host_reg: register address for CPU command status response
// @cpu_boot_status_reg: boot status register
// @cpu_boot_dev_status0_reg: boot device status register 0
// @cpu_boot_dev_status1_reg: boot device status register 1
// @boot_err0_reg: boot error register 0
// @boot_err1_reg: boot error register 1
// @preboot_version_offset_reg: SRAM offset to preboot version register
// @boot_fit_version_offset_reg: SRAM offset to boot fit version register
// @sram_offset_mask: mask for getting offset into the SRAM
// @cpu_reset_wait_msec: used when setting WFE via kmd_msg_to_cpu_reg
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct static_fw_load_mgr {
    pub preboot_version_max_off: u64,
    pub boot_fit_version_max_off: u64,
    pub kmd_msg_to_cpu_reg: u32,
    pub cpu_cmd_status_to_host_reg: u32,
    pub cpu_boot_status_reg: u32,
    pub cpu_boot_dev_status0_reg: u32,
    pub cpu_boot_dev_status1_reg: u32,
    pub boot_err0_reg: u32,
    pub boot_err1_reg: u32,
    pub preboot_version_offset_reg: u32,
    pub boot_fit_version_offset_reg: u32,
    pub sram_offset_mask: u32,
    pub cpu_reset_wait_msec: u32,
}

//
// struct fw_response - FW response to LKD command
// @ram_offset: descriptor offset into the RAM
// @ram_type: RAM type containing the descriptor (SRAM/DRAM)
// @status: command status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_response {
    pub ram_offset: u32,
    pub ram_type: u8,
    pub status: u8,
}

//
// struct dynamic_fw_load_mgr - dynamic FW load manager
// @response: FW to LKD response
// @comm_desc: the communication descriptor with FW
// @image_region: region to copy the FW image to
// @fw_image_size: size of FW image to load
// @wait_for_bl_timeout: timeout for waiting for boot loader to respond
// @fw_desc_valid: true if FW descriptor has been validated and hence the data can be used
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dynamic_fw_load_mgr {
    pub response: fw_response,
    pub comm_desc: lkd_fw_comms_desc,
    pub image_region: *mut pci_mem_region,
    pub fw_image_size: usize,
    pub wait_for_bl_timeout: u32,
    pub fw_desc_valid: bool,
}

//
// struct pre_fw_load_props - needed properties for pre-FW load
// @cpu_boot_status_reg: cpu_boot_status register address
// @sts_boot_dev_sts0_reg: sts_boot_dev_sts0 register address
// @sts_boot_dev_sts1_reg: sts_boot_dev_sts1 register address
// @boot_err0_reg: boot_err0 register address
// @boot_err1_reg: boot_err1 register address
// @wait_for_preboot_timeout: timeout to poll for preboot ready
// @wait_for_preboot_extended_timeout: timeout to pull for preboot ready in case where we know
// preboot needs longer time.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pre_fw_load_props {
    pub cpu_boot_status_reg: u32,
    pub sts_boot_dev_sts0_reg: u32,
    pub sts_boot_dev_sts1_reg: u32,
    pub boot_err0_reg: u32,
    pub boot_err1_reg: u32,
    pub wait_for_preboot_timeout: u32,
    pub wait_for_preboot_extended_timeout: u32,
}

//
// struct fw_image_props - properties of FW image
// @image_name: name of the image
// @src_off: offset in src FW to copy from
// @copy_size: amount of bytes to copy (0 to copy the whole binary)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_image_props {
    pub image_name: *mut c_char,
    pub src_off: u32,
    pub copy_size: u32,
}

//
// struct fw_load_mgr - manager FW loading process
// @dynamic_loader: specific structure for dynamic load
// @static_loader: specific structure for static load
// @pre_fw_load_props: parameter for pre FW load
// @boot_fit_img: boot fit image properties
// @linux_img: linux image properties
// @cpu_timeout: CPU response timeout in usec
// @boot_fit_timeout: Boot fit load timeout in usec
// @skip_bmc: should BMC be skipped
// @sram_bar_id: SRAM bar ID
// @dram_bar_id: DRAM bar ID
// @fw_comp_loaded: bitmask of loaded FW components. set bit meaning loaded
// component. values are set according to enum hl_fw_types.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_load_mgr {
    pub dynamic_loader: dynamic_fw_load_mgr,
    pub static_loader: static_fw_load_mgr,
}

//
// struct engines_data - asic engines data
// @buf: buffer for engines data in ascii
// @actual_size: actual size of data that was written by the driver to the allocated buffer
// @allocated_buf_size: total size of allocated buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct engines_data {
    pub buf: *mut c_char,
    pub actual_size: c_int,
    pub allocated_buf_size: u32,
}

//
// struct hl_asic_funcs - ASIC specific functions that are can be called from
// common code.
// @early_init: sets up early driver state (pre sw_init), doesn't configure H/W.
// @early_fini: tears down what was done in early_init.
// @late_init: sets up late driver/hw state (post hw_init) - Optional.
// @late_fini: tears down what was done in late_init (pre hw_fini) - Optional.
// @sw_init: sets up driver state, does not configure H/W.
// @sw_fini: tears down driver state, does not configure H/W.
// @hw_init: sets up the H/W state.
// @hw_fini: tears down the H/W state.
// @halt_engines: halt engines, needed for reset sequence. This also disables
// interrupts from the device. Should be called before
// hw_fini and before CS rollback.
// @suspend: handles IP specific H/W or SW changes for suspend.
// @resume: handles IP specific H/W or SW changes for resume.
// @mmap: maps a memory.
// @ring_doorbell: increment PI on a given QMAN.
// @pqe_write: Write the PQ entry to the PQ. This is ASIC-specific
// function because the PQs are located in different memory areas
// per ASIC (SRAM, DRAM, Host memory) and therefore, the method of
// writing the PQE must match the destination memory area
// properties.
// @asic_dma_alloc_coherent: Allocate coherent DMA memory by calling
// dma_alloc_coherent(). This is ASIC function because
// its implementation is not trivial when the driver
// is loaded in simulation mode (not upstreamed).
// @asic_dma_free_coherent:  Free coherent DMA memory by calling
// dma_free_coherent(). This is ASIC function because
// its implementation is not trivial when the driver
// is loaded in simulation mode (not upstreamed).
// @scrub_device_mem: Scrub the entire SRAM and DRAM.
// @scrub_device_dram: Scrub the dram memory of the device.
// @get_int_queue_base: get the internal queue base address.
// @test_queues: run simple test on all queues for sanity check.
// @asic_dma_pool_zalloc: small DMA allocation of coherent memory from DMA pool.
// size of allocation is HL_DMA_POOL_BLK_SIZE.
// @asic_dma_pool_free: free small DMA allocation from pool.
// @cpu_accessible_dma_pool_alloc: allocate CPU PQ packet from DMA pool.
// @cpu_accessible_dma_pool_free: free CPU PQ packet from DMA pool.
// @dma_unmap_sgtable: DMA unmap scatter-gather table.
// @dma_map_sgtable: DMA map scatter-gather table.
// @cs_parser: parse Command Submission.
// @add_end_of_cb_packets: Add packets to the end of CB, if device requires it.
// @update_eq_ci: update event queue CI.
// @context_switch: called upon ASID context switch.
// @restore_phase_topology: clear all SOBs amd MONs.
// @debugfs_read_dma: debug interface for reading up to 2MB from the device's
// internal memory via DMA engine.
// @add_device_attr: add ASIC specific device attributes.
// @handle_eqe: handle event queue entry (IRQ) from CPU-CP.
// @get_events_stat: retrieve event queue entries histogram.
// @read_pte: read MMU page table entry from DRAM.
// @write_pte: write MMU page table entry to DRAM.
// @mmu_invalidate_cache: flush MMU STLB host/DRAM cache, either with soft
// (L1 only) or hard (L0 & L1) flush.
// @mmu_invalidate_cache_range: flush specific MMU STLB cache lines with ASID-VA-size mask.
// @mmu_prefetch_cache_range: pre-fetch specific MMU STLB cache lines with ASID-VA-size mask.
// @send_heartbeat: send is-alive packet to CPU-CP and verify response.
// @debug_coresight: perform certain actions on Coresight for debugging.
// @is_device_idle: return true if device is idle, false otherwise.
// @compute_reset_late_init: perform certain actions needed after a compute reset
// @hw_queues_lock: acquire H/W queues lock.
// @hw_queues_unlock: release H/W queues lock.
// @get_pci_id: retrieve PCI ID.
// @get_eeprom_data: retrieve EEPROM data from F/W.
// @get_monitor_dump: retrieve monitor registers dump from F/W.
// @send_cpu_message: send message to F/W. If the message is timedout, the
// driver will eventually reset the device. The timeout can
// be determined by the calling function or it can be 0 and
// then the timeout is the default timeout for the specific
// ASIC
// @get_hw_state: retrieve the H/W state
// @pci_bars_map: Map PCI BARs.
// @init_iatu: Initialize the iATU unit inside the PCI controller.
// @rreg: Read a register. Needed for simulator support.
// @wreg: Write a register. Needed for simulator support.
// @halt_coresight: stop the ETF and ETR traces.
// @ctx_init: context dependent initialization.
// @ctx_fini: context dependent cleanup.
// @pre_schedule_cs: Perform pre-CS-scheduling operations.
// @get_queue_id_for_cq: Get the H/W queue id related to the given CQ index.
// @load_firmware_to_device: load the firmware to the device's memory
// @load_boot_fit_to_device: load boot fit to device's memory
// @get_signal_cb_size: Get signal CB size.
// @get_wait_cb_size: Get wait CB size.
// @gen_signal_cb: Generate a signal CB.
// @gen_wait_cb: Generate a wait CB.
// @reset_sob: Reset a SOB.
// @reset_sob_group: Reset SOB group
// @get_device_time: Get the device time.
// @pb_print_security_errors: print security errors according block and cause
// @collective_wait_init_cs: Generate collective master/slave packets
// and place them in the relevant cs jobs
// @collective_wait_create_jobs: allocate collective wait cs jobs
// @get_dec_base_addr: get the base address of a given decoder.
// @scramble_addr: Routine to scramble the address prior of mapping it
// in the MMU.
// @descramble_addr: Routine to de-scramble the address prior of
// showing it to users.
// @ack_protection_bits_errors: ack and dump all security violations
// @get_hw_block_id: retrieve a HW block id to be used by the user to mmap it.
// also returns the size of the block if caller supplies
// a valid pointer for it
// @hw_block_mmap: mmap a HW block with a given id.
// @enable_events_from_fw: send interrupt to firmware to notify them the
// driver is ready to receive asynchronous events. This
// function should be called during the first init and
// after every hard-reset of the device
// @ack_mmu_errors: check and ack mmu errors, page fault, access violation.
// @get_msi_info: Retrieve asic-specific MSI ID of the f/w async event
// @map_pll_idx_to_fw_idx: convert driver specific per asic PLL index to
// generic f/w compatible PLL Indexes
// @init_firmware_preload_params: initialize pre FW-load parameters.
// @init_firmware_loader: initialize data for FW loader.
// @init_cpu_scrambler_dram: Enable CPU specific DRAM scrambling
// @state_dump_init: initialize constants required for state dump
// @get_sob_addr: get SOB base address offset.
// @set_pci_memory_regions: setting properties of PCI memory regions
// @get_stream_master_qid_arr: get pointer to stream masters QID array
// @check_if_razwi_happened: check if there was a razwi due to RR violation.
// @access_dev_mem: access device memory
// @set_dram_bar_base: set the base of the DRAM BAR
// @set_engine_cores: set a config command to engine cores
// @set_engines: set a config command to user engines
// @send_device_activity: indication to FW about device availability
// @set_dram_properties: set DRAM related properties.
// @set_binning_masks: set binning/enable masks for all relevant components.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_asic_funcs {
    pub hdev): *mut *mut int (early_init)(struct hl_device,
    pub hdev): *mut *mut int (early_fini)(struct hl_device,
    pub hdev): *mut *mut int (late_init)(struct hl_device,
    pub hdev): *mut *mut void (late_fini)(struct hl_device,
    pub hdev): *mut *mut int (sw_init)(struct hl_device,
    pub hdev): *mut *mut int (sw_fini)(struct hl_device,
    pub hdev): *mut *mut int (hw_init)(struct hl_device,
    pub fw_reset): *mut *mut *mut int (hw_fini)(struct hl_device hdev, bool hard_reset, bool,
    pub fw_reset): *mut *mut *mut void (halt_engines)(struct hl_device hdev, bool hard_reset, bool,
    pub hdev): *mut *mut int (suspend)(struct hl_device,
    pub hdev): *mut *mut int (resume)(struct hl_device,
    pub size): *mut *mut void cpu_addr, dma_addr_t dma_addr, size_t,
    pub pi): *mut *mut *mut void (ring_doorbell)(struct hl_device hdev, u32 hw_queue_id, u32,
    pub bd): *mut hl_bd,
    pub flag): *mut *mut dma_addr_t dma_handle, gfp_t,
    pub dma_handle): *mut *mut void cpu_addr, dma_addr_t,
    pub hdev): *mut *mut int (scrub_device_mem)(struct hl_device,
    pub val): *mut *mut *mut int (scrub_device_dram)(struct hl_device hdev, u64,
    pub queue_len): *mut *mut dma_addr_t dma_handle, u16,
    pub hdev): *mut *mut int (test_queues)(struct hl_device,
    pub dma_handle): *mut gfp_t mem_flags, dma_addr_t,
    pub dma_addr): dma_addr_t,
    pub dma_handle): *mut size_t size, dma_addr_t,
    pub vaddr): *mut size_t size, void,
    pub dir): dma_data_direction,
    pub dir): dma_data_direction,
    pub parser): *mut *mut *mut int (cs_parser)(struct hl_device hdev, struct hl_cs_parser,
    pub eb): bool,
    pub val): *mut *mut *mut void (update_eq_ci)(struct hl_device hdev, u32,
    pub asid): *mut *mut *mut int (context_switch)(struct hl_device hdev, u32,
    pub hdev): *mut *mut void (restore_phase_topology)(struct hl_device,
    pub blob_addr): *mut c_void,
    pub dev_vrm_attr_grp): *mut attribute_group,
    pub eq_entry): *mut hl_eq_entry,
    pub size): *mut u32,
    pub addr): *mut *mut *mut u64 (read_pte)(struct hl_device hdev, u64,
    pub val): *mut *mut *mut void (write_pte)(struct hl_device hdev, u64 addr, u64,
    pub flags): u32,
    pub size): u32 flags, u32 asid, u64 va, u64,
    pub size): *mut *mut *mut int (mmu_prefetch_cache_range)(struct hl_ctx ctx, u32 flags, u32 asid, u64 va, u64,
    pub hdev): *mut *mut int (send_heartbeat)(struct hl_device,
    pub data): *mut *mut *mut *mut int (debug_coresight)(struct hl_device hdev, struct hl_ctx ctx, void,
    pub e): *mut engines_data,
    pub hdev): *mut *mut int (compute_reset_late_init)(struct hl_device,
    pub hdev): *mut *mut void (hw_queues_lock)(struct hl_device,
    pub hdev): *mut *mut void (hw_queues_unlock)(struct hl_device,
    pub hdev): *mut *mut u32 (get_pci_id)(struct hl_device,
    pub max_size): *mut *mut *mut *mut int (get_eeprom_data)(struct hl_device hdev, void data, size_t,
    pub data): *mut *mut *mut int (get_monitor_dump)(struct hl_device hdev, void,
    pub result): *mut u16 len, u32 timeout, u64,
    pub hdev): *mut *mut int (pci_bars_map)(struct hl_device,
    pub hdev): *mut *mut int (init_iatu)(struct hl_device,
    pub reg): *mut *mut *mut u32 (rreg)(struct hl_device hdev, u32,
    pub val): *mut *mut *mut void (wreg)(struct hl_device hdev, u32 reg, u32,
    pub ctx): *mut *mut *mut void (halt_coresight)(struct hl_device hdev, struct hl_ctx,
    pub ctx): *mut *mut int (ctx_init)(struct hl_ctx,
    pub ctx): *mut *mut void (ctx_fini)(struct hl_ctx,
    pub cs): *mut *mut int (pre_schedule_cs)(struct hl_cs,
    pub cq_idx): *mut *mut *mut u32 (get_queue_id_for_cq)(struct hl_device hdev, u32,
    pub hdev): *mut *mut int (load_firmware_to_device)(struct hl_device,
    pub hdev): *mut *mut int (load_boot_fit_to_device)(struct hl_device,
    pub hdev): *mut *mut u32 (get_signal_cb_size)(struct hl_device,
    pub hdev): *mut *mut u32 (get_wait_cb_size)(struct hl_device,
    pub eb): u32 size, bool,
    pub prop): *mut hl_gen_wait_properties,
    pub data): *mut *mut *mut void (reset_sob)(struct hl_device hdev, void,
    pub sob_group): *mut *mut *mut void (reset_sob_group)(struct hl_device hdev, u16,
    pub hdev): *mut *mut u64 (get_device_time)(struct hl_device,
    pub offended_addr): u32 block_addr, u32 cause, u32,
    pub cs): *mut *mut int (collective_wait_init_cs)(struct hl_cs,
    pub encaps_signal_offset): u32,
    pub core_id): *mut *mut *mut u32 (get_dec_base_addr)(struct hl_device hdev, u32,
    pub addr): *mut *mut *mut u64 (scramble_addr)(struct hl_device hdev, u64,
    pub addr): *mut *mut *mut u64 (descramble_addr)(struct hl_device hdev, u64,
    pub hdev): *mut *mut void (ack_protection_bits_errors)(struct hl_device,
    pub block_id): *mut *mut u32 block_size, u32,
    pub block_size): u32 block_id, u32,
    pub hdev): *mut *mut void (enable_events_from_fw)(struct hl_device,
    pub mmu_cap_mask): *mut *mut *mut int (ack_mmu_errors)(struct hl_device hdev, u64,
    pub table): *mut *mut void (get_msi_info)(__le32,
    pub pll_idx): *mut *mut int (map_pll_idx_to_fw_idx)(u32,
    pub hdev): *mut *mut void (init_firmware_preload_params)(struct hl_device,
    pub hdev): *mut *mut void (init_firmware_loader)(struct hl_device,
    pub hdev): *mut *mut void (init_cpu_scrambler_dram)(struct hl_device,
    pub hdev): *mut *mut void (state_dump_init)(struct hl_device,
    pub sob_id): *mut *mut *mut u32 (get_sob_addr)(struct hl_device hdev, u32,
    pub hdev): *mut *mut void (set_pci_memory_regions)(struct hl_device,
    pub (*get_stream_master_qid_arr)(void): *mut *mut u32,
    pub hdev): *mut *mut void (check_if_razwi_happened)(struct hl_device,
    pub is_dram_addr): *mut *mut u32 page_size, u32 real_page_size, bool,
    pub acc_type): *mut *mut u64 addr, u64 val, enum debugfs_access_type,
    pub addr): *mut *mut *mut u64 (set_dram_bar_base)(struct hl_device hdev, u64,
    pub core_command): u32 num_cores, u32,
    pub engine_command): u32 num_engines, u32,
    pub open): *mut *mut *mut int (send_device_activity)(struct hl_device hdev, bool,
    pub hdev): *mut *mut int (set_dram_properties)(struct hl_device,
    pub hdev): *mut *mut int (set_binning_masks)(struct hl_device,
}

//
// CONTEXTS
//
pub const HL_KERNEL_ASID_ID: c_int = 0;
//
// enum hl_va_range_type - virtual address range type.
// @HL_VA_RANGE_TYPE_HOST: range type of host pages
// @HL_VA_RANGE_TYPE_HOST_HUGE: range type of host huge pages
// @HL_VA_RANGE_TYPE_DRAM: range type of dram pages
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hl_va_range_type {
    HL_VA_RANGE_TYPE_HOST,
    HL_VA_RANGE_TYPE_HOST_HUGE,
    HL_VA_RANGE_TYPE_DRAM,
    HL_VA_RANGE_TYPE_MAX
}

//
// struct hl_va_range - virtual addresses range.
// @lock: protects the virtual addresses list.
// @list: list of virtual addresses blocks available for mappings.
// @start_addr: range start address.
// @end_addr: range end address.
// @page_size: page size of this va range.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_va_range {
    pub lock: mutex,
    pub list: list_head,
    pub start_addr: u64,
    pub end_addr: u64,
    pub page_size: u32,
}

//
// struct hl_cs_counters_atomic - command submission counters
// @out_of_mem_drop_cnt: dropped due to memory allocation issue
// @parsing_drop_cnt: dropped due to error in packet parsing
// @queue_full_drop_cnt: dropped due to queue full
// @device_in_reset_drop_cnt: dropped due to device in reset
// @max_cs_in_flight_drop_cnt: dropped due to maximum CS in-flight
// @validation_drop_cnt: dropped due to error in validation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_cs_counters_atomic {
    pub out_of_mem_drop_cnt: core::sync::atomic::AtomicI64,
    pub parsing_drop_cnt: core::sync::atomic::AtomicI64,
    pub queue_full_drop_cnt: core::sync::atomic::AtomicI64,
    pub device_in_reset_drop_cnt: core::sync::atomic::AtomicI64,
    pub max_cs_in_flight_drop_cnt: core::sync::atomic::AtomicI64,
    pub validation_drop_cnt: core::sync::atomic::AtomicI64,
}

//
// struct hl_dmabuf_priv - a dma-buf private object.
// @dmabuf: pointer to dma-buf object.
// @ctx: pointer to the dma-buf owner's context.
// @phys_pg_pack: pointer to physical page pack if the dma-buf was exported
// where virtual memory is supported.
// @memhash_hnode: pointer to the memhash node. this object holds the export count.
// @offset: the offset into the buffer from which the memory is exported.
// Relevant only if virtual memory is supported and phys_pg_pack is being used.
// device_phys_addr: physical address of the device's memory. Relevant only
// if phys_pg_pack is NULL (dma-buf was exported from address).
// The total size can be taken from the dmabuf object.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_dmabuf_priv {
    pub dmabuf: *mut dma_buf,
    pub ctx: *mut hl_ctx,
    pub phys_pg_pack: *mut hl_vm_phys_pg_pack,
    pub memhash_hnode: *mut hl_vm_hash_node,
    pub offset: u64,
    pub device_phys_addr: u64,
}

pub const HL_CS_OUTCOME_HISTORY_LEN: c_int = 256;
//
// struct hl_cs_outcome - represents a single completed CS outcome
// @list_link: link to either container's used list or free list
// @map_link: list to the container hash map
// @ts: completion ts
// @seq: the original cs sequence
// @error: error code cs completed with, if any
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_cs_outcome {
    pub list_link: list_head,
    pub map_link: hlist_node,
    pub ts: ktime_t,
    pub seq: u64,
    pub error: c_int,
}

//
// struct hl_cs_outcome_store - represents a limited store of completed CS outcomes
// @outcome_map: index of completed CS searchable by sequence number
// @used_list: list of outcome objects currently in use
// @free_list: list of outcome objects currently not in use
// @nodes_pool: a static pool of pre-allocated outcome objects
// @db_lock: any operation on the store must take this lock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_cs_outcome_store {
    pub 8): DECLARE_HASHTABLE(outcome_map,,
    pub used_list: list_head,
    pub free_list: list_head,
    pub nodes_pool: [hl_cs_outcome; HL_CS_OUTCOME_HISTORY_LEN],
    pub db_lock: spinlock_t,
}

//
// struct hl_ctx - user/kernel context.
// @mem_hash: holds mapping from virtual address to virtual memory area
// descriptor (hl_vm_phys_pg_list or hl_userptr).
// @mmu_shadow_hash: holds a mapping from shadow address to pgt_info structure.
// @hr_mmu_phys_hash: if host-resident MMU is used, holds a mapping from
// MMU-hop-page physical address to its host-resident
// pgt_info structure.
// @hpriv: pointer to the private (Kernel Driver) data of the process (fd).
// @hdev: pointer to the device structure.
// @refcount: reference counter for the context. Context is released only when
// this hits 0. It is incremented on CS and CS_WAIT.
// @cs_pending: array of hl fence objects representing pending CS.
// @outcome_store: storage data structure used to remember outcomes of completed
// command submissions for a long time after CS id wraparound.
// @va_range: holds available virtual addresses for host and dram mappings.
// @mem_hash_lock: protects the mem_hash.
// @hw_block_list_lock: protects the HW block memory list.
// @ts_reg_lock: timestamp registration ioctls lock.
// @debugfs_list: node in debugfs list of contexts.
// @hw_block_mem_list: list of HW block virtual mapped addresses.
// @cs_counters: context command submission counters.
// @cb_va_pool: device VA pool for command buffers which are mapped to the
// device's MMU.
// @sig_mgr: encaps signals handle manager.
// @cb_va_pool_base: the base address for the device VA pool
// @cs_sequence: sequence number for CS. Value is assigned to a CS and passed
// to user so user could inquire about CS. It is used as
// index to cs_pending array.
// @dram_default_hops: array that holds all hops addresses needed for default
// DRAM mapping.
// @cs_lock: spinlock to protect cs_sequence.
// @dram_phys_mem: amount of used physical DRAM memory by this context.
// @thread_ctx_switch_token: token to prevent multiple threads of the same
// context	from running the context switch phase.
// Only a single thread should run it.
// @thread_ctx_switch_wait_token: token to prevent the threads that didn't run
// the context switch phase from moving to their
// execution phase before the context switch phase
// has finished.
// @asid: context's unique address space ID in the device's MMU.
// @handle: context's opaque handle for user
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_ctx {
    pub MEM_HASH_TABLE_BITS): DECLARE_HASHTABLE(mem_hash,,
    pub MMU_HASH_TABLE_BITS): DECLARE_HASHTABLE(mmu_shadow_hash,,
    pub MMU_HASH_TABLE_BITS): DECLARE_HASHTABLE(hr_mmu_phys_hash,,
    pub hpriv: *mut hl_fpriv,
    pub hdev: *mut hl_device,
    pub refcount: kref,
    pub cs_pending: *mut hl_fence,
    pub outcome_store: hl_cs_outcome_store,
    pub va_range: [*mut hl_va_range; HL_VA_RANGE_TYPE_MAX],
    pub mem_hash_lock: mutex,
    pub hw_block_list_lock: mutex,
    pub ts_reg_lock: mutex,
    pub debugfs_list: list_head,
    pub hw_block_mem_list: list_head,
    pub cs_counters: hl_cs_counters_atomic,
    pub cb_va_pool: *mut gen_pool,
    pub sig_mgr: hl_encaps_signals_mgr,
    pub cb_va_pool_base: u64,
    pub cs_sequence: u64,
    pub dram_default_hops: *mut u64,
    pub cs_lock: spinlock_t,
    pub dram_phys_mem: core::sync::atomic::AtomicI64,
    pub thread_ctx_switch_token: core::sync::atomic::AtomicI32,
    pub thread_ctx_switch_wait_token: u32,
    pub asid: u32,
    pub handle: u32,
}

//
// struct hl_ctx_mgr - for handling multiple contexts.
// @lock: protects ctx_handles.
// @handles: idr to hold all ctx handles.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_ctx_mgr {
    pub lock: mutex,
    pub handles: idr,
}

//
// COMMAND SUBMISSIONS
//
// struct hl_userptr - memory mapping chunk information
// @vm_type: type of the VM.
// @job_node: linked-list node for hanging the object on the Job's list.
// @pages: pointer to struct page array
// @npages: size of @pages array
// @sgt: pointer to the scatter-gather table that holds the pages.
// @dir: for DMA unmapping, the direction must be supplied, so save it.
// @debugfs_list: node in debugfs list of command submissions.
// @pid: the pid of the user process owning the memory
// @addr: user-space virtual address of the start of the memory area.
// @size: size of the memory area to pin & map.
// @dma_mapped: true if the SG was mapped to DMA addresses, false otherwise.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_userptr {
    pub /: *mut *mut vm_type vm_type; / must be first,
    pub job_node: list_head,
    pub pages: *mut page,
    pub npages: c_uint,
    pub sgt: *mut sg_table,
    pub dir: dma_data_direction,
    pub debugfs_list: list_head,
    pub pid: pid_t,
    pub addr: u64,
    pub size: u64,
    pub dma_mapped: u8,
}

//
// struct hl_cs - command submission.
// @jobs_in_queue_cnt: per each queue, maintain counter of submitted jobs.
// @ctx: the context this CS belongs to.
// @job_list: list of the CS's jobs in the various queues.
// @job_lock: spinlock for the CS's jobs list. Needed for free_job.
// @refcount: reference counter for usage of the CS.
// @fence: pointer to the fence object of this CS.
// @signal_fence: pointer to the fence object of the signal CS (used by wait
// CS only).
// @finish_work: workqueue object to run when CS is completed by H/W.
// @work_tdr: delayed work node for TDR.
// @mirror_node : node in device mirror list of command submissions.
// @staged_cs_node: node in the staged cs list.
// @debugfs_list: node in debugfs list of command submissions.
// @encaps_sig_hdl: holds the encaps signals handle.
// @sequence: the sequence number of this CS.
// @staged_sequence: the sequence of the staged submission this CS is part of,
// relevant only if staged_cs is set.
// @timeout_jiffies: cs timeout in jiffies.
// @submission_time_jiffies: submission time of the cs
// @type: CS_TYPE_*.
// @jobs_cnt: counter of submitted jobs on all queues.
// @encaps_sig_hdl_id: encaps signals handle id, set for the first staged cs.
// @completion_timestamp: timestamp of the last completed cs job.
// @sob_addr_offset: sob offset from the configuration base address.
// @initial_sob_count: count of completed signals in SOB before current submission of signal or
// cs with encaps signals.
// @submitted: true if CS was submitted to H/W.
// @completed: true if CS was completed by device.
// @timedout : true if CS was timedout.
// @tdr_active: true if TDR was activated for this CS (to prevent
// double TDR activation).
// @aborted: true if CS was aborted due to some device error.
// @timestamp: true if a timestamp must be captured upon completion.
// @staged_last: true if this is the last staged CS and needs completion.
// @staged_first: true if this is the first staged CS and we need to receive
// timeout for this CS.
// @staged_cs: true if this CS is part of a staged submission.
// @skip_reset_on_timeout: true if we shall not reset the device in case
// timeout occurs (debug scenario).
// @encaps_signals: true if this CS has encaps reserved signals.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_cs {
    pub jobs_in_queue_cnt: *mut u16,
    pub ctx: *mut hl_ctx,
    pub job_list: list_head,
    pub job_lock: spinlock_t,
    pub refcount: kref,
    pub fence: *mut hl_fence,
    pub signal_fence: *mut hl_fence,
    pub finish_work: work_struct,
    pub work_tdr: delayed_work,
    pub mirror_node: list_head,
    pub staged_cs_node: list_head,
    pub debugfs_list: list_head,
    pub encaps_sig_hdl: *mut hl_cs_encaps_sig_handle,
    pub completion_timestamp: ktime_t,
    pub sequence: u64,
    pub staged_sequence: u64,
    pub timeout_jiffies: u64,
    pub submission_time_jiffies: u64,
    pub type: hl_cs_type,
    pub jobs_cnt: u32,
    pub encaps_sig_hdl_id: u32,
    pub sob_addr_offset: u32,
    pub initial_sob_count: u16,
    pub submitted: u8,
    pub completed: u8,
    pub timedout: u8,
    pub tdr_active: u8,
    pub aborted: u8,
    pub timestamp: u8,
    pub staged_last: u8,
    pub staged_first: u8,
    pub staged_cs: u8,
    pub skip_reset_on_timeout: u8,
    pub encaps_signals: u8,
}

//
// struct hl_cs_job - command submission job.
// @cs_node: the node to hang on the CS jobs list.
// @cs: the CS this job belongs to.
// @user_cb: the CB we got from the user.
// @patched_cb: in case of patching, this is internal CB which is submitted on
// the queue instead of the CB we got from the IOCTL.
// @finish_work: workqueue object to run when job is completed.
// @userptr_list: linked-list of userptr mappings that belong to this job and
// wait for completion.
// @debugfs_list: node in debugfs list of command submission jobs.
// @refcount: reference counter for usage of the CS job.
// @queue_type: the type of the H/W queue this job is submitted to.
// @timestamp: timestamp upon job completion
// @id: the id of this job inside a CS.
// @hw_queue_id: the id of the H/W queue this job is submitted to.
// @user_cb_size: the actual size of the CB we got from the user.
// @job_cb_size: the actual size of the CB that we put on the queue.
// @encaps_sig_wait_offset: encapsulated signals offset, which allow user
// to wait on part of the reserved signals.
// @is_kernel_allocated_cb: true if the CB handle we got from the user holds a
// handle to a kernel-allocated CB object, false
// otherwise (SRAM/DRAM/host address).
// @contains_dma_pkt: whether the JOB contains at least one DMA packet. This
// info is needed later, when adding the 2xMSG_PROT at the
// end of the JOB, to know which barriers to put in the
// MSG_PROT packets. Relevant only for GAUDI as GOYA doesn't
// have streams so the engine can't be busy by another
// stream.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_cs_job {
    pub cs_node: list_head,
    pub cs: *mut hl_cs,
    pub user_cb: *mut hl_cb,
    pub patched_cb: *mut hl_cb,
    pub finish_work: work_struct,
    pub userptr_list: list_head,
    pub debugfs_list: list_head,
    pub refcount: kref,
    pub queue_type: hl_queue_type,
    pub timestamp: ktime_t,
    pub id: u32,
    pub hw_queue_id: u32,
    pub user_cb_size: u32,
    pub job_cb_size: u32,
    pub encaps_sig_wait_offset: u32,
    pub is_kernel_allocated_cb: u8,
    pub contains_dma_pkt: u8,
}

//
// struct hl_cs_parser - command submission parser properties.
// @user_cb: the CB we got from the user.
// @patched_cb: in case of patching, this is internal CB which is submitted on
// the queue instead of the CB we got from the IOCTL.
// @job_userptr_list: linked-list of userptr mappings that belong to the related
// job and wait for completion.
// @cs_sequence: the sequence number of the related CS.
// @queue_type: the type of the H/W queue this job is submitted to.
// @ctx_id: the ID of the context the related CS belongs to.
// @hw_queue_id: the id of the H/W queue this job is submitted to.
// @user_cb_size: the actual size of the CB we got from the user.
// @patched_cb_size: the size of the CB after parsing.
// @job_id: the id of the related job inside the related CS.
// @is_kernel_allocated_cb: true if the CB handle we got from the user holds a
// handle to a kernel-allocated CB object, false
// otherwise (SRAM/DRAM/host address).
// @contains_dma_pkt: whether the JOB contains at least one DMA packet. This
// info is needed later, when adding the 2xMSG_PROT at the
// end of the JOB, to know which barriers to put in the
// MSG_PROT packets. Relevant only for GAUDI as GOYA doesn't
// have streams so the engine can't be busy by another
// stream.
// @completion: true if we need completion for this CS.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_cs_parser {
    pub user_cb: *mut hl_cb,
    pub patched_cb: *mut hl_cb,
    pub job_userptr_list: *mut list_head,
    pub cs_sequence: u64,
    pub queue_type: hl_queue_type,
    pub ctx_id: u32,
    pub hw_queue_id: u32,
    pub user_cb_size: u32,
    pub patched_cb_size: u32,
    pub job_id: u8,
    pub is_kernel_allocated_cb: u8,
    pub contains_dma_pkt: u8,
    pub completion: u8,
}

//
// MEMORY STRUCTURE
//
// struct hl_vm_hash_node - hash element from virtual address to virtual
// memory area descriptor (hl_vm_phys_pg_list or
// hl_userptr).
// @node: node to hang on the hash table in context object.
// @vaddr: key virtual address.
// @handle: memory handle for device memory allocation.
// @ptr: value pointer (hl_vm_phys_pg_list or hl_userptr).
// @export_cnt: number of exports from within the VA block.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_vm_hash_node {
    pub node: hlist_node,
    pub vaddr: u64,
    pub handle: u64,
    pub ptr: *mut c_void,
    pub export_cnt: c_int,
}

//
// struct hl_vm_hw_block_list_node - list element from user virtual address to
// HW block id.
// @node: node to hang on the list in context object.
// @ctx: the context this node belongs to.
// @vaddr: virtual address of the HW block.
// @block_size: size of the block.
// @mapped_size: size of the block which is mapped. May change if partial un-mappings are done.
// @id: HW block id (handle).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_vm_hw_block_list_node {
    pub node: list_head,
    pub ctx: *mut hl_ctx,
    pub vaddr: c_ulong,
    pub block_size: u32,
    pub mapped_size: u32,
    pub id: u32,
}

//
// struct hl_vm_phys_pg_pack - physical page pack.
// @vm_type: describes the type of the virtual area descriptor.
// @pages: the physical page array.
// @npages: num physical pages in the pack.
// @total_size: total size of all the pages in this list.
// @node: used to attach to deletion list that is used when all the allocations are cleared
// at the teardown of the context.
// @mapping_cnt: number of shared mappings.
// @asid: the context related to this list.
// @page_size: size of each page in the pack.
// @flags: HL_MEM_* flags related to this list.
// @handle: the provided handle related to this list.
// @offset: offset from the first page.
// @contiguous: is contiguous physical memory.
// @created_from_userptr: is product of host virtual address.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_vm_phys_pg_pack {
    pub /: *mut *mut vm_type vm_type; / must be first,
    pub pages: *mut u64,
    pub npages: u64,
    pub total_size: u64,
    pub node: list_head,
    pub mapping_cnt: core::sync::atomic::AtomicI32,
    pub asid: u32,
    pub page_size: u32,
    pub flags: u32,
    pub handle: u32,
    pub offset: u32,
    pub contiguous: u8,
    pub created_from_userptr: u8,
}

//
// struct hl_vm_va_block - virtual range block information.
// @node: node to hang on the virtual range list in context object.
// @start: virtual range start address.
// @end: virtual range end address.
// @size: virtual range size.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_vm_va_block {
    pub node: list_head,
    pub start: u64,
    pub end: u64,
    pub size: u64,
}

//
// struct hl_vm - virtual memory manager for MMU.
// @dram_pg_pool: pool for DRAM physical pages of 2MB.
// @dram_pg_pool_refcount: reference counter for the pool usage.
// @idr_lock: protects the phys_pg_list_handles.
// @phys_pg_pack_handles: idr to hold all device allocations handles.
// @init_done: whether initialization was done. We need this because VM
// initialization might be skipped during device initialization.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_vm {
    pub dram_pg_pool: *mut gen_pool,
    pub dram_pg_pool_refcount: kref,
    pub idr_lock: spinlock_t,
    pub phys_pg_pack_handles: idr,
    pub init_done: u8,
}

//
// DEBUG, PROFILING STRUCTURE
//
// struct hl_debug_params - Coresight debug parameters.
// @input: pointer to component specific input parameters.
// @output: pointer to component specific output parameters.
// @output_size: size of output buffer.
// @reg_idx: relevant register ID.
// @op: component operation to execute.
// @enable: true if to enable component debugging, false otherwise.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_debug_params {
    pub input: *mut c_void,
    pub output: *mut c_void,
    pub output_size: u32,
    pub reg_idx: u32,
    pub op: u32,
    pub enable: bool,
}

//
// struct hl_notifier_event - holds the notifier data structure
// @eventfd: the event file descriptor to raise the notifications
// @lock: mutex lock to protect the notifier data flows
// @events_mask: indicates the bitmap events
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_notifier_event {
    pub eventfd: *mut eventfd_ctx,
    pub lock: mutex,
    pub events_mask: u64,
}

//
// FILE PRIVATE STRUCTURE
//
// struct hl_fpriv - process information stored in FD private data.
// @hdev: habanalabs device structure.
// @file_priv: pointer to the DRM file private data structure.
// @taskpid: current process ID.
// @ctx: current executing context. TODO: remove for multiple ctx per process
// @ctx_mgr: context manager to handle multiple context for this FD.
// @mem_mgr: manager descriptor for memory exportable via mmap
// @notifier_event: notifier eventfd towards user process
// @debugfs_list: list of relevant ASIC debugfs.
// @dev_node: node in the device list of file private data
// @refcount: number of related contexts.
// @restore_phase_mutex: lock for context switch and restore phase.
// @ctx_lock: protects the pointer to current executing context pointer. TODO: remove for multiple
// ctx per process.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_fpriv {
    pub hdev: *mut hl_device,
    pub file_priv: *mut drm_file,
    pub taskpid: *mut pid,
    pub ctx: *mut hl_ctx,
    pub ctx_mgr: hl_ctx_mgr,
    pub mem_mgr: hl_mem_mgr,
    pub notifier_event: hl_notifier_event,
    pub debugfs_list: list_head,
    pub dev_node: list_head,
    pub refcount: kref,
    pub restore_phase_mutex: mutex,
    pub ctx_lock: mutex,
}

//
// DebugFS
//
// struct hl_info_list - debugfs file ops.
// @name: file name.
// @show: function to output information.
// @write: function to write to the file.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_info_list {
    pub name: *const c_char,
    pub data): *mut *mut *mut int (show)(struct seq_file s, void,
    pub f_pos): *mut size_t count, loff_t,
}

//
// struct hl_debugfs_entry - debugfs dentry wrapper.
// @info_ent: dentry related ops.
// @dev_entry: ASIC specific debugfs manager.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_debugfs_entry {
    pub info_ent: *const hl_info_list,
    pub dev_entry: *mut hl_dbg_device_entry,
}

//
// struct hl_dbg_device_entry - ASIC specific debugfs manager.
// @root: root dentry.
// @hdev: habanalabs device structure.
// @entry_arr: array of available hl_debugfs_entry.
// @file_list: list of available debugfs files.
// @file_mutex: protects file_list.
// @cb_list: list of available CBs.
// @cb_spinlock: protects cb_list.
// @cs_list: list of available CSs.
// @cs_spinlock: protects cs_list.
// @cs_job_list: list of available CB jobs.
// @cs_job_spinlock: protects cs_job_list.
// @userptr_list: list of available userptrs (virtual memory chunk descriptor).
// @userptr_spinlock: protects userptr_list.
// @ctx_mem_hash_list: list of available contexts with MMU mappings.
// @ctx_mem_hash_mutex: protects list of available contexts with MMU mappings.
// @data_dma_blob_desc: data DMA descriptor of blob.
// @mon_dump_blob_desc: monitor dump descriptor of blob.
// @state_dump: data of the system states in case of a bad cs.
// @state_dump_sem: protects state_dump.
// @addr: next address to read/write from/to in read/write32.
// @mmu_addr: next virtual address to translate to physical address in mmu_show.
// @mmu_cap_mask: mmu hw capability mask, to be used in mmu_ack_error.
// @userptr_lookup: the target user ptr to look up for on demand.
// @mmu_asid: ASID to use while translating in mmu_show.
// @state_dump_head: index of the latest state dump
// @i2c_bus: generic u8 debugfs file for bus value to use in i2c_data_read.
// @i2c_addr: generic u8 debugfs file for address value to use in i2c_data_read.
// @i2c_reg: generic u8 debugfs file for register value to use in i2c_data_read.
// @i2c_len: generic u8 debugfs file for length value to use in i2c_data_read.
// @dio_stats: Direct I/O statistics
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_dbg_device_entry {
    pub root: *mut dentry,
    pub hdev: *mut hl_device,
    pub entry_arr: *mut hl_debugfs_entry,
    pub file_list: list_head,
    pub file_mutex: mutex,
    pub cb_list: list_head,
    pub cb_spinlock: spinlock_t,
    pub cs_list: list_head,
    pub cs_spinlock: spinlock_t,
    pub cs_job_list: list_head,
    pub cs_job_spinlock: spinlock_t,
    pub userptr_list: list_head,
    pub userptr_spinlock: spinlock_t,
    pub ctx_mem_hash_list: list_head,
    pub ctx_mem_hash_mutex: mutex,
    pub data_dma_blob_desc: debugfs_blob_wrapper,
    pub mon_dump_blob_desc: debugfs_blob_wrapper,
    pub state_dump: [*mut c_char; HL_STATE_DUMP_HIST_LEN],
    pub state_dump_sem: rw_semaphore,
    pub addr: u64,
    pub mmu_addr: u64,
    pub mmu_cap_mask: u64,
    pub userptr_lookup: u64,
    pub mmu_asid: u32,
    pub state_dump_head: u32,
    pub i2c_bus: u8,
    pub i2c_addr: u8,
    pub i2c_reg: u8,
    pub i2c_len: u8,

    pub dio_stats: hl_dio_stats,

}

//
// struct hl_debugfs_cfg_access_entry - single debugfs config access object, member of
// hl_debugfs_cfg_access.
// @seconds_since_epoch: seconds since January 1, 1970, used for time comparisons.
// @debugfs_type: the debugfs operation requested, can be READ32, WRITE32, READ64 or WRITE64.
// @addr: the requested address to access.
// @valid: if set, this entry has valid data for dumping at interrupt time.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_debugfs_cfg_access_entry {
    pub seconds_since_epoch: ktime_t,
    pub debugfs_type: debugfs_access_type,
    pub addr: u64,
    pub valid: bool,
}

//
// struct hl_debugfs_cfg_access - saves debugfs config region access requests history.
// @cfg_access_list: list of objects describing config region access requests.
// @head: next valid index to add new entry to in cfg_access_list.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_debugfs_cfg_access {
    pub cfg_access_list: [hl_debugfs_cfg_access_entry; HL_DBGFS_CFG_ACCESS_HIST_LEN],
    pub head: u32,
    pub /: *mut *mut spinlock_t lock; / protects head and entries,
}

//
// struct hl_hw_obj_name_entry - single hw object name, member of
// hl_state_dump_specs
// @node: link to the containing hash table
// @name: hw object name
// @id: object identifier
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_hw_obj_name_entry {
    pub node: hlist_node,
    pub name: *const c_char,
    pub id: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hl_state_dump_specs_props {
    SP_SYNC_OBJ_BASE_ADDR,
    SP_NEXT_SYNC_OBJ_ADDR,
    SP_SYNC_OBJ_AMOUNT,
    SP_MON_OBJ_WR_ADDR_LOW,
    SP_MON_OBJ_WR_ADDR_HIGH,
    SP_MON_OBJ_WR_DATA,
    SP_MON_OBJ_ARM_DATA,
    SP_MON_OBJ_STATUS,
    SP_MONITORS_AMOUNT,
    SP_TPC0_CMDQ,
    SP_TPC0_CFG_SO,
    SP_NEXT_TPC,
    SP_MME_CMDQ,
    SP_MME_CFG_SO,
    SP_NEXT_MME,
    SP_DMA_CMDQ,
    SP_DMA_CFG_SO,
    SP_DMA_QUEUES_OFFSET,
    SP_NUM_OF_MME_ENGINES,
    SP_SUB_MME_ENG_NUM,
    SP_NUM_OF_DMA_ENGINES,
    SP_NUM_OF_TPC_ENGINES,
    SP_ENGINE_NUM_OF_QUEUES,
    SP_ENGINE_NUM_OF_STREAMS,
    SP_ENGINE_NUM_OF_FENCES,
    SP_FENCE0_CNT_OFFSET,
    SP_FENCE0_RDATA_OFFSET,
    SP_CP_STS_OFFSET,
    SP_NUM_CORES,

    SP_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hl_sync_engine_type {
    ENGINE_TPC,
    ENGINE_DMA,
    ENGINE_MME,
}

//
// struct hl_mon_state_dump - represents a state dump of a single monitor
// @id: monitor id
// @wr_addr_low: address monitor will write to, low bits
// @wr_addr_high: address monitor will write to, high bits
// @wr_data: data monitor will write
// @arm_data: register value containing monitor configuration
// @status: monitor status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_mon_state_dump {
    pub id: u32,
    pub wr_addr_low: u32,
    pub wr_addr_high: u32,
    pub wr_data: u32,
    pub arm_data: u32,
    pub status: u32,
}

//
// struct hl_sync_to_engine_map_entry - sync object id to engine mapping entry
// @engine_type: type of the engine
// @engine_id: id of the engine
// @sync_id: id of the sync object
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_sync_to_engine_map_entry {
    pub node: hlist_node,
    pub engine_type: hl_sync_engine_type,
    pub engine_id: u32,
    pub sync_id: u32,
}

//
// struct hl_sync_to_engine_map - maps sync object id to associated engine id
// @tb: hash table containing the mapping, each element is of type
// struct hl_sync_to_engine_map_entry
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_sync_to_engine_map {
    pub SYNC_TO_ENGINE_HASH_TABLE_BITS): DECLARE_HASHTABLE(tb,,
}

//
// struct hl_state_dump_specs_funcs - virtual functions used by the state dump
// @gen_sync_to_engine_map: generate a hash map from sync obj id to its engine
// @print_single_monitor: format monitor data as string
// @monitor_valid: return true if given monitor dump is valid
// @print_fences_single_engine: format fences data as string
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_state_dump_specs_funcs {
    pub map): *mut hl_sync_to_engine_map,
    pub mon): *mut hl_mon_state_dump,
    pub mon): *mut *mut int (monitor_valid)(struct hl_mon_state_dump,
    pub offset): *mut *mut size_t size, size_t,
}

//
// struct hl_state_dump_specs - defines ASIC known hw objects names
// @so_id_to_str_tb: sync objects names index table
// @monitor_id_to_str_tb: monitors names index table
// @funcs: virtual functions used for state dump
// @sync_namager_names: readable names for sync manager if available (ex: N_E)
// @props: pointer to a per asic const props array required for state dump
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_state_dump_specs {
    pub OBJ_NAMES_HASH_TABLE_BITS): DECLARE_HASHTABLE(so_id_to_str_tb,,
    pub OBJ_NAMES_HASH_TABLE_BITS): DECLARE_HASHTABLE(monitor_id_to_str_tb,,
    pub funcs: hl_state_dump_specs_funcs,
    pub sync_namager_names: *const *const c_char,
    pub props: *mut i64,
}

//
// DEVICES
//
pub const HL_STR_MAX: c_int = 64;

// Theoretical limit only. A single host can only contain up to 4 or 8 PCIe
// x16 cards. In extreme cases, there are hosts that can accommodate 16 cards.
//
pub const HL_MAX_MINORS: c_int = 256;
//
// Registers read & write functions.
//
extern "C" {
    pub fn hl_rreg(hdev: *mut hl_device, reg: u32) -> u32;
}
extern "C" {
    pub fn hl_wreg(hdev: *mut hl_device, reg: u32, val: u32);
}

// Timeout should be longer when working with simulator but cap the
// increased timeout to some maximum
//

//
// poll array of register addresses.
// condition is satisfied if all registers values match the expected value.
// once some register in the array satisfies the condition it will not be polled again,
// this is done both for efficiency and due to some registers are "clear on read".
// TODO: use read from PCI bar in other places in the code (SW-91406)
//

//
// address in this macro points always to a memory location in the
// host's (server's) memory. That location is updated asynchronously
// either by the direct access of the device or by another core.
//
// To work both in LE and BE architectures, we need to distinguish between the
// two states (device or another core updates the memory location). Therefore,
// if mem_written_by_device is true, the host memory being polled will be
// updated directly by the device. If false, the host memory being polled will
// be updated by host CPU. Required so host knows whether or not the memory
// might need to be byte-swapped before returning value to caller.
//
// On the first 4 polling iterations the macro goes to sleep for short period of
// time that gradually increases and reaches sleep_us on the fifth iteration.
//

// Verify we read updates done by other cores or by device */ \

//
// struct hl_device_reset_work - reset work wrapper.
// @reset_work: reset work to be done.
// @hdev: habanalabs device structure.
// @flags: reset flags.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_device_reset_work {
    pub reset_work: delayed_work,
    pub hdev: *mut hl_device,
    pub flags: u32,
}

//
// struct hl_mmu_hr_pgt_priv - used for holding per-device mmu host-resident
// page-table internal information.
// @mmu_pgt_pool: pool of page tables used by a host-resident MMU for
// allocating hops.
// @mmu_asid_hop0: per-ASID array of host-resident hop0 tables.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_mmu_hr_priv {
    pub mmu_pgt_pool: *mut gen_pool,
    pub mmu_asid_hop0: *mut pgt_info,
}

//
// struct hl_mmu_dr_pgt_priv - used for holding per-device mmu device-resident
// page-table internal information.
// @mmu_pgt_pool: pool of page tables used by MMU for allocating hops.
// @mmu_shadow_hop0: shadow array of hop0 tables.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_mmu_dr_priv {
    pub mmu_pgt_pool: *mut gen_pool,
    pub mmu_shadow_hop0: *mut c_void,
}

//
// struct hl_mmu_priv - used for holding per-device mmu internal information.
// @dr: information on the device-resident MMU, when exists.
// @hr: information on the host-resident MMU, when exists.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_mmu_priv {
    pub dr: hl_mmu_dr_priv,
    pub hr: hl_mmu_hr_priv,
}

//
// struct hl_mmu_per_hop_info - A structure describing one TLB HOP and its entry
// that was created in order to translate a virtual address to a
// physical one.
// @hop_addr: The address of the hop.
// @hop_pte_addr: The address of the hop entry.
// @hop_pte_val: The value in the hop entry.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_mmu_per_hop_info {
    pub hop_addr: u64,
    pub hop_pte_addr: u64,
    pub hop_pte_val: u64,
}

//
// struct hl_mmu_hop_info - A structure describing the TLB hops and their
// hop-entries that were created in order to translate a virtual address to a
// physical one.
// @scrambled_vaddr: The value of the virtual address after scrambling. This
// address replaces the original virtual-address when mapped
// in the MMU tables.
// @unscrambled_paddr: The un-scrambled physical address.
// @hop_info: Array holding the per-hop information used for the translation.
// @used_hops: The number of hops used for the translation.
// @range_type: virtual address range type.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_mmu_hop_info {
    pub scrambled_vaddr: u64,
    pub unscrambled_paddr: u64,
    pub hop_info: [hl_mmu_per_hop_info; MMU_ARCH_6_HOPS],
    pub used_hops: u32,
    pub range_type: hl_va_range_type,
}

//
// struct hl_hr_mmu_funcs - Device related host resident MMU functions.
// @get_hop0_pgt_info: get page table info structure for HOP0.
// @get_pgt_info: get page table info structure for HOP other than HOP0.
// @add_pgt_info: add page table info structure to hash.
// @get_tlb_mapping_params: get mapping parameters needed for getting TLB info for specific mapping.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_hr_mmu_funcs {
    pub ctx): *mut *mut *mut pgt_info (get_hop0_pgt_info)(hl_ctx,
    pub phys_hop_addr): *mut *mut *mut *mut pgt_info (get_pgt_info)(hl_ctx ctx, u64,
    pub phys_addr): *mut *mut *mut *mut void (add_pgt_info)(struct hl_ctx ctx, struct pgt_info pgt_info, dma_addr_t,
    pub is_huge): *mut u64 virt_addr, bool,
}

//
// struct hl_mmu_funcs - Device related MMU functions.
// @init: initialize the MMU module.
// @fini: release the MMU module.
// @ctx_init: Initialize a context for using the MMU module.
// @ctx_fini: disable a ctx from using the mmu module.
// @map: maps a virtual address to physical address for a context.
// @unmap: unmap a virtual address of a context.
// @flush: flush all writes from all cores to reach device MMU.
// @swap_out: marks all mapping of the given context as swapped out.
// @swap_in: marks all mapping of the given context as swapped in.
// @get_tlb_info: returns the list of hops and hop-entries used that were
// created in order to translate the giver virtual address to a
// physical one.
// @hr_funcs: functions specific to host resident MMU.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_mmu_funcs {
    pub hdev): *mut *mut int (init)(struct hl_device,
    pub hdev): *mut *mut void (fini)(struct hl_device,
    pub ctx): *mut *mut int (ctx_init)(struct hl_ctx,
    pub ctx): *mut *mut void (ctx_fini)(struct hl_ctx,
    pub is_dram_addr): bool,
    pub is_dram_addr): *mut *mut *mut int (unmap)(struct hl_ctx ctx, u64 virt_addr, bool,
    pub ctx): *mut *mut void (flush)(struct hl_ctx,
    pub ctx): *mut *mut void (swap_out)(struct hl_ctx,
    pub ctx): *mut *mut void (swap_in)(struct hl_ctx,
    pub hops): *mut *mut *mut int (get_tlb_info)(struct hl_ctx ctx, u64 virt_addr, struct hl_mmu_hop_info,
    pub hr_funcs: hl_hr_mmu_funcs,
}

//
// struct hl_prefetch_work - prefetch work structure handler
// @prefetch_work: actual work struct.
// @ctx: compute context.
// @va: virtual address to pre-fetch.
// @size: pre-fetch size.
// @flags: operation flags.
// @asid: ASID for maintenance operation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_prefetch_work {
    pub prefetch_work: work_struct,
    pub ctx: *mut hl_ctx,
    pub va: u64,
    pub size: u64,
    pub flags: u32,
    pub asid: u32,
}

//
// number of user contexts allowed to call wait_for_multi_cs ioctl in
// parallel
//
pub const MULTI_CS_MAX_USER_CTX: c_int = 2;
//
// struct multi_cs_completion - multi CS wait completion.
// @completion: completion of any of the CS in the list
// @lock: spinlock for the completion structure
// @timestamp: timestamp for the multi-CS completion
// @stream_master_qid_map: bitmap of all stream masters on which the multi-CS
// is waiting
// @used: 1 if in use, otherwise 0
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct multi_cs_completion {
    pub completion: completion,
    pub lock: spinlock_t,
    pub timestamp: i64,
    pub stream_master_qid_map: u32,
    pub used: u8,
}

//
// struct multi_cs_data - internal data for multi CS call
// @ctx: pointer to the context structure
// @fence_arr: array of fences of all CSs
// @seq_arr: array of CS sequence numbers
// @timeout_jiffies: timeout in jiffies for waiting for CS to complete
// @timestamp: timestamp of first completed CS
// @wait_status: wait for CS status
// @completion_bitmap: bitmap of completed CSs (1- completed, otherwise 0)
// @arr_len: fence_arr and seq_arr array length
// @gone_cs: indication of gone CS (1- there was gone CS, otherwise 0)
// @update_ts: update timestamp. 1- update the timestamp, otherwise 0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct multi_cs_data {
    pub ctx: *mut hl_ctx,
    pub fence_arr: *mut hl_fence,
    pub seq_arr: *mut u64,
    pub timeout_jiffies: i64,
    pub timestamp: i64,
    pub wait_status: c_long,
    pub completion_bitmap: u32,
    pub arr_len: u8,
    pub gone_cs: u8,
    pub update_ts: u8,
}

//
// struct hl_clk_throttle_timestamp - current/last clock throttling timestamp
// @start: timestamp taken when 'start' event is received in driver
// @end: timestamp taken when 'end' event is received in driver
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_clk_throttle_timestamp {
    pub start: ktime_t,
    pub end: ktime_t,
}

//
// struct hl_clk_throttle - keeps current/last clock throttling timestamps
// @timestamp: timestamp taken by driver and firmware, index 0 refers to POWER
// index 1 refers to THERMAL
// @lock: protects this structure as it can be accessed from both event queue
// context and info_ioctl context
// @current_reason: bitmask represents the current clk throttling reasons
// @aggregated_reason: bitmask represents aggregated clk throttling reasons since driver load
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_clk_throttle {
    pub timestamp: [hl_clk_throttle_timestamp; HL_CLK_THROTTLE_TYPE_MAX],
    pub lock: mutex,
    pub current_reason: u32,
    pub aggregated_reason: u32,
}

//
// struct user_mapped_block - describes a hw block allowed to be mmapped by user
// @address: physical HW block address
// @size: allowed size for mmap
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_mapped_block {
    pub address: u32,
    pub size: u32,
}

//
// struct cs_timeout_info - info of last CS timeout occurred.
// @timestamp: CS timeout timestamp.
// @write_enable: if set writing to CS parameters in the structure is enabled. otherwise - disabled,
// so the first (root cause) CS timeout will not be overwritten.
// @seq: CS timeout sequence number.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_timeout_info {
    pub timestamp: ktime_t,
    pub write_enable: core::sync::atomic::AtomicI32,
    pub seq: u64,
}

pub const MAX_QMAN_STREAMS_INFO: c_int = 4;
pub const OPCODE_INFO_MAX_ADDR_SIZE: c_int = 8;
//
// struct undefined_opcode_info - info about last undefined opcode error
// @timestamp: timestamp of the undefined opcode error
// @cb_addr_streams: CB addresses (per stream) that are currently exists in the PQ
// entries. In case all streams array entries are
// filled with values, it means the execution was in Lower-CP.
// @cq_addr: the address of the current handled command buffer
// @cq_size: the size of the current handled command buffer
// @cb_addr_streams_len: num of streams - actual len of cb_addr_streams array.
// should be equal to 1 in case of undefined opcode
// in Upper-CP (specific stream) and equal to 4 in case
// of undefined opcode in Lower-CP.
// @engine_id: engine-id that the error occurred on
// @stream_id: the stream id the error occurred on. In case the stream equals to
// MAX_QMAN_STREAMS_INFO it means the error occurred on a Lower-CP.
// @write_enable: if set, writing to undefined opcode parameters in the structure
// is enable so the first (root cause) undefined opcode will not be
// overwritten.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct undefined_opcode_info {
    pub timestamp: ktime_t,
    pub cb_addr_streams: [u64; MAX_QMAN_STREAMS_INFO][OPCODE_INFO_MAX_ADDR_SIZE],
    pub cq_addr: u64,
    pub cq_size: u32,
    pub cb_addr_streams_len: u32,
    pub engine_id: u32,
    pub stream_id: u32,
    pub write_enable: bool,
}

//
// struct page_fault_info - page fault information.
// @page_fault: holds information collected during a page fault.
// @user_mappings: buffer containing user mappings.
// @num_of_user_mappings: number of user mappings.
// @page_fault_detected: if set as 1, then a page-fault was discovered for the
// first time after the driver has finished booting-up.
// Since we're looking for the page-fault's root cause,
// we don't care of the others that might follow it-
// so once changed to 1, it will remain that way.
// @page_fault_info_available: indicates that a page fault info is now available.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct page_fault_info {
    pub page_fault: hl_page_fault_info,
    pub user_mappings: *mut hl_user_mapping,
    pub num_of_user_mappings: u64,
    pub page_fault_detected: core::sync::atomic::AtomicI32,
    pub page_fault_info_available: bool,
}

//
// struct razwi_info - RAZWI information.
// @razwi: holds information collected during a RAZWI
// @razwi_detected: if set as 1, then a RAZWI was discovered for the
// first time after the driver has finished booting-up.
// Since we're looking for the RAZWI's root cause,
// we don't care of the others that might follow it-
// so once changed to 1, it will remain that way.
// @razwi_info_available: indicates that a RAZWI info is now available.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct razwi_info {
    pub razwi: hl_info_razwi_event,
    pub razwi_detected: core::sync::atomic::AtomicI32,
    pub razwi_info_available: bool,
}

//
// struct hw_err_info - HW error information.
// @event: holds information on the event.
// @event_detected: if set as 1, then a HW event was discovered for the
// first time after the driver has finished booting-up.
// currently we assume that only fatal events (that require hard-reset) are
// reported so we don't care of the others that might follow it.
// so once changed to 1, it will remain that way.
// TODO: support multiple events.
// @event_info_available: indicates that a HW event info is now available.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_err_info {
    pub event: hl_info_hw_err_event,
    pub event_detected: core::sync::atomic::AtomicI32,
    pub event_info_available: bool,
}

//
// struct fw_err_info - FW error information.
// @event: holds information on the event.
// @event_detected: if set as 1, then a FW event was discovered for the
// first time after the driver has finished booting-up.
// currently we assume that only fatal events (that require hard-reset) are
// reported so we don't care of the others that might follow it.
// so once changed to 1, it will remain that way.
// TODO: support multiple events.
// @event_info_available: indicates that a HW event info is now available.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_err_info {
    pub event: hl_info_fw_err_event,
    pub event_detected: core::sync::atomic::AtomicI32,
    pub event_info_available: bool,
}

//
// struct engine_err_info - engine error information.
// @event: holds information on the event.
// @event_detected: if set as 1, then an engine event was discovered for the
// first time after the driver has finished booting-up.
// @event_info_available: indicates that an engine event info is now available.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct engine_err_info {
    pub event: hl_info_engine_err_event,
    pub event_detected: core::sync::atomic::AtomicI32,
    pub event_info_available: bool,
}

//
// struct hl_error_info - holds information collected during an error.
// @cs_timeout: CS timeout error information.
// @razwi_info: RAZWI information.
// @undef_opcode: undefined opcode information.
// @page_fault_info: page fault information.
// @hw_err: (fatal) hardware error information.
// @fw_err: firmware error information.
// @engine_err: engine error information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_error_info {
    pub cs_timeout: cs_timeout_info,
    pub razwi_info: razwi_info,
    pub undef_opcode: undefined_opcode_info,
    pub page_fault_info: page_fault_info,
    pub hw_err: hw_err_info,
    pub fw_err: fw_err_info,
    pub engine_err: engine_err_info,
}

//
// struct hl_reset_info - holds current device reset information.
// @lock: lock to protect critical reset flows.
// @compute_reset_cnt: number of compute resets since the driver was loaded.
// @hard_reset_cnt: number of hard resets since the driver was loaded.
// @hard_reset_schedule_flags: hard reset is scheduled to after current compute reset,
// here we hold the hard reset flags.
// @in_reset: is device in reset flow.
// @in_compute_reset: Device is currently in reset but not in hard-reset.
// @needs_reset: true if reset_on_lockup is false and device should be reset
// due to lockup.
// @hard_reset_pending: is there a hard reset work pending.
// @curr_reset_cause: saves an enumerated reset cause when a hard reset is
// triggered, and cleared after it is shared with preboot.
// @prev_reset_trigger: saves the previous trigger which caused a reset, overridden
// with a new value on next reset
// @reset_trigger_repeated: set if device reset is triggered more than once with
// same cause.
// @skip_reset_on_timeout: Skip device reset if CS has timed out, wait for it to
// complete instead.
// @watchdog_active: true if a device release watchdog work is scheduled.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_reset_info {
    pub lock: spinlock_t,
    pub compute_reset_cnt: u32,
    pub hard_reset_cnt: u32,
    pub hard_reset_schedule_flags: u32,
    pub in_reset: u8,
    pub in_compute_reset: u8,
    pub needs_reset: u8,
    pub hard_reset_pending: u8,
    pub curr_reset_cause: u8,
    pub prev_reset_trigger: u8,
    pub reset_trigger_repeated: u8,
    pub skip_reset_on_timeout: u8,
    pub watchdog_active: u8,
}

//
// struct eq_heartbeat_debug_info - stores debug info to be used upon heartbeat failure.
// @last_pq_heartbeat_ts: timestamp of the last test packet that was sent to FW.
// This packet is the trigger in FW to send the EQ heartbeat event.
// @last_eq_heartbeat_ts: timestamp of the last EQ heartbeat event that was received from FW.
// @heartbeat_event_counter: number of heartbeat events received.
// @cpu_queue_id: used to read the queue pi/ci
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eq_heartbeat_debug_info {
    pub last_pq_heartbeat_ts: time64_t,
    pub last_eq_heartbeat_ts: time64_t,
    pub heartbeat_event_counter: u32,
    pub cpu_queue_id: u32,
}

//
// struct hl_device - habanalabs device structure.
// @pdev: pointer to PCI device, can be NULL in case of simulator device.
// @pcie_bar_phys: array of available PCIe bars physical addresses.
// (required only for PCI address match mode)
// @pcie_bar: array of available PCIe bars virtual addresses.
// @rmmio: configuration area address on SRAM.
// @drm: related DRM device.
// @cdev_ctrl: char device for control operations only (INFO IOCTL)
// @dev: related kernel basic device structure.
// @dev_ctrl: related kernel device structure for the control device
// @work_heartbeat: delayed work for CPU-CP is-alive check.
// @device_reset_work: delayed work which performs hard reset
// @device_release_watchdog_work: watchdog work that performs hard reset if user doesn't release
// device upon certain error cases.
// @asic_name: ASIC specific name.
// @asic_type: ASIC specific type.
// @completion_queue: array of hl_cq.
// @user_interrupt: array of hl_user_interrupt. upon the corresponding user
// interrupt, driver will monitor the list of fences
// registered to this interrupt.
// @tpc_interrupt: single TPC interrupt for all TPCs.
// @unexpected_error_interrupt: single interrupt for unexpected user error indication.
// @common_user_cq_interrupt: common user CQ interrupt for all user CQ interrupts.
// upon any user CQ interrupt, driver will monitor the
// list of fences registered to this common structure.
// @common_decoder_interrupt: common decoder interrupt for all user decoder interrupts.
// @shadow_cs_queue: pointer to a shadow queue that holds pointers to
// outstanding command submissions.
// @cq_wq: work queues of completion queues for executing work in process
// context.
// @eq_wq: work queue of event queue for executing work in process context.
// @cs_cmplt_wq: work queue of CS completions for executing work in process
// context.
// @ts_free_obj_wq: work queue for timestamp registration objects release.
// @prefetch_wq: work queue for MMU pre-fetch operations.
// @reset_wq: work queue for device reset procedure.
// @kernel_ctx: Kernel driver context structure.
// @kernel_queues: array of hl_hw_queue.
// @cs_mirror_list: CS mirror list for TDR.
// @cs_mirror_lock: protects cs_mirror_list.
// @kernel_mem_mgr: memory manager for memory buffers with lifespan of driver.
// @event_queue: event queue for IRQ from CPU-CP.
// @dma_pool: DMA pool for small allocations.
// @cpu_accessible_dma_mem: Host <-> CPU-CP shared memory CPU address.
// @cpu_accessible_dma_address: Host <-> CPU-CP shared memory DMA address.
// @cpu_accessible_dma_pool: Host <-> CPU-CP shared memory pool.
// @asid_bitmap: holds used/available ASIDs.
// @asid_mutex: protects asid_bitmap.
// @send_cpu_message_lock: enforces only one message in Host <-> CPU-CP queue.
// @debug_lock: protects critical section of setting debug mode for device
// @mmu_lock: protects the MMU page tables and invalidation h/w. Although the
// page tables are per context, the invalidation h/w is per MMU.
// Therefore, we can't allow multiple contexts (we only have two,
// user and kernel) to access the invalidation h/w at the same time.
// In addition, any change to the PGT, modifying the MMU hash or
// walking the PGT requires talking this lock.
// @asic_prop: ASIC specific immutable properties.
// @asic_funcs: ASIC specific functions.
// @asic_specific: ASIC specific information to use only from ASIC files.
// @vm: virtual memory manager for MMU.
// @hwmon_dev: H/W monitor device.
// @hl_chip_info: ASIC's sensors information.
// @device_status_description: device status description.
// @hl_debugfs: device's debugfs manager.
// @debugfs_cfg_accesses: list of last debugfs config region accesses.
// @cb_pool: list of pre allocated CBs.
// @cb_pool_lock: protects the CB pool.
// @internal_cb_pool_virt_addr: internal command buffer pool virtual address.
// @internal_cb_pool_dma_addr: internal command buffer pool dma address.
// @internal_cb_pool: internal command buffer memory pool.
// @internal_cb_va_base: internal cb pool mmu virtual address base
// @fpriv_list: list of file private data structures. Each structure is created
// when a user opens the device
// @fpriv_ctrl_list: list of file private data structures. Each structure is created
// when a user opens the control device
// @fpriv_list_lock: protects the fpriv_list
// @fpriv_ctrl_list_lock: protects the fpriv_ctrl_list
// @aggregated_cs_counters: aggregated cs counters among all contexts
// @mmu_priv: device-specific MMU data.
// @mmu_func: device-related MMU functions.
// @dec: list of decoder sw instance
// @fw_loader: FW loader manager.
// @pci_mem_region: array of memory regions in the PCI
// @state_dump_specs: constants and dictionaries needed to dump system state.
// @multi_cs_completion: array of multi-CS completion.
// @clk_throttling: holds information about current/previous clock throttling events
// @captured_err_info: holds information about errors.
// @reset_info: holds current device reset information.
// @heartbeat_debug_info: counters used to debug heartbeat failures.
// @hldio: describes habanalabs direct storage interaction interface.
// @irq_affinity_mask: mask of available CPU cores for user and decoder interrupt handling.
// @stream_master_qid_arr: pointer to array with QIDs of master streams.
// @fw_inner_major_ver: the major of current loaded preboot inner version.
// @fw_inner_minor_ver: the minor of current loaded preboot inner version.
// @fw_sw_major_ver: the major of current loaded preboot SW version.
// @fw_sw_minor_ver: the minor of current loaded preboot SW version.
// @fw_sw_sub_minor_ver: the sub-minor of current loaded preboot SW version.
// @dram_used_mem: current DRAM memory consumption.
// @memory_scrub_val: the value to which the dram will be scrubbed to using cb scrub_device_dram
// @timeout_jiffies: device CS timeout value.
// @max_power: the max power of the device, as configured by the sysadmin. This
// value is saved so in case of hard-reset, the driver will restore
// this value and update the F/W after the re-initialization
// @boot_error_status_mask: contains a mask of the device boot error status.
// Each bit represents a different error, according to
// the defines in hl_boot_if.h. If the bit is cleared,
// the error will be ignored by the driver during
// device initialization. Mainly used to debug and
// workaround firmware bugs
// @dram_pci_bar_start: start bus address of PCIe bar towards DRAM.
// @last_successful_open_ktime: timestamp (ktime) of the last successful device open.
// @last_successful_open_jif: timestamp (jiffies) of the last successful
// device open.
// @last_open_session_duration_jif: duration (jiffies) of the last device open
// session.
// @open_counter: number of successful device open operations.
// @fw_poll_interval_usec: FW status poll interval in usec.
// used for CPU boot status
// @fw_comms_poll_interval_usec: FW comms/protocol poll interval in usec.
// used for COMMs protocols cmds(COMMS_STS_*)
// @dram_binning: contains mask of drams that is received from the f/w which indicates which
// drams are binned-out
// @tpc_binning: contains mask of tpc engines that is received from the f/w which indicates which
// tpc engines are binned-out
// @dmabuf_export_cnt: number of dma-buf exporting.
// @card_type: Various ASICs have several card types. This indicates the card
// type of the current device.
// @major: habanalabs kernel driver major.
// @high_pll: high PLL profile frequency.
// @decoder_binning: contains mask of decoder engines that is received from the f/w which
// indicates which decoder engines are binned-out
// @edma_binning: contains mask of edma engines that is received from the f/w which
// indicates which edma engines are binned-out
// @device_release_watchdog_timeout_sec: device release watchdog timeout value in seconds.
// @rotator_binning: contains mask of rotators engines that is received from the f/w
// which indicates which rotator engines are binned-out(Gaudi3 and above).
// @id: device minor.
// @cdev_idx: char device index.
// @cpu_pci_msb_addr: 50-bit extension bits for the device CPU's 40-bit
// addresses.
// @is_in_dram_scrub: true if dram scrub operation is on going.
// @disabled: is device disabled.
// @cpld_shutdown: is cpld shutdown.
// @late_init_done: is late init stage was done during initialization.
// @hwmon_initialized: is H/W monitor sensors was initialized.
// @reset_on_lockup: true if a reset should be done in case of stuck CS, false
// otherwise.
// @dram_default_page_mapping: is DRAM default page mapping enabled.
// @memory_scrub: true to perform device memory scrub in various locations,
// such as context-switch, context close, page free, etc.
// @pmmu_huge_range: is a different virtual addresses range used for PMMU with
// huge pages.
// @init_done: is the initialization of the device done.
// @device_cpu_disabled: is the device CPU disabled (due to timeouts)
// @in_debug: whether the device is in a state where the profiling/tracing infrastructure
// can be used. This indication is needed because in some ASICs we need to do
// specific operations to enable that infrastructure.
// @cdev_sysfs_debugfs_created: were char devices and sysfs/debugfs files created.
// @stop_on_err: true if engines should stop on error.
// @supports_sync_stream: is sync stream supported.
// @sync_stream_queue_idx: helper index for sync stream queues initialization.
// @collective_mon_idx: helper index for collective initialization
// @supports_coresight: is CoreSight supported.
// @supports_cb_mapping: is mapping a CB to the device's MMU supported.
// @process_kill_trial_cnt: number of trials reset thread tried killing
// user processes
// @device_fini_pending: true if device_fini was called and might be
// waiting for the reset thread to finish
// @supports_staged_submission: true if staged submissions are supported
// @device_cpu_is_halted: Flag to indicate whether the device CPU was already
// halted. We can't halt it again because the COMMS
// protocol will throw an error. Relevant only for
// cases where Linux was not loaded to device CPU
// @supports_wait_for_multi_cs: true if wait for multi CS is supported
// @is_compute_ctx_active: Whether there is an active compute context executing.
// @compute_ctx_in_release: true if the current compute context is being released.
// @supports_mmu_prefetch: true if prefetch is supported, otherwise false.
// @reset_upon_device_release: reset the device when the user closes the file descriptor of the
// device.
// @supports_ctx_switch: true if a ctx switch is required upon first submission.
// @support_preboot_binning: true if we support read binning info from preboot.
// @eq_heartbeat_received: indication that eq heartbeat event has received from FW.
// @nic_ports_mask: Controls which NIC ports are enabled. Used only for testing.
// @fw_components: Controls which f/w components to load to the device. There are multiple f/w
// stages and sometimes we want to stop at a certain stage. Used only for testing.
// @mmu_disable: Disable the device MMU(s). Used only for testing.
// @cpu_queues_enable: Whether to enable queues communication vs. the f/w. Used only for testing.
// @pldm: Whether we are running in Palladium environment. Used only for testing.
// @hard_reset_on_fw_events: Whether to do device hard-reset when a fatal event is received from
// the f/w. Used only for testing.
// @bmc_enable: Whether we are running in a box with BMC. Used only for testing.
// @reset_on_preboot_fail: Whether to reset the device if preboot f/w fails to load.
// Used only for testing.
// @heartbeat: Controls if we want to enable the heartbeat mechanism vs. the f/w, which verifies
// that the f/w is always alive. Used only for testing.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_device {
    pub pdev: *mut pci_dev,
    pub pcie_bar_phys: [u64; HL_PCI_NUM_BARS],
    pub pcie_bar: [*mut void __iomem; HL_PCI_NUM_BARS],
    pub rmmio: *mut void __iomem,
    pub drm: drm_device,
    pub cdev_ctrl: cdev,
    pub dev: *mut device,
    pub dev_ctrl: *mut device,
    pub work_heartbeat: delayed_work,
    pub device_reset_work: hl_device_reset_work,
    pub device_release_watchdog_work: hl_device_reset_work,
    pub asic_name: [c_char; HL_STR_MAX],
    pub status: [c_char; HL_DEV_STS_MAX][HL_STR_MAX],
    pub asic_type: hl_asic_type,
    pub completion_queue: *mut hl_cq,
    pub user_interrupt: *mut hl_user_interrupt,
    pub tpc_interrupt: hl_user_interrupt,
    pub unexpected_error_interrupt: hl_user_interrupt,
    pub common_user_cq_interrupt: hl_user_interrupt,
    pub common_decoder_interrupt: hl_user_interrupt,
    pub shadow_cs_queue: *mut hl_cs,
    pub cq_wq: *mut workqueue_struct,
    pub eq_wq: *mut workqueue_struct,
    pub cs_cmplt_wq: *mut workqueue_struct,
    pub ts_free_obj_wq: *mut workqueue_struct,
    pub prefetch_wq: *mut workqueue_struct,
    pub reset_wq: *mut workqueue_struct,
    pub kernel_ctx: *mut hl_ctx,
    pub kernel_queues: *mut hl_hw_queue,
    pub cs_mirror_list: list_head,
    pub cs_mirror_lock: spinlock_t,
    pub kernel_mem_mgr: hl_mem_mgr,
    pub event_queue: hl_eq,
    pub dma_pool: *mut dma_pool,
    pub cpu_accessible_dma_mem: *mut c_void,
    pub cpu_accessible_dma_address: dma_addr_t,
    pub cpu_accessible_dma_pool: *mut gen_pool,
    pub asid_bitmap: *mut c_ulong,
    pub asid_mutex: mutex,
    pub send_cpu_message_lock: mutex,
    pub debug_lock: mutex,
    pub mmu_lock: mutex,
    pub asic_prop: asic_fixed_properties,
    pub asic_funcs: *const hl_asic_funcs,
    pub asic_specific: *mut c_void,
    pub vm: hl_vm,
    pub hwmon_dev: *mut device,
    pub hl_chip_info: *mut hwmon_chip_info,
    pub hl_debugfs: hl_dbg_device_entry,
    pub debugfs_cfg_accesses: hl_debugfs_cfg_access,
    pub cb_pool: list_head,
    pub cb_pool_lock: spinlock_t,
    pub internal_cb_pool_virt_addr: *mut c_void,
    pub internal_cb_pool_dma_addr: dma_addr_t,
    pub internal_cb_pool: *mut gen_pool,
    pub internal_cb_va_base: u64,
    pub fpriv_list: list_head,
    pub fpriv_ctrl_list: list_head,
    pub fpriv_list_lock: mutex,
    pub fpriv_ctrl_list_lock: mutex,
    pub aggregated_cs_counters: hl_cs_counters_atomic,
    pub mmu_priv: hl_mmu_priv,
    pub mmu_func: [hl_mmu_funcs; MMU_NUM_PGT_LOCATIONS],
    pub dec: *mut hl_dec,
    pub fw_loader: fw_load_mgr,
    pub pci_mem_region: [pci_mem_region; PCI_REGION_NUMBER],
    pub state_dump_specs: hl_state_dump_specs,
    pub clk_throttling: hl_clk_throttle,
    pub captured_err_info: hl_error_info,
    pub reset_info: hl_reset_info,
    pub heartbeat_debug_info: eq_heartbeat_debug_info,

    pub hldio: hl_dio,

    pub irq_affinity_mask: cpumask_t,
    pub stream_master_qid_arr: *mut u32,
    pub fw_inner_major_ver: u32,
    pub fw_inner_minor_ver: u32,
    pub fw_sw_major_ver: u32,
    pub fw_sw_minor_ver: u32,
    pub fw_sw_sub_minor_ver: u32,
    pub dram_used_mem: core::sync::atomic::AtomicI64,
    pub memory_scrub_val: u64,
    pub timeout_jiffies: u64,
    pub max_power: u64,
    pub boot_error_status_mask: u64,
    pub dram_pci_bar_start: u64,
    pub last_successful_open_jif: u64,
    pub last_open_session_duration_jif: u64,
    pub open_counter: u64,
    pub fw_poll_interval_usec: u64,
    pub last_successful_open_ktime: ktime_t,
    pub fw_comms_poll_interval_usec: u64,
    pub dram_binning: u64,
    pub tpc_binning: u64,
    pub dmabuf_export_cnt: core::sync::atomic::AtomicI32,
    pub card_type: cpucp_card_types,
    pub major: u32,
    pub high_pll: u32,
    pub decoder_binning: u32,
    pub edma_binning: u32,
    pub device_release_watchdog_timeout_sec: u32,
    pub rotator_binning: u32,
    pub id: u16,
    pub cdev_idx: u16,
    pub cpu_pci_msb_addr: u16,
    pub is_in_dram_scrub: u8,
    pub disabled: u8,
    pub cpld_shutdown: u8,
    pub late_init_done: u8,
    pub hwmon_initialized: u8,
    pub reset_on_lockup: u8,
    pub dram_default_page_mapping: u8,
    pub memory_scrub: u8,
    pub pmmu_huge_range: u8,
    pub init_done: u8,
    pub device_cpu_disabled: u8,
    pub in_debug: u8,
    pub cdev_sysfs_debugfs_created: u8,
    pub stop_on_err: u8,
    pub supports_sync_stream: u8,
    pub sync_stream_queue_idx: u8,
    pub collective_mon_idx: u8,
    pub supports_coresight: u8,
    pub supports_cb_mapping: u8,
    pub process_kill_trial_cnt: u8,
    pub device_fini_pending: u8,
    pub supports_staged_submission: u8,
    pub device_cpu_is_halted: u8,
    pub supports_wait_for_multi_cs: u8,
    pub stream_master_qid_arr_size: u8,
    pub is_compute_ctx_active: u8,
    pub compute_ctx_in_release: u8,
    pub supports_mmu_prefetch: u8,
    pub reset_upon_device_release: u8,
    pub supports_ctx_switch: u8,
    pub support_preboot_binning: u8,
    pub eq_heartbeat_received: u8,
// Parameters for bring-up to be upstreamed
    pub nic_ports_mask: u64,
    pub fw_components: u64,
    pub mmu_disable: u8,
    pub cpu_queues_enable: u8,
    pub pldm: u8,
    pub hard_reset_on_fw_events: u8,
    pub bmc_enable: u8,
    pub reset_on_preboot_fail: u8,
    pub heartbeat: u8,
}

// Retrieve PCI device name in case of a PCI device or dev name in simulator

//
// struct hl_cs_encaps_sig_handle - encapsulated signals handle structure
// @refcount: refcount used to protect removing this id when several
// wait cs are used to wait of the reserved encaps signals.
// @hdev: pointer to habanalabs device structure.
// @hw_sob: pointer to  H/W SOB used in the reservation.
// @ctx: pointer to the user's context data structure
// @cs_seq: staged cs sequence which contains encapsulated signals
// @id: idr handler id to be used to fetch the handler info
// @q_idx: stream queue index
// @pre_sob_val: current SOB value before reservation
// @count: signals number
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_cs_encaps_sig_handle {
    pub refcount: kref,
    pub hdev: *mut hl_device,
    pub hw_sob: *mut hl_hw_sob,
    pub ctx: *mut hl_ctx,
    pub cs_seq: u64,
    pub id: u32,
    pub q_idx: u32,
    pub pre_sob_val: u32,
    pub count: u32,
}

//
// struct hl_info_fw_err_info - firmware error information structure
// @err_type: The type of error detected (or reported).
// @event_mask: Pointer to the event mask to be modified with the detected error flag
// (can be NULL)
// @event_id: The id of the event that reported the error
// (applicable when err_type is HL_INFO_FW_REPORTED_ERR).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_info_fw_err_info {
    pub err_type: hl_info_fw_err_type,
    pub event_mask: *mut u64,
    pub event_id: u16,
}

//
// IOCTLs
//
// typedef hl_ioctl_t - typedef for ioctl function in the driver
// @hpriv: pointer to the FD's private data, which contains state of
// user process
// @data: pointer to the input/output arguments structure of the IOCTL
//
// Return: 0 for success, negative value for error
//
extern "C" {
    pub fn hl_ioctl_t(hpriv: *mut hl_fpriv, data: *mut c_void) -> typedef int;
}
//
// struct hl_ioctl_desc - describes an IOCTL entry of the driver.
// @cmd: the IOCTL code as created by the kernel macros.
// @func: pointer to the driver's function that should be called for this IOCTL.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hl_ioctl_desc {
    pub cmd: c_uint,
    pub func: *mut hl_ioctl_t,
}

//
// Kernel module functions that can be accessed by entire module
//
// hl_get_sg_info() - get number of pages and the DMA address from SG list.
// @sg: the SG list.
// @dma_addr: pointer to DMA address to return.
//
// Calculate the number of consecutive pages described by the SG list. Take the
// offset of the address in the first page, add to it the length and round it up
// to the number of needed pages.
//
// dma_addr = sg_dma_address(sg);
//
// hl_mem_area_inside_range() - Checks whether address+size are inside a range.
// @address: The start address of the area we want to validate.
// @size: The size in bytes of the area we want to validate.
// @range_start_address: The start address of the valid range.
// @range_end_address: The end address of the valid range.
//
// Return: true if the area is inside the valid range, false otherwise.
//
extern "C" {
    pub fn container_of(_arg: ddev, hl_device: struct, _arg: drm) -> return;
}
//
// hl_mem_area_crosses_range() - Checks whether address+size crossing a range.
// @address: The start address of the area we want to validate.
// @size: The size in bytes of the area we want to validate.
// @range_start_address: The start address of the valid range.
// @range_end_address: The end address of the valid range.
//
// Return: true if the area overlaps part or all of the valid range,
// false otherwise.
//
extern "C" {
    pub fn hl_set_dram_bar_default(hdev: *mut hl_device, addr: u64) -> u64;
}
extern "C" {
    pub fn hl_cpu_accessible_dma_pool_free(hdev: *mut hl_device, size: usize, vaddr: *mut c_void);
}
extern "C" {
    pub fn hl_mmap(filp: *mut file, vma: *mut vm_area_struct) -> c_int;
}
extern "C" {
    pub fn hl_device_open(drm: *mut drm_device, file_priv: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn hl_device_release(ddev: *mut drm_device, file_priv: *mut drm_file);
}
extern "C" {
    pub fn hl_device_open_ctrl(inode: *mut inode, filp: *mut file) -> c_int;
}
extern "C" {
    pub fn hl_device_status(hdev: *mut hl_device) -> hl_device_status;
}
extern "C" {
    pub fn hl_device_set_debug_mode(hdev: *mut hl_device, ctx: *mut hl_ctx, enable: bool) -> c_int;
}
extern "C" {
    pub fn hl_hw_queues_create(hdev: *mut hl_device) -> c_int;
}
extern "C" {
    pub fn hl_hw_queues_destroy(hdev: *mut hl_device);
}
extern "C" {
    pub fn hl_hw_queue_schedule_cs(cs: *mut hl_cs) -> c_int;
}
extern "C" {
    pub fn hl_hw_queue_add_ptr(ptr: u32, val: u16) -> u32;
}
extern "C" {
    pub fn hl_hw_queue_inc_ci_kernel(hdev: *mut hl_device, hw_queue_id: u32);
}
extern "C" {
    pub fn hl_hw_queue_update_ci(cs: *mut hl_cs);
}
extern "C" {
    pub fn hl_hw_queue_reset(hdev: *mut hl_device, hard_reset: bool);
}

extern "C" {
    pub fn hl_cq_init(hdev: *mut hl_device, q: *mut hl_cq, hw_queue_id: u32) -> c_int;
}
extern "C" {
    pub fn hl_cq_fini(hdev: *mut hl_device, q: *mut hl_cq);
}
extern "C" {
    pub fn hl_eq_init(hdev: *mut hl_device, q: *mut hl_eq) -> c_int;
}
extern "C" {
    pub fn hl_eq_fini(hdev: *mut hl_device, q: *mut hl_eq);
}
extern "C" {
    pub fn hl_cq_reset(hdev: *mut hl_device, q: *mut hl_cq);
}
extern "C" {
    pub fn hl_eq_reset(hdev: *mut hl_device, q: *mut hl_eq);
}
extern "C" {
    pub fn hl_eq_dump(hdev: *mut hl_device, q: *mut hl_eq);
}
extern "C" {
    pub fn hl_irq_handler_cq(irq: c_int, arg: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn hl_irq_handler_eq(irq: c_int, arg: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn hl_irq_handler_dec_abnrm(irq: c_int, arg: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn hl_irq_user_interrupt_handler(irq: c_int, arg: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn hl_irq_user_interrupt_thread_handler(irq: c_int, arg: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn hl_irq_eq_error_interrupt_thread_handler(irq: c_int, arg: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn hl_cq_inc_ptr(ptr: u32) -> u32;
}
extern "C" {
    pub fn hl_asid_init(hdev: *mut hl_device) -> c_int;
}
extern "C" {
    pub fn hl_asid_fini(hdev: *mut hl_device);
}
extern "C" {
    pub fn hl_asid_alloc(hdev: *mut hl_device) -> c_ulong;
}
extern "C" {
    pub fn hl_asid_free(hdev: *mut hl_device, asid: c_ulong);
}
extern "C" {
    pub fn hl_ctx_create(hdev: *mut hl_device, hpriv: *mut hl_fpriv) -> c_int;
}
extern "C" {
    pub fn hl_ctx_free(hdev: *mut hl_device, ctx: *mut hl_ctx);
}
extern "C" {
    pub fn hl_ctx_init(hdev: *mut hl_device, ctx: *mut hl_ctx, is_kernel_ctx: bool) -> c_int;
}
extern "C" {
    pub fn hl_ctx_do_release(ref: *mut kref);
}
extern "C" {
    pub fn hl_ctx_get(ctx: *mut hl_ctx);
}
extern "C" {
    pub fn hl_ctx_put(ctx: *mut hl_ctx) -> c_int;
}
extern "C" {
    pub fn hl_ctx_mgr_init(mgr: *mut hl_ctx_mgr);
}
extern "C" {
    pub fn hl_ctx_mgr_fini(hdev: *mut hl_device, mgr: *mut hl_ctx_mgr);
}
extern "C" {
    pub fn hl_device_init(hdev: *mut hl_device) -> c_int;
}
extern "C" {
    pub fn hl_device_fini(hdev: *mut hl_device);
}
extern "C" {
    pub fn hl_device_suspend(hdev: *mut hl_device) -> c_int;
}
extern "C" {
    pub fn hl_device_resume(hdev: *mut hl_device) -> c_int;
}
extern "C" {
    pub fn hl_device_reset(hdev: *mut hl_device, flags: u32) -> c_int;
}
extern "C" {
    pub fn hl_device_cond_reset(hdev: *mut hl_device, flags: u32, event_mask: u64) -> c_int;
}
extern "C" {
    pub fn hl_hpriv_get(hpriv: *mut hl_fpriv);
}
extern "C" {
    pub fn hl_hpriv_put(hpriv: *mut hl_fpriv) -> c_int;
}
extern "C" {
    pub fn hl_device_utilization(hdev: *mut hl_device, utilization: *mut u32) -> c_int;
}
extern "C" {
    pub fn hl_notifier_event_send_all(hdev: *mut hl_device, event_mask: u64);
}
extern "C" {
    pub fn hl_sysfs_init(hdev: *mut hl_device) -> c_int;
}
extern "C" {
    pub fn hl_sysfs_fini(hdev: *mut hl_device);
}
extern "C" {
    pub fn hl_hwmon_init(hdev: *mut hl_device) -> c_int;
}
extern "C" {
    pub fn hl_hwmon_fini(hdev: *mut hl_device);
}
extern "C" {
    pub fn hl_hwmon_release_resources(hdev: *mut hl_device);
}
extern "C" {
    pub fn hl_cb_destroy(mmg: *mut hl_mem_mgr, cb_handle: u64) -> c_int;
}
extern "C" {
    pub fn hl_hw_block_mmap(hpriv: *mut hl_fpriv, vma: *mut vm_area_struct) -> c_int;
}
extern "C" {
    pub fn hl_cb_put(cb: *mut hl_cb);
}
extern "C" {
    pub fn hl_cb_pool_init(hdev: *mut hl_device) -> c_int;
}
extern "C" {
    pub fn hl_cb_pool_fini(hdev: *mut hl_device) -> c_int;
}
extern "C" {
    pub fn hl_cb_va_pool_init(ctx: *mut hl_ctx) -> c_int;
}
extern "C" {
    pub fn hl_cb_va_pool_fini(ctx: *mut hl_ctx);
}
extern "C" {
    pub fn hl_cs_rollback_all(hdev: *mut hl_device, skip_wq_flush: bool);
}
extern "C" {
    pub fn hl_sob_reset_error(ref: *mut kref);
}
extern "C" {
    pub fn hl_gen_sob_mask(sob_base: u16, sob_mask: u8, mask: *mut u8) -> c_int;
}
extern "C" {
    pub fn hl_fence_put(fence: *mut hl_fence);
}
extern "C" {
    pub fn hl_fences_put(fence: *mut hl_fence, len: c_int);
}
extern "C" {
    pub fn hl_fence_get(fence: *mut hl_fence);
}
extern "C" {
    pub fn cs_get(cs: *mut hl_cs);
}
extern "C" {
    pub fn cs_needs_completion(cs: *mut hl_cs) -> bool;
}
extern "C" {
    pub fn cs_needs_timeout(cs: *mut hl_cs) -> bool;
}
extern "C" {
    pub fn is_staged_cs_last_exists(hdev: *mut hl_device, cs: *mut hl_cs) -> bool;
}
extern "C" {
    pub fn hl_multi_cs_completion_init(hdev: *mut hl_device);
}
extern "C" {
    pub fn hl_get_active_cs_num(hdev: *mut hl_device) -> u32;
}
extern "C" {
    pub fn goya_set_asic_funcs(hdev: *mut hl_device);
}
extern "C" {
    pub fn gaudi_set_asic_funcs(hdev: *mut hl_device);
}
extern "C" {
    pub fn gaudi2_set_asic_funcs(hdev: *mut hl_device);
}
extern "C" {
    pub fn hl_vm_ctx_init(ctx: *mut hl_ctx) -> c_int;
}
extern "C" {
    pub fn hl_vm_ctx_fini(ctx: *mut hl_ctx);
}
extern "C" {
    pub fn hl_vm_init(hdev: *mut hl_device) -> c_int;
}
extern "C" {
    pub fn hl_vm_fini(hdev: *mut hl_device);
}
extern "C" {
    pub fn hl_hw_block_mem_init(ctx: *mut hl_ctx);
}
extern "C" {
    pub fn hl_hw_block_mem_fini(ctx: *mut hl_ctx);
}
extern "C" {
    pub fn hl_unpin_host_memory(hdev: *mut hl_device, userptr: *mut hl_userptr);
}
extern "C" {
    pub fn hl_mmu_init(hdev: *mut hl_device) -> c_int;
}
extern "C" {
    pub fn hl_mmu_fini(hdev: *mut hl_device);
}
extern "C" {
    pub fn hl_mmu_ctx_init(ctx: *mut hl_ctx) -> c_int;
}
extern "C" {
    pub fn hl_mmu_ctx_fini(ctx: *mut hl_ctx);
}
extern "C" {
    pub fn hl_mmu_unmap_contiguous(ctx: *mut hl_ctx, virt_addr: u64, size: u32) -> c_int;
}
extern "C" {
    pub fn hl_mmu_invalidate_cache(hdev: *mut hl_device, is_hard: bool, flags: u32) -> c_int;
}
extern "C" {
    pub fn hl_mmu_prefetch_cache_range(ctx: *mut hl_ctx, flags: u32, asid: u32, va: u64, size: u64) -> c_int;
}
extern "C" {
    pub fn hl_mmu_get_next_hop_addr(ctx: *mut hl_ctx, curr_pte: u64) -> u64;
}
extern "C" {
    pub fn hl_mmu_hr_flush(ctx: *mut hl_ctx);
}
extern "C" {
    pub fn hl_mmu_hr_fini(hdev: *mut hl_device, hr_priv: *mut hl_mmu_hr_priv, hop_table_size: u32);
}
extern "C" {
    pub fn hl_mmu_hr_get_pte(ctx: *mut hl_ctx, hr_func: *mut hl_hr_mmu_funcs, phys_hop_addr: u64);
}
extern "C" {
    pub fn hl_mmu_if_set_funcs(hdev: *mut hl_device) -> c_int;
}
extern "C" {
    pub fn hl_mmu_v1_set_funcs(hdev: *mut hl_device, mmu: *mut hl_mmu_funcs);
}
extern "C" {
    pub fn hl_mmu_v2_set_funcs(hdev: *mut hl_device, mmu: *mut hl_mmu_funcs);
}
extern "C" {
    pub fn hl_mmu_v2_hr_set_funcs(hdev: *mut hl_device, mmu: *mut hl_mmu_funcs);
}
extern "C" {
    pub fn hl_mmu_va_to_pa(ctx: *mut hl_ctx, virt_addr: u64, phys_addr: *mut u64) -> c_int;
}
extern "C" {
    pub fn hl_mmu_scramble_addr(hdev: *mut hl_device, addr: u64) -> u64;
}
extern "C" {
    pub fn hl_mmu_descramble_addr(hdev: *mut hl_device, addr: u64) -> u64;
}
extern "C" {
    pub fn hl_is_dram_va(hdev: *mut hl_device, virt_addr: u64) -> bool;
}
extern "C" {
    pub fn hl_mmu_dr_free_hop(ctx: *mut hl_ctx, hop_addr: u64);
}
extern "C" {
    pub fn hl_mmu_dr_free_pgt_node(ctx: *mut hl_ctx, pgt_info: *mut pgt_info);
}
extern "C" {
    pub fn hl_mmu_dr_get_phys_hop0_addr(ctx: *mut hl_ctx) -> u64;
}
extern "C" {
    pub fn hl_mmu_dr_get_hop0_addr(ctx: *mut hl_ctx) -> u64;
}
extern "C" {
    pub fn hl_mmu_dr_write_pte(ctx: *mut hl_ctx, shadow_pte_addr: u64, val: u64);
}
extern "C" {
    pub fn hl_mmu_dr_write_final_pte(ctx: *mut hl_ctx, shadow_pte_addr: u64, val: u64);
}
extern "C" {
    pub fn hl_mmu_dr_clear_pte(ctx: *mut hl_ctx, pte_addr: u64);
}
extern "C" {
    pub fn hl_mmu_dr_get_phys_addr(ctx: *mut hl_ctx, shadow_addr: u64) -> u64;
}
extern "C" {
    pub fn hl_mmu_dr_get_pte(ctx: *mut hl_ctx, hop_addr: u64);
}
extern "C" {
    pub fn hl_mmu_dr_put_pte(ctx: *mut hl_ctx, hop_addr: u64) -> c_int;
}
extern "C" {
    pub fn hl_mmu_dr_get_alloc_next_hop_addr(ctx: *mut hl_ctx, curr_pte: u64, is_new_hop: *mut bool) -> u64;
}
extern "C" {
    pub fn hl_mmu_dr_alloc_hop(ctx: *mut hl_ctx) -> u64;
}
extern "C" {
    pub fn hl_mmu_dr_flush(ctx: *mut hl_ctx);
}
extern "C" {
    pub fn hl_mmu_dr_init(hdev: *mut hl_device) -> c_int;
}
extern "C" {
    pub fn hl_mmu_dr_fini(hdev: *mut hl_device);
}
extern "C" {
    pub fn hl_fw_version_cmp(hdev: *mut hl_device, major: u32, minor: u32, subminor: u32) -> c_int;
}
extern "C" {
    pub fn hl_fw_send_pci_access_msg(hdev: *mut hl_device, opcode: u32, value: u64) -> c_int;
}
extern "C" {
    pub fn hl_fw_unmask_irq(hdev: *mut hl_device, event_type: u16) -> c_int;
}
extern "C" {
    pub fn hl_fw_test_cpu_queue(hdev: *mut hl_device) -> c_int;
}
extern "C" {
    pub fn hl_fw_send_heartbeat(hdev: *mut hl_device) -> c_int;
}
extern "C" {
    pub fn hl_fw_get_eeprom_data(hdev: *mut hl_device, data: *mut c_void, max_size: usize) -> c_int;
}
extern "C" {
    pub fn hl_fw_get_monitor_dump(hdev: *mut hl_device, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn hl_fw_cpucp_power_get(hdev: *mut hl_device, power: *mut u64) -> c_int;
}
extern "C" {
    pub fn hl_fw_ask_hard_reset_without_linux(hdev: *mut hl_device);
}
extern "C" {
    pub fn hl_fw_ask_halt_machine_without_linux(hdev: *mut hl_device);
}
extern "C" {
    pub fn hl_fw_init_cpu(hdev: *mut hl_device) -> c_int;
}
extern "C" {
    pub fn hl_fw_wait_preboot_ready(hdev: *mut hl_device) -> c_int;
}
extern "C" {
    pub fn hl_fw_read_preboot_status(hdev: *mut hl_device) -> c_int;
}
extern "C" {
    pub fn hl_fw_dram_pending_row_get(hdev: *mut hl_device, pend_rows_num: *mut u32) -> c_int;
}
extern "C" {
    pub fn hl_fw_cpucp_engine_core_asid_set(hdev: *mut hl_device, asid: u32) -> c_int;
}
extern "C" {
    pub fn hl_fw_send_device_activity(hdev: *mut hl_device, open: bool) -> c_int;
}
extern "C" {
    pub fn hl_fw_send_soft_reset(hdev: *mut hl_device) -> c_int;
}
extern "C" {
    pub fn hl_pci_elbi_read(hdev: *mut hl_device, addr: u64, data: *mut u32) -> c_int;
}
extern "C" {
    pub fn hl_pci_iatu_write(hdev: *mut hl_device, addr: u32, data: u32) -> c_int;
}
extern "C" {
    pub fn hl_get_pci_memory_region(hdev: *mut hl_device, addr: u64) -> pci_region;
}
extern "C" {
    pub fn hl_pci_init(hdev: *mut hl_device) -> c_int;
}
extern "C" {
    pub fn hl_pci_fini(hdev: *mut hl_device);
}
extern "C" {
    pub fn hl_fw_get_frequency(hdev: *mut hl_device, pll_index: u32, curr: bool) -> c_long;
}
extern "C" {
    pub fn hl_fw_set_frequency(hdev: *mut hl_device, pll_index: u32, freq: u64);
}
extern "C" {
    pub fn hl_get_temperature(hdev: *mut hl_device, sensor_index: c_int, attr: u32, value: *mut c_long) -> c_int;
}
extern "C" {
    pub fn hl_set_temperature(hdev: *mut hl_device, sensor_index: c_int, attr: u32, value: c_long) -> c_int;
}
extern "C" {
    pub fn hl_get_voltage(hdev: *mut hl_device, sensor_index: c_int, attr: u32, value: *mut c_long) -> c_int;
}
extern "C" {
    pub fn hl_get_current(hdev: *mut hl_device, sensor_index: c_int, attr: u32, value: *mut c_long) -> c_int;
}
extern "C" {
    pub fn hl_get_fan_speed(hdev: *mut hl_device, sensor_index: c_int, attr: u32, value: *mut c_long) -> c_int;
}
extern "C" {
    pub fn hl_get_pwm_info(hdev: *mut hl_device, sensor_index: c_int, attr: u32, value: *mut c_long) -> c_int;
}
extern "C" {
    pub fn hl_set_pwm_info(hdev: *mut hl_device, sensor_index: c_int, attr: u32, value: c_long);
}
extern "C" {
    pub fn hl_fw_get_max_power(hdev: *mut hl_device) -> c_long;
}
extern "C" {
    pub fn hl_fw_set_max_power(hdev: *mut hl_device);
}
extern "C" {
    pub fn hl_set_voltage(hdev: *mut hl_device, sensor_index: c_int, attr: u32, value: c_long) -> c_int;
}
extern "C" {
    pub fn hl_set_current(hdev: *mut hl_device, sensor_index: c_int, attr: u32, value: c_long) -> c_int;
}
extern "C" {
    pub fn hl_set_power(hdev: *mut hl_device, sensor_index: c_int, attr: u32, value: c_long) -> c_int;
}
extern "C" {
    pub fn hl_get_power(hdev: *mut hl_device, sensor_index: c_int, attr: u32, value: *mut c_long) -> c_int;
}
extern "C" {
    pub fn hl_fw_get_clk_rate(hdev: *mut hl_device, cur_clk: *mut u32, max_clk: *mut u32) -> c_int;
}
extern "C" {
    pub fn hl_fw_set_pll_profile(hdev: *mut hl_device);
}
extern "C" {
    pub fn hl_sysfs_add_dev_clk_attr(hdev: *mut hl_device, dev_clk_attr_grp: *mut attribute_group);
}
extern "C" {
    pub fn hl_sysfs_add_dev_vrm_attr(hdev: *mut hl_device, dev_vrm_attr_grp: *mut attribute_group);
}
extern "C" {
    pub fn hw_sob_get(hw_sob: *mut hl_hw_sob);
}
extern "C" {
    pub fn hw_sob_put(hw_sob: *mut hl_hw_sob);
}
extern "C" {
    pub fn hl_encaps_release_handle_and_put_ctx(ref: *mut kref);
}
extern "C" {
    pub fn hl_encaps_release_handle_and_put_sob_ctx(ref: *mut kref);
}
extern "C" {
    pub fn hl_dec_init(hdev: *mut hl_device) -> c_int;
}
extern "C" {
    pub fn hl_dec_fini(hdev: *mut hl_device);
}
extern "C" {
    pub fn hl_dec_ctx_fini(ctx: *mut hl_ctx);
}
extern "C" {
    pub fn hl_release_pending_user_interrupts(hdev: *mut hl_device);
}
extern "C" {
    pub fn hl_abort_waiting_for_cs_completions(hdev: *mut hl_device);
}
extern "C" {
    pub fn hl_state_dump(hdev: *mut hl_device) -> c_int;
}
extern "C" {
    pub fn hl_state_dump_free_sync_to_engine_map(map: *mut hl_sync_to_engine_map);
}
extern "C" {
    pub fn hl_mem_mgr_init(dev: *mut device, mmg: *mut hl_mem_mgr);
}
extern "C" {
    pub fn hl_mem_mgr_fini(mmg: *mut hl_mem_mgr, stats: *mut hl_mem_mgr_fini_stats);
}
extern "C" {
    pub fn hl_mem_mgr_idr_destroy(mmg: *mut hl_mem_mgr);
}
extern "C" {
    pub fn hl_mmap_mem_buf_put_handle(mmg: *mut hl_mem_mgr, handle: u64) -> c_int;
}
extern "C" {
    pub fn hl_mmap_mem_buf_put(buf: *mut hl_mmap_mem_buf) -> c_int;
}
extern "C" {
    pub fn hl_capture_page_fault(hdev: *mut hl_device, addr: u64, eng_id: u16, is_pmmu: bool);
}
extern "C" {
    pub fn hl_handle_critical_hw_err(hdev: *mut hl_device, event_id: u16, event_mask: *mut u64);
}
extern "C" {
    pub fn hl_handle_fw_err(hdev: *mut hl_device, info: *mut hl_info_fw_err_info);
}
extern "C" {
    pub fn hl_capture_engine_err(hdev: *mut hl_device, engine_id: u16, error_count: u16);
}
extern "C" {
    pub fn hl_enable_err_info_capture(captured_err_info: *mut hl_error_info);
}
extern "C" {
    pub fn hl_init_cpu_for_irq(hdev: *mut hl_device);
}
extern "C" {
    pub fn hl_set_irq_affinity(hdev: *mut hl_device, irq: c_int);
}
extern "C" {
    pub fn hl_eq_heartbeat_event_handle(hdev: *mut hl_device);
}
extern "C" {
    pub fn hl_handle_clk_change_event(hdev: *mut hl_device, event_type: u16, event_mask: *mut u64);
}
extern "C" {
    pub fn hl_eq_cpld_shutdown_event_handle(hdev: *mut hl_device, event_id: u16, event_mask: *mut u64);
}

extern "C" {
    pub fn hl_debugfs_device_init(hdev: *mut hl_device) -> c_int;
}
extern "C" {
    pub fn hl_debugfs_device_fini(hdev: *mut hl_device);
}
extern "C" {
    pub fn hl_debugfs_add_device(hdev: *mut hl_device);
}
extern "C" {
    pub fn hl_debugfs_add_file(hpriv: *mut hl_fpriv);
}
extern "C" {
    pub fn hl_debugfs_remove_file(hpriv: *mut hl_fpriv);
}
extern "C" {
    pub fn hl_debugfs_add_cb(cb: *mut hl_cb);
}
extern "C" {
    pub fn hl_debugfs_remove_cb(cb: *mut hl_cb);
}
extern "C" {
    pub fn hl_debugfs_add_cs(cs: *mut hl_cs);
}
extern "C" {
    pub fn hl_debugfs_remove_cs(cs: *mut hl_cs);
}
extern "C" {
    pub fn hl_debugfs_add_job(hdev: *mut hl_device, job: *mut hl_cs_job);
}
extern "C" {
    pub fn hl_debugfs_remove_job(hdev: *mut hl_device, job: *mut hl_cs_job);
}
extern "C" {
    pub fn hl_debugfs_add_userptr(hdev: *mut hl_device, userptr: *mut hl_userptr);
}
extern "C" {
    pub fn hl_debugfs_add_ctx_mem_hash(hdev: *mut hl_device, ctx: *mut hl_ctx);
}
extern "C" {
    pub fn hl_debugfs_remove_ctx_mem_hash(hdev: *mut hl_device, ctx: *mut hl_ctx);
}
extern "C" {
    pub fn hl_debugfs_cfg_access_history_dump(hdev: *mut hl_device);
}

// Security
// IOCTLs
extern "C" {
    pub fn hl_ioctl_control(filep: *mut file, cmd: c_uint, arg: c_ulong) -> c_long;
}
extern "C" {
    pub fn hl_info_ioctl(ddev: *mut drm_device, data: *mut c_void, file_priv: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn hl_cb_ioctl(ddev: *mut drm_device, data: *mut c_void, file_priv: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn hl_cs_ioctl(ddev: *mut drm_device, data: *mut c_void, file_priv: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn hl_wait_ioctl(ddev: *mut drm_device, data: *mut c_void, file_priv: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn hl_mem_ioctl(ddev: *mut drm_device, data: *mut c_void, file_priv: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn hl_debug_ioctl(ddev: *mut drm_device, data: *mut c_void, file_priv: *mut drm_file) -> c_int;
}
