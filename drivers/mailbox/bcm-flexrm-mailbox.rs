//! Automatically rewritten from C to Rust
//! Source: drivers/mailbox/bcm-flexrm-mailbox.c
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
// Copyright (C) 2017 Broadcom
//
// Broadcom FlexRM Mailbox Driver
//
// Each Broadcom FlexSparx4 offload engine is implemented as an
// extension to Broadcom FlexRM ring manager. The FlexRM ring
// manager provides a set of rings which can be used to submit
// work to a FlexSparx4 offload engine.
//
// This driver creates a mailbox controller using a set of FlexRM
// rings where each mailbox channel represents a separate FlexRM ring.
//

// ====== FlexRM register defines =====
// FlexRM configuration
pub const RING_REGS_SIZE: c_uint = 0x10000;
pub const RING_DESC_SIZE: c_int = 8;

    ((offset) / RING_DESC_SIZE)

    ((index) * RING_DESC_SIZE)
pub const RING_MAX_REQ_COUNT: c_int = 1024;
pub const RING_BD_ALIGN_ORDER: c_int = 12;

    (!((addr) & ((0x1 << RING_BD_ALIGN_ORDER) - 1)))

    (((offset) >> RING_BD_ALIGN_ORDER) & 0x1)

    (!RING_BD_TOGGLE_INVALID(offset))
pub const RING_BD_DESC_PER_REQ: c_int = 32;

    (RING_MAX_REQ_COUNT * RING_BD_DESC_PER_REQ)

    (RING_BD_DESC_COUNT * RING_DESC_SIZE)
pub const RING_CMPL_ALIGN_ORDER: c_int = 13;

    (RING_CMPL_DESC_COUNT * RING_DESC_SIZE)
pub const RING_VER_MAGIC: c_uint = 0x76303031;
// Per-Ring register offsets
pub const RING_VER: c_uint = 0x000;
pub const RING_BD_START_ADDR: c_uint = 0x004;
pub const RING_BD_READ_PTR: c_uint = 0x008;
pub const RING_BD_WRITE_PTR: c_uint = 0x00c;
pub const RING_BD_READ_PTR_DDR_LS: c_uint = 0x010;
pub const RING_BD_READ_PTR_DDR_MS: c_uint = 0x014;
pub const RING_CMPL_START_ADDR: c_uint = 0x018;
pub const RING_CMPL_WRITE_PTR: c_uint = 0x01c;
pub const RING_NUM_REQ_RECV_LS: c_uint = 0x020;
pub const RING_NUM_REQ_RECV_MS: c_uint = 0x024;
pub const RING_NUM_REQ_TRANS_LS: c_uint = 0x028;
pub const RING_NUM_REQ_TRANS_MS: c_uint = 0x02c;
pub const RING_NUM_REQ_OUTSTAND: c_uint = 0x030;
pub const RING_CONTROL: c_uint = 0x034;
pub const RING_FLUSH_DONE: c_uint = 0x038;
pub const RING_MSI_ADDR_LS: c_uint = 0x03c;
pub const RING_MSI_ADDR_MS: c_uint = 0x040;
pub const RING_MSI_CONTROL: c_uint = 0x048;
pub const RING_BD_READ_PTR_DDR_CONTROL: c_uint = 0x04c;
pub const RING_MSI_DATA_VALUE: c_uint = 0x064;
// Register RING_BD_START_ADDR fields
pub const BD_LAST_UPDATE_HW_SHIFT: c_int = 28;
pub const BD_LAST_UPDATE_HW_MASK: c_uint = 0x1;

    ((u32)((((dma_addr_t)(pa)) >> RING_BD_ALIGN_ORDER) & 0x0fffffff))

    ((dma_addr_t)((val) & 0x0fffffff) << RING_BD_ALIGN_ORDER)
// Register RING_CMPL_START_ADDR fields

    ((u32)((((u64)(pa)) >> RING_CMPL_ALIGN_ORDER) & 0x07ffffff))
// Register RING_CONTROL fields
pub const CONTROL_MASK_DISABLE_CONTROL: c_int = 12;
pub const CONTROL_FLUSH_SHIFT: c_int = 5;
pub const CONTROL_ACTIVE_SHIFT: c_int = 4;
pub const CONTROL_RATE_ADAPT_MASK: c_uint = 0xf;
pub const CONTROL_RATE_DYNAMIC: c_uint = 0x0;
pub const CONTROL_RATE_FAST: c_uint = 0x8;
pub const CONTROL_RATE_MEDIUM: c_uint = 0x9;
pub const CONTROL_RATE_SLOW: c_uint = 0xa;
pub const CONTROL_RATE_IDLE: c_uint = 0xb;
// Register RING_FLUSH_DONE fields
pub const FLUSH_DONE_MASK: c_uint = 0x1;
// Register RING_MSI_CONTROL fields
pub const MSI_TIMER_VAL_SHIFT: c_int = 16;
pub const MSI_TIMER_VAL_MASK: c_uint = 0xffff;
pub const MSI_ENABLE_SHIFT: c_int = 15;
pub const MSI_ENABLE_MASK: c_uint = 0x1;
pub const MSI_COUNT_SHIFT: c_int = 0;
pub const MSI_COUNT_MASK: c_uint = 0x3ff;
// Register RING_BD_READ_PTR_DDR_CONTROL fields
pub const BD_READ_PTR_DDR_TIMER_VAL_SHIFT: c_int = 16;
pub const BD_READ_PTR_DDR_TIMER_VAL_MASK: c_uint = 0xffff;
pub const BD_READ_PTR_DDR_ENABLE_SHIFT: c_int = 15;
pub const BD_READ_PTR_DDR_ENABLE_MASK: c_uint = 0x1;
// ====== FlexRM ring descriptor defines =====
// Completion descriptor format
pub const CMPL_OPAQUE_SHIFT: c_int = 0;
pub const CMPL_OPAQUE_MASK: c_uint = 0xffff;
pub const CMPL_ENGINE_STATUS_SHIFT: c_int = 16;
pub const CMPL_ENGINE_STATUS_MASK: c_uint = 0xffff;
pub const CMPL_DME_STATUS_SHIFT: c_int = 32;
pub const CMPL_DME_STATUS_MASK: c_uint = 0xffff;
pub const CMPL_RM_STATUS_SHIFT: c_int = 48;
pub const CMPL_RM_STATUS_MASK: c_uint = 0xffff;
// Completion DME status code

    DME_STATUS_MEM_UCOR_ERR | \
    DME_STATUS_FIFO_UNDERFLOW | \
    DME_STATUS_FIFO_OVERFLOW | \
    DME_STATUS_RRESP_ERR | \
    DME_STATUS_BRESP_ERR)
// Completion RM status code
pub const RM_STATUS_CODE_SHIFT: c_int = 0;
pub const RM_STATUS_CODE_MASK: c_uint = 0x3ff;
pub const RM_STATUS_CODE_GOOD: c_uint = 0x0;
pub const RM_STATUS_CODE_AE_TIMEOUT: c_uint = 0x3ff;
// General descriptor format
pub const DESC_TYPE_SHIFT: c_int = 60;
pub const DESC_TYPE_MASK: c_uint = 0xf;
pub const DESC_PAYLOAD_SHIFT: c_int = 0;
pub const DESC_PAYLOAD_MASK: c_uint = 0x0fffffffffffffff;
// Null descriptor format
pub const NULL_TYPE: c_int = 0;
pub const NULL_TOGGLE_SHIFT: c_int = 58;
pub const NULL_TOGGLE_MASK: c_uint = 0x1;
// Header descriptor format
pub const HEADER_TYPE: c_int = 1;
pub const HEADER_TOGGLE_SHIFT: c_int = 58;
pub const HEADER_TOGGLE_MASK: c_uint = 0x1;
pub const HEADER_ENDPKT_SHIFT: c_int = 57;
pub const HEADER_ENDPKT_MASK: c_uint = 0x1;
pub const HEADER_STARTPKT_SHIFT: c_int = 56;
pub const HEADER_STARTPKT_MASK: c_uint = 0x1;
pub const HEADER_BDCOUNT_SHIFT: c_int = 36;
pub const HEADER_BDCOUNT_MASK: c_uint = 0x1f;

