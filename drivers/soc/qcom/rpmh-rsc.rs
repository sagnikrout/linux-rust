//! Automatically rewritten from C to Rust
//! Source: drivers/soc/qcom/rpmh-rsc.c
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
// Copyright (c) 2016-2018, The Linux Foundation. All rights reserved.
// Copyright (c) 2023-2024, Qualcomm Innovation Center, Inc. All rights reserved.
//

// Macro flag: #define CREATE_TRACE_POINTS

pub const RSC_DRV_ID: c_int = 0;
pub const MAJOR_VER_MASK: c_uint = 0xFF;
pub const MAJOR_VER_SHIFT: c_int = 16;
pub const MINOR_VER_MASK: c_uint = 0xFF;
pub const MINOR_VER_SHIFT: c_int = 8;
    enum {
    RSC_DRV_TCS_OFFSET,
    RSC_DRV_CMD_OFFSET,
    DRV_SOLVER_CONFIG,
    DRV_PRNT_CHLD_CONFIG,
    RSC_DRV_IRQ_ENABLE,
    RSC_DRV_IRQ_STATUS,
    RSC_DRV_IRQ_CLEAR,
    RSC_DRV_CMD_WAIT_FOR_CMPL,
    RSC_DRV_CONTROL,
    RSC_DRV_STATUS,
    RSC_DRV_CMD_ENABLE,
    RSC_DRV_CMD_MSGID,
    RSC_DRV_CMD_ADDR,
    RSC_DRV_CMD_DATA,
    RSC_DRV_CMD_STATUS,
    RSC_DRV_CMD_RESP_DATA,
    };
// DRV HW Solver Configuration Information Register
pub const DRV_HW_SOLVER_MASK: c_int = 1;
pub const DRV_HW_SOLVER_SHIFT: c_int = 24;
// DRV TCS Configuration Information Register
pub const DRV_NUM_TCS_MASK: c_uint = 0x3F;
pub const DRV_NUM_TCS_SHIFT: c_int = 6;
pub const DRV_NCPT_MASK: c_uint = 0x1F;
pub const DRV_NCPT_SHIFT: c_int = 27;
// Offsets for CONTROL TCS Registers
pub const RSC_DRV_CTL_TCS_DATA_HI: c_uint = 0x38;
pub const RSC_DRV_CTL_TCS_DATA_HI_MASK: c_uint = 0xFFFFFF;

pub const RSC_DRV_CTL_TCS_DATA_LO: c_uint = 0x40;
pub const RSC_DRV_CTL_TCS_DATA_LO_MASK: c_uint = 0xFFFFFFFF;
pub const RSC_DRV_CTL_TCS_DATA_SIZE: c_int = 32;

// TCS CMD register bit mask
pub const CMD_MSGID_LEN: c_int = 8;

//
// Here's a high level overview of how all the registers in RPMH work
// together:
//
// - The main rpmh-rsc address is the base of a register space that can
// be used to find overall configuration of the hardware
// (DRV_PRNT_CHLD_CONFIG). Also found within the rpmh-rsc register
// space are all the TCS blocks. The offset of the TCS blocks is
// specified in the device tree by "qcom,tcs-offset" and used to
// compute tcs_base.
// - TCS blocks come one after another. Type, count, and order are
// specified by the device tree as "qcom,tcs-config".
// - Each TCS block has some registers, then space for up to 16 commands.
// Note that though address space is reserved for 16 commands, fewer
// might be present. See ncpt (num cmds per TCS).
//
// Here's a picture:
//
// +---------------------------------------------------+
// |RSC                                                |
// | ctrl                                              |
// |                                                   |
// | Drvs:                                             |
// | +-----------------------------------------------+ |
// | |DRV0                                           | |
// | | ctrl/config                                   | |
// | | IRQ                                           | |
// | |                                               | |
// | | TCSes:                                        | |
// | | +------------------------------------------+  | |
// | | |TCS0  |  |  |  |  |  |  |  |  |  |  |  |  |  | |
// | | | ctrl | 0| 1| 2| 3| 4| 5| .| .| .| .|14|15|  | |
// | | |      |  |  |  |  |  |  |  |  |  |  |  |  |  | |
// | | +------------------------------------------+  | |
// | | |TCS1  |  |  |  |  |  |  |  |  |  |  |  |  |  | |
// | | | ctrl | 0| 1| 2| 3| 4| 5| .| .| .| .|14|15|  | |
// | | |      |  |  |  |  |  |  |  |  |  |  |  |  |  | |
// | | +------------------------------------------+  | |
// | | |TCS2  |  |  |  |  |  |  |  |  |  |  |  |  |  | |
// | | | ctrl | 0| 1| 2| 3| 4| 5| .| .| .| .|14|15|  | |
// | | |      |  |  |  |  |  |  |  |  |  |  |  |  |  | |
// | | +------------------------------------------+  | |
// | |                    ......                     | |
// | +-----------------------------------------------+ |
// | |DRV1                                           | |
// | | (same as DRV0)                                | |
// | +-----------------------------------------------+ |
// |                      ......                       |
// +---------------------------------------------------+
//

    xloops_to_cycles((time_usecs) * 0x10C7UL)
