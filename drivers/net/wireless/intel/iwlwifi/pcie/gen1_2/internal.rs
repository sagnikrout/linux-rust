//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/pcie/gen1_2/internal.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (C) 2003-2015, 2018-2026 Intel Corporation
// Copyright (C) 2013-2015 Intel Mobile Communications GmbH
// Copyright (C) 2016-2017 Intel Deutschland GmbH
//

// Macro flag: #define __iwl_trans_int_pcie_h__

//
// RX related structures and functions
//
pub const RX_NUM_QUEUES: c_int = 1;
pub const RX_POST_REQ_ALLOC: c_int = 2;
pub const RX_CLAIM_REQ_ALLOC: c_int = 8;
pub const RX_PENDING_WATERMARK: c_int = 16;
pub const FIRST_RX_QUEUE: c_int = 512;
// This file includes the declaration that are internal to the
// trans_pcie layer
//
// struct iwl_rx_mem_buffer - driver-side RX buffer descriptor
// @page_dma: bus address of rxb page
// @page: driver's pointer to the rxb page
// @list: list entry for the membuffer
// @invalid: rxb is in driver ownership - not owned by HW
// @vid: index of this rxb in the global table
// @offset: indicates which offset of the page (in bytes)
// this buffer uses (if multiple RBs fit into one page)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_rx_mem_buffer {
    pub page_dma: dma_addr_t,
    pub page: *mut page,
    pub list: list_head,
    pub offset: u32,
    pub vid: u16,
    pub invalid: bool,
}

// interrupt statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isr_statistics {
    pub hw: u32,
    pub sw: u32,
    pub err_code: u32,
    pub sch: u32,
    pub alive: u32,
    pub rfkill: u32,
    pub ctkill: u32,
    pub wakeup: u32,
    pub rx: u32,
    pub tx: u32,
    pub unhandled: u32,
}

//
// struct iwl_rx_transfer_desc - transfer descriptor
// @addr: ptr to free buffer start address
// @rbid: unique tag of the buffer
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_rx_transfer_desc {
    pub rbid: __le16,
    pub reserved: [__le16; 3],
    pub addr: __le64,
    pub __packed: },

//
// struct iwl_rx_completion_desc - completion descriptor
// @reserved1: reserved
// @rbid: unique tag of the received buffer
// @flags: flags (0: fragmented, all others: reserved)
// @reserved2: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_rx_completion_desc {
    pub reserved1: __le32,
    pub rbid: __le16,
    pub flags: u8,
    pub reserved2: [u8; 25],
    pub __packed: },
//
// struct iwl_rx_completion_desc_bz - Bz completion descriptor
// @rbid: unique tag of the received buffer
// @flags: flags (0: fragmented, all others: reserved)
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_rx_completion_desc_bz {
    pub rbid: __le16,
    pub flags: u8,
    pub reserved: [u8; 1],
    pub __packed: },
