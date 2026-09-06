//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dmaengine.h
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
// Copyright(c) 2004 - 2006 Intel Corporation. All rights reserved.
//

//
// typedef dma_cookie_t - an opaque DMA cookie
//
// if dma_cookie_t is >0 it's a DMA request cookie, <0 it's an error code
//
pub type dma_cookie_t = i32;
pub const DMA_MIN_COOKIE: c_int = 1;
//
// enum dma_status - DMA transaction status
// @DMA_COMPLETE: transaction completed
// @DMA_IN_PROGRESS: transaction not yet processed
// @DMA_PAUSED: transaction is paused
// @DMA_ERROR: transaction failed
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dma_status {
    DMA_COMPLETE,
    DMA_IN_PROGRESS,
    DMA_PAUSED,
    DMA_ERROR,
    DMA_OUT_OF_ORDER,
}

//
// enum dma_transaction_type - DMA transaction types/indexes
//
// Note: The DMA_ASYNC_TX capability is not to be set by drivers.  It is
// automatically set as dma devices are registered.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dma_transaction_type {
    DMA_MEMCPY,
    DMA_XOR,
    DMA_PQ,
    DMA_XOR_VAL,
    DMA_PQ_VAL,
    DMA_MEMSET,
    DMA_MEMSET_SG,
    DMA_INTERRUPT,
    DMA_PRIVATE,
    DMA_ASYNC_TX,
    DMA_SLAVE,
    DMA_CYCLIC,
    DMA_INTERLEAVE,
    DMA_COMPLETION_NO_ORDER,
    DMA_REPEAT,
    DMA_LOAD_EOT,
// last transaction type for creation of the capabilities mask
    DMA_TX_TYPE_END,
}

//
// enum dma_transfer_direction - dma transfer mode and direction indicator
// @DMA_MEM_TO_MEM: Async/Memcpy mode
// @DMA_MEM_TO_DEV: Slave mode & From Memory to Device
// @DMA_DEV_TO_MEM: Slave mode & From Device to Memory
// @DMA_DEV_TO_DEV: Slave mode & From Device to Device
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dma_transfer_direction {
    DMA_MEM_TO_MEM,
    DMA_MEM_TO_DEV,
    DMA_DEV_TO_MEM,
    DMA_DEV_TO_DEV,
    DMA_TRANS_NONE,
}

//
// Interleaved Transfer Request
// ----------------------------
// A chunk is collection of contiguous bytes to be transferred.
// The gap(in bytes) between two chunks is called inter-chunk-gap(ICG).
// ICGs may or may not change between chunks.
// A FRAME is the smallest series of contiguous {chunk,icg} pairs,
// that when repeated an integral number of times, specifies the transfer.
// A transfer template is specification of a Frame, the number of times
// it is to be repeated and other per-transfer attributes.
//
// Practically, a client driver would have ready a template for each
// type of transfer it is going to need during its lifetime and
// set only 'src_start' and 'dst_start' before submitting the requests.
//
// |      Frame-1        |       Frame-2       | ~ |       Frame-'numf'  |
// |====....==.===...=...|====....==.===...=...| ~ |====....==.===...=...|
//
// ==  Chunk size
// ... ICG
//
// struct data_chunk - Element of scatter-gather list that makes a frame.
// @size: Number of bytes to read from source.
// size_dst := fn(op, size_src), so doesn't mean much for destination.
// @icg: Number of bytes to jump after last src/dst address of this
// chunk and before first src/dst address for next chunk.
// Ignored for dst(assumed 0), if dst_inc is true and dst_sgl is false.
// Ignored for src(assumed 0), if src_inc is true and src_sgl is false.
// @dst_icg: Number of bytes to jump after last dst address of this
// chunk and before the first dst address for next chunk.
// Ignored if dst_inc is true and dst_sgl is false.
// @src_icg: Number of bytes to jump after last src address of this
// chunk and before the first src address for next chunk.
// Ignored if src_inc is true and src_sgl is false.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct data_chunk {
    pub size: usize,
    pub icg: usize,
    pub dst_icg: usize,
    pub src_icg: usize,
}

//
// struct dma_interleaved_template - Template to convey DMAC the transfer pattern
// and attributes.
// @src_start: Bus address of source for the first chunk.
// @dst_start: Bus address of destination for the first chunk.
// @dir: Specifies the type of Source and Destination.
// @src_inc: If the source address increments after reading from it.
// @dst_inc: If the destination address increments after writing to it.
// @src_sgl: If the 'icg' of sgl[] applies to Source (scattered read).
// Otherwise, source is read contiguously (icg ignored).
// Ignored if src_inc is false.
// @dst_sgl: If the 'icg' of sgl[] applies to Destination (scattered write).
// Otherwise, destination is filled contiguously (icg ignored).
// Ignored if dst_inc is false.
// @numf: Number of frames in this template.
// @frame_size: Number of chunks in a frame i.e, size of sgl[].
// @sgl: Array of {chunk,icg} pairs that make up a frame.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_interleaved_template {
    pub src_start: dma_addr_t,
    pub dst_start: dma_addr_t,
    pub dir: dma_transfer_direction,
    pub src_inc: bool,
    pub dst_inc: bool,
    pub src_sgl: bool,
    pub dst_sgl: bool,
    pub numf: usize,
    pub frame_size: usize,
    pub sgl: [data_chunk; ],
}

