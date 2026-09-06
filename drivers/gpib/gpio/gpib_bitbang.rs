//! Automatically rewritten from C to Rust
//! Source: drivers/gpib/gpio/gpib_bitbang.c
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
// This code has been developed at the Institute of Sensor and Actuator
// Systems (Technical University of Vienna, Austria) to enable the GPIO
// lines (e.g. of a raspberry pi) to function as a GPIO master device
//
// authors		 : Thomas Klima
// Marcello Carla'
// Dave Penkler
//
// copyright		 : (C) 2016 Thomas Klima
//
// limitations:
// works only on RPi
// cannot function as non-CIC system controller with SN7516x because
// SN75161B cannot simultaneously make ATN input with IFC and REN as
// outputs.
// not implemented:
// parallel poll
// return2local
// device support (non master operation)
//

//
// Debug print levels:
// 0 = load/unload info and errors that make the driver fail;
// 1 = + warnings for unforeseen events that may break the current
// operation and lead to a timeout, but do not affect the
// driver integrity (mainly unexpected interrupts);
// 2 = + trace of function calls;
// 3 = + trace of protocol codes;
// 4 = + trace of interrupt operation.
//

    do { if (debug >= (level))					\
    dev_dbg(board.gpib_dev, frm, ## __VA_ARGS__); } \
    while (0)

    gpiod_get_value(NRFD),		\
    gpiod_get_value(NDAC),		\
    gpiod_get_value(SRQ)

    let mut sn7516x_used: static int = 1, sn7516x;
    module_param(sn7516x_used, int, 0660);

    static char *pin_map = PINMAP_0;
    module_param(pin_map, charp, 0660);
    MODULE_PARM_DESC(pin_map, " valid values: " PINMAP_0 " " PINMAP_1 " " PINMAP_2);
//
// Signal pairing and pin wiring between the
// Raspberry-Pi connector and the GPIB bus
//
// signal		  pin wiring
// GPIB  Pi-gpio	GPIB  ->  RPi
//
    enum lines_t {
    D01_pin_nr =  20,     /*   1  .  38  */
    D02_pin_nr =  26,     /*   2  .  37  */
    D03_pin_nr =  16,     /*   3  .  36  */
    D04_pin_nr =  19,     /*   4  .  35  */
    D05_pin_nr =  13,     /*  13  .  33  */
    D06_pin_nr =  12,     /*  14  .  32  */
    D07_pin_nr =   6,     /*  15  .  31  */
    D08_pin_nr =   5,     /*  16  .  29  */
    EOI_pin_nr =   9,     /*   5  .  21  */
    DAV_pin_nr =  10,     /*   6  .  19  */
    NRFD_pin_nr = 24,     /*   7  .  18  */
    NDAC_pin_nr = 23,     /*   8  .  16  */
    IFC_pin_nr =  22,     /*   9  .  15  */
    SRQ_pin_nr =  11,     /*  10  .  23  */
    _ATN_pin_nr = 25,     /*  11  .  22  */
    REN_pin_nr =  27,     /*  17  .  13  */
//
// GROUND PINS
// 12,18,19,20,21,22,23,24  => 14,20,25,30,34,39
//
// These lines are used to control the external
// SN75160/161 driver chips when used.
// When not used there is reduced fan out;
// currently tested with up to 4 devices.
//
// Pi GPIO	RPI   75161B 75160B   Description
    PE_pin_nr =    7,    /*	 26  .	  nc	 11   Pullup Enable     */
    DC_pin_nr =    8,    /*	 24  .	  12	 nc   Direction control */
    TE_pin_nr =   18,    /*	 12  .	   2	  1   Talk Enable       */
    ACT_LED_pin_nr = 4,  /*	  7  .	 LED  */
// YOGA adapter uses different pinout to ease layout
    YOGA_D03_pin_nr =  13,
    YOGA_D04_pin_nr =  12,
    YOGA_D05_pin_nr =  21,
    YOGA_D06_pin_nr =  19,
    };
//
// GPIO descriptors and pins - WARNING: STRICTLY KEEP ITEMS ORDER
//
pub const GPIB_PINS: c_int = 16;
pub const SN7516X_PINS: c_int = 4;

    if (ACT_LED)					\
    gpiod_direction_output(ACT_LED, 1);	\
    } while (0)

    if (ACT_LED)					\
    gpiod_direction_output(ACT_LED, 0);	\
    } while (0)
    static struct gpio_desc *all_descriptors[GPIB_PINS + SN7516X_PINS];

// YOGA adapter uses a global enable for the buffer chips, re-using the TE pin

    static int gpios_vector[] = {
    D01_pin_nr,
    D02_pin_nr,
    D03_pin_nr,
    D04_pin_nr,
    D05_pin_nr,
    D06_pin_nr,
    D07_pin_nr,
    D08_pin_nr,
    EOI_pin_nr,
    NRFD_pin_nr,
    IFC_pin_nr,
    _ATN_pin_nr,
    REN_pin_nr,
    DAV_pin_nr,
    NDAC_pin_nr,
    SRQ_pin_nr,
    PE_pin_nr,
    DC_pin_nr,
    TE_pin_nr,
    ACT_LED_pin_nr
    };
