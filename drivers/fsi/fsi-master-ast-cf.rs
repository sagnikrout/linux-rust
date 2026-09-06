//! Automatically rewritten from C to Rust
//! Source: drivers/fsi/fsi-master-ast-cf.c
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
// Copyright 2018 IBM Corp
//
// A FSI master based on Aspeed ColdFire coprocessor
//

// Common SCU based coprocessor control registers
pub const SCU_COPRO_CTRL: c_uint = 0x100;
pub const SCU_COPRO_RESET: c_uint = 0x00000002;
pub const SCU_COPRO_CLK_EN: c_uint = 0x00000001;
// AST2500 specific ones
pub const SCU_2500_COPRO_SEG0: c_uint = 0x104;
pub const SCU_2500_COPRO_SEG1: c_uint = 0x108;
pub const SCU_2500_COPRO_SEG2: c_uint = 0x10c;
pub const SCU_2500_COPRO_SEG3: c_uint = 0x110;
pub const SCU_2500_COPRO_SEG4: c_uint = 0x114;
pub const SCU_2500_COPRO_SEG5: c_uint = 0x118;
pub const SCU_2500_COPRO_SEG6: c_uint = 0x11c;
pub const SCU_2500_COPRO_SEG7: c_uint = 0x120;
pub const SCU_2500_COPRO_SEG8: c_uint = 0x124;
pub const SCU_2500_COPRO_SEG_SWAP: c_uint = 0x00000001;
pub const SCU_2500_COPRO_CACHE_CTL: c_uint = 0x128;
pub const SCU_2500_COPRO_CACHE_EN: c_uint = 0x00000001;
pub const SCU_2500_COPRO_SEG0_CACHE_EN: c_uint = 0x00000002;
pub const SCU_2500_COPRO_SEG1_CACHE_EN: c_uint = 0x00000004;
pub const SCU_2500_COPRO_SEG2_CACHE_EN: c_uint = 0x00000008;
pub const SCU_2500_COPRO_SEG3_CACHE_EN: c_uint = 0x00000010;
pub const SCU_2500_COPRO_SEG4_CACHE_EN: c_uint = 0x00000020;
pub const SCU_2500_COPRO_SEG5_CACHE_EN: c_uint = 0x00000040;
pub const SCU_2500_COPRO_SEG6_CACHE_EN: c_uint = 0x00000080;
pub const SCU_2500_COPRO_SEG7_CACHE_EN: c_uint = 0x00000100;
pub const SCU_2500_COPRO_SEG8_CACHE_EN: c_uint = 0x00000200;
pub const SCU_2400_COPRO_SEG0: c_uint = 0x104;
pub const SCU_2400_COPRO_SEG2: c_uint = 0x108;
pub const SCU_2400_COPRO_SEG4: c_uint = 0x10c;
pub const SCU_2400_COPRO_SEG6: c_uint = 0x110;
pub const SCU_2400_COPRO_SEG8: c_uint = 0x114;
pub const SCU_2400_COPRO_SEG_SWAP: c_uint = 0x80000000;
pub const SCU_2400_COPRO_CACHE_CTL: c_uint = 0x118;
pub const SCU_2400_COPRO_CACHE_EN: c_uint = 0x00000001;
pub const SCU_2400_COPRO_SEG0_CACHE_EN: c_uint = 0x00000002;
pub const SCU_2400_COPRO_SEG2_CACHE_EN: c_uint = 0x00000004;
pub const SCU_2400_COPRO_SEG4_CACHE_EN: c_uint = 0x00000008;
pub const SCU_2400_COPRO_SEG6_CACHE_EN: c_uint = 0x00000010;
pub const SCU_2400_COPRO_SEG8_CACHE_EN: c_uint = 0x00000020;
// CVIC registers
pub const CVIC_EN_REG: c_uint = 0x10;
pub const CVIC_TRIG_REG: c_uint = 0x18;
//
// System register base address (needed for configuring the
// coldfire maps)
//
pub const SYSREG_BASE: c_uint = 0x1e600000;
// Amount of SRAM required
pub const SRAM_SIZE: c_uint = 0x1000;
pub const LAST_ADDR_INVALID: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsi_master_acf {
    pub master: fsi_master,
    pub dev: *mut device,
    pub scu: *mut regmap,
    pub /: *mut *mut mutex lock; / mutex for command ordering,
    pub gpio_clk: *mut gpio_desc,
    pub gpio_data: *mut gpio_desc,
    pub /: *mut *mut *mut gpio_desc gpio_trans; / Voltage translator,
    pub /: *mut *mut *mut gpio_desc gpio_enable; / FSI enable,
    pub /: *mut *mut *mut gpio_desc gpio_mux; / Mux control,
    pub gpio_clk_vreg: u16,
    pub gpio_clk_dreg: u16,
    pub gpio_dat_vreg: u16,
    pub gpio_dat_dreg: u16,
    pub gpio_tra_vreg: u16,
    pub gpio_tra_dreg: u16,
    pub gpio_clk_bit: u8,
    pub gpio_dat_bit: u8,
    pub gpio_tra_bit: u8,
    pub cf_mem_addr: u32,
    pub cf_mem_size: usize,
    pub cf_mem: *mut void __iomem,
    pub cvic: *mut void __iomem,
    pub sram_pool: *mut gen_pool,
    pub sram: *mut void __iomem,
    pub is_ast2500: bool,
    pub external_mode: bool,
    pub trace_enabled: bool,
    pub last_addr: u32,
    pub t_send_delay: u8,
    pub t_echo_delay: u8,
    pub cvic_sw_irq: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsi_msg {
    pub msg: u64,
    pub bits: u8,
}

// Macro flag: #define CREATE_TRACE_POINTS

#[no_mangle]
unsafe extern "C" fn msg_push_bits(msg: *mut fsi_msg, data: u64, bits: c_int) {
    static void msg_push_bits(struct fsi_msg *msg, uint64_t data, int bits)
    {
    msg.msg <<= bits;
    msg.msg |= data & ((1ull << bits) - 1);
    msg.bits += bits;
    }
#[no_mangle]
unsafe extern "C" fn msg_push_crc(msg: *mut fsi_msg) {
    static void msg_push_crc(struct fsi_msg *msg)
    {
    uint8_t crc;
    int top;
    top = msg.bits & 0x3;
// start bit, and any non-aligned top bits
    crc = crc4(0, 1 << top | msg.msg >> (msg.bits - top), top + 1);
// aligned bits
    crc = crc4(crc, msg.msg, msg.bits - top);
    msg_push_bits(msg, crc, 4);
    }
#[no_mangle]
unsafe extern "C" fn msg_finish_cmd(cmd: *mut fsi_msg) {
    static void msg_finish_cmd(struct fsi_msg *cmd)
    {
// Left align message
    cmd.msg <<= (64 - cmd.bits);
    }
    static bool check_same_address(struct fsi_master_acf *master, int id,
    uint32_t addr)
    {
// this will also handle LAST_ADDR_INVALID
    return master.last_addr == (((id & 0x3) << 21) | (addr & ~0x3));
    }
    static bool check_relative_address(struct fsi_master_acf *master, int id,
    uint32_t addr, uint32_t *rel_addrp)
    {
    let mut last_addr: u32 = master.last_addr;
    int32_t rel_addr;
    if (last_addr == LAST_ADDR_INVALID)
    return false;
// We may be in 23-bit addressing mode, which uses the id as the
// top two address bits. So, if we're referencing a different ID,
// use absolute addresses.
//
    if (((last_addr >> 21) & 0x3) != id)
    return false;
// remove the top two bits from any 23-bit addressing
    last_addr &= (1 << 21) - 1;
// We know that the addresses are limited to 21 bits, so this won't
// overflow the signed rel_addr
    rel_addr = addr - last_addr;
    if (rel_addr > 255 || rel_addr < -256)
    return false;
// rel_addrp = (uint32_t)rel_addr;
    return true;
    }
    static void last_address_update(struct fsi_master_acf *master,
    int id, bool valid, uint32_t addr)
    {
    if (!valid)
    master.last_addr = LAST_ADDR_INVALID;
    else
    master.last_addr = ((id & 0x3) << 21) | (addr & ~0x3);
    }
//
// Encode an Absolute/Relative/Same Address command
//
    static void build_ar_command(struct fsi_master_acf *master,
    struct fsi_msg *cmd, uint8_t id,
    uint32_t addr, size_t size,
    const void *data)
    {
    int i, addr_bits, opcode_bits;
    let mut write: bool = !!data;
    uint8_t ds, opcode;
    uint32_t rel_addr;
    cmd.bits = 0;
    cmd.msg = 0;
// we have 21 bits of address max
    addr &= ((1 << 21) - 1);
// cmd opcodes are variable length - SAME_AR is only two bits
    opcode_bits = 3;
    if (check_same_address(master, id, addr)) {
// we still address the byte offset within the word
    addr_bits = 2;
    opcode_bits = 2;
    opcode = FSI_CMD_SAME_AR;
    trace_fsi_master_acf_cmd_same_addr(master);
    } else if (check_relative_address(master, id, addr, &rel_addr)) {
// 8 bits plus sign
    addr_bits = 9;
    addr = rel_addr;
    opcode = FSI_CMD_REL_AR;
    trace_fsi_master_acf_cmd_rel_addr(master, rel_addr);
    } else {
    addr_bits = 21;
    opcode = FSI_CMD_ABS_AR;
    trace_fsi_master_acf_cmd_abs_addr(master, addr);
    }
//
// The read/write size is encoded in the lower bits of the address
// (as it must be naturally-aligned), and the following ds bit.
//
// size	addr:1	addr:0	ds
// 1	x	x	0
// 2	x	0	1
// 4	0	1	1
//
    ds = size > 1 ? 1 : 0;
    addr &= ~(size - 1);
    if (size == 4)
    addr |= 1;
    msg_push_bits(cmd, id, 2);
    msg_push_bits(cmd, opcode, opcode_bits);
    msg_push_bits(cmd, write ? 0 : 1, 1);
    msg_push_bits(cmd, addr, addr_bits);
    msg_push_bits(cmd, ds, 1);
    for (i = 0; write && i < size; i++)
    msg_push_bits(cmd, ((uint8_t *)data)[i], 8);
    msg_push_crc(cmd);
    msg_finish_cmd(cmd);
    }
#[no_mangle]
unsafe extern "C" fn build_dpoll_command(cmd: *mut fsi_msg, slave_id: u8) {
    static void build_dpoll_command(struct fsi_msg *cmd, uint8_t slave_id)
    {
    cmd.bits = 0;
    cmd.msg = 0;
    msg_push_bits(cmd, slave_id, 2);
    msg_push_bits(cmd, FSI_CMD_DPOLL, 3);
    msg_push_crc(cmd);
    msg_finish_cmd(cmd);
    }
#[no_mangle]
unsafe extern "C" fn build_epoll_command(cmd: *mut fsi_msg, slave_id: u8) {
    static void build_epoll_command(struct fsi_msg *cmd, uint8_t slave_id)
    {
    cmd.bits = 0;
    cmd.msg = 0;
    msg_push_bits(cmd, slave_id, 2);
    msg_push_bits(cmd, FSI_CMD_EPOLL, 3);
    msg_push_crc(cmd);
    msg_finish_cmd(cmd);
    }
#[no_mangle]
unsafe extern "C" fn build_term_command(cmd: *mut fsi_msg, slave_id: u8) {
    static void build_term_command(struct fsi_msg *cmd, uint8_t slave_id)
    {
    cmd.bits = 0;
    cmd.msg = 0;
    msg_push_bits(cmd, slave_id, 2);
    msg_push_bits(cmd, FSI_CMD_TERM, 6);
    msg_push_crc(cmd);
    msg_finish_cmd(cmd);
    }
#[no_mangle]
unsafe extern "C" fn do_copro_command(master: *mut fsi_master_acf, op: u32) -> c_int {
    static int do_copro_command(struct fsi_master_acf *master, uint32_t op)
    {
    let mut timeout: u32 = 10000000;
    uint8_t stat;
    trace_fsi_master_acf_copro_command(master, op);
// Send command
    iowrite32be(op, master.sram + CMD_STAT_REG);
// Ring doorbell if any
    if (master.cvic)
    iowrite32(0x2, master.cvic + CVIC_TRIG_REG);
// Wait for status to indicate completion (or error)
    do {
    if (timeout-- == 0) {
    dev_warn(master.dev,
    "Timeout waiting for coprocessor completion\n");
    return -ETIMEDOUT;
    }
    stat = ioread8(master.sram + CMD_STAT_REG);
    } while(stat < STAT_COMPLETE || stat == 0xff);
    if (stat == STAT_COMPLETE)
    return 0;
    switch(stat) {
    case STAT_ERR_INVAL_CMD:
    return -EINVAL;
    case STAT_ERR_INVAL_IRQ:
    return -EIO;
    case STAT_ERR_MTOE:
    return -ESHUTDOWN;
    }
    return -ENXIO;
    }
#[no_mangle]
unsafe extern "C" fn clock_zeros(master: *mut fsi_master_acf, count: c_int) -> c_int {
    static int clock_zeros(struct fsi_master_acf *master, int count)
    {
    while (count) {
    int rc, lcnt = min(count, 255);
    rc = do_copro_command(master,
    CMD_IDLE_CLOCKS | (lcnt << CMD_REG_CLEN_SHIFT));
    if (rc)
    return rc;
    count -= lcnt;
    }
    return 0;
    }
    static int send_request(struct fsi_master_acf *master, struct fsi_msg *cmd,
    unsigned int resp_bits)
    {
    uint32_t op;
    trace_fsi_master_acf_send_request(master, cmd, resp_bits);
// Store message into SRAM
    iowrite32be((cmd.msg >> 32), master.sram + CMD_DATA);
    iowrite32be((cmd.msg & 0xffffffff), master.sram + CMD_DATA + 4);
    op = CMD_COMMAND;
    op |= cmd.bits << CMD_REG_CLEN_SHIFT;
    if (resp_bits)
    op |= resp_bits << CMD_REG_RLEN_SHIFT;
    return do_copro_command(master, op);
    }
    static int read_copro_response(struct fsi_master_acf *master, uint8_t size,
    uint32_t *response, u8 *tag)
    {
    let mut rtag: u8 = ioread8(master.sram + STAT_RTAG) & 0xf;
    let mut rcrc: u8 = ioread8(master.sram + STAT_RCRC) & 0xf;
    let mut rdata: u32 = 0;
    uint32_t crc;
    uint8_t ack;
// tag = ack = rtag & 3;
// we have a whole message now; check CRC
    crc = crc4(0, 1, 1);
    crc = crc4(crc, rtag, 4);
    if (ack == FSI_RESP_ACK && size) {
    rdata = ioread32be(master.sram + RSP_DATA);
    crc = crc4(crc, rdata, size);
    if (response)
// response = rdata;
    }
    crc = crc4(crc, rcrc, 4);
    trace_fsi_master_acf_copro_response(master, rtag, rcrc, rdata, crc == 0);
    if (crc) {
//
// Check if it's all 1's or all 0's, that probably means
// the host is off
//
    if ((rtag == 0xf && rcrc == 0xf) || (rtag == 0 && rcrc == 0))
    return -ENODEV;
    dev_dbg(master.dev, "Bad response CRC !\n");
    return -EAGAIN;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn send_term(master: *mut fsi_master_acf, slave: u8) -> c_int {
    static int send_term(struct fsi_master_acf *master, uint8_t slave)
    {
    struct fsi_msg cmd;
    uint8_t tag;
    int rc;
    build_term_command(&cmd, slave);
    rc = send_request(master, &cmd, 0);
    if (rc) {
    dev_warn(master.dev, "Error %d sending term\n", rc);
    return rc;
    }
    rc = read_copro_response(master, 0, core::ptr::null_mut(), &tag);
    if (rc < 0) {
    dev_err(master.dev,
    "TERM failed; lost communication with slave\n");
    return -EIO;
    } else if (tag != FSI_RESP_ACK) {
    dev_err(master.dev, "TERM failed; response %d\n", tag);
    return -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dump_ucode_trace(master: *mut fsi_master_acf) {
    static void dump_ucode_trace(struct fsi_master_acf *master)
    {
    char trbuf[52];
    char *p;
    int i;
    dev_dbg(master.dev,
    "CMDSTAT:%08x RTAG=%02x RCRC=%02x RDATA=%02x #INT=%08x\n",
    ioread32be(master.sram + CMD_STAT_REG),
    ioread8(master.sram + STAT_RTAG),
    ioread8(master.sram + STAT_RCRC),
    ioread32be(master.sram + RSP_DATA),
    ioread32be(master.sram + INT_CNT));
    for (i = 0; i < 512; i++) {
    uint8_t v;
    if ((i % 16) == 0)
    p = trbuf;
    v = ioread8(master.sram + TRACEBUF + i);
    p += sprintf(p, "%02x ", v);
    if (((i % 16) == 15) || v == TR_END)
    dev_dbg(master.dev, "%s\n", trbuf);
    if (v == TR_END)
    break;
    }
    }
    static int handle_response(struct fsi_master_acf *master,
    uint8_t slave, uint8_t size, void *data)
    {
    let mut busy_count: c_int = 0, rc;
    let mut crc_err_retries: c_int = 0;
    struct fsi_msg cmd;
    uint32_t response;
    uint8_t tag;
    retry:
    rc = read_copro_response(master, size, &response, &tag);
// Handle retries on CRC errors
    if (rc == -EAGAIN) {
// Too many retries ?
    if (crc_err_retries++ > FSI_CRC_ERR_RETRIES) {
//
// Pass it up as a -EIO otherwise upper level will retry
// the whole command which isn't what we want here.
//
    rc = -EIO;
    goto bail;
    }
    trace_fsi_master_acf_crc_rsp_error(master, crc_err_retries);
    if (master.trace_enabled)
    dump_ucode_trace(master);
    rc = clock_zeros(master, FSI_MASTER_EPOLL_CLOCKS);
    if (rc) {
    dev_warn(master.dev,
    "Error %d clocking zeros for E_POLL\n", rc);
    return rc;
    }
    build_epoll_command(&cmd, slave);
    rc = send_request(master, &cmd, size);
    if (rc) {
    dev_warn(master.dev, "Error %d sending E_POLL\n", rc);
    return -EIO;
    }
    goto retry;
    }
    if (rc)
    return rc;
    switch (tag) {
    case FSI_RESP_ACK:
    if (size && data) {
    if (size == 32)
// (__be32 *)data = cpu_to_be32(response);
#[no_mangle]
pub unsafe extern "C" fn if(16: size ==) -> else {
    else if (size == 16)
// (__be16 *)data = cpu_to_be16(response);
    else
// (u8 *)data = response;
    }
    break;
    case FSI_RESP_BUSY:
//
// Its necessary to clock slave before issuing
// d-poll, not indicated in the hardware protocol
// spec. < 20 clocks causes slave to hang, 21 ok.
//
    dev_dbg(master.dev, "Busy, retrying...\n");
    if (master.trace_enabled)
    dump_ucode_trace(master);
    rc = clock_zeros(master, FSI_MASTER_DPOLL_CLOCKS);
    if (rc) {
    dev_warn(master.dev,
    "Error %d clocking zeros for D_POLL\n", rc);
    break;
    }
    if (busy_count++ < FSI_MASTER_MAX_BUSY) {
    build_dpoll_command(&cmd, slave);
    rc = send_request(master, &cmd, size);
    if (rc) {
    dev_warn(master.dev, "Error %d sending D_POLL\n", rc);
    break;
    }
    goto retry;
    }
    dev_dbg(master.dev,
    "ERR slave is stuck in busy state, issuing TERM\n");
    send_term(master, slave);
    rc = -EIO;
    break;
    case FSI_RESP_ERRA:
    dev_dbg(master.dev, "ERRA received\n");
    if (master.trace_enabled)
    dump_ucode_trace(master);
    rc = -EIO;
    break;
    case FSI_RESP_ERRC:
    dev_dbg(master.dev, "ERRC received\n");
    if (master.trace_enabled)
    dump_ucode_trace(master);
    rc = -EAGAIN;
    break;
    }
    bail:
    if (busy_count > 0) {
    trace_fsi_master_acf_poll_response_busy(master, busy_count);
    }
    return rc;
    }
    static int fsi_master_acf_xfer(struct fsi_master_acf *master, uint8_t slave,
    struct fsi_msg *cmd, size_t resp_len, void *resp)
    {
    let mut rc: c_int = -EAGAIN, retries = 0;
    resp_len <<= 3;
    while ((retries++) < FSI_CRC_ERR_RETRIES) {
    rc = send_request(master, cmd, resp_len);
    if (rc) {
    if (rc != -ESHUTDOWN)
    dev_warn(master.dev, "Error %d sending command\n", rc);
    break;
    }
    rc = handle_response(master, slave, resp_len, resp);
    if (rc != -EAGAIN)
    break;
    rc = -EIO;
    dev_dbg(master.dev, "ECRC retry %d\n", retries);
// Pace it a bit before retry
    msleep(1);
    }
    return rc;
    }
    static int fsi_master_acf_read(struct fsi_master *_master, int link,
    uint8_t id, uint32_t addr, void *val,
    size_t size)
    {
    struct fsi_master_acf *master = to_fsi_master_acf(_master);
    struct fsi_msg cmd;
    int rc;
    if (link != 0)
    return -ENODEV;
    mutex_lock(&master.lock);
    dev_dbg(master.dev, "read id %d addr %x size %zd\n", id, addr, size);
    build_ar_command(master, &cmd, id, addr, size, core::ptr::null_mut());
    rc = fsi_master_acf_xfer(master, id, &cmd, size, val);
    last_address_update(master, id, rc == 0, addr);
    if (rc)
    dev_dbg(master.dev, "read id %d addr 0x%08x err: %d\n",
    id, addr, rc);
    mutex_unlock(&master.lock);
    return rc;
    }
    static int fsi_master_acf_write(struct fsi_master *_master, int link,
    uint8_t id, uint32_t addr, const void *val,
    size_t size)
    {
    struct fsi_master_acf *master = to_fsi_master_acf(_master);
    struct fsi_msg cmd;
    int rc;
    if (link != 0)
    return -ENODEV;
    mutex_lock(&master.lock);
    build_ar_command(master, &cmd, id, addr, size, val);
    dev_dbg(master.dev, "write id %d addr %x size %zd raw_data: %08x\n",
    id, addr, size, *(uint32_t *)val);
    rc = fsi_master_acf_xfer(master, id, &cmd, 0, core::ptr::null_mut());
    last_address_update(master, id, rc == 0, addr);
    if (rc)
    dev_dbg(master.dev, "write id %d addr 0x%08x err: %d\n",
    id, addr, rc);
    mutex_unlock(&master.lock);
    return rc;
    }
    static int fsi_master_acf_term(struct fsi_master *_master,
    int link, uint8_t id)
    {
    struct fsi_master_acf *master = to_fsi_master_acf(_master);
    struct fsi_msg cmd;
    int rc;
    if (link != 0)
    return -ENODEV;
    mutex_lock(&master.lock);
    build_term_command(&cmd, id);
    dev_dbg(master.dev, "term id %d\n", id);
    rc = fsi_master_acf_xfer(master, id, &cmd, 0, core::ptr::null_mut());
    last_address_update(master, id, false, 0);
    mutex_unlock(&master.lock);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn fsi_master_acf_break(_master: *mut fsi_master, link: c_int) -> c_int {
    static int fsi_master_acf_break(struct fsi_master *_master, int link)
    {
    struct fsi_master_acf *master = to_fsi_master_acf(_master);
    int rc;
    if (link != 0)
    return -ENODEV;
    mutex_lock(&master.lock);
    if (master.external_mode) {
    mutex_unlock(&master.lock);
    return -EBUSY;
    }
    dev_dbg(master.dev, "sending BREAK\n");
    rc = do_copro_command(master, CMD_BREAK);
    last_address_update(master, 0, false, 0);
    mutex_unlock(&master.lock);
// Wait for logic reset to take effect
    udelay(200);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn reset_cf(master: *mut fsi_master_acf) {
    static void reset_cf(struct fsi_master_acf *master)
    {
    regmap_write(master.scu, SCU_COPRO_CTRL, SCU_COPRO_RESET);
    usleep_range(20,20);
    regmap_write(master.scu, SCU_COPRO_CTRL, 0);
    usleep_range(20,20);
    }
#[no_mangle]
unsafe extern "C" fn start_cf(master: *mut fsi_master_acf) {
    static void start_cf(struct fsi_master_acf *master)
    {
    regmap_write(master.scu, SCU_COPRO_CTRL, SCU_COPRO_CLK_EN);
    }
#[no_mangle]
unsafe extern "C" fn setup_ast2500_cf_maps(master: *mut fsi_master_acf) {
    static void setup_ast2500_cf_maps(struct fsi_master_acf *master)
    {
//
// Note about byteswap setting: the bus is wired backwards,
// so setting the byteswap bit actually makes the ColdFire
// work "normally" for a BE processor, ie, put the MSB in
// the lowest address byte.
//
// We thus need to set the bit for our main memory which
// contains our program code. We create two mappings for
// the register, one with each setting.
//
// Segments 2 and 3 has a "swapped" mapping (BE)
// and 6 and 7 have a non-swapped mapping (LE) which allows
// us to avoid byteswapping register accesses since the
// registers are all LE.
//
// Setup segment 0 to our memory region
    regmap_write(master.scu, SCU_2500_COPRO_SEG0, master.cf_mem_addr |
    SCU_2500_COPRO_SEG_SWAP);
// Segments 2 and 3 to sysregs with byteswap (for SRAM)
    regmap_write(master.scu, SCU_2500_COPRO_SEG2, SYSREG_BASE |
    SCU_2500_COPRO_SEG_SWAP);
    regmap_write(master.scu, SCU_2500_COPRO_SEG3, SYSREG_BASE | 0x100000 |
    SCU_2500_COPRO_SEG_SWAP);
// And segment 6 and 7 to sysregs no byteswap
    regmap_write(master.scu, SCU_2500_COPRO_SEG6, SYSREG_BASE);
    regmap_write(master.scu, SCU_2500_COPRO_SEG7, SYSREG_BASE | 0x100000);
// Memory cachable, regs and SRAM not cachable
    regmap_write(master.scu, SCU_2500_COPRO_CACHE_CTL,
    SCU_2500_COPRO_SEG0_CACHE_EN | SCU_2500_COPRO_CACHE_EN);
    }
#[no_mangle]
unsafe extern "C" fn setup_ast2400_cf_maps(master: *mut fsi_master_acf) {
    static void setup_ast2400_cf_maps(struct fsi_master_acf *master)
    {
// Setup segment 0 to our memory region
    regmap_write(master.scu, SCU_2400_COPRO_SEG0, master.cf_mem_addr |
    SCU_2400_COPRO_SEG_SWAP);
// Segments 2 to sysregs with byteswap (for SRAM)
    regmap_write(master.scu, SCU_2400_COPRO_SEG2, SYSREG_BASE |
    SCU_2400_COPRO_SEG_SWAP);
// And segment 6 to sysregs no byteswap
    regmap_write(master.scu, SCU_2400_COPRO_SEG6, SYSREG_BASE);
// Memory cachable, regs and SRAM not cachable
    regmap_write(master.scu, SCU_2400_COPRO_CACHE_CTL,
    SCU_2400_COPRO_SEG0_CACHE_EN | SCU_2400_COPRO_CACHE_EN);
    }
    static void setup_common_fw_config(struct fsi_master_acf *master,
    void __iomem *base)
    {
    iowrite16be(master.gpio_clk_vreg, base + HDR_CLOCK_GPIO_VADDR);
    iowrite16be(master.gpio_clk_dreg, base + HDR_CLOCK_GPIO_DADDR);
    iowrite16be(master.gpio_dat_vreg, base + HDR_DATA_GPIO_VADDR);
    iowrite16be(master.gpio_dat_dreg, base + HDR_DATA_GPIO_DADDR);
    iowrite16be(master.gpio_tra_vreg, base + HDR_TRANS_GPIO_VADDR);
    iowrite16be(master.gpio_tra_dreg, base + HDR_TRANS_GPIO_DADDR);
    iowrite8(master.gpio_clk_bit, base + HDR_CLOCK_GPIO_BIT);
    iowrite8(master.gpio_dat_bit, base + HDR_DATA_GPIO_BIT);
    iowrite8(master.gpio_tra_bit, base + HDR_TRANS_GPIO_BIT);
    }
#[no_mangle]
unsafe extern "C" fn setup_ast2500_fw_config(master: *mut fsi_master_acf) {
    static void setup_ast2500_fw_config(struct fsi_master_acf *master)
    {
    void __iomem *base = master.cf_mem + HDR_OFFSET;
    setup_common_fw_config(master, base);
    iowrite32be(FW_CONTROL_USE_STOP, base + HDR_FW_CONTROL);
    }
#[no_mangle]
unsafe extern "C" fn setup_ast2400_fw_config(master: *mut fsi_master_acf) {
    static void setup_ast2400_fw_config(struct fsi_master_acf *master)
    {
    void __iomem *base = master.cf_mem + HDR_OFFSET;
    setup_common_fw_config(master, base);
    iowrite32be(FW_CONTROL_CONT_CLOCK|FW_CONTROL_DUMMY_RD, base + HDR_FW_CONTROL);
    }
#[no_mangle]
unsafe extern "C" fn setup_gpios_for_copro(master: *mut fsi_master_acf) -> c_int {
    static int setup_gpios_for_copro(struct fsi_master_acf *master)
    {
    int rc;
// This aren't under ColdFire control, just set them up appropriately
    gpiod_direction_output(master.gpio_mux, 1);
    gpiod_direction_output(master.gpio_enable, 1);
// Those are under ColdFire control, let it configure them
    rc = aspeed_gpio_copro_grab_gpio(master.gpio_clk, &master.gpio_clk_vreg,
    &master.gpio_clk_dreg, &master.gpio_clk_bit);
    if (rc) {
    dev_err(master.dev, "failed to assign clock gpio to coprocessor\n");
    return rc;
    }
    rc = aspeed_gpio_copro_grab_gpio(master.gpio_data, &master.gpio_dat_vreg,
    &master.gpio_dat_dreg, &master.gpio_dat_bit);
    if (rc) {
    dev_err(master.dev, "failed to assign data gpio to coprocessor\n");
    aspeed_gpio_copro_release_gpio(master.gpio_clk);
    return rc;
    }
    rc = aspeed_gpio_copro_grab_gpio(master.gpio_trans, &master.gpio_tra_vreg,
    &master.gpio_tra_dreg, &master.gpio_tra_bit);
    if (rc) {
    dev_err(master.dev, "failed to assign trans gpio to coprocessor\n");
    aspeed_gpio_copro_release_gpio(master.gpio_clk);
    aspeed_gpio_copro_release_gpio(master.gpio_data);
    return rc;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn release_copro_gpios(master: *mut fsi_master_acf) {
    static void release_copro_gpios(struct fsi_master_acf *master)
    {
    aspeed_gpio_copro_release_gpio(master.gpio_clk);
    aspeed_gpio_copro_release_gpio(master.gpio_data);
    aspeed_gpio_copro_release_gpio(master.gpio_trans);
    }
#[no_mangle]
unsafe extern "C" fn load_copro_firmware(master: *mut fsi_master_acf) -> c_int {
    static int load_copro_firmware(struct fsi_master_acf *master)
    {
    const struct firmware *fw;
    let mut sig: u16 = 0, wanted_sig;
    const u8 *data;
    let mut size: usize = 0;
    int rc;
// Get the binary
    rc = request_firmware(&fw, FW_FILE_NAME, master.dev);
    if (rc) {
    dev_err(
    master.dev, "Error %d to load firmware '%s' !\n",
    rc, FW_FILE_NAME);
    return rc;
    }
// Which image do we want ? (shared vs. split clock/data GPIOs)
    if (master.gpio_clk_vreg == master.gpio_dat_vreg)
    wanted_sig = SYS_SIG_SHARED;
    else
    wanted_sig = SYS_SIG_SPLIT;
    dev_dbg(master.dev, "Looking for image sig %04x\n", wanted_sig);
// Try to find it
    for (data = fw.data; data < (fw.data + fw.size);) {
    sig = be16_to_cpup((__be16 *)(data + HDR_OFFSET + HDR_SYS_SIG));
    size = be32_to_cpup((__be32 *)(data + HDR_OFFSET + HDR_FW_SIZE));
    if (sig == wanted_sig)
    break;
    data += size;
    }
    if (sig != wanted_sig) {
    dev_err(master.dev, "Failed to locate image sig %04x in FW blob\n",
    wanted_sig);
    rc = -ENODEV;
    goto release_fw;
    }
    if (size > master.cf_mem_size) {
    dev_err(master.dev, "FW size (%zd) bigger than memory reserve (%zd)\n",
    fw.size, master.cf_mem_size);
    rc = -ENOMEM;
    } else {
    memcpy_toio(master.cf_mem, data, size);
    }
    release_fw:
    release_firmware(fw);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn check_firmware_image(master: *mut fsi_master_acf) -> c_int {
    static int check_firmware_image(struct fsi_master_acf *master)
    {
    uint32_t fw_vers, fw_api, fw_options;
    fw_vers = ioread16be(master.cf_mem + HDR_OFFSET + HDR_FW_VERS);
    fw_api = ioread16be(master.cf_mem + HDR_OFFSET + HDR_API_VERS);
    fw_options = ioread32be(master.cf_mem + HDR_OFFSET + HDR_FW_OPTIONS);
    master.trace_enabled = !!(fw_options & FW_OPTION_TRACE_EN);
// Check version and signature
    dev_info(master.dev, "ColdFire initialized, firmware v%d API v%d.%d (trace %s)\n",
    fw_vers, fw_api >> 8, fw_api & 0xff,
    master.trace_enabled ? "enabled" : "disabled");
    if ((fw_api >> 8) != API_VERSION_MAJ) {
    dev_err(master.dev, "Unsupported coprocessor API version !\n");
    return -ENODEV;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn copro_enable_sw_irq(master: *mut fsi_master_acf) -> c_int {
    static int copro_enable_sw_irq(struct fsi_master_acf *master)
    {
    int timeout;
    uint32_t val;
//
// Enable coprocessor interrupt input. I've had problems getting the
// value to stick, so try in a loop
//
    for (timeout = 0; timeout < 10; timeout++) {
    iowrite32(0x2, master.cvic + CVIC_EN_REG);
    val = ioread32(master.cvic + CVIC_EN_REG);
    if (val & 2)
    break;
    msleep(1);
    }
    if (!(val & 2)) {
    dev_err(master.dev, "Failed to enable coprocessor interrupt !\n");
    return -ENODEV;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fsi_master_acf_setup(master: *mut fsi_master_acf) -> c_int {
    static int fsi_master_acf_setup(struct fsi_master_acf *master)
    {
    int timeout, rc;
    uint32_t val;
// Make sure the ColdFire is stopped
    reset_cf(master);
//
// Clear SRAM. This needs to happen before we setup the GPIOs
// as we might start trying to arbitrate as soon as that happens.
//
    memset_io(master.sram, 0, SRAM_SIZE);
// Configure GPIOs
    rc = setup_gpios_for_copro(master);
    if (rc)
    return rc;
// Load the firmware into the reserved memory
    rc = load_copro_firmware(master);
    if (rc)
    return rc;
// Read signature and check versions
    rc = check_firmware_image(master);
    if (rc)
    return rc;
// Setup coldfire memory map
    if (master.is_ast2500) {
    setup_ast2500_cf_maps(master);
    setup_ast2500_fw_config(master);
    } else {
    setup_ast2400_cf_maps(master);
    setup_ast2400_fw_config(master);
    }
// Start the ColdFire
    start_cf(master);
// Wait for status register to indicate command completion
// which signals the initialization is complete
//
    for (timeout = 0; timeout < 10; timeout++) {
    val = ioread8(master.sram + CF_STARTED);
    if (val)
    break;
    msleep(1);
    }
    if (!val) {
    dev_err(master.dev, "Coprocessor startup timeout !\n");
    rc = -ENODEV;
    goto err;
    }
// Configure echo & send delay
    iowrite8(master.t_send_delay, master.sram + SEND_DLY_REG);
    iowrite8(master.t_echo_delay, master.sram + ECHO_DLY_REG);
// Enable SW interrupt to copro if any
    if (master.cvic) {
    rc = copro_enable_sw_irq(master);
    if (rc)
    goto err;
    }
    return 0;
    err:
// An error occurred, don't leave the coprocessor running
    reset_cf(master);
// Release the GPIOs
    release_copro_gpios(master);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn fsi_master_acf_terminate(master: *mut fsi_master_acf) {
    static void fsi_master_acf_terminate(struct fsi_master_acf *master)
    {
    unsigned long flags;
//
// A GPIO arbitration requestion could come in while this is
// happening. To avoid problems, we disable interrupts so it
// cannot preempt us on this CPU
//
    local_irq_save(flags);
// Stop the coprocessor
    reset_cf(master);
// We mark the copro not-started
    iowrite32(0, master.sram + CF_STARTED);
// We mark the ARB register as having given up arbitration to
// deal with a potential race with the arbitration request
//
    iowrite8(ARB_ARM_ACK, master.sram + ARB_REG);
    local_irq_restore(flags);
// Return the GPIOs to the ARM
    release_copro_gpios(master);
    }
#[no_mangle]
unsafe extern "C" fn fsi_master_acf_setup_external(master: *mut fsi_master_acf) {
    static void fsi_master_acf_setup_external(struct fsi_master_acf *master)
    {
// Setup GPIOs for external FSI master (FSP box)
    gpiod_direction_output(master.gpio_mux, 0);
    gpiod_direction_output(master.gpio_trans, 0);
    gpiod_direction_output(master.gpio_enable, 1);
    gpiod_direction_input(master.gpio_clk);
    gpiod_direction_input(master.gpio_data);
    }
    static int fsi_master_acf_link_enable(struct fsi_master *_master, int link,
    bool enable)
    {
    struct fsi_master_acf *master = to_fsi_master_acf(_master);
    let mut rc: c_int = -EBUSY;
    if (link != 0)
    return -ENODEV;
    mutex_lock(&master.lock);
    if (!master.external_mode) {
    gpiod_set_value(master.gpio_enable, enable ? 1 : 0);
    rc = 0;
    }
    mutex_unlock(&master.lock);
    return rc;
    }
    static int fsi_master_acf_link_config(struct fsi_master *_master, int link,
    u8 t_send_delay, u8 t_echo_delay)
    {
    struct fsi_master_acf *master = to_fsi_master_acf(_master);
    if (link != 0)
    return -ENODEV;
    mutex_lock(&master.lock);
    master.t_send_delay = t_send_delay;
    master.t_echo_delay = t_echo_delay;
    dev_dbg(master.dev, "Changing delays: send=%d echo=%d\n",
    t_send_delay, t_echo_delay);
    iowrite8(master.t_send_delay, master.sram + SEND_DLY_REG);
    iowrite8(master.t_echo_delay, master.sram + ECHO_DLY_REG);
    mutex_unlock(&master.lock);
    return 0;
    }
    static ssize_t external_mode_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct fsi_master_acf *master = dev_get_drvdata(dev);
    return snprintf(buf, PAGE_SIZE - 1, "%u\n",
    master.external_mode ? 1 : 0);
    }
    static ssize_t external_mode_store(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    struct fsi_master_acf *master = dev_get_drvdata(dev);
    unsigned long val;
    bool external_mode;
    int err;
    err = kstrtoul(buf, 0, &val);
    if (err)
    return err;
    external_mode = !!val;
    mutex_lock(&master.lock);
    if (external_mode == master.external_mode) {
    mutex_unlock(&master.lock);
    return count;
    }
    master.external_mode = external_mode;
    if (master.external_mode) {
    fsi_master_acf_terminate(master);
    fsi_master_acf_setup_external(master);
    } else
    fsi_master_acf_setup(master);
    mutex_unlock(&master.lock);
    fsi_master_rescan(&master.master);
    return count;
    }
    static DEVICE_ATTR(external_mode, 0664,
    external_mode_show, external_mode_store);
#[no_mangle]
unsafe extern "C" fn fsi_master_acf_gpio_request(data: *mut c_void) -> c_int {
    static int fsi_master_acf_gpio_request(void *data)
    {
    struct fsi_master_acf *master = data;
    int timeout;
    u8 val;
// Note: This doesn't require holding out mutex
// Write request
    iowrite8(ARB_ARM_REQ, master.sram + ARB_REG);
//
// There is a race (which does happen at boot time) when we get an
// arbitration request as we are either about to or just starting
// the coprocessor.
//
// To handle it, we first check if we are running. If not yet we
// check whether the copro is started in the SCU.
//
// If it's not started, we can basically just assume we have arbitration
// and return. Otherwise, we wait normally expecting for the arbitration
// to eventually complete.
//
    if (ioread32(master.sram + CF_STARTED) == 0) {
    let mut reg: c_uint = 0;
    regmap_read(master.scu, SCU_COPRO_CTRL, &reg);
    if (!(reg & SCU_COPRO_CLK_EN))
    return 0;
    }
// Ring doorbell if any
    if (master.cvic)
    iowrite32(0x2, master.cvic + CVIC_TRIG_REG);
    for (timeout = 0; timeout < 10000; timeout++) {
    val = ioread8(master.sram + ARB_REG);
    if (val != ARB_ARM_REQ)
    break;
    udelay(1);
    }
// If it failed, override anyway
    if (val != ARB_ARM_ACK)
    dev_warn(master.dev, "GPIO request arbitration timeout\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fsi_master_acf_gpio_release(data: *mut c_void) -> c_int {
    static int fsi_master_acf_gpio_release(void *data)
    {
    struct fsi_master_acf *master = data;
// Write release
    iowrite8(0, master.sram + ARB_REG);
// Ring doorbell if any
    if (master.cvic)
    iowrite32(0x2, master.cvic + CVIC_TRIG_REG);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fsi_master_acf_release(dev: *mut device) {
    static void fsi_master_acf_release(struct device *dev)
    {
    struct fsi_master_acf *master = to_fsi_master_acf(to_fsi_master(dev));
// Cleanup, stop coprocessor
    mutex_lock(&master.lock);
    fsi_master_acf_terminate(master);
    aspeed_gpio_copro_set_ops(core::ptr::null_mut(), core::ptr::null_mut());
    mutex_unlock(&master.lock);
// Free resources
    gen_pool_free(master.sram_pool, (unsigned long)master.sram, SRAM_SIZE);
    of_node_put(dev_of_node(master.dev));
    kfree(master);
    }
    static const struct aspeed_gpio_copro_ops fsi_master_acf_gpio_ops = {
    .request_access = fsi_master_acf_gpio_request,
    .release_access = fsi_master_acf_gpio_release,
    };
#[no_mangle]
unsafe extern "C" fn fsi_master_acf_probe(pdev: *mut platform_device) -> c_int {
    static int fsi_master_acf_probe(struct platform_device *pdev)
    {
    struct device_node *np, *mnode = dev_of_node(&pdev.dev);
    struct genpool_data_fixed gpdf;
    struct fsi_master_acf *master;
    struct gpio_desc *gpio;
    struct resource res;
    uint32_t cf_mem_align;
    int rc;
    master = kzalloc_obj(*master);
    if (!master)
    return -ENOMEM;
    master.dev = &pdev.dev;
    master.master.dev.parent = master.dev;
    master.last_addr = LAST_ADDR_INVALID;
// AST2400 vs. AST2500
    master.is_ast2500 = of_device_is_compatible(mnode, "aspeed,ast2500-cf-fsi-master");
// Grab the SCU, we'll need to access it to configure the coprocessor
    if (master.is_ast2500)
    master.scu = syscon_regmap_lookup_by_compatible("aspeed,ast2500-scu");
    else
    master.scu = syscon_regmap_lookup_by_compatible("aspeed,ast2400-scu");
    if (IS_ERR(master.scu)) {
    dev_err(&pdev.dev, "failed to find SCU regmap\n");
    rc = PTR_ERR(master.scu);
    goto err_free;
    }
// Grab all the GPIOs we need
    gpio = devm_gpiod_get(&pdev.dev, "clock", 0);
    if (IS_ERR(gpio)) {
    dev_err(&pdev.dev, "failed to get clock gpio\n");
    rc = PTR_ERR(gpio);
    goto err_free;
    }
    master.gpio_clk = gpio;
    gpio = devm_gpiod_get(&pdev.dev, "data", 0);
    if (IS_ERR(gpio)) {
    dev_err(&pdev.dev, "failed to get data gpio\n");
    rc = PTR_ERR(gpio);
    goto err_free;
    }
    master.gpio_data = gpio;
// Optional GPIOs
    gpio = devm_gpiod_get_optional(&pdev.dev, "trans", 0);
    if (IS_ERR(gpio)) {
    dev_err(&pdev.dev, "failed to get trans gpio\n");
    rc = PTR_ERR(gpio);
    goto err_free;
    }
    master.gpio_trans = gpio;
    gpio = devm_gpiod_get_optional(&pdev.dev, "enable", 0);
    if (IS_ERR(gpio)) {
    dev_err(&pdev.dev, "failed to get enable gpio\n");
    rc = PTR_ERR(gpio);
    goto err_free;
    }
    master.gpio_enable = gpio;
    gpio = devm_gpiod_get_optional(&pdev.dev, "mux", 0);
    if (IS_ERR(gpio)) {
    dev_err(&pdev.dev, "failed to get mux gpio\n");
    rc = PTR_ERR(gpio);
    goto err_free;
    }
    master.gpio_mux = gpio;
// Grab the reserved memory region (use DMA API instead ?)
    rc = of_reserved_mem_region_to_resource(mnode, 0, &res);
    if (rc) {
    dev_err(&pdev.dev, "Couldn't address to resource for reserved memory\n");
    rc = -ENOMEM;
    goto err_free;
    }
    master.cf_mem_size = resource_size(&res);
    master.cf_mem_addr = (uint32_t)res.start;
    cf_mem_align = master.is_ast2500 ? 0x00100000 : 0x00200000;
    if (master.cf_mem_addr & (cf_mem_align - 1)) {
    dev_err(&pdev.dev, "Reserved memory has insufficient alignment\n");
    rc = -ENOMEM;
    goto err_free;
    }
    master.cf_mem = devm_ioremap_resource(&pdev.dev, &res);
    if (IS_ERR(master.cf_mem)) {
    rc = PTR_ERR(master.cf_mem);
    goto err_free;
    }
    dev_dbg(&pdev.dev, "DRAM allocation @%x\n", master.cf_mem_addr);
// AST2500 has a SW interrupt to the coprocessor
    if (master.is_ast2500) {
// Grab the CVIC (ColdFire interrupts controller)
    np = of_parse_phandle(mnode, "aspeed,cvic", 0);
    if (!np) {
    dev_err(&pdev.dev, "Didn't find CVIC\n");
    rc = -EINVAL;
    goto err_free;
    }
    master.cvic = devm_of_iomap(&pdev.dev, np, 0, core::ptr::null_mut());
    if (IS_ERR(master.cvic)) {
    of_node_put(np);
    rc = PTR_ERR(master.cvic);
    dev_err(&pdev.dev, "Error %d mapping CVIC\n", rc);
    goto err_free;
    }
    rc = of_property_read_u32(np, "copro-sw-interrupts",
    &master.cvic_sw_irq);
    of_node_put(np);
    if (rc) {
    dev_err(&pdev.dev, "Can't find coprocessor SW interrupt\n");
    goto err_free;
    }
    }
// Grab the SRAM
    master.sram_pool = of_gen_pool_get(dev_of_node(&pdev.dev), "aspeed,sram", 0);
    if (!master.sram_pool) {
    rc = -ENODEV;
    dev_err(&pdev.dev, "Can't find sram pool\n");
    goto err_free;
    }
// Current microcode only deals with fixed location in SRAM
    gpdf.offset = 0;
    master.sram = (void __iomem *)gen_pool_alloc_algo(master.sram_pool, SRAM_SIZE,
    gen_pool_fixed_alloc, &gpdf);
    if (!master.sram) {
    rc = -ENOMEM;
    dev_err(&pdev.dev, "Failed to allocate sram from pool\n");
    goto err_free;
    }
    dev_dbg(&pdev.dev, "SRAM allocation @%lx\n",
    (unsigned long)gen_pool_virt_to_phys(master.sram_pool,
    (unsigned long)master.sram));
//
// Hookup with the GPIO driver for arbitration of GPIO banks
// ownership.
//
    aspeed_gpio_copro_set_ops(&fsi_master_acf_gpio_ops, master);
// Default FSI command delays
    master.t_send_delay = FSI_SEND_DELAY_CLOCKS;
    master.t_echo_delay = FSI_ECHO_DELAY_CLOCKS;
    master.master.n_links = 1;
    if (master.is_ast2500)
    master.master.flags = FSI_MASTER_FLAG_SWCLOCK;
    master.master.read = fsi_master_acf_read;
    master.master.write = fsi_master_acf_write;
    master.master.term = fsi_master_acf_term;
    master.master.send_break = fsi_master_acf_break;
    master.master.link_enable = fsi_master_acf_link_enable;
    master.master.link_config = fsi_master_acf_link_config;
    master.master.dev.of_node = of_node_get(dev_of_node(master.dev));
    master.master.dev.release = fsi_master_acf_release;
    platform_set_drvdata(pdev, master);
    mutex_init(&master.lock);
    mutex_lock(&master.lock);
    rc = fsi_master_acf_setup(master);
    mutex_unlock(&master.lock);
    if (rc)
    goto release_of_dev;
    rc = device_create_file(&pdev.dev, &dev_attr_external_mode);
    if (rc)
    goto stop_copro;
    rc = fsi_master_register(&master.master);
    if (!rc)
    return 0;
    device_remove_file(master.dev, &dev_attr_external_mode);
    put_device(&master.master.dev);
    return rc;
    stop_copro:
    fsi_master_acf_terminate(master);
    release_of_dev:
    aspeed_gpio_copro_set_ops(core::ptr::null_mut(), core::ptr::null_mut());
    gen_pool_free(master.sram_pool, (unsigned long)master.sram, SRAM_SIZE);
    of_node_put(dev_of_node(master.dev));
    err_free:
    kfree(master);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn fsi_master_acf_remove(pdev: *mut platform_device) {
    static void fsi_master_acf_remove(struct platform_device *pdev)
    {
    struct fsi_master_acf *master = platform_get_drvdata(pdev);
    device_remove_file(master.dev, &dev_attr_external_mode);
    fsi_master_unregister(&master.master);
    }
    static const struct of_device_id fsi_master_acf_match[] = {
    { .compatible = "aspeed,ast2400-cf-fsi-master" },
    { .compatible = "aspeed,ast2500-cf-fsi-master" },
    { },
    };
    MODULE_DEVICE_TABLE(of, fsi_master_acf_match);
    static struct platform_driver fsi_master_acf = {
    .driver = {
    .name		= "fsi-master-acf",
    .of_match_table	= fsi_master_acf_match,
    },
    .probe	= fsi_master_acf_probe,
    .remove = fsi_master_acf_remove,
    };
    module_platform_driver(fsi_master_acf);
    MODULE_DESCRIPTION("A FSI master based on Aspeed ColdFire coprocessor");
    MODULE_LICENSE("GPL");
    MODULE_FIRMWARE(FW_FILE_NAME);
