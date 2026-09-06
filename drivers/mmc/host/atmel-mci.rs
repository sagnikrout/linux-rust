//! Automatically rewritten from C to Rust
//! Source: drivers/mmc/host/atmel-mci.c
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
// Atmel MultiMedia Card Interface driver
//
// Copyright (C) 2004-2008 Atmel Corporation
//

pub const ATMCI_MAX_NR_SLOTS: c_int = 2;
//
// Superset of MCI IP registers integrated in Atmel AT91 Processor
// Registers and bitfields marked with [2] are only available in MCI2
//
// MCI Register Definitions
pub const ATMCI_CR: c_uint = 0x0000	/* Control */;

pub const ATMCI_MR: c_uint = 0x0004	/* Mode */;

pub const ATMCI_DTOR: c_uint = 0x0008	/* Data Timeout */;

pub const ATMCI_SDCR: c_uint = 0x000c	/* SD Card / SDIO */;

pub const ATMCI_ARGR: c_uint = 0x0010	/* Command Argument */;
pub const ATMCI_CMDR: c_uint = 0x0014	/* Command */;

pub const ATMCI_BLKR: c_uint = 0x0018	/* Block */;

pub const ATMCI_CSTOR: c_uint = 0x001c	/* Completion Signal Timeout[2] */;

pub const ATMCI_RSPR: c_uint = 0x0020	/* Response 0 */;
pub const ATMCI_RSPR1: c_uint = 0x0024	/* Response 1 */;
pub const ATMCI_RSPR2: c_uint = 0x0028	/* Response 2 */;
pub const ATMCI_RSPR3: c_uint = 0x002c	/* Response 3 */;
pub const ATMCI_RDR: c_uint = 0x0030	/* Receive Data */;
pub const ATMCI_TDR: c_uint = 0x0034	/* Transmit Data */;
pub const ATMCI_SR: c_uint = 0x0040	/* Status */;
pub const ATMCI_IER: c_uint = 0x0044	/* Interrupt Enable */;
pub const ATMCI_IDR: c_uint = 0x0048	/* Interrupt Disable */;
pub const ATMCI_IMR: c_uint = 0x004c	/* Interrupt Mask */;

pub const ATMCI_DMA: c_uint = 0x0050	/* DMA Configuration[2] */;

pub const ATMCI_CFG: c_uint = 0x0054	/* Configuration[2] */;

pub const ATMCI_WPMR: c_uint = 0x00e4	/* Write Protection Mode[2] */;

pub const ATMCI_WPSR: c_uint = 0x00e8	/* Write Protection Status[2] */;

pub const ATMCI_VERSION: c_uint = 0x00FC  /* Version */;
pub const ATMCI_FIFO_APERTURE: c_uint = 0x0200	/* FIFO Aperture[2] */;
// This is not including the FIFO Aperture on MCI2
pub const ATMCI_REGS_SIZE: c_uint = 0x100;
// Register access macros

    __raw_readl((port).regs + reg)

    __raw_writel((value), (port).regs + reg)
pub const ATMCI_CMD_TIMEOUT_MS: c_int = 2000;
pub const AUTOSUSPEND_DELAY: c_int = 50;

pub const ATMCI_DMA_THRESHOLD: c_int = 16;
    enum {
    EVENT_CMD_RDY = 0,
    EVENT_XFER_COMPLETE,
    EVENT_NOTBUSY,
    EVENT_DATA_ERROR,
    };
    enum atmel_mci_state {
    STATE_IDLE = 0,
    STATE_SENDING_CMD,
    STATE_DATA_XFER,
    STATE_WAITING_NOTBUSY,
    STATE_SENDING_STOP,
    STATE_END_REQUEST,
    };
    enum atmci_xfer_dir {
    XFER_RECEIVE = 0,
    XFER_TRANSMIT,
    };
    enum atmci_pdc_buf {
    PDC_FIRST_BUF = 0,
    PDC_SECOND_BUF,
    };
//
// struct mci_slot_pdata - board-specific per-slot configuration
// @bus_width: Number of data lines wired up the slot
// @detect_pin: GPIO pin wired to the card detect switch
// @wp_pin: GPIO pin wired to the write protect sensor
// @non_removable: The slot is not removable, only detect once
//
// If a given slot is not present on the board, @bus_width should be
// set to 0. The other fields are ignored in this case.
//
// Any pins that aren't available should be set to a negative value.
//
// Note that support for multiple slots is experimental -- some cards
// might get upset if we don't get the clock management exactly right.
// But in most cases, it should work just fine.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mci_slot_pdata {
    pub bus_width: c_uint,
    pub detect_pin: *mut gpio_desc,
    pub wp_pin: *mut gpio_desc,
    pub non_removable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_mci_caps {
    pub has_dma_conf_reg: bool,
    pub has_pdc: bool,
    pub has_cfg_reg: bool,
    pub has_cstor_reg: bool,
    pub has_highspeed: bool,
    pub has_rwproof: bool,
    pub has_odd_clk_div: bool,
    pub has_bad_data_ordering: bool,
    pub need_reset_after_xfer: bool,
    pub need_blksz_mul_4: bool,
    pub need_notbusy_for_read_ops: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_mci_dma {
    pub chan: *mut dma_chan,
    pub data_desc: *mut dma_async_tx_descriptor,
}

//
// struct atmel_mci - MMC controller state shared between all slots
// @lock: Spinlock protecting the queue and associated data.
// @regs: Pointer to MMIO registers.
// @sg: Scatterlist entry currently being processed by PIO or PDC code.
// @sg_len: Size of the scatterlist
// @pio_offset: Offset into the current scatterlist entry.
// @buffer: Buffer used if we don't have the r/w proof capability. We
// don't have the time to switch pdc buffers so we have to use only
// one buffer for the full transaction.
// @buf_size: size of the buffer.
// @buf_phys_addr: buffer address needed for pdc.
// @cur_slot: The slot which is currently using the controller.
// @mrq: The request currently being processed on @cur_slot,
// or NULL if the controller is idle.
// @cmd: The command currently being sent to the card, or NULL.
// @data: The data currently being transferred, or NULL if no data
// transfer is in progress.
// @data_size: just data->blocks * data->blksz.
// @dma: DMA client state.
// @data_chan: DMA channel being used for the current data transfer.
// @dma_conf: Configuration for the DMA slave
// @cmd_status: Snapshot of SR taken upon completion of the current
// command. Only valid when EVENT_CMD_COMPLETE is pending.
// @data_status: Snapshot of SR taken upon completion of the current
// data transfer. Only valid when EVENT_DATA_COMPLETE or
// EVENT_DATA_ERROR is pending.
// @stop_cmdr: Value to be loaded into CMDR when the stop command is
// to be sent.
// @bh_work: Work running the request state machine.
// @pending_events: Bitmask of events flagged by the interrupt handler
// to be processed by the work.
// @completed_events: Bitmask of events which the state machine has
// processed.
// @state: Work state.
// @queue: List of slots waiting for access to the controller.
// @need_clock_update: Update the clock rate before the next request.
// @need_reset: Reset controller before next request.
// @timer: Timer to balance the data timeout error flag which cannot rise.
// @mode_reg: Value of the MR register.
// @cfg_reg: Value of the CFG register.
// @bus_hz: The rate of @mck in Hz. This forms the basis for MMC bus
// rate and timeout calculations.
// @mapbase: Physical address of the MMIO registers.
// @mck: The peripheral bus clock hooked up to the MMC controller.
// @dev: Device associated with the MMC controller.
// @pdata: Per-slot configuration data.
// @slot: Slots sharing this MMC controller.
// @caps: MCI capabilities depending on MCI version.
// @prepare_data: function to setup MCI before data transfer which
// depends on MCI capabilities.
// @submit_data: function to start data transfer which depends on MCI
// capabilities.
// @stop_transfer: function to stop data transfer which depends on MCI
// capabilities.
//
// Locking
// =======
//
// @lock is a softirq-safe spinlock protecting @queue as well as
// @cur_slot, @mrq and @state. These must always be updated
// at the same time while holding @lock.
//
// @lock also protects mode_reg and need_clock_update since these are
// used to synchronize mode register updates with the queue
// processing.
//
// The @mrq field of struct atmel_mci_slot is also protected by @lock,
// and must always be written at the same time as the slot is added to
// @queue.
//
// @pending_events and @completed_events are accessed using atomic bit
// operations, so they don't need any locking.
//
// None of the fields touched by the interrupt handler need any
// locking. However, ordering is important: Before EVENT_DATA_ERROR or
// EVENT_DATA_COMPLETE is set in @pending_events, all data-related
// interrupts must be disabled and @data_status updated with a
// snapshot of SR. Similarly, before EVENT_CMD_COMPLETE is set, the
// CMDRDY interrupt must be disabled and @cmd_status updated with a
// snapshot of SR, and before EVENT_XFER_COMPLETE can be set, the
// bytes_xfered field of @data must be written. This is ensured by
// using barriers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_mci {
    pub lock: spinlock_t,
    pub regs: *mut void __iomem,
    pub sg: *mut scatterlist,
    pub sg_len: c_uint,
    pub pio_offset: c_uint,
    pub buffer: *mut c_uint,
    pub buf_size: c_uint,
    pub buf_phys_addr: dma_addr_t,
    pub cur_slot: *mut atmel_mci_slot,
    pub mrq: *mut mmc_request,
    pub cmd: *mut mmc_command,
    pub data: *mut mmc_data,
    pub data_size: c_uint,
    pub dma: atmel_mci_dma,
    pub data_chan: *mut dma_chan,
    pub dma_conf: dma_slave_config,
    pub cmd_status: u32,
    pub data_status: u32,
    pub stop_cmdr: u32,
    pub bh_work: work_struct,
    pub pending_events: c_ulong,
    pub completed_events: c_ulong,
    pub state: enum atmel_mci_state,
    pub queue: list_head,
    pub need_clock_update: bool,
    pub need_reset: bool,
    pub timer: timer_list,
    pub mode_reg: u32,
    pub cfg_reg: u32,
    pub bus_hz: c_ulong,
    pub mapbase: c_ulong,
    pub mck: *mut clk,
    pub dev: *mut device,
    pub pdata: [mci_slot_pdata; ATMCI_MAX_NR_SLOTS],
    pub slot: [*mut atmel_mci_slot; ATMCI_MAX_NR_SLOTS],
    pub caps: atmel_mci_caps,
    pub data): *mut *mut *mut u32 (prepare_data)(struct atmel_mci host, struct mmc_data,
    pub data): *mut *mut *mut void (submit_data)(struct atmel_mci host, struct mmc_data,
    pub host): *mut *mut void (stop_transfer)(struct atmel_mci,
}

//
// struct atmel_mci_slot - MMC slot state
// @mmc: The mmc_host representing this slot.
// @host: The MMC controller this slot is using.
// @sdc_reg: Value of SDCR to be written before using this slot.
// @sdio_irq: SDIO irq mask for this slot.
// @mrq: mmc_request currently being processed or waiting to be
// processed, or NULL when the slot is idle.
// @queue_node: List node for placing this node in the @queue list of
// &struct atmel_mci.
// @clock: Clock rate configured by set_ios(). Protected by host->lock.
// @flags: Random state bits associated with the slot.
// @detect_pin: GPIO pin used for card detection, or negative if not
// available.
// @wp_pin: GPIO pin used for card write protect sending, or negative
// if not available.
// @detect_timer: Timer used for debouncing @detect_pin interrupts.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_mci_slot {
    pub mmc: *mut mmc_host,
    pub host: *mut atmel_mci,
    pub sdc_reg: u32,
    pub sdio_irq: u32,
    pub mrq: *mut mmc_request,
    pub queue_node: list_head,
    pub clock: c_uint,
    pub flags: c_ulong,
