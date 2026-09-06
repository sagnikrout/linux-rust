//! Automatically rewritten from C to Rust
//! Source: drivers/net/can/grcan.c
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
// Socket CAN driver for Aeroflex Gaisler GRCAN and GRHCAN.
//
// 2012 (c) Aeroflex Gaisler AB
//
// This driver supports GRCAN and GRHCAN CAN controllers available in the GRLIB
// VHDL IP core library.
//
// Full documentation of the GRCAN core can be found here:
// http://www.gaisler.com/products/grlib/grip.pdf
//
// See "Documentation/devicetree/bindings/net/can/grcan.txt" for information on
// open firmware properties.
//
// See "Documentation/ABI/testing/sysfs-class-net-grcan" for information on the
// sysfs interface.
//
// See "Documentation/admin-guide/kernel-parameters.rst" for information on the module
// parameters.
//
// Contributors: Andreas Larsson <andreas@gaisler.com>
//

pub const GRCAN_NAPI_WEIGHT: c_int = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct grcan_registers {
    pub /: *mut *mut u32 conf; / 0x00,
    pub /: *mut *mut u32 stat; / 0x04,
    pub /: *mut *mut u32 ctrl; / 0x08,
    pub 0x18)]: u32 __reserved1[GRCAN_RESERVE_SIZE(0x08,,
    pub /: *mut *mut u32 smask; / 0x18 - CanMASK,
    pub /: *mut *mut u32 scode; / 0x1c - CanCODE,
    pub 0x100)]: u32 __reserved2[GRCAN_RESERVE_SIZE(0x1c,,
    pub /: *mut *mut u32 pimsr; / 0x100,
    pub /: *mut *mut u32 pimr; / 0x104,
    pub /: *mut *mut u32 pisr; / 0x108,
    pub /: *mut *mut u32 pir; / 0x10C,
    pub /: *mut *mut u32 imr; / 0x110,
    pub /: *mut *mut u32 picr; / 0x114,
    pub 0x200)]: u32 __reserved3[GRCAN_RESERVE_SIZE(0x114,,
    pub /: *mut *mut u32 txctrl; / 0x200,
    pub /: *mut *mut u32 txaddr; / 0x204,
    pub /: *mut *mut u32 txsize; / 0x208,
    pub /: *mut *mut u32 txwr; / 0x20C,
    pub /: *mut *mut u32 txrd; / 0x210,
    pub /: *mut *mut u32 txirq; / 0x214,
    pub 0x300)]: u32 __reserved4[GRCAN_RESERVE_SIZE(0x214,,
    pub /: *mut *mut u32 rxctrl; / 0x300,
    pub /: *mut *mut u32 rxaddr; / 0x304,
    pub /: *mut *mut u32 rxsize; / 0x308,
    pub /: *mut *mut u32 rxwr; / 0x30C,
    pub /: *mut *mut u32 rxrd; / 0x310,
    pub /: *mut *mut u32 rxirq; / 0x314,
    pub /: *mut *mut u32 rxmask; / 0x318,
    pub /: *mut *mut u32 rxcode; / 0x31C,
}

pub const GRCAN_CONF_ABORT: c_uint = 0x00000001;
pub const GRCAN_CONF_ENABLE0: c_uint = 0x00000002;
pub const GRCAN_CONF_ENABLE1: c_uint = 0x00000004;
pub const GRCAN_CONF_SELECT: c_uint = 0x00000008;
pub const GRCAN_CONF_SILENT: c_uint = 0x00000010;
pub const GRCAN_CONF_SAM: c_uint = 0x00000020 /* Available in some hardware */;
pub const GRCAN_CONF_BPR: c_uint = 0x00000300 /* Note: not BRP */;
pub const GRCAN_CONF_RSJ: c_uint = 0x00007000;
pub const GRCAN_CONF_PS1: c_uint = 0x00f00000;
pub const GRCAN_CONF_PS2: c_uint = 0x000f0000;
pub const GRCAN_CONF_SCALER: c_uint = 0xff000000;

    (GRCAN_CONF_ABORT | GRCAN_CONF_ENABLE0 | GRCAN_CONF_ENABLE1	\
    | GRCAN_CONF_SELECT | GRCAN_CONF_SILENT | GRCAN_CONF_SAM)

    (GRCAN_CONF_BPR | GRCAN_CONF_RSJ | GRCAN_CONF_PS1		\
    | GRCAN_CONF_PS2 | GRCAN_CONF_SCALER)
pub const GRCAN_CONF_RSJ_MIN: c_int = 1;
pub const GRCAN_CONF_RSJ_MAX: c_int = 4;
pub const GRCAN_CONF_PS1_MIN: c_int = 1;
pub const GRCAN_CONF_PS1_MAX: c_int = 15;
pub const GRCAN_CONF_PS2_MIN: c_int = 2;
pub const GRCAN_CONF_PS2_MAX: c_int = 8;
pub const GRCAN_CONF_SCALER_MIN: c_int = 0;
pub const GRCAN_CONF_SCALER_MAX: c_int = 255;
pub const GRCAN_CONF_SCALER_INC: c_int = 1;
pub const GRCAN_CONF_BPR_BIT: c_int = 8;
pub const GRCAN_CONF_RSJ_BIT: c_int = 12;
pub const GRCAN_CONF_PS1_BIT: c_int = 20;
pub const GRCAN_CONF_PS2_BIT: c_int = 16;
pub const GRCAN_CONF_SCALER_BIT: c_int = 24;
pub const GRCAN_STAT_PASS: c_uint = 0x000001;
pub const GRCAN_STAT_OFF: c_uint = 0x000002;
pub const GRCAN_STAT_OR: c_uint = 0x000004;
pub const GRCAN_STAT_AHBERR: c_uint = 0x000008;
pub const GRCAN_STAT_ACTIVE: c_uint = 0x000010;
pub const GRCAN_STAT_RXERRCNT: c_uint = 0x00ff00;
pub const GRCAN_STAT_TXERRCNT: c_uint = 0xff0000;

pub const GRCAN_STAT_RXERRCNT_BIT: c_int = 8;
pub const GRCAN_STAT_TXERRCNT_BIT: c_int = 16;
pub const GRCAN_STAT_ERRCNT_WARNING_LIMIT: c_int = 96;
pub const GRCAN_STAT_ERRCNT_PASSIVE_LIMIT: c_int = 127;
pub const GRCAN_CTRL_RESET: c_uint = 0x2;
pub const GRCAN_CTRL_ENABLE: c_uint = 0x1;
pub const GRCAN_TXCTRL_ENABLE: c_uint = 0x1;
pub const GRCAN_TXCTRL_ONGOING: c_uint = 0x2;
pub const GRCAN_TXCTRL_SINGLE: c_uint = 0x4;
pub const GRCAN_RXCTRL_ENABLE: c_uint = 0x1;
pub const GRCAN_RXCTRL_ONGOING: c_uint = 0x2;
// Relative offset of IRQ sources to AMBA Plug&Play
pub const GRCAN_IRQIX_IRQ: c_int = 0;
pub const GRCAN_IRQIX_TXSYNC: c_int = 1;
pub const GRCAN_IRQIX_RXSYNC: c_int = 2;
pub const GRCAN_IRQ_PASS: c_uint = 0x00001;
pub const GRCAN_IRQ_OFF: c_uint = 0x00002;
pub const GRCAN_IRQ_OR: c_uint = 0x00004;
pub const GRCAN_IRQ_RXAHBERR: c_uint = 0x00008;
pub const GRCAN_IRQ_TXAHBERR: c_uint = 0x00010;
pub const GRCAN_IRQ_RXIRQ: c_uint = 0x00020;
pub const GRCAN_IRQ_TXIRQ: c_uint = 0x00040;
pub const GRCAN_IRQ_RXFULL: c_uint = 0x00080;
pub const GRCAN_IRQ_TXEMPTY: c_uint = 0x00100;
pub const GRCAN_IRQ_RX: c_uint = 0x00200;
pub const GRCAN_IRQ_TX: c_uint = 0x00400;
pub const GRCAN_IRQ_RXSYNC: c_uint = 0x00800;
pub const GRCAN_IRQ_TXSYNC: c_uint = 0x01000;
pub const GRCAN_IRQ_RXERRCTR: c_uint = 0x02000;
pub const GRCAN_IRQ_TXERRCTR: c_uint = 0x04000;
pub const GRCAN_IRQ_RXMISS: c_uint = 0x08000;
pub const GRCAN_IRQ_TXLOSS: c_uint = 0x10000;
pub const GRCAN_IRQ_NONE: c_int = 0;

    (GRCAN_IRQ_PASS | GRCAN_IRQ_OFF | GRCAN_IRQ_OR			\
    | GRCAN_IRQ_RXAHBERR | GRCAN_IRQ_TXAHBERR			\
    | GRCAN_IRQ_RXIRQ | GRCAN_IRQ_TXIRQ				\
    | GRCAN_IRQ_RXFULL | GRCAN_IRQ_TXEMPTY				\
    | GRCAN_IRQ_RX | GRCAN_IRQ_TX | GRCAN_IRQ_RXSYNC		\
    | GRCAN_IRQ_TXSYNC | GRCAN_IRQ_RXERRCTR			\
    | GRCAN_IRQ_TXERRCTR | GRCAN_IRQ_RXMISS			\
    | GRCAN_IRQ_TXLOSS)

    | GRCAN_IRQ_PASS | GRCAN_IRQ_OFF)

    | GRCAN_IRQ_TXAHBERR | GRCAN_IRQ_RXAHBERR	\
    | GRCAN_IRQ_TXLOSS)

