//! Automatically rewritten from C to Rust
//! Source: drivers/mailbox/arm_mhuv3.c
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
// ARM Message Handling Unit Version 3 (MHUv3) driver.
//
// Copyright (C) 2024 ARM Ltd.
//
// Based on ARM MHUv2 driver.
//

// ====== MHUv3 Registers ======
// Maximum number of Doorbell channel windows
pub const MHUV3_DBCW_MAX: c_int = 128;
// Number of DBCH combined interrupt status registers
pub const MHUV3_DBCH_CMB_INT_ST_REG_CNT: c_int = 4;
// Number of FFCH combined interrupt status registers
pub const MHUV3_FFCH_CMB_INT_ST_REG_CNT: c_int = 2;
pub const MHUV3_FLAG_BITS: c_int = 32;
// Not a typo ...
pub const MHUV3_MAJOR_VERSION: c_int = 2;
    enum {
    MHUV3_MBOX_CELL_TYPE,
    MHUV3_MBOX_CELL_CHWN,
    MHUV3_MBOX_CELL_PARAM,
    MHUV3_MBOX_CELLS
    };
// Padding bitfields/fields represents hole in the regs MMIO
// CTRL_Page
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_id {

    pub val: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct feat_spt0 {

    pub val: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct feat_spt1 {

    pub val: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbch_cfg0 {

    pub val: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ffch_cfg0 {

    pub val: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fch_cfg0 {

    pub val: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctrl {

    pub val: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fch_ctrl {

    pub val: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iidr {

    pub val: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aidr {

    pub val: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctrl_page {
    pub blk_id: blk_id,
    pub pad: [u8; 12],
    pub feat_spt0: feat_spt0,
    pub feat_spt1: feat_spt1,
    pub pad1: [u8; 8],
    pub dbch_cfg0: dbch_cfg0,
    pub pad2: [u8; 12],
    pub ffch_cfg0: ffch_cfg0,
    pub pad3: [u8; 12],
    pub fch_cfg0: fch_cfg0,
    pub pad4: [u8; 188],
    pub x_ctrl: ctrl,
// -- MBX-only registers --
    pub pad5: [u8; 60],
    pub fch_ctrl: fch_ctrl,
    pub fcg_int_en: u32,
    pub pad6: [u8; 696],
// -- End of MBX-only ----
    pub dbch_int_st: [u32; MHUV3_DBCH_CMB_INT_ST_REG_CNT],
    pub ffch_int_st: [u32; MHUV3_FFCH_CMB_INT_ST_REG_CNT],
// -- MBX-only registers --
    pub pad7: [u8; 88],
    pub fcg_int_st: u32,
    pub pad8: [u8; 12],
    pub fcg_grp_int_st: [u32; 32],
    pub pad9: [u8; 2760],
// -- End of MBX-only ----
    pub iidr: iidr,
    pub aidr: aidr,
    pub imp_def_id: [u32; 12],
    pub __packed: },
// DBCW_Page
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xbcw_ctrl {

    pub val: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pdbcw_int {

    pub val: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pdbcw_page {
    pub st: u32,
    pub pad: [u8; 8],
    pub set: u32,
    pub int_st: pdbcw_int,
    pub int_clr: pdbcw_int,
    pub int_en: pdbcw_int,
    pub ctrl: xbcw_ctrl,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdbcw_page {
    pub st: u32,
    pub st_msk: u32,
    pub clr: u32,
    pub pad: [u8; 4],
    pub msk_st: u32,
    pub msk_set: u32,
    pub msk_clr: u32,
    pub ctrl: xbcw_ctrl,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dummy_page {
    pub pad: [u8; SZ_4K],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhu3_pbx_frame_reg {
    pub ctrl: ctrl_page,
    pub dbcw: [pdbcw_page; MHUV3_DBCW_MAX],
    pub ffcw: dummy_page,
    pub fcw: dummy_page,
    pub 11]: *mut *mut u8 pad[SZ_4K,
    pub impdef: dummy_page,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhu3_mbx_frame_reg {
    pub ctrl: ctrl_page,
    pub dbcw: [mdbcw_page; MHUV3_DBCW_MAX],
    pub ffcw: dummy_page,
    pub fcw: dummy_page,
    pub 11]: *mut *mut u8 pad[SZ_4K,
    pub impdef: dummy_page,
    pub __packed: },
// Macro for reading a bitmask within a physically mapped packed struct

    ({								\
    pub \: unsigned long _rval;,
    pub \: _rval = readl_relaxed(_regptr);,
    pub \: FIELD_GET(_bitmask, _rval);,
    })
// Macro for writing a bitmask within a physically mapped packed struct

    ({								\
    pub \: unsigned long _rval;,
    pub \: typeof(_regptr) _rptr = _regptr;,
    pub \: typeof(_bitmask) _bmask = _bitmask;,
    pub \: _rval = readl_relaxed(_rptr);,
    pub \: _rval &= ~(_bmask);,
    pub _value);\: _rval |= FIELD_PREP((unsigned long long)_bmask,,
    pub \: writel_relaxed(_rval, _rptr);,
    })
// ====== MHUv3 data structures ======
    enum mhuv3_frame {
    PBX_FRAME,
    MBX_FRAME,
}

    static char *mhuv3_str[] = {
    "PBX",
    "MBX"
    };
    enum mhuv3_extension_type {
    DBE_EXT,
    FCE_EXT,
    FE_EXT,
    NUM_EXT
    };
    static char *mhuv3_ext_str[] = {
    "DBE",
    "FCE",
    "FE"
    };
    struct mhuv3;
//
// struct mhuv3_protocol_ops - MHUv3 operations
//
// @rx_startup: Receiver startup callback.
// @rx_shutdown: Receiver shutdown callback.
// @read_data: Read available Sender in-band LE data (if any).
// @rx_complete: Acknowledge data reception to the Sender. Any out-of-band data
// has to have been already retrieved before calling this.
// @tx_startup: Sender startup callback.
// @tx_shutdown: Sender shutdown callback.
// @last_tx_done: Report back to the Sender if the last transfer has completed.
// @send_data: Send data to the receiver.
//
// Each supported transport protocol provides its own implementation of
// these operations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhuv3_protocol_ops {
    pub chan): *mut *mut *mut int (rx_startup)(struct mhuv3 mhu, struct mbox_chan,
    pub chan): *mut *mut *mut void (rx_shutdown)(struct mhuv3 mhu, struct mbox_chan,
    pub chan): *mut *mut *mut *mut void (read_data)(struct mhuv3 mhu, struct mbox_chan,
    pub chan): *mut *mut *mut void (rx_complete)(struct mhuv3 mhu, struct mbox_chan,
    pub chan): *mut *mut *mut void (tx_startup)(struct mhuv3 mhu, struct mbox_chan,
    pub chan): *mut *mut *mut void (tx_shutdown)(struct mhuv3 mhu, struct mbox_chan,
    pub chan): *mut *mut *mut int (last_tx_done)(struct mhuv3 mhu, struct mbox_chan,
    pub arg): *mut *mut *mut *mut int (send_data)(struct mhuv3 mhu, struct mbox_chan chan, void,
}

//
// struct mhuv3_mbox_chan_priv - MHUv3 channel private information
//
// @ch_idx: Channel window index associated to this mailbox channel.
// @doorbell: Doorbell bit number within the @ch_idx window.
// Only relevant to Doorbell transport.
// @ops: Transport protocol specific operations for this channel.
//
// Transport specific data attached to mmailbox channel priv data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhuv3_mbox_chan_priv {
    pub ch_idx: u32,
    pub doorbell: u32,
    pub ops: *const mhuv3_protocol_ops,
}

//
// struct mhuv3_extension - MHUv3 extension descriptor
//
// @type: Type of extension
// @num_chans: Max number of channels found for this extension.
// @base_ch_idx: First channel number assigned to this extension, picked from
// the set of all mailbox channels descriptors created.
// @mbox_of_xlate: Extension specific helper to parse DT and lookup associated
// channel from the related 'mboxes' property.
// @combined_irq_setup: Extension specific helper to setup the combined irq.
// @channels_init: Extension specific helper to initialize channels.
// @chan_from_comb_irq_get: Extension specific helper to lookup which channel
// triggered the combined irq.
// @pending_db: Array of per-channel pending doorbells.
// @pending_lock: Protect access to pending_db.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhuv3_extension {
    pub type: enum mhuv3_extension_type,
    pub num_chans: c_uint,
    pub base_ch_idx: c_uint,
    struct mbox_chan *(*mbox_of_xlate)(struct mhuv3 *mhu,
    unsigned int channel,
    pub param): c_uint,
    pub mhu): *mut *mut void (combined_irq_setup)(struct mhuv3,
    pub mhu): *mut *mut int (channels_init)(struct mhuv3,
    pub mhu): *mut *mut *mut mbox_chan (chan_from_comb_irq_get)(mhuv3,
    pub pending_db: [u32; MHUV3_DBCW_MAX],
// Protect access to pending_db
    pub pending_lock: spinlock_t,
}

//
// struct mhuv3 - MHUv3 mailbox controller data
//
// @frame:	Frame type: MBX_FRAME or PBX_FRAME.
// @auto_op_full: Flag to indicate if the MHU supports AutoOp full mode.
// @major: MHUv3 controller architectural major version.
// @minor: MHUv3 controller architectural minor version.
// @implem: MHUv3 controller IIDR implementer.
// @rev: MHUv3 controller IIDR revision.
// @var: MHUv3 controller IIDR variant.
// @prod_id: MHUv3 controller IIDR product_id.
// @num_chans: The total number of channels discovered across all extensions.
// @cmb_irq: Combined IRQ number if any found defined.
// @ctrl: A reference to the MHUv3 control page for this block.
// @pbx: Base address of the PBX register mapping region.
// @mbx: Base address of the MBX register mapping region.
// @ext: Array holding descriptors for any found implemented extension.
// @mbox: Mailbox controller belonging to the MHU frame.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhuv3 {
    pub frame: enum mhuv3_frame,
    pub auto_op_full: bool,
    pub major: c_uint,
    pub minor: c_uint,
    pub implem: c_uint,
    pub rev: c_uint,
    pub var: c_uint,
    pub prod_id: c_uint,
    pub num_chans: c_uint,
    pub cmb_irq: c_int,
    pub ctrl: *mut ctrl_page __iomem,
    union {
    pub pbx: *mut mhu3_pbx_frame_reg __iomem,
    pub mbx: *mut mhu3_mbx_frame_reg __iomem,
}

    struct mhuv3_extension *ext[NUM_EXT];
    struct mbox_controller mbox;
    };

    typedef int (*mhuv3_extension_initializer)(struct mhuv3 *mhu);
// =================== Doorbell transport protocol operations ===============
#[no_mangle]
unsafe extern "C" fn mhuv3_doorbell_tx_startup(mhu: *mut mhuv3, chan: *mut mbox_chan) {
    static void mhuv3_doorbell_tx_startup(struct mhuv3 *mhu, struct mbox_chan *chan)
    {
    struct mhuv3_mbox_chan_priv *priv = chan.con_priv;
// Enable Transfer Acknowledgment events
    writel_relaxed_bitmask(0x1, &mhu.pbx.dbcw[priv.ch_idx].int_en, tfr_ack);
    }
#[no_mangle]
unsafe extern "C" fn mhuv3_doorbell_tx_shutdown(mhu: *mut mhuv3, chan: *mut mbox_chan) {
    static void mhuv3_doorbell_tx_shutdown(struct mhuv3 *mhu, struct mbox_chan *chan)
    {
    struct mhuv3_mbox_chan_priv *priv = chan.con_priv;
    struct mhuv3_extension *e = mhu.ext[DBE_EXT];
    unsigned long flags;
// Disable Channel Transfer Ack events
    writel_relaxed_bitmask(0x0, &mhu.pbx.dbcw[priv.ch_idx].int_en, tfr_ack);
// Clear Channel Transfer Ack and pending doorbells
    writel_relaxed_bitmask(0x1, &mhu.pbx.dbcw[priv.ch_idx].int_clr, tfr_ack);
    spin_lock_irqsave(&e.pending_lock, flags);
    e.pending_db[priv.ch_idx] = 0;
    spin_unlock_irqrestore(&e.pending_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn mhuv3_doorbell_rx_startup(mhu: *mut mhuv3, chan: *mut mbox_chan) -> c_int {
    static int mhuv3_doorbell_rx_startup(struct mhuv3 *mhu, struct mbox_chan *chan)
    {
    struct mhuv3_mbox_chan_priv *priv = chan.con_priv;
// Unmask Channel Transfer events
    writel_relaxed(BIT(priv.doorbell), &mhu.mbx.dbcw[priv.ch_idx].msk_clr);
    return 0;
    }
    static void mhuv3_doorbell_rx_shutdown(struct mhuv3 *mhu,
    struct mbox_chan *chan)
    {
    struct mhuv3_mbox_chan_priv *priv = chan.con_priv;
// Mask Channel Transfer events
    writel_relaxed(BIT(priv.doorbell), &mhu.mbx.dbcw[priv.ch_idx].msk_set);
    }
#[no_mangle]
unsafe extern "C" fn mhuv3_doorbell_rx_complete(mhu: *mut mhuv3, chan: *mut mbox_chan) {
    static void mhuv3_doorbell_rx_complete(struct mhuv3 *mhu, struct mbox_chan *chan)
    {
    struct mhuv3_mbox_chan_priv *priv = chan.con_priv;
// Clearing the pending transfer generates the Channel Transfer Ack
    writel_relaxed(BIT(priv.doorbell), &mhu.mbx.dbcw[priv.ch_idx].clr);
    }
    static int mhuv3_doorbell_last_tx_done(struct mhuv3 *mhu,
    struct mbox_chan *chan)
    {
    struct mhuv3_mbox_chan_priv *priv = chan.con_priv;
    int done;
    done = !(readl_relaxed(&mhu.pbx.dbcw[priv.ch_idx].st) &
    BIT(priv.doorbell));
    if (done) {
    struct mhuv3_extension *e = mhu.ext[DBE_EXT];
    unsigned long flags;
// Take care to clear the pending doorbell also when polling
    spin_lock_irqsave(&e.pending_lock, flags);
    e.pending_db[priv.ch_idx] &= ~BIT(priv.doorbell);
    spin_unlock_irqrestore(&e.pending_lock, flags);
    }
    return done;
    }
    static int mhuv3_doorbell_send_data(struct mhuv3 *mhu, struct mbox_chan *chan,
    void *arg)
    {
    struct mhuv3_mbox_chan_priv *priv = chan.con_priv;
    struct mhuv3_extension *e = mhu.ext[DBE_EXT];
    scoped_guard(spinlock_irqsave, &e.pending_lock) {
// Only one in-flight Transfer is allowed per-doorbell
    if (e.pending_db[priv.ch_idx] & BIT(priv.doorbell))
    return -EBUSY;
    e.pending_db[priv.ch_idx] |= BIT(priv.doorbell);
    }
    writel_relaxed(BIT(priv.doorbell), &mhu.pbx.dbcw[priv.ch_idx].set);
    return 0;
    }
    static const struct mhuv3_protocol_ops mhuv3_doorbell_ops = {
    .tx_startup = mhuv3_doorbell_tx_startup,
    .tx_shutdown = mhuv3_doorbell_tx_shutdown,
    .rx_startup = mhuv3_doorbell_rx_startup,
    .rx_shutdown = mhuv3_doorbell_rx_shutdown,
    .rx_complete = mhuv3_doorbell_rx_complete,
    .last_tx_done = mhuv3_doorbell_last_tx_done,
    .send_data = mhuv3_doorbell_send_data,
    };
// Sender and receiver mailbox ops
#[no_mangle]
unsafe extern "C" fn mhuv3_sender_last_tx_done(chan: *mut mbox_chan) -> bool {
    static bool mhuv3_sender_last_tx_done(struct mbox_chan *chan)
    {
    struct mhuv3_mbox_chan_priv *priv = chan.con_priv;
    struct mhuv3 *mhu = mhu_from_mbox(chan.mbox);
    return priv.ops.last_tx_done(mhu, chan);
    }
#[no_mangle]
unsafe extern "C" fn mhuv3_sender_send_data(chan: *mut mbox_chan, data: *mut c_void) -> c_int {
    static int mhuv3_sender_send_data(struct mbox_chan *chan, void *data)
    {
    struct mhuv3_mbox_chan_priv *priv = chan.con_priv;
    struct mhuv3 *mhu = mhu_from_mbox(chan.mbox);
    if (!priv.ops.last_tx_done(mhu, chan))
    return -EBUSY;
    return priv.ops.send_data(mhu, chan, data);
    }
#[no_mangle]
unsafe extern "C" fn mhuv3_sender_startup(chan: *mut mbox_chan) -> c_int {
    static int mhuv3_sender_startup(struct mbox_chan *chan)
    {
    struct mhuv3_mbox_chan_priv *priv = chan.con_priv;
    struct mhuv3 *mhu = mhu_from_mbox(chan.mbox);
    if (priv.ops.tx_startup)
    priv.ops.tx_startup(mhu, chan);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mhuv3_sender_shutdown(chan: *mut mbox_chan) {
    static void mhuv3_sender_shutdown(struct mbox_chan *chan)
    {
    struct mhuv3_mbox_chan_priv *priv = chan.con_priv;
    struct mhuv3 *mhu = mhu_from_mbox(chan.mbox);
    if (priv.ops.tx_shutdown)
    priv.ops.tx_shutdown(mhu, chan);
    }
    static const struct mbox_chan_ops mhuv3_sender_ops = {
    .send_data = mhuv3_sender_send_data,
    .startup = mhuv3_sender_startup,
    .shutdown = mhuv3_sender_shutdown,
    .last_tx_done = mhuv3_sender_last_tx_done,
    };
#[no_mangle]
unsafe extern "C" fn mhuv3_receiver_startup(chan: *mut mbox_chan) -> c_int {
    static int mhuv3_receiver_startup(struct mbox_chan *chan)
    {
    struct mhuv3_mbox_chan_priv *priv = chan.con_priv;
    struct mhuv3 *mhu = mhu_from_mbox(chan.mbox);
    return priv.ops.rx_startup(mhu, chan);
    }
#[no_mangle]
unsafe extern "C" fn mhuv3_receiver_shutdown(chan: *mut mbox_chan) {
    static void mhuv3_receiver_shutdown(struct mbox_chan *chan)
    {
    struct mhuv3_mbox_chan_priv *priv = chan.con_priv;
    struct mhuv3 *mhu = mhu_from_mbox(chan.mbox);
    priv.ops.rx_shutdown(mhu, chan);
    }
#[no_mangle]
unsafe extern "C" fn mhuv3_receiver_send_data(chan: *mut mbox_chan, data: *mut c_void) -> c_int {
    static int mhuv3_receiver_send_data(struct mbox_chan *chan, void *data)
    {
    dev_err(chan.mbox.dev,
    "Trying to transmit on a MBX MHUv3 frame\n");
    return -EIO;
    }
#[no_mangle]
unsafe extern "C" fn mhuv3_receiver_last_tx_done(chan: *mut mbox_chan) -> bool {
    static bool mhuv3_receiver_last_tx_done(struct mbox_chan *chan)
    {
    dev_err(chan.mbox.dev, "Trying to Tx poll on a MBX MHUv3 frame\n");
    return true;
    }
    static const struct mbox_chan_ops mhuv3_receiver_ops = {
    .send_data = mhuv3_receiver_send_data,
    .startup = mhuv3_receiver_startup,
    .shutdown = mhuv3_receiver_shutdown,
    .last_tx_done = mhuv3_receiver_last_tx_done,
    };
    static struct mbox_chan *mhuv3_dbe_mbox_of_xlate(struct mhuv3 *mhu,
    unsigned int channel,
    unsigned int doorbell)
    {
    struct mhuv3_extension *e = mhu.ext[DBE_EXT];
    struct mbox_controller *mbox = &mhu.mbox;
    struct mbox_chan *chans = mbox.chans;
    if (channel >= e.num_chans || doorbell >= MHUV3_FLAG_BITS) {
    dev_err(mbox.dev, "Couldn't xlate to a valid channel (%d: %d)\n",
    channel, doorbell);
    return ERR_PTR(-ENODEV);
    }
    return &chans[e.base_ch_idx + channel * MHUV3_FLAG_BITS + doorbell];
    }
#[no_mangle]
unsafe extern "C" fn mhuv3_dbe_combined_irq_setup(mhu: *mut mhuv3) {
    static void mhuv3_dbe_combined_irq_setup(struct mhuv3 *mhu)
    {
    struct mhuv3_extension *e = mhu.ext[DBE_EXT];
    int i;
    if (mhu.frame == PBX_FRAME) {
    struct pdbcw_page __iomem *dbcw = mhu.pbx.dbcw;
    for (i = 0; i < e.num_chans; i++) {
    writel_relaxed_bitmask(0x1, &dbcw[i].int_clr, tfr_ack);
    writel_relaxed_bitmask(0x0, &dbcw[i].int_en, tfr_ack);
    writel_relaxed_bitmask(0x1, &dbcw[i].ctrl, comb_en);
    }
    } else {
    struct mdbcw_page __iomem *dbcw = mhu.mbx.dbcw;
    for (i = 0; i < e.num_chans; i++) {
    writel_relaxed(0xFFFFFFFF, &dbcw[i].clr);
    writel_relaxed(0xFFFFFFFF, &dbcw[i].msk_set);
    writel_relaxed_bitmask(0x1, &dbcw[i].ctrl, comb_en);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn mhuv3_dbe_channels_init(mhu: *mut mhuv3) -> c_int {
    static int mhuv3_dbe_channels_init(struct mhuv3 *mhu)
    {
    struct mhuv3_extension *e = mhu.ext[DBE_EXT];
    struct mbox_controller *mbox = &mhu.mbox;
    struct mbox_chan *chans;
    int i;
    chans = mbox.chans + mbox.num_chans;
    e.base_ch_idx = mbox.num_chans;
    for (i = 0; i < e.num_chans; i++) {
    struct mhuv3_mbox_chan_priv *priv;
    int k;
    for (k = 0; k < MHUV3_FLAG_BITS; k++) {
    priv = devm_kmalloc(mbox.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.ch_idx = i;
    priv.ops = &mhuv3_doorbell_ops;
    priv.doorbell = k;
    chans++.con_priv = priv;
    mbox.num_chans++;
    }
    }
    spin_lock_init(&e.pending_lock);
    return 0;
    }
    static bool mhuv3_dbe_doorbell_lookup(struct mhuv3 *mhu, unsigned int channel,
    unsigned int *db)
    {
    struct mhuv3_extension *e = mhu.ext[DBE_EXT];
    struct device *dev = mhu.mbox.dev;
    u32 st;
    if (mhu.frame == PBX_FRAME) {
    u32 active_dbs, fired_dbs;
    st = readl_relaxed_bitmask(&mhu.pbx.dbcw[channel].int_st,
    tfr_ack);
    if (!st)
    goto err_spurious;
    active_dbs = readl_relaxed(&mhu.pbx.dbcw[channel].st);
    scoped_guard(spinlock_irqsave, &e.pending_lock) {
    fired_dbs = e.pending_db[channel] & ~active_dbs;
    if (!fired_dbs)
    goto err_spurious;
// db = __ffs(fired_dbs);
    e.pending_db[channel] &= ~BIT(*db);
    }
    fired_dbs &= ~BIT(*db);
// Clear TFR Ack if no more doorbells pending
    if (!fired_dbs)
    writel_relaxed_bitmask(0x1,
    &mhu.pbx.dbcw[channel].int_clr,
    tfr_ack);
    } else {
    st = readl_relaxed(&mhu.mbx.dbcw[channel].st_msk);
    if (!st)
    goto err_spurious;
// db = __ffs(st);
    }
    return true;
    err_spurious:
    dev_warn(dev, "Spurious IRQ on %s channel:%d\n",
    mhuv3_str[mhu.frame], channel);
    return false;
    }
    static struct mbox_chan *mhuv3_dbe_chan_from_comb_irq_get(struct mhuv3 *mhu)
    {
    struct mhuv3_extension *e = mhu.ext[DBE_EXT];
    struct device *dev = mhu.mbox.dev;
    int i;
    for (i = 0; i < MHUV3_DBCH_CMB_INT_ST_REG_CNT; i++) {
    unsigned int channel, db;
    u32 cmb_st;
    cmb_st = readl_relaxed(&mhu.ctrl.dbch_int_st[i]);
    if (!cmb_st)
    continue;
    channel = i * MHUV3_FLAG_BITS + __ffs(cmb_st);
    if (channel >= e.num_chans) {
    dev_err(dev, "Invalid %s channel:%d\n",
    mhuv3_str[mhu.frame], channel);
    return ERR_PTR(-EIO);
    }
    if (!mhuv3_dbe_doorbell_lookup(mhu, channel, &db))
    continue;
    dev_dbg(dev, "Found %s ch[%d]/db[%d]\n",
    mhuv3_str[mhu.frame], channel, db);
    return &mhu.mbox.chans[channel * MHUV3_FLAG_BITS + db];
    }
    return ERR_PTR(-EIO);
    }
#[no_mangle]
unsafe extern "C" fn mhuv3_dbe_init(mhu: *mut mhuv3) -> c_int {
    static int mhuv3_dbe_init(struct mhuv3 *mhu)
    {
    struct device *dev = mhu.mbox.dev;
    struct mhuv3_extension *e;
    if (!readl_relaxed_bitmask(&mhu.ctrl.feat_spt0, dbe_spt))
    return 0;
    dev_dbg(dev, "%s: Initializing DBE Extension.\n", mhuv3_str[mhu.frame]);
    e = devm_kzalloc(dev, sizeof(*e), GFP_KERNEL);
    if (!e)
    return -ENOMEM;
    e.type = DBE_EXT;
// Note that, by the spec, the number of channels is (num_dbch + 1)
    e.num_chans =
    readl_relaxed_bitmask(&mhu.ctrl.dbch_cfg0, num_dbch) + 1;
    e.mbox_of_xlate = mhuv3_dbe_mbox_of_xlate;
    e.combined_irq_setup = mhuv3_dbe_combined_irq_setup;
    e.channels_init = mhuv3_dbe_channels_init;
    e.chan_from_comb_irq_get = mhuv3_dbe_chan_from_comb_irq_get;
    mhu.num_chans += e.num_chans * MHUV3_FLAG_BITS;
    mhu.ext[DBE_EXT] = e;
    dev_dbg(dev, "%s: found %d DBE channels.\n",
    mhuv3_str[mhu.frame], e.num_chans);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mhuv3_fce_init(mhu: *mut mhuv3) -> c_int {
    static int mhuv3_fce_init(struct mhuv3 *mhu)
    {
    struct device *dev = mhu.mbox.dev;
    if (!readl_relaxed_bitmask(&mhu.ctrl.feat_spt0, fce_spt))
    return 0;
    dev_dbg(dev, "%s: FCE Extension not supported by driver.\n",
    mhuv3_str[mhu.frame]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mhuv3_fe_init(mhu: *mut mhuv3) -> c_int {
    static int mhuv3_fe_init(struct mhuv3 *mhu)
    {
    struct device *dev = mhu.mbox.dev;
    if (!readl_relaxed_bitmask(&mhu.ctrl.feat_spt0, fe_spt))
    return 0;
    dev_dbg(dev, "%s: FE Extension not supported by driver.\n",
    mhuv3_str[mhu.frame]);
    return 0;
    }
    static mhuv3_extension_initializer mhuv3_extension_init[NUM_EXT] = {
    mhuv3_dbe_init,
    mhuv3_fce_init,
    mhuv3_fe_init,
    };
#[no_mangle]
unsafe extern "C" fn mhuv3_initialize_channels(dev: *mut device, mhu: *mut mhuv3) -> c_int {
    static int mhuv3_initialize_channels(struct device *dev, struct mhuv3 *mhu)
    {
    struct mbox_controller *mbox = &mhu.mbox;
    int i, ret = 0;
    mbox.chans = devm_kcalloc(dev, mhu.num_chans,
    sizeof(*mbox.chans), GFP_KERNEL);
    if (!mbox.chans)
    return dev_err_probe(dev, -ENOMEM,
    "Failed to initialize channels\n");
    for (i = 0; i < NUM_EXT && !ret; i++)
    if (mhu.ext[i])
    ret = mhu.ext[i].channels_init(mhu);
    return ret;
    }
    static struct mbox_chan *mhuv3_mbox_of_xlate(struct mbox_controller *mbox,
    const struct of_phandle_args *pa)
    {
    struct mhuv3 *mhu = mhu_from_mbox(mbox);
    unsigned int type, channel, param;
    if (pa.args_count != MHUV3_MBOX_CELLS)
    return ERR_PTR(-EINVAL);
    type = pa.args[MHUV3_MBOX_CELL_TYPE];
    if (type >= NUM_EXT)
    return ERR_PTR(-EINVAL);
    channel = pa.args[MHUV3_MBOX_CELL_CHWN];
    param = pa.args[MHUV3_MBOX_CELL_PARAM];
    return mhu.ext[type].mbox_of_xlate(mhu, channel, param);
    }
#[no_mangle]
unsafe extern "C" fn mhu_frame_cleanup_actions(data: *mut c_void) {
    static void mhu_frame_cleanup_actions(void *data)
    {
    struct mhuv3 *mhu = data;
    writel_relaxed_bitmask(0x0, &mhu.ctrl.x_ctrl, op_req);
    }
#[no_mangle]
unsafe extern "C" fn mhuv3_frame_init(mhu: *mut mhuv3, regs: *mut void __iomem) -> c_int {
    static int mhuv3_frame_init(struct mhuv3 *mhu, void __iomem *regs)
    {
    struct device *dev = mhu.mbox.dev;
    int i;
    mhu.ctrl = regs;
    mhu.frame = readl_relaxed_bitmask(&mhu.ctrl.blk_id, id);
    if (mhu.frame > MBX_FRAME)
    return dev_err_probe(dev, -EINVAL,
    "Invalid Frame type- %d\n", mhu.frame);
    mhu.major = readl_relaxed_bitmask(&mhu.ctrl.aidr, arch_major_rev);
    mhu.minor = readl_relaxed_bitmask(&mhu.ctrl.aidr, arch_minor_rev);
    mhu.implem = readl_relaxed_bitmask(&mhu.ctrl.iidr, implementer);
    mhu.rev = readl_relaxed_bitmask(&mhu.ctrl.iidr, revision);
    mhu.var = readl_relaxed_bitmask(&mhu.ctrl.iidr, variant);
    mhu.prod_id = readl_relaxed_bitmask(&mhu.ctrl.iidr, product_id);
    if (mhu.major != MHUV3_MAJOR_VERSION)
    return dev_err_probe(dev, -EINVAL,
    "Unsupported MHU %s block - major:%d  minor:%d\n",
    mhuv3_str[mhu.frame], mhu.major,
    mhu.minor);
    mhu.auto_op_full =
    !!readl_relaxed_bitmask(&mhu.ctrl.feat_spt1, auto_op_spt);
// Request the PBX/MBX to remain operational
    if (mhu.auto_op_full) {
    writel_relaxed_bitmask(0x1, &mhu.ctrl.x_ctrl, op_req);
    devm_add_action_or_reset(dev, mhu_frame_cleanup_actions, mhu);
    }
    dev_dbg(dev,
    "Found MHU %s block - major:%d  minor:%d\n  implem:0x%X  rev:0x%X  var:0x%X  prod_id:0x%X",
    mhuv3_str[mhu.frame], mhu.major, mhu.minor,
    mhu.implem, mhu.rev, mhu.var, mhu.prod_id);
    if (mhu.frame == PBX_FRAME)
    mhu.pbx = regs;
    else
    mhu.mbx = regs;
    for (i = 0; i < NUM_EXT; i++) {
    int ret;
//
// Note that extensions initialization fails only when such
// extension initialization routine fails and the extensions
// was found to be supported in hardware and in software.
//
    ret = mhuv3_extension_init[i](mhu);
    if (ret)
    return dev_err_probe(dev, ret,
    "Failed to initialize %s %s\n",
    mhuv3_str[mhu.frame],
    mhuv3_ext_str[i]);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mhuv3_pbx_comb_interrupt(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t mhuv3_pbx_comb_interrupt(int irq, void *arg)
    {
    unsigned int i, found = 0;
    struct mhuv3 *mhu = arg;
    struct mbox_chan *chan;
    struct device *dev;
    let mut ret: c_int = IRQ_NONE;
    dev = mhu.mbox.dev;
    for (i = 0; i < NUM_EXT; i++) {
    struct mhuv3_mbox_chan_priv *priv;
// FCE does not participate to the PBX combined
    if (i == FCE_EXT || !mhu.ext[i])
    continue;
    chan = mhu.ext[i].chan_from_comb_irq_get(mhu);
    if (IS_ERR(chan))
    continue;
    found++;
    priv = chan.con_priv;
    if (!chan.cl) {
    dev_warn(dev, "TX Ack on UNBOUND channel (%u)\n",
    priv.ch_idx);
    continue;
    }
    mbox_chan_txdone(chan, 0);
    ret = IRQ_HANDLED;
    }
    if (found == 0)
    dev_warn_once(dev, "Failed to find channel for the TX interrupt\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mhuv3_mbx_comb_interrupt(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t mhuv3_mbx_comb_interrupt(int irq, void *arg)
    {
    unsigned int i, found = 0;
    struct mhuv3 *mhu = arg;
    struct mbox_chan *chan;
    struct device *dev;
    let mut ret: c_int = IRQ_NONE;
    dev = mhu.mbox.dev;
    for (i = 0; i < NUM_EXT; i++) {
    struct mhuv3_mbox_chan_priv *priv;
    void *data __free(kfree) = core::ptr::null_mut();
    if (!mhu.ext[i])
    continue;
// Process any extension which could be source of the IRQ
    chan = mhu.ext[i].chan_from_comb_irq_get(mhu);
    if (IS_ERR(chan))
    continue;
    found++;
// From here on we need to call rx_complete even on error
    priv = chan.con_priv;
    if (!chan.cl) {
    dev_warn(dev, "RX Data on UNBOUND channel (%u)\n",
    priv.ch_idx);
    goto rx_ack;
    }
// Read optional in-band LE data first.
    if (priv.ops.read_data) {
    data = priv.ops.read_data(mhu, chan);
    if (IS_ERR(data)) {
    dev_err(dev,
    "Failed to read in-band data. err:%ld\n",
    PTR_ERR(data));
    goto rx_ack;
    }
    }
    mbox_chan_received_data(chan, data);
    ret = IRQ_HANDLED;
//
// Acknowledge transfer after any possible optional
// out-of-band data has also been retrieved via
// mbox_chan_received_data().
//
    rx_ack:
    if (priv.ops.rx_complete)
    priv.ops.rx_complete(mhu, chan);
    }
    if (found == 0)
    dev_warn_once(dev, "Failed to find channel for the RX interrupt\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mhuv3_setup_pbx(mhu: *mut mhuv3) -> c_int {
    static int mhuv3_setup_pbx(struct mhuv3 *mhu)
    {
    struct device *dev = mhu.mbox.dev;
    mhu.mbox.ops = &mhuv3_sender_ops;
    if (mhu.cmb_irq > 0) {
    int ret, i;
    ret = devm_request_threaded_irq(dev, mhu.cmb_irq, core::ptr::null_mut(),
    mhuv3_pbx_comb_interrupt,
    IRQF_ONESHOT, "mhuv3-pbx", mhu);
    if (ret)
    return ret;
    mhu.mbox.txdone_irq = true;
    mhu.mbox.txdone_poll = false;
    for (i = 0; i < NUM_EXT; i++)
    if (mhu.ext[i])
    mhu.ext[i].combined_irq_setup(mhu);
    dev_dbg(dev, "MHUv3 PBX IRQs initialized.\n");
    return 0;
    }
    dev_info(dev, "Using PBX in Tx polling mode.\n");
    mhu.mbox.txdone_irq = false;
    mhu.mbox.txdone_poll = true;
    mhu.mbox.txpoll_period = 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mhuv3_setup_mbx(mhu: *mut mhuv3) -> c_int {
    static int mhuv3_setup_mbx(struct mhuv3 *mhu)
    {
    struct device *dev = mhu.mbox.dev;
    int ret, i;
    mhu.mbox.ops = &mhuv3_receiver_ops;
    if (mhu.cmb_irq <= 0)
    return dev_err_probe(dev, -EINVAL,
    "MBX combined IRQ is missing !\n");
    ret = devm_request_threaded_irq(dev, mhu.cmb_irq, core::ptr::null_mut(),
    mhuv3_mbx_comb_interrupt, IRQF_ONESHOT,
    "mhuv3-mbx", mhu);
    if (ret)
    return ret;
    for (i = 0; i < NUM_EXT; i++)
    if (mhu.ext[i])
    mhu.ext[i].combined_irq_setup(mhu);
    dev_dbg(dev, "MHUv3 MBX IRQs initialized.\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mhuv3_irqs_init(mhu: *mut mhuv3, pdev: *mut platform_device) -> c_int {
    static int mhuv3_irqs_init(struct mhuv3 *mhu, struct platform_device *pdev)
    {
    dev_dbg(mhu.mbox.dev, "Initializing %s block.\n",
    mhuv3_str[mhu.frame]);
    if (mhu.frame == PBX_FRAME) {
    mhu.cmb_irq =
    platform_get_irq_byname_optional(pdev, "combined");
    return mhuv3_setup_pbx(mhu);
    }
    mhu.cmb_irq = platform_get_irq_byname(pdev, "combined");
    return mhuv3_setup_mbx(mhu);
    }
#[no_mangle]
unsafe extern "C" fn mhuv3_probe(pdev: *mut platform_device) -> c_int {
    static int mhuv3_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    void __iomem *regs;
    struct mhuv3 *mhu;
    int ret;
    mhu = devm_kzalloc(dev, sizeof(*mhu), GFP_KERNEL);
    if (!mhu)
    return -ENOMEM;
    regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(regs))
    return PTR_ERR(regs);
    mhu.mbox.dev = dev;
    ret = mhuv3_frame_init(mhu, regs);
    if (ret)
    return ret;
    ret = mhuv3_irqs_init(mhu, pdev);
    if (ret)
    return ret;
    mhu.mbox.of_xlate = mhuv3_mbox_of_xlate;
    ret = mhuv3_initialize_channels(dev, mhu);
    if (ret)
    return ret;
    ret = devm_mbox_controller_register(dev, &mhu.mbox);
    if (ret)
    return dev_err_probe(dev, ret,
    "Failed to register ARM MHUv3 driver\n");
    return ret;
    }
    static const struct of_device_id mhuv3_of_match[] = {
    { .compatible = "arm,mhuv3", .data = core::ptr::null_mut() },
    {}
    };
    MODULE_DEVICE_TABLE(of, mhuv3_of_match);
    static struct platform_driver mhuv3_driver = {
    .driver = {
    .name = "arm-mhuv3-mailbox",
    .of_match_table = mhuv3_of_match,
    },
    .probe = mhuv3_probe,
    };
    module_platform_driver(mhuv3_driver);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("ARM MHUv3 Driver");
    MODULE_AUTHOR("Cristian Marussi <cristian.marussi@arm.com>");
