//! Automatically rewritten from C to Rust
//! Source: sound/drivers/portman2x4.c
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
// Driver for Midiman Portman2x4 parallel port midi interface
//
// Copyright (c) by Levent Guendogdu <levon@feature-it.com>
//
// ChangeLog
// Jan 24 2007 Matthias Koenig <mkoenig@suse.de>
// - cleanup and rewrite
// Sep 30 2004 Tobias Gehrig <tobias@gehrig.tk>
// - source code cleanup
// Sep 03 2004 Tobias Gehrig <tobias@gehrig.tk>
// - fixed compilation problem with alsa 1.0.6a (removed MODULE_CLASSES,
// MODULE_PARM_SYNTAX and changed MODULE_DEVICES to
// MODULE_SUPPORTED_DEVICE)
// Mar 24 2004 Tobias Gehrig <tobias@gehrig.tk>
// - added 2.6 kernel support
// Mar 18 2004 Tobias Gehrig <tobias@gehrig.tk>
// - added parport_unregister_driver to the startup routine if the driver fails to detect a portman
// - added support for all 4 output ports in portman_putmidi
// Mar 17 2004 Tobias Gehrig <tobias@gehrig.tk>
// - added checks for opened input device in interrupt handler
// Feb 20 2004 Tobias Gehrig <tobias@gehrig.tk>
// - ported from alsa 0.5 to 1.0
//

    static int index[SNDRV_CARDS]  = SNDRV_DEFAULT_IDX;
    static char *id[SNDRV_CARDS]   = SNDRV_DEFAULT_STR;
    static bool enable[SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP;
    static struct platform_device *platform_devices[SNDRV_CARDS];
    static int device_count;
    module_param_array(index, int, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(index, "Index value for " CARD_NAME " soundcard.");
    module_param_array(id, charp, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(id, "ID string for " CARD_NAME " soundcard.");
    module_param_array(enable, bool, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(enable, "Enable " CARD_NAME " soundcard.");
    MODULE_AUTHOR("Levent Guendogdu, Tobias Gehrig, Matthias Koenig");
    MODULE_DESCRIPTION("Midiman Portman2x4");
    MODULE_LICENSE("GPL");
//
// Chip specific
//
pub const PORTMAN_NUM_INPUT_PORTS: c_int = 2;
pub const PORTMAN_NUM_OUTPUT_PORTS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct portman {
    pub reg_lock: spinlock_t,
    pub card: *mut snd_card,
    pub rmidi: *mut snd_rawmidi,
    pub pardev: *mut pardevice,
    pub open_count: c_int,
    pub mode: [c_int; PORTMAN_NUM_INPUT_PORTS],
    pub midi_input: [*mut snd_rawmidi_substream; PORTMAN_NUM_INPUT_PORTS],
}

#[no_mangle]
unsafe extern "C" fn portman_free(pm: *mut portman) -> c_int {
    static int portman_free(struct portman *pm)
    {
    kfree(pm);
    return 0;
    }
    static int portman_create(struct snd_card *card,
    struct pardevice *pardev,
    struct portman **rchip)
    {
    struct portman *pm;
// rchip = NULL;
    pm = kzalloc_obj(struct portman);
    if (pm == core::ptr::null_mut())
    return -ENOMEM;
// Init chip specific data
    spin_lock_init(&pm.reg_lock);
    pm.card = card;
    pm.pardev = pardev;
// rchip = pm;
    return 0;
    }
//
// HW related constants
//
// Standard PC parallel port status register equates.
pub const PP_STAT_BSY: c_uint = 0x80	/* Busy status.  Inverted. */;
pub const PP_STAT_ACK: c_uint = 0x40	/* Acknowledge.  Non-Inverted. */;
pub const PP_STAT_POUT: c_uint = 0x20	/* Paper Out.    Non-Inverted. */;
pub const PP_STAT_SEL: c_uint = 0x10	/* Select.       Non-Inverted. */;
pub const PP_STAT_ERR: c_uint = 0x08	/* Error.        Non-Inverted. */;
// Standard PC parallel port command register equates.
pub const PP_CMD_IEN: c_uint = 0x10	/* IRQ Enable.   Non-Inverted. */;
pub const PP_CMD_SELI: c_uint = 0x08	/* Select Input. Inverted. */;
pub const PP_CMD_INIT: c_uint = 0x04	/* Init Printer. Non-Inverted. */;
pub const PP_CMD_FEED: c_uint = 0x02	/* Auto Feed.    Inverted. */;
pub const PP_CMD_STB: c_uint = 0x01	/* Strobe.       Inverted. */;
// Parallel Port Command Register as implemented by PCP2x4.

// The parallel port command register field (b1..b3) selects the
// various "registers" within the PC/P 2x4.  These are the internal
// address of these "registers" that must be written to the parallel
// port command register.
//

// Parallel Port Status Register as implemented by PCP2x4.

// Parallel Port Status Register BUSY and SELECT lines are multiplexed
// between several functions.  Depending on which 2x4 "register" is
// currently selected (b1..b3), the BUSY and SELECT lines are
// assigned as follows:
//
// SELECT LINE:                                                    A3 A2 A1
// --------
//

// RXAVAIL1    PP_STAT_SEL             /* Rx Available, channel 1.   0 0 1

// /* Reserved.                  0 1 1

// TXEMPTY1        PP_STAT_SEL     /* Tx Empty, channel 1.       1 0 1
// TXEMPTY2    PP_STAT_SEL             /* Tx Empty, channel 2.       1 1 0
// TXEMPTY3    PP_STAT_SEL             /* Tx Empty, channel 3.       1 1 1
// BUSY LINE:                                                      A3 A2 A1
// --------
//

// RXDATA1         PP_STAT_BSY     /* Rx Input Data, channel 1.  0 0 1

// Reserved.                  0 1 1

pub const PORTMAN2X4_MODE_INPUT_TRIGGERED: c_uint = 0x01;
//
// Hardware specific functions
//
#[no_mangle]
pub unsafe extern "C" fn portman_write_command(pm: *mut portman, value: u8) {
    static inline void portman_write_command(struct portman *pm, u8 value)
    {
    parport_write_control(pm.pardev.port, value);
    }
#[no_mangle]
pub unsafe extern "C" fn portman_read_status(pm: *mut portman) -> u8 {
    static inline u8 portman_read_status(struct portman *pm)
    {
    return parport_read_status(pm.pardev.port);
    }
#[no_mangle]
pub unsafe extern "C" fn portman_write_data(pm: *mut portman, value: u8) {
    static inline void portman_write_data(struct portman *pm, u8 value)
    {
    parport_write_data(pm.pardev.port, value);
    }
    static void portman_write_midi(struct portman *pm,
    int port, u8 mididata)
    {
    let mut command: c_int = ((port + 4) << 1);
// Get entering data byte and port number in BL and BH respectively.
// Set up Tx Channel address field for use with PP Cmd Register.
// Store address field in BH register.
// Inputs:      AH = Output port number (0..3).
// AL = Data byte.
// command = TXDATA0 | INT_EN;
// Align port num with address field (b1...b3),
// set address for TXDatax, Strobe=0
//
    command |= INT_EN;
// Disable interrupts so that the process is not interrupted, then
// write the address associated with the current Tx channel to the
// PP Command Reg.  Do not set the Strobe signal yet.
//
    do {
    portman_write_command(pm, command);
// While the address lines settle, write parallel output data to
// PP Data Reg.  This has no effect until Strobe signal is asserted.
//
    portman_write_data(pm, mididata);
// If PCP channel's TxEmpty is set (TxEmpty is read through the PP
// Status Register), then go write data.  Else go back and wait.
//
    } while ((portman_read_status(pm) & TXEMPTY) != TXEMPTY);
// TxEmpty is set.  Maintain PC/P destination address and assert
// Strobe through the PP Command Reg.  This will Strobe data into
// the PC/P transmitter and set the PC/P BUSY signal.
//
    portman_write_command(pm, command | STROBE);
// Wait for strobe line to settle and echo back through hardware.
// Once it has echoed back, assume that the address and data lines
// have settled!
//
    while ((portman_read_status(pm) & ESTB) == 0)
    cpu_relax();
// Release strobe and immediately re-allow interrupts.
    portman_write_command(pm, command);
    while ((portman_read_status(pm) & ESTB) == ESTB)
    cpu_relax();
// PC/P BUSY is now set.  We must wait until BUSY resets itself.
// We'll reenable ints while we're waiting.
//
    while ((portman_read_status(pm) & BUSY) == BUSY)
    cpu_relax();
// Data sent.
    }
//
// Read MIDI byte from port
// Attempt to read input byte from specified hardware input port (0..).
// Return -1 if no data
//
#[no_mangle]
unsafe extern "C" fn portman_read_midi(pm: *mut portman, port: c_int) -> c_int {
    static int portman_read_midi(struct portman *pm, int port)
    {
    let mut midi_data: c_uchar = 0;
    unsigned char cmdout;	/* Saved address+IE bit. */
// Make sure clocking edge is down before starting...
    portman_write_data(pm, 0);	/* Make sure edge is down. */
// Set destination address to PCP.
    cmdout = (port << 1) | INT_EN;	/* Address + IE + No Strobe. */
    portman_write_command(pm, cmdout);
    while ((portman_read_status(pm) & ESTB) == ESTB)
    cpu_relax();	/* Wait for strobe echo. */
// After the address lines settle, check multiplexed RxAvail signal.
// If data is available, read it.
//
    if ((portman_read_status(pm) & RXAVAIL) == 0)
    return -1;	/* No data. */
// Set the Strobe signal to enable the Rx clocking circuitry.
    portman_write_command(pm, cmdout | STROBE);	/* Write address+IE+Strobe. */
    while ((portman_read_status(pm) & ESTB) == 0)
    cpu_relax(); /* Wait for strobe echo. */
// The first data bit (msb) is already sitting on the input line.
    midi_data = (portman_read_status(pm) & 128);
    portman_write_data(pm, 1);	/* Cause rising edge, which shifts data. */
// Data bit 6.
    portman_write_data(pm, 0);	/* Cause falling edge while data settles. */
    midi_data |= (portman_read_status(pm) >> 1) & 64;
    portman_write_data(pm, 1);	/* Cause rising edge, which shifts data. */
// Data bit 5.
    portman_write_data(pm, 0);	/* Cause falling edge while data settles. */
    midi_data |= (portman_read_status(pm) >> 2) & 32;
    portman_write_data(pm, 1);	/* Cause rising edge, which shifts data. */
// Data bit 4.
    portman_write_data(pm, 0);	/* Cause falling edge while data settles. */
    midi_data |= (portman_read_status(pm) >> 3) & 16;
    portman_write_data(pm, 1);	/* Cause rising edge, which shifts data. */
// Data bit 3.
    portman_write_data(pm, 0);	/* Cause falling edge while data settles. */
    midi_data |= (portman_read_status(pm) >> 4) & 8;
    portman_write_data(pm, 1);	/* Cause rising edge, which shifts data. */
// Data bit 2.
    portman_write_data(pm, 0);	/* Cause falling edge while data settles. */
    midi_data |= (portman_read_status(pm) >> 5) & 4;
    portman_write_data(pm, 1);	/* Cause rising edge, which shifts data. */
// Data bit 1.
    portman_write_data(pm, 0);	/* Cause falling edge while data settles. */
    midi_data |= (portman_read_status(pm) >> 6) & 2;
    portman_write_data(pm, 1);	/* Cause rising edge, which shifts data. */
// Data bit 0.
    portman_write_data(pm, 0);	/* Cause falling edge while data settles. */
    midi_data |= (portman_read_status(pm) >> 7) & 1;
    portman_write_data(pm, 1);	/* Cause rising edge, which shifts data. */
    portman_write_data(pm, 0);	/* Return data clock low. */
// De-assert Strobe and return data.
    portman_write_command(pm, cmdout);	/* Output saved address+IE. */
// Wait for strobe echo.
    while ((portman_read_status(pm) & ESTB) == ESTB)
    cpu_relax();
    return (midi_data & 255);	/* Shift back and return value. */
    }
//
// Checks if any input data on the given channel is available
// Checks RxAvail
//
#[no_mangle]
unsafe extern "C" fn portman_data_avail(pm: *mut portman, channel: c_int) -> c_int {
    static int portman_data_avail(struct portman *pm, int channel)
    {
    let mut command: c_int = INT_EN;
    switch (channel) {
    case 0:
    command |= RXDATA0;
    break;
    case 1:
    command |= RXDATA1;
    break;
    }
// Write hardware (assumme STROBE=0)
    portman_write_command(pm, command);
// Check multiplexed RxAvail signal
    if ((portman_read_status(pm) & RXAVAIL) == RXAVAIL)
    return 1;	/* Data available */
// No Data available
    return 0;
    }
//
// Flushes any input
//
#[no_mangle]
unsafe extern "C" fn portman_flush_input(pm: *mut portman, port: c_uchar) {
    static void portman_flush_input(struct portman *pm, unsigned char port)
    {
// Local variable for counting things
    let mut i: c_uint = 0;
    let mut command: c_uchar = 0;
    switch (port) {
    case 0:
    command = RXDATA0;
    break;
    case 1:
    command = RXDATA1;
    break;
    default:
    dev_warn(pm.card.dev, "%s Won't flush port %i\n",
    __func__, port);
    return;
    }
// Set address for specified channel in port and allow to settle.
    portman_write_command(pm, command);
// Assert the Strobe and wait for echo back.
    portman_write_command(pm, command | STROBE);
// Wait for ESTB
    while ((portman_read_status(pm) & ESTB) == 0)
    cpu_relax();
// Output clock cycles to the Rx circuitry.
    portman_write_data(pm, 0);
// Flush 250 bits...
    for (i = 0; i < 250; i++) {
    portman_write_data(pm, 1);
    portman_write_data(pm, 0);
    }
// Deassert the Strobe signal of the port and wait for it to settle.
    portman_write_command(pm, command | INT_EN);
// Wait for settling
    while ((portman_read_status(pm) & ESTB) == ESTB)
    cpu_relax();
    }
#[no_mangle]
unsafe extern "C" fn portman_probe(p: *mut parport) -> c_int {
    static int portman_probe(struct parport *p)
    {
// Initialize the parallel port data register.  Will set Rx clocks
// low in case we happen to be addressing the Rx ports at this time.
//
// 1
    parport_write_data(p, 0);
// Initialize the parallel port command register, thus initializing
// hardware handshake lines to midi box:
//
// Strobe = 0
// Interrupt Enable = 0
//
// 2
    parport_write_control(p, 0);
// Check if Portman PC/P 2x4 is out there.
// 3
    parport_write_control(p, RXDATA0);	/* Write Strobe=0 to command reg. */
// Check for ESTB to be clear
// 4
    if ((parport_read_status(p) & ESTB) == ESTB)
    return 1;	/* CODE 1 - Strobe Failure. */
// Set for RXDATA0 where no damage will be done.
// 5
    parport_write_control(p, RXDATA0 | STROBE);	/* Write Strobe=1 to command reg. */
// 6
    if ((parport_read_status(p) & ESTB) != ESTB)
    return 1;	/* CODE 1 - Strobe Failure. */
// 7
    parport_write_control(p, 0);	/* Reset Strobe=0. */
// Check if Tx circuitry is functioning properly.  If initialized
// unit TxEmpty is false, send out char and see if it goes true.
//
// 8
    parport_write_control(p, TXDATA0);	/* Tx channel 0, strobe off. */
// If PCP channel's TxEmpty is set (TxEmpty is read through the PP
// Status Register), then go write data.  Else go back and wait.
//
// 9
    if ((parport_read_status(p) & TXEMPTY) == 0)
    return 2;
// Return OK status.
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn portman_device_init(pm: *mut portman) -> c_int {
    static int portman_device_init(struct portman *pm)
    {
    portman_flush_input(pm, 0);
    portman_flush_input(pm, 1);
    return 0;
    }
//
// Rawmidi
//
#[no_mangle]
unsafe extern "C" fn snd_portman_midi_open(substream: *mut snd_rawmidi_substream) -> c_int {
    static int snd_portman_midi_open(struct snd_rawmidi_substream *substream)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_portman_midi_close(substream: *mut snd_rawmidi_substream) -> c_int {
    static int snd_portman_midi_close(struct snd_rawmidi_substream *substream)
    {
    return 0;
    }
    static void snd_portman_midi_input_trigger(struct snd_rawmidi_substream *substream,
    int up)
    {
    struct portman *pm = substream.rmidi.private_data;
    guard(spinlock_irqsave)(&pm.reg_lock);
    if (up)
    pm.mode[substream.number] |= PORTMAN2X4_MODE_INPUT_TRIGGERED;
    else
    pm.mode[substream.number] &= ~PORTMAN2X4_MODE_INPUT_TRIGGERED;
    }
    static void snd_portman_midi_output_trigger(struct snd_rawmidi_substream *substream,
    int up)
    {
    struct portman *pm = substream.rmidi.private_data;
    unsigned char byte;
    guard(spinlock_irqsave)(&pm.reg_lock);
    if (up) {
    while ((snd_rawmidi_transmit(substream, &byte, 1) == 1))
    portman_write_midi(pm, substream.number, byte);
    }
    }
    static const struct snd_rawmidi_ops snd_portman_midi_output = {
    .open =		snd_portman_midi_open,
    .close =	snd_portman_midi_close,
    .trigger =	snd_portman_midi_output_trigger,
    };
    static const struct snd_rawmidi_ops snd_portman_midi_input = {
    .open =		snd_portman_midi_open,
    .close =	snd_portman_midi_close,
    .trigger =	snd_portman_midi_input_trigger,
    };
// Create and initialize the rawmidi component
#[no_mangle]
unsafe extern "C" fn snd_portman_rawmidi_create(card: *mut snd_card) -> c_int {
    static int snd_portman_rawmidi_create(struct snd_card *card)
    {
    struct portman *pm = card.private_data;
    struct snd_rawmidi *rmidi;
    struct snd_rawmidi_substream *substream;
    int err;
    err = snd_rawmidi_new(card, CARD_NAME, 0,
    PORTMAN_NUM_OUTPUT_PORTS,
    PORTMAN_NUM_INPUT_PORTS,
    &rmidi);
    if (err < 0)
    return err;
    rmidi.private_data = pm;
    strscpy(rmidi.name, CARD_NAME);
    rmidi.info_flags = SNDRV_RAWMIDI_INFO_OUTPUT |
    SNDRV_RAWMIDI_INFO_INPUT |
    SNDRV_RAWMIDI_INFO_DUPLEX;
    pm.rmidi = rmidi;
// register rawmidi ops
    snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_OUTPUT,
    &snd_portman_midi_output);
    snd_rawmidi_set_ops(rmidi, SNDRV_RAWMIDI_STREAM_INPUT,
    &snd_portman_midi_input);
// name substreams
// output
    list_for_each_entry(substream,
    &rmidi.streams[SNDRV_RAWMIDI_STREAM_OUTPUT].substreams,
    list) {
    sprintf(substream.name,
    "Portman2x4 %d", substream.number+1);
    }
// input
    list_for_each_entry(substream,
    &rmidi.streams[SNDRV_RAWMIDI_STREAM_INPUT].substreams,
    list) {
    pm.midi_input[substream.number] = substream;
    sprintf(substream.name,
    "Portman2x4 %d", substream.number+1);
    }
    return err;
    }
//
// parport stuff
//
#[no_mangle]
unsafe extern "C" fn snd_portman_interrupt(userdata: *mut c_void) {
    static void snd_portman_interrupt(void *userdata)
    {
    let mut midivalue: c_uchar = 0;
    struct portman *pm = ((struct snd_card*)userdata).private_data;
    guard(spinlock)(&pm.reg_lock);
// While any input data is waiting
    while ((portman_read_status(pm) & INT_REQ) == INT_REQ) {
// If data available on channel 0,
    read it and stuff it into the queue. */
    if (portman_data_avail(pm, 0)) {
// Read Midi
    midivalue = portman_read_midi(pm, 0);
// put midi into queue...
    if (pm.mode[0] & PORTMAN2X4_MODE_INPUT_TRIGGERED)
    snd_rawmidi_receive(pm.midi_input[0],
    &midivalue, 1);
    }
// If data available on channel 1,
    read it and stuff it into the queue. */
    if (portman_data_avail(pm, 1)) {
// Read Midi
    midivalue = portman_read_midi(pm, 1);
// put midi into queue...
    if (pm.mode[1] & PORTMAN2X4_MODE_INPUT_TRIGGERED)
    snd_rawmidi_receive(pm.midi_input[1],
    &midivalue, 1);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn snd_portman_attach(p: *mut parport) {
    static void snd_portman_attach(struct parport *p)
    {
    struct platform_device *device;
    device = platform_device_alloc(PLATFORM_DRIVER, device_count);
    if (!device)
    return;
// Temporary assignment to forward the parport
    platform_set_drvdata(device, p);
    if (platform_device_add(device) < 0) {
    platform_device_put(device);
    return;
    }
// Since we dont get the return value of probe
// We need to check if device probing succeeded or not
    if (!platform_get_drvdata(device)) {
    platform_device_unregister(device);
    return;
    }
// register device in global table
    platform_devices[device_count] = device;
    device_count++;
    }
#[no_mangle]
unsafe extern "C" fn snd_portman_detach(p: *mut parport) {
    static void snd_portman_detach(struct parport *p)
    {
// nothing to do here
    }
#[no_mangle]
unsafe extern "C" fn snd_portman_dev_probe(pardev: *mut pardevice) -> c_int {
    static int snd_portman_dev_probe(struct pardevice *pardev)
    {
    if (strcmp(pardev.name, DRIVER_NAME))
    return -ENODEV;
    return 0;
    }
    static struct parport_driver portman_parport_driver = {
    .name		= "portman2x4",
    .probe		= snd_portman_dev_probe,
    .match_port	= snd_portman_attach,
    .detach		= snd_portman_detach,
    };
//
// platform stuff
//
#[no_mangle]
unsafe extern "C" fn snd_portman_card_private_free(card: *mut snd_card) {
    static void snd_portman_card_private_free(struct snd_card *card)
    {
    struct portman *pm = card.private_data;
    struct pardevice *pardev = pm.pardev;
    if (pardev) {
    parport_release(pardev);
    parport_unregister_device(pardev);
    }
    portman_free(pm);
    }
#[no_mangle]
unsafe extern "C" fn snd_portman_probe(pdev: *mut platform_device) -> c_int {
    static int snd_portman_probe(struct platform_device *pdev)
    {
    struct pardevice *pardev;
    struct parport *p;
    let mut dev: c_int = pdev.id;
    struct snd_card *card = core::ptr::null_mut();
    struct portman *pm = core::ptr::null_mut();
    int err;
    struct pardev_cb portman_cb = {
    .preempt = core::ptr::null_mut(),
    .wakeup = core::ptr::null_mut(),
    .irq_func = snd_portman_interrupt,	/* ISR */
    .flags = PARPORT_DEV_EXCL,		/* flags */
    };
    p = platform_get_drvdata(pdev);
    platform_set_drvdata(pdev, core::ptr::null_mut());
    if (dev < 0) {
    dev_warn(&pdev.dev,
    "Invalid card index %d, using default 0\n", dev);
    dev = 0;
    }
    if (dev >= SNDRV_CARDS)
    return -ENODEV;
    if (!enable[dev])
    return -ENOENT;
    err = snd_card_new(&pdev.dev, index[dev], id[dev], THIS_MODULE,
    0, &card);
    if (err < 0) {
    dev_dbg(&pdev.dev, "Cannot create card\n");
    return err;
    }
    strscpy(card.driver, DRIVER_NAME);
    strscpy(card.shortname, CARD_NAME);
    sprintf(card.longname,  "%s at 0x%lx, irq %i",
    card.shortname, p.base, p.irq);
    portman_cb.private = card;			   /* private */
    pardev = parport_register_dev_model(p,		   /* port */
    DRIVER_NAME,   /* name */
    &portman_cb,   /* callbacks */
    pdev.id);	   /* device number */
    if (pardev == core::ptr::null_mut()) {
    dev_dbg(card.dev, "Cannot register pardevice\n");
    err = -EIO;
    goto __err;
    }
// claim parport
    if (parport_claim(pardev)) {
    dev_dbg(card.dev, "Cannot claim parport 0x%lx\n", pardev.port.base);
    err = -EIO;
    goto free_pardev;
    }
    err = portman_create(card, pardev, &pm);
    if (err < 0) {
    dev_dbg(card.dev, "Cannot create main component\n");
    goto release_pardev;
    }
    card.private_data = pm;
    card.private_free = snd_portman_card_private_free;
    err = portman_probe(p);
    if (err) {
    err = -EIO;
    goto __err;
    }
    err = snd_portman_rawmidi_create(card);
    if (err < 0) {
    dev_dbg(card.dev, "Creating Rawmidi component failed\n");
    goto __err;
    }
// init device
    err = portman_device_init(pm);
    if (err < 0)
    goto __err;
    platform_set_drvdata(pdev, card);
// At this point card will be usable
    err = snd_card_register(card);
    if (err < 0) {
    dev_dbg(card.dev, "Cannot register card\n");
    goto __err;
    }
    dev_info(card.dev, "Portman 2x4 on 0x%lx\n", p.base);
    return 0;
    release_pardev:
    parport_release(pardev);
    free_pardev:
    parport_unregister_device(pardev);
    __err:
    snd_card_free(card);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn snd_portman_remove(pdev: *mut platform_device) {
    static void snd_portman_remove(struct platform_device *pdev)
    {
    struct snd_card *card = platform_get_drvdata(pdev);
    if (card)
    snd_card_free(card);
    }
    static struct platform_driver snd_portman_driver = {
    .probe  = snd_portman_probe,
    .remove = snd_portman_remove,
    .driver = {
    .name = PLATFORM_DRIVER,
    }
    };
//
// module init stuff
//
#[no_mangle]
unsafe extern "C" fn snd_portman_unregister_all() {
    static void snd_portman_unregister_all(void)
    {
    int i;
    for (i = 0; i < SNDRV_CARDS; ++i) {
    if (platform_devices[i]) {
    platform_device_unregister(platform_devices[i]);
    platform_devices[i] = core::ptr::null_mut();
    }
    }
    platform_driver_unregister(&snd_portman_driver);
    parport_unregister_driver(&portman_parport_driver);
    }
#[no_mangle]
unsafe extern "C" fn snd_portman_module_init() -> int __init {
    static int __init snd_portman_module_init(void)
    {
    int err;
    err = platform_driver_register(&snd_portman_driver);
    if (err < 0)
    return err;
    if (parport_register_driver(&portman_parport_driver) != 0) {
    platform_driver_unregister(&snd_portman_driver);
    return -EIO;
    }
    if (device_count == 0) {
    snd_portman_unregister_all();
    return -ENODEV;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_portman_module_exit() -> void __exit {
    static void __exit snd_portman_module_exit(void)
    {
    snd_portman_unregister_all();
    }
    module_init(snd_portman_module_init);
    module_exit(snd_portman_module_exit);