pub const GRCAN_MSG_SIZE: c_int = 16;
pub const GRCAN_MSG_IDE: c_uint = 0x80000000;
pub const GRCAN_MSG_RTR: c_uint = 0x40000000;
pub const GRCAN_MSG_BID: c_uint = 0x1ffc0000;
pub const GRCAN_MSG_EID: c_uint = 0x1fffffff;
pub const GRCAN_MSG_IDE_BIT: c_int = 31;
pub const GRCAN_MSG_RTR_BIT: c_int = 30;
pub const GRCAN_MSG_BID_BIT: c_int = 18;
pub const GRCAN_MSG_EID_BIT: c_int = 0;
pub const GRCAN_MSG_DLC: c_uint = 0xf0000000;
pub const GRCAN_MSG_TXERRC: c_uint = 0x00ff0000;
pub const GRCAN_MSG_RXERRC: c_uint = 0x0000ff00;
pub const GRCAN_MSG_DLC_BIT: c_int = 28;
pub const GRCAN_MSG_TXERRC_BIT: c_int = 16;
pub const GRCAN_MSG_RXERRC_BIT: c_int = 8;
pub const GRCAN_MSG_AHBERR: c_uint = 0x00000008;
pub const GRCAN_MSG_OR: c_uint = 0x00000004;
pub const GRCAN_MSG_OFF: c_uint = 0x00000002;
pub const GRCAN_MSG_PASS: c_uint = 0x00000001;

pub const GRCAN_BUFFER_ALIGNMENT: c_int = 1024;
pub const GRCAN_DEFAULT_BUFFER_SIZE: c_int = 1024;
pub const GRCAN_VALID_TR_SIZE_MASK: c_uint = 0x001fffc0;

    ((s) == 0 || ((s) & ~GRCAN_VALID_TR_SIZE_MASK))

#[repr(C)]
#[derive(Copy, Clone)]
pub struct grcan_dma_buffer {
    pub size: usize,
    pub buf: *mut c_void,
    pub handle: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct grcan_dma {
    pub base_size: usize,
    pub base_buf: *mut c_void,
    pub base_handle: dma_addr_t,
    pub tx: grcan_dma_buffer,
    pub rx: grcan_dma_buffer,
}

// GRCAN configuration parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct grcan_device_config {
    pub enable0: c_ushort,
    pub enable1: c_ushort,
    pub select: c_ushort,
    pub txsize: c_uint,
    pub rxsize: c_uint,
}

    .enable0	= 0,				\
    .enable1	= 0,				\
    .select		= 0,				\
    .txsize		= GRCAN_DEFAULT_BUFFER_SIZE,	\
    .rxsize		= GRCAN_DEFAULT_BUFFER_SIZE,	\
    }
pub const GRCAN_TXBUG_SAFE_GRLIB_VERSION: c_int = 4100;
pub const GRLIB_VERSION_MASK: c_uint = 0xffff;
// GRCAN private data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct grcan_priv {
    pub /: *mut *mut can_priv can; / must be the first member,
    pub dev: *mut net_device,
    pub ofdev_dev: *mut device,
    pub napi: napi_struct,
    pub /: *mut *mut *mut grcan_registers __iomem regs; / ioremap'ed registers,
    pub config: grcan_device_config,
    pub dma: grcan_dma,
    pub /: *mut *mut *mut *mut sk_buff echo_skb; / We allocate this on our own,
// The echo skb pointer, pointing into echo_skb and indicating which
// frames can be echoed back. See the "Notes on the tx cyclic buffer
// handling"-comment for grcan_start_xmit for more details.
//
    pub eskbp: u32,
// Lock for controlling changes to the netif tx queue state, accesses to
// the echo_skb pointer eskbp and for making sure that a running reset
// and/or a close of the interface is done without interference from
// other parts of the code.
//
// The echo_skb pointer, eskbp, should only be accessed under this lock
// as it can be changed in several places and together with decisions on
// whether to wake up the tx queue.
//
// The tx queue must never be woken up if there is a running reset or
// close in progress.
//
// A running reset (see below on need_txbug_workaround) should never be
// done if the interface is closing down and several running resets
// should never be scheduled simultaneously.
//
    pub lock: spinlock_t,
// Whether a workaround is needed due to a bug in older hardware. In
// this case, the driver both tries to prevent the bug from being
// triggered and recovers, if the bug nevertheless happens, by doing a
// running reset. A running reset, resets the device and continues from
// where it were without being noticeable from outside the driver (apart
// from slight delays).
//
    pub need_txbug_workaround: bool,
// To trigger initization of running reset and to trigger running reset
// respectively in the case of a hanged device due to a txbug.
//
    pub hang_timer: timer_list,
    pub rr_timer: timer_list,
// To avoid waking up the netif queue and restarting timers
// when a reset is scheduled or when closing of the device is
// undergoing
//
    pub resetting: bool,
    pub closing: bool,
}

// Wait time for a short wait for ongoing to clear
pub const GRCAN_SHORTWAIT_USECS: c_int = 10;
// Limit on the number of transmitted bits of an eff frame according to the CAN
// specification: 1 bit start of frame, 32 bits arbitration field, 6 bits
// control field, 8 bytes data field, 16 bits crc field, 2 bits ACK field and 7
// bits end of frame
//

