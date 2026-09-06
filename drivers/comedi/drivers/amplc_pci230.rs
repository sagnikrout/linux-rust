//! Automatically rewritten from C to Rust
//! Source: drivers/comedi/drivers/amplc_pci230.c
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
// comedi/drivers/amplc_pci230.c
// Driver for Amplicon PCI230 and PCI260 Multifunction I/O boards.
//
// Copyright (C) 2001 Allan Willcox <allanwillcox@ozemail.com.au>
//
// COMEDI - Linux Control and Measurement Device Interface
// Copyright (C) 2000 David A. Schleef <ds@schleef.org>
//
// Driver: amplc_pci230
// Description: Amplicon PCI230, PCI260 Multifunction I/O boards
// Author: Allan Willcox <allanwillcox@ozemail.com.au>,
// Steve D Sharples <steve.sharples@nottingham.ac.uk>,
// Ian Abbott <abbotti@mev.co.uk>
// Updated: Mon, 01 Sep 2014 10:09:16 +0000
// Devices: [Amplicon] PCI230 (amplc_pci230), PCI230+, PCI260, PCI260+
// Status: works
//
// Configuration options:
// none
//
// Manual configuration of PCI cards is not supported; they are configured
// automatically.
//
// The PCI230+ and PCI260+ have the same PCI device IDs as the PCI230 and
// PCI260, but can be distinguished by the size of the PCI regions.  A
// card will be configured as a "+" model if detected as such.
//
// Subdevices:
//
// PCI230(+)    PCI260(+)
// ---------    ---------
// Subdevices       3            1
// 0          AI           AI
// 1          AO
// 2          DIO
//
// AI Subdevice:
//
// The AI subdevice has 16 single-ended channels or 8 differential
// channels.
//
// The PCI230 and PCI260 cards have 12-bit resolution.  The PCI230+ and
// PCI260+ cards have 16-bit resolution.
//
// For differential mode, use inputs 2N and 2N+1 for channel N (e.g. use
// inputs 14 and 15 for channel 7).  If the card is physically a PCI230
// or PCI260 then it actually uses a "pseudo-differential" mode where the
// inputs are sampled a few microseconds apart.  The PCI230+ and PCI260+
// use true differential sampling.  Another difference is that if the
// card is physically a PCI230 or PCI260, the inverting input is 2N,
// whereas for a PCI230+ or PCI260+ the inverting input is 2N+1.  So if a
// PCI230 is physically replaced by a PCI230+ (or a PCI260 with a
// PCI260+) and differential mode is used, the differential inputs need
// to be physically swapped on the connector.
//
// The following input ranges are supported:
//
// 0 => [-10, +10] V
// 1 => [-5, +5] V
// 2 => [-2.5, +2.5] V
// 3 => [-1.25, +1.25] V
// 4 => [0, 10] V
// 5 => [0, 5] V
// 6 => [0, 2.5] V
//
// AI Commands:
//
// +=========+==============+===========+============+==========+
// |start_src|scan_begin_src|convert_src|scan_end_src| stop_src |
// +=========+==============+===========+============+==========+
// |TRIG_NOW | TRIG_FOLLOW  |TRIG_TIMER | TRIG_COUNT |TRIG_NONE |
// |TRIG_INT |              |TRIG_EXT(3)|            |TRIG_COUNT|
// |         |              |TRIG_INT   |            |          |
// |         |--------------|-----------|            |          |
// |         | TRIG_TIMER(1)|TRIG_TIMER |            |          |
// |         | TRIG_EXT(2)  |           |            |          |
// |         | TRIG_INT     |           |            |          |
// +---------+--------------+-----------+------------+----------+
//
// Note 1: If AI command and AO command are used simultaneously, only
// one may have scan_begin_src == TRIG_TIMER.
//
// Note 2: For PCI230 and PCI230+, scan_begin_src == TRIG_EXT uses
// DIO channel 16 (pin 49) which will need to be configured as
// a digital input.  For PCI260+, the EXTTRIG/EXTCONVCLK input
// (pin 17) is used instead.  For PCI230, scan_begin_src ==
// TRIG_EXT is not supported.  The trigger is a rising edge
// on the input.
//
// Note 3: For convert_src == TRIG_EXT, the EXTTRIG/EXTCONVCLK input
// (pin 25 on PCI230(+), pin 17 on PCI260(+)) is used.  The
// convert_arg value is interpreted as follows:
//
// convert_arg == (CR_EDGE | 0) => rising edge
// convert_arg == (CR_EDGE | CR_INVERT | 0) => falling edge
// convert_arg == 0 => falling edge (backwards compatibility)
// convert_arg == 1 => rising edge (backwards compatibility)
//
// All entries in the channel list must use the same analogue reference.
// If the analogue reference is not AREF_DIFF (not differential) each
// pair of channel numbers (0 and 1, 2 and 3, etc.) must use the same
// input range.  The input ranges used in the sequence must be all
// bipolar (ranges 0 to 3) or all unipolar (ranges 4 to 6).  The channel
// sequence must consist of 1 or more identical subsequences.  Within the
// subsequence, channels must be in ascending order with no repeated
// channels.  For example, the following sequences are valid: 0 1 2 3
// (single valid subsequence), 0 2 3 5 0 2 3 5 (repeated valid
// subsequence), 1 1 1 1 (repeated valid subsequence).  The following
// sequences are invalid: 0 3 2 1 (invalid subsequence), 0 2 3 5 0 2 3
// (incompletely repeated subsequence).  Some versions of the PCI230+ and
// PCI260+ have a bug that requires a subsequence longer than one entry
// long to include channel 0.
//
// AO Subdevice:
//
// The AO subdevice has 2 channels with 12-bit resolution.
// The following output ranges are supported:
// 0 => [0, 10] V
// 1 => [-10, +10] V
//
// AO Commands:
//
// +=========+==============+===========+============+==========+
// |start_src|scan_begin_src|convert_src|scan_end_src| stop_src |
// +=========+==============+===========+============+==========+
// |TRIG_INT | TRIG_TIMER(1)| TRIG_NOW  | TRIG_COUNT |TRIG_NONE |
// |         | TRIG_EXT(2)  |           |            |TRIG_COUNT|
// |         | TRIG_INT     |           |            |          |
// +---------+--------------+-----------+------------+----------+
//
// Note 1: If AI command and AO command are used simultaneously, only
// one may have scan_begin_src == TRIG_TIMER.
//
// Note 2: scan_begin_src == TRIG_EXT is only supported if the card is
// configured as a PCI230+ and is only supported on later
// versions of the card.  As a card configured as a PCI230+ is
// not guaranteed to support external triggering, please consider
// this support to be a bonus.  It uses the EXTTRIG/ EXTCONVCLK
// input (PCI230+ pin 25).  Triggering will be on the rising edge
// unless the CR_INVERT flag is set in scan_begin_arg.
//
// The channels in the channel sequence must be in ascending order with
// no repeats.  All entries in the channel sequence must use the same
// output range.
//
// DIO Subdevice:
//
// The DIO subdevice is a 8255 chip providing 24 DIO channels.  The DIO
// channels are configurable as inputs or outputs in four groups:
//
// Port A  - channels  0 to  7
// Port B  - channels  8 to 15
// Port CL - channels 16 to 19
// Port CH - channels 20 to 23
//
// Only mode 0 of the 8255 chip is supported.
//
// Bit 0 of port C (DIO channel 16) is also used as an external scan
// trigger input for AI commands on PCI230 and PCI230+, so would need to
// be configured as an input to use it for that purpose.
//
// Extra triggered scan functionality, interrupt bug-fix added by Steve
// Sharples.  Support for PCI230+/260+, more triggered scan functionality,
// and workarounds for (or detection of) various hardware problems added
// by Ian Abbott.
//

//
// PCI230 PCI configuration register information
//
pub const PCI_DEVICE_ID_PCI230: c_uint = 0x0000;
pub const PCI_DEVICE_ID_PCI260: c_uint = 0x0006;
//
// PCI230 i/o space 1 registers.
//
pub const PCI230_PPI_X_BASE: c_uint = 0x00	/* User PPI (82C55) base */;
pub const PCI230_PPI_X_A: c_uint = 0x00	/* User PPI (82C55) port A */;
pub const PCI230_PPI_X_B: c_uint = 0x01	/* User PPI (82C55) port B */;
pub const PCI230_PPI_X_C: c_uint = 0x02	/* User PPI (82C55) port C */;
pub const PCI230_PPI_X_CMD: c_uint = 0x03	/* User PPI (82C55) control word */;
pub const PCI230_Z2_CT_BASE: c_uint = 0x14	/* 82C54 counter/timer base */;
pub const PCI230_ZCLK_SCE: c_uint = 0x1A	/* Group Z Clock Configuration */;
pub const PCI230_ZGAT_SCE: c_uint = 0x1D	/* Group Z Gate Configuration */;
pub const PCI230_INT_SCE: c_uint = 0x1E	/* Interrupt source mask (w) */;
pub const PCI230_INT_STAT: c_uint = 0x1E	/* Interrupt status (r) */;
//
// PCI230 i/o space 2 registers.
//
pub const PCI230_DACCON: c_uint = 0x00	/* DAC control */;
pub const PCI230_DACOUT1: c_uint = 0x02	/* DAC channel 0 (w) */;
pub const PCI230_DACOUT2: c_uint = 0x04	/* DAC channel 1 (w) (not FIFO mode) */;
pub const PCI230_ADCDATA: c_uint = 0x08	/* ADC data (r) */;
pub const PCI230_ADCSWTRIG: c_uint = 0x08	/* ADC software trigger (w) */;
pub const PCI230_ADCCON: c_uint = 0x0A	/* ADC control */;
pub const PCI230_ADCEN: c_uint = 0x0C	/* ADC channel enable bits */;
pub const PCI230_ADCG: c_uint = 0x0E	/* ADC gain control bits */;
// PCI230+ i/o space 2 additional registers.
pub const PCI230P_ADCTRIG: c_uint = 0x10	/* ADC start acquisition trigger */;
pub const PCI230P_ADCTH: c_uint = 0x12	/* ADC analog trigger threshold */;
pub const PCI230P_ADCFFTH: c_uint = 0x14	/* ADC FIFO interrupt threshold */;
pub const PCI230P_ADCFFLEV: c_uint = 0x16	/* ADC FIFO level (r) */;
pub const PCI230P_ADCPTSC: c_uint = 0x18	/* ADC pre-trigger sample count (r) */;
pub const PCI230P_ADCHYST: c_uint = 0x1A	/* ADC analog trigger hysteresys */;
pub const PCI230P_EXTFUNC: c_uint = 0x1C	/* Extended functions */;
pub const PCI230P_HWVER: c_uint = 0x1E	/* Hardware version (r) */;
// PCI230+ hardware version 2 onwards.
pub const PCI230P2_DACDATA: c_uint = 0x02	/* DAC data (FIFO mode) (w) */;
pub const PCI230P2_DACSWTRIG: c_uint = 0x02	/* DAC soft trigger (FIFO mode) (r) */;
pub const PCI230P2_DACEN: c_uint = 0x06	/* DAC channel enable (FIFO mode) */;
//
// DACCON read-write values.
//

//
// The following applies only if DAC FIFO support is enabled in the EXTFUNC
// register (and only for PCI230+ hardware version 2 onwards).
//

//
// The following apply only if the DAC FIFO is enabled (and only for PCI230+
// hardware version 2 onwards).
//

//
// DACCON read-only values.
//

//
// The following apply only if the DAC FIFO is enabled (and only for PCI230+
// hardware version 2 onwards).
//

