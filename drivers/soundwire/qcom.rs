//! Automatically rewritten from C to Rust
//! Source: drivers/soundwire/qcom.c
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
// Copyright (c) 2019, Linaro Limited

pub const SWRM_COMP_SW_RESET: c_uint = 0x008;
pub const SWRM_COMP_STATUS: c_uint = 0x014;
pub const SWRM_LINK_MANAGER_EE: c_uint = 0x018;
pub const SWRM_EE_CPU: c_int = 1;

pub const SWRM_VERSION_1_3_0: c_uint = 0x01030000;
pub const SWRM_VERSION_1_5_1: c_uint = 0x01050001;
pub const SWRM_VERSION_1_7_0: c_uint = 0x01070000;
pub const SWRM_VERSION_2_0_0: c_uint = 0x02000000;
pub const SWRM_VERSION_3_1_0: c_uint = 0x03010000;
pub const SWRM_COMP_HW_VERSION: c_uint = 0x00;
pub const SWRM_COMP_CFG_ADDR: c_uint = 0x04;

pub const SWRM_COMP_PARAMS: c_uint = 0x100;

pub const SWRM_COMP_MASTER_ID: c_uint = 0x104;
pub const SWRM_V1_3_INTERRUPT_STATUS: c_uint = 0x200;
pub const SWRM_V2_0_INTERRUPT_STATUS: c_uint = 0x5000;

pub const SWRM_INTERRUPT_MAX: c_int = 17;
pub const SWRM_V1_3_INTERRUPT_MASK_ADDR: c_uint = 0x204;
pub const SWRM_V1_3_INTERRUPT_CLEAR: c_uint = 0x208;
pub const SWRM_V2_0_INTERRUPT_CLEAR: c_uint = 0x5008;
pub const SWRM_V1_3_INTERRUPT_CPU_EN: c_uint = 0x210;
pub const SWRM_V2_0_INTERRUPT_CPU_EN: c_uint = 0x5004;
pub const SWRM_V1_3_CMD_FIFO_WR_CMD: c_uint = 0x300;
pub const SWRM_V2_0_CMD_FIFO_WR_CMD: c_uint = 0x5020;
pub const SWRM_V1_3_CMD_FIFO_RD_CMD: c_uint = 0x304;
pub const SWRM_V2_0_CMD_FIFO_RD_CMD: c_uint = 0x5024;
pub const SWRM_CMD_FIFO_CMD: c_uint = 0x308;
pub const SWRM_CMD_FIFO_FLUSH: c_uint = 0x1;
pub const SWRM_V1_3_CMD_FIFO_STATUS: c_uint = 0x30C;
pub const SWRM_V2_0_CMD_FIFO_STATUS: c_uint = 0x5050;

pub const SWRM_CMD_FIFO_CFG_ADDR: c_uint = 0x314;

pub const SWRM_RD_WR_CMD_RETRIES: c_uint = 0x7;
pub const SWRM_V1_3_CMD_FIFO_RD_FIFO_ADDR: c_uint = 0x318;
pub const SWRM_V2_0_CMD_FIFO_RD_FIFO_ADDR: c_uint = 0x5040;

pub const SWRM_ENUMERATOR_CFG_ADDR: c_uint = 0x500;

pub const SWRM_MCP_BUS_CTRL: c_uint = 0x1044;

pub const SWRM_MCP_CFG_ADDR: c_uint = 0x1048;

pub const SWRM_DEF_CMD_NO_PINGS: c_uint = 0x1f;
pub const SWRM_MCP_STATUS: c_uint = 0x104C;

pub const SWRM_MCP_SLV_STATUS: c_uint = 0x1090;

pub const SWRM_MCP_SLV_STATUS_SZ: c_int = 2;

pub const SWR_V1_3_MSTR_MAX_REG_ADDR: c_uint = 0x1740;
pub const SWR_V2_0_MSTR_MAX_REG_ADDR: c_uint = 0x50ac;
pub const SWRM_V2_0_CLK_CTRL: c_uint = 0x5060;

pub const SWRM_V2_0_LINK_STATUS: c_uint = 0x5064;
pub const SWRM_DP_PORT_CTRL_EN_CHAN_SHFT: c_uint = 0x18;
pub const SWRM_DP_PORT_CTRL_OFFSET2_SHFT: c_uint = 0x10;
pub const SWRM_DP_PORT_CTRL_OFFSET1_SHFT: c_uint = 0x08;
pub const SWRM_AHB_BRIDGE_WR_DATA_0: c_uint = 0xc85;
pub const SWRM_AHB_BRIDGE_WR_ADDR_0: c_uint = 0xc89;
pub const SWRM_AHB_BRIDGE_RD_ADDR_0: c_uint = 0xc8d;
pub const SWRM_AHB_BRIDGE_RD_DATA_0: c_uint = 0xc91;

    ((reg) | ((id) << 16) | ((dev) << 20) | ((data) << 24))
