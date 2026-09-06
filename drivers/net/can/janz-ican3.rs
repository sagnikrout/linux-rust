//! Automatically rewritten from C to Rust
//! Source: drivers/net/can/janz-ican3.c
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
// Janz MODULbus VMOD-ICAN3 CAN Interface Driver
//
// Copyright (c) 2010 Ira W. Snyder <iws@ovro.caltech.edu>
//

// the DPM has 64k of memory, organized into 256x 256 byte pages
pub const DPM_NUM_PAGES: c_int = 256;
pub const DPM_PAGE_SIZE: c_int = 256;

// JANZ ICAN3 "old-style" host interface queue page numbers
pub const QUEUE_OLD_CONTROL: c_int = 0;
pub const QUEUE_OLD_RB0: c_int = 1;
pub const QUEUE_OLD_RB1: c_int = 2;
pub const QUEUE_OLD_WB0: c_int = 3;
pub const QUEUE_OLD_WB1: c_int = 4;
// Janz ICAN3 "old-style" host interface control registers
pub const MSYNC_PEER: c_uint = 0x00		/* ICAN only */;
pub const MSYNC_LOCL: c_uint = 0x01		/* host only */;
pub const TARGET_RUNNING: c_uint = 0x02;
pub const FIRMWARE_STAMP: c_uint = 0x60		/* big endian firmware stamp */;
pub const MSYNC_RB0: c_uint = 0x01;
pub const MSYNC_RB1: c_uint = 0x02;
pub const MSYNC_RBLW: c_uint = 0x04;

pub const MSYNC_WB0: c_uint = 0x10;
pub const MSYNC_WB1: c_uint = 0x20;
pub const MSYNC_WBLW: c_uint = 0x40;

// Janz ICAN3 "new-style" host interface queue page numbers
pub const QUEUE_TOHOST: c_int = 5;
pub const QUEUE_FROMHOST_MID: c_int = 6;
pub const QUEUE_FROMHOST_HIGH: c_int = 7;
pub const QUEUE_FROMHOST_LOW: c_int = 8;
// The first free page in the DPM is #9
pub const DPM_FREE_START: c_int = 9;
// Janz ICAN3 "new-style" and "fast" host interface descriptor flags
pub const DESC_VALID: c_uint = 0x80;
pub const DESC_WRAP: c_uint = 0x40;
pub const DESC_INTERRUPT: c_uint = 0x20;
pub const DESC_IVALID: c_uint = 0x10;

// Janz ICAN3 Firmware Messages
pub const MSG_CONNECTI: c_uint = 0x02;
pub const MSG_DISCONNECT: c_uint = 0x03;
pub const MSG_IDVERS: c_uint = 0x04;
pub const MSG_MSGLOST: c_uint = 0x05;
pub const MSG_NEWHOSTIF: c_uint = 0x08;
pub const MSG_INQUIRY: c_uint = 0x0a;
pub const MSG_SETAFILMASK: c_uint = 0x10;
pub const MSG_INITFDPMQUEUE: c_uint = 0x11;
pub const MSG_HWCONF: c_uint = 0x12;
pub const MSG_FMSGLOST: c_uint = 0x15;
pub const MSG_CEVTIND: c_uint = 0x37;
pub const MSG_CBTRREQ: c_uint = 0x41;
pub const MSG_COFFREQ: c_uint = 0x42;
pub const MSG_CONREQ: c_uint = 0x43;
pub const MSG_CCONFREQ: c_uint = 0x47;
pub const MSG_NMTS: c_uint = 0xb0;
pub const MSG_LMTS: c_uint = 0xb4;
//
// Janz ICAN3 CAN Inquiry Message Types
//
// NOTE: there appears to be a firmware bug here. You must send
// NOTE: INQUIRY_STATUS and expect to receive an INQUIRY_EXTENDED
// NOTE: response. The controller never responds to a message with
// NOTE: the INQUIRY_EXTENDED subspec :(
//
pub const INQUIRY_STATUS: c_uint = 0x00;
pub const INQUIRY_TERMINATION: c_uint = 0x01;
pub const INQUIRY_EXTENDED: c_uint = 0x04;
// Janz ICAN3 CAN Set Acceptance Filter Mask Message Types
pub const SETAFILMASK_REJECT: c_uint = 0x00;
pub const SETAFILMASK_FASTIF: c_uint = 0x02;
// Janz ICAN3 CAN Hardware Configuration Message Types
pub const HWCONF_TERMINATE_ON: c_uint = 0x01;
pub const HWCONF_TERMINATE_OFF: c_uint = 0x00;
// Janz ICAN3 CAN Event Indication Message Types
pub const CEVTIND_EI: c_uint = 0x01;
pub const CEVTIND_DOI: c_uint = 0x02;
pub const CEVTIND_LOST: c_uint = 0x04;
pub const CEVTIND_FULL: c_uint = 0x08;
pub const CEVTIND_BEI: c_uint = 0x10;
pub const CEVTIND_CHIP_SJA1000: c_uint = 0x02;
pub const ICAN3_BUSERR_QUOTA_MAX: c_int = 255;
// Janz ICAN3 CAN Frame Conversion
pub const ICAN3_SNGL: c_uint = 0x02;
pub const ICAN3_ECHO: c_uint = 0x10;
pub const ICAN3_EFF_RTR: c_uint = 0x40;
pub const ICAN3_SFF_RTR: c_uint = 0x10;
pub const ICAN3_EFF: c_uint = 0x80;
pub const ICAN3_CAN_TYPE_MASK: c_uint = 0x0f;
pub const ICAN3_CAN_TYPE_SFF: c_uint = 0x00;
pub const ICAN3_CAN_TYPE_EFF: c_uint = 0x01;
pub const ICAN3_CAN_DLC_MASK: c_uint = 0x0f;
// Janz ICAN3 NMTS subtypes
pub const NMTS_CREATE_NODE_REQ: c_uint = 0x0;
pub const NMTS_SLAVE_STATE_IND: c_uint = 0x8;
pub const NMTS_SLAVE_EVENT_IND: c_uint = 0x9;
// Janz ICAN3 LMTS subtypes
pub const LMTS_BUSON_REQ: c_uint = 0x0;
pub const LMTS_BUSOFF_REQ: c_uint = 0x1;
pub const LMTS_CAN_CONF_REQ: c_uint = 0x2;
// Janz ICAN3 NMTS Event indications
pub const NE_LOCAL_OCCURRED: c_uint = 0x3;
pub const NE_LOCAL_RESOLVED: c_uint = 0x2;
pub const NE_REMOTE_OCCURRED: c_uint = 0xc;
pub const NE_REMOTE_RESOLVED: c_uint = 0x8;
//
// SJA1000 Status and Error Register Definitions
//
// Copied from drivers/net/can/sja1000/sja1000.h
//
// status register content
pub const SR_BS: c_uint = 0x80;
pub const SR_ES: c_uint = 0x40;
pub const SR_TS: c_uint = 0x20;
pub const SR_RS: c_uint = 0x10;
pub const SR_TCS: c_uint = 0x08;
pub const SR_TBS: c_uint = 0x04;
pub const SR_DOS: c_uint = 0x02;
pub const SR_RBS: c_uint = 0x01;

// ECC register
pub const ECC_SEG: c_uint = 0x1F;
pub const ECC_DIR: c_uint = 0x20;
pub const ECC_ERR: c_int = 6;
pub const ECC_BIT: c_uint = 0x00;
pub const ECC_FORM: c_uint = 0x40;
pub const ECC_STUFF: c_uint = 0x80;
pub const ECC_MASK: c_uint = 0xc0;
// Number of buffers for use in the "new-style" host interface
pub const ICAN3_NEW_BUFFERS: c_int = 16;
// Number of buffers for use in the "fast" host interface
pub const ICAN3_TX_BUFFERS: c_int = 512;
pub const ICAN3_RX_BUFFERS: c_int = 1024;
// SJA1000 Clock Input
pub const ICAN3_CAN_CLOCK: c_int = 8000000;
// Janz ICAN3 firmware types
    enum ican3_fwtype {
    ICAN3_FWTYPE_ICANOS,
    ICAN3_FWTYPE_CAL_CANOPEN,
    };
// Driver Name

// DPM Control Registers -- starts at offset 0x100 in the MODULbus registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ican3_dpm_control {
// window address register
    pub window_address: u8,
    pub unused1: u8,
//
// Read access: clear interrupt from microcontroller
// Write access: send interrupt to microcontroller
//
    pub interrupt: u8,
    pub unused2: u8,
// write-only: reset all hardware on the module
    pub hwreset: u8,
    pub unused3: u8,
