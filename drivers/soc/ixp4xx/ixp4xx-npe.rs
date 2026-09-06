//! Automatically rewritten from C to Rust
//! Source: drivers/soc/ixp4xx/ixp4xx-npe.c
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
//
// Intel IXP4xx Network Processor Engine driver for Linux
//
// Copyright (C) 2007 Krzysztof Halasa <khc@pm.waw.pl>
//
// The code is based on publicly available information:
// - Intel IXP4xx Developer's Manual and other e-papers
// - Intel IXP400 Access Library Software (BSD license)
// - previous works by Christian Hohnstaedt <chohnstaedt@innominate.com>
// Thanks, Christian.
//

pub const DEBUG_MSG: c_int = 0;
pub const DEBUG_FW: c_int = 0;
pub const NPE_COUNT: c_int = 3;

pub const NPE_42X_DATA_SIZE: c_uint = 0x800	/* in dwords */;
pub const NPE_46X_DATA_SIZE: c_uint = 0x1000;
pub const NPE_A_42X_INSTR_SIZE: c_uint = 0x1000;
pub const NPE_B_AND_C_42X_INSTR_SIZE: c_uint = 0x800;
pub const NPE_46X_INSTR_SIZE: c_uint = 0x1000;
pub const REGS_SIZE: c_uint = 0x1000;
pub const NPE_PHYS_REG: c_int = 32;
pub const FW_MAGIC: c_uint = 0xFEEDF00D;
pub const FW_BLOCK_TYPE_INSTR: c_uint = 0x0;
pub const FW_BLOCK_TYPE_DATA: c_uint = 0x1;
pub const FW_BLOCK_TYPE_EOF: c_uint = 0xF;
// NPE exec status (read) and command (write)
pub const CMD_NPE_STEP: c_uint = 0x01;
pub const CMD_NPE_START: c_uint = 0x02;
pub const CMD_NPE_STOP: c_uint = 0x03;
pub const CMD_NPE_CLR_PIPE: c_uint = 0x04;
pub const CMD_CLR_PROFILE_CNT: c_uint = 0x0C;
pub const CMD_RD_INS_MEM: c_uint = 0x10 /* instruction memory */;
pub const CMD_WR_INS_MEM: c_uint = 0x11;
pub const CMD_RD_DATA_MEM: c_uint = 0x12 /* data memory */;
pub const CMD_WR_DATA_MEM: c_uint = 0x13;
pub const CMD_RD_ECS_REG: c_uint = 0x14 /* exec access register */;
pub const CMD_WR_ECS_REG: c_uint = 0x15;
pub const STAT_RUN: c_uint = 0x80000000;
pub const STAT_STOP: c_uint = 0x40000000;
pub const STAT_CLEAR: c_uint = 0x20000000;
pub const STAT_ECS_K: c_uint = 0x00800000 /* pipeline clean */;
pub const NPE_STEVT: c_uint = 0x1B;
pub const NPE_STARTPC: c_uint = 0x1C;
pub const NPE_REGMAP: c_uint = 0x1E;
pub const NPE_CINDEX: c_uint = 0x1F;
pub const INSTR_WR_REG_SHORT: c_uint = 0x0000C000;
pub const INSTR_WR_REG_BYTE: c_uint = 0x00004000;
pub const INSTR_RD_FIFO: c_uint = 0x0F888220;
pub const INSTR_RESET_MBOX: c_uint = 0x0FAC8210;
pub const ECS_BG_CTXT_REG_0: c_uint = 0x00 /* Background Executing Context */;
pub const ECS_BG_CTXT_REG_1: c_uint = 0x01 /*		Stack level */;
pub const ECS_BG_CTXT_REG_2: c_uint = 0x02;
pub const ECS_PRI_1_CTXT_REG_0: c_uint = 0x04 /* Priority 1 Executing Context */;
pub const ECS_PRI_1_CTXT_REG_1: c_uint = 0x05 /*		Stack level */;
pub const ECS_PRI_1_CTXT_REG_2: c_uint = 0x06;
pub const ECS_PRI_2_CTXT_REG_0: c_uint = 0x08 /* Priority 2 Executing Context */;
pub const ECS_PRI_2_CTXT_REG_1: c_uint = 0x09 /*		Stack level */;
pub const ECS_PRI_2_CTXT_REG_2: c_uint = 0x0A;
pub const ECS_DBG_CTXT_REG_0: c_uint = 0x0C /* Debug Executing Context */;
pub const ECS_DBG_CTXT_REG_1: c_uint = 0x0D /*		Stack level */;
pub const ECS_DBG_CTXT_REG_2: c_uint = 0x0E;
pub const ECS_INSTRUCT_REG: c_uint = 0x11 /* NPE Instruction Register */;
pub const ECS_REG_0_ACTIVE: c_uint = 0x80000000 /* all levels */;
pub const ECS_REG_0_NEXTPC_MASK: c_uint = 0x1FFF0000 /* BG/PRI1/PRI2 levels */;
pub const ECS_REG_0_LDUR_BITS: c_int = 8;
pub const ECS_REG_0_LDUR_MASK: c_uint = 0x00000700 /* all levels */;
pub const ECS_REG_1_CCTXT_BITS: c_int = 16;
pub const ECS_REG_1_CCTXT_MASK: c_uint = 0x000F0000 /* all levels */;
pub const ECS_REG_1_SELCTXT_BITS: c_int = 0;
pub const ECS_REG_1_SELCTXT_MASK: c_uint = 0x0000000F /* all levels */;
pub const ECS_DBG_REG_2_IF: c_uint = 0x00100000 /* debug level */;
pub const ECS_DBG_REG_2_IE: c_uint = 0x00080000 /* debug level */;
// NPE watchpoint_fifo register bit
pub const WFIFO_VALID: c_uint = 0x80000000;
// NPE messaging_status register bit definitions
pub const MSGSTAT_OFNE: c_uint = 0x00010000 /* OutFifoNotEmpty */;
pub const MSGSTAT_IFNF: c_uint = 0x00020000 /* InFifoNotFull */;
pub const MSGSTAT_OFNF: c_uint = 0x00040000 /* OutFifoNotFull */;
pub const MSGSTAT_IFNE: c_uint = 0x00080000 /* InFifoNotEmpty */;
pub const MSGSTAT_MBINT: c_uint = 0x00100000 /* Mailbox interrupt */;
pub const MSGSTAT_IFINT: c_uint = 0x00200000 /* InFifo interrupt */;
pub const MSGSTAT_OFINT: c_uint = 0x00400000 /* OutFifo interrupt */;
pub const MSGSTAT_WFINT: c_uint = 0x00800000 /* WatchFifo interrupt */;
// NPE messaging_control register bit definitions
pub const MSGCTL_OUT_FIFO: c_uint = 0x00010000 /* enable output FIFO */;
pub const MSGCTL_IN_FIFO: c_uint = 0x00020000 /* enable input FIFO */;
pub const MSGCTL_OUT_FIFO_WRITE: c_uint = 0x01000000 /* enable FIFO + WRITE */;
pub const MSGCTL_IN_FIFO_WRITE: c_uint = 0x02000000;
// NPE mailbox_status value for reset
pub const RESET_MBOX_STAT: c_uint = 0x0000F0F0;

    const char *npe_names[] = { NPE_A_FIRMWARE, NPE_B_FIRMWARE, NPE_C_FIRMWARE };

    printk(pri "%s: " fmt, npe_name(npe), ## __VA_ARGS__)

    print_npe(KERN_DEBUG, npe, fmt, ## __VA_ARGS__)

    static struct {
    u32 reg, val;
    } ecs_reset[] = {
    { ECS_BG_CTXT_REG_0,	0xA0000000 },
    { ECS_BG_CTXT_REG_1,	0x01000000 },
    { ECS_BG_CTXT_REG_2,	0x00008000 },
    { ECS_PRI_1_CTXT_REG_0,	0x20000080 },
    { ECS_PRI_1_CTXT_REG_1,	0x01000000 },
    { ECS_PRI_1_CTXT_REG_2,	0x00008000 },
    { ECS_PRI_2_CTXT_REG_0,	0x20000080 },
    { ECS_PRI_2_CTXT_REG_1,	0x01000000 },
    { ECS_PRI_2_CTXT_REG_2,	0x00008000 },
    { ECS_DBG_CTXT_REG_0,	0x20000000 },
    { ECS_DBG_CTXT_REG_1,	0x00000000 },
    { ECS_DBG_CTXT_REG_2,	0x001E0000 },
    { ECS_INSTRUCT_REG,	0x1003C00F },
    };
    static struct npe npe_tab[NPE_COUNT] = {
    {
    .id	= 0,
    }, {
    .id	= 1,
    }, {
    .id	= 2,
    }
    };
#[no_mangle]
pub unsafe extern "C" fn npe_running(npe: *mut npe) -> c_int {
    int npe_running(struct npe *npe)
    {
    return (__raw_readl(&npe.regs.exec_status_cmd) & STAT_RUN) != 0;
    }
#[no_mangle]
unsafe extern "C" fn npe_cmd_write(npe: *mut npe, addr: u32, cmd: c_int, data: u32) {
    static void npe_cmd_write(struct npe *npe, u32 addr, int cmd, u32 data)
    {
    __raw_writel(data, &npe.regs.exec_data);
    __raw_writel(addr, &npe.regs.exec_addr);
    __raw_writel(cmd, &npe.regs.exec_status_cmd);
    }
#[no_mangle]
unsafe extern "C" fn npe_cmd_read(npe: *mut npe, addr: u32, cmd: c_int) -> u32 {
    static u32 npe_cmd_read(struct npe *npe, u32 addr, int cmd)
    {
    __raw_writel(addr, &npe.regs.exec_addr);
    __raw_writel(cmd, &npe.regs.exec_status_cmd);
// Iintroduce extra read cycles after issuing read command to NPE
    so that we read the register after the NPE has updated it.
    This is to overcome race condition between XScale and NPE */
    __raw_readl(&npe.regs.exec_data);
    __raw_readl(&npe.regs.exec_data);
    return __raw_readl(&npe.regs.exec_data);
    }
#[no_mangle]
unsafe extern "C" fn npe_clear_active(npe: *mut npe, reg: u32) {
    static void npe_clear_active(struct npe *npe, u32 reg)
    {
    let mut val: u32 = npe_cmd_read(npe, reg, CMD_RD_ECS_REG);
    npe_cmd_write(npe, reg, CMD_WR_ECS_REG, val & ~ECS_REG_0_ACTIVE);
    }
#[no_mangle]
unsafe extern "C" fn npe_start(npe: *mut npe) {
    static void npe_start(struct npe *npe)
    {
// ensure only Background Context Stack Level is active
    npe_clear_active(npe, ECS_PRI_1_CTXT_REG_0);
    npe_clear_active(npe, ECS_PRI_2_CTXT_REG_0);
    npe_clear_active(npe, ECS_DBG_CTXT_REG_0);
    __raw_writel(CMD_NPE_CLR_PIPE, &npe.regs.exec_status_cmd);
    __raw_writel(CMD_NPE_START, &npe.regs.exec_status_cmd);
    }
#[no_mangle]
unsafe extern "C" fn npe_stop(npe: *mut npe) {
    static void npe_stop(struct npe *npe)
    {
    __raw_writel(CMD_NPE_STOP, &npe.regs.exec_status_cmd);
    __raw_writel(CMD_NPE_CLR_PIPE, &npe.regs.exec_status_cmd); /*FIXME?*/
    }
    static int __must_check npe_debug_instr(struct npe *npe, u32 instr, u32 ctx,
    u32 ldur)
    {
    u32 wc;
    int i;
// set the Active bit, and the LDUR, in the debug level
    npe_cmd_write(npe, ECS_DBG_CTXT_REG_0, CMD_WR_ECS_REG,
    ECS_REG_0_ACTIVE | (ldur << ECS_REG_0_LDUR_BITS));
// set CCTXT at ECS DEBUG L3 to specify in which context to execute
    the instruction, and set SELCTXT at ECS DEBUG Level to specify
    which context store to access.
    Debug ECS Level Reg 1 has form 0x000n000n, where n = context number
//
    npe_cmd_write(npe, ECS_DBG_CTXT_REG_1, CMD_WR_ECS_REG,
    (ctx << ECS_REG_1_CCTXT_BITS) |
    (ctx << ECS_REG_1_SELCTXT_BITS));
// clear the pipeline
    __raw_writel(CMD_NPE_CLR_PIPE, &npe.regs.exec_status_cmd);
// load NPE instruction into the instruction register
    npe_cmd_write(npe, ECS_INSTRUCT_REG, CMD_WR_ECS_REG, instr);
// we need this value later to wait for completion of NPE execution
    step */
    wc = __raw_readl(&npe.regs.watch_count);
// issue a Step One command via the Execution Control register
    __raw_writel(CMD_NPE_STEP, &npe.regs.exec_status_cmd);
// Watch Count register increments when NPE completes an instruction
    for (i = 0; i < MAX_RETRIES; i++) {
    if (wc != __raw_readl(&npe.regs.watch_count))
    return 0;
    udelay(1);
    }
    print_npe(KERN_ERR, npe, "reset: npe_debug_instr(): timeout\n");
    return -ETIMEDOUT;
    }
    static int __must_check npe_logical_reg_write8(struct npe *npe, u32 addr,
    u8 val, u32 ctx)
    {
// here we build the NPE assembler instruction: mov8 d0, #0
    u32 instr = INSTR_WR_REG_BYTE |	/* OpCode */
    addr << 9 |		/* base Operand */
    (val & 0x1F) << 4 |	/* lower 5 bits to immediate data */
    (val & ~0x1F) << (18 - 5);/* higher 3 bits to CoProc instr. */
    return npe_debug_instr(npe, instr, ctx, 1); /* execute it */
    }
    static int __must_check npe_logical_reg_write16(struct npe *npe, u32 addr,
    u16 val, u32 ctx)
    {
// here we build the NPE assembler instruction: mov16 d0, #0
    u32 instr = INSTR_WR_REG_SHORT | /* OpCode */
    addr << 9 |		/* base Operand */
    (val & 0x1F) << 4 |	/* lower 5 bits to immediate data */
    (val & ~0x1F) << (18 - 5);/* higher 11 bits to CoProc instr. */
    return npe_debug_instr(npe, instr, ctx, 1); /* execute it */
    }
    static int __must_check npe_logical_reg_write32(struct npe *npe, u32 addr,
    u32 val, u32 ctx)
    {
// write in 16 bit steps first the high and then the low value
    if (npe_logical_reg_write16(npe, addr, val >> 16, ctx))
    return -ETIMEDOUT;
    return npe_logical_reg_write16(npe, addr + 2, val & 0xFFFF, ctx);
    }
#[no_mangle]
unsafe extern "C" fn npe_reset(npe: *mut npe) -> c_int {
    static int npe_reset(struct npe *npe)
    {
    let mut reset_bit: u32 = (IXP4XX_FEATURE_RESET_NPEA << npe.id);
    u32 val, ctl, exec_count, ctx_reg2;
    int i;
    ctl = (__raw_readl(&npe.regs.messaging_control) | 0x3F000000) &
    0x3F3FFFFF;
// disable parity interrupt
    __raw_writel(ctl & 0x3F00FFFF, &npe.regs.messaging_control);
// pre exec - debug instruction
// turn off the halt bit by clearing Execution Count register.
    exec_count = __raw_readl(&npe.regs.exec_count);
    __raw_writel(0, &npe.regs.exec_count);
// ensure that IF and IE are on (temporarily), so that we don't end up
    stepping forever */
    ctx_reg2 = npe_cmd_read(npe, ECS_DBG_CTXT_REG_2, CMD_RD_ECS_REG);
    npe_cmd_write(npe, ECS_DBG_CTXT_REG_2, CMD_WR_ECS_REG, ctx_reg2 |
    ECS_DBG_REG_2_IF | ECS_DBG_REG_2_IE);
// clear the FIFOs
    while (__raw_readl(&npe.regs.watchpoint_fifo) & WFIFO_VALID)
    ;
    while (__raw_readl(&npe.regs.messaging_status) & MSGSTAT_OFNE)
// read from the outFIFO until empty
    print_npe(KERN_DEBUG, npe, "npe_reset: read FIFO = 0x%X\n",
    __raw_readl(&npe.regs.in_out_fifo));
    while (__raw_readl(&npe.regs.messaging_status) & MSGSTAT_IFNE)
// step execution of the NPE intruction to read inFIFO using
    the Debug Executing Context stack */
    if (npe_debug_instr(npe, INSTR_RD_FIFO, 0, 0))
    return -ETIMEDOUT;
// reset the mailbox reg from the XScale side
    __raw_writel(RESET_MBOX_STAT, &npe.regs.mailbox_status);
// from NPE side
    if (npe_debug_instr(npe, INSTR_RESET_MBOX, 0, 0))
    return -ETIMEDOUT;
// Reset the physical registers in the NPE register file
    for (val = 0; val < NPE_PHYS_REG; val++) {
    if (npe_logical_reg_write16(npe, NPE_REGMAP, val >> 1, 0))
    return -ETIMEDOUT;
// address is either 0 or 4
    if (npe_logical_reg_write32(npe, (val & 1) * 4, 0, 0))
    return -ETIMEDOUT;
    }
// Reset the context store = each context's Context Store registers
// Context 0 has no STARTPC. Instead, this value is used to set NextPC
    for Background ECS, to set where NPE starts executing code */
    val = npe_cmd_read(npe, ECS_BG_CTXT_REG_0, CMD_RD_ECS_REG);
    val &= ~ECS_REG_0_NEXTPC_MASK;
    val |= (0 /* NextPC */ << 16) & ECS_REG_0_NEXTPC_MASK;
    npe_cmd_write(npe, ECS_BG_CTXT_REG_0, CMD_WR_ECS_REG, val);
    for (i = 0; i < 16; i++) {
    if (i) {	/* Context 0 has no STEVT nor STARTPC */
// STEVT = off, 0x80
    if (npe_logical_reg_write8(npe, NPE_STEVT, 0x80, i))
    return -ETIMEDOUT;
    if (npe_logical_reg_write16(npe, NPE_STARTPC, 0, i))
    return -ETIMEDOUT;
    }
// REGMAP = d0->p0, d8->p2, d16->p4
    if (npe_logical_reg_write16(npe, NPE_REGMAP, 0x820, i))
    return -ETIMEDOUT;
    if (npe_logical_reg_write8(npe, NPE_CINDEX, 0, i))
    return -ETIMEDOUT;
    }
// post exec
// clear active bit in debug level
    npe_cmd_write(npe, ECS_DBG_CTXT_REG_0, CMD_WR_ECS_REG, 0);
// clear the pipeline
    __raw_writel(CMD_NPE_CLR_PIPE, &npe.regs.exec_status_cmd);
// restore previous values
    __raw_writel(exec_count, &npe.regs.exec_count);
    npe_cmd_write(npe, ECS_DBG_CTXT_REG_2, CMD_WR_ECS_REG, ctx_reg2);
// write reset values to Execution Context Stack registers
    for (val = 0; val < ARRAY_SIZE(ecs_reset); val++)
    npe_cmd_write(npe, ecs_reset[val].reg, CMD_WR_ECS_REG,
    ecs_reset[val].val);
// clear the profile counter
    __raw_writel(CMD_CLR_PROFILE_CNT, &npe.regs.exec_status_cmd);
    __raw_writel(0, &npe.regs.exec_count);
    __raw_writel(0, &npe.regs.action_points[0]);
    __raw_writel(0, &npe.regs.action_points[1]);
    __raw_writel(0, &npe.regs.action_points[2]);
    __raw_writel(0, &npe.regs.action_points[3]);
    __raw_writel(0, &npe.regs.watch_count);
//
// We need to work on cached values here because the register
// will read inverted but needs to be written non-inverted.
//
    val = cpu_ixp4xx_features(npe.rmap);
// reset the NPE
    regmap_write(npe.rmap, IXP4XX_EXP_CNFG2, val & ~reset_bit);
// deassert reset
    regmap_write(npe.rmap, IXP4XX_EXP_CNFG2, val | reset_bit);
    for (i = 0; i < MAX_RETRIES; i++) {
    val = cpu_ixp4xx_features(npe.rmap);
    if (val & reset_bit)
    break;	/* NPE is back alive */
    udelay(1);
    }
    if (i == MAX_RETRIES)
    return -ETIMEDOUT;
    npe_stop(npe);
// restore NPE configuration bus Control Register - parity settings
    __raw_writel(ctl, &npe.regs.messaging_control);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn npe_send_message(npe: *mut npe, msg: *const c_void, what: *const c_char) -> c_int {
    int npe_send_message(struct npe *npe, const void *msg, const char *what)
    {
    const u32 *send = msg;
    let mut cycles: c_int = 0;
    debug_msg(npe, "Trying to send message %s [%08X:%08X]\n",
    what, send[0], send[1]);
    if (__raw_readl(&npe.regs.messaging_status) & MSGSTAT_IFNE) {
    debug_msg(npe, "NPE input FIFO not empty\n");
    return -EIO;
    }
    __raw_writel(send[0], &npe.regs.in_out_fifo);
    if (!(__raw_readl(&npe.regs.messaging_status) & MSGSTAT_IFNF)) {
    debug_msg(npe, "NPE input FIFO full\n");
    return -EIO;
    }
    __raw_writel(send[1], &npe.regs.in_out_fifo);
    while ((cycles < MAX_RETRIES) &&
    (__raw_readl(&npe.regs.messaging_status) & MSGSTAT_IFNE)) {
    udelay(1);
    cycles++;
    }
    if (cycles == MAX_RETRIES) {
    debug_msg(npe, "Timeout sending message\n");
    return -ETIMEDOUT;
    }

    debug_msg(npe, "Sending a message took %i cycles\n", cycles);

    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn npe_recv_message(npe: *mut npe, msg: *mut c_void, what: *const c_char) -> c_int {
    int npe_recv_message(struct npe *npe, void *msg, const char *what)
    {
    u32 *recv = msg;
    let mut cycles: c_int = 0, cnt = 0;
    debug_msg(npe, "Trying to receive message %s\n", what);
    while (cycles < MAX_RETRIES) {
    if (__raw_readl(&npe.regs.messaging_status) & MSGSTAT_OFNE) {
    recv[cnt++] = __raw_readl(&npe.regs.in_out_fifo);
    if (cnt == 2)
    break;
    } else {
    udelay(1);
    cycles++;
    }
    }
    switch(cnt) {
    case 1:
    debug_msg(npe, "Received [%08X]\n", recv[0]);
    break;
    case 2:
    debug_msg(npe, "Received [%08X:%08X]\n", recv[0], recv[1]);
    break;
    }
    if (cycles == MAX_RETRIES) {
    debug_msg(npe, "Timeout waiting for message\n");
    return -ETIMEDOUT;
    }

    debug_msg(npe, "Receiving a message took %i cycles\n", cycles);

    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn npe_send_recv_message(npe: *mut npe, msg: *mut c_void, what: *const c_char) -> c_int {
    int npe_send_recv_message(struct npe *npe, void *msg, const char *what)
    {
    int result;
    u32 *send = msg, recv[2];
    if ((result = npe_send_message(npe, msg, what)) != 0)
    return result;
    if ((result = npe_recv_message(npe, recv, what)) != 0)
    return result;
    if ((recv[0] != send[0]) || (recv[1] != send[1])) {
    debug_msg(npe, "Message %s: unexpected message received\n",
    what);
    return -EIO;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn npe_load_firmware(npe: *mut npe, name: *const c_char, dev: *mut device) -> c_int {
    int npe_load_firmware(struct npe *npe, const char *name, struct device *dev)
    {
    const struct firmware *fw_entry;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dl_block {
    pub type: u32,
    pub offset: u32,
    pub blk: *mut },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dl_image {
    pub magic: u32,
    pub id: u32,
    pub size: u32,
    union {
    pub data): DECLARE_FLEX_ARRAY(u32,,
    pub blocks): DECLARE_FLEX_ARRAY(struct dl_block,,
}

    } *image;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dl_codeblock {
    pub npe_addr: u32,
    pub size: u32,
    pub data: [u32; ],
    pub cb: *mut },
    pub table_end: int i, j, err, data_size, instr_size, blocks,,
    pub cmd: u32,
    if ((err = request_firmware(&fw_entry, name, dev)) != 0)
    pub err: return,
    pub -EINVAL: err =,
    if (fw_entry.size < sizeof(struct dl_image)) {
    pub file\n"): print_npe(KERN_ERR, npe, "incomplete firmware,
    pub err: goto,
    }
    pub dl_image*)fw_entry->data: *mut image = (struct,

    print_npe(KERN_DEBUG, npe, "firmware: %08X %08X %08X (0x%X bytes)\n",
    pub 4): *mut *mut image->magic, image->id, image->size, image->size,

    if (image.magic == swab32(FW_MAGIC)) { /* swapped file */
    pub swab32(image->id): image->id =,
    pub swab32(image->size): image->size =,
    } else if (image.magic != FW_MAGIC) {
    print_npe(KERN_ERR, npe, "bad firmware file magic: 0x%X\n",
    pub err: goto,
    }
    if ((image.size * 4 + sizeof(struct dl_image)) != fw_entry.size) {
    print_npe(KERN_ERR, npe,
    pub file\n"): "inconsistent size of firmware,
    pub err: goto,
    }
    if (((image.id >> 24) & 0xF /* NPE ID */) != npe.id) {
    pub mismatch\n"): print_npe(KERN_ERR, npe, "firmware file NPE ID,
    pub err: goto,
    }
    if (image.magic == swab32(FW_MAGIC))
    pub i++): for (i = 0; i < image->size;,
    pub swab32(image->data[i]): image->data[i] =,
    if (cpu_is_ixp42x() && ((image.id >> 28) & 0xF /* device ID */)) {
    print_npe(KERN_INFO, npe, "IXP43x/IXP46x firmware ignored on "
    pub err: goto,
    }
    if (npe_running(npe)) {
    print_npe(KERN_INFO, npe, "unable to load firmware, NPE is "
    pub running\n"): "already,
    pub -EBUSY: err =,
    pub err: goto,
    }

    print_npe(KERN_INFO, npe, "firmware functionality 0x%X, "
    "revision 0x%X:%X\n", (image.id >> 16) & 0xFF,
    pub 0xFF): (image->id >> 8) & 0xFF, image->id &,
    if (cpu_is_ixp42x()) {
    if (!npe.id)
    pub NPE_A_42X_INSTR_SIZE: instr_size =,
    else
    pub NPE_B_AND_C_42X_INSTR_SIZE: instr_size =,
    pub NPE_42X_DATA_SIZE: data_size =,
    } else {
    pub NPE_46X_INSTR_SIZE: instr_size =,
    pub NPE_46X_DATA_SIZE: data_size =,
    }
    pub image->size: *mut *mut for (blocks = 0; blocks  sizeof(struct dl_block) / 4 <,
    blocks++)
    if (image.blocks[blocks].type == FW_BLOCK_TYPE_EOF)
    if (blocks * sizeof(struct dl_block) / 4 >= image.size) {
    print_npe(KERN_INFO, npe, "firmware EOF block marker not "
    pub err: goto,
    }

    pub blocks): print_npe(KERN_DEBUG, npe, "%i firmware blocks found\n",,

    pub /: *mut *mut *mut table_end = blocks  sizeof(struct dl_block) / 4 + 1 / EOF marker,
    pub {: for (i = 0, blk = image->blocks; i < blocks; i++, blk++),
    if (blk.offset > image.size - sizeof(struct dl_codeblock) / 4
    || blk.offset < table_end) {
    print_npe(KERN_INFO, npe, "invalid offset 0x%X of "
    pub i): "firmware block #%i\n", blk->offset,,
    pub err: goto,
    }
    pub dl_codeblock*)&image->data[blk->offset]: *mut cb = (struct,
    if (blk.type == FW_BLOCK_TYPE_INSTR) {
    if (cb.npe_addr + cb.size > instr_size)
    pub too_big: goto,
    pub CMD_WR_INS_MEM: cmd =,
    } else if (blk.type == FW_BLOCK_TYPE_DATA) {
    if (cb.npe_addr + cb.size > data_size)
    pub too_big: goto,
    pub CMD_WR_DATA_MEM: cmd =,
    } else {
    print_npe(KERN_INFO, npe, "invalid firmware block #%i "
    pub blk->type): "type 0x%X\n", i,,
    pub err: goto,
    }
    if (blk.offset + sizeof(*cb) / 4 + cb.size > image.size) {
    print_npe(KERN_INFO, npe, "firmware block #%i doesn't "
    "fit in firmware image: type %c, start 0x%X,"
    " length 0x%X\n", i,
    blk.type == FW_BLOCK_TYPE_INSTR ? 'I' : 'D',
    pub cb->size): cb->npe_addr,,
    pub err: goto,
    }
    pub j++): for (j = 0; j < cb->size;,
    pub cb->data[j]): npe_cmd_write(npe, cb->npe_addr + j, cmd,,
    }
    if (!npe_running(npe))
    pub start\n"): print_npe(KERN_ERR, npe, "unable to,
    pub 0: return,
    too_big:
    print_npe(KERN_INFO, npe, "firmware block #%i doesn't fit in NPE "
    "memory: type %c, start 0x%X, length 0x%X\n", i,
    blk.type == FW_BLOCK_TYPE_INSTR ? 'I' : 'D',
    pub cb->size): cb->npe_addr,,
    err:
    pub err: return,
    }
    struct npe *npe_request(unsigned id)
    {
    if (id < NPE_COUNT)
    if (npe_tab[id].valid)
    if (try_module_get(THIS_MODULE))
    pub &npe_tab[id]: return,
    pub NULL: return,
    }
#[no_mangle]
pub unsafe extern "C" fn npe_release(npe: *mut npe) {
    void npe_release(struct npe *npe)
    {
    }
#[no_mangle]
unsafe extern "C" fn ixp4xx_npe_probe(pdev: *mut platform_device) -> c_int {
    static int ixp4xx_npe_probe(struct platform_device *pdev)
    {
    pub 0: int i, found =,
    pub &pdev->dev: *mut *mut device dev =,
    pub dev->of_node: *mut *mut device_node np =,
    pub res: *mut resource,
    pub rmap: *mut regmap,
    pub val: u32,
// This system has only one syscon, so fetch it
    pub syscon_regmap_lookup_by_compatible("syscon"): rmap =,
    if (IS_ERR(rmap))
    return dev_err_probe(dev, PTR_ERR(rmap),
    pub syscon\n"): "failed to look up,
    pub {: for (i = 0; i < NPE_COUNT; i++),
    pub &npe_tab[i]: *mut *mut npe npe =,
    pub i): res = platform_get_resource(pdev, IORESOURCE_MEM,,
    if (!res)
    pub -ENODEV: return,
    pub cpu_ixp4xx_features(rmap): val =,
    if (!(val & (IXP4XX_FEATURE_RESET_NPEA << i))) {
    dev_info(dev, "NPE%d at %pR not available\n",
    pub res): i,,
    pub /: *mut *mut continue; / NPE already disabled or not present,
    }
    pub res): npe->regs = devm_ioremap_resource(dev,,
    if (IS_ERR(npe.regs))
    pub PTR_ERR(npe->regs): return,
    pub rmap: npe->rmap =,
    if (npe_reset(npe)) {
    dev_info(dev, "NPE%d at %pR does not reset\n",
    pub res): i,,
    }
    pub 1: npe->valid =,
    pub res): dev_info(dev, "NPE%d at %pR registered\n", i,,
    }
    if (!found)
    pub -ENODEV: return,
// Spawn crypto subdevice if using device tree
    if (IS_ENABLED(CONFIG_OF) && np)
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn ixp4xx_npe_remove(pdev: *mut platform_device) {
    static void ixp4xx_npe_remove(struct platform_device *pdev)
    {
    pub i: c_int,
    pub i++): for (i = 0; i < NPE_COUNT;,
    if (npe_tab[i].regs) {
    }
    }
    static const struct of_device_id ixp4xx_npe_of_match[] = {
    {
    .compatible = "intel,ixp4xx-network-processing-engine",
    },
    {},
}

    MODULE_DEVICE_TABLE(of, ixp4xx_npe_of_match);
    static struct platform_driver ixp4xx_npe_driver = {
    .driver = {
    .name           = "ixp4xx-npe",
    .of_match_table = ixp4xx_npe_of_match,
    },
    .probe = ixp4xx_npe_probe,
    .remove = ixp4xx_npe_remove,
    };
    module_platform_driver(ixp4xx_npe_driver);
    MODULE_AUTHOR("Krzysztof Halasa");
    MODULE_DESCRIPTION("Intel IXP4xx Network Processor Engine driver");
    MODULE_LICENSE("GPL v2");
    MODULE_FIRMWARE(NPE_A_FIRMWARE);
    MODULE_FIRMWARE(NPE_B_FIRMWARE);
    MODULE_FIRMWARE(NPE_C_FIRMWARE);
    EXPORT_SYMBOL(npe_names);
    EXPORT_SYMBOL(npe_running);
    EXPORT_SYMBOL(npe_request);
    EXPORT_SYMBOL(npe_release);
    EXPORT_SYMBOL(npe_load_firmware);
    EXPORT_SYMBOL(npe_send_message);
    EXPORT_SYMBOL(npe_recv_message);
    EXPORT_SYMBOL(npe_send_recv_message);