pub const MAX_FREQ_NUM: c_int = 1;
pub const TIMEOUT_MS: c_int = 100;
pub const QCOM_SWRM_MAX_RD_LEN: c_uint = 0x1;
pub const DEFAULT_CLK_FREQ: c_int = 9600000;
pub const SWR_INVALID_PARAM: c_uint = 0xFF;
pub const SWR_HSTOP_MAX_VAL: c_uint = 0xF;
pub const SWR_HSTART_MIN_VAL: c_uint = 0x0;
pub const SWR_BROADCAST_CMD_ID: c_uint = 0x0F;
pub const SWR_MAX_CMD_ID: c_int = 14;
pub const MAX_FIFO_RD_RETRY: c_int = 3;
pub const SWR_OVERFLOW_RETRY_COUNT: c_int = 30;
pub const SWRM_LINK_STATUS_RETRY_CNT: c_int = 100;
    enum {
    MASTER_ID_WSA = 1,
    MASTER_ID_RX,
    MASTER_ID_TX
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_swrm_port_config {
    pub si: u16,
    pub off1: u8,
    pub off2: u8,
    pub bp_mode: u8,
    pub hstart: u8,
    pub hstop: u8,
    pub word_length: u8,
    pub blk_group_count: u8,
    pub lane_control: u8,
}

//
// Internal IDs for different register layouts.  Only few registers differ per
// each variant, so the list of IDs below does not include all of registers.
//
    enum {
    SWRM_REG_FRAME_GEN_ENABLED,
    SWRM_REG_INTERRUPT_STATUS,
    SWRM_REG_INTERRUPT_MASK_ADDR,
    SWRM_REG_INTERRUPT_CLEAR,
    SWRM_REG_INTERRUPT_CPU_EN,
    SWRM_REG_CMD_FIFO_WR_CMD,
    SWRM_REG_CMD_FIFO_RD_CMD,
    SWRM_REG_CMD_FIFO_STATUS,
    SWRM_REG_CMD_FIFO_RD_FIFO_ADDR,
    SWRM_OFFSET_DP_PORT_CTRL_BANK,
    SWRM_OFFSET_DP_PORT_CTRL_2_BANK,
    SWRM_OFFSET_DP_BLOCK_CTRL_1,
    SWRM_OFFSET_DP_BLOCK_CTRL2_BANK,
    SWRM_OFFSET_DP_PORT_HCTRL_BANK,
    SWRM_OFFSET_DP_BLOCK_CTRL3_BANK,
    SWRM_OFFSET_DP_SAMPLECTRL2_BANK,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_swrm_ctrl {
    pub bus: sdw_bus,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub max_reg: u32,
    pub reg_layout: *const c_uint,
    pub mmio: *mut void __iomem,
    pub audio_cgcr: *mut reset_control,

    pub debugfs: *mut dentry,

    pub broadcast: completion,
    pub enumeration: completion,
// Port alloc/free lock
    pub port_lock: mutex,
    pub hclk: *mut clk,
    pub irq: c_int,
    pub version: c_uint,
    pub wake_irq: c_int,
    pub num_din_ports: c_int,
    pub num_dout_ports: c_int,
    pub nports: c_int,
    pub cols_index: c_int,
    pub rows_index: c_int,
    pub port_mask: c_ulong,
    pub intr_mask: u32,
    pub rcmd_id: u8,
    pub wcmd_id: u8,
// Port numbers are 1 - 14
    pub pconfig: *mut qcom_swrm_port_config,
    pub sruntime: *mut sdw_stream_runtime,
    pub 1]: enum sdw_slave_status status[SDW_MAX_DEVICES +,
    pub val): *mut *mut *mut int (reg_read)(struct qcom_swrm_ctrl ctrl, int reg, u32,
    pub val): *mut *mut *mut int (reg_write)(struct qcom_swrm_ctrl ctrl, int reg, int,
    pub slave_status: u32,
    pub wr_fifo_depth: u32,
    pub clock_stop_not_supported: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_swrm_data {
    pub default_cols: u32,
    pub default_rows: u32,
    pub sw_clk_gate_required: bool,
    pub max_reg: u32,
    pub reg_layout: *const c_uint,
}

    static const unsigned int swrm_v1_3_reg_layout[] = {
    [SWRM_REG_FRAME_GEN_ENABLED] = SWRM_COMP_STATUS,
    [SWRM_REG_INTERRUPT_STATUS] = SWRM_V1_3_INTERRUPT_STATUS,
    [SWRM_REG_INTERRUPT_MASK_ADDR] = SWRM_V1_3_INTERRUPT_MASK_ADDR,
    [SWRM_REG_INTERRUPT_CLEAR] = SWRM_V1_3_INTERRUPT_CLEAR,
    [SWRM_REG_INTERRUPT_CPU_EN] = SWRM_V1_3_INTERRUPT_CPU_EN,
    [SWRM_REG_CMD_FIFO_WR_CMD] = SWRM_V1_3_CMD_FIFO_WR_CMD,
    [SWRM_REG_CMD_FIFO_RD_CMD] = SWRM_V1_3_CMD_FIFO_RD_CMD,
    [SWRM_REG_CMD_FIFO_STATUS] = SWRM_V1_3_CMD_FIFO_STATUS,
    [SWRM_REG_CMD_FIFO_RD_FIFO_ADDR] = SWRM_V1_3_CMD_FIFO_RD_FIFO_ADDR,
    [SWRM_OFFSET_DP_PORT_CTRL_BANK]		= 0x1124,
    [SWRM_OFFSET_DP_PORT_CTRL_2_BANK]	= 0x1128,
    [SWRM_OFFSET_DP_BLOCK_CTRL_1]		= 0x112c,
    [SWRM_OFFSET_DP_BLOCK_CTRL2_BANK]	= 0x1130,
    [SWRM_OFFSET_DP_PORT_HCTRL_BANK]	= 0x1134,
    [SWRM_OFFSET_DP_BLOCK_CTRL3_BANK]	= 0x1138,
    [SWRM_OFFSET_DP_SAMPLECTRL2_BANK]	= 0x113c,
    };
    static const struct qcom_swrm_data swrm_v1_3_data = {
    .default_rows = 48,
    .default_cols = 16,
    .max_reg = SWR_V1_3_MSTR_MAX_REG_ADDR,
    .reg_layout = swrm_v1_3_reg_layout,
    };
    static const struct qcom_swrm_data swrm_v1_5_data = {
    .default_rows = 50,
    .default_cols = 16,
    .max_reg = SWR_V1_3_MSTR_MAX_REG_ADDR,
    .reg_layout = swrm_v1_3_reg_layout,
    };
    static const struct qcom_swrm_data swrm_v1_6_data = {
    .default_rows = 50,
    .default_cols = 16,
    .sw_clk_gate_required = true,
    .max_reg = SWR_V1_3_MSTR_MAX_REG_ADDR,
    .reg_layout = swrm_v1_3_reg_layout,
    };
    static const unsigned int swrm_v2_0_reg_layout[] = {
    [SWRM_REG_FRAME_GEN_ENABLED] = SWRM_V2_0_LINK_STATUS,
    [SWRM_REG_INTERRUPT_STATUS] = SWRM_V2_0_INTERRUPT_STATUS,
    [SWRM_REG_INTERRUPT_MASK_ADDR] = 0, /* Not present */
    [SWRM_REG_INTERRUPT_CLEAR] = SWRM_V2_0_INTERRUPT_CLEAR,
    [SWRM_REG_INTERRUPT_CPU_EN] = SWRM_V2_0_INTERRUPT_CPU_EN,
    [SWRM_REG_CMD_FIFO_WR_CMD] = SWRM_V2_0_CMD_FIFO_WR_CMD,
    [SWRM_REG_CMD_FIFO_RD_CMD] = SWRM_V2_0_CMD_FIFO_RD_CMD,
    [SWRM_REG_CMD_FIFO_STATUS] = SWRM_V2_0_CMD_FIFO_STATUS,
    [SWRM_REG_CMD_FIFO_RD_FIFO_ADDR] = SWRM_V2_0_CMD_FIFO_RD_FIFO_ADDR,
    [SWRM_OFFSET_DP_PORT_CTRL_BANK]		= 0x1124,
    [SWRM_OFFSET_DP_PORT_CTRL_2_BANK]	= 0x1128,
    [SWRM_OFFSET_DP_BLOCK_CTRL_1]		= 0x112c,
    [SWRM_OFFSET_DP_BLOCK_CTRL2_BANK]	= 0x1130,
    [SWRM_OFFSET_DP_PORT_HCTRL_BANK]	= 0x1134,
    [SWRM_OFFSET_DP_BLOCK_CTRL3_BANK]	= 0x1138,
    [SWRM_OFFSET_DP_SAMPLECTRL2_BANK]	= 0x113c,
    };
    static const struct qcom_swrm_data swrm_v2_0_data = {
    .default_rows = 50,
    .default_cols = 16,
    .sw_clk_gate_required = true,
    .max_reg = SWR_V2_0_MSTR_MAX_REG_ADDR,
    .reg_layout = swrm_v2_0_reg_layout,
    };
    static const unsigned int swrm_v3_0_reg_layout[] = {
    [SWRM_REG_FRAME_GEN_ENABLED] = SWRM_V2_0_LINK_STATUS,
    [SWRM_REG_INTERRUPT_STATUS] = SWRM_V2_0_INTERRUPT_STATUS,
    [SWRM_REG_INTERRUPT_MASK_ADDR] = 0, /* Not present */
    [SWRM_REG_INTERRUPT_CLEAR] = SWRM_V2_0_INTERRUPT_CLEAR,
    [SWRM_REG_INTERRUPT_CPU_EN] = SWRM_V2_0_INTERRUPT_CPU_EN,
    [SWRM_REG_CMD_FIFO_WR_CMD] = SWRM_V2_0_CMD_FIFO_WR_CMD,
    [SWRM_REG_CMD_FIFO_RD_CMD] = SWRM_V2_0_CMD_FIFO_RD_CMD,
    [SWRM_REG_CMD_FIFO_STATUS] = SWRM_V2_0_CMD_FIFO_STATUS,
    [SWRM_REG_CMD_FIFO_RD_FIFO_ADDR] = SWRM_V2_0_CMD_FIFO_RD_FIFO_ADDR,
    [SWRM_OFFSET_DP_PORT_CTRL_BANK]		= 0x1224,
    [SWRM_OFFSET_DP_PORT_CTRL_2_BANK]	= 0x1228,
    [SWRM_OFFSET_DP_BLOCK_CTRL_1]		= 0x122c,
    [SWRM_OFFSET_DP_BLOCK_CTRL2_BANK]	= 0x1230,
    [SWRM_OFFSET_DP_PORT_HCTRL_BANK]	= 0x1234,
    [SWRM_OFFSET_DP_BLOCK_CTRL3_BANK]	= 0x1238,
    [SWRM_OFFSET_DP_SAMPLECTRL2_BANK]	= 0x123c,
    };
    static const struct qcom_swrm_data swrm_v3_0_data = {
    .default_rows = 50,
    .default_cols = 16,
    .sw_clk_gate_required = true,
    .max_reg = SWR_V2_0_MSTR_MAX_REG_ADDR,
    .reg_layout = swrm_v3_0_reg_layout,
    };

    static int qcom_swrm_ahb_reg_read(struct qcom_swrm_ctrl *ctrl, int reg,
    u32 *val)
    {
    struct regmap *wcd_regmap = ctrl.regmap;
    int ret;
// pg register + offset
    ret = regmap_bulk_write(wcd_regmap, SWRM_AHB_BRIDGE_RD_ADDR_0,
    (u8 *)&reg, 4);
    if (ret < 0)
    return SDW_CMD_FAIL;
    ret = regmap_bulk_read(wcd_regmap, SWRM_AHB_BRIDGE_RD_DATA_0,
    val, 4);
    if (ret < 0)
    return SDW_CMD_FAIL;
    return SDW_CMD_OK;
    }
    static int qcom_swrm_ahb_reg_write(struct qcom_swrm_ctrl *ctrl,
    int reg, int val)
    {
    struct regmap *wcd_regmap = ctrl.regmap;
    int ret;
// pg register + offset
    ret = regmap_bulk_write(wcd_regmap, SWRM_AHB_BRIDGE_WR_DATA_0,
    (u8 *)&val, 4);
    if (ret)
    return SDW_CMD_FAIL;
// write address register
    ret = regmap_bulk_write(wcd_regmap, SWRM_AHB_BRIDGE_WR_ADDR_0,
    (u8 *)&reg, 4);
    if (ret)
    return SDW_CMD_FAIL;
    return SDW_CMD_OK;
    }
    static int qcom_swrm_cpu_reg_read(struct qcom_swrm_ctrl *ctrl, int reg,
    u32 *val)
    {
// val = readl(ctrl->mmio + reg);
    return SDW_CMD_OK;
    }
    static int qcom_swrm_cpu_reg_write(struct qcom_swrm_ctrl *ctrl, int reg,
    int val)
    {
    writel(val, ctrl.mmio + reg);
    return SDW_CMD_OK;
    }
    static u32 swrm_get_packed_reg_val(u8 *cmd_id, u8 cmd_data,
    u8 dev_addr, u16 reg_addr)
    {
    u32 val;
    let mut id: u8 = *cmd_id;
    if (id != SWR_BROADCAST_CMD_ID) {
    if (id < SWR_MAX_CMD_ID)
    id += 1;
    else
    id = 0;
// cmd_id = id;
    }
    val = SWRM_REG_VAL_PACK(cmd_data, dev_addr, id, reg_addr);
    return val;
    }
#[no_mangle]
unsafe extern "C" fn swrm_wait_for_rd_fifo_avail(ctrl: *mut qcom_swrm_ctrl) -> c_int {
    static int swrm_wait_for_rd_fifo_avail(struct qcom_swrm_ctrl *ctrl)
    {
    u32 fifo_outstanding_data, value;
    let mut fifo_retry_count: c_int = SWR_OVERFLOW_RETRY_COUNT;
    do {
// Check for fifo underflow during read
    ctrl.reg_read(ctrl, ctrl.reg_layout[SWRM_REG_CMD_FIFO_STATUS],
    &value);
    fifo_outstanding_data = FIELD_GET(SWRM_RD_CMD_FIFO_CNT_MASK, value);
// Check if read data is available in read fifo
    if (fifo_outstanding_data > 0)
    return 0;
    usleep_range(500, 510);
    } while (fifo_retry_count--);
    if (fifo_outstanding_data == 0) {
    dev_err_ratelimited(ctrl.dev, "%s err read underflow\n", __func__);
    return -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn swrm_wait_for_wr_fifo_avail(ctrl: *mut qcom_swrm_ctrl) -> c_int {
    static int swrm_wait_for_wr_fifo_avail(struct qcom_swrm_ctrl *ctrl)
    {
    u32 fifo_outstanding_cmds, value;
    let mut fifo_retry_count: c_int = SWR_OVERFLOW_RETRY_COUNT;
    do {
// Check for fifo overflow during write
    ctrl.reg_read(ctrl, ctrl.reg_layout[SWRM_REG_CMD_FIFO_STATUS],
    &value);
    fifo_outstanding_cmds = FIELD_GET(SWRM_WR_CMD_FIFO_CNT_MASK, value);
// Check for space in write fifo before writing
    if (fifo_outstanding_cmds < ctrl.wr_fifo_depth)
    return 0;
    usleep_range(500, 510);
    } while (fifo_retry_count--);
    if (fifo_outstanding_cmds == ctrl.wr_fifo_depth) {
    dev_err_ratelimited(ctrl.dev, "%s err write overflow\n", __func__);
    return -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn swrm_wait_for_wr_fifo_done(ctrl: *mut qcom_swrm_ctrl) -> bool {
    static bool swrm_wait_for_wr_fifo_done(struct qcom_swrm_ctrl *ctrl)
    {
    u32 fifo_outstanding_cmds, value;
    let mut fifo_retry_count: c_int = SWR_OVERFLOW_RETRY_COUNT;
// Check for fifo overflow during write
    ctrl.reg_read(ctrl, ctrl.reg_layout[SWRM_REG_CMD_FIFO_STATUS], &value);
    fifo_outstanding_cmds = FIELD_GET(SWRM_WR_CMD_FIFO_CNT_MASK, value);
    if (fifo_outstanding_cmds) {
    while (fifo_retry_count) {
    usleep_range(500, 510);
    ctrl.reg_read(ctrl, ctrl.reg_layout[SWRM_REG_CMD_FIFO_STATUS], &value);
    fifo_outstanding_cmds = FIELD_GET(SWRM_WR_CMD_FIFO_CNT_MASK, value);
    fifo_retry_count--;
    if (fifo_outstanding_cmds == 0)
    return true;
    }
    } else {
    return true;
    }
    return false;
    }
    static int qcom_swrm_cmd_fifo_wr_cmd(struct qcom_swrm_ctrl *ctrl, u8 cmd_data,
    u8 dev_addr, u16 reg_addr)
    {
    u32 val;
    let mut ret: c_int = 0;
    let mut cmd_id: u8 = 0x0;
    if (dev_addr == SDW_BROADCAST_DEV_NUM) {
    cmd_id = SWR_BROADCAST_CMD_ID;
    val = swrm_get_packed_reg_val(&cmd_id, cmd_data,
    dev_addr, reg_addr);
    } else {
    val = swrm_get_packed_reg_val(&ctrl.wcmd_id, cmd_data,
    dev_addr, reg_addr);
    }
    if (swrm_wait_for_wr_fifo_avail(ctrl))
    return SDW_CMD_FAIL_OTHER;
    if (cmd_id == SWR_BROADCAST_CMD_ID)
    reinit_completion(&ctrl.broadcast);
// Its assumed that write is okay as we do not get any status back
    ctrl.reg_write(ctrl, ctrl.reg_layout[SWRM_REG_CMD_FIFO_WR_CMD], val);
    if (ctrl.version <= SWRM_VERSION_1_3_0)
    usleep_range(150, 155);
    if (cmd_id == SWR_BROADCAST_CMD_ID) {
    swrm_wait_for_wr_fifo_done(ctrl);
//
// sleep for 10ms for MSM soundwire variant to allow broadcast
// command to complete.
//
    ret = wait_for_completion_timeout(&ctrl.broadcast,
    msecs_to_jiffies(TIMEOUT_MS));
    if (!ret)
    ret = SDW_CMD_IGNORED;
    else
    ret = SDW_CMD_OK;
    } else {
    ret = SDW_CMD_OK;
    }
    return ret;
    }
    static int qcom_swrm_cmd_fifo_rd_cmd(struct qcom_swrm_ctrl *ctrl,
    u8 dev_addr, u16 reg_addr,
    u32 len, u8 *rval)
    {
    u32 cmd_data, cmd_id, val, retry_attempt = 0;
    val = swrm_get_packed_reg_val(&ctrl.rcmd_id, len, dev_addr, reg_addr);
//
// Check for outstanding cmd wrt. write fifo depth to avoid
// overflow as read will also increase write fifo cnt.
//
    swrm_wait_for_wr_fifo_avail(ctrl);
// wait for FIFO RD to complete to avoid overflow
    usleep_range(100, 105);
    ctrl.reg_write(ctrl, ctrl.reg_layout[SWRM_REG_CMD_FIFO_RD_CMD], val);
// wait for FIFO RD CMD complete to avoid overflow
    usleep_range(250, 255);
    if (swrm_wait_for_rd_fifo_avail(ctrl))
    return SDW_CMD_FAIL_OTHER;
    do {
    ctrl.reg_read(ctrl, ctrl.reg_layout[SWRM_REG_CMD_FIFO_RD_FIFO_ADDR],
    &cmd_data);
    rval[0] = cmd_data & 0xFF;
    cmd_id = FIELD_GET(SWRM_RD_FIFO_CMD_ID_MASK, cmd_data);
    if (cmd_id != ctrl.rcmd_id) {
    if (retry_attempt < (MAX_FIFO_RD_RETRY - 1)) {
// wait 500 us before retry on fifo read failure
    usleep_range(500, 505);
    ctrl.reg_write(ctrl, SWRM_CMD_FIFO_CMD,
    SWRM_CMD_FIFO_FLUSH);
    ctrl.reg_write(ctrl,
    ctrl.reg_layout[SWRM_REG_CMD_FIFO_RD_CMD],
    val);
    }
    retry_attempt++;
    } else {
    return SDW_CMD_OK;
    }
    } while (retry_attempt < MAX_FIFO_RD_RETRY);
    dev_err(ctrl.dev, "failed to read fifo: reg: 0x%x, rcmd_id: 0x%x,\
    dev_num: 0x%x, cmd_data: 0x%x\n",
    reg_addr, ctrl.rcmd_id, dev_addr, cmd_data);
    return SDW_CMD_IGNORED;
    }
#[no_mangle]
unsafe extern "C" fn qcom_swrm_get_alert_slave_dev_num(ctrl: *mut qcom_swrm_ctrl) -> c_int {
    static int qcom_swrm_get_alert_slave_dev_num(struct qcom_swrm_ctrl *ctrl)
    {
    u32 val, status;
    int dev_num;
    ctrl.reg_read(ctrl, SWRM_MCP_SLV_STATUS, &val);
    for (dev_num = 1; dev_num <= SDW_MAX_DEVICES; dev_num++) {
    status = (val >> (dev_num * SWRM_MCP_SLV_STATUS_SZ));
    if ((status & SWRM_MCP_SLV_STATUS_MASK) == SDW_SLAVE_ALERT) {
    ctrl.status[dev_num] = status & SWRM_MCP_SLV_STATUS_MASK;
    return dev_num;
    }
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn qcom_swrm_get_device_status(ctrl: *mut qcom_swrm_ctrl) {
    static void qcom_swrm_get_device_status(struct qcom_swrm_ctrl *ctrl)
    {
    u32 val;
    int i;
    ctrl.reg_read(ctrl, SWRM_MCP_SLV_STATUS, &val);
    ctrl.slave_status = val;
    for (i = 1; i <= SDW_MAX_DEVICES; i++) {
    u32 s;
    s = (val >> (i * 2));
    s &= SWRM_MCP_SLV_STATUS_MASK;
    ctrl.status[i] = s;
    }
    }
    static void qcom_swrm_set_slave_dev_num(struct sdw_bus *bus,
    struct sdw_slave *slave, int devnum)
    {
    struct qcom_swrm_ctrl *ctrl = to_qcom_sdw(bus);
    u32 status;
    ctrl.reg_read(ctrl, SWRM_MCP_SLV_STATUS, &status);
    status = (status >> (devnum * SWRM_MCP_SLV_STATUS_SZ));
    status &= SWRM_MCP_SLV_STATUS_MASK;
    if (status == SDW_SLAVE_ATTACHED) {
    if (slave)
    slave.dev_num = devnum;
    mutex_lock(&bus.bus_lock);
    set_bit(devnum, bus.assigned);
    mutex_unlock(&bus.bus_lock);
    }
    }
#[no_mangle]
unsafe extern "C" fn qcom_swrm_enumerate(bus: *mut sdw_bus) -> c_int {
    static int qcom_swrm_enumerate(struct sdw_bus *bus)
    {
    struct qcom_swrm_ctrl *ctrl = to_qcom_sdw(bus);
    struct sdw_slave *slave, *_s;
    struct sdw_slave_id id;
    u32 val1, val2;
    bool found;
    u64 addr;
    int i;
    char *buf1 = (char *)&val1, *buf2 = (char *)&val2;
    for (i = 1; i <= SDW_MAX_DEVICES; i++) {
// do not continue if the status is Not Present
    if (!ctrl.status[i])
    continue;
// SCP_Devid5 - Devid 4
    ctrl.reg_read(ctrl, SWRM_ENUMERATOR_SLAVE_DEV_ID_1(i), &val1);
// SCP_Devid3 - DevId 2 Devid 1 Devid 0
    ctrl.reg_read(ctrl, SWRM_ENUMERATOR_SLAVE_DEV_ID_2(i), &val2);
    if (!val1 && !val2)
    break;
    addr = buf2[1] | (buf2[0] << 8) | (buf1[3] << 16) |
    ((u64)buf1[2] << 24) | ((u64)buf1[1] << 32) |
    ((u64)buf1[0] << 40);
    sdw_extract_slave_id(bus, addr, &id);
    found = false;
    ctrl.clock_stop_not_supported = false;
// Now compare with entries
    list_for_each_entry_safe(slave, _s, &bus.slaves, node) {
    if (sdw_compare_devid(slave, id) == 0) {
    qcom_swrm_set_slave_dev_num(bus, slave, i);
    if (slave.prop.clk_stop_mode1)
    ctrl.clock_stop_not_supported = true;
    found = true;
    break;
    }
    }
    if (!found) {
    qcom_swrm_set_slave_dev_num(bus, core::ptr::null_mut(), i);
    sdw_slave_add(bus, &id, core::ptr::null_mut());
    }
    }
    complete(&ctrl.enumeration);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_swrm_wake_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t qcom_swrm_wake_irq_handler(int irq, void *dev_id)
    {
    struct qcom_swrm_ctrl *ctrl = dev_id;
    int ret;
    ret = pm_runtime_get_sync(ctrl.dev);
    if (ret < 0 && ret != -EACCES) {
    dev_err_ratelimited(ctrl.dev,
    "pm_runtime_get_sync failed in %s, ret %d\n",
    __func__, ret);
    pm_runtime_put_noidle(ctrl.dev);
    return ret;
    }
    if (ctrl.wake_irq > 0) {
    if (!irqd_irq_disabled(irq_get_irq_data(ctrl.wake_irq)))
    disable_irq_nosync(ctrl.wake_irq);
    }
    pm_runtime_mark_last_busy(ctrl.dev);
    pm_runtime_put_autosuspend(ctrl.dev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn qcom_swrm_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t qcom_swrm_irq_handler(int irq, void *dev_id)
    {
    struct qcom_swrm_ctrl *ctrl = dev_id;
    u32 value, intr_sts, intr_sts_masked, slave_status;
    u32 i;
    int devnum;
    let mut ret: c_int = IRQ_HANDLED;
    clk_prepare_enable(ctrl.hclk);
    ctrl.reg_read(ctrl, ctrl.reg_layout[SWRM_REG_INTERRUPT_STATUS],
    &intr_sts);
    intr_sts_masked = intr_sts & ctrl.intr_mask;
    do {
    for (i = 0; i < SWRM_INTERRUPT_MAX; i++) {
    value = intr_sts_masked & BIT(i);
    if (!value)
    continue;
    switch (value) {
    case SWRM_INTERRUPT_STATUS_SLAVE_PEND_IRQ:
    devnum = qcom_swrm_get_alert_slave_dev_num(ctrl);
    if (devnum < 0) {
    dev_err_ratelimited(ctrl.dev,
    "no slave alert found.spurious interrupt\n");
    } else {
    sdw_handle_slave_status(&ctrl.bus, ctrl.status);
    }
    break;
    case SWRM_INTERRUPT_STATUS_NEW_SLAVE_ATTACHED:
    case SWRM_INTERRUPT_STATUS_CHANGE_ENUM_SLAVE_STATUS:
    dev_dbg_ratelimited(ctrl.dev, "SWR new slave attached\n");
    ctrl.reg_read(ctrl, SWRM_MCP_SLV_STATUS, &slave_status);
    if (ctrl.slave_status == slave_status) {
    dev_dbg(ctrl.dev, "Slave status not changed %x\n",
    slave_status);
    } else {
    qcom_swrm_get_device_status(ctrl);
    qcom_swrm_enumerate(&ctrl.bus);
    sdw_handle_slave_status(&ctrl.bus, ctrl.status);
    }
    break;
    case SWRM_INTERRUPT_STATUS_MASTER_CLASH_DET:
    dev_err_ratelimited(ctrl.dev,
    "%s: SWR bus clsh detected\n",
    __func__);
    ctrl.intr_mask &= ~SWRM_INTERRUPT_STATUS_MASTER_CLASH_DET;
    ctrl.reg_write(ctrl,
    ctrl.reg_layout[SWRM_REG_INTERRUPT_CPU_EN],
    ctrl.intr_mask);
    break;
    case SWRM_INTERRUPT_STATUS_RD_FIFO_OVERFLOW:
    ctrl.reg_read(ctrl,
    ctrl.reg_layout[SWRM_REG_CMD_FIFO_STATUS],
    &value);
    dev_err_ratelimited(ctrl.dev,
    "%s: SWR read FIFO overflow fifo status 0x%x\n",
    __func__, value);
    break;
    case SWRM_INTERRUPT_STATUS_RD_FIFO_UNDERFLOW:
    ctrl.reg_read(ctrl,
    ctrl.reg_layout[SWRM_REG_CMD_FIFO_STATUS],
    &value);
    dev_err_ratelimited(ctrl.dev,
    "%s: SWR read FIFO underflow fifo status 0x%x\n",
    __func__, value);
    break;
    case SWRM_INTERRUPT_STATUS_WR_CMD_FIFO_OVERFLOW:
    ctrl.reg_read(ctrl,
    ctrl.reg_layout[SWRM_REG_CMD_FIFO_STATUS],
    &value);
    dev_err(ctrl.dev,
    "%s: SWR write FIFO overflow fifo status %x\n",
    __func__, value);
    ctrl.reg_write(ctrl, SWRM_CMD_FIFO_CMD, 0x1);
    break;
    case SWRM_INTERRUPT_STATUS_CMD_ERROR:
    ctrl.reg_read(ctrl,
    ctrl.reg_layout[SWRM_REG_CMD_FIFO_STATUS],
    &value);
    dev_err_ratelimited(ctrl.dev,
    "%s: SWR CMD error, fifo status 0x%x, flushing fifo\n",
    __func__, value);
    ctrl.reg_write(ctrl, SWRM_CMD_FIFO_CMD, 0x1);
    break;
    case SWRM_INTERRUPT_STATUS_DOUT_PORT_COLLISION:
    dev_err_ratelimited(ctrl.dev,
    "%s: SWR Port collision detected\n",
    __func__);
    ctrl.intr_mask &= ~SWRM_INTERRUPT_STATUS_DOUT_PORT_COLLISION;
    ctrl.reg_write(ctrl,
    ctrl.reg_layout[SWRM_REG_INTERRUPT_CPU_EN],
    ctrl.intr_mask);
    break;
    case SWRM_INTERRUPT_STATUS_READ_EN_RD_VALID_MISMATCH:
    dev_err_ratelimited(ctrl.dev,
    "%s: SWR read enable valid mismatch\n",
    __func__);
    ctrl.intr_mask &=
    ~SWRM_INTERRUPT_STATUS_READ_EN_RD_VALID_MISMATCH;
    ctrl.reg_write(ctrl,
    ctrl.reg_layout[SWRM_REG_INTERRUPT_CPU_EN],
    ctrl.intr_mask);
    break;
    case SWRM_INTERRUPT_STATUS_SPECIAL_CMD_ID_FINISHED:
    complete(&ctrl.broadcast);
    break;
    case SWRM_INTERRUPT_STATUS_BUS_RESET_FINISHED_V2:
    break;
    case SWRM_INTERRUPT_STATUS_CLK_STOP_FINISHED_V2:
    break;
    case SWRM_INTERRUPT_STATUS_EXT_CLK_STOP_WAKEUP:
    break;
    case SWRM_INTERRUPT_STATUS_CMD_IGNORED_AND_EXEC_CONTINUED:
    ctrl.reg_read(ctrl,
    ctrl.reg_layout[SWRM_REG_CMD_FIFO_STATUS],
    &value);
    dev_err(ctrl.dev,
    "%s: SWR CMD ignored, fifo status %x\n",
    __func__, value);
// Wait 3.5ms to clear
    usleep_range(3500, 3505);
    break;
    default:
    dev_err_ratelimited(ctrl.dev,
    "%s: SWR unknown interrupt value: %d\n",
    __func__, value);
    ret = IRQ_NONE;
    break;
    }
    }
    ctrl.reg_write(ctrl, ctrl.reg_layout[SWRM_REG_INTERRUPT_CLEAR],
    intr_sts);
    ctrl.reg_read(ctrl, ctrl.reg_layout[SWRM_REG_INTERRUPT_STATUS],
    &intr_sts);
    intr_sts_masked = intr_sts & ctrl.intr_mask;
    } while (intr_sts_masked);
    clk_disable_unprepare(ctrl.hclk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn swrm_wait_for_frame_gen_enabled(ctrl: *mut qcom_swrm_ctrl) -> bool {
    static bool swrm_wait_for_frame_gen_enabled(struct qcom_swrm_ctrl *ctrl)
    {
    let mut retry: c_int = SWRM_LINK_STATUS_RETRY_CNT;
    int comp_sts;
    do {
    ctrl.reg_read(ctrl, ctrl.reg_layout[SWRM_REG_FRAME_GEN_ENABLED],
    &comp_sts);
    if (comp_sts & SWRM_FRM_GEN_ENABLED)
    return true;
    usleep_range(500, 510);
    } while (retry--);
    dev_err(ctrl.dev, "%s: link status not %s\n", __func__,
    comp_sts & SWRM_FRM_GEN_ENABLED ? "connected" : "disconnected");
    return false;
    }
#[no_mangle]
unsafe extern "C" fn qcom_swrm_init(ctrl: *mut qcom_swrm_ctrl) -> c_int {
    static int qcom_swrm_init(struct qcom_swrm_ctrl *ctrl)
    {
    u32 val;
// Clear Rows and Cols
    val = FIELD_PREP(SWRM_MCP_FRAME_CTRL_BANK_ROW_CTRL_BMSK, ctrl.rows_index);
    val |= FIELD_PREP(SWRM_MCP_FRAME_CTRL_BANK_COL_CTRL_BMSK, ctrl.cols_index);
    reset_control_reset(ctrl.audio_cgcr);
    ctrl.reg_write(ctrl, SWRM_MCP_FRAME_CTRL_BANK_ADDR(0), val);
// Enable Auto enumeration
    ctrl.reg_write(ctrl, SWRM_ENUMERATOR_CFG_ADDR, 1);
    ctrl.intr_mask = SWRM_INTERRUPT_STATUS_RMSK;
// Mask soundwire interrupts
    if (ctrl.version < SWRM_VERSION_2_0_0)
    ctrl.reg_write(ctrl, ctrl.reg_layout[SWRM_REG_INTERRUPT_MASK_ADDR],
    SWRM_INTERRUPT_STATUS_RMSK);
// Configure No pings
    ctrl.reg_read(ctrl, SWRM_MCP_CFG_ADDR, &val);
    u32p_replace_bits(&val, SWRM_DEF_CMD_NO_PINGS, SWRM_MCP_CFG_MAX_NUM_OF_CMD_NO_PINGS_BMSK);
    ctrl.reg_write(ctrl, SWRM_MCP_CFG_ADDR, val);
    if (ctrl.version == SWRM_VERSION_1_7_0) {
    ctrl.reg_write(ctrl, SWRM_LINK_MANAGER_EE, SWRM_EE_CPU);
    ctrl.reg_write(ctrl, SWRM_MCP_BUS_CTRL,
    SWRM_MCP_BUS_CLK_START << SWRM_EE_CPU);
    } else if (ctrl.version >= SWRM_VERSION_2_0_0) {
    ctrl.reg_write(ctrl, SWRM_LINK_MANAGER_EE, SWRM_EE_CPU);
    ctrl.reg_write(ctrl, SWRM_V2_0_CLK_CTRL,
    SWRM_V2_0_CLK_CTRL_CLK_START);
    } else {
    ctrl.reg_write(ctrl, SWRM_MCP_BUS_CTRL, SWRM_MCP_BUS_CLK_START);
    }
// Configure number of retries of a read/write cmd
    if (ctrl.version >= SWRM_VERSION_1_5_1) {
    ctrl.reg_write(ctrl, SWRM_CMD_FIFO_CFG_ADDR,
    SWRM_RD_WR_CMD_RETRIES |
    SWRM_CONTINUE_EXEC_ON_CMD_IGNORE);
    } else {
    ctrl.reg_write(ctrl, SWRM_CMD_FIFO_CFG_ADDR,
    SWRM_RD_WR_CMD_RETRIES);
    }
// COMP Enable
    ctrl.reg_write(ctrl, SWRM_COMP_CFG_ADDR, SWRM_COMP_CFG_ENABLE_MSK);
// Set IRQ to PULSE
    ctrl.reg_write(ctrl, SWRM_COMP_CFG_ADDR,
    SWRM_COMP_CFG_IRQ_LEVEL_OR_PULSE_MSK);
    ctrl.reg_write(ctrl, ctrl.reg_layout[SWRM_REG_INTERRUPT_CLEAR],
    0xFFFFFFFF);
// enable CPU IRQs
    if (ctrl.mmio) {
    ctrl.reg_write(ctrl, ctrl.reg_layout[SWRM_REG_INTERRUPT_CPU_EN],
    SWRM_INTERRUPT_STATUS_RMSK);
    }
// Set IRQ to PULSE
    ctrl.reg_write(ctrl, SWRM_COMP_CFG_ADDR,
    SWRM_COMP_CFG_IRQ_LEVEL_OR_PULSE_MSK |
    SWRM_COMP_CFG_ENABLE_MSK);
    swrm_wait_for_frame_gen_enabled(ctrl);
    ctrl.slave_status = 0;
    ctrl.reg_read(ctrl, SWRM_COMP_PARAMS, &val);
    if (ctrl.version >= SWRM_VERSION_3_1_0)
    ctrl.wr_fifo_depth = FIELD_GET(SWRM_V3_COMP_PARAMS_WR_FIFO_DEPTH, val);
    else
    ctrl.wr_fifo_depth = FIELD_GET(SWRM_COMP_PARAMS_WR_FIFO_DEPTH, val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_swrm_read_prop(bus: *mut sdw_bus) -> c_int {
    static int qcom_swrm_read_prop(struct sdw_bus *bus)
    {
    struct qcom_swrm_ctrl *ctrl = to_qcom_sdw(bus);
    if (ctrl.version >= SWRM_VERSION_2_0_0) {
    bus.multi_link = true;
    bus.hw_sync_min_links = 3;
    }
    return 0;
    }
    static enum sdw_command_response qcom_swrm_xfer_msg(struct sdw_bus *bus,
    struct sdw_msg *msg)
    {
    struct qcom_swrm_ctrl *ctrl = to_qcom_sdw(bus);
    int ret, i, len;
    if (msg.page) {
    ret = qcom_swrm_cmd_fifo_wr_cmd(ctrl, msg.addr_page1,
    msg.dev_num,
    SDW_SCP_ADDRPAGE1);
    if (ret)
    return ret;
    ret = qcom_swrm_cmd_fifo_wr_cmd(ctrl, msg.addr_page2,
    msg.dev_num,
    SDW_SCP_ADDRPAGE2);
    if (ret)
    return ret;
    }
    if (msg.flags == SDW_MSG_FLAG_READ) {
    for (i = 0; i < msg.len;) {
    len = min(msg.len - i, QCOM_SWRM_MAX_RD_LEN);
    ret = qcom_swrm_cmd_fifo_rd_cmd(ctrl, msg.dev_num,
    msg.addr + i, len,
    &msg.buf[i]);
    if (ret)
    return ret;
    i = i + len;
    }
    } else if (msg.flags == SDW_MSG_FLAG_WRITE) {
    for (i = 0; i < msg.len; i++) {
    ret = qcom_swrm_cmd_fifo_wr_cmd(ctrl, msg.buf[i],
    msg.dev_num,
    msg.addr + i);
    if (ret)
    return SDW_CMD_IGNORED;
    }
    }
    return SDW_CMD_OK;
    }
#[no_mangle]
unsafe extern "C" fn qcom_swrm_pre_bank_switch(bus: *mut sdw_bus) -> c_int {
    static int qcom_swrm_pre_bank_switch(struct sdw_bus *bus)
    {
    let mut reg: u32 = SWRM_MCP_FRAME_CTRL_BANK_ADDR(bus.params.next_bank);
    struct qcom_swrm_ctrl *ctrl = to_qcom_sdw(bus);
    u32 val;
    ctrl.reg_read(ctrl, reg, &val);
    u32p_replace_bits(&val, ctrl.cols_index, SWRM_MCP_FRAME_CTRL_BANK_COL_CTRL_BMSK);
    u32p_replace_bits(&val, ctrl.rows_index, SWRM_MCP_FRAME_CTRL_BANK_ROW_CTRL_BMSK);
    return ctrl.reg_write(ctrl, reg, val);
    }
    static int qcom_swrm_port_params(struct sdw_bus *bus,
    struct sdw_port_params *p_params,
    unsigned int bank)
    {
    struct qcom_swrm_ctrl *ctrl = to_qcom_sdw(bus);
    let mut offset: u32 = ctrl.reg_layout[SWRM_OFFSET_DP_BLOCK_CTRL_1];
    return ctrl.reg_write(ctrl, SWRM_DPn_BLOCK_CTRL_1(offset, p_params.num),
    p_params.bps - 1);
    }
    static int qcom_swrm_transport_params(struct sdw_bus *bus,
    struct sdw_transport_params *params,
    enum sdw_reg_bank bank)
    {
    struct qcom_swrm_ctrl *ctrl = to_qcom_sdw(bus);
    struct qcom_swrm_port_config *pcfg;
    u32 value;
    int reg, offset = ctrl.reg_layout[SWRM_OFFSET_DP_PORT_CTRL_BANK];
    int ret;
    reg = SWRM_DPn_PORT_CTRL_BANK(offset, params.port_num, bank);
    pcfg = &ctrl.pconfig[params.port_num];
    value = pcfg.off1 << SWRM_DP_PORT_CTRL_OFFSET1_SHFT;
    value |= pcfg.off2 << SWRM_DP_PORT_CTRL_OFFSET2_SHFT;
    value |= pcfg.si & 0xff;
    ret = ctrl.reg_write(ctrl, reg, value);
    if (ret)
    goto err;
    if (pcfg.si > 0xff) {
    offset = ctrl.reg_layout[SWRM_OFFSET_DP_SAMPLECTRL2_BANK];
    value = (pcfg.si >> 8) & 0xff;
    reg = SWRM_DPn_SAMPLECTRL2_BANK(offset, params.port_num, bank);
    ret = ctrl.reg_write(ctrl, reg, value);
    if (ret)
    goto err;
    }
    if (pcfg.lane_control != SWR_INVALID_PARAM) {
    offset = ctrl.reg_layout[SWRM_OFFSET_DP_PORT_CTRL_2_BANK];
    reg = SWRM_DPn_PORT_CTRL_2_BANK(offset, params.port_num, bank);
    value = pcfg.lane_control;
    ret = ctrl.reg_write(ctrl, reg, value);
    if (ret)
    goto err;
    }
    if (pcfg.blk_group_count != SWR_INVALID_PARAM) {
    offset = ctrl.reg_layout[SWRM_OFFSET_DP_BLOCK_CTRL2_BANK];
    reg = SWRM_DPn_BLOCK_CTRL2_BANK(offset, params.port_num, bank);
    value = pcfg.blk_group_count;
    ret = ctrl.reg_write(ctrl, reg, value);
    if (ret)
    goto err;
    }
    offset = ctrl.reg_layout[SWRM_OFFSET_DP_PORT_HCTRL_BANK];
    reg = SWRM_DPn_PORT_HCTRL_BANK(offset, params.port_num, bank);
    if (pcfg.hstart != SWR_INVALID_PARAM && pcfg.hstop != SWR_INVALID_PARAM) {
    value = (pcfg.hstop << 4) | pcfg.hstart;
    ret = ctrl.reg_write(ctrl, reg, value);
    } else {
    value = (SWR_HSTOP_MAX_VAL << 4) | SWR_HSTART_MIN_VAL;
    ret = ctrl.reg_write(ctrl, reg, value);
    }
    if (ret)
    goto err;
    if (pcfg.bp_mode != SWR_INVALID_PARAM) {
    offset = ctrl.reg_layout[SWRM_OFFSET_DP_BLOCK_CTRL3_BANK];
    reg = SWRM_DPn_BLOCK_CTRL3_BANK(offset, params.port_num, bank);
    ret = ctrl.reg_write(ctrl, reg, pcfg.bp_mode);
    }
    err:
    return ret;
    }
    static int qcom_swrm_port_enable(struct sdw_bus *bus,
    struct sdw_enable_ch *enable_ch,
    unsigned int bank)
    {
    u32 reg;
    struct qcom_swrm_ctrl *ctrl = to_qcom_sdw(bus);
    u32 val;
    let mut offset: u32 = ctrl.reg_layout[SWRM_OFFSET_DP_PORT_CTRL_BANK];
    reg = SWRM_DPn_PORT_CTRL_BANK(offset, enable_ch.port_num, bank);
    ctrl.reg_read(ctrl, reg, &val);
    if (enable_ch.enable)
    val |= (enable_ch.ch_mask << SWRM_DP_PORT_CTRL_EN_CHAN_SHFT);
    else
    val &= ~(0xff << SWRM_DP_PORT_CTRL_EN_CHAN_SHFT);
    return ctrl.reg_write(ctrl, reg, val);
    }
    static const struct sdw_master_port_ops qcom_swrm_port_ops = {
    .dpn_set_port_params = qcom_swrm_port_params,
    .dpn_set_port_transport_params = qcom_swrm_transport_params,
    .dpn_port_enable_ch = qcom_swrm_port_enable,
    };
    static const struct sdw_master_ops qcom_swrm_ops = {
    .read_prop = qcom_swrm_read_prop,
    .xfer_msg = qcom_swrm_xfer_msg,
    .pre_bank_switch = qcom_swrm_pre_bank_switch,
    };
#[no_mangle]
unsafe extern "C" fn qcom_swrm_compute_params(bus: *mut sdw_bus, stream: *mut sdw_stream_runtime) -> c_int {
    static int qcom_swrm_compute_params(struct sdw_bus *bus, struct sdw_stream_runtime *stream)
    {
    struct qcom_swrm_ctrl *ctrl = to_qcom_sdw(bus);
    struct sdw_master_runtime *m_rt;
    struct sdw_slave_runtime *s_rt;
    struct sdw_port_runtime *p_rt;
    struct qcom_swrm_port_config *pcfg;
    struct sdw_slave *slave;
    unsigned int m_port;
    let mut i: c_int = 1;
    list_for_each_entry(m_rt, &bus.m_rt_list, bus_node) {
    list_for_each_entry(p_rt, &m_rt.port_list, port_node) {
    pcfg = &ctrl.pconfig[p_rt.num];
    p_rt.transport_params.port_num = p_rt.num;
    if (pcfg.word_length != SWR_INVALID_PARAM) {
    sdw_fill_port_params(&p_rt.port_params,
    p_rt.num,  pcfg.word_length + 1,
    SDW_PORT_FLOW_MODE_ISOCH,
    SDW_PORT_DATA_MODE_NORMAL);
    }
    }
    list_for_each_entry(s_rt, &m_rt.slave_rt_list, m_rt_node) {
    slave = s_rt.slave;
    list_for_each_entry(p_rt, &s_rt.port_list, port_node) {
    m_port = slave.m_port_map[p_rt.num];
// port config starts at offset 0 so -1 from actual port number
    if (m_port)
    pcfg = &ctrl.pconfig[m_port];
    else
    pcfg = &ctrl.pconfig[i];
    p_rt.transport_params.port_num = p_rt.num;
    p_rt.transport_params.sample_interval =
    pcfg.si + 1;
    p_rt.transport_params.offset1 = pcfg.off1;
    p_rt.transport_params.offset2 = pcfg.off2;
    p_rt.transport_params.blk_pkg_mode = pcfg.bp_mode;
    p_rt.transport_params.blk_grp_ctrl = pcfg.blk_group_count;
    p_rt.transport_params.hstart = pcfg.hstart;
    p_rt.transport_params.hstop = pcfg.hstop;
    p_rt.transport_params.lane_ctrl = pcfg.lane_control;
    if (pcfg.word_length != SWR_INVALID_PARAM) {
    sdw_fill_port_params(&p_rt.port_params,
    p_rt.num,
    pcfg.word_length + 1,
    SDW_PORT_FLOW_MODE_ISOCH,
    SDW_PORT_DATA_MODE_NORMAL);
    }
    i++;
    }
    }
    }
    return 0;
    }
    static u32 qcom_swrm_freq_tbl[MAX_FREQ_NUM] = {
    DEFAULT_CLK_FREQ,
    };
    static void qcom_swrm_stream_free_ports(struct qcom_swrm_ctrl *ctrl,
    struct sdw_stream_runtime *stream)
    {
    struct sdw_master_runtime *m_rt;
    struct sdw_port_runtime *p_rt;
    unsigned long *port_mask;
    mutex_lock(&ctrl.port_lock);
    list_for_each_entry(m_rt, &stream.master_list, stream_node) {
    port_mask = &ctrl.port_mask;
    list_for_each_entry(p_rt, &m_rt.port_list, port_node)
    clear_bit(p_rt.num, port_mask);
    }
    mutex_unlock(&ctrl.port_lock);
    }
    static int qcom_swrm_stream_alloc_ports(struct qcom_swrm_ctrl *ctrl,
    struct sdw_stream_runtime *stream,
    struct snd_pcm_hw_params *params,
    int direction)
    {
    struct sdw_stream_config sconfig;
    struct sdw_master_runtime *m_rt;
    struct sdw_slave_runtime *s_rt;
    struct sdw_port_runtime *p_rt;
    struct sdw_slave *slave;
    unsigned long *port_mask;
    int maxport, pn, nports = 0;
    unsigned int m_port;
    struct sdw_port_config *pconfig __free(kfree) = kzalloc_objs(*pconfig,
    ctrl.nports);
    if (!pconfig)
    return -ENOMEM;
    if (direction == SNDRV_PCM_STREAM_CAPTURE)
    sconfig.direction = SDW_DATA_DIR_TX;
    else
    sconfig.direction = SDW_DATA_DIR_RX;
// hw parameters will be ignored as we only support PDM
    sconfig.ch_count = 1;
    sconfig.frame_rate = params_rate(params);
    sconfig.type = stream.type;
    sconfig.bps = 1;
    guard(mutex)(&ctrl.port_lock);
    list_for_each_entry(m_rt, &stream.master_list, stream_node) {
//
// For streams with multiple masters:
// Allocate ports only for devices connected to this master.
// Such devices will have ports allocated by their own master
// and its qcom_swrm_stream_alloc_ports() call.
//
    if (ctrl.bus.id != m_rt.bus.id)
    continue;
    port_mask = &ctrl.port_mask;
    maxport = ctrl.nports;
    list_for_each_entry(s_rt, &m_rt.slave_rt_list, m_rt_node) {
    slave = s_rt.slave;
    list_for_each_entry(p_rt, &s_rt.port_list, port_node) {
    m_port = slave.m_port_map[p_rt.num];
// Port numbers start from 1 - 14
    if (m_port)
    pn = m_port;
    else
    pn = find_first_zero_bit(port_mask, maxport);
    if (pn >= maxport) {
    dev_err(ctrl.dev, "All ports busy\n");
    return -EBUSY;
    }
    set_bit(pn, port_mask);
    pconfig[nports].num = pn;
    pconfig[nports].ch_mask = p_rt.ch_mask;
    nports++;
    }
    }
    }
    sdw_stream_add_master(&ctrl.bus, &sconfig, pconfig,
    nports, stream);
    return 0;
    }
    static int qcom_swrm_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params,
    struct snd_soc_dai *dai)
    {
    struct qcom_swrm_ctrl *ctrl = dev_get_drvdata(dai.dev);
    struct sdw_stream_runtime *sruntime = ctrl.sruntime[dai.id];
    int ret;
    ret = qcom_swrm_stream_alloc_ports(ctrl, sruntime, params,
    substream.stream);
    if (ret)
    qcom_swrm_stream_free_ports(ctrl, sruntime);
    return ret;
    }
    static int qcom_swrm_hw_free(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct qcom_swrm_ctrl *ctrl = dev_get_drvdata(dai.dev);
    struct sdw_stream_runtime *sruntime = ctrl.sruntime[dai.id];
    qcom_swrm_stream_free_ports(ctrl, sruntime);
    sdw_stream_remove_master(&ctrl.bus, sruntime);
    return 0;
    }
    static int qcom_swrm_set_sdw_stream(struct snd_soc_dai *dai,
    void *stream, int direction)
    {
    struct qcom_swrm_ctrl *ctrl = dev_get_drvdata(dai.dev);
    ctrl.sruntime[dai.id] = stream;
    return 0;
    }
    static void *qcom_swrm_get_sdw_stream(struct snd_soc_dai *dai, int direction)
    {
    struct qcom_swrm_ctrl *ctrl = dev_get_drvdata(dai.dev);
    return ctrl.sruntime[dai.id];
    }
    static int qcom_swrm_startup(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct qcom_swrm_ctrl *ctrl = dev_get_drvdata(dai.dev);
    int ret;
    ret = pm_runtime_get_sync(ctrl.dev);
    if (ret < 0 && ret != -EACCES) {
    dev_err_ratelimited(ctrl.dev,
    "pm_runtime_get_sync failed in %s, ret %d\n",
    __func__, ret);
    pm_runtime_put_noidle(ctrl.dev);
    return ret;
    }
    return 0;
    }
    static void qcom_swrm_shutdown(struct snd_pcm_substream *substream,
    struct snd_soc_dai *dai)
    {
    struct qcom_swrm_ctrl *ctrl = dev_get_drvdata(dai.dev);
    swrm_wait_for_wr_fifo_done(ctrl);
    pm_runtime_mark_last_busy(ctrl.dev);
    pm_runtime_put_autosuspend(ctrl.dev);
    }
    static const struct snd_soc_dai_ops qcom_swrm_pdm_dai_ops = {
    .hw_params = qcom_swrm_hw_params,
    .hw_free = qcom_swrm_hw_free,
    .startup = qcom_swrm_startup,
    .shutdown = qcom_swrm_shutdown,
    .set_stream = qcom_swrm_set_sdw_stream,
    .get_stream = qcom_swrm_get_sdw_stream,
    };
    static const struct snd_soc_component_driver qcom_swrm_dai_component = {
    .name = "soundwire",
    };
#[no_mangle]
unsafe extern "C" fn qcom_swrm_register_dais(ctrl: *mut qcom_swrm_ctrl) -> c_int {
    static int qcom_swrm_register_dais(struct qcom_swrm_ctrl *ctrl)
    {
    let mut num_dais: c_int = ctrl.num_dout_ports + ctrl.num_din_ports;
    struct snd_soc_dai_driver *dais;
    struct snd_soc_pcm_stream *stream;
    struct device *dev = ctrl.dev;
    int i;
    ctrl.sruntime = devm_kcalloc(dev, num_dais, sizeof(*ctrl.sruntime), GFP_KERNEL);
    if (!ctrl.sruntime)
    return -ENOMEM;
// PDM dais are only tested for now
    dais = devm_kcalloc(dev, num_dais, sizeof(*dais), GFP_KERNEL);
    if (!dais)
    return -ENOMEM;
    for (i = 0; i < num_dais; i++) {
    dais[i].name = devm_kasprintf(dev, GFP_KERNEL, "SDW Pin%d", i);
    if (!dais[i].name)
    return -ENOMEM;
    if (i < ctrl.num_dout_ports)
    stream = &dais[i].playback;
    else
    stream = &dais[i].capture;
    stream.channels_min = 1;
    stream.channels_max = 1;
    stream.rates = SNDRV_PCM_RATE_48000;
    stream.formats = SNDRV_PCM_FMTBIT_S16_LE;
    dais[i].ops = &qcom_swrm_pdm_dai_ops;
    dais[i].id = i;
    }
    return devm_snd_soc_register_component(ctrl.dev,
    &qcom_swrm_dai_component,
    dais, num_dais);
    }
#[no_mangle]
unsafe extern "C" fn qcom_swrm_get_port_config(ctrl: *mut qcom_swrm_ctrl) -> c_int {
    static int qcom_swrm_get_port_config(struct qcom_swrm_ctrl *ctrl)
    {
    struct device_node *np = ctrl.dev.of_node;
    struct qcom_swrm_port_config *pcfg;
    int i, ret, val;
    ctrl.reg_read(ctrl, SWRM_COMP_PARAMS, &val);
    ctrl.num_dout_ports = FIELD_GET(SWRM_COMP_PARAMS_DOUT_PORTS_MASK, val);
    ctrl.num_din_ports = FIELD_GET(SWRM_COMP_PARAMS_DIN_PORTS_MASK, val);
    ret = of_property_read_u32(np, "qcom,din-ports", &val);
    if (!ret) { /* only if present */
    if (val != ctrl.num_din_ports) {
    dev_err(ctrl.dev, "din-ports (%d) mismatch with controller (%d)",
    val, ctrl.num_din_ports);
    }
    ctrl.num_din_ports = val;
    }
    ret = of_property_read_u32(np, "qcom,dout-ports", &val);
    if (!ret) { /* only if present */
    if (val != ctrl.num_dout_ports) {
    dev_err(ctrl.dev, "dout-ports (%d) mismatch with controller (%d)",
    val, ctrl.num_dout_ports);
    }
    ctrl.num_dout_ports = val;
    }
    ctrl.nports = ctrl.num_dout_ports + ctrl.num_din_ports;
    ctrl.pconfig = devm_kcalloc(ctrl.dev, ctrl.nports + 1,
    sizeof(*ctrl.pconfig), GFP_KERNEL);
    if (!ctrl.pconfig)
    return -ENOMEM;
    set_bit(0, &ctrl.port_mask);
// Valid port numbers are from 1, so mask out port 0 explicitly
    for (i = 0; i < ctrl.nports; i++) {
    pcfg = &ctrl.pconfig[i + 1];
    ret = of_property_read_u8_index(np, "qcom,ports-offset1", i, &pcfg.off1);
    if (ret)
    return ret;
    ret = of_property_read_u8_index(np, "qcom,ports-offset2", i, &pcfg.off2);
    if (ret)
    return ret;
    ret = of_property_read_u8_index(np, "qcom,ports-sinterval-low", i, (u8 *)&pcfg.si);
    if (ret) {
    ret = of_property_read_u16_index(np, "qcom,ports-sinterval", i, &pcfg.si);
    if (ret)
    return ret;
    }
    ret = of_property_read_u8_index(np, "qcom,ports-block-pack-mode",
    i, &pcfg.bp_mode);
    if (ret) {
    if (ctrl.version <= SWRM_VERSION_1_3_0)
    pcfg.bp_mode = SWR_INVALID_PARAM;
    else
    return ret;
    }
// Optional properties
    pcfg.hstart = SWR_INVALID_PARAM;
    pcfg.hstop = SWR_INVALID_PARAM;
    pcfg.word_length = SWR_INVALID_PARAM;
    pcfg.blk_group_count = SWR_INVALID_PARAM;
    pcfg.lane_control = SWR_INVALID_PARAM;
    of_property_read_u8_index(np, "qcom,ports-hstart", i, &pcfg.hstart);
    of_property_read_u8_index(np, "qcom,ports-hstop", i, &pcfg.hstop);
    of_property_read_u8_index(np, "qcom,ports-word-length", i, &pcfg.word_length);
    of_property_read_u8_index(np, "qcom,ports-block-group-count",
    i, &pcfg.blk_group_count);
    of_property_read_u8_index(np, "qcom,ports-lane-control", i, &pcfg.lane_control);
    }
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn swrm_reg_show(s_file: *mut seq_file, data: *mut c_void) -> c_int {
    static int swrm_reg_show(struct seq_file *s_file, void *data)
    {
    struct qcom_swrm_ctrl *ctrl = s_file.private;
    int reg, reg_val, ret;
    ret = pm_runtime_get_sync(ctrl.dev);
    if (ret < 0 && ret != -EACCES) {
    dev_err_ratelimited(ctrl.dev,
    "pm_runtime_get_sync failed in %s, ret %d\n",
    __func__, ret);
    pm_runtime_put_noidle(ctrl.dev);
    return ret;
    }
    for (reg = 0; reg <= ctrl.max_reg; reg += 4) {
    ctrl.reg_read(ctrl, reg, &reg_val);
    seq_printf(s_file, "0x%.3x: 0x%.2x\n", reg, reg_val);
    }
    pm_runtime_mark_last_busy(ctrl.dev);
    pm_runtime_put_autosuspend(ctrl.dev);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(swrm_reg);

#[no_mangle]
unsafe extern "C" fn qcom_swrm_probe(pdev: *mut platform_device) -> c_int {
    static int qcom_swrm_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct sdw_master_prop *prop;
    struct sdw_bus_params *params;
    struct qcom_swrm_ctrl *ctrl;
    const struct qcom_swrm_data *data;
    int ret;
    u32 val;
    ctrl = devm_kzalloc(dev, sizeof(*ctrl), GFP_KERNEL);
    if (!ctrl)
    return -ENOMEM;
    data = of_device_get_match_data(dev);
    ctrl.max_reg = data.max_reg;
    ctrl.reg_layout = data.reg_layout;
    ctrl.rows_index = sdw_find_row_index(data.default_rows);
    ctrl.cols_index = sdw_find_col_index(data.default_cols);

    if (dev.parent.bus == &slimbus_bus) {

    if (false) {

    ctrl.reg_read = qcom_swrm_ahb_reg_read;
    ctrl.reg_write = qcom_swrm_ahb_reg_write;
    ctrl.regmap = dev_get_regmap(dev.parent, core::ptr::null_mut());
    if (!ctrl.regmap)
    return -EINVAL;
    } else {
    ctrl.reg_read = qcom_swrm_cpu_reg_read;
    ctrl.reg_write = qcom_swrm_cpu_reg_write;
    ctrl.mmio = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(ctrl.mmio))
    return PTR_ERR(ctrl.mmio);
    }
    if (data.sw_clk_gate_required) {
    ctrl.audio_cgcr = devm_reset_control_get_optional_exclusive(dev, "swr_audio_cgcr");
    if (IS_ERR(ctrl.audio_cgcr)) {
    dev_err(dev, "Failed to get cgcr reset ctrl required for SW gating\n");
    ret = PTR_ERR(ctrl.audio_cgcr);
    goto err_init;
    }
    }
    ctrl.irq = of_irq_get(dev.of_node, 0);
    if (ctrl.irq < 0) {
    ret = ctrl.irq;
    goto err_init;
    }
    ctrl.hclk = devm_clk_get(dev, "iface");
    if (IS_ERR(ctrl.hclk)) {
    ret = dev_err_probe(dev, PTR_ERR(ctrl.hclk), "unable to get iface clock\n");
    goto err_init;
    }
    clk_prepare_enable(ctrl.hclk);
    ctrl.dev = dev;
    dev_set_drvdata(&pdev.dev, ctrl);
    mutex_init(&ctrl.port_lock);
    init_completion(&ctrl.broadcast);
    init_completion(&ctrl.enumeration);
    ctrl.bus.ops = &qcom_swrm_ops;
    ctrl.bus.port_ops = &qcom_swrm_port_ops;
    ctrl.bus.compute_params = &qcom_swrm_compute_params;
    ctrl.bus.clk_stop_timeout = 300;
    ret = qcom_swrm_get_port_config(ctrl);
    if (ret)
    goto err_clk;
    params = &ctrl.bus.params;
    params.max_dr_freq = DEFAULT_CLK_FREQ;
    params.curr_dr_freq = DEFAULT_CLK_FREQ;
    params.col = data.default_cols;
    params.row = data.default_rows;
    ctrl.reg_read(ctrl, SWRM_MCP_STATUS, &val);
    params.curr_bank = val & SWRM_MCP_STATUS_BANK_NUM_MASK;
    params.next_bank = !params.curr_bank;
    prop = &ctrl.bus.prop;
    prop.max_clk_freq = DEFAULT_CLK_FREQ;
    prop.mclk_freq = DEFAULT_CLK_FREQ;
    prop.num_clk_gears = 0;
    prop.num_clk_freq = MAX_FREQ_NUM;
    prop.clk_freq = &qcom_swrm_freq_tbl[0];
    prop.default_col = data.default_cols;
    prop.default_row = data.default_rows;
    ctrl.reg_read(ctrl, SWRM_COMP_HW_VERSION, &ctrl.version);
    ret = devm_request_threaded_irq(dev, ctrl.irq, core::ptr::null_mut(),
    qcom_swrm_irq_handler,
    IRQF_TRIGGER_RISING |
    IRQF_ONESHOT,
    "soundwire", ctrl);
    if (ret) {
    dev_err(dev, "Failed to request soundwire irq\n");
    goto err_clk;
    }
    ctrl.wake_irq = of_irq_get(dev.of_node, 1);
    if (ctrl.wake_irq > 0) {
    ret = devm_request_threaded_irq(dev, ctrl.wake_irq, core::ptr::null_mut(),
    qcom_swrm_wake_irq_handler,
    IRQF_TRIGGER_HIGH | IRQF_ONESHOT,
    "swr_wake_irq", ctrl);
    if (ret) {
    dev_err(dev, "Failed to request soundwire wake irq\n");
    goto err_init;
    }
    }
    ctrl.bus.controller_id = -1;
    if (ctrl.version > SWRM_VERSION_1_3_0) {
    ctrl.reg_read(ctrl, SWRM_COMP_MASTER_ID, &val);
    ctrl.bus.controller_id = val;
    }
    ret = sdw_bus_master_add(&ctrl.bus, dev, dev.fwnode);
    if (ret) {
    dev_err(dev, "Failed to register Soundwire controller (%d)\n",
    ret);
    goto err_clk;
    }
    qcom_swrm_init(ctrl);
    wait_for_completion_timeout(&ctrl.enumeration,
    msecs_to_jiffies(TIMEOUT_MS));
    ret = qcom_swrm_register_dais(ctrl);
    if (ret)
    goto err_master_add;
    dev_dbg(dev, "Qualcomm Soundwire controller v%x.%x.%x registered\n",
    (ctrl.version >> 24) & 0xff, (ctrl.version >> 16) & 0xff,
    ctrl.version & 0xffff);
    pm_runtime_set_autosuspend_delay(dev, 3000);
    pm_runtime_use_autosuspend(dev);
    pm_runtime_mark_last_busy(dev);
    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);

    ctrl.debugfs = debugfs_create_dir("qualcomm-sdw", ctrl.bus.debugfs);
    debugfs_create_file("qualcomm-registers", 0400, ctrl.debugfs, ctrl,
    &swrm_reg_fops);

    return 0;
    err_master_add:
    sdw_bus_master_delete(&ctrl.bus);
    err_clk:
    clk_disable_unprepare(ctrl.hclk);
    err_init:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qcom_swrm_remove(pdev: *mut platform_device) {
    static void qcom_swrm_remove(struct platform_device *pdev)
    {
    struct qcom_swrm_ctrl *ctrl = dev_get_drvdata(&pdev.dev);
    sdw_bus_master_delete(&ctrl.bus);
    clk_disable_unprepare(ctrl.hclk);
    }
#[no_mangle]
unsafe extern "C" fn swrm_runtime_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused swrm_runtime_resume(struct device *dev)
    {
    struct qcom_swrm_ctrl *ctrl = dev_get_drvdata(dev);
    int ret;
    if (ctrl.wake_irq > 0) {
    if (!irqd_irq_disabled(irq_get_irq_data(ctrl.wake_irq)))
    disable_irq_nosync(ctrl.wake_irq);
    }
    clk_prepare_enable(ctrl.hclk);
    if (ctrl.clock_stop_not_supported) {
    reinit_completion(&ctrl.enumeration);
    ctrl.reg_write(ctrl, SWRM_COMP_SW_RESET, 0x01);
    usleep_range(100, 105);
    qcom_swrm_init(ctrl);
    usleep_range(100, 105);
    if (!swrm_wait_for_frame_gen_enabled(ctrl))
    dev_err(ctrl.dev, "link failed to connect\n");
// wait for hw enumeration to complete
    wait_for_completion_timeout(&ctrl.enumeration,
    msecs_to_jiffies(TIMEOUT_MS));
    qcom_swrm_get_device_status(ctrl);
    sdw_handle_slave_status(&ctrl.bus, ctrl.status);
    } else {
    reset_control_reset(ctrl.audio_cgcr);
    if (ctrl.version == SWRM_VERSION_1_7_0) {
    ctrl.reg_write(ctrl, SWRM_LINK_MANAGER_EE, SWRM_EE_CPU);
    ctrl.reg_write(ctrl, SWRM_MCP_BUS_CTRL,
    SWRM_MCP_BUS_CLK_START << SWRM_EE_CPU);
    } else if (ctrl.version >= SWRM_VERSION_2_0_0) {
    ctrl.reg_write(ctrl, SWRM_LINK_MANAGER_EE, SWRM_EE_CPU);
    ctrl.reg_write(ctrl, SWRM_V2_0_CLK_CTRL,
    SWRM_V2_0_CLK_CTRL_CLK_START);
    } else {
    ctrl.reg_write(ctrl, SWRM_MCP_BUS_CTRL, SWRM_MCP_BUS_CLK_START);
    }
    ctrl.reg_write(ctrl, ctrl.reg_layout[SWRM_REG_INTERRUPT_CLEAR],
    SWRM_INTERRUPT_STATUS_MASTER_CLASH_DET);
    ctrl.intr_mask |= SWRM_INTERRUPT_STATUS_MASTER_CLASH_DET;
    if (ctrl.version < SWRM_VERSION_2_0_0)
    ctrl.reg_write(ctrl,
    ctrl.reg_layout[SWRM_REG_INTERRUPT_MASK_ADDR],
    ctrl.intr_mask);
    ctrl.reg_write(ctrl, ctrl.reg_layout[SWRM_REG_INTERRUPT_CPU_EN],
    ctrl.intr_mask);
    usleep_range(100, 105);
    if (!swrm_wait_for_frame_gen_enabled(ctrl))
    dev_err(ctrl.dev, "link failed to connect\n");
    ret = sdw_bus_exit_clk_stop(&ctrl.bus);
    if (ret < 0)
    dev_err(ctrl.dev, "bus failed to exit clock stop %d\n", ret);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn swrm_runtime_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused swrm_runtime_suspend(struct device *dev)
    {
    struct qcom_swrm_ctrl *ctrl = dev_get_drvdata(dev);
    int ret;
    swrm_wait_for_wr_fifo_done(ctrl);
    if (!ctrl.clock_stop_not_supported) {
// Mask bus clash interrupt
    ctrl.intr_mask &= ~SWRM_INTERRUPT_STATUS_MASTER_CLASH_DET;
    if (ctrl.version < SWRM_VERSION_2_0_0)
    ctrl.reg_write(ctrl,
    ctrl.reg_layout[SWRM_REG_INTERRUPT_MASK_ADDR],
    ctrl.intr_mask);
    ctrl.reg_write(ctrl, ctrl.reg_layout[SWRM_REG_INTERRUPT_CPU_EN],
    ctrl.intr_mask);
// Prepare slaves for clock stop
    ret = sdw_bus_prep_clk_stop(&ctrl.bus);
    if (ret < 0 && ret != -ENODATA) {
    dev_err(dev, "prepare clock stop failed %d", ret);
    return ret;
    }
    ret = sdw_bus_clk_stop(&ctrl.bus);
    if (ret < 0 && ret != -ENODATA) {
    dev_err(dev, "bus clock stop failed %d", ret);
    return ret;
    }
    }
    clk_disable_unprepare(ctrl.hclk);
    usleep_range(300, 305);
    if (ctrl.wake_irq > 0) {
    if (irqd_irq_disabled(irq_get_irq_data(ctrl.wake_irq)))
    enable_irq(ctrl.wake_irq);
    }
    return 0;
    }
    static const struct dev_pm_ops swrm_dev_pm_ops = {
    SET_RUNTIME_PM_OPS(swrm_runtime_suspend, swrm_runtime_resume, core::ptr::null_mut())
    };
    static const struct of_device_id qcom_swrm_of_match[] = {
    { .compatible = "qcom,soundwire-v1.3.0", .data = &swrm_v1_3_data },
    { .compatible = "qcom,soundwire-v1.5.1", .data = &swrm_v1_5_data },
    { .compatible = "qcom,soundwire-v1.6.0", .data = &swrm_v1_6_data },
    { .compatible = "qcom,soundwire-v1.7.0", .data = &swrm_v1_5_data },
    { .compatible = "qcom,soundwire-v2.0.0", .data = &swrm_v2_0_data },
    { .compatible = "qcom,soundwire-v3.1.0", .data = &swrm_v3_0_data },
    {/* sentinel */},
    };
    MODULE_DEVICE_TABLE(of, qcom_swrm_of_match);
    static struct platform_driver qcom_swrm_driver = {
    .probe	= &qcom_swrm_probe,
    .remove = qcom_swrm_remove,
    .driver = {
    .name	= "qcom-soundwire",
    .of_match_table = qcom_swrm_of_match,
    .pm = &swrm_dev_pm_ops,
    }
    };
    module_platform_driver(qcom_swrm_driver);
    MODULE_DESCRIPTION("Qualcomm soundwire driver");
    MODULE_LICENSE("GPL v2");