//
// DACCON write-only, transient values.
//
// The following apply only if the DAC FIFO is enabled (and only for PCI230+
// hardware version 2 onwards).
//

//
// PCI230+ hardware version 2 DAC FIFO levels.
//
pub const PCI230P2_DAC_FIFOLEVEL_HALF: c_int = 512;
pub const PCI230P2_DAC_FIFOLEVEL_FULL: c_int = 1024;
// Free space in DAC FIFO.

    (PCI230P2_DAC_FIFOLEVEL_FULL - PCI230P2_DAC_FIFOLEVEL_HALF)
pub const PCI230P2_DAC_FIFOROOM_HALFTOFULL: c_int = 1;
pub const PCI230P2_DAC_FIFOROOM_FULL: c_int = 0;
//
// ADCCON read/write values.
//

//
// ADCCON write-only, transient values.
//

//
// ADCCON read-only values.
//

//
// PCI230 ADC FIFO levels.
//

//
// PCI230+ EXTFUNC values.
//
// Route EXTTRIG pin to external gate inputs.

// PCI230+ hardware version 2 values.
// Allow DAC FIFO to be enabled.

//
// Counter/timer clock input configuration sources.
//

#[no_mangle]
unsafe extern "C" fn pci230_clk_config(chan: c_uint, src: c_uint) -> c_uint {
    static unsigned int pci230_clk_config(unsigned int chan, unsigned int src)
    {
    return ((chan & 3) << 3) | (src & 7);
    }
//
// Counter/timer gate input configuration sources.
//

#[no_mangle]
unsafe extern "C" fn pci230_gat_config(chan: c_uint, src: c_uint) -> c_uint {
    static unsigned int pci230_gat_config(unsigned int chan, unsigned int src)
    {
    return ((chan & 3) << 3) | (src & 7);
    }
//
// Summary of CLK_OUTNM1 and GAT_NOUTNM2 connections for PCI230 and PCI260:
//
// Channel's       Channel's
// clock input     gate input
// Channel      CLK_OUTNM1      GAT_NOUTNM2
// -------      ----------      -----------
// Z2-CT0       Z2-CT2-OUT      /Z2-CT1-OUT
// Z2-CT1       Z2-CT0-OUT      /Z2-CT2-OUT
// Z2-CT2       Z2-CT1-OUT      /Z2-CT0-OUT
//
// Interrupt enables/status register values.
//
pub const PCI230_INT_DISABLE: c_int = 0;

// For PCI230+ hardware version 2 when DAC FIFO enabled.

//
// (Potentially) shared resources and their owners
//
    enum {
    RES_Z2CT0 = BIT(0),	/* Z2-CT0 */
    RES_Z2CT1 = BIT(1),	/* Z2-CT1 */
    RES_Z2CT2 = BIT(2)	/* Z2-CT2 */
    };
    enum {
    OWNER_AICMD,		/* Owned by AI command */
    OWNER_AOCMD,		/* Owned by AO command */
    NUM_OWNERS		/* Number of owners */
    };
//
// Handy macros.
//
// Combine old and new bits.

// Current CPU.  XXX should this be hard_smp_processor_id()?

//
// Board descriptions for the two boards supported.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci230_board {
    pub name: *const c_char,
    pub id: c_ushort,
    pub ai_bits: c_uchar,
    pub ao_bits: c_uchar,
    pub /: *mut *mut unsigned char min_hwver; / Minimum hardware version supported.,
    pub have_dio:1: c_uint,
}

    static const struct pci230_board pci230_boards[] = {
    {
    .name		= "pci230+",
    .id		= PCI_DEVICE_ID_PCI230,
    .ai_bits	= 16,
    .ao_bits	= 12,
    .have_dio	= true,
    .min_hwver	= 1,
    },
    {
    .name		= "pci260+",
    .id		= PCI_DEVICE_ID_PCI260,
    .ai_bits	= 16,
    .min_hwver	= 1,
    },
    {
    .name		= "pci230",
    .id		= PCI_DEVICE_ID_PCI230,
    .ai_bits	= 12,
    .ao_bits	= 12,
    .have_dio	= true,
    },
    {
    .name		= "pci260",
    .id		= PCI_DEVICE_ID_PCI260,
    .ai_bits	= 12,
    },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci230_private {
    pub /: *mut *mut spinlock_t isr_spinlock; / Interrupt spin lock,
    pub /: *mut *mut spinlock_t res_spinlock; / Shared resources spin lock,
    pub /: *mut *mut spinlock_t ai_stop_spinlock; / Spin lock for stopping AI command,
    pub /: *mut *mut spinlock_t ao_stop_spinlock; / Spin lock for stopping AO command,
    pub /: *mut *mut unsigned long daqio; / PCI230's DAQ I/O space,
    pub /: *mut *mut int intr_cpuid; / ID of CPU running ISR,
    pub /: *mut *mut unsigned short hwver; / Hardware version (for '+' models),
    pub /: *mut *mut unsigned short adccon; / ADCCON register value,
    pub /: *mut *mut unsigned short daccon; / DACCON register value,
    pub /: *mut *mut unsigned short adcfifothresh; / ADC FIFO threshold (PCI230+/260+),
    pub /: *mut *mut unsigned short adcg; / ADCG register value,
    pub /: *mut *mut unsigned char ier; / Interrupt enable bits,
    pub /: *mut *mut unsigned char res_owned[NUM_OWNERS]; / Owned resources,
    pub /: *mut *mut unsigned int intr_running:1; / Flag set in interrupt routine,
    pub /: *mut *mut unsigned int ai_bipolar:1; / Flag AI range is bipolar,
    pub /: *mut *mut unsigned int ao_bipolar:1; / Flag AO range is bipolar,
    pub /: *mut *mut unsigned int ai_cmd_started:1; / Flag AI command started,
    pub /: *mut *mut unsigned int ao_cmd_started:1; / Flag AO command started,
}

// PCI230 clock source periods in ns
    static const unsigned int pci230_timebase[8] = {
    [CLK_10MHZ]	= I8254_OSC_BASE_10MHZ,
    [CLK_1MHZ]	= I8254_OSC_BASE_1MHZ,
    [CLK_100KHZ]	= I8254_OSC_BASE_100KHZ,
    [CLK_10KHZ]	= I8254_OSC_BASE_10KHZ,
    [CLK_1KHZ]	= I8254_OSC_BASE_1KHZ,
    };
// PCI230 analogue input range table
    static const struct comedi_lrange pci230_ai_range = {
    7, {
    BIP_RANGE(10),
    BIP_RANGE(5),
    BIP_RANGE(2.5),
    BIP_RANGE(1.25),
    UNI_RANGE(10),
    UNI_RANGE(5),
    UNI_RANGE(2.5)
    }
    };
// PCI230 analogue gain bits for each input range.
    static const unsigned char pci230_ai_gain[7] = { 0, 1, 2, 3, 1, 2, 3 };
// PCI230 analogue output range table
    static const struct comedi_lrange pci230_ao_range = {
    2, {
    UNI_RANGE(10),
    BIP_RANGE(10)
    }
    };
#[no_mangle]
unsafe extern "C" fn pci230_ai_read(dev: *mut comedi_device) -> c_ushort {
    static unsigned short pci230_ai_read(struct comedi_device *dev)
    {
    const struct pci230_board *board = dev.board_ptr;
    struct pci230_private *devpriv = dev.private;
    unsigned short data;
// Read sample.
    data = inw(devpriv.daqio + PCI230_ADCDATA);
//
// PCI230 is 12 bit - stored in upper bits of 16 bit register
// (lower four bits reserved for expansion).  PCI230+ is 16 bit AI.
//
// If a bipolar range was specified, mangle it
// (twos complement->straight binary).
//
    if (devpriv.ai_bipolar)
    data ^= 0x8000;
    data >>= (16 - board.ai_bits);
    return data;
    }
    static unsigned short pci230_ao_mangle_datum(struct comedi_device *dev,
    unsigned short datum)
    {
    const struct pci230_board *board = dev.board_ptr;
    struct pci230_private *devpriv = dev.private;
//
// PCI230 is 12 bit - stored in upper bits of 16 bit register (lower
// four bits reserved for expansion).  PCI230+ is also 12 bit AO.
//
    datum <<= (16 - board.ao_bits);
//
// If a bipolar range was specified, mangle it
// (straight binary->twos complement).
//
    if (devpriv.ao_bipolar)
    datum ^= 0x8000;
    return datum;
    }
    static void pci230_ao_write_nofifo(struct comedi_device *dev,
    unsigned short datum, unsigned int chan)
    {
    struct pci230_private *devpriv = dev.private;
// Write mangled datum to appropriate DACOUT register.
    outw(pci230_ao_mangle_datum(dev, datum),
    devpriv.daqio + ((chan == 0) ? PCI230_DACOUT1 : PCI230_DACOUT2));
    }
    static void pci230_ao_write_fifo(struct comedi_device *dev,
    unsigned short datum, unsigned int chan)
    {
    struct pci230_private *devpriv = dev.private;
// Write mangled datum to appropriate DACDATA register.
    outw(pci230_ao_mangle_datum(dev, datum),
    devpriv.daqio + PCI230P2_DACDATA);
    }
    static bool pci230_claim_shared(struct comedi_device *dev,
    unsigned char res_mask, unsigned int owner)
    {
    struct pci230_private *devpriv = dev.private;
    unsigned int o;
    unsigned long irqflags;
    spin_lock_irqsave(&devpriv.res_spinlock, irqflags);
    for (o = 0; o < NUM_OWNERS; o++) {
    if (o == owner)
    continue;
    if (devpriv.res_owned[o] & res_mask) {
    spin_unlock_irqrestore(&devpriv.res_spinlock,
    irqflags);
    return false;
    }
    }
    devpriv.res_owned[owner] |= res_mask;
    spin_unlock_irqrestore(&devpriv.res_spinlock, irqflags);
    return true;
    }
    static void pci230_release_shared(struct comedi_device *dev,
    unsigned char res_mask, unsigned int owner)
    {
    struct pci230_private *devpriv = dev.private;
    unsigned long irqflags;
    spin_lock_irqsave(&devpriv.res_spinlock, irqflags);
    devpriv.res_owned[owner] &= ~res_mask;
    spin_unlock_irqrestore(&devpriv.res_spinlock, irqflags);
    }
    static void pci230_release_all_resources(struct comedi_device *dev,
    unsigned int owner)
    {
    pci230_release_shared(dev, (unsigned char)~0, owner);
    }
    static unsigned int pci230_divide_ns(u64 ns, unsigned int timebase,
    unsigned int flags)
    {
    u64 div;
    unsigned int rem;
    div = ns;
    rem = do_div(div, timebase);
    switch (flags & CMDF_ROUND_MASK) {
    default:
    case CMDF_ROUND_NEAREST:
    div += DIV_ROUND_CLOSEST(rem, timebase);
    break;
    case CMDF_ROUND_DOWN:
    break;
    case CMDF_ROUND_UP:
    div += DIV_ROUND_UP(rem, timebase);
    break;
    }
    return div > UINT_MAX ? UINT_MAX : (unsigned int)div;
    }
//
// Given desired period in ns, returns the required internal clock source
// and gets the initial count.
//
    static unsigned int pci230_choose_clk_count(u64 ns, unsigned int *count,
    unsigned int flags)
    {
    unsigned int clk_src, cnt;
    for (clk_src = CLK_10MHZ;; clk_src++) {
    cnt = pci230_divide_ns(ns, pci230_timebase[clk_src], flags);
    if (cnt <= 65536 || clk_src == CLK_1KHZ)
    break;
    }
// count = cnt;
    return clk_src;
    }
#[no_mangle]
unsafe extern "C" fn pci230_ns_to_single_timer(ns: *mut c_uint, flags: c_uint) {
    static void pci230_ns_to_single_timer(unsigned int *ns, unsigned int flags)
    {
    unsigned int count;
    unsigned int clk_src;
    clk_src = pci230_choose_clk_count(*ns, &count, flags);
// ns = count * pci230_timebase[clk_src];
    }
    static void pci230_ct_setup_ns_mode(struct comedi_device *dev, unsigned int ct,
    unsigned int mode, u64 ns,
    unsigned int flags)
    {
    unsigned int clk_src;
    unsigned int count;
// Set mode.
    comedi_8254_set_mode(dev.pacer, ct, mode);
// Determine clock source and count.
    clk_src = pci230_choose_clk_count(ns, &count, flags);
// Program clock source.
    outb(pci230_clk_config(ct, clk_src), dev.iobase + PCI230_ZCLK_SCE);
// Set initial count.
    if (count >= 65536)
    count = 0;
    comedi_8254_write(dev.pacer, ct, count);
    }
#[no_mangle]
unsafe extern "C" fn pci230_cancel_ct(dev: *mut comedi_device, ct: c_uint) {
    static void pci230_cancel_ct(struct comedi_device *dev, unsigned int ct)
    {
// Counter ct, 8254 mode 1, initial count not written.
    comedi_8254_set_mode(dev.pacer, ct, I8254_MODE1);
    }
    static int pci230_ai_eoc(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned long context)
    {
    struct pci230_private *devpriv = dev.private;
    unsigned int status;
    status = inw(devpriv.daqio + PCI230_ADCCON);
    if ((status & PCI230_ADC_FIFO_EMPTY) == 0)
    return 0;
    return -EBUSY;
    }
    static int pci230_ai_insn_read(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn, unsigned int *data)
    {
    struct pci230_private *devpriv = dev.private;
    unsigned int n;
    unsigned int chan, range, aref;
    unsigned int gainshift;
    unsigned short adccon, adcen;
    int ret;
// Unpack channel and range.
    chan = CR_CHAN(insn.chanspec);
    range = CR_RANGE(insn.chanspec);
    aref = CR_AREF(insn.chanspec);
    if (aref == AREF_DIFF) {
// Differential.
    if (chan >= s.n_chan / 2) {
    dev_dbg(dev.class_dev,
    "%s: differential channel number out of range 0 to %u\n",
    __func__, (s.n_chan / 2) - 1);
    return -EINVAL;
    }
    }
//
// Use Z2-CT2 as a conversion trigger instead of the built-in
// software trigger, as otherwise triggering of differential channels
// doesn't work properly for some versions of PCI230/260.  Also set
// FIFO mode because the ADC busy bit only works for software triggers.
//
    adccon = PCI230_ADC_TRIG_Z2CT2 | PCI230_ADC_FIFO_EN;
// Set Z2-CT2 output low to avoid any false triggers.
    comedi_8254_set_mode(dev.pacer, 2, I8254_MODE0);
    devpriv.ai_bipolar = comedi_range_is_bipolar(s, range);
    if (aref == AREF_DIFF) {
// Differential.
    gainshift = chan * 2;
    if (devpriv.hwver == 0) {
//
// Original PCI230/260 expects both inputs of the
// differential channel to be enabled.
//
    adcen = 3 << gainshift;
    } else {
//
// PCI230+/260+ expects only one input of the
// differential channel to be enabled.
//
    adcen = 1 << gainshift;
    }
    adccon |= PCI230_ADC_IM_DIF;
    } else {
// Single ended.
    adcen = 1 << chan;
    gainshift = chan & ~1;
    adccon |= PCI230_ADC_IM_SE;
    }
    devpriv.adcg = (devpriv.adcg & ~(3 << gainshift)) |
    (pci230_ai_gain[range] << gainshift);
    if (devpriv.ai_bipolar)
    adccon |= PCI230_ADC_IR_BIP;
    else
    adccon |= PCI230_ADC_IR_UNI;
//
// Enable only this channel in the scan list - otherwise by default
// we'll get one sample from each channel.
//
    outw(adcen, devpriv.daqio + PCI230_ADCEN);
// Set gain for channel.
    outw(devpriv.adcg, devpriv.daqio + PCI230_ADCG);
// Specify uni/bip, se/diff, conversion source, and reset FIFO.
    devpriv.adccon = adccon;
    outw(adccon | PCI230_ADC_FIFO_RESET, devpriv.daqio + PCI230_ADCCON);
// Convert n samples
    for (n = 0; n < insn.n; n++) {
//
// Trigger conversion by toggling Z2-CT2 output
// (finish with output high).
//
    comedi_8254_set_mode(dev.pacer, 2, I8254_MODE0);
    comedi_8254_set_mode(dev.pacer, 2, I8254_MODE1);
// wait for conversion to end
    ret = comedi_timeout(dev, s, insn, pci230_ai_eoc, 0);
    if (ret)
    return ret;
// read data
    data[n] = pci230_ai_read(dev);
    }
// return the number of samples read/written
    return n;
    }
    static int pci230_ao_insn_write(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_insn *insn,
    unsigned int *data)
    {
    struct pci230_private *devpriv = dev.private;
    let mut chan: c_uint = CR_CHAN(insn.chanspec);
    let mut range: c_uint = CR_RANGE(insn.chanspec);
    let mut val: c_uint = s.readback[chan];
    int i;
//
// Set range - see analogue output range table; 0 => unipolar 10V,
// 1 => bipolar +/-10V range scale
//
    devpriv.ao_bipolar = comedi_range_is_bipolar(s, range);
    outw(range, devpriv.daqio + PCI230_DACCON);
    for (i = 0; i < insn.n; i++) {
    val = data[i];
    pci230_ao_write_nofifo(dev, val, chan);
    }
    s.readback[chan] = val;
    return insn.n;
    }
    static int pci230_ao_check_chanlist(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_cmd *cmd)
    {
    let mut prev_chan: c_uint = CR_CHAN(cmd.chanlist[0]);
    let mut range0: c_uint = CR_RANGE(cmd.chanlist[0]);
    int i;
    for (i = 1; i < cmd.chanlist_len; i++) {
    let mut chan: c_uint = CR_CHAN(cmd.chanlist[i]);
    let mut range: c_uint = CR_RANGE(cmd.chanlist[i]);
    if (chan < prev_chan) {
    dev_dbg(dev.class_dev,
    "%s: channel numbers must increase\n",
    __func__);
    return -EINVAL;
    }
    if (range != range0) {
    dev_dbg(dev.class_dev,
    "%s: channels must have the same range\n",
    __func__);
    return -EINVAL;
    }
    prev_chan = chan;
    }
    return 0;
    }
    static int pci230_ao_cmdtest(struct comedi_device *dev,
    struct comedi_subdevice *s, struct comedi_cmd *cmd)
    {
    const struct pci230_board *board = dev.board_ptr;
    struct pci230_private *devpriv = dev.private;
    let mut err: c_int = 0;
    unsigned int tmp;
// Step 1 : check if triggers are trivially valid
    err |= comedi_check_trigger_src(&cmd.start_src, TRIG_INT);
    tmp = TRIG_TIMER | TRIG_INT;
    if (board.min_hwver > 0 && devpriv.hwver >= 2) {
//
// For PCI230+ hardware version 2 onwards, allow external
// trigger from EXTTRIG/EXTCONVCLK input (PCI230+ pin 25).
//
// FIXME: The permitted scan_begin_src values shouldn't depend
// on devpriv->hwver (the detected card's actual hardware
// version).  They should only depend on board->min_hwver
// (the static capabilities of the configured card).  To fix
// it, a new card model, e.g. "pci230+2" would have to be
// defined with min_hwver set to 2.  It doesn't seem worth it
// for this alone.  At the moment, please consider
// scan_begin_src==TRIG_EXT support to be a bonus rather than a
// guarantee!
//
    tmp |= TRIG_EXT;
    }
    err |= comedi_check_trigger_src(&cmd.scan_begin_src, tmp);
    err |= comedi_check_trigger_src(&cmd.convert_src, TRIG_NOW);
    err |= comedi_check_trigger_src(&cmd.scan_end_src, TRIG_COUNT);
    err |= comedi_check_trigger_src(&cmd.stop_src, TRIG_COUNT | TRIG_NONE);
    if (err)
    return 1;
// Step 2a : make sure trigger sources are unique
    err |= comedi_check_trigger_is_unique(cmd.scan_begin_src);
    err |= comedi_check_trigger_is_unique(cmd.stop_src);
// Step 2b : and mutually compatible
    if (err)
    return 2;
// Step 3: check if arguments are trivially valid
    err |= comedi_check_trigger_arg_is(&cmd.start_arg, 0);

//
// Comedi limit due to unsigned int cmd.  Driver limit =
// 2^16 (16bit * counter) * 1000000ns (1kHz onboard clock) = 65.536s
//

    switch (cmd.scan_begin_src) {
    case TRIG_TIMER:
    err |= comedi_check_trigger_arg_min(&cmd.scan_begin_arg,
    MAX_SPEED_AO);
    err |= comedi_check_trigger_arg_max(&cmd.scan_begin_arg,
    MIN_SPEED_AO);
    break;
    case TRIG_EXT:
//
// External trigger - for PCI230+ hardware version 2 onwards.
//
// Trigger number must be 0.
    if (cmd.scan_begin_arg & ~CR_FLAGS_MASK) {
    cmd.scan_begin_arg = COMBINE(cmd.scan_begin_arg, 0,
    ~CR_FLAGS_MASK);
    err |= -EINVAL;
    }
//
// The only flags allowed are CR_EDGE and CR_INVERT.
// The CR_EDGE flag is ignored.
//
    if (cmd.scan_begin_arg & CR_FLAGS_MASK &
    ~(CR_EDGE | CR_INVERT)) {
    cmd.scan_begin_arg =
    COMBINE(cmd.scan_begin_arg, 0,
    CR_FLAGS_MASK & ~(CR_EDGE | CR_INVERT));
    err |= -EINVAL;
    }
    break;
    default:
    err |= comedi_check_trigger_arg_is(&cmd.scan_begin_arg, 0);
    break;
    }
    err |= comedi_check_trigger_arg_is(&cmd.scan_end_arg,
    cmd.chanlist_len);
    if (cmd.stop_src == TRIG_COUNT)
    err |= comedi_check_trigger_arg_min(&cmd.stop_arg, 1);
    else	/* TRIG_NONE */
    err |= comedi_check_trigger_arg_is(&cmd.stop_arg, 0);
    if (err)
    return 3;
// Step 4: fix up any arguments
    if (cmd.scan_begin_src == TRIG_TIMER) {
    tmp = cmd.scan_begin_arg;
    pci230_ns_to_single_timer(&cmd.scan_begin_arg, cmd.flags);
    if (tmp != cmd.scan_begin_arg)
    err++;
    }
    if (err)
    return 4;
// Step 5: check channel list if it exists
    if (cmd.chanlist && cmd.chanlist_len > 0)
    err |= pci230_ao_check_chanlist(dev, s, cmd);
    if (err)
    return 5;
    return 0;
    }
    static void pci230_ao_stop(struct comedi_device *dev,
    struct comedi_subdevice *s)
    {
    struct pci230_private *devpriv = dev.private;
    unsigned long irqflags;
    unsigned char intsrc;
    bool started;
    struct comedi_cmd *cmd;
    spin_lock_irqsave(&devpriv.ao_stop_spinlock, irqflags);
    started = devpriv.ao_cmd_started;
    devpriv.ao_cmd_started = false;
    spin_unlock_irqrestore(&devpriv.ao_stop_spinlock, irqflags);
    if (!started)
    return;
    cmd = &s.async.cmd;
    if (cmd.scan_begin_src == TRIG_TIMER) {
// Stop scan rate generator.
    pci230_cancel_ct(dev, 1);
    }
// Determine interrupt source.
    if (devpriv.hwver < 2) {
// Not using DAC FIFO.  Using CT1 interrupt.
    intsrc = PCI230_INT_ZCLK_CT1;
    } else {
// Using DAC FIFO interrupt.
    intsrc = PCI230P2_INT_DAC;
    }
//
// Disable interrupt and wait for interrupt routine to finish running
// unless we are called from the interrupt routine.
//
    spin_lock_irqsave(&devpriv.isr_spinlock, irqflags);
    devpriv.ier &= ~intsrc;
    while (devpriv.intr_running && devpriv.intr_cpuid != THISCPU) {
    spin_unlock_irqrestore(&devpriv.isr_spinlock, irqflags);
    spin_lock_irqsave(&devpriv.isr_spinlock, irqflags);
    }
    outb(devpriv.ier, dev.iobase + PCI230_INT_SCE);
    spin_unlock_irqrestore(&devpriv.isr_spinlock, irqflags);
    if (devpriv.hwver >= 2) {
//
// Using DAC FIFO.  Reset FIFO, clear underrun error,
// disable FIFO.
//
    devpriv.daccon &= PCI230_DAC_OR_MASK;
    outw(devpriv.daccon | PCI230P2_DAC_FIFO_RESET |
    PCI230P2_DAC_FIFO_UNDERRUN_CLEAR,
    devpriv.daqio + PCI230_DACCON);
    }
// Release resources.
    pci230_release_all_resources(dev, OWNER_AOCMD);
    }
    static void pci230_handle_ao_nofifo(struct comedi_device *dev,
    struct comedi_subdevice *s)
    {
    struct comedi_async *async = s.async;
    struct comedi_cmd *cmd = &async.cmd;
    unsigned short data;
    int i;
    if (cmd.stop_src == TRIG_COUNT && async.scans_done >= cmd.stop_arg)
    return;
    for (i = 0; i < cmd.chanlist_len; i++) {
    let mut chan: c_uint = CR_CHAN(cmd.chanlist[i]);
    if (!comedi_buf_read_samples(s, &data, 1)) {
    async.events |= COMEDI_CB_OVERFLOW;
    return;
    }
    pci230_ao_write_nofifo(dev, data, chan);
    s.readback[chan] = data;
    }
    if (cmd.stop_src == TRIG_COUNT && async.scans_done >= cmd.stop_arg)
    async.events |= COMEDI_CB_EOA;
    }
//
// Loads DAC FIFO (if using it) from buffer.
// Returns false if AO finished due to completion or error, true if still going.
//
    static bool pci230_handle_ao_fifo(struct comedi_device *dev,
    struct comedi_subdevice *s)
    {
    struct pci230_private *devpriv = dev.private;
    struct comedi_async *async = s.async;
    struct comedi_cmd *cmd = &async.cmd;
    let mut num_scans: c_uint = comedi_nscans_left(s, 0);
    unsigned int room;
    unsigned short dacstat;
    unsigned int i, n;
    let mut events: c_uint = 0;
// Get DAC FIFO status.
    dacstat = inw(devpriv.daqio + PCI230_DACCON);
    if (cmd.stop_src == TRIG_COUNT && num_scans == 0)
    events |= COMEDI_CB_EOA;
    if (events == 0) {
// Check for FIFO underrun.
    if (dacstat & PCI230P2_DAC_FIFO_UNDERRUN_LATCHED) {
    dev_err(dev.class_dev, "AO FIFO underrun\n");
    events |= COMEDI_CB_OVERFLOW | COMEDI_CB_ERROR;
    }
//
// Check for buffer underrun if FIFO less than half full
// (otherwise there will be loads of "DAC FIFO not half full"
// interrupts).
//
    if (num_scans == 0 &&
    (dacstat & PCI230P2_DAC_FIFO_HALF) == 0) {
    dev_err(dev.class_dev, "AO buffer underrun\n");
    events |= COMEDI_CB_OVERFLOW | COMEDI_CB_ERROR;
    }
    }
    if (events == 0) {
// Determine how much room is in the FIFO (in samples).
    if (dacstat & PCI230P2_DAC_FIFO_FULL)
    room = PCI230P2_DAC_FIFOROOM_FULL;
#[no_mangle]
pub unsafe extern "C" fn if(PCI230P2_DAC_FIFO_HALF: dacstat &) -> else {
    else if (dacstat & PCI230P2_DAC_FIFO_HALF)
    room = PCI230P2_DAC_FIFOROOM_HALFTOFULL;
#[no_mangle]
pub unsafe extern "C" fn if(PCI230P2_DAC_FIFO_EMPTY: dacstat &) -> else {
    else if (dacstat & PCI230P2_DAC_FIFO_EMPTY)
    room = PCI230P2_DAC_FIFOROOM_EMPTY;
    else
    room = PCI230P2_DAC_FIFOROOM_ONETOHALF;
// Convert room to number of scans that can be added.
    room /= cmd.chanlist_len;
// Determine number of scans to process.
    if (num_scans > room)
    num_scans = room;
// Process scans.
    for (n = 0; n < num_scans; n++) {
    for (i = 0; i < cmd.chanlist_len; i++) {
    let mut chan: c_uint = CR_CHAN(cmd.chanlist[i]);
    unsigned short datum;
    comedi_buf_read_samples(s, &datum, 1);
    pci230_ao_write_fifo(dev, datum, chan);
    s.readback[chan] = datum;
    }
    }
    if (cmd.stop_src == TRIG_COUNT &&
    async.scans_done >= cmd.stop_arg) {
//
// All data for the command has been written
// to FIFO.  Set FIFO interrupt trigger level
// to 'empty'.
//
    devpriv.daccon &= ~PCI230P2_DAC_INT_FIFO_MASK;
    devpriv.daccon |= PCI230P2_DAC_INT_FIFO_EMPTY;
    outw(devpriv.daccon, devpriv.daqio + PCI230_DACCON);
    }
// Check if FIFO underrun occurred while writing to FIFO.
    dacstat = inw(devpriv.daqio + PCI230_DACCON);
    if (dacstat & PCI230P2_DAC_FIFO_UNDERRUN_LATCHED) {
    dev_err(dev.class_dev, "AO FIFO underrun\n");
    events |= COMEDI_CB_OVERFLOW | COMEDI_CB_ERROR;
    }
    }
    async.events |= events;
    return !(async.events & COMEDI_CB_CANCEL_MASK);
    }
    static int pci230_ao_inttrig_scan_begin(struct comedi_device *dev,
    struct comedi_subdevice *s,
    unsigned int trig_num)
    {
    struct pci230_private *devpriv = dev.private;
    unsigned long irqflags;
    if (trig_num)
    return -EINVAL;
    spin_lock_irqsave(&devpriv.ao_stop_spinlock, irqflags);
    if (!devpriv.ao_cmd_started) {
    spin_unlock_irqrestore(&devpriv.ao_stop_spinlock, irqflags);
    return 1;
    }
// Perform scan.
    if (devpriv.hwver < 2) {
// Not using DAC FIFO.
    spin_unlock_irqrestore(&devpriv.ao_stop_spinlock, irqflags);
    pci230_handle_ao_nofifo(dev, s);
    comedi_handle_events(dev, s);
    } else {
// Using DAC FIFO.
// Read DACSWTRIG register to trigger conversion.
    inw(devpriv.daqio + PCI230P2_DACSWTRIG);
    spin_unlock_irqrestore(&devpriv.ao_stop_spinlock, irqflags);
    }
// Delay.  Should driver be responsible for this?
// XXX TODO: See if DAC busy bit can be used.
    udelay(8);
    return 1;
    }
    static void pci230_ao_start(struct comedi_device *dev,
    struct comedi_subdevice *s)
    {
    struct pci230_private *devpriv = dev.private;
    struct comedi_async *async = s.async;
    struct comedi_cmd *cmd = &async.cmd;
    unsigned long irqflags;
    devpriv.ao_cmd_started = true;
    if (devpriv.hwver >= 2) {
// Using DAC FIFO.
    unsigned short scantrig;
    bool run;
// Preload FIFO data.
    run = pci230_handle_ao_fifo(dev, s);
    comedi_handle_events(dev, s);
    if (!run) {
// Stopped.
    return;
    }
// Set scan trigger source.
    switch (cmd.scan_begin_src) {
    case TRIG_TIMER:
    scantrig = PCI230P2_DAC_TRIG_Z2CT1;
    break;
    case TRIG_EXT:
// Trigger on EXTTRIG/EXTCONVCLK pin.
    if ((cmd.scan_begin_arg & CR_INVERT) == 0) {
// +ve edge
    scantrig = PCI230P2_DAC_TRIG_EXTP;
    } else {
// -ve edge
    scantrig = PCI230P2_DAC_TRIG_EXTN;
    }
    break;
    case TRIG_INT:
    scantrig = PCI230P2_DAC_TRIG_SW;
    break;
    default:
// Shouldn't get here.
    scantrig = PCI230P2_DAC_TRIG_NONE;
    break;
    }
    devpriv.daccon =
    (devpriv.daccon & ~PCI230P2_DAC_TRIG_MASK) | scantrig;
    outw(devpriv.daccon, devpriv.daqio + PCI230_DACCON);
    }
    switch (cmd.scan_begin_src) {
    case TRIG_TIMER:
    if (devpriv.hwver < 2) {
// Not using DAC FIFO.
// Enable CT1 timer interrupt.
    spin_lock_irqsave(&devpriv.isr_spinlock, irqflags);
    devpriv.ier |= PCI230_INT_ZCLK_CT1;
    outb(devpriv.ier, dev.iobase + PCI230_INT_SCE);
    spin_unlock_irqrestore(&devpriv.isr_spinlock,
    irqflags);
    }
// Set CT1 gate high to start counting.
    outb(pci230_gat_config(1, GAT_VCC),
    dev.iobase + PCI230_ZGAT_SCE);
    break;
    case TRIG_INT:
    async.inttrig = pci230_ao_inttrig_scan_begin;
    break;
    }
    if (devpriv.hwver >= 2) {
// Using DAC FIFO.  Enable DAC FIFO interrupt.
    spin_lock_irqsave(&devpriv.isr_spinlock, irqflags);
    devpriv.ier |= PCI230P2_INT_DAC;
    outb(devpriv.ier, dev.iobase + PCI230_INT_SCE);
    spin_unlock_irqrestore(&devpriv.isr_spinlock, irqflags);
    }
    }
    static int pci230_ao_inttrig_start(struct comedi_device *dev,
    struct comedi_subdevice *s,
    unsigned int trig_num)
    {
    struct comedi_cmd *cmd = &s.async.cmd;
    if (trig_num != cmd.start_src)
    return -EINVAL;
    s.async.inttrig = core::ptr::null_mut();
    pci230_ao_start(dev, s);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn pci230_ao_cmd(dev: *mut comedi_device, s: *mut comedi_subdevice) -> c_int {
    static int pci230_ao_cmd(struct comedi_device *dev, struct comedi_subdevice *s)
    {
    struct pci230_private *devpriv = dev.private;
    unsigned short daccon;
    unsigned int range;
// Get the command.
    struct comedi_cmd *cmd = &s.async.cmd;
    if (cmd.scan_begin_src == TRIG_TIMER) {
// Claim Z2-CT1.
    if (!pci230_claim_shared(dev, RES_Z2CT1, OWNER_AOCMD))
    return -EBUSY;
    }
//
// Set range - see analogue output range table; 0 => unipolar 10V,
// 1 => bipolar +/-10V range scale
//
    range = CR_RANGE(cmd.chanlist[0]);
    devpriv.ao_bipolar = comedi_range_is_bipolar(s, range);
    daccon = devpriv.ao_bipolar ? PCI230_DAC_OR_BIP : PCI230_DAC_OR_UNI;
// Use DAC FIFO for hardware version 2 onwards.
    if (devpriv.hwver >= 2) {
    unsigned short dacen;
    unsigned int i;
    dacen = 0;
    for (i = 0; i < cmd.chanlist_len; i++)
    dacen |= 1 << CR_CHAN(cmd.chanlist[i]);
// Set channel scan list.
    outw(dacen, devpriv.daqio + PCI230P2_DACEN);
//
// Enable DAC FIFO.
// Set DAC scan source to 'none'.
// Set DAC FIFO interrupt trigger level to 'not half full'.
// Reset DAC FIFO and clear underrun.
//
// N.B. DAC FIFO interrupts are currently disabled.
//
    daccon |= PCI230P2_DAC_FIFO_EN | PCI230P2_DAC_FIFO_RESET |
    PCI230P2_DAC_FIFO_UNDERRUN_CLEAR |
    PCI230P2_DAC_TRIG_NONE | PCI230P2_DAC_INT_FIFO_NHALF;
    }
// Set DACCON.
    outw(daccon, devpriv.daqio + PCI230_DACCON);
// Preserve most of DACCON apart from write-only, transient bits.
    devpriv.daccon = daccon & ~(PCI230P2_DAC_FIFO_RESET |
    PCI230P2_DAC_FIFO_UNDERRUN_CLEAR);
    if (cmd.scan_begin_src == TRIG_TIMER) {
//
// Set the counter timer 1 to the specified scan frequency.
// cmd->scan_begin_arg is sampling period in ns.
// Gate it off for now.
//
    outb(pci230_gat_config(1, GAT_GND),
    dev.iobase + PCI230_ZGAT_SCE);
    pci230_ct_setup_ns_mode(dev, 1, I8254_MODE3,
    cmd.scan_begin_arg,
    cmd.flags);
    }
// N.B. cmd->start_src == TRIG_INT
    s.async.inttrig = pci230_ao_inttrig_start;
    return 0;
    }
    static int pci230_ao_cancel(struct comedi_device *dev,
    struct comedi_subdevice *s)
    {
    pci230_ao_stop(dev, s);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pci230_ai_check_scan_period(cmd: *mut comedi_cmd) -> c_int {
    static int pci230_ai_check_scan_period(struct comedi_cmd *cmd)
    {
    unsigned int min_scan_period, chanlist_len;
    let mut err: c_int = 0;
    chanlist_len = cmd.chanlist_len;
    if (cmd.chanlist_len == 0)
    chanlist_len = 1;
    min_scan_period = chanlist_len * cmd.convert_arg;
    if (min_scan_period < chanlist_len ||
    min_scan_period < cmd.convert_arg) {
// Arithmetic overflow.
    min_scan_period = UINT_MAX;
    err++;
    }
    if (cmd.scan_begin_arg < min_scan_period) {
    cmd.scan_begin_arg = min_scan_period;
    err++;
    }
    return !err;
    }
    static int pci230_ai_check_chanlist(struct comedi_device *dev,
    struct comedi_subdevice *s,
    struct comedi_cmd *cmd)
    {
    struct pci230_private *devpriv = dev.private;
    let mut max_diff_chan: c_uint = (s.n_chan / 2) - 1;
    let mut prev_chan: c_uint = 0;
    let mut prev_range: c_uint = 0;
    let mut prev_aref: c_uint = 0;
    let mut prev_bipolar: bool = false;
    let mut subseq_len: c_uint = 0;
    int i;
    for (i = 0; i < cmd.chanlist_len; i++) {
    let mut chanspec: c_uint = cmd.chanlist[i];
    let mut chan: c_uint = CR_CHAN(chanspec);
    let mut range: c_uint = CR_RANGE(chanspec);
    let mut aref: c_uint = CR_AREF(chanspec);
    let mut bipolar: bool = comedi_range_is_bipolar(s, range);
    if (aref == AREF_DIFF && chan >= max_diff_chan) {
    dev_dbg(dev.class_dev,
    "%s: differential channel number out of range 0 to %u\n",
    __func__, max_diff_chan);
    return -EINVAL;
    }
    if (i > 0) {
//
// Channel numbers must strictly increase or
// subsequence must repeat exactly.
//
    if (chan <= prev_chan && subseq_len == 0)
    subseq_len = i;
    if (subseq_len > 0 &&
    cmd.chanlist[i % subseq_len] != chanspec) {
    dev_dbg(dev.class_dev,
    "%s: channel numbers must increase or sequence must repeat exactly\n",
    __func__);
    return -EINVAL;
    }
    if (aref != prev_aref) {
    dev_dbg(dev.class_dev,
    "%s: channel sequence analogue references must be all the same (single-ended or differential)\n",
    __func__);
    return -EINVAL;
    }
    if (bipolar != prev_bipolar) {
    dev_dbg(dev.class_dev,
    "%s: channel sequence ranges must be all bipolar or all unipolar\n",
    __func__);
    return -EINVAL;
    }
    if (aref != AREF_DIFF && range != prev_range &&
    ((chan ^ prev_chan) & ~1) == 0) {
    dev_dbg(dev.class_dev,
    "%s: single-ended channel pairs must have the same range\n",
    __func__);
    return -EINVAL;
    }
    }
    prev_chan = chan;
    prev_range = range;
    prev_aref = aref;
    prev_bipolar = bipolar;
    }
    if (subseq_len == 0)
    subseq_len = cmd.chanlist_len;
    if (cmd.chanlist_len % subseq_len) {
    dev_dbg(dev.class_dev,
    "%s: sequence must repeat exactly\n", __func__);
    return -EINVAL;
    }
//
// Buggy PCI230+ or PCI260+ requires channel 0 to be (first) in the
// sequence if the sequence contains more than one channel. Hardware
// versions 1 and 2 have the bug. There is no hardware version 3.
//
// Actually, there are two firmwares that report themselves as
// hardware version 1 (the boards have different ADC chips with
// slightly different timing requirements, which was supposed to
// be invisible to software). The first one doesn't seem to have
// the bug, but the second one does, and we can't tell them apart!
//
    if (devpriv.hwver > 0 && devpriv.hwver < 4) {
    if (subseq_len > 1 && CR_CHAN(cmd.chanlist[0])) {
    dev_info(dev.class_dev,
    "amplc_pci230: ai_cmdtest: Buggy PCI230+/260+ h/w version %u requires first channel of multi-channel sequence to be 0 (corrected in h/w version 4)\n",
    devpriv.hwver);
    return -EINVAL;
    }
    }
    return 0;
    }
    static int pci230_ai_cmdtest(struct comedi_device *dev,
    struct comedi_subdevice *s, struct comedi_cmd *cmd)
    {
    const struct pci230_board *board = dev.board_ptr;
    struct pci230_private *devpriv = dev.private;
    let mut err: c_int = 0;
    unsigned int tmp;
// Step 1 : check if triggers are trivially valid
    err |= comedi_check_trigger_src(&cmd.start_src, TRIG_NOW | TRIG_INT);
    tmp = TRIG_FOLLOW | TRIG_TIMER | TRIG_INT;
    if (board.have_dio || board.min_hwver > 0) {
//
// Unfortunately, we cannot trigger a scan off an external
// source on the PCI260 board, since it uses the PPIC0 (DIO)
// input, which isn't present on the PCI260.  For PCI260+
// we can use the EXTTRIG/EXTCONVCLK input on pin 17 instead.
//
    tmp |= TRIG_EXT;
    }
    err |= comedi_check_trigger_src(&cmd.scan_begin_src, tmp);
    err |= comedi_check_trigger_src(&cmd.convert_src,
    TRIG_TIMER | TRIG_INT | TRIG_EXT);
    err |= comedi_check_trigger_src(&cmd.scan_end_src, TRIG_COUNT);
    err |= comedi_check_trigger_src(&cmd.stop_src, TRIG_COUNT | TRIG_NONE);
    if (err)
    return 1;
// Step 2a : make sure trigger sources are unique
    err |= comedi_check_trigger_is_unique(cmd.start_src);
    err |= comedi_check_trigger_is_unique(cmd.scan_begin_src);
    err |= comedi_check_trigger_is_unique(cmd.convert_src);
    err |= comedi_check_trigger_is_unique(cmd.stop_src);
// Step 2b : and mutually compatible
//
// If scan_begin_src is not TRIG_FOLLOW, then a monostable will be
// set up to generate a fixed number of timed conversion pulses.
//
    if (cmd.scan_begin_src != TRIG_FOLLOW &&
    cmd.convert_src != TRIG_TIMER)
    err |= -EINVAL;
    if (err)
    return 2;
// Step 3: check if arguments are trivially valid
    err |= comedi_check_trigger_arg_is(&cmd.start_arg, 0);

//
// Comedi limit due to unsigned int cmd.  Driver limit =
// 2^16 (16bit * counter) * 1000000ns (1kHz onboard clock) = 65.536s
//

    if (cmd.convert_src == TRIG_TIMER) {
    unsigned int max_speed_ai;
    if (devpriv.hwver == 0) {
//
// PCI230 or PCI260.  Max speed depends whether
// single-ended or pseudo-differential.
//
    if (cmd.chanlist && cmd.chanlist_len > 0) {
// Peek analogue reference of first channel.
    if (CR_AREF(cmd.chanlist[0]) == AREF_DIFF)
    max_speed_ai = MAX_SPEED_AI_DIFF;
    else
    max_speed_ai = MAX_SPEED_AI_SE;
    } else {
// No channel list.  Assume single-ended.
    max_speed_ai = MAX_SPEED_AI_SE;
    }
    } else {
// PCI230+ or PCI260+.
    max_speed_ai = MAX_SPEED_AI_PLUS;
    }
    err |= comedi_check_trigger_arg_min(&cmd.convert_arg,
    max_speed_ai);
    err |= comedi_check_trigger_arg_max(&cmd.convert_arg,
    MIN_SPEED_AI);
    } else if (cmd.convert_src == TRIG_EXT) {
//
// external trigger
//
// convert_arg == (CR_EDGE | 0)
// => trigger on +ve edge.
// convert_arg == (CR_EDGE | CR_INVERT | 0)
// => trigger on -ve edge.
//
    if (cmd.convert_arg & CR_FLAGS_MASK) {
// Trigger number must be 0.
    if (cmd.convert_arg & ~CR_FLAGS_MASK) {
    cmd.convert_arg = COMBINE(cmd.convert_arg, 0,
    ~CR_FLAGS_MASK);
    err |= -EINVAL;
    }
//
// The only flags allowed are CR_INVERT and CR_EDGE.
// CR_EDGE is required.
//
    if ((cmd.convert_arg & CR_FLAGS_MASK & ~CR_INVERT) !=
    CR_EDGE) {
// Set CR_EDGE, preserve CR_INVERT.
    cmd.convert_arg =
    COMBINE(cmd.start_arg, CR_EDGE | 0,
    CR_FLAGS_MASK & ~CR_INVERT);
    err |= -EINVAL;
    }
    } else {
//
// Backwards compatibility with previous versions:
// convert_arg == 0 => trigger on -ve edge.
// convert_arg == 1 => trigger on +ve edge.
//
    err |= comedi_check_trigger_arg_max(&cmd.convert_arg,
    1);
    }
    } else {
    err |= comedi_check_trigger_arg_is(&cmd.convert_arg, 0);
    }
    err |= comedi_check_trigger_arg_is(&cmd.scan_end_arg,
    cmd.chanlist_len);
    if (cmd.stop_src == TRIG_COUNT)
    err |= comedi_check_trigger_arg_min(&cmd.stop_arg, 1);
    else	/* TRIG_NONE */
    err |= comedi_check_trigger_arg_is(&cmd.stop_arg, 0);
    if (cmd.scan_begin_src == TRIG_EXT) {
//
// external "trigger" to begin each scan:
// scan_begin_arg==0 => use PPC0 input -> gate of CT0 -> gate
// of CT2 (sample convert trigger is CT2)
//
    if (cmd.scan_begin_arg & ~CR_FLAGS_MASK) {
    cmd.scan_begin_arg = COMBINE(cmd.scan_begin_arg, 0,
    ~CR_FLAGS_MASK);
    err |= -EINVAL;
    }
// The only flag allowed is CR_EDGE, which is ignored.
    if (cmd.scan_begin_arg & CR_FLAGS_MASK & ~CR_EDGE) {
    cmd.scan_begin_arg = COMBINE(cmd.scan_begin_arg, 0,
    CR_FLAGS_MASK & ~CR_EDGE);
    err |= -EINVAL;
    }
    } else if (cmd.scan_begin_src == TRIG_TIMER) {
// N.B. cmd->convert_arg is also TRIG_TIMER
    if (!pci230_ai_check_scan_period(cmd))
    err |= -EINVAL;
    } else {
    err |= comedi_check_trigger_arg_is(&cmd.scan_begin_arg, 0);
    }
    if (err)
    return 3;
// Step 4: fix up any arguments
    if (cmd.convert_src == TRIG_TIMER) {
    tmp = cmd.convert_arg;
    pci230_ns_to_single_timer(&cmd.convert_arg, cmd.flags);
    if (tmp != cmd.convert_arg)
    err++;
    }
    if (cmd.scan_begin_src == TRIG_TIMER) {
// N.B. cmd->convert_arg is also TRIG_TIMER
    tmp = cmd.scan_begin_arg;
    pci230_ns_to_single_timer(&cmd.scan_begin_arg, cmd.flags);
    if (!pci230_ai_check_scan_period(cmd)) {
// Was below minimum required.  Round up.
    pci230_ns_to_single_timer(&cmd.scan_begin_arg,
    CMDF_ROUND_UP);
    pci230_ai_check_scan_period(cmd);
    }
    if (tmp != cmd.scan_begin_arg)
    err++;
    }
    if (err)
    return 4;
// Step 5: check channel list if it exists
    if (cmd.chanlist && cmd.chanlist_len > 0)
    err |= pci230_ai_check_chanlist(dev, s, cmd);
    if (err)
    return 5;
    return 0;
    }
    static void pci230_ai_update_fifo_trigger_level(struct comedi_device *dev,
    struct comedi_subdevice *s)
    {
    struct pci230_private *devpriv = dev.private;
    struct comedi_cmd *cmd = &s.async.cmd;
    unsigned int wake;
    unsigned short triglev;
    unsigned short adccon;
    if (cmd.flags & CMDF_WAKE_EOS)
    wake = cmd.scan_end_arg - s.async.cur_chan;
    else
    wake = comedi_nsamples_left(s, PCI230_ADC_FIFOLEVEL_HALFFULL);
    if (wake >= PCI230_ADC_FIFOLEVEL_HALFFULL) {
    triglev = PCI230_ADC_INT_FIFO_HALF;
    } else if (wake > 1 && devpriv.hwver > 0) {
// PCI230+/260+ programmable FIFO interrupt level.
    if (devpriv.adcfifothresh != wake) {
    devpriv.adcfifothresh = wake;
    outw(wake, devpriv.daqio + PCI230P_ADCFFTH);
    }
    triglev = PCI230P_ADC_INT_FIFO_THRESH;
    } else {
    triglev = PCI230_ADC_INT_FIFO_NEMPTY;
    }
    adccon = (devpriv.adccon & ~PCI230_ADC_INT_FIFO_MASK) | triglev;
    if (adccon != devpriv.adccon) {
    devpriv.adccon = adccon;
    outw(adccon, devpriv.daqio + PCI230_ADCCON);
    }
    }
    static int pci230_ai_inttrig_convert(struct comedi_device *dev,
    struct comedi_subdevice *s,
    unsigned int trig_num)
    {
    struct pci230_private *devpriv = dev.private;
    unsigned long irqflags;
    unsigned int delayus;
    if (trig_num)
    return -EINVAL;
    spin_lock_irqsave(&devpriv.ai_stop_spinlock, irqflags);
    if (!devpriv.ai_cmd_started) {
    spin_unlock_irqrestore(&devpriv.ai_stop_spinlock, irqflags);
    return 1;
    }
//
// Trigger conversion by toggling Z2-CT2 output.
// Finish with output high.
//
    comedi_8254_set_mode(dev.pacer, 2, I8254_MODE0);
    comedi_8254_set_mode(dev.pacer, 2, I8254_MODE1);
//
// Delay.  Should driver be responsible for this?  An
// alternative would be to wait until conversion is complete,
// but we can't tell when it's complete because the ADC busy
// bit has a different meaning when FIFO enabled (and when
// FIFO not enabled, it only works for software triggers).
//
    if ((devpriv.adccon & PCI230_ADC_IM_MASK) == PCI230_ADC_IM_DIF &&
    devpriv.hwver == 0) {
// PCI230/260 in differential mode
    delayus = 8;
    } else {
// single-ended or PCI230+/260+
    delayus = 4;
    }
    spin_unlock_irqrestore(&devpriv.ai_stop_spinlock, irqflags);
    udelay(delayus);
    return 1;
    }
    static int pci230_ai_inttrig_scan_begin(struct comedi_device *dev,
    struct comedi_subdevice *s,
    unsigned int trig_num)
    {
    struct pci230_private *devpriv = dev.private;
    unsigned long irqflags;
    unsigned char zgat;
    if (trig_num)
    return -EINVAL;
    spin_lock_irqsave(&devpriv.ai_stop_spinlock, irqflags);
    if (devpriv.ai_cmd_started) {
// Trigger scan by waggling CT0 gate source.
    zgat = pci230_gat_config(0, GAT_GND);
    outb(zgat, dev.iobase + PCI230_ZGAT_SCE);
    zgat = pci230_gat_config(0, GAT_VCC);
    outb(zgat, dev.iobase + PCI230_ZGAT_SCE);
    }
    spin_unlock_irqrestore(&devpriv.ai_stop_spinlock, irqflags);
    return 1;
    }
    static void pci230_ai_stop(struct comedi_device *dev,
    struct comedi_subdevice *s)
    {
    struct pci230_private *devpriv = dev.private;
    unsigned long irqflags;
    struct comedi_cmd *cmd;
    bool started;
    spin_lock_irqsave(&devpriv.ai_stop_spinlock, irqflags);
    started = devpriv.ai_cmd_started;
    devpriv.ai_cmd_started = false;
    spin_unlock_irqrestore(&devpriv.ai_stop_spinlock, irqflags);
    if (!started)
    return;
    cmd = &s.async.cmd;
    if (cmd.convert_src == TRIG_TIMER) {
// Stop conversion rate generator.
    pci230_cancel_ct(dev, 2);
    }
    if (cmd.scan_begin_src != TRIG_FOLLOW) {
// Stop scan period monostable.
    pci230_cancel_ct(dev, 0);
    }
    spin_lock_irqsave(&devpriv.isr_spinlock, irqflags);
//
// Disable ADC interrupt and wait for interrupt routine to finish
// running unless we are called from the interrupt routine.
//
    devpriv.ier &= ~PCI230_INT_ADC;
    while (devpriv.intr_running && devpriv.intr_cpuid != THISCPU) {
    spin_unlock_irqrestore(&devpriv.isr_spinlock, irqflags);
    spin_lock_irqsave(&devpriv.isr_spinlock, irqflags);
    }
    outb(devpriv.ier, dev.iobase + PCI230_INT_SCE);
    spin_unlock_irqrestore(&devpriv.isr_spinlock, irqflags);
//
// Reset FIFO, disable FIFO and set start conversion source to none.
// Keep se/diff and bip/uni settings.
//
    devpriv.adccon =
    (devpriv.adccon & (PCI230_ADC_IR_MASK | PCI230_ADC_IM_MASK)) |
    PCI230_ADC_TRIG_NONE;
    outw(devpriv.adccon | PCI230_ADC_FIFO_RESET,
    devpriv.daqio + PCI230_ADCCON);
// Release resources.
    pci230_release_all_resources(dev, OWNER_AICMD);
    }
    static void pci230_ai_start(struct comedi_device *dev,
    struct comedi_subdevice *s)
    {
    struct pci230_private *devpriv = dev.private;
    unsigned long irqflags;
    unsigned short conv;
    struct comedi_async *async = s.async;
    struct comedi_cmd *cmd = &async.cmd;
    devpriv.ai_cmd_started = true;
// Enable ADC FIFO trigger level interrupt.
    spin_lock_irqsave(&devpriv.isr_spinlock, irqflags);
    devpriv.ier |= PCI230_INT_ADC;
    outb(devpriv.ier, dev.iobase + PCI230_INT_SCE);
    spin_unlock_irqrestore(&devpriv.isr_spinlock, irqflags);
//
// Update conversion trigger source which is currently set
// to CT2 output, which is currently stuck high.
//
    switch (cmd.convert_src) {
    default:
    conv = PCI230_ADC_TRIG_NONE;
    break;
    case TRIG_TIMER:
// Using CT2 output.
    conv = PCI230_ADC_TRIG_Z2CT2;
    break;
    case TRIG_EXT:
    if (cmd.convert_arg & CR_EDGE) {
    if ((cmd.convert_arg & CR_INVERT) == 0) {
// Trigger on +ve edge.
    conv = PCI230_ADC_TRIG_EXTP;
    } else {
// Trigger on -ve edge.
    conv = PCI230_ADC_TRIG_EXTN;
    }
    } else {
// Backwards compatibility.
    if (cmd.convert_arg) {
// Trigger on +ve edge.
    conv = PCI230_ADC_TRIG_EXTP;
    } else {
// Trigger on -ve edge.
    conv = PCI230_ADC_TRIG_EXTN;
    }
    }
    break;
    case TRIG_INT:
//
// Use CT2 output for software trigger due to problems
// in differential mode on PCI230/260.
//
    conv = PCI230_ADC_TRIG_Z2CT2;
    break;
    }
    devpriv.adccon = (devpriv.adccon & ~PCI230_ADC_TRIG_MASK) | conv;
    outw(devpriv.adccon, devpriv.daqio + PCI230_ADCCON);
    if (cmd.convert_src == TRIG_INT)
    async.inttrig = pci230_ai_inttrig_convert;
//
// Update FIFO interrupt trigger level, which is currently
// set to "full".
//
    pci230_ai_update_fifo_trigger_level(dev, s);
    if (cmd.convert_src == TRIG_TIMER) {
// Update timer gates.
    unsigned char zgat;
    if (cmd.scan_begin_src != TRIG_FOLLOW) {
//
// Conversion timer CT2 needs to be gated by
// inverted output of monostable CT2.
//
    zgat = pci230_gat_config(2, GAT_NOUTNM2);
    } else {
//
// Conversion timer CT2 needs to be gated on
// continuously.
//
    zgat = pci230_gat_config(2, GAT_VCC);
    }
    outb(zgat, dev.iobase + PCI230_ZGAT_SCE);
    if (cmd.scan_begin_src != TRIG_FOLLOW) {
// Set monostable CT0 trigger source.
    switch (cmd.scan_begin_src) {
    default:
    zgat = pci230_gat_config(0, GAT_VCC);
    break;
    case TRIG_EXT:
//
// For CT0 on PCI230, the external trigger
// (gate) signal comes from PPC0, which is
// channel 16 of the DIO subdevice.  The
// application needs to configure this as an
// input in order to use it as an external scan
// trigger.
//
    zgat = pci230_gat_config(0, GAT_EXT);
    break;
    case TRIG_TIMER:
//
// Monostable CT0 triggered by rising edge on
// inverted output of CT1 (falling edge on CT1).
//
    zgat = pci230_gat_config(0, GAT_NOUTNM2);
    break;
    case TRIG_INT:
//
// Monostable CT0 is triggered by inttrig
// function waggling the CT0 gate source.
//
    zgat = pci230_gat_config(0, GAT_VCC);
    break;
    }
    outb(zgat, dev.iobase + PCI230_ZGAT_SCE);
    switch (cmd.scan_begin_src) {
    case TRIG_TIMER:
//
// Scan period timer CT1 needs to be
// gated on to start counting.
//
    zgat = pci230_gat_config(1, GAT_VCC);
    outb(zgat, dev.iobase + PCI230_ZGAT_SCE);
    break;
    case TRIG_INT:
    async.inttrig = pci230_ai_inttrig_scan_begin;
    break;
    }
    }
    } else if (cmd.convert_src != TRIG_INT) {
// No longer need Z2-CT2.
    pci230_release_shared(dev, RES_Z2CT2, OWNER_AICMD);
    }
    }
    static int pci230_ai_inttrig_start(struct comedi_device *dev,
    struct comedi_subdevice *s,
    unsigned int trig_num)
    {
    struct comedi_cmd *cmd = &s.async.cmd;
    if (trig_num != cmd.start_arg)
    return -EINVAL;
    s.async.inttrig = core::ptr::null_mut();
    pci230_ai_start(dev, s);
    return 1;
    }
    static void pci230_handle_ai(struct comedi_device *dev,
    struct comedi_subdevice *s)
    {
    struct pci230_private *devpriv = dev.private;
    struct comedi_async *async = s.async;
    struct comedi_cmd *cmd = &async.cmd;
    unsigned int status_fifo;
    unsigned int i;
    unsigned int nsamples;
    unsigned int fifoamount;
    unsigned short val;
// Determine number of samples to read.
    nsamples = comedi_nsamples_left(s, PCI230_ADC_FIFOLEVEL_HALFFULL);
    if (nsamples == 0)
    return;
    fifoamount = 0;
    for (i = 0; i < nsamples; i++) {
    if (fifoamount == 0) {
// Read FIFO state.
    status_fifo = inw(devpriv.daqio + PCI230_ADCCON);
    if (status_fifo & PCI230_ADC_FIFO_FULL_LATCHED) {
//
// Report error otherwise FIFO overruns will go
// unnoticed by the caller.
//
    dev_err(dev.class_dev, "AI FIFO overrun\n");
    async.events |= COMEDI_CB_ERROR;
    break;
    } else if (status_fifo & PCI230_ADC_FIFO_EMPTY) {
// FIFO empty.
    break;
    } else if (status_fifo & PCI230_ADC_FIFO_HALF) {
// FIFO half full.
    fifoamount = PCI230_ADC_FIFOLEVEL_HALFFULL;
    } else if (devpriv.hwver > 0) {
// Read PCI230+/260+ ADC FIFO level.
    fifoamount = inw(devpriv.daqio +
    PCI230P_ADCFFLEV);
    if (fifoamount == 0)
    break;	/* Shouldn't happen. */
    } else {
// FIFO not empty.
    fifoamount = 1;
    }
    }
    val = pci230_ai_read(dev);
    if (!comedi_buf_write_samples(s, &val, 1))
    break;
    fifoamount--;
    if (cmd.stop_src == TRIG_COUNT &&
    async.scans_done >= cmd.stop_arg) {
    async.events |= COMEDI_CB_EOA;
    break;
    }
    }
// update FIFO interrupt trigger level if still running
    if (!(async.events & COMEDI_CB_CANCEL_MASK))
    pci230_ai_update_fifo_trigger_level(dev, s);
    }
#[no_mangle]
unsafe extern "C" fn pci230_ai_cmd(dev: *mut comedi_device, s: *mut comedi_subdevice) -> c_int {
    static int pci230_ai_cmd(struct comedi_device *dev, struct comedi_subdevice *s)
    {
    struct pci230_private *devpriv = dev.private;
    unsigned int i, chan, range, diff;
    unsigned int res_mask;
    unsigned short adccon, adcen;
    unsigned char zgat;
// Get the command.
    struct comedi_async *async = s.async;
    struct comedi_cmd *cmd = &async.cmd;
//
// Determine which shared resources are needed.
//
    res_mask = 0;
//
// Need Z2-CT2 to supply a conversion trigger source at a high
// logic level, even if not doing timed conversions.
//
    res_mask |= RES_Z2CT2;
    if (cmd.scan_begin_src != TRIG_FOLLOW) {
// Using Z2-CT0 monostable to gate Z2-CT2 conversion timer
    res_mask |= RES_Z2CT0;
    if (cmd.scan_begin_src == TRIG_TIMER) {
// Using Z2-CT1 for scan frequency
    res_mask |= RES_Z2CT1;
    }
    }
// Claim resources.
    if (!pci230_claim_shared(dev, res_mask, OWNER_AICMD))
    return -EBUSY;
//
// Steps:
// - Set channel scan list.
// - Set channel gains.
// - Enable and reset FIFO, specify uni/bip, se/diff, and set
// start conversion source to point to something at a high logic
// level (we use the output of counter/timer 2 for this purpose.
// - PAUSE to allow things to settle down.
// - Reset the FIFO again because it needs resetting twice and there
// may have been a false conversion trigger on some versions of
// PCI230/260 due to the start conversion source being set to a
// high logic level.
// - Enable ADC FIFO level interrupt.
// - Set actual conversion trigger source and FIFO interrupt trigger
// level.
// - If convert_src is TRIG_TIMER, set up the timers.
//
    adccon = PCI230_ADC_FIFO_EN;
    adcen = 0;
    if (CR_AREF(cmd.chanlist[0]) == AREF_DIFF) {
// Differential - all channels must be differential.
    diff = 1;
    adccon |= PCI230_ADC_IM_DIF;
    } else {
// Single ended - all channels must be single-ended.
    diff = 0;
    adccon |= PCI230_ADC_IM_SE;
    }
    range = CR_RANGE(cmd.chanlist[0]);
    devpriv.ai_bipolar = comedi_range_is_bipolar(s, range);
    if (devpriv.ai_bipolar)
    adccon |= PCI230_ADC_IR_BIP;
    else
    adccon |= PCI230_ADC_IR_UNI;
    for (i = 0; i < cmd.chanlist_len; i++) {
    unsigned int gainshift;
    chan = CR_CHAN(cmd.chanlist[i]);
    range = CR_RANGE(cmd.chanlist[i]);
    if (diff) {
    gainshift = 2 * chan;
    if (devpriv.hwver == 0) {
//
// Original PCI230/260 expects both inputs of
// the differential channel to be enabled.
//
    adcen |= 3 << gainshift;
    } else {
//
// PCI230+/260+ expects only one input of the
// differential channel to be enabled.
//
    adcen |= 1 << gainshift;
    }
    } else {
    gainshift = chan & ~1;
    adcen |= 1 << chan;
    }
    devpriv.adcg = (devpriv.adcg & ~(3 << gainshift)) |
    (pci230_ai_gain[range] << gainshift);
    }
// Set channel scan list.
    outw(adcen, devpriv.daqio + PCI230_ADCEN);
// Set channel gains.
    outw(devpriv.adcg, devpriv.daqio + PCI230_ADCG);
//
// Set counter/timer 2 output high for use as the initial start
// conversion source.
//
    comedi_8254_set_mode(dev.pacer, 2, I8254_MODE1);
//
// Temporarily use CT2 output as conversion trigger source and
// temporarily set FIFO interrupt trigger level to 'full'.
//
    adccon |= PCI230_ADC_INT_FIFO_FULL | PCI230_ADC_TRIG_Z2CT2;
//
// Enable and reset FIFO, specify FIFO trigger level full, specify
// uni/bip, se/diff, and temporarily set the start conversion source
// to CT2 output.  Note that CT2 output is currently high, and this
// will produce a false conversion trigger on some versions of the
// PCI230/260, but that will be dealt with later.
//
    devpriv.adccon = adccon;
    outw(adccon | PCI230_ADC_FIFO_RESET, devpriv.daqio + PCI230_ADCCON);
//
// Delay -
// Failure to include this will result in the first few channels'-worth
// of data being corrupt, normally manifesting itself by large negative
// voltages. It seems the board needs time to settle between the first
// FIFO reset (above) and the second FIFO reset (below). Setting the
// channel gains and scan list _before_ the first FIFO reset also
// helps, though only slightly.
//
    usleep_range(25, 100);
// Reset FIFO again.
    outw(adccon | PCI230_ADC_FIFO_RESET, devpriv.daqio + PCI230_ADCCON);
    if (cmd.convert_src == TRIG_TIMER) {
//
// Set up CT2 as conversion timer, but gate it off for now.
// Note, counter/timer output 2 can be monitored on the
// connector: PCI230 pin 21, PCI260 pin 18.
//
    zgat = pci230_gat_config(2, GAT_GND);
    outb(zgat, dev.iobase + PCI230_ZGAT_SCE);
// Set counter/timer 2 to the specified conversion period.
    pci230_ct_setup_ns_mode(dev, 2, I8254_MODE3, cmd.convert_arg,
    cmd.flags);
    if (cmd.scan_begin_src != TRIG_FOLLOW) {
//
// Set up monostable on CT0 output for scan timing.  A
// rising edge on the trigger (gate) input of CT0 will
// trigger the monostable, causing its output to go low
// for the configured period.  The period depends on
// the conversion period and the number of conversions
// in the scan.
//
// Set the trigger high before setting up the
// monostable to stop it triggering.  The trigger
// source will be changed later.
//
    zgat = pci230_gat_config(0, GAT_VCC);
    outb(zgat, dev.iobase + PCI230_ZGAT_SCE);
    pci230_ct_setup_ns_mode(dev, 0, I8254_MODE1,
    ((u64)cmd.convert_arg *
    cmd.scan_end_arg),
    CMDF_ROUND_UP);
    if (cmd.scan_begin_src == TRIG_TIMER) {
//
// Monostable on CT0 will be triggered by
// output of CT1 at configured scan frequency.
//
// Set up CT1 but gate it off for now.
//
    zgat = pci230_gat_config(1, GAT_GND);
    outb(zgat, dev.iobase + PCI230_ZGAT_SCE);
    pci230_ct_setup_ns_mode(dev, 1, I8254_MODE3,
    cmd.scan_begin_arg,
    cmd.flags);
    }
    }
    }
    if (cmd.start_src == TRIG_INT)
    s.async.inttrig = pci230_ai_inttrig_start;
    else	/* TRIG_NOW */
    pci230_ai_start(dev, s);
    return 0;
    }
    static int pci230_ai_cancel(struct comedi_device *dev,
    struct comedi_subdevice *s)
    {
    pci230_ai_stop(dev, s);
    return 0;
    }
// Interrupt handler
#[no_mangle]
unsafe extern "C" fn pci230_interrupt(irq: c_int, d: *mut c_void) -> irqreturn_t {
    static irqreturn_t pci230_interrupt(int irq, void *d)
    {
    unsigned char status_int, valid_status_int, temp_ier;
    struct comedi_device *dev = d;
    struct pci230_private *devpriv = dev.private;
    struct comedi_subdevice *s_ao = dev.write_subdev;
    struct comedi_subdevice *s_ai = dev.read_subdev;
    unsigned long irqflags;
// Read interrupt status/enable register.
    status_int = inb(dev.iobase + PCI230_INT_STAT);
    if (status_int == PCI230_INT_DISABLE)
    return IRQ_NONE;
    spin_lock_irqsave(&devpriv.isr_spinlock, irqflags);
    valid_status_int = devpriv.ier & status_int;
//
// Disable triggered interrupts.
// (Only those interrupts that need re-enabling, are, later in the
// handler).
//
    temp_ier = devpriv.ier & ~status_int;
    outb(temp_ier, dev.iobase + PCI230_INT_SCE);
    devpriv.intr_running = true;
    devpriv.intr_cpuid = THISCPU;
    spin_unlock_irqrestore(&devpriv.isr_spinlock, irqflags);
//
// Check the source of interrupt and handle it.
// The PCI230 can cope with concurrent ADC, DAC, PPI C0 and C3
// interrupts.  However, at present (Comedi-0.7.60) does not allow
// concurrent execution of commands, instructions or a mixture of the
// two.
//
    if (valid_status_int & PCI230_INT_ZCLK_CT1)
    pci230_handle_ao_nofifo(dev, s_ao);
    if (valid_status_int & PCI230P2_INT_DAC)
    pci230_handle_ao_fifo(dev, s_ao);
    if (valid_status_int & PCI230_INT_ADC)
    pci230_handle_ai(dev, s_ai);
// Reenable interrupts.
    spin_lock_irqsave(&devpriv.isr_spinlock, irqflags);
    if (devpriv.ier != temp_ier)
    outb(devpriv.ier, dev.iobase + PCI230_INT_SCE);
    devpriv.intr_running = false;
    spin_unlock_irqrestore(&devpriv.isr_spinlock, irqflags);
    if (s_ao)
    comedi_handle_events(dev, s_ao);
    comedi_handle_events(dev, s_ai);
    return IRQ_HANDLED;
    }
// Check if PCI device matches a specific board.
    static bool pci230_match_pci_board(const struct pci230_board *board,
    struct pci_dev *pci_dev)
    {
// assume pci_dev->device != PCI_DEVICE_ID_INVALID
    if (board.id != pci_dev.device)
    return false;
    if (board.min_hwver == 0)
    return true;
// Looking for a '+' model.  First check length of registers.
    if (pci_resource_len(pci_dev, 3) < 32)
    return false;	/* Not a '+' model. */
//
// TODO: temporarily enable PCI device and read the hardware version
// register.  For now, assume it's okay.
//
    return true;
    }
// Look for board matching PCI device.
    static const struct pci230_board *pci230_find_pci_board(struct pci_dev *pci_dev)
    {
    unsigned int i;
    for (i = 0; i < ARRAY_SIZE(pci230_boards); i++)
    if (pci230_match_pci_board(&pci230_boards[i], pci_dev))
    return &pci230_boards[i];
    return core::ptr::null_mut();
    }
    static int pci230_auto_attach(struct comedi_device *dev,
    unsigned long context_unused)
    {
    struct pci_dev *pci_dev = comedi_to_pci_dev(dev);
    const struct pci230_board *board;
    struct pci230_private *devpriv;
    struct comedi_subdevice *s;
    int rc;
    dev_info(dev.class_dev, "amplc_pci230: attach pci %s\n",
    pci_name(pci_dev));
    devpriv = comedi_alloc_devpriv(dev, sizeof(*devpriv));
    if (!devpriv)
    return -ENOMEM;
    spin_lock_init(&devpriv.isr_spinlock);
    spin_lock_init(&devpriv.res_spinlock);
    spin_lock_init(&devpriv.ai_stop_spinlock);
    spin_lock_init(&devpriv.ao_stop_spinlock);
    board = pci230_find_pci_board(pci_dev);
    if (!board) {
    dev_err(dev.class_dev,
    "amplc_pci230: BUG! cannot determine board type!\n");
    return -EINVAL;
    }
    dev.board_ptr = board;
    dev.board_name = board.name;
    rc = comedi_pci_enable(dev);
    if (rc)
    return rc;
//
// Read base addresses of the PCI230's two I/O regions from PCI
// configuration register.
//
    dev.iobase = pci_resource_start(pci_dev, 2);
    devpriv.daqio = pci_resource_start(pci_dev, 3);
    dev_dbg(dev.class_dev,
    "%s I/O region 1 0x%04lx I/O region 2 0x%04lx\n",
    dev.board_name, dev.iobase, devpriv.daqio);
// Read bits of DACCON register - only the output range.
    devpriv.daccon = inw(devpriv.daqio + PCI230_DACCON) &
    PCI230_DAC_OR_MASK;
//
// Read hardware version register and set extended function register
// if they exist.
//
    if (pci_resource_len(pci_dev, 3) >= 32) {
    let mut extfunc: c_ushort = 0;
    devpriv.hwver = inw(devpriv.daqio + PCI230P_HWVER);
    if (devpriv.hwver < board.min_hwver) {
    dev_err(dev.class_dev,
    "%s - bad hardware version - got %u, need %u\n",
    dev.board_name, devpriv.hwver,
    board.min_hwver);
    return -EIO;
    }
    if (devpriv.hwver > 0) {
    if (!board.have_dio) {
//
// No DIO ports.  Route counters' external gates
// to the EXTTRIG signal (PCI260+ pin 17).
// (Otherwise, they would be routed to DIO
// inputs PC0, PC1 and PC2 which don't exist
// on PCI260[+].)
//
    extfunc |= PCI230P_EXTFUNC_GAT_EXTTRIG;
    }
    if (board.ao_bits && devpriv.hwver >= 2) {
// Enable DAC FIFO functionality.
    extfunc |= PCI230P2_EXTFUNC_DACFIFO;
    }
    }
    outw(extfunc, devpriv.daqio + PCI230P_EXTFUNC);
    if (extfunc & PCI230P2_EXTFUNC_DACFIFO) {
//
// Temporarily enable DAC FIFO, reset it and disable
// FIFO wraparound.
//
    outw(devpriv.daccon | PCI230P2_DAC_FIFO_EN |
    PCI230P2_DAC_FIFO_RESET,
    devpriv.daqio + PCI230_DACCON);
// Clear DAC FIFO channel enable register.
    outw(0, devpriv.daqio + PCI230P2_DACEN);
// Disable DAC FIFO.
    outw(devpriv.daccon, devpriv.daqio + PCI230_DACCON);
    }
    }
// Disable board's interrupts.
    outb(0, dev.iobase + PCI230_INT_SCE);
// Set ADC to a reasonable state.
    devpriv.adcg = 0;
    devpriv.adccon = PCI230_ADC_TRIG_NONE | PCI230_ADC_IM_SE |
    PCI230_ADC_IR_BIP;
    outw(BIT(0), devpriv.daqio + PCI230_ADCEN);
    outw(devpriv.adcg, devpriv.daqio + PCI230_ADCG);
    outw(devpriv.adccon | PCI230_ADC_FIFO_RESET,
    devpriv.daqio + PCI230_ADCCON);
    if (pci_dev.irq) {
    rc = request_irq(pci_dev.irq, pci230_interrupt, IRQF_SHARED,
    dev.board_name, dev);
    if (rc == 0)
    dev.irq = pci_dev.irq;
    }
    dev.pacer = comedi_8254_io_alloc(dev.iobase + PCI230_Z2_CT_BASE,
    0, I8254_IO8, 0);
    if (IS_ERR(dev.pacer))
    return PTR_ERR(dev.pacer);
    rc = comedi_alloc_subdevices(dev, 3);
    if (rc)
    return rc;
    s = &dev.subdevices[0];
// analog input subdevice
    s.type = COMEDI_SUBD_AI;
    s.subdev_flags = SDF_READABLE | SDF_DIFF | SDF_GROUND;
    s.n_chan = 16;
    s.maxdata = (1 << board.ai_bits) - 1;
    s.range_table = &pci230_ai_range;
    s.insn_read = pci230_ai_insn_read;
    s.len_chanlist = 256;	/* but there are restrictions. */
    if (dev.irq) {
    dev.read_subdev = s;
    s.subdev_flags |= SDF_CMD_READ;
    s.do_cmd = pci230_ai_cmd;
    s.do_cmdtest = pci230_ai_cmdtest;
    s.cancel = pci230_ai_cancel;
    }
    s = &dev.subdevices[1];
// analog output subdevice
    if (board.ao_bits) {
    s.type = COMEDI_SUBD_AO;
    s.subdev_flags = SDF_WRITABLE | SDF_GROUND;
    s.n_chan = 2;
    s.maxdata = (1 << board.ao_bits) - 1;
    s.range_table = &pci230_ao_range;
    s.insn_write = pci230_ao_insn_write;
    s.len_chanlist = 2;
    if (dev.irq) {
    dev.write_subdev = s;
    s.subdev_flags |= SDF_CMD_WRITE;
    s.do_cmd = pci230_ao_cmd;
    s.do_cmdtest = pci230_ao_cmdtest;
    s.cancel = pci230_ao_cancel;
    }
    rc = comedi_alloc_subdev_readback(s);
    if (rc)
    return rc;
    } else {
    s.type = COMEDI_SUBD_UNUSED;
    }
    s = &dev.subdevices[2];
// digital i/o subdevice
    if (board.have_dio) {
    rc = subdev_8255_io_init(dev, s, PCI230_PPI_X_BASE);
    if (rc)
    return rc;
    } else {
    s.type = COMEDI_SUBD_UNUSED;
    }
    return 0;
    }
    static struct comedi_driver amplc_pci230_driver = {
    .driver_name	= "amplc_pci230",
    .module		= THIS_MODULE,
    .auto_attach	= pci230_auto_attach,
    .detach		= comedi_pci_detach,
    };
    static int amplc_pci230_pci_probe(struct pci_dev *dev,
    const struct pci_device_id *id)
    {
    return comedi_pci_auto_config(dev, &amplc_pci230_driver,
    id.driver_data);
    }
    static const struct pci_device_id amplc_pci230_pci_table[] = {
    { PCI_VDEVICE(AMPLICON, PCI_DEVICE_ID_PCI230) },
    { PCI_VDEVICE(AMPLICON, PCI_DEVICE_ID_PCI260) },
    { }
    };
    MODULE_DEVICE_TABLE(pci, amplc_pci230_pci_table);
    static struct pci_driver amplc_pci230_pci_driver = {
    .name		= "amplc_pci230",
    .id_table	= amplc_pci230_pci_table,
    .probe		= amplc_pci230_pci_probe,
    .remove		= comedi_pci_auto_unconfig,
    };
    module_comedi_pci_driver(amplc_pci230_driver, amplc_pci230_pci_driver);
    MODULE_AUTHOR("Comedi https://www.comedi.org");
    MODULE_DESCRIPTION("Comedi driver for Amplicon PCI230(+) and PCI260(+)");
    MODULE_LICENSE("GPL");
