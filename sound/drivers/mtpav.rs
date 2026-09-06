//! Automatically rewritten from C to Rust
//! Source: sound/drivers/mtpav.c
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
// MOTU Midi Timepiece ALSA Main routines
// Copyright by Michael T. Mayers (c) Jan 09, 2000
// mail: michael@tweakoz.com
// Thanks to John Galbraith
//
// This driver is for the 'Mark Of The Unicorn' (MOTU)
// MidiTimePiece AV multiport MIDI interface
//
// IOPORTS
// -------
// 8 MIDI Ins and 8 MIDI outs
// Video Sync In (BNC), Word Sync Out (BNC),
// ADAT Sync Out (DB9)
// SMPTE in/out (1/4")
// 2 programmable pedal/footswitch inputs and 4 programmable MIDI controller knobs.
// Macintosh RS422 serial port
// RS422 "network" port for ganging multiple MTP's
// PC Parallel Port ( which this driver currently uses )
//
// MISC FEATURES
// -------------
// Hardware MIDI routing, merging, and filtering
// MIDI Synchronization to Video, ADAT, SMPTE and other Clock sources
// 128 'scene' memories, recallable from MIDI program change
//
// ChangeLog
// Jun 11 2001	Takashi Iwai <tiwai@suse.de>
// - Recoded & debugged
// - Added timer interrupt for midi outputs
// - hwports is between 1 and 8, which specifies the number of hardware ports.
// The three global ports, computer, adat and broadcast ports, are created
// always after h/w and remote ports.
//

//
// globals
//
    MODULE_AUTHOR("Michael T. Mayers");
    MODULE_DESCRIPTION("MOTU MidiTimePiece AV multiport MIDI");
    MODULE_LICENSE("GPL");
// io resources
pub const MTPAV_IOBASE: c_uint = 0x378;
pub const MTPAV_IRQ: c_int = 7;
pub const MTPAV_MAX_PORTS: c_int = 8;
    let mut index: static int = SNDRV_DEFAULT_IDX1;
    static char *id = SNDRV_DEFAULT_STR1;
    static long port = MTPAV_IOBASE;	/* 0x378, 0x278 */
    static int irq = MTPAV_IRQ;		/* 7, 5 */
    static int hwports = MTPAV_MAX_PORTS;	/* use hardware ports 1-8 */
    module_param(index, int, 0444);
    MODULE_PARM_DESC(index, "Index value for MotuMTPAV MIDI.");
    module_param(id, charp, 0444);
    MODULE_PARM_DESC(id, "ID string for MotuMTPAV MIDI.");
    module_param_hw(port, long, ioport, 0444);
    MODULE_PARM_DESC(port, "Parallel port # for MotuMTPAV MIDI.");
    module_param_hw(irq, int, irq, 0444);
    MODULE_PARM_DESC(irq, "Parallel IRQ # for MotuMTPAV MIDI.");
    module_param(hwports, int, 0444);
    MODULE_PARM_DESC(hwports, "Hardware ports # for MotuMTPAV MIDI.");
    static struct platform_device *device;