//
// struct iwl_rxq - Rx queue
// @id: queue index
// @bd: driver's pointer to buffer of receive buffer descriptors (rbd).
// Address size is 32 bit in pre-9000 devices and 64 bit in 9000 devices.
// In AX210 devices it is a pointer to a list of iwl_rx_transfer_desc's
// @bd_dma: bus address of buffer of receive buffer descriptors (rbd)
// @used_bd: driver's pointer to buffer of used receive buffer descriptors (rbd)
// @used_bd_dma: physical address of buffer of used receive buffer descriptors (rbd)
// @read: Shared index to newest available Rx buffer
// @write: Shared index to oldest written Rx packet
// @write_actual: actual write pointer written to device, since we update in
// blocks of 8 only
// @free_count: Number of pre-allocated buffers in rx_free
// @used_count: Number of RBDs handled to allocator to use for allocation
// @write_actual:
// @rx_free: list of RBDs with allocated RB ready for use
// @rx_used: list of RBDs with no RB attached
// @need_update: flag to indicate we need to update read/write index
// @rb_stts: driver's pointer to receive buffer status
// @rb_stts_dma: bus address of receive buffer status
// @lock: per-queue lock
// @queue: actual rx queue. Not used for multi-rx queue.
// @next_rb_is_fragment: indicates that the previous RB that we handled set
// the fragmented flag, so the next one is still another fragment
// @napi: NAPI struct for this queue
// @queue_size: size of this queue
//
// NOTE:  rx_free and rx_used are used as a FIFO for iwl_rx_mem_buffers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_rxq {
    pub id: c_int,
    pub bd: *mut c_void,
    pub bd_dma: dma_addr_t,
    pub used_bd: *mut c_void,
    pub used_bd_dma: dma_addr_t,
    pub read: u32,
    pub write: u32,
    pub free_count: u32,
    pub used_count: u32,
    pub write_actual: u32,
    pub queue_size: u32,
    pub rx_free: list_head,
    pub rx_used: list_head,
    pub next_rb_is_fragment: bool need_update,,
    pub rb_stts: *mut c_void,
    pub rb_stts_dma: dma_addr_t,
    pub lock: spinlock_t,
    pub napi: napi_struct,
    pub queue: [*mut iwl_rx_mem_buffer; RX_QUEUE_SIZE],
}

//
// struct iwl_rb_allocator - Rx allocator
// @req_pending: number of requests the allcator had not processed yet
// @req_ready: number of requests honored and ready for claiming
// @rbd_allocated: RBDs with pages allocated and ready to be handled to
// the queue. This is a list of &struct iwl_rx_mem_buffer
// @rbd_empty: RBDs with no page attached for allocator use. This is a list
// of &struct iwl_rx_mem_buffer
// @lock: protects the rbd_allocated and rbd_empty lists
// @alloc_wq: work queue for background calls
// @rx_alloc: work struct for background calls
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_rb_allocator {
    pub req_pending: core::sync::atomic::AtomicI32,
    pub req_ready: core::sync::atomic::AtomicI32,
    pub rbd_allocated: list_head,
    pub rbd_empty: list_head,
    pub lock: spinlock_t,
    pub alloc_wq: *mut workqueue_struct,
    pub rx_alloc: work_struct,
}

//
// iwl_get_closed_rb_stts - get closed rb stts from different structs
// @trans: transport pointer (for configuration)
// @rxq: the rxq to get the rb stts from
// Return: last closed RB index
//
extern "C" {
    pub fn le16_to_cpu(_arg: *mut READ_ONCE(rb_stts)) -> return;
}

//
// enum iwl_fw_mon_dbgfs_state - the different states of the monitor_data
// debugfs file
//
// @IWL_FW_MON_DBGFS_STATE_CLOSED: the file is closed.
// @IWL_FW_MON_DBGFS_STATE_OPEN: the file is open.
// @IWL_FW_MON_DBGFS_STATE_DISABLED: the file is disabled, once this state is
// set the file can no longer be used.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_fw_mon_dbgfs_state {
    IWL_FW_MON_DBGFS_STATE_CLOSED,
    IWL_FW_MON_DBGFS_STATE_OPEN,
    IWL_FW_MON_DBGFS_STATE_DISABLED,
}

//
// enum iwl_shared_irq_flags - level of sharing for irq
// @IWL_SHARED_IRQ_NON_RX: interrupt vector serves non rx causes.
// @IWL_SHARED_IRQ_FIRST_RSS: interrupt vector serves first RSS queue.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_shared_irq_flags {
    IWL_SHARED_IRQ_NON_RX		= BIT(0),
    IWL_SHARED_IRQ_FIRST_RSS	= BIT(1),
}

//
// enum iwl_image_response_code - image response values
// @IWL_IMAGE_RESP_DEF: the default value of the register
// @IWL_IMAGE_RESP_SUCCESS: iml was read successfully
// @IWL_IMAGE_RESP_FAIL: iml reading failed
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_image_response_code {
    IWL_IMAGE_RESP_DEF		= 0,
    IWL_IMAGE_RESP_SUCCESS		= 1,
    IWL_IMAGE_RESP_FAIL		= 2,
}

