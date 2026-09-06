//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dma/ti-cppi5.h
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
// CPPI5 descriptors interface
//
// Copyright (C) 2019 Texas Instruments Incorporated - https://www.ti.com
//

//
// struct cppi5_desc_hdr_t - Descriptor header, present in all types of
// descriptors
// @pkt_info0:		Packet info word 0 (n/a in Buffer desc)
// @pkt_info1:		Packet info word 1 (n/a in Buffer desc)
// @pkt_info2:		Packet info word 2 (n/a in Buffer desc)
// @src_dst_tag:	Packet info word 3 (n/a in Buffer desc)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cppi5_desc_hdr_t {
    pub pkt_info0: u32,
    pub pkt_info1: u32,
    pub pkt_info2: u32,
    pub src_dst_tag: u32,
    pub __packed: },
//
// struct cppi5_host_desc_t - Host-mode packet and buffer descriptor definition
// @hdr:		Descriptor header
// @next_desc:		word 4/5: Linking word
// @buf_ptr:		word 6/7: Buffer pointer
// @buf_info1:		word 8: Buffer valid data length
// @org_buf_len:	word 9: Original buffer length
// @org_buf_ptr:	word 10/11: Original buffer pointer
// @epib:		Extended Packet Info Data (optional, 4 words), and/or
// Protocol Specific Data (optional, 0-128 bytes in
// multiples of 4), and/or
// Other Software Data (0-N bytes, optional)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cppi5_host_desc_t {
    pub hdr: cppi5_desc_hdr_t,
    pub next_desc: u64,
    pub buf_ptr: u64,
    pub buf_info1: u32,
    pub org_buf_len: u32,
    pub org_buf_ptr: u64,
    pub epib: [u32; ],
    pub __packed: },

//
// Protocol Specific Words location:
// 0 - located in the descriptor,
// 1 = located in the SOP Buffer immediately prior to the data.
//

// Return Policy: 0 - Entire packet 1 - Each buffer

//
// Early Return:
// 0 = desc pointers should be returned after all reads have been completed
// 1 = desc pointers should be returned immediately upon fetching
// the descriptor and beginning to transfer data.
//

//
// Return Push Policy:
// 0 = Descriptor must be returned to tail of queue
// 1 = Descriptor must be returned to head of queue
//

//
// struct cppi5_desc_epib_t - Host Packet Descriptor Extended Packet Info Block
// @timestamp:		word 0: application specific timestamp
// @sw_info0:		word 1: Software Info 0
// @sw_info1:		word 1: Software Info 1
// @sw_info2:		word 1: Software Info 2
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cppi5_desc_epib_t {
    pub /: *mut *mut u32 timestamp; / w0: application specific timestamp,
    pub /: *mut *mut u32 sw_info0; / w1: Software Info 0,
    pub /: *mut *mut u32 sw_info1; / w2: Software Info 1,
    pub /: *mut *mut u32 sw_info2; / w3: Software Info 2,
}

//
// struct cppi5_monolithic_desc_t - Monolithic-mode packet descriptor
// @hdr:		Descriptor header
// @epib:		Extended Packet Info Data (optional, 4 words), and/or
// Protocol Specific Data (optional, 0-128 bytes in
// multiples of 4), and/or
// Other Software Data (0-N bytes, optional)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cppi5_monolithic_desc_t {
    pub hdr: cppi5_desc_hdr_t,
    pub epib: [u32; ],
}

//
// Reload Count:
// 0 = Finish the packet and place the descriptor back on the return queue
// 1-0x1ff = Vector to the Reload Index and resume processing
// 0x1ff indicates perpetual loop, infinite reload until the channel is stopped
//