//
// struct dma_vec - DMA vector
// @addr: Bus address of the start of the vector
// @len: Length in bytes of the DMA vector
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_vec {
    pub addr: dma_addr_t,
    pub len: usize,
}

//
// enum dma_ctrl_flags - DMA flags to augment operation preparation,
// control completion, and communicate status.
// @DMA_PREP_INTERRUPT - trigger an interrupt (callback) upon completion of
// this transaction
// @DMA_CTRL_ACK - if clear, the descriptor cannot be reused until the client
// acknowledges receipt, i.e. has a chance to establish any dependency
// chains
// @DMA_PREP_PQ_DISABLE_P - prevent generation of P while generating Q
// @DMA_PREP_PQ_DISABLE_Q - prevent generation of Q while generating P
// @DMA_PREP_CONTINUE - indicate to a driver that it is reusing buffers as
// sources that were the result of a previous operation, in the case of a PQ
// operation it continues the calculation with new sources
// @DMA_PREP_FENCE - tell the driver that subsequent operations depend
// on the result of this operation
// @DMA_CTRL_REUSE: client can reuse the descriptor and submit again till
// cleared or freed
// @DMA_PREP_CMD: tell the driver that the data passed to DMA API is command
// data and the descriptor should be in different format from normal
// data descriptors.
// @DMA_PREP_REPEAT: tell the driver that the transaction shall be automatically
// repeated when it ends until a transaction is issued on the same channel
// with the DMA_PREP_LOAD_EOT flag set. This flag is only applicable to
// interleaved transactions and is ignored for all other transaction types.
// @DMA_PREP_LOAD_EOT: tell the driver that the transaction shall replace any
// active repeated (as indicated by DMA_PREP_REPEAT) transaction when the
// repeated transaction ends. Not setting this flag when the previously queued
// transaction is marked with DMA_PREP_REPEAT will cause the new transaction
// to never be processed and stay in the issued queue forever. The flag is
// ignored if the previous transaction is not a repeated transaction.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dma_ctrl_flags {
    DMA_PREP_INTERRUPT = (1 << 0),
    DMA_CTRL_ACK = (1 << 1),
    DMA_PREP_PQ_DISABLE_P = (1 << 2),
    DMA_PREP_PQ_DISABLE_Q = (1 << 3),
    DMA_PREP_CONTINUE = (1 << 4),
    DMA_PREP_FENCE = (1 << 5),
    DMA_CTRL_REUSE = (1 << 6),
    DMA_PREP_CMD = (1 << 7),
    DMA_PREP_REPEAT = (1 << 8),
    DMA_PREP_LOAD_EOT = (1 << 9),
}

//
// enum sum_check_bits - bit position of pq_check_flags
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sum_check_bits {
    SUM_CHECK_P = 0,
    SUM_CHECK_Q = 1,
}

//
// enum sum_check_flags - result of async_{xor,pq}_zero_sum operations
// @SUM_CHECK_P_RESULT - 1 if xor zero sum error, 0 otherwise
// @SUM_CHECK_Q_RESULT - 1 if reed-solomon zero sum error, 0 otherwise
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sum_check_flags {
    SUM_CHECK_P_RESULT = (1 << SUM_CHECK_P),
    SUM_CHECK_Q_RESULT = (1 << SUM_CHECK_Q),
}

//
// dma_cap_mask_t - capabilities bitmap modeled after cpumask_t.
// See linux/cpumask.h
//
// enum dma_desc_metadata_mode - per descriptor metadata mode types supported
// @DESC_METADATA_CLIENT - the metadata buffer is allocated/provided by the
// client driver and it is attached (via the dmaengine_desc_attach_metadata()
// helper) to the descriptor.
//
// Client drivers interested to use this mode can follow:
// - DMA_MEM_TO_DEV / DEV_MEM_TO_MEM:
// 1. prepare the descriptor (dmaengine_prep_*)
// construct the metadata in the client's buffer
// 2. use dmaengine_desc_attach_metadata() to attach the buffer to the
// descriptor
// 3. submit the transfer
// - DMA_DEV_TO_MEM:
// 1. prepare the descriptor (dmaengine_prep_*)
// 2. use dmaengine_desc_attach_metadata() to attach the buffer to the
// descriptor
// 3. submit the transfer
// 4. when the transfer is completed, the metadata should be available in the
// attached buffer
//
// @DESC_METADATA_ENGINE - the metadata buffer is allocated/managed by the DMA
// driver. The client driver can ask for the pointer, maximum size and the
// currently used size of the metadata and can directly update or read it.
// dmaengine_desc_get_metadata_ptr() and dmaengine_desc_set_metadata_len() is
// provided as helper functions.
//
// Note: the metadata area for the descriptor is no longer valid after the
// transfer has been completed (valid up to the point when the completion
// callback returns if used).
//
// Client drivers interested to use this mode can follow:
// - DMA_MEM_TO_DEV / DEV_MEM_TO_MEM:
// 1. prepare the descriptor (dmaengine_prep_*)
// 2. use dmaengine_desc_get_metadata_ptr() to get the pointer to the engine's
// metadata area
// 3. update the metadata at the pointer
// 4. use dmaengine_desc_set_metadata_len()  to tell the DMA engine the amount
// of data the client has placed into the metadata buffer
// 5. submit the transfer
// - DMA_DEV_TO_MEM:
// 1. prepare the descriptor (dmaengine_prep_*)
// 2. submit the transfer
// 3. on transfer completion, use dmaengine_desc_get_metadata_ptr() to get the
// pointer to the engine's metadata area
// 4. Read out the metadata from the pointer
//
// Warning: the two modes are not compatible and clients must use one mode for a
// descriptor.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dma_desc_metadata_mode {
    DESC_METADATA_NONE = 0,
    DESC_METADATA_CLIENT = BIT(0),
    DESC_METADATA_ENGINE = BIT(1),
}