//
// struct cont_rec: continuous recording data structure
// @prev_wr_ptr: the last address that was read in monitor_data
// debugfs file
// @prev_wrap_cnt: the wrap count that was used during the last read in
// monitor_data debugfs file
// @state: the state of monitor_data debugfs file as described
// in &iwl_fw_mon_dbgfs_state enum
// @mutex: locked while reading from monitor_data debugfs file
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cont_rec {
    pub prev_wr_ptr: u32,
    pub prev_wrap_cnt: u32,
    pub state: u8,
// Used to sync monitor_data debugfs file with driver unload flow
    pub mutex: mutex,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_pcie_fw_reset_state {
    FW_RESET_IDLE,
    FW_RESET_REQUESTED,
    FW_RESET_OK,
    FW_RESET_ERROR,
    FW_RESET_TOP_REQUESTED,
}

//
// enum iwl_pcie_imr_status - imr dma transfer state
// @IMR_D2S_IDLE: default value of the dma transfer
// @IMR_D2S_REQUESTED: dma transfer requested
// @IMR_D2S_COMPLETED: dma transfer completed
// @IMR_D2S_ERROR: dma transfer error
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_pcie_imr_status {
    IMR_D2S_IDLE,
    IMR_D2S_REQUESTED,
    IMR_D2S_COMPLETED,
    IMR_D2S_ERROR,
}

//
// The FH will write back to the first TB only, so we need to copy some data
// into the buffer regardless of whether it should be mapped or not.
// This indicates how big the first TB must be to include the scratch buffer
// and the assigned PN.
// Since PN location is 8 bytes at offset 12, it's 20 now.
// If we make it bigger then allocations will be bigger and copy slower, so
// that's probably not useful.
//
pub const IWL_FIRST_TB_SIZE: c_int = 20;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_pcie_txq_entry {
    pub cmd: *mut c_void,
    pub skb: *mut sk_buff,
// buffer to free after command completes
    pub free_buf: *const c_void,
    pub meta: iwl_cmd_meta,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_pcie_first_tb_buf {
    pub buf: [u8; IWL_FIRST_TB_SIZE_ALIGN],
}

//
// struct iwl_txq - Tx Queue for DMA
// @tfds: transmit frame descriptors (DMA memory)
// @first_tb_bufs: start of command headers, including scratch buffers, for
// the writeback -- this is DMA memory and an array holding one buffer
// for each command on the queue
// @first_tb_dma: DMA address for the first_tb_bufs start
// @entries: transmit entries (driver state)
// @lock: queue lock
// @reclaim_lock: reclaim lock
// @stuck_timer: timer that fires if queue gets stuck
// @trans: pointer back to transport (for timer)
// @need_update: indicates need to update read/write index
// @ampdu: true if this queue is an ampdu queue for a specific RA/TID
// @wd_timeout: queue watchdog timeout (jiffies) - per queue
// @frozen: tx stuck queue timer is frozen
// @frozen_expiry_remainder: remember how long until the timer fires
// @block: queue is blocked
// @bc_tbl: byte count table of the queue (relevant only for gen2 transport)
// @write_ptr: 1-st empty entry (index) host_w
// @read_ptr: last used entry (index) host_r
// @dma_addr: physical addr for BDs
// @n_window: safe queue window
// @id: queue id
// @low_mark: low watermark, resume queue if free space more than this
// @high_mark: high watermark, stop queue if free space less than this
// @overflow_q: overflow queue for handling frames that didn't fit on HW queue
// @overflow_tx: need to transmit from overflow
//
// A Tx queue consists of circular buffer of BDs (a.k.a. TFDs, transmit frame
// descriptors) and required locking structures.
//
// Note the difference between TFD_QUEUE_SIZE_MAX and n_window: the hardware
// always assumes 256 descriptors, so TFD_QUEUE_SIZE_MAX is always 256 (unless
// there might be HW changes in the future). For the normal TX
// queues, n_window, which is the size of the software queue data
// is also 256; however, for the command queue, n_window is only
// 32 since we don't need so many commands pending. Since the HW
// still uses 256 BDs for DMA though, TFD_QUEUE_SIZE_MAX stays 256.
// This means that we end up with the following:
// HW entries: | 0 | ... | N * 32 | ... | N * 32 + 31 | ... | 255 |
// SW entries:           | 0      | ... | 31          |
// where N is a number between 0 and 7. This means that the SW
// data is a window overlaid over the HW queue.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_txq {
    pub tfds: *mut c_void,
    pub first_tb_bufs: *mut iwl_pcie_first_tb_buf,
    pub first_tb_dma: dma_addr_t,
    pub entries: *mut iwl_pcie_txq_entry,
// lock for syncing changes on the queue
    pub lock: spinlock_t,
// lock to prevent concurrent reclaim
    pub reclaim_lock: spinlock_t,
    pub frozen_expiry_remainder: c_ulong,
    pub stuck_timer: timer_list,
    pub trans: *mut iwl_trans,
    pub need_update: bool,
    pub frozen: bool,
    pub ampdu: bool,
    pub block: c_int,
    pub wd_timeout: c_ulong,
    pub overflow_q: sk_buff_head,
    pub bc_tbl: iwl_dma_ptr,
    pub write_ptr: c_int,
    pub read_ptr: c_int,
    pub dma_addr: dma_addr_t,
    pub n_window: c_int,
    pub id: u32,
    pub low_mark: c_int,
    pub high_mark: c_int,
    pub overflow_tx: bool,
}

//
// struct iwl_pcie_txqs - TX queues data
//
// @queue_used: bit mask of used queues
// @queue_stopped: bit mask of stopped queues
// @txq: array of TXQ data structures representing the TXQs
// @scd_bc_tbls: gen1 pointer to the byte count table of the scheduler
// @bc_pool: bytecount DMA allocations pool
// @bc_tbl_size: bytecount table size
// @tso_hdr_page: page allocated (per CPU) for A-MSDU headers when doing TSO
// (and similar usage)
// @tfd: TFD data
// @tfd.max_tbs: max number of buffers per TFD
// @tfd.size: TFD size
// @tfd.addr_size: TFD/TB address size
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_pcie_txqs {
    pub queue_used: [c_ulong; BITS_TO_LONGS(IWL_MAX_TVQM_QUEUES)],
    pub queue_stopped: [c_ulong; BITS_TO_LONGS(IWL_MAX_TVQM_QUEUES)],
    pub txq: [*mut iwl_txq; IWL_MAX_TVQM_QUEUES],
    pub bc_pool: *mut dma_pool,
    pub bc_tbl_size: usize,
    pub tso_hdr_page: *mut iwl_tso_hdr_page __percpu,
    pub max_tbs: u8,
    pub size: u16,
    pub addr_size: u8,
    pub tfd: },
    pub scd_bc_tbls: iwl_dma_ptr,
}