//
// cppi5_desc_is_tdcm - check if the paddr indicates Teardown Complete Message
// @paddr: Physical address of the packet popped from the ring
//
// Returns: true if the address indicates TDCM
//
// cppi5_desc_get_type - get descriptor type
// @desc_hdr: packet descriptor/TR header
//
// Returns: descriptor type:
// CPPI5_INFO0_DESC_TYPE_VAL_HOST
// CPPI5_INFO0_DESC_TYPE_VAL_MONO
// CPPI5_INFO0_DESC_TYPE_VAL_TR
//
// cppi5_desc_get_errflags - get Error Flags from Desc
// @desc_hdr: packet/TR descriptor header
//
// Returns: Error Flags from Packet/TR Descriptor
//
// cppi5_desc_get_pktids - get Packet and Flow ids from Desc
// @desc_hdr: packet/TR descriptor header
// @pkt_id: Packet ID
// @flow_id: Flow ID
//
// Returns Packet and Flow ids from packet/TR descriptor
//
// pkt_id = (desc_hdr->pkt_info1 & CPPI5_INFO1_DESC_PKTID_MASK) >>
// flow_id = (desc_hdr->pkt_info1 & CPPI5_INFO1_DESC_FLOWID_MASK) >>
//
// cppi5_desc_set_pktids - set Packet and Flow ids in Desc
// @desc_hdr: packet/TR descriptor header
// @pkt_id: Packet ID
// @flow_id: Flow ID
//
// cppi5_desc_set_retpolicy - set Packet Return Policy in Desc
// @desc_hdr: packet/TR descriptor header
// @flags: fags, supported values
// CPPI5_INFO2_HDESC_RETPOLICY
// CPPI5_INFO2_HDESC_EARLYRET
// CPPI5_INFO2_DESC_RETPUSHPOLICY
// @return_ring_id: Packet Return Queue/Ring id, value 0xFFFF reserved
//
// cppi5_desc_get_tags_ids - get Packet Src/Dst Tags from Desc
// @desc_hdr: packet/TR descriptor header
// @src_tag_id: Source Tag
// @dst_tag_id: Dest Tag
//
// Returns Packet Src/Dst Tags from packet/TR descriptor
//
// src_tag_id = (desc_hdr->src_dst_tag &
// dst_tag_id = desc_hdr->src_dst_tag &
//
// cppi5_desc_set_tags_ids - set Packet Src/Dst Tags in HDesc
// @desc_hdr: packet/TR descriptor header
// @src_tag_id: Source Tag
// @dst_tag_id: Dest Tag
//
// Returns Packet Src/Dst Tags from packet/TR descriptor
//
// cppi5_hdesc_calc_size - Calculate Host Packet Descriptor size
// @epib: is EPIB present
// @psdata_size: PSDATA size
// @sw_data_size: SWDATA size
//
// Returns: required Host Packet Descriptor size
// 0 - if PSDATA > CPPI5_INFO0_HDESC_PSDATA_MAX_SIZE
//
extern "C" {
    pub fn ALIGN(_arg: desc_size, _arg: CPPI5_DESC_MIN_ALIGN) -> return;
}
//
// cppi5_hdesc_init - Init Host Packet Descriptor size
// @desc: Host packet descriptor
// @flags: supported values
// CPPI5_INFO0_HDESC_EPIB_PRESENT
// CPPI5_INFO0_HDESC_PSINFO_LOCATION
// @psdata_size: PSDATA size
//
// Returns required Host Packet Descriptor size
// 0 - if PSDATA > CPPI5_INFO0_HDESC_PSDATA_MAX_SIZE
//
// cppi5_hdesc_update_flags - Replace descriptor flags
// @desc: Host packet descriptor
// @flags: supported values
// CPPI5_INFO0_HDESC_EPIB_PRESENT
// CPPI5_INFO0_HDESC_PSINFO_LOCATION
//
// cppi5_hdesc_update_psdata_size - Replace PSdata size
// @desc: Host packet descriptor
// @psdata_size: PSDATA size
//
// cppi5_hdesc_get_psdata_size - get PSdata size in bytes
// @desc: Host packet descriptor
//
// Returns: PSdata size in bytes
//
// cppi5_hdesc_get_pktlen - get Packet Length from HDesc
// @desc: Host packet descriptor
//
// Returns: Packet Length from Host Packet Descriptor
//
// cppi5_hdesc_set_pktlen - set Packet Length in HDesc
// @desc: Host packet descriptor
// @pkt_len: Packet length to set
//
// cppi5_hdesc_get_psflags - get Protocol Specific Flags from HDesc
// @desc: Host packet descriptor
//
// Returns: Protocol Specific Flags from Host Packet Descriptor
//
// cppi5_hdesc_set_psflags - set Protocol Specific Flags in HDesc
// @desc: Host packet descriptor
// @ps_flags: Protocol Specific flags to set
//
// cppi5_hdesc_get_pkttype - get Packet Type from HDesc
// @desc: Host packet descriptor
//
// Returns: Packet type
//
// cppi5_hdesc_set_pkttype - set Packet Type in HDesc
// @desc: Host packet descriptor
// @pkt_type: Packet Type
//
// cppi5_hdesc_attach_buf - attach buffer to HDesc
// @desc: Host packet descriptor
// @buf: Buffer physical address
// @buf_data_len: Buffer length
// @obuf: Original Buffer physical address
// @obuf_len: Original Buffer length
//
// Attaches buffer to Host Packet Descriptor
//
// obuf = desc->org_buf_ptr;
// obuf_len = desc->org_buf_len & CPPI5_OBUFINFO0_HDESC_BUF_LEN_MASK;
//
// cppi5_hdesc_link_hbdesc - link Host Buffer Descriptor to HDesc
// @desc: Host Packet Descriptor
// @hbuf_desc: Host Buffer Descriptor physical address
//
// add and link Host Buffer Descriptor to HDesc
//
// cppi5_hdesc_epib_present -  check if EPIB present
// @desc_hdr: packet descriptor/TR header
//
// Returns: true if EPIB present in the packet
//
// cppi5_hdesc_get_psdata -  Get pointer on PSDATA
// @desc: Host packet descriptor
//
// Returns: pointer on PSDATA in HDesc.
// NULL - if ps_data placed at the start of data buffer.
//
// cppi5_hdesc_get_swdata -  Get pointer on swdata
// @desc: Host packet descriptor
//
// Returns: pointer on SWDATA in HDesc.
// NOTE. It's caller responsibility to be sure hdesc actually has swdata.
//
// ================================== TR ==================================

