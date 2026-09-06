//! Automatically rewritten from C to Rust
//! Source: drivers/i3c/master/renesas-i3c.c
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
// Renesas I3C Controller driver
// Copyright (C) 2023-25 Renesas Electronics Corp.
//
// TODO: IBI support, HotJoin support, Target support
//

pub const PRTS: c_uint = 0x00;

pub const BCTL: c_uint = 0x14;

pub const MSDVAD: c_uint = 0x18;

pub const RSTCTL: c_uint = 0x20;

pub const INST: c_uint = 0x30;
pub const IBINCTL: c_uint = 0x58;

pub const SVCTL: c_uint = 0x64;
pub const REFCKCTL: c_uint = 0x70;

pub const STDBR: c_uint = 0x74;

pub const EXTBR: c_uint = 0x78;

pub const BFRECDT: c_uint = 0x7c;

pub const BAVLCDT: c_uint = 0x80;

pub const BIDLCDT: c_uint = 0x84;

pub const ACKCTL: c_uint = 0xa0;

pub const SCSTRCTL: c_uint = 0xa4;

pub const SCSTLCTL: c_uint = 0xb0;
pub const CNDCTL: c_uint = 0x140;

pub const NCMDQP: c_uint = 0x150 /* Normal Command Queue */;

pub const NCMDQP_IMMED_XFER: c_uint = 0x01;
pub const NCMDQP_ADDR_ASSGN: c_uint = 0x02;

pub const NRSPQP: c_uint = 0x154 /* Normal Respone Queue */;
pub const NRSPQP_NO_ERROR: c_int = 0;
pub const NRSPQP_ERROR_CRC: c_int = 1;
pub const NRSPQP_ERROR_PARITY: c_int = 2;
pub const NRSPQP_ERROR_FRAME: c_int = 3;
pub const NRSPQP_ERROR_IBA_NACK: c_int = 4;
pub const NRSPQP_ERROR_ADDRESS_NACK: c_int = 5;
pub const NRSPQP_ERROR_OVER_UNDER_FLOW: c_int = 6;
pub const NRSPQP_ERROR_TRANSF_ABORT: c_int = 8;
pub const NRSPQP_ERROR_I2C_W_NACK_ERR: c_int = 9;
pub const NRSPQP_ERROR_UNSUPPORTED: c_int = 10;

pub const NTDTBP0: c_uint = 0x158 /* Normal Transfer Data Buffer */;
pub const NTDTBP0_DEPTH: c_int = 16;
pub const NQTHCTL: c_uint = 0x190;

pub const NTBTHCTL0: c_uint = 0x194;
pub const NRQTHCTL: c_uint = 0x1c0;
pub const BST: c_uint = 0x1d0;

pub const BSTE: c_uint = 0x1d4;

    BSTE_NACKDE | BSTE_TENDE |\
    BSTE_ALE | BSTE_TODE | BSTE_WUCNDDE)
pub const BIE: c_uint = 0x1d8;

pub const NTST: c_uint = 0x1e0;

pub const NTSTE: c_uint = 0x1e4;

    NTSTE_IBIQEFE | NTSTE_CMDQEE |\
    NTSTE_RSPQFE | NTSTE_TABTE |\
    NTSTE_TEE | NTSTE_RSQFE)
pub const NTIE: c_uint = 0x1e8;

pub const BCST: c_uint = 0x210;

pub const NDBSTLV0: c_uint = 0x398;

