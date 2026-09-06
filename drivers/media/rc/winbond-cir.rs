//! Automatically rewritten from C to Rust
//! Source: drivers/media/rc/winbond-cir.c
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
// winbond-cir.c - Driver for the Consumer IR functionality of Winbond
// SuperI/O chips.
//
// Currently supports the Winbond WPCD376i chip (PNP id WEC1022), but
// could probably support others (Winbond WEC102X, NatSemi, etc)
// with minor modifications.
//
// Original Author: David Härdeman <david@hardeman.nu>
// Copyright (C) 2012 Sean Young <sean@mess.org>
// Copyright (C) 2009 - 2011 David Härdeman <david@hardeman.nu>
//
// Dedicated to my daughter Matilda, without whose loving attention this
// driver would have been finished in half the time and with a fraction
// of the bugs.
//
// Written using:
// o Winbond WPCD376I datasheet helpfully provided by Jesse Barnes at Intel
// o NatSemi PC87338/PC97338 datasheet (for the serial port stuff)
// o DSDT dumps
//
// Supported features:
// o IR Receive
// o IR Transmit
// o Wake-On-CIR functionality
// o Carrier detection
//

// CEIR Wake-Up Registers, relative to data->wbase
pub const WBCIR_REG_WCEIR_CTL: c_uint = 0x03 /* CEIR Receiver Control		*/;
pub const WBCIR_REG_WCEIR_STS: c_uint = 0x04 /* CEIR Receiver Status		*/;
pub const WBCIR_REG_WCEIR_EV_EN: c_uint = 0x05 /* CEIR Receiver Event Enable	*/;
pub const WBCIR_REG_WCEIR_CNTL: c_uint = 0x06 /* CEIR Receiver Counter Low	*/;
pub const WBCIR_REG_WCEIR_CNTH: c_uint = 0x07 /* CEIR Receiver Counter High	*/;
pub const WBCIR_REG_WCEIR_INDEX: c_uint = 0x08 /* CEIR Receiver Index		*/;
pub const WBCIR_REG_WCEIR_DATA: c_uint = 0x09 /* CEIR Receiver Data		*/;
pub const WBCIR_REG_WCEIR_CSL: c_uint = 0x0A /* CEIR Re. Compare Strlen		*/;
pub const WBCIR_REG_WCEIR_CFG1: c_uint = 0x0B /* CEIR Re. Configuration 1	*/;
pub const WBCIR_REG_WCEIR_CFG2: c_uint = 0x0C /* CEIR Re. Configuration 2	*/;
// CEIR Enhanced Functionality Registers, relative to data->ebase
pub const WBCIR_REG_ECEIR_CTS: c_uint = 0x00 /* Enhanced IR Control Status	*/;
pub const WBCIR_REG_ECEIR_CCTL: c_uint = 0x01 /* Infrared Counter Control	*/;
pub const WBCIR_REG_ECEIR_CNT_LO: c_uint = 0x02 /* Infrared Counter LSB		*/;
pub const WBCIR_REG_ECEIR_CNT_HI: c_uint = 0x03 /* Infrared Counter MSB		*/;
pub const WBCIR_REG_ECEIR_IREM: c_uint = 0x04 /* Infrared Emitter Status		*/;
// SP3 Banked Registers, relative to data->sbase
pub const WBCIR_REG_SP3_BSR: c_uint = 0x03 /* Bank Select, all banks		*/;
// Bank 0
pub const WBCIR_REG_SP3_RXDATA: c_uint = 0x00 /* FIFO RX data (r)		*/;
pub const WBCIR_REG_SP3_TXDATA: c_uint = 0x00 /* FIFO TX data (w)		*/;
pub const WBCIR_REG_SP3_IER: c_uint = 0x01 /* Interrupt Enable		*/;
pub const WBCIR_REG_SP3_EIR: c_uint = 0x02 /* Event Identification (r)	*/;
pub const WBCIR_REG_SP3_FCR: c_uint = 0x02 /* FIFO Control (w)		*/;
pub const WBCIR_REG_SP3_MCR: c_uint = 0x04 /* Mode Control			*/;
pub const WBCIR_REG_SP3_LSR: c_uint = 0x05 /* Link Status			*/;
pub const WBCIR_REG_SP3_MSR: c_uint = 0x06 /* Modem Status			*/;
pub const WBCIR_REG_SP3_ASCR: c_uint = 0x07 /* Aux Status and Control		*/;
// Bank 2
pub const WBCIR_REG_SP3_BGDL: c_uint = 0x00 /* Baud Divisor LSB		*/;
pub const WBCIR_REG_SP3_BGDH: c_uint = 0x01 /* Baud Divisor MSB		*/;
pub const WBCIR_REG_SP3_EXCR1: c_uint = 0x02 /* Extended Control 1		*/;
pub const WBCIR_REG_SP3_EXCR2: c_uint = 0x04 /* Extended Control 2		*/;
pub const WBCIR_REG_SP3_TXFLV: c_uint = 0x06 /* TX FIFO Level			*/;
pub const WBCIR_REG_SP3_RXFLV: c_uint = 0x07 /* RX FIFO Level			*/;
// Bank 3
pub const WBCIR_REG_SP3_MRID: c_uint = 0x00 /* Module Identification		*/;
pub const WBCIR_REG_SP3_SH_LCR: c_uint = 0x01 /* LCR Shadow			*/;
pub const WBCIR_REG_SP3_SH_FCR: c_uint = 0x02 /* FCR Shadow			*/;
// Bank 4
pub const WBCIR_REG_SP3_IRCR1: c_uint = 0x02 /* Infrared Control 1		*/;
// Bank 5
pub const WBCIR_REG_SP3_IRCR2: c_uint = 0x04 /* Infrared Control 2		*/;
// Bank 6
pub const WBCIR_REG_SP3_IRCR3: c_uint = 0x00 /* Infrared Control 3		*/;
pub const WBCIR_REG_SP3_SIR_PW: c_uint = 0x02 /* SIR Pulse Width			*/;
// Bank 7
pub const WBCIR_REG_SP3_IRRXDC: c_uint = 0x00 /* IR RX Demod Control		*/;
pub const WBCIR_REG_SP3_IRTXMC: c_uint = 0x01 /* IR TX Mod Control		*/;
pub const WBCIR_REG_SP3_RCCFG: c_uint = 0x02 /* CEIR Config			*/;
pub const WBCIR_REG_SP3_IRCFG1: c_uint = 0x04 /* Infrared Config 1		*/;
pub const WBCIR_REG_SP3_IRCFG4: c_uint = 0x07 /* Infrared Config 4		*/;
//
// Magic values follow
//
// No interrupts for WBCIR_REG_SP3_IER and WBCIR_REG_SP3_EIR
pub const WBCIR_IRQ_NONE: c_uint = 0x00;
// RX data bit for WBCIR_REG_SP3_IER and WBCIR_REG_SP3_EIR
pub const WBCIR_IRQ_RX: c_uint = 0x01;
// TX data low bit for WBCIR_REG_SP3_IER and WBCIR_REG_SP3_EIR
pub const WBCIR_IRQ_TX_LOW: c_uint = 0x02;
// Over/Under-flow bit for WBCIR_REG_SP3_IER and WBCIR_REG_SP3_EIR
pub const WBCIR_IRQ_ERR: c_uint = 0x04;
// TX data empty bit for WBCEIR_REG_SP3_IER and WBCIR_REG_SP3_EIR
pub const WBCIR_IRQ_TX_EMPTY: c_uint = 0x20;
// Led enable/disable bit for WBCIR_REG_ECEIR_CTS
pub const WBCIR_LED_ENABLE: c_uint = 0x80;
// RX data available bit for WBCIR_REG_SP3_LSR
pub const WBCIR_RX_AVAIL: c_uint = 0x01;
// RX data overrun error bit for WBCIR_REG_SP3_LSR
pub const WBCIR_RX_OVERRUN: c_uint = 0x02;
// TX End-Of-Transmission bit for WBCIR_REG_SP3_ASCR
pub const WBCIR_TX_EOT: c_uint = 0x04;
// RX disable bit for WBCIR_REG_SP3_ASCR
pub const WBCIR_RX_DISABLE: c_uint = 0x20;
// TX data underrun error bit for WBCIR_REG_SP3_ASCR
pub const WBCIR_TX_UNDERRUN: c_uint = 0x40;
// Extended mode enable bit for WBCIR_REG_SP3_EXCR1
pub const WBCIR_EXT_ENABLE: c_uint = 0x01;
// Select compare register in WBCIR_REG_WCEIR_INDEX (bits 5 & 6)
pub const WBCIR_REGSEL_COMPARE: c_uint = 0x10;
// Select mask register in WBCIR_REG_WCEIR_INDEX (bits 5 & 6)
pub const WBCIR_REGSEL_MASK: c_uint = 0x20;
// Starting address of selected register in WBCIR_REG_WCEIR_INDEX
pub const WBCIR_REG_ADDR0: c_uint = 0x00;
// Enable carrier counter
pub const WBCIR_CNTR_EN: c_uint = 0x01;
// Reset carrier counter
pub const WBCIR_CNTR_R: c_uint = 0x02;
// Invert TX
pub const WBCIR_IRTX_INV: c_uint = 0x04;
// Receiver oversampling
pub const WBCIR_RX_T_OV: c_uint = 0x40;
// Valid banks for the SP3 UART
    enum wbcir_bank {
    WBCIR_BANK_0          = 0x00,
    WBCIR_BANK_1          = 0x80,
    WBCIR_BANK_2          = 0xE0,
    WBCIR_BANK_3          = 0xE4,
    WBCIR_BANK_4          = 0xE8,
    WBCIR_BANK_5          = 0xEC,
    WBCIR_BANK_6          = 0xF0,
    WBCIR_BANK_7          = 0xF4,
    };