//
// struct iwl_trans_pcie - PCIe transport specific data
// @rxq: all the RX queue data
// @rx_pool: initial pool of iwl_rx_mem_buffer for all the queues
// @global_table: table mapping received VID from hw to rxb
// @rba: allocator for RX replenishing
// @ctxt_info: context information for FW self init
// @ctxt_info_v2: context information for v1 devices
// @prph_info: prph info for self init
// @prph_scratch: prph scratch for self init
// @ctxt_info_dma_addr: dma addr of context information
// @prph_info_dma_addr: dma addr of prph info
// @prph_scratch_dma_addr: dma addr of prph scratch
// @ctxt_info_dma_addr: dma addr of context information
// @iml: image loader image virtual address
// @iml_len: image loader image size
// @iml_dma_addr: image loader image DMA address
// @trans: pointer to the generic transport area
// @scd_base_addr: scheduler sram base address in SRAM
// @kw: keep warm address
// @pnvm_data: holds info about pnvm payloads allocated in DRAM
// @reduced_tables_data: holds info about power reduced tablse
// payloads allocated in DRAM
// @pci_dev: basic pci-network driver stuff
// @hw_base: pci hardware address support
// @ucode_write_complete: indicates that the ucode has been copied.
// @ucode_write_waitq: wait queue for uCode load
// @rx_page_order: page order for receive buffer size
// @rx_buf_bytes: RX buffer (RB) size in bytes
// @reg_lock: protect hw register access
// @mutex: to protect stop_device / start_fw / start_hw
// @fw_mon_data: fw continuous recording data
// @cmd_hold_nic_awake: indicates NIC is held awake for APMG workaround
// during commands in flight
// @msix_entries: array of MSI-X entries
// @msix_enabled: true if managed to enable MSI-X
// @shared_vec_mask: the type of causes the shared vector handles
// (see iwl_shared_irq_flags).
// @alloc_vecs: the number of interrupt vectors allocated by the OS
// @def_irq: default irq for non rx causes
// @fh_init_mask: initial unmasked fh causes
// @hw_init_mask: initial unmasked hw causes
// @fh_mask: current unmasked fh causes
// @hw_mask: current unmasked hw causes
// @in_rescan: true if we have triggered a device rescan
// @base_rb_stts: base virtual address of receive buffer status for all queues
// @base_rb_stts_dma: base physical address of receive buffer status
// @supported_dma_mask: DMA mask to validate the actual address against,
// will be DMA_BIT_MASK(11) or DMA_BIT_MASK(12) depending on the device
// @alloc_page_lock: spinlock for the page allocator
// @alloc_page: allocated page to still use parts of
// @alloc_page_used: how much of the allocated page was already used (bytes)
// @imr_status: imr dma state machine
// @imr_waitq: imr wait queue for dma completion
// @rf_name: name/version of the CRF, if any
// @use_ict: whether or not ICT (interrupt table) is used
// @ict_index: current ICT read index
// @ict_tbl: ICT table pointer
// @ict_tbl_dma: ICT table DMA address
// @inta_mask: interrupt (INT-A) mask
// @irq_lock: lock to synchronize IRQ handling
// @txq_memory: TXQ allocation array
// @sx_waitq: waitqueue for Sx transitions
// @sx_state: state tracking Sx transitions
// @opmode_down: indicates opmode went away
// @num_rx_bufs: number of RX buffers to allocate/use
// @affinity_mask: IRQ affinity mask for each RX queue
// @debug_rfkill: RF-kill debugging state, -1 for unset, 0/1 for radio
// enable/disable
// @fw_reset_state: state of FW reset handshake
// @fw_reset_waitq: waitqueue for FW reset handshake
// @is_down: indicates the NIC is down
// @isr_stats: interrupt statistics
// @napi_dev: (fake) netdev for NAPI registration
// @txqs: transport tx queues data.
// @me_present: WiAMT/CSME is detected as present (1), not present (0)
// or unknown (-1, so can still use it as a boolean safely)
// @me_recheck_wk: worker to recheck WiAMT/CSME presence
// @invalid_tx_cmd: invalid TX command buffer
// @wait_command_queue: wait queue for sync commands
// @dev_cmd_pool: pool for Tx cmd allocation - for internal use only.
// The user should use iwl_trans_{alloc,free}_tx_cmd.
// @dev_cmd_pool_name: name for the TX command allocation pool
// @pm_support: set to true in start_hw if link pm is supported
// @ltr_enabled: set to true if the LTR is enabled
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_trans_pcie {
    pub rxq: *mut iwl_rxq,
    pub rx_pool: *mut iwl_rx_mem_buffer,
    pub global_table: *mut iwl_rx_mem_buffer,
    pub rba: iwl_rb_allocator,
    pub ctxt_info: *mut iwl_context_info,
    pub ctxt_info_v2: *mut iwl_context_info_v2,
}