pub const RENESAS_I3C_MAX_DEVS: c_int = 8;

    enum i3c_internal_state {
    I3C_INTERNAL_STATE_DISABLED,
    I3C_INTERNAL_STATE_CONTROLLER_IDLE,
    I3C_INTERNAL_STATE_CONTROLLER_ENTDAA,
    I3C_INTERNAL_STATE_CONTROLLER_SETDASA,
    I3C_INTERNAL_STATE_CONTROLLER_WRITE,
    I3C_INTERNAL_STATE_CONTROLLER_READ,
    I3C_INTERNAL_STATE_CONTROLLER_COMMAND_WRITE,
    I3C_INTERNAL_STATE_CONTROLLER_COMMAND_READ,
    };
    enum renesas_i3c_event {
    I3C_COMMAND_ADDRESS_ASSIGNMENT,
    I3C_WRITE,
    I3C_READ,
    I3C_COMMAND_WRITE,
    I3C_COMMAND_READ,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct renesas_i3c_cmd {
    pub tx_buf: *const c_void,
    pub rx_buf: *mut c_void,
// i2c xfer
    pub i2c_buf: *mut u8,
    pub msg: *const i2c_msg,
    pub i2c_bytes_left: c_int,
    pub i2c_is_last: c_int,
    pub cmd0: u32,
    pub len: u32,
    pub tx_count: u32,
    pub rx_count: u32,
    pub err: u32,
    pub rnw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct renesas_i3c_xfer {
    pub node: list_head,
    pub comp: completion,
    pub ret: c_int,
    pub is_i2c_xfer: bool,
    pub ncmds: c_uint,
    pub __counted_by(ncmds): renesas_i3c_cmd cmds[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct renesas_i3c_xferqueue {
    pub list: list_head,
    pub cur: *mut renesas_i3c_xfer,
// Lock for accessing the xfer queue
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct renesas_i3c {
    pub regs: *mut void __iomem,
    pub tclk: *mut clk,
    pub presetn: *mut reset_control,
    pub tresetn: *mut reset_control,
    pub dev: *mut device,
    pub xferqueue: renesas_i3c_xferqueue,
    pub base: i3c_master_controller,
    pub addrs: [u8; RENESAS_I3C_MAX_DEVS],
    pub rate: c_ulong,
    pub internal_state: enum i3c_internal_state,
    pub free_pos: u32,
    pub dyn_addr: u32,
    pub i2c_STDBR: u32,
    pub i3c_STDBR: u32,
    pub extbr: u32,
    pub maxdevs: u16,
    pub refclk_div: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct renesas_i3c_i2c_dev_data {
    pub index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct renesas_i3c_irq_desc {
    pub name: *const c_char,
    pub isr: irq_handler_t,
    pub desc: *const c_char,
}

#[no_mangle]
pub unsafe extern "C" fn renesas_i3c_reg_update(reg: *mut void __iomem, mask: u32, val: u32) {
    static inline void renesas_i3c_reg_update(void __iomem *reg, u32 mask, u32 val)
    {
    let mut data: u32 = readl(reg);
    data &= ~mask;
    data |= (val & mask);
    writel(data, reg);
    }
#[no_mangle]
pub unsafe extern "C" fn renesas_readl(base: *mut void __iomem, reg: u32) -> u32 {
    static inline u32 renesas_readl(void __iomem *base, u32 reg)
    {
    return readl(base + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn renesas_writel(base: *mut void __iomem, reg: u32, val: u32) {
    static inline void renesas_writel(void __iomem *base, u32 reg, u32 val)
    {
    writel(val, base + reg);
    }
#[no_mangle]
unsafe extern "C" fn renesas_set_bit(base: *mut void __iomem, reg: u32, val: u32) {
    static void renesas_set_bit(void __iomem *base, u32 reg, u32 val)
    {
    renesas_i3c_reg_update(base + reg, val, val);
    }
#[no_mangle]
unsafe extern "C" fn renesas_clear_bit(base: *mut void __iomem, reg: u32, val: u32) {
    static void renesas_clear_bit(void __iomem *base, u32 reg, u32 val)
    {
    renesas_i3c_reg_update(base + reg, val, 0);
    }
    static inline struct renesas_i3c *to_renesas_i3c(struct i3c_master_controller *m)
    {
    return container_of(m, struct renesas_i3c, base);
    }
#[no_mangle]
pub unsafe extern "C" fn datbas_dvdyad_with_parity(addr: u8) -> u32 {
    static inline u32 datbas_dvdyad_with_parity(u8 addr)
    {
    return DATBAS_DVDYAD(addr | (parity8(addr) ? 0 : BIT(7)));
    }
#[no_mangle]
unsafe extern "C" fn renesas_i3c_get_free_pos(i3c: *mut renesas_i3c) -> c_int {
    static int renesas_i3c_get_free_pos(struct renesas_i3c *i3c)
    {
    if (!(i3c.free_pos & GENMASK(i3c.maxdevs - 1, 0)))
    return -ENOSPC;
    return ffs(i3c.free_pos) - 1;
    }
#[no_mangle]
unsafe extern "C" fn renesas_i3c_get_addr_pos(i3c: *mut renesas_i3c, addr: u8) -> c_int {
    static int renesas_i3c_get_addr_pos(struct renesas_i3c *i3c, u8 addr)
    {
    int pos;
    for (pos = 0; pos < i3c.maxdevs; pos++) {
    if (addr == i3c.addrs[pos])
    return pos;
    }
    return -EINVAL;
    }
    static struct renesas_i3c_xfer *renesas_i3c_alloc_xfer(struct renesas_i3c *i3c,
    unsigned int ncmds)
    {
    struct renesas_i3c_xfer *xfer;
    xfer = kzalloc_flex(*xfer, cmds, ncmds);
    if (!xfer)
    return core::ptr::null_mut();
    INIT_LIST_HEAD(&xfer.node);
    xfer.ncmds = ncmds;
    xfer.ret = -ETIMEDOUT;
    return xfer;
    }
#[no_mangle]
unsafe extern "C" fn renesas_i3c_start_xfer_locked(i3c: *mut renesas_i3c) {
    static void renesas_i3c_start_xfer_locked(struct renesas_i3c *i3c)
    {
    struct renesas_i3c_xfer *xfer = i3c.xferqueue.cur;
    struct renesas_i3c_cmd *cmd;
    u32 cmd1;
    if (!xfer)
    return;
    cmd = xfer.cmds;
    switch (i3c.internal_state) {
    case I3C_INTERNAL_STATE_CONTROLLER_ENTDAA:
    case I3C_INTERNAL_STATE_CONTROLLER_SETDASA:
    renesas_set_bit(i3c.regs, NTIE, NTIE_RSPQFIE);
    renesas_writel(i3c.regs, NCMDQP, cmd.cmd0);
    renesas_writel(i3c.regs, NCMDQP, 0);
    break;
    case I3C_INTERNAL_STATE_CONTROLLER_WRITE:
    case I3C_INTERNAL_STATE_CONTROLLER_COMMAND_WRITE:
    renesas_set_bit(i3c.regs, NTIE, NTIE_RSPQFIE);
    if (cmd.len <= 4) {
    cmd.cmd0 |= NCMDQP_CMD_ATTR(NCMDQP_IMMED_XFER);
    cmd.cmd0 |= NCMDQP_BYTE_CNT(cmd.len);
    cmd.tx_count = cmd.len;
    cmd1 = cmd.len == 0 ? 0 : *(u32 *)cmd.tx_buf;
    } else {
    cmd1 = NCMDQP_DATA_LENGTH(cmd.len);
    }
    renesas_writel(i3c.regs, NCMDQP, cmd.cmd0);
    renesas_writel(i3c.regs, NCMDQP, cmd1);
    break;
    case I3C_INTERNAL_STATE_CONTROLLER_READ:
    case I3C_INTERNAL_STATE_CONTROLLER_COMMAND_READ:
    renesas_set_bit(i3c.regs, NTIE, NTIE_RDBFIE0);
    cmd1 = NCMDQP_DATA_LENGTH(cmd.len);
    renesas_writel(i3c.regs, NCMDQP, cmd.cmd0);
    renesas_writel(i3c.regs, NCMDQP, cmd1);
    break;
    default:
    break;
    }
// Clear the command queue empty flag
    renesas_clear_bit(i3c.regs, NTST, NTST_CMDQEF);
    }
    static void renesas_i3c_dequeue_xfer_locked(struct renesas_i3c *i3c,
    struct renesas_i3c_xfer *xfer)
    {
    if (i3c.xferqueue.cur == xfer)
    i3c.xferqueue.cur = core::ptr::null_mut();
    else
    list_del_init(&xfer.node);
    }
#[no_mangle]
unsafe extern "C" fn renesas_i3c_dequeue_xfer(i3c: *mut renesas_i3c, xfer: *mut renesas_i3c_xfer) {
    static void renesas_i3c_dequeue_xfer(struct renesas_i3c *i3c, struct renesas_i3c_xfer *xfer)
    {
    scoped_guard(spinlock_irqsave, &i3c.xferqueue.lock)
    renesas_i3c_dequeue_xfer_locked(i3c, xfer);
    }
#[no_mangle]
unsafe extern "C" fn renesas_i3c_enqueue_xfer(i3c: *mut renesas_i3c, xfer: *mut renesas_i3c_xfer) {
    static void renesas_i3c_enqueue_xfer(struct renesas_i3c *i3c, struct renesas_i3c_xfer *xfer)
    {
    reinit_completion(&xfer.comp);
    scoped_guard(spinlock_irqsave, &i3c.xferqueue.lock) {
    if (i3c.xferqueue.cur) {
    list_add_tail(&xfer.node, &i3c.xferqueue.list);
    } else {
    i3c.xferqueue.cur = xfer;
    if (!xfer.is_i2c_xfer)
    renesas_i3c_start_xfer_locked(i3c);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn renesas_i3c_irqs_mask_and_clear_locked(i3c: *mut renesas_i3c) {
    static void renesas_i3c_irqs_mask_and_clear_locked(struct renesas_i3c *i3c)
    {
// Disable all the interrupts.
    renesas_writel(i3c.regs, BIE, 0);
    renesas_writel(i3c.regs, NTIE, 0);
// Clear normal transfer status flags.
    renesas_writel(i3c.regs, NTST, 0);
// Clear bus status flags.
    renesas_writel(i3c.regs, BST, 0);
// Read back registers to confirm writes have fully propagated.
    renesas_readl(i3c.regs, BST);
    }
#[no_mangle]
unsafe extern "C" fn renesas_i3c_irqs_mask_and_clear(i3c: *mut renesas_i3c) {
    static void renesas_i3c_irqs_mask_and_clear(struct renesas_i3c *i3c)
    {
    guard(spinlock_irqsave)(&i3c.xferqueue.lock);
    renesas_i3c_irqs_mask_and_clear_locked(i3c);
    }
#[no_mangle]
unsafe extern "C" fn renesas_i3c_wait_xfer(i3c: *mut renesas_i3c, xfer: *mut renesas_i3c_xfer) -> c_ulong {
    static unsigned long renesas_i3c_wait_xfer(struct renesas_i3c *i3c, struct renesas_i3c_xfer *xfer)
    {
    unsigned long time_left;
    renesas_i3c_enqueue_xfer(i3c, xfer);
    time_left = wait_for_completion_timeout(&xfer.comp, msecs_to_jiffies(1000));
    if (!time_left)
    renesas_i3c_dequeue_xfer(i3c, xfer);
    return time_left;
    }
#[no_mangle]
unsafe extern "C" fn renesas_i3c_set_prts(i3c: *mut renesas_i3c, val: u32) {
    static void renesas_i3c_set_prts(struct renesas_i3c *i3c, u32 val)
    {
// Required sequence according to tnrza0140ae
    renesas_set_bit(i3c.regs, RSTCTL, RSTCTL_INTLRST);
    renesas_writel(i3c.regs, PRTS, val);
    renesas_clear_bit(i3c.regs, RSTCTL, RSTCTL_INTLRST);
    }
#[no_mangle]
unsafe extern "C" fn renesas_i3c_bus_enable(m: *mut i3c_master_controller, i3c_mode: bool) {
    static void renesas_i3c_bus_enable(struct i3c_master_controller *m, bool i3c_mode)
    {
    struct renesas_i3c *i3c = to_renesas_i3c(m);
// Setup either I3C or I2C protocol
    if (i3c_mode) {
    renesas_i3c_set_prts(i3c, 0);
// Revisit: INCBA handling, especially after I2C transfers
    renesas_set_bit(i3c.regs, BCTL, BCTL_HJACKCTL | BCTL_INCBA);
    renesas_set_bit(i3c.regs, MSDVAD, MSDVAD_MDYADV);
    renesas_writel(i3c.regs, STDBR, i3c.i3c_STDBR);
    } else {
    renesas_i3c_set_prts(i3c, PRTS_PRTMD);
    renesas_writel(i3c.regs, STDBR, i3c.i2c_STDBR);
    }
// Enable I3C bus
    renesas_set_bit(i3c.regs, BCTL, BCTL_BUSE);
    }
#[no_mangle]
unsafe extern "C" fn renesas_i3c_reset(i3c: *mut renesas_i3c) -> c_int {
    static int renesas_i3c_reset(struct renesas_i3c *i3c)
    {
    u32 val;
    int ret;
    PM_RUNTIME_ACQUIRE_IF_ENABLED_AUTOSUSPEND(i3c.dev, pm);
    ret = PM_RUNTIME_ACQUIRE_ERR(&pm);
    if (ret)
    return ret;
    renesas_writel(i3c.regs, BCTL, 0);
    renesas_set_bit(i3c.regs, RSTCTL, RSTCTL_RI3CRST);
    return read_poll_timeout(renesas_readl, val, !(val & RSTCTL_RI3CRST),
    0, 1000, false, i3c.regs, RSTCTL);
    }
#[no_mangle]
unsafe extern "C" fn renesas_i3c_hw_init(i3c: *mut renesas_i3c) {
    static void renesas_i3c_hw_init(struct renesas_i3c *i3c)
    {
    u32 val;
// Disable Slave Mode
    renesas_writel(i3c.regs, SVCTL, 0);
// Initialize Queue/Buffer threshold
    renesas_writel(i3c.regs, NQTHCTL, NQTHCTL_IBIDSSZ(6) |
    NQTHCTL_CMDQTH(1));
// The only supported configuration is two entries
    renesas_writel(i3c.regs, NTBTHCTL0, 0);
// Interrupt when there is one entry in the queue
    renesas_writel(i3c.regs, NRQTHCTL, 0);
// Enable all Bus/Transfer Status Flags
    renesas_writel(i3c.regs, BSTE, BSTE_ALL_FLAG);
    renesas_writel(i3c.regs, NTSTE, NTSTE_ALL_FLAG);
// Interrupt enable settings
    renesas_writel(i3c.regs, BIE, BIE_NACKDIE | BIE_TENDIE);
    renesas_writel(i3c.regs, NTIE, 0);
// Clear Status register
    renesas_writel(i3c.regs, NTST, 0);
    renesas_writel(i3c.regs, INST, 0);
    renesas_writel(i3c.regs, BST, 0);
// Hot-Join Acknowlege setting.
    renesas_set_bit(i3c.regs, BCTL, BCTL_HJACKCTL);
    renesas_writel(i3c.regs, IBINCTL, IBINCTL_NRHJCTL | IBINCTL_NRMRCTL |
    IBINCTL_NRSIRCTL);
    renesas_writel(i3c.regs, SCSTLCTL, 0);
    renesas_set_bit(i3c.regs, SCSTRCTL, SCSTRCTL_ACKTWE);
// Bus condition timing
    val = DIV_ROUND_UP(I3C_BUS_TBUF_MIXED_FM_MIN_NS,
    NSEC_PER_SEC / i3c.rate);
    renesas_writel(i3c.regs, BFRECDT, BFRECDT_FRECYC(val));
    val = DIV_ROUND_UP(I3C_BUS_TAVAL_MIN_NS,
    NSEC_PER_SEC / i3c.rate);
    renesas_writel(i3c.regs, BAVLCDT, BAVLCDT_AVLCYC(val));
    val = DIV_ROUND_UP(I3C_BUS_TIDLE_MIN_NS,
    NSEC_PER_SEC / i3c.rate);
    renesas_writel(i3c.regs, BIDLCDT, BIDLCDT_IDLCYC(val));
    }
#[no_mangle]
unsafe extern "C" fn renesas_i3c_bus_init(m: *mut i3c_master_controller) -> c_int {
    static int renesas_i3c_bus_init(struct i3c_master_controller *m)
    {
    struct renesas_i3c *i3c = to_renesas_i3c(m);
    struct i3c_bus *bus = i3c_master_get_bus(m);
    let mut info: i3c_device_info = {};
    struct i2c_timings t;
    u32 double_SBR;
    int cks, pp_high_ticks, pp_low_ticks, i3c_total_ticks;
    int od_high_ticks, od_low_ticks, i2c_total_ticks;
    int ret;
    i3c.rate = clk_get_rate(i3c.tclk);
    if (!i3c.rate)
    return -EINVAL;
    i2c_total_ticks = DIV_ROUND_UP(i3c.rate, bus.scl_rate.i2c);
    i3c_total_ticks = DIV_ROUND_UP(i3c.rate, bus.scl_rate.i3c);
    i2c_parse_fw_timings(&m.dev, &t, true);
    for (cks = 0; cks < 7; cks++) {
// SCL low-period calculation in Open-drain mode
    od_low_ticks = ((i2c_total_ticks * 6) / 10);
// SCL clock calculation in Push-Pull mode
    if (bus.mode == I3C_BUS_MODE_PURE)
    pp_high_ticks = ((i3c_total_ticks * 5) / 10);
    else
    pp_high_ticks = DIV_ROUND_UP(I3C_BUS_THIGH_MIXED_MAX_NS,
    NSEC_PER_SEC / i3c.rate);
    pp_low_ticks = i3c_total_ticks - pp_high_ticks;
    if ((od_low_ticks / 2) <= 0xFF && pp_low_ticks < 0x3F)
    break;
    i2c_total_ticks /= 2;
    i3c_total_ticks /= 2;
    i3c.rate /= 2;
    }
// SCL clock period calculation in Open-drain mode
    if ((od_low_ticks / 2) > 0xFF || pp_low_ticks > 0x3F) {
    dev_err(&m.dev, "invalid speed (i2c-scl = %lu Hz, i3c-scl = %lu Hz). Too slow.\n",
    (unsigned long)bus.scl_rate.i2c, (unsigned long)bus.scl_rate.i3c);
    return -EINVAL;
    }
// SCL high-period calculation in Open-drain mode
    od_high_ticks = i2c_total_ticks - od_low_ticks;
// Standard Bit Rate setting
    double_SBR = od_low_ticks > 0xFF ? 1 : 0;
    i3c.i3c_STDBR = (double_SBR ? STDBR_DSBRPO : 0) |
    STDBR_SBRLO(double_SBR, od_low_ticks) |
    STDBR_SBRHO(double_SBR, od_high_ticks) |
    STDBR_SBRLP(pp_low_ticks) |
    STDBR_SBRHP(pp_high_ticks);
    od_low_ticks -= t.scl_fall_ns / (NSEC_PER_SEC / i3c.rate) + 1;
    od_high_ticks -= t.scl_rise_ns / (NSEC_PER_SEC / i3c.rate) + 1;
    i3c.i2c_STDBR = (double_SBR ? STDBR_DSBRPO : 0) |
    STDBR_SBRLO(double_SBR, od_low_ticks) |
    STDBR_SBRHO(double_SBR, od_high_ticks) |
    STDBR_SBRLP(pp_low_ticks) |
    STDBR_SBRHP(pp_high_ticks);
// Extended Bit Rate setting
    i3c.extbr = EXTBR_EBRLO(od_low_ticks) | EXTBR_EBRHO(od_high_ticks) |
    EXTBR_EBRLP(pp_low_ticks) | EXTBR_EBRHP(pp_high_ticks);
    ret = i3c_master_get_free_addr(m, 0);
    if (ret < 0)
    return ret;
    info.dyn_addr = ret;
    i3c.dyn_addr = ret;
    i3c.refclk_div = cks;
    ret = renesas_i3c_reset(i3c);
    if (ret)
    return ret;
    PM_RUNTIME_ACQUIRE_IF_ENABLED_AUTOSUSPEND(i3c.dev, pm);
    ret = PM_RUNTIME_ACQUIRE_ERR(&pm);
    if (ret)
    return ret;
    renesas_writel(i3c.regs, STDBR, i3c.i3c_STDBR);
    renesas_writel(i3c.regs, EXTBR, i3c.extbr);
    renesas_writel(i3c.regs, REFCKCTL, REFCKCTL_IREFCKS(cks));
    renesas_writel(i3c.regs, MSDVAD, MSDVAD_MDYAD(i3c.dyn_addr) | MSDVAD_MDYADV);
// I3C hw init
    renesas_i3c_hw_init(i3c);
    return i3c_master_set_info(&i3c.base, &info);
    }
#[no_mangle]
unsafe extern "C" fn renesas_i3c_bus_cleanup(m: *mut i3c_master_controller) {
    static void renesas_i3c_bus_cleanup(struct i3c_master_controller *m)
    {
    struct renesas_i3c *i3c = to_renesas_i3c(m);
    renesas_i3c_reset(i3c);
    }
#[no_mangle]
unsafe extern "C" fn renesas_i3c_daa(m: *mut i3c_master_controller) -> c_int {
    static int renesas_i3c_daa(struct i3c_master_controller *m)
    {
    struct renesas_i3c *i3c = to_renesas_i3c(m);
    struct renesas_i3c_cmd *cmd;
    unsigned long time_left;
    u32 olddevs, newdevs;
    let mut last_addr: u8 = 0, pos;
    int ret;
    struct renesas_i3c_xfer *xfer __free(kfree) = renesas_i3c_alloc_xfer(i3c, 1);
    if (!xfer)
    return -ENOMEM;
    init_completion(&xfer.comp);
    cmd = xfer.cmds;
    cmd.rx_count = i3c.maxdevs;
    PM_RUNTIME_ACQUIRE_IF_ENABLED_AUTOSUSPEND(i3c.dev, pm);
    ret = PM_RUNTIME_ACQUIRE_ERR(&pm);
    if (ret)
    return ret;
// Enable I3C bus.
    renesas_i3c_bus_enable(m, true);
    olddevs = ~(i3c.free_pos);
    i3c.internal_state = I3C_INTERNAL_STATE_CONTROLLER_ENTDAA;
// Setting DATBASn registers for target devices.
    for (pos = 0; pos < i3c.maxdevs; pos++) {
    if (olddevs & BIT(pos))
    continue;
    ret = i3c_master_get_free_addr(m, last_addr + 1);
    if (ret < 0)
    return -ENOSPC;
    i3c.addrs[pos] = ret;
    last_addr = ret;
    renesas_writel(i3c.regs, DATBAS(pos), datbas_dvdyad_with_parity(ret));
    }
    ret = renesas_i3c_get_free_pos(i3c);
    if (ret < 0)
    return ret;
//
// Setup the command descriptor to start the ENTDAA command
// and starting at the selected device index.
//
    cmd.cmd0 = NCMDQP_CMD_ATTR(NCMDQP_ADDR_ASSGN) | NCMDQP_ROC |
    NCMDQP_TID(I3C_COMMAND_ADDRESS_ASSIGNMENT) |
    NCMDQP_CMD(I3C_CCC_ENTDAA) | NCMDQP_DEV_INDEX(ret) |
    NCMDQP_DEV_COUNT(i3c.maxdevs - ret) | NCMDQP_TOC;
    time_left = renesas_i3c_wait_xfer(i3c, xfer);
    if (!time_left)
    renesas_i3c_irqs_mask_and_clear(i3c);
    if (cmd.rx_count >= i3c.maxdevs)
    newdevs = 0;
    else
    newdevs = GENMASK(i3c.maxdevs - cmd.rx_count - 1, 0);
    newdevs &= ~olddevs;
    for (pos = 0; pos < i3c.maxdevs; pos++) {
    if (newdevs & BIT(pos))
    i3c_master_add_i3c_dev_locked(m, i3c.addrs[pos]);
    }
    return 0;
    }
    static bool renesas_i3c_supports_ccc_cmd(struct i3c_master_controller *m,
    const struct i3c_ccc_cmd *cmd)
    {
    if (cmd.ndests > 1)
    return false;
    switch (cmd.id) {
    case I3C_CCC_ENEC(true):
    case I3C_CCC_ENEC(false):
    case I3C_CCC_DISEC(true):
    case I3C_CCC_DISEC(false):
    case I3C_CCC_ENTAS(0, true):
    case I3C_CCC_ENTAS(1, true):
    case I3C_CCC_ENTAS(2, true):
    case I3C_CCC_ENTAS(3, true):
    case I3C_CCC_ENTAS(0, false):
    case I3C_CCC_ENTAS(1, false):
    case I3C_CCC_ENTAS(2, false):
    case I3C_CCC_ENTAS(3, false):
    case I3C_CCC_RSTDAA(true):
    case I3C_CCC_RSTDAA(false):
    case I3C_CCC_ENTDAA:
    case I3C_CCC_DEFSLVS:
    case I3C_CCC_SETMWL(true):
    case I3C_CCC_SETMWL(false):
    case I3C_CCC_SETMRL(true):
    case I3C_CCC_SETMRL(false):
    case I3C_CCC_ENTTM:
    case I3C_CCC_SETDASA:
    case I3C_CCC_SETNEWDA:
    case I3C_CCC_GETMWL:
    case I3C_CCC_GETMRL:
    case I3C_CCC_GETPID:
    case I3C_CCC_GETBCR:
    case I3C_CCC_GETDCR:
    case I3C_CCC_GETSTATUS:
    case I3C_CCC_GETACCMST:
    case I3C_CCC_GETMXDS:
    return true;
    default:
    return false;
    }
    }
    static int renesas_i3c_send_ccc_cmd(struct i3c_master_controller *m,
    struct i3c_ccc_cmd *ccc)
    {
    struct renesas_i3c *i3c = to_renesas_i3c(m);
    struct renesas_i3c_cmd *cmd;
    unsigned long time_left;
    int ret, pos = 0;
    if (ccc.id & I3C_CCC_DIRECT) {
    pos = renesas_i3c_get_addr_pos(i3c, ccc.dests[0].addr);
    if (pos < 0)
    return pos;
    }
    struct renesas_i3c_xfer *xfer __free(kfree) = renesas_i3c_alloc_xfer(i3c, 1);
    if (!xfer)
    return -ENOMEM;
    init_completion(&xfer.comp);
    cmd = xfer.cmds;
    cmd.rnw = ccc.rnw;
    cmd.cmd0 = 0;
    PM_RUNTIME_ACQUIRE_IF_ENABLED_AUTOSUSPEND(i3c.dev, pm);
    ret = PM_RUNTIME_ACQUIRE_ERR(&pm);
    if (ret)
    return ret;
    renesas_i3c_bus_enable(m, true);
// Calculate the command descriptor.
    switch (ccc.id) {
    case I3C_CCC_SETDASA:
    renesas_writel(i3c.regs, DATBAS(pos),
    DATBAS_DVSTAD(ccc.dests[0].addr) |
    DATBAS_DVDYAD(*(u8 *)ccc.dests[0].payload.data >> 1));
    cmd.cmd0 = NCMDQP_CMD_ATTR(NCMDQP_ADDR_ASSGN) | NCMDQP_ROC |
    NCMDQP_TID(I3C_COMMAND_ADDRESS_ASSIGNMENT) |
    NCMDQP_CMD(I3C_CCC_SETDASA) | NCMDQP_DEV_INDEX(pos) |
    NCMDQP_DEV_COUNT(0) | NCMDQP_TOC;
    i3c.internal_state = I3C_INTERNAL_STATE_CONTROLLER_SETDASA;
    break;
    default:
// Calculate the command descriptor.
    cmd.cmd0 = NCMDQP_TID(I3C_COMMAND_WRITE) | NCMDQP_MODE(0) |
    NCMDQP_RNW(ccc.rnw) | NCMDQP_CMD(ccc.id) |
    NCMDQP_ROC | NCMDQP_TOC | NCMDQP_CP |
    NCMDQP_DEV_INDEX(pos);
    if (ccc.rnw) {
    cmd.rx_buf = ccc.dests[0].payload.data;
    cmd.len = ccc.dests[0].payload.len;
    cmd.rx_count = 0;
    i3c.internal_state = I3C_INTERNAL_STATE_CONTROLLER_COMMAND_READ;
    } else {
    cmd.tx_buf = ccc.dests[0].payload.data;
    cmd.len = ccc.dests[0].payload.len;
    cmd.tx_count = 0;
    i3c.internal_state = I3C_INTERNAL_STATE_CONTROLLER_COMMAND_WRITE;
    }
    }
    time_left = renesas_i3c_wait_xfer(i3c, xfer);
    if (!time_left)
    renesas_i3c_irqs_mask_and_clear(i3c);
    ret = xfer.ret;
    if (ret)
    ccc.err = I3C_ERROR_M2;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: ccc->rnw) -> else {
    else if (ccc.rnw)
    ccc.dests[0].payload.actual_len = cmd.rx_count;
    return ret;
    }
    static int renesas_i3c_i3c_xfers(struct i3c_dev_desc *dev, struct i3c_xfer *i3c_xfers,
    int i3c_nxfers, enum i3c_xfer_mode mode)
    {
    struct i3c_master_controller *m = i3c_dev_get_master(dev);
    struct renesas_i3c *i3c = to_renesas_i3c(m);
    struct renesas_i3c_i2c_dev_data *data = i3c_dev_get_master_data(dev);
    unsigned long time_left;
    let mut xfer_failed: bool = false;
    int i, ret;
    struct renesas_i3c_xfer *xfer __free(kfree) = renesas_i3c_alloc_xfer(i3c, 1);
    if (!xfer)
    return -ENOMEM;
    init_completion(&xfer.comp);
    PM_RUNTIME_ACQUIRE_IF_ENABLED_AUTOSUSPEND(i3c.dev, pm);
    ret = PM_RUNTIME_ACQUIRE_ERR(&pm);
    if (ret)
    return ret;
// Enable I3C bus.
    renesas_i3c_bus_enable(m, true);
    for (i = 0; i < i3c_nxfers; i++) {
    struct renesas_i3c_cmd *cmd = xfer.cmds;
// Calculate the Transfer Command Descriptor
    cmd.rnw = i3c_xfers[i].rnw;
    cmd.cmd0 = NCMDQP_DEV_INDEX(data.index) | NCMDQP_MODE(0) |
    NCMDQP_RNW(cmd.rnw) | NCMDQP_ROC | NCMDQP_TOC;
    if (i3c_xfers[i].rnw) {
    cmd.rx_count = 0;
    cmd.cmd0 |= NCMDQP_TID(I3C_READ);
    cmd.rx_buf = i3c_xfers[i].data.in;
    cmd.len = i3c_xfers[i].len;
    i3c.internal_state = I3C_INTERNAL_STATE_CONTROLLER_READ;
    } else {
    cmd.tx_count = 0;
    cmd.cmd0 |= NCMDQP_TID(I3C_WRITE);
    cmd.tx_buf = i3c_xfers[i].data.out;
    cmd.len = i3c_xfers[i].len;
    i3c.internal_state = I3C_INTERNAL_STATE_CONTROLLER_WRITE;
    }
    if (!i3c_xfers[i].rnw && i3c_xfers[i].len > 4) {
    i3c_writel_fifo(i3c.regs + NTDTBP0, cmd.tx_buf, cmd.len);
    if (cmd.len > NTDTBP0_DEPTH * sizeof(u32))
    renesas_set_bit(i3c.regs, NTIE, NTIE_TDBEIE0);
    }
    time_left = renesas_i3c_wait_xfer(i3c, xfer);
    if (!time_left)
    xfer_failed = true;
    }
    if (xfer_failed)
    renesas_i3c_irqs_mask_and_clear(i3c);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn renesas_i3c_attach_i3c_dev(dev: *mut i3c_dev_desc) -> c_int {
    static int renesas_i3c_attach_i3c_dev(struct i3c_dev_desc *dev)
    {
    struct i3c_master_controller *m = i3c_dev_get_master(dev);
    struct renesas_i3c *i3c = to_renesas_i3c(m);
    struct renesas_i3c_i2c_dev_data *data;
    int pos, ret;
    pos = renesas_i3c_get_free_pos(i3c);
    if (pos < 0)
    return pos;
    PM_RUNTIME_ACQUIRE_IF_ENABLED_AUTOSUSPEND(i3c.dev, pm);
    ret = PM_RUNTIME_ACQUIRE_ERR(&pm);
    if (ret)
    return ret;
    data = kzalloc_obj(*data);
    if (!data)
    return -ENOMEM;
    data.index = pos;
    i3c.addrs[pos] = dev.info.dyn_addr ? : dev.info.static_addr;
    i3c.free_pos &= ~BIT(pos);
    renesas_writel(i3c.regs, DATBAS(pos), DATBAS_DVSTAD(dev.info.static_addr) |
    datbas_dvdyad_with_parity(i3c.addrs[pos]));
    i3c_dev_set_master_data(dev, data);
    return 0;
    }
    static int renesas_i3c_reattach_i3c_dev(struct i3c_dev_desc *dev,
    u8 old_dyn_addr)
    {
    struct i3c_master_controller *m = i3c_dev_get_master(dev);
    struct renesas_i3c *i3c = to_renesas_i3c(m);
    struct renesas_i3c_i2c_dev_data *data = i3c_dev_get_master_data(dev);
    int pos, ret;
    PM_RUNTIME_ACQUIRE_IF_ENABLED_AUTOSUSPEND(i3c.dev, pm);
    ret = PM_RUNTIME_ACQUIRE_ERR(&pm);
    if (ret)
    return ret;
    pos = renesas_i3c_get_free_pos(i3c);
    if (data.index > pos && pos >= 0) {
    renesas_writel(i3c.regs, DATBAS(data.index), 0);
    i3c.addrs[data.index] = 0;
    i3c.free_pos |= BIT(data.index);
    data.index = pos;
    i3c.free_pos &= ~BIT(data.index);
    }
    i3c.addrs[data.index] = dev.info.dyn_addr ? dev.info.dyn_addr :
    dev.info.static_addr;
    renesas_writel(i3c.regs, DATBAS(data.index),
    DATBAS_DVSTAD(dev.info.static_addr) |
    datbas_dvdyad_with_parity(i3c.addrs[data.index]));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn renesas_i3c_detach_i3c_dev(dev: *mut i3c_dev_desc) {
    static void renesas_i3c_detach_i3c_dev(struct i3c_dev_desc *dev)
    {
    struct renesas_i3c_i2c_dev_data *data = i3c_dev_get_master_data(dev);
    struct i3c_master_controller *m = i3c_dev_get_master(dev);
    struct renesas_i3c *i3c = to_renesas_i3c(m);
    int ret;
    PM_RUNTIME_ACQUIRE_IF_ENABLED_AUTOSUSPEND(i3c.dev, pm);
    ret = PM_RUNTIME_ACQUIRE_ERR(&pm);
    if (!ret)
    renesas_writel(i3c.regs, DATBAS(data.index), 0);
    i3c_dev_set_master_data(dev, core::ptr::null_mut());
    i3c.addrs[data.index] = 0;
    i3c.free_pos |= BIT(data.index);
    kfree(data);
    }
    static int renesas_i3c_i2c_xfers(struct i2c_dev_desc *dev,
    struct i2c_msg *i2c_xfers,
    int i2c_nxfers)
    {
    struct i3c_master_controller *m = i2c_dev_get_master(dev);
    struct renesas_i3c *i3c = to_renesas_i3c(m);
    struct renesas_i3c_cmd *cmd;
    let mut start_bit: u8 = CNDCTL_STCND;
    unsigned long time_left;
    let mut xfer_failed: bool = false;
    int i, ret;
    if (!i2c_nxfers)
    return 0;
    struct renesas_i3c_xfer *xfer __free(kfree) = renesas_i3c_alloc_xfer(i3c, 1);
    if (!xfer)
    return -ENOMEM;
    init_completion(&xfer.comp);
    xfer.is_i2c_xfer = true;
    cmd = xfer.cmds;
    PM_RUNTIME_ACQUIRE_IF_ENABLED_AUTOSUSPEND(i3c.dev, pm);
    ret = PM_RUNTIME_ACQUIRE_ERR(&pm);
    if (ret)
    return ret;
    renesas_i3c_bus_enable(m, false);
    if (!(renesas_readl(i3c.regs, BCST) & BCST_BFREF)) {
    cmd.err = -EBUSY;
    return cmd.err;
    }
    renesas_writel(i3c.regs, BST, 0);
    renesas_i3c_enqueue_xfer(i3c, xfer);
    for (i = 0; i < i2c_nxfers; i++) {
    cmd.i2c_bytes_left = I2C_INIT_MSG;
    cmd.i2c_buf = i2c_xfers[i].buf;
    cmd.msg = &i2c_xfers[i];
    cmd.i2c_is_last = (i == i2c_nxfers - 1);
    renesas_set_bit(i3c.regs, BIE, BIE_NACKDIE);
    renesas_set_bit(i3c.regs, NTIE, NTIE_TDBEIE0);
    renesas_set_bit(i3c.regs, BIE, BIE_STCNDDIE);
// Issue Start condition
    renesas_set_bit(i3c.regs, CNDCTL, start_bit);
    renesas_set_bit(i3c.regs, NTSTE, NTSTE_TDBEE0);
    time_left = wait_for_completion_timeout(&xfer.comp, m.i2c.timeout);
    if (!time_left)
    xfer_failed = true;
    if (cmd.err)
    break;
    start_bit = CNDCTL_SRCND;
    }
    renesas_i3c_dequeue_xfer(i3c, xfer);
    if (xfer_failed)
    renesas_i3c_irqs_mask_and_clear(i3c);
    return cmd.err;
    }
#[no_mangle]
unsafe extern "C" fn renesas_i3c_attach_i2c_dev(dev: *mut i2c_dev_desc) -> c_int {
    static int renesas_i3c_attach_i2c_dev(struct i2c_dev_desc *dev)
    {
    struct i3c_master_controller *m = i2c_dev_get_master(dev);
    struct renesas_i3c *i3c = to_renesas_i3c(m);
    struct renesas_i3c_i2c_dev_data *data;
    int pos;
    pos = renesas_i3c_get_free_pos(i3c);
    if (pos < 0)
    return pos;
    data = kzalloc_obj(*data);
    if (!data)
    return -ENOMEM;
    data.index = pos;
    i3c.addrs[pos] = dev.addr;
    i3c.free_pos &= ~BIT(pos);
    i2c_dev_set_master_data(dev, data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn renesas_i3c_detach_i2c_dev(dev: *mut i2c_dev_desc) {
    static void renesas_i3c_detach_i2c_dev(struct i2c_dev_desc *dev)
    {
    struct renesas_i3c_i2c_dev_data *data = i2c_dev_get_master_data(dev);
    struct i3c_master_controller *m = i2c_dev_get_master(dev);
    struct renesas_i3c *i3c = to_renesas_i3c(m);
    i2c_dev_set_master_data(dev, core::ptr::null_mut());
    i3c.addrs[data.index] = 0;
    i3c.free_pos |= BIT(data.index);
    kfree(data);
    }
#[no_mangle]
unsafe extern "C" fn renesas_i3c_tx_isr(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t renesas_i3c_tx_isr(int irq, void *data)
    {
    struct renesas_i3c *i3c = data;
    struct renesas_i3c_xfer *xfer;
    struct renesas_i3c_cmd *cmd;
    u8 val;
    scoped_guard(spinlock, &i3c.xferqueue.lock) {
    xfer = i3c.xferqueue.cur;
    if (!xfer) {
    renesas_i3c_irqs_mask_and_clear_locked(i3c);
    return IRQ_HANDLED;
    }
    cmd = xfer.cmds;
    if (xfer.is_i2c_xfer) {
    if (!cmd.i2c_bytes_left)
    return IRQ_NONE;
    if (cmd.i2c_bytes_left != I2C_INIT_MSG) {
    val = *cmd.i2c_buf;
    cmd.i2c_buf++;
    cmd.i2c_bytes_left--;
    renesas_writel(i3c.regs, NTDTBP0, val);
    }
    if (cmd.i2c_bytes_left == 0) {
    renesas_clear_bit(i3c.regs, NTIE, NTIE_TDBEIE0);
    renesas_set_bit(i3c.regs, BIE, BIE_TENDIE);
    }
// Clear the Transmit Buffer Empty status flag.
    renesas_clear_bit(i3c.regs, NTST, NTST_TDBEF0);
    } else {
    i3c_writel_fifo(i3c.regs + NTDTBP0, cmd.tx_buf, cmd.len);
    }
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn renesas_i3c_resp_isr(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t renesas_i3c_resp_isr(int irq, void *data)
    {
    struct renesas_i3c *i3c = data;
    struct renesas_i3c_xfer *xfer;
    struct renesas_i3c_cmd *cmd;
    let mut resp_descriptor: u32 = renesas_readl(i3c.regs, NRSPQP);
    let mut bytes_remaining: u32 = 0;
    u32 ntst, data_len;
    let mut ret: c_int = 0;
    scoped_guard(spinlock, &i3c.xferqueue.lock) {
    xfer = i3c.xferqueue.cur;
    if (!xfer) {
    renesas_i3c_irqs_mask_and_clear_locked(i3c);
    return IRQ_HANDLED;
    }
    cmd = xfer.cmds;
// Clear the Respone Queue Full status flag
    renesas_clear_bit(i3c.regs, NTST, NTST_RSPQFF);
    data_len = NRSPQP_DATA_LEN(resp_descriptor);
    switch (i3c.internal_state) {
    case I3C_INTERNAL_STATE_CONTROLLER_ENTDAA:
    cmd.rx_count = data_len;
    break;
    case I3C_INTERNAL_STATE_CONTROLLER_WRITE:
    case I3C_INTERNAL_STATE_CONTROLLER_COMMAND_WRITE:
// Disable the transmit IRQ if it hasn't been disabled already.
    renesas_clear_bit(i3c.regs, NTIE, NTIE_TDBEIE0);
    break;
    case I3C_INTERNAL_STATE_CONTROLLER_READ:
    case I3C_INTERNAL_STATE_CONTROLLER_COMMAND_READ:
    if (!cmd.err) {
    let mut rx_count: u32 = min(cmd.rx_count, data_len);
    bytes_remaining = data_len - rx_count;
    if (bytes_remaining)
    i3c_readl_fifo(i3c.regs + NTDTBP0,
    cmd.rx_buf + rx_count,
    bytes_remaining);
    cmd.rx_count = data_len;
    }
    renesas_clear_bit(i3c.regs, NTIE, NTIE_RDBFIE0);
    break;
    default:
    break;
    }
    switch (NRSPQP_ERR_STATUS(resp_descriptor)) {
    case NRSPQP_NO_ERROR:
    break;
    case NRSPQP_ERROR_PARITY:
    case NRSPQP_ERROR_IBA_NACK:
    case NRSPQP_ERROR_TRANSF_ABORT:
    case NRSPQP_ERROR_CRC:
    case NRSPQP_ERROR_FRAME:
    ret = -EIO;
    break;
    case NRSPQP_ERROR_OVER_UNDER_FLOW:
    ret = -ENOSPC;
    break;
    case NRSPQP_ERROR_UNSUPPORTED:
    ret = -EOPNOTSUPP;
    break;
    case NRSPQP_ERROR_I2C_W_NACK_ERR:
    case NRSPQP_ERROR_ADDRESS_NACK:
    default:
    ret = -EINVAL;
    break;
    }
//
// If the transfer was aborted, then the abort flag must be cleared
// before notifying the application that a transfer has completed.
//
    ntst = renesas_readl(i3c.regs, NTST);
    if (ntst & NTST_TABTF)
    renesas_clear_bit(i3c.regs, BCTL, BCTL_ABT);
// Clear error status flags.
    renesas_clear_bit(i3c.regs, NTST, NTST_TEF | NTST_TABTF);
    xfer.ret = ret;
    complete(&xfer.comp);
    xfer = list_first_entry_or_null(&i3c.xferqueue.list,
    struct renesas_i3c_xfer, node);
    if (xfer)
    list_del_init(&xfer.node);
    i3c.xferqueue.cur = xfer;
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn renesas_i3c_tend_isr(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t renesas_i3c_tend_isr(int irq, void *data)
    {
    struct renesas_i3c *i3c = data;
    struct renesas_i3c_xfer *xfer;
    struct renesas_i3c_cmd *cmd;
    scoped_guard(spinlock, &i3c.xferqueue.lock) {
    xfer = i3c.xferqueue.cur;
    if (!xfer) {
    renesas_i3c_irqs_mask_and_clear_locked(i3c);
    return IRQ_HANDLED;
    }
    cmd = xfer.cmds;
    if (xfer.is_i2c_xfer) {
    if (renesas_readl(i3c.regs, BST) & BST_NACKDF) {
// We got a NACKIE
    renesas_readl(i3c.regs, NTDTBP0); /* dummy read */
    renesas_clear_bit(i3c.regs, BST, BST_NACKDF);
    cmd.err = -ENXIO;
    } else if (cmd.i2c_bytes_left) {
    renesas_set_bit(i3c.regs, NTIE, NTIE_TDBEIE0);
    return IRQ_NONE;
    }
    if (cmd.i2c_is_last || cmd.err) {
    renesas_clear_bit(i3c.regs, BIE, BIE_TENDIE);
    renesas_set_bit(i3c.regs, BIE, BIE_SPCNDDIE);
    renesas_set_bit(i3c.regs, CNDCTL, CNDCTL_SPCND);
    } else {
// Transfer is complete, but do not send STOP
    renesas_clear_bit(i3c.regs, NTSTE, NTSTE_TDBEE0);
    renesas_clear_bit(i3c.regs, BIE, BIE_TENDIE);
    xfer.ret = 0;
    complete(&xfer.comp);
    }
    }
// Clear the Transmit Buffer Empty status flag.
    renesas_clear_bit(i3c.regs, BST, BST_TENDF);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn renesas_i3c_rx_isr(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t renesas_i3c_rx_isr(int irq, void *data)
    {
    struct renesas_i3c *i3c = data;
    struct renesas_i3c_xfer *xfer;
    struct renesas_i3c_cmd *cmd;
    int read_bytes;
// If resp_isr already read the data and updated 'xfer', we can just leave
    if (!(renesas_readl(i3c.regs, NTIE) & NTIE_RDBFIE0))
    return IRQ_NONE;
    scoped_guard(spinlock, &i3c.xferqueue.lock) {
    xfer = i3c.xferqueue.cur;
    if (!xfer) {
    renesas_i3c_irqs_mask_and_clear_locked(i3c);
    return IRQ_HANDLED;
    }
    cmd = xfer.cmds;
    if (xfer.is_i2c_xfer) {
    if (!cmd.i2c_bytes_left)
    return IRQ_NONE;
    if (cmd.i2c_bytes_left == I2C_INIT_MSG) {
    cmd.i2c_bytes_left = cmd.msg.len;
    renesas_set_bit(i3c.regs, SCSTRCTL, SCSTRCTL_RWE);
    renesas_readl(i3c.regs, NTDTBP0); /* dummy read */
    if (cmd.i2c_bytes_left == 1)
    renesas_writel(i3c.regs, ACKCTL, ACKCTL_ACKT | ACKCTL_ACKTWP);
    return IRQ_HANDLED;
    }
    if (cmd.i2c_bytes_left == 1) {
// STOP must come before we set ACKCTL!
    if (cmd.i2c_is_last) {
    renesas_set_bit(i3c.regs, BIE, BIE_SPCNDDIE);
    renesas_clear_bit(i3c.regs, BST, BST_SPCNDDF);
    renesas_set_bit(i3c.regs, CNDCTL, CNDCTL_SPCND);
    }
    renesas_writel(i3c.regs, ACKCTL, ACKCTL_ACKT | ACKCTL_ACKTWP);
    } else {
    renesas_writel(i3c.regs, ACKCTL, ACKCTL_ACKTWP);
    }
// Reading acks the RIE interrupt
// cmd->i2c_buf = renesas_readl(i3c->regs, NTDTBP0);
    cmd.i2c_buf++;
    cmd.i2c_bytes_left--;
    } else {
    read_bytes = NDBSTLV0_RDBLV(renesas_readl(i3c.regs, NDBSTLV0)) * sizeof(u32);
    i3c_readl_fifo(i3c.regs + NTDTBP0, cmd.rx_buf, read_bytes);
    cmd.rx_count = read_bytes;
    }
// Clear the Read Buffer Full status flag.
    renesas_clear_bit(i3c.regs, NTST, NTST_RDBFF0);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn renesas_i3c_stop_isr(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t renesas_i3c_stop_isr(int irq, void *data)
    {
    struct renesas_i3c *i3c = data;
    struct renesas_i3c_xfer *xfer;
    scoped_guard(spinlock, &i3c.xferqueue.lock) {
    renesas_i3c_irqs_mask_and_clear_locked(i3c);
    renesas_clear_bit(i3c.regs, SCSTRCTL, SCSTRCTL_RWE);
    xfer = i3c.xferqueue.cur;
    if (!xfer)
    return IRQ_HANDLED;
    xfer.ret = 0;
    complete(&xfer.comp);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn renesas_i3c_start_isr(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t renesas_i3c_start_isr(int irq, void *data)
    {
    struct renesas_i3c *i3c = data;
    struct renesas_i3c_xfer *xfer;
    struct renesas_i3c_cmd *cmd;
    u8 val;
    scoped_guard(spinlock, &i3c.xferqueue.lock) {
    xfer = i3c.xferqueue.cur;
    if (!xfer) {
    renesas_i3c_irqs_mask_and_clear_locked(i3c);
    return IRQ_HANDLED;
    }
    cmd = xfer.cmds;
    if (xfer.is_i2c_xfer) {
    if (!cmd.i2c_bytes_left)
    return IRQ_NONE;
    if (cmd.i2c_bytes_left == I2C_INIT_MSG) {
    if (cmd.msg.flags & I2C_M_RD) {
// On read, switch over to receive interrupt
    renesas_clear_bit(i3c.regs, NTIE, NTIE_TDBEIE0);
    renesas_set_bit(i3c.regs, NTIE, NTIE_RDBFIE0);
    } else {
// On write, initialize length
    cmd.i2c_bytes_left = cmd.msg.len;
    }
    val = i2c_8bit_addr_from_msg(cmd.msg);
    renesas_writel(i3c.regs, NTDTBP0, val);
    }
    }
    renesas_clear_bit(i3c.regs, BIE, BIE_STCNDDIE);
    renesas_clear_bit(i3c.regs, BST, BST_STCNDDF);
    }
    return IRQ_HANDLED;
    }
    static const struct i3c_master_controller_ops renesas_i3c_ops = {
    .bus_init = renesas_i3c_bus_init,
    .bus_cleanup = renesas_i3c_bus_cleanup,
    .attach_i3c_dev = renesas_i3c_attach_i3c_dev,
    .reattach_i3c_dev = renesas_i3c_reattach_i3c_dev,
    .detach_i3c_dev = renesas_i3c_detach_i3c_dev,
    .do_daa = renesas_i3c_daa,
    .supports_ccc_cmd = renesas_i3c_supports_ccc_cmd,
    .send_ccc_cmd = renesas_i3c_send_ccc_cmd,
    .i3c_xfers = renesas_i3c_i3c_xfers,
    .attach_i2c_dev = renesas_i3c_attach_i2c_dev,
    .detach_i2c_dev = renesas_i3c_detach_i2c_dev,
    .i2c_xfers = renesas_i3c_i2c_xfers,
    };
    static const struct renesas_i3c_irq_desc renesas_i3c_irqs[] = {
    { .name = "resp", .isr = renesas_i3c_resp_isr, .desc = "i3c-resp" },
    { .name = "rx", .isr = renesas_i3c_rx_isr, .desc = "i3c-rx" },
    { .name = "tx", .isr = renesas_i3c_tx_isr, .desc = "i3c-tx" },
    { .name = "st", .isr = renesas_i3c_start_isr, .desc = "i3c-start" },
    { .name = "sp", .isr = renesas_i3c_stop_isr, .desc = "i3c-stop" },
    { .name = "tend", .isr = renesas_i3c_tend_isr, .desc = "i3c-tend" },
    { .name = "nack", .isr = renesas_i3c_tend_isr, .desc = "i3c-nack" },
    };
#[no_mangle]
unsafe extern "C" fn renesas_i3c_probe(pdev: *mut platform_device) -> c_int {
    static int renesas_i3c_probe(struct platform_device *pdev)
    {
    struct renesas_i3c *i3c;
    int ret, i;
    i3c = devm_kzalloc(&pdev.dev, sizeof(*i3c), GFP_KERNEL);
    if (!i3c)
    return -ENOMEM;
    i3c.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(i3c.regs))
    return PTR_ERR(i3c.regs);
    i3c.tclk = devm_clk_get(&pdev.dev, "tclk");
    if (IS_ERR(i3c.tclk))
    return dev_err_probe(&pdev.dev, PTR_ERR(i3c.tclk), "Failed to get tclk");
    i3c.dev = &pdev.dev;
    pm_runtime_set_autosuspend_delay(&pdev.dev, 300);
    pm_runtime_use_autosuspend(&pdev.dev);
    ret = devm_pm_runtime_enable(&pdev.dev);
    if (ret)
    return ret;
    i3c.tresetn = devm_reset_control_get_optional_exclusive_deasserted(&pdev.dev, "tresetn");
    if (IS_ERR(i3c.tresetn))
    return dev_err_probe(&pdev.dev, PTR_ERR(i3c.tresetn),
    "Error: missing tresetn ctrl\n");
    i3c.presetn = devm_reset_control_get_optional_exclusive_deasserted(&pdev.dev, "presetn");
    if (IS_ERR(i3c.presetn))
    return dev_err_probe(&pdev.dev, PTR_ERR(i3c.presetn),
    "Error: missing presetn ctrl\n");
    spin_lock_init(&i3c.xferqueue.lock);
    INIT_LIST_HEAD(&i3c.xferqueue.list);
    ret = renesas_i3c_reset(i3c);
    if (ret)
    return ret;
    for (i = 0; i < ARRAY_SIZE(renesas_i3c_irqs); i++) {
    const char *irqname;
    ret = platform_get_irq_byname(pdev, renesas_i3c_irqs[i].name);
    if (ret < 0)
    return ret;
    irqname = devm_kasprintf(&pdev.dev, GFP_KERNEL, "%s:%s", dev_name(&pdev.dev),
    renesas_i3c_irqs[i].desc);
    if (!irqname)
    return -ENOMEM;
    ret = devm_request_irq(&pdev.dev, ret, renesas_i3c_irqs[i].isr,
    0, irqname, i3c);
    if (ret)
    return ret;
    }
    platform_set_drvdata(pdev, i3c);
    i3c.maxdevs = RENESAS_I3C_MAX_DEVS;
    i3c.free_pos = GENMASK(i3c.maxdevs - 1, 0);
    return i3c_master_register(&i3c.base, &pdev.dev, &renesas_i3c_ops, false);
    }
#[no_mangle]
unsafe extern "C" fn renesas_i3c_remove(pdev: *mut platform_device) {
    static void renesas_i3c_remove(struct platform_device *pdev)
    {
    struct renesas_i3c *i3c = platform_get_drvdata(pdev);
    i3c_master_unregister(&i3c.base);
    }
#[no_mangle]
unsafe extern "C" fn renesas_i3c_suspend(dev: *mut device) -> c_int {
    static int renesas_i3c_suspend(struct device *dev)
    {
    struct renesas_i3c *i3c = dev_get_drvdata(dev);
    struct reset_control_bulk_data resets[] = {
    { .rstc = i3c.presetn },
    { .rstc = i3c.tresetn },
    };
    int ret;
    i2c_mark_adapter_suspended(&i3c.base.i2c);
    ret = reset_control_bulk_assert(ARRAY_SIZE(resets), resets);
    if (ret)
    goto err_mark_resumed;
    return 0;
    err_mark_resumed:
    i2c_mark_adapter_resumed(&i3c.base.i2c);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn renesas_i3c_resume(dev: *mut device) -> c_int {
    static int renesas_i3c_resume(struct device *dev)
    {
    struct renesas_i3c *i3c = dev_get_drvdata(dev);
    struct reset_control_bulk_data resets[] = {
    { .rstc = i3c.presetn },
    { .rstc = i3c.tresetn },
    };
    int ret;
    ret = reset_control_bulk_deassert(ARRAY_SIZE(resets), resets);
    if (ret)
    return ret;
    ret = renesas_i3c_reset(i3c);
    if (ret)
    goto err_resets_asserted;
    ret = pm_runtime_resume_and_get(dev);
    if (ret)
    goto err_resets_asserted;
// Re-store I3C registers value.
    renesas_writel(i3c.regs, STDBR, i3c.i3c_STDBR);
    renesas_writel(i3c.regs, EXTBR, i3c.extbr);
    renesas_writel(i3c.regs, REFCKCTL,
    REFCKCTL_IREFCKS(i3c.refclk_div));
    renesas_writel(i3c.regs, MSDVAD, MSDVAD_MDYADV |
    MSDVAD_MDYAD(i3c.dyn_addr));
// I3C hw init.
    renesas_i3c_hw_init(i3c);
    ret = i3c_master_do_daa_ext(&i3c.base, true);
    if (ret)
    dev_err(dev, "DAA failed on resume, ret=%d", ret);
    i2c_mark_adapter_resumed(&i3c.base.i2c);
    pm_runtime_put_autosuspend(dev);
//
// I3C devices may have retained their dynamic address anyway. Do not
// fail the resume because of DAA error.
//
    return 0;
    err_resets_asserted:
//
// If this happens, there is no way to recover from this state without
// reloading the driver. We want to avoid keeping the reset line
// deasserted unnecessarily. The runtime paths will still work correctly
// even if the IP registers are accessed while reset is asserted (e.g.
// if a runtime path is triggered after a failed resume). Checked on
// RZ/G3S.
//
    reset_control_bulk_assert(ARRAY_SIZE(resets), resets);
    return ret;
    }
    static const struct dev_pm_ops renesas_i3c_pm_ops = {
    SYSTEM_SLEEP_PM_OPS(renesas_i3c_suspend, renesas_i3c_resume)
    };
    static const struct of_device_id renesas_i3c_of_ids[] = {
    { .compatible = "renesas,r9a08g045-i3c" },
    { .compatible = "renesas,r9a09g047-i3c" },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, renesas_i3c_of_ids);
    static struct platform_driver renesas_i3c = {
    .probe = renesas_i3c_probe,
    .remove = renesas_i3c_remove,
    .driver = {
    .name = "renesas-i3c",
    .of_match_table = renesas_i3c_of_ids,
    .pm = pm_sleep_ptr(&renesas_i3c_pm_ops),
    },
    };
    module_platform_driver(renesas_i3c);
    MODULE_AUTHOR("Wolfram Sang <wsa+renesas@sang-engineering.com>");
    MODULE_AUTHOR("Renesas BSP teams");
    MODULE_DESCRIPTION("Renesas I3C controller driver");
    MODULE_LICENSE("GPL");