// Supported power-on IR Protocols
    enum wbcir_protocol {
    IR_PROTOCOL_RC5          = 0x0,
    IR_PROTOCOL_NEC          = 0x1,
    IR_PROTOCOL_RC6          = 0x2,
    };
// Possible states for IR reception
    enum wbcir_rxstate {
    WBCIR_RXSTATE_INACTIVE = 0,
    WBCIR_RXSTATE_ACTIVE,
    WBCIR_RXSTATE_ERROR
    };
// Possible states for IR transmission
    enum wbcir_txstate {
    WBCIR_TXSTATE_INACTIVE = 0,
    WBCIR_TXSTATE_ACTIVE,
    WBCIR_TXSTATE_ERROR
    };
// Misc

pub const WBCIR_ID_FAMILY: c_uint = 0xF1 /* Family ID for the WPCD376I	*/;
pub const WBCIR_ID_CHIP: c_uint = 0x04 /* Chip ID for the WPCD376I	*/;
pub const WAKEUP_IOMEM_LEN: c_uint = 0x10 /* Wake-Up I/O Reg Len		*/;
pub const EHFUNC_IOMEM_LEN: c_uint = 0x10 /* Enhanced Func I/O Reg Len	*/;
pub const SP_IOMEM_LEN: c_uint = 0x08 /* Serial Port 3 (IR) Reg Len	*/;
// Per-device data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wbcir_data {
    pub spinlock: spinlock_t,
    pub dev: *mut rc_dev,
    pub led: led_classdev,
    pub /: *mut *mut unsigned long wbase; / Wake-Up Baseaddr,
    pub /: *mut *mut unsigned long ebase; / Enhanced Func. Baseaddr,
    pub /: *mut *mut unsigned long sbase; / Serial Port Baseaddr,
    pub /: *mut *mut unsigned int irq; / Serial Port IRQ,
    pub irqmask: u8,
// RX state
    pub rxstate: enum wbcir_rxstate,
    pub carrier_report_enabled: c_int,
    pub pulse_duration: u32,
// TX state
    pub txstate: enum wbcir_txstate,
    pub txlen: u32,
    pub txoff: u32,
    pub txbuf: *mut u32,
    pub txmask: u8,
    pub txcarrier: u32,
}

    static bool invert; /* default = 0 */
    module_param(invert, bool, 0444);
    MODULE_PARM_DESC(invert, "Invert the signal from the IR receiver");
    static bool txandrx; /* default = 0 */
    module_param(txandrx, bool, 0444);
    MODULE_PARM_DESC(txandrx, "Allow simultaneous TX and RX");
