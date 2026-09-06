//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-tegra.c
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
// drivers/i2c/busses/i2c-tegra.c
//
// Copyright (C) 2010 Google, Inc.
// Author: Colin Cross <ccross@android.com>
//

pub const BYTES_PER_FIFO_WORD: c_int = 4;

pub const DVC_CTRL_REG1: c_uint = 0x000;

pub const DVC_CTRL_REG3: c_uint = 0x008;

pub const DVC_STATUS: c_uint = 0x00c;

pub const I2C_ERR_NONE: c_uint = 0x00;

pub const PACKET_HEADER0_PROTOCOL_I2C: c_int = 1;

pub const I2C_HEADER_SLAVE_ADDR_SHIFT: c_int = 1;

pub const I2C_SW_MUTEX_ID_CCPLEX: c_int = 9;
// SW mutex acquire timeout value in microseconds.

// configuration load timeout in microseconds
pub const I2C_CONFIG_LOAD_TIMEOUT: c_int = 1000000;
// packet header size in bytes
pub const I2C_PACKET_HEADER_SIZE: c_int = 12;
//
// I2C Controller will use PIO mode for transfers up to 32 bytes in order to
// avoid DMA overhead, otherwise external APB DMA controller will be used.
// Note that the actual MAX PIO length is 20 bytes because 32 bytes include
// I2C_PACKET_HEADER_SIZE.
//
pub const I2C_PIO_MODE_PREFERRED_LEN: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_i2c_regs {
    pub cnfg: c_uint,
    pub status: c_uint,
    pub sl_cnfg: c_uint,
    pub sl_addr1: c_uint,
    pub sl_addr2: c_uint,
    pub tlow_sext: c_uint,
    pub tx_fifo: c_uint,
    pub rx_fifo: c_uint,
    pub packet_transfer_status: c_uint,
    pub fifo_control: c_uint,
    pub fifo_status: c_uint,
    pub int_mask: c_uint,
    pub int_status: c_uint,
    pub clk_divisor: c_uint,
    pub bus_clear_cnfg: c_uint,
    pub bus_clear_status: c_uint,
    pub config_load: c_uint,
    pub clken_override: c_uint,
    pub interface_timing_0: c_uint,
    pub interface_timing_1: c_uint,
    pub hs_interface_timing_0: c_uint,
    pub hs_interface_timing_1: c_uint,
    pub master_reset_cntrl: c_uint,
    pub mst_fifo_control: c_uint,
    pub mst_fifo_status: c_uint,
    pub fairness_arb: c_uint,
    pub sw_mutex: c_uint,
}

    static const struct tegra_i2c_regs tegra20_i2c_regs = {
    .cnfg = 0x000,
    .status = 0x01c,
    .sl_cnfg = 0x020,
    .sl_addr1 = 0x02c,
    .sl_addr2 = 0x030,
    .tx_fifo = 0x050,
    .rx_fifo = 0x054,
    .packet_transfer_status = 0x058,
    .fifo_control = 0x05c,
    .fifo_status = 0x060,
    .int_mask = 0x064,
    .int_status = 0x068,
    .clk_divisor = 0x06c,
    .bus_clear_cnfg = 0x084,
    .bus_clear_status = 0x088,
    .config_load = 0x08c,
    .clken_override = 0x090,
    .interface_timing_0 = 0x094,
    .interface_timing_1 = 0x098,
    .hs_interface_timing_0 = 0x09c,
    .hs_interface_timing_1 = 0x0a0,
    .master_reset_cntrl = 0x0a8,
    .mst_fifo_control = 0x0b4,
    .mst_fifo_status = 0x0b8,
    };

    static const struct tegra_i2c_regs tegra20_dvc_i2c_regs = {
    .cnfg = 0x040,
    .status = 0x05c,
    .tx_fifo = 0x060,
    .rx_fifo = 0x064,
    .packet_transfer_status = 0x068,
    .fifo_control = 0x06c,
    .fifo_status = 0x070,
    .int_mask = 0x074,
    .int_status = 0x078,
    .clk_divisor = 0x07c,
    .bus_clear_cnfg = 0x094,
    .bus_clear_status = 0x098,
    .config_load = 0x09c,
    .clken_override = 0x0a0,
    .interface_timing_0 = 0x0a4,
    .interface_timing_1 = 0x0a8,
    .hs_interface_timing_0 = 0x0ac,
    .hs_interface_timing_1 = 0x0b0,
    .master_reset_cntrl = 0x0b8,
    .mst_fifo_control = 0x0c4,
    .mst_fifo_status = 0x0c8,
    };

    static const struct tegra_i2c_regs tegra210_vi_i2c_regs = {
    .cnfg = 0x0c00,
    .status = 0x0c70,
    .tlow_sext = 0x0cd0,
    .tx_fifo = 0x0d40,
    .rx_fifo = 0x0d50,
    .packet_transfer_status = 0x0d60,
    .fifo_control = 0x0d70,
    .fifo_status = 0x0d80,
    .int_mask = 0x0d90,
    .int_status = 0x0da0,
    .clk_divisor = 0x0db0,
    .bus_clear_cnfg = 0x0e10,
    .bus_clear_status = 0x0e20,
    .config_load = 0x0e30,
    .clken_override = 0x0e40,
    .interface_timing_0 = 0x0e50,
    .interface_timing_1 = 0x0e60,
    .hs_interface_timing_0 = 0x0e70,
    .hs_interface_timing_1 = 0x0e80,
    .master_reset_cntrl = 0x0ea0,
    .mst_fifo_control = 0x0ed0,
    .mst_fifo_status = 0x0ee0,
    };

    static const struct tegra_i2c_regs tegra264_i2c_regs = {
    .cnfg = 0x000,
    .status = 0x01c,
    .sl_cnfg = 0x020,
    .sl_addr1 = 0x02c,
    .sl_addr2 = 0x030,
    .tx_fifo = 0x050,
    .rx_fifo = 0x054,
    .packet_transfer_status = 0x058,
    .fifo_control = 0x05c,
    .fifo_status = 0x060,
    .int_mask = 0x064,
    .int_status = 0x068,
    .clk_divisor = 0x06c,
    .bus_clear_cnfg = 0x084,
    .bus_clear_status = 0x088,
    .config_load = 0x08c,
    .clken_override = 0x090,
    .interface_timing_0 = 0x094,
    .interface_timing_1 = 0x098,
    .hs_interface_timing_0 = 0x09c,
    .hs_interface_timing_1 = 0x0a0,
    .master_reset_cntrl = 0x0a8,
    .mst_fifo_control = 0x0b4,
    .mst_fifo_status = 0x0b8,
    .fairness_arb = 0x0e8,
    .sw_mutex = 0x0ec,
    };
    static const struct tegra_i2c_regs tegra410_i2c_regs = {
    .cnfg = 0x000,
    .status = 0x01c,
    .sl_cnfg = 0x020,
    .sl_addr1 = 0x02c,
    .sl_addr2 = 0x030,
    .tx_fifo = 0x054,
    .rx_fifo = 0x058,
    .packet_transfer_status = 0x05c,
    .fifo_control = 0x060,
    .fifo_status = 0x064,
    .int_mask = 0x068,
    .int_status = 0x06c,
    .clk_divisor = 0x070,
    .bus_clear_cnfg = 0x088,
    .bus_clear_status = 0x08c,
    .config_load = 0x090,
    .clken_override = 0x094,
    .interface_timing_0 = 0x098,
    .interface_timing_1 = 0x09c,
    .hs_interface_timing_0 = 0x0a0,
    .hs_interface_timing_1 = 0x0a4,
    .master_reset_cntrl = 0x0ac,
    .mst_fifo_control = 0x0b8,
    .mst_fifo_status = 0x0bc,
    .fairness_arb = 0x0ec,
    .sw_mutex = 0x0f0,
    };
//
// msg_end_type: The bus control which needs to be sent at end of transfer.
// @MSG_END_STOP: Send stop pulse.
// @MSG_END_REPEAT_START: Send repeat-start.
// @MSG_END_CONTINUE: Don't send stop or repeat-start.
//
    enum msg_end_type {
    MSG_END_STOP,
    MSG_END_REPEAT_START,
    MSG_END_CONTINUE,
    };
//
// tegra_i2c_variant: Identifies the variant of I2C controller.
// @TEGRA_I2C_VARIANT_DEFAULT: Identifies the default I2C controller.
// @TEGRA_I2C_VARIANT_DVC: Identifies the DVC I2C controller, has a different register layout.
// @TEGRA_I2C_VARIANT_VI: Identifies the VI I2C controller, has a different register layout.
//
    enum tegra_i2c_variant {
    TEGRA_I2C_VARIANT_DEFAULT,
    TEGRA_I2C_VARIANT_DVC,
    TEGRA_I2C_VARIANT_VI,
    };
//
// struct tegra_i2c_hw_feature : per hardware generation features
// @has_continue_xfer_support: continue-transfer supported
// @has_per_pkt_xfer_complete_irq: Has enable/disable capability for transfer
// completion interrupt on per packet basis.
// @has_config_load_reg: Has the config load register to load the new
// configuration.
// @clk_divisor_hs_mode: Clock divisor in HS mode.
// @clk_divisor_std_mode: Clock divisor in standard mode. It is
// applicable if there is no fast clock source i.e. single clock
// source.
// @clk_divisor_fast_mode: Clock divisor in fast mode. It is
// applicable if there is no fast clock source i.e. single clock
// source.
// @clk_divisor_fast_plus_mode: Clock divisor in fast mode plus. It is
// applicable if there is no fast clock source (i.e. single
// clock source).
// @has_multi_master_mode: The I2C controller supports running in single-master
// or multi-master mode.
// @has_slcg_override_reg: The I2C controller supports a register that
// overrides the second level clock gating.
// @has_mst_fifo: The I2C controller contains the new MST FIFO interface that
// provides additional features and allows for longer messages to
// be transferred in one go.
// @has_mst_reset: The I2C controller contains MASTER_RESET_CTRL register which
// provides an alternative to controller reset when configured as
// I2C master
// @quirks: I2C adapter quirks for limiting write/read transfer size and not
// allowing 0 length transfers.
// @supports_bus_clear: Bus Clear support to recover from bus hang during
// SDA stuck low from device for some unknown reasons.
// @has_apb_dma: Support of APBDMA on corresponding Tegra chip.
// @tlow_std_mode: Low period of the clock in standard mode.
// @thigh_std_mode: High period of the clock in standard mode.
// @tlow_fast_mode: Low period of the clock in fast mode.
// @thigh_fast_mode: High period of the clock in fast mode.
// @tlow_fastplus_mode: Low period of the clock in fast-plus mode.
// @thigh_fastplus_mode: High period of the clock in fast-plus mode.
// @tlow_hs_mode: Low period of the clock in HS mode.
// @thigh_hs_mode: High period of the clock in HS mode.
// @setup_hold_time_std_mode: Setup and hold time for start and stop conditions
// in standard mode.
// @setup_hold_time_fast_mode: Setup and hold time for start and stop
// conditions in fast mode.
// @setup_hold_time_fastplus_mode: Setup and hold time for start and stop
// conditions in fast-plus mode.
// @setup_hold_time_hs_mode: Setup and hold time for start and stop conditions
// in HS mode.
// @has_interface_timing_reg: Has interface timing register to program the tuned
// timing settings.
// @enable_hs_mode_support: Enable support for high speed (HS) mode transfers.
// @has_mutex: Has mutex register for mutual exclusion with other firmwares or VMs.
// @has_fairarb_reg: Has fairness arbitration register for SMBUS/MCTP support.
// @variant: This represents the I2C controller variant.
// @regs: Register offsets for the specific SoC variant.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_i2c_hw_feature {
    pub has_continue_xfer_support: bool,
    pub has_per_pkt_xfer_complete_irq: bool,
    pub has_config_load_reg: bool,
    pub clk_divisor_hs_mode: u32,
    pub clk_divisor_std_mode: u32,
    pub clk_divisor_fast_mode: u32,
    pub clk_divisor_fast_plus_mode: u32,
    pub has_multi_master_mode: bool,
    pub has_slcg_override_reg: bool,
    pub has_mst_fifo: bool,
    pub has_mst_reset: bool,
    pub quirks: *const i2c_adapter_quirks,
    pub supports_bus_clear: bool,
    pub has_apb_dma: bool,
    pub tlow_std_mode: u32,
    pub thigh_std_mode: u32,
    pub tlow_fast_mode: u32,
    pub thigh_fast_mode: u32,
    pub tlow_fastplus_mode: u32,
    pub thigh_fastplus_mode: u32,
    pub tlow_hs_mode: u32,
    pub thigh_hs_mode: u32,
    pub setup_hold_time_std_mode: u32,
    pub setup_hold_time_fast_mode: u32,
    pub setup_hold_time_fastplus_mode: u32,
    pub setup_hold_time_hs_mode: u32,
    pub has_interface_timing_reg: bool,
    pub enable_hs_mode_support: bool,
    pub has_mutex: bool,
    pub has_fairarb_reg: bool,
    pub variant: enum tegra_i2c_variant,
    pub regs: *const tegra_i2c_regs,
}