// INT ICT Table
// pnvm data
// PCI bus related data
// allocator lock for the two values below
// protect hw register

//
// Before sending the interrupt the HW disables it to prevent
// a nested interrupt. This is done by writing 1 to the corresponding
// bit in the mask register. After handling the interrupt, it should be
// re-enabled by clearing this bit. This register is defined as
// write 1 clear (W1C) register, meaning that it's being clear
// by writing 1 to the bit.
//
// Convention: trans API functions: iwl_trans_pcie_XXX
// Other functions: iwl_pcie_XXX
//
extern "C" {
    pub fn iwl_trans_pcie_free(trans: *mut iwl_trans);
}
extern "C" {
    pub fn _iwl_trans_pcie_grab_nic_access(trans: *mut iwl_trans, silent: bool) -> bool;
}
extern "C" {
    pub fn iwl_trans_pcie_check_product_reset_status(pdev: *mut pci_dev);
}
extern "C" {
    pub fn iwl_trans_pcie_check_product_reset_mode(pdev: *mut pci_dev);
}
//
// RX
//
extern "C" {
    pub fn iwl_pcie_rx_init(trans: *mut iwl_trans) -> c_int;
}
extern "C" {
    pub fn iwl_pcie_gen2_rx_init(trans: *mut iwl_trans) -> c_int;
}
extern "C" {
    pub fn iwl_pcie_msix_isr(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn iwl_pcie_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn iwl_pcie_irq_msix_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn iwl_pcie_irq_rx_msix_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn iwl_pcie_rx_stop(trans: *mut iwl_trans) -> c_int;
}
extern "C" {
    pub fn iwl_pcie_rx_free(trans: *mut iwl_trans);
}
extern "C" {
    pub fn iwl_pcie_free_rbs_pool(trans: *mut iwl_trans);
}
extern "C" {
    pub fn iwl_pcie_rx_init_rxb_lists(rxq: *mut iwl_rxq);
}
extern "C" {
    pub fn iwl_pcie_rx_napi_sync(trans: *mut iwl_trans);
}
//
// ICT - interrupt handling
//
extern "C" {
    pub fn iwl_pcie_isr(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn iwl_pcie_alloc_ict(trans: *mut iwl_trans) -> c_int;
}
extern "C" {
    pub fn iwl_pcie_free_ict(trans: *mut iwl_trans);
}
extern "C" {
    pub fn iwl_pcie_reset_ict(trans: *mut iwl_trans);
}
extern "C" {
    pub fn iwl_pcie_disable_ict(trans: *mut iwl_trans);
}
//
// TX / HCMD
//
// We need 2 entries for the TX command and header, and another one might
// be needed for potential data in the SKB's head. The remaining ones can
// be used for frags.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tso_hdr_page {
    pub page: *mut page,
    pub pos: *mut u8,
}

//
// Note that we put this struct *last* in the page. By doing that, we ensure
// that no TB referencing this page can trigger the 32-bit boundary hardware
// bug.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tso_page_info {
    pub dma_addr: dma_addr_t,
    pub next: *mut page,
    pub use_count: refcount_t,
}