//
// struct dma_chan_percpu - the per-CPU part of struct dma_chan
// @memcpy_count: transaction counter
// @bytes_transferred: byte counter
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_chan_percpu {
// stats
    pub memcpy_count: c_ulong,
    pub bytes_transferred: c_ulong,
}

//
// struct dma_router - DMA router structure
// @dev: pointer to the DMA router device
// @route_free: function to be called when the route can be disconnected
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_router {
    pub dev: *mut device,
    pub route_data): *mut *mut *mut void (route_free)(struct device dev, void,
}

//
// struct dma_chan - devices supply DMA channels, clients use them
// @device: ptr to the dma device who supplies this channel, always !%NULL
// @slave: ptr to the device using this channel
// @cookie: last cookie value returned to client
// @completed_cookie: last completed cookie for this channel
// @lock: protect between config and prepare transfer when driver have not
// implemented callback device_prep_config_sg().
// @chan_id: channel ID for sysfs
// @dev: class device for sysfs
// @name: backlink name for sysfs
// @dbg_client_name: slave name for debugfs in format:
// dev_name(requester's dev):channel name, for example: "2b00000.mcasp:tx"
// @device_node: used to add this to the device chan list
// @local: per-cpu pointer to a struct dma_chan_percpu
// @client_count: how many clients are using this channel
// @table_count: number of appearances in the mem-to-mem allocation table
// @router: pointer to the DMA router structure
// @route_data: channel specific data for the router
// @private: private data for certain client-channel associations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_chan {
    pub device: *mut dma_device,
    pub slave: *mut device,
    pub cookie: dma_cookie_t,
    pub completed_cookie: dma_cookie_t,
//
// protect between config and prepare transfer because *_prep() may be
// called from complete callback, which is in GFP_NOSLEEP context.
//
    pub lock: spinlock_t,
// sysfs
    pub chan_id: c_int,
    pub dev: *mut dma_chan_dev,
    pub name: *const c_char,

    pub dbg_client_name: *mut c_char,

    pub device_node: list_head,
    pub local: *mut dma_chan_percpu __percpu,
    pub client_count: c_int,
    pub table_count: c_int,
// DMA router
    pub router: *mut dma_router,
    pub route_data: *mut c_void,
    pub private: *mut c_void,
}

//
// struct dma_chan_dev - relate sysfs device node to backing channel device
// @chan: driver channel device
// @device: sysfs device
// @dev_id: parent dma_device dev_id
// @chan_dma_dev: The channel is using custom/different dma-mapping
// compared to the parent dma_device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_chan_dev {
    pub chan: *mut dma_chan,
    pub device: device,
    pub dev_id: c_int,
    pub chan_dma_dev: bool,
}

//
// enum dma_slave_buswidth - defines bus width of the DMA slave
// device, source or target buses
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dma_slave_buswidth {
    DMA_SLAVE_BUSWIDTH_UNDEFINED = 0,
    DMA_SLAVE_BUSWIDTH_1_BYTE = 1,
    DMA_SLAVE_BUSWIDTH_2_BYTES = 2,
    DMA_SLAVE_BUSWIDTH_3_BYTES = 3,
    DMA_SLAVE_BUSWIDTH_4_BYTES = 4,
    DMA_SLAVE_BUSWIDTH_8_BYTES = 8,
    DMA_SLAVE_BUSWIDTH_16_BYTES = 16,
    DMA_SLAVE_BUSWIDTH_32_BYTES = 32,
    DMA_SLAVE_BUSWIDTH_64_BYTES = 64,
    DMA_SLAVE_BUSWIDTH_128_BYTES = 128,
}

