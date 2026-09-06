//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-fsi.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// FSI-attached I2C controller algorithm
//
// Copyright 2018 IBM Corporation
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//

pub const FSI_ENGID_I2C: c_uint = 0x7;
pub const I2C_DEFAULT_CLK_DIV: c_int = 6;
// i2c registers
pub const I2C_FSI_FIFO: c_uint = 0x00;
pub const I2C_FSI_CMD: c_uint = 0x04;
pub const I2C_FSI_MODE: c_uint = 0x08;
pub const I2C_FSI_WATER_MARK: c_uint = 0x0C;
pub const I2C_FSI_INT_MASK: c_uint = 0x10;
pub const I2C_FSI_INT_COND: c_uint = 0x14;
pub const I2C_FSI_OR_INT_MASK: c_uint = 0x14;
pub const I2C_FSI_INTS: c_uint = 0x18;
pub const I2C_FSI_AND_INT_MASK: c_uint = 0x18;
pub const I2C_FSI_STAT: c_uint = 0x1C;
pub const I2C_FSI_RESET_I2C: c_uint = 0x1C;
pub const I2C_FSI_ESTAT: c_uint = 0x20;
pub const I2C_FSI_RESET_ERR: c_uint = 0x20;
pub const I2C_FSI_RESID_LEN: c_uint = 0x24;
pub const I2C_FSI_SET_SCL: c_uint = 0x24;
pub const I2C_FSI_PORT_BUSY: c_uint = 0x28;
pub const I2C_FSI_RESET_SCL: c_uint = 0x2C;
pub const I2C_FSI_SET_SDA: c_uint = 0x30;
pub const I2C_FSI_RESET_SDA: c_uint = 0x34;
// cmd register

// mode register

// watermark register

pub const I2C_FIFO_HI_LVL: c_int = 4;
pub const I2C_FIFO_LO_LVL: c_int = 4;
// interrupt register

// status register

    I2C_STAT_PARITY |			\
    I2C_STAT_BE_OVERRUN |			\
    I2C_STAT_BE_ACCESS |			\
    I2C_STAT_LOST_ARB |			\
    I2C_STAT_NACK |			\
    I2C_STAT_STOP_ERR)

    I2C_STAT_DAT_REQ |			\
    I2C_STAT_CMD_COMP)
// extended status register

// port busy register

