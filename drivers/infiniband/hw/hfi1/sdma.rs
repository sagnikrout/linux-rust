//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/hfi1/sdma.h
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
// Copyright(c) 2015 - 2018 Intel Corporation.
//

// Hardware limit
pub const MAX_DESC: c_int = 64;
// Hardware limit for SDMA packet size

pub const SDMA_MAP_NONE: c_int = 0;
pub const SDMA_MAP_SINGLE: c_int = 1;
pub const SDMA_MAP_PAGE: c_int = 2;
pub const SDMA_AHG_VALUE_MASK: c_uint = 0xffff;
pub const SDMA_AHG_VALUE_SHIFT: c_int = 0;
pub const SDMA_AHG_INDEX_MASK: c_uint = 0xf;
pub const SDMA_AHG_INDEX_SHIFT: c_int = 16;
pub const SDMA_AHG_FIELD_LEN_MASK: c_uint = 0xf;
pub const SDMA_AHG_FIELD_LEN_SHIFT: c_int = 20;
pub const SDMA_AHG_FIELD_START_MASK: c_uint = 0x1f;
pub const SDMA_AHG_FIELD_START_SHIFT: c_int = 24;
pub const SDMA_AHG_UPDATE_ENABLE_MASK: c_uint = 0x1;
pub const SDMA_AHG_UPDATE_ENABLE_SHIFT: c_int = 31;
// AHG modes
//
// Be aware the ordering and values
// for SDMA_AHG_APPLY_UPDATE[123]
// are assumed in generating a skip
// count in submit_tx() in sdma.c
//
pub const SDMA_AHG_NO_AHG: c_int = 0;
pub const SDMA_AHG_COPY: c_int = 1;
pub const SDMA_AHG_APPLY_UPDATE1: c_int = 2;
pub const SDMA_AHG_APPLY_UPDATE2: c_int = 3;
pub const SDMA_AHG_APPLY_UPDATE3: c_int = 4;
//
// Bits defined in the send DMA descriptor.
//

pub const SDMA_DESC0_BYTE_COUNT_SHIFT: c_int = 48;
pub const SDMA_DESC0_BYTE_COUNT_WIDTH: c_int = 14;

pub const SDMA_DESC0_PHY_ADDR_SHIFT: c_int = 0;
pub const SDMA_DESC0_PHY_ADDR_WIDTH: c_int = 48;

pub const SDMA_DESC1_HEADER_UPDATE1_SHIFT: c_int = 32;
pub const SDMA_DESC1_HEADER_UPDATE1_WIDTH: c_int = 32;

pub const SDMA_DESC1_HEADER_MODE_SHIFT: c_int = 13;
pub const SDMA_DESC1_HEADER_MODE_WIDTH: c_int = 3;

pub const SDMA_DESC1_HEADER_INDEX_SHIFT: c_int = 8;
pub const SDMA_DESC1_HEADER_INDEX_WIDTH: c_int = 5;

pub const SDMA_DESC1_HEADER_DWS_SHIFT: c_int = 4;
pub const SDMA_DESC1_HEADER_DWS_WIDTH: c_int = 4;