extern "C" {
    pub fn iwl_pcie_tx_init(trans: *mut iwl_trans) -> c_int;
}
extern "C" {
    pub fn iwl_pcie_tx_start(trans: *mut iwl_trans);
}
extern "C" {
    pub fn iwl_pcie_tx_stop(trans: *mut iwl_trans) -> c_int;
}
extern "C" {
    pub fn iwl_pcie_tx_free(trans: *mut iwl_trans);
}
extern "C" {
    pub fn iwl_pcie_txq_check_wrptrs(trans: *mut iwl_trans);
}
extern "C" {
    pub fn iwl_trans_pcie_tx_reset(trans: *mut iwl_trans);
}
//
// We need this inline in case dma_addr_t is only 32-bits - since the
// hardware is always 64-bit, the issue can still occur in that case,
// so use u64 for 'phys' here to force the addition in 64-bit.
//
extern "C" {
    pub fn upper_32_bits(len: phys) != upper_32_bits(phys +) -> return;
}
extern "C" {
    pub fn iwl_txq_space(trans: *mut iwl_trans, q: *const iwl_txq) -> c_int;
}
//
// iwl_txq_inc_wrap - increment queue index, wrap back to beginning
// @trans: the transport (for configuration data)
// @index: current index
// Return: the queue index incremented, subject to wrapping
//
// iwl_txq_dec_wrap - decrement queue index, wrap back to end
// @trans: the transport (for configuration data)
// @index: current index
// Return: the queue index decremented, subject to wrapping
//
extern "C" {
    pub fn iwl_txq_log_scd_error(trans: *mut iwl_trans, txq: *mut iwl_txq);
}
extern "C" {
    pub fn iwl_txq_dyn_free(trans: *mut iwl_trans, queue: c_int);
}
extern "C" {
    pub fn iwl_txq_gen2_tx_free(trans: *mut iwl_trans);
}
extern "C" {
    pub fn le16_to_cpu(_arg: tfh_tb->tb_len) -> return;
}
extern "C" {
    pub fn kmem_cache_zalloc(_arg: trans_pcie->dev_cmd_pool, _arg: GFP_ATOMIC) -> return;
}
extern "C" {
    pub fn iwl_pcie_set_q_ptrs(trans: *mut iwl_trans, txq_id: c_int, ptr: c_int);
}
extern "C" {
    pub fn iwl_trans_pcie_wait_txq_empty(trans: *mut iwl_trans, txq_idx: c_int) -> c_int;
}
extern "C" {
    pub fn iwl_trans_pcie_wait_txqs_empty(trans: *mut iwl_trans, txq_bm: u32) -> c_int;
}
//
// Error handling
//
extern "C" {
    pub fn iwl_pcie_dump_csr(trans: *mut iwl_trans);
}
//
// Helpers
//
// disable interrupts from uCode/NIC to host
// acknowledge/clear/reset any interrupts still pending
// from uCode or flow handler (Rx/Tx DMA)
// disable all the interrupt we might use
//
// fh/hw_mask keeps all the unmasked causes.
// Unlike msi, in msix cause is enabled when it is unset.
//
// When we'll receive the ALIVE interrupt, the ISR will call
// iwl_enable_fw_load_int_ctx_info again to set the ALIVE
// interrupt (which is not really needed anymore) but also the
// RX interrupt which will allow us to receive the ALIVE
// notification (which is Rx) and continue the flow.
//
// Leave all the FH causes enabled to get the ALIVE
// notification.
//
// On 9000-series devices this bit isn't enabled by default, so
// when we power down the device we need set the bit to allow it
// to wake up the PCI-E bus for RF-kill interrupts.
//
extern "C" {
    pub fn iwl_pcie_handle_rfkill_irq(trans: *mut iwl_trans, from_irq: bool);
}
extern "C" {
    pub fn iwl_trans_pcie_rf_kill(trans: *mut iwl_trans, state: bool, from_irq: bool);
}