//
// struct dma_slave_config - dma slave channel runtime config
// @direction: whether the data shall go in or out on this slave
// channel, right now. DMA_MEM_TO_DEV and DMA_DEV_TO_MEM are
// legal values. DEPRECATED, drivers should use the direction argument
// to the device_prep_slave_sg and device_prep_dma_cyclic functions or
// the dir field in the dma_interleaved_template structure.
// @src_addr: this is the physical address where DMA slave data
// should be read (RX), if the source is memory this argument is
// ignored.
// @dst_addr: this is the physical address where DMA slave data
// should be written (TX), if the destination is memory this argument
// is ignored.
// @src_addr_width: this is the width in bytes of the source (RX)
// register where DMA data shall be read. If the source
// is memory this may be ignored depending on architecture.
// Legal values: 1, 2, 3, 4, 8, 16, 32, 64, 128.
// @dst_addr_width: same as src_addr_width but for destination
// target (TX) mutatis mutandis.
// @src_maxburst: the maximum number of words (note: words, as in
// units of the src_addr_width member, not bytes) that can be sent
// in one burst to the device. Typically something like half the
// FIFO depth on I/O peripherals so you don't overflow it. This
// may or may not be applicable on memory sources.
// @dst_maxburst: same as src_maxburst but for destination target
// mutatis mutandis.
// @src_port_window_size: The length of the register area in words the data need
// to be accessed on the device side. It is only used for devices which is using
// an area instead of a single register to receive the data. Typically the DMA
// loops in this area in order to transfer the data.
// @dst_port_window_size: same as src_port_window_size but for the destination
// port.
// @device_fc: Flow Controller Settings. Only valid for slave channels. Fill
// with 'true' if peripheral should be flow controller. Direction will be
// selected at Runtime.
// @peripheral_config: peripheral configuration for programming peripheral
// for dmaengine transfer
// @peripheral_size: peripheral configuration buffer size
//
// This struct is passed in as configuration data to a DMA engine
// in order to set up a certain channel for DMA transport at runtime.
// The DMA device/engine has to provide support for an additional
// callback in the dma_device structure, device_config and this struct
// will then be passed in as an argument to the function.
//
// The rationale for adding configuration information to this struct is as
// follows: if it is likely that more than one DMA slave controllers in
// the world will support the configuration option, then make it generic.
// If not: if it is fixed so that it be sent in static from the platform
// data, then prefer to do that.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_slave_config {
    pub direction: dma_transfer_direction,
    pub src_addr: phys_addr_t,
    pub dst_addr: phys_addr_t,
    pub src_addr_width: dma_slave_buswidth,
    pub dst_addr_width: dma_slave_buswidth,
    pub src_maxburst: u32,
    pub dst_maxburst: u32,
    pub src_port_window_size: u32,
    pub dst_port_window_size: u32,
    pub device_fc: bool,
    pub peripheral_config: *mut c_void,
    pub peripheral_size: usize,
}

//
// enum dma_residue_granularity - Granularity of the reported transfer residue
// @DMA_RESIDUE_GRANULARITY_DESCRIPTOR: Residue reporting is not support. The
// DMA channel is only able to tell whether a descriptor has been completed or
// not, which means residue reporting is not supported by this channel. The
// residue field of the dma_tx_state field will always be 0.
// @DMA_RESIDUE_GRANULARITY_SEGMENT: Residue is updated after each successfully
// completed segment of the transfer (For cyclic transfers this is after each
// period). This is typically implemented by having the hardware generate an
// interrupt after each transferred segment and then the drivers updates the
// outstanding residue by the size of the segment. Another possibility is if
// the hardware supports scatter-gather and the segment descriptor has a field
// which gets set after the segment has been completed. The driver then counts
// the number of segments without the flag set to compute the residue.
// @DMA_RESIDUE_GRANULARITY_BURST: Residue is updated after each transferred
// burst. This is typically only supported if the hardware has a progress
// register of some sort (E.g. a register with the current read/write address
// or a register with the amount of bursts/beats/bytes that have been
// transferred or still need to be transferred).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dma_residue_granularity {
    DMA_RESIDUE_GRANULARITY_DESCRIPTOR = 0,
    DMA_RESIDUE_GRANULARITY_SEGMENT = 1,
    DMA_RESIDUE_GRANULARITY_BURST = 2,
}

//
// struct dma_slave_caps - expose capabilities of a slave channel only
// @src_addr_widths: bit mask of src addr widths the channel supports.
// Width is specified in bytes, e.g. for a channel supporting
// a width of 4 the mask should have BIT(4) set.
// @dst_addr_widths: bit mask of dst addr widths the channel supports
// @directions: bit mask of slave directions the channel supports.
// Since the enum dma_transfer_direction is not defined as bit flag for
// each type, the dma controller should set BIT(<TYPE>) and same
// should be checked by controller as well
// @min_burst: min burst capability per-transfer
// @max_burst: max burst capability per-transfer
// @max_sg_burst: max number of SG list entries executed in a single burst
// DMA tansaction with no software intervention for reinitialization.
// Zero value means unlimited number of entries.
// @cmd_pause: true, if pause is supported (i.e. for reading residue or
// for resume later)
// @cmd_resume: true, if resume is supported
// @cmd_terminate: true, if terminate cmd is supported
// @residue_granularity: granularity of the reported transfer residue
// @descriptor_reuse: if a descriptor can be reused by client and
// resubmitted multiple times
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_slave_caps {
    pub src_addr_widths: u32,
    pub dst_addr_widths: u32,
    pub directions: u32,
    pub min_burst: u32,
    pub max_burst: u32,
    pub max_sg_burst: u32,
    pub cmd_pause: bool,
    pub cmd_resume: bool,
    pub cmd_terminate: bool,
    pub residue_granularity: dma_residue_granularity,
    pub descriptor_reuse: bool,
}