// wait for command complete or data request
pub const I2C_CMD_SLEEP_MAX_US: c_int = 500;
pub const I2C_CMD_SLEEP_MIN_US: c_int = 50;
// wait after reset; choose time from legacy driver
pub const I2C_RESET_SLEEP_MAX_US: c_int = 2000;
pub const I2C_RESET_SLEEP_MIN_US: c_int = 1000;
// choose timeout length from legacy driver; it's well tested

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsi_i2c_ctrl {
    pub fsi: *mut fsi_device,
    pub fifo_size: u8,
    pub ports: list_head,
    pub lock: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsi_i2c_port {
    pub list: list_head,
    pub adapter: i2c_adapter,
    pub ctrl: *mut fsi_i2c_ctrl,
    pub port: u16,
    pub xfrd: u16,
}

    static int fsi_i2c_read_reg(struct fsi_device *fsi, unsigned int reg,
    u32 *data)
    {
    int rc;
    __be32 data_be;
    rc = fsi_device_read(fsi, reg, &data_be, sizeof(data_be));
    if (rc)
    return rc;
// data = be32_to_cpu(data_be);
    return 0;
    }
    static int fsi_i2c_write_reg(struct fsi_device *fsi, unsigned int reg,
    u32 *data)
    {
    let mut data_be: __be32 = cpu_to_be32p(data);
    return fsi_device_write(fsi, reg, &data_be, sizeof(data_be));
    }
#[no_mangle]
unsafe extern "C" fn fsi_i2c_dev_init(i2c: *mut fsi_i2c_ctrl) -> c_int {
    static int fsi_i2c_dev_init(struct fsi_i2c_ctrl *i2c)
    {
    int rc;
    let mut mode: u32 = I2C_MODE_ENHANCED, extended_status, watermark;
    let mut interrupt: u32 = 0;
// since we use polling, disable interrupts
    rc = fsi_i2c_write_reg(i2c.fsi, I2C_FSI_INT_MASK, &interrupt);
    if (rc)
    return rc;
    mode |= FIELD_PREP(I2C_MODE_CLKDIV, I2C_DEFAULT_CLK_DIV);
    rc = fsi_i2c_write_reg(i2c.fsi, I2C_FSI_MODE, &mode);
    if (rc)
    return rc;
    rc = fsi_i2c_read_reg(i2c.fsi, I2C_FSI_ESTAT, &extended_status);
    if (rc)
    return rc;
    i2c.fifo_size = FIELD_GET(I2C_ESTAT_FIFO_SZ, extended_status);
    watermark = FIELD_PREP(I2C_WATERMARK_HI,
    i2c.fifo_size - I2C_FIFO_HI_LVL);
    watermark |= FIELD_PREP(I2C_WATERMARK_LO, I2C_FIFO_LO_LVL);
    return fsi_i2c_write_reg(i2c.fsi, I2C_FSI_WATER_MARK, &watermark);
    }
#[no_mangle]
unsafe extern "C" fn fsi_i2c_set_port(port: *mut fsi_i2c_port) -> c_int {
    static int fsi_i2c_set_port(struct fsi_i2c_port *port)
    {
    int rc;
    struct fsi_device *fsi = port.ctrl.fsi;
    u32 mode, dummy = 0;
    rc = fsi_i2c_read_reg(fsi, I2C_FSI_MODE, &mode);
    if (rc)
    return rc;
    if (FIELD_GET(I2C_MODE_PORT, mode) == port.port)
    return 0;
    mode = (mode & ~I2C_MODE_PORT) | FIELD_PREP(I2C_MODE_PORT, port.port);
    rc = fsi_i2c_write_reg(fsi, I2C_FSI_MODE, &mode);
    if (rc)
    return rc;
// reset engine when port is changed
    return fsi_i2c_write_reg(fsi, I2C_FSI_RESET_ERR, &dummy);
    }
    static int fsi_i2c_start(struct fsi_i2c_port *port, struct i2c_msg *msg,
    bool stop)
    {
    struct fsi_i2c_ctrl *i2c = port.ctrl;
    let mut cmd: u32 = I2C_CMD_WITH_START | I2C_CMD_WITH_ADDR;
    port.xfrd = 0;
    if (msg.flags & I2C_M_RD)
    cmd |= I2C_CMD_READ;
    if (stop || msg.flags & I2C_M_STOP)
    cmd |= I2C_CMD_WITH_STOP;
    cmd |= FIELD_PREP(I2C_CMD_ADDR, msg.addr);
    cmd |= FIELD_PREP(I2C_CMD_LEN, msg.len);
    return fsi_i2c_write_reg(i2c.fsi, I2C_FSI_CMD, &cmd);
    }
#[no_mangle]
unsafe extern "C" fn fsi_i2c_get_op_bytes(op_bytes: c_int) -> c_int {
    static int fsi_i2c_get_op_bytes(int op_bytes)
    {
// fsi is limited to max 4 byte aligned ops
    if (op_bytes > 4)
    return 4;
#[no_mangle]
pub unsafe extern "C" fn if(3: op_bytes ==) -> else {
    else if (op_bytes == 3)
    return 2;
    return op_bytes;
    }
    static int fsi_i2c_write_fifo(struct fsi_i2c_port *port, struct i2c_msg *msg,
    u8 fifo_count)
    {
    int write;
    int rc;
    struct fsi_i2c_ctrl *i2c = port.ctrl;
    let mut bytes_to_write: c_int = i2c.fifo_size - fifo_count;
    let mut bytes_remaining: c_int = msg.len - port.xfrd;
    bytes_to_write = min(bytes_to_write, bytes_remaining);
    while (bytes_to_write) {
    write = fsi_i2c_get_op_bytes(bytes_to_write);
    rc = fsi_device_write(i2c.fsi, I2C_FSI_FIFO,
    &msg.buf[port.xfrd], write);
    if (rc)
    return rc;
    port.xfrd += write;
    bytes_to_write -= write;
    }
    return 0;
    }
    static int fsi_i2c_read_fifo(struct fsi_i2c_port *port, struct i2c_msg *msg,
    u8 fifo_count)
    {
    int read;
    int rc;
    struct fsi_i2c_ctrl *i2c = port.ctrl;
    int bytes_to_read;
    let mut xfr_remaining: c_int = msg.len - port.xfrd;
    u32 dummy;
    bytes_to_read = min_t(int, fifo_count, xfr_remaining);
    while (bytes_to_read) {
    read = fsi_i2c_get_op_bytes(bytes_to_read);
    if (xfr_remaining) {
    rc = fsi_device_read(i2c.fsi, I2C_FSI_FIFO,
    &msg.buf[port.xfrd], read);
    if (rc)
    return rc;
    port.xfrd += read;
    xfr_remaining -= read;
    } else {
// no more buffer but data in fifo, need to clear it
    rc = fsi_device_read(i2c.fsi, I2C_FSI_FIFO, &dummy,
    read);
    if (rc)
    return rc;
    }
    bytes_to_read -= read;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fsi_i2c_get_scl(adap: *mut i2c_adapter) -> c_int {
    static int fsi_i2c_get_scl(struct i2c_adapter *adap)
    {
    let mut stat: u32 = 0;
    struct fsi_i2c_port *port = adap.algo_data;
    struct fsi_i2c_ctrl *i2c = port.ctrl;
    fsi_i2c_read_reg(i2c.fsi, I2C_FSI_STAT, &stat);
    return !!(stat & I2C_STAT_SCL_IN);
    }
#[no_mangle]
unsafe extern "C" fn fsi_i2c_set_scl(adap: *mut i2c_adapter, val: c_int) {
    static void fsi_i2c_set_scl(struct i2c_adapter *adap, int val)
    {
    let mut dummy: u32 = 0;
    struct fsi_i2c_port *port = adap.algo_data;
    struct fsi_i2c_ctrl *i2c = port.ctrl;
    if (val)
    fsi_i2c_write_reg(i2c.fsi, I2C_FSI_SET_SCL, &dummy);
    else
    fsi_i2c_write_reg(i2c.fsi, I2C_FSI_RESET_SCL, &dummy);
    }
#[no_mangle]
unsafe extern "C" fn fsi_i2c_get_sda(adap: *mut i2c_adapter) -> c_int {
    static int fsi_i2c_get_sda(struct i2c_adapter *adap)
    {
    let mut stat: u32 = 0;
    struct fsi_i2c_port *port = adap.algo_data;
    struct fsi_i2c_ctrl *i2c = port.ctrl;
    fsi_i2c_read_reg(i2c.fsi, I2C_FSI_STAT, &stat);
    return !!(stat & I2C_STAT_SDA_IN);
    }
#[no_mangle]
unsafe extern "C" fn fsi_i2c_set_sda(adap: *mut i2c_adapter, val: c_int) {
    static void fsi_i2c_set_sda(struct i2c_adapter *adap, int val)
    {
    let mut dummy: u32 = 0;
    struct fsi_i2c_port *port = adap.algo_data;
    struct fsi_i2c_ctrl *i2c = port.ctrl;
    if (val)
    fsi_i2c_write_reg(i2c.fsi, I2C_FSI_SET_SDA, &dummy);
    else
    fsi_i2c_write_reg(i2c.fsi, I2C_FSI_RESET_SDA, &dummy);
    }
#[no_mangle]
unsafe extern "C" fn fsi_i2c_prepare_recovery(adap: *mut i2c_adapter) {
    static void fsi_i2c_prepare_recovery(struct i2c_adapter *adap)
    {
    int rc;
    u32 mode;
    struct fsi_i2c_port *port = adap.algo_data;
    struct fsi_i2c_ctrl *i2c = port.ctrl;
    rc = fsi_i2c_read_reg(i2c.fsi, I2C_FSI_MODE, &mode);
    if (rc)
    return;
    mode |= I2C_MODE_DIAG;
    fsi_i2c_write_reg(i2c.fsi, I2C_FSI_MODE, &mode);
    }
#[no_mangle]
unsafe extern "C" fn fsi_i2c_unprepare_recovery(adap: *mut i2c_adapter) {
    static void fsi_i2c_unprepare_recovery(struct i2c_adapter *adap)
    {
    int rc;
    u32 mode;
    struct fsi_i2c_port *port = adap.algo_data;
    struct fsi_i2c_ctrl *i2c = port.ctrl;
    rc = fsi_i2c_read_reg(i2c.fsi, I2C_FSI_MODE, &mode);
    if (rc)
    return;
    mode &= ~I2C_MODE_DIAG;
    fsi_i2c_write_reg(i2c.fsi, I2C_FSI_MODE, &mode);
    }
    static int fsi_i2c_reset_bus(struct fsi_i2c_ctrl *i2c,
    struct fsi_i2c_port *port)
    {
    int rc;
    u32 stat, dummy = 0;
// force bus reset, ignore errors
    i2c_recover_bus(&port.adapter);
// reset errors
    rc = fsi_i2c_write_reg(i2c.fsi, I2C_FSI_RESET_ERR, &dummy);
    if (rc)
    return rc;
// wait for command complete
    usleep_range(I2C_RESET_SLEEP_MIN_US, I2C_RESET_SLEEP_MAX_US);
    rc = fsi_i2c_read_reg(i2c.fsi, I2C_FSI_STAT, &stat);
    if (rc)
    return rc;
    if (stat & I2C_STAT_CMD_COMP)
    return 0;
// failed to get command complete; reset engine again
    rc = fsi_i2c_write_reg(i2c.fsi, I2C_FSI_RESET_I2C, &dummy);
    if (rc)
    return rc;
// re-init engine again
    return fsi_i2c_dev_init(i2c);
    }
#[no_mangle]
unsafe extern "C" fn fsi_i2c_reset_engine(i2c: *mut fsi_i2c_ctrl, port: u16) -> c_int {
    static int fsi_i2c_reset_engine(struct fsi_i2c_ctrl *i2c, u16 port)
    {
    int rc;
    u32 mode, dummy = 0;
// reset engine
    rc = fsi_i2c_write_reg(i2c.fsi, I2C_FSI_RESET_I2C, &dummy);
    if (rc)
    return rc;
// re-init engine
    rc = fsi_i2c_dev_init(i2c);
    if (rc)
    return rc;
    rc = fsi_i2c_read_reg(i2c.fsi, I2C_FSI_MODE, &mode);
    if (rc)
    return rc;
// set port; default after reset is 0
    if (port) {
    mode &= ~I2C_MODE_PORT;
    mode |= FIELD_PREP(I2C_MODE_PORT, port);
    rc = fsi_i2c_write_reg(i2c.fsi, I2C_FSI_MODE, &mode);
    if (rc)
    return rc;
    }
// reset busy register; hw workaround
    dummy = I2C_PORT_BUSY_RESET;
    rc = fsi_i2c_write_reg(i2c.fsi, I2C_FSI_PORT_BUSY, &dummy);
    if (rc)
    return rc;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fsi_i2c_abort(port: *mut fsi_i2c_port, status: u32) -> c_int {
    static int fsi_i2c_abort(struct fsi_i2c_port *port, u32 status)
    {
    int rc;
    unsigned long start;
    let mut cmd: u32 = I2C_CMD_WITH_STOP;
    u32 stat;
    struct fsi_i2c_ctrl *i2c = port.ctrl;
    struct fsi_device *fsi = i2c.fsi;
    rc = fsi_i2c_reset_engine(i2c, port.port);
    if (rc)
    return rc;
    rc = fsi_i2c_read_reg(fsi, I2C_FSI_STAT, &stat);
    if (rc)
    return rc;
// if sda is low, peform full bus reset
    if (!(stat & I2C_STAT_SDA_IN)) {
    rc = fsi_i2c_reset_bus(i2c, port);
    if (rc)
    return rc;
    }
// skip final stop command for these errors
    if (status & (I2C_STAT_PARITY | I2C_STAT_LOST_ARB | I2C_STAT_STOP_ERR))
    return 0;
// write stop command
    rc = fsi_i2c_write_reg(fsi, I2C_FSI_CMD, &cmd);
    if (rc)
    return rc;
// wait until we see command complete in the controller
    start = jiffies;
    do {
    rc = fsi_i2c_read_reg(fsi, I2C_FSI_STAT, &status);
    if (rc)
    return rc;
    if (status & I2C_STAT_CMD_COMP)
    return 0;
    usleep_range(I2C_CMD_SLEEP_MIN_US, I2C_CMD_SLEEP_MAX_US);
    } while (time_after(start + I2C_ABORT_TIMEOUT, jiffies));
    return -ETIMEDOUT;
    }
    static int fsi_i2c_handle_status(struct fsi_i2c_port *port,
    struct i2c_msg *msg, u32 status)
    {
    int rc;
    u8 fifo_count;
    if (status & I2C_STAT_ERR) {
    rc = fsi_i2c_abort(port, status);
    if (rc)
    return rc;
    if (status & I2C_STAT_INV_CMD)
    return -EINVAL;
    if (status & (I2C_STAT_PARITY | I2C_STAT_BE_OVERRUN |
    I2C_STAT_BE_ACCESS))
    return -EPROTO;
    if (status & I2C_STAT_NACK)
    return -ENXIO;
    if (status & I2C_STAT_LOST_ARB)
    return -EAGAIN;
    if (status & I2C_STAT_STOP_ERR)
    return -EBADMSG;
    return -EIO;
    }
    if (status & I2C_STAT_DAT_REQ) {
    fifo_count = FIELD_GET(I2C_STAT_FIFO_COUNT, status);
    if (msg.flags & I2C_M_RD)
    return fsi_i2c_read_fifo(port, msg, fifo_count);
    return fsi_i2c_write_fifo(port, msg, fifo_count);
    }
    if (status & I2C_STAT_CMD_COMP) {
    if (port.xfrd < msg.len)
    return -ENODATA;
    return msg.len;
    }
    return 0;
    }
    static int fsi_i2c_wait(struct fsi_i2c_port *port, struct i2c_msg *msg,
    unsigned long timeout)
    {
    let mut status: u32 = 0;
    int rc;
    let mut start: c_ulong = jiffies;
    do {
    rc = fsi_i2c_read_reg(port.ctrl.fsi, I2C_FSI_STAT,
    &status);
    if (rc)
    return rc;
    if (status & I2C_STAT_ANY_RESP) {
    rc = fsi_i2c_handle_status(port, msg, status);
    if (rc < 0)
    return rc;
// cmd complete and all data xfrd
    if (rc == msg.len)
    return 0;
// need to xfr more data, but maybe don't need wait
    continue;
    }
    usleep_range(I2C_CMD_SLEEP_MIN_US, I2C_CMD_SLEEP_MAX_US);
    } while (time_after(start + timeout, jiffies));
    return -ETIMEDOUT;
    }
    static int fsi_i2c_xfer(struct i2c_adapter *adap, struct i2c_msg *msgs,
    int num)
    {
    int i, rc;
    unsigned long start_time;
    struct fsi_i2c_port *port = adap.algo_data;
    struct fsi_i2c_ctrl *ctrl = port.ctrl;
    struct i2c_msg *msg;
    mutex_lock(&ctrl.lock);
    rc = fsi_i2c_set_port(port);
    if (rc)
    goto unlock;
    for (i = 0; i < num; i++) {
    msg = msgs + i;
    start_time = jiffies;
    rc = fsi_i2c_start(port, msg, i == num - 1);
    if (rc)
    goto unlock;
    rc = fsi_i2c_wait(port, msg,
    adap.timeout - (jiffies - start_time));
    if (rc)
    goto unlock;
    }
    unlock:
    mutex_unlock(&ctrl.lock);
    return rc ? : num;
    }
#[no_mangle]
unsafe extern "C" fn fsi_i2c_functionality(adap: *mut i2c_adapter) -> u32 {
    static u32 fsi_i2c_functionality(struct i2c_adapter *adap)
    {
    return I2C_FUNC_I2C | I2C_FUNC_PROTOCOL_MANGLING |
    I2C_FUNC_SMBUS_EMUL | I2C_FUNC_SMBUS_BLOCK_DATA;
    }
    static struct i2c_bus_recovery_info fsi_i2c_bus_recovery_info = {
    .recover_bus = i2c_generic_scl_recovery,
    .get_scl = fsi_i2c_get_scl,
    .set_scl = fsi_i2c_set_scl,
    .get_sda = fsi_i2c_get_sda,
    .set_sda = fsi_i2c_set_sda,
    .prepare_recovery = fsi_i2c_prepare_recovery,
    .unprepare_recovery = fsi_i2c_unprepare_recovery,
    };
    static const struct i2c_algorithm fsi_i2c_algorithm = {
    .xfer = fsi_i2c_xfer,
    .functionality = fsi_i2c_functionality,
    };
    static struct device_node *fsi_i2c_find_port_of_node(struct device_node *fsi,
    int port)
    {
    struct device_node *np;
    u32 port_no;
    int rc;
    for_each_child_of_node(fsi, np) {
    rc = of_property_read_u32(np, "reg", &port_no);
    if (!rc && port_no == port)
    return np;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn fsi_i2c_probe(fsi_dev: *mut fsi_device) -> c_int {
    static int fsi_i2c_probe(struct fsi_device *fsi_dev)
    {
    struct device *dev = &fsi_dev.dev;
    struct fsi_i2c_ctrl *i2c;
    struct fsi_i2c_port *port;
    struct device_node *np;
    u32 port_no, ports, stat;
    int rc;
    i2c = devm_kzalloc(dev, sizeof(*i2c), GFP_KERNEL);
    if (!i2c)
    return -ENOMEM;
    mutex_init(&i2c.lock);
    i2c.fsi = to_fsi_dev(dev);
    INIT_LIST_HEAD(&i2c.ports);
    rc = fsi_i2c_dev_init(i2c);
    if (rc)
    return rc;
    rc = fsi_i2c_read_reg(i2c.fsi, I2C_FSI_STAT, &stat);
    if (rc)
    return rc;
    ports = FIELD_GET(I2C_STAT_MAX_PORT, stat) + 1;
    dev_dbg(dev, "I2C controller has %d ports\n", ports);
    for (port_no = 0; port_no < ports; port_no++) {
    np = fsi_i2c_find_port_of_node(dev.of_node, port_no);
    if (!of_device_is_available(np))
    continue;
    port = kzalloc_obj(*port);
    if (!port) {
    of_node_put(np);
    break;
    }
    port.ctrl = i2c;
    port.port = port_no;
    port.adapter.owner = THIS_MODULE;
    port.adapter.dev.of_node = np;
    port.adapter.dev.parent = dev;
    port.adapter.algo = &fsi_i2c_algorithm;
    port.adapter.bus_recovery_info = &fsi_i2c_bus_recovery_info;
    port.adapter.algo_data = port;
    snprintf(port.adapter.name, sizeof(port.adapter.name),
    "i2c_bus-%u", port_no);
    rc = i2c_add_adapter(&port.adapter);
    if (rc < 0) {
    dev_err(dev, "Failed to register adapter: %d\n", rc);
    of_node_put(np);
    kfree(port);
    continue;
    }
    list_add(&port.list, &i2c.ports);
    }
    fsi_set_drvdata(fsi_dev, i2c);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fsi_i2c_remove(fsi_dev: *mut fsi_device) {
    static void fsi_i2c_remove(struct fsi_device *fsi_dev)
    {
    struct fsi_i2c_ctrl *i2c = fsi_get_drvdata(fsi_dev);
    struct fsi_i2c_port *port, *tmp;
    list_for_each_entry_safe(port, tmp, &i2c.ports, list) {
    list_del(&port.list);
    i2c_del_adapter(&port.adapter);
    kfree(port);
    }
    }
    static const struct fsi_device_id fsi_i2c_ids[] = {
    { FSI_ENGID_I2C, FSI_VERSION_ANY },
    { }
    };
    static struct fsi_driver fsi_i2c_driver = {
    .id_table = fsi_i2c_ids,
    .probe = fsi_i2c_probe,
    .remove = fsi_i2c_remove,
    .drv = {
    .name = "i2c-fsi",
    },
    };
    module_fsi_driver(fsi_i2c_driver);
    MODULE_AUTHOR("Eddie James <eajames@us.ibm.com>");
    MODULE_DESCRIPTION("FSI attached I2C controller");
    MODULE_LICENSE("GPL");