//
// enum cppi5_tr_types - TR types
// @CPPI5_TR_TYPE0:	One dimensional data move
// @CPPI5_TR_TYPE1:	Two dimensional data move
// @CPPI5_TR_TYPE2:	Three dimensional data move
// @CPPI5_TR_TYPE3:	Four dimensional data move
// @CPPI5_TR_TYPE4:	Four dimensional data move with data formatting
// @CPPI5_TR_TYPE5:	Four dimensional Cache Warm
// @CPPI5_TR_TYPE8:	Four Dimensional Block Move
// @CPPI5_TR_TYPE9:	Four Dimensional Block Move with Repacking
// @CPPI5_TR_TYPE10:	Two Dimensional Block Move
// @CPPI5_TR_TYPE11:	Two Dimensional Block Move with Repacking
// @CPPI5_TR_TYPE15:	Four Dimensional Block Move with Repacking and
// Indirection
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cppi5_tr_types {
    CPPI5_TR_TYPE0 = 0,
    CPPI5_TR_TYPE1,
    CPPI5_TR_TYPE2,
    CPPI5_TR_TYPE3,
    CPPI5_TR_TYPE4,
    CPPI5_TR_TYPE5,
// type6-7: Reserved
    CPPI5_TR_TYPE8 = 8,
    CPPI5_TR_TYPE9,
    CPPI5_TR_TYPE10,
    CPPI5_TR_TYPE11,
// type12-14: Reserved
    CPPI5_TR_TYPE15 = 15,
// private:
    CPPI5_TR_TYPE_MAX
}