#[no_mangle]
pub unsafe extern "C" fn xloops_to_cycles(xloops: u64) -> c_ulong {
    static inline unsigned long xloops_to_cycles(u64 xloops)
    {
    return (xloops * loops_per_jiffy * HZ) >> 32;
    }
    static u32 rpmh_rsc_reg_offset_ver_2_7[] = {
    [RSC_DRV_TCS_OFFSET]		= 672,
    [RSC_DRV_CMD_OFFSET]		= 20,
    [DRV_SOLVER_CONFIG]		= 0x04,
    [DRV_PRNT_CHLD_CONFIG]		= 0x0C,
    [RSC_DRV_IRQ_ENABLE]		= 0x00,
    [RSC_DRV_IRQ_STATUS]		= 0x04,
    [RSC_DRV_IRQ_CLEAR]		= 0x08,
    [RSC_DRV_CMD_WAIT_FOR_CMPL]	= 0x10,
    [RSC_DRV_CONTROL]		= 0x14,
    [RSC_DRV_STATUS]		= 0x18,
    [RSC_DRV_CMD_ENABLE]		= 0x1C,
    [RSC_DRV_CMD_MSGID]		= 0x30,
    [RSC_DRV_CMD_ADDR]		= 0x34,
    [RSC_DRV_CMD_DATA]		= 0x38,
    [RSC_DRV_CMD_STATUS]		= 0x3C,
    [RSC_DRV_CMD_RESP_DATA]		= 0x40,
    };
    static u32 rpmh_rsc_reg_offset_ver_3_0[] = {
    [RSC_DRV_TCS_OFFSET]		= 672,
    [RSC_DRV_CMD_OFFSET]		= 24,
    [DRV_SOLVER_CONFIG]		= 0x04,
    [DRV_PRNT_CHLD_CONFIG]		= 0x0C,
    [RSC_DRV_IRQ_ENABLE]		= 0x00,
    [RSC_DRV_IRQ_STATUS]		= 0x04,
    [RSC_DRV_IRQ_CLEAR]		= 0x08,
    [RSC_DRV_CMD_WAIT_FOR_CMPL]	= 0x20,
    [RSC_DRV_CONTROL]		= 0x24,
    [RSC_DRV_STATUS]		= 0x28,
    [RSC_DRV_CMD_ENABLE]		= 0x2C,
    [RSC_DRV_CMD_MSGID]		= 0x34,
    [RSC_DRV_CMD_ADDR]		= 0x38,
    [RSC_DRV_CMD_DATA]		= 0x3C,
    [RSC_DRV_CMD_STATUS]		= 0x40,
    [RSC_DRV_CMD_RESP_DATA]		= 0x44,
    };
    static inline void __iomem *
    tcs_reg_addr(const struct rsc_drv *drv, int reg, int tcs_id)
    {
    return drv.tcs_base + drv.regs[RSC_DRV_TCS_OFFSET] * tcs_id + reg;
    }
    static inline void __iomem *
    tcs_cmd_addr(const struct rsc_drv *drv, int reg, int tcs_id, int cmd_id)
    {
    return tcs_reg_addr(drv, reg, tcs_id) + drv.regs[RSC_DRV_CMD_OFFSET] * cmd_id;
    }
    static u32 read_tcs_cmd(const struct rsc_drv *drv, int reg, int tcs_id,
    int cmd_id)
    {
    return readl_relaxed(tcs_cmd_addr(drv, reg, tcs_id, cmd_id));
    }
#[no_mangle]
unsafe extern "C" fn read_tcs_reg(drv: *const rsc_drv, reg: c_int, tcs_id: c_int) -> u32 {
    static u32 read_tcs_reg(const struct rsc_drv *drv, int reg, int tcs_id)
    {
    return readl_relaxed(tcs_reg_addr(drv, reg, tcs_id));
    }
    static void write_tcs_cmd(const struct rsc_drv *drv, int reg, int tcs_id,
    int cmd_id, u32 data)
    {
    writel_relaxed(data, tcs_cmd_addr(drv, reg, tcs_id, cmd_id));
    }
    static void write_tcs_reg(const struct rsc_drv *drv, int reg, int tcs_id,
    u32 data)
    {
    writel_relaxed(data, tcs_reg_addr(drv, reg, tcs_id));
    }
    static void write_tcs_reg_sync(const struct rsc_drv *drv, int reg, int tcs_id,
    u32 data)
    {
    int i;
    writel(data, tcs_reg_addr(drv, reg, tcs_id));
//
// Wait until we read back the same value.  Use a counter rather than
// ktime for timeout since this may be called after timekeeping stops.
//
    for (i = 0; i < USEC_PER_SEC; i++) {
    if (readl(tcs_reg_addr(drv, reg, tcs_id)) == data)
    return;
    udelay(1);
    }
    pr_err("%s: error writing %#x to %d:%#x\n", drv.name,
    data, tcs_id, reg);
    }
//
// tcs_invalidate() - Invalidate all TCSes of the given type (sleep or wake).
// @drv:  The RSC controller.
// @type: SLEEP_TCS or WAKE_TCS
//
// This will clear the "slots" variable of the given tcs_group and also
// tell the hardware to forget about all entries.
//
// The caller must ensure that no other RPMH actions are happening when this
// function is called, since otherwise the device may immediately become
// used again even before this function exits.
//
#[no_mangle]
unsafe extern "C" fn tcs_invalidate(drv: *mut rsc_drv, type: c_int) {
    static void tcs_invalidate(struct rsc_drv *drv, int type)
    {
    int m;
    struct tcs_group *tcs = &drv.tcs[type];
// Caller ensures nobody else is running so no lock
    if (bitmap_empty(tcs.slots, MAX_TCS_SLOTS))
    return;
    for (m = tcs.offset; m < tcs.offset + tcs.num_tcs; m++)
    write_tcs_reg_sync(drv, drv.regs[RSC_DRV_CMD_ENABLE], m, 0);
    bitmap_zero(tcs.slots, MAX_TCS_SLOTS);
    }
//
// rpmh_rsc_invalidate() - Invalidate sleep and wake TCSes.
// @drv: The RSC controller.
//
// The caller must ensure that no other RPMH actions are happening when this
// function is called, since otherwise the device may immediately become
// used again even before this function exits.
//
#[no_mangle]
pub unsafe extern "C" fn rpmh_rsc_invalidate(drv: *mut rsc_drv) {
    void rpmh_rsc_invalidate(struct rsc_drv *drv)
    {
    tcs_invalidate(drv, SLEEP_TCS);
    tcs_invalidate(drv, WAKE_TCS);
    }
//
// get_tcs_for_msg() - Get the tcs_group used to send the given message.
// @drv: The RSC controller.
// @msg: The message we want to send.
//
// This is normally pretty straightforward except if we are trying to send
// an ACTIVE_ONLY message but don't have any active_only TCSes.
//
// Return: A pointer to a tcs_group or an ERR_PTR.
//
    static struct tcs_group *get_tcs_for_msg(struct rsc_drv *drv,
    const struct tcs_request *msg)
    {
    int type;
    struct tcs_group *tcs;
    switch (msg.state) {
    case RPMH_ACTIVE_ONLY_STATE:
    type = ACTIVE_TCS;
    break;
    case RPMH_WAKE_ONLY_STATE:
    type = WAKE_TCS;
    break;
    case RPMH_SLEEP_STATE:
    type = SLEEP_TCS;
    break;
    default:
    return ERR_PTR(-EINVAL);
    }
//
// If we are making an active request on a RSC that does not have a
// dedicated TCS for active state use, then re-purpose a wake TCS to
// send active votes. This is safe because we ensure any active-only
// transfers have finished before we use it (maybe by running from
// the last CPU in PM code).
//
    tcs = &drv.tcs[type];
    if (msg.state == RPMH_ACTIVE_ONLY_STATE && !tcs.num_tcs)
    tcs = &drv.tcs[WAKE_TCS];
    return tcs;
    }
//
// get_req_from_tcs() - Get a stashed request that was xfering on the given TCS.
// @drv:    The RSC controller.
// @tcs_id: The global ID of this TCS.
//
// For ACTIVE_ONLY transfers we want to call back into the client when the
// transfer finishes. To do this we need the "request" that the client
// originally provided us. This function grabs the request that we stashed
// when we started the transfer.
//
// This only makes sense for ACTIVE_ONLY transfers since those are the only
// ones we track sending (the only ones we enable interrupts for and the only
// ones we call back to the client for).
//
// Return: The stashed request.
//
    static const struct tcs_request *get_req_from_tcs(struct rsc_drv *drv,
    int tcs_id)
    {
    struct tcs_group *tcs;
    int i;
    for (i = 0; i < TCS_TYPE_NR; i++) {
    tcs = &drv.tcs[i];
    if (tcs.mask & BIT(tcs_id))
    return tcs.req[tcs_id - tcs.offset];
    }
    return core::ptr::null_mut();
    }
//
// __tcs_set_trigger() - Start xfer on a TCS or unset trigger on a borrowed TCS
// @drv:     The controller.
// @tcs_id:  The global ID of this TCS.
// @trigger: If true then untrigger/retrigger. If false then just untrigger.
//
// In the normal case we only ever call with "trigger=true" to start a
// transfer. That will un-trigger/disable the TCS from the last transfer
// then trigger/enable for this transfer.
//
// If we borrowed a wake TCS for an active-only transfer we'll also call
// this function with "trigger=false" to just do the un-trigger/disable
// before using the TCS for wake purposes again.
//
// Note that the AP is only in charge of triggering active-only transfers.
// The AP never triggers sleep/wake values using this function.
//
#[no_mangle]
unsafe extern "C" fn __tcs_set_trigger(drv: *mut rsc_drv, tcs_id: c_int, trigger: bool) {
    static void __tcs_set_trigger(struct rsc_drv *drv, int tcs_id, bool trigger)
    {
    u32 enable;
    let mut reg: u32 = drv.regs[RSC_DRV_CONTROL];
//
// HW req: Clear the DRV_CONTROL and enable TCS again
// While clearing ensure that the AMC mode trigger is cleared
// and then the mode enable is cleared.
//
    enable = read_tcs_reg(drv, reg, tcs_id);
    enable &= ~TCS_AMC_MODE_TRIGGER;
    write_tcs_reg_sync(drv, reg, tcs_id, enable);
    enable &= ~TCS_AMC_MODE_ENABLE;
    write_tcs_reg_sync(drv, reg, tcs_id, enable);
    if (trigger) {
// Enable the AMC mode on the TCS and then trigger the TCS
    enable = TCS_AMC_MODE_ENABLE;
    write_tcs_reg_sync(drv, reg, tcs_id, enable);
    enable |= TCS_AMC_MODE_TRIGGER;
    write_tcs_reg(drv, reg, tcs_id, enable);
    }
    }
//
// enable_tcs_irq() - Enable or disable interrupts on the given TCS.
// @drv:     The controller.
// @tcs_id:  The global ID of this TCS.
// @enable:  If true then enable; if false then disable
//
// We only ever call this when we borrow a wake TCS for an active-only
// transfer. For active-only TCSes interrupts are always left enabled.
//
#[no_mangle]
unsafe extern "C" fn enable_tcs_irq(drv: *mut rsc_drv, tcs_id: c_int, enable: bool) {
    static void enable_tcs_irq(struct rsc_drv *drv, int tcs_id, bool enable)
    {
    u32 data;
    let mut reg: u32 = drv.regs[RSC_DRV_IRQ_ENABLE];
    data = readl_relaxed(drv.tcs_base + reg);
    if (enable)
    data |= BIT(tcs_id);
    else
    data &= ~BIT(tcs_id);
    writel_relaxed(data, drv.tcs_base + reg);
    }
//
// tcs_tx_done() - TX Done interrupt handler.
// @irq: The IRQ number (ignored).
// @p:   Pointer to "struct rsc_drv".
//
// Called for ACTIVE_ONLY transfers (those are the only ones we enable the
// IRQ for) when a transfer is done.
//
// Return: IRQ_HANDLED
//
#[no_mangle]
unsafe extern "C" fn tcs_tx_done(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t tcs_tx_done(int irq, void *p)
    {
    struct rsc_drv *drv = p;
    int i;
    unsigned long irq_status;
    const struct tcs_request *req;
    u32 reg;
    irq_status = readl_relaxed(drv.tcs_base + drv.regs[RSC_DRV_IRQ_STATUS]);
    for_each_set_bit(i, &irq_status, BITS_PER_TYPE(u32)) {
    req = get_req_from_tcs(drv, i);
    if (WARN_ON(!req))
    goto skip;
    trace_rpmh_tx_done(drv, i, req);
    if (req.is_read) {
    reg = drv.regs[RSC_DRV_CMD_RESP_DATA];
    req.cmds[0].data = read_tcs_reg(drv, reg, i);
    }
// Clear AMC trigger & enable modes and
// disable interrupt for this TCS
//
    __tcs_set_trigger(drv, i, false);
    skip:
// Reclaim the TCS
    write_tcs_reg(drv, drv.regs[RSC_DRV_CMD_ENABLE], i, 0);
    writel_relaxed(BIT(i), drv.tcs_base + drv.regs[RSC_DRV_IRQ_CLEAR]);
    spin_lock(&drv.lock);
    clear_bit(i, drv.tcs_in_use);
//
// Disable interrupt for WAKE TCS to avoid being
// spammed with interrupts coming when the solver
// sends its wake votes.
//
    if (!drv.tcs[ACTIVE_TCS].num_tcs)
    enable_tcs_irq(drv, i, false);
    spin_unlock(&drv.lock);
    wake_up(&drv.tcs_wait);
    if (req)
    rpmh_tx_done(req);
    }
    return IRQ_HANDLED;
    }
//
// __tcs_buffer_write() - Write to TCS hardware from a request; don't trigger.
// @drv:    The controller.
// @tcs_id: The global ID of this TCS.
// @cmd_id: The index within the TCS to start writing.
// @msg:    The message we want to send, which will contain several addr/data
// pairs to program (but few enough that they all fit in one TCS).
//
// This is used for all types of transfers (active, sleep, and wake).
//
    static void __tcs_buffer_write(struct rsc_drv *drv, int tcs_id, int cmd_id,
    const struct tcs_request *msg)
    {
    u32 msgid;
    let mut cmd_msgid: u32 = CMD_MSGID_LEN;
    let mut cmd_enable: u32 = 0;
    struct tcs_cmd *cmd;
    int i, j;
// Convert all commands to RR when the request has wait_for_compl set
    cmd_msgid |= msg.wait_for_compl ? CMD_MSGID_RESP_REQ : 0;
    if (!msg.is_read)
    cmd_msgid |= CMD_MSGID_WRITE;
    for (i = 0, j = cmd_id; i < msg.num_cmds; i++, j++) {
    cmd = &msg.cmds[i];
    cmd_enable |= BIT(j);
    msgid = cmd_msgid;
//
// Additionally, if the cmd->wait is set, make the command
// response reqd even if the overall request was fire-n-forget.
//
    msgid |= cmd.wait ? CMD_MSGID_RESP_REQ : 0;
    write_tcs_cmd(drv, drv.regs[RSC_DRV_CMD_MSGID], tcs_id, j, msgid);
    write_tcs_cmd(drv, drv.regs[RSC_DRV_CMD_ADDR], tcs_id, j, cmd.addr);
    if (!msg.is_read)
    write_tcs_cmd(drv, drv.regs[RSC_DRV_CMD_DATA], tcs_id, j, cmd.data);
    trace_rpmh_send_msg(drv, tcs_id, msg.state, j, msgid, cmd);
    }
    cmd_enable |= read_tcs_reg(drv, drv.regs[RSC_DRV_CMD_ENABLE], tcs_id);
    write_tcs_reg(drv, drv.regs[RSC_DRV_CMD_ENABLE], tcs_id, cmd_enable);
    }
//
// check_for_req_inflight() - Look to see if conflicting cmds are in flight.
// @drv: The controller.
// @tcs: A pointer to the tcs_group used for ACTIVE_ONLY transfers.
// @msg: The message we want to send, which will contain several addr/data
// pairs to program (but few enough that they all fit in one TCS).
//
// This will walk through the TCSes in the group and check if any of them
// appear to be sending to addresses referenced in the message. If it finds
// one it'll return -EBUSY.
//
// Only for use for active-only transfers.
//
// Must be called with the drv->lock held since that protects tcs_in_use.
//
// Return: 0 if nothing in flight or -EBUSY if we should try again later.
// The caller must re-enable interrupts between tries since that's
// the only way tcs_in_use will ever be updated and the only way
// RSC_DRV_CMD_ENABLE will ever be cleared.
//
    static int check_for_req_inflight(struct rsc_drv *drv, struct tcs_group *tcs,
    const struct tcs_request *msg)
    {
    unsigned long curr_enabled;
    u32 addr;
    int j, k;
    let mut i: c_int = tcs.offset;
    for_each_set_bit_from(i, drv.tcs_in_use, tcs.offset + tcs.num_tcs) {
    curr_enabled = read_tcs_reg(drv, drv.regs[RSC_DRV_CMD_ENABLE], i);
    for_each_set_bit(j, &curr_enabled, MAX_CMDS_PER_TCS) {
    addr = read_tcs_cmd(drv, drv.regs[RSC_DRV_CMD_ADDR], i, j);
    for (k = 0; k < msg.num_cmds; k++) {
    if (cmd_db_match_resource_addr(msg.cmds[k].addr, addr))
    return -EBUSY;
    }
    }
    }
    return 0;
    }
//
// find_free_tcs() - Find free tcs in the given tcs_group; only for active.
// @tcs: A pointer to the active-only tcs_group (or the wake tcs_group if
// we borrowed it because there are zero active-only ones).
//
// Must be called with the drv->lock held since that protects tcs_in_use.
//
// Return: The first tcs that's free or -EBUSY if all in use.
//
#[no_mangle]
unsafe extern "C" fn find_free_tcs(tcs: *mut tcs_group) -> c_int {
    static int find_free_tcs(struct tcs_group *tcs)
    {
    const struct rsc_drv *drv = tcs.drv;
    unsigned long i;
    let mut max: c_ulong = tcs.offset + tcs.num_tcs;
    i = find_next_zero_bit(drv.tcs_in_use, max, tcs.offset);
    if (i >= max)
    return -EBUSY;
    return i;
    }
//
// claim_tcs_for_req() - Claim a tcs in the given tcs_group; only for active.
// @drv: The controller.
// @tcs: The tcs_group used for ACTIVE_ONLY transfers.
// @msg: The data to be sent.
//
// Claims a tcs in the given tcs_group while making sure that no existing cmd
// is in flight that would conflict with the one in @msg.
//
// Context: Must be called with the drv->lock held since that protects
// tcs_in_use.
//
// Return: The id of the claimed tcs or -EBUSY if a matching msg is in flight
// or the tcs_group is full.
//
    static int claim_tcs_for_req(struct rsc_drv *drv, struct tcs_group *tcs,
    const struct tcs_request *msg)
    {
    int ret;
//
// The h/w does not like if we send a request to the same address,
// when one is already in-flight or being processed.
//
    ret = check_for_req_inflight(drv, tcs, msg);
    if (ret)
    return ret;
    return find_free_tcs(tcs);
    }
//
// rpmh_rsc_send_data() - Write / trigger active-only message.
// @drv: The controller.
// @msg: The data to be sent.
//
// NOTES:
// - This is only used for "ACTIVE_ONLY" since the limitations of this
// function don't make sense for sleep/wake cases.
// - To do the transfer, we will grab a whole TCS for ourselves--we don't
// try to share. If there are none available we'll wait indefinitely
// for a free one.
// - This function will not wait for the commands to be finished, only for
// data to be programmed into the RPMh. See rpmh_tx_done() which will
// be called when the transfer is fully complete.
// - This function must be called with interrupts enabled. If the hardware
// is busy doing someone else's transfer we need that transfer to fully
// finish so that we can have the hardware, and to fully finish it needs
// the interrupt handler to run. If the interrupts is set to run on the
// active CPU this can never happen if interrupts are disabled.
//
// Return: 0 on success, -EINVAL on error.
//
#[no_mangle]
pub unsafe extern "C" fn rpmh_rsc_send_data(drv: *mut rsc_drv, msg: *const tcs_request) -> c_int {
    int rpmh_rsc_send_data(struct rsc_drv *drv, const struct tcs_request *msg)
    {
    struct tcs_group *tcs;
    int tcs_id;
    might_sleep();
    tcs = get_tcs_for_msg(drv, msg);
    if (IS_ERR(tcs))
    return PTR_ERR(tcs);
    spin_lock_irq(&drv.lock);
// Wait forever for a free tcs. It better be there eventually!
    wait_event_lock_irq(drv.tcs_wait,
    (tcs_id = claim_tcs_for_req(drv, tcs, msg)) >= 0,
    drv.lock);
    tcs.req[tcs_id - tcs.offset] = msg;
    set_bit(tcs_id, drv.tcs_in_use);
    if (msg.state == RPMH_ACTIVE_ONLY_STATE && tcs.type != ACTIVE_TCS) {
//
// Clear previously programmed WAKE commands in selected
// repurposed TCS to avoid triggering them. tcs->slots will be
// cleaned from rpmh_flush() by invoking rpmh_rsc_invalidate()
//
    write_tcs_reg_sync(drv, drv.regs[RSC_DRV_CMD_ENABLE], tcs_id, 0);
    enable_tcs_irq(drv, tcs_id, true);
    }
    spin_unlock_irq(&drv.lock);
//
// These two can be done after the lock is released because:
// - We marked "tcs_in_use" under lock.
// - Once "tcs_in_use" has been marked nobody else could be writing
// to these registers until the interrupt goes off.
// - The interrupt can't go off until we trigger w/ the last line
// of __tcs_set_trigger() below.
//
    __tcs_buffer_write(drv, tcs_id, 0, msg);
    __tcs_set_trigger(drv, tcs_id, true);
    return 0;
    }
//
// find_slots() - Find a place to write the given message.
// @tcs:    The tcs group to search.
// @msg:    The message we want to find room for.
// @tcs_id: If we return 0 from the function, we return the global ID of the
// TCS to write to here.
// @cmd_id: If we return 0 from the function, we return the index of
// the command array of the returned TCS where the client should
// start writing the message.
//
// Only for use on sleep/wake TCSes since those are the only ones we maintain
// tcs->slots for.
//
// Return: -ENOMEM if there was no room, else 0.
//
    static int find_slots(struct tcs_group *tcs, const struct tcs_request *msg,
    int *tcs_id, int *cmd_id)
    {
    int slot, offset;
    let mut i: c_int = 0;
// Do over, until we can fit the full payload in a single TCS
    do {
    slot = bitmap_find_next_zero_area(tcs.slots, MAX_TCS_SLOTS,
    i, msg.num_cmds, 0);
    if (slot >= tcs.num_tcs * tcs.ncpt)
    return -ENOMEM;
    i += tcs.ncpt;
    } while (slot + msg.num_cmds - 1 >= i);
    bitmap_set(tcs.slots, slot, msg.num_cmds);
    offset = slot / tcs.ncpt;
// tcs_id = offset + tcs->offset;
// cmd_id = slot % tcs->ncpt;
    return 0;
    }
//
// rpmh_rsc_write_ctrl_data() - Write request to controller but don't trigger.
// @drv: The controller.
// @msg: The data to be written to the controller.
//
// This should only be called for sleep/wake state, never active-only
// state.
//
// The caller must ensure that no other RPMH actions are happening and the
// controller is idle when this function is called since it runs lockless.
//
// Return: 0 if no error; else -error.
//
#[no_mangle]
pub unsafe extern "C" fn rpmh_rsc_write_ctrl_data(drv: *mut rsc_drv, msg: *const tcs_request) -> c_int {
    int rpmh_rsc_write_ctrl_data(struct rsc_drv *drv, const struct tcs_request *msg)
    {
    struct tcs_group *tcs;
    let mut tcs_id: c_int = 0, cmd_id = 0;
    int ret;
    tcs = get_tcs_for_msg(drv, msg);
    if (IS_ERR(tcs))
    return PTR_ERR(tcs);
// find the TCS id and the command in the TCS to write to
    ret = find_slots(tcs, msg, &tcs_id, &cmd_id);
    if (!ret)
    __tcs_buffer_write(drv, tcs_id, cmd_id, msg);
    return ret;
    }
//
// rpmh_rsc_ctrlr_is_busy() - Check if any of the AMCs are busy.
// @drv: The controller
//
// Checks if any of the AMCs are busy in handling ACTIVE sets.
// This is called from the last cpu powering down before flushing
// SLEEP and WAKE sets. If AMCs are busy, controller can not enter
// power collapse, so deny from the last cpu's pm notification.
//
// Context: Must be called with the drv->lock held.
//
// Return:
// * False		- AMCs are idle
// * True		- AMCs are busy
//
#[no_mangle]
unsafe extern "C" fn rpmh_rsc_ctrlr_is_busy(drv: *mut rsc_drv) -> bool {
    static bool rpmh_rsc_ctrlr_is_busy(struct rsc_drv *drv)
    {
    unsigned long set;
    const struct tcs_group *tcs = &drv.tcs[ACTIVE_TCS];
    unsigned long max;
//
// If we made an active request on a RSC that does not have a
// dedicated TCS for active state use, then re-purposed wake TCSes
// should be checked for not busy, because we used wake TCSes for
// active requests in this case.
//
    if (!tcs.num_tcs)
    tcs = &drv.tcs[WAKE_TCS];
    max = tcs.offset + tcs.num_tcs;
    set = find_next_bit(drv.tcs_in_use, max, tcs.offset);
    return set < max;
    }
//
// rpmh_rsc_write_next_wakeup() - Write next wakeup in CONTROL_TCS.
// @drv: The controller
//
// Writes maximum wakeup cycles when called from suspend.
// Writes earliest hrtimer wakeup when called from idle.
//
#[no_mangle]
pub unsafe extern "C" fn rpmh_rsc_write_next_wakeup(drv: *mut rsc_drv) {
    void rpmh_rsc_write_next_wakeup(struct rsc_drv *drv)
    {
    ktime_t now, wakeup;
    u64 wakeup_us, wakeup_cycles = ~0;
    u32 lo, hi;
    if (!drv.tcs[CONTROL_TCS].num_tcs || !drv.genpd_nb.notifier_call)
    return;
// Set highest time when system (timekeeping) is suspended
    if (system_state == SYSTEM_SUSPEND)
    goto exit;
// Find the earliest hrtimer wakeup from online cpus
    wakeup = dev_pm_genpd_get_next_hrtimer(drv.dev);
// Find the relative wakeup in kernel time scale
    now = ktime_get();
    wakeup = ktime_sub(wakeup, now);
    wakeup_us = ktime_to_us(wakeup);
// Convert the wakeup to arch timer scale
    wakeup_cycles = USECS_TO_CYCLES(wakeup_us);
    wakeup_cycles += arch_timer_read_counter();
    exit:
    lo = wakeup_cycles & RSC_DRV_CTL_TCS_DATA_LO_MASK;
    hi = wakeup_cycles >> RSC_DRV_CTL_TCS_DATA_SIZE;
    hi &= RSC_DRV_CTL_TCS_DATA_HI_MASK;
    hi |= RSC_DRV_CTL_TCS_DATA_HI_VALID;
    writel_relaxed(lo, drv.base + RSC_DRV_CTL_TCS_DATA_LO);
    writel_relaxed(hi, drv.base + RSC_DRV_CTL_TCS_DATA_HI);
    }
//
// rpmh_rsc_cpu_pm_callback() - Check if any of the AMCs are busy.
// @nfb:    Pointer to the notifier block in struct rsc_drv.
// @action: CPU_PM_ENTER, CPU_PM_ENTER_FAILED, or CPU_PM_EXIT.
// @v:      Unused
//
// This function is given to cpu_pm_register_notifier so we can be informed
// about when CPUs go down. When all CPUs go down we know no more active
// transfers will be started so we write sleep/wake sets. This function gets
// called from cpuidle code paths and also at system suspend time.
//
// If its last CPU going down and AMCs are not busy then writes cached sleep
// and wake messages to TCSes. The firmware then takes care of triggering
// them when entering deepest low power modes.
//
// Return: See cpu_pm_register_notifier()
//
    static int rpmh_rsc_cpu_pm_callback(struct notifier_block *nfb,
    unsigned long action, void *v)
    {
    struct rsc_drv *drv = container_of(nfb, struct rsc_drv, rsc_pm);
    let mut ret: c_int = NOTIFY_OK;
    int cpus_in_pm;
    switch (action) {
    case CPU_PM_ENTER:
    cpus_in_pm = atomic_inc_return(&drv.cpus_in_pm);
//
// NOTE: comments for num_online_cpus() point out that it's
// only a snapshot so we need to be careful. It should be OK
// for us to use, though.  It's important for us not to miss
// if we're the last CPU going down so it would only be a
// problem if a CPU went offline right after we did the check
// AND that CPU was not idle AND that CPU was the last non-idle
// CPU. That can't happen. CPUs would have to come out of idle
// before the CPU could go offline.
//
    if (cpus_in_pm < num_online_cpus())
    return NOTIFY_OK;
    break;
    case CPU_PM_ENTER_FAILED:
    case CPU_PM_EXIT:
    atomic_dec(&drv.cpus_in_pm);
    return NOTIFY_OK;
    default:
    return NOTIFY_DONE;
    }
//
// It's likely we're on the last CPU. Grab the drv->lock and write
// out the sleep/wake commands to RPMH hardware. Grabbing the lock
// means that if we race with another CPU coming up we are still
// guaranteed to be safe. If another CPU came up just after we checked
// and has grabbed the lock or started an active transfer then we'll
// notice we're busy and abort. If another CPU comes up after we start
// flushing it will be blocked from starting an active transfer until
// we're done flushing. If another CPU starts an active transfer after
// we release the lock we're still OK because we're no longer the last
// CPU.
//
    if (spin_trylock(&drv.lock)) {
    if (rpmh_rsc_ctrlr_is_busy(drv) || rpmh_flush(&drv.client))
    ret = NOTIFY_BAD;
    spin_unlock(&drv.lock);
    } else {
// Another CPU must be up
    return NOTIFY_OK;
    }
    if (ret == NOTIFY_BAD) {
// Double-check if we're here because someone else is up
    if (cpus_in_pm < num_online_cpus())
    ret = NOTIFY_OK;
    else
// We won't be called w/ CPU_PM_ENTER_FAILED
    atomic_dec(&drv.cpus_in_pm);
    }
    return ret;
    }
//
// rpmh_rsc_pd_callback() - Check if any of the AMCs are busy.
// @nfb:    Pointer to the genpd notifier block in struct rsc_drv.
// @action: GENPD_NOTIFY_PRE_OFF, GENPD_NOTIFY_OFF, GENPD_NOTIFY_PRE_ON or GENPD_NOTIFY_ON.
// @v:      Unused
//
// This function is given to dev_pm_genpd_add_notifier() so we can be informed
// about when cluster-pd is going down. When cluster go down we know no more active
// transfers will be started so we write sleep/wake sets. This function gets
// called from cpuidle code paths and also at system suspend time.
//
// If AMCs are not busy then writes cached sleep and wake messages to TCSes.
// The firmware then takes care of triggering them when entering deepest low power modes.
//
// Return:
// * NOTIFY_OK          - success
// * NOTIFY_BAD         - failure
//
    static int rpmh_rsc_pd_callback(struct notifier_block *nfb,
    unsigned long action, void *v)
    {
    struct rsc_drv *drv = container_of(nfb, struct rsc_drv, genpd_nb);
// We don't need to lock as genpd on/off are serialized
    if ((action == GENPD_NOTIFY_PRE_OFF) &&
    (rpmh_rsc_ctrlr_is_busy(drv) || rpmh_flush(&drv.client)))
    return NOTIFY_BAD;
    return NOTIFY_OK;
    }
#[no_mangle]
unsafe extern "C" fn rpmh_rsc_pd_detach(data: *mut c_void) {
    static void rpmh_rsc_pd_detach(void *data)
    {
    dev_pm_genpd_remove_notifier(data);
    }
#[no_mangle]
unsafe extern "C" fn rpmh_rsc_pd_attach(drv: *mut rsc_drv, dev: *mut device) -> c_int {
    static int rpmh_rsc_pd_attach(struct rsc_drv *drv, struct device *dev)
    {
    int ret;
    ret = devm_pm_runtime_enable(dev);
    if (ret)
    return ret;
    drv.genpd_nb.notifier_call = rpmh_rsc_pd_callback;
    ret = dev_pm_genpd_add_notifier(dev, &drv.genpd_nb);
    if (ret)
    return ret;
    return devm_add_action_or_reset(dev, rpmh_rsc_pd_detach, dev);
    }
#[no_mangle]
unsafe extern "C" fn rpmh_rsc_cpu_pm_unregister(data: *mut c_void) {
    static void rpmh_rsc_cpu_pm_unregister(void *data)
    {
    cpu_pm_unregister_notifier(data);
    }
#[no_mangle]
unsafe extern "C" fn rpmh_probe_tcs_config(pdev: *mut platform_device, drv: *mut rsc_drv) -> c_int {
    static int rpmh_probe_tcs_config(struct platform_device *pdev, struct rsc_drv *drv)
    {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcs_type_config {
    pub type: u32,
    pub n: u32,
    pub }: } tcs_cfg[TCS_TYPE_NR] = { { 0 },
    pub pdev->dev.of_node: *mut *mut device_node dn =,
    pub offset: u32 config, max_tcs, ncpt,,
    pub 0: int i, ret, n, st =,
    pub tcs: *mut tcs_group,
    pub &offset): ret = of_property_read_u32(dn, "qcom,tcs-offset",,
    if (ret)
    pub ret: return,
    pub offset: drv->tcs_base = drv->base +,
    pub drv->regs[DRV_PRNT_CHLD_CONFIG]): config = readl_relaxed(drv->base +,
    pub config: max_tcs =,
    pub drv->id): *mut *mut max_tcs &= DRV_NUM_TCS_MASK << (DRV_NUM_TCS_SHIFT,
    pub drv->id): *mut *mut max_tcs = max_tcs >> (DRV_NUM_TCS_SHIFT,
    pub DRV_NCPT_SHIFT): ncpt = config & (DRV_NCPT_MASK <<,
    pub DRV_NCPT_SHIFT: ncpt = ncpt >>,
    pub "qcom,tcs-config"): n = of_property_count_u32_elems(dn,,
    if (n != 2 * TCS_TYPE_NR)
    pub -EINVAL: return,
    pub {: for (i = 0; i < TCS_TYPE_NR; i++),
    ret = of_property_read_u32_index(dn, "qcom,tcs-config",
    pub &tcs_cfg[i].type): *mut *mut i  2,,
    if (ret)
    pub ret: return,
    if (tcs_cfg[i].type >= TCS_TYPE_NR)
    pub -EINVAL: return,
    ret = of_property_read_u32_index(dn, "qcom,tcs-config",
    pub &tcs_cfg[i].n): *mut *mut i  2 + 1,,
    if (ret)
    pub ret: return,
    if (tcs_cfg[i].n > MAX_TCS_PER_TYPE)
    pub -EINVAL: return,
    }
    pub {: for (i = 0; i < TCS_TYPE_NR; i++),
    pub &drv->tcs[tcs_cfg[i].type]: tcs =,
    if (tcs.drv)
    pub -EINVAL: return,
    pub drv: tcs->drv =,
    pub tcs_cfg[i].type: tcs->type =,
    pub tcs_cfg[i].n: tcs->num_tcs =,
    pub ncpt: tcs->ncpt =,
    if (!tcs.num_tcs || tcs.type == CONTROL_TCS)
    if (st + tcs.num_tcs > max_tcs ||
    st + tcs.num_tcs >= BITS_PER_BYTE * sizeof(tcs.mask))
    pub -EINVAL: return,
    pub st: tcs->mask = ((1 << tcs->num_tcs) - 1) <<,
    pub st: tcs->offset =,
    pub tcs->num_tcs: st +=,
    }
    pub st: drv->num_tcs =,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn rpmh_rsc_probe(pdev: *mut platform_device) -> c_int {
    static int rpmh_rsc_probe(struct platform_device *pdev)
    {
    pub pdev->dev.of_node: *mut *mut device_node dn =,
    pub drv: *mut rsc_drv,
    pub {0}: char drv_id[10] =,
    pub irq: int ret,,
    pub solver_config: u32,
    pub rsc_id: u32,
//
// Even though RPMh doesn't directly use cmd-db, all of its children
// do. To avoid adding this check to our children we'll do it now.
//
    pub cmd_db_ready(): ret =,
    if (ret)
    return dev_err_probe(&pdev.dev, ret,
    pub available\n"): "Command DB not,
    pub GFP_KERNEL): *mut *mut drv = devm_kzalloc(&pdev->dev, sizeof(drv),,
    if (!drv)
    pub -ENOMEM: return,
    pub &drv->id): ret = of_property_read_u32(dn, "qcom,drv-id",,
    if (ret)
    pub ret: return,
    pub NULL): drv->name = of_get_property(dn, "label",,
    if (!drv.name)
    pub dev_name(&pdev->dev): drv->name =,
    pub drv->id): snprintf(drv_id, ARRAY_SIZE(drv_id), "drv-%d",,
    pub drv_id): drv->base = devm_platform_ioremap_resource_byname(pdev,,
    if (IS_ERR(drv.base))
    pub PTR_ERR(drv->base): return,
    pub RSC_DRV_ID): rsc_id = readl_relaxed(drv->base +,
    pub MAJOR_VER_SHIFT): drv->ver.major = rsc_id & (MAJOR_VER_MASK <<,
    pub MAJOR_VER_SHIFT: drv->ver.major >>=,
    pub MINOR_VER_SHIFT): drv->ver.minor = rsc_id & (MINOR_VER_MASK <<,
    pub MINOR_VER_SHIFT: drv->ver.minor >>=,
    if (drv.ver.major >= 3)
    pub rpmh_rsc_reg_offset_ver_3_0: drv->regs =,
    else
    pub rpmh_rsc_reg_offset_ver_2_7: drv->regs =,
    pub drv): ret = rpmh_probe_tcs_config(pdev,,
    if (ret)
    pub ret: return,
    pub MAX_TCS_NR): bitmap_zero(drv->tcs_in_use,,
    pub drv->id): irq = platform_get_irq(pdev,,
    if (irq < 0)
    pub irq: return,
    ret = devm_request_irq(&pdev.dev, irq, tcs_tx_done,
    IRQF_TRIGGER_HIGH | IRQF_NO_SUSPEND,
    pub drv): drv->name,,
    if (ret)
    pub ret: return,
//
// CPU PM/genpd notification are not required for controllers that support
// 'HW solver' mode where they can be in autonomous mode executing low
// power mode to power down.
//
    pub drv->regs[DRV_SOLVER_CONFIG]): solver_config = readl_relaxed(drv->base +,
    pub DRV_HW_SOLVER_SHIFT: solver_config &= DRV_HW_SOLVER_MASK <<,
    pub DRV_HW_SOLVER_SHIFT: solver_config = solver_config >>,
    if (!solver_config) {
    if (pdev.dev.pm_domain) {
    pub &pdev->dev): ret = rpmh_rsc_pd_attach(drv,,
    if (ret)
    pub ret: return,
    } else {
    pub rpmh_rsc_cpu_pm_callback: drv->rsc_pm.notifier_call =,
    pub cpu_pm_register_notifier(&drv->rsc_pm): ret =,
    if (ret)
    pub ret: return,
    ret = devm_add_action_or_reset(&pdev.dev,
    rpmh_rsc_cpu_pm_unregister,
    if (ret)
    pub ret: return,
    }
    }
// Enable the active TCS to send requests immediately
    writel_relaxed(drv.tcs[ACTIVE_TCS].mask,
    pub drv->regs[RSC_DRV_IRQ_ENABLE]): drv->tcs_base +,
    pub drv): dev_set_drvdata(&pdev->dev,,
    pub &pdev->dev: drv->dev =,
    pub devm_of_platform_populate(&pdev->dev): return,
    }
    static const struct of_device_id rpmh_drv_match[] = {
    { .compatible = "qcom,rpmh-rsc", },
    { }
}

    MODULE_DEVICE_TABLE(of, rpmh_drv_match);
    static struct platform_driver rpmh_driver = {
    .probe = rpmh_rsc_probe,
    .driver = {
    .name = "rpmh",
    .of_match_table = rpmh_drv_match,
    .suppress_bind_attrs = true,
    },
    };
#[no_mangle]
unsafe extern "C" fn rpmh_driver_init() -> int __init {
    static int __init rpmh_driver_init(void)
    {
    return platform_driver_register(&rpmh_driver);
    }
    core_initcall(rpmh_driver_init);
    MODULE_DESCRIPTION("Qualcomm Technologies, Inc. RPMh Driver");
    MODULE_LICENSE("GPL v2");