extern "C" {
    pub fn dev_name(_arg: &chan->dev->device) -> return;
}
//
// typedef dma_filter_fn - callback filter for dma_request_channel
// @chan: channel to be reviewed
// @filter_param: opaque parameter passed through dma_request_channel
//
// When this optional parameter is specified in a call to dma_request_channel a
// suitable channel is passed to this routine for further dispositioning before
// being returned.  Where 'suitable' indicates a non-busy channel that
// satisfies the given capability mask.  It returns 'true' to indicate that the
// channel is suitable.
//
extern "C" {
    pub fn bool(chan: *mut *mut dma_filter_fn)(struct dma_chan, filter_param: *mut c_void) -> typedef;
}
extern "C" {
    pub fn void(dma_async_param: *mut *mut dma_async_tx_callback)(void) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmaengine_tx_result {
    DMA_TRANS_NOERROR = 0,		/* SUCCESS */
    DMA_TRANS_READ_FAILED,		/* Source DMA read failed */
    DMA_TRANS_WRITE_FAILED,		/* Destination DMA write failed */
    DMA_TRANS_ABORTED,		/* Op never submitted / aborted */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmaengine_result {
    pub result: dmaengine_tx_result,
    pub residue: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmaengine_unmap_data {

    pub map_cnt: u16,

    pub map_cnt: u8,

    pub to_cnt: u8,
    pub from_cnt: u8,
    pub bidi_cnt: u8,
    pub dev: *mut device,
    pub kref: kref,
    pub len: usize,
    pub addr: [dma_addr_t; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_descriptor_metadata_ops {
    pub len): usize,
    pub max_len): *mut *mut size_t payload_len, size_t,
    pub payload_len): usize,
}

//
// struct dma_async_tx_descriptor - async transaction descriptor
// ---dma generic offload fields---
// @cookie: tracking cookie for this transaction, set to -EBUSY if
// this tx is sitting on a dependency list
// @flags: flags to augment operation preparation, control completion, and
// communicate status
// @phys: physical address of the descriptor
// @chan: target channel for this operation
// @tx_submit: accept the descriptor, assign ordered cookie and mark the
// descriptor pending. To be pushed on .issue_pending() call
// @desc_free: driver's callback function to free a resusable descriptor
// after completion
// @callback: routine to call after this operation is complete
// @callback_result: error result from a DMA transaction
// @callback_param: general parameter to pass to the callback routine
// @unmap: hook for generic DMA unmap data
// @desc_metadata_mode: core managed metadata mode to protect mixed use of
// DESC_METADATA_CLIENT or DESC_METADATA_ENGINE. Otherwise
// DESC_METADATA_NONE
// @metadata_ops: DMA driver provided metadata mode ops, need to be set by the
// DMA driver if metadata mode is supported with the descriptor
// ---async_tx api specific fields---
// @next: at completion submit this descriptor
// @parent: pointer to the next level up in the dependency chain
// @lock: protect the parent and next pointers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_async_tx_descriptor {
    pub cookie: dma_cookie_t,
    pub /: *mut *mut dma_ctrl_flags flags; / not a 'long' to pack with cookie,
    pub phys: dma_addr_t,
    pub chan: *mut dma_chan,
    pub tx): *mut *mut dma_cookie_t (tx_submit)(struct dma_async_tx_descriptor,
    pub tx): *mut *mut int (desc_free)(struct dma_async_tx_descriptor,
    pub callback: dma_async_tx_callback,
    pub callback_result: dma_async_tx_callback_result,
    pub callback_param: *mut c_void,
    pub unmap: *mut dmaengine_unmap_data,
    pub desc_metadata_mode: dma_desc_metadata_mode,
    pub metadata_ops: *const dma_descriptor_metadata_ops,

    pub next: *mut dma_async_tx_descriptor,
    pub parent: *mut dma_async_tx_descriptor,
    pub lock: spinlock_t,

}

extern "C" {
    pub fn dmaengine_unmap_put(unmap: *mut dmaengine_unmap_data);
}

//
// struct dma_tx_state - filled in to report the status of
// a transfer.
// @last: last completed DMA cookie
// @used: last issued DMA cookie (i.e. the one in progress)
// @residue: the remaining number of bytes left to transmit
// on the selected transfer for states DMA_IN_PROGRESS and
// DMA_PAUSED if this is implemented in the driver, else 0
// @in_flight_bytes: amount of data in bytes cached by the DMA.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_tx_state {
    pub last: dma_cookie_t,
    pub used: dma_cookie_t,
    pub residue: u32,
    pub in_flight_bytes: u32,
}

//
// enum dmaengine_alignment - defines alignment of the DMA async tx
// buffers
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmaengine_alignment {
    DMAENGINE_ALIGN_1_BYTE = 0,
    DMAENGINE_ALIGN_2_BYTES = 1,
    DMAENGINE_ALIGN_4_BYTES = 2,
    DMAENGINE_ALIGN_8_BYTES = 3,
    DMAENGINE_ALIGN_16_BYTES = 4,
    DMAENGINE_ALIGN_32_BYTES = 5,
    DMAENGINE_ALIGN_64_BYTES = 6,
    DMAENGINE_ALIGN_128_BYTES = 7,
    DMAENGINE_ALIGN_256_BYTES = 8,
}

//
// struct dma_slave_map - associates slave device and it's slave channel with
// parameter to be used by a filter function
// @devname: name of the device
// @slave: slave channel name
// @param: opaque parameter to pass to struct dma_filter.fn
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_slave_map {
    pub devname: *const c_char,
    pub slave: *const c_char,
    pub param: *mut c_void,
}

//
// struct dma_filter - information for slave device/channel to filter_fn/param
// mapping
// @fn: filter function callback
// @mapcnt: number of slave device/channel in the map
// @map: array of channel to filter mapping data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_filter {
    pub fn: dma_filter_fn,
    pub mapcnt: c_int,
    pub map: *const dma_slave_map,
}