pub const HEADER_FLAGS_SHIFT: c_int = 16;
pub const HEADER_FLAGS_MASK: c_uint = 0xffff;
pub const HEADER_OPAQUE_SHIFT: c_int = 0;
pub const HEADER_OPAQUE_MASK: c_uint = 0xffff;
// Source (SRC) descriptor format
pub const SRC_TYPE: c_int = 2;
pub const SRC_LENGTH_SHIFT: c_int = 44;
pub const SRC_LENGTH_MASK: c_uint = 0xffff;
pub const SRC_ADDR_SHIFT: c_int = 0;
pub const SRC_ADDR_MASK: c_uint = 0x00000fffffffffff;
// Destination (DST) descriptor format
pub const DST_TYPE: c_int = 3;
pub const DST_LENGTH_SHIFT: c_int = 44;
pub const DST_LENGTH_MASK: c_uint = 0xffff;
pub const DST_ADDR_SHIFT: c_int = 0;
pub const DST_ADDR_MASK: c_uint = 0x00000fffffffffff;
// Immediate (IMM) descriptor format
pub const IMM_TYPE: c_int = 4;
pub const IMM_DATA_SHIFT: c_int = 0;
pub const IMM_DATA_MASK: c_uint = 0x0fffffffffffffff;
// Next pointer (NPTR) descriptor format
pub const NPTR_TYPE: c_int = 5;
pub const NPTR_TOGGLE_SHIFT: c_int = 58;
pub const NPTR_TOGGLE_MASK: c_uint = 0x1;
pub const NPTR_ADDR_SHIFT: c_int = 0;
pub const NPTR_ADDR_MASK: c_uint = 0x00000fffffffffff;
// Mega source (MSRC) descriptor format
pub const MSRC_TYPE: c_int = 6;
pub const MSRC_LENGTH_SHIFT: c_int = 44;
pub const MSRC_LENGTH_MASK: c_uint = 0xffff;
pub const MSRC_ADDR_SHIFT: c_int = 0;
pub const MSRC_ADDR_MASK: c_uint = 0x00000fffffffffff;
// Mega destination (MDST) descriptor format
pub const MDST_TYPE: c_int = 7;
pub const MDST_LENGTH_SHIFT: c_int = 44;
pub const MDST_LENGTH_MASK: c_uint = 0xffff;
pub const MDST_ADDR_SHIFT: c_int = 0;
pub const MDST_ADDR_MASK: c_uint = 0x00000fffffffffff;
// Source with tlast (SRCT) descriptor format
pub const SRCT_TYPE: c_int = 8;
pub const SRCT_LENGTH_SHIFT: c_int = 44;
pub const SRCT_LENGTH_MASK: c_uint = 0xffff;
pub const SRCT_ADDR_SHIFT: c_int = 0;
pub const SRCT_ADDR_MASK: c_uint = 0x00000fffffffffff;
// Destination with tlast (DSTT) descriptor format
pub const DSTT_TYPE: c_int = 9;
pub const DSTT_LENGTH_SHIFT: c_int = 44;
pub const DSTT_LENGTH_MASK: c_uint = 0xffff;
pub const DSTT_ADDR_SHIFT: c_int = 0;
pub const DSTT_ADDR_MASK: c_uint = 0x00000fffffffffff;
// Immediate with tlast (IMMT) descriptor format
pub const IMMT_TYPE: c_int = 10;
pub const IMMT_DATA_SHIFT: c_int = 0;
pub const IMMT_DATA_MASK: c_uint = 0x0fffffffffffffff;
// Descriptor helper macros

    do { \
    (_d) &= ~((u64)(_m) << (_s)); \
    (_d) |= (((u64)(_v) & (_m)) << (_s)); \
    } while (0)