//
// enum cppi5_tr_event_size - TR Flags EVENT_SIZE field specifies when an event
// is generated for each TR.
// @CPPI5_TR_EVENT_SIZE_COMPLETION:	When TR is complete and all status for
// the TR has been received
// @CPPI5_TR_EVENT_SIZE_ICNT1_DEC:	Type 0: when the last data transaction
// is sent for the TR
// Type 1-11: when ICNT1 is decremented
// @CPPI5_TR_EVENT_SIZE_ICNT2_DEC:	Type 0-1,10-11: when the last data
// transaction is sent for the TR
// All other types: when ICNT2 is
// decremented
// @CPPI5_TR_EVENT_SIZE_ICNT3_DEC:	Type 0-2,10-11: when the last data
// transaction is sent for the TR
// All other types: when ICNT3 is
// decremented
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cppi5_tr_event_size {
    CPPI5_TR_EVENT_SIZE_COMPLETION,
    CPPI5_TR_EVENT_SIZE_ICNT1_DEC,
    CPPI5_TR_EVENT_SIZE_ICNT2_DEC,
    CPPI5_TR_EVENT_SIZE_ICNT3_DEC,
// private:
    CPPI5_TR_EVENT_SIZE_MAX
}

//
// enum cppi5_tr_trigger - TR Flags TRIGGERx field specifies the type of trigger
// used to enable the TR to transfer data as specified
// by TRIGGERx_TYPE field.
// @CPPI5_TR_TRIGGER_NONE:		No trigger
// @CPPI5_TR_TRIGGER_GLOBAL0:		Global trigger 0
// @CPPI5_TR_TRIGGER_GLOBAL1:		Global trigger 1
// @CPPI5_TR_TRIGGER_LOCAL_EVENT:	Local Event
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cppi5_tr_trigger {
    CPPI5_TR_TRIGGER_NONE,
    CPPI5_TR_TRIGGER_GLOBAL0,
    CPPI5_TR_TRIGGER_GLOBAL1,
    CPPI5_TR_TRIGGER_LOCAL_EVENT,
// private:
    CPPI5_TR_TRIGGER_MAX
}

//
// enum cppi5_tr_trigger_type - TR Flags TRIGGERx_TYPE field specifies the type
// of data transfer that will be enabled by
// receiving a trigger as specified by TRIGGERx.
// @CPPI5_TR_TRIGGER_TYPE_ICNT1_DEC:	The second inner most loop (ICNT1) will
// be decremented by 1
// @CPPI5_TR_TRIGGER_TYPE_ICNT2_DEC:	The third inner most loop (ICNT2) will
// be decremented by 1
// @CPPI5_TR_TRIGGER_TYPE_ICNT3_DEC:	The outer most loop (ICNT3) will be
// decremented by 1
// @CPPI5_TR_TRIGGER_TYPE_ALL:		The entire TR will be allowed to
// complete
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cppi5_tr_trigger_type {
    CPPI5_TR_TRIGGER_TYPE_ICNT1_DEC,
    CPPI5_TR_TRIGGER_TYPE_ICNT2_DEC,
    CPPI5_TR_TRIGGER_TYPE_ICNT3_DEC,
    CPPI5_TR_TRIGGER_TYPE_ALL,
// private:
    CPPI5_TR_TRIGGER_TYPE_MAX
}

pub type cppi5_tr_flags_t = u32;
//
// struct cppi5_tr_type0_t - Type 0 (One dimensional data move) TR (16 byte)
// @flags:		TR flags (type, triggers, event, configuration)
// @icnt0:		Total loop iteration count for level 0 (innermost)
// @_reserved:		Not used
// @addr:		Starting address for the source data or destination data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cppi5_tr_type0_t {
    pub flags: cppi5_tr_flags_t,
    pub icnt0: u16,
    pub _reserved: u16,
    pub addr: u64,
    pub __packed: } __aligned(16),
//
// struct cppi5_tr_type1_t - Type 1 (Two dimensional data move) TR (32 byte)
// @flags:		TR flags (type, triggers, event, configuration)
// @icnt0:		Total loop iteration count for level 0 (innermost)
// @icnt1:		Total loop iteration count for level 1
// @addr:		Starting address for the source data or destination data
// @dim1:		Signed dimension for loop level 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cppi5_tr_type1_t {
    pub flags: cppi5_tr_flags_t,
    pub icnt0: u16,
    pub icnt1: u16,
    pub addr: u64,
    pub dim1: i32,
    pub __packed: } __aligned(32),