extern "C" {
    pub fn iwl_trans_pcie_dbgfs_register(trans: *mut iwl_trans);
}
extern "C" {
    pub fn iwl_trans_pcie_debugfs_cleanup(trans: *mut iwl_trans);
}

extern "C" {
    pub fn iwl_pcie_rx_allocator_work(data: *mut work_struct);
}
// common trans ops for all generations transports
extern "C" {
    pub fn iwl_pcie_gen1_2_op_mode_enter(trans: *mut iwl_trans);
}
extern "C" {
    pub fn _iwl_trans_pcie_start_hw(trans: *mut iwl_trans) -> c_int;
}
extern "C" {
    pub fn iwl_trans_pcie_start_hw(trans: *mut iwl_trans) -> c_int;
}
extern "C" {
    pub fn iwl_trans_pcie_op_mode_leave(trans: *mut iwl_trans);
}
extern "C" {
    pub fn iwl_trans_pcie_write8(trans: *mut iwl_trans, ofs: u32, val: u8);
}
extern "C" {
    pub fn iwl_trans_pcie_write32(trans: *mut iwl_trans, ofs: u32, val: u32);
}
extern "C" {
    pub fn iwl_trans_pcie_read32(trans: *mut iwl_trans, ofs: u32) -> u32;
}
extern "C" {
    pub fn iwl_trans_pcie_read_prph(trans: *mut iwl_trans, reg: u32) -> u32;
}
extern "C" {
    pub fn iwl_trans_pcie_write_prph(trans: *mut iwl_trans, addr: u32, val: u32);
}
extern "C" {
    pub fn iwl_trans_pcie_sw_reset(trans: *mut iwl_trans, retake_ownership: bool) -> c_int;
}
extern "C" {
    pub fn iwl_trans_pcie_d3_suspend(trans: *mut iwl_trans, reset: bool) -> c_int;
}
extern "C" {
    pub fn iwl_trans_pci_interrupts(trans: *mut iwl_trans, enable: bool);
}
extern "C" {
    pub fn iwl_trans_pcie_sync_nmi(trans: *mut iwl_trans);
}
extern "C" {
    pub fn iwl_trans_pcie_grab_nic_access(trans: *mut iwl_trans) -> bool;
}
extern "C" {
    pub fn iwl_trans_pcie_resched_with_nic_access(trans: *mut iwl_trans);
}
extern "C" {
    pub fn iwl_pcie_alloc_fw_monitor(trans: *mut iwl_trans, max_power: u8);
}
extern "C" {
    pub fn iwl_pcie_gen1_2_remove(trans: *mut iwl_trans);
}
// transport gen 1 exported functions
extern "C" {
    pub fn iwl_trans_pcie_fw_alive(trans: *mut iwl_trans);
}
extern "C" {
    pub fn iwl_trans_pcie_stop_device(trans: *mut iwl_trans);
}
// common functions that are used by gen2 transport
extern "C" {
    pub fn iwl_trans_pcie_gen2_op_mode_leave(trans: *mut iwl_trans);
}
extern "C" {
    pub fn iwl_pcie_gen2_apm_init(trans: *mut iwl_trans) -> c_int;
}
extern "C" {
    pub fn iwl_pcie_apm_config(trans: *mut iwl_trans);
}
extern "C" {
    pub fn iwl_pcie_prepare_card_hw(trans: *mut iwl_trans) -> c_int;
}
extern "C" {
    pub fn iwl_pcie_synchronize_irqs(trans: *mut iwl_trans);
}
extern "C" {
    pub fn iwl_pcie_check_hw_rf_kill(trans: *mut iwl_trans) -> bool;
}
extern "C" {
    pub fn iwl_pcie_apm_stop_master(trans: *mut iwl_trans);
}
extern "C" {
    pub fn iwl_pcie_conf_msix_hw(trans_pcie: *mut iwl_trans_pcie);
}
extern "C" {
    pub fn iwl_pcie_free_dma_ptr(trans: *mut iwl_trans, ptr: *mut iwl_dma_ptr);
}
extern "C" {
    pub fn iwl_pcie_apply_destination(trans: *mut iwl_trans);
}
extern "C" {
    pub fn iwl_pcie_gen1_2_activate_nic(trans: *mut iwl_trans) -> c_int;
}
// transport gen 2 exported functions
extern "C" {
    pub fn iwl_trans_pcie_gen2_fw_alive(trans: *mut iwl_trans);
}
extern "C" {
    pub fn iwl_trans_pcie_gen2_stop_device(trans: *mut iwl_trans);
}