//
// struct dma_device - info on the entity supplying DMA services
// @ref: reference is taken and put every time a channel is allocated or freed
// @chancnt: how many DMA channels are supported
// @privatecnt: how many DMA channels are requested by dma_request_channel
// @channels: the list of struct dma_chan
// @global_node: list_head for global dma_device_list
// @filter: information for device/slave to filter function/param mapping
// @cap_mask: one or more dma_capability flags
// @desc_metadata_modes: supported metadata modes by the DMA device
// @max_xor: maximum number of xor sources, 0 if no capability
// @max_pq: maximum number of PQ sources and PQ-continue capability
// @copy_align: alignment shift for memcpy operations
// @xor_align: alignment shift for xor operations
// @pq_align: alignment shift for pq operations
// @fill_align: alignment shift for memset operations
// @dev_id: unique device ID
// @dev: struct device reference for dma mapping api
// @owner: owner module (automatically set based on the provided dev)
// @chan_ida: unique channel ID
// @src_addr_widths: bit mask of src addr widths the device supports
// Width is specified in bytes, e.g. for a device supporting
// a width of 4 the mask should have BIT(4) set.
// @dst_addr_widths: bit mask of dst addr widths the device supports
// @directions: bit mask of slave directions the device supports.
// Since the enum dma_transfer_direction is not defined as bit flag for
// each type, the dma controller should set BIT(<TYPE>) and same
// should be checked by controller as well
// @min_burst: min burst capability per-transfer
// @max_burst: max burst capability per-transfer
// @max_sg_burst: max number of SG list entries executed in a single burst
// DMA tansaction with no software intervention for reinitialization.
// Zero value means unlimited number of entries.
// @descriptor_reuse: a submitted transfer can be resubmitted after completion
// @residue_granularity: granularity of the transfer residue reported
// by tx_status
// @device_alloc_chan_resources: allocate resources and return the
// number of allocated descriptors
// @device_router_config: optional callback for DMA router configuration
// @device_free_chan_resources: release DMA channel's resources
// @device_prep_dma_memcpy: prepares a memcpy operation
// @device_prep_dma_xor: prepares a xor operation
// @device_prep_dma_xor_val: prepares a xor validation operation
// @device_prep_dma_pq: prepares a pq operation
// @device_prep_dma_pq_val: prepares a pqzero_sum operation
// @device_prep_dma_memset: prepares a memset operation
// @device_prep_dma_memset_sg: prepares a memset operation over a scatter list
// @device_prep_dma_interrupt: prepares an end of chain interrupt operation
// @device_prep_peripheral_dma_vec: prepares a scatter-gather DMA transfer,
// where the address and size of each segment is located in one entry of
// the dma_vec array.
// @device_prep_slave_sg: prepares a slave dma operation
// @device_prep_config_sg: prepares a slave DMA operation with dma_slave_config
// @device_prep_dma_cyclic: prepare a cyclic dma operation suitable for audio.
// The function takes a buffer of size buf_len. The callback function will
// be called after period_len bytes have been transferred.
// @device_prep_interleaved_dma: Transfer expression in a generic way.
// @device_caps: May be used to override the generic DMA slave capabilities
// with per-channel specific ones
// @device_config: Pushes a new configuration to a channel, return 0 or an error
// code
// @device_pause: Pauses any transfer happening on a channel. Returns
// 0 or an error code
// @device_resume: Resumes any transfer on a channel previously
// paused. Returns 0 or an error code
// @device_terminate_all: Aborts all transfers on a channel. Returns 0
// or an error code
// @device_synchronize: Synchronizes the termination of a transfers to the
// current context.
// @device_tx_status: poll for transaction completion, the optional
// txstate parameter can be supplied with a pointer to get a
// struct with auxiliary transfer status information, otherwise the call
// will just return a simple status code
// @device_issue_pending: push pending transactions to hardware
// @device_release: called sometime atfer dma_async_device_unregister() is
// called and there are no further references to this structure. This
// must be implemented to free resources however many existing drivers
// do not and are therefore not safe to unbind while in use.
// @dbg_summary_show: optional routine to show contents in debugfs; default code
// will be used when this is omitted, but custom code can show extra,
// controller specific information.
// @dbg_dev_root: the root folder in debugfs for this device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_device {
    pub ref: kref,
    pub chancnt: c_uint,
    pub privatecnt: c_uint,
    pub channels: list_head,
    pub global_node: list_head,
    pub filter: dma_filter,
    pub cap_mask: dma_cap_mask_t,
    pub desc_metadata_modes: dma_desc_metadata_mode,
    pub max_xor: c_ushort,
    pub max_pq: c_ushort,
    pub copy_align: dmaengine_alignment,
    pub xor_align: dmaengine_alignment,
    pub pq_align: dmaengine_alignment,
    pub fill_align: dmaengine_alignment,

    pub dev_id: c_int,
    pub dev: *mut device,
    pub owner: *mut module,
    pub chan_ida: ida,
    pub src_addr_widths: u32,
    pub dst_addr_widths: u32,
    pub directions: u32,
    pub min_burst: u32,
    pub max_burst: u32,
    pub max_sg_burst: u32,
    pub descriptor_reuse: bool,
    pub residue_granularity: dma_residue_granularity,
    pub chan): *mut *mut int (device_alloc_chan_resources)(struct dma_chan,
    pub chan): *mut *mut int (device_router_config)(struct dma_chan,
    pub chan): *mut *mut void (device_free_chan_resources)(struct dma_chan,
    pub flags): size_t len, unsigned long,
    pub flags): unsigned int src_cnt, size_t len, unsigned long,
    pub flags): *mut *mut size_t len, enum sum_check_flags result, unsigned long,
    pub flags): size_t len, unsigned long,
    pub flags): *mut *mut sum_check_flags pqres, unsigned long,
    pub flags): c_ulong,
    pub flags): unsigned int nents, int value, unsigned long,
    pub flags): *mut *mut dma_chan chan, unsigned long,
    pub flags): c_ulong,
    pub context): *mut unsigned long flags, void,
    pub config): *mut unsigned long flags, struct dma_slave_config,
    pub flags): c_ulong,
    pub flags): c_ulong,
    pub caps): *mut *mut *mut void (device_caps)(struct dma_chan chan, struct dma_slave_caps,
    pub config): *mut *mut *mut int (device_config)(struct dma_chan chan, struct dma_slave_config,
    pub chan): *mut *mut int (device_pause)(struct dma_chan,
    pub chan): *mut *mut int (device_resume)(struct dma_chan,
    pub chan): *mut *mut int (device_terminate_all)(struct dma_chan,
    pub chan): *mut *mut void (device_synchronize)(struct dma_chan,
    pub txstate): *mut dma_tx_state,
    pub chan): *mut *mut void (device_issue_pending)(struct dma_chan,
    pub dev): *mut *mut void (device_release)(struct dma_device,
// debugfs support
    pub dev): *mut *mut *mut void (dbg_summary_show)(struct seq_file s, struct dma_device,
    pub dbg_dev_root: *mut dentry,
}