//
// struct cppi5_tr_type2_t - Type 2 (Three dimensional data move) TR (32 byte)
// @flags:		TR flags (type, triggers, event, configuration)
// @icnt0:		Total loop iteration count for level 0 (innermost)
// @icnt1:		Total loop iteration count for level 1
// @addr:		Starting address for the source data or destination data
// @dim1:		Signed dimension for loop level 1
// @icnt2:		Total loop iteration count for level 2
// @_reserved:		Not used
// @dim2:		Signed dimension for loop level 2
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cppi5_tr_type2_t {
    pub flags: cppi5_tr_flags_t,
    pub icnt0: u16,
    pub icnt1: u16,
    pub addr: u64,
    pub dim1: i32,
    pub icnt2: u16,
    pub _reserved: u16,
    pub dim2: i32,
    pub __packed: } __aligned(32),
//
// struct cppi5_tr_type3_t - Type 3 (Four dimensional data move) TR (32 byte)
// @flags:		TR flags (type, triggers, event, configuration)
// @icnt0:		Total loop iteration count for level 0 (innermost)
// @icnt1:		Total loop iteration count for level 1
// @addr:		Starting address for the source data or destination data
// @dim1:		Signed dimension for loop level 1
// @icnt2:		Total loop iteration count for level 2
// @icnt3:		Total loop iteration count for level 3 (outermost)
// @dim2:		Signed dimension for loop level 2
// @dim3:		Signed dimension for loop level 3
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cppi5_tr_type3_t {
    pub flags: cppi5_tr_flags_t,
    pub icnt0: u16,
    pub icnt1: u16,
    pub addr: u64,
    pub dim1: i32,
    pub icnt2: u16,
    pub icnt3: u16,
    pub dim2: i32,
    pub dim3: i32,
    pub __packed: } __aligned(32),
//
// struct cppi5_tr_type15_t - Type 15 (Four Dimensional Block Copy with
// Repacking and Indirection Support) TR (64 byte)
// @flags:		TR flags (type, triggers, event, configuration)
// @icnt0:		Total loop iteration count for level 0 (innermost) for
// source
// @icnt1:		Total loop iteration count for level 1 for source
// @addr:		Starting address for the source data
// @dim1:		Signed dimension for loop level 1 for source
// @icnt2:		Total loop iteration count for level 2 for source
// @icnt3:		Total loop iteration count for level 3 (outermost) for
// source
// @dim2:		Signed dimension for loop level 2 for source
// @dim3:		Signed dimension for loop level 3 for source
// @_reserved:		Not used
// @ddim1:		Signed dimension for loop level 1 for destination
// @daddr:		Starting address for the destination data
// @ddim2:		Signed dimension for loop level 2 for destination
// @ddim3:		Signed dimension for loop level 3 for destination
// @dicnt0:		Total loop iteration count for level 0 (innermost) for
// destination
// @dicnt1:		Total loop iteration count for level 1 for destination
// @dicnt2:		Total loop iteration count for level 2 for destination
// @dicnt3:		Total loop iteration count for level 3 (outermost) for
// destination
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cppi5_tr_type15_t {
    pub flags: cppi5_tr_flags_t,
    pub icnt0: u16,
    pub icnt1: u16,
    pub addr: u64,
    pub dim1: i32,
    pub icnt2: u16,
    pub icnt3: u16,
    pub dim2: i32,
    pub dim3: i32,
    pub _reserved: u32,
    pub ddim1: i32,
    pub daddr: u64,
    pub ddim2: i32,
    pub ddim3: i32,
    pub dicnt0: u16,
    pub dicnt1: u16,
    pub dicnt2: u16,
    pub dicnt3: u16,
    pub __packed: } __aligned(64),
//
// struct cppi5_tr_resp_t - TR response record
// @status:		Status type and info
// @_reserved:		Not used
// @cmd_id:		Command ID for the TR for TR identification
// @flags:		Configuration Specific Flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cppi5_tr_resp_t {
    pub status: u8,
    pub _reserved: u8,
    pub cmd_id: u8,
    pub flags: u8,
    pub __packed: },

