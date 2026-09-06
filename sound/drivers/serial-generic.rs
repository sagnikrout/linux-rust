//! Automatically rewritten from C to Rust
//! Source: sound/drivers/serial-generic.c
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
// serial-generic.c
// Copyright (c) by Daniel Kaehn <kaehndan@gmail.com
// Based on serial-u16550.c by Jaroslav Kysela <perex@perex.cz>,
// Isaku Yamahata <yamahata@private.email.ne.jp>,
// George Hansper <ghansper@apana.org.au>,
// Hannu Savolainen
//
// Generic serial MIDI driver using the serdev serial bus API for hardware interaction
//

    MODULE_DESCRIPTION("Generic serial MIDI driver");
    MODULE_LICENSE("GPL");
pub const SERIAL_MODE_INPUT_OPEN: c_int = 1;
pub const SERIAL_MODE_OUTPUT_OPEN: c_int = 2;
pub const SERIAL_MODE_INPUT_TRIGGERED: c_int = 3;
pub const SERIAL_MODE_OUTPUT_TRIGGERED: c_int = 4;
pub const SERIAL_TX_STATE_ACTIVE: c_int = 1;
pub const SERIAL_TX_STATE_WAKEUP: c_int = 2;
pub const INTERNAL_BUF_SIZE: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_serial_generic {
    pub serdev: *mut serdev_device,
    pub card: *mut snd_card,
    pub rmidi: *mut snd_rawmidi,
    pub midi_output: *mut snd_rawmidi_substream,
    pub midi_input: *mut snd_rawmidi_substream,
    pub baudrate: c_uint,
    pub /: *mut *mut unsigned long filemode; / open status of file,
    pub tx_work: work_struct,
    pub tx_state: c_ulong,
    pub tx_buf: [c_char; INTERNAL_BUF_SIZE],
}