//
// UTILITY FUNCTIONS
//
// Caller needs to hold wbcir_lock
    static void
    wbcir_set_bits(unsigned long addr, u8 bits, u8 mask)
    {
    u8 val;
    val = inb(addr);
    val = ((val & ~mask) | (bits & mask));
    outb(val, addr);
    }
// Selects the register bank for the serial port
    static inline void
    wbcir_select_bank(struct wbcir_data *data, enum wbcir_bank bank)
    {
    outb(bank, data.sbase + WBCIR_REG_SP3_BSR);
    }
    static inline void
    wbcir_set_irqmask(struct wbcir_data *data, u8 irqmask)
    {
    if (data.irqmask == irqmask)
    return;
    wbcir_select_bank(data, WBCIR_BANK_0);
    outb(irqmask, data.sbase + WBCIR_REG_SP3_IER);
    data.irqmask = irqmask;
    }
    static enum led_brightness
    wbcir_led_brightness_get(struct led_classdev *led_cdev)
    {
    struct wbcir_data *data = container_of(led_cdev,
    struct wbcir_data,
    led);
    if (inb(data.ebase + WBCIR_REG_ECEIR_CTS) & WBCIR_LED_ENABLE)
    return LED_FULL;
    else
    return LED_OFF;
    }
    static void
    wbcir_led_brightness_set(struct led_classdev *led_cdev,
    enum led_brightness brightness)
    {
    struct wbcir_data *data = container_of(led_cdev,
    struct wbcir_data,
    led);
    wbcir_set_bits(data.ebase + WBCIR_REG_ECEIR_CTS,
    brightness == LED_OFF ? 0x00 : WBCIR_LED_ENABLE,
    WBCIR_LED_ENABLE);
    }
// Manchester encodes bits to RC6 message cells (see wbcir_shutdown)
    static u8
    wbcir_to_rc6cells(u8 val)
    {
    let mut coded: u8 = 0x00;
    int i;
    val &= 0x0F;
    for (i = 0; i < 4; i++) {
    if (val & 0x01)
    coded |= 0x02 << (i * 2);
    else
    coded |= 0x01 << (i * 2);
    val >>= 1;
    }
    return coded;
    }
//
// INTERRUPT FUNCTIONS
//
    static void
    wbcir_carrier_report(struct wbcir_data *data)
    {
    unsigned counter = inb(data.ebase + WBCIR_REG_ECEIR_CNT_LO) |
    inb(data.ebase + WBCIR_REG_ECEIR_CNT_HI) << 8;
    if (counter > 0 && counter < 0xffff) {
    struct ir_raw_event ev = {
    .carrier_report = 1,
    .carrier = DIV_ROUND_CLOSEST(counter * 1000000u,
    data.pulse_duration)
    };
    ir_raw_event_store(data.dev, &ev);
    }
// reset and restart the counter
    data.pulse_duration = 0;
    wbcir_set_bits(data.ebase + WBCIR_REG_ECEIR_CCTL, WBCIR_CNTR_R,
    WBCIR_CNTR_EN | WBCIR_CNTR_R);
    wbcir_set_bits(data.ebase + WBCIR_REG_ECEIR_CCTL, WBCIR_CNTR_EN,
    WBCIR_CNTR_EN | WBCIR_CNTR_R);
    }
    static void
    wbcir_idle_rx(struct rc_dev *dev, bool idle)
    {
    struct wbcir_data *data = dev.priv;
    if (!idle && data.rxstate == WBCIR_RXSTATE_INACTIVE)
    data.rxstate = WBCIR_RXSTATE_ACTIVE;
    if (idle && data.rxstate != WBCIR_RXSTATE_INACTIVE) {
    data.rxstate = WBCIR_RXSTATE_INACTIVE;
    if (data.carrier_report_enabled)
    wbcir_carrier_report(data);
// Tell hardware to go idle by setting RXINACTIVE
    outb(WBCIR_RX_DISABLE, data.sbase + WBCIR_REG_SP3_ASCR);
    }
    }
    static void
    wbcir_irq_rx(struct wbcir_data *data, struct pnp_dev *device)
    {
    u8 irdata;
    let mut rawir: ir_raw_event = {};
// Since RXHDLEV is set, at least 8 bytes are in the FIFO
    while (inb(data.sbase + WBCIR_REG_SP3_LSR) & WBCIR_RX_AVAIL) {
    irdata = inb(data.sbase + WBCIR_REG_SP3_RXDATA);
    if (data.rxstate == WBCIR_RXSTATE_ERROR)
    continue;
    rawir.duration = ((irdata & 0x7F) + 1) *
    (data.carrier_report_enabled ? 2 : 10);
    rawir.pulse = irdata & 0x80 ? false : true;
    if (rawir.pulse)
    data.pulse_duration += rawir.duration;
    ir_raw_event_store_with_filter(data.dev, &rawir);
    }
    ir_raw_event_handle(data.dev);
    }
    static void
    wbcir_irq_tx(struct wbcir_data *data)
    {
    unsigned int space;
    unsigned int used;
    u8 bytes[16];
    u8 byte;
    if (!data.txbuf)
    return;
    switch (data.txstate) {
    case WBCIR_TXSTATE_INACTIVE:
// TX FIFO empty
    space = 16;
    break;
    case WBCIR_TXSTATE_ACTIVE:
// TX FIFO low (3 bytes or less)
    space = 13;
    break;
    case WBCIR_TXSTATE_ERROR:
    space = 0;
    break;
    default:
    return;
    }
//
// TX data is run-length coded in bytes: YXXXXXXX
// Y = space (1) or pulse (0)
// X = duration, encoded as (X + 1) * 10us (i.e 10 to 1280 us)
//
    for (used = 0; used < space && data.txoff != data.txlen; used++) {
    if (data.txbuf[data.txoff] == 0) {
    data.txoff++;
    continue;
    }
    byte = min((u32)0x80, data.txbuf[data.txoff]);
    data.txbuf[data.txoff] -= byte;
    byte--;
    byte |= (data.txoff % 2 ? 0x80 : 0x00); /* pulse/space */
    bytes[used] = byte;
    }
    while (data.txoff != data.txlen && data.txbuf[data.txoff] == 0)
    data.txoff++;
    if (used == 0) {
// Finished
    if (data.txstate == WBCIR_TXSTATE_ERROR)
// Clear TX underrun bit
    outb(WBCIR_TX_UNDERRUN, data.sbase + WBCIR_REG_SP3_ASCR);
    wbcir_set_irqmask(data, WBCIR_IRQ_RX | WBCIR_IRQ_ERR);
    kfree(data.txbuf);
    data.txbuf = core::ptr::null_mut();
    data.txstate = WBCIR_TXSTATE_INACTIVE;
    } else if (data.txoff == data.txlen) {
// At the end of transmission, tell the hw before last byte
    outsb(data.sbase + WBCIR_REG_SP3_TXDATA, bytes, used - 1);
    outb(WBCIR_TX_EOT, data.sbase + WBCIR_REG_SP3_ASCR);
    outb(bytes[used - 1], data.sbase + WBCIR_REG_SP3_TXDATA);
    wbcir_set_irqmask(data, WBCIR_IRQ_RX | WBCIR_IRQ_ERR |
    WBCIR_IRQ_TX_EMPTY);
    } else {
// More data to follow...
    outsb(data.sbase + WBCIR_REG_SP3_RXDATA, bytes, used);
    if (data.txstate == WBCIR_TXSTATE_INACTIVE) {
    wbcir_set_irqmask(data, WBCIR_IRQ_RX | WBCIR_IRQ_ERR |
    WBCIR_IRQ_TX_LOW);
    data.txstate = WBCIR_TXSTATE_ACTIVE;
    }
    }
    }
    static irqreturn_t
    wbcir_irq_handler(int irqno, void *cookie)
    {
    struct pnp_dev *device = cookie;
    struct wbcir_data *data = pnp_get_drvdata(device);
    unsigned long flags;
    u8 status;
    spin_lock_irqsave(&data.spinlock, flags);
    wbcir_select_bank(data, WBCIR_BANK_0);
    status = inb(data.sbase + WBCIR_REG_SP3_EIR);
    status &= data.irqmask;
    if (!status) {
    spin_unlock_irqrestore(&data.spinlock, flags);
    return IRQ_NONE;
    }
    if (status & WBCIR_IRQ_ERR) {
// RX overflow? (read clears bit)
    if (inb(data.sbase + WBCIR_REG_SP3_LSR) & WBCIR_RX_OVERRUN) {
    data.rxstate = WBCIR_RXSTATE_ERROR;
    ir_raw_event_overflow(data.dev);
    }
// TX underflow?
    if (inb(data.sbase + WBCIR_REG_SP3_ASCR) & WBCIR_TX_UNDERRUN)
    data.txstate = WBCIR_TXSTATE_ERROR;
    }
    if (status & WBCIR_IRQ_RX)
    wbcir_irq_rx(data, device);
    if (status & (WBCIR_IRQ_TX_LOW | WBCIR_IRQ_TX_EMPTY))
    wbcir_irq_tx(data);
    spin_unlock_irqrestore(&data.spinlock, flags);
    return IRQ_HANDLED;
    }