//
// enum cppi5_tr_resp_status_type - TR Response Status Type field is used to
// determine what type of status is being
// returned.
// @CPPI5_TR_RESPONSE_STATUS_NONE:		No error, completion: completed
// @CPPI5_TR_RESPONSE_STATUS_TRANSFER_ERR:	Transfer Error, completion: none
// or partially completed
// @CPPI5_TR_RESPONSE_STATUS_ABORTED_ERR:	Aborted Error, completion: none
// or partially completed
// @CPPI5_TR_RESPONSE_STATUS_SUBMISSION_ERR:	Submission Error, completion:
// none
// @CPPI5_TR_RESPONSE_STATUS_UNSUPPORTED_ERR:	Unsupported Error, completion:
// none
// @CPPI5_TR_RESPONSE_STATUS_TRANSFER_EXCEPTION: Transfer Exception, completion:
// partially completed
// @CPPI5_TR_RESPONSE_STATUS__TEARDOWN_FLUSH:	Teardown Flush, completion: none
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cppi5_tr_resp_status_type {
    CPPI5_TR_RESPONSE_STATUS_NONE,
    CPPI5_TR_RESPONSE_STATUS_TRANSFER_ERR,
    CPPI5_TR_RESPONSE_STATUS_ABORTED_ERR,
    CPPI5_TR_RESPONSE_STATUS_SUBMISSION_ERR,
    CPPI5_TR_RESPONSE_STATUS_UNSUPPORTED_ERR,
    CPPI5_TR_RESPONSE_STATUS_TRANSFER_EXCEPTION,
    CPPI5_TR_RESPONSE_STATUS__TEARDOWN_FLUSH,
// private:
    CPPI5_TR_RESPONSE_STATUS_MAX
}

//
// enum cppi5_tr_resp_status_submission - TR Response Status field values which
// corresponds Submission Error
// @CPPI5_TR_RESPONSE_STATUS_SUBMISSION_ICNT0:	ICNT0 was 0
// @CPPI5_TR_RESPONSE_STATUS_SUBMISSION_FIFO_FULL: Channel FIFO was full when TR
// received
// @CPPI5_TR_RESPONSE_STATUS_SUBMISSION_OWN:	Channel is not owned by the
// submitter
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cppi5_tr_resp_status_submission {
    CPPI5_TR_RESPONSE_STATUS_SUBMISSION_ICNT0,
    CPPI5_TR_RESPONSE_STATUS_SUBMISSION_FIFO_FULL,
    CPPI5_TR_RESPONSE_STATUS_SUBMISSION_OWN,
// private:
    CPPI5_TR_RESPONSE_STATUS_SUBMISSION_MAX
}

//
// enum cppi5_tr_resp_status_unsupported - TR Response Status field values which
// corresponds Unsupported Error
// @CPPI5_TR_RESPONSE_STATUS_UNSUPPORTED_TR_TYPE:	TR Type not supported
// @CPPI5_TR_RESPONSE_STATUS_UNSUPPORTED_STATIC:	STATIC not supported
// @CPPI5_TR_RESPONSE_STATUS_UNSUPPORTED_EOL:		EOL not supported
// @CPPI5_TR_RESPONSE_STATUS_UNSUPPORTED_CFG_SPECIFIC:	CONFIGURATION SPECIFIC
// not supported
// @CPPI5_TR_RESPONSE_STATUS_UNSUPPORTED_AMODE:		AMODE not supported
// @CPPI5_TR_RESPONSE_STATUS_UNSUPPORTED_ELTYPE:	ELTYPE not supported
// @CPPI5_TR_RESPONSE_STATUS_UNSUPPORTED_DFMT:		DFMT not supported
// @CPPI5_TR_RESPONSE_STATUS_UNSUPPORTED_SECTR:		SECTR not supported
// @CPPI5_TR_RESPONSE_STATUS_UNSUPPORTED_AMODE_SPECIFIC: AMODE SPECIFIC field
// not supported
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cppi5_tr_resp_status_unsupported {
    CPPI5_TR_RESPONSE_STATUS_UNSUPPORTED_TR_TYPE,
    CPPI5_TR_RESPONSE_STATUS_UNSUPPORTED_STATIC,
    CPPI5_TR_RESPONSE_STATUS_UNSUPPORTED_EOL,
    CPPI5_TR_RESPONSE_STATUS_UNSUPPORTED_CFG_SPECIFIC,
    CPPI5_TR_RESPONSE_STATUS_UNSUPPORTED_AMODE,
    CPPI5_TR_RESPONSE_STATUS_UNSUPPORTED_ELTYPE,
    CPPI5_TR_RESPONSE_STATUS_UNSUPPORTED_DFMT,
    CPPI5_TR_RESPONSE_STATUS_UNSUPPORTED_SECTR,
    CPPI5_TR_RESPONSE_STATUS_UNSUPPORTED_AMODE_SPECIFIC,
// private:
    CPPI5_TR_RESPONSE_STATUS_UNSUPPORTED_MAX
}