pub const ATMCI_CARD_PRESENT: c_int = 0;
pub const ATMCI_CARD_NEED_INIT: c_int = 1;
pub const ATMCI_SHUTDOWN: c_int = 2;
    pub detect_pin: *mut gpio_desc,
    pub wp_pin: *mut gpio_desc,
    pub detect_timer: timer_list,
}

    test_and_clear_bit(event, &host.pending_events)

    set_bit(event, &host.completed_events)

    set_bit(event, &host.pending_events)
//
// The debugfs stuff below is mostly optimized away when
// CONFIG_DEBUG_FS is not set.
//
#[no_mangle]
unsafe extern "C" fn atmci_req_show(s: *mut seq_file, v: *mut c_void) -> c_int {
    static int atmci_req_show(struct seq_file *s, void *v)
    {
    struct atmel_mci_slot	*slot = s.private;
    struct mmc_request	*mrq;
    struct mmc_command	*cmd;
    struct mmc_command	*stop;
    struct mmc_data		*data;
// Make sure we get a consistent snapshot
    spin_lock_bh(&slot.host.lock);
    mrq = slot.mrq;
    if (mrq) {
    cmd = mrq.cmd;
    data = mrq.data;
    stop = mrq.stop;
    if (cmd)
    seq_printf(s,
    "CMD%u(0x%x) flg %x rsp %x %x %x %x err %d\n",
    cmd.opcode, cmd.arg, cmd.flags,
    cmd.resp[0], cmd.resp[1], cmd.resp[2],
    cmd.resp[3], cmd.error);
    if (data)
    seq_printf(s, "DATA %u / %u * %u flg %x err %d\n",
    data.bytes_xfered, data.blocks,
    data.blksz, data.flags, data.error);
    if (stop)
    seq_printf(s,
    "CMD%u(0x%x) flg %x rsp %x %x %x %x err %d\n",
    stop.opcode, stop.arg, stop.flags,
    stop.resp[0], stop.resp[1], stop.resp[2],
    stop.resp[3], stop.error);
    }
    spin_unlock_bh(&slot.host.lock);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(atmci_req);
    static void atmci_show_status_reg(struct seq_file *s,
    const char *regname, u32 value)
    {
    static const char	*sr_bit[] = {
    [0]	= "CMDRDY",
    [1]	= "RXRDY",
    [2]	= "TXRDY",
    [3]	= "BLKE",
    [4]	= "DTIP",
    [5]	= "NOTBUSY",
    [6]	= "ENDRX",
    [7]	= "ENDTX",
    [8]	= "SDIOIRQA",
    [9]	= "SDIOIRQB",
    [12]	= "SDIOWAIT",
    [14]	= "RXBUFF",
    [15]	= "TXBUFE",
    [16]	= "RINDE",
    [17]	= "RDIRE",
    [18]	= "RCRCE",
    [19]	= "RENDE",
    [20]	= "RTOE",
    [21]	= "DCRCE",
    [22]	= "DTOE",
    [23]	= "CSTOE",
    [24]	= "BLKOVRE",
    [25]	= "DMADONE",
    [26]	= "FIFOEMPTY",
    [27]	= "XFRDONE",
    [30]	= "OVRE",
    [31]	= "UNRE",
    };
    unsigned int		i;
    seq_printf(s, "%s:\t0x%08x", regname, value);
    for (i = 0; i < ARRAY_SIZE(sr_bit); i++) {
    if (value & (1 << i)) {
    if (sr_bit[i])
    seq_printf(s, " %s", sr_bit[i]);
    else
    seq_puts(s, " UNKNOWN");
    }
    }
    seq_putc(s, '\n');
    }
#[no_mangle]
unsafe extern "C" fn atmci_regs_show(s: *mut seq_file, v: *mut c_void) -> c_int {
    static int atmci_regs_show(struct seq_file *s, void *v)
    {
    struct atmel_mci	*host = s.private;
    struct device		*dev = host.dev;
    u32			*buf;
    let mut ret: c_int = 0;
    buf = kmalloc(ATMCI_REGS_SIZE, GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
    pm_runtime_get_sync(dev);
//
// Grab a more or less consistent snapshot. Note that we're
// not disabling interrupts, so IMR and SR may not be
// consistent.
//
    spin_lock_bh(&host.lock);
    memcpy_fromio(buf, host.regs, ATMCI_REGS_SIZE);
    spin_unlock_bh(&host.lock);
    pm_runtime_put_autosuspend(dev);
    seq_printf(s, "MR:\t0x%08x%s%s ",
    buf[ATMCI_MR / 4],
    buf[ATMCI_MR / 4] & ATMCI_MR_RDPROOF ? " RDPROOF" : "",
    buf[ATMCI_MR / 4] & ATMCI_MR_WRPROOF ? " WRPROOF" : "");
    if (host.caps.has_odd_clk_div)
    seq_printf(s, "{CLKDIV,CLKODD}=%u\n",
    ((buf[ATMCI_MR / 4] & 0xff) << 1)
    | ((buf[ATMCI_MR / 4] >> 16) & 1));
    else
    seq_printf(s, "CLKDIV=%u\n",
    (buf[ATMCI_MR / 4] & 0xff));
    seq_printf(s, "DTOR:\t0x%08x\n", buf[ATMCI_DTOR / 4]);
    seq_printf(s, "SDCR:\t0x%08x\n", buf[ATMCI_SDCR / 4]);
    seq_printf(s, "ARGR:\t0x%08x\n", buf[ATMCI_ARGR / 4]);
    seq_printf(s, "BLKR:\t0x%08x BCNT=%u BLKLEN=%u\n",
    buf[ATMCI_BLKR / 4],
    buf[ATMCI_BLKR / 4] & 0xffff,
    (buf[ATMCI_BLKR / 4] >> 16) & 0xffff);
    if (host.caps.has_cstor_reg)
    seq_printf(s, "CSTOR:\t0x%08x\n", buf[ATMCI_CSTOR / 4]);
// Don't read RSPR and RDR; it will consume the data there
    atmci_show_status_reg(s, "SR", buf[ATMCI_SR / 4]);
    atmci_show_status_reg(s, "IMR", buf[ATMCI_IMR / 4]);
    if (host.caps.has_dma_conf_reg) {
    u32 val;
    val = buf[ATMCI_DMA / 4];
    seq_printf(s, "DMA:\t0x%08x OFFSET=%u CHKSIZE=%u%s\n",
    val, val & 3,
    ((val >> 4) & 3) ?
    1 << (((val >> 4) & 3) + 1) : 1,
    val & ATMCI_DMAEN ? " DMAEN" : "");
    }
    if (host.caps.has_cfg_reg) {
    u32 val;
    val = buf[ATMCI_CFG / 4];
    seq_printf(s, "CFG:\t0x%08x%s%s%s%s\n",
    val,
    val & ATMCI_CFG_FIFOMODE_1DATA ? " FIFOMODE_ONE_DATA" : "",
    val & ATMCI_CFG_FERRCTRL_COR ? " FERRCTRL_CLEAR_ON_READ" : "",
    val & ATMCI_CFG_HSMODE ? " HSMODE" : "",
    val & ATMCI_CFG_LSYNC ? " LSYNC" : "");
    }
    kfree(buf);
    return ret;
    }
    DEFINE_SHOW_ATTRIBUTE(atmci_regs);
#[no_mangle]
unsafe extern "C" fn atmci_init_debugfs(slot: *mut atmel_mci_slot) {
    static void atmci_init_debugfs(struct atmel_mci_slot *slot)
    {
    struct mmc_host		*mmc = slot.mmc;
    struct atmel_mci	*host = slot.host;
    struct dentry		*root;
    root = mmc.debugfs_root;
    if (!root)
    return;
    debugfs_create_file("regs", 0400, root, host, &atmci_regs_fops);
    debugfs_create_file("req", 0400, root, slot, &atmci_req_fops);
    debugfs_create_u32("state", 0400, root, &host.state);
    debugfs_create_xul("pending_events", 0400, root,
    &host.pending_events);
    debugfs_create_xul("completed_events", 0400, root,
    &host.completed_events);
    }
    static const struct of_device_id atmci_dt_ids[] = {
    { .compatible = "atmel,hsmci" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, atmci_dt_ids);
#[no_mangle]
unsafe extern "C" fn atmci_of_init(host: *mut atmel_mci) -> c_int {
    static int atmci_of_init(struct atmel_mci *host)
    {
    struct device *dev = host.dev;
    struct device_node *np = dev.of_node;
    u32 slot_id;
    int err;
    if (!np)
    return dev_err_probe(dev, -EINVAL, "device node not found\n");
    for_each_child_of_node_scoped(np, cnp) {
    if (of_property_read_u32(cnp, "reg", &slot_id)) {
    dev_warn(dev, "reg property is missing for %pOF\n", cnp);
    continue;
    }
    if (slot_id >= ATMCI_MAX_NR_SLOTS) {
    dev_warn(dev, "can't have more than %d slots\n",
    ATMCI_MAX_NR_SLOTS);
    break;
    }
    if (of_property_read_u32(cnp, "bus-width",
    &host.pdata[slot_id].bus_width))
    host.pdata[slot_id].bus_width = 1;
    host.pdata[slot_id].detect_pin =
    devm_fwnode_gpiod_get(dev, of_fwnode_handle(cnp),
    "cd", GPIOD_IN, "cd-gpios");
    err = PTR_ERR_OR_ZERO(host.pdata[slot_id].detect_pin);
    if (err) {
    if (err != -ENOENT)
    return err;
    host.pdata[slot_id].detect_pin = core::ptr::null_mut();
    }
    host.pdata[slot_id].non_removable =
    of_property_read_bool(cnp, "non-removable");
    host.pdata[slot_id].wp_pin =
    devm_fwnode_gpiod_get(dev, of_fwnode_handle(cnp),
    "wp", GPIOD_IN, "wp-gpios");
    err = PTR_ERR_OR_ZERO(host.pdata[slot_id].wp_pin);
    if (err) {
    if (err != -ENOENT)
    return err;
    host.pdata[slot_id].wp_pin = core::ptr::null_mut();
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn atmci_get_version(host: *mut atmel_mci) -> c_uint {
    static inline unsigned int atmci_get_version(struct atmel_mci *host)
    {
    return atmci_readl(host, ATMCI_VERSION) & 0x00000fff;
    }
//
// Fix sconfig's burst size according to atmel MCI. We need to convert them as:
// 1 -> 0, 4 -> 1, 8 -> 2, 16 -> 3.
// With version 0x600, we need to convert them as: 1 -> 0, 2 -> 1, 4 -> 2,
// 8 -> 3, 16 -> 4.
//
// This can be done by finding most significant bit set.
//
    static inline unsigned int atmci_convert_chksize(struct atmel_mci *host,
    unsigned int maxburst)
    {
    let mut version: c_uint = atmci_get_version(host);
    let mut offset: c_uint = 2;
    if (version >= 0x600)
    offset = 1;
    if (maxburst > 1)
    return fls(maxburst) - offset;
    else
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn atmci_timeout_timer(t: *mut timer_list) {
    static void atmci_timeout_timer(struct timer_list *t)
    {
    struct atmel_mci *host = timer_container_of(host, t, timer);
    struct device *dev = host.dev;
    dev_dbg(dev, "software timeout\n");
    if (host.mrq.cmd.data) {
    host.mrq.cmd.data.error = -ETIMEDOUT;
    host.data = core::ptr::null_mut();
//
// With some SDIO modules, sometimes DMA transfer hangs. If
// stop_transfer() is not called then the DMA request is not
// removed, following ones are queued and never computed.
//
    if (host.state == STATE_DATA_XFER)
    host.stop_transfer(host);
    } else {
    host.mrq.cmd.error = -ETIMEDOUT;
    host.cmd = core::ptr::null_mut();
    }
    host.need_reset = 1;
    host.state = STATE_END_REQUEST;
    smp_wmb();
    queue_work(system_bh_wq, &host.bh_work);
    }
    static inline unsigned int atmci_ns_to_clocks(struct atmel_mci *host,
    unsigned int ns)
    {
//
// It is easier here to use us instead of ns for the timeout,
// it prevents from overflows during calculation.
//
    let mut us: c_uint = DIV_ROUND_UP(ns, 1000);
// Maximum clock frequency is host->bus_hz/2
    return us * (DIV_ROUND_UP(host.bus_hz, 2000000));
    }
    static void atmci_set_timeout(struct atmel_mci *host,
    struct atmel_mci_slot *slot, struct mmc_data *data)
    {
    static unsigned	dtomul_to_shift[] = {
    0, 4, 7, 8, 10, 12, 16, 20
    };
    unsigned	timeout;
    unsigned	dtocyc;
    unsigned	dtomul;
    timeout = atmci_ns_to_clocks(host, data.timeout_ns)
    + data.timeout_clks;
    for (dtomul = 0; dtomul < 8; dtomul++) {
    let mut shift: unsigned = dtomul_to_shift[dtomul];
    dtocyc = (timeout + (1 << shift) - 1) >> shift;
    if (dtocyc < 15)
    break;
    }
    if (dtomul >= 8) {
    dtomul = 7;
    dtocyc = 15;
    }
    dev_vdbg(&slot.mmc.class_dev, "setting timeout to %u cycles\n",
    dtocyc << dtomul_to_shift[dtomul]);
    atmci_writel(host, ATMCI_DTOR, (ATMCI_DTOMUL(dtomul) | ATMCI_DTOCYC(dtocyc)));
    }
//
// Return mask with command flags to be enabled for this command.
//
    static u32 atmci_prepare_command(struct mmc_host *mmc,
    struct mmc_command *cmd)
    {
    struct mmc_data	*data;
    u32		cmdr;
    cmd.error = -EINPROGRESS;
    cmdr = ATMCI_CMDR_CMDNB(cmd.opcode);
    if (cmd.flags & MMC_RSP_PRESENT) {
    if (cmd.flags & MMC_RSP_136)
    cmdr |= ATMCI_CMDR_RSPTYP_136BIT;
    else
    cmdr |= ATMCI_CMDR_RSPTYP_48BIT;
    }
//
// This should really be MAXLAT_5 for CMD2 and ACMD41, but
// it's too difficult to determine whether this is an ACMD or
// not. Better make it 64.
//
    cmdr |= ATMCI_CMDR_MAXLAT_64CYC;
    if (mmc.ios.bus_mode == MMC_BUSMODE_OPENDRAIN)
    cmdr |= ATMCI_CMDR_OPDCMD;
    data = cmd.data;
    if (data) {
    cmdr |= ATMCI_CMDR_START_XFER;
    if (cmd.opcode == SD_IO_RW_EXTENDED) {
    cmdr |= ATMCI_CMDR_SDIO_BLOCK;
    } else {
    if (data.blocks > 1)
    cmdr |= ATMCI_CMDR_MULTI_BLOCK;
    else
    cmdr |= ATMCI_CMDR_BLOCK;
    }
    if (data.flags & MMC_DATA_READ)
    cmdr |= ATMCI_CMDR_TRDIR_READ;
    }
    return cmdr;
    }
    static void atmci_send_command(struct atmel_mci *host,
    struct mmc_command *cmd, u32 cmd_flags)
    {
    struct device *dev = host.dev;
    unsigned int timeout_ms = cmd.busy_timeout ? cmd.busy_timeout :
    ATMCI_CMD_TIMEOUT_MS;
    WARN_ON(host.cmd);
    host.cmd = cmd;
    dev_vdbg(dev, "start command: ARGR=0x%08x CMDR=0x%08x\n", cmd.arg, cmd_flags);
    atmci_writel(host, ATMCI_ARGR, cmd.arg);
    atmci_writel(host, ATMCI_CMDR, cmd_flags);
    mod_timer(&host.timer, jiffies + msecs_to_jiffies(timeout_ms));
    }
#[no_mangle]
unsafe extern "C" fn atmci_send_stop_cmd(host: *mut atmel_mci, data: *mut mmc_data) {
    static void atmci_send_stop_cmd(struct atmel_mci *host, struct mmc_data *data)
    {
    struct device *dev = host.dev;
    dev_dbg(dev, "send stop command\n");
    atmci_send_command(host, data.stop, host.stop_cmdr);
    atmci_writel(host, ATMCI_IER, ATMCI_CMDRDY);
    }
//
// Configure given PDC buffer taking care of alignment issues.
// Update host->data_size and host->sg.
//
    static void atmci_pdc_set_single_buf(struct atmel_mci *host,
    enum atmci_xfer_dir dir, enum atmci_pdc_buf buf_nb)
    {
    u32 pointer_reg, counter_reg;
    unsigned int buf_size;
    if (dir == XFER_RECEIVE) {
    pointer_reg = ATMEL_PDC_RPR;
    counter_reg = ATMEL_PDC_RCR;
    } else {
    pointer_reg = ATMEL_PDC_TPR;
    counter_reg = ATMEL_PDC_TCR;
    }
    if (buf_nb == PDC_SECOND_BUF) {
    pointer_reg += ATMEL_PDC_SCND_BUF_OFF;
    counter_reg += ATMEL_PDC_SCND_BUF_OFF;
    }
    if (!host.caps.has_rwproof) {
    buf_size = host.buf_size;
    atmci_writel(host, pointer_reg, host.buf_phys_addr);
    } else {
    buf_size = sg_dma_len(host.sg);
    atmci_writel(host, pointer_reg, sg_dma_address(host.sg));
    }
    if (host.data_size <= buf_size) {
    if (host.data_size & 0x3) {
// If size is different from modulo 4, transfer bytes
    atmci_writel(host, counter_reg, host.data_size);
    atmci_writel(host, ATMCI_MR, host.mode_reg | ATMCI_MR_PDCFBYTE);
    } else {
// Else transfer 32-bits words
    atmci_writel(host, counter_reg, host.data_size / 4);
    }
    host.data_size = 0;
    } else {
// We assume the size of a page is 32-bits aligned
    atmci_writel(host, counter_reg, sg_dma_len(host.sg) / 4);
    host.data_size -= sg_dma_len(host.sg);
    if (host.data_size)
    host.sg = sg_next(host.sg);
    }
    }
//
// Configure PDC buffer according to the data size ie configuring one or two
// buffers. Don't use this function if you want to configure only the second
// buffer. In this case, use atmci_pdc_set_single_buf.
//
#[no_mangle]
unsafe extern "C" fn atmci_pdc_set_both_buf(host: *mut atmel_mci, dir: c_int) {
    static void atmci_pdc_set_both_buf(struct atmel_mci *host, int dir)
    {
    atmci_pdc_set_single_buf(host, dir, PDC_FIRST_BUF);
    if (host.data_size)
    atmci_pdc_set_single_buf(host, dir, PDC_SECOND_BUF);
    }
//
// Unmap sg lists, called when transfer is finished.
//
#[no_mangle]
unsafe extern "C" fn atmci_pdc_cleanup(host: *mut atmel_mci) {
    static void atmci_pdc_cleanup(struct atmel_mci *host)
    {
    struct mmc_data         *data = host.data;
    struct device		*dev = host.dev;
    if (data)
    dma_unmap_sg(dev, data.sg, data.sg_len, mmc_get_dma_dir(data));
    }
//
// Disable PDC transfers. Update pending flags to EVENT_XFER_COMPLETE after
// having received ATMCI_TXBUFE or ATMCI_RXBUFF interrupt. Enable ATMCI_NOTBUSY
// interrupt needed for both transfer directions.
//
#[no_mangle]
unsafe extern "C" fn atmci_pdc_complete(host: *mut atmel_mci) {
    static void atmci_pdc_complete(struct atmel_mci *host)
    {
    struct device *dev = host.dev;
    let mut transfer_size: c_int = host.data.blocks * host.data.blksz;
    int i;
    atmci_writel(host, ATMEL_PDC_PTCR, ATMEL_PDC_RXTDIS | ATMEL_PDC_TXTDIS);
    if ((!host.caps.has_rwproof)
    && (host.data.flags & MMC_DATA_READ)) {
    if (host.caps.has_bad_data_ordering)
    for (i = 0; i < transfer_size; i++)
    host.buffer[i] = swab32(host.buffer[i]);
    sg_copy_from_buffer(host.data.sg, host.data.sg_len,
    host.buffer, transfer_size);
    }
    atmci_pdc_cleanup(host);
    dev_dbg(dev, "(%s) set pending xfer complete\n", __func__);
    atmci_set_pending(host, EVENT_XFER_COMPLETE);
    queue_work(system_bh_wq, &host.bh_work);
    }
#[no_mangle]
unsafe extern "C" fn atmci_dma_cleanup(host: *mut atmel_mci) {
    static void atmci_dma_cleanup(struct atmel_mci *host)
    {
    struct mmc_data                 *data = host.data;
    if (data)
    dma_unmap_sg(host.dma.chan.device.dev,
    data.sg, data.sg_len,
    mmc_get_dma_dir(data));
    }
//
// This function is called by the DMA driver from bh context.
//
#[no_mangle]
unsafe extern "C" fn atmci_dma_complete(arg: *mut c_void) {
    static void atmci_dma_complete(void *arg)
    {
    struct atmel_mci	*host = arg;
    struct mmc_data		*data = host.data;
    struct device		*dev = host.dev;
    dev_vdbg(dev, "DMA complete\n");
    if (host.caps.has_dma_conf_reg)
// Disable DMA hardware handshaking on MCI
    atmci_writel(host, ATMCI_DMA, atmci_readl(host, ATMCI_DMA) & ~ATMCI_DMAEN);
    atmci_dma_cleanup(host);
//
// If the card was removed, data will be NULL. No point trying
// to send the stop command or waiting for NBUSY in this case.
//
    if (data) {
    dev_dbg(dev, "(%s) set pending xfer complete\n", __func__);
    atmci_set_pending(host, EVENT_XFER_COMPLETE);
    queue_work(system_bh_wq, &host.bh_work);
//
// Regardless of what the documentation says, we have
// to wait for NOTBUSY even after block read
// operations.
//
// When the DMA transfer is complete, the controller
// may still be reading the CRC from the card, i.e.
// the data transfer is still in progress and we
// haven't seen all the potential error bits yet.
//
// The interrupt handler will schedule a different
// bh work to finish things up when the data transfer
// is completely done.
//
// We may not complete the mmc request here anyway
// because the mmc layer may call back and cause us to
// violate the "don't submit new operations from the
// completion callback" rule of the dma engine
// framework.
//
    atmci_writel(host, ATMCI_IER, ATMCI_NOTBUSY);
    }
    }
//
// Returns a mask of interrupt flags to be enabled after the whole
// request has been prepared.
//
#[no_mangle]
unsafe extern "C" fn atmci_prepare_data(host: *mut atmel_mci, data: *mut mmc_data) -> u32 {
    static u32 atmci_prepare_data(struct atmel_mci *host, struct mmc_data *data)
    {
    u32 iflags;
    data.error = -EINPROGRESS;
    host.sg = data.sg;
    host.sg_len = data.sg_len;
    host.data = data;
    host.data_chan = core::ptr::null_mut();
    iflags = ATMCI_DATA_ERROR_FLAGS;
//
// Errata: MMC data write operation with less than 12
// bytes is impossible.
//
// Errata: MCI Transmit Data Register (TDR) FIFO
// corruption when length is not multiple of 4.
//
    if (data.blocks * data.blksz < 12
    || (data.blocks * data.blksz) & 3)
    host.need_reset = true;
    host.pio_offset = 0;
    if (data.flags & MMC_DATA_READ)
    iflags |= ATMCI_RXRDY;
    else
    iflags |= ATMCI_TXRDY;
    return iflags;
    }
//
// Set interrupt flags and set block length into the MCI mode register even
// if this value is also accessible in the MCI block register. It seems to be
// necessary before the High Speed MCI version. It also map sg and configure
// PDC registers.
//
    static u32
    atmci_prepare_data_pdc(struct atmel_mci *host, struct mmc_data *data)
    {
    struct device *dev = host.dev;
    u32 iflags, tmp;
    int i;
    data.error = -EINPROGRESS;
    host.data = data;
    host.sg = data.sg;
    iflags = ATMCI_DATA_ERROR_FLAGS;
// Enable pdc mode
    atmci_writel(host, ATMCI_MR, host.mode_reg | ATMCI_MR_PDCMODE);
    if (data.flags & MMC_DATA_READ)
    iflags |= ATMCI_ENDRX | ATMCI_RXBUFF;
    else
    iflags |= ATMCI_ENDTX | ATMCI_TXBUFE | ATMCI_BLKE;
// Set BLKLEN
    tmp = atmci_readl(host, ATMCI_MR);
    tmp &= 0x0000ffff;
    tmp |= ATMCI_BLKLEN(data.blksz);
    atmci_writel(host, ATMCI_MR, tmp);
// Configure PDC
    host.data_size = data.blocks * data.blksz;
    dma_map_sg(dev, data.sg, data.sg_len, mmc_get_dma_dir(data));
    if ((!host.caps.has_rwproof)
    && (host.data.flags & MMC_DATA_WRITE)) {
    sg_copy_to_buffer(host.data.sg, host.data.sg_len,
    host.buffer, host.data_size);
    if (host.caps.has_bad_data_ordering)
    for (i = 0; i < host.data_size; i++)
    host.buffer[i] = swab32(host.buffer[i]);
    }
    if (host.data_size)
    atmci_pdc_set_both_buf(host, data.flags & MMC_DATA_READ ?
    XFER_RECEIVE : XFER_TRANSMIT);
    return iflags;
    }
    static u32
    atmci_prepare_data_dma(struct atmel_mci *host, struct mmc_data *data)
    {
    struct dma_chan			*chan;
    struct dma_async_tx_descriptor	*desc;
    struct scatterlist		*sg;
    unsigned int			i;
    enum dma_transfer_direction	slave_dirn;
    unsigned int			sglen;
    u32				maxburst;
    u32 iflags;
    data.error = -EINPROGRESS;
    WARN_ON(host.data);
    host.sg = core::ptr::null_mut();
    host.data = data;
    iflags = ATMCI_DATA_ERROR_FLAGS;
//
// We don't do DMA on "complex" transfers, i.e. with
// non-word-aligned buffers or lengths. Also, we don't bother
// with all the DMA setup overhead for short transfers.
//
    if (data.blocks * data.blksz < ATMCI_DMA_THRESHOLD)
    return atmci_prepare_data(host, data);
    if (data.blksz & 3)
    return atmci_prepare_data(host, data);
    for_each_sg(data.sg, sg, data.sg_len, i) {
    if (sg.offset & 3 || sg.length & 3)
    return atmci_prepare_data(host, data);
    }
// If we don't have a channel, we can't do DMA
    if (!host.dma.chan)
    return -ENODEV;
    chan = host.dma.chan;
    host.data_chan = chan;
    if (data.flags & MMC_DATA_READ) {
    host.dma_conf.direction = slave_dirn = DMA_DEV_TO_MEM;
    maxburst = atmci_convert_chksize(host,
    host.dma_conf.src_maxburst);
    } else {
    host.dma_conf.direction = slave_dirn = DMA_MEM_TO_DEV;
    maxburst = atmci_convert_chksize(host,
    host.dma_conf.dst_maxburst);
    }
    if (host.caps.has_dma_conf_reg)
    atmci_writel(host, ATMCI_DMA, ATMCI_DMA_CHKSIZE(maxburst) |
    ATMCI_DMAEN);
    sglen = dma_map_sg(chan.device.dev, data.sg,
    data.sg_len, mmc_get_dma_dir(data));
    dmaengine_slave_config(chan, &host.dma_conf);
    desc = dmaengine_prep_slave_sg(chan,
    data.sg, sglen, slave_dirn,
    DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    if (!desc)
    goto unmap_exit;
    host.dma.data_desc = desc;
    desc.callback = atmci_dma_complete;
    desc.callback_param = host;
    return iflags;
    unmap_exit:
    dma_unmap_sg(chan.device.dev, data.sg, data.sg_len,
    mmc_get_dma_dir(data));
    return -ENOMEM;
    }
    static void
    atmci_submit_data(struct atmel_mci *host, struct mmc_data *data)
    {
    return;
    }
//
// Start PDC according to transfer direction.
//
    static void
    atmci_submit_data_pdc(struct atmel_mci *host, struct mmc_data *data)
    {
    if (data.flags & MMC_DATA_READ)
    atmci_writel(host, ATMEL_PDC_PTCR, ATMEL_PDC_RXTEN);
    else
    atmci_writel(host, ATMEL_PDC_PTCR, ATMEL_PDC_TXTEN);
    }
    static void
    atmci_submit_data_dma(struct atmel_mci *host, struct mmc_data *data)
    {
    struct dma_chan			*chan = host.data_chan;
    struct dma_async_tx_descriptor	*desc = host.dma.data_desc;
    if (chan) {
    dmaengine_submit(desc);
    dma_async_issue_pending(chan);
    }
    }
#[no_mangle]
unsafe extern "C" fn atmci_stop_transfer(host: *mut atmel_mci) {
    static void atmci_stop_transfer(struct atmel_mci *host)
    {
    struct device *dev = host.dev;
    dev_dbg(dev, "(%s) set pending xfer complete\n", __func__);
    atmci_set_pending(host, EVENT_XFER_COMPLETE);
    atmci_writel(host, ATMCI_IER, ATMCI_NOTBUSY);
    }
//
// Stop data transfer because error(s) occurred.
//
#[no_mangle]
unsafe extern "C" fn atmci_stop_transfer_pdc(host: *mut atmel_mci) {
    static void atmci_stop_transfer_pdc(struct atmel_mci *host)
    {
    atmci_writel(host, ATMEL_PDC_PTCR, ATMEL_PDC_RXTDIS | ATMEL_PDC_TXTDIS);
    }
#[no_mangle]
unsafe extern "C" fn atmci_stop_transfer_dma(host: *mut atmel_mci) {
    static void atmci_stop_transfer_dma(struct atmel_mci *host)
    {
    struct dma_chan *chan = host.data_chan;
    struct device *dev = host.dev;
    if (chan) {
    dmaengine_terminate_all(chan);
    atmci_dma_cleanup(host);
    } else {
// Data transfer was stopped by the interrupt handler
    dev_dbg(dev, "(%s) set pending xfer complete\n", __func__);
    atmci_set_pending(host, EVENT_XFER_COMPLETE);
    atmci_writel(host, ATMCI_IER, ATMCI_NOTBUSY);
    }
    }
//
// Start a request: prepare data if needed, prepare the command and activate
// interrupts.
//
    static void atmci_start_request(struct atmel_mci *host,
    struct atmel_mci_slot *slot)
    {
    struct device		*dev = host.dev;
    struct mmc_request	*mrq;
    struct mmc_command	*cmd;
    struct mmc_data		*data;
    u32			iflags;
    u32			cmdflags;
    mrq = slot.mrq;
    host.cur_slot = slot;
    host.mrq = mrq;
    host.pending_events = 0;
    host.completed_events = 0;
    host.cmd_status = 0;
    host.data_status = 0;
    dev_dbg(dev, "start request: cmd %u\n", mrq.cmd.opcode);
    if (host.need_reset || host.caps.need_reset_after_xfer) {
    iflags = atmci_readl(host, ATMCI_IMR);
    iflags &= (ATMCI_SDIOIRQA | ATMCI_SDIOIRQB);
    atmci_writel(host, ATMCI_CR, ATMCI_CR_SWRST);
    atmci_writel(host, ATMCI_CR, ATMCI_CR_MCIEN);
    atmci_writel(host, ATMCI_MR, host.mode_reg);
    if (host.caps.has_cfg_reg)
    atmci_writel(host, ATMCI_CFG, host.cfg_reg);
    atmci_writel(host, ATMCI_IER, iflags);
    host.need_reset = false;
    }
    atmci_writel(host, ATMCI_SDCR, slot.sdc_reg);
    iflags = atmci_readl(host, ATMCI_IMR);
    if (iflags & ~(ATMCI_SDIOIRQA | ATMCI_SDIOIRQB))
    dev_dbg(&slot.mmc.class_dev, "WARNING: IMR=0x%08x\n",
    iflags);
    if (unlikely(test_and_clear_bit(ATMCI_CARD_NEED_INIT, &slot.flags))) {
// Send init sequence (74 clock cycles)
    atmci_writel(host, ATMCI_CMDR, ATMCI_CMDR_SPCMD_INIT);
    while (!(atmci_readl(host, ATMCI_SR) & ATMCI_CMDRDY))
    cpu_relax();
    }
    iflags = 0;
    data = mrq.data;
    if (data) {
    atmci_set_timeout(host, slot, data);
// Must set block count/size before sending command
    atmci_writel(host, ATMCI_BLKR, ATMCI_BCNT(data.blocks)
    | ATMCI_BLKLEN(data.blksz));
    dev_vdbg(&slot.mmc.class_dev, "BLKR=0x%08x\n",
    ATMCI_BCNT(data.blocks) | ATMCI_BLKLEN(data.blksz));
    iflags |= host.prepare_data(host, data);
    }
    iflags |= ATMCI_CMDRDY;
    cmd = mrq.cmd;
    cmdflags = atmci_prepare_command(slot.mmc, cmd);
//
// DMA transfer should be started before sending the command to avoid
// unexpected errors especially for read operations in SDIO mode.
// Unfortunately, in PDC mode, command has to be sent before starting
// the transfer.
//
    if (host.submit_data != &atmci_submit_data_dma)
    atmci_send_command(host, cmd, cmdflags);
    if (data)
    host.submit_data(host, data);
    if (host.submit_data == &atmci_submit_data_dma)
    atmci_send_command(host, cmd, cmdflags);
    if (mrq.stop) {
    host.stop_cmdr = atmci_prepare_command(slot.mmc, mrq.stop);
    host.stop_cmdr |= ATMCI_CMDR_STOP_XFER;
    if (!(data.flags & MMC_DATA_WRITE))
    host.stop_cmdr |= ATMCI_CMDR_TRDIR_READ;
    host.stop_cmdr |= ATMCI_CMDR_MULTI_BLOCK;
    }
//
// We could have enabled interrupts earlier, but I suspect
// that would open up a nice can of interesting race
// conditions (e.g. command and data complete, but stop not
// prepared yet.)
//
    atmci_writel(host, ATMCI_IER, iflags);
    }
    static void atmci_queue_request(struct atmel_mci *host,
    struct atmel_mci_slot *slot, struct mmc_request *mrq)
    {
    struct device *dev = host.dev;
    dev_vdbg(&slot.mmc.class_dev, "queue request: state=%d\n",
    host.state);
    spin_lock_bh(&host.lock);
    slot.mrq = mrq;
    if (host.state == STATE_IDLE) {
    host.state = STATE_SENDING_CMD;
    atmci_start_request(host, slot);
    } else {
    dev_dbg(dev, "queue request\n");
    list_add_tail(&slot.queue_node, &host.queue);
    }
    spin_unlock_bh(&host.lock);
    }
#[no_mangle]
unsafe extern "C" fn atmci_request(mmc: *mut mmc_host, mrq: *mut mmc_request) {
    static void atmci_request(struct mmc_host *mmc, struct mmc_request *mrq)
    {
    struct atmel_mci_slot	*slot = mmc_priv(mmc);
    struct atmel_mci	*host = slot.host;
    struct device		*dev = host.dev;
    struct mmc_data		*data;
    WARN_ON(slot.mrq);
    dev_dbg(dev, "MRQ: cmd %u\n", mrq.cmd.opcode);
//
// We may "know" the card is gone even though there's still an
// electrical connection. If so, we really need to communicate
// this to the MMC core since there won't be any more
// interrupts as the card is completely removed. Otherwise,
// the MMC core might believe the card is still there even
// though the card was just removed very slowly.
//
    if (!test_bit(ATMCI_CARD_PRESENT, &slot.flags)) {
    mrq.cmd.error = -ENOMEDIUM;
    mmc_request_done(mmc, mrq);
    return;
    }
// We don't support multiple blocks of weird lengths.
    data = mrq.data;
    if (data && data.blocks > 1 && data.blksz & 3) {
    mrq.cmd.error = -EINVAL;
    mmc_request_done(mmc, mrq);
    }
    atmci_queue_request(host, slot, mrq);
    }
#[no_mangle]
unsafe extern "C" fn atmci_set_ios(mmc: *mut mmc_host, ios: *mut mmc_ios) {
    static void atmci_set_ios(struct mmc_host *mmc, struct mmc_ios *ios)
    {
    struct atmel_mci_slot	*slot = mmc_priv(mmc);
    struct atmel_mci	*host = slot.host;
    unsigned int		i;
    slot.sdc_reg &= ~ATMCI_SDCBUS_MASK;
    switch (ios.bus_width) {
    case MMC_BUS_WIDTH_1:
    slot.sdc_reg |= ATMCI_SDCBUS_1BIT;
    break;
    case MMC_BUS_WIDTH_4:
    slot.sdc_reg |= ATMCI_SDCBUS_4BIT;
    break;
    case MMC_BUS_WIDTH_8:
    slot.sdc_reg |= ATMCI_SDCBUS_8BIT;
    break;
    }
    if (ios.clock) {
    let mut clock_min: c_uint = ~0U;
    int clkdiv;
    spin_lock_bh(&host.lock);
    if (!host.mode_reg) {
    atmci_writel(host, ATMCI_CR, ATMCI_CR_SWRST);
    atmci_writel(host, ATMCI_CR, ATMCI_CR_MCIEN);
    if (host.caps.has_cfg_reg)
    atmci_writel(host, ATMCI_CFG, host.cfg_reg);
    }
//
// Use mirror of ios->clock to prevent race with mmc
// core ios update when finding the minimum.
//
    slot.clock = ios.clock;
    for (i = 0; i < ATMCI_MAX_NR_SLOTS; i++) {
    if (host.slot[i] && host.slot[i].clock
    && host.slot[i].clock < clock_min)
    clock_min = host.slot[i].clock;
    }
// Calculate clock divider
    if (host.caps.has_odd_clk_div) {
    clkdiv = DIV_ROUND_UP(host.bus_hz, clock_min) - 2;
    if (clkdiv < 0) {
    dev_warn(&mmc.class_dev,
    "clock %u too fast; using %lu\n",
    clock_min, host.bus_hz / 2);
    clkdiv = 0;
    } else if (clkdiv > 511) {
    dev_warn(&mmc.class_dev,
    "clock %u too slow; using %lu\n",
    clock_min, host.bus_hz / (511 + 2));
    clkdiv = 511;
    }
    host.mode_reg = ATMCI_MR_CLKDIV(clkdiv >> 1)
    | ATMCI_MR_CLKODD(clkdiv & 1);
    } else {
    clkdiv = DIV_ROUND_UP(host.bus_hz, 2 * clock_min) - 1;
    if (clkdiv > 255) {
    dev_warn(&mmc.class_dev,
    "clock %u too slow; using %lu\n",
    clock_min, host.bus_hz / (2 * 256));
    clkdiv = 255;
    }
    host.mode_reg = ATMCI_MR_CLKDIV(clkdiv);
    }
//
// WRPROOF and RDPROOF prevent overruns/underruns by
// stopping the clock when the FIFO is full/empty.
// This state is not expected to last for long.
//
    if (host.caps.has_rwproof)
    host.mode_reg |= (ATMCI_MR_WRPROOF | ATMCI_MR_RDPROOF);
    if (host.caps.has_cfg_reg) {
// setup High Speed mode in relation with card capacity
    if (ios.timing == MMC_TIMING_SD_HS)
    host.cfg_reg |= ATMCI_CFG_HSMODE;
    else
    host.cfg_reg &= ~ATMCI_CFG_HSMODE;
    }
    if (list_empty(&host.queue)) {
    atmci_writel(host, ATMCI_MR, host.mode_reg);
    if (host.caps.has_cfg_reg)
    atmci_writel(host, ATMCI_CFG, host.cfg_reg);
    } else {
    host.need_clock_update = true;
    }
    spin_unlock_bh(&host.lock);
    } else {
    let mut any_slot_active: bool = false;
    spin_lock_bh(&host.lock);
    slot.clock = 0;
    for (i = 0; i < ATMCI_MAX_NR_SLOTS; i++) {
    if (host.slot[i] && host.slot[i].clock) {
    any_slot_active = true;
    break;
    }
    }
    if (!any_slot_active) {
    atmci_writel(host, ATMCI_CR, ATMCI_CR_MCIDIS);
    if (host.mode_reg) {
    atmci_readl(host, ATMCI_MR);
    }
    host.mode_reg = 0;
    }
    spin_unlock_bh(&host.lock);
    }
    switch (ios.power_mode) {
    case MMC_POWER_OFF:
    if (!IS_ERR(mmc.supply.vmmc))
    mmc_regulator_set_ocr(mmc, mmc.supply.vmmc, 0);
    break;
    case MMC_POWER_UP:
    set_bit(ATMCI_CARD_NEED_INIT, &slot.flags);
    if (!IS_ERR(mmc.supply.vmmc))
    mmc_regulator_set_ocr(mmc, mmc.supply.vmmc, ios.vdd);
    break;
    default:
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn atmci_get_ro(mmc: *mut mmc_host) -> c_int {
    static int atmci_get_ro(struct mmc_host *mmc)
    {
    let mut read_only: c_int = -ENOSYS;
    struct atmel_mci_slot	*slot = mmc_priv(mmc);
    if (slot.wp_pin) {
    read_only = gpiod_get_value(slot.wp_pin);
    dev_dbg(&mmc.class_dev, "card is %s\n",
    read_only ? "read-only" : "read-write");
    }
    return read_only;
    }
#[no_mangle]
unsafe extern "C" fn atmci_get_cd(mmc: *mut mmc_host) -> c_int {
    static int atmci_get_cd(struct mmc_host *mmc)
    {
    let mut present: c_int = -ENOSYS;
    struct atmel_mci_slot	*slot = mmc_priv(mmc);
    if (slot.detect_pin) {
    present = gpiod_get_value_cansleep(slot.detect_pin);
    dev_dbg(&mmc.class_dev, "card is %spresent\n",
    present ? "" : "not ");
    }
    return present;
    }
#[no_mangle]
unsafe extern "C" fn atmci_enable_sdio_irq(mmc: *mut mmc_host, enable: c_int) {
    static void atmci_enable_sdio_irq(struct mmc_host *mmc, int enable)
    {
    struct atmel_mci_slot	*slot = mmc_priv(mmc);
    struct atmel_mci	*host = slot.host;
    if (enable)
    atmci_writel(host, ATMCI_IER, slot.sdio_irq);
    else
    atmci_writel(host, ATMCI_IDR, slot.sdio_irq);
    }
    static const struct mmc_host_ops atmci_ops = {
    .request	= atmci_request,
    .set_ios	= atmci_set_ios,
    .get_ro		= atmci_get_ro,
    .get_cd		= atmci_get_cd,
    .enable_sdio_irq = atmci_enable_sdio_irq,
    };
// Called with host->lock held
#[no_mangle]
unsafe extern "C" fn atmci_request_end(host: *mut atmel_mci, mrq: *mut mmc_request) {
    static void atmci_request_end(struct atmel_mci *host, struct mmc_request *mrq)
    __releases(&host.lock)
    __acquires(&host.lock)
    {
    struct atmel_mci_slot	*slot = core::ptr::null_mut();
    struct mmc_host		*prev_mmc = host.cur_slot.mmc;
    struct device		*dev = host.dev;
    WARN_ON(host.cmd || host.data);
    timer_delete(&host.timer);
//
// Update the MMC clock rate if necessary. This may be
// necessary if set_ios() is called when a different slot is
// busy transferring data.
//
    if (host.need_clock_update) {
    atmci_writel(host, ATMCI_MR, host.mode_reg);
    if (host.caps.has_cfg_reg)
    atmci_writel(host, ATMCI_CFG, host.cfg_reg);
    }
    host.cur_slot.mrq = core::ptr::null_mut();
    host.mrq = core::ptr::null_mut();
    if (!list_empty(&host.queue)) {
    slot = list_entry(host.queue.next,
    struct atmel_mci_slot, queue_node);
    list_del(&slot.queue_node);
    dev_vdbg(dev, "list not empty: %s is next\n", mmc_hostname(slot.mmc));
    host.state = STATE_SENDING_CMD;
    atmci_start_request(host, slot);
    } else {
    dev_vdbg(dev, "list empty\n");
    host.state = STATE_IDLE;
    }
    spin_unlock(&host.lock);
    mmc_request_done(prev_mmc, mrq);
    spin_lock(&host.lock);
    }
    static void atmci_command_complete(struct atmel_mci *host,
    struct mmc_command *cmd)
    {
    let mut status: u32 = host.cmd_status;
// Read the response from the card (up to 16 bytes)
    cmd.resp[0] = atmci_readl(host, ATMCI_RSPR);
    cmd.resp[1] = atmci_readl(host, ATMCI_RSPR);
    cmd.resp[2] = atmci_readl(host, ATMCI_RSPR);
    cmd.resp[3] = atmci_readl(host, ATMCI_RSPR);
    if (status & ATMCI_RTOE)
    cmd.error = -ETIMEDOUT;
#[no_mangle]
pub unsafe extern "C" fn if(ATMCI_RCRCE): (cmd->flags & MMC_RSP_CRC) && (status &) -> else {
    else if ((cmd.flags & MMC_RSP_CRC) && (status & ATMCI_RCRCE))
    cmd.error = -EILSEQ;
#[no_mangle]
pub unsafe extern "C" fn if(ATMCI_RENDE): status & (ATMCI_RINDE | ATMCI_RDIRE |) -> else {
    else if (status & (ATMCI_RINDE | ATMCI_RDIRE | ATMCI_RENDE))
    cmd.error = -EIO;
#[no_mangle]
pub unsafe extern "C" fn if(3): host->mrq->data && (host->mrq->data->blksz &) -> else {
    if (host.caps.need_blksz_mul_4) {
    cmd.error = -EINVAL;
    host.need_reset = 1;
    }
    } else
    cmd.error = 0;
    }
#[no_mangle]
unsafe extern "C" fn atmci_detect_change(t: *mut timer_list) {
    static void atmci_detect_change(struct timer_list *t)
    {
    struct atmel_mci_slot	*slot = timer_container_of(slot, t,
    detect_timer);
    bool			present;
    bool			present_old;
//
// atmci_cleanup_slot() sets the ATMCI_SHUTDOWN flag before
// freeing the interrupt. We must not re-enable the interrupt
// if it has been freed, and if we're shutting down, it
// doesn't really matter whether the card is present or not.
//
    smp_rmb();
    if (test_bit(ATMCI_SHUTDOWN, &slot.flags))
    return;
    enable_irq(gpiod_to_irq(slot.detect_pin));
    present = gpiod_get_value_cansleep(slot.detect_pin);
    present_old = test_bit(ATMCI_CARD_PRESENT, &slot.flags);
    dev_vdbg(&slot.mmc.class_dev, "detect change: %d (was %d)\n",
    present, present_old);
    if (present != present_old) {
    struct atmel_mci	*host = slot.host;
    struct mmc_request	*mrq;
    dev_dbg(&slot.mmc.class_dev, "card %s\n",
    present ? "inserted" : "removed");
    spin_lock(&host.lock);
    if (!present)
    clear_bit(ATMCI_CARD_PRESENT, &slot.flags);
    else
    set_bit(ATMCI_CARD_PRESENT, &slot.flags);
// Clean up queue if present
    mrq = slot.mrq;
    if (mrq) {
    if (mrq == host.mrq) {
//
// Reset controller to terminate any ongoing
// commands or data transfers.
//
    atmci_writel(host, ATMCI_CR, ATMCI_CR_SWRST);
    atmci_writel(host, ATMCI_CR, ATMCI_CR_MCIEN);
    atmci_writel(host, ATMCI_MR, host.mode_reg);
    if (host.caps.has_cfg_reg)
    atmci_writel(host, ATMCI_CFG, host.cfg_reg);
    host.data = core::ptr::null_mut();
    host.cmd = core::ptr::null_mut();
    switch (host.state) {
    case STATE_IDLE:
    break;
    case STATE_SENDING_CMD:
    mrq.cmd.error = -ENOMEDIUM;
    if (mrq.data)
    host.stop_transfer(host);
    break;
    case STATE_DATA_XFER:
    mrq.data.error = -ENOMEDIUM;
    host.stop_transfer(host);
    break;
    case STATE_WAITING_NOTBUSY:
    mrq.data.error = -ENOMEDIUM;
    break;
    case STATE_SENDING_STOP:
    mrq.stop.error = -ENOMEDIUM;
    break;
    case STATE_END_REQUEST:
    break;
    }
    atmci_request_end(host, mrq);
    } else {
    list_del(&slot.queue_node);
    mrq.cmd.error = -ENOMEDIUM;
    if (mrq.data)
    mrq.data.error = -ENOMEDIUM;
    if (mrq.stop)
    mrq.stop.error = -ENOMEDIUM;
    spin_unlock(&host.lock);
    mmc_request_done(slot.mmc, mrq);
    spin_lock(&host.lock);
    }
    }
    spin_unlock(&host.lock);
    mmc_detect_change(slot.mmc, 0);
    }
    }
#[no_mangle]
unsafe extern "C" fn atmci_work_func(t: *mut work_struct) {
    static void atmci_work_func(struct work_struct *t)
    {
    struct atmel_mci        *host = from_work(host, t, bh_work);
    struct mmc_request	*mrq = host.mrq;
    struct mmc_data		*data = host.data;
    struct device		*dev = host.dev;
    let mut state: enum atmel_mci_state = host.state;
    enum atmel_mci_state	prev_state;
    u32			status;
    spin_lock(&host.lock);
    state = host.state;
    dev_vdbg(dev, "bh_work: state %u pending/completed/mask %lx/%lx/%x\n",
    state, host.pending_events, host.completed_events,
    atmci_readl(host, ATMCI_IMR));
    do {
    prev_state = state;
    dev_dbg(dev, "FSM: state=%d\n", state);
    switch (state) {
    case STATE_IDLE:
    break;
    case STATE_SENDING_CMD:
//
// Command has been sent, we are waiting for command
// ready. Then we have three next states possible:
// END_REQUEST by default, WAITING_NOTBUSY if it's a
// command needing it or DATA_XFER if there is data.
//
    dev_dbg(dev, "FSM: cmd ready?\n");
    if (!atmci_test_and_clear_pending(host,
    EVENT_CMD_RDY))
    break;
    dev_dbg(dev, "set completed cmd ready\n");
    host.cmd = core::ptr::null_mut();
    atmci_set_completed(host, EVENT_CMD_RDY);
    atmci_command_complete(host, mrq.cmd);
    if (mrq.data) {
    dev_dbg(dev, "command with data transfer\n");
//
// If there is a command error don't start
// data transfer.
//
    if (mrq.cmd.error) {
    host.stop_transfer(host);
    host.data = core::ptr::null_mut();
    atmci_writel(host, ATMCI_IDR,
    ATMCI_TXRDY | ATMCI_RXRDY
    | ATMCI_DATA_ERROR_FLAGS);
    state = STATE_END_REQUEST;
    } else
    state = STATE_DATA_XFER;
    } else if ((!mrq.data) && (mrq.cmd.flags & MMC_RSP_BUSY)) {
    dev_dbg(dev, "command response need waiting notbusy\n");
    atmci_writel(host, ATMCI_IER, ATMCI_NOTBUSY);
    state = STATE_WAITING_NOTBUSY;
    } else
    state = STATE_END_REQUEST;
    break;
    case STATE_DATA_XFER:
    if (atmci_test_and_clear_pending(host,
    EVENT_DATA_ERROR)) {
    dev_dbg(dev, "set completed data error\n");
    atmci_set_completed(host, EVENT_DATA_ERROR);
    state = STATE_END_REQUEST;
    break;
    }
//
// A data transfer is in progress. The event expected
// to move to the next state depends of data transfer
// type (PDC or DMA). Once transfer done we can move
// to the next step which is WAITING_NOTBUSY in write
// case and directly SENDING_STOP in read case.
//
    dev_dbg(dev, "FSM: xfer complete?\n");
    if (!atmci_test_and_clear_pending(host,
    EVENT_XFER_COMPLETE))
    break;
    dev_dbg(dev, "(%s) set completed xfer complete\n", __func__);
    atmci_set_completed(host, EVENT_XFER_COMPLETE);
    if (host.caps.need_notbusy_for_read_ops ||
    (host.data.flags & MMC_DATA_WRITE)) {
    atmci_writel(host, ATMCI_IER, ATMCI_NOTBUSY);
    state = STATE_WAITING_NOTBUSY;
    } else if (host.mrq.stop) {
    atmci_send_stop_cmd(host, data);
    state = STATE_SENDING_STOP;
    } else {
    host.data = core::ptr::null_mut();
    data.bytes_xfered = data.blocks * data.blksz;
    data.error = 0;
    state = STATE_END_REQUEST;
    }
    break;
    case STATE_WAITING_NOTBUSY:
//
// We can be in the state for two reasons: a command
// requiring waiting not busy signal (stop command
// included) or a write operation. In the latest case,
// we need to send a stop command.
//
    dev_dbg(dev, "FSM: not busy?\n");
    if (!atmci_test_and_clear_pending(host,
    EVENT_NOTBUSY))
    break;
    dev_dbg(dev, "set completed not busy\n");
    atmci_set_completed(host, EVENT_NOTBUSY);
    if (host.data) {
//
// For some commands such as CMD53, even if
// there is data transfer, there is no stop
// command to send.
//
    if (host.mrq.stop) {
    atmci_send_stop_cmd(host, data);
    state = STATE_SENDING_STOP;
    } else {
    host.data = core::ptr::null_mut();
    data.bytes_xfered = data.blocks
// data->blksz;
    data.error = 0;
    state = STATE_END_REQUEST;
    }
    } else
    state = STATE_END_REQUEST;
    break;
    case STATE_SENDING_STOP:
//
// In this state, it is important to set host->data to
// NULL (which is tested in the waiting notbusy state)
// in order to go to the end request state instead of
// sending stop again.
//
    dev_dbg(dev, "FSM: cmd ready?\n");
    if (!atmci_test_and_clear_pending(host,
    EVENT_CMD_RDY))
    break;
    dev_dbg(dev, "FSM: cmd ready\n");
    host.cmd = core::ptr::null_mut();
    data.bytes_xfered = data.blocks * data.blksz;
    data.error = 0;
    atmci_command_complete(host, mrq.stop);
    if (mrq.stop.error) {
    host.stop_transfer(host);
    atmci_writel(host, ATMCI_IDR,
    ATMCI_TXRDY | ATMCI_RXRDY
    | ATMCI_DATA_ERROR_FLAGS);
    state = STATE_END_REQUEST;
    } else {
    atmci_writel(host, ATMCI_IER, ATMCI_NOTBUSY);
    state = STATE_WAITING_NOTBUSY;
    }
    host.data = core::ptr::null_mut();
    break;
    case STATE_END_REQUEST:
    atmci_writel(host, ATMCI_IDR, ATMCI_TXRDY | ATMCI_RXRDY
    | ATMCI_DATA_ERROR_FLAGS);
    status = host.data_status;
    if (unlikely(status)) {
    host.stop_transfer(host);
    host.data = core::ptr::null_mut();
    if (data) {
    if (status & ATMCI_DTOE) {
    data.error = -ETIMEDOUT;
    } else if (status & ATMCI_DCRCE) {
    data.error = -EILSEQ;
    } else {
    data.error = -EIO;
    }
    }
    }
    atmci_request_end(host, host.mrq);
    goto unlock; /* atmci_request_end() sets host.state */
    break;
    }
    } while (state != prev_state);
    host.state = state;
    unlock:
    spin_unlock(&host.lock);
    }
#[no_mangle]
unsafe extern "C" fn atmci_read_data_pio(host: *mut atmel_mci) {
    static void atmci_read_data_pio(struct atmel_mci *host)
    {
    struct scatterlist	*sg = host.sg;
    let mut offset: c_uint = host.pio_offset;
    struct mmc_data		*data = host.data;
    u32			value;
    u32			status;
    let mut nbytes: c_uint = 0;
    do {
    value = atmci_readl(host, ATMCI_RDR);
    if (likely(offset + 4 <= sg.length)) {
    sg_pcopy_from_buffer(sg, 1, &value, sizeof(u32), offset);
    offset += 4;
    nbytes += 4;
    if (offset == sg.length) {
    flush_dcache_page(sg_page(sg));
    host.sg = sg = sg_next(sg);
    host.sg_len--;
    if (!sg || !host.sg_len)
    goto done;
    offset = 0;
    }
    } else {
    let mut remaining: c_uint = sg.length - offset;
    sg_pcopy_from_buffer(sg, 1, &value, remaining, offset);
    nbytes += remaining;
    flush_dcache_page(sg_page(sg));
    host.sg = sg = sg_next(sg);
    host.sg_len--;
    if (!sg || !host.sg_len)
    goto done;
    offset = 4 - remaining;
    sg_pcopy_from_buffer(sg, 1, (u8 *)&value + remaining,
    offset, 0);
    nbytes += offset;
    }
    status = atmci_readl(host, ATMCI_SR);
    if (status & ATMCI_DATA_ERROR_FLAGS) {
    atmci_writel(host, ATMCI_IDR, (ATMCI_NOTBUSY | ATMCI_RXRDY
    | ATMCI_DATA_ERROR_FLAGS));
    host.data_status = status;
    data.bytes_xfered += nbytes;
    return;
    }
    } while (status & ATMCI_RXRDY);
    host.pio_offset = offset;
    data.bytes_xfered += nbytes;
    return;
    done:
    atmci_writel(host, ATMCI_IDR, ATMCI_RXRDY);
    atmci_writel(host, ATMCI_IER, ATMCI_NOTBUSY);
    data.bytes_xfered += nbytes;
    smp_wmb();
    atmci_set_pending(host, EVENT_XFER_COMPLETE);
    }
#[no_mangle]
unsafe extern "C" fn atmci_write_data_pio(host: *mut atmel_mci) {
    static void atmci_write_data_pio(struct atmel_mci *host)
    {
    struct scatterlist	*sg = host.sg;
    let mut offset: c_uint = host.pio_offset;
    struct mmc_data		*data = host.data;
    u32			value;
    u32			status;
    let mut nbytes: c_uint = 0;
    do {
    if (likely(offset + 4 <= sg.length)) {
    sg_pcopy_to_buffer(sg, 1, &value, sizeof(u32), offset);
    atmci_writel(host, ATMCI_TDR, value);
    offset += 4;
    nbytes += 4;
    if (offset == sg.length) {
    host.sg = sg = sg_next(sg);
    host.sg_len--;
    if (!sg || !host.sg_len)
    goto done;
    offset = 0;
    }
    } else {
    let mut remaining: c_uint = sg.length - offset;
    value = 0;
    sg_pcopy_to_buffer(sg, 1, &value, remaining, offset);
    nbytes += remaining;
    host.sg = sg = sg_next(sg);
    host.sg_len--;
    if (!sg || !host.sg_len) {
    atmci_writel(host, ATMCI_TDR, value);
    goto done;
    }
    offset = 4 - remaining;
    sg_pcopy_to_buffer(sg, 1, (u8 *)&value + remaining,
    offset, 0);
    atmci_writel(host, ATMCI_TDR, value);
    nbytes += offset;
    }
    status = atmci_readl(host, ATMCI_SR);
    if (status & ATMCI_DATA_ERROR_FLAGS) {
    atmci_writel(host, ATMCI_IDR, (ATMCI_NOTBUSY | ATMCI_TXRDY
    | ATMCI_DATA_ERROR_FLAGS));
    host.data_status = status;
    data.bytes_xfered += nbytes;
    return;
    }
    } while (status & ATMCI_TXRDY);
    host.pio_offset = offset;
    data.bytes_xfered += nbytes;
    return;
    done:
    atmci_writel(host, ATMCI_IDR, ATMCI_TXRDY);
    atmci_writel(host, ATMCI_IER, ATMCI_NOTBUSY);
    data.bytes_xfered += nbytes;
    smp_wmb();
    atmci_set_pending(host, EVENT_XFER_COMPLETE);
    }
#[no_mangle]
unsafe extern "C" fn atmci_sdio_interrupt(host: *mut atmel_mci, status: u32) {
    static void atmci_sdio_interrupt(struct atmel_mci *host, u32 status)
    {
    int	i;
    for (i = 0; i < ATMCI_MAX_NR_SLOTS; i++) {
    struct atmel_mci_slot *slot = host.slot[i];
    if (slot && (status & slot.sdio_irq)) {
    mmc_signal_sdio_irq(slot.mmc);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn atmci_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t atmci_interrupt(int irq, void *dev_id)
    {
    struct atmel_mci	*host = dev_id;
    struct device		*dev = host.dev;
    u32			status, mask, pending;
    let mut pass_count: c_uint = 0;
    do {
    status = atmci_readl(host, ATMCI_SR);
    mask = atmci_readl(host, ATMCI_IMR);
    pending = status & mask;
    if (!pending)
    break;
    if (pending & ATMCI_DATA_ERROR_FLAGS) {
    dev_dbg(dev, "IRQ: data error\n");
    atmci_writel(host, ATMCI_IDR, ATMCI_DATA_ERROR_FLAGS
    | ATMCI_RXRDY | ATMCI_TXRDY
    | ATMCI_ENDRX | ATMCI_ENDTX
    | ATMCI_RXBUFF | ATMCI_TXBUFE);
    host.data_status = status;
    dev_dbg(dev, "set pending data error\n");
    smp_wmb();
    atmci_set_pending(host, EVENT_DATA_ERROR);
    queue_work(system_bh_wq, &host.bh_work);
    }
    if (pending & ATMCI_TXBUFE) {
    dev_dbg(dev, "IRQ: tx buffer empty\n");
    atmci_writel(host, ATMCI_IDR, ATMCI_TXBUFE);
    atmci_writel(host, ATMCI_IDR, ATMCI_ENDTX);
//
// We can receive this interruption before having configured
// the second pdc buffer, so we need to reconfigure first and
// second buffers again
//
    if (host.data_size) {
    atmci_pdc_set_both_buf(host, XFER_TRANSMIT);
    atmci_writel(host, ATMCI_IER, ATMCI_ENDTX);
    atmci_writel(host, ATMCI_IER, ATMCI_TXBUFE);
    } else {
    atmci_pdc_complete(host);
    }
    } else if (pending & ATMCI_ENDTX) {
    dev_dbg(dev, "IRQ: end of tx buffer\n");
    atmci_writel(host, ATMCI_IDR, ATMCI_ENDTX);
    if (host.data_size) {
    atmci_pdc_set_single_buf(host,
    XFER_TRANSMIT, PDC_SECOND_BUF);
    atmci_writel(host, ATMCI_IER, ATMCI_ENDTX);
    }
    }
    if (pending & ATMCI_RXBUFF) {
    dev_dbg(dev, "IRQ: rx buffer full\n");
    atmci_writel(host, ATMCI_IDR, ATMCI_RXBUFF);
    atmci_writel(host, ATMCI_IDR, ATMCI_ENDRX);
//
// We can receive this interruption before having configured
// the second pdc buffer, so we need to reconfigure first and
// second buffers again
//
    if (host.data_size) {
    atmci_pdc_set_both_buf(host, XFER_RECEIVE);
    atmci_writel(host, ATMCI_IER, ATMCI_ENDRX);
    atmci_writel(host, ATMCI_IER, ATMCI_RXBUFF);
    } else {
    atmci_pdc_complete(host);
    }
    } else if (pending & ATMCI_ENDRX) {
    dev_dbg(dev, "IRQ: end of rx buffer\n");
    atmci_writel(host, ATMCI_IDR, ATMCI_ENDRX);
    if (host.data_size) {
    atmci_pdc_set_single_buf(host,
    XFER_RECEIVE, PDC_SECOND_BUF);
    atmci_writel(host, ATMCI_IER, ATMCI_ENDRX);
    }
    }
//
// First mci IPs, so mainly the ones having pdc, have some
// issues with the notbusy signal. You can't get it after
// data transmission if you have not sent a stop command.
// The appropriate workaround is to use the BLKE signal.
//
    if (pending & ATMCI_BLKE) {
    dev_dbg(dev, "IRQ: blke\n");
    atmci_writel(host, ATMCI_IDR, ATMCI_BLKE);
    smp_wmb();
    dev_dbg(dev, "set pending notbusy\n");
    atmci_set_pending(host, EVENT_NOTBUSY);
    queue_work(system_bh_wq, &host.bh_work);
    }
    if (pending & ATMCI_NOTBUSY) {
    dev_dbg(dev, "IRQ: not_busy\n");
    atmci_writel(host, ATMCI_IDR, ATMCI_NOTBUSY);
    smp_wmb();
    dev_dbg(dev, "set pending notbusy\n");
    atmci_set_pending(host, EVENT_NOTBUSY);
    queue_work(system_bh_wq, &host.bh_work);
    }
    if (pending & ATMCI_RXRDY)
    atmci_read_data_pio(host);
    if (pending & ATMCI_TXRDY)
    atmci_write_data_pio(host);
    if (pending & ATMCI_CMDRDY) {
    dev_dbg(dev, "IRQ: cmd ready\n");
    atmci_writel(host, ATMCI_IDR, ATMCI_CMDRDY);
    host.cmd_status = status;
    smp_wmb();
    dev_dbg(dev, "set pending cmd rdy\n");
    atmci_set_pending(host, EVENT_CMD_RDY);
    queue_work(system_bh_wq, &host.bh_work);
    }
    if (pending & (ATMCI_SDIOIRQA | ATMCI_SDIOIRQB))
    atmci_sdio_interrupt(host, status);
    } while (pass_count++ < 5);
    return pass_count ? IRQ_HANDLED : IRQ_NONE;
    }
#[no_mangle]
unsafe extern "C" fn atmci_detect_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t atmci_detect_interrupt(int irq, void *dev_id)
    {
    struct atmel_mci_slot	*slot = dev_id;
//
// Disable interrupts until the pin has stabilized and check
// the state then. Use mod_timer() since we may be in the
// middle of the timer routine when this interrupt triggers.
//
    disable_irq_nosync(irq);
    mod_timer(&slot.detect_timer, jiffies + msecs_to_jiffies(20));
    return IRQ_HANDLED;
    }
    static int atmci_init_slot(struct atmel_mci *host,
    struct mci_slot_pdata *slot_data, unsigned int id,
    u32 sdc_reg, u32 sdio_irq)
    {
    struct device			*dev = host.dev;
    struct mmc_host			*mmc;
    struct atmel_mci_slot		*slot;
    int ret;
    mmc = devm_mmc_alloc_host(dev, sizeof(*slot));
    if (!mmc)
    return -ENOMEM;
    slot = mmc_priv(mmc);
    slot.mmc = mmc;
    slot.host = host;
    slot.detect_pin = slot_data.detect_pin;
    slot.wp_pin = slot_data.wp_pin;
    slot.sdc_reg = sdc_reg;
    slot.sdio_irq = sdio_irq;
    dev_dbg(&mmc.class_dev,
    "slot[%u]: bus_width=%u, detect_pin=%d, "
    "detect_is_active_high=%s, wp_pin=%d\n",
    id, slot_data.bus_width, desc_to_gpio(slot_data.detect_pin),
    str_true_false(!gpiod_is_active_low(slot_data.detect_pin)),
    desc_to_gpio(slot_data.wp_pin));
    mmc.ops = &atmci_ops;
    mmc.f_min = DIV_ROUND_UP(host.bus_hz, 512);
    mmc.f_max = host.bus_hz / 2;
    mmc.ocr_avail	= MMC_VDD_32_33 | MMC_VDD_33_34;
    if (sdio_irq)
    mmc.caps |= MMC_CAP_SDIO_IRQ;
    if (host.caps.has_highspeed)
    mmc.caps |= MMC_CAP_SD_HIGHSPEED;
//
// Without the read/write proof capability, it is strongly suggested to
// use only one bit for data to prevent fifo underruns and overruns
// which will corrupt data.
//
    if ((slot_data.bus_width >= 4) && host.caps.has_rwproof) {
    mmc.caps |= MMC_CAP_4_BIT_DATA;
    if (slot_data.bus_width >= 8)
    mmc.caps |= MMC_CAP_8_BIT_DATA;
    }
    if (atmci_get_version(host) < 0x200) {
    mmc.max_segs = 256;
    mmc.max_blk_size = 4095;
    mmc.max_blk_count = 256;
    mmc.max_req_size = mmc.max_blk_size * mmc.max_blk_count;
    mmc.max_seg_size = mmc.max_blk_size * mmc.max_segs;
    } else {
    mmc.max_segs = 64;
    mmc.max_req_size = 32768 * 512;
    mmc.max_blk_size = 32768;
    mmc.max_blk_count = 512;
    }
// Assume card is present initially
    set_bit(ATMCI_CARD_PRESENT, &slot.flags);
    if (slot.detect_pin) {
    if (!gpiod_get_value_cansleep(slot.detect_pin))
    clear_bit(ATMCI_CARD_PRESENT, &slot.flags);
    } else {
    dev_dbg(&mmc.class_dev, "no detect pin available\n");
    }
    if (!slot.detect_pin) {
    if (slot_data.non_removable)
    mmc.caps |= MMC_CAP_NONREMOVABLE;
    else
    mmc.caps |= MMC_CAP_NEEDS_POLL;
    }
    if (!slot.wp_pin)
    dev_dbg(&mmc.class_dev, "no WP pin available\n");
    host.slot[id] = slot;
    mmc_regulator_get_supply(mmc);
    ret = mmc_add_host(mmc);
    if (ret)
    return ret;
    if (slot.detect_pin) {
    timer_setup(&slot.detect_timer, atmci_detect_change, 0);
    ret = request_irq(gpiod_to_irq(slot.detect_pin),
    atmci_detect_interrupt,
    IRQF_TRIGGER_FALLING | IRQF_TRIGGER_RISING,
    "mmc-detect", slot);
    if (ret) {
    dev_dbg(&mmc.class_dev,
    "could not request IRQ %d for detect pin\n",
    gpiod_to_irq(slot.detect_pin));
    slot.detect_pin = core::ptr::null_mut();
    }
    }
    atmci_init_debugfs(slot);
    return 0;
    }
    static void atmci_cleanup_slot(struct atmel_mci_slot *slot,
    unsigned int id)
    {
// Debugfs stuff is cleaned up by mmc core
    set_bit(ATMCI_SHUTDOWN, &slot.flags);
    smp_wmb();
    mmc_remove_host(slot.mmc);
    if (slot.detect_pin) {
    free_irq(gpiod_to_irq(slot.detect_pin), slot);
    timer_delete_sync(&slot.detect_timer);
    }
    slot.host.slot[id] = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn atmci_configure_dma(host: *mut atmel_mci) -> c_int {
    static int atmci_configure_dma(struct atmel_mci *host)
    {
    struct device *dev = host.dev;
    host.dma.chan = dma_request_chan(dev, "rxtx");
    if (IS_ERR(host.dma.chan))
    return PTR_ERR(host.dma.chan);
    dev_info(dev, "using %s for DMA transfers\n", dma_chan_name(host.dma.chan));
    host.dma_conf.src_addr = host.mapbase + ATMCI_RDR;
    host.dma_conf.src_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    host.dma_conf.src_maxburst = 1;
    host.dma_conf.dst_addr = host.mapbase + ATMCI_TDR;
    host.dma_conf.dst_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    host.dma_conf.dst_maxburst = 1;
    host.dma_conf.device_fc = false;
    return 0;
    }
//
// HSMCI (High Speed MCI) module is not fully compatible with MCI module.
// HSMCI provides DMA support and a new config register but no more supports
// PDC.
//
#[no_mangle]
unsafe extern "C" fn atmci_get_cap(host: *mut atmel_mci) {
    static void atmci_get_cap(struct atmel_mci *host)
    {
    struct device *dev = host.dev;
    unsigned int version;
    version = atmci_get_version(host);
    dev_info(dev, "version: 0x%x\n", version);
    host.caps.has_dma_conf_reg = false;
    host.caps.has_pdc = true;
    host.caps.has_cfg_reg = false;
    host.caps.has_cstor_reg = false;
    host.caps.has_highspeed = false;
    host.caps.has_rwproof = false;
    host.caps.has_odd_clk_div = false;
    host.caps.has_bad_data_ordering = true;
    host.caps.need_reset_after_xfer = true;
    host.caps.need_blksz_mul_4 = true;
    host.caps.need_notbusy_for_read_ops = false;
// keep only major version number
    switch (version & 0xf00) {
    case 0x600:
    case 0x500:
    host.caps.has_odd_clk_div = true;
    fallthrough;
    case 0x400:
    case 0x300:
    host.caps.has_dma_conf_reg = true;
    host.caps.has_pdc = false;
    host.caps.has_cfg_reg = true;
    host.caps.has_cstor_reg = true;
    host.caps.has_highspeed = true;
    fallthrough;
    case 0x200:
    host.caps.has_rwproof = true;
    host.caps.need_blksz_mul_4 = false;
    host.caps.need_notbusy_for_read_ops = true;
    fallthrough;
    case 0x100:
    host.caps.has_bad_data_ordering = false;
    host.caps.need_reset_after_xfer = false;
    fallthrough;
    case 0x0:
    break;
    default:
    host.caps.has_pdc = false;
    dev_warn(dev, "Unmanaged mci version, set minimum capabilities\n");
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn atmci_probe(pdev: *mut platform_device) -> c_int {
    static int atmci_probe(struct platform_device *pdev)
    {
    struct device			*dev = &pdev.dev;
    struct atmel_mci		*host;
    struct resource			*regs;
    unsigned int			nr_slots;
    int				irq;
    int				ret, i;
    regs = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!regs)
    return -ENXIO;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    host = devm_kzalloc(dev, sizeof(*host), GFP_KERNEL);
    if (!host)
    return -ENOMEM;
    host.dev = dev;
    spin_lock_init(&host.lock);
    INIT_LIST_HEAD(&host.queue);
    ret = atmci_of_init(host);
    if (ret)
    return dev_err_probe(dev, ret, "Slot information not available\n");
    host.mck = devm_clk_get(dev, "mci_clk");
    if (IS_ERR(host.mck))
    return PTR_ERR(host.mck);
    host.regs = devm_ioremap(dev, regs.start, resource_size(regs));
    if (!host.regs)
    return -ENOMEM;
    ret = clk_prepare_enable(host.mck);
    if (ret)
    return ret;
    atmci_writel(host, ATMCI_CR, ATMCI_CR_SWRST);
    host.bus_hz = clk_get_rate(host.mck);
    host.mapbase = regs.start;
    INIT_WORK(&host.bh_work, atmci_work_func);
    ret = request_irq(irq, atmci_interrupt, 0, dev_name(dev), host);
    if (ret) {
    clk_disable_unprepare(host.mck);
    return ret;
    }
// Get MCI capabilities and set operations according to it
    atmci_get_cap(host);
    ret = atmci_configure_dma(host);
    if (ret == -EPROBE_DEFER) {
    clk_disable_unprepare(host.mck);
    goto err_dma_probe_defer;
    }
    if (ret == 0) {
    host.prepare_data = &atmci_prepare_data_dma;
    host.submit_data = &atmci_submit_data_dma;
    host.stop_transfer = &atmci_stop_transfer_dma;
    } else if (host.caps.has_pdc) {
    dev_info(dev, "using PDC\n");
    host.prepare_data = &atmci_prepare_data_pdc;
    host.submit_data = &atmci_submit_data_pdc;
    host.stop_transfer = &atmci_stop_transfer_pdc;
    } else {
    dev_info(dev, "using PIO\n");
    host.prepare_data = &atmci_prepare_data;
    host.submit_data = &atmci_submit_data;
    host.stop_transfer = &atmci_stop_transfer;
    }
    platform_set_drvdata(pdev, host);
    timer_setup(&host.timer, atmci_timeout_timer, 0);
    pm_runtime_get_noresume(dev);
    pm_runtime_set_active(dev);
    pm_runtime_set_autosuspend_delay(dev, AUTOSUSPEND_DELAY);
    pm_runtime_use_autosuspend(dev);
    pm_runtime_enable(dev);
// We need at least one slot to succeed
    nr_slots = 0;
    ret = -ENODEV;
    if (host.pdata[0].bus_width) {
    ret = atmci_init_slot(host, &host.pdata[0],
    0, ATMCI_SDCSEL_SLOT_A, ATMCI_SDIOIRQA);
    if (!ret) {
    nr_slots++;
    host.buf_size = host.slot[0].mmc.max_req_size;
    }
    }
    if (host.pdata[1].bus_width) {
    ret = atmci_init_slot(host, &host.pdata[1],
    1, ATMCI_SDCSEL_SLOT_B, ATMCI_SDIOIRQB);
    if (!ret) {
    nr_slots++;
    if (host.slot[1].mmc.max_req_size > host.buf_size)
    host.buf_size =
    host.slot[1].mmc.max_req_size;
    }
    }
    if (!nr_slots) {
    dev_err_probe(dev, ret, "init failed: no slot defined\n");
    goto err_init_slot;
    }
    if (!host.caps.has_rwproof) {
    host.buffer = dma_alloc_coherent(dev, host.buf_size,
    &host.buf_phys_addr,
    GFP_KERNEL);
    if (!host.buffer) {
    ret = dev_err_probe(dev, -ENOMEM, "buffer allocation failed\n");
    goto err_dma_alloc;
    }
    }
    dev_info(dev, "Atmel MCI controller at 0x%08lx irq %d, %u slots\n",
    host.mapbase, irq, nr_slots);
    pm_runtime_put_autosuspend(dev);
    return 0;
    err_dma_alloc:
    for (i = 0; i < ATMCI_MAX_NR_SLOTS; i++) {
    if (host.slot[i])
    atmci_cleanup_slot(host.slot[i], i);
    }
    err_init_slot:
    clk_disable_unprepare(host.mck);
    pm_runtime_disable(dev);
    pm_runtime_put_noidle(dev);
    timer_delete_sync(&host.timer);
    if (!IS_ERR(host.dma.chan))
    dma_release_channel(host.dma.chan);
    err_dma_probe_defer:
    free_irq(irq, host);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn atmci_remove(pdev: *mut platform_device) {
    static void atmci_remove(struct platform_device *pdev)
    {
    struct atmel_mci	*host = platform_get_drvdata(pdev);
    struct device		*dev = &pdev.dev;
    unsigned int		i;
    pm_runtime_get_sync(dev);
    if (host.buffer)
    dma_free_coherent(dev, host.buf_size, host.buffer, host.buf_phys_addr);
    for (i = 0; i < ATMCI_MAX_NR_SLOTS; i++) {
    if (host.slot[i])
    atmci_cleanup_slot(host.slot[i], i);
    }
    atmci_writel(host, ATMCI_IDR, ~0UL);
    atmci_writel(host, ATMCI_CR, ATMCI_CR_MCIDIS);
    atmci_readl(host, ATMCI_SR);
    timer_delete_sync(&host.timer);
    if (!IS_ERR(host.dma.chan))
    dma_release_channel(host.dma.chan);
    free_irq(platform_get_irq(pdev, 0), host);
    cancel_work_sync(&host.bh_work);
    clk_disable_unprepare(host.mck);
    pm_runtime_disable(dev);
    pm_runtime_put_noidle(dev);
    }
#[no_mangle]
unsafe extern "C" fn atmci_runtime_suspend(dev: *mut device) -> c_int {
    static int atmci_runtime_suspend(struct device *dev)
    {
    struct atmel_mci *host = dev_get_drvdata(dev);
    clk_disable_unprepare(host.mck);
    pinctrl_pm_select_sleep_state(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn atmci_runtime_resume(dev: *mut device) -> c_int {
    static int atmci_runtime_resume(struct device *dev)
    {
    struct atmel_mci *host = dev_get_drvdata(dev);
    pinctrl_select_default_state(dev);
    return clk_prepare_enable(host.mck);
    }
    static const struct dev_pm_ops atmci_dev_pm_ops = {
    SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend, pm_runtime_force_resume)
    RUNTIME_PM_OPS(atmci_runtime_suspend, atmci_runtime_resume, core::ptr::null_mut())
    };
    static struct platform_driver atmci_driver = {
    .probe		= atmci_probe,
    .remove		= atmci_remove,
    .driver		= {
    .name		= "atmel_mci",
    .probe_type	= PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table	= atmci_dt_ids,
    .pm		= pm_ptr(&atmci_dev_pm_ops),
    },
    };
    module_platform_driver(atmci_driver);
    MODULE_DESCRIPTION("Atmel Multimedia Card Interface driver");
    MODULE_AUTHOR("Haavard Skinnemoen (Atmel)");
    MODULE_LICENSE("GPL v2");