//
// RC-CORE INTERFACE FUNCTIONS
//
    static int
    wbcir_set_carrier_report(struct rc_dev *dev, int enable)
    {
    struct wbcir_data *data = dev.priv;
    unsigned long flags;
    spin_lock_irqsave(&data.spinlock, flags);
    if (data.carrier_report_enabled == enable) {
    spin_unlock_irqrestore(&data.spinlock, flags);
    return 0;
    }
    data.pulse_duration = 0;
    wbcir_set_bits(data.ebase + WBCIR_REG_ECEIR_CCTL, WBCIR_CNTR_R,
    WBCIR_CNTR_EN | WBCIR_CNTR_R);
    if (enable && data.dev.idle)
    wbcir_set_bits(data.ebase + WBCIR_REG_ECEIR_CCTL,
    WBCIR_CNTR_EN, WBCIR_CNTR_EN | WBCIR_CNTR_R);
// Set a higher sampling resolution if carrier reports are enabled
    wbcir_select_bank(data, WBCIR_BANK_2);
    data.dev.rx_resolution = enable ? 2 : 10;
    outb(enable ? 0x03 : 0x0f, data.sbase + WBCIR_REG_SP3_BGDL);
    outb(0x00, data.sbase + WBCIR_REG_SP3_BGDH);
// Enable oversampling if carrier reports are enabled
    wbcir_select_bank(data, WBCIR_BANK_7);
    wbcir_set_bits(data.sbase + WBCIR_REG_SP3_RCCFG,
    enable ? WBCIR_RX_T_OV : 0, WBCIR_RX_T_OV);
    data.carrier_report_enabled = enable;
    spin_unlock_irqrestore(&data.spinlock, flags);
    return 0;
    }
    static int
    wbcir_txcarrier(struct rc_dev *dev, u32 carrier)
    {
    struct wbcir_data *data = dev.priv;
    unsigned long flags;
    u8 val;
    u32 freq;
    freq = DIV_ROUND_CLOSEST(carrier, 1000);
    if (freq < 30 || freq > 60)
    return -EINVAL;
    switch (freq) {
    case 58:
    case 59:
    case 60:
    val = freq - 58;
    freq *= 1000;
    break;
    case 57:
    val = freq - 27;
    freq = 56900;
    break;
    default:
    val = freq - 27;
    freq *= 1000;
    break;
    }
    spin_lock_irqsave(&data.spinlock, flags);
    if (data.txstate != WBCIR_TXSTATE_INACTIVE) {
    spin_unlock_irqrestore(&data.spinlock, flags);
    return -EBUSY;
    }
    if (data.txcarrier != freq) {
    wbcir_select_bank(data, WBCIR_BANK_7);
    wbcir_set_bits(data.sbase + WBCIR_REG_SP3_IRTXMC, val, 0x1F);
    data.txcarrier = freq;
    }
    spin_unlock_irqrestore(&data.spinlock, flags);
    return 0;
    }
    static int
    wbcir_txmask(struct rc_dev *dev, u32 mask)
    {
    struct wbcir_data *data = dev.priv;
    unsigned long flags;
    u8 val;
// return the number of transmitters
    if (mask > 15)
    return 4;
// Four outputs, only one output can be enabled at a time
    switch (mask) {
    case 0x1:
    val = 0x0;
    break;
    case 0x2:
    val = 0x1;
    break;
    case 0x4:
    val = 0x2;
    break;
    case 0x8:
    val = 0x3;
    break;
    default:
    return -EINVAL;
    }
    spin_lock_irqsave(&data.spinlock, flags);
    if (data.txstate != WBCIR_TXSTATE_INACTIVE) {
    spin_unlock_irqrestore(&data.spinlock, flags);
    return -EBUSY;
    }
    if (data.txmask != mask) {
    wbcir_set_bits(data.ebase + WBCIR_REG_ECEIR_CTS, val, 0x0c);
    data.txmask = mask;
    }
    spin_unlock_irqrestore(&data.spinlock, flags);
    return 0;
    }
    static int
    wbcir_tx(struct rc_dev *dev, unsigned *b, unsigned count)
    {
    struct wbcir_data *data = dev.priv;
    unsigned *buf;
    unsigned i;
    unsigned long flags;
    buf = kmalloc_array(count, sizeof(*b), GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
// Convert values to multiples of 10us
    for (i = 0; i < count; i++)
    buf[i] = DIV_ROUND_CLOSEST(b[i], 10);
// Not sure if this is possible, but better safe than sorry
    spin_lock_irqsave(&data.spinlock, flags);
    if (data.txstate != WBCIR_TXSTATE_INACTIVE) {
    spin_unlock_irqrestore(&data.spinlock, flags);
    kfree(buf);
    return -EBUSY;
    }
// Fill the TX fifo once, the irq handler will do the rest
    data.txbuf = buf;
    data.txlen = count;
    data.txoff = 0;
    wbcir_irq_tx(data);
// We're done
    spin_unlock_irqrestore(&data.spinlock, flags);
    return count;
    }
//
// SETUP/INIT/SUSPEND/RESUME FUNCTIONS
//
    static void
    wbcir_shutdown(struct pnp_dev *device)
    {
    struct device *dev = &device.dev;
    struct wbcir_data *data = pnp_get_drvdata(device);
    struct rc_dev *rc = data.dev;
    let mut do_wake: bool = true;
    u8 match[11];
    u8 mask[11];
    let mut rc6_csl: u8 = 0;
    u8 proto;
    let mut wake_sc: u32 = rc.scancode_wakeup_filter.data;
    let mut mask_sc: u32 = rc.scancode_wakeup_filter.mask;
    int i;
    memset(match, 0, sizeof(match));
    memset(mask, 0, sizeof(mask));
    if (!mask_sc || !device_may_wakeup(dev)) {
    do_wake = false;
    goto finish;
    }
    switch (rc.wakeup_protocol) {
    case RC_PROTO_RC5:
// Mask = 13 bits, ex toggle
    mask[0]  = (mask_sc & 0x003f);
    mask[0] |= (mask_sc & 0x0300) >> 2;
    mask[1]  = (mask_sc & 0x1c00) >> 10;
    if (mask_sc & 0x0040)		      /* 2nd start bit  */
    match[1] |= 0x10;
    match[0]  = (wake_sc & 0x003F);       /* 6 command bits */
    match[0] |= (wake_sc & 0x0300) >> 2;  /* 2 address bits */
    match[1]  = (wake_sc & 0x1c00) >> 10; /* 3 address bits */
    if (!(wake_sc & 0x0040))	      /* 2nd start bit  */
    match[1] |= 0x10;
    proto = IR_PROTOCOL_RC5;
    break;
    case RC_PROTO_NEC:
    mask[1] = bitrev8(mask_sc);
    mask[0] = mask[1];
    mask[3] = bitrev8(mask_sc >> 8);
    mask[2] = mask[3];
    match[1] = bitrev8(wake_sc);
    match[0] = ~match[1];
    match[3] = bitrev8(wake_sc >> 8);
    match[2] = ~match[3];
    proto = IR_PROTOCOL_NEC;
    break;
    case RC_PROTO_NECX:
    mask[1] = bitrev8(mask_sc);
    mask[0] = mask[1];
    mask[2] = bitrev8(mask_sc >> 8);
    mask[3] = bitrev8(mask_sc >> 16);
    match[1] = bitrev8(wake_sc);
    match[0] = ~match[1];
    match[2] = bitrev8(wake_sc >> 8);
    match[3] = bitrev8(wake_sc >> 16);
    proto = IR_PROTOCOL_NEC;
    break;
    case RC_PROTO_NEC32:
    mask[0] = bitrev8(mask_sc);
    mask[1] = bitrev8(mask_sc >> 8);
    mask[2] = bitrev8(mask_sc >> 16);
    mask[3] = bitrev8(mask_sc >> 24);
    match[0] = bitrev8(wake_sc);
    match[1] = bitrev8(wake_sc >> 8);
    match[2] = bitrev8(wake_sc >> 16);
    match[3] = bitrev8(wake_sc >> 24);
    proto = IR_PROTOCOL_NEC;
    break;
    case RC_PROTO_RC6_0:
// Command
    match[0] = wbcir_to_rc6cells(wake_sc >> 0);
    mask[0]  = wbcir_to_rc6cells(mask_sc >> 0);
    match[1] = wbcir_to_rc6cells(wake_sc >> 4);
    mask[1]  = wbcir_to_rc6cells(mask_sc >> 4);
// Address
    match[2] = wbcir_to_rc6cells(wake_sc >>  8);
    mask[2]  = wbcir_to_rc6cells(mask_sc >>  8);
    match[3] = wbcir_to_rc6cells(wake_sc >> 12);
    mask[3]  = wbcir_to_rc6cells(mask_sc >> 12);
// Header
    match[4] = 0x50; /* mode1 = mode0 = 0, ignore toggle */
    mask[4]  = 0xF0;
    match[5] = 0x09; /* start bit = 1, mode2 = 0 */
    mask[5]  = 0x0F;
    rc6_csl = 44;
    proto = IR_PROTOCOL_RC6;
    break;
    case RC_PROTO_RC6_6A_24:
    case RC_PROTO_RC6_6A_32:
    case RC_PROTO_RC6_MCE:
    i = 0;
// Command
    match[i]  = wbcir_to_rc6cells(wake_sc >>  0);
    mask[i++] = wbcir_to_rc6cells(mask_sc >>  0);
    match[i]  = wbcir_to_rc6cells(wake_sc >>  4);
    mask[i++] = wbcir_to_rc6cells(mask_sc >>  4);
// Address + Toggle
    match[i]  = wbcir_to_rc6cells(wake_sc >>  8);
    mask[i++] = wbcir_to_rc6cells(mask_sc >>  8);
    match[i]  = wbcir_to_rc6cells(wake_sc >> 12);
    mask[i++] = wbcir_to_rc6cells(mask_sc >> 12);
// Customer bits 7 - 0
    match[i]  = wbcir_to_rc6cells(wake_sc >> 16);
    mask[i++] = wbcir_to_rc6cells(mask_sc >> 16);
    if (rc.wakeup_protocol == RC_PROTO_RC6_6A_20) {
    rc6_csl = 52;
    } else {
    match[i]  = wbcir_to_rc6cells(wake_sc >> 20);
    mask[i++] = wbcir_to_rc6cells(mask_sc >> 20);
    if (rc.wakeup_protocol == RC_PROTO_RC6_6A_24) {
    rc6_csl = 60;
    } else {
// Customer range bit and bits 15 - 8
    match[i]  = wbcir_to_rc6cells(wake_sc >> 24);
    mask[i++] = wbcir_to_rc6cells(mask_sc >> 24);
    match[i]  = wbcir_to_rc6cells(wake_sc >> 28);
    mask[i++] = wbcir_to_rc6cells(mask_sc >> 28);
    rc6_csl = 76;
    }
    }
// Header
    match[i]  = 0x93; /* mode1 = mode0 = 1, submode = 0 */
    mask[i++] = 0xFF;
    match[i]  = 0x0A; /* start bit = 1, mode2 = 1 */
    mask[i++] = 0x0F;
    proto = IR_PROTOCOL_RC6;
    break;
    default:
    do_wake = false;
    break;
    }
    finish:
    if (do_wake) {
// Set compare and compare mask
    wbcir_set_bits(data.wbase + WBCIR_REG_WCEIR_INDEX,
    WBCIR_REGSEL_COMPARE | WBCIR_REG_ADDR0,
    0x3F);
    outsb(data.wbase + WBCIR_REG_WCEIR_DATA, match, 11);
    wbcir_set_bits(data.wbase + WBCIR_REG_WCEIR_INDEX,
    WBCIR_REGSEL_MASK | WBCIR_REG_ADDR0,
    0x3F);
    outsb(data.wbase + WBCIR_REG_WCEIR_DATA, mask, 11);
// RC6 Compare String Len
    outb(rc6_csl, data.wbase + WBCIR_REG_WCEIR_CSL);
// Clear status bits NEC_REP, BUFF, MSG_END, MATCH
    wbcir_set_bits(data.wbase + WBCIR_REG_WCEIR_STS, 0x17, 0x17);
// Clear BUFF_EN, Clear END_EN, Set MATCH_EN
    wbcir_set_bits(data.wbase + WBCIR_REG_WCEIR_EV_EN, 0x01, 0x07);
// Set CEIR_EN
    wbcir_set_bits(data.wbase + WBCIR_REG_WCEIR_CTL,
    (proto << 4) | 0x01, 0x31);
    } else {
// Clear BUFF_EN, Clear END_EN, Clear MATCH_EN
    wbcir_set_bits(data.wbase + WBCIR_REG_WCEIR_EV_EN, 0x00, 0x07);
// Clear CEIR_EN
    wbcir_set_bits(data.wbase + WBCIR_REG_WCEIR_CTL, 0x00, 0x01);
    }
//
// ACPI will set the HW disable bit for SP3 which means that the
// output signals are left in an undefined state which may cause
// spurious interrupts which we need to ignore until the hardware
// is reinitialized.
//
    wbcir_set_irqmask(data, WBCIR_IRQ_NONE);
    disable_irq(data.irq);
    }
//
// Wakeup handling is done on shutdown.
//
    static int
    wbcir_set_wakeup_filter(struct rc_dev *rc, struct rc_scancode_filter *filter)
    {
    return 0;
    }
    static int
    wbcir_suspend(struct pnp_dev *device, pm_message_t state)
    {
    struct wbcir_data *data = pnp_get_drvdata(device);
    led_classdev_suspend(&data.led);
    wbcir_shutdown(device);
    return 0;
    }
    static void
    wbcir_init_hw(struct wbcir_data *data)
    {
// Disable interrupts
    wbcir_set_irqmask(data, WBCIR_IRQ_NONE);
// Set RX_INV, Clear CEIR_EN (needed for the led)
    wbcir_set_bits(data.wbase + WBCIR_REG_WCEIR_CTL, invert ? 8 : 0, 0x09);
// Clear status bits NEC_REP, BUFF, MSG_END, MATCH
    wbcir_set_bits(data.wbase + WBCIR_REG_WCEIR_STS, 0x17, 0x17);
// Clear BUFF_EN, Clear END_EN, Clear MATCH_EN
    wbcir_set_bits(data.wbase + WBCIR_REG_WCEIR_EV_EN, 0x00, 0x07);
// Set RC5 cell time to correspond to 36 kHz
    wbcir_set_bits(data.wbase + WBCIR_REG_WCEIR_CFG1, 0x4A, 0x7F);
// Set IRTX_INV
    if (invert)
    outb(WBCIR_IRTX_INV, data.ebase + WBCIR_REG_ECEIR_CCTL);
    else
    outb(0x00, data.ebase + WBCIR_REG_ECEIR_CCTL);
//
// Clear IR LED, set SP3 clock to 24Mhz, set TX mask to IRTX1,
// set SP3_IRRX_SW to binary 01, helpfully not documented
//
    outb(0x10, data.ebase + WBCIR_REG_ECEIR_CTS);
    data.txmask = 0x1;
// Enable extended mode
    wbcir_select_bank(data, WBCIR_BANK_2);
    outb(WBCIR_EXT_ENABLE, data.sbase + WBCIR_REG_SP3_EXCR1);
//
// Configure baud generator, IR data will be sampled at
// a bitrate of: (24Mhz * prescaler) / (divisor * 16).
//
// The ECIR registers include a flag to change the
// 24Mhz clock freq to 48Mhz.
//
// It's not documented in the specs, but fifo levels
// other than 16 seems to be unsupported.
//
// prescaler 1.0, tx/rx fifo lvl 16
    outb(0x30, data.sbase + WBCIR_REG_SP3_EXCR2);
// Set baud divisor to sample every 10 us
    outb(0x0f, data.sbase + WBCIR_REG_SP3_BGDL);
    outb(0x00, data.sbase + WBCIR_REG_SP3_BGDH);
// Set CEIR mode
    wbcir_select_bank(data, WBCIR_BANK_0);
    outb(0xC0, data.sbase + WBCIR_REG_SP3_MCR);
    inb(data.sbase + WBCIR_REG_SP3_LSR); /* Clear LSR */
    inb(data.sbase + WBCIR_REG_SP3_MSR); /* Clear MSR */
// Disable RX demod, enable run-length enc/dec, set freq span
    wbcir_select_bank(data, WBCIR_BANK_7);
    outb(0x90, data.sbase + WBCIR_REG_SP3_RCCFG);
// Disable timer
    wbcir_select_bank(data, WBCIR_BANK_4);
    outb(0x00, data.sbase + WBCIR_REG_SP3_IRCR1);
// Disable MSR interrupt, clear AUX_IRX, mask RX during TX?
    wbcir_select_bank(data, WBCIR_BANK_5);
    outb(txandrx ? 0x03 : 0x02, data.sbase + WBCIR_REG_SP3_IRCR2);
// Disable CRC
    wbcir_select_bank(data, WBCIR_BANK_6);
    outb(0x20, data.sbase + WBCIR_REG_SP3_IRCR3);
// Set RX demodulation freq, not really used
    wbcir_select_bank(data, WBCIR_BANK_7);
    outb(0xF2, data.sbase + WBCIR_REG_SP3_IRRXDC);
// Set TX modulation, 36kHz, 7us pulse width
    outb(0x69, data.sbase + WBCIR_REG_SP3_IRTXMC);
    data.txcarrier = 36000;
// Set invert and pin direction
    if (invert)
    outb(0x10, data.sbase + WBCIR_REG_SP3_IRCFG4);
    else
    outb(0x00, data.sbase + WBCIR_REG_SP3_IRCFG4);
// Set FIFO thresholds (RX = 8, TX = 3), reset RX/TX
    wbcir_select_bank(data, WBCIR_BANK_0);
    outb(0x97, data.sbase + WBCIR_REG_SP3_FCR);
// Clear AUX status bits
    outb(0xE0, data.sbase + WBCIR_REG_SP3_ASCR);
// Clear RX state
    data.rxstate = WBCIR_RXSTATE_INACTIVE;
    wbcir_idle_rx(data.dev, true);
// Clear TX state
    if (data.txstate == WBCIR_TXSTATE_ACTIVE) {
    kfree(data.txbuf);
    data.txbuf = core::ptr::null_mut();
    data.txstate = WBCIR_TXSTATE_INACTIVE;
    }
// Enable interrupts
    wbcir_set_irqmask(data, WBCIR_IRQ_RX | WBCIR_IRQ_ERR);
    }
    static int
    wbcir_resume(struct pnp_dev *device)
    {
    struct wbcir_data *data = pnp_get_drvdata(device);
    wbcir_init_hw(data);
    enable_irq(data.irq);
    led_classdev_resume(&data.led);
    return 0;
    }
    static int
    wbcir_probe(struct pnp_dev *device, const struct pnp_device_id *dev_id)
    {
    struct device *dev = &device.dev;
    struct wbcir_data *data;
    int err;
    if (!(pnp_port_len(device, 0) == EHFUNC_IOMEM_LEN &&
    pnp_port_len(device, 1) == WAKEUP_IOMEM_LEN &&
    pnp_port_len(device, 2) == SP_IOMEM_LEN)) {
    dev_err(dev, "Invalid resources\n");
    return -ENODEV;
    }
    data = kzalloc_obj(*data);
    if (!data) {
    err = -ENOMEM;
    goto exit;
    }
    pnp_set_drvdata(device, data);
    spin_lock_init(&data.spinlock);
    data.ebase = pnp_port_start(device, 0);
    data.wbase = pnp_port_start(device, 1);
    data.sbase = pnp_port_start(device, 2);
    data.irq = pnp_irq(device, 0);
    if (data.wbase == 0 || data.ebase == 0 ||
    data.sbase == 0 || data.irq == -1) {
    err = -ENODEV;
    dev_err(dev, "Invalid resources\n");
    goto exit_free_data;
    }
    dev_dbg(&device.dev, "Found device (w: 0x%lX, e: 0x%lX, s: 0x%lX, i: %u)\n",
    data.wbase, data.ebase, data.sbase, data.irq);
    data.led.name = "cir::activity";
    data.led.default_trigger = "rc-feedback";
    data.led.brightness_set = wbcir_led_brightness_set;
    data.led.brightness_get = wbcir_led_brightness_get;
    err = led_classdev_register(&device.dev, &data.led);
    if (err)
    goto exit_free_data;
    data.dev = rc_allocate_device(RC_DRIVER_IR_RAW);
    if (!data.dev) {
    err = -ENOMEM;
    goto exit_unregister_led;
    }
    data.dev.driver_name = DRVNAME;
    data.dev.device_name = WBCIR_NAME;
    data.dev.input_phys = "wbcir/cir0";
    data.dev.input_id.bustype = BUS_HOST;
    data.dev.input_id.vendor = PCI_VENDOR_ID_WINBOND;
    data.dev.input_id.product = WBCIR_ID_FAMILY;
    data.dev.input_id.version = WBCIR_ID_CHIP;
    data.dev.map_name = RC_MAP_RC6_MCE;
    data.dev.s_idle = wbcir_idle_rx;
    data.dev.s_carrier_report = wbcir_set_carrier_report;
    data.dev.s_tx_mask = wbcir_txmask;
    data.dev.s_tx_carrier = wbcir_txcarrier;
    data.dev.tx_ir = wbcir_tx;
    data.dev.priv = data;
    data.dev.dev.parent = &device.dev;
    data.dev.min_timeout = 1;
    data.dev.timeout = IR_DEFAULT_TIMEOUT;
    data.dev.max_timeout = 10 * IR_DEFAULT_TIMEOUT;
    data.dev.rx_resolution = 2;
    data.dev.allowed_protocols = RC_PROTO_BIT_ALL_IR_DECODER;
    data.dev.allowed_wakeup_protocols = RC_PROTO_BIT_NEC |
    RC_PROTO_BIT_NECX | RC_PROTO_BIT_NEC32 | RC_PROTO_BIT_RC5 |
    RC_PROTO_BIT_RC6_0 | RC_PROTO_BIT_RC6_6A_20 |
    RC_PROTO_BIT_RC6_6A_24 | RC_PROTO_BIT_RC6_6A_32 |
    RC_PROTO_BIT_RC6_MCE;
    data.dev.wakeup_protocol = RC_PROTO_RC6_MCE;
    data.dev.scancode_wakeup_filter.data = 0x800f040c;
    data.dev.scancode_wakeup_filter.mask = 0xffff7fff;
    data.dev.s_wakeup_filter = wbcir_set_wakeup_filter;
    err = rc_register_device(data.dev);
    if (err)
    goto exit_free_rc;
    if (!request_region(data.wbase, WAKEUP_IOMEM_LEN, DRVNAME)) {
    dev_err(dev, "Region 0x%lx-0x%lx already in use!\n",
    data.wbase, data.wbase + WAKEUP_IOMEM_LEN - 1);
    err = -EBUSY;
    goto exit_unregister_device;
    }
    if (!request_region(data.ebase, EHFUNC_IOMEM_LEN, DRVNAME)) {
    dev_err(dev, "Region 0x%lx-0x%lx already in use!\n",
    data.ebase, data.ebase + EHFUNC_IOMEM_LEN - 1);
    err = -EBUSY;
    goto exit_release_wbase;
    }
    if (!request_region(data.sbase, SP_IOMEM_LEN, DRVNAME)) {
    dev_err(dev, "Region 0x%lx-0x%lx already in use!\n",
    data.sbase, data.sbase + SP_IOMEM_LEN - 1);
    err = -EBUSY;
    goto exit_release_ebase;
    }
    err = request_irq(data.irq, wbcir_irq_handler,
    0, DRVNAME, device);
    if (err) {
    dev_err(dev, "Failed to claim IRQ %u\n", data.irq);
    err = -EBUSY;
    goto exit_release_sbase;
    }
    device_init_wakeup(&device.dev, 1);
    wbcir_init_hw(data);
    return 0;
    exit_release_sbase:
    release_region(data.sbase, SP_IOMEM_LEN);
    exit_release_ebase:
    release_region(data.ebase, EHFUNC_IOMEM_LEN);
    exit_release_wbase:
    release_region(data.wbase, WAKEUP_IOMEM_LEN);
    exit_unregister_device:
    rc_unregister_device(data.dev);
    exit_free_rc:
    rc_free_device(data.dev);
    exit_unregister_led:
    led_classdev_unregister(&data.led);
    exit_free_data:
    kfree(data);
    pnp_set_drvdata(device, core::ptr::null_mut());
    exit:
    return err;
    }
    static void
    wbcir_remove(struct pnp_dev *device)
    {
    struct wbcir_data *data = pnp_get_drvdata(device);
// Disable interrupts
    wbcir_set_irqmask(data, WBCIR_IRQ_NONE);
    free_irq(data.irq, device);
// Clear status bits NEC_REP, BUFF, MSG_END, MATCH
    wbcir_set_bits(data.wbase + WBCIR_REG_WCEIR_STS, 0x17, 0x17);
// Clear CEIR_EN
    wbcir_set_bits(data.wbase + WBCIR_REG_WCEIR_CTL, 0x00, 0x01);
// Clear BUFF_EN, END_EN, MATCH_EN
    wbcir_set_bits(data.wbase + WBCIR_REG_WCEIR_EV_EN, 0x00, 0x07);
    rc_unregister_device(data.dev);
    rc_free_device(data.dev);
    led_classdev_unregister(&data.led);
// This is ok since &data->led isn't actually used
    wbcir_led_brightness_set(&data.led, LED_OFF);
    release_region(data.wbase, WAKEUP_IOMEM_LEN);
    release_region(data.ebase, EHFUNC_IOMEM_LEN);
    release_region(data.sbase, SP_IOMEM_LEN);
    kfree(data);
    pnp_set_drvdata(device, core::ptr::null_mut());
    }
    static const struct pnp_device_id wbcir_ids[] = {
    { .id = "WEC1022" },
    { }
    };
    MODULE_DEVICE_TABLE(pnp, wbcir_ids);
    static struct pnp_driver wbcir_driver = {
    .name     = DRVNAME,
    .id_table = wbcir_ids,
    .probe    = wbcir_probe,
    .remove   = wbcir_remove,
    .suspend  = wbcir_suspend,
    .resume   = wbcir_resume,
    .shutdown = wbcir_shutdown
    };
    static int __init
    wbcir_init(void)
    {
    int ret;
    ret = pnp_register_driver(&wbcir_driver);
    if (ret)
    pr_err("Unable to register driver\n");
    return ret;
    }
    static void __exit
    wbcir_exit(void)
    {
    pnp_unregister_driver(&wbcir_driver);
    }
    module_init(wbcir_init);
    module_exit(wbcir_exit);
    MODULE_AUTHOR("David Härdeman <david@hardeman.nu>");
    MODULE_DESCRIPTION("Winbond SuperI/O Consumer IR Driver");
    MODULE_LICENSE("GPL");
