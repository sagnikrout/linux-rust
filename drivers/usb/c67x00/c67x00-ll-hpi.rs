//! Automatically rewritten from C to Rust
//! Source: drivers/usb/c67x00/c67x00-ll-hpi.c
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
// c67x00-ll-hpi.c: Cypress C67X00 USB Low level interface using HPI
//
// Copyright (C) 2006-2008 Barco N.V.
// Derived from the Cypress cy7c67200/300 ezusb linux driver and
// based on multiple host controller drivers inside the linux kernel.
//

pub const COMM_REGS: c_int = 14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c67x00_lcp_int_data {
    pub regs: [u16; COMM_REGS],
}

// --------------------------------------------------------------------------
// Interface definitions
pub const COMM_ACK: c_uint = 0x0FED;
pub const COMM_NAK: c_uint = 0xDEAD;
pub const COMM_RESET: c_uint = 0xFA50;
pub const COMM_EXEC_INT: c_uint = 0xCE01;
pub const COMM_INT_NUM: c_uint = 0x01C2;
// Registers 0 to COMM_REGS-1

pub const HUSB_pEOT: c_uint = 0x01B4;
// Software interrupts
// 114, 115:

pub const HUSB_RESET_INT: c_uint = 0x0074;
pub const SUSB_INIT_INT: c_uint = 0x0071;