// Lookup table for general GPIOs
    static struct gpiod_lookup_table gpib_gpio_table_1 = {
// for bcm2835/6
    .dev_id = "",	 // device id of board device
    .table = {
    GPIO_LOOKUP_IDX("GPIO_GCLK",  U16_MAX, core::ptr::null_mut(),  4, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("GPIO5",	  U16_MAX, core::ptr::null_mut(),  5, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("GPIO6",	  U16_MAX, core::ptr::null_mut(),  6, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("SPI_CE1_N",  U16_MAX, core::ptr::null_mut(),  7, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("SPI_CE0_N",  U16_MAX, core::ptr::null_mut(),  8, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("SPI_MISO",	  U16_MAX, core::ptr::null_mut(),  9, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("SPI_MOSI",	  U16_MAX, core::ptr::null_mut(), 10, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("SPI_SCLK",	  U16_MAX, core::ptr::null_mut(), 11, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("GPIO12",	  U16_MAX, core::ptr::null_mut(), 12, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("GPIO13",	  U16_MAX, core::ptr::null_mut(), 13, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("GPIO16",	  U16_MAX, core::ptr::null_mut(), 16, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("GPIO17",	  U16_MAX, core::ptr::null_mut(), 17, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("GPIO18",	  U16_MAX, core::ptr::null_mut(), 18, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("GPIO19",	  U16_MAX, core::ptr::null_mut(), 19, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("GPIO20",	  U16_MAX, core::ptr::null_mut(), 20, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("GPIO21",	  U16_MAX, core::ptr::null_mut(), 21, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("GPIO22",	  U16_MAX, core::ptr::null_mut(), 22, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("GPIO23",	  U16_MAX, core::ptr::null_mut(), 23, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("GPIO24",	  U16_MAX, core::ptr::null_mut(), 24, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("GPIO25",	  U16_MAX, core::ptr::null_mut(), 25, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("GPIO26",	  U16_MAX, core::ptr::null_mut(), 26, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("GPIO27",	  U16_MAX, core::ptr::null_mut(), 27, GPIO_ACTIVE_HIGH),
    { }
    },
    };
    static struct gpiod_lookup_table gpib_gpio_table_0 = {
    .dev_id = "",	 // device id of board device
    .table = {
// for bcm27xx based pis (b b+ 2b 3b 3b+ 4 5)
    GPIO_LOOKUP_IDX("GPIO4",  U16_MAX, core::ptr::null_mut(),  4, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("GPIO5",  U16_MAX, core::ptr::null_mut(),  5, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("GPIO6",  U16_MAX, core::ptr::null_mut(),  6, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("GPIO7",  U16_MAX, core::ptr::null_mut(),  7, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("GPIO8",  U16_MAX, core::ptr::null_mut(),  8, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("GPIO9",  U16_MAX, core::ptr::null_mut(),  9, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("GPIO10", U16_MAX, core::ptr::null_mut(), 10, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("GPIO11", U16_MAX, core::ptr::null_mut(), 11, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("GPIO12", U16_MAX, core::ptr::null_mut(), 12, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("GPIO13", U16_MAX, core::ptr::null_mut(), 13, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("GPIO16", U16_MAX, core::ptr::null_mut(), 16, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("GPIO17", U16_MAX, core::ptr::null_mut(), 17, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("GPIO18", U16_MAX, core::ptr::null_mut(), 18, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("GPIO19", U16_MAX, core::ptr::null_mut(), 19, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("GPIO20", U16_MAX, core::ptr::null_mut(), 20, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("GPIO21", U16_MAX, core::ptr::null_mut(), 21, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("GPIO22", U16_MAX, core::ptr::null_mut(), 22, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("GPIO23", U16_MAX, core::ptr::null_mut(), 23, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("GPIO24", U16_MAX, core::ptr::null_mut(), 24, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("GPIO25", U16_MAX, core::ptr::null_mut(), 25, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("GPIO26", U16_MAX, core::ptr::null_mut(), 26, GPIO_ACTIVE_HIGH),
    GPIO_LOOKUP_IDX("GPIO27", U16_MAX, core::ptr::null_mut(), 27, GPIO_ACTIVE_HIGH),
    { }
    },
    };
    static struct gpiod_lookup_table *lookup_tables[] = {
    &gpib_gpio_table_0,
    &gpib_gpio_table_1,
    core::ptr::null_mut()
    };
// struct which defines private_data for gpio driver
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bb_priv {
    pub irq_NRFD: c_int,
    pub irq_NDAC: c_int,
    pub irq_DAV: c_int,
    pub irq_SRQ: c_int,
    pub /: *mut *mut int dav_mode; / dav interrupt mode 0/1 -> edge/levels,
    pub /: *mut *mut int nrfd_mode; / nrfd interrupt mode 0/1 -> edge/levels,
    pub /: *mut *mut int ndac_mode; / nrfd interrupt mode 0/1 -> edge/levels,
    pub /: *mut *mut int dav_tx; / keep trace of DAV status while sending,
    pub /: *mut *mut int dav_rx; / keep trace of DAV status while receiving,
    pub /: *mut *mut u8 eos; / eos character,
    pub /: *mut *mut short eos_flags; / eos mode,
    pub /: *mut *mut short eos_check; / eos check required in current operation ...,
    pub /: *mut *mut short eos_check_8; / ... with byte comparison,
    pub /: *mut *mut short eos_mask_7; / ... with 7 bit masked character,
    pub end: short int,
    pub request: c_int,
    pub count: c_int,
    pub direction: c_int,
    pub t1_delay: c_int,
    pub rbuf: *mut u8,
    pub wbuf: *mut u8,
    pub end_flag: c_int,
    pub /: *mut *mut int r_busy; / 0==idle 1==busy,
    pub w_busy: c_int,
    pub write_done: c_int,
    pub /: *mut *mut int cmd; / 1 = cmd write in progress,
    pub w_cnt: usize,
    pub length: usize,
    pub w_buf: *mut u8,
    pub /: *mut *mut spinlock_t rw_lock; / protect mods to rw_lock,
    pub phase: c_int,
    pub ndac_idle: c_int,
    pub ndac_seq: c_int,
    pub nrfd_idle: c_int,
    pub nrfd_seq: c_int,
    pub dav_seq: c_int,
    pub all_irqs: c_long,
    pub dav_idle: c_int,
    pub talker_state: enum talker_function_state,
    pub listener_state: enum listener_function_state,
}

    static inline long usec_diff(struct timespec64 *a, struct timespec64 *b);
    static void bb_buffer_print(struct gpib_board *board, unsigned char *buffer, size_t length,
    int cmd, int eoi);
    static void set_data_lines(u8 byte);
    static u8 get_data_lines(void);
    static void set_data_lines_input(void);
    static void set_data_lines_output(void);
    static inline int check_for_eos(struct bb_priv *priv, u8 byte);
    static void set_atn(struct gpib_board *board, int atn_asserted);
    static inline void SET_DIR_WRITE(struct bb_priv *priv);
    static inline void SET_DIR_READ(struct bb_priv *priv);
pub const DIR_READ: c_int = 0;
pub const DIR_WRITE: c_int = 1;
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("GPIB helper functions for bitbanging I/O");
// global variables
    static int debug;
    module_param(debug, int, 0644);
#[no_mangle]
unsafe extern "C" fn printable(x: c_char) -> c_char {
    static char printable(char x)
    {
    if (x < 32 || x > 126)
    return ' ';
    return x;
    }
//
// READ
//
    static int bb_read(struct gpib_board *board, u8 *buffer, size_t length,
    int *end, size_t *bytes_read)
    {
    struct bb_priv *priv = board.private_data;
    unsigned long flags;
    let mut retval: c_int = 0;
    ACT_LED_ON;
    SET_DIR_READ(priv);
    dbg_printk(2, "board: %p  lock %d  length: %zu\n",
    board, mutex_is_locked(&board.user_mutex), length);
    priv.end = 0;
    priv.count = 0;
    priv.rbuf = buffer;
    if (length == 0)
    goto read_end;
    priv.request = length;
    priv.eos_check = (priv.eos_flags & REOS) == 0; /* do eos check */
    priv.eos_check_8 = priv.eos_flags & BIN;	 /* over 8 bits */
    priv.eos_mask_7 = priv.eos & 0x7f;		 /* with this 7 bit eos */
    dbg_printk(3, ".........." LINFMT "\n", LINVAL);
    spin_lock_irqsave(&priv.rw_lock, flags);
    priv.dav_mode = 1;
    priv.dav_rx = 1;
    ENABLE_IRQ(priv.irq_DAV, IRQ_TYPE_LEVEL_LOW);
    priv.end_flag = 0;
    gpiod_set_value(NRFD, 1); // ready for data
    priv.r_busy = 1;
    priv.phase = 100;
    spin_unlock_irqrestore(&priv.rw_lock, flags);
// wait for the interrupt routines finish their work
    retval = wait_event_interruptible(board.wait,
    (priv.end_flag || board.status & TIMO));
    dbg_printk(3, "awake from wait queue: %d\n", retval);
    if (retval == 0 && board.status & TIMO) {
    retval = -ETIMEDOUT;
    dbg_printk(1, "timeout\n");
    } else if (retval) {
    retval = -ERESTARTSYS;
    }
    DISABLE_IRQ(priv.irq_DAV);
    spin_lock_irqsave(&priv.rw_lock, flags);
    gpiod_set_value(NRFD, 0); // DIR_READ line state
    priv.r_busy = 0;
    spin_unlock_irqrestore(&priv.rw_lock, flags);
    read_end:
    ACT_LED_OFF;
// bytes_read = priv->count;
// end = priv->end;
    priv.r_busy = 0;
    dbg_printk(2, "return: %d  eoi|eos: %d count: %d\n\n", retval, priv.end, priv.count);
    return retval;
    }
//
// READ interrupt routine (DAV line)
//
#[no_mangle]
unsafe extern "C" fn bb_DAV_interrupt(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t bb_DAV_interrupt(int irq, void *arg)
    {
    struct gpib_board *board = arg;
    struct bb_priv *priv = board.private_data;
    int val;
    unsigned long flags;
    spin_lock_irqsave(&priv.rw_lock, flags);
    priv.all_irqs++;
    if (priv.dav_mode) {
    ENABLE_IRQ(priv.irq_DAV, IRQ_TYPE_EDGE_BOTH);
    priv.dav_mode = 0;
    }
    if (priv.r_busy == 0) {
    dbg_printk(1, "interrupt while idle after %d at %d\n",
    priv.count, priv.phase);
    priv.dav_idle++;
    priv.phase = 200;
    goto dav_exit;	/* idle */
    }
    val = gpiod_get_value(DAV);
    if (val == priv.dav_rx) {
    dbg_printk(1, "out of order DAV interrupt %d/%d after %zu/%zu at %d cmd %d "
    LINFMT ".\n", val, priv.dav_rx, priv.w_cnt, priv.length,
    priv.phase, priv.cmd, LINVAL);
    priv.dav_seq++;
    }
    priv.dav_rx = val;
    dbg_printk(3, "> irq: %d  DAV: %d  st: %4lx dir: %d  busy: %d:%d\n",
    irq, val, board.status, priv.direction, priv.r_busy, priv.w_busy);
    if (val == 0) {
    gpiod_set_value(NRFD, 0); // not ready for data
    priv.rbuf[priv.count++] = get_data_lines();
    priv.end = !gpiod_get_value(EOI);
    gpiod_set_value(NDAC, 1); // data accepted
    priv.end |= check_for_eos(priv, priv.rbuf[priv.count - 1]);
    priv.end_flag = ((priv.count >= priv.request) || priv.end);
    priv.phase = 210;
    } else {
    gpiod_set_value(NDAC, 0);	// data not accepted
    if (priv.end_flag) {
    priv.r_busy = 0;
    wake_up_interruptible(&board.wait);
    priv.phase = 220;
    } else {
    gpiod_set_value(NRFD, 1);     // ready for data
    priv.phase = 230;
    }
    }
    dav_exit:
    spin_unlock_irqrestore(&priv.rw_lock, flags);
    dbg_printk(3, "< irq: %d  count %d\n", irq, priv.count);
    return IRQ_HANDLED;
    }
//
// WRITE
//
    static int bb_write(struct gpib_board *board, u8 *buffer, size_t length,
    int send_eoi, size_t *bytes_written)
    {
    unsigned long flags;
    let mut retval: c_int = 0;
    struct bb_priv *priv = board.private_data;
    ACT_LED_ON;
    priv.w_cnt = 0;
    priv.w_buf = buffer;
    dbg_printk(2, "board %p	lock %d	 length: %zu\n",
    board, mutex_is_locked(&board.user_mutex), length);
    if (debug > 1)
    bb_buffer_print(board, buffer, length, priv.cmd, send_eoi);
    priv.count = 0;
    priv.phase = 300;
    if (length == 0)
    goto write_end;
    priv.end = send_eoi;
    priv.length = length;
    SET_DIR_WRITE(priv);
    dbg_printk(2, "Enabling interrupts - NRFD: %d   NDAC: %d\n",
    gpiod_get_value(NRFD), gpiod_get_value(NDAC));
    if (gpiod_get_value(NRFD) && gpiod_get_value(NDAC)) { /* check for listener */
    retval = -ENOTCONN;
    goto write_end;
    }
    spin_lock_irqsave(&priv.rw_lock, flags);
    priv.w_busy = 1;	   /* make the interrupt routines active */
    priv.write_done = 0;
    priv.nrfd_mode = 1;
    priv.ndac_mode = 1;
    priv.dav_tx = 1;
    ENABLE_IRQ(priv.irq_NDAC, IRQ_TYPE_LEVEL_HIGH);
    ENABLE_IRQ(priv.irq_NRFD, IRQ_TYPE_LEVEL_HIGH);
    spin_unlock_irqrestore(&priv.rw_lock, flags);
// wait for the interrupt routines finish their work
    retval = wait_event_interruptible(board.wait,
    priv.write_done || (board.status & TIMO));
    dbg_printk(3, "awake from wait queue: %d\n", retval);
    if (retval == 0) {
    if (board.status & TIMO) {
    retval = -ETIMEDOUT;
    dbg_printk(1, "timeout after %zu/%zu at %d " LINFMT " eoi: %d\n",
    priv.w_cnt, length, priv.phase, LINVAL, send_eoi);
    } else {
    retval = priv.w_cnt;
    }
    } else {
    retval = -ERESTARTSYS;
    }
    DISABLE_IRQ(priv.irq_NRFD);
    DISABLE_IRQ(priv.irq_NDAC);
    spin_lock_irqsave(&priv.rw_lock, flags);
    priv.w_busy = 0;
    gpiod_set_value(DAV, 1); // DIR_WRITE line state
    gpiod_set_value(EOI, 1); // De-assert EOI (in case)
    spin_unlock_irqrestore(&priv.rw_lock, flags);
    write_end:
// bytes_written = priv->w_cnt;
    ACT_LED_OFF;
    dbg_printk(2, "sent %zu bytes\r\n\r\n", *bytes_written);
    priv.phase = 310;
    return retval;
    }
//
// WRITE interrupt routine (NRFD line)
//
#[no_mangle]
unsafe extern "C" fn bb_NRFD_interrupt(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t bb_NRFD_interrupt(int irq, void *arg)
    {
    struct gpib_board *board = arg;
    struct bb_priv *priv = board.private_data;
    unsigned long flags;
    int nrfd;
    spin_lock_irqsave(&priv.rw_lock, flags);
    nrfd = gpiod_get_value(NRFD);
    priv.all_irqs++;
    dbg_printk(3, "> irq: %d  NRFD: %d   NDAC: %d	st: %4lx dir: %d  busy: %d:%d\n",
    irq, nrfd, gpiod_get_value(NDAC), board.status, priv.direction,
    priv.w_busy, priv.r_busy);
    if (priv.nrfd_mode) {
    ENABLE_IRQ(priv.irq_NRFD, IRQ_TYPE_EDGE_RISING);
    priv.nrfd_mode = 0;
    }
    if (priv.w_busy == 0) {
    dbg_printk(1, "interrupt while idle after %zu/%zu at %d\n",
    priv.w_cnt, priv.length, priv.phase);
    priv.nrfd_idle++;
    goto nrfd_exit;	 /* idle */
    }
    if (nrfd == 0) {
    dbg_printk(1, "out of order interrupt after %zu/%zu at %d cmd %d " LINFMT ".\n",
    priv.w_cnt, priv.length, priv.phase, priv.cmd, LINVAL);
    priv.phase = 400;
    priv.nrfd_seq++;
    goto nrfd_exit;
    }
    if (!priv.dav_tx) {
    dbg_printk(1, "DAV low after %zu/%zu cmd %d " LINFMT ". No action.\n",
    priv.w_cnt, priv.length, priv.cmd, LINVAL);
    priv.dav_seq++;
    goto nrfd_exit;
    }
    if (priv.w_cnt >= priv.length) { // test for missed NDAC end of transfer
    dev_err(board.gpib_dev, "Unexpected NRFD exit\n");
    priv.write_done = 1;
    priv.w_busy = 0;
    wake_up_interruptible(&board.wait);
    goto nrfd_exit;
    }
    dbg_printk(3, "sending %zu\n", priv.w_cnt);
    set_data_lines(priv.w_buf[priv.w_cnt++]); // put the data on the lines
    if (priv.w_cnt == priv.length && priv.end) {
    dbg_printk(3, "Asserting EOI\n");
    gpiod_set_value(EOI, 0); // Assert EOI
    }
    gpiod_set_value(DAV, 0); // Data available
    priv.dav_tx = 0;
    priv.phase = 410;
    nrfd_exit:
    spin_unlock_irqrestore(&priv.rw_lock, flags);
    return IRQ_HANDLED;
    }
//
// WRITE interrupt routine (NDAC line)
//
#[no_mangle]
unsafe extern "C" fn bb_NDAC_interrupt(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t bb_NDAC_interrupt(int irq, void *arg)
    {
    struct gpib_board *board = arg;
    struct bb_priv *priv = board.private_data;
    unsigned long flags;
    int ndac;
    spin_lock_irqsave(&priv.rw_lock, flags);
    ndac = gpiod_get_value(NDAC);
    priv.all_irqs++;
    dbg_printk(3, "> irq: %d  NRFD: %d   NDAC: %d	st: %4lx dir: %d  busy: %d:%d\n",
    irq, gpiod_get_value(NRFD), ndac, board.status, priv.direction,
    priv.w_busy, priv.r_busy);
    if (priv.ndac_mode) {
    ENABLE_IRQ(priv.irq_NDAC, IRQ_TYPE_EDGE_RISING);
    priv.ndac_mode = 0;
    }
    if (priv.w_busy == 0) {
    dbg_printk(1, "interrupt while idle.\n");
    priv.ndac_idle++;
    goto ndac_exit;
    }
    if (ndac == 0) {
    dbg_printk(1, "out of order interrupt at %zu:%d.\n", priv.w_cnt, priv.phase);
    priv.phase = 500;
    priv.ndac_seq++;
    goto ndac_exit;
    }
    if (priv.dav_tx) {
    dbg_printk(1, "DAV high after %zu/%zu cmd %d " LINFMT ". No action.\n",
    priv.w_cnt, priv.length, priv.cmd, LINVAL);
    priv.dav_seq++;
    goto ndac_exit;
    }
    dbg_printk(3, "accepted %zu\n", priv.w_cnt - 1);
    gpiod_set_value(DAV, 1); // Data not available
    priv.dav_tx = 1;
    priv.phase = 510;
    if (priv.w_cnt >= priv.length) { // test for end of transfer
    priv.write_done = 1;
    priv.w_busy = 0;
    wake_up_interruptible(&board.wait);
    }
    ndac_exit:
    spin_unlock_irqrestore(&priv.rw_lock, flags);
    return IRQ_HANDLED;
    }
//
// interrupt routine for SRQ line
//
#[no_mangle]
unsafe extern "C" fn bb_SRQ_interrupt(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t bb_SRQ_interrupt(int irq, void *arg)
    {
    struct gpib_board  *board = arg;
    let mut val: c_int = gpiod_get_value(SRQ);
    dbg_printk(3, "> %d   st: %4lx\n", val, board.status);
    if (!val)
    set_bit(SRQI_NUM, &board.status);  /* set_bit() is atomic */
    wake_up_interruptible(&board.wait);
    return IRQ_HANDLED;
    }
    static int bb_command(struct gpib_board *board, u8 *buffer,
    size_t length, size_t *bytes_written)
    {
    int ret;
    struct bb_priv *priv = board.private_data;
    int i;
    dbg_printk(2, "%p  %p\n", buffer, board.buffer);
// the _ATN line has already been asserted by bb_take_control()
    priv.cmd = 1;
    ret = bb_write(board, buffer, length, 0, bytes_written); // no eoi
    for (i = 0; i < length; i++) {
    if (buffer[i] == UNT) {
    priv.talker_state = talker_idle;
    } else {
    if (buffer[i] == UNL) {
    priv.listener_state = listener_idle;
    } else {
    if (buffer[i] == (MTA(board.pad))) {
    priv.talker_state = talker_addressed;
    priv.listener_state = listener_idle;
    } else if (buffer[i] == (MLA(board.pad))) {
    priv.listener_state = listener_addressed;
    priv.talker_state = talker_idle;
    }
    }
    }
    }
// the _ATN line will be released by bb_go_to_stby
    priv.cmd = 0;
    return ret;
    }
//
// Buffer print with decode for debug/trace
//
    static char *cmd_string[32] = {
    "",    // 0x00
    "GTL", // 0x01
    "",    // 0x02
    "",    // 0x03
    "SDC", // 0x04
    "PPC", // 0x05
    "",    // 0x06
    "",    // 0x07
    "GET", // 0x08
    "TCT", // 0x09
    "",    // 0x0a
    "",    // 0x0b
    "",    // 0x0c
    "",    // 0x0d
    "",    // 0x0e
    "",    // 0x0f
    "",    // 0x10
    "LLO", // 0x11
    "",    // 0x12
    "",    // 0x13
    "DCL", // 0x14
    "PPU", // 0x15
    "",    // 0x16
    "",    // 0x17
    "SPE", // 0x18
    "SPD", // 0x19
    "",    // 0x1a
    "",    // 0x1b
    "",    // 0x1c
    "",    // 0x1d
    "",    // 0x1e
    "CFE"  // 0x1f
    };
    static void bb_buffer_print(struct gpib_board *board, unsigned char *buffer, size_t length,
    int cmd, int eoi)
    {
    int i;
    if (cmd) {
    dbg_printk(2, "<cmd len %zu>\n", length);
    for (i = 0; i < length; i++) {
    if (buffer[i] < 0x20) {
    dbg_printk(3, "0x%x=%s\n", buffer[i], cmd_string[buffer[i]]);
    } else if (buffer[i] == 0x3f) {
    dbg_printk(3, "0x%x=%s\n", buffer[i], "UNL");
    } else if (buffer[i] == 0x5f) {
    dbg_printk(3, "0x%x=%s\n", buffer[i], "UNT");
    } else	if (buffer[i] < 0x60) {
    dbg_printk(3, "0x%x=%s%d\n", buffer[i],
    (buffer[i] & 0x40) ? "TLK" : "LSN", buffer[i] & 0x1F);
    } else {
    dbg_printk(3, "0x%x\n", buffer[i]);
    }
    }
    } else {
    dbg_printk(2, "<data len %zu %s>\n", length, (eoi) ? "w.EOI" : " ");
    for (i = 0; i < length; i++)
    dbg_printk(2, "%3d  0x%x.%c\n", i, buffer[i], printable(buffer[i]));
    }
    }
//
// STATUS Management
//
#[no_mangle]
unsafe extern "C" fn set_atn(board: *mut gpib_board, atn_asserted: c_int) {
    static void set_atn(struct gpib_board *board, int atn_asserted)
    {
    struct bb_priv *priv = board.private_data;
    if (priv.listener_state != listener_idle &&
    priv.talker_state != talker_idle) {
    dev_err(board.gpib_dev, "listener/talker state machine conflict\n");
    }
    if (atn_asserted) {
    if (priv.listener_state == listener_active)
    priv.listener_state = listener_addressed;
    if (priv.talker_state == talker_active)
    priv.talker_state = talker_addressed;
    SET_DIR_WRITE(priv);  // need to be able to read bus NRFD/NDAC
    } else {
    if (priv.listener_state == listener_addressed) {
    priv.listener_state = listener_active;
    SET_DIR_READ(priv); // make sure holdoff is active when we unassert ATN
    }
    if (priv.talker_state == talker_addressed)
    priv.talker_state = talker_active;
    }
    gpiod_direction_output(_ATN, !atn_asserted);
    }
#[no_mangle]
unsafe extern "C" fn bb_take_control(board: *mut gpib_board, synchronous: c_int) -> c_int {
    static int bb_take_control(struct gpib_board *board, int synchronous)
    {
    dbg_printk(2, "%d\n", synchronous);
    set_atn(board, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bb_go_to_standby(board: *mut gpib_board) -> c_int {
    static int bb_go_to_standby(struct gpib_board *board)
    {
    dbg_printk(2, "\n");
    set_atn(board, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bb_request_system_control(board: *mut gpib_board, request_control: c_int) -> c_int {
    static int bb_request_system_control(struct gpib_board *board, int request_control)
    {
    struct bb_priv *priv = board.private_data;
    dbg_printk(2, "%d\n", request_control);
    if (!request_control)
    return -EINVAL;
    gpiod_direction_output(REN, 1); /* user space must enable REN if needed */
    gpiod_direction_output(IFC, 1); /* user space must toggle IFC if needed */
    if (sn7516x)
    gpiod_direction_output(DC, 0); /* enable ATN as output on SN75161/2 */
    gpiod_direction_input(SRQ);
    ENABLE_IRQ(priv.irq_SRQ, IRQ_TYPE_EDGE_FALLING);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bb_interface_clear(board: *mut gpib_board, assert: c_int) {
    static void bb_interface_clear(struct gpib_board *board, int assert)
    {
    struct bb_priv *priv = board.private_data;
    dbg_printk(2, "%d\n", assert);
    if (assert) {
    gpiod_direction_output(IFC, 0);
    priv.talker_state = talker_idle;
    priv.listener_state = listener_idle;
    set_bit(CIC_NUM, &board.status);
    } else {
    gpiod_direction_output(IFC, 1);
    }
    }
#[no_mangle]
unsafe extern "C" fn bb_remote_enable(board: *mut gpib_board, enable: c_int) {
    static void bb_remote_enable(struct gpib_board *board, int enable)
    {
    dbg_printk(2, "%d\n", enable);
    if (enable) {
    set_bit(REM_NUM, &board.status);
    gpiod_direction_output(REN, 0);
    } else {
    clear_bit(REM_NUM, &board.status);
    gpiod_direction_output(REN, 1);
    }
    }
#[no_mangle]
unsafe extern "C" fn bb_enable_eos(board: *mut gpib_board, eos_byte: u8, compare_8_bits: c_int) -> c_int {
    static int bb_enable_eos(struct gpib_board *board, u8 eos_byte, int compare_8_bits)
    {
    struct bb_priv *priv = board.private_data;
    dbg_printk(2, "%s\n", "EOS_en");
    priv.eos = eos_byte;
    priv.eos_flags = REOS;
    if (compare_8_bits)
    priv.eos_flags |= BIN;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bb_disable_eos(board: *mut gpib_board) {
    static void bb_disable_eos(struct gpib_board *board)
    {
    struct bb_priv *priv = board.private_data;
    dbg_printk(2, "\n");
    priv.eos_flags &= ~REOS;
    }
#[no_mangle]
unsafe extern "C" fn bb_update_status(board: *mut gpib_board, clear_mask: c_uint) -> c_uint {
    static unsigned int bb_update_status(struct gpib_board *board, unsigned int clear_mask)
    {
    struct bb_priv *priv = board.private_data;
    board.status &= ~clear_mask;
    if (gpiod_get_value(SRQ))	       /* SRQ asserted low */
    clear_bit(SRQI_NUM, &board.status);
    else
    set_bit(SRQI_NUM, &board.status);
    if (gpiod_get_value(_ATN))			/* ATN asserted low */
    clear_bit(ATN_NUM, &board.status);
    else
    set_bit(ATN_NUM, &board.status);
    if (priv.talker_state == talker_active ||
    priv.talker_state == talker_addressed)
    set_bit(TACS_NUM, &board.status);
    else
    clear_bit(TACS_NUM, &board.status);
    if (priv.listener_state == listener_active ||
    priv.listener_state == listener_addressed)
    set_bit(LACS_NUM, &board.status);
    else
    clear_bit(LACS_NUM, &board.status);
    dbg_printk(2, "0x%lx mask 0x%x\n", board.status, clear_mask);
    return board.status;
    }
#[no_mangle]
unsafe extern "C" fn bb_primary_address(board: *mut gpib_board, address: c_uint) -> c_int {
    static int bb_primary_address(struct gpib_board *board, unsigned int address)
    {
    dbg_printk(2, "%d\n", address);
    board.pad = address;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bb_secondary_address(board: *mut gpib_board, address: c_uint, enable: c_int) -> c_int {
    static int bb_secondary_address(struct gpib_board *board, unsigned int address, int enable)
    {
    dbg_printk(2, "%d %d\n", address, enable);
    if (enable)
    board.sad = address;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bb_parallel_poll(board: *mut gpib_board, result: *mut u8) -> c_int {
    static int bb_parallel_poll(struct gpib_board *board, u8 *result)
    {
    return -ENOENT;
    }
#[no_mangle]
unsafe extern "C" fn bb_parallel_poll_configure(board: *mut gpib_board, config: u8) {
    static void bb_parallel_poll_configure(struct gpib_board *board, u8 config)
    {
    }
#[no_mangle]
unsafe extern "C" fn bb_parallel_poll_response(board: *mut gpib_board, ist: c_int) {
    static void bb_parallel_poll_response(struct gpib_board *board, int ist)
    {
    }
#[no_mangle]
unsafe extern "C" fn bb_serial_poll_response(board: *mut gpib_board, status: u8) {
    static void bb_serial_poll_response(struct gpib_board *board, u8 status)
    {
    }
#[no_mangle]
unsafe extern "C" fn bb_serial_poll_status(board: *mut gpib_board) -> u8 {
    static u8 bb_serial_poll_status(struct gpib_board *board)
    {
    return 0; // -ENOENT;
    }
#[no_mangle]
unsafe extern "C" fn bb_t1_delay(board: *mut gpib_board, nano_sec: c_uint) -> c_int {
    static int bb_t1_delay(struct gpib_board *board,  unsigned int nano_sec)
    {
    struct bb_priv *priv = board.private_data;
    if (nano_sec <= 350)
    priv.t1_delay = 350;
#[no_mangle]
pub unsafe extern "C" fn if(1100: nano_sec <=) -> else {
    else if (nano_sec <= 1100)
    priv.t1_delay = 1100;
    else
    priv.t1_delay = 2000;
    dbg_printk(2, "t1 delay set to %d nanosec\n", priv.t1_delay);
    return priv.t1_delay;
    }
#[no_mangle]
unsafe extern "C" fn bb_return_to_local(board: *mut gpib_board) {
    static void bb_return_to_local(struct gpib_board *board)
    {
    }
#[no_mangle]
unsafe extern "C" fn bb_line_status(board: *const gpib_board) -> c_int {
    static int bb_line_status(const struct gpib_board *board)
    {
    let mut line_status: c_int = VALID_ALL;
    if (gpiod_get_value(REN) == 0)
    line_status |= BUS_REN;
    if (gpiod_get_value(IFC) == 0)
    line_status |= BUS_IFC;
    if (gpiod_get_value(NDAC) == 0)
    line_status |= BUS_NDAC;
    if (gpiod_get_value(NRFD) == 0)
    line_status |= BUS_NRFD;
    if (gpiod_get_value(DAV) == 0)
    line_status |= BUS_DAV;
    if (gpiod_get_value(EOI) == 0)
    line_status |= BUS_EOI;
    if (gpiod_get_value(_ATN) == 0)
    line_status |= BUS_ATN;
    if (gpiod_get_value(SRQ) == 0)
    line_status |= BUS_SRQ;
    dbg_printk(2, "status lines: %4x\n", line_status);
    return line_status;
    }
//
// Module Management
//
#[no_mangle]
unsafe extern "C" fn allocate_private(board: *mut gpib_board) -> c_int {
    static int allocate_private(struct gpib_board *board)
    {
    board.private_data = kzalloc_obj(struct bb_priv);
    if (!board.private_data)
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn free_private(board: *mut gpib_board) {
    static void free_private(struct gpib_board *board)
    {
    kfree(board.private_data);
    board.private_data = core::ptr::null_mut();
    }
    static int bb_get_irq(struct gpib_board *board, char *name,
    struct gpio_desc *gpio, int *irq,
    irq_handler_t handler, irq_handler_t thread_fn, unsigned long flags)
    {
    if (!gpio)
    return -1;
    gpiod_direction_input(gpio);
// irq = gpiod_to_irq(gpio);
    dbg_printk(2, "IRQ %s: %d\n", name, *irq);
    if (*irq < 0) {
    dev_err(board.gpib_dev, "can't get IRQ for %s\n", name);
    return -1;
    }
    if (request_threaded_irq(*irq, handler, thread_fn, flags, name, board)) {
    dev_err(board.gpib_dev, "can't request IRQ for %s %d\n", name, *irq);
// irq = 0;
    return -1;
    }
    DISABLE_IRQ(*irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bb_free_irq(board: *mut gpib_board, irq: *mut c_int, name: *mut c_char) {
    static void bb_free_irq(struct gpib_board *board, int *irq, char *name)
    {
    if (*irq) {
    free_irq(*irq, board);
    dbg_printk(2, "IRQ %d(%s) freed\n", *irq, name);
// irq = 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn release_gpios() {
    static void release_gpios(void)
    {
    int j;
    for (j = 0 ; j < NUM_PINS ; j++) {
    if (all_descriptors[j]) {
    gpiod_put(all_descriptors[j]);
    all_descriptors[j] = core::ptr::null_mut();
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn allocate_gpios(board: *mut gpib_board) -> c_int {
    static int allocate_gpios(struct gpib_board *board)
    {
    int j;
    let mut table_index: c_int = 0;
    char name[256];
    struct gpio_desc *desc;
    struct gpiod_lookup_table *lookup_table;
    if (!board.gpib_dev) {
    pr_err("core::ptr::null_mut() gpib dev for board\n");
    return -ENOENT;
    }
    lookup_table = lookup_tables[table_index];
    lookup_table.dev_id = dev_name(board.gpib_dev);
    gpiod_add_lookup_table(lookup_table);
    dbg_printk(1, "Allocating gpios using table index %d\n", table_index);
    for (j = 0 ; j < NUM_PINS ; j++) {
    if (gpios_vector[j] < 0)
    continue;
// name not really used in gpiod_get_index()
    sprintf(name, "GPIO%d", gpios_vector[j]);
    try_again:
    dbg_printk(1, "Allocating gpio %s pin no %d\n", name, gpios_vector[j]);
    desc = gpiod_get_index(board.gpib_dev, name, gpios_vector[j], GPIOD_IN);
    if (IS_ERR(desc)) {
    gpiod_remove_lookup_table(lookup_table);
    table_index++;
    lookup_table = lookup_tables[table_index];
    if (!lookup_table) {
    dev_err(board.gpib_dev, "Unable to obtain gpio descriptor for pin %d error %ld\n",
    gpios_vector[j], PTR_ERR(desc));
    goto alloc_gpios_fail;
    }
    dbg_printk(1, "Allocation failed, now using table_index %d\n", table_index);
    lookup_table.dev_id = dev_name(board.gpib_dev);
    gpiod_add_lookup_table(lookup_table);
    goto try_again;
    }
    all_descriptors[j] = desc;
    }
    gpiod_remove_lookup_table(lookup_table);
    return 0;
    alloc_gpios_fail:
    release_gpios();
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn bb_detach(board: *mut gpib_board) {
    static void bb_detach(struct gpib_board *board)
    {
    struct bb_priv *priv = board.private_data;
    dbg_printk(2, "Enter with data %p\n", board.private_data);
    if (!board.private_data)
    return;
    bb_free_irq(board, &priv.irq_DAV, NAME "_DAV");
    bb_free_irq(board, &priv.irq_NRFD, NAME "_NRFD");
    bb_free_irq(board, &priv.irq_NDAC, NAME "_NDAC");
    bb_free_irq(board, &priv.irq_SRQ, NAME "_SRQ");
    if (strcmp(PINMAP_2, pin_map) == 0) { /* YOGA */
    gpiod_set_value(YOGA_ENABLE, 0);
    }
    release_gpios();
    dbg_printk(2, "detached board: %d\n", board.minor);
    dbg_printk(0, "NRFD: idle %d, seq %d,  NDAC: idle %d, seq %d  DAV: idle %d  seq: %d  all: %ld",
    priv.nrfd_idle, priv.nrfd_seq,
    priv.ndac_idle, priv.ndac_seq,
    priv.dav_idle, priv.dav_seq, priv.all_irqs);
    free_private(board);
    }
#[no_mangle]
unsafe extern "C" fn bb_attach(board: *mut gpib_board, config: *const gpib_board_config) -> c_int {
    static int bb_attach(struct gpib_board *board, const struct gpib_board_config *config)
    {
    struct bb_priv *priv;
    int retval;
    dbg_printk(2, "%s\n", "Enter ...");
    board.status = 0;
    retval = allocate_private(board);
    if (retval)
    return retval;
    priv = board.private_data;
    priv.direction = -1;
    priv.t1_delay = 2000;
    priv.listener_state = listener_idle;
    priv.talker_state = talker_idle;
    sn7516x = sn7516x_used;
    if (strcmp(PINMAP_0, pin_map) == 0) {
    if (!sn7516x) {
    gpios_vector[&(PE) - &all_descriptors[0]] = -1;
    gpios_vector[&(DC) - &all_descriptors[0]] = -1;
    gpios_vector[&(TE) - &all_descriptors[0]] = -1;
    }
    } else if (strcmp(PINMAP_1, pin_map) == 0) {
    if (!sn7516x) {
    gpios_vector[&(PE) - &all_descriptors[0]] = -1;
    gpios_vector[&(DC) - &all_descriptors[0]] = -1;
    gpios_vector[&(TE) - &all_descriptors[0]] = -1;
    }
    gpios_vector[&(REN) - &all_descriptors[0]] = 0; /* 27 . 0 REN on GPIB pin 0 */
    } else if (strcmp(PINMAP_2, pin_map) == 0) { /* YOGA */
    sn7516x = 0;
    gpios_vector[&(D03) - &all_descriptors[0]] = YOGA_D03_pin_nr;
    gpios_vector[&(D04) - &all_descriptors[0]] = YOGA_D04_pin_nr;
    gpios_vector[&(D05) - &all_descriptors[0]] = YOGA_D05_pin_nr;
    gpios_vector[&(D06) - &all_descriptors[0]] = YOGA_D06_pin_nr;
    gpios_vector[&(PE)  - &all_descriptors[0]] = -1;
    gpios_vector[&(DC)  - &all_descriptors[0]] = -1;
    } else {
    dev_err(board.gpib_dev, "Unrecognized pin map %s\n", pin_map);
    goto bb_attach_fail;
    }
    dbg_printk(0, "Using pin map \"%s\" %s\n", pin_map, (sn7516x) ?
    " with SN7516x driver support" : "");
    if (allocate_gpios(board))
    goto bb_attach_fail;
//
// Configure SN7516X control lines.
// drive ATN, IFC and REN as outputs only when master
// i.e. system controller. In this mode can only be the CIC
// When not master then enable device mode ATN, IFC & REN as inputs
//
    if (sn7516x) {
    gpiod_direction_output(DC, 0);
    gpiod_direction_output(TE, 1);
    gpiod_direction_output(PE, 1);
    }
// Set main control lines to a known state
    gpiod_direction_output(IFC, 1);
    gpiod_direction_output(REN, 1);
    gpiod_direction_output(_ATN, 1);
    if (strcmp(PINMAP_2, pin_map) == 0) { /* YOGA: enable level shifters */
    gpiod_direction_output(YOGA_ENABLE, 1);
    }
    spin_lock_init(&priv.rw_lock);
// request DAV interrupt for read
    if (bb_get_irq(board, NAME "_DAV", DAV, &priv.irq_DAV, bb_DAV_interrupt, core::ptr::null_mut(),
    IRQF_TRIGGER_NONE))
    goto bb_attach_fail_r;
// request NRFD interrupt for write
    if (bb_get_irq(board, NAME "_NRFD", NRFD, &priv.irq_NRFD, bb_NRFD_interrupt, core::ptr::null_mut(),
    IRQF_TRIGGER_NONE))
    goto bb_attach_fail_r;
// request NDAC interrupt for write
    if (bb_get_irq(board, NAME "_NDAC", NDAC, &priv.irq_NDAC, bb_NDAC_interrupt, core::ptr::null_mut(),
    IRQF_TRIGGER_NONE))
    goto bb_attach_fail_r;
// request SRQ interrupt for Service Request
    if (bb_get_irq(board, NAME "_SRQ", SRQ, &priv.irq_SRQ, bb_SRQ_interrupt, core::ptr::null_mut(),
    IRQF_TRIGGER_NONE))
    goto bb_attach_fail_r;
    dbg_printk(0, "attached board %d\n", board.minor);
    goto bb_attach_out;
    bb_attach_fail_r:
    release_gpios();
    bb_attach_fail:
    retval = -1;
    bb_attach_out:
    return retval;
    }
    static struct gpib_interface bb_interface = {
    .name =	NAME,
    .attach = bb_attach,
    .detach = bb_detach,
    .read = bb_read,
    .write = bb_write,
    .command = bb_command,
    .take_control = bb_take_control,
    .go_to_standby = bb_go_to_standby,
    .request_system_control = bb_request_system_control,
    .interface_clear = bb_interface_clear,
    .remote_enable = bb_remote_enable,
    .enable_eos = bb_enable_eos,
    .disable_eos = bb_disable_eos,
    .parallel_poll = bb_parallel_poll,
    .parallel_poll_configure = bb_parallel_poll_configure,
    .parallel_poll_response = bb_parallel_poll_response,
    .line_status = bb_line_status,
    .update_status = bb_update_status,
    .primary_address = bb_primary_address,
    .secondary_address = bb_secondary_address,
    .serial_poll_response = bb_serial_poll_response,
    .serial_poll_status = bb_serial_poll_status,
    .t1_delay = bb_t1_delay,
    .return_to_local = bb_return_to_local,
    };
#[no_mangle]
unsafe extern "C" fn bb_init_module() -> int __init {
    static int __init bb_init_module(void)
    {
    let mut result: c_int = gpib_register_driver(&bb_interface, THIS_MODULE);
    if (result) {
    pr_err("gpib_register_driver failed: error = %d\n", result);
    return result;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bb_exit_module() -> void __exit {
    static void __exit bb_exit_module(void)
    {
    gpib_unregister_driver(&bb_interface);
    }
    module_init(bb_init_module);
    module_exit(bb_exit_module);
//
// UTILITY Functions
//
#[no_mangle]
pub unsafe extern "C" fn usec_diff(a: *mut timespec64, b: *mut timespec64) -> c_long {
    inline long usec_diff(struct timespec64 *a, struct timespec64 *b)
    {
    return ((a.tv_sec - b.tv_sec) * 1000000 +
    (a.tv_nsec - b.tv_nsec) / 1000);
    }
#[no_mangle]
pub unsafe extern "C" fn check_for_eos(priv: *mut bb_priv, byte: u8) -> c_int {
    static inline int check_for_eos(struct bb_priv *priv, u8 byte)
    {
    if (priv.eos_check)
    return 0;
    if (priv.eos_check_8) {
    if (priv.eos == byte)
    return 1;
    } else {
    if (priv.eos_mask_7 == (byte & 0x7f))
    return 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn set_data_lines_output() {
    static void set_data_lines_output(void)
    {
    gpiod_direction_output(D01, 1);
    gpiod_direction_output(D02, 1);
    gpiod_direction_output(D03, 1);
    gpiod_direction_output(D04, 1);
    gpiod_direction_output(D05, 1);
    gpiod_direction_output(D06, 1);
    gpiod_direction_output(D07, 1);
    gpiod_direction_output(D08, 1);
    }
#[no_mangle]
unsafe extern "C" fn set_data_lines(byte: u8) {
    static void set_data_lines(u8 byte)
    {
    gpiod_set_value(D01, !(byte & 0x01));
    gpiod_set_value(D02, !(byte & 0x02));
    gpiod_set_value(D03, !(byte & 0x04));
    gpiod_set_value(D04, !(byte & 0x08));
    gpiod_set_value(D05, !(byte & 0x10));
    gpiod_set_value(D06, !(byte & 0x20));
    gpiod_set_value(D07, !(byte & 0x40));
    gpiod_set_value(D08, !(byte & 0x80));
    }
#[no_mangle]
unsafe extern "C" fn get_data_lines() -> u8 {
    static u8 get_data_lines(void)
    {
    u8 ret;
    ret = gpiod_get_value(D01);
    ret |= gpiod_get_value(D02) << 1;
    ret |= gpiod_get_value(D03) << 2;
    ret |= gpiod_get_value(D04) << 3;
    ret |= gpiod_get_value(D05) << 4;
    ret |= gpiod_get_value(D06) << 5;
    ret |= gpiod_get_value(D07) << 6;
    ret |= gpiod_get_value(D08) << 7;
    return ~ret;
    }
#[no_mangle]
unsafe extern "C" fn set_data_lines_input() {
    static void set_data_lines_input(void)
    {
    gpiod_direction_input(D01);
    gpiod_direction_input(D02);
    gpiod_direction_input(D03);
    gpiod_direction_input(D04);
    gpiod_direction_input(D05);
    gpiod_direction_input(D06);
    gpiod_direction_input(D07);
    gpiod_direction_input(D08);
    }
#[no_mangle]
pub unsafe extern "C" fn SET_DIR_WRITE(priv: *mut bb_priv) {
    static inline void SET_DIR_WRITE(struct bb_priv *priv)
    {
    if (priv.direction == DIR_WRITE)
    return;
    gpiod_direction_input(NRFD);
    gpiod_direction_input(NDAC);
    set_data_lines_output();
    gpiod_direction_output(DAV, 1);
    gpiod_direction_output(EOI, 1);
    if (sn7516x) {
    gpiod_set_value(PE, 1);	 /* set data lines to transmit on sn75160b */
    gpiod_set_value(TE, 1);	 /* set NDAC and NRFD to receive and DAV to transmit */
    }
    priv.direction = DIR_WRITE;
    }
#[no_mangle]
pub unsafe extern "C" fn SET_DIR_READ(priv: *mut bb_priv) {
    static inline void SET_DIR_READ(struct bb_priv *priv)
    {
    if (priv.direction == DIR_READ)
    return;
    gpiod_direction_input(DAV);
    gpiod_direction_input(EOI);
    set_data_lines_input();
    if (sn7516x) {
    gpiod_set_value(PE, 0);	 /* set data lines to receive on sn75160b */
    gpiod_set_value(TE, 0);	 /* set NDAC and NRFD to transmit and DAV to receive */
    }
    gpiod_direction_output(NRFD, 0); /* hold off the talker */
    gpiod_direction_output(NDAC, 0); /* data not accepted */
    priv.direction = DIR_READ;
    }