// write-only: generate an interrupt to the TPU
    pub tpuinterrupt: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ican3_dev {
// must be the first member
    pub can: can_priv,
// CAN network device
    pub ndev: *mut net_device,
    pub napi: napi_struct,
// module number
    pub num: c_uint,
// base address of registers and IRQ
    pub ctrl: *mut janz_cmodio_onboard_regs __iomem,
    pub dpmctrl: *mut ican3_dpm_control __iomem,
    pub dpm: *mut void __iomem,
    pub irq: c_int,
// CAN bus termination status
    pub termination_comp: completion,
    pub termination_enabled: bool,
// CAN bus error status registers
    pub buserror_comp: completion,
    pub bec: can_berr_counter,
// firmware type
    pub fwtype: enum ican3_fwtype,
    pub fwinfo: [c_char; 32],
// old and new style host interface
    pub iftype: c_uint,
// queue for echo packets
    pub echoq: sk_buff_head,
//
// Any function which changes the current DPM page must hold this
// lock while it is performing data accesses. This ensures that the
// function will not be preempted and end up reading data from a
// different DPM page than it expects.
//
    pub lock: spinlock_t,
// new host interface
    pub rx_int: c_uint,
    pub rx_num: c_uint,
    pub tx_num: c_uint,
// fast host interface
    pub fastrx_start: c_uint,
    pub fastrx_num: c_uint,
    pub fasttx_start: c_uint,
    pub fasttx_num: c_uint,
// first free DPM page
    pub free_page: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ican3_msg {
    pub control: u8,
    pub spec: u8,
    pub len: __le16,
    pub data: [u8; 252],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ican3_new_desc {
    pub control: u8,
    pub pointer: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ican3_fast_desc {
    pub control: u8,
    pub command: u8,
    pub data: [u8; 14],
}

// write to the window basic address register
#[no_mangle]
pub unsafe extern "C" fn ican3_set_page(mod: *mut ican3_dev, page: c_uint) {
    static inline void ican3_set_page(struct ican3_dev *mod, unsigned int page)
    {
    BUG_ON(page >= DPM_NUM_PAGES);
    iowrite8(page, &mod.dpmctrl.window_address);
    }
//
// ICAN3 "old-style" host interface
//
// Receive a message from the ICAN3 "old-style" firmware interface
//
// LOCKING: must hold mod->lock
//
// returns 0 on success, -ENOMEM when no message exists
//
#[no_mangle]
unsafe extern "C" fn ican3_old_recv_msg(mod: *mut ican3_dev, msg: *mut ican3_msg) -> c_int {
    static int ican3_old_recv_msg(struct ican3_dev *mod, struct ican3_msg *msg)
    {
    unsigned int mbox, mbox_page;
    u8 locl, peer, xord;
// get the MSYNC registers
    ican3_set_page(mod, QUEUE_OLD_CONTROL);
    peer = ioread8(mod.dpm + MSYNC_PEER);
    locl = ioread8(mod.dpm + MSYNC_LOCL);
    xord = locl ^ peer;
    if ((xord & MSYNC_RB_MASK) == 0x00) {
    netdev_dbg(mod.ndev, "no mbox for reading\n");
    return -ENOMEM;
    }
// find the first free mbox to read
    if ((xord & MSYNC_RB_MASK) == MSYNC_RB_MASK)
    mbox = (xord & MSYNC_RBLW) ? MSYNC_RB0 : MSYNC_RB1;
    else
    mbox = (xord & MSYNC_RB0) ? MSYNC_RB0 : MSYNC_RB1;
// copy the message
    mbox_page = (mbox == MSYNC_RB0) ? QUEUE_OLD_RB0 : QUEUE_OLD_RB1;
    ican3_set_page(mod, mbox_page);
    memcpy_fromio(msg, mod.dpm, sizeof(*msg));
//
// notify the firmware that the read buffer is available
// for it to fill again
//
    locl ^= mbox;
    ican3_set_page(mod, QUEUE_OLD_CONTROL);
    iowrite8(locl, mod.dpm + MSYNC_LOCL);
    return 0;
    }
//
// Send a message through the "old-style" firmware interface
//
// LOCKING: must hold mod->lock
//
// returns 0 on success, -ENOMEM when no free space exists
//
#[no_mangle]
unsafe extern "C" fn ican3_old_send_msg(mod: *mut ican3_dev, msg: *mut ican3_msg) -> c_int {
    static int ican3_old_send_msg(struct ican3_dev *mod, struct ican3_msg *msg)
    {
    unsigned int mbox, mbox_page;
    u8 locl, peer, xord;
// get the MSYNC registers
    ican3_set_page(mod, QUEUE_OLD_CONTROL);
    peer = ioread8(mod.dpm + MSYNC_PEER);
    locl = ioread8(mod.dpm + MSYNC_LOCL);
    xord = locl ^ peer;
    if ((xord & MSYNC_WB_MASK) == MSYNC_WB_MASK) {
    netdev_err(mod.ndev, "no mbox for writing\n");
    return -ENOMEM;
    }
// calculate a free mbox to use
    mbox = (xord & MSYNC_WB0) ? MSYNC_WB1 : MSYNC_WB0;
// copy the message to the DPM
    mbox_page = (mbox == MSYNC_WB0) ? QUEUE_OLD_WB0 : QUEUE_OLD_WB1;
    ican3_set_page(mod, mbox_page);
    memcpy_toio(mod.dpm, msg, sizeof(*msg));
    locl ^= mbox;
    if (mbox == MSYNC_WB1)
    locl |= MSYNC_WBLW;
    ican3_set_page(mod, QUEUE_OLD_CONTROL);
    iowrite8(locl, mod.dpm + MSYNC_LOCL);
    return 0;
    }
//
// ICAN3 "new-style" Host Interface Setup
//
#[no_mangle]
unsafe extern "C" fn ican3_init_new_host_interface(mod: *mut ican3_dev) {
    static void ican3_init_new_host_interface(struct ican3_dev *mod)
    {
    struct ican3_new_desc desc;
    unsigned long flags;
    void __iomem *dst;
    int i;
    spin_lock_irqsave(&mod.lock, flags);
// setup the internal datastructures for RX
    mod.rx_num = 0;
    mod.rx_int = 0;
// tohost queue descriptors are in page 5
    ican3_set_page(mod, QUEUE_TOHOST);
    dst = mod.dpm;
// initialize the tohost (rx) queue descriptors: pages 9-24
    for (i = 0; i < ICAN3_NEW_BUFFERS; i++) {
    desc.control = DESC_INTERRUPT | DESC_LEN(1); /* I L=1 */
    desc.pointer = mod.free_page;
// set wrap flag on last buffer
    if (i == ICAN3_NEW_BUFFERS - 1)
    desc.control |= DESC_WRAP;
    memcpy_toio(dst, &desc, sizeof(desc));
    dst += sizeof(desc);
    mod.free_page++;
    }
// fromhost (tx) mid queue descriptors are in page 6
    ican3_set_page(mod, QUEUE_FROMHOST_MID);
    dst = mod.dpm;
// setup the internal datastructures for TX
    mod.tx_num = 0;
// initialize the fromhost mid queue descriptors: pages 25-40
    for (i = 0; i < ICAN3_NEW_BUFFERS; i++) {
    desc.control = DESC_VALID | DESC_LEN(1); /* V L=1 */
    desc.pointer = mod.free_page;
// set wrap flag on last buffer
    if (i == ICAN3_NEW_BUFFERS - 1)
    desc.control |= DESC_WRAP;
    memcpy_toio(dst, &desc, sizeof(desc));
    dst += sizeof(desc);
    mod.free_page++;
    }
// fromhost hi queue descriptors are in page 7
    ican3_set_page(mod, QUEUE_FROMHOST_HIGH);
    dst = mod.dpm;
// initialize only a single buffer in the fromhost hi queue (unused)
    desc.control = DESC_VALID | DESC_WRAP | DESC_LEN(1); /* VW L=1 */
    desc.pointer = mod.free_page;
    memcpy_toio(dst, &desc, sizeof(desc));
    mod.free_page++;
// fromhost low queue descriptors are in page 8
    ican3_set_page(mod, QUEUE_FROMHOST_LOW);
    dst = mod.dpm;
// initialize only a single buffer in the fromhost low queue (unused)
    desc.control = DESC_VALID | DESC_WRAP | DESC_LEN(1); /* VW L=1 */
    desc.pointer = mod.free_page;
    memcpy_toio(dst, &desc, sizeof(desc));
    mod.free_page++;
    spin_unlock_irqrestore(&mod.lock, flags);
    }
//
// ICAN3 Fast Host Interface Setup
//
#[no_mangle]
unsafe extern "C" fn ican3_init_fast_host_interface(mod: *mut ican3_dev) {
    static void ican3_init_fast_host_interface(struct ican3_dev *mod)
    {
    struct ican3_fast_desc desc;
    unsigned long flags;
    unsigned int addr;
    void __iomem *dst;
    int i;
    spin_lock_irqsave(&mod.lock, flags);
// save the start recv page
    mod.fastrx_start = mod.free_page;
    mod.fastrx_num = 0;
// build a single fast tohost queue descriptor
    memset(&desc, 0, sizeof(desc));
    desc.control = 0x00;
    desc.command = 1;
// build the tohost queue descriptor ring in memory
    addr = 0;
    for (i = 0; i < ICAN3_RX_BUFFERS; i++) {
// set the wrap bit on the last buffer
    if (i == ICAN3_RX_BUFFERS - 1)
    desc.control |= DESC_WRAP;
// switch to the correct page
    ican3_set_page(mod, mod.free_page);
// copy the descriptor to the DPM
    dst = mod.dpm + addr;
    memcpy_toio(dst, &desc, sizeof(desc));
    addr += sizeof(desc);
// move to the next page if necessary
    if (addr >= DPM_PAGE_SIZE) {
    addr = 0;
    mod.free_page++;
    }
    }
// make sure we page-align the next queue
    if (addr != 0)
    mod.free_page++;
// save the start xmit page
    mod.fasttx_start = mod.free_page;
    mod.fasttx_num = 0;
// build a single fast fromhost queue descriptor
    memset(&desc, 0, sizeof(desc));
    desc.control = DESC_VALID;
    desc.command = 1;
// build the fromhost queue descriptor ring in memory
    addr = 0;
    for (i = 0; i < ICAN3_TX_BUFFERS; i++) {
// set the wrap bit on the last buffer
    if (i == ICAN3_TX_BUFFERS - 1)
    desc.control |= DESC_WRAP;
// switch to the correct page
    ican3_set_page(mod, mod.free_page);
// copy the descriptor to the DPM
    dst = mod.dpm + addr;
    memcpy_toio(dst, &desc, sizeof(desc));
    addr += sizeof(desc);
// move to the next page if necessary
    if (addr >= DPM_PAGE_SIZE) {
    addr = 0;
    mod.free_page++;
    }
    }
    spin_unlock_irqrestore(&mod.lock, flags);
    }
//
// ICAN3 "new-style" Host Interface Message Helpers
//
// LOCKING: must hold mod->lock
//
#[no_mangle]
unsafe extern "C" fn ican3_new_send_msg(mod: *mut ican3_dev, msg: *mut ican3_msg) -> c_int {
    static int ican3_new_send_msg(struct ican3_dev *mod, struct ican3_msg *msg)
    {
    struct ican3_new_desc desc;
    void __iomem *desc_addr = mod.dpm + (mod.tx_num * sizeof(desc));
// switch to the fromhost mid queue, and read the buffer descriptor
    ican3_set_page(mod, QUEUE_FROMHOST_MID);
    memcpy_fromio(&desc, desc_addr, sizeof(desc));
    if (!(desc.control & DESC_VALID)) {
    netdev_dbg(mod.ndev, "%s: no free buffers\n", __func__);
    return -ENOMEM;
    }
// switch to the data page, copy the data
    ican3_set_page(mod, desc.pointer);
    memcpy_toio(mod.dpm, msg, sizeof(*msg));
// switch back to the descriptor, set the valid bit, write it back
    ican3_set_page(mod, QUEUE_FROMHOST_MID);
    desc.control ^= DESC_VALID;
    memcpy_toio(desc_addr, &desc, sizeof(desc));
// update the tx number
    mod.tx_num = (desc.control & DESC_WRAP) ? 0 : (mod.tx_num + 1);
    return 0;
    }
//
// LOCKING: must hold mod->lock
//
#[no_mangle]
unsafe extern "C" fn ican3_new_recv_msg(mod: *mut ican3_dev, msg: *mut ican3_msg) -> c_int {
    static int ican3_new_recv_msg(struct ican3_dev *mod, struct ican3_msg *msg)
    {
    struct ican3_new_desc desc;
    void __iomem *desc_addr = mod.dpm + (mod.rx_num * sizeof(desc));
// switch to the tohost queue, and read the buffer descriptor
    ican3_set_page(mod, QUEUE_TOHOST);
    memcpy_fromio(&desc, desc_addr, sizeof(desc));
    if (!(desc.control & DESC_VALID)) {
    netdev_dbg(mod.ndev, "%s: no buffers to recv\n", __func__);
    return -ENOMEM;
    }
// switch to the data page, copy the data
    ican3_set_page(mod, desc.pointer);
    memcpy_fromio(msg, mod.dpm, sizeof(*msg));
// switch back to the descriptor, toggle the valid bit, write it back
    ican3_set_page(mod, QUEUE_TOHOST);
    desc.control ^= DESC_VALID;
    memcpy_toio(desc_addr, &desc, sizeof(desc));
// update the rx number
    mod.rx_num = (desc.control & DESC_WRAP) ? 0 : (mod.rx_num + 1);
    return 0;
    }
//
// Message Send / Recv Helpers
//
#[no_mangle]
unsafe extern "C" fn ican3_send_msg(mod: *mut ican3_dev, msg: *mut ican3_msg) -> c_int {
    static int ican3_send_msg(struct ican3_dev *mod, struct ican3_msg *msg)
    {
    unsigned long flags;
    int ret;
    spin_lock_irqsave(&mod.lock, flags);
    if (mod.iftype == 0)
    ret = ican3_old_send_msg(mod, msg);
    else
    ret = ican3_new_send_msg(mod, msg);
    spin_unlock_irqrestore(&mod.lock, flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ican3_recv_msg(mod: *mut ican3_dev, msg: *mut ican3_msg) -> c_int {
    static int ican3_recv_msg(struct ican3_dev *mod, struct ican3_msg *msg)
    {
    unsigned long flags;
    int ret;
    spin_lock_irqsave(&mod.lock, flags);
    if (mod.iftype == 0)
    ret = ican3_old_recv_msg(mod, msg);
    else
    ret = ican3_new_recv_msg(mod, msg);
    spin_unlock_irqrestore(&mod.lock, flags);
    return ret;
    }
//
// Quick Pre-constructed Messages
//
#[no_mangle]
unsafe extern "C" fn ican3_msg_connect(mod: *mut ican3_dev) -> c_int {
    static int ican3_msg_connect(struct ican3_dev *mod)
    {
    struct ican3_msg msg;
    memset(&msg, 0, sizeof(msg));
    msg.spec = MSG_CONNECTI;
    msg.len = cpu_to_le16(0);
    return ican3_send_msg(mod, &msg);
    }
#[no_mangle]
unsafe extern "C" fn ican3_msg_disconnect(mod: *mut ican3_dev) -> c_int {
    static int ican3_msg_disconnect(struct ican3_dev *mod)
    {
    struct ican3_msg msg;
    memset(&msg, 0, sizeof(msg));
    msg.spec = MSG_DISCONNECT;
    msg.len = cpu_to_le16(0);
    return ican3_send_msg(mod, &msg);
    }
#[no_mangle]
unsafe extern "C" fn ican3_msg_newhostif(mod: *mut ican3_dev) -> c_int {
    static int ican3_msg_newhostif(struct ican3_dev *mod)
    {
    struct ican3_msg msg;
    int ret;
    memset(&msg, 0, sizeof(msg));
    msg.spec = MSG_NEWHOSTIF;
    msg.len = cpu_to_le16(0);
// If we're not using the old interface, switching seems bogus
    WARN_ON(mod.iftype != 0);
    ret = ican3_send_msg(mod, &msg);
    if (ret)
    return ret;
// mark the module as using the new host interface
    mod.iftype = 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ican3_msg_fasthostif(mod: *mut ican3_dev) -> c_int {
    static int ican3_msg_fasthostif(struct ican3_dev *mod)
    {
    struct ican3_msg msg;
    unsigned int addr;
    memset(&msg, 0, sizeof(msg));
    msg.spec = MSG_INITFDPMQUEUE;
    msg.len = cpu_to_le16(8);
// write the tohost queue start address
    addr = DPM_PAGE_ADDR(mod.fastrx_start);
    msg.data[0] = addr & 0xff;
    msg.data[1] = (addr >> 8) & 0xff;
    msg.data[2] = (addr >> 16) & 0xff;
    msg.data[3] = (addr >> 24) & 0xff;
// write the fromhost queue start address
    addr = DPM_PAGE_ADDR(mod.fasttx_start);
    msg.data[4] = addr & 0xff;
    msg.data[5] = (addr >> 8) & 0xff;
    msg.data[6] = (addr >> 16) & 0xff;
    msg.data[7] = (addr >> 24) & 0xff;
// If we're not using the new interface yet, we cannot do this
    WARN_ON(mod.iftype != 1);
    return ican3_send_msg(mod, &msg);
    }
//
// Setup the CAN filter to either accept or reject all
// messages from the CAN bus.
//
#[no_mangle]
unsafe extern "C" fn ican3_set_id_filter(mod: *mut ican3_dev, accept: bool) -> c_int {
    static int ican3_set_id_filter(struct ican3_dev *mod, bool accept)
    {
    struct ican3_msg msg;
    int ret;
// Standard Frame Format
    memset(&msg, 0, sizeof(msg));
    msg.spec = MSG_SETAFILMASK;
    msg.len = cpu_to_le16(5);
    msg.data[0] = 0x00; /* IDLo LSB */
    msg.data[1] = 0x00; /* IDLo MSB */
    msg.data[2] = 0xff; /* IDHi LSB */
    msg.data[3] = 0x07; /* IDHi MSB */
// accept all frames for fast host if, or reject all frames
    msg.data[4] = accept ? SETAFILMASK_FASTIF : SETAFILMASK_REJECT;
    ret = ican3_send_msg(mod, &msg);
    if (ret)
    return ret;
// Extended Frame Format
    memset(&msg, 0, sizeof(msg));
    msg.spec = MSG_SETAFILMASK;
    msg.len = cpu_to_le16(13);
    msg.data[0] = 0;    /* MUX = 0 */
    msg.data[1] = 0x00; /* IDLo LSB */
    msg.data[2] = 0x00;
    msg.data[3] = 0x00;
    msg.data[4] = 0x20; /* IDLo MSB */
    msg.data[5] = 0xff; /* IDHi LSB */
    msg.data[6] = 0xff;
    msg.data[7] = 0xff;
    msg.data[8] = 0x3f; /* IDHi MSB */
// accept all frames for fast host if, or reject all frames
    msg.data[9] = accept ? SETAFILMASK_FASTIF : SETAFILMASK_REJECT;
    return ican3_send_msg(mod, &msg);
    }
//
// Bring the CAN bus online or offline
//
#[no_mangle]
unsafe extern "C" fn ican3_set_bus_state(mod: *mut ican3_dev, on: bool) -> c_int {
    static int ican3_set_bus_state(struct ican3_dev *mod, bool on)
    {
    struct can_bittiming *bt = &mod.can.bittiming;
    struct ican3_msg msg;
    u8 btr0, btr1;
    int res;
// This algorithm was stolen from drivers/net/can/sja1000/sja1000.c
// The bittiming register command for the ICAN3 just sets the bit timing
// registers on the SJA1000 chip directly
    btr0 = ((bt.brp - 1) & 0x3f) | (((bt.sjw - 1) & 0x3) << 6);
    btr1 = ((bt.prop_seg + bt.phase_seg1 - 1) & 0xf) |
    (((bt.phase_seg2 - 1) & 0x7) << 4);
    if (mod.can.ctrlmode & CAN_CTRLMODE_3_SAMPLES)
    btr1 |= 0x80;
    if (mod.fwtype == ICAN3_FWTYPE_ICANOS) {
    if (on) {
// set bittiming
    memset(&msg, 0, sizeof(msg));
    msg.spec = MSG_CBTRREQ;
    msg.len = cpu_to_le16(4);
    msg.data[0] = 0x00;
    msg.data[1] = 0x00;
    msg.data[2] = btr0;
    msg.data[3] = btr1;
    res = ican3_send_msg(mod, &msg);
    if (res)
    return res;
    }
// can-on/off request
    memset(&msg, 0, sizeof(msg));
    msg.spec = on ? MSG_CONREQ : MSG_COFFREQ;
    msg.len = cpu_to_le16(0);
    return ican3_send_msg(mod, &msg);
    } else if (mod.fwtype == ICAN3_FWTYPE_CAL_CANOPEN) {
// bittiming + can-on/off request
    memset(&msg, 0, sizeof(msg));
    msg.spec = MSG_LMTS;
    if (on) {
    msg.len = cpu_to_le16(4);
    msg.data[0] = LMTS_BUSON_REQ;
    msg.data[1] = 0;
    msg.data[2] = btr0;
    msg.data[3] = btr1;
    } else {
    msg.len = cpu_to_le16(2);
    msg.data[0] = LMTS_BUSOFF_REQ;
    msg.data[1] = 0;
    }
    res = ican3_send_msg(mod, &msg);
    if (res)
    return res;
    if (on) {
// create NMT Slave Node for error processing
// class 2 (with error capability, see CiA/DS203-1)
// id    1
// name  locnod1 (must be exactly 7 bytes)
//
    memset(&msg, 0, sizeof(msg));
    msg.spec = MSG_NMTS;
    msg.len = cpu_to_le16(11);
    msg.data[0] = NMTS_CREATE_NODE_REQ;
    msg.data[1] = 0;
    msg.data[2] = 2;                 /* node class */
    msg.data[3] = 1;                 /* node id */
    strcpy(msg.data + 4, "locnod1"); /* node name  */
    return ican3_send_msg(mod, &msg);
    }
    return 0;
    }
    return -ENOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn ican3_set_termination(mod: *mut ican3_dev, on: bool) -> c_int {
    static int ican3_set_termination(struct ican3_dev *mod, bool on)
    {
    struct ican3_msg msg;
    memset(&msg, 0, sizeof(msg));
    msg.spec = MSG_HWCONF;
    msg.len = cpu_to_le16(2);
    msg.data[0] = 0x00;
    msg.data[1] = on ? HWCONF_TERMINATE_ON : HWCONF_TERMINATE_OFF;
    return ican3_send_msg(mod, &msg);
    }
#[no_mangle]
unsafe extern "C" fn ican3_send_inquiry(mod: *mut ican3_dev, subspec: u8) -> c_int {
    static int ican3_send_inquiry(struct ican3_dev *mod, u8 subspec)
    {
    struct ican3_msg msg;
    memset(&msg, 0, sizeof(msg));
    msg.spec = MSG_INQUIRY;
    msg.len = cpu_to_le16(2);
    msg.data[0] = subspec;
    msg.data[1] = 0x00;
    return ican3_send_msg(mod, &msg);
    }
#[no_mangle]
unsafe extern "C" fn ican3_set_buserror(mod: *mut ican3_dev, quota: u8) -> c_int {
    static int ican3_set_buserror(struct ican3_dev *mod, u8 quota)
    {
    struct ican3_msg msg;
    if (mod.fwtype == ICAN3_FWTYPE_ICANOS) {
    memset(&msg, 0, sizeof(msg));
    msg.spec = MSG_CCONFREQ;
    msg.len = cpu_to_le16(2);
    msg.data[0] = 0x00;
    msg.data[1] = quota;
    } else if (mod.fwtype == ICAN3_FWTYPE_CAL_CANOPEN) {
    memset(&msg, 0, sizeof(msg));
    msg.spec = MSG_LMTS;
    msg.len = cpu_to_le16(4);
    msg.data[0] = LMTS_CAN_CONF_REQ;
    msg.data[1] = 0x00;
    msg.data[2] = 0x00;
    msg.data[3] = quota;
    } else {
    return -ENOTSUPP;
    }
    return ican3_send_msg(mod, &msg);
    }
//
// ICAN3 to Linux CAN Frame Conversion
//
    static void ican3_to_can_frame(struct ican3_dev *mod,
    struct ican3_fast_desc *desc,
    struct can_frame *cf)
    {
    if ((desc.command & ICAN3_CAN_TYPE_MASK) == ICAN3_CAN_TYPE_SFF) {
    if (desc.data[1] & ICAN3_SFF_RTR)
    cf.can_id |= CAN_RTR_FLAG;
    cf.can_id |= desc.data[0] << 3;
    cf.can_id |= (desc.data[1] & 0xe0) >> 5;
    cf.len = can_cc_dlc2len(desc.data[1] & ICAN3_CAN_DLC_MASK);
    memcpy(cf.data, &desc.data[2], cf.len);
    } else {
    cf.len = can_cc_dlc2len(desc.data[0] & ICAN3_CAN_DLC_MASK);
    if (desc.data[0] & ICAN3_EFF_RTR)
    cf.can_id |= CAN_RTR_FLAG;
    if (desc.data[0] & ICAN3_EFF) {
    cf.can_id |= CAN_EFF_FLAG;
    cf.can_id |= desc.data[2] << 21; /* 28-21 */
    cf.can_id |= desc.data[3] << 13; /* 20-13 */
    cf.can_id |= desc.data[4] << 5;  /* 12-5  */
    cf.can_id |= (desc.data[5] & 0xf8) >> 3;
    } else {
    cf.can_id |= desc.data[2] << 3;  /* 10-3  */
    cf.can_id |= desc.data[3] >> 5;  /* 2-0   */
    }
    memcpy(cf.data, &desc.data[6], cf.len);
    }
    }
    static void can_frame_to_ican3(struct ican3_dev *mod,
    struct can_frame *cf,
    struct ican3_fast_desc *desc)
    {
// clear out any stale data in the descriptor
    memset(desc.data, 0, sizeof(desc.data));
// we always use the extended format, with the ECHO flag set
    desc.command = ICAN3_CAN_TYPE_EFF;
    desc.data[0] |= cf.len;
    desc.data[1] |= ICAN3_ECHO;
// support single transmission (no retries) mode
    if (mod.can.ctrlmode & CAN_CTRLMODE_ONE_SHOT)
    desc.data[1] |= ICAN3_SNGL;
    if (cf.can_id & CAN_RTR_FLAG)
    desc.data[0] |= ICAN3_EFF_RTR;
// pack the id into the correct places
    if (cf.can_id & CAN_EFF_FLAG) {
    desc.data[0] |= ICAN3_EFF;
    desc.data[2] = (cf.can_id & 0x1fe00000) >> 21; /* 28-21 */
    desc.data[3] = (cf.can_id & 0x001fe000) >> 13; /* 20-13 */
    desc.data[4] = (cf.can_id & 0x00001fe0) >> 5;  /* 12-5  */
    desc.data[5] = (cf.can_id & 0x0000001f) << 3;  /* 4-0   */
    } else {
    desc.data[2] = (cf.can_id & 0x7F8) >> 3; /* bits 10-3 */
    desc.data[3] = (cf.can_id & 0x007) << 5; /* bits 2-0  */
    }
// copy the data bits into the descriptor
    memcpy(&desc.data[6], cf.data, cf.len);
    }
//
// Interrupt Handling
//
// Handle an ID + Version message response from the firmware. We never generate
// this message in production code, but it is very useful when debugging to be
// able to display this message.
//
#[no_mangle]
unsafe extern "C" fn ican3_handle_idvers(mod: *mut ican3_dev, msg: *mut ican3_msg) {
    static void ican3_handle_idvers(struct ican3_dev *mod, struct ican3_msg *msg)
    {
    netdev_dbg(mod.ndev, "IDVERS response: %s\n", msg.data);
    }
#[no_mangle]
unsafe extern "C" fn ican3_handle_msglost(mod: *mut ican3_dev, msg: *mut ican3_msg) {
    static void ican3_handle_msglost(struct ican3_dev *mod, struct ican3_msg *msg)
    {
    struct net_device *dev = mod.ndev;
    struct net_device_stats *stats = &dev.stats;
    struct can_frame *cf;
    struct sk_buff *skb;
//
// Report that communication messages with the microcontroller firmware
// are being lost. These are never CAN frames, so we do not generate an
// error frame for userspace
//
    if (msg.spec == MSG_MSGLOST) {
    netdev_err(mod.ndev, "lost %d control messages\n", msg.data[0]);
    return;
    }
//
// Oops, this indicates that we have lost messages in the fast queue,
// which are exclusively CAN messages. Our driver isn't reading CAN
// frames fast enough.
//
// We'll pretend that the SJA1000 told us that it ran out of buffer
// space, because there is not a better message for this.
//
    skb = alloc_can_err_skb(dev, &cf);
    if (skb) {
    cf.can_id |= CAN_ERR_CRTL;
    cf.data[1] = CAN_ERR_CRTL_RX_OVERFLOW;
    stats.rx_over_errors++;
    stats.rx_errors++;
    netif_rx(skb);
    }
    }
//
// Handle CAN Event Indication Messages from the firmware
//
// The ICAN3 firmware provides the values of some SJA1000 registers when it
// generates this message. The code below is largely copied from the
// drivers/net/can/sja1000/sja1000.c file, and adapted as necessary
//
#[no_mangle]
unsafe extern "C" fn ican3_handle_cevtind(mod: *mut ican3_dev, msg: *mut ican3_msg) -> c_int {
    static int ican3_handle_cevtind(struct ican3_dev *mod, struct ican3_msg *msg)
    {
    struct net_device *dev = mod.ndev;
    struct net_device_stats *stats = &dev.stats;
    let mut state: enum can_state = mod.can.state;
    u8 isrc, ecc, status, rxerr, txerr;
    struct can_frame *cf;
    struct sk_buff *skb;
// we can only handle the SJA1000 part
    if (msg.data[1] != CEVTIND_CHIP_SJA1000) {
    netdev_err(mod.ndev, "unable to handle errors on non-SJA1000\n");
    return -ENODEV;
    }
// check the message length for sanity
    if (le16_to_cpu(msg.len) < 6) {
    netdev_err(mod.ndev, "error message too short\n");
    return -EINVAL;
    }
    isrc = msg.data[0];
    ecc = msg.data[2];
    status = msg.data[3];
    rxerr = msg.data[4];
    txerr = msg.data[5];
//
// This hardware lacks any support other than bus error messages to
// determine if packet transmission has failed.
//
// When TX errors happen, one echo skb needs to be dropped from the
// front of the queue.
//
// A small bit of code is duplicated here and below, to avoid error
// skb allocation when it will just be freed immediately.
//
    if (isrc == CEVTIND_BEI) {
    int ret;
    netdev_dbg(mod.ndev, "bus error interrupt\n");
// TX error
    if (!(ecc & ECC_DIR)) {
    kfree_skb(skb_dequeue(&mod.echoq));
    stats.tx_errors++;
    } else {
    stats.rx_errors++;
    }
//
// The controller automatically disables bus-error interrupts
// and therefore we must re-enable them.
//
    ret = ican3_set_buserror(mod, 1);
    if (ret) {
    netdev_err(mod.ndev, "unable to re-enable bus-error\n");
    return ret;
    }
// bus error reporting is off, return immediately
    if (!(mod.can.ctrlmode & CAN_CTRLMODE_BERR_REPORTING))
    return 0;
    }
    skb = alloc_can_err_skb(dev, &cf);
    if (skb == core::ptr::null_mut())
    return -ENOMEM;
// data overrun interrupt
    if (isrc == CEVTIND_DOI || isrc == CEVTIND_LOST) {
    netdev_dbg(mod.ndev, "data overrun interrupt\n");
    cf.can_id |= CAN_ERR_CRTL;
    cf.data[1] = CAN_ERR_CRTL_RX_OVERFLOW;
    stats.rx_over_errors++;
    stats.rx_errors++;
    }
// error warning + passive interrupt
    if (isrc == CEVTIND_EI) {
    netdev_dbg(mod.ndev, "error warning + passive interrupt\n");
    if (status & SR_BS) {
    state = CAN_STATE_BUS_OFF;
    cf.can_id |= CAN_ERR_BUSOFF;
    mod.can.can_stats.bus_off++;
    can_bus_off(dev);
    } else if (status & SR_ES) {
    if (rxerr >= 128 || txerr >= 128)
    state = CAN_STATE_ERROR_PASSIVE;
    else
    state = CAN_STATE_ERROR_WARNING;
    } else {
    state = CAN_STATE_ERROR_ACTIVE;
    }
    }
// bus error interrupt
    if (isrc == CEVTIND_BEI) {
    mod.can.can_stats.bus_error++;
    cf.can_id |= CAN_ERR_PROT | CAN_ERR_BUSERROR | CAN_ERR_CNT;
    switch (ecc & ECC_MASK) {
    case ECC_BIT:
    cf.data[2] |= CAN_ERR_PROT_BIT;
    break;
    case ECC_FORM:
    cf.data[2] |= CAN_ERR_PROT_FORM;
    break;
    case ECC_STUFF:
    cf.data[2] |= CAN_ERR_PROT_STUFF;
    break;
    default:
    cf.data[3] = ecc & ECC_SEG;
    break;
    }
    if (!(ecc & ECC_DIR))
    cf.data[2] |= CAN_ERR_PROT_TX;
    cf.data[6] = txerr;
    cf.data[7] = rxerr;
    }
    if (state != mod.can.state && (state == CAN_STATE_ERROR_WARNING ||
    state == CAN_STATE_ERROR_PASSIVE)) {
    cf.can_id |= CAN_ERR_CRTL | CAN_ERR_CNT;
    if (state == CAN_STATE_ERROR_WARNING) {
    mod.can.can_stats.error_warning++;
    cf.data[1] = (txerr > rxerr) ?
    CAN_ERR_CRTL_TX_WARNING :
    CAN_ERR_CRTL_RX_WARNING;
    } else {
    mod.can.can_stats.error_passive++;
    cf.data[1] = (txerr > rxerr) ?
    CAN_ERR_CRTL_TX_PASSIVE :
    CAN_ERR_CRTL_RX_PASSIVE;
    }
    cf.data[6] = txerr;
    cf.data[7] = rxerr;
    }
    mod.can.state = state;
    netif_rx(skb);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ican3_handle_inquiry(mod: *mut ican3_dev, msg: *mut ican3_msg) {
    static void ican3_handle_inquiry(struct ican3_dev *mod, struct ican3_msg *msg)
    {
    switch (msg.data[0]) {
    case INQUIRY_STATUS:
    case INQUIRY_EXTENDED:
    mod.bec.rxerr = msg.data[5];
    mod.bec.txerr = msg.data[6];
    complete(&mod.buserror_comp);
    break;
    case INQUIRY_TERMINATION:
    mod.termination_enabled = msg.data[6] & HWCONF_TERMINATE_ON;
    complete(&mod.termination_comp);
    break;
    default:
    netdev_err(mod.ndev, "received an unknown inquiry response\n");
    break;
    }
    }
// Handle NMTS Slave Event Indication Messages from the firmware
#[no_mangle]
unsafe extern "C" fn ican3_handle_nmtsind(mod: *mut ican3_dev, msg: *mut ican3_msg) {
    static void ican3_handle_nmtsind(struct ican3_dev *mod, struct ican3_msg *msg)
    {
    u16 subspec;
    subspec = msg.data[0] + msg.data[1] * 0x100;
    if (subspec == NMTS_SLAVE_EVENT_IND) {
    switch (msg.data[2]) {
    case NE_LOCAL_OCCURRED:
    case NE_LOCAL_RESOLVED:
// now follows the same message as Raw ICANOS CEVTIND
// shift the data at the same place and call this method
//
    le16_add_cpu(&msg.len, -3);
    memmove(msg.data, msg.data + 3, le16_to_cpu(msg.len));
    ican3_handle_cevtind(mod, msg);
    break;
    case NE_REMOTE_OCCURRED:
    case NE_REMOTE_RESOLVED:
// should not occurre, ignore
    break;
    default:
    netdev_warn(mod.ndev, "unknown NMTS event indication %x\n",
    msg.data[2]);
    break;
    }
    } else if (subspec == NMTS_SLAVE_STATE_IND) {
// ignore state indications
    } else {
    netdev_warn(mod.ndev, "unhandled NMTS indication %x\n",
    subspec);
    return;
    }
    }
    static void ican3_handle_unknown_message(struct ican3_dev *mod,
    struct ican3_msg *msg)
    {
    netdev_warn(mod.ndev, "received unknown message: spec 0x%.2x length %d\n",
    msg.spec, le16_to_cpu(msg.len));
    }
//
// Handle a control message from the firmware
//
#[no_mangle]
unsafe extern "C" fn ican3_handle_message(mod: *mut ican3_dev, msg: *mut ican3_msg) {
    static void ican3_handle_message(struct ican3_dev *mod, struct ican3_msg *msg)
    {
    netdev_dbg(mod.ndev, "%s: modno %d spec 0x%.2x len %d bytes\n", __func__,
    mod.num, msg.spec, le16_to_cpu(msg.len));
    switch (msg.spec) {
    case MSG_IDVERS:
    ican3_handle_idvers(mod, msg);
    break;
    case MSG_MSGLOST:
    case MSG_FMSGLOST:
    ican3_handle_msglost(mod, msg);
    break;
    case MSG_CEVTIND:
    ican3_handle_cevtind(mod, msg);
    break;
    case MSG_INQUIRY:
    ican3_handle_inquiry(mod, msg);
    break;
    case MSG_NMTS:
    ican3_handle_nmtsind(mod, msg);
    break;
    default:
    ican3_handle_unknown_message(mod, msg);
    break;
    }
    }
//
// The ican3 needs to store all echo skbs, and therefore cannot
// use the generic infrastructure for this.
//
#[no_mangle]
unsafe extern "C" fn ican3_put_echo_skb(mod: *mut ican3_dev, skb: *mut sk_buff) {
    static void ican3_put_echo_skb(struct ican3_dev *mod, struct sk_buff *skb)
    {
    skb = can_create_echo_skb(skb);
    if (!skb)
    return;
    skb_tx_timestamp(skb);
// save this skb for tx interrupt echo handling
    skb_queue_tail(&mod.echoq, skb);
    }
#[no_mangle]
unsafe extern "C" fn ican3_get_echo_skb(mod: *mut ican3_dev) -> c_uint {
    static unsigned int ican3_get_echo_skb(struct ican3_dev *mod)
    {
    struct sk_buff *skb = skb_dequeue(&mod.echoq);
    struct can_frame *cf;
    let mut dlc: u8 = 0;
// this should never trigger unless there is a driver bug
    if (!skb) {
    netdev_err(mod.ndev, "BUG: echo skb not occupied\n");
    return 0;
    }
    cf = (struct can_frame *)skb.data;
    if (!(cf.can_id & CAN_RTR_FLAG))
    dlc = cf.len;
// check flag whether this packet has to be looped back
    if (skb.pkt_type != PACKET_LOOPBACK) {
    kfree_skb(skb);
    return dlc;
    }
    skb.protocol = htons(ETH_P_CAN);
    skb.pkt_type = PACKET_BROADCAST;
    skb.ip_summed = CHECKSUM_UNNECESSARY;
    skb.dev = mod.ndev;
    netif_receive_skb(skb);
    return dlc;
    }
//
// Compare an skb with an existing echo skb
//
// This function will be used on devices which have a hardware loopback.
// On these devices, this function can be used to compare a received skb
// with the saved echo skbs so that the hardware echo skb can be dropped.
//
// Returns true if the skb's are identical, false otherwise.
//
#[no_mangle]
unsafe extern "C" fn ican3_echo_skb_matches(mod: *mut ican3_dev, skb: *mut sk_buff) -> bool {
    static bool ican3_echo_skb_matches(struct ican3_dev *mod, struct sk_buff *skb)
    {
    struct can_frame *cf = (struct can_frame *)skb.data;
    struct sk_buff *echo_skb = skb_peek(&mod.echoq);
    struct can_frame *echo_cf;
    if (!echo_skb)
    return false;
    echo_cf = (struct can_frame *)echo_skb.data;
    if (cf.can_id != echo_cf.can_id)
    return false;
    if (cf.len != echo_cf.len)
    return false;
    return memcmp(cf.data, echo_cf.data, cf.len) == 0;
    }
//
// Check that there is room in the TX ring to transmit another skb
//
// LOCKING: must hold mod->lock
//
#[no_mangle]
unsafe extern "C" fn ican3_txok(mod: *mut ican3_dev) -> bool {
    static bool ican3_txok(struct ican3_dev *mod)
    {
    struct ican3_fast_desc __iomem *desc;
    u8 control;
// check that we have echo queue space
    if (skb_queue_len(&mod.echoq) >= ICAN3_TX_BUFFERS)
    return false;
// copy the control bits of the descriptor
    ican3_set_page(mod, mod.fasttx_start + (mod.fasttx_num / 16));
    desc = mod.dpm + ((mod.fasttx_num % 16) * sizeof(*desc));
    control = ioread8(&desc.control);
// if the control bits are not valid, then we have no more space
    if (!(control & DESC_VALID))
    return false;
    return true;
    }
//
// Receive one CAN frame from the hardware
//
// CONTEXT: must be called from user context
//
#[no_mangle]
unsafe extern "C" fn ican3_recv_skb(mod: *mut ican3_dev) -> c_int {
    static int ican3_recv_skb(struct ican3_dev *mod)
    {
    struct net_device *ndev = mod.ndev;
    struct net_device_stats *stats = &ndev.stats;
    struct ican3_fast_desc desc;
    void __iomem *desc_addr;
    struct can_frame *cf;
    struct sk_buff *skb;
    unsigned long flags;
    spin_lock_irqsave(&mod.lock, flags);
// copy the whole descriptor
    ican3_set_page(mod, mod.fastrx_start + (mod.fastrx_num / 16));
    desc_addr = mod.dpm + ((mod.fastrx_num % 16) * sizeof(desc));
    memcpy_fromio(&desc, desc_addr, sizeof(desc));
    spin_unlock_irqrestore(&mod.lock, flags);
// check that we actually have a CAN frame
    if (!(desc.control & DESC_VALID))
    return -ENOBUFS;
// allocate an skb
    skb = alloc_can_skb(ndev, &cf);
    if (unlikely(skb == core::ptr::null_mut())) {
    stats.rx_dropped++;
    goto err_noalloc;
    }
// convert the ICAN3 frame into Linux CAN format
    ican3_to_can_frame(mod, &desc, cf);
//
// If this is an ECHO frame received from the hardware loopback
// feature, use the skb saved in the ECHO stack instead. This allows
// the Linux CAN core to support CAN_RAW_RECV_OWN_MSGS correctly.
//
// Since this is a confirmation of a successfully transmitted packet
// sent from this host, update the transmit statistics.
//
// Also, the netdevice queue needs to be allowed to send packets again.
//
    if (ican3_echo_skb_matches(mod, skb)) {
    stats.tx_packets++;
    stats.tx_bytes += ican3_get_echo_skb(mod);
    kfree_skb(skb);
    goto err_noalloc;
    }
// update statistics, receive the skb
    stats.rx_packets++;
    if (!(cf.can_id & CAN_RTR_FLAG))
    stats.rx_bytes += cf.len;
    netif_receive_skb(skb);
    err_noalloc:
// toggle the valid bit and return the descriptor to the ring
    desc.control ^= DESC_VALID;
    spin_lock_irqsave(&mod.lock, flags);
    ican3_set_page(mod, mod.fastrx_start + (mod.fastrx_num / 16));
    memcpy_toio(desc_addr, &desc, 1);
// update the next buffer pointer
    mod.fastrx_num = (desc.control & DESC_WRAP) ? 0
    : (mod.fastrx_num + 1);
// there are still more buffers to process
    spin_unlock_irqrestore(&mod.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ican3_napi(napi: *mut napi_struct, budget: c_int) -> c_int {
    static int ican3_napi(struct napi_struct *napi, int budget)
    {
    struct ican3_dev *mod = container_of(napi, struct ican3_dev, napi);
    unsigned long flags;
    let mut received: c_int = 0;
    int ret;
// process all communication messages
    while (true) {
    struct ican3_msg msg;
    ret = ican3_recv_msg(mod, &msg);
    if (ret)
    break;
    ican3_handle_message(mod, &msg);
    }
// process all CAN frames from the fast interface
    while (received < budget) {
    ret = ican3_recv_skb(mod);
    if (ret)
    break;
    received++;
    }
// We have processed all packets that the adapter had, but it
// was less than our budget, stop polling
    if (received < budget)
    napi_complete_done(napi, received);
    spin_lock_irqsave(&mod.lock, flags);
// Wake up the transmit queue if necessary
    if (netif_queue_stopped(mod.ndev) && ican3_txok(mod))
    netif_wake_queue(mod.ndev);
    spin_unlock_irqrestore(&mod.lock, flags);
// re-enable interrupt generation
    iowrite8(1 << mod.num, &mod.ctrl.int_enable);
    return received;
    }
#[no_mangle]
unsafe extern "C" fn ican3_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t ican3_irq(int irq, void *dev_id)
    {
    struct ican3_dev *mod = dev_id;
    u8 stat;
//
// The interrupt status register on this device reports interrupts
// as zeroes instead of using ones like most other devices
//
    stat = ioread8(&mod.ctrl.int_disable) & (1 << mod.num);
    if (stat == (1 << mod.num))
    return IRQ_NONE;
// clear the MODULbus interrupt from the microcontroller
    ioread8(&mod.dpmctrl.interrupt);
// disable interrupt generation, schedule the NAPI poller
    iowrite8(1 << mod.num, &mod.ctrl.int_disable);
    napi_schedule(&mod.napi);
    return IRQ_HANDLED;
    }
//
// Firmware reset, startup, and shutdown
//
// Reset an ICAN module to its power-on state
//
// CONTEXT: no network device registered
//
#[no_mangle]
unsafe extern "C" fn ican3_reset_module(mod: *mut ican3_dev) -> c_int {
    static int ican3_reset_module(struct ican3_dev *mod)
    {
    unsigned long start;
    u8 runold, runnew;
// disable interrupts so no more work is scheduled
    iowrite8(1 << mod.num, &mod.ctrl.int_disable);
// the first unallocated page in the DPM is #9
    mod.free_page = DPM_FREE_START;
    ican3_set_page(mod, QUEUE_OLD_CONTROL);
    runold = ioread8(mod.dpm + TARGET_RUNNING);
// reset the module
    iowrite8(0x00, &mod.dpmctrl.hwreset);
// wait until the module has finished resetting and is running
    start = jiffies;
    do {
    ican3_set_page(mod, QUEUE_OLD_CONTROL);
    runnew = ioread8(mod.dpm + TARGET_RUNNING);
    if (runnew == (runold ^ 0xff))
    return 0;
    msleep(10);
    } while (time_before(jiffies, start + HZ / 2));
    netdev_err(mod.ndev, "failed to reset CAN module\n");
    return -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn ican3_shutdown_module(mod: *mut ican3_dev) {
    static void ican3_shutdown_module(struct ican3_dev *mod)
    {
    ican3_msg_disconnect(mod);
    ican3_reset_module(mod);
    }
//
// Startup an ICAN module, bringing it into fast mode
//
#[no_mangle]
unsafe extern "C" fn ican3_startup_module(mod: *mut ican3_dev) -> c_int {
    static int ican3_startup_module(struct ican3_dev *mod)
    {
    int ret;
    ret = ican3_reset_module(mod);
    if (ret) {
    netdev_err(mod.ndev, "unable to reset module\n");
    return ret;
    }
// detect firmware
    memcpy_fromio(mod.fwinfo, mod.dpm + FIRMWARE_STAMP, sizeof(mod.fwinfo) - 1);
    if (strncmp(mod.fwinfo, "JANZ-ICAN3", 10)) {
    netdev_err(mod.ndev, "ICAN3 not detected (found %s)\n", mod.fwinfo);
    return -ENODEV;
    }
    if (strstr(mod.fwinfo, "CAL/CANopen"))
    mod.fwtype = ICAN3_FWTYPE_CAL_CANOPEN;
    else
    mod.fwtype = ICAN3_FWTYPE_ICANOS;
// re-enable interrupts so we can send messages
    iowrite8(1 << mod.num, &mod.ctrl.int_enable);
    ret = ican3_msg_connect(mod);
    if (ret) {
    netdev_err(mod.ndev, "unable to connect to module\n");
    return ret;
    }
    ican3_init_new_host_interface(mod);
    ret = ican3_msg_newhostif(mod);
    if (ret) {
    netdev_err(mod.ndev, "unable to switch to new-style interface\n");
    return ret;
    }
// default to "termination on"
    ret = ican3_set_termination(mod, true);
    if (ret) {
    netdev_err(mod.ndev, "unable to enable termination\n");
    return ret;
    }
// default to "bus errors enabled"
    ret = ican3_set_buserror(mod, 1);
    if (ret) {
    netdev_err(mod.ndev, "unable to set bus-error\n");
    return ret;
    }
    ican3_init_fast_host_interface(mod);
    ret = ican3_msg_fasthostif(mod);
    if (ret) {
    netdev_err(mod.ndev, "unable to switch to fast host interface\n");
    return ret;
    }
    ret = ican3_set_id_filter(mod, true);
    if (ret) {
    netdev_err(mod.ndev, "unable to set acceptance filter\n");
    return ret;
    }
    return 0;
    }
//
// CAN Network Device
//
#[no_mangle]
unsafe extern "C" fn ican3_open(ndev: *mut net_device) -> c_int {
    static int ican3_open(struct net_device *ndev)
    {
    struct ican3_dev *mod = netdev_priv(ndev);
    int ret;
// open the CAN layer
    ret = open_candev(ndev);
    if (ret) {
    netdev_err(mod.ndev, "unable to start CAN layer\n");
    return ret;
    }
// bring the bus online
    ret = ican3_set_bus_state(mod, true);
    if (ret) {
    netdev_err(mod.ndev, "unable to set bus-on\n");
    close_candev(ndev);
    return ret;
    }
// start up the network device
    mod.can.state = CAN_STATE_ERROR_ACTIVE;
    netif_start_queue(ndev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ican3_stop(ndev: *mut net_device) -> c_int {
    static int ican3_stop(struct net_device *ndev)
    {
    struct ican3_dev *mod = netdev_priv(ndev);
    int ret;
// stop the network device xmit routine
    netif_stop_queue(ndev);
    mod.can.state = CAN_STATE_STOPPED;
// bring the bus offline, stop receiving packets
    ret = ican3_set_bus_state(mod, false);
    if (ret) {
    netdev_err(mod.ndev, "unable to set bus-off\n");
    return ret;
    }
// drop all outstanding echo skbs
    skb_queue_purge(&mod.echoq);
// close the CAN layer
    close_candev(ndev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ican3_xmit(skb: *mut sk_buff, ndev: *mut net_device) -> netdev_tx_t {
    static netdev_tx_t ican3_xmit(struct sk_buff *skb, struct net_device *ndev)
    {
    struct ican3_dev *mod = netdev_priv(ndev);
    struct can_frame *cf = (struct can_frame *)skb.data;
    struct ican3_fast_desc desc;
    void __iomem *desc_addr;
    unsigned long flags;
    if (can_dev_dropped_skb(ndev, skb))
    return NETDEV_TX_OK;
    spin_lock_irqsave(&mod.lock, flags);
// check that we can actually transmit
    if (!ican3_txok(mod)) {
    netdev_err(mod.ndev, "BUG: no free descriptors\n");
    spin_unlock_irqrestore(&mod.lock, flags);
    return NETDEV_TX_BUSY;
    }
// copy the control bits of the descriptor
    ican3_set_page(mod, mod.fasttx_start + (mod.fasttx_num / 16));
    desc_addr = mod.dpm + ((mod.fasttx_num % 16) * sizeof(desc));
    memset(&desc, 0, sizeof(desc));
    memcpy_fromio(&desc, desc_addr, 1);
// convert the Linux CAN frame into ICAN3 format
    can_frame_to_ican3(mod, cf, &desc);
//
// This hardware doesn't have TX-done notifications, so we'll try and
// emulate it the best we can using ECHO skbs. Add the skb to the ECHO
// stack. Upon packet reception, check if the ECHO skb and received
// skb match, and use that to wake the queue.
//
    ican3_put_echo_skb(mod, skb);
//
// the programming manual says that you must set the IVALID bit, then
// interrupt, then set the valid bit. Quite weird, but it seems to be
// required for this to work
//
    desc.control |= DESC_IVALID;
    memcpy_toio(desc_addr, &desc, sizeof(desc));
// generate a MODULbus interrupt to the microcontroller
    iowrite8(0x01, &mod.dpmctrl.interrupt);
    desc.control ^= DESC_VALID;
    memcpy_toio(desc_addr, &desc, sizeof(desc));
// update the next buffer pointer
    mod.fasttx_num = (desc.control & DESC_WRAP) ? 0
    : (mod.fasttx_num + 1);
// if there is no free descriptor space, stop the transmit queue
    if (!ican3_txok(mod))
    netif_stop_queue(ndev);
    spin_unlock_irqrestore(&mod.lock, flags);
    return NETDEV_TX_OK;
    }
    static const struct net_device_ops ican3_netdev_ops = {
    .ndo_open	= ican3_open,
    .ndo_stop	= ican3_stop,
    .ndo_start_xmit	= ican3_xmit,
    };
    static const struct ethtool_ops ican3_ethtool_ops = {
    .get_ts_info = ethtool_op_get_ts_info,
    };
//
// Low-level CAN Device
//
// This structure was stolen from drivers/net/can/sja1000/sja1000.c
    static const struct can_bittiming_const ican3_bittiming_const = {
    .name = DRV_NAME,
    .tseg1_min = 1,
    .tseg1_max = 16,
    .tseg2_min = 1,
    .tseg2_max = 8,
    .sjw_max = 4,
    .brp_min = 1,
    .brp_max = 64,
    .brp_inc = 1,
    };
#[no_mangle]
unsafe extern "C" fn ican3_set_mode(ndev: *mut net_device, mode: enum can_mode) -> c_int {
    static int ican3_set_mode(struct net_device *ndev, enum can_mode mode)
    {
    struct ican3_dev *mod = netdev_priv(ndev);
    int ret;
    if (mode != CAN_MODE_START)
    return -ENOTSUPP;
// bring the bus online
    ret = ican3_set_bus_state(mod, true);
    if (ret) {
    netdev_err(ndev, "unable to set bus-on\n");
    return ret;
    }
// start up the network device
    mod.can.state = CAN_STATE_ERROR_ACTIVE;
    if (netif_queue_stopped(ndev))
    netif_wake_queue(ndev);
    return 0;
    }
    static int ican3_get_berr_counter(const struct net_device *ndev,
    struct can_berr_counter *bec)
    {
    struct ican3_dev *mod = netdev_priv(ndev);
    int ret;
    ret = ican3_send_inquiry(mod, INQUIRY_STATUS);
    if (ret)
    return ret;
    if (!wait_for_completion_timeout(&mod.buserror_comp, HZ)) {
    netdev_info(mod.ndev, "%s timed out\n", __func__);
    return -ETIMEDOUT;
    }
    bec.rxerr = mod.bec.rxerr;
    bec.txerr = mod.bec.txerr;
    return 0;
    }
//
// Sysfs Attributes
//
    static ssize_t termination_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct ican3_dev *mod = netdev_priv(to_net_dev(dev));
    int ret;
    ret = ican3_send_inquiry(mod, INQUIRY_TERMINATION);
    if (ret)
    return ret;
    if (!wait_for_completion_timeout(&mod.termination_comp, HZ)) {
    netdev_info(mod.ndev, "%s timed out\n", __func__);
    return -ETIMEDOUT;
    }
    return sysfs_emit(buf, "%u\n", mod.termination_enabled);
    }
    static ssize_t termination_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct ican3_dev *mod = netdev_priv(to_net_dev(dev));
    unsigned long enable;
    int ret;
    if (kstrtoul(buf, 0, &enable))
    return -EINVAL;
    ret = ican3_set_termination(mod, enable);
    if (ret)
    return ret;
    return count;
    }
    static ssize_t fwinfo_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct ican3_dev *mod = netdev_priv(to_net_dev(dev));
    return sysfs_emit(buf, "%s\n", mod.fwinfo);
    }
    static DEVICE_ATTR_RW(termination);
    static DEVICE_ATTR_RO(fwinfo);
    static struct attribute *ican3_sysfs_attrs[] = {
    &dev_attr_termination.attr,
    &dev_attr_fwinfo.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group ican3_sysfs_attr_group = {
    .attrs = ican3_sysfs_attrs,
    };
//
// PCI Subsystem
//
#[no_mangle]
unsafe extern "C" fn ican3_probe(pdev: *mut platform_device) -> c_int {
    static int ican3_probe(struct platform_device *pdev)
    {
    struct janz_platform_data *pdata;
    struct net_device *ndev;
    struct ican3_dev *mod;
    struct resource *res;
    struct device *dev;
    int ret;
    pdata = dev_get_platdata(&pdev.dev);
    if (!pdata)
    return -ENXIO;
    dev_dbg(&pdev.dev, "probe: module number %d\n", pdata.modno);
// save the struct device for printing
    dev = &pdev.dev;
// allocate the CAN device and private data
    ndev = alloc_candev(sizeof(*mod), 0);
    if (!ndev) {
    dev_err(dev, "unable to allocate CANdev\n");
    ret = -ENOMEM;
    goto out_return;
    }
    platform_set_drvdata(pdev, ndev);
    mod = netdev_priv(ndev);
    mod.ndev = ndev;
    mod.num = pdata.modno;
    netif_napi_add_weight(ndev, &mod.napi, ican3_napi, ICAN3_RX_BUFFERS);
    skb_queue_head_init(&mod.echoq);
    spin_lock_init(&mod.lock);
    init_completion(&mod.termination_comp);
    init_completion(&mod.buserror_comp);
// setup device-specific sysfs attributes
    ndev.sysfs_groups[0] = &ican3_sysfs_attr_group;
// the first unallocated page in the DPM is 9
    mod.free_page = DPM_FREE_START;
    ndev.netdev_ops = &ican3_netdev_ops;
    ndev.ethtool_ops = &ican3_ethtool_ops;
    ndev.flags |= IFF_ECHO;
    SET_NETDEV_DEV(ndev, &pdev.dev);
    mod.can.clock.freq = ICAN3_CAN_CLOCK;
    mod.can.bittiming_const = &ican3_bittiming_const;
    mod.can.do_set_mode = ican3_set_mode;
    mod.can.do_get_berr_counter = ican3_get_berr_counter;
    mod.can.ctrlmode_supported = CAN_CTRLMODE_3_SAMPLES
    | CAN_CTRLMODE_BERR_REPORTING
    | CAN_CTRLMODE_ONE_SHOT;
// find our IRQ number
    mod.irq = platform_get_irq(pdev, 0);
    if (mod.irq < 0) {
    ret = -ENODEV;
    goto out_free_ndev;
    }
    ndev.irq = mod.irq;
// get access to the MODULbus registers for this module
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res) {
    dev_err(dev, "MODULbus registers not found\n");
    ret = -ENODEV;
    goto out_free_ndev;
    }
    mod.dpm = ioremap(res.start, resource_size(res));
    if (!mod.dpm) {
    dev_err(dev, "MODULbus registers not ioremap\n");
    ret = -ENOMEM;
    goto out_free_ndev;
    }
    mod.dpmctrl = mod.dpm + DPM_PAGE_SIZE;
// get access to the control registers for this module
    res = platform_get_resource(pdev, IORESOURCE_MEM, 1);
    if (!res) {
    dev_err(dev, "CONTROL registers not found\n");
    ret = -ENODEV;
    goto out_iounmap_dpm;
    }
    mod.ctrl = ioremap(res.start, resource_size(res));
    if (!mod.ctrl) {
    dev_err(dev, "CONTROL registers not ioremap\n");
    ret = -ENOMEM;
    goto out_iounmap_dpm;
    }
// disable our IRQ, then hookup the IRQ handler
    iowrite8(1 << mod.num, &mod.ctrl.int_disable);
    ret = request_irq(mod.irq, ican3_irq, IRQF_SHARED, DRV_NAME, mod);
    if (ret) {
    dev_err(dev, "unable to request IRQ\n");
    goto out_iounmap_ctrl;
    }
// reset and initialize the CAN controller into fast mode
    napi_enable(&mod.napi);
    ret = ican3_startup_module(mod);
    if (ret) {
    dev_err(dev, "%s: unable to start CANdev\n", __func__);
    goto out_free_irq;
    }
// register with the Linux CAN layer
    ret = register_candev(ndev);
    if (ret) {
    dev_err(dev, "%s: unable to register CANdev\n", __func__);
    goto out_free_irq;
    }
    netdev_info(mod.ndev, "module %d: registered CAN device\n", pdata.modno);
    return 0;
    out_free_irq:
    napi_disable(&mod.napi);
    iowrite8(1 << mod.num, &mod.ctrl.int_disable);
    free_irq(mod.irq, mod);
    out_iounmap_ctrl:
    iounmap(mod.ctrl);
    out_iounmap_dpm:
    iounmap(mod.dpm);
    out_free_ndev:
    free_candev(ndev);
    out_return:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ican3_remove(pdev: *mut platform_device) {
    static void ican3_remove(struct platform_device *pdev)
    {
    struct net_device *ndev = platform_get_drvdata(pdev);
    struct ican3_dev *mod = netdev_priv(ndev);
// unregister the netdevice, stop interrupts
    unregister_netdev(ndev);
    napi_disable(&mod.napi);
    iowrite8(1 << mod.num, &mod.ctrl.int_disable);
    free_irq(mod.irq, mod);
// put the module into reset
    ican3_shutdown_module(mod);
// unmap all registers
    iounmap(mod.ctrl);
    iounmap(mod.dpm);
    free_candev(ndev);
    }
    static struct platform_driver ican3_driver = {
    .driver		= {
    .name	= DRV_NAME,
    },
    .probe		= ican3_probe,
    .remove		= ican3_remove,
    };
    module_platform_driver(ican3_driver);
    MODULE_AUTHOR("Ira W. Snyder <iws@ovro.caltech.edu>");
    MODULE_DESCRIPTION("Janz MODULbus VMOD-ICAN3 Driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:janz-ican3");