pub const SDMA_DESC1_GENERATION_SHIFT: c_int = 2;
pub const SDMA_DESC1_GENERATION_WIDTH: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sdma_states {
    sdma_state_s00_hw_down,
    sdma_state_s10_hw_start_up_halt_wait,
    sdma_state_s15_hw_start_up_clean_wait,
    sdma_state_s20_idle,
    sdma_state_s30_sw_clean_up_wait,
    sdma_state_s40_hw_clean_up_wait,
    sdma_state_s50_hw_halt_wait,
    sdma_state_s60_idle_halt_wait,
    sdma_state_s80_hw_freeze,
    sdma_state_s82_freeze_sw_clean,
    sdma_state_s99_running,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sdma_events {
    sdma_event_e00_go_hw_down,
    sdma_event_e10_go_hw_start,
    sdma_event_e15_hw_halt_done,
    sdma_event_e25_hw_clean_up_done,
    sdma_event_e30_go_running,
    sdma_event_e40_sw_cleaned,
    sdma_event_e50_hw_cleaned,
    sdma_event_e60_hw_halted,
    sdma_event_e70_go_idle,
    sdma_event_e80_hw_freeze,
    sdma_event_e81_hw_frozen,
    sdma_event_e82_hw_unfreeze,
    sdma_event_e85_link_down,
    sdma_event_e90_sw_halted,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdma_set_state_action {
    pub op_enable:1: unsigned,
    pub op_intenable:1: unsigned,
    pub op_halt:1: unsigned,
    pub op_cleanup:1: unsigned,
    pub go_s99_running_tofalse:1: unsigned,
    pub go_s99_running_totrue:1: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdma_state {
    pub kref: kref,
    pub comp: completion,
    pub current_state: sdma_states,
    pub current_op: unsigned,
    pub go_s99_running: unsigned,
// debugging/development
    pub previous_state: sdma_states,
    pub previous_op: unsigned,
    pub last_event: sdma_events,
}

//
// DOC: sdma exported routines
//
// These sdma routines fit into three categories:
// - The SDMA API for building and submitting packets
// to the ring
//
// - Initialization and tear down routines to buildup
// and tear down SDMA
//
// - ISR entrances to handle interrupts, state changes
// and errors
//
// DOC: sdma PSM/verbs API
//
// The sdma API is designed to be used by both PSM
// and verbs to supply packets to the SDMA ring.
//
// The usage of the API is as follows:
//
// Embed a struct iowait in the QP or
// PQ.  The iowait should be initialized with a
// call to iowait_init().
//
// The user of the API should create an allocation method
// for their version of the txreq. slabs, pre-allocated lists,
// and dma pools can be used.  Once the user's overload of
// the sdma_txreq has been allocated, the sdma_txreq member
// must be initialized with sdma_txinit() or sdma_txinit_ahg().
//
// The txreq must be declared with the sdma_txreq first.
//
// The tx request, once initialized,  is manipulated with calls to
// sdma_txadd_daddr(), sdma_txadd_page(), or sdma_txadd_kvaddr()
// for each disjoint memory location.  It is the user's responsibility
// to understand the packet boundaries and page boundaries to do the
// appropriate number of sdma_txadd_* calls..  The user
// must be prepared to deal with failures from these routines due to
// either memory allocation or dma_mapping failures.
//
// The mapping specifics for each memory location are recorded
// in the tx. Memory locations added with sdma_txadd_page()
// and sdma_txadd_kvaddr() are automatically mapped when added
// to the tx and nmapped as part of the progress processing in the
// SDMA interrupt handling.
//
// sdma_txadd_daddr() is used to add an dma_addr_t memory to the
// tx.   An example of a use case would be a pre-allocated
// set of headers allocated via dma_pool_alloc() or
// dma_alloc_coherent().  For these memory locations, it
// is the responsibility of the user to handle that unmapping.
// (This would usually be at an unload or job termination.)
//
// The routine sdma_send_txreq() is used to submit
// a tx to the ring after the appropriate number of
// sdma_txadd_* have been done.
//
// If it is desired to send a burst of sdma_txreqs, sdma_send_txlist()
// can be used to submit a list of packets.
//
// The user is free to use the link overhead in the struct sdma_txreq as
// long as the tx isn't in flight.
//
// The extreme degenerate case of the number of descriptors
// exceeding the ring size is automatically handled as
// memory locations are added.  An overflow of the descriptor
// array that is part of the sdma_txreq is also automatically
// handled.
//
// DOC: Infrastructure calls
//
// sdma_init() is used to initialize data structures and
// CSRs for the desired number of SDMA engines.
//
// sdma_start() is used to kick the SDMA engines initialized
// with sdma_init().   Interrupts must be enabled at this
// point since aspects of the state machine are interrupt
// driven.
//
// sdma_engine_error() and sdma_engine_interrupt() are
// entrances for interrupts.
//
// sdma_map_init() is for the management of the mapping
// table when the number of vls is changed.
//
// struct hw_sdma_desc - raw 128 bit SDMA descriptor
//
// This is the raw descriptor in the SDMA ring
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_sdma_desc {
// private:  don't use directly
    pub qw: [__le64; 2],
}

//
// struct sdma_engine - Data pertaining to each SDMA engine.
// @dd: a back-pointer to the device data
// @ppd: per port back-pointer
// @imask: mask for irq manipulation
// @idle_mask: mask for determining if an interrupt is due to sdma_idle
//
// This structure has the state for each sdma_engine.
//
// Accessing to non public fields are not supported
// since the private members are subject to change.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdma_engine {
// read mostly
    pub dd: *mut hfi1_devdata,
    pub ppd: *mut hfi1_pportdata,
// private:
    pub tail_csr: *mut void __iomem,
    pub /: *mut *mut u64 imask; / clear interrupt mask,
    pub idle_mask: u64,
    pub progress_mask: u64,
    pub int_mask: u64,
// private:
    pub /: *mut *mut *mut volatile __le64 head_dma; / DMA'ed by chip,
// private:
    pub head_phys: dma_addr_t,
// private:
    pub descq: *mut hw_sdma_desc,
// private:
    pub descq_full_count: unsigned,
    pub tx_ring: *mut sdma_txreq,
// private:
    pub descq_phys: dma_addr_t,
// private
    pub sdma_mask: u32,
// private
    pub state: sdma_state,
// private
    pub cpu: c_int,
// private:
    pub sdma_shift: u8,
// private:
    pub /: *mut *mut u8 this_idx; / zero relative engine,
// protect changes to senddmactrl shadow
    pub senddmactrl_lock: spinlock_t,
// private:
    pub /: *mut *mut u64 p_senddmactrl; / shadow per-engine SendDmaCtrl,
// read/write using tail_lock
    pub ____cacheline_aligned_in_smp: spinlock_t tail_lock,

// private:
    pub tail_sn: u64,

// private:
    pub descq_tail: u32,
// private:
    pub ahg_bits: c_ulong,
// private:
    pub desc_avail: u16,
// private:
    pub tx_tail: u16,
// private:
    pub descq_cnt: u16,
// read/write using head_lock
// private:
    pub ____cacheline_aligned_in_smp: seqlock_t head_lock,

// private:
    pub head_sn: u64,

// private:
    pub descq_head: u32,
// private:
    pub tx_head: u16,
// private:
    pub last_status: u64,
// private
    pub err_cnt: u64,
// private
    pub sdma_int_cnt: u64,
    pub idle_int_cnt: u64,
    pub progress_int_cnt: u64,
// private:
    pub waitlock: seqlock_t,
    pub dmawait: list_head,
// CONFIG SDMA for now, just blindly duplicate
// private:
    pub err_halt_worker: work_struct,
// private
    pub err_progress_check_timer: timer_list,
    pub progress_check_head: u32,
// private:
    pub flush_worker: work_struct,
// protect flush list
    pub flushlist_lock: spinlock_t,
// private:
    pub flushlist: list_head,
    pub cpu_mask: cpumask,
    pub kobj: kobject,
    pub msix_intr: u32,
}

extern "C" {
    pub fn sdma_init(dd: *mut hfi1_devdata, port: u8) -> c_int;
}
extern "C" {
    pub fn sdma_start(dd: *mut hfi1_devdata);
}
extern "C" {
    pub fn sdma_exit(dd: *mut hfi1_devdata);
}
extern "C" {
    pub fn sdma_clean(dd: *mut hfi1_devdata, num_engines: usize);
}
extern "C" {
    pub fn sdma_all_running(dd: *mut hfi1_devdata);
}
extern "C" {
    pub fn sdma_freeze_notify(dd: *mut hfi1_devdata, go_idle: c_int);
}
extern "C" {
    pub fn sdma_freeze(dd: *mut hfi1_devdata);
}
extern "C" {
    pub fn sdma_unfreeze(dd: *mut hfi1_devdata);
}
extern "C" {
    pub fn sdma_wait(dd: *mut hfi1_devdata);
}
//
// sdma_empty() - idle engine test
// @engine: sdma engine
//
// Currently used by verbs as a latency optimization.
//
// Return:
// 1 - empty, 0 - non-empty
//
// Either head_lock or tail lock required to see
// a steady state.
//
// sdma_running() - state suitability test
// @engine: sdma engine
//
// sdma_running probes the internal state to determine if it is suitable
// for submitting packets.
//
// Return:
// 1 - ok to submit, 0 - not ok to submit
//
// sdma_txinit_ahg() - initialize an sdma_txreq struct with AHG
// @tx: tx request to initialize
// @flags: flags to key last descriptor additions
// @tlen: total packet length (pbc + headers + data)
// @ahg_entry: ahg entry to use  (0 - 31)
// @num_ahg: ahg descriptor for first descriptor (0 - 9)
// @ahg: array of AHG descriptors (up to 9 entries)
// @ahg_hlen: number of bytes from ASIC entry to use
// @cb: callback
//
// The allocation of the sdma_txreq and it enclosing structure is user
// dependent.  This routine must be called to initialize the user independent
// fields.
//
// The currently supported flags are SDMA_TXREQ_F_URGENT,
// SDMA_TXREQ_F_AHG_COPY, and SDMA_TXREQ_F_USE_AHG.
//
// SDMA_TXREQ_F_URGENT is used for latency sensitive situations where the
// completion is desired as soon as possible.
//
// SDMA_TXREQ_F_AHG_COPY causes the header in the first descriptor to be
// copied to chip entry. SDMA_TXREQ_F_USE_AHG causes the code to add in
// the AHG descriptors into the first 1 to 3 descriptors.
//
// Completions of submitted requests can be gotten on selected
// txreqs by giving a completion routine callback to sdma_txinit() or
// sdma_txinit_ahg().  The environment in which the callback runs
// can be from an ISR, a tasklet, or a thread, so no sleeping
// kernel routines can be used.   Aspects of the sdma ring may
// be locked so care should be taken with locking.
//
// The callback pointer can be NULL to avoid any callback for the packet
// being submitted. The callback will be provided this tx, a status, and a flag.
//
// The status will be one of SDMA_TXREQ_S_OK, SDMA_TXREQ_S_SENDERROR,
// SDMA_TXREQ_S_ABORTED, or SDMA_TXREQ_S_SHUTDOWN.
//
// The flag, if the is the iowait had been used, indicates the iowait
// sdma_busy count has reached zero.
//
// user data portion of tlen should be precise.   The sdma_txadd_* entrances
// will pad with a descriptor references 1 - 3 bytes when the number of bytes
// specified in tlen have been supplied to the sdma_txreq.
//
// ahg_hlen is used to determine the number of on-chip entry bytes to
// use as the header.   This is for cases where the stored header is
// larger than the header to be used in a packet.  This is typical
// for verbs where an RDMA_WRITE_FIRST is larger than the packet in
// and RDMA_WRITE_MIDDLE.
//
// sdma_txinit() - initialize an sdma_txreq struct (no AHG)
// @tx: tx request to initialize
// @flags: flags to key last descriptor additions
// @tlen: total packet length (pbc + headers + data)
// @cb: callback pointer
//
// The allocation of the sdma_txreq and it enclosing structure is user
// dependent.  This routine must be called to initialize the user
// independent fields.
//
// The currently supported flags is SDMA_TXREQ_F_URGENT.
//
// SDMA_TXREQ_F_URGENT is used for latency sensitive situations where the
// completion is desired as soon as possible.
//
// Completions of submitted requests can be gotten on selected
// txreqs by giving a completion routine callback to sdma_txinit() or
// sdma_txinit_ahg().  The environment in which the callback runs
// can be from an ISR, a tasklet, or a thread, so no sleeping
// kernel routines can be used.   The head size of the sdma ring may
// be locked so care should be taken with locking.
//
// The callback pointer can be NULL to avoid any callback for the packet
// being submitted.
//
// The callback, if non-NULL,  will be provided this tx and a status.  The
// status will be one of SDMA_TXREQ_S_OK, SDMA_TXREQ_S_SENDERROR,
// SDMA_TXREQ_S_ABORTED, or SDMA_TXREQ_S_SHUTDOWN.
//
extern "C" {
    pub fn sdma_txinit_ahg(_arg: tx, _arg: flags, _arg: tlen, _arg: 0, _arg: 0, _arg: NULL, _arg: 0, _arg: cb) -> return;
}
// helpers - don't use
// qw[0] zero; qw[1] first, ahg mode already in from init
// helper to extend txreq
extern "C" {
    pub fn _pad_sdma_tx_descs(: *mut hfi1_devdata, : *mut sdma_txreq) -> c_int;
}
extern "C" {
    pub fn __sdma_txclean(: *mut hfi1_devdata, : *mut sdma_txreq);
}
// helpers used by public routines
// special cases for last
//
// sdma_txadd_page() - add a page to the sdma_txreq
// @dd: the device to use for mapping
// @tx: tx request to which the page is added
// @page: page to map
// @offset: offset within the page
// @len: length in bytes
// @pinning_ctx: context to be stored on struct sdma_desc .pinning_ctx. Not
// added if coalesce buffer is used. E.g. pointer to pinned-page
// cache entry for the sdma_desc.
// @ctx_get: optional function to take reference to @pinning_ctx. Not called if
// @pinning_ctx is NULL.
// @ctx_put: optional function to release reference to @pinning_ctx after
// sdma_desc completes. May be called in interrupt context so must
// not sleep. Not called if @pinning_ctx is NULL.
//
// This is used to add a page/offset/length descriptor.
//
// The mapping/unmapping of the page/offset/len is automatically handled.
//
// Return:
// 0 - success, -ENOSPC - mapping fail, -ENOMEM - couldn't
// extend/coalesce descriptor array
//
// sdma_txadd_daddr() - add a dma address to the sdma_txreq
// @dd: the device to use for mapping
// @tx: sdma_txreq to which the page is added
// @addr: dma address mapped by caller
// @len: length in bytes
//
// This is used to add a descriptor for memory that is already dma mapped.
//
// In this case, there is no unmapping as part of the progress processing for
// this memory location.
//
// Return:
// 0 - success, -ENOMEM - couldn't extend descriptor array
//
// sdma_txadd_kvaddr() - add a kernel virtual address to sdma_txreq
// @dd: the device to use for mapping
// @tx: sdma_txreq to which the page is added
// @kvaddr: the kernel virtual address
// @len: length in bytes
//
// This is used to add a descriptor referenced by the indicated kvaddr and
// len.
//
// The mapping/unmapping of the kvaddr and len is automatically handled.
//
// Return:
// 0 - success, -ENOSPC - mapping fail, -ENOMEM - couldn't extend/coalesce
// descriptor array
//
extern "C" {
    pub fn sdma_ahg_alloc(sde: *mut sdma_engine) -> c_int;
}
extern "C" {
    pub fn sdma_ahg_free(sde: *mut sdma_engine, ahg_index: c_int);
}
//
// sdma_build_ahg - build ahg descriptor
// @data
// @dwindex
// @startbit
// @bits
//
// Build and return a 32 bit descriptor.
//
// sdma_progress - use seq number of detect head progress
// @sde: sdma_engine to check
// @seq: base seq count
// @tx: txreq for which we need to check descriptor availability
//
// This is used in the appropriate spot in the sleep routine
// to check for potential ring progress.  This routine gets the
// seqcount before queuing the iowait structure for progress.
//
// If the seqcount indicates that progress needs to be checked,
// re-submission is detected by checking whether the descriptor
// queue has enough descriptor for the txreq.
//
// for use by interrupt handling
extern "C" {
    pub fn sdma_engine_error(sde: *mut sdma_engine, status: u64);
}
extern "C" {
    pub fn sdma_engine_interrupt(sde: *mut sdma_engine, status: u64);
}
//
// The diagram below details the relationship of the mapping structures
//
// Since the mapping now allows for non-uniform engines per vl, the
// number of engines for a vl is either the vl_engines[vl] or
// a computation based on num_sdma/num_vls:
//
// For example:
// nactual = vl_engines ? vl_engines[vl] : num_sdma/num_vls
//
// n = roundup to next highest power of 2 using nactual
//
// In the case where there are num_sdma/num_vls doesn't divide
// evenly, the extras are added from the last vl downward.
//
// For the case where n > nactual, the engines are assigned
// in a round robin fashion wrapping back to the first engine
// for a particular vl.
//
// dd->sdma_map
// |                                   sdma_map_elem[0]
// |                                +--------------------+
// v                                |       mask         |
// sdma_vl_map                           |--------------------|
// +--------------------------+                   | sde[0] -> eng 1    |
// |    list (RCU)            |                   |--------------------|
// |--------------------------|                 ->| sde[1] -> eng 2    |
// |    mask                  |              --/  |--------------------|
// |--------------------------|            -/     |        *           |
// |    actual_vls (max 8)    |          -/       |--------------------|
// |--------------------------|       --/         | sde[n-1] -> eng n  |
// |    vls (max 8)           |     -/            +--------------------+
// |--------------------------|  --
// |    map[0]                |-
// |--------------------------|                   +---------------------+
// |    map[1]                |---                |       mask          |
// |--------------------------|   \----           |---------------------|
// |           *              |        \--        | sde[0] -> eng 1+n   |
// |           *              |           \----   |---------------------|
// |           *              |                \->| sde[1] -> eng 2+n   |
// |--------------------------|                   |---------------------|
// |   map[vls - 1]           |-                  |         *           |
// +--------------------------+ \-                |---------------------|
// \-              | sde[m-1] -> eng m+n |
// \             +---------------------+
// \-
// \
// \-        +----------------------+
// \-      |       mask           |
// \     |----------------------|
// \-   | sde[0] -> eng 1+m+n  |
// \- |----------------------|
// >| sde[1] -> eng 2+m+n  |
// |----------------------|
// |         *            |
// |----------------------|
// | sde[o-1] -> eng o+m+n|
// +----------------------+
//
// struct sdma_map_elem - mapping for a vl
// @mask - selector mask
// @sde - array of engines for this vl
//
// The mask is used to "mod" the selector
// to produce index into the trailing
// array of sdes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdma_map_elem {
    pub mask: u32,
    pub sde: [*mut sdma_engine; ],
}

//
// struct sdma_map_el - mapping for a vl
// @engine_to_vl - map of an engine to a vl
// @list - rcu head for free callback
// @mask - vl mask to "mod" the vl to produce an index to map array
// @actual_vls - number of vls
// @vls - number of vls rounded to next power of 2
// @map - array of sdma_map_elem entries
//
// This is the parent mapping structure.  The trailing
// members of the struct point to sdma_map_elem entries, which
// in turn point to an array of sde's for that vl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdma_vl_map {
    pub engine_to_vl: [i8; TXE_NUM_SDMA_ENGINES],
    pub list: rcu_head,
    pub mask: u32,
    pub actual_vls: u8,
    pub vls: u8,
    pub map: [*mut sdma_map_elem; ],
}

// slow path
extern "C" {
    pub fn _sdma_engine_progress_schedule(sde: *mut sdma_engine);
}
//
// sdma_engine_progress_schedule() - schedule progress on engine
// @sde: sdma_engine to schedule progress
//
// This is the fast path.
//
extern "C" {
    pub fn sdma_get_cpu_to_sde_map(sde: *mut sdma_engine, buf: *mut c_char) -> isize;
}
extern "C" {
    pub fn sdma_engine_get_vl(sde: *mut sdma_engine) -> c_int;
}
extern "C" {
    pub fn sdma_seqfile_dump_sde(s: *mut seq_file, : *mut sdma_engine);
}

extern "C" {
    pub fn sdma_dumpstate(: *mut sdma_engine);
}

extern "C" {
    pub fn sdma_get_descq_cnt() -> u16;
}
extern "C" {
    pub fn sdma_update_lmc(dd: *mut hfi1_devdata, mask: u64, lid: u32);
}