extern "C" {
    pub fn dmaengine_prep_config_single(_arg: chan, _arg: buf, _arg: len, _arg: dir, _arg: flags, _arg: NULL) -> return;
}
//
// dmaengine_prep_peripheral_dma_vec() - Prepare a DMA scatter-gather descriptor
// @chan: The channel to be used for this descriptor
// @vecs: The array of DMA vectors that should be transferred
// @nents: The number of DMA vectors in the array
// @dir: Specifies the direction of the data transfer
// @flags: DMA engine flags - DMA_PREP_REPEAT can be used to mark a cyclic
// DMA transfer
//
extern "C" {
    pub fn dmaengine_prep_config_sg(_arg: chan, _arg: sgl, _arg: sg_len, _arg: dir, _arg: flags, _arg: NULL) -> return;
}
//
// dmaengine_prep_config_sg_safe - prepare a scatter-gather DMA transfer
// with atomic slave configuration update
// @chan: DMA channel
// @sgl: scatterlist for the transfer
// @sg_len: number of entries in @sgl
// @dir: DMA transfer direction
// @flags: transfer preparation flags
// @config: DMA slave configuration for this transfer
//
// Prepare a DMA scatter-gather transfer together with a corresponding slave
// configuration update in a re-entrant and race-safe manner.
//
// DMA engine drivers may implement the optional
// device_prep_config_sg() callback to perform both the slave configuration
// and descriptor preparation atomically. In this case, the operation is
// fully handled by the DMA engine driver.
//
// If the DMA engine driver does not implement device_prep_config_sg(), falls
// back to calling dmaengine_slave_config() followed by dmaengine_prep_slave_sg().
// The fallback path is protected by a per-channel spinlock to ensure that
// concurrent callers cannot interleave configuration and descriptor preparation
// on the same DMA channel.
//
// Return: Pointer to a prepared DMA async transaction descriptor on success,
// or %NULL if the transfer could not be prepared.
//
// dmaengine_prep_config_single_safe - prepare a single-buffer DMA transfer
// with atomic slave configuration update
// @chan: DMA channel
// @buf: DMA buffer address
// @len: length of the transfer in bytes
// @dir: DMA transfer direction
// @flags: transfer preparation flags
// @config: DMA slave configuration for this transfer
//
// Detail see dmaengine_prep_config_sg_safe().
//
extern "C" {
    pub fn dmaengine_prep_config_sg_safe(_arg: chan, _arg: &sg, _arg: 1, _arg: dir, _arg: flags, _arg: config) -> return;
}

//
// dmaengine_prep_dma_memset() - Prepare a DMA memset descriptor.
// @chan: The channel to be used for this descriptor
// @dest: Address of buffer to be set
// @value: Treated as a single byte value that fills the destination buffer
// @len: The total size of dest
// @flags: DMA engine flags
//