#[no_mangle]
unsafe extern "C" fn snd_serial_generic_tx_wakeup(drvdata: *mut snd_serial_generic) {
    static void snd_serial_generic_tx_wakeup(struct snd_serial_generic *drvdata)
    {
    if (test_and_set_bit(SERIAL_TX_STATE_ACTIVE, &drvdata.tx_state))
    set_bit(SERIAL_TX_STATE_WAKEUP, &drvdata.tx_state);
    schedule_work(&drvdata.tx_work);
    }
#[no_mangle]
unsafe extern "C" fn snd_serial_generic_tx_work(work: *mut work_struct) {
    static void snd_serial_generic_tx_work(struct work_struct *work)
    {
    int num_bytes;
    struct snd_serial_generic *drvdata = container_of(work, struct snd_serial_generic,
    tx_work);
    struct snd_rawmidi_substream *substream = drvdata.midi_output;
    clear_bit(SERIAL_TX_STATE_WAKEUP, &drvdata.tx_state);
    while (!snd_rawmidi_transmit_empty(substream)) {
    if (!test_bit(SERIAL_MODE_OUTPUT_OPEN, &drvdata.filemode))
    break;
    num_bytes = snd_rawmidi_transmit_peek(substream, drvdata.tx_buf,
    INTERNAL_BUF_SIZE);
    num_bytes = serdev_device_write_buf(drvdata.serdev, drvdata.tx_buf,
    num_bytes);
    if (!num_bytes)
    break;
    snd_rawmidi_transmit_ack(substream, num_bytes);
    if (!test_bit(SERIAL_TX_STATE_WAKEUP, &drvdata.tx_state))
    break;
    }
    clear_bit(SERIAL_TX_STATE_ACTIVE, &drvdata.tx_state);
    }
#[no_mangle]
unsafe extern "C" fn snd_serial_generic_write_wakeup(serdev: *mut serdev_device) {
    static void snd_serial_generic_write_wakeup(struct serdev_device *serdev)
    {
    struct snd_serial_generic *drvdata = serdev_device_get_drvdata(serdev);
    snd_serial_generic_tx_wakeup(drvdata);
    }
    static size_t snd_serial_generic_receive_buf(struct serdev_device *serdev,
    const u8 *buf, size_t count)
    {
    int ret;
    struct snd_serial_generic *drvdata = serdev_device_get_drvdata(serdev);
    if (!test_bit(SERIAL_MODE_INPUT_OPEN, &drvdata.filemode))
    return 0;
    ret = snd_rawmidi_receive(drvdata.midi_input, buf, count);
    return ret < 0 ? 0 : ret;
    }
    static const struct serdev_device_ops snd_serial_generic_serdev_device_ops = {
    .receive_buf = snd_serial_generic_receive_buf,
    .write_wakeup = snd_serial_generic_write_wakeup
    };
#[no_mangle]
unsafe extern "C" fn snd_serial_generic_ensure_serdev_open(drvdata: *mut snd_serial_generic) -> c_int {
    static int snd_serial_generic_ensure_serdev_open(struct snd_serial_generic *drvdata)
    {
    int err;
    unsigned int actual_baud;
    if (drvdata.filemode)
    return 0;
    dev_dbg(drvdata.card.dev, "Opening serial port for card %s\n",
    drvdata.card.shortname);
    err = serdev_device_open(drvdata.serdev);
    if (err < 0)
    return err;
    actual_baud = serdev_device_set_baudrate(drvdata.serdev,
    drvdata.baudrate);
    if (actual_baud != drvdata.baudrate) {
    dev_warn(drvdata.card.dev, "requested %d baud for card %s but it was actually set to %d\n",
    drvdata.baudrate, drvdata.card.shortname, actual_baud);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_serial_generic_input_open(substream: *mut snd_rawmidi_substream) -> c_int {
    static int snd_serial_generic_input_open(struct snd_rawmidi_substream *substream)
    {
    int err;
    struct snd_serial_generic *drvdata = substream.rmidi.card.private_data;
    dev_dbg(drvdata.card.dev, "Opening input for card %s\n",
    drvdata.card.shortname);
    err = snd_serial_generic_ensure_serdev_open(drvdata);
    if (err < 0)
    return err;
    set_bit(SERIAL_MODE_INPUT_OPEN, &drvdata.filemode);
    drvdata.midi_input = substream;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_serial_generic_input_close(substream: *mut snd_rawmidi_substream) -> c_int {
    static int snd_serial_generic_input_close(struct snd_rawmidi_substream *substream)
    {
    struct snd_serial_generic *drvdata = substream.rmidi.card.private_data;
    dev_dbg(drvdata.card.dev, "Closing input for card %s\n",
    drvdata.card.shortname);
    clear_bit(SERIAL_MODE_INPUT_OPEN, &drvdata.filemode);
    clear_bit(SERIAL_MODE_INPUT_TRIGGERED, &drvdata.filemode);
    drvdata.midi_input = core::ptr::null_mut();
    if (!drvdata.filemode)
    serdev_device_close(drvdata.serdev);
    return 0;
    }
    static void snd_serial_generic_input_trigger(struct snd_rawmidi_substream *substream,
    int up)
    {
    struct snd_serial_generic *drvdata = substream.rmidi.card.private_data;
    if (up)
    set_bit(SERIAL_MODE_INPUT_TRIGGERED, &drvdata.filemode);
    else
    clear_bit(SERIAL_MODE_INPUT_TRIGGERED, &drvdata.filemode);
    }
#[no_mangle]
unsafe extern "C" fn snd_serial_generic_output_open(substream: *mut snd_rawmidi_substream) -> c_int {
    static int snd_serial_generic_output_open(struct snd_rawmidi_substream *substream)
    {
    struct snd_serial_generic *drvdata = substream.rmidi.card.private_data;
    int err;
    dev_dbg(drvdata.card.dev, "Opening output for card %s\n",
    drvdata.card.shortname);
    err = snd_serial_generic_ensure_serdev_open(drvdata);
    if (err < 0)
    return err;
    set_bit(SERIAL_MODE_OUTPUT_OPEN, &drvdata.filemode);
    drvdata.midi_output = substream;
    return 0;
    };
#[no_mangle]
unsafe extern "C" fn snd_serial_generic_output_close(substream: *mut snd_rawmidi_substream) -> c_int {
    static int snd_serial_generic_output_close(struct snd_rawmidi_substream *substream)
    {
    struct snd_serial_generic *drvdata = substream.rmidi.card.private_data;
    dev_dbg(drvdata.card.dev, "Closing output for card %s\n",
    drvdata.card.shortname);
    clear_bit(SERIAL_MODE_OUTPUT_OPEN, &drvdata.filemode);
    clear_bit(SERIAL_MODE_OUTPUT_TRIGGERED, &drvdata.filemode);
    if (!drvdata.filemode)
    serdev_device_close(drvdata.serdev);
    drvdata.midi_output = core::ptr::null_mut();
    return 0;
    };
    static void snd_serial_generic_output_trigger(struct snd_rawmidi_substream *substream,
    int up)
    {
    struct snd_serial_generic *drvdata = substream.rmidi.card.private_data;
    if (up)
    set_bit(SERIAL_MODE_OUTPUT_TRIGGERED, &drvdata.filemode);
    else
    clear_bit(SERIAL_MODE_OUTPUT_TRIGGERED, &drvdata.filemode);
    if (up)
    snd_serial_generic_tx_wakeup(drvdata);
    }
#[no_mangle]
unsafe extern "C" fn snd_serial_generic_output_drain(substream: *mut snd_rawmidi_substream) {
    static void snd_serial_generic_output_drain(struct snd_rawmidi_substream *substream)
    {
    struct snd_serial_generic *drvdata = substream.rmidi.card.private_data;
// Flush any pending characters
    serdev_device_write_flush(drvdata.serdev);
    cancel_work_sync(&drvdata.tx_work);
    }
    static const struct snd_rawmidi_ops snd_serial_generic_output = {
    .open =		snd_serial_generic_output_open,
    .close =	snd_serial_generic_output_close,
    .trigger =	snd_serial_generic_output_trigger,
    .drain =	snd_serial_generic_output_drain,
    };
    static const struct snd_rawmidi_ops snd_serial_generic_input = {
    .open =		snd_serial_generic_input_open,
    .close =	snd_serial_generic_input_close,
    .trigger =	snd_serial_generic_input_trigger,
    };
    static void snd_serial_generic_parse_dt(struct serdev_device *serdev,
    struct snd_serial_generic *drvdata)
    {
    int err;
    err = of_property_read_u32(serdev.dev.of_node, "current-speed",
    &drvdata.baudrate);
    if (err < 0) {
    dev_dbg(drvdata.card.dev,
    "MIDI device reading of current-speed DT param failed with error %d, using default of 38400\n",
    err);
    drvdata.baudrate = 38400;
    }
    }
#[no_mangle]
unsafe extern "C" fn snd_serial_generic_substreams(stream: *mut snd_rawmidi_str, dev_num: c_int) {
    static void snd_serial_generic_substreams(struct snd_rawmidi_str *stream, int dev_num)
    {
    struct snd_rawmidi_substream *substream;
    list_for_each_entry(substream, &stream.substreams, list) {
    sprintf(substream.name, "Serial MIDI %d-%d", dev_num, substream.number);
    }
    }
    static int snd_serial_generic_rmidi(struct snd_serial_generic *drvdata,
    int outs, int ins, struct snd_rawmidi **rmidi)
    {
    struct snd_rawmidi *rrawmidi;
    int err;
    err = snd_rawmidi_new(drvdata.card, drvdata.card.driver, 0,
    outs, ins, &rrawmidi);
    if (err < 0)
    return err;
    snd_rawmidi_set_ops(rrawmidi, SNDRV_RAWMIDI_STREAM_INPUT,
    &snd_serial_generic_input);
    snd_rawmidi_set_ops(rrawmidi, SNDRV_RAWMIDI_STREAM_OUTPUT,
    &snd_serial_generic_output);
    strscpy(rrawmidi.name, drvdata.card.shortname);
    snd_serial_generic_substreams(&rrawmidi.streams[SNDRV_RAWMIDI_STREAM_OUTPUT],
    drvdata.serdev.ctrl.nr);
    snd_serial_generic_substreams(&rrawmidi.streams[SNDRV_RAWMIDI_STREAM_INPUT],
    drvdata.serdev.ctrl.nr);
    rrawmidi.info_flags = SNDRV_RAWMIDI_INFO_OUTPUT |
    SNDRV_RAWMIDI_INFO_INPUT |
    SNDRV_RAWMIDI_INFO_DUPLEX;
    if (rmidi)
// rmidi = rrawmidi;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_serial_generic_probe(serdev: *mut serdev_device) -> c_int {
    static int snd_serial_generic_probe(struct serdev_device *serdev)
    {
    struct snd_card *card;
    struct snd_serial_generic *drvdata;
    int err;
    err  = snd_devm_card_new(&serdev.dev, SNDRV_DEFAULT_IDX1,
    SNDRV_DEFAULT_STR1, THIS_MODULE,
    sizeof(struct snd_serial_generic), &card);
    if (err < 0)
    return err;
    strscpy(card.driver, "SerialMIDI");
    sprintf(card.shortname, "SerialMIDI-%d", serdev.ctrl.nr);
    sprintf(card.longname, "Serial MIDI device at serial%d", serdev.ctrl.nr);
    drvdata = card.private_data;
    drvdata.serdev = serdev;
    drvdata.card = card;
    snd_serial_generic_parse_dt(serdev, drvdata);
    INIT_WORK(&drvdata.tx_work, snd_serial_generic_tx_work);
    err = snd_serial_generic_rmidi(drvdata, 1, 1, &drvdata.rmidi);
    if (err < 0)
    return err;
    serdev_device_set_client_ops(serdev, &snd_serial_generic_serdev_device_ops);
    serdev_device_set_drvdata(drvdata.serdev, drvdata);
    err = snd_card_register(card);
    if (err < 0)
    return err;
    return 0;
    }
    static const struct of_device_id snd_serial_generic_dt_ids[] = {
    { .compatible = "serial-midi" },
    {},
    };
    MODULE_DEVICE_TABLE(of, snd_serial_generic_dt_ids);
    static struct serdev_device_driver snd_serial_generic_driver = {
    .driver	= {
    .name		= "snd-serial-generic",
    .of_match_table	= snd_serial_generic_dt_ids,
    },
    .probe	= snd_serial_generic_probe,
    };
    module_serdev_device_driver(snd_serial_generic_driver);