// -----------------------------------------------------------------------
// HPI implementation
//
// The c67x00 chip also support control via SPI or HSS serial
// interfaces. However, this driver assumes that register access can
// be performed from IRQ context. While this is a safe assumption with
// the HPI interface, it is not true for the serial interfaces.
//
// HPI registers
pub const HPI_DATA: c_int = 0;
pub const HPI_MAILBOX: c_int = 1;
pub const HPI_ADDR: c_int = 2;
pub const HPI_STATUS: c_int = 3;
//
// According to CY7C67300 specification (tables 140 and 141) HPI read and
// write cycle duration Tcyc must be at least 6T long, where T is 1/48MHz,
// which is 125ns.
//
pub const HPI_T_CYC_NS: c_int = 125;
#[no_mangle]
pub unsafe extern "C" fn hpi_read_reg(dev: *mut c67x00_device, reg: c_int) -> u16 {
    static inline u16 hpi_read_reg(struct c67x00_device *dev, int reg)
    {
    ndelay(HPI_T_CYC_NS);
    return __raw_readw(dev.hpi.base + reg * dev.hpi.regstep);
    }
#[no_mangle]
pub unsafe extern "C" fn hpi_write_reg(dev: *mut c67x00_device, reg: c_int, value: u16) {
    static inline void hpi_write_reg(struct c67x00_device *dev, int reg, u16 value)
    {
    ndelay(HPI_T_CYC_NS);
    __raw_writew(value, dev.hpi.base + reg * dev.hpi.regstep);
    }
#[no_mangle]
pub unsafe extern "C" fn hpi_read_word_nolock(dev: *mut c67x00_device, reg: u16) -> u16 {
    static inline u16 hpi_read_word_nolock(struct c67x00_device *dev, u16 reg)
    {
    hpi_write_reg(dev, HPI_ADDR, reg);
    return hpi_read_reg(dev, HPI_DATA);
    }
#[no_mangle]
unsafe extern "C" fn hpi_read_word(dev: *mut c67x00_device, reg: u16) -> u16 {
    static u16 hpi_read_word(struct c67x00_device *dev, u16 reg)
    {
    u16 value;
    unsigned long flags;
    spin_lock_irqsave(&dev.hpi.lock, flags);
    value = hpi_read_word_nolock(dev, reg);
    spin_unlock_irqrestore(&dev.hpi.lock, flags);
    return value;
    }
#[no_mangle]
unsafe extern "C" fn hpi_write_word_nolock(dev: *mut c67x00_device, reg: u16, value: u16) {
    static void hpi_write_word_nolock(struct c67x00_device *dev, u16 reg, u16 value)
    {
    hpi_write_reg(dev, HPI_ADDR, reg);
    hpi_write_reg(dev, HPI_DATA, value);
    }
#[no_mangle]
unsafe extern "C" fn hpi_write_word(dev: *mut c67x00_device, reg: u16, value: u16) {
    static void hpi_write_word(struct c67x00_device *dev, u16 reg, u16 value)
    {
    unsigned long flags;
    spin_lock_irqsave(&dev.hpi.lock, flags);
    hpi_write_word_nolock(dev, reg, value);
    spin_unlock_irqrestore(&dev.hpi.lock, flags);
    }
//
// Only data is little endian, addr has cpu endianess
//
    static void hpi_write_words_le16(struct c67x00_device *dev, u16 addr,
    __le16 *data, u16 count)
    {
    unsigned long flags;
    int i;
    spin_lock_irqsave(&dev.hpi.lock, flags);
    hpi_write_reg(dev, HPI_ADDR, addr);
    for (i = 0; i < count; i++)
    hpi_write_reg(dev, HPI_DATA, le16_to_cpu(*data++));
    spin_unlock_irqrestore(&dev.hpi.lock, flags);
    }
//
// Only data is little endian, addr has cpu endianess
//
    static void hpi_read_words_le16(struct c67x00_device *dev, u16 addr,
    __le16 *data, u16 count)
    {
    unsigned long flags;
    int i;
    spin_lock_irqsave(&dev.hpi.lock, flags);
    hpi_write_reg(dev, HPI_ADDR, addr);
    for (i = 0; i < count; i++)
// data++ = cpu_to_le16(hpi_read_reg(dev, HPI_DATA));
    spin_unlock_irqrestore(&dev.hpi.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn hpi_set_bits(dev: *mut c67x00_device, reg: u16, mask: u16) {
    static void hpi_set_bits(struct c67x00_device *dev, u16 reg, u16 mask)
    {
    u16 value;
    unsigned long flags;
    spin_lock_irqsave(&dev.hpi.lock, flags);
    value = hpi_read_word_nolock(dev, reg);
    hpi_write_word_nolock(dev, reg, value | mask);
    spin_unlock_irqrestore(&dev.hpi.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn hpi_clear_bits(dev: *mut c67x00_device, reg: u16, mask: u16) {
    static void hpi_clear_bits(struct c67x00_device *dev, u16 reg, u16 mask)
    {
    u16 value;
    unsigned long flags;
    spin_lock_irqsave(&dev.hpi.lock, flags);
    value = hpi_read_word_nolock(dev, reg);
    hpi_write_word_nolock(dev, reg, value & ~mask);
    spin_unlock_irqrestore(&dev.hpi.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn hpi_recv_mbox(dev: *mut c67x00_device) -> u16 {
    static u16 hpi_recv_mbox(struct c67x00_device *dev)
    {
    u16 value;
    unsigned long flags;
    spin_lock_irqsave(&dev.hpi.lock, flags);
    value = hpi_read_reg(dev, HPI_MAILBOX);
    spin_unlock_irqrestore(&dev.hpi.lock, flags);
    return value;
    }
#[no_mangle]
unsafe extern "C" fn hpi_send_mbox(dev: *mut c67x00_device, value: u16) -> u16 {
    static u16 hpi_send_mbox(struct c67x00_device *dev, u16 value)
    {
    unsigned long flags;
    spin_lock_irqsave(&dev.hpi.lock, flags);
    hpi_write_reg(dev, HPI_MAILBOX, value);
    spin_unlock_irqrestore(&dev.hpi.lock, flags);
    return value;
    }
#[no_mangle]
pub unsafe extern "C" fn c67x00_ll_hpi_status(dev: *mut c67x00_device) -> u16 {
    u16 c67x00_ll_hpi_status(struct c67x00_device *dev)
    {
    u16 value;
    unsigned long flags;
    spin_lock_irqsave(&dev.hpi.lock, flags);
    value = hpi_read_reg(dev, HPI_STATUS);
    spin_unlock_irqrestore(&dev.hpi.lock, flags);
    return value;
    }
#[no_mangle]
pub unsafe extern "C" fn c67x00_ll_hpi_reg_init(dev: *mut c67x00_device) {
    void c67x00_ll_hpi_reg_init(struct c67x00_device *dev)
    {
    int i;
    hpi_recv_mbox(dev);
    c67x00_ll_hpi_status(dev);
    hpi_write_word(dev, HPI_IRQ_ROUTING_REG, 0);
    for (i = 0; i < C67X00_SIES; i++) {
    hpi_write_word(dev, SIEMSG_REG(i), 0);
    hpi_read_word(dev, SIEMSG_REG(i));
    }
    }
#[no_mangle]
pub unsafe extern "C" fn c67x00_ll_hpi_enable_sofeop(sie: *mut c67x00_sie) {
    void c67x00_ll_hpi_enable_sofeop(struct c67x00_sie *sie)
    {
    hpi_set_bits(sie.dev, HPI_IRQ_ROUTING_REG,
    SOFEOP_TO_HPI_EN(sie.sie_num));
    }
#[no_mangle]
pub unsafe extern "C" fn c67x00_ll_hpi_disable_sofeop(sie: *mut c67x00_sie) {
    void c67x00_ll_hpi_disable_sofeop(struct c67x00_sie *sie)
    {
    hpi_clear_bits(sie.dev, HPI_IRQ_ROUTING_REG,
    SOFEOP_TO_HPI_EN(sie.sie_num));
    }
// --------------------------------------------------------------------------
// Transactions
#[no_mangle]
pub unsafe extern "C" fn ll_recv_msg(dev: *mut c67x00_device) -> c_int {
    static inline int ll_recv_msg(struct c67x00_device *dev)
    {
    u16 res;
    res = wait_for_completion_timeout(&dev.hpi.lcp.msg_received, 5 * HZ);
    WARN_ON(!res);
    return (res == 0) ? -EIO : 0;
    }
// --------------------------------------------------------------------------
// General functions
#[no_mangle]
pub unsafe extern "C" fn c67x00_ll_fetch_siemsg(dev: *mut c67x00_device, sie_num: c_int) -> u16 {
    u16 c67x00_ll_fetch_siemsg(struct c67x00_device *dev, int sie_num)
    {
    u16 val;
    val = hpi_read_word(dev, SIEMSG_REG(sie_num));
// clear register to allow next message
    hpi_write_word(dev, SIEMSG_REG(sie_num), 0);
    return val;
    }
#[no_mangle]
pub unsafe extern "C" fn c67x00_ll_get_usb_ctl(sie: *mut c67x00_sie) -> u16 {
    u16 c67x00_ll_get_usb_ctl(struct c67x00_sie *sie)
    {
    return hpi_read_word(sie.dev, USB_CTL_REG(sie.sie_num));
    }
//
// c67x00_ll_usb_clear_status - clear the USB status bits
//
#[no_mangle]
pub unsafe extern "C" fn c67x00_ll_usb_clear_status(sie: *mut c67x00_sie, bits: u16) {
    void c67x00_ll_usb_clear_status(struct c67x00_sie *sie, u16 bits)
    {
    hpi_write_word(sie.dev, USB_STAT_REG(sie.sie_num), bits);
    }
#[no_mangle]
pub unsafe extern "C" fn c67x00_ll_usb_get_status(sie: *mut c67x00_sie) -> u16 {
    u16 c67x00_ll_usb_get_status(struct c67x00_sie *sie)
    {
    return hpi_read_word(sie.dev, USB_STAT_REG(sie.sie_num));
    }
// --------------------------------------------------------------------------
    static int c67x00_comm_exec_int(struct c67x00_device *dev, u16 nr,
    struct c67x00_lcp_int_data *data)
    {
    int i, rc;
    mutex_lock(&dev.hpi.lcp.mutex);
    hpi_write_word(dev, COMM_INT_NUM, nr);
    for (i = 0; i < COMM_REGS; i++)
    hpi_write_word(dev, COMM_R(i), data.regs[i]);
    hpi_send_mbox(dev, COMM_EXEC_INT);
    rc = ll_recv_msg(dev);
    mutex_unlock(&dev.hpi.lcp.mutex);
    return rc;
    }
// --------------------------------------------------------------------------
// Host specific functions
#[no_mangle]
pub unsafe extern "C" fn c67x00_ll_set_husb_eot(dev: *mut c67x00_device, value: u16) {
    void c67x00_ll_set_husb_eot(struct c67x00_device *dev, u16 value)
    {
    mutex_lock(&dev.hpi.lcp.mutex);
    hpi_write_word(dev, HUSB_pEOT, value);
    mutex_unlock(&dev.hpi.lcp.mutex);
    }
#[no_mangle]
pub unsafe extern "C" fn c67x00_ll_husb_sie_init(sie: *mut c67x00_sie) {
    static inline void c67x00_ll_husb_sie_init(struct c67x00_sie *sie)
    {
    struct c67x00_device *dev = sie.dev;
    struct c67x00_lcp_int_data data;
    int rc;
    rc = c67x00_comm_exec_int(dev, HUSB_SIE_INIT_INT(sie.sie_num), &data);
    BUG_ON(rc); /* No return path for error code; crash spectacularly */
    }
#[no_mangle]
pub unsafe extern "C" fn c67x00_ll_husb_reset(sie: *mut c67x00_sie, port: c_int) {
    void c67x00_ll_husb_reset(struct c67x00_sie *sie, int port)
    {
    struct c67x00_device *dev = sie.dev;
    struct c67x00_lcp_int_data data;
    int rc;
    data.regs[0] = 50;	/* Reset USB port for 50ms */
    data.regs[1] = port | (sie.sie_num << 1);
    rc = c67x00_comm_exec_int(dev, HUSB_RESET_INT, &data);
    BUG_ON(rc); /* No return path for error code; crash spectacularly */
    }
#[no_mangle]
pub unsafe extern "C" fn c67x00_ll_husb_set_current_td(sie: *mut c67x00_sie, addr: u16) {
    void c67x00_ll_husb_set_current_td(struct c67x00_sie *sie, u16 addr)
    {
    hpi_write_word(sie.dev, HUSB_SIE_pCurrentTDPtr(sie.sie_num), addr);
    }
#[no_mangle]
pub unsafe extern "C" fn c67x00_ll_husb_get_current_td(sie: *mut c67x00_sie) -> u16 {
    u16 c67x00_ll_husb_get_current_td(struct c67x00_sie *sie)
    {
    return hpi_read_word(sie.dev, HUSB_SIE_pCurrentTDPtr(sie.sie_num));
    }
#[no_mangle]
pub unsafe extern "C" fn c67x00_ll_husb_get_frame(sie: *mut c67x00_sie) -> u16 {
    u16 c67x00_ll_husb_get_frame(struct c67x00_sie *sie)
    {
    return hpi_read_word(sie.dev, HOST_FRAME_REG(sie.sie_num));
    }
#[no_mangle]
pub unsafe extern "C" fn c67x00_ll_husb_init_host_port(sie: *mut c67x00_sie) {
    void c67x00_ll_husb_init_host_port(struct c67x00_sie *sie)
    {
// Set port into host mode
    hpi_set_bits(sie.dev, USB_CTL_REG(sie.sie_num), HOST_MODE);
    c67x00_ll_husb_sie_init(sie);
// Clear interrupts
    c67x00_ll_usb_clear_status(sie, HOST_STAT_MASK);
// Check
    if (!(hpi_read_word(sie.dev, USB_CTL_REG(sie.sie_num)) & HOST_MODE))
    dev_warn(sie_dev(sie),
    "SIE %d not set to host mode\n", sie.sie_num);
    }
#[no_mangle]
pub unsafe extern "C" fn c67x00_ll_husb_reset_port(sie: *mut c67x00_sie, port: c_int) {
    void c67x00_ll_husb_reset_port(struct c67x00_sie *sie, int port)
    {
// Clear connect change
    c67x00_ll_usb_clear_status(sie, PORT_CONNECT_CHANGE(port));
// Enable interrupts
    hpi_set_bits(sie.dev, HPI_IRQ_ROUTING_REG,
    SOFEOP_TO_CPU_EN(sie.sie_num));
    hpi_set_bits(sie.dev, HOST_IRQ_EN_REG(sie.sie_num),
    SOF_EOP_IRQ_EN | DONE_IRQ_EN);
// Enable pull down transistors
    hpi_set_bits(sie.dev, USB_CTL_REG(sie.sie_num), PORT_RES_EN(port));
    }
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn c67x00_ll_irq(dev: *mut c67x00_device, int_status: u16) {
    void c67x00_ll_irq(struct c67x00_device *dev, u16 int_status)
    {
    if ((int_status & MBX_OUT_FLG) == 0)
    return;
    dev.hpi.lcp.last_msg = hpi_recv_mbox(dev);
    complete(&dev.hpi.lcp.msg_received);
    }
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn c67x00_ll_reset(dev: *mut c67x00_device) -> c_int {
    int c67x00_ll_reset(struct c67x00_device *dev)
    {
    int rc;
    mutex_lock(&dev.hpi.lcp.mutex);
    hpi_send_mbox(dev, COMM_RESET);
    rc = ll_recv_msg(dev);
    mutex_unlock(&dev.hpi.lcp.mutex);
    return rc;
    }
// --------------------------------------------------------------------------
//
// c67x00_ll_write_mem_le16 - write into c67x00 memory
// Only data is little endian, addr has cpu endianess.
//
    void c67x00_ll_write_mem_le16(struct c67x00_device *dev, u16 addr,
    void *data, int len)
    {
    u8 *buf = data;
// Sanity check
    if (addr + len > 0xffff) {
    dev_err(&dev.pdev.dev,
    "Trying to write beyond writable region!\n");
    return;
    }
    if (addr & 0x01) {
// unaligned access
    u16 tmp;
    tmp = hpi_read_word(dev, addr - 1);
    tmp = (tmp & 0x00ff) | (*buf++ << 8);
    hpi_write_word(dev, addr - 1, tmp);
    addr++;
    len--;
    }
    hpi_write_words_le16(dev, addr, (__le16 *)buf, len / 2);
    buf += len & ~0x01;
    addr += len & ~0x01;
    len &= 0x01;
    if (len) {
    u16 tmp;
    tmp = hpi_read_word(dev, addr);
    tmp = (tmp & 0xff00) | *buf;
    hpi_write_word(dev, addr, tmp);
    }
    }
//
// c67x00_ll_read_mem_le16 - read from c67x00 memory
// Only data is little endian, addr has cpu endianess.
//
    void c67x00_ll_read_mem_le16(struct c67x00_device *dev, u16 addr,
    void *data, int len)
    {
    u8 *buf = data;
    if (addr & 0x01) {
// unaligned access
    u16 tmp;
    tmp = hpi_read_word(dev, addr - 1);
// buf++ = (tmp >> 8) & 0x00ff;
    addr++;
    len--;
    }
    hpi_read_words_le16(dev, addr, (__le16 *)buf, len / 2);
    buf += len & ~0x01;
    addr += len & ~0x01;
    len &= 0x01;
    if (len) {
    u16 tmp;
    tmp = hpi_read_word(dev, addr);
// buf = tmp & 0x00ff;
    }
    }
// --------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn c67x00_ll_init(dev: *mut c67x00_device) {
    void c67x00_ll_init(struct c67x00_device *dev)
    {
    mutex_init(&dev.hpi.lcp.mutex);
    init_completion(&dev.hpi.lcp.msg_received);
    }
#[no_mangle]
pub unsafe extern "C" fn c67x00_ll_release(dev: *mut c67x00_device) {
    void c67x00_ll_release(struct c67x00_device *dev)
    {
    }
