//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/hisilicon/hibmc/dp/dp_aux.c
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
// Copyright (c) 2024 Hisilicon Limited.

pub const HIBMC_BYTES_IN_U32: c_int = 4;
pub const HIBMC_AUX_I2C_WRITE_SUCCESS: c_uint = 0x1;
pub const HIBMC_DP_MIN_PULSE_NUM: c_uint = 0x9;
pub const BITS_IN_U8: c_int = 8;
#[no_mangle]
pub unsafe extern "C" fn hibmc_dp_aux_reset(dp: *mut hibmc_dp_dev) {
    static inline void hibmc_dp_aux_reset(struct hibmc_dp_dev *dp)
    {
    hibmc_dp_reg_write_field(dp, HIBMC_DP_DPTX_RST_CTRL, HIBMC_DP_CFG_AUX_RST_N, 0x0);
    usleep_range(10, 15);
    hibmc_dp_reg_write_field(dp, HIBMC_DP_DPTX_RST_CTRL, HIBMC_DP_CFG_AUX_RST_N, 0x1);
    }
#[no_mangle]
unsafe extern "C" fn hibmc_dp_aux_read_data(dp: *mut hibmc_dp_dev, buf: *mut u8, size: u8) {
    static void hibmc_dp_aux_read_data(struct hibmc_dp_dev *dp, u8 *buf, u8 size)
    {
    u32 reg_num;
    u32 value;
    u32 num;
    u8 i, j;
    reg_num = DIV_ROUND_UP(size, HIBMC_BYTES_IN_U32);
    for (i = 0; i < reg_num; i++) {
// number of bytes read from a single register
    num = min(size - i * HIBMC_BYTES_IN_U32, HIBMC_BYTES_IN_U32);
    value = readl(dp.base + HIBMC_DP_AUX_RD_DATA0 + i * HIBMC_BYTES_IN_U32);
// convert the 32-bit value of the register to the buffer.
    for (j = 0; j < num; j++)
    buf[i * HIBMC_BYTES_IN_U32 + j] = value >> (j * BITS_IN_U8);
    }
    }
#[no_mangle]
unsafe extern "C" fn hibmc_dp_aux_write_data(dp: *mut hibmc_dp_dev, buf: *mut u8, size: u8) {
    static void hibmc_dp_aux_write_data(struct hibmc_dp_dev *dp, u8 *buf, u8 size)
    {
    u32 reg_num;
    u32 value;
    u32 num;
    u8 i, j;
    reg_num = DIV_ROUND_UP(size, HIBMC_BYTES_IN_U32);
    for (i = 0; i < reg_num; i++) {
// number of bytes written to a single register
    num = min_t(u8, size - i * HIBMC_BYTES_IN_U32, HIBMC_BYTES_IN_U32);
    value = 0;
// obtain the 32-bit value written to a single register.
    for (j = 0; j < num; j++)
    value |= buf[i * HIBMC_BYTES_IN_U32 + j] << (j * BITS_IN_U8);
// writing data to a single register
    writel(value, dp.base + HIBMC_DP_AUX_WR_DATA0 + i * HIBMC_BYTES_IN_U32);
    }
    }
#[no_mangle]
unsafe extern "C" fn hibmc_dp_aux_build_cmd(msg: *const drm_dp_aux_msg) -> u32 {
    static u32 hibmc_dp_aux_build_cmd(const struct drm_dp_aux_msg *msg)
    {
    let mut aux_cmd: u32 = msg.request;
    if (msg.size)
    aux_cmd |= FIELD_PREP(HIBMC_AUX_CMD_REQ_LEN, (msg.size - 1));
    else
    aux_cmd |= FIELD_PREP(HIBMC_AUX_CMD_I2C_ADDR_ONLY, 1);
    aux_cmd |= FIELD_PREP(HIBMC_AUX_CMD_ADDR, msg.address);
    return aux_cmd;
    }
// ret >= 0, ret is size; ret < 0, ret is err code
#[no_mangle]
unsafe extern "C" fn hibmc_dp_aux_parse_xfer(dp: *mut hibmc_dp_dev, msg: *mut drm_dp_aux_msg) -> c_int {
    static int hibmc_dp_aux_parse_xfer(struct hibmc_dp_dev *dp, struct drm_dp_aux_msg *msg)
    {
    u32 buf_data_cnt;
    u32 aux_status;
    aux_status = readl(dp.base + HIBMC_DP_AUX_STATUS);
    msg.reply = FIELD_GET(HIBMC_DP_CFG_AUX_STATUS, aux_status);
    if (aux_status & HIBMC_DP_CFG_AUX_TIMEOUT)
    return -ETIMEDOUT;
// only address
    if (!msg.size)
    return 0;
    if (msg.reply != DP_AUX_NATIVE_REPLY_ACK)
    return -EIO;
    buf_data_cnt = FIELD_GET(HIBMC_DP_CFG_AUX_READY_DATA_BYTE, aux_status);
    switch (msg.request) {
    case DP_AUX_NATIVE_WRITE:
    return msg.size;
    case DP_AUX_I2C_WRITE | DP_AUX_I2C_MOT:
    if (buf_data_cnt == HIBMC_AUX_I2C_WRITE_SUCCESS)
    return msg.size;
    else
    return FIELD_GET(HIBMC_DP_CFG_AUX, aux_status);
    case DP_AUX_NATIVE_READ:
    case DP_AUX_I2C_READ | DP_AUX_I2C_MOT:
    buf_data_cnt--;
    if (buf_data_cnt != msg.size) {
// only the successful part of data is read
    return -EBUSY;
    }
// all data is successfully read
    hibmc_dp_aux_read_data(dp, msg.buffer, msg.size);
    return msg.size;
    default:
    return -EINVAL;
    }
    }
// ret >= 0 ,ret is size; ret < 0, ret is err code
#[no_mangle]
unsafe extern "C" fn hibmc_dp_aux_xfer(aux: *mut drm_dp_aux, msg: *mut drm_dp_aux_msg) -> isize {
    static ssize_t hibmc_dp_aux_xfer(struct drm_dp_aux *aux, struct drm_dp_aux_msg *msg)
    {
    struct hibmc_dp *dp_priv = container_of(aux, struct hibmc_dp, aux);
    struct hibmc_dp_dev *dp = dp_priv.dp_dev;
    u32 aux_cmd;
    int ret;
    u32 val; /* val will be assigned at the beginning of readl_poll_timeout function */
    writel(0, dp.base + HIBMC_DP_AUX_WR_DATA0);
    writel(0, dp.base + HIBMC_DP_AUX_WR_DATA1);
    writel(0, dp.base + HIBMC_DP_AUX_WR_DATA2);
    writel(0, dp.base + HIBMC_DP_AUX_WR_DATA3);
    hibmc_dp_aux_write_data(dp, msg.buffer, msg.size);
    aux_cmd = hibmc_dp_aux_build_cmd(msg);
    writel(aux_cmd, dp.base + HIBMC_DP_AUX_CMD_ADDR);
// enable aux transfer
    hibmc_dp_reg_write_field(dp, HIBMC_DP_AUX_REQ, HIBMC_DP_CFG_AUX_REQ, 0x1);
    ret = readl_poll_timeout(dp.base + HIBMC_DP_AUX_REQ, val,
    !(val & HIBMC_DP_CFG_AUX_REQ), 50, 5000);
    if (ret) {
    hibmc_dp_aux_reset(dp);
    return ret;
    }
    return hibmc_dp_aux_parse_xfer(dp, msg);
    }
#[no_mangle]
pub unsafe extern "C" fn hibmc_dp_aux_init(dp: *mut hibmc_dp) {
    void hibmc_dp_aux_init(struct hibmc_dp *dp)
    {
    hibmc_dp_reg_write_field(dp.dp_dev, HIBMC_DP_AUX_REQ, HIBMC_DP_CFG_AUX_SYNC_LEN_SEL, 0x0);
    hibmc_dp_reg_write_field(dp.dp_dev, HIBMC_DP_AUX_REQ, HIBMC_DP_CFG_AUX_TIMER_TIMEOUT, 0x1);
    hibmc_dp_reg_write_field(dp.dp_dev, HIBMC_DP_AUX_REQ, HIBMC_DP_CFG_AUX_MIN_PULSE_NUM,
    HIBMC_DP_MIN_PULSE_NUM);
    dp.aux.transfer = hibmc_dp_aux_xfer;
    dp.aux.name = "HIBMC DRM dp aux";
    dp.aux.drm_dev = dp.drm_dev;
    drm_dp_aux_init(&dp.aux);
    dp.dp_dev.aux = &dp.aux;
    }