// ====== FlexRM data structures =====
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flexrm_ring {
// Unprotected members
    pub num: c_int,
    pub mbox: *mut flexrm_mbox,
    pub regs: *mut void __iomem,
    pub irq_requested: bool,
    pub irq: c_uint,
    pub irq_aff_hint: cpumask_t,
    pub msi_timer_val: c_uint,
    pub msi_count_threshold: c_uint,
    pub requests: [*mut brcm_message; RING_MAX_REQ_COUNT],
    pub bd_base: *mut c_void,
    pub bd_dma_base: dma_addr_t,
    pub bd_write_offset: u32,
    pub cmpl_base: *mut c_void,
    pub cmpl_dma_base: dma_addr_t,
// Atomic stats
    pub msg_send_count: core::sync::atomic::AtomicI32,
    pub msg_cmpl_count: core::sync::atomic::AtomicI32,
// Protected members
    pub lock: spinlock_t,
    pub RING_MAX_REQ_COUNT): DECLARE_BITMAP(requests_bmap,,
    pub cmpl_read_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flexrm_mbox {
    pub dev: *mut device,
    pub regs: *mut void __iomem,
    pub num_rings: u32,
    pub rings: *mut flexrm_ring,
    pub bd_pool: *mut dma_pool,
    pub cmpl_pool: *mut dma_pool,
    pub root: *mut dentry,
    pub controller: mbox_controller,
}

// ====== FlexRM ring descriptor helper routines =====
#[no_mangle]
unsafe extern "C" fn flexrm_read_desc(desc_ptr: *mut c_void) -> u64 {
    static u64 flexrm_read_desc(void *desc_ptr)
    {
    return le64_to_cpu(*((u64 *)desc_ptr));
    }
#[no_mangle]
unsafe extern "C" fn flexrm_write_desc(desc_ptr: *mut c_void, desc: u64) {
    static void flexrm_write_desc(void *desc_ptr, u64 desc)
    {
// ((u64 *)desc_ptr) = cpu_to_le64(desc);
    }
#[no_mangle]
unsafe extern "C" fn flexrm_cmpl_desc_to_reqid(cmpl_desc: u64) -> u32 {
    static u32 flexrm_cmpl_desc_to_reqid(u64 cmpl_desc)
    {
    return (u32)(cmpl_desc & CMPL_OPAQUE_MASK);
    }
#[no_mangle]
unsafe extern "C" fn flexrm_cmpl_desc_to_error(cmpl_desc: u64) -> c_int {
    static int flexrm_cmpl_desc_to_error(u64 cmpl_desc)
    {
    u32 status;
    status = DESC_DEC(cmpl_desc, CMPL_DME_STATUS_SHIFT,
    CMPL_DME_STATUS_MASK);
    if (status & DME_STATUS_ERROR_MASK)
    return -EIO;
    status = DESC_DEC(cmpl_desc, CMPL_RM_STATUS_SHIFT,
    CMPL_RM_STATUS_MASK);
    status &= RM_STATUS_CODE_MASK;
    if (status == RM_STATUS_CODE_AE_TIMEOUT)
    return -ETIMEDOUT;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn flexrm_is_next_table_desc(desc_ptr: *mut c_void) -> bool {
    static bool flexrm_is_next_table_desc(void *desc_ptr)
    {
    let mut desc: u64 = flexrm_read_desc(desc_ptr);
    let mut type: u32 = DESC_DEC(desc, DESC_TYPE_SHIFT, DESC_TYPE_MASK);
    return (type == NPTR_TYPE) ? true : false;
    }
#[no_mangle]
unsafe extern "C" fn flexrm_next_table_desc(toggle: u32, next_addr: dma_addr_t) -> u64 {
    static u64 flexrm_next_table_desc(u32 toggle, dma_addr_t next_addr)
    {
    let mut desc: u64 = 0;
    DESC_ENC(desc, NPTR_TYPE, DESC_TYPE_SHIFT, DESC_TYPE_MASK);
    DESC_ENC(desc, toggle, NPTR_TOGGLE_SHIFT, NPTR_TOGGLE_MASK);
    DESC_ENC(desc, next_addr, NPTR_ADDR_SHIFT, NPTR_ADDR_MASK);
    return desc;
    }
#[no_mangle]
unsafe extern "C" fn flexrm_null_desc(toggle: u32) -> u64 {
    static u64 flexrm_null_desc(u32 toggle)
    {
    let mut desc: u64 = 0;
    DESC_ENC(desc, NULL_TYPE, DESC_TYPE_SHIFT, DESC_TYPE_MASK);
    DESC_ENC(desc, toggle, NULL_TOGGLE_SHIFT, NULL_TOGGLE_MASK);
    return desc;
    }
#[no_mangle]
unsafe extern "C" fn flexrm_estimate_header_desc_count(nhcnt: u32) -> u32 {
    static u32 flexrm_estimate_header_desc_count(u32 nhcnt)
    {
    let mut hcnt: u32 = nhcnt / HEADER_BDCOUNT_MAX;
    if (!(nhcnt % HEADER_BDCOUNT_MAX))
    hcnt += 1;
    return hcnt;
    }
#[no_mangle]
unsafe extern "C" fn flexrm_flip_header_toggle(desc_ptr: *mut c_void) {
    static void flexrm_flip_header_toggle(void *desc_ptr)
    {
    let mut desc: u64 = flexrm_read_desc(desc_ptr);
    if (desc & ((u64)0x1 << HEADER_TOGGLE_SHIFT))
    desc &= ~((u64)0x1 << HEADER_TOGGLE_SHIFT);
    else
    desc |= ((u64)0x1 << HEADER_TOGGLE_SHIFT);
    flexrm_write_desc(desc_ptr, desc);
    }
    static u64 flexrm_header_desc(u32 toggle, u32 startpkt, u32 endpkt,
    u32 bdcount, u32 flags, u32 opaque)
    {
    let mut desc: u64 = 0;
    DESC_ENC(desc, HEADER_TYPE, DESC_TYPE_SHIFT, DESC_TYPE_MASK);
    DESC_ENC(desc, toggle, HEADER_TOGGLE_SHIFT, HEADER_TOGGLE_MASK);
    DESC_ENC(desc, startpkt, HEADER_STARTPKT_SHIFT, HEADER_STARTPKT_MASK);
    DESC_ENC(desc, endpkt, HEADER_ENDPKT_SHIFT, HEADER_ENDPKT_MASK);
    DESC_ENC(desc, bdcount, HEADER_BDCOUNT_SHIFT, HEADER_BDCOUNT_MASK);
    DESC_ENC(desc, flags, HEADER_FLAGS_SHIFT, HEADER_FLAGS_MASK);
    DESC_ENC(desc, opaque, HEADER_OPAQUE_SHIFT, HEADER_OPAQUE_MASK);
    return desc;
    }
    static void flexrm_enqueue_desc(u32 nhpos, u32 nhcnt, u32 reqid,
    u64 desc, void **desc_ptr, u32 *toggle,
    void *start_desc, void *end_desc)
    {
    u64 d;
    u32 nhavail, _toggle, _startpkt, _endpkt, _bdcount;
// Sanity check
    if (nhcnt <= nhpos)
    return;
//
// Each request or packet start with a HEADER descriptor followed
// by one or more non-HEADER descriptors (SRC, SRCT, MSRC, DST,
// DSTT, MDST, IMM, and IMMT). The number of non-HEADER descriptors
// following a HEADER descriptor is represented by BDCOUNT field
// of HEADER descriptor. The max value of BDCOUNT field is 31 which
// means we can only have 31 non-HEADER descriptors following one
// HEADER descriptor.
//
// In general use, number of non-HEADER descriptors can easily go
// beyond 31. To tackle this situation, we have packet (or request)
// extension bits (STARTPKT and ENDPKT) in the HEADER descriptor.
//
// To use packet extension, the first HEADER descriptor of request
// (or packet) will have STARTPKT=1 and ENDPKT=0. The intermediate
// HEADER descriptors will have STARTPKT=0 and ENDPKT=0. The last
// HEADER descriptor will have STARTPKT=0 and ENDPKT=1. Also, the
// TOGGLE bit of the first HEADER will be set to invalid state to
// ensure that FlexRM does not start fetching descriptors till all
// descriptors are enqueued. The user of this function will flip
// the TOGGLE bit of first HEADER after all descriptors are
// enqueued.
//
    if ((nhpos % HEADER_BDCOUNT_MAX == 0) && (nhcnt - nhpos)) {
// Prepare the header descriptor
    nhavail = (nhcnt - nhpos);
    _toggle = (nhpos == 0) ? !(*toggle) : (*toggle);
    _startpkt = (nhpos == 0) ? 0x1 : 0x0;
    _endpkt = (nhavail <= HEADER_BDCOUNT_MAX) ? 0x1 : 0x0;
    _bdcount = (nhavail <= HEADER_BDCOUNT_MAX) ?
    nhavail : HEADER_BDCOUNT_MAX;
    if (nhavail <= HEADER_BDCOUNT_MAX)
    _bdcount = nhavail;
    else
    _bdcount = HEADER_BDCOUNT_MAX;
    d = flexrm_header_desc(_toggle, _startpkt, _endpkt,
    _bdcount, 0x0, reqid);
// Write header descriptor
    flexrm_write_desc(*desc_ptr, d);
// Point to next descriptor
// desc_ptr += sizeof(desc);
    if (*desc_ptr == end_desc)
// desc_ptr = start_desc;
// Skip next pointer descriptors
    while (flexrm_is_next_table_desc(*desc_ptr)) {
// toggle = (*toggle) ? 0 : 1;
// desc_ptr += sizeof(desc);
    if (*desc_ptr == end_desc)
// desc_ptr = start_desc;
    }
    }
// Write desired descriptor
    flexrm_write_desc(*desc_ptr, desc);
// Point to next descriptor
// desc_ptr += sizeof(desc);
    if (*desc_ptr == end_desc)
// desc_ptr = start_desc;
// Skip next pointer descriptors
    while (flexrm_is_next_table_desc(*desc_ptr)) {
// toggle = (*toggle) ? 0 : 1;
// desc_ptr += sizeof(desc);
    if (*desc_ptr == end_desc)
// desc_ptr = start_desc;
    }
    }
#[no_mangle]
unsafe extern "C" fn flexrm_src_desc(addr: dma_addr_t, length: c_uint) -> u64 {
    static u64 flexrm_src_desc(dma_addr_t addr, unsigned int length)
    {
    let mut desc: u64 = 0;
    DESC_ENC(desc, SRC_TYPE, DESC_TYPE_SHIFT, DESC_TYPE_MASK);
    DESC_ENC(desc, length, SRC_LENGTH_SHIFT, SRC_LENGTH_MASK);
    DESC_ENC(desc, addr, SRC_ADDR_SHIFT, SRC_ADDR_MASK);
    return desc;
    }
#[no_mangle]
unsafe extern "C" fn flexrm_msrc_desc(addr: dma_addr_t, length_div_16: c_uint) -> u64 {
    static u64 flexrm_msrc_desc(dma_addr_t addr, unsigned int length_div_16)
    {
    let mut desc: u64 = 0;
    DESC_ENC(desc, MSRC_TYPE, DESC_TYPE_SHIFT, DESC_TYPE_MASK);
    DESC_ENC(desc, length_div_16, MSRC_LENGTH_SHIFT, MSRC_LENGTH_MASK);
    DESC_ENC(desc, addr, MSRC_ADDR_SHIFT, MSRC_ADDR_MASK);
    return desc;
    }
#[no_mangle]
unsafe extern "C" fn flexrm_dst_desc(addr: dma_addr_t, length: c_uint) -> u64 {
    static u64 flexrm_dst_desc(dma_addr_t addr, unsigned int length)
    {
    let mut desc: u64 = 0;
    DESC_ENC(desc, DST_TYPE, DESC_TYPE_SHIFT, DESC_TYPE_MASK);
    DESC_ENC(desc, length, DST_LENGTH_SHIFT, DST_LENGTH_MASK);
    DESC_ENC(desc, addr, DST_ADDR_SHIFT, DST_ADDR_MASK);
    return desc;
    }
#[no_mangle]
unsafe extern "C" fn flexrm_mdst_desc(addr: dma_addr_t, length_div_16: c_uint) -> u64 {
    static u64 flexrm_mdst_desc(dma_addr_t addr, unsigned int length_div_16)
    {
    let mut desc: u64 = 0;
    DESC_ENC(desc, MDST_TYPE, DESC_TYPE_SHIFT, DESC_TYPE_MASK);
    DESC_ENC(desc, length_div_16, MDST_LENGTH_SHIFT, MDST_LENGTH_MASK);
    DESC_ENC(desc, addr, MDST_ADDR_SHIFT, MDST_ADDR_MASK);
    return desc;
    }
#[no_mangle]
unsafe extern "C" fn flexrm_imm_desc(data: u64) -> u64 {
    static u64 flexrm_imm_desc(u64 data)
    {
    let mut desc: u64 = 0;
    DESC_ENC(desc, IMM_TYPE, DESC_TYPE_SHIFT, DESC_TYPE_MASK);
    DESC_ENC(desc, data, IMM_DATA_SHIFT, IMM_DATA_MASK);
    return desc;
    }
#[no_mangle]
unsafe extern "C" fn flexrm_srct_desc(addr: dma_addr_t, length: c_uint) -> u64 {
    static u64 flexrm_srct_desc(dma_addr_t addr, unsigned int length)
    {
    let mut desc: u64 = 0;
    DESC_ENC(desc, SRCT_TYPE, DESC_TYPE_SHIFT, DESC_TYPE_MASK);
    DESC_ENC(desc, length, SRCT_LENGTH_SHIFT, SRCT_LENGTH_MASK);
    DESC_ENC(desc, addr, SRCT_ADDR_SHIFT, SRCT_ADDR_MASK);
    return desc;
    }
#[no_mangle]
unsafe extern "C" fn flexrm_dstt_desc(addr: dma_addr_t, length: c_uint) -> u64 {
    static u64 flexrm_dstt_desc(dma_addr_t addr, unsigned int length)
    {
    let mut desc: u64 = 0;
    DESC_ENC(desc, DSTT_TYPE, DESC_TYPE_SHIFT, DESC_TYPE_MASK);
    DESC_ENC(desc, length, DSTT_LENGTH_SHIFT, DSTT_LENGTH_MASK);
    DESC_ENC(desc, addr, DSTT_ADDR_SHIFT, DSTT_ADDR_MASK);
    return desc;
    }
#[no_mangle]
unsafe extern "C" fn flexrm_immt_desc(data: u64) -> u64 {
    static u64 flexrm_immt_desc(u64 data)
    {
    let mut desc: u64 = 0;
    DESC_ENC(desc, IMMT_TYPE, DESC_TYPE_SHIFT, DESC_TYPE_MASK);
    DESC_ENC(desc, data, IMMT_DATA_SHIFT, IMMT_DATA_MASK);
    return desc;
    }
#[no_mangle]
unsafe extern "C" fn flexrm_spu_sanity_check(msg: *mut brcm_message) -> bool {
    static bool flexrm_spu_sanity_check(struct brcm_message *msg)
    {
    struct scatterlist *sg;
    if (!msg.spu.src || !msg.spu.dst)
    return false;
    for (sg = msg.spu.src; sg; sg = sg_next(sg)) {
    if (sg.length & 0xf) {
    if (sg.length > SRC_LENGTH_MASK)
    return false;
    } else {
    if (sg.length > (MSRC_LENGTH_MASK * 16))
    return false;
    }
    }
    for (sg = msg.spu.dst; sg; sg = sg_next(sg)) {
    if (sg.length & 0xf) {
    if (sg.length > DST_LENGTH_MASK)
    return false;
    } else {
    if (sg.length > (MDST_LENGTH_MASK * 16))
    return false;
    }
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn flexrm_spu_estimate_nonheader_desc_count(msg: *mut brcm_message) -> u32 {
    static u32 flexrm_spu_estimate_nonheader_desc_count(struct brcm_message *msg)
    {
    let mut cnt: u32 = 0;
    let mut dst_target: c_uint = 0;
    struct scatterlist *src_sg = msg.spu.src, *dst_sg = msg.spu.dst;
    while (src_sg || dst_sg) {
    if (src_sg) {
    cnt++;
    dst_target = src_sg.length;
    src_sg = sg_next(src_sg);
    } else
    dst_target = UINT_MAX;
    while (dst_target && dst_sg) {
    cnt++;
    if (dst_sg.length < dst_target)
    dst_target -= dst_sg.length;
    else
    dst_target = 0;
    dst_sg = sg_next(dst_sg);
    }
    }
    return cnt;
    }
#[no_mangle]
unsafe extern "C" fn flexrm_spu_dma_map(dev: *mut device, msg: *mut brcm_message) -> c_int {
    static int flexrm_spu_dma_map(struct device *dev, struct brcm_message *msg)
    {
    int rc;
    rc = dma_map_sg(dev, msg.spu.src, sg_nents(msg.spu.src),
    DMA_TO_DEVICE);
    if (!rc)
    return -EIO;
    rc = dma_map_sg(dev, msg.spu.dst, sg_nents(msg.spu.dst),
    DMA_FROM_DEVICE);
    if (!rc) {
    dma_unmap_sg(dev, msg.spu.src, sg_nents(msg.spu.src),
    DMA_TO_DEVICE);
    return -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn flexrm_spu_dma_unmap(dev: *mut device, msg: *mut brcm_message) {
    static void flexrm_spu_dma_unmap(struct device *dev, struct brcm_message *msg)
    {
    dma_unmap_sg(dev, msg.spu.dst, sg_nents(msg.spu.dst),
    DMA_FROM_DEVICE);
    dma_unmap_sg(dev, msg.spu.src, sg_nents(msg.spu.src),
    DMA_TO_DEVICE);
    }
    static void *flexrm_spu_write_descs(struct brcm_message *msg, u32 nhcnt,
    u32 reqid, void *desc_ptr, u32 toggle,
    void *start_desc, void *end_desc)
    {
    u64 d;
    let mut nhpos: u32 = 0;
    void *orig_desc_ptr = desc_ptr;
    let mut dst_target: c_uint = 0;
    struct scatterlist *src_sg = msg.spu.src, *dst_sg = msg.spu.dst;
    while (src_sg || dst_sg) {
    if (src_sg) {
    if (sg_dma_len(src_sg) & 0xf)
    d = flexrm_src_desc(sg_dma_address(src_sg),
    sg_dma_len(src_sg));
    else
    d = flexrm_msrc_desc(sg_dma_address(src_sg),
    sg_dma_len(src_sg)/16);
    flexrm_enqueue_desc(nhpos, nhcnt, reqid,
    d, &desc_ptr, &toggle,
    start_desc, end_desc);
    nhpos++;
    dst_target = sg_dma_len(src_sg);
    src_sg = sg_next(src_sg);
    } else
    dst_target = UINT_MAX;
    while (dst_target && dst_sg) {
    if (sg_dma_len(dst_sg) & 0xf)
    d = flexrm_dst_desc(sg_dma_address(dst_sg),
    sg_dma_len(dst_sg));
    else
    d = flexrm_mdst_desc(sg_dma_address(dst_sg),
    sg_dma_len(dst_sg)/16);
    flexrm_enqueue_desc(nhpos, nhcnt, reqid,
    d, &desc_ptr, &toggle,
    start_desc, end_desc);
    nhpos++;
    if (sg_dma_len(dst_sg) < dst_target)
    dst_target -= sg_dma_len(dst_sg);
    else
    dst_target = 0;
    dst_sg = sg_next(dst_sg);
    }
    }
// Null descriptor with invalid toggle bit
    flexrm_write_desc(desc_ptr, flexrm_null_desc(!toggle));
// Ensure that descriptors have been written to memory
    wmb();
// Flip toggle bit in header
    flexrm_flip_header_toggle(orig_desc_ptr);
    return desc_ptr;
    }
#[no_mangle]
unsafe extern "C" fn flexrm_sba_sanity_check(msg: *mut brcm_message) -> bool {
    static bool flexrm_sba_sanity_check(struct brcm_message *msg)
    {
    u32 i;
    if (!msg.sba.cmds || !msg.sba.cmds_count)
    return false;
    for (i = 0; i < msg.sba.cmds_count; i++) {
    if (((msg.sba.cmds[i].flags & BRCM_SBA_CMD_TYPE_B) ||
    (msg.sba.cmds[i].flags & BRCM_SBA_CMD_TYPE_C)) &&
    (msg.sba.cmds[i].flags & BRCM_SBA_CMD_HAS_OUTPUT))
    return false;
    if ((msg.sba.cmds[i].flags & BRCM_SBA_CMD_TYPE_B) &&
    (msg.sba.cmds[i].data_len > SRCT_LENGTH_MASK))
    return false;
    if ((msg.sba.cmds[i].flags & BRCM_SBA_CMD_TYPE_C) &&
    (msg.sba.cmds[i].data_len > SRCT_LENGTH_MASK))
    return false;
    if ((msg.sba.cmds[i].flags & BRCM_SBA_CMD_HAS_RESP) &&
    (msg.sba.cmds[i].resp_len > DSTT_LENGTH_MASK))
    return false;
    if ((msg.sba.cmds[i].flags & BRCM_SBA_CMD_HAS_OUTPUT) &&
    (msg.sba.cmds[i].data_len > DSTT_LENGTH_MASK))
    return false;
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn flexrm_sba_estimate_nonheader_desc_count(msg: *mut brcm_message) -> u32 {
    static u32 flexrm_sba_estimate_nonheader_desc_count(struct brcm_message *msg)
    {
    u32 i, cnt;
    cnt = 0;
    for (i = 0; i < msg.sba.cmds_count; i++) {
    cnt++;
    if ((msg.sba.cmds[i].flags & BRCM_SBA_CMD_TYPE_B) ||
    (msg.sba.cmds[i].flags & BRCM_SBA_CMD_TYPE_C))
    cnt++;
    if (msg.sba.cmds[i].flags & BRCM_SBA_CMD_HAS_RESP)
    cnt++;
    if (msg.sba.cmds[i].flags & BRCM_SBA_CMD_HAS_OUTPUT)
    cnt++;
    }
    return cnt;
    }
    static void *flexrm_sba_write_descs(struct brcm_message *msg, u32 nhcnt,
    u32 reqid, void *desc_ptr, u32 toggle,
    void *start_desc, void *end_desc)
    {
    u64 d;
    u32 i, nhpos = 0;
    struct brcm_sba_command *c;
    void *orig_desc_ptr = desc_ptr;
// Convert SBA commands into descriptors
    for (i = 0; i < msg.sba.cmds_count; i++) {
    c = &msg.sba.cmds[i];
    if ((c.flags & BRCM_SBA_CMD_HAS_RESP) &&
    (c.flags & BRCM_SBA_CMD_HAS_OUTPUT)) {
// Destination response descriptor
    d = flexrm_dst_desc(c.resp, c.resp_len);
    flexrm_enqueue_desc(nhpos, nhcnt, reqid,
    d, &desc_ptr, &toggle,
    start_desc, end_desc);
    nhpos++;
    } else if (c.flags & BRCM_SBA_CMD_HAS_RESP) {
// Destination response with tlast descriptor
    d = flexrm_dstt_desc(c.resp, c.resp_len);
    flexrm_enqueue_desc(nhpos, nhcnt, reqid,
    d, &desc_ptr, &toggle,
    start_desc, end_desc);
    nhpos++;
    }
    if (c.flags & BRCM_SBA_CMD_HAS_OUTPUT) {
// Destination with tlast descriptor
    d = flexrm_dstt_desc(c.data, c.data_len);
    flexrm_enqueue_desc(nhpos, nhcnt, reqid,
    d, &desc_ptr, &toggle,
    start_desc, end_desc);
    nhpos++;
    }
    if (c.flags & BRCM_SBA_CMD_TYPE_B) {
// Command as immediate descriptor
    d = flexrm_imm_desc(c.cmd);
    flexrm_enqueue_desc(nhpos, nhcnt, reqid,
    d, &desc_ptr, &toggle,
    start_desc, end_desc);
    nhpos++;
    } else {
// Command as immediate descriptor with tlast
    d = flexrm_immt_desc(c.cmd);
    flexrm_enqueue_desc(nhpos, nhcnt, reqid,
    d, &desc_ptr, &toggle,
    start_desc, end_desc);
    nhpos++;
    }
    if ((c.flags & BRCM_SBA_CMD_TYPE_B) ||
    (c.flags & BRCM_SBA_CMD_TYPE_C)) {
// Source with tlast descriptor
    d = flexrm_srct_desc(c.data, c.data_len);
    flexrm_enqueue_desc(nhpos, nhcnt, reqid,
    d, &desc_ptr, &toggle,
    start_desc, end_desc);
    nhpos++;
    }
    }
// Null descriptor with invalid toggle bit
    flexrm_write_desc(desc_ptr, flexrm_null_desc(!toggle));
// Ensure that descriptors have been written to memory
    wmb();
// Flip toggle bit in header
    flexrm_flip_header_toggle(orig_desc_ptr);
    return desc_ptr;
    }
#[no_mangle]
unsafe extern "C" fn flexrm_sanity_check(msg: *mut brcm_message) -> bool {
    static bool flexrm_sanity_check(struct brcm_message *msg)
    {
    if (!msg)
    return false;
    switch (msg.type) {
    case BRCM_MESSAGE_SPU:
    return flexrm_spu_sanity_check(msg);
    case BRCM_MESSAGE_SBA:
    return flexrm_sba_sanity_check(msg);
    default:
    return false;
    };
    }
#[no_mangle]
unsafe extern "C" fn flexrm_estimate_nonheader_desc_count(msg: *mut brcm_message) -> u32 {
    static u32 flexrm_estimate_nonheader_desc_count(struct brcm_message *msg)
    {
    if (!msg)
    return 0;
    switch (msg.type) {
    case BRCM_MESSAGE_SPU:
    return flexrm_spu_estimate_nonheader_desc_count(msg);
    case BRCM_MESSAGE_SBA:
    return flexrm_sba_estimate_nonheader_desc_count(msg);
    default:
    return 0;
    };
    }
#[no_mangle]
unsafe extern "C" fn flexrm_dma_map(dev: *mut device, msg: *mut brcm_message) -> c_int {
    static int flexrm_dma_map(struct device *dev, struct brcm_message *msg)
    {
    if (!dev || !msg)
    return -EINVAL;
    switch (msg.type) {
    case BRCM_MESSAGE_SPU:
    return flexrm_spu_dma_map(dev, msg);
    default:
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn flexrm_dma_unmap(dev: *mut device, msg: *mut brcm_message) {
    static void flexrm_dma_unmap(struct device *dev, struct brcm_message *msg)
    {
    if (!dev || !msg)
    return;
    switch (msg.type) {
    case BRCM_MESSAGE_SPU:
    flexrm_spu_dma_unmap(dev, msg);
    break;
    default:
    break;
    }
    }
    static void *flexrm_write_descs(struct brcm_message *msg, u32 nhcnt,
    u32 reqid, void *desc_ptr, u32 toggle,
    void *start_desc, void *end_desc)
    {
    if (!msg || !desc_ptr || !start_desc || !end_desc)
    return ERR_PTR(-ENOTSUPP);
    if ((desc_ptr < start_desc) || (end_desc <= desc_ptr))
    return ERR_PTR(-ERANGE);
    switch (msg.type) {
    case BRCM_MESSAGE_SPU:
    return flexrm_spu_write_descs(msg, nhcnt, reqid,
    desc_ptr, toggle,
    start_desc, end_desc);
    case BRCM_MESSAGE_SBA:
    return flexrm_sba_write_descs(msg, nhcnt, reqid,
    desc_ptr, toggle,
    start_desc, end_desc);
    default:
    return ERR_PTR(-ENOTSUPP);
    };
    }
// ====== FlexRM driver helper routines =====
    static void flexrm_write_config_in_seqfile(struct flexrm_mbox *mbox,
    struct seq_file *file)
    {
    int i;
    const char *state;
    struct flexrm_ring *ring;
    seq_printf(file, "%-5s %-9s %-18s %-10s %-18s %-10s\n",
    "Ring#", "State", "BD_Addr", "BD_Size",
    "Cmpl_Addr", "Cmpl_Size");
    for (i = 0; i < mbox.num_rings; i++) {
    ring = &mbox.rings[i];
    if (readl(ring.regs + RING_CONTROL) &
    BIT(CONTROL_ACTIVE_SHIFT))
    state = "active";
    else
    state = "inactive";
    seq_printf(file,
    "%-5d %-9s 0x%016llx 0x%08x 0x%016llx 0x%08x\n",
    ring.num, state,
    (unsigned long long)ring.bd_dma_base,
    (u32)RING_BD_SIZE,
    (unsigned long long)ring.cmpl_dma_base,
    (u32)RING_CMPL_SIZE);
    }
    }
    static void flexrm_write_stats_in_seqfile(struct flexrm_mbox *mbox,
    struct seq_file *file)
    {
    int i;
    u32 val, bd_read_offset;
    struct flexrm_ring *ring;
    seq_printf(file, "%-5s %-10s %-10s %-10s %-11s %-11s\n",
    "Ring#", "BD_Read", "BD_Write",
    "Cmpl_Read", "Submitted", "Completed");
    for (i = 0; i < mbox.num_rings; i++) {
    ring = &mbox.rings[i];
    bd_read_offset = readl_relaxed(ring.regs + RING_BD_READ_PTR);
    val = readl_relaxed(ring.regs + RING_BD_START_ADDR);
    bd_read_offset *= RING_DESC_SIZE;
    bd_read_offset += (u32)(BD_START_ADDR_DECODE(val) -
    ring.bd_dma_base);
    seq_printf(file, "%-5d 0x%08x 0x%08x 0x%08x %-11d %-11d\n",
    ring.num,
    (u32)bd_read_offset,
    (u32)ring.bd_write_offset,
    (u32)ring.cmpl_read_offset,
    (u32)atomic_read(&ring.msg_send_count),
    (u32)atomic_read(&ring.msg_cmpl_count));
    }
    }
    static int flexrm_new_request(struct flexrm_ring *ring,
    struct brcm_message *batch_msg,
    struct brcm_message *msg)
    {
    void *next;
    unsigned long flags;
    u32 val, count, nhcnt;
    u32 read_offset, write_offset;
    let mut exit_cleanup: bool = false;
    let mut ret: c_int = 0, reqid;
// Do sanity check on message
    if (!flexrm_sanity_check(msg))
    return -EIO;
    msg.error = 0;
// If no requests possible then save data pointer and goto done.
    spin_lock_irqsave(&ring.lock, flags);
    reqid = bitmap_find_free_region(ring.requests_bmap,
    RING_MAX_REQ_COUNT, 0);
    spin_unlock_irqrestore(&ring.lock, flags);
    if (reqid < 0)
    return -ENOSPC;
    ring.requests[reqid] = msg;
// Do DMA mappings for the message
    ret = flexrm_dma_map(ring.mbox.dev, msg);
    if (ret < 0) {
    ring.requests[reqid] = core::ptr::null_mut();
    spin_lock_irqsave(&ring.lock, flags);
    bitmap_release_region(ring.requests_bmap, reqid, 0);
    spin_unlock_irqrestore(&ring.lock, flags);
    return ret;
    }
// Determine current HW BD read offset
    read_offset = readl_relaxed(ring.regs + RING_BD_READ_PTR);
    val = readl_relaxed(ring.regs + RING_BD_START_ADDR);
    read_offset *= RING_DESC_SIZE;
    read_offset += (u32)(BD_START_ADDR_DECODE(val) - ring.bd_dma_base);
//
// Number required descriptors = number of non-header descriptors +
// number of header descriptors +
// 1x null descriptor
//
    nhcnt = flexrm_estimate_nonheader_desc_count(msg);
    count = flexrm_estimate_header_desc_count(nhcnt) + nhcnt + 1;
// Check for available descriptor space.
    write_offset = ring.bd_write_offset;
    while (count) {
    if (!flexrm_is_next_table_desc(ring.bd_base + write_offset))
    count--;
    write_offset += RING_DESC_SIZE;
    if (write_offset == RING_BD_SIZE)
    write_offset = 0x0;
    if (write_offset == read_offset)
    break;
    }
    if (count) {
    ret = -ENOSPC;
    exit_cleanup = true;
    goto exit;
    }
// Write descriptors to ring
    next = flexrm_write_descs(msg, nhcnt, reqid,
    ring.bd_base + ring.bd_write_offset,
    RING_BD_TOGGLE_VALID(ring.bd_write_offset),
    ring.bd_base, ring.bd_base + RING_BD_SIZE);
    if (IS_ERR(next)) {
    ret = PTR_ERR(next);
    exit_cleanup = true;
    goto exit;
    }
// Save ring BD write offset
    ring.bd_write_offset = (unsigned long)(next - ring.bd_base);
// Increment number of messages sent
    atomic_inc_return(&ring.msg_send_count);
    exit:
// Update error status in message
    msg.error = ret;
// Cleanup if we failed
    if (exit_cleanup) {
    flexrm_dma_unmap(ring.mbox.dev, msg);
    ring.requests[reqid] = core::ptr::null_mut();
    spin_lock_irqsave(&ring.lock, flags);
    bitmap_release_region(ring.requests_bmap, reqid, 0);
    spin_unlock_irqrestore(&ring.lock, flags);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn flexrm_process_completions(ring: *mut flexrm_ring) -> c_int {
    static int flexrm_process_completions(struct flexrm_ring *ring)
    {
    u64 desc;
    int err, count = 0;
    unsigned long flags;
    struct brcm_message *msg = core::ptr::null_mut();
    u32 reqid, cmpl_read_offset, cmpl_write_offset;
    struct mbox_chan *chan = &ring.mbox.controller.chans[ring.num];
    spin_lock_irqsave(&ring.lock, flags);
//
// Get current completion read and write offset
//
// Note: We should read completion write pointer at least once
// after we get a MSI interrupt because HW maintains internal
// MSI status which will allow next MSI interrupt only after
// completion write pointer is read.
//
    cmpl_write_offset = readl_relaxed(ring.regs + RING_CMPL_WRITE_PTR);
    cmpl_write_offset *= RING_DESC_SIZE;
    cmpl_read_offset = ring.cmpl_read_offset;
    ring.cmpl_read_offset = cmpl_write_offset;
    spin_unlock_irqrestore(&ring.lock, flags);
// For each completed request notify mailbox clients
    reqid = 0;
    while (cmpl_read_offset != cmpl_write_offset) {
// Dequeue next completion descriptor
    desc = *((u64 *)(ring.cmpl_base + cmpl_read_offset));
// Next read offset
    cmpl_read_offset += RING_DESC_SIZE;
    if (cmpl_read_offset == RING_CMPL_SIZE)
    cmpl_read_offset = 0;
// Decode error from completion descriptor
    err = flexrm_cmpl_desc_to_error(desc);
    if (err < 0) {
    dev_warn(ring.mbox.dev,
    "ring%d got completion desc=0x%lx with error %d\n",
    ring.num, (unsigned long)desc, err);
    }
// Determine request id from completion descriptor
    reqid = flexrm_cmpl_desc_to_reqid(desc);
// Determine message pointer based on reqid
    msg = ring.requests[reqid];
    if (!msg) {
    dev_warn(ring.mbox.dev,
    "ring%d null msg pointer for completion desc=0x%lx\n",
    ring.num, (unsigned long)desc);
    continue;
    }
// Release reqid for recycling
    ring.requests[reqid] = core::ptr::null_mut();
    spin_lock_irqsave(&ring.lock, flags);
    bitmap_release_region(ring.requests_bmap, reqid, 0);
    spin_unlock_irqrestore(&ring.lock, flags);
// Unmap DMA mappings
    flexrm_dma_unmap(ring.mbox.dev, msg);
// Give-back message to mailbox client
    msg.error = err;
    mbox_chan_received_data(chan, msg);
// Increment number of completions processed
    atomic_inc_return(&ring.msg_cmpl_count);
    count++;
    }
    return count;
    }
// ====== FlexRM Debugfs callbacks ======
#[no_mangle]
unsafe extern "C" fn flexrm_debugfs_conf_show(file: *mut seq_file, offset: *mut c_void) -> c_int {
    static int flexrm_debugfs_conf_show(struct seq_file *file, void *offset)
    {
    struct flexrm_mbox *mbox = dev_get_drvdata(file.private);
// Write config in file
    flexrm_write_config_in_seqfile(mbox, file);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn flexrm_debugfs_stats_show(file: *mut seq_file, offset: *mut c_void) -> c_int {
    static int flexrm_debugfs_stats_show(struct seq_file *file, void *offset)
    {
    struct flexrm_mbox *mbox = dev_get_drvdata(file.private);
// Write stats in file
    flexrm_write_stats_in_seqfile(mbox, file);
    return 0;
    }
// ====== FlexRM interrupt handler =====
#[no_mangle]
unsafe extern "C" fn flexrm_irq_thread(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t flexrm_irq_thread(int irq, void *dev_id)
    {
    flexrm_process_completions(dev_id);
    return IRQ_HANDLED;
    }
// ====== FlexRM mailbox callbacks =====
#[no_mangle]
unsafe extern "C" fn flexrm_send_data(chan: *mut mbox_chan, data: *mut c_void) -> c_int {
    static int flexrm_send_data(struct mbox_chan *chan, void *data)
    {
    int i, rc;
    struct flexrm_ring *ring = chan.con_priv;
    struct brcm_message *msg = data;
    if (msg.type == BRCM_MESSAGE_BATCH) {
    for (i = msg.batch.msgs_queued;
    i < msg.batch.msgs_count; i++) {
    rc = flexrm_new_request(ring, msg,
    &msg.batch.msgs[i]);
    if (rc) {
    msg.error = rc;
    return rc;
    }
    msg.batch.msgs_queued++;
    }
    return 0;
    }
    return flexrm_new_request(ring, core::ptr::null_mut(), data);
    }
#[no_mangle]
unsafe extern "C" fn flexrm_peek_data(chan: *mut mbox_chan) -> bool {
    static bool flexrm_peek_data(struct mbox_chan *chan)
    {
    let mut cnt: c_int = flexrm_process_completions(chan.con_priv);
    return (cnt > 0) ? true : false;
    }
#[no_mangle]
unsafe extern "C" fn flexrm_startup(chan: *mut mbox_chan) -> c_int {
    static int flexrm_startup(struct mbox_chan *chan)
    {
    u64 d;
    u32 val, off;
    let mut ret: c_int = 0;
    dma_addr_t next_addr;
    struct flexrm_ring *ring = chan.con_priv;
// Allocate BD memory
    ring.bd_base = dma_pool_alloc(ring.mbox.bd_pool,
    GFP_KERNEL, &ring.bd_dma_base);
    if (!ring.bd_base) {
    dev_err(ring.mbox.dev,
    "can't allocate BD memory for ring%d\n",
    ring.num);
    ret = -ENOMEM;
    goto fail;
    }
// Configure next table pointer entries in BD memory
    for (off = 0; off < RING_BD_SIZE; off += RING_DESC_SIZE) {
    next_addr = off + RING_DESC_SIZE;
    if (next_addr == RING_BD_SIZE)
    next_addr = 0;
    next_addr += ring.bd_dma_base;
    if (RING_BD_ALIGN_CHECK(next_addr))
    d = flexrm_next_table_desc(RING_BD_TOGGLE_VALID(off),
    next_addr);
    else
    d = flexrm_null_desc(RING_BD_TOGGLE_INVALID(off));
    flexrm_write_desc(ring.bd_base + off, d);
    }
// Allocate completion memory
    ring.cmpl_base = dma_pool_zalloc(ring.mbox.cmpl_pool,
    GFP_KERNEL, &ring.cmpl_dma_base);
    if (!ring.cmpl_base) {
    dev_err(ring.mbox.dev,
    "can't allocate completion memory for ring%d\n",
    ring.num);
    ret = -ENOMEM;
    goto fail_free_bd_memory;
    }
// Request IRQ
    if (ring.irq == UINT_MAX) {
    dev_err(ring.mbox.dev,
    "ring%d IRQ not available\n", ring.num);
    ret = -ENODEV;
    goto fail_free_cmpl_memory;
    }
    ret = request_threaded_irq(ring.irq, core::ptr::null_mut(), flexrm_irq_thread,
    IRQF_ONESHOT, dev_name(ring.mbox.dev), ring);
    if (ret) {
    dev_err(ring.mbox.dev,
    "failed to request ring%d IRQ\n", ring.num);
    goto fail_free_cmpl_memory;
    }
    ring.irq_requested = true;
// Set IRQ affinity hint
    ring.irq_aff_hint = CPU_MASK_NONE;
    val = ring.mbox.num_rings;
    val = (num_online_cpus() < val) ? val / num_online_cpus() : 1;
    cpumask_set_cpu((ring.num / val) % num_online_cpus(),
    &ring.irq_aff_hint);
    ret = irq_update_affinity_hint(ring.irq, &ring.irq_aff_hint);
    if (ret) {
    dev_err(ring.mbox.dev,
    "failed to set IRQ affinity hint for ring%d\n",
    ring.num);
    goto fail_free_irq;
    }
// Disable/inactivate ring
    writel_relaxed(0x0, ring.regs + RING_CONTROL);
// Program BD start address
    val = BD_START_ADDR_VALUE(ring.bd_dma_base);
    writel_relaxed(val, ring.regs + RING_BD_START_ADDR);
// BD write pointer will be same as HW write pointer
    ring.bd_write_offset =
    readl_relaxed(ring.regs + RING_BD_WRITE_PTR);
    ring.bd_write_offset *= RING_DESC_SIZE;
// Program completion start address
    val = CMPL_START_ADDR_VALUE(ring.cmpl_dma_base);
    writel_relaxed(val, ring.regs + RING_CMPL_START_ADDR);
// Completion read pointer will be same as HW write pointer
    ring.cmpl_read_offset =
    readl_relaxed(ring.regs + RING_CMPL_WRITE_PTR);
    ring.cmpl_read_offset *= RING_DESC_SIZE;
// Read ring Tx, Rx, and Outstanding counts to clear
    readl_relaxed(ring.regs + RING_NUM_REQ_RECV_LS);
    readl_relaxed(ring.regs + RING_NUM_REQ_RECV_MS);
    readl_relaxed(ring.regs + RING_NUM_REQ_TRANS_LS);
    readl_relaxed(ring.regs + RING_NUM_REQ_TRANS_MS);
    readl_relaxed(ring.regs + RING_NUM_REQ_OUTSTAND);
// Configure RING_MSI_CONTROL
    val = 0;
    val |= (ring.msi_timer_val << MSI_TIMER_VAL_SHIFT);
    val |= BIT(MSI_ENABLE_SHIFT);
    val |= (ring.msi_count_threshold & MSI_COUNT_MASK) << MSI_COUNT_SHIFT;
    writel_relaxed(val, ring.regs + RING_MSI_CONTROL);
// Enable/activate ring
    val = BIT(CONTROL_ACTIVE_SHIFT);
    writel_relaxed(val, ring.regs + RING_CONTROL);
// Reset stats to zero
    atomic_set(&ring.msg_send_count, 0);
    atomic_set(&ring.msg_cmpl_count, 0);
    return 0;
    fail_free_irq:
    free_irq(ring.irq, ring);
    ring.irq_requested = false;
    fail_free_cmpl_memory:
    dma_pool_free(ring.mbox.cmpl_pool,
    ring.cmpl_base, ring.cmpl_dma_base);
    ring.cmpl_base = core::ptr::null_mut();
    fail_free_bd_memory:
    dma_pool_free(ring.mbox.bd_pool,
    ring.bd_base, ring.bd_dma_base);
    ring.bd_base = core::ptr::null_mut();
    fail:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn flexrm_shutdown(chan: *mut mbox_chan) {
    static void flexrm_shutdown(struct mbox_chan *chan)
    {
    u32 reqid;
    unsigned int timeout;
    struct brcm_message *msg;
    struct flexrm_ring *ring = chan.con_priv;
// Disable/inactivate ring
    writel_relaxed(0x0, ring.regs + RING_CONTROL);
// Set ring flush state
    timeout = 1000; /* timeout of 1s */
    writel_relaxed(BIT(CONTROL_FLUSH_SHIFT),
    ring.regs + RING_CONTROL);
    do {
    if (readl_relaxed(ring.regs + RING_FLUSH_DONE) &
    FLUSH_DONE_MASK)
    break;
    mdelay(1);
    } while (--timeout);
    if (!timeout)
    dev_err(ring.mbox.dev,
    "setting ring%d flush state timedout\n", ring.num);
// Clear ring flush state
    timeout = 1000; /* timeout of 1s */
    writel_relaxed(0x0, ring.regs + RING_CONTROL);
    do {
    if (!(readl_relaxed(ring.regs + RING_FLUSH_DONE) &
    FLUSH_DONE_MASK))
    break;
    mdelay(1);
    } while (--timeout);
    if (!timeout)
    dev_err(ring.mbox.dev,
    "clearing ring%d flush state timedout\n", ring.num);
// Abort all in-flight requests
    for (reqid = 0; reqid < RING_MAX_REQ_COUNT; reqid++) {
    msg = ring.requests[reqid];
    if (!msg)
    continue;
// Release reqid for recycling
    ring.requests[reqid] = core::ptr::null_mut();
// Unmap DMA mappings
    flexrm_dma_unmap(ring.mbox.dev, msg);
// Give-back message to mailbox client
    msg.error = -EIO;
    mbox_chan_received_data(chan, msg);
    }
// Clear requests bitmap
    bitmap_zero(ring.requests_bmap, RING_MAX_REQ_COUNT);
// Release IRQ
    if (ring.irq_requested) {
    irq_update_affinity_hint(ring.irq, core::ptr::null_mut());
    free_irq(ring.irq, ring);
    ring.irq_requested = false;
    }
// Free-up completion descriptor ring
    if (ring.cmpl_base) {
    dma_pool_free(ring.mbox.cmpl_pool,
    ring.cmpl_base, ring.cmpl_dma_base);
    ring.cmpl_base = core::ptr::null_mut();
    }
// Free-up BD descriptor ring
    if (ring.bd_base) {
    dma_pool_free(ring.mbox.bd_pool,
    ring.bd_base, ring.bd_dma_base);
    ring.bd_base = core::ptr::null_mut();
    }
    }
    static const struct mbox_chan_ops flexrm_mbox_chan_ops = {
    .send_data	= flexrm_send_data,
    .startup	= flexrm_startup,
    .shutdown	= flexrm_shutdown,
    .peek_data	= flexrm_peek_data,
    };
    static struct mbox_chan *flexrm_mbox_of_xlate(struct mbox_controller *cntlr,
    const struct of_phandle_args *pa)
    {
    struct mbox_chan *chan;
    struct flexrm_ring *ring;
    if (pa.args_count < 3)
    return ERR_PTR(-EINVAL);
    if (pa.args[0] >= cntlr.num_chans)
    return ERR_PTR(-ENOENT);
    if (pa.args[1] > MSI_COUNT_MASK)
    return ERR_PTR(-EINVAL);
    if (pa.args[2] > MSI_TIMER_VAL_MASK)
    return ERR_PTR(-EINVAL);
    chan = &cntlr.chans[pa.args[0]];
    ring = chan.con_priv;
    ring.msi_count_threshold = pa.args[1];
    ring.msi_timer_val = pa.args[2];
    return chan;
    }
// ====== FlexRM platform driver =====
#[no_mangle]
unsafe extern "C" fn flexrm_mbox_msi_write(desc: *mut msi_desc, msg: *mut msi_msg) {
    static void flexrm_mbox_msi_write(struct msi_desc *desc, struct msi_msg *msg)
    {
    struct device *dev = msi_desc_to_dev(desc);
    struct flexrm_mbox *mbox = dev_get_drvdata(dev);
    struct flexrm_ring *ring = &mbox.rings[desc.msi_index];
// Configure per-Ring MSI registers
    writel_relaxed(msg.address_lo, ring.regs + RING_MSI_ADDR_LS);
    writel_relaxed(msg.address_hi, ring.regs + RING_MSI_ADDR_MS);
    writel_relaxed(msg.data, ring.regs + RING_MSI_DATA_VALUE);
    }
#[no_mangle]
unsafe extern "C" fn flexrm_mbox_probe(pdev: *mut platform_device) -> c_int {
    static int flexrm_mbox_probe(struct platform_device *pdev)
    {
    int index, ret = 0;
    void __iomem *regs;
    void __iomem *regs_end;
    struct resource *iomem;
    struct flexrm_ring *ring;
    struct flexrm_mbox *mbox;
    struct device *dev = &pdev.dev;
// Allocate driver mailbox struct
    mbox = devm_kzalloc(dev, sizeof(*mbox), GFP_KERNEL);
    if (!mbox) {
    ret = -ENOMEM;
    goto fail;
    }
    mbox.dev = dev;
    platform_set_drvdata(pdev, mbox);
// Get resource for registers and map registers of all rings
    mbox.regs = devm_platform_get_and_ioremap_resource(pdev, 0, &iomem);
    if (!iomem || (resource_size(iomem) < RING_REGS_SIZE)) {
    ret = -ENODEV;
    goto fail;
    } else if (IS_ERR(mbox.regs)) {
    ret = PTR_ERR(mbox.regs);
    goto fail;
    }
    regs_end = mbox.regs + resource_size(iomem);
// Scan and count available rings
    mbox.num_rings = 0;
    for (regs = mbox.regs; regs < regs_end; regs += RING_REGS_SIZE) {
    if (readl_relaxed(regs + RING_VER) == RING_VER_MAGIC)
    mbox.num_rings++;
    }
    if (!mbox.num_rings) {
    ret = -ENODEV;
    goto fail;
    }
// Allocate driver ring structs
    ring = devm_kcalloc(dev, mbox.num_rings, sizeof(*ring), GFP_KERNEL);
    if (!ring) {
    ret = -ENOMEM;
    goto fail;
    }
    mbox.rings = ring;
// Initialize members of driver ring structs
    regs = mbox.regs;
    for (index = 0; index < mbox.num_rings; index++) {
    ring = &mbox.rings[index];
    ring.num = index;
    ring.mbox = mbox;
    while ((regs < regs_end) &&
    (readl_relaxed(regs + RING_VER) != RING_VER_MAGIC))
    regs += RING_REGS_SIZE;
    if (regs_end <= regs) {
    ret = -ENODEV;
    goto fail;
    }
    ring.regs = regs;
    regs += RING_REGS_SIZE;
    ring.irq = UINT_MAX;
    ring.irq_requested = false;
    ring.msi_timer_val = MSI_TIMER_VAL_MASK;
    ring.msi_count_threshold = 0x1;
    memset(ring.requests, 0, sizeof(ring.requests));
    ring.bd_base = core::ptr::null_mut();
    ring.bd_dma_base = 0;
    ring.cmpl_base = core::ptr::null_mut();
    ring.cmpl_dma_base = 0;
    atomic_set(&ring.msg_send_count, 0);
    atomic_set(&ring.msg_cmpl_count, 0);
    spin_lock_init(&ring.lock);
    bitmap_zero(ring.requests_bmap, RING_MAX_REQ_COUNT);
    ring.cmpl_read_offset = 0;
    }
// FlexRM is capable of 40-bit physical addresses only
    ret = dma_set_mask_and_coherent(dev, DMA_BIT_MASK(40));
    if (ret) {
    ret = dma_set_mask_and_coherent(dev, DMA_BIT_MASK(32));
    if (ret)
    goto fail;
    }
// Create DMA pool for ring BD memory
    mbox.bd_pool = dma_pool_create("bd", dev, RING_BD_SIZE,
    1 << RING_BD_ALIGN_ORDER, 0);
    if (!mbox.bd_pool) {
    ret = -ENOMEM;
    goto fail;
    }
// Create DMA pool for ring completion memory
    mbox.cmpl_pool = dma_pool_create("cmpl", dev, RING_CMPL_SIZE,
    1 << RING_CMPL_ALIGN_ORDER, 0);
    if (!mbox.cmpl_pool) {
    ret = -ENOMEM;
    goto fail_destroy_bd_pool;
    }
// Allocate platform MSIs for each ring
    ret = platform_device_msi_init_and_alloc_irqs(dev, mbox.num_rings,
    flexrm_mbox_msi_write);
    if (ret)
    goto fail_destroy_cmpl_pool;
// Save alloced IRQ numbers for each ring
    for (index = 0; index < mbox.num_rings; index++)
    mbox.rings[index].irq = msi_get_virq(dev, index);
// Check availability of debugfs
    if (!debugfs_initialized())
    goto skip_debugfs;
// Create debugfs root entry
    mbox.root = debugfs_create_dir(dev_name(mbox.dev), core::ptr::null_mut());
// Create debugfs config entry
    debugfs_create_devm_seqfile(mbox.dev, "config", mbox.root,
    flexrm_debugfs_conf_show);
// Create debugfs stats entry
    debugfs_create_devm_seqfile(mbox.dev, "stats", mbox.root,
    flexrm_debugfs_stats_show);
    skip_debugfs:
// Initialize mailbox controller
    mbox.controller.txdone_irq = false;
    mbox.controller.txdone_poll = false;
    mbox.controller.ops = &flexrm_mbox_chan_ops;
    mbox.controller.dev = dev;
    mbox.controller.num_chans = mbox.num_rings;
    mbox.controller.of_xlate = flexrm_mbox_of_xlate;
    mbox.controller.chans = devm_kcalloc(dev, mbox.num_rings,
    sizeof(*mbox.controller.chans), GFP_KERNEL);
    if (!mbox.controller.chans) {
    ret = -ENOMEM;
    goto fail_free_debugfs_root;
    }
    for (index = 0; index < mbox.num_rings; index++)
    mbox.controller.chans[index].con_priv = &mbox.rings[index];
// Register mailbox controller
    ret = devm_mbox_controller_register(dev, &mbox.controller);
    if (ret)
    goto fail_free_debugfs_root;
    dev_info(dev, "registered flexrm mailbox with %d channels\n",
    mbox.controller.num_chans);
    return 0;
    fail_free_debugfs_root:
    debugfs_remove_recursive(mbox.root);
    platform_device_msi_free_irqs_all(dev);
    fail_destroy_cmpl_pool:
    dma_pool_destroy(mbox.cmpl_pool);
    fail_destroy_bd_pool:
    dma_pool_destroy(mbox.bd_pool);
    fail:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn flexrm_mbox_remove(pdev: *mut platform_device) {
    static void flexrm_mbox_remove(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct flexrm_mbox *mbox = platform_get_drvdata(pdev);
    debugfs_remove_recursive(mbox.root);
    platform_device_msi_free_irqs_all(dev);
    dma_pool_destroy(mbox.cmpl_pool);
    dma_pool_destroy(mbox.bd_pool);
    }
    static const struct of_device_id flexrm_mbox_of_match[] = {
    { .compatible = "brcm,iproc-flexrm-mbox", },
    {},
    };
    MODULE_DEVICE_TABLE(of, flexrm_mbox_of_match);
    static struct platform_driver flexrm_mbox_driver = {
    .driver = {
    .name = "brcm-flexrm-mbox",
    .of_match_table = flexrm_mbox_of_match,
    },
    .probe		= flexrm_mbox_probe,
    .remove		= flexrm_mbox_remove,
    };
    module_platform_driver(flexrm_mbox_driver);
    MODULE_AUTHOR("Anup Patel <anup.patel@broadcom.com>");
    MODULE_DESCRIPTION("Broadcom FlexRM mailbox driver");
    MODULE_LICENSE("GPL v2");