//
// cppi5_trdesc_calc_size - Calculate TR Descriptor size
// @tr_count: number of TR records
// @tr_size: Nominal size of TR record (max) [16, 32, 64, 128]
//
// Returns: required TR Descriptor size
//
// The Size of a TR descriptor is:
// 1 x tr_size : the first 16 bytes is used by the packet info block +
// tr_count x tr_size : Transfer Request Records +
// tr_count x sizeof(struct cppi5_tr_resp_t) : Transfer Response Records
//
    pub tr_count: *mut *mut sizeof(struct cppi5_tr_resp_t),
//
// cppi5_trdesc_init - Init TR Descriptor
// @desc_hdr: TR Descriptor
// @tr_count: number of TR records
// @tr_size: Nominal size of TR record (max) [16, 32, 64, 128]
// @reload_idx: Absolute index to jump to on the 2nd and following passes
// through the TR packet.
// @reload_count: Number of times to jump from last entry to reload_idx. 0x1ff
// indicates infinite looping.
//
// Init TR Descriptor
//
    pub CPPI5_INFO0_TRDESC_LASTIDX_MASK: desc_hdr->pkt_info0 |= (tr_count - 1) &,
//
// cppi5_tr_init - Init TR record
// @flags: Pointer to the TR's flags
// @type: TR type
// @static_tr: TR is static
// @wait: Wait for TR completion before allow the next TR to start
// @event_size: output event generation cfg
// @cmd_id: TR identifier (application specifics)
//
// Init TR record
//
// flags = type;
// flags |= (event_size << CPPI5_TR_EVENT_SIZE_SHIFT) &
// flags |= (cmd_id << CPPI5_TR_CMD_ID_SHIFT) &
// flags |= CPPI5_TR_STATIC;
// flags |= CPPI5_TR_WAIT;
//
// cppi5_tr_set_trigger - Configure trigger0/1 and trigger0/1_type
// @flags: Pointer to the TR's flags
// @trigger0: trigger0 selection
// @trigger0_type: type of data transfer that will be enabled by trigger0
// @trigger1: trigger1 selection
// @trigger1_type: type of data transfer that will be enabled by trigger1
//
// Configure the triggers for the TR
//
// flags &= ~(CPPI5_TR_TRIGGER0_MASK | CPPI5_TR_TRIGGER0_TYPE_MASK |
    pub CPPI5_TR_TRIGGER1_TYPE_MASK): CPPI5_TR_TRIGGER1_MASK |,
// flags |= (trigger0 << CPPI5_TR_TRIGGER0_SHIFT) &
// flags |= (trigger0_type << CPPI5_TR_TRIGGER0_TYPE_SHIFT) &
// flags |= (trigger1 << CPPI5_TR_TRIGGER1_SHIFT) &
// flags |= (trigger1_type << CPPI5_TR_TRIGGER1_TYPE_SHIFT) &
//
// cppi5_tr_csf_set - Update the Configuration specific flags
// @flags: Pointer to the TR's flags
// @csf: Configuration specific flags
//
// Set a bit in Configuration Specific Flags section of the TR flags.
//
// flags &= ~CPPI5_TR_CSF_FLAGS_MASK;
// flags |= (csf << CPPI5_TR_CSF_FLAGS_SHIFT) &