#[no_mangle]
pub unsafe extern "C" fn grcan_read_reg(reg: *mut u32 __iomem) -> u32 {
    static inline u32 grcan_read_reg(u32 __iomem *reg)
    {
    return ioread32be(reg);
    }
#[no_mangle]
pub unsafe extern "C" fn grcan_write_reg(reg: *mut u32 __iomem, val: u32) {
    static inline void grcan_write_reg(u32 __iomem *reg, u32 val)
    {
    iowrite32be(val, reg);
    }

#[no_mangle]
pub unsafe extern "C" fn grcan_read_reg(reg: *mut u32 __iomem) -> u32 {
    static inline u32 grcan_read_reg(u32 __iomem *reg)
    {
    return ioread32(reg);
    }
#[no_mangle]
pub unsafe extern "C" fn grcan_write_reg(reg: *mut u32 __iomem, val: u32) {
    static inline void grcan_write_reg(u32 __iomem *reg, u32 val)
    {
    iowrite32(val, reg);
    }

#[no_mangle]
pub unsafe extern "C" fn grcan_clear_bits(reg: *mut u32 __iomem, mask: u32) {
    static inline void grcan_clear_bits(u32 __iomem *reg, u32 mask)
    {
    grcan_write_reg(reg, grcan_read_reg(reg) & ~mask);
    }
#[no_mangle]
pub unsafe extern "C" fn grcan_set_bits(reg: *mut u32 __iomem, mask: u32) {
    static inline void grcan_set_bits(u32 __iomem *reg, u32 mask)
    {
    grcan_write_reg(reg, grcan_read_reg(reg) | mask);
    }
#[no_mangle]
pub unsafe extern "C" fn grcan_read_bits(reg: *mut u32 __iomem, mask: u32) -> u32 {
    static inline u32 grcan_read_bits(u32 __iomem *reg, u32 mask)
    {
    return grcan_read_reg(reg) & mask;
    }
#[no_mangle]
pub unsafe extern "C" fn grcan_write_bits(reg: *mut u32 __iomem, value: u32, mask: u32) {
    static inline void grcan_write_bits(u32 __iomem *reg, u32 value, u32 mask)
    {
    let mut old: u32 = grcan_read_reg(reg);
    grcan_write_reg(reg, (old & ~mask) | (value & mask));
    }
// a and b should both be in [0,size] and a == b == size should not hold
#[no_mangle]
pub unsafe extern "C" fn grcan_ring_add(a: u32, b: u32, size: u32) -> u32 {
    static inline u32 grcan_ring_add(u32 a, u32 b, u32 size)
    {
    let mut sum: u32 = a + b;
    if (sum < size)
    return sum;
    else
    return sum - size;
    }
// a and b should both be in [0,size)
#[no_mangle]
pub unsafe extern "C" fn grcan_ring_sub(a: u32, b: u32, size: u32) -> u32 {
    static inline u32 grcan_ring_sub(u32 a, u32 b, u32 size)
    {
    return grcan_ring_add(a, size - b, size);
    }
// Available slots for new transmissions
#[no_mangle]
pub unsafe extern "C" fn grcan_txspace(txsize: usize, txwr: u32, eskbp: u32) -> u32 {
    static inline u32 grcan_txspace(size_t txsize, u32 txwr, u32 eskbp)
    {
    let mut slots: u32 = txsize / GRCAN_MSG_SIZE - 1;
    let mut used: u32 = grcan_ring_sub(txwr, eskbp, txsize) / GRCAN_MSG_SIZE;
    return slots - used;
    }
// Configuration parameters that can be set via module parameters
    static struct grcan_device_config grcan_module_config =
    GRCAN_DEFAULT_DEVICE_CONFIG;
    static const struct can_bittiming_const grcan_bittiming_const = {
    .name		= DRV_NAME,
    .tseg1_min	= GRCAN_CONF_PS1_MIN + 1,
    .tseg1_max	= GRCAN_CONF_PS1_MAX + 1,
    .tseg2_min	= GRCAN_CONF_PS2_MIN,
    .tseg2_max	= GRCAN_CONF_PS2_MAX,
    .sjw_max	= GRCAN_CONF_RSJ_MAX,
    .brp_min	= GRCAN_CONF_SCALER_MIN + 1,
    .brp_max	= GRCAN_CONF_SCALER_MAX + 1,
    .brp_inc	= GRCAN_CONF_SCALER_INC,
    };
#[no_mangle]
unsafe extern "C" fn grcan_set_bittiming(dev: *mut net_device) -> c_int {
    static int grcan_set_bittiming(struct net_device *dev)
    {
    struct grcan_priv *priv = netdev_priv(dev);
    struct grcan_registers __iomem *regs = priv.regs;
    struct can_bittiming *bt = &priv.can.bittiming;
    let mut timing: u32 = 0;
    int bpr, rsj, ps1, ps2, scaler;
// Should never happen - function will not be called when
// device is up
//
    if (grcan_read_bits(&regs.ctrl, GRCAN_CTRL_ENABLE))
    return -EBUSY;
    bpr = 0; /* Note bpr and brp are different concepts */
    rsj = bt.sjw;
    ps1 = (bt.prop_seg + bt.phase_seg1) - 1; /* tseg1 - 1 */
    ps2 = bt.phase_seg2;
    scaler = (bt.brp - 1);
    netdev_dbg(dev, "Request for BPR=%d, RSJ=%d, PS1=%d, PS2=%d, SCALER=%d",
    bpr, rsj, ps1, ps2, scaler);
    if (!(ps1 > ps2)) {
    netdev_err(dev, "PS1 > PS2 must hold: PS1=%d, PS2=%d\n",
    ps1, ps2);
    return -EINVAL;
    }
    if (!(ps2 >= rsj)) {
    netdev_err(dev, "PS2 >= RSJ must hold: PS2=%d, RSJ=%d\n",
    ps2, rsj);
    return -EINVAL;
    }
    timing |= (bpr << GRCAN_CONF_BPR_BIT) & GRCAN_CONF_BPR;
    timing |= (rsj << GRCAN_CONF_RSJ_BIT) & GRCAN_CONF_RSJ;
    timing |= (ps1 << GRCAN_CONF_PS1_BIT) & GRCAN_CONF_PS1;
    timing |= (ps2 << GRCAN_CONF_PS2_BIT) & GRCAN_CONF_PS2;
    timing |= (scaler << GRCAN_CONF_SCALER_BIT) & GRCAN_CONF_SCALER;
    netdev_info(dev, "setting timing=0x%x\n", timing);
    grcan_write_bits(&regs.conf, timing, GRCAN_CONF_TIMING);
    return 0;
    }
    static int grcan_get_berr_counter(const struct net_device *dev,
    struct can_berr_counter *bec)
    {
    struct grcan_priv *priv = netdev_priv(dev);
    struct grcan_registers __iomem *regs = priv.regs;
    let mut status: u32 = grcan_read_reg(&regs.stat);
    bec.txerr = (status & GRCAN_STAT_TXERRCNT) >> GRCAN_STAT_TXERRCNT_BIT;
    bec.rxerr = (status & GRCAN_STAT_RXERRCNT) >> GRCAN_STAT_RXERRCNT_BIT;
    return 0;
    }
    static int grcan_poll(struct napi_struct *napi, int budget);
// Reset device, but keep configuration information
#[no_mangle]
unsafe extern "C" fn grcan_reset(dev: *mut net_device) {
    static void grcan_reset(struct net_device *dev)
    {
    struct grcan_priv *priv = netdev_priv(dev);
    struct grcan_registers __iomem *regs = priv.regs;
    let mut config: u32 = grcan_read_reg(&regs.conf);
    grcan_set_bits(&regs.ctrl, GRCAN_CTRL_RESET);
    grcan_write_reg(&regs.conf, config);
    priv.eskbp = grcan_read_reg(&regs.txrd);
    priv.can.state = CAN_STATE_STOPPED;
// Turn off hardware filtering - regs->rxcode set to 0 by reset
    grcan_write_reg(&regs.rxmask, 0);
    }
// stop device without changing any configurations
#[no_mangle]
unsafe extern "C" fn grcan_stop_hardware(dev: *mut net_device) {
    static void grcan_stop_hardware(struct net_device *dev)
    {
    struct grcan_priv *priv = netdev_priv(dev);
    struct grcan_registers __iomem *regs = priv.regs;
    grcan_write_reg(&regs.imr, GRCAN_IRQ_NONE);
    grcan_clear_bits(&regs.txctrl, GRCAN_TXCTRL_ENABLE);
    grcan_clear_bits(&regs.rxctrl, GRCAN_RXCTRL_ENABLE);
    grcan_clear_bits(&regs.ctrl, GRCAN_CTRL_ENABLE);
    }
// Let priv->eskbp catch up to regs->txrd and echo back the skbs if echo
// is true and free them otherwise.
//
// If budget is >= 0, stop after handling at most budget skbs. Otherwise,
// continue until priv->eskbp catches up to regs->txrd.
//
// priv->lock *must* be held when calling this function
//
#[no_mangle]
unsafe extern "C" fn catch_up_echo_skb(dev: *mut net_device, budget: c_int, echo: bool) -> c_int {
    static int catch_up_echo_skb(struct net_device *dev, int budget, bool echo)
    {
    struct grcan_priv *priv = netdev_priv(dev);
    struct grcan_registers __iomem *regs = priv.regs;
    struct grcan_dma *dma = &priv.dma;
    struct net_device_stats *stats = &dev.stats;
    int i, work_done;
// Updates to priv->eskbp and wake-ups of the queue needs to
// be atomic towards the reads of priv->eskbp and shut-downs
// of the queue in grcan_start_xmit.
//
    let mut txrd: u32 = grcan_read_reg(&regs.txrd);
    for (work_done = 0; work_done < budget || budget < 0; work_done++) {
    if (priv.eskbp == txrd)
    break;
    i = priv.eskbp / GRCAN_MSG_SIZE;
    if (echo) {
// Normal echo of messages
    stats.tx_packets++;
    stats.tx_bytes += can_get_echo_skb(dev, i, core::ptr::null_mut());
    } else {
// For cleanup of untransmitted messages
    can_free_echo_skb(dev, i, core::ptr::null_mut());
    }
    priv.eskbp = grcan_ring_add(priv.eskbp, GRCAN_MSG_SIZE,
    dma.tx.size);
    txrd = grcan_read_reg(&regs.txrd);
    }
    return work_done;
    }
#[no_mangle]
unsafe extern "C" fn grcan_lost_one_shot_frame(dev: *mut net_device) {
    static void grcan_lost_one_shot_frame(struct net_device *dev)
    {
    struct grcan_priv *priv = netdev_priv(dev);
    struct grcan_registers __iomem *regs = priv.regs;
    struct grcan_dma *dma = &priv.dma;
    u32 txrd;
    unsigned long flags;
    spin_lock_irqsave(&priv.lock, flags);
    catch_up_echo_skb(dev, -1, true);
    if (unlikely(grcan_read_bits(&regs.txctrl, GRCAN_TXCTRL_ENABLE))) {
// Should never happen
    netdev_err(dev, "TXCTRL enabled at TXLOSS in one shot mode\n");
    } else {
// By the time an GRCAN_IRQ_TXLOSS is generated in
// one-shot mode there is no problem in writing
// to TXRD even in versions of the hardware in
// which GRCAN_TXCTRL_ONGOING is not cleared properly
// in one-shot mode.
//
// Skip message and discard echo-skb
    txrd = grcan_read_reg(&regs.txrd);
    txrd = grcan_ring_add(txrd, GRCAN_MSG_SIZE, dma.tx.size);
    grcan_write_reg(&regs.txrd, txrd);
    catch_up_echo_skb(dev, -1, false);
    if (!priv.resetting && !priv.closing &&
    !(priv.can.ctrlmode & CAN_CTRLMODE_LISTENONLY)) {
    netif_wake_queue(dev);
    grcan_set_bits(&regs.txctrl, GRCAN_TXCTRL_ENABLE);
    }
    }
    spin_unlock_irqrestore(&priv.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn grcan_err(dev: *mut net_device, sources: u32, status: u32) {
    static void grcan_err(struct net_device *dev, u32 sources, u32 status)
    {
    struct grcan_priv *priv = netdev_priv(dev);
    struct grcan_registers __iomem *regs = priv.regs;
    struct grcan_dma *dma = &priv.dma;
    struct net_device_stats *stats = &dev.stats;
    struct can_frame cf;
// Zero potential error_frame
    memset(&cf, 0, sizeof(cf));
// Message lost interrupt. This might be due to arbitration error, but
// is also triggered when there is no one else on the can bus or when
// there is a problem with the hardware interface or the bus itself. As
// arbitration errors can not be singled out, no error frames are
// generated reporting this event as an arbitration error.
//
    if (sources & GRCAN_IRQ_TXLOSS) {
// Take care of failed one-shot transmit
    if (priv.can.ctrlmode & CAN_CTRLMODE_ONE_SHOT)
    grcan_lost_one_shot_frame(dev);
// Stop printing as soon as error passive or bus off is in
// effect to limit the amount of txloss debug printouts.
//
    if (!(status & GRCAN_STAT_ERRCTR_RELATED)) {
    netdev_dbg(dev, "tx message lost\n");
    stats.tx_errors++;
    }
    }
// Conditions dealing with the error counters. There is no interrupt for
// error warning, but there are interrupts for increases of the error
// counters.
//
    if ((sources & GRCAN_IRQ_ERRCTR_RELATED) ||
    (status & GRCAN_STAT_ERRCTR_RELATED)) {
    let mut state: enum can_state = priv.can.state;
    let mut oldstate: enum can_state = state;
    u32 txerr = (status & GRCAN_STAT_TXERRCNT)
    >> GRCAN_STAT_TXERRCNT_BIT;
    u32 rxerr = (status & GRCAN_STAT_RXERRCNT)
    >> GRCAN_STAT_RXERRCNT_BIT;
// Figure out current state
    if (status & GRCAN_STAT_OFF) {
    state = CAN_STATE_BUS_OFF;
    } else if (status & GRCAN_STAT_PASS) {
    state = CAN_STATE_ERROR_PASSIVE;
    } else if (txerr >= GRCAN_STAT_ERRCNT_WARNING_LIMIT ||
    rxerr >= GRCAN_STAT_ERRCNT_WARNING_LIMIT) {
    state = CAN_STATE_ERROR_WARNING;
    } else {
    state = CAN_STATE_ERROR_ACTIVE;
    }
// Handle and report state changes
    if (state != oldstate) {
    switch (state) {
    case CAN_STATE_BUS_OFF:
    netdev_dbg(dev, "bus-off\n");
    netif_carrier_off(dev);
    priv.can.can_stats.bus_off++;
// Prevent the hardware from recovering from bus
// off on its own if restart is disabled.
//
    if (!priv.can.restart_ms)
    grcan_stop_hardware(dev);
    cf.can_id |= CAN_ERR_BUSOFF;
    break;
    case CAN_STATE_ERROR_PASSIVE:
    netdev_dbg(dev, "Error passive condition\n");
    priv.can.can_stats.error_passive++;
    cf.can_id |= CAN_ERR_CRTL;
    if (txerr >= GRCAN_STAT_ERRCNT_PASSIVE_LIMIT)
    cf.data[1] |= CAN_ERR_CRTL_TX_PASSIVE;
    if (rxerr >= GRCAN_STAT_ERRCNT_PASSIVE_LIMIT)
    cf.data[1] |= CAN_ERR_CRTL_RX_PASSIVE;
    break;
    case CAN_STATE_ERROR_WARNING:
    netdev_dbg(dev, "Error warning condition\n");
    priv.can.can_stats.error_warning++;
    cf.can_id |= CAN_ERR_CRTL;
    if (txerr >= GRCAN_STAT_ERRCNT_WARNING_LIMIT)
    cf.data[1] |= CAN_ERR_CRTL_TX_WARNING;
    if (rxerr >= GRCAN_STAT_ERRCNT_WARNING_LIMIT)
    cf.data[1] |= CAN_ERR_CRTL_RX_WARNING;
    break;
    case CAN_STATE_ERROR_ACTIVE:
    netdev_dbg(dev, "Error active condition\n");
    cf.can_id |= CAN_ERR_CRTL;
    break;
    default:
// There are no others at this point
    break;
    }
    cf.can_id |= CAN_ERR_CNT;
    cf.data[6] = txerr;
    cf.data[7] = rxerr;
    priv.can.state = state;
    }
// Report automatic restarts
    if (priv.can.restart_ms && oldstate == CAN_STATE_BUS_OFF) {
    unsigned long flags;
    cf.can_id |= CAN_ERR_RESTARTED;
    netdev_dbg(dev, "restarted\n");
    priv.can.can_stats.restarts++;
    netif_carrier_on(dev);
    spin_lock_irqsave(&priv.lock, flags);
    if (!priv.resetting && !priv.closing) {
    let mut txwr: u32 = grcan_read_reg(&regs.txwr);
    if (grcan_txspace(dma.tx.size, txwr,
    priv.eskbp))
    netif_wake_queue(dev);
    }
    spin_unlock_irqrestore(&priv.lock, flags);
    }
    }
// Data overrun interrupt
    if ((sources & GRCAN_IRQ_OR) || (status & GRCAN_STAT_OR)) {
    netdev_dbg(dev, "got data overrun interrupt\n");
    stats.rx_over_errors++;
    stats.rx_errors++;
    cf.can_id |= CAN_ERR_CRTL;
    cf.data[1] |= CAN_ERR_CRTL_RX_OVERFLOW;
    }
// AHB bus error interrupts (not CAN bus errors) - shut down the
// device.
//
    if (sources & (GRCAN_IRQ_TXAHBERR | GRCAN_IRQ_RXAHBERR) ||
    (status & GRCAN_STAT_AHBERR)) {
    char *txrx = "";
    unsigned long flags;
    if (sources & GRCAN_IRQ_TXAHBERR) {
    txrx = "on tx ";
    stats.tx_errors++;
    } else if (sources & GRCAN_IRQ_RXAHBERR) {
    txrx = "on rx ";
    stats.rx_errors++;
    }
    netdev_err(dev, "Fatal AHB bus error %s- halting device\n",
    txrx);
    spin_lock_irqsave(&priv.lock, flags);
// Prevent anything to be enabled again and halt device
    priv.closing = true;
    netif_stop_queue(dev);
    grcan_stop_hardware(dev);
    priv.can.state = CAN_STATE_STOPPED;
    spin_unlock_irqrestore(&priv.lock, flags);
    }
// Pass on error frame if something to report,
// i.e. id contains some information
//
    if (cf.can_id) {
    struct can_frame *skb_cf;
    struct sk_buff *skb = alloc_can_err_skb(dev, &skb_cf);
    if (skb == core::ptr::null_mut()) {
    netdev_dbg(dev, "could not allocate error frame\n");
    return;
    }
    skb_cf.can_id |= cf.can_id;
    memcpy(skb_cf.data, cf.data, sizeof(cf.data));
    netif_rx(skb);
    }
    }
#[no_mangle]
unsafe extern "C" fn grcan_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t grcan_interrupt(int irq, void *dev_id)
    {
    struct net_device *dev = dev_id;
    struct grcan_priv *priv = netdev_priv(dev);
    struct grcan_registers __iomem *regs = priv.regs;
    u32 sources, status;
// Find out the source
    sources = grcan_read_reg(&regs.pimsr);
    if (!sources)
    return IRQ_NONE;
    grcan_write_reg(&regs.picr, sources);
    status = grcan_read_reg(&regs.stat);
// If we got TX progress, the device has not hanged,
// so disable the hang timer
//
    if (priv.need_txbug_workaround &&
    (sources & (GRCAN_IRQ_TX | GRCAN_IRQ_TXLOSS))) {
    timer_delete(&priv.hang_timer);
    }
// Frame(s) received or transmitted
    if (sources & (GRCAN_IRQ_TX | GRCAN_IRQ_RX)) {
// Disable tx/rx interrupts and schedule poll(). No need for
// locking as interference from a running reset at worst leads
// to an extra interrupt.
//
    grcan_clear_bits(&regs.imr, GRCAN_IRQ_TX | GRCAN_IRQ_RX);
    napi_schedule(&priv.napi);
    }
// (Potential) error conditions to take care of
    if (sources & GRCAN_IRQ_ERRORS)
    grcan_err(dev, sources, status);
    return IRQ_HANDLED;
    }
// Reset device and restart operations from where they were.
//
// This assumes that RXCTRL & RXCTRL is properly disabled and that RX
// is not ONGOING (TX might be stuck in ONGOING due to a harwrware bug
// for single shot)
//
#[no_mangle]
unsafe extern "C" fn grcan_running_reset(t: *mut timer_list) {
    static void grcan_running_reset(struct timer_list *t)
    {
    struct grcan_priv *priv = timer_container_of(priv, t, rr_timer);
    struct net_device *dev = priv.dev;
    struct grcan_registers __iomem *regs = priv.regs;
    unsigned long flags;
// This temporarily messes with eskbp, so we need to lock
// priv->lock
//
    spin_lock_irqsave(&priv.lock, flags);
    priv.resetting = false;
    timer_delete(&priv.hang_timer);
    timer_delete(&priv.rr_timer);
    if (!priv.closing) {
// Save and reset - config register preserved by grcan_reset
    let mut imr: u32 = grcan_read_reg(&regs.imr);
    let mut txaddr: u32 = grcan_read_reg(&regs.txaddr);
    let mut txsize: u32 = grcan_read_reg(&regs.txsize);
    let mut txwr: u32 = grcan_read_reg(&regs.txwr);
    let mut txrd: u32 = grcan_read_reg(&regs.txrd);
    let mut eskbp: u32 = priv.eskbp;
    let mut rxaddr: u32 = grcan_read_reg(&regs.rxaddr);
    let mut rxsize: u32 = grcan_read_reg(&regs.rxsize);
    let mut rxwr: u32 = grcan_read_reg(&regs.rxwr);
    let mut rxrd: u32 = grcan_read_reg(&regs.rxrd);
    grcan_reset(dev);
// Restore
    grcan_write_reg(&regs.txaddr, txaddr);
    grcan_write_reg(&regs.txsize, txsize);
    grcan_write_reg(&regs.txwr, txwr);
    grcan_write_reg(&regs.txrd, txrd);
    priv.eskbp = eskbp;
    grcan_write_reg(&regs.rxaddr, rxaddr);
    grcan_write_reg(&regs.rxsize, rxsize);
    grcan_write_reg(&regs.rxwr, rxwr);
    grcan_write_reg(&regs.rxrd, rxrd);
// Turn on device again
    grcan_write_reg(&regs.imr, imr);
    priv.can.state = CAN_STATE_ERROR_ACTIVE;
    grcan_write_reg(&regs.txctrl, GRCAN_TXCTRL_ENABLE
    | (priv.can.ctrlmode & CAN_CTRLMODE_ONE_SHOT
    ? GRCAN_TXCTRL_SINGLE : 0));
    grcan_write_reg(&regs.rxctrl, GRCAN_RXCTRL_ENABLE);
    grcan_write_reg(&regs.ctrl, GRCAN_CTRL_ENABLE);
// Start queue if there is size and listen-onle mode is not
// enabled
//
    if (grcan_txspace(priv.dma.tx.size, txwr, priv.eskbp) &&
    !(priv.can.ctrlmode & CAN_CTRLMODE_LISTENONLY))
    netif_wake_queue(dev);
    }
    spin_unlock_irqrestore(&priv.lock, flags);
    netdev_err(dev, "Device reset and restored\n");
    }
// Waiting time in usecs corresponding to the transmission of three maximum
// sized can frames in the given bitrate (in bits/sec). Waiting for this amount
// of time makes sure that the can controller have time to finish sending or
// receiving a frame with a good margin.
//
// usecs/sec * number of frames * bits/frame / bits/sec
//
#[no_mangle]
pub unsafe extern "C" fn grcan_ongoing_wait_usecs(bitrate: __u32) -> u32 {
    static inline u32 grcan_ongoing_wait_usecs(__u32 bitrate)
    {
    return 1000000 * 3 * GRCAN_EFF_FRAME_MAX_BITS / bitrate;
    }
// Set timer so that it will not fire until after a period in which the can
// controller have a good margin to finish transmitting a frame unless it has
// hanged
//
#[no_mangle]
pub unsafe extern "C" fn grcan_reset_timer(timer: *mut timer_list, bitrate: __u32) {
    static inline void grcan_reset_timer(struct timer_list *timer, __u32 bitrate)
    {
    let mut wait_jiffies: u32 = usecs_to_jiffies(grcan_ongoing_wait_usecs(bitrate));
    mod_timer(timer, jiffies + wait_jiffies);
    }
// Disable channels and schedule a running reset
#[no_mangle]
unsafe extern "C" fn grcan_initiate_running_reset(t: *mut timer_list) {
    static void grcan_initiate_running_reset(struct timer_list *t)
    {
    struct grcan_priv *priv = timer_container_of(priv, t, hang_timer);
    struct net_device *dev = priv.dev;
    struct grcan_registers __iomem *regs = priv.regs;
    unsigned long flags;
    netdev_err(dev, "Device seems hanged - reset scheduled\n");
    spin_lock_irqsave(&priv.lock, flags);
// The main body of this function must never be executed again
// until after an execution of grcan_running_reset
//
    if (!priv.resetting && !priv.closing) {
    priv.resetting = true;
    netif_stop_queue(dev);
    grcan_clear_bits(&regs.txctrl, GRCAN_TXCTRL_ENABLE);
    grcan_clear_bits(&regs.rxctrl, GRCAN_RXCTRL_ENABLE);
    grcan_reset_timer(&priv.rr_timer, priv.can.bittiming.bitrate);
    }
    spin_unlock_irqrestore(&priv.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn grcan_free_dma_buffers(dev: *mut net_device) {
    static void grcan_free_dma_buffers(struct net_device *dev)
    {
    struct grcan_priv *priv = netdev_priv(dev);
    struct grcan_dma *dma = &priv.dma;
    dma_free_coherent(priv.ofdev_dev, dma.base_size, dma.base_buf,
    dma.base_handle);
    memset(dma, 0, sizeof(*dma));
    }
    static int grcan_allocate_dma_buffers(struct net_device *dev,
    size_t tsize, size_t rsize)
    {
    struct grcan_priv *priv = netdev_priv(dev);
    struct grcan_dma *dma = &priv.dma;
    struct grcan_dma_buffer *large = rsize > tsize ? &dma.rx : &dma.tx;
    struct grcan_dma_buffer *small = rsize > tsize ? &dma.tx : &dma.rx;
    size_t shift;
// Need a whole number of GRCAN_BUFFER_ALIGNMENT for the large,
// i.e. first buffer
//
    let mut maxs: usize = max(tsize, rsize);
    let mut lsize: usize = ALIGN(maxs, GRCAN_BUFFER_ALIGNMENT);
// Put the small buffer after that
    let mut ssize: usize = min(tsize, rsize);
// Extra GRCAN_BUFFER_ALIGNMENT to allow for alignment
    dma.base_size = lsize + ssize + GRCAN_BUFFER_ALIGNMENT;
    dma.base_buf = dma_alloc_coherent(priv.ofdev_dev,
    dma.base_size,
    &dma.base_handle,
    GFP_KERNEL);
    if (!dma.base_buf)
    return -ENOMEM;
    dma.tx.size = tsize;
    dma.rx.size = rsize;
    large.handle = ALIGN(dma.base_handle, GRCAN_BUFFER_ALIGNMENT);
    small.handle = large.handle + lsize;
    shift = large.handle - dma.base_handle;
    large.buf = dma.base_buf + shift;
    small.buf = large.buf + lsize;
    return 0;
    }
// priv->lock *must* be held when calling this function
#[no_mangle]
unsafe extern "C" fn grcan_start(dev: *mut net_device) -> c_int {
    static int grcan_start(struct net_device *dev)
    {
    struct grcan_priv *priv = netdev_priv(dev);
    struct grcan_registers __iomem *regs = priv.regs;
    u32 confop, txctrl;
    grcan_reset(dev);
    grcan_write_reg(&regs.txaddr, priv.dma.tx.handle);
    grcan_write_reg(&regs.txsize, priv.dma.tx.size);
// regs->txwr, regs->txrd and priv->eskbp already set to 0 by reset
    grcan_write_reg(&regs.rxaddr, priv.dma.rx.handle);
    grcan_write_reg(&regs.rxsize, priv.dma.rx.size);
// regs->rxwr and regs->rxrd already set to 0 by reset
// Enable interrupts
    grcan_read_reg(&regs.pir);
    grcan_write_reg(&regs.imr, GRCAN_IRQ_DEFAULT);
// Enable interfaces, channels and device
    confop = GRCAN_CONF_ABORT
    | (priv.config.enable0 ? GRCAN_CONF_ENABLE0 : 0)
    | (priv.config.enable1 ? GRCAN_CONF_ENABLE1 : 0)
    | (priv.config.select ? GRCAN_CONF_SELECT : 0)
    | (priv.can.ctrlmode & CAN_CTRLMODE_LISTENONLY ?
    GRCAN_CONF_SILENT : 0)
    | (priv.can.ctrlmode & CAN_CTRLMODE_3_SAMPLES ?
    GRCAN_CONF_SAM : 0);
    grcan_write_bits(&regs.conf, confop, GRCAN_CONF_OPERATION);
    txctrl = GRCAN_TXCTRL_ENABLE
    | (priv.can.ctrlmode & CAN_CTRLMODE_ONE_SHOT
    ? GRCAN_TXCTRL_SINGLE : 0);
    grcan_write_reg(&regs.txctrl, txctrl);
    grcan_write_reg(&regs.rxctrl, GRCAN_RXCTRL_ENABLE);
    grcan_write_reg(&regs.ctrl, GRCAN_CTRL_ENABLE);
    priv.can.state = CAN_STATE_ERROR_ACTIVE;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn grcan_set_mode(dev: *mut net_device, mode: enum can_mode) -> c_int {
    static int grcan_set_mode(struct net_device *dev, enum can_mode mode)
    {
    struct grcan_priv *priv = netdev_priv(dev);
    unsigned long flags;
    let mut err: c_int = 0;
    if (mode == CAN_MODE_START) {
// This might be called to restart the device to recover from
// bus off errors
//
    spin_lock_irqsave(&priv.lock, flags);
    if (priv.closing || priv.resetting) {
    err = -EBUSY;
    } else {
    netdev_info(dev, "Restarting device\n");
    grcan_start(dev);
    if (!(priv.can.ctrlmode & CAN_CTRLMODE_LISTENONLY))
    netif_wake_queue(dev);
    }
    spin_unlock_irqrestore(&priv.lock, flags);
    return err;
    }
    return -EOPNOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn grcan_open(dev: *mut net_device) -> c_int {
    static int grcan_open(struct net_device *dev)
    {
    struct grcan_priv *priv = netdev_priv(dev);
    struct grcan_dma *dma = &priv.dma;
    unsigned long flags;
    int err;
// Allocate memory
    err = grcan_allocate_dma_buffers(dev, priv.config.txsize,
    priv.config.rxsize);
    if (err) {
    netdev_err(dev, "could not allocate DMA buffers\n");
    return err;
    }
    priv.echo_skb = kzalloc_objs(*priv.echo_skb, dma.tx.size);
    if (!priv.echo_skb) {
    err = -ENOMEM;
    goto exit_free_dma_buffers;
    }
    priv.can.echo_skb_max = dma.tx.size;
    priv.can.echo_skb = priv.echo_skb;
// Get can device up
    err = open_candev(dev);
    if (err)
    goto exit_free_echo_skb;
    err = request_irq(dev.irq, grcan_interrupt, IRQF_SHARED,
    dev.name, dev);
    if (err)
    goto exit_close_candev;
    napi_enable(&priv.napi);
    spin_lock_irqsave(&priv.lock, flags);
    grcan_start(dev);
    if (!(priv.can.ctrlmode & CAN_CTRLMODE_LISTENONLY))
    netif_start_queue(dev);
    priv.resetting = false;
    priv.closing = false;
    spin_unlock_irqrestore(&priv.lock, flags);
    return 0;
    exit_close_candev:
    close_candev(dev);
    exit_free_echo_skb:
    kfree(priv.echo_skb);
    exit_free_dma_buffers:
    grcan_free_dma_buffers(dev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn grcan_close(dev: *mut net_device) -> c_int {
    static int grcan_close(struct net_device *dev)
    {
    struct grcan_priv *priv = netdev_priv(dev);
    unsigned long flags;
    napi_disable(&priv.napi);
    spin_lock_irqsave(&priv.lock, flags);
    priv.closing = true;
    if (priv.need_txbug_workaround) {
    spin_unlock_irqrestore(&priv.lock, flags);
    timer_delete_sync(&priv.hang_timer);
    timer_delete_sync(&priv.rr_timer);
    spin_lock_irqsave(&priv.lock, flags);
    }
    netif_stop_queue(dev);
    grcan_stop_hardware(dev);
    priv.can.state = CAN_STATE_STOPPED;
    spin_unlock_irqrestore(&priv.lock, flags);
    free_irq(dev.irq, dev);
    close_candev(dev);
    grcan_free_dma_buffers(dev);
    priv.can.echo_skb_max = 0;
    priv.can.echo_skb = core::ptr::null_mut();
    kfree(priv.echo_skb);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn grcan_transmit_catch_up(dev: *mut net_device) {
    static void grcan_transmit_catch_up(struct net_device *dev)
    {
    struct grcan_priv *priv = netdev_priv(dev);
    unsigned long flags;
    int work_done;
    spin_lock_irqsave(&priv.lock, flags);
    work_done = catch_up_echo_skb(dev, -1, true);
    if (work_done) {
    if (!priv.resetting && !priv.closing &&
    !(priv.can.ctrlmode & CAN_CTRLMODE_LISTENONLY))
    netif_wake_queue(dev);
// With napi we don't get TX interrupts for a while,
// so prevent a running reset while catching up
//
    if (priv.need_txbug_workaround)
    timer_delete(&priv.hang_timer);
    }
    spin_unlock_irqrestore(&priv.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn grcan_receive(dev: *mut net_device, budget: c_int) -> c_int {
    static int grcan_receive(struct net_device *dev, int budget)
    {
    struct grcan_priv *priv = netdev_priv(dev);
    struct grcan_registers __iomem *regs = priv.regs;
    struct grcan_dma *dma = &priv.dma;
    struct net_device_stats *stats = &dev.stats;
    struct can_frame *cf;
    struct sk_buff *skb;
    u32 wr, rd, startrd;
    u32 *slot;
    u32 i, rtr, eff, j, shift;
    let mut work_done: c_int = 0;
    rd = grcan_read_reg(&regs.rxrd);
    startrd = rd;
    for (work_done = 0; work_done < budget; work_done++) {
// Check for packet to receive
    wr = grcan_read_reg(&regs.rxwr);
    if (rd == wr)
    break;
// Take care of packet
    skb = alloc_can_skb(dev, &cf);
    if (skb == core::ptr::null_mut()) {
    netdev_err(dev,
    "dropping frame: skb allocation failed\n");
    stats.rx_dropped++;
    continue;
    }
    slot = dma.rx.buf + rd;
    eff = slot[0] & GRCAN_MSG_IDE;
    rtr = slot[0] & GRCAN_MSG_RTR;
    if (eff) {
    cf.can_id = ((slot[0] & GRCAN_MSG_EID)
    >> GRCAN_MSG_EID_BIT);
    cf.can_id |= CAN_EFF_FLAG;
    } else {
    cf.can_id = ((slot[0] & GRCAN_MSG_BID)
    >> GRCAN_MSG_BID_BIT);
    }
    cf.len = can_cc_dlc2len((slot[1] & GRCAN_MSG_DLC)
    >> GRCAN_MSG_DLC_BIT);
    if (rtr) {
    cf.can_id |= CAN_RTR_FLAG;
    } else {
    for (i = 0; i < cf.len; i++) {
    j = GRCAN_MSG_DATA_SLOT_INDEX(i);
    shift = GRCAN_MSG_DATA_SHIFT(i);
    cf.data[i] = (u8)(slot[j] >> shift);
    }
    stats.rx_bytes += cf.len;
    }
    stats.rx_packets++;
    netif_receive_skb(skb);
    rd = grcan_ring_add(rd, GRCAN_MSG_SIZE, dma.rx.size);
    }
// Make sure everything is read before allowing hardware to
// use the memory
//
    mb();
// Update read pointer - no need to check for ongoing
    if (likely(rd != startrd))
    grcan_write_reg(&regs.rxrd, rd);
    return work_done;
    }
#[no_mangle]
unsafe extern "C" fn grcan_poll(napi: *mut napi_struct, budget: c_int) -> c_int {
    static int grcan_poll(struct napi_struct *napi, int budget)
    {
    struct grcan_priv *priv = container_of(napi, struct grcan_priv, napi);
    struct net_device *dev = priv.dev;
    struct grcan_registers __iomem *regs = priv.regs;
    unsigned long flags;
    int work_done;
    work_done = grcan_receive(dev, budget);
    grcan_transmit_catch_up(dev);
    if (work_done < budget) {
    napi_complete(napi);
// Guarantee no interference with a running reset that otherwise
// could turn off interrupts.
//
    spin_lock_irqsave(&priv.lock, flags);
// Enable tx and rx interrupts again. No need to check
// priv->closing as napi_disable in grcan_close is waiting for
// scheduled napi calls to finish.
//
    grcan_set_bits(&regs.imr, GRCAN_IRQ_TX | GRCAN_IRQ_RX);
    spin_unlock_irqrestore(&priv.lock, flags);
    }
    return work_done;
    }
// Work tx bug by waiting while for the risky situation to clear. If that fails,
// drop a frame in one-shot mode or indicate a busy device otherwise.
//
// Returns 0 on successful wait. Otherwise it sets *netdev_tx_status to the
// value that should be returned by grcan_start_xmit when aborting the xmit.
//
    static int grcan_txbug_workaround(struct net_device *dev, struct sk_buff *skb,
    u32 txwr, u32 oneshotmode,
    netdev_tx_t *netdev_tx_status)
    {
    struct grcan_priv *priv = netdev_priv(dev);
    struct grcan_registers __iomem *regs = priv.regs;
    struct grcan_dma *dma = &priv.dma;
    int i;
    unsigned long flags;
// Wait a while for ongoing to be cleared or read pointer to catch up to
// write pointer. The latter is needed due to a bug in older versions of
// GRCAN in which ONGOING is not cleared properly one-shot mode when a
// transmission fails.
//
    for (i = 0; i < GRCAN_SHORTWAIT_USECS; i++) {
    udelay(1);
    if (!grcan_read_bits(&regs.txctrl, GRCAN_TXCTRL_ONGOING) ||
    grcan_read_reg(&regs.txrd) == txwr) {
    return 0;
    }
    }
// Clean up, in case the situation was not resolved
    spin_lock_irqsave(&priv.lock, flags);
    if (!priv.resetting && !priv.closing) {
// Queue might have been stopped earlier in grcan_start_xmit
    if (grcan_txspace(dma.tx.size, txwr, priv.eskbp))
    netif_wake_queue(dev);
// Set a timer to resolve a hanged tx controller
    if (!timer_pending(&priv.hang_timer))
    grcan_reset_timer(&priv.hang_timer,
    priv.can.bittiming.bitrate);
    }
    spin_unlock_irqrestore(&priv.lock, flags);
    if (oneshotmode) {
// In one-shot mode we should never end up here because
// then the interrupt handler increases txrd on TXLOSS,
// but it is consistent with one-shot mode to drop the
// frame in this case.
//
    kfree_skb(skb);
// netdev_tx_status = NETDEV_TX_OK;
    } else {
// In normal mode the socket-can transmission queue get
// to keep the frame so that it can be retransmitted
// later
//
// netdev_tx_status = NETDEV_TX_BUSY;
    }
    return -EBUSY;
    }
// Notes on the tx cyclic buffer handling:
//
// regs->txwr	- the next slot for the driver to put data to be sent
// regs->txrd	- the next slot for the device to read data
// priv->eskbp	- the next slot for the driver to call can_put_echo_skb for
//
// grcan_start_xmit can enter more messages as long as regs->txwr does
// not reach priv->eskbp (within 1 message gap)
//
// The device sends messages until regs->txrd reaches regs->txwr
//
// The interrupt calls handler calls can_put_echo_skb until
// priv->eskbp reaches regs->txrd
//
    static netdev_tx_t grcan_start_xmit(struct sk_buff *skb,
    struct net_device *dev)
    {
    struct grcan_priv *priv = netdev_priv(dev);
    struct grcan_registers __iomem *regs = priv.regs;
    struct grcan_dma *dma = &priv.dma;
    struct can_frame *cf = (struct can_frame *)skb.data;
    u32 id, txwr, txrd, space, txctrl;
    int slotindex;
    u32 *slot;
    u32 i, rtr, eff, dlc, tmp, err;
    int j, shift;
    unsigned long flags;
    let mut oneshotmode: u32 = priv.can.ctrlmode & CAN_CTRLMODE_ONE_SHOT;
    if (can_dev_dropped_skb(dev, skb))
    return NETDEV_TX_OK;
// Trying to transmit in silent mode will generate error interrupts, but
// this should never happen - the queue should not have been started.
//
    if (priv.can.ctrlmode & CAN_CTRLMODE_LISTENONLY)
    return NETDEV_TX_BUSY;
// Reads of priv->eskbp and shut-downs of the queue needs to
// be atomic towards the updates to priv->eskbp and wake-ups
// of the queue in the interrupt handler.
//
    spin_lock_irqsave(&priv.lock, flags);
    txwr = grcan_read_reg(&regs.txwr);
    space = grcan_txspace(dma.tx.size, txwr, priv.eskbp);
    slotindex = txwr / GRCAN_MSG_SIZE;
    slot = dma.tx.buf + txwr;
    if (unlikely(space == 1))
    netif_stop_queue(dev);
    spin_unlock_irqrestore(&priv.lock, flags);
// End of critical section
// This should never happen. If circular buffer is full, the
// netif_stop_queue should have been stopped already.
//
    if (unlikely(!space)) {
    netdev_err(dev, "No buffer space, but queue is non-stopped.\n");
    return NETDEV_TX_BUSY;
    }
// Convert and write CAN message to DMA buffer
    eff = cf.can_id & CAN_EFF_FLAG;
    rtr = cf.can_id & CAN_RTR_FLAG;
    id = cf.can_id & (eff ? CAN_EFF_MASK : CAN_SFF_MASK);
    dlc = cf.len;
    if (eff)
    tmp = (id << GRCAN_MSG_EID_BIT) & GRCAN_MSG_EID;
    else
    tmp = (id << GRCAN_MSG_BID_BIT) & GRCAN_MSG_BID;
    slot[0] = (eff ? GRCAN_MSG_IDE : 0) | (rtr ? GRCAN_MSG_RTR : 0) | tmp;
    slot[1] = ((dlc << GRCAN_MSG_DLC_BIT) & GRCAN_MSG_DLC);
    slot[2] = 0;
    slot[3] = 0;
    for (i = 0; i < dlc; i++) {
    j = GRCAN_MSG_DATA_SLOT_INDEX(i);
    shift = GRCAN_MSG_DATA_SHIFT(i);
    slot[j] |= cf.data[i] << shift;
    }
// Checking that channel has not been disabled. These cases
// should never happen
//
    txctrl = grcan_read_reg(&regs.txctrl);
    if (!(txctrl & GRCAN_TXCTRL_ENABLE))
    netdev_err(dev, "tx channel spuriously disabled\n");
    if (oneshotmode && !(txctrl & GRCAN_TXCTRL_SINGLE))
    netdev_err(dev, "one-shot mode spuriously disabled\n");
// Bug workaround for old version of grcan where updating txwr
// in the same clock cycle as the controller updates txrd to
// the current txwr could hang the can controller
//
    if (priv.need_txbug_workaround) {
    txrd = grcan_read_reg(&regs.txrd);
    if (unlikely(grcan_ring_sub(txwr, txrd, dma.tx.size) == 1)) {
    netdev_tx_t txstatus;
    err = grcan_txbug_workaround(dev, skb, txwr,
    oneshotmode, &txstatus);
    if (err)
    return txstatus;
    }
    }
// Prepare skb for echoing. This must be after the bug workaround above
// as ownership of the skb is passed on by calling can_put_echo_skb.
// Returning NETDEV_TX_BUSY or accessing skb or cf after a call to
// can_put_echo_skb would be an error unless other measures are
// taken.
//
    can_put_echo_skb(skb, dev, slotindex, 0);
// Make sure everything is written before allowing hardware to
// read from the memory
//
    wmb();
// Update write pointer to start transmission
    grcan_write_reg(&regs.txwr,
    grcan_ring_add(txwr, GRCAN_MSG_SIZE, dma.tx.size));
    return NETDEV_TX_OK;
    }
// ========== Setting up sysfs interface and module parameters ==========

    static void grcan_sanitize_##name(struct platform_device *pd)	\
    {								\
    struct grcan_device_config grcan_default_config		\
    = GRCAN_DEFAULT_DEVICE_CONFIG;			\
    if (valcheckf(grcan_module_config.name)) {		\
    dev_err(&pd.dev,				\
    "Invalid module parameter value for "	\

    grcan_module_config.name =			\
    grcan_default_config.name;		\
    }							\
    }								\
    module_param_named(name, grcan_module_config.name,		\
    mtype, 0444);				\
    MODULE_PARM_DESC(name, desc)

    static ssize_t grcan_store_##name(struct device *sdev,		\
    struct device_attribute *att,	\
    const char *buf,		\
    size_t count)			\
    {								\
    struct net_device *dev = to_net_dev(sdev);		\
    struct grcan_priv *priv = netdev_priv(dev);		\
    u8 val;							\
    int ret;						\
    if (dev.flags & IFF_UP)				\
    return -EBUSY;					\
    ret = kstrtou8(buf, 0, &val);				\
    if (ret < 0 || val > 1)					\
    return -EINVAL;					\
    priv.config.name = val;				\
    return count;						\
    }								\
    static ssize_t grcan_show_##name(struct device *sdev,		\
    struct device_attribute *att,	\
    char *buf)			\
    {								\
    struct net_device *dev = to_net_dev(sdev);		\
    struct grcan_priv *priv = netdev_priv(dev);		\
    return sprintf(buf, "%d\n", priv.config.name);		\
    }								\
    static DEVICE_ATTR(name, 0644,					\
    grcan_show_##name,				\
    grcan_store_##name);				\
    GRCAN_MODULE_PARAM(name, ushort, GRCAN_NOT_BOOL, desc)
// The following configuration options are made available both via module
// parameters and writable sysfs files. See the chapter about GRCAN in the
// documentation for the GRLIB VHDL library for further details.
//
    GRCAN_CONFIG_ATTR(enable0,
    "Configuration of physical interface 0. Determines\n"	\
    "the \"Enable 0\" bit of the configuration register.\n" \
    "Format: 0 | 1\nDefault: 0\n");
    GRCAN_CONFIG_ATTR(enable1,
    "Configuration of physical interface 1. Determines\n"	\
    "the \"Enable 1\" bit of the configuration register.\n" \
    "Format: 0 | 1\nDefault: 0\n");
    GRCAN_CONFIG_ATTR(select,
    "Select which physical interface to use.\n"	\
    "Format: 0 | 1\nDefault: 0\n");
// The tx and rx buffer size configuration options are only available via module
// parameters.
//
    GRCAN_MODULE_PARAM(txsize, uint, GRCAN_INVALID_BUFFER_SIZE,
    "Sets the size of the tx buffer.\n"			\
    "Format: <unsigned int> where (txsize & ~0x1fffc0) == 0\n" \
    "Default: 1024\n");
    GRCAN_MODULE_PARAM(rxsize, uint, GRCAN_INVALID_BUFFER_SIZE,
    "Sets the size of the rx buffer.\n"			\
    "Format: <unsigned int> where (size & ~0x1fffc0) == 0\n" \
    "Default: 1024\n");
// Function that makes sure that configuration done using
// module parameters are set to valid values
//
#[no_mangle]
unsafe extern "C" fn grcan_sanitize_module_config(ofdev: *mut platform_device) {
    static void grcan_sanitize_module_config(struct platform_device *ofdev)
    {
    grcan_sanitize_enable0(ofdev);
    grcan_sanitize_enable1(ofdev);
    grcan_sanitize_select(ofdev);
    grcan_sanitize_txsize(ofdev);
    grcan_sanitize_rxsize(ofdev);
    }
    static const struct attribute *const sysfs_grcan_attrs[] = {
// Config attrs
    &dev_attr_enable0.attr,
    &dev_attr_enable1.attr,
    &dev_attr_select.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group sysfs_grcan_group = {
    .name	= "grcan",
    .attrs	= (struct attribute **)sysfs_grcan_attrs,
    };
// ========== Setting up the driver ==========
    static const struct net_device_ops grcan_netdev_ops = {
    .ndo_open	= grcan_open,
    .ndo_stop	= grcan_close,
    .ndo_start_xmit	= grcan_start_xmit,
    };
    static const struct ethtool_ops grcan_ethtool_ops = {
    .get_ts_info = ethtool_op_get_ts_info,
    };
    static int grcan_setup_netdev(struct platform_device *ofdev,
    void __iomem *base,
    int irq, u32 ambafreq, bool txbug)
    {
    struct net_device *dev;
    struct grcan_priv *priv;
    struct grcan_registers __iomem *regs;
    int err;
    dev = alloc_candev(sizeof(struct grcan_priv), 0);
    if (!dev)
    return -ENOMEM;
    dev.irq = irq;
    dev.flags |= IFF_ECHO;
    dev.netdev_ops = &grcan_netdev_ops;
    dev.ethtool_ops = &grcan_ethtool_ops;
    dev.sysfs_groups[0] = &sysfs_grcan_group;
    priv = netdev_priv(dev);
    memcpy(&priv.config, &grcan_module_config,
    sizeof(struct grcan_device_config));
    priv.dev = dev;
    priv.ofdev_dev = &ofdev.dev;
    priv.regs = base;
    priv.can.bittiming_const = &grcan_bittiming_const;
    priv.can.do_set_bittiming = grcan_set_bittiming;
    priv.can.do_set_mode = grcan_set_mode;
    priv.can.do_get_berr_counter = grcan_get_berr_counter;
    priv.can.clock.freq = ambafreq;
    priv.can.ctrlmode_supported =
    CAN_CTRLMODE_LISTENONLY | CAN_CTRLMODE_ONE_SHOT;
    priv.need_txbug_workaround = txbug;
// Discover if triple sampling is supported by hardware
    regs = priv.regs;
    grcan_set_bits(&regs.ctrl, GRCAN_CTRL_RESET);
    grcan_set_bits(&regs.conf, GRCAN_CONF_SAM);
    if (grcan_read_bits(&regs.conf, GRCAN_CONF_SAM)) {
    priv.can.ctrlmode_supported |= CAN_CTRLMODE_3_SAMPLES;
    dev_dbg(&ofdev.dev, "Hardware supports triple-sampling\n");
    }
    spin_lock_init(&priv.lock);
    if (priv.need_txbug_workaround) {
    timer_setup(&priv.rr_timer, grcan_running_reset, 0);
    timer_setup(&priv.hang_timer, grcan_initiate_running_reset, 0);
    }
    netif_napi_add_weight(dev, &priv.napi, grcan_poll, GRCAN_NAPI_WEIGHT);
    SET_NETDEV_DEV(dev, &ofdev.dev);
    dev_info(&ofdev.dev, "regs=0x%p, irq=%d, clock=%d\n",
    priv.regs, dev.irq, priv.can.clock.freq);
    err = register_candev(dev);
    if (err)
    goto exit_free_candev;
    platform_set_drvdata(ofdev, dev);
// Reset device to allow bit-timing to be set. No need to call
// grcan_reset at this stage. That is done in grcan_open.
//
    grcan_write_reg(&regs.ctrl, GRCAN_CTRL_RESET);
    return 0;
    exit_free_candev:
    free_candev(dev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn grcan_probe(ofdev: *mut platform_device) -> c_int {
    static int grcan_probe(struct platform_device *ofdev)
    {
    struct device_node *np = ofdev.dev.of_node;
    struct device_node *sysid_parent;
    u32 sysid, ambafreq;
    int irq, err;
    void __iomem *base;
    let mut txbug: bool = true;
// Compare GRLIB version number with the first that does not
// have the tx bug (see start_xmit)
//
    sysid_parent = of_find_node_by_path("/ambapp0");
    if (sysid_parent) {
    err = of_property_read_u32(sysid_parent, "systemid", &sysid);
    if (!err && ((sysid & GRLIB_VERSION_MASK) >=
    GRCAN_TXBUG_SAFE_GRLIB_VERSION))
    txbug = false;
    of_node_put(sysid_parent);
    }
    err = of_property_read_u32(np, "freq", &ambafreq);
    if (err) {
    dev_err(&ofdev.dev, "unable to fetch \"freq\" property\n");
    goto exit_error;
    }
    base = devm_platform_ioremap_resource(ofdev, 0);
    if (IS_ERR(base)) {
    err = PTR_ERR(base);
    goto exit_error;
    }
    irq = irq_of_parse_and_map(np, GRCAN_IRQIX_IRQ);
    if (!irq) {
    dev_err(&ofdev.dev, "no irq found\n");
    err = -ENODEV;
    goto exit_error;
    }
    grcan_sanitize_module_config(ofdev);
    err = grcan_setup_netdev(ofdev, base, irq, ambafreq, txbug);
    if (err)
    goto exit_dispose_irq;
    return 0;
    exit_dispose_irq:
    irq_dispose_mapping(irq);
    exit_error:
    dev_err(&ofdev.dev,
    "%s socket CAN driver initialization failed with error %d\n",
    DRV_NAME, err);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn grcan_remove(ofdev: *mut platform_device) {
    static void grcan_remove(struct platform_device *ofdev)
    {
    struct net_device *dev = platform_get_drvdata(ofdev);
    struct grcan_priv *priv = netdev_priv(dev);
    unregister_candev(dev); /* Will in turn call grcan_close */
    irq_dispose_mapping(dev.irq);
    netif_napi_del(&priv.napi);
    free_candev(dev);
    }
    static const struct of_device_id grcan_match[] = {
    {.name = "GAISLER_GRCAN"},
    {.name = "01_03d"},
    {.name = "GAISLER_GRHCAN"},
    {.name = "01_034"},
    {},
    };
    MODULE_DEVICE_TABLE(of, grcan_match);
    static struct platform_driver grcan_driver = {
    .driver = {
    .name = DRV_NAME,
    .of_match_table = grcan_match,
    },
    .probe = grcan_probe,
    .remove = grcan_remove,
    };
    module_platform_driver(grcan_driver);
    MODULE_AUTHOR("Aeroflex Gaisler AB.");
    MODULE_DESCRIPTION("Socket CAN driver for Aeroflex Gaisler GRCAN");
    MODULE_LICENSE("GPL");