//
// struct tegra_i2c_dev - per device I2C context
// @dev: device reference for power management
// @hw: Tegra I2C HW feature
// @adapter: core I2C layer adapter information
// @div_clk: clock reference for div clock of I2C controller
// @clocks: array of I2C controller clocks
// @nclocks: number of clocks in the array
// @base: ioremapped registers cookie
// @base_phys: physical base address of the I2C controller
// @cont_id: I2C controller ID, used for packet header
// @irq: IRQ number of transfer complete interrupt
// @msg_complete: transfer completion notifier
// @msg_buf_remaining: size of unsent data in the message buffer
// @msg_len: length of message in current transfer
// @msg_err: error code for completed message
// @msg_buf: pointer to current message data
// @msg_read: indicates that the transfer is a read access
// @timings: i2c timings information like bus frequency
// @multimaster_mode: indicates that I2C controller is in multi-master mode
// @is_mctp: indicates that the I2C controller is used as an MCTP controller
// @dma_chan: DMA channel
// @dma_phys: handle to DMA resources
// @dma_buf: pointer to allocated DMA buffer
// @dma_buf_size: DMA buffer size
// @dma_dev: DMA device used for transfers
// @dma_mode: indicates active DMA transfer
// @dma_complete: DMA completion notifier
// @atomic_mode: indicates active atomic transfer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_i2c_dev {
    pub dev: *mut device,
    pub adapter: i2c_adapter,
    pub hw: *const tegra_i2c_hw_feature,
    pub cont_id: c_uint,
    pub irq: c_uint,
    pub base_phys: phys_addr_t,
    pub base: *mut void __iomem,
    pub clocks: [clk_bulk_data; 2],
    pub nclocks: c_uint,
    pub div_clk: *mut clk,
    pub timings: i2c_timings,
    pub msg_complete: completion,
    pub msg_buf_remaining: usize,
    pub msg_len: c_uint,
    pub msg_err: c_int,
    pub msg_buf: *mut u8,
    pub dma_complete: completion,
    pub dma_chan: *mut dma_chan,
    pub dma_buf_size: c_uint,
    pub dma_dev: *mut device,
    pub dma_phys: dma_addr_t,
    pub dma_buf: *mut c_void,
    pub multimaster_mode: bool,
    pub is_mctp: bool,
    pub atomic_mode: bool,
    pub dma_mode: bool,
    pub msg_read: bool,
}

    (dev).hw.variant == TEGRA_I2C_VARIANT_DVC)

    (dev).hw.variant == TEGRA_I2C_VARIANT_VI)
    static void dvc_writel(struct tegra_i2c_dev *i2c_dev, u32 val,
    unsigned int reg)
    {
    writel_relaxed(val, i2c_dev.base + reg);
    }