//
// dmaengine_terminate_all() - Terminate all active DMA transfers
// @chan: The channel for which to terminate the transfers
//
// This function is DEPRECATED use either dmaengine_terminate_sync() or
// dmaengine_terminate_async() instead.
//
// dmaengine_terminate_async() - Terminate all active DMA transfers
// @chan: The channel for which to terminate the transfers
//
// Calling this function will terminate all active and pending descriptors
// that have previously been submitted to the channel. It is not guaranteed
// though that the transfer for the active descriptor has stopped when the
// function returns. Furthermore it is possible the complete callback of a
// submitted transfer is still running when this function returns.
//
// dmaengine_synchronize() needs to be called before it is safe to free
// any memory that is accessed by previously submitted descriptors or before
// freeing any resources accessed from within the completion callback of any
// previously submitted descriptors.
//
// This function can be called from atomic context as well as from within a
// complete callback of a descriptor submitted on the same channel.
//
// If none of the two conditions above apply consider using
// dmaengine_terminate_sync() instead.
//
// dmaengine_synchronize() - Synchronize DMA channel termination
// @chan: The channel to synchronize
//
// Synchronizes to the DMA channel termination to the current context. When this
// function returns it is guaranteed that all transfers for previously issued
// descriptors have stopped and it is safe to free the memory associated
// with them. Furthermore it is guaranteed that all complete callback functions
// for a previously submitted descriptor have finished running and it is safe to
// free resources accessed from within the complete callbacks.
//
// The behavior of this function is undefined if dma_async_issue_pending() has
// been called between dmaengine_terminate_async() and this function.
//
// This function must only be called from non-atomic context and must not be
// called from within a complete callback of a descriptor submitted on the same
// channel.
//
// dmaengine_terminate_sync() - Terminate all active DMA transfers
// @chan: The channel for which to terminate the transfers
//
// Calling this function will terminate all active and pending transfers
// that have previously been submitted to the channel. It is similar to
// dmaengine_terminate_async() but guarantees that the DMA transfer has actually
// stopped and that all complete callbacks have finished running when the
// function returns.
//
// This function must only be called from non-atomic context and must not be
// called from within a complete callback of a descriptor submitted on the same
// channel.
//
extern "C" {
    pub fn dmaengine_check_align(_arg: dev->copy_align, _arg: off1, _arg: off2, _arg: len) -> return;
}
extern "C" {
    pub fn dmaengine_check_align(_arg: dev->xor_align, _arg: off1, _arg: off2, _arg: len) -> return;
}
extern "C" {
    pub fn dmaengine_check_align(_arg: dev->pq_align, _arg: off1, _arg: off2, _arg: len) -> return;
}
extern "C" {
    pub fn dmaengine_check_align(_arg: dev->fill_align, _arg: off1, _arg: off2, _arg: len) -> return;
}
// dma_maxpq - reduce maxpq in the face of continued operations
// @dma - dma device with PQ capability
// @flags - to check if DMA_PREP_CONTINUE and DMA_PREP_PQ_DISABLE_P are set
//
// When an engine does not support native continuation we need 3 extra
// source slots to reuse P and Q with the following coefficients:
// 1/ {00} * P : remove P from Q', but use it as a source for P'
// 2/ {01} * Q : use Q to continue Q' calculation
// 3/ {00} * Q : subtract Q from P' to cancel (2)
//
// In the case where P is disabled we only need 1 extra source:
// 1/ {01} * Q : use Q to continue Q' calculation
//
extern "C" {
    pub fn dma_dev_to_maxpq(_arg: dma) -> return;
}
// --- public DMA engine API ---

extern "C" {
    pub fn dmaengine_get();
}
extern "C" {
    pub fn dmaengine_put();
}

extern "C" {
    pub fn test_bit(_arg: tx_type, _arg: srcp->bits) -> return;
}

//
// dma_async_issue_pending - flush pending transactions to HW
// @chan: target DMA channel
//
// This allows drivers to push copies to HW in batches,
// reducing MMIO writes where possible.
//
// dma_async_is_tx_complete - poll for transaction completion
// @chan: DMA channel
// @cookie: transaction identifier to check status of
// @last: returns last completed cookie, can be NULL
// @used: returns last issued cookie, can be NULL
//
// If @last and @used are passed in, upon return they reflect the driver
// internal state and can be used with dma_async_is_complete() to check
// the status of multiple cookies without re-checking hardware state.
//
// last = state.last;
// used = state.used;
//
// dma_async_is_complete - test a cookie against chan state
// @cookie: transaction identifier to test status of
// @last_complete: last know completed transaction
// @last_used: last cookie value handed out
//
// dma_async_is_complete() is used in dma_async_is_tx_complete()
// the test logic is separated for lightweight testing of multiple cookies
//

extern "C" {
    pub fn dma_sync_wait(chan: *mut dma_chan, cookie: dma_cookie_t) -> dma_status;
}
extern "C" {
    pub fn dma_wait_for_async_tx(tx: *mut dma_async_tx_descriptor) -> dma_status;
}
extern "C" {
    pub fn dma_issue_pending_all();
}
extern "C" {
    pub fn dma_release_channel(chan: *mut dma_chan);
}
extern "C" {
    pub fn dma_get_slave_caps(chan: *mut dma_chan, caps: *mut dma_slave_caps) -> c_int;
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

// this is supported for reusable desc, so check that
// --- DMA device ---
extern "C" {
    pub fn dma_async_device_register(device: *mut dma_device) -> c_int;
}
extern "C" {
    pub fn dmaenginem_async_device_register(device: *mut dma_device) -> c_int;
}
extern "C" {
    pub fn dma_async_device_unregister(device: *mut dma_device);
}
extern "C" {
    pub fn dma_run_dependencies(tx: *mut dma_async_tx_descriptor);
}

// Deprecated, please use dma_request_chan() directly
// dma_request_slave_channel_compat(const dma_cap_mask_t mask,
extern "C" {
    pub fn dma_request_channel(_arg: mask, _arg: fn, _arg: fn_param) -> return;
}