//
// defines
//
// #define USE_FAKE_MTP //       don't actually read/write to MTP device (for debugging without an actual unit) (does not work yet)
// parallel port usage masks
pub const SIGS_BYTE: c_uint = 0x08;
pub const SIGS_RFD: c_uint = 0x80;
pub const SIGS_IRQ: c_uint = 0x40;
pub const SIGS_IN0: c_uint = 0x10;
pub const SIGS_IN1: c_uint = 0x20;
pub const SIGC_WRITE: c_uint = 0x04;
pub const SIGC_READ: c_uint = 0x08;
pub const SIGC_INTEN: c_uint = 0x10;
pub const DREG: c_int = 0;
pub const SREG: c_int = 1;
pub const CREG: c_int = 2;
//
pub const MTPAV_MODE_INPUT_OPENED: c_uint = 0x01;
pub const MTPAV_MODE_OUTPUT_OPENED: c_uint = 0x02;
pub const MTPAV_MODE_INPUT_TRIGGERED: c_uint = 0x04;
pub const MTPAV_MODE_OUTPUT_TRIGGERED: c_uint = 0x08;

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtpav_port {
    pub number: u8,
    pub hwport: u8,
    pub mode: u8,
    pub running_status: u8,
    pub input: *mut snd_rawmidi_substream,
    pub output: *mut snd_rawmidi_substream,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtpav {
    pub card: *mut snd_card,
    pub port: c_ulong,
    pub res_port: *mut resource,
    pub /: *mut *mut int irq; / interrupt (for inputs),
    pub spinlock: spinlock_t,
    pub /: *mut *mut int share_irq; / number of accesses to input interrupts,
    pub /: *mut *mut int istimer; / number of accesses to timer interrupts,
    pub /: *mut *mut timer_list timer; / timer interrupts for outputs,
    pub rmidi: *mut snd_rawmidi,
    pub /: *mut *mut int num_ports; / number of hw ports (1-8),
    pub /: *mut *mut mtpav_port ports[NUMPORTS]; / all ports including computer, adat and bc,
    pub /: *mut *mut u32 inmidiport; / selected input midi port,
    pub /: *mut *mut u32 inmidistate; / during midi command 0xf5,
    pub /: *mut *mut u32 outmidihwport; / selected output midi hw port,
}

//
// possible hardware ports (selected by 0xf5 port message)
// 0x00		all ports
// 0x01 .. 0x08    this MTP's ports 1..8
// 0x09 .. 0x10    networked MTP's ports (9..16)
// 0x11            networked MTP's computer port
// 0x63            to ADAT
//
// mappig:
// subdevice 0 - (X-1)    ports
// X - (2*X-1)  networked ports
// X            computer
// X+1          ADAT
// X+2          all ports
//
// where X = chip->num_ports
//
pub const MTPAV_PIDX_COMPUTER: c_int = 0;
pub const MTPAV_PIDX_ADAT: c_int = 1;
pub const MTPAV_PIDX_BROADCAST: c_int = 2;
#[no_mangle]
unsafe extern "C" fn translate_subdevice_to_hwport(chip: *mut mtpav, subdev: c_int) -> c_int {
    static int translate_subdevice_to_hwport(struct mtpav *chip, int subdev)
    {
    if (subdev < 0)
    return 0x01; /* invalid - use port 0 as default */
#[no_mangle]
pub unsafe extern "C" fn if(chip->num_ports: subdev <) -> else {
    else if (subdev < chip.num_ports)
    return subdev + 1; /* single mtp port */
#[no_mangle]
pub unsafe extern "C" fn if(2: *mut *mut subdev < chip->num_ports) -> else {
    else if (subdev < chip.num_ports * 2)
    return subdev - chip.num_ports + 0x09; /* remote port */
#[no_mangle]
pub unsafe extern "C" fn if(MTPAV_PIDX_COMPUTER: *mut *mut subdev == chip->num_ports  2 +) -> else {
    else if (subdev == chip.num_ports * 2 + MTPAV_PIDX_COMPUTER)
    return 0x11; /* computer port */
#[no_mangle]
pub unsafe extern "C" fn if(MTPAV_PIDX_ADAT: subdev == chip->num_ports +) -> else {
    else if (subdev == chip.num_ports + MTPAV_PIDX_ADAT)
    return 0x63;		/* ADAT */
    return 0; /* all ports */
    }
#[no_mangle]
unsafe extern "C" fn translate_hwport_to_subdevice(chip: *mut mtpav, hwport: c_int) -> c_int {
    static int translate_hwport_to_subdevice(struct mtpav *chip, int hwport)
    {
    int p;
    if (hwport <= 0x00) /* all ports */
    return chip.num_ports + MTPAV_PIDX_BROADCAST;
    else if (hwport <= 0x08) { /* single port */
    p = hwport - 1;
    if (p >= chip.num_ports)
    p = 0;
    return p;
    } else if (hwport <= 0x10) { /* remote port */
    p = hwport - 0x09 + chip.num_ports;
    if (p >= chip.num_ports * 2)
    p = chip.num_ports;
    return p;
    } else if (hwport == 0x11)  /* computer port */
    return chip.num_ports + MTPAV_PIDX_COMPUTER;
    else  /* ADAT */
    return chip.num_ports + MTPAV_PIDX_ADAT;
    }
//
#[no_mangle]
unsafe extern "C" fn snd_mtpav_getreg(chip: *mut mtpav, reg: u16) -> u8 {
    static u8 snd_mtpav_getreg(struct mtpav *chip, u16 reg)
    {
    let mut rval: u8 = 0;
    if (reg == SREG) {
    rval = inb(chip.port + SREG);
    rval = (rval & 0xf8);
    } else if (reg == CREG) {
    rval = inb(chip.port + CREG);
    rval = (rval & 0x1c);
    }
    return rval;
    }
//
#[no_mangle]
pub unsafe extern "C" fn snd_mtpav_mputreg(chip: *mut mtpav, reg: u16, val: u8) {
    static inline void snd_mtpav_mputreg(struct mtpav *chip, u16 reg, u8 val)
    {
    if (reg == DREG || reg == CREG)
    outb(val, chip.port + reg);
    }
//
#[no_mangle]
unsafe extern "C" fn snd_mtpav_wait_rfdhi(chip: *mut mtpav) {
    static void snd_mtpav_wait_rfdhi(struct mtpav *chip)
    {
    let mut counts: c_int = 10000;
    u8 sbyte;
    sbyte = snd_mtpav_getreg(chip, SREG);
    while (!(sbyte & SIGS_RFD) && counts--) {
    sbyte = snd_mtpav_getreg(chip, SREG);
    udelay(10);
    }
    }
#[no_mangle]
unsafe extern "C" fn snd_mtpav_send_byte(chip: *mut mtpav, byte: u8) {
    static void snd_mtpav_send_byte(struct mtpav *chip, u8 byte)
    {
    u8 tcbyt;
    u8 clrwrite;
    u8 setwrite;
    snd_mtpav_wait_rfdhi(chip);
//
    tcbyt = snd_mtpav_getreg(chip, CREG);
    clrwrite = tcbyt & (SIGC_WRITE ^ 0xff);
    setwrite = tcbyt | SIGC_WRITE;
    snd_mtpav_mputreg(chip, DREG, byte);
    snd_mtpav_mputreg(chip, CREG, clrwrite);	// clear write bit
    snd_mtpav_mputreg(chip, CREG, setwrite);	// set write bit
    }
//
// call this with spin lock held
    static void snd_mtpav_output_port_write(struct mtpav *mtp_card,
    struct mtpav_port *portp,
    struct snd_rawmidi_substream *substream)
    {
    u8 outbyte;
// Get the outbyte first, so we can emulate running status if
// necessary
    if (snd_rawmidi_transmit(substream, &outbyte, 1) != 1)
    return;
// send port change command if necessary
    if (portp.hwport != mtp_card.outmidihwport) {
    mtp_card.outmidihwport = portp.hwport;
    snd_mtpav_send_byte(mtp_card, 0xf5);
    snd_mtpav_send_byte(mtp_card, portp.hwport);
    if (!(outbyte & 0x80) && portp.running_status)
    snd_mtpav_send_byte(mtp_card, portp.running_status);
    }
// send data
    do {
    if (outbyte & 0x80)
    portp.running_status = outbyte;
    snd_mtpav_send_byte(mtp_card, outbyte);
    } while (snd_rawmidi_transmit(substream, &outbyte, 1) == 1);
    }
#[no_mangle]
unsafe extern "C" fn snd_mtpav_output_write(substream: *mut snd_rawmidi_substream) {
    static void snd_mtpav_output_write(struct snd_rawmidi_substream *substream)
    {
    struct mtpav *mtp_card = substream.rmidi.private_data;
    struct mtpav_port *portp = &mtp_card.ports[substream.number];
    guard(spinlock_irqsave)(&mtp_card.spinlock);
    snd_mtpav_output_port_write(mtp_card, portp, substream);
    }
//
// mtpav control
//
    static void snd_mtpav_portscan(struct mtpav *chip)	// put mtp into smart routing mode
    {
    u8 p;
    for (p = 0; p < 8; p++) {
    snd_mtpav_send_byte(chip, 0xf5);
    snd_mtpav_send_byte(chip, p);
    snd_mtpav_send_byte(chip, 0xfe);
    }
    }
//
#[no_mangle]
unsafe extern "C" fn snd_mtpav_input_open(substream: *mut snd_rawmidi_substream) -> c_int {
    static int snd_mtpav_input_open(struct snd_rawmidi_substream *substream)
    {
    struct mtpav *mtp_card = substream.rmidi.private_data;
    struct mtpav_port *portp = &mtp_card.ports[substream.number];
    guard(spinlock_irqsave)(&mtp_card.spinlock);
    portp.mode |= MTPAV_MODE_INPUT_OPENED;
    portp.input = substream;
    if (mtp_card.share_irq++ == 0)
    snd_mtpav_mputreg(mtp_card, CREG, (SIGC_INTEN | SIGC_WRITE));	// enable pport interrupts
    return 0;
    }
//
#[no_mangle]
unsafe extern "C" fn snd_mtpav_input_close(substream: *mut snd_rawmidi_substream) -> c_int {
    static int snd_mtpav_input_close(struct snd_rawmidi_substream *substream)
    {
    struct mtpav *mtp_card = substream.rmidi.private_data;
    struct mtpav_port *portp = &mtp_card.ports[substream.number];
    guard(spinlock_irqsave)(&mtp_card.spinlock);
    portp.mode &= ~MTPAV_MODE_INPUT_OPENED;
    portp.input = core::ptr::null_mut();
    if (--mtp_card.share_irq == 0)
    snd_mtpav_mputreg(mtp_card, CREG, 0);	// disable pport interrupts
    return 0;
    }
//
#[no_mangle]
unsafe extern "C" fn snd_mtpav_input_trigger(substream: *mut snd_rawmidi_substream, up: c_int) {
    static void snd_mtpav_input_trigger(struct snd_rawmidi_substream *substream, int up)
    {
    struct mtpav *mtp_card = substream.rmidi.private_data;
    struct mtpav_port *portp = &mtp_card.ports[substream.number];
    guard(spinlock_irqsave)(&mtp_card.spinlock);
    if (up)
    portp.mode |= MTPAV_MODE_INPUT_TRIGGERED;
    else
    portp.mode &= ~MTPAV_MODE_INPUT_TRIGGERED;
    }
//
// timer interrupt for outputs
//
#[no_mangle]
unsafe extern "C" fn snd_mtpav_output_timer(t: *mut timer_list) {
    static void snd_mtpav_output_timer(struct timer_list *t)
    {
    struct mtpav *chip = timer_container_of(chip, t, timer);
    int p;
    guard(spinlock_irqsave)(&chip.spinlock);
// reprogram timer
    mod_timer(&chip.timer, 1 + jiffies);
// process each port
    for (p = 0; p <= chip.num_ports * 2 + MTPAV_PIDX_BROADCAST; p++) {
    struct mtpav_port *portp = &chip.ports[p];
    if ((portp.mode & MTPAV_MODE_OUTPUT_TRIGGERED) && portp.output)
    snd_mtpav_output_port_write(chip, portp, portp.output);
    }
    }
// spinlock held!
#[no_mangle]
unsafe extern "C" fn snd_mtpav_add_output_timer(chip: *mut mtpav) {
    static void snd_mtpav_add_output_timer(struct mtpav *chip)
    {
    mod_timer(&chip.timer, 1 + jiffies);
    }
// spinlock held!
#[no_mangle]
unsafe extern "C" fn snd_mtpav_remove_output_timer(chip: *mut mtpav) {
    static void snd_mtpav_remove_output_timer(struct mtpav *chip)
    {
    timer_delete(&chip.timer);
    }
//
#[no_mangle]
unsafe extern "C" fn snd_mtpav_output_open(substream: *mut snd_rawmidi_substream) -> c_int {
    static int snd_mtpav_output_open(struct snd_rawmidi_substream *substream)
    {
    struct mtpav *mtp_card = substream.rmidi.private_data;
    struct mtpav_port *portp = &mtp_card.ports[substream.number];
    guard(spinlock_irqsave)(&mtp_card.spinlock);
    portp.mode |= MTPAV_MODE_OUTPUT_OPENED;
    portp.output = substream;
    return 0;
    };
//
#[no_mangle]
unsafe extern "C" fn snd_mtpav_output_close(substream: *mut snd_rawmidi_substream) -> c_int {
    static int snd_mtpav_output_close(struct snd_rawmidi_substream *substream)
    {
    struct mtpav *mtp_card = substream.rmidi.private_data;
    struct mtpav_port *portp = &mtp_card.ports[substream.number];
    guard(spinlock_irqsave)(&mtp_card.spinlock);
    portp.mode &= ~MTPAV_MODE_OUTPUT_OPENED;
    portp.output = core::ptr::null_mut();
    return 0;
    };
//
#[no_mangle]
unsafe extern "C" fn snd_mtpav_output_trigger(substream: *mut snd_rawmidi_substream, up: c_int) {
    static void snd_mtpav_output_trigger(struct snd_rawmidi_substream *substream, int up)
    {
    struct mtpav *mtp_card = substream.rmidi.private_data;
    struct mtpav_port *portp = &mtp_card.ports[substream.number];
    scoped_guard(spinlock_irqsave, &mtp_card.spinlock) {
    if (up) {
    if ((portp.mode & MTPAV_MODE_OUTPUT_TRIGGERED)) {
    if (mtp_card.istimer++ == 0)
    snd_mtpav_add_output_timer(mtp_card);
    portp.mode |= MTPAV_MODE_OUTPUT_TRIGGERED;
    }
    } else {
    portp.mode &= ~MTPAV_MODE_OUTPUT_TRIGGERED;
    if (--mtp_card.istimer == 0)
    snd_mtpav_remove_output_timer(mtp_card);
    }
    }
    if (up)
    snd_mtpav_output_write(substream);
    }
//
// midi interrupt for inputs
//
#[no_mangle]
unsafe extern "C" fn snd_mtpav_inmidi_process(mcrd: *mut mtpav, inbyte: u8) {
    static void snd_mtpav_inmidi_process(struct mtpav *mcrd, u8 inbyte)
    {
    struct mtpav_port *portp;
    if ((int)mcrd.inmidiport > mcrd.num_ports * 2 + MTPAV_PIDX_BROADCAST)
    return;
    portp = &mcrd.ports[mcrd.inmidiport];
    if (portp.mode & MTPAV_MODE_INPUT_TRIGGERED)
    snd_rawmidi_receive(portp.input, &inbyte, 1);
    }
#[no_mangle]
unsafe extern "C" fn snd_mtpav_inmidi_h(mcrd: *mut mtpav, inbyte: u8) {
    static void snd_mtpav_inmidi_h(struct mtpav *mcrd, u8 inbyte)
    {
    if (inbyte >= 0xf8) {
// real-time midi code
    snd_mtpav_inmidi_process(mcrd, inbyte);
    return;
    }
    if (mcrd.inmidistate == 0) {	// awaiting command
    if (inbyte == 0xf5)	// MTP port #
    mcrd.inmidistate = 1;
    else
    snd_mtpav_inmidi_process(mcrd, inbyte);
    } else if (mcrd.inmidistate) {
    mcrd.inmidiport = translate_hwport_to_subdevice(mcrd, inbyte);
    mcrd.inmidistate = 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn snd_mtpav_read_bytes(mcrd: *mut mtpav) {
    static void snd_mtpav_read_bytes(struct mtpav *mcrd)
    {
    u8 clrread, setread;
    u8 mtp_read_byte;
    u8 sr, cbyt;
    int i;
    let mut sbyt: u8 = snd_mtpav_getreg(mcrd, SREG);
    if (!(sbyt & SIGS_BYTE))
    return;
    cbyt = snd_mtpav_getreg(mcrd, CREG);
    clrread = cbyt & (SIGC_READ ^ 0xff);
    setread = cbyt | SIGC_READ;
    do {
    mtp_read_byte = 0;
    for (i = 0; i < 4; i++) {
    snd_mtpav_mputreg(mcrd, CREG, setread);
    sr = snd_mtpav_getreg(mcrd, SREG);
    snd_mtpav_mputreg(mcrd, CREG, clrread);
    sr &= SIGS_IN0 | SIGS_IN1;
    sr >>= 4;
    mtp_read_byte |= sr << (i * 2);
    }
    snd_mtpav_inmidi_h(mcrd, mtp_read_byte);
    sbyt = snd_mtpav_getreg(mcrd, SREG);
    } while (sbyt & SIGS_BYTE);
    }
#[no_mangle]
unsafe extern "C" fn snd_mtpav_irqh(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t snd_mtpav_irqh(int irq, void *dev_id)
    {
    struct mtpav *mcard = dev_id;
    guard(spinlock)(&mcard.spinlock);
    snd_mtpav_read_bytes(mcard);
    return IRQ_HANDLED;
    }
//
// get ISA resources
//
#[no_mangle]
unsafe extern "C" fn snd_mtpav_get_ISA(mcard: *mut mtpav) -> c_int {
    static int snd_mtpav_get_ISA(struct mtpav *mcard)
    {
    mcard.res_port = devm_request_region(mcard.card.dev, port, 3,
    "MotuMTPAV MIDI");
    if (!mcard.res_port) {
    dev_err(mcard.card.dev, "MTVAP port 0x%lx is busy\n", port);
    return -EBUSY;
    }
    mcard.port = port;
    if (devm_request_irq(mcard.card.dev, irq, snd_mtpav_irqh, 0,
    "MOTU MTPAV", mcard)) {
    dev_err(mcard.card.dev, "MTVAP IRQ %d busy\n", irq);
    return -EBUSY;
    }
    mcard.irq = irq;
    return 0;
    }
//
    static const struct snd_rawmidi_ops snd_mtpav_output = {
    .open =		snd_mtpav_output_open,
    .close =	snd_mtpav_output_close,
    .trigger =	snd_mtpav_output_trigger,
    };
    static const struct snd_rawmidi_ops snd_mtpav_input = {
    .open =		snd_mtpav_input_open,
    .close =	snd_mtpav_input_close,
    .trigger =	snd_mtpav_input_trigger,
    };
//
// get RAWMIDI resources
//
    static void snd_mtpav_set_name(struct mtpav *chip,
    struct snd_rawmidi_substream *substream)
    {
    if (substream.number >= 0 && substream.number < chip.num_ports)
    sprintf(substream.name, "MTP direct %d", (substream.number % chip.num_ports) + 1);
#[no_mangle]
pub unsafe extern "C" fn if(2: *mut *mut substream->number >= 8 && substream->number < chip->num_ports) -> else {
    else if (substream.number >= 8 && substream.number < chip.num_ports * 2)
    sprintf(substream.name, "MTP remote %d", (substream.number % chip.num_ports) + 1);
#[no_mangle]
pub unsafe extern "C" fn if(2: *mut *mut substream->number == chip->num_ports) -> else {
    else if (substream.number == chip.num_ports * 2)
    strscpy(substream.name, "MTP computer");
#[no_mangle]
pub unsafe extern "C" fn if(1: *mut *mut substream->number == chip->num_ports  2 +) -> else {
    else if (substream.number == chip.num_ports * 2 + 1)
    strscpy(substream.name, "MTP ADAT");
    else
    strscpy(substream.name, "MTP broadcast");
    }
#[no_mangle]
unsafe extern "C" fn snd_mtpav_get_RAWMIDI(mcard: *mut mtpav) -> c_int {
    static int snd_mtpav_get_RAWMIDI(struct mtpav *mcard)
    {
    int rval;
    struct snd_rawmidi *rawmidi;
    struct snd_rawmidi_substream *substream;
    struct list_head *list;
    if (hwports < 1)
    hwports = 1;
#[no_mangle]
pub unsafe extern "C" fn if(8: hwports >) -> else {
    else if (hwports > 8)
    hwports = 8;
    mcard.num_ports = hwports;
    rval = snd_rawmidi_new(mcard.card, "MotuMIDI", 0,
    mcard.num_ports * 2 + MTPAV_PIDX_BROADCAST + 1,
    mcard.num_ports * 2 + MTPAV_PIDX_BROADCAST + 1,
    &mcard.rmidi);
    if (rval < 0)
    return rval;
    rawmidi = mcard.rmidi;
    rawmidi.private_data = mcard;
    list_for_each(list, &rawmidi.streams[SNDRV_RAWMIDI_STREAM_INPUT].substreams) {
    substream = list_entry(list, struct snd_rawmidi_substream, list);
    snd_mtpav_set_name(mcard, substream);
    substream.ops = &snd_mtpav_input;
    }
    list_for_each(list, &rawmidi.streams[SNDRV_RAWMIDI_STREAM_OUTPUT].substreams) {
    substream = list_entry(list, struct snd_rawmidi_substream, list);
    snd_mtpav_set_name(mcard, substream);
    substream.ops = &snd_mtpav_output;
    mcard.ports[substream.number].hwport = translate_subdevice_to_hwport(mcard, substream.number);
    }
    rawmidi.info_flags |= SNDRV_RAWMIDI_INFO_OUTPUT | SNDRV_RAWMIDI_INFO_INPUT |
    SNDRV_RAWMIDI_INFO_DUPLEX;
    sprintf(rawmidi.name, "MTP AV MIDI");
    return 0;
    }
//
#[no_mangle]
unsafe extern "C" fn snd_mtpav_free(card: *mut snd_card) {
    static void snd_mtpav_free(struct snd_card *card)
    {
    struct mtpav *crd = card.private_data;
    timer_shutdown_sync(&crd.timer);
    }
//
#[no_mangle]
unsafe extern "C" fn snd_mtpav_probe(dev: *mut platform_device) -> c_int {
    static int snd_mtpav_probe(struct platform_device *dev)
    {
    struct snd_card *card;
    int err;
    struct mtpav *mtp_card;
    err = snd_devm_card_new(&dev.dev, index, id, THIS_MODULE,
    sizeof(*mtp_card), &card);
    if (err < 0)
    return err;
    mtp_card = card.private_data;
    spin_lock_init(&mtp_card.spinlock);
    mtp_card.card = card;
    mtp_card.irq = -1;
    mtp_card.share_irq = 0;
    mtp_card.inmidistate = 0;
    mtp_card.outmidihwport = 0xffffffff;
    timer_setup(&mtp_card.timer, snd_mtpav_output_timer, 0);
    err = snd_mtpav_get_RAWMIDI(mtp_card);
    if (err < 0)
    return err;
    mtp_card.inmidiport = mtp_card.num_ports + MTPAV_PIDX_BROADCAST;
    err = snd_mtpav_get_ISA(mtp_card);
    if (err < 0)
    return err;
    strscpy(card.driver, "MTPAV");
    strscpy(card.shortname, "MTPAV on parallel port");
    snprintf(card.longname, sizeof(card.longname),
    "MTPAV on parallel port at 0x%lx", port);
    snd_mtpav_portscan(mtp_card);
    err = snd_card_register(mtp_card.card);
    if (err < 0)
    return err;
    card.private_free = snd_mtpav_free;
    platform_set_drvdata(dev, card);
    dev_info(card.dev,
    "Motu MidiTimePiece on parallel port irq: %d ioport: 0x%lx\n",
    irq, port);
    return 0;
    }

    static struct platform_driver snd_mtpav_driver = {
    .probe		= snd_mtpav_probe,
    .driver		= {
    .name	= SND_MTPAV_DRIVER,
    },
    };
#[no_mangle]
unsafe extern "C" fn alsa_card_mtpav_init() -> int __init {
    static int __init alsa_card_mtpav_init(void)
    {
    int err;
    err = platform_driver_register(&snd_mtpav_driver);
    if (err < 0)
    return err;
    device = platform_device_register_simple(SND_MTPAV_DRIVER, -1, core::ptr::null_mut(), 0);
    if (!IS_ERR(device)) {
    if (platform_get_drvdata(device))
    return 0;
    platform_device_unregister(device);
    err = -ENODEV;
    } else
    err = PTR_ERR(device);
    platform_driver_unregister(&snd_mtpav_driver);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn alsa_card_mtpav_exit() -> void __exit {
    static void __exit alsa_card_mtpav_exit(void)
    {
    platform_device_unregister(device);
    platform_driver_unregister(&snd_mtpav_driver);
    }
    module_init(alsa_card_mtpav_init)
    module_exit(alsa_card_mtpav_exit)