#[no_mangle]
unsafe extern "C" fn dvc_readl(i2c_dev: *mut tegra_i2c_dev, reg: c_uint) -> u32 {
    static u32 dvc_readl(struct tegra_i2c_dev *i2c_dev, unsigned int reg)
    {
    return readl_relaxed(i2c_dev.base + reg);
    }
#[no_mangle]
unsafe extern "C" fn i2c_writel(i2c_dev: *mut tegra_i2c_dev, val: u32, reg: c_uint) {
    static void i2c_writel(struct tegra_i2c_dev *i2c_dev, u32 val, unsigned int reg)
    {
    writel_relaxed(val, i2c_dev.base + reg);
// read back register to make sure that register writes completed
    if (reg != i2c_dev.hw.regs.tx_fifo)
    readl_relaxed(i2c_dev.base + reg);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: IS_VI(i2c_dev)) -> else {
    else if (IS_VI(i2c_dev))
    readl_relaxed(i2c_dev.base + i2c_dev.hw.regs.int_status);
    }
#[no_mangle]
unsafe extern "C" fn i2c_readl(i2c_dev: *mut tegra_i2c_dev, reg: c_uint) -> u32 {
    static u32 i2c_readl(struct tegra_i2c_dev *i2c_dev, unsigned int reg)
    {
    return readl_relaxed(i2c_dev.base + reg);
    }
    static void i2c_writesl(struct tegra_i2c_dev *i2c_dev, void *data,
    unsigned int reg, unsigned int len)
    {
    writesl(i2c_dev.base + reg, data, len);
    }
    static void i2c_writesl_vi(struct tegra_i2c_dev *i2c_dev, void *data,
    unsigned int reg, unsigned int len)
    {
    u32 *data32 = data;
//
// VI I2C controller has known hardware bug where writes get stuck
// when immediate multiple writes happen to TX_FIFO register.
// Recommended software work around is to read I2C register after
// each write to TX_FIFO register to flush out the data.
//
    while (len--)
    i2c_writel(i2c_dev, *data32++, reg);
    }
    static void i2c_readsl(struct tegra_i2c_dev *i2c_dev, void *data,
    unsigned int reg, unsigned int len)
    {
    readsl(i2c_dev.base + reg, data, len);
    }
#[no_mangle]
unsafe extern "C" fn tegra_i2c_mutex_acquired(i2c_dev: *mut tegra_i2c_dev) -> bool {
    static bool tegra_i2c_mutex_acquired(struct tegra_i2c_dev *i2c_dev)
    {
    let mut reg: c_uint = i2c_dev.hw.regs.sw_mutex;
    u32 val, id;
    val = readl(i2c_dev.base + reg);
    id = FIELD_GET(I2C_SW_MUTEX_GRANT, val);
    let mut id: return = = I2C_SW_MUTEX_ID_CCPLEX;
    }
#[no_mangle]
unsafe extern "C" fn tegra_i2c_mutex_trylock(i2c_dev: *mut tegra_i2c_dev) -> bool {
    static bool tegra_i2c_mutex_trylock(struct tegra_i2c_dev *i2c_dev)
    {
    let mut reg: c_uint = i2c_dev.hw.regs.sw_mutex;
    u32 val, id;
    val = readl(i2c_dev.base + reg);
    id = FIELD_GET(I2C_SW_MUTEX_GRANT, val);
    if (id != 0 && id != I2C_SW_MUTEX_ID_CCPLEX)
    return false;
    val = FIELD_PREP(I2C_SW_MUTEX_REQUEST, I2C_SW_MUTEX_ID_CCPLEX);
    writel(val, i2c_dev.base + reg);
    return tegra_i2c_mutex_acquired(i2c_dev);
    }
#[no_mangle]
unsafe extern "C" fn tegra_i2c_mutex_lock(i2c_dev: *mut tegra_i2c_dev) -> c_int {
    static int tegra_i2c_mutex_lock(struct tegra_i2c_dev *i2c_dev)
    {
    bool locked;
    int ret;
    if (!i2c_dev.hw.has_mutex)
    return 0;
    if (i2c_dev.atomic_mode)
    ret = read_poll_timeout_atomic(tegra_i2c_mutex_trylock, locked, locked,
    USEC_PER_MSEC, I2C_SW_MUTEX_TIMEOUT_US,
    false, i2c_dev);
    else
    ret = read_poll_timeout(tegra_i2c_mutex_trylock, locked, locked, USEC_PER_MSEC,
    I2C_SW_MUTEX_TIMEOUT_US, false, i2c_dev);
    if (ret)
    dev_warn(i2c_dev.dev, "failed to acquire mutex\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tegra_i2c_mutex_unlock(i2c_dev: *mut tegra_i2c_dev) {
    static void tegra_i2c_mutex_unlock(struct tegra_i2c_dev *i2c_dev)
    {
    let mut reg: c_uint = i2c_dev.hw.regs.sw_mutex;
    u32 val, id;
    if (!i2c_dev.hw.has_mutex)
    return;
    val = readl(i2c_dev.base + reg);
    id = FIELD_GET(I2C_SW_MUTEX_GRANT, val);
    if (WARN(id && id != I2C_SW_MUTEX_ID_CCPLEX,
    "unable to unlock mutex, mutex is owned by: %u\n", id))
    return;
    writel(0, i2c_dev.base + reg);
    }
#[no_mangle]
unsafe extern "C" fn tegra_i2c_mask_irq(i2c_dev: *mut tegra_i2c_dev, mask: u32) {
    static void tegra_i2c_mask_irq(struct tegra_i2c_dev *i2c_dev, u32 mask)
    {
    u32 int_mask;
    int_mask = i2c_readl(i2c_dev, i2c_dev.hw.regs.int_mask) & ~mask;
    i2c_writel(i2c_dev, int_mask, i2c_dev.hw.regs.int_mask);
    }
#[no_mangle]
unsafe extern "C" fn tegra_i2c_unmask_irq(i2c_dev: *mut tegra_i2c_dev, mask: u32) {
    static void tegra_i2c_unmask_irq(struct tegra_i2c_dev *i2c_dev, u32 mask)
    {
    u32 int_mask;
    int_mask = i2c_readl(i2c_dev, i2c_dev.hw.regs.int_mask) | mask;
    i2c_writel(i2c_dev, int_mask, i2c_dev.hw.regs.int_mask);
    }
#[no_mangle]
unsafe extern "C" fn tegra_i2c_dma_complete(args: *mut c_void) {
    static void tegra_i2c_dma_complete(void *args)
    {
    struct tegra_i2c_dev *i2c_dev = args;
    complete(&i2c_dev.dma_complete);
    }
#[no_mangle]
unsafe extern "C" fn tegra_i2c_dma_submit(i2c_dev: *mut tegra_i2c_dev, len: usize) -> c_int {
    static int tegra_i2c_dma_submit(struct tegra_i2c_dev *i2c_dev, size_t len)
    {
    struct dma_async_tx_descriptor *dma_desc;
    enum dma_transfer_direction dir;
    dev_dbg(i2c_dev.dev, "starting DMA for length: %zu\n", len);
    reinit_completion(&i2c_dev.dma_complete);
    dir = i2c_dev.msg_read ? DMA_DEV_TO_MEM : DMA_MEM_TO_DEV;
    dma_desc = dmaengine_prep_slave_single(i2c_dev.dma_chan, i2c_dev.dma_phys,
    len, dir, DMA_PREP_INTERRUPT |
    DMA_CTRL_ACK);
    if (!dma_desc) {
    dev_err(i2c_dev.dev, "failed to get %s DMA descriptor\n",
    i2c_dev.msg_read ? "RX" : "TX");
    return -EINVAL;
    }
    dma_desc.callback = tegra_i2c_dma_complete;
    dma_desc.callback_param = i2c_dev;
    dmaengine_submit(dma_desc);
    dma_async_issue_pending(i2c_dev.dma_chan);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_i2c_release_dma(i2c_dev: *mut tegra_i2c_dev) {
    static void tegra_i2c_release_dma(struct tegra_i2c_dev *i2c_dev)
    {
    if (i2c_dev.dma_buf) {
    dma_free_coherent(i2c_dev.dma_dev, i2c_dev.dma_buf_size,
    i2c_dev.dma_buf, i2c_dev.dma_phys);
    i2c_dev.dma_buf = core::ptr::null_mut();
    }
    if (i2c_dev.dma_chan) {
    dma_release_channel(i2c_dev.dma_chan);
    i2c_dev.dma_chan = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn tegra_i2c_init_dma(i2c_dev: *mut tegra_i2c_dev) -> c_int {
    static int tegra_i2c_init_dma(struct tegra_i2c_dev *i2c_dev)
    {
    dma_addr_t dma_phys;
    u32 *dma_buf;
    int err;
    if (IS_VI(i2c_dev))
    return 0;
    if (!of_property_present(i2c_dev.dev.of_node, "dmas")) {
    dev_dbg(i2c_dev.dev, "DMA not available, falling back to PIO\n");
    return 0;
    }
    if (i2c_dev.hw.has_apb_dma) {
    if (!IS_ENABLED(CONFIG_TEGRA20_APB_DMA)) {
    dev_dbg(i2c_dev.dev, "APB DMA support not enabled\n");
    return 0;
    }
    } else if (!IS_ENABLED(CONFIG_TEGRA186_GPC_DMA)) {
    dev_dbg(i2c_dev.dev, "GPC DMA support not enabled\n");
    return 0;
    }
//
// The same channel will be used for both RX and TX.
// Keeping the name as "tx" for backward compatibility
// with existing devicetrees.
//
    i2c_dev.dma_chan = dma_request_chan(i2c_dev.dev, "tx");
    if (IS_ERR(i2c_dev.dma_chan)) {
    err = PTR_ERR(i2c_dev.dma_chan);
    i2c_dev.dma_chan = core::ptr::null_mut();
    goto err_out;
    }
    i2c_dev.dma_dev = dmaengine_get_dma_device(i2c_dev.dma_chan);
    i2c_dev.dma_buf_size = i2c_dev.hw.quirks.max_write_len +
    I2C_PACKET_HEADER_SIZE;
    dma_buf = dma_alloc_coherent(i2c_dev.dma_dev, i2c_dev.dma_buf_size,
    &dma_phys, GFP_KERNEL | __GFP_NOWARN);
    if (!dma_buf) {
    err = dev_err_probe(i2c_dev.dev, -ENOMEM,
    "failed to allocate DMA buffer\n");
    goto err_out;
    }
    i2c_dev.dma_buf = dma_buf;
    i2c_dev.dma_phys = dma_phys;
    return 0;
    err_out:
    tegra_i2c_release_dma(i2c_dev);
    if (err != -EPROBE_DEFER) {
    dev_err(i2c_dev.dev, "cannot use DMA, falling back to PIO\n");
    return 0;
    }
    return err;
    }
//
// One of the Tegra I2C blocks is inside the DVC (Digital Voltage Controller)
// block.  This block is identical to the rest of the I2C blocks, except that
// it only supports master mode, it has registers moved around, and it needs
// some extra init to get it into I2C mode.  The register moves are handled
// by i2c_readl() and i2c_writel().
//
#[no_mangle]
unsafe extern "C" fn tegra_dvc_init(i2c_dev: *mut tegra_i2c_dev) {
    static void tegra_dvc_init(struct tegra_i2c_dev *i2c_dev)
    {
    u32 val;
    val = dvc_readl(i2c_dev, DVC_CTRL_REG3);
    val |= DVC_CTRL_REG3_SW_PROG;
    val |= DVC_CTRL_REG3_I2C_DONE_INTR_EN;
    dvc_writel(i2c_dev, val, DVC_CTRL_REG3);
    val = dvc_readl(i2c_dev, DVC_CTRL_REG1);
    val |= DVC_CTRL_REG1_INTR_EN;
    dvc_writel(i2c_dev, val, DVC_CTRL_REG1);
    }
#[no_mangle]
unsafe extern "C" fn tegra_i2c_vi_init(i2c_dev: *mut tegra_i2c_dev) {
    static void tegra_i2c_vi_init(struct tegra_i2c_dev *i2c_dev)
    {
    u32 value;
    value = FIELD_PREP(I2C_INTERFACE_TIMING_THIGH, 2) |
    FIELD_PREP(I2C_INTERFACE_TIMING_TLOW, 4);
    i2c_writel(i2c_dev, value, i2c_dev.hw.regs.interface_timing_0);
    value = FIELD_PREP(I2C_INTERFACE_TIMING_TBUF, 4) |
    FIELD_PREP(I2C_INTERFACE_TIMING_TSU_STO, 7) |
    FIELD_PREP(I2C_INTERFACE_TIMING_THD_STA, 4) |
    FIELD_PREP(I2C_INTERFACE_TIMING_TSU_STA, 4);
    i2c_writel(i2c_dev, value, i2c_dev.hw.regs.interface_timing_1);
    value = FIELD_PREP(I2C_HS_INTERFACE_TIMING_THIGH, 3) |
    FIELD_PREP(I2C_HS_INTERFACE_TIMING_TLOW, 8);
    i2c_writel(i2c_dev, value, i2c_dev.hw.regs.hs_interface_timing_0);
    value = FIELD_PREP(I2C_HS_INTERFACE_TIMING_TSU_STO, 11) |
    FIELD_PREP(I2C_HS_INTERFACE_TIMING_THD_STA, 11) |
    FIELD_PREP(I2C_HS_INTERFACE_TIMING_TSU_STA, 11);
    i2c_writel(i2c_dev, value, i2c_dev.hw.regs.hs_interface_timing_1);
    value = FIELD_PREP(I2C_BC_SCLK_THRESHOLD, 9) | I2C_BC_STOP_COND;
    i2c_writel(i2c_dev, value, i2c_dev.hw.regs.bus_clear_cnfg);
    i2c_writel(i2c_dev, 0x0, i2c_dev.hw.regs.tlow_sext);
    }
    static int tegra_i2c_poll_register(struct tegra_i2c_dev *i2c_dev,
    u32 reg, u32 mask, u32 delay_us,
    u32 timeout_us)
    {
    void __iomem *addr = i2c_dev.base + reg;
    u32 val;
    if (!i2c_dev.atomic_mode)
    return readl_relaxed_poll_timeout(addr, val, !(val & mask),
    delay_us, timeout_us);
    return readl_relaxed_poll_timeout_atomic(addr, val, !(val & mask),
    delay_us, timeout_us);
    }
#[no_mangle]
unsafe extern "C" fn tegra_i2c_flush_fifos(i2c_dev: *mut tegra_i2c_dev) -> c_int {
    static int tegra_i2c_flush_fifos(struct tegra_i2c_dev *i2c_dev)
    {
    u32 mask, val, offset;
    int err;
    if (i2c_dev.hw.has_mst_fifo) {
    mask = I2C_MST_FIFO_CONTROL_TX_FLUSH |
    I2C_MST_FIFO_CONTROL_RX_FLUSH;
    offset = i2c_dev.hw.regs.mst_fifo_control;
    } else {
    mask = I2C_FIFO_CONTROL_TX_FLUSH |
    I2C_FIFO_CONTROL_RX_FLUSH;
    offset = i2c_dev.hw.regs.fifo_control;
    }
    val = i2c_readl(i2c_dev, offset);
    val |= mask;
    i2c_writel(i2c_dev, val, offset);
    err = tegra_i2c_poll_register(i2c_dev, offset, mask, 1000, 1000000);
    if (err) {
    dev_err(i2c_dev.dev, "failed to flush FIFO\n");
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_i2c_wait_for_config_load(i2c_dev: *mut tegra_i2c_dev) -> c_int {
    static int tegra_i2c_wait_for_config_load(struct tegra_i2c_dev *i2c_dev)
    {
    int err;
    if (!i2c_dev.hw.has_config_load_reg)
    return 0;
    i2c_writel(i2c_dev, I2C_MSTR_CONFIG_LOAD, i2c_dev.hw.regs.config_load);
    err = tegra_i2c_poll_register(i2c_dev, i2c_dev.hw.regs.config_load, 0xffffffff,
    1000, I2C_CONFIG_LOAD_TIMEOUT);
    if (err) {
    dev_err(i2c_dev.dev, "failed to load config\n");
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_i2c_master_reset(i2c_dev: *mut tegra_i2c_dev) -> c_int {
    static int tegra_i2c_master_reset(struct tegra_i2c_dev *i2c_dev)
    {
    if (!i2c_dev.hw.has_mst_reset)
    return -EOPNOTSUPP;
//
// Writing 1 to I2C_MASTER_RESET_CNTRL will reset all internal state of
// Master logic including FIFOs. Clear this bit to 0 for normal operation.
// SW needs to wait for 2us after assertion and de-assertion of this soft
// reset.
//
    i2c_writel(i2c_dev, 0x1, i2c_dev.hw.regs.master_reset_cntrl);
    fsleep(2);
    i2c_writel(i2c_dev, 0x0, i2c_dev.hw.regs.master_reset_cntrl);
    fsleep(2);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_i2c_init(i2c_dev: *mut tegra_i2c_dev) -> c_int {
    static int tegra_i2c_init(struct tegra_i2c_dev *i2c_dev)
    {
    u32 val, clk_divisor, clk_multiplier, tsu_thd, tlow, thigh, non_hs_mode;
    u32 max_bus_freq_hz;
    struct i2c_timings *t = &i2c_dev.timings;
    int err;
//
// Reset the controller before initializing it.
// In case if device_reset() returns -ENOENT, i.e. when the reset is
// not available, the internal software reset will be used if it is
// supported by the controller.
//
    err = device_reset(i2c_dev.dev);
    if (err == -ENOENT)
    err = tegra_i2c_master_reset(i2c_dev);
//
// The reset shouldn't ever fail in practice. The failure will be a
// sign of a severe problem that needs to be resolved. Still we don't
// want to fail the initialization completely because this may break
// kernel boot up since voltage regulators use I2C. Hence, we will
// emit a noisy warning on error, which won't stay unnoticed and
// won't hose machine entirely.
//
    WARN_ON_ONCE(err);
    if (IS_DVC(i2c_dev))
    tegra_dvc_init(i2c_dev);
    val = I2C_CNFG_NEW_MASTER_FSM | I2C_CNFG_PACKET_MODE_EN |
    FIELD_PREP(I2C_CNFG_DEBOUNCE_CNT, 2);
    if (i2c_dev.hw.has_multi_master_mode)
    val |= I2C_CNFG_MULTI_MASTER_MODE;
    i2c_writel(i2c_dev, val, i2c_dev.hw.regs.cnfg);
    i2c_writel(i2c_dev, 0, i2c_dev.hw.regs.int_mask);
    if (IS_VI(i2c_dev))
    tegra_i2c_vi_init(i2c_dev);
// Disable fairness arbitration if not an MCTP controller
    if (i2c_dev.hw.has_fairarb_reg && !i2c_dev.is_mctp)
    i2c_writel(i2c_dev, 0, i2c_dev.hw.regs.fairness_arb);
    if (i2c_dev.hw.enable_hs_mode_support)
    max_bus_freq_hz = I2C_MAX_HIGH_SPEED_MODE_FREQ;
    else
    max_bus_freq_hz = I2C_MAX_FAST_MODE_PLUS_FREQ;
    if (WARN_ON(t.bus_freq_hz > max_bus_freq_hz))
    t.bus_freq_hz = max_bus_freq_hz;
    if (t.bus_freq_hz <= I2C_MAX_STANDARD_MODE_FREQ) {
    tlow = i2c_dev.hw.tlow_std_mode;
    thigh = i2c_dev.hw.thigh_std_mode;
    tsu_thd = i2c_dev.hw.setup_hold_time_std_mode;
    non_hs_mode = i2c_dev.hw.clk_divisor_std_mode;
    } else if (t.bus_freq_hz <= I2C_MAX_FAST_MODE_FREQ) {
    tlow = i2c_dev.hw.tlow_fast_mode;
    thigh = i2c_dev.hw.thigh_fast_mode;
    tsu_thd = i2c_dev.hw.setup_hold_time_fast_mode;
    non_hs_mode = i2c_dev.hw.clk_divisor_fast_mode;
    } else if (t.bus_freq_hz <= I2C_MAX_FAST_MODE_PLUS_FREQ) {
    tlow = i2c_dev.hw.tlow_fastplus_mode;
    thigh = i2c_dev.hw.thigh_fastplus_mode;
    tsu_thd = i2c_dev.hw.setup_hold_time_fastplus_mode;
    non_hs_mode = i2c_dev.hw.clk_divisor_fast_plus_mode;
    } else {
//
// When using HS mode, i.e. when the bus frequency is greater than fast plus mode,
// the non-hs timing registers will be used for sending the master code byte for
// transition to HS mode. Configure the non-hs timing registers for Fast Mode to
// send the master code byte at 400kHz.
//
    tlow = i2c_dev.hw.tlow_fast_mode;
    thigh = i2c_dev.hw.thigh_fast_mode;
    tsu_thd = i2c_dev.hw.setup_hold_time_fast_mode;
    non_hs_mode = i2c_dev.hw.clk_divisor_fast_mode;
    }
// make sure clock divisor programmed correctly
    clk_divisor = FIELD_PREP(I2C_CLK_DIVISOR_HSMODE,
    i2c_dev.hw.clk_divisor_hs_mode) |
    FIELD_PREP(I2C_CLK_DIVISOR_STD_FAST_MODE, non_hs_mode);
    i2c_writel(i2c_dev, clk_divisor, i2c_dev.hw.regs.clk_divisor);
    if (i2c_dev.hw.has_interface_timing_reg) {
    val = FIELD_PREP(I2C_INTERFACE_TIMING_THIGH, thigh) |
    FIELD_PREP(I2C_INTERFACE_TIMING_TLOW, tlow);
    i2c_writel(i2c_dev, val, i2c_dev.hw.regs.interface_timing_0);
    }
//
// Configure setup and hold times only when tsu_thd is non-zero.
// Otherwise, preserve the chip default values.
//
    if (i2c_dev.hw.has_interface_timing_reg && tsu_thd)
    i2c_writel(i2c_dev, tsu_thd, i2c_dev.hw.regs.interface_timing_1);
// Write HS mode registers. These will get used only for HS mode
    if (i2c_dev.hw.enable_hs_mode_support) {
    tlow = i2c_dev.hw.tlow_hs_mode;
    thigh = i2c_dev.hw.thigh_hs_mode;
    tsu_thd = i2c_dev.hw.setup_hold_time_hs_mode;
    val = FIELD_PREP(I2C_HS_INTERFACE_TIMING_THIGH, thigh) |
    FIELD_PREP(I2C_HS_INTERFACE_TIMING_TLOW, tlow);
    i2c_writel(i2c_dev, val, i2c_dev.hw.regs.hs_interface_timing_0);
    i2c_writel(i2c_dev, tsu_thd, i2c_dev.hw.regs.hs_interface_timing_1);
    }
    clk_multiplier = (tlow + thigh + 2) * (non_hs_mode + 1);
    err = clk_set_rate(i2c_dev.div_clk,
    t.bus_freq_hz * clk_multiplier);
    if (err) {
    dev_err(i2c_dev.dev, "failed to set div-clk rate: %d\n", err);
    return err;
    }
    if (!IS_DVC(i2c_dev) && !IS_VI(i2c_dev)) {
    let mut sl_cfg: u32 = i2c_readl(i2c_dev, i2c_dev.hw.regs.sl_cnfg);
    sl_cfg |= I2C_SL_CNFG_NACK | I2C_SL_CNFG_NEWSL;
    i2c_writel(i2c_dev, sl_cfg, i2c_dev.hw.regs.sl_cnfg);
    i2c_writel(i2c_dev, 0xfc, i2c_dev.hw.regs.sl_addr1);
    i2c_writel(i2c_dev, 0x00, i2c_dev.hw.regs.sl_addr2);
    }
    err = tegra_i2c_flush_fifos(i2c_dev);
    if (err)
    return err;
    if (i2c_dev.multimaster_mode && i2c_dev.hw.has_slcg_override_reg)
    i2c_writel(i2c_dev, I2C_MST_CORE_CLKEN_OVR, i2c_dev.hw.regs.clken_override);
    err = tegra_i2c_wait_for_config_load(i2c_dev);
    if (err)
    return err;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_i2c_disable_packet_mode(i2c_dev: *mut tegra_i2c_dev) -> c_int {
    static int tegra_i2c_disable_packet_mode(struct tegra_i2c_dev *i2c_dev)
    {
    u32 cnfg;
//
// NACK interrupt is generated before the I2C controller generates
// the STOP condition on the bus.  So, wait for 2 clock periods
// before disabling the controller so that the STOP condition has
// been delivered properly.
//
    udelay(DIV_ROUND_UP(2 * 1000000, i2c_dev.timings.bus_freq_hz));
    cnfg = i2c_readl(i2c_dev, i2c_dev.hw.regs.cnfg);
    if (cnfg & I2C_CNFG_PACKET_MODE_EN)
    i2c_writel(i2c_dev, cnfg & ~I2C_CNFG_PACKET_MODE_EN, i2c_dev.hw.regs.cnfg);
    return tegra_i2c_wait_for_config_load(i2c_dev);
    }
#[no_mangle]
unsafe extern "C" fn tegra_i2c_empty_rx_fifo(i2c_dev: *mut tegra_i2c_dev) -> c_int {
    static int tegra_i2c_empty_rx_fifo(struct tegra_i2c_dev *i2c_dev)
    {
    let mut buf_remaining: usize = i2c_dev.msg_buf_remaining;
    unsigned int words_to_transfer, rx_fifo_avail;
    u8 *buf = i2c_dev.msg_buf;
    u32 val;
//
// Catch overflow due to message fully sent before the check for
// RX FIFO availability.
//
    if (WARN_ON_ONCE(!(i2c_dev.msg_buf_remaining)))
    return -EINVAL;
    if (i2c_dev.hw.has_mst_fifo) {
    val = i2c_readl(i2c_dev, i2c_dev.hw.regs.mst_fifo_status);
    rx_fifo_avail = FIELD_GET(I2C_MST_FIFO_STATUS_RX, val);
    } else {
    val = i2c_readl(i2c_dev, i2c_dev.hw.regs.fifo_status);
    rx_fifo_avail = FIELD_GET(I2C_FIFO_STATUS_RX, val);
    }
// round down to exclude partial word at the end of buffer
    words_to_transfer = buf_remaining / BYTES_PER_FIFO_WORD;
    if (words_to_transfer > rx_fifo_avail)
    words_to_transfer = rx_fifo_avail;
    i2c_readsl(i2c_dev, buf, i2c_dev.hw.regs.rx_fifo, words_to_transfer);
    buf += words_to_transfer * BYTES_PER_FIFO_WORD;
    buf_remaining -= words_to_transfer * BYTES_PER_FIFO_WORD;
    rx_fifo_avail -= words_to_transfer;
//
// If there is a partial word at the end of buffer, handle it
// manually to prevent overwriting past the end of buffer.
//
    if (rx_fifo_avail > 0 && buf_remaining > 0) {
//
// buf_remaining > 3 check not needed as rx_fifo_avail == 0
// when (words_to_transfer was > rx_fifo_avail) earlier
// in this function.
//
    val = i2c_readl(i2c_dev, i2c_dev.hw.regs.rx_fifo);
    val = cpu_to_le32(val);
    memcpy(buf, &val, buf_remaining);
    buf_remaining = 0;
    rx_fifo_avail--;
    }
// RX FIFO must be drained, otherwise it's an Overflow case.
    if (WARN_ON_ONCE(rx_fifo_avail))
    return -EINVAL;
    i2c_dev.msg_buf_remaining = buf_remaining;
    i2c_dev.msg_buf = buf;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_i2c_fill_tx_fifo(i2c_dev: *mut tegra_i2c_dev) -> c_int {
    static int tegra_i2c_fill_tx_fifo(struct tegra_i2c_dev *i2c_dev)
    {
    let mut buf_remaining: usize = i2c_dev.msg_buf_remaining;
    unsigned int words_to_transfer, tx_fifo_avail;
    u8 *buf = i2c_dev.msg_buf;
    u32 val;
    if (i2c_dev.hw.has_mst_fifo) {
    val = i2c_readl(i2c_dev, i2c_dev.hw.regs.mst_fifo_status);
    tx_fifo_avail = FIELD_GET(I2C_MST_FIFO_STATUS_TX, val);
    } else {
    val = i2c_readl(i2c_dev, i2c_dev.hw.regs.fifo_status);
    tx_fifo_avail = FIELD_GET(I2C_FIFO_STATUS_TX, val);
    }
// round down to exclude partial word at the end of buffer
    words_to_transfer = buf_remaining / BYTES_PER_FIFO_WORD;
//
// This hunk pushes 4 bytes at a time into the TX FIFO.
//
// It's very common to have < 4 bytes, hence there is no word
// to push if we have less than 4 bytes to transfer.
//
    if (words_to_transfer) {
    if (words_to_transfer > tx_fifo_avail)
    words_to_transfer = tx_fifo_avail;
//
// Update state before writing to FIFO.  Note that this may
// cause us to finish writing all bytes (AKA buf_remaining
// goes to 0), hence we have a potential for an interrupt
// (PACKET_XFER_COMPLETE is not maskable), but GIC interrupt
// is disabled at this point.
//
    buf_remaining -= words_to_transfer * BYTES_PER_FIFO_WORD;
    tx_fifo_avail -= words_to_transfer;
    i2c_dev.msg_buf_remaining = buf_remaining;
    i2c_dev.msg_buf = buf + words_to_transfer * BYTES_PER_FIFO_WORD;
    if (IS_VI(i2c_dev))
    i2c_writesl_vi(i2c_dev, buf, i2c_dev.hw.regs.tx_fifo, words_to_transfer);
    else
    i2c_writesl(i2c_dev, buf, i2c_dev.hw.regs.tx_fifo, words_to_transfer);
    buf += words_to_transfer * BYTES_PER_FIFO_WORD;
    }
//
// If there is a partial word at the end of buffer, handle it manually
// to prevent reading past the end of buffer, which could cross a page
// boundary and fault.
//
    if (tx_fifo_avail > 0 && buf_remaining > 0) {
//
// buf_remaining > 3 check not needed as tx_fifo_avail == 0
// when (words_to_transfer was > tx_fifo_avail) earlier
// in this function for non-zero words_to_transfer.
//
    memcpy(&val, buf, buf_remaining);
    val = le32_to_cpu(val);
    i2c_dev.msg_buf_remaining = 0;
    i2c_dev.msg_buf = core::ptr::null_mut();
    i2c_writel(i2c_dev, val, i2c_dev.hw.regs.tx_fifo);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_i2c_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t tegra_i2c_isr(int irq, void *dev_id)
    {
    let mut status_err: u32 = I2C_INT_NO_ACK | I2C_INT_ARBITRATION_LOST;
    struct tegra_i2c_dev *i2c_dev = dev_id;
    u32 status;
    status = i2c_readl(i2c_dev, i2c_dev.hw.regs.int_status);
    if (status == 0) {
    dev_warn(i2c_dev.dev, "IRQ status 0 %08x %08x %08x\n",
    i2c_readl(i2c_dev, i2c_dev.hw.regs.packet_transfer_status),
    i2c_readl(i2c_dev, i2c_dev.hw.regs.status),
    i2c_readl(i2c_dev, i2c_dev.hw.regs.cnfg));
    i2c_dev.msg_err |= I2C_ERR_UNKNOWN_INTERRUPT;
    goto err;
    }
    if (status & status_err) {
    tegra_i2c_disable_packet_mode(i2c_dev);
    if (status & I2C_INT_NO_ACK)
    i2c_dev.msg_err |= I2C_ERR_NO_ACK;
    if (status & I2C_INT_ARBITRATION_LOST)
    i2c_dev.msg_err |= I2C_ERR_ARBITRATION_LOST;
    goto err;
    }
//
// I2C transfer is terminated during the bus clear, so skip
// processing the other interrupts.
//
    if (i2c_dev.hw.supports_bus_clear && (status & I2C_INT_BUS_CLR_DONE))
    goto err;
    if (!i2c_dev.dma_mode) {
    if (i2c_dev.msg_read && (status & I2C_INT_RX_FIFO_DATA_REQ)) {
    if (tegra_i2c_empty_rx_fifo(i2c_dev)) {
//
// Overflow error condition: message fully sent,
// with no XFER_COMPLETE interrupt but hardware
// asks to transfer more.
//
    i2c_dev.msg_err |= I2C_ERR_RX_BUFFER_OVERFLOW;
    goto err;
    }
    }
    if (!i2c_dev.msg_read && (status & I2C_INT_TX_FIFO_DATA_REQ)) {
    if (i2c_dev.msg_buf_remaining)
    tegra_i2c_fill_tx_fifo(i2c_dev);
    else
    tegra_i2c_mask_irq(i2c_dev,
    I2C_INT_TX_FIFO_DATA_REQ);
    }
    }
    i2c_writel(i2c_dev, status, i2c_dev.hw.regs.int_status);
    if (IS_DVC(i2c_dev))
    dvc_writel(i2c_dev, DVC_STATUS_I2C_DONE_INTR, DVC_STATUS);
//
// During message read XFER_COMPLETE interrupt is triggered prior to
// DMA completion and during message write XFER_COMPLETE interrupt is
// triggered after DMA completion.
//
// PACKETS_XFER_COMPLETE indicates completion of all bytes of transfer,
// so forcing msg_buf_remaining to 0 in DMA mode.
//
    if (status & I2C_INT_PACKET_XFER_COMPLETE) {
    if (i2c_dev.dma_mode)
    i2c_dev.msg_buf_remaining = 0;
//
// Underflow error condition: XFER_COMPLETE before message
// fully sent.
//
    if (WARN_ON_ONCE(i2c_dev.msg_buf_remaining)) {
    i2c_dev.msg_err |= I2C_ERR_UNKNOWN_INTERRUPT;
    goto err;
    }
    complete(&i2c_dev.msg_complete);
    }
    goto done;
    err:
// mask all interrupts on error
    tegra_i2c_mask_irq(i2c_dev,
    I2C_INT_NO_ACK |
    I2C_INT_ARBITRATION_LOST |
    I2C_INT_PACKET_XFER_COMPLETE |
    I2C_INT_TX_FIFO_DATA_REQ |
    I2C_INT_RX_FIFO_DATA_REQ);
    if (i2c_dev.hw.supports_bus_clear)
    tegra_i2c_mask_irq(i2c_dev, I2C_INT_BUS_CLR_DONE);
    i2c_writel(i2c_dev, status, i2c_dev.hw.regs.int_status);
    if (IS_DVC(i2c_dev))
    dvc_writel(i2c_dev, DVC_STATUS_I2C_DONE_INTR, DVC_STATUS);
    if (i2c_dev.dma_mode) {
    dmaengine_terminate_async(i2c_dev.dma_chan);
    complete(&i2c_dev.dma_complete);
    }
    complete(&i2c_dev.msg_complete);
    done:
    return IRQ_HANDLED;
    }
    static void tegra_i2c_config_fifo_trig(struct tegra_i2c_dev *i2c_dev,
    size_t len)
    {
    let mut slv_config: dma_slave_config = {0};
    u32 val, reg, dma_burst, reg_offset;
    int err;
    if (i2c_dev.hw.has_mst_fifo)
    reg = i2c_dev.hw.regs.mst_fifo_control;
    else
    reg = i2c_dev.hw.regs.fifo_control;
    if (i2c_dev.dma_mode) {
    if (len & 0xF)
    dma_burst = 1;
#[no_mangle]
pub unsafe extern "C" fn if(0x10: len &) -> else {
    else if (len & 0x10)
    dma_burst = 4;
    else
    dma_burst = 8;
    if (i2c_dev.msg_read) {
    reg_offset = i2c_dev.hw.regs.rx_fifo;
    slv_config.src_addr = i2c_dev.base_phys + reg_offset;
    slv_config.src_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    slv_config.src_maxburst = dma_burst;
    if (i2c_dev.hw.has_mst_fifo)
    val = I2C_MST_FIFO_CONTROL_RX_TRIG(dma_burst);
    else
    val = I2C_FIFO_CONTROL_RX_TRIG(dma_burst);
    } else {
    reg_offset = i2c_dev.hw.regs.tx_fifo;
    slv_config.dst_addr = i2c_dev.base_phys + reg_offset;
    slv_config.dst_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    slv_config.dst_maxburst = dma_burst;
    if (i2c_dev.hw.has_mst_fifo)
    val = I2C_MST_FIFO_CONTROL_TX_TRIG(dma_burst);
    else
    val = I2C_FIFO_CONTROL_TX_TRIG(dma_burst);
    }
    slv_config.device_fc = true;
    err = dmaengine_slave_config(i2c_dev.dma_chan, &slv_config);
    if (err) {
    dev_err(i2c_dev.dev, "DMA config failed: %d\n", err);
    dev_err(i2c_dev.dev, "falling back to PIO\n");
    tegra_i2c_release_dma(i2c_dev);
    i2c_dev.dma_mode = false;
    } else {
    goto out;
    }
    }
    if (i2c_dev.hw.has_mst_fifo)
    val = I2C_MST_FIFO_CONTROL_TX_TRIG(8) |
    I2C_MST_FIFO_CONTROL_RX_TRIG(1);
    else
    val = I2C_FIFO_CONTROL_TX_TRIG(8) |
    I2C_FIFO_CONTROL_RX_TRIG(1);
    out:
    i2c_writel(i2c_dev, val, reg);
    }
    static unsigned long tegra_i2c_poll_completion(struct tegra_i2c_dev *i2c_dev,
    struct completion *complete,
    unsigned int timeout_ms)
    {
    let mut ktime: ktime_t = ktime_get();
    let mut ktimeout: ktime_t = ktime_add_ms(ktime, timeout_ms);
    do {
    let mut status: u32 = i2c_readl(i2c_dev, i2c_dev.hw.regs.int_status);
    if (status)
    tegra_i2c_isr(i2c_dev.irq, i2c_dev);
    if (completion_done(complete)) {
    let mut delta: i64 = ktime_ms_delta(ktimeout, ktime);
    return msecs_to_jiffies(delta) ?: 1;
    }
    ktime = ktime_get();
    } while (ktime_before(ktime, ktimeout));
    return 0;
    }
    static unsigned long tegra_i2c_wait_completion(struct tegra_i2c_dev *i2c_dev,
    struct completion *complete,
    unsigned int timeout_ms)
    {
    unsigned long ret;
    if (i2c_dev.atomic_mode) {
    ret = tegra_i2c_poll_completion(i2c_dev, complete, timeout_ms);
    } else {
    enable_irq(i2c_dev.irq);
    ret = wait_for_completion_timeout(complete,
    msecs_to_jiffies(timeout_ms));
    disable_irq(i2c_dev.irq);
//
// Under some rare circumstances (like running KASAN +
// NFS root) CPU, which handles interrupt, may stuck in
// uninterruptible state for a significant time.  In this
// case we will get timeout if I2C transfer is running on
// a sibling CPU, despite of IRQ being raised.
//
// In order to handle this rare condition, the IRQ status
// needs to be checked after timeout.
//
    if (ret == 0)
    ret = tegra_i2c_poll_completion(i2c_dev, complete, 0);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tegra_i2c_issue_bus_clear(adap: *mut i2c_adapter) -> c_int {
    static int tegra_i2c_issue_bus_clear(struct i2c_adapter *adap)
    {
    struct tegra_i2c_dev *i2c_dev = i2c_get_adapdata(adap);
    u32 val, time_left;
    int err;
    reinit_completion(&i2c_dev.msg_complete);
    val = FIELD_PREP(I2C_BC_SCLK_THRESHOLD, 9) | I2C_BC_STOP_COND |
    I2C_BC_TERMINATE;
    i2c_writel(i2c_dev, val, i2c_dev.hw.regs.bus_clear_cnfg);
    err = tegra_i2c_wait_for_config_load(i2c_dev);
    if (err)
    return err;
    val |= I2C_BC_ENABLE;
    i2c_writel(i2c_dev, val, i2c_dev.hw.regs.bus_clear_cnfg);
    tegra_i2c_unmask_irq(i2c_dev, I2C_INT_BUS_CLR_DONE);
    time_left = tegra_i2c_wait_completion(i2c_dev, &i2c_dev.msg_complete, 50);
    tegra_i2c_mask_irq(i2c_dev, I2C_INT_BUS_CLR_DONE);
    if (time_left == 0) {
    dev_err(i2c_dev.dev, "failed to clear bus\n");
    return -ETIMEDOUT;
    }
    val = i2c_readl(i2c_dev, i2c_dev.hw.regs.bus_clear_status);
    if (!(val & I2C_BC_STATUS)) {
    dev_err(i2c_dev.dev, "un-recovered arbitration lost\n");
    return -EIO;
    }
    return -EAGAIN;
    }
    static void tegra_i2c_push_packet_header(struct tegra_i2c_dev *i2c_dev,
    struct i2c_msg *msg,
    enum msg_end_type end_state)
    {
    u32 *dma_buf = i2c_dev.dma_buf;
    u32 packet_header;
    packet_header = FIELD_PREP(PACKET_HEADER0_HEADER_SIZE, 0) |
    FIELD_PREP(PACKET_HEADER0_PROTOCOL,
    PACKET_HEADER0_PROTOCOL_I2C) |
    FIELD_PREP(PACKET_HEADER0_CONT_ID, i2c_dev.cont_id) |
    FIELD_PREP(PACKET_HEADER0_PACKET_ID, 1);
    if (i2c_dev.dma_mode && !i2c_dev.msg_read)
// dma_buf++ = packet_header;
    else
    i2c_writel(i2c_dev, packet_header, i2c_dev.hw.regs.tx_fifo);
    packet_header = i2c_dev.msg_len - 1;
    if (i2c_dev.dma_mode && !i2c_dev.msg_read)
// dma_buf++ = packet_header;
    else
    i2c_writel(i2c_dev, packet_header, i2c_dev.hw.regs.tx_fifo);
    packet_header = I2C_HEADER_IE_ENABLE;
    if (end_state == MSG_END_CONTINUE)
    packet_header |= I2C_HEADER_CONTINUE_XFER;
#[no_mangle]
pub unsafe extern "C" fn if(MSG_END_REPEAT_START: end_state ==) -> else {
    else if (end_state == MSG_END_REPEAT_START)
    packet_header |= I2C_HEADER_REPEAT_START;
    if (msg.flags & I2C_M_TEN) {
    packet_header |= msg.addr;
    packet_header |= I2C_HEADER_10BIT_ADDR;
    } else {
    packet_header |= msg.addr << I2C_HEADER_SLAVE_ADDR_SHIFT;
    }
    if (msg.flags & I2C_M_IGNORE_NAK)
    packet_header |= I2C_HEADER_CONT_ON_NAK;
    if (msg.flags & I2C_M_RD)
    packet_header |= I2C_HEADER_READ;
    if (i2c_dev.timings.bus_freq_hz > I2C_MAX_FAST_MODE_PLUS_FREQ)
    packet_header |= I2C_HEADER_HS_MODE;
    if (i2c_dev.dma_mode && !i2c_dev.msg_read)
// dma_buf++ = packet_header;
    else
    i2c_writel(i2c_dev, packet_header, i2c_dev.hw.regs.tx_fifo);
    }
    static int tegra_i2c_error_recover(struct tegra_i2c_dev *i2c_dev,
    struct i2c_msg *msg)
    {
    if (i2c_dev.msg_err == I2C_ERR_NONE)
    return 0;
    tegra_i2c_init(i2c_dev);
// start recovery upon arbitration loss in single master mode
    if (i2c_dev.msg_err == I2C_ERR_ARBITRATION_LOST) {
    if (!i2c_dev.multimaster_mode)
    return i2c_recover_bus(&i2c_dev.adapter);
    return -EAGAIN;
    }
    if (i2c_dev.msg_err == I2C_ERR_NO_ACK) {
    if (msg.flags & I2C_M_IGNORE_NAK)
    return 0;
    return -EREMOTEIO;
    }
    return -EIO;
    }
    static int tegra_i2c_xfer_msg(struct tegra_i2c_dev *i2c_dev,
    struct i2c_msg *msg,
    enum msg_end_type end_state)
    {
    unsigned long time_left, xfer_time = 100;
    size_t xfer_size;
    u32 int_mask;
    int err;
    err = tegra_i2c_flush_fifos(i2c_dev);
    if (err)
    return err;
    i2c_dev.msg_buf = msg.buf;
    i2c_dev.msg_len = msg.len;
    i2c_dev.msg_err = I2C_ERR_NONE;
    i2c_dev.msg_read = !!(msg.flags & I2C_M_RD);
    reinit_completion(&i2c_dev.msg_complete);
//
// For SMBUS block read command, read only 1 byte in the first transfer.
// Adjust that 1 byte for the next transfer in the msg buffer and msg
// length.
//
    if (msg.flags & I2C_M_RECV_LEN) {
    if (end_state == MSG_END_CONTINUE) {
    i2c_dev.msg_len = 1;
    } else {
    i2c_dev.msg_buf += 1;
    i2c_dev.msg_len -= 1;
    }
    }
    i2c_dev.msg_buf_remaining = i2c_dev.msg_len;
    if (i2c_dev.msg_read)
    xfer_size = i2c_dev.msg_len;
    else
    xfer_size = i2c_dev.msg_len + I2C_PACKET_HEADER_SIZE;
    xfer_size = ALIGN(xfer_size, BYTES_PER_FIFO_WORD);
    i2c_dev.dma_mode = xfer_size > I2C_PIO_MODE_PREFERRED_LEN &&
    i2c_dev.dma_buf && !i2c_dev.atomic_mode;
    tegra_i2c_config_fifo_trig(i2c_dev, xfer_size);
//
// Transfer time in mSec = Total bits / transfer rate
// Total bits = 9 bits per byte (including ACK bit) + Start & stop bits
//
    xfer_time += DIV_ROUND_CLOSEST(((xfer_size * 9) + 2) * MSEC_PER_SEC,
    i2c_dev.timings.bus_freq_hz);
    int_mask = I2C_INT_NO_ACK | I2C_INT_ARBITRATION_LOST;
    tegra_i2c_unmask_irq(i2c_dev, int_mask);
    if (i2c_dev.dma_mode) {
    if (i2c_dev.msg_read) {
    err = tegra_i2c_dma_submit(i2c_dev, xfer_size);
    if (err)
    return err;
    }
    }
    tegra_i2c_push_packet_header(i2c_dev, msg, end_state);
    if (!i2c_dev.msg_read) {
    if (i2c_dev.dma_mode) {
    memcpy(i2c_dev.dma_buf + I2C_PACKET_HEADER_SIZE,
    msg.buf, i2c_dev.msg_len);
    err = tegra_i2c_dma_submit(i2c_dev, xfer_size);
    if (err)
    return err;
    } else {
    tegra_i2c_fill_tx_fifo(i2c_dev);
    }
    }
    if (i2c_dev.hw.has_per_pkt_xfer_complete_irq)
    int_mask |= I2C_INT_PACKET_XFER_COMPLETE;
    if (!i2c_dev.dma_mode) {
    if (msg.flags & I2C_M_RD)
    int_mask |= I2C_INT_RX_FIFO_DATA_REQ;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: i2c_dev->msg_buf_remaining) -> else {
    else if (i2c_dev.msg_buf_remaining)
    int_mask |= I2C_INT_TX_FIFO_DATA_REQ;
    }
    tegra_i2c_unmask_irq(i2c_dev, int_mask);
    dev_dbg(i2c_dev.dev, "unmasked IRQ: %02x\n",
    i2c_readl(i2c_dev, i2c_dev.hw.regs.int_mask));
    if (i2c_dev.dma_mode) {
    time_left = tegra_i2c_wait_completion(i2c_dev,
    &i2c_dev.dma_complete,
    xfer_time);
//
// Synchronize DMA first, since dmaengine_terminate_sync()
// performs synchronization after the transfer's termination
// and we want to get a completion if transfer succeeded.
//
    dmaengine_synchronize(i2c_dev.dma_chan);
    dmaengine_terminate_sync(i2c_dev.dma_chan);
    if (!time_left && !completion_done(&i2c_dev.dma_complete)) {
    tegra_i2c_init(i2c_dev);
    return -ETIMEDOUT;
    }
    if (i2c_dev.msg_read && i2c_dev.msg_err == I2C_ERR_NONE)
    memcpy(i2c_dev.msg_buf, i2c_dev.dma_buf, i2c_dev.msg_len);
    }
    time_left = tegra_i2c_wait_completion(i2c_dev, &i2c_dev.msg_complete,
    xfer_time);
    tegra_i2c_mask_irq(i2c_dev, int_mask);
    if (time_left == 0) {
    tegra_i2c_init(i2c_dev);
    return -ETIMEDOUT;
    }
    dev_dbg(i2c_dev.dev, "transfer complete: %lu %d %d\n",
    time_left, completion_done(&i2c_dev.msg_complete),
    i2c_dev.msg_err);
    i2c_dev.dma_mode = false;
    err = tegra_i2c_error_recover(i2c_dev, msg);
    if (err)
    return err;
    return 0;
    }
    static int tegra_i2c_xfer(struct i2c_adapter *adap, struct i2c_msg msgs[],
    int num)
    {
    struct tegra_i2c_dev *i2c_dev = i2c_get_adapdata(adap);
    int i, ret;
    ret = pm_runtime_get_sync(i2c_dev.dev);
    if (ret < 0) {
    dev_err(i2c_dev.dev, "runtime resume failed %d\n", ret);
    pm_runtime_put_noidle(i2c_dev.dev);
    return ret;
    }
    ret = tegra_i2c_mutex_lock(i2c_dev);
    if (ret) {
    pm_runtime_put(i2c_dev.dev);
    return ret;
    }
    for (i = 0; i < num; i++) {
    let mut end_type: enum msg_end_type = MSG_END_STOP;
    if (i < (num - 1)) {
// check whether follow up message is coming
    if (msgs[i + 1].flags & I2C_M_NOSTART)
    end_type = MSG_END_CONTINUE;
    else
    end_type = MSG_END_REPEAT_START;
    }
// If M_RECV_LEN use ContinueXfer to read the first byte
    if (msgs[i].flags & I2C_M_RECV_LEN) {
    ret = tegra_i2c_xfer_msg(i2c_dev, &msgs[i], MSG_END_CONTINUE);
    if (ret)
    break;
// Validate message length before proceeding
    if (msgs[i].buf[0] == 0 || msgs[i].buf[0] > I2C_SMBUS_BLOCK_MAX)
    break;
// Set the msg length from first byte
    msgs[i].len += msgs[i].buf[0];
    dev_dbg(i2c_dev.dev, "reading %d bytes\n", msgs[i].len);
    }
    ret = tegra_i2c_xfer_msg(i2c_dev, &msgs[i], end_type);
    if (ret)
    break;
    }
    tegra_i2c_mutex_unlock(i2c_dev);
    pm_runtime_put(i2c_dev.dev);
    return ret ?: i;
    }
    static int tegra_i2c_xfer_atomic(struct i2c_adapter *adap,
    struct i2c_msg msgs[], int num)
    {
    struct tegra_i2c_dev *i2c_dev = i2c_get_adapdata(adap);
    int ret;
    i2c_dev.atomic_mode = true;
    ret = tegra_i2c_xfer(adap, msgs, num);
    i2c_dev.atomic_mode = false;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tegra_i2c_func(adap: *mut i2c_adapter) -> u32 {
    static u32 tegra_i2c_func(struct i2c_adapter *adap)
    {
    struct tegra_i2c_dev *i2c_dev = i2c_get_adapdata(adap);
    u32 ret = I2C_FUNC_I2C | (I2C_FUNC_SMBUS_EMUL & ~I2C_FUNC_SMBUS_QUICK) |
    I2C_FUNC_10BIT_ADDR | I2C_FUNC_PROTOCOL_MANGLING;
    if (i2c_dev.hw.has_continue_xfer_support)
    ret |= I2C_FUNC_NOSTART | I2C_FUNC_SMBUS_READ_BLOCK_DATA;
    return ret;
    }
    static const struct i2c_algorithm tegra_i2c_algo = {
    .xfer = tegra_i2c_xfer,
    .xfer_atomic = tegra_i2c_xfer_atomic,
    .functionality = tegra_i2c_func,
    };
// payload size is only 12 bit
    static const struct i2c_adapter_quirks tegra_i2c_quirks = {
    .flags = I2C_AQ_NO_ZERO_LEN,
    .max_read_len = SZ_4K,
    .max_write_len = SZ_4K - I2C_PACKET_HEADER_SIZE,
    };
    static const struct i2c_adapter_quirks tegra194_i2c_quirks = {
    .flags = I2C_AQ_NO_ZERO_LEN,
    .max_write_len = SZ_64K - I2C_PACKET_HEADER_SIZE,
    };
    static struct i2c_bus_recovery_info tegra_i2c_recovery_info = {
    .recover_bus = tegra_i2c_issue_bus_clear,
    };
    static const struct tegra_i2c_hw_feature tegra20_i2c_hw = {
    .has_continue_xfer_support = false,
    .has_per_pkt_xfer_complete_irq = false,
    .clk_divisor_hs_mode = 3,
    .clk_divisor_std_mode = 0,
    .clk_divisor_fast_mode = 0,
    .clk_divisor_fast_plus_mode = 0,
    .has_config_load_reg = false,
    .has_multi_master_mode = false,
    .has_slcg_override_reg = false,
    .has_mst_fifo = false,
    .has_mst_reset = false,
    .quirks = &tegra_i2c_quirks,
    .supports_bus_clear = false,
    .has_apb_dma = true,
    .tlow_std_mode = 0x4,
    .thigh_std_mode = 0x2,
    .tlow_fast_mode = 0x4,
    .thigh_fast_mode = 0x2,
    .tlow_fastplus_mode = 0x4,
    .thigh_fastplus_mode = 0x2,
    .setup_hold_time_std_mode = 0x0,
    .setup_hold_time_fast_mode = 0x0,
    .setup_hold_time_fastplus_mode = 0x0,
    .setup_hold_time_hs_mode = 0x0,
    .has_interface_timing_reg = false,
    .enable_hs_mode_support = false,
    .has_mutex = false,
    .has_fairarb_reg = false,
    .variant = TEGRA_I2C_VARIANT_DEFAULT,
    .regs = &tegra20_i2c_regs,
    };

    static const struct tegra_i2c_hw_feature tegra20_dvc_i2c_hw = {
    .has_continue_xfer_support = false,
    .has_per_pkt_xfer_complete_irq = false,
    .clk_divisor_hs_mode = 3,
    .clk_divisor_std_mode = 0,
    .clk_divisor_fast_mode = 0,
    .clk_divisor_fast_plus_mode = 0,
    .has_config_load_reg = false,
    .has_multi_master_mode = false,
    .has_slcg_override_reg = false,
    .has_mst_fifo = false,
    .has_mst_reset = false,
    .quirks = &tegra_i2c_quirks,
    .supports_bus_clear = false,
    .has_apb_dma = true,
    .tlow_std_mode = 0x4,
    .thigh_std_mode = 0x2,
    .tlow_fast_mode = 0x4,
    .thigh_fast_mode = 0x2,
    .tlow_fastplus_mode = 0x4,
    .thigh_fastplus_mode = 0x2,
    .setup_hold_time_std_mode = 0x0,
    .setup_hold_time_fast_mode = 0x0,
    .setup_hold_time_fastplus_mode = 0x0,
    .setup_hold_time_hs_mode = 0x0,
    .has_interface_timing_reg = false,
    .enable_hs_mode_support = false,
    .has_mutex = false,
    .has_fairarb_reg = false,
    .variant = TEGRA_I2C_VARIANT_DVC,
    .regs = &tegra20_dvc_i2c_regs,
    };

    static const struct tegra_i2c_hw_feature tegra30_i2c_hw = {
    .has_continue_xfer_support = true,
    .has_per_pkt_xfer_complete_irq = false,
    .clk_divisor_hs_mode = 3,
    .clk_divisor_std_mode = 0,
    .clk_divisor_fast_mode = 0,
    .clk_divisor_fast_plus_mode = 0,
    .has_config_load_reg = false,
    .has_multi_master_mode = false,
    .has_slcg_override_reg = false,
    .has_mst_fifo = false,
    .has_mst_reset = false,
    .quirks = &tegra_i2c_quirks,
    .supports_bus_clear = false,
    .has_apb_dma = true,
    .tlow_std_mode = 0x4,
    .thigh_std_mode = 0x2,
    .tlow_fast_mode = 0x4,
    .thigh_fast_mode = 0x2,
    .tlow_fastplus_mode = 0x4,
    .thigh_fastplus_mode = 0x2,
    .setup_hold_time_std_mode = 0x0,
    .setup_hold_time_fast_mode = 0x0,
    .setup_hold_time_fastplus_mode = 0x0,
    .setup_hold_time_hs_mode = 0x0,
    .has_interface_timing_reg = false,
    .enable_hs_mode_support = false,
    .has_mutex = false,
    .has_fairarb_reg = false,
    .variant = TEGRA_I2C_VARIANT_DEFAULT,
    .regs = &tegra20_i2c_regs,
    };
    static const struct tegra_i2c_hw_feature tegra114_i2c_hw = {
    .has_continue_xfer_support = true,
    .has_per_pkt_xfer_complete_irq = true,
    .clk_divisor_hs_mode = 1,
    .clk_divisor_std_mode = 0x19,
    .clk_divisor_fast_mode = 0x19,
    .clk_divisor_fast_plus_mode = 0x10,
    .has_config_load_reg = false,
    .has_multi_master_mode = false,
    .has_slcg_override_reg = false,
    .has_mst_fifo = false,
    .has_mst_reset = false,
    .quirks = &tegra_i2c_quirks,
    .supports_bus_clear = true,
    .has_apb_dma = true,
    .tlow_std_mode = 0x4,
    .thigh_std_mode = 0x2,
    .tlow_fast_mode = 0x4,
    .thigh_fast_mode = 0x2,
    .tlow_fastplus_mode = 0x4,
    .thigh_fastplus_mode = 0x2,
    .setup_hold_time_std_mode = 0x0,
    .setup_hold_time_fast_mode = 0x0,
    .setup_hold_time_fastplus_mode = 0x0,
    .setup_hold_time_hs_mode = 0x0,
    .has_interface_timing_reg = false,
    .enable_hs_mode_support = false,
    .has_mutex = false,
    .has_fairarb_reg = false,
    .variant = TEGRA_I2C_VARIANT_DEFAULT,
    .regs = &tegra20_i2c_regs,
    };
    static const struct tegra_i2c_hw_feature tegra124_i2c_hw = {
    .has_continue_xfer_support = true,
    .has_per_pkt_xfer_complete_irq = true,
    .clk_divisor_hs_mode = 1,
    .clk_divisor_std_mode = 0x19,
    .clk_divisor_fast_mode = 0x19,
    .clk_divisor_fast_plus_mode = 0x10,
    .has_config_load_reg = true,
    .has_multi_master_mode = false,
    .has_slcg_override_reg = true,
    .has_mst_fifo = false,
    .has_mst_reset = false,
    .quirks = &tegra_i2c_quirks,
    .supports_bus_clear = true,
    .has_apb_dma = true,
    .tlow_std_mode = 0x4,
    .thigh_std_mode = 0x2,
    .tlow_fast_mode = 0x4,
    .thigh_fast_mode = 0x2,
    .tlow_fastplus_mode = 0x4,
    .thigh_fastplus_mode = 0x2,
    .setup_hold_time_std_mode = 0x0,
    .setup_hold_time_fast_mode = 0x0,
    .setup_hold_time_fastplus_mode = 0x0,
    .setup_hold_time_hs_mode = 0x0,
    .has_interface_timing_reg = true,
    .enable_hs_mode_support = false,
    .has_mutex = false,
    .has_fairarb_reg = false,
    .variant = TEGRA_I2C_VARIANT_DEFAULT,
    .regs = &tegra20_i2c_regs,
    };
    static const struct tegra_i2c_hw_feature tegra210_i2c_hw = {
    .has_continue_xfer_support = true,
    .has_per_pkt_xfer_complete_irq = true,
    .clk_divisor_hs_mode = 1,
    .clk_divisor_std_mode = 0x19,
    .clk_divisor_fast_mode = 0x19,
    .clk_divisor_fast_plus_mode = 0x10,
    .has_config_load_reg = true,
    .has_multi_master_mode = false,
    .has_slcg_override_reg = true,
    .has_mst_fifo = false,
    .has_mst_reset = false,
    .quirks = &tegra_i2c_quirks,
    .supports_bus_clear = true,
    .has_apb_dma = true,
    .tlow_std_mode = 0x4,
    .thigh_std_mode = 0x2,
    .tlow_fast_mode = 0x4,
    .thigh_fast_mode = 0x2,
    .tlow_fastplus_mode = 0x4,
    .thigh_fastplus_mode = 0x2,
    .setup_hold_time_std_mode = 0,
    .setup_hold_time_fast_mode = 0,
    .setup_hold_time_fastplus_mode = 0,
    .setup_hold_time_hs_mode = 0,
    .has_interface_timing_reg = true,
    .enable_hs_mode_support = false,
    .has_mutex = false,
    .has_fairarb_reg = false,
    .variant = TEGRA_I2C_VARIANT_DEFAULT,
    .regs = &tegra20_i2c_regs,
    };

    static const struct tegra_i2c_hw_feature tegra210_vi_i2c_hw = {
    .has_continue_xfer_support = true,
    .has_per_pkt_xfer_complete_irq = true,
    .clk_divisor_hs_mode = 1,
    .clk_divisor_std_mode = 0x19,
    .clk_divisor_fast_mode = 0x19,
    .clk_divisor_fast_plus_mode = 0x10,
    .has_config_load_reg = true,
    .has_multi_master_mode = false,
    .has_slcg_override_reg = true,
    .has_mst_fifo = false,
    .has_mst_reset = false,
    .quirks = &tegra_i2c_quirks,
    .supports_bus_clear = true,
    .has_apb_dma = true,
    .tlow_std_mode = 0x4,
    .thigh_std_mode = 0x2,
    .tlow_fast_mode = 0x4,
    .thigh_fast_mode = 0x2,
    .tlow_fastplus_mode = 0x4,
    .thigh_fastplus_mode = 0x2,
    .setup_hold_time_std_mode = 0,
    .setup_hold_time_fast_mode = 0,
    .setup_hold_time_fastplus_mode = 0,
    .setup_hold_time_hs_mode = 0,
    .has_interface_timing_reg = true,
    .enable_hs_mode_support = false,
    .has_mutex = false,
    .has_fairarb_reg = false,
    .variant = TEGRA_I2C_VARIANT_VI,
    .regs = &tegra210_vi_i2c_regs,
    };

    static const struct tegra_i2c_hw_feature tegra186_i2c_hw = {
    .has_continue_xfer_support = true,
    .has_per_pkt_xfer_complete_irq = true,
    .clk_divisor_hs_mode = 1,
    .clk_divisor_std_mode = 0x16,
    .clk_divisor_fast_mode = 0x19,
    .clk_divisor_fast_plus_mode = 0x10,
    .has_config_load_reg = true,
    .has_multi_master_mode = false,
    .has_slcg_override_reg = true,
    .has_mst_fifo = false,
    .has_mst_reset = false,
    .quirks = &tegra_i2c_quirks,
    .supports_bus_clear = true,
    .has_apb_dma = false,
    .tlow_std_mode = 0x4,
    .thigh_std_mode = 0x3,
    .tlow_fast_mode = 0x4,
    .thigh_fast_mode = 0x2,
    .tlow_fastplus_mode = 0x4,
    .thigh_fastplus_mode = 0x2,
    .setup_hold_time_std_mode = 0,
    .setup_hold_time_fast_mode = 0,
    .setup_hold_time_fastplus_mode = 0,
    .setup_hold_time_hs_mode = 0,
    .has_interface_timing_reg = true,
    .enable_hs_mode_support = false,
    .has_mutex = false,
    .has_fairarb_reg = false,
    .variant = TEGRA_I2C_VARIANT_DEFAULT,
    .regs = &tegra20_i2c_regs,
    };
    static const struct tegra_i2c_hw_feature tegra194_i2c_hw = {
    .has_continue_xfer_support = true,
    .has_per_pkt_xfer_complete_irq = true,
    .clk_divisor_hs_mode = 1,
    .clk_divisor_std_mode = 0x4f,
    .clk_divisor_fast_mode = 0x3c,
    .clk_divisor_fast_plus_mode = 0x16,
    .has_config_load_reg = true,
    .has_multi_master_mode = true,
    .has_slcg_override_reg = true,
    .has_mst_fifo = true,
    .has_mst_reset = true,
    .quirks = &tegra194_i2c_quirks,
    .supports_bus_clear = true,
    .has_apb_dma = false,
    .tlow_std_mode = 0x8,
    .thigh_std_mode = 0x7,
    .tlow_fast_mode = 0x2,
    .thigh_fast_mode = 0x2,
    .tlow_fastplus_mode = 0x2,
    .thigh_fastplus_mode = 0x2,
    .tlow_hs_mode = 0x8,
    .thigh_hs_mode = 0x3,
    .setup_hold_time_std_mode = 0x08080808,
    .setup_hold_time_fast_mode = 0x02020202,
    .setup_hold_time_fastplus_mode = 0x02020202,
    .setup_hold_time_hs_mode = 0x090909,
    .has_interface_timing_reg = true,
    .enable_hs_mode_support = true,
    .has_mutex = false,
    .has_fairarb_reg = false,
    .variant = TEGRA_I2C_VARIANT_DEFAULT,
    .regs = &tegra20_i2c_regs,
    };
    static const struct tegra_i2c_hw_feature tegra256_i2c_hw = {
    .has_continue_xfer_support = true,
    .has_per_pkt_xfer_complete_irq = true,
    .clk_divisor_hs_mode = 9,
    .clk_divisor_std_mode = 0x7a,
    .clk_divisor_fast_mode = 0x40,
    .clk_divisor_fast_plus_mode = 0x14,
    .has_config_load_reg = true,
    .has_multi_master_mode = true,
    .has_slcg_override_reg = true,
    .has_mst_fifo = true,
    .has_mst_reset = true,
    .quirks = &tegra194_i2c_quirks,
    .supports_bus_clear = true,
    .has_apb_dma = false,
    .tlow_std_mode = 0x8,
    .thigh_std_mode = 0x7,
    .tlow_fast_mode = 0x4,
    .thigh_fast_mode = 0x2,
    .tlow_fastplus_mode = 0x4,
    .thigh_fastplus_mode = 0x4,
    .tlow_hs_mode = 0x3,
    .thigh_hs_mode = 0x2,
    .setup_hold_time_std_mode = 0x08080808,
    .setup_hold_time_fast_mode = 0x04010101,
    .setup_hold_time_fastplus_mode = 0x04020202,
    .setup_hold_time_hs_mode = 0x030303,
    .has_interface_timing_reg = true,
    .enable_hs_mode_support = true,
    .has_mutex = true,
    .has_fairarb_reg = true,
    .variant = TEGRA_I2C_VARIANT_DEFAULT,
    .regs = &tegra264_i2c_regs,
    };
    static const struct tegra_i2c_hw_feature tegra264_i2c_hw = {
    .has_continue_xfer_support = true,
    .has_per_pkt_xfer_complete_irq = true,
    .clk_divisor_hs_mode = 1,
    .clk_divisor_std_mode = 0x1d,
    .clk_divisor_fast_mode = 0x15,
    .clk_divisor_fast_plus_mode = 0x8,
    .has_config_load_reg = true,
    .has_multi_master_mode = true,
    .has_slcg_override_reg = true,
    .has_mst_fifo = true,
    .has_mst_reset = true,
    .quirks = &tegra194_i2c_quirks,
    .supports_bus_clear = true,
    .has_apb_dma = false,
    .tlow_std_mode = 0x8,
    .thigh_std_mode = 0x7,
    .tlow_fast_mode = 0x2,
    .thigh_fast_mode = 0x2,
    .tlow_fastplus_mode = 0x2,
    .thigh_fastplus_mode = 0x2,
    .tlow_hs_mode = 0x4,
    .thigh_hs_mode = 0x2,
    .setup_hold_time_std_mode = 0x08080808,
    .setup_hold_time_fast_mode = 0x02020202,
    .setup_hold_time_fastplus_mode = 0x02020202,
    .setup_hold_time_hs_mode = 0x090909,
    .has_interface_timing_reg = true,
    .enable_hs_mode_support = true,
    .has_mutex = true,
    .has_fairarb_reg = true,
    .variant = TEGRA_I2C_VARIANT_DEFAULT,
    .regs = &tegra264_i2c_regs,
    };
    static const struct tegra_i2c_hw_feature tegra410_i2c_hw = {
    .has_continue_xfer_support = true,
    .has_per_pkt_xfer_complete_irq = true,
    .clk_divisor_hs_mode = 2,
    .clk_divisor_std_mode = 0x3f,
    .clk_divisor_fast_mode = 0x2f,
    .clk_divisor_fast_plus_mode = 0x11,
    .has_config_load_reg = true,
    .has_multi_master_mode = true,
    .has_slcg_override_reg = true,
    .has_mst_fifo = true,
    .has_mst_reset = true,
    .quirks = &tegra194_i2c_quirks,
    .supports_bus_clear = true,
    .has_apb_dma = false,
    .tlow_std_mode = 0x8,
    .thigh_std_mode = 0x7,
    .tlow_fast_mode = 0x2,
    .thigh_fast_mode = 0x2,
    .tlow_fastplus_mode = 0x2,
    .thigh_fastplus_mode = 0x2,
    .tlow_hs_mode = 0x5,
    .thigh_hs_mode = 0x2,
    .setup_hold_time_std_mode = 0x08080808,
    .setup_hold_time_fast_mode = 0x02020202,
    .setup_hold_time_fastplus_mode = 0x02020202,
    .setup_hold_time_hs_mode = 0x0b0b0b,
    .has_interface_timing_reg = true,
    .enable_hs_mode_support = true,
    .has_mutex = true,
    .has_fairarb_reg = true,
    .variant = TEGRA_I2C_VARIANT_DEFAULT,
    .regs = &tegra410_i2c_regs,
    };
    static const struct of_device_id tegra_i2c_of_match[] = {
    { .compatible = "nvidia,tegra264-i2c", .data = &tegra264_i2c_hw, },
    { .compatible = "nvidia,tegra256-i2c", .data = &tegra256_i2c_hw, },
    { .compatible = "nvidia,tegra194-i2c", .data = &tegra194_i2c_hw, },
    { .compatible = "nvidia,tegra186-i2c", .data = &tegra186_i2c_hw, },

    { .compatible = "nvidia,tegra210-i2c-vi", .data = &tegra210_vi_i2c_hw, },

    { .compatible = "nvidia,tegra210-i2c", .data = &tegra210_i2c_hw, },
    { .compatible = "nvidia,tegra124-i2c", .data = &tegra124_i2c_hw, },
    { .compatible = "nvidia,tegra114-i2c", .data = &tegra114_i2c_hw, },
    { .compatible = "nvidia,tegra30-i2c", .data = &tegra30_i2c_hw, },
    { .compatible = "nvidia,tegra20-i2c", .data = &tegra20_i2c_hw, },

    { .compatible = "nvidia,tegra20-i2c-dvc", .data = &tegra20_dvc_i2c_hw, },

    {},
    };
    MODULE_DEVICE_TABLE(of, tegra_i2c_of_match);
#[no_mangle]
unsafe extern "C" fn tegra_i2c_parse_dt(i2c_dev: *mut tegra_i2c_dev) {
    static void tegra_i2c_parse_dt(struct tegra_i2c_dev *i2c_dev)
    {
    bool multi_mode;
    i2c_parse_fw_timings(i2c_dev.dev, &i2c_dev.timings, true);
    multi_mode = device_property_read_bool(i2c_dev.dev, "multi-master");
    i2c_dev.multimaster_mode = multi_mode;
    i2c_dev.is_mctp = device_property_present(i2c_dev.dev, "mctp-controller");
    }
#[no_mangle]
unsafe extern "C" fn tegra_i2c_init_clocks(i2c_dev: *mut tegra_i2c_dev) -> c_int {
    static int tegra_i2c_init_clocks(struct tegra_i2c_dev *i2c_dev)
    {
    int err;
    if (ACPI_HANDLE(i2c_dev.dev))
    return 0;
    i2c_dev.clocks[i2c_dev.nclocks++].id = "div-clk";
    if (i2c_dev.hw == &tegra20_i2c_hw || i2c_dev.hw == &tegra30_i2c_hw)
    i2c_dev.clocks[i2c_dev.nclocks++].id = "fast-clk";
    if (IS_VI(i2c_dev))
    i2c_dev.clocks[i2c_dev.nclocks++].id = "slow";
    err = devm_clk_bulk_get(i2c_dev.dev, i2c_dev.nclocks,
    i2c_dev.clocks);
    if (err)
    return err;
    err = clk_bulk_prepare(i2c_dev.nclocks, i2c_dev.clocks);
    if (err)
    return err;
    i2c_dev.div_clk = i2c_dev.clocks[0].clk;
    if (!i2c_dev.multimaster_mode)
    return 0;
    err = clk_enable(i2c_dev.div_clk);
    if (err) {
    dev_err_probe(i2c_dev.dev, err, "failed to enable div-clk\n");
    goto unprepare_clocks;
    }
    return 0;
    unprepare_clocks:
    clk_bulk_unprepare(i2c_dev.nclocks, i2c_dev.clocks);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn tegra_i2c_release_clocks(i2c_dev: *mut tegra_i2c_dev) {
    static void tegra_i2c_release_clocks(struct tegra_i2c_dev *i2c_dev)
    {
    if (i2c_dev.multimaster_mode)
    clk_disable(i2c_dev.div_clk);
    clk_bulk_unprepare(i2c_dev.nclocks, i2c_dev.clocks);
    }
#[no_mangle]
unsafe extern "C" fn tegra_i2c_init_hardware(i2c_dev: *mut tegra_i2c_dev) -> c_int {
    static int tegra_i2c_init_hardware(struct tegra_i2c_dev *i2c_dev)
    {
    int ret;
    ret = pm_runtime_get_sync(i2c_dev.dev);
    if (ret < 0)
    dev_err_probe(i2c_dev.dev, ret, "runtime resume failed\n");
    else
    ret = tegra_i2c_init(i2c_dev);
    pm_runtime_put_sync(i2c_dev.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tegra_i2c_probe(pdev: *mut platform_device) -> c_int {
    static int tegra_i2c_probe(struct platform_device *pdev)
    {
    struct tegra_i2c_dev *i2c_dev;
    struct resource *res;
    int err;
    i2c_dev = devm_kzalloc(&pdev.dev, sizeof(*i2c_dev), GFP_KERNEL);
    if (!i2c_dev)
    return -ENOMEM;
    platform_set_drvdata(pdev, i2c_dev);
    init_completion(&i2c_dev.msg_complete);
    init_completion(&i2c_dev.dma_complete);
    i2c_dev.hw = device_get_match_data(&pdev.dev);
    i2c_dev.cont_id = pdev.id;
    i2c_dev.dev = &pdev.dev;
    i2c_dev.base = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(i2c_dev.base))
    return PTR_ERR(i2c_dev.base);
    i2c_dev.base_phys = res.start;
    err = platform_get_irq(pdev, 0);
    if (err < 0)
    return err;
    i2c_dev.irq = err;
// interrupt will be enabled during of transfer time
    irq_set_status_flags(i2c_dev.irq, IRQ_NOAUTOEN);
    err = devm_request_threaded_irq(i2c_dev.dev, i2c_dev.irq,
    core::ptr::null_mut(), tegra_i2c_isr,
    IRQF_NO_SUSPEND | IRQF_ONESHOT,
    dev_name(i2c_dev.dev), i2c_dev);
    if (err)
    return err;
    tegra_i2c_parse_dt(i2c_dev);
    err = tegra_i2c_init_clocks(i2c_dev);
    if (err)
    return err;
    err = tegra_i2c_init_dma(i2c_dev);
    if (err)
    goto release_clocks;
//
// VI I2C is in VE power domain which is not always ON and not
// IRQ-safe.  Thus, IRQ-safe device shouldn't be attached to a
// non IRQ-safe domain because this prevents powering off the power
// domain.
//
// VI I2C device shouldn't be marked as IRQ-safe because VI I2C won't
// be used for atomic transfers. ACPI device is not IRQ safe also.
//
// Devices with pinctrl states cannot be marked IRQ-safe as the pinctrl
// state transitions during runtime PM require mutexes.
//
    if (!IS_VI(i2c_dev) && !has_acpi_companion(i2c_dev.dev) && !i2c_dev.dev.pins)
    pm_runtime_irq_safe(i2c_dev.dev);
    pm_runtime_enable(i2c_dev.dev);
    err = tegra_i2c_init_hardware(i2c_dev);
    if (err)
    goto release_rpm;
    i2c_set_adapdata(&i2c_dev.adapter, i2c_dev);
    i2c_dev.adapter.dev.of_node = i2c_dev.dev.of_node;
    i2c_dev.adapter.dev.parent = i2c_dev.dev;
    i2c_dev.adapter.retries = 1;
    i2c_dev.adapter.timeout = 6 * HZ;
    i2c_dev.adapter.quirks = i2c_dev.hw.quirks;
    i2c_dev.adapter.owner = THIS_MODULE;
    i2c_dev.adapter.class = I2C_CLASS_DEPRECATED;
    i2c_dev.adapter.algo = &tegra_i2c_algo;
    i2c_dev.adapter.nr = pdev.id;
    ACPI_COMPANION_SET(&i2c_dev.adapter.dev, ACPI_COMPANION(&pdev.dev));
    if (i2c_dev.hw.supports_bus_clear)
    i2c_dev.adapter.bus_recovery_info = &tegra_i2c_recovery_info;
    strscpy(i2c_dev.adapter.name, dev_name(i2c_dev.dev),
    sizeof(i2c_dev.adapter.name));
    err = i2c_add_numbered_adapter(&i2c_dev.adapter);
    if (err)
    goto release_rpm;
    return 0;
    release_rpm:
    pm_runtime_disable(i2c_dev.dev);
    tegra_i2c_release_dma(i2c_dev);
    release_clocks:
    tegra_i2c_release_clocks(i2c_dev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn tegra_i2c_remove(pdev: *mut platform_device) {
    static void tegra_i2c_remove(struct platform_device *pdev)
    {
    struct tegra_i2c_dev *i2c_dev = platform_get_drvdata(pdev);
    i2c_del_adapter(&i2c_dev.adapter);
    pm_runtime_force_suspend(i2c_dev.dev);
    tegra_i2c_release_dma(i2c_dev);
    tegra_i2c_release_clocks(i2c_dev);
    }
#[no_mangle]
unsafe extern "C" fn tegra_i2c_runtime_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused tegra_i2c_runtime_resume(struct device *dev)
    {
    struct tegra_i2c_dev *i2c_dev = dev_get_drvdata(dev);
    int err;
    err = pinctrl_pm_select_default_state(dev);
    if (err)
    return err;
    err = clk_bulk_enable(i2c_dev.nclocks, i2c_dev.clocks);
    if (err)
    return err;
//
// VI I2C device is attached to VE power domain which goes through
// power ON/OFF during runtime PM resume/suspend, meaning that
// controller needs to be re-initialized after power ON.
//
    if (IS_VI(i2c_dev)) {
    err = tegra_i2c_init(i2c_dev);
    if (err)
    goto disable_clocks;
    }
    return 0;
    disable_clocks:
    clk_bulk_disable(i2c_dev.nclocks, i2c_dev.clocks);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn tegra_i2c_runtime_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused tegra_i2c_runtime_suspend(struct device *dev)
    {
    struct tegra_i2c_dev *i2c_dev = dev_get_drvdata(dev);
    clk_bulk_disable(i2c_dev.nclocks, i2c_dev.clocks);
    return pinctrl_pm_select_idle_state(dev);
    }
#[no_mangle]
unsafe extern "C" fn tegra_i2c_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused tegra_i2c_suspend(struct device *dev)
    {
//
// Bring the controller up and hold a usage count so it stays
// available until the noirq phase.
//
    return pm_runtime_resume_and_get(dev);
    }
#[no_mangle]
unsafe extern "C" fn tegra_i2c_suspend_noirq(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused tegra_i2c_suspend_noirq(struct device *dev)
    {
    struct tegra_i2c_dev *i2c_dev = dev_get_drvdata(dev);
    i2c_mark_adapter_suspended(&i2c_dev.adapter);
//
// Runtime PM is already disabled at this point, so invoke the
// runtime_suspend callback directly to put the controller down.
//
    return tegra_i2c_runtime_suspend(dev);
    }
#[no_mangle]
unsafe extern "C" fn tegra_i2c_resume_noirq(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused tegra_i2c_resume_noirq(struct device *dev)
    {
    struct tegra_i2c_dev *i2c_dev = dev_get_drvdata(dev);
    int err;
//
// Runtime PM is still disabled at this point, so invoke the
// runtime_resume callback directly to bring the controller back up
// before re-initializing the hardware. The adapter is then marked
// resumed so that consumers can issue transfers from their own
// resume_noirq() handlers and onwards.
//
    err = tegra_i2c_runtime_resume(dev);
    if (err)
    return err;
    err = tegra_i2c_init(i2c_dev);
    if (err)
    return err;
    i2c_mark_adapter_resumed(&i2c_dev.adapter);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_i2c_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused tegra_i2c_resume(struct device *dev)
    {
    pm_runtime_put(dev);
    return 0;
    }
    static const struct dev_pm_ops tegra_i2c_pm = {
    SET_SYSTEM_SLEEP_PM_OPS(tegra_i2c_suspend, tegra_i2c_resume)
    SET_NOIRQ_SYSTEM_SLEEP_PM_OPS(tegra_i2c_suspend_noirq,
    tegra_i2c_resume_noirq)
    SET_RUNTIME_PM_OPS(tegra_i2c_runtime_suspend, tegra_i2c_runtime_resume,
    core::ptr::null_mut())
    };
    static const struct acpi_device_id tegra_i2c_acpi_match[] = {
    {.id = "NVDA0101", .driver_data = (kernel_ulong_t)&tegra210_i2c_hw},
    {.id = "NVDA0201", .driver_data = (kernel_ulong_t)&tegra186_i2c_hw},
    {.id = "NVDA0301", .driver_data = (kernel_ulong_t)&tegra194_i2c_hw},
    {.id = "NVDA2017", .driver_data = (kernel_ulong_t)&tegra410_i2c_hw},
    { }
    };
    MODULE_DEVICE_TABLE(acpi, tegra_i2c_acpi_match);
    static struct platform_driver tegra_i2c_driver = {
    .probe = tegra_i2c_probe,
    .remove = tegra_i2c_remove,
    .driver = {
    .name = "tegra-i2c",
    .of_match_table = tegra_i2c_of_match,
    .acpi_match_table = tegra_i2c_acpi_match,
    .pm = &tegra_i2c_pm,
    },
    };
    module_platform_driver(tegra_i2c_driver);
    MODULE_DESCRIPTION("NVIDIA Tegra I2C Bus Controller driver");
    MODULE_AUTHOR("Colin Cross");
    MODULE_LICENSE("GPL v2");
