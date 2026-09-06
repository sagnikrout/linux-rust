//! Automatically rewritten from C to Rust
//! Source: sound/core/seq/seq_midi.c
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
// Generic MIDI synth driver for ALSA sequencer
// Copyright (c) 1998 by Frank van de Pol <fvdpol@coil.demon.nl>
// Jaroslav Kysela <perex@perex.cz>
//
    Possible options for midisynth module:
    - automatic opening of midi ports on first received event or subscription
    (close will be performed when client leaves)
//

    MODULE_AUTHOR("Frank van de Pol <fvdpol@coil.demon.nl>, Jaroslav Kysela <perex@perex.cz>");
    MODULE_DESCRIPTION("Advanced Linux Sound Architecture sequencer MIDI synth.");
    MODULE_LICENSE("GPL");
    let mut output_buffer_size: static int = PAGE_SIZE;
    module_param(output_buffer_size, int, 0644);
    MODULE_PARM_DESC(output_buffer_size, "Output buffer size in bytes.");
    let mut input_buffer_size: static int = PAGE_SIZE;
    module_param(input_buffer_size, int, 0644);
    MODULE_PARM_DESC(input_buffer_size, "Input buffer size in bytes.");
// data for this midi synth driver
#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_midisynth {
    pub card: *mut snd_card,
    pub rmidi: *mut snd_rawmidi,
    pub device: c_int,
    pub subdevice: c_int,
    pub input_substream: *mut snd_rawmidi_substream __rcu,
    pub /: *mut *mut snd_use_lock_t input_use_lock; / in-flight event_input users,
    pub input_rfile: snd_rawmidi_file,
    pub /: *mut *mut snd_use_lock_t output_use_lock; / in-flight event_input users,
    pub output_substream: *mut snd_rawmidi_substream __rcu,
    pub output_rfile: snd_rawmidi_file,
    pub seq_client: c_int,
    pub seq_port: c_int,
    pub parser: *mut snd_midi_event,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_midisynth_client {
    pub seq_client: c_int,
    pub num_ports: c_int,
    pub ports_per_device: [c_int; SNDRV_RAWMIDI_DEVICES],
    pub ports: [*mut seq_midisynth; SNDRV_RAWMIDI_DEVICES],
}

    static struct seq_midisynth_client *synths[SNDRV_CARDS];
    static DEFINE_MUTEX(register_mutex);
// handle rawmidi input event (MIDI v1.0 stream)
#[no_mangle]
unsafe extern "C" fn snd_midi_input_event(substream: *mut snd_rawmidi_substream) {
    static void snd_midi_input_event(struct snd_rawmidi_substream *substream)
    {
    struct snd_rawmidi_runtime *runtime;
    struct seq_midisynth *msynth;
    struct snd_seq_event ev;
    char buf[16], *pbuf;
    long res;
    if (substream == core::ptr::null_mut())
    return;
    runtime = substream.runtime;
    msynth = runtime.private_data;
    if (msynth == core::ptr::null_mut())
    return;
    scoped_guard(rcu) {
    if (rcu_dereference(msynth.input_substream) != substream)
    return;
    snd_use_lock_use(&msynth.input_use_lock);
    }
    memset(&ev, 0, sizeof(ev));
    while (runtime.avail > 0) {
    res = snd_rawmidi_kernel_read(substream, buf, sizeof(buf));
    if (res <= 0)
    continue;
    if (msynth.parser == core::ptr::null_mut())
    continue;
    pbuf = buf;
    while (res-- > 0) {
    if (!snd_midi_event_encode_byte(msynth.parser,
// pbuf++, &ev))
    continue;
    ev.source.port = msynth.seq_port;
    ev.dest.client = SNDRV_SEQ_ADDRESS_SUBSCRIBERS;
    snd_seq_kernel_client_dispatch(msynth.seq_client, &ev, 1, 0);
// clear event and reset header
    memset(&ev, 0, sizeof(ev));
    }
    }
    snd_use_lock_free(&msynth.input_use_lock);
    }
#[no_mangle]
unsafe extern "C" fn dump_midi(substream: *mut snd_rawmidi_substream, buf: *const c_char, count: c_int) -> c_int {
    static int dump_midi(struct snd_rawmidi_substream *substream, const char *buf, int count)
    {
    struct snd_rawmidi_runtime *runtime;
    int tmp;
    if (snd_BUG_ON(!substream || !buf))
    return -EINVAL;
    runtime = substream.runtime;
    tmp = runtime.avail;
    if (tmp < count) {
    if (printk_ratelimit())
    pr_err("ALSA: seq_midi: MIDI output buffer overrun\n");
    return -ENOMEM;
    }
    if (snd_rawmidi_kernel_write(substream, buf, count) < count)
    return -EINVAL;
    return 0;
    }
// callback for snd_seq_dump_var_event(), bridging to dump_midi()
#[no_mangle]
unsafe extern "C" fn __dump_midi(ptr: *mut c_void, buf: *mut c_void, count: c_int) -> c_int {
    static int __dump_midi(void *ptr, void *buf, int count)
    {
    return dump_midi(ptr, buf, count);
    }
    static int event_process_midi(struct snd_seq_event *ev, int direct,
    void *private_data, int atomic, int hop)
    {
    struct seq_midisynth *msynth = private_data;
    unsigned char msg[10];	/* buffer for constructing midi messages */
    struct snd_rawmidi_substream *substream;
    let mut err: c_int = 0;
    int len;
    if (snd_BUG_ON(!msynth))
    return -EINVAL;
    scoped_guard(rcu) {
    substream = rcu_dereference(msynth.output_substream);
    if (!substream)
    return -ENODEV;
    snd_use_lock_use(&msynth.output_use_lock);
    }
    if (ev.type == SNDRV_SEQ_EVENT_SYSEX) {	/* special case, to save space */
    if ((ev.flags & SNDRV_SEQ_EVENT_LENGTH_MASK) != SNDRV_SEQ_EVENT_LENGTH_VARIABLE) {
// invalid event
    pr_debug("ALSA: seq_midi: invalid sysex event flags = 0x%x\n", ev.flags);
    goto out;
    }
    snd_seq_dump_var_event(ev, __dump_midi, substream);
    snd_midi_event_reset_decode(msynth.parser);
    } else {
    if (!msynth.parser) {
    err = -EIO;
    goto out;
    }
    len = snd_midi_event_decode(msynth.parser, msg, sizeof(msg), ev);
    if (len < 0)
    goto out;
    if (dump_midi(substream, msg, len) < 0)
    snd_midi_event_reset_decode(msynth.parser);
    }
    out:
    snd_use_lock_free(&msynth.output_use_lock);
    return err;
    }
    static int snd_seq_midisynth_new(struct seq_midisynth *msynth,
    struct snd_card *card,
    int device,
    int subdevice)
    {
    if (snd_midi_event_new(MAX_MIDI_EVENT_BUF, &msynth.parser) < 0)
    return -ENOMEM;
    msynth.card = card;
    msynth.device = device;
    msynth.subdevice = subdevice;
    snd_use_lock_init(&msynth.input_use_lock);
    snd_use_lock_init(&msynth.output_use_lock);
    return 0;
    }
// open associated midi device for input
#[no_mangle]
unsafe extern "C" fn midisynth_subscribe(private_data: *mut c_void, info: *mut snd_seq_port_subscribe) -> c_int {
    static int midisynth_subscribe(void *private_data, struct snd_seq_port_subscribe *info)
    {
    int err;
    struct seq_midisynth *msynth = private_data;
    struct snd_rawmidi_runtime *runtime;
    let mut rfile: snd_rawmidi_file = {};
    struct snd_rawmidi_params params;
// open midi port
    err = snd_rawmidi_kernel_open(msynth.rmidi, msynth.subdevice,
    SNDRV_RAWMIDI_LFLG_INPUT,
    &rfile);
    if (err < 0) {
    pr_debug("ALSA: seq_midi: midi input open failed!!!\n");
    return err;
    }
    runtime = rfile.input.runtime;
    memset(&params, 0, sizeof(params));
    params.avail_min = 1;
    params.buffer_size = input_buffer_size;
    err = snd_rawmidi_input_params(rfile.input, &params);
    if (err < 0) {
    snd_rawmidi_kernel_release(&rfile);
    return err;
    }
    snd_midi_event_reset_encode(msynth.parser);
    runtime.event = snd_midi_input_event;
    runtime.private_data = msynth;
    msynth.input_rfile = rfile;
    rcu_assign_pointer(msynth.input_substream, rfile.input);
    snd_rawmidi_kernel_read(msynth.input_rfile.input, core::ptr::null_mut(), 0);
    return 0;
    }
// close associated midi device for input
#[no_mangle]
unsafe extern "C" fn midisynth_unsubscribe(private_data: *mut c_void, info: *mut snd_seq_port_subscribe) -> c_int {
    static int midisynth_unsubscribe(void *private_data, struct snd_seq_port_subscribe *info)
    {
    int err;
    struct seq_midisynth *msynth = private_data;
    struct snd_rawmidi_file rfile;
    rcu_assign_pointer(msynth.input_substream, core::ptr::null_mut());
    synchronize_rcu();
    snd_use_lock_sync(&msynth.input_use_lock);
    rfile = msynth.input_rfile;
    msynth.input_rfile = (struct snd_rawmidi_file){};
    if (snd_BUG_ON(!rfile.input))
    return -EINVAL;
    err = snd_rawmidi_kernel_release(&rfile);
    return err;
    }
// open associated midi device for output
#[no_mangle]
unsafe extern "C" fn midisynth_use(private_data: *mut c_void, info: *mut snd_seq_port_subscribe) -> c_int {
    static int midisynth_use(void *private_data, struct snd_seq_port_subscribe *info)
    {
    int err;
    struct seq_midisynth *msynth = private_data;
    let mut rfile: snd_rawmidi_file = {};
    struct snd_rawmidi_params params;
// open midi port
    err = snd_rawmidi_kernel_open(msynth.rmidi, msynth.subdevice,
    SNDRV_RAWMIDI_LFLG_OUTPUT,
    &rfile);
    if (err < 0) {
    pr_debug("ALSA: seq_midi: midi output open failed!!!\n");
    return err;
    }
    memset(&params, 0, sizeof(params));
    params.avail_min = 1;
    params.buffer_size = output_buffer_size;
    params.no_active_sensing = 1;
    err = snd_rawmidi_output_params(rfile.output, &params);
    if (err < 0) {
    snd_rawmidi_kernel_release(&rfile);
    return err;
    }
    snd_midi_event_reset_decode(msynth.parser);
    msynth.output_rfile = rfile;
    rcu_assign_pointer(msynth.output_substream, rfile.output);
    return 0;
    }
// close associated midi device for output
#[no_mangle]
unsafe extern "C" fn midisynth_unuse(private_data: *mut c_void, info: *mut snd_seq_port_subscribe) -> c_int {
    static int midisynth_unuse(void *private_data, struct snd_seq_port_subscribe *info)
    {
    struct seq_midisynth *msynth = private_data;
    struct snd_rawmidi_file rfile;
    rcu_assign_pointer(msynth.output_substream, core::ptr::null_mut());
    synchronize_rcu();
    snd_use_lock_sync(&msynth.output_use_lock);
    rfile = msynth.output_rfile;
    msynth.output_rfile = (struct snd_rawmidi_file){};
    if (snd_BUG_ON(!rfile.output))
    return -EINVAL;
    snd_rawmidi_drain_output(rfile.output);
    return snd_rawmidi_kernel_release(&rfile);
    }
// delete given midi synth port
#[no_mangle]
unsafe extern "C" fn snd_seq_midisynth_delete(msynth: *mut seq_midisynth) {
    static void snd_seq_midisynth_delete(struct seq_midisynth *msynth)
    {
    if (msynth == core::ptr::null_mut())
    return;
    if (msynth.seq_client > 0) {
// delete port
    snd_seq_event_port_detach(msynth.seq_client, msynth.seq_port);
    }
    snd_midi_event_free(msynth.parser);
    }
// register new midi synth port
    static int
    snd_seq_midisynth_probe(struct snd_seq_device *dev)
    {
    struct seq_midisynth_client *client;
    struct seq_midisynth *msynth, *ms;
    struct snd_rawmidi *rmidi = dev.private_data;
    let mut newclient: c_int = 0;
    unsigned int p, ports;
    struct snd_seq_port_callback pcallbacks;
    struct snd_card *card = dev.card;
    let mut device: c_int = dev.device;
    let mut input_count: c_uint = 0, output_count = 0;
    if (snd_BUG_ON(!card || device < 0 || device >= SNDRV_RAWMIDI_DEVICES))
    return -EINVAL;
    struct snd_rawmidi_info *info __free(kfree) =
    kmalloc_obj(*info);
    if (! info)
    return -ENOMEM;
    info.device = device;
    info.stream = SNDRV_RAWMIDI_STREAM_OUTPUT;
    info.subdevice = 0;
    if (snd_rawmidi_info_select(card, info) >= 0)
    output_count = info.subdevices_count;
    info.stream = SNDRV_RAWMIDI_STREAM_INPUT;
    if (snd_rawmidi_info_select(card, info) >= 0) {
    input_count = info.subdevices_count;
    }
    ports = output_count;
    if (ports < input_count)
    ports = input_count;
    if (ports == 0)
    return -ENODEV;
    if (ports > (256 / SNDRV_RAWMIDI_DEVICES))
    ports = 256 / SNDRV_RAWMIDI_DEVICES;
    guard(mutex)(&register_mutex);
    client = synths[card.number];
    if (client == core::ptr::null_mut()) {
    newclient = 1;
    client = kzalloc_obj(*client);
    if (client == core::ptr::null_mut())
    return -ENOMEM;
    client.seq_client =
    snd_seq_create_kernel_client(
    card, 0, "%s", card.shortname[0] ?
    (const char *)card.shortname : "External MIDI");
    if (client.seq_client < 0) {
    kfree(client);
    return -ENOMEM;
    }
    }
    msynth = kzalloc_objs(struct seq_midisynth, ports);
    struct snd_seq_port_info *port __free(kfree) =
    kmalloc_obj(*port);
    if (msynth == core::ptr::null_mut() || port == core::ptr::null_mut())
    goto __nomem;
    for (p = 0; p < ports; p++) {
    ms = &msynth[p];
    ms.rmidi = rmidi;
    if (snd_seq_midisynth_new(ms, card, device, p) < 0)
    goto __nomem;
// declare port
    memset(port, 0, sizeof(*port));
    port.addr.client = client.seq_client;
    port.addr.port = device * (256 / SNDRV_RAWMIDI_DEVICES) + p;
    port.flags = SNDRV_SEQ_PORT_FLG_GIVEN_PORT;
    memset(info, 0, sizeof(*info));
    info.device = device;
    if (p < output_count)
    info.stream = SNDRV_RAWMIDI_STREAM_OUTPUT;
    else
    info.stream = SNDRV_RAWMIDI_STREAM_INPUT;
    info.subdevice = p;
    if (snd_rawmidi_info_select(card, info) >= 0)
    strscpy(port.name, info.subname);
    if (! port.name[0]) {
    if (info.name[0]) {
    if (ports > 1)
    scnprintf(port.name, sizeof(port.name), "%s-%u", info.name, p);
    else
    scnprintf(port.name, sizeof(port.name), "%s", info.name);
    } else {
// last resort
    if (ports > 1)
    sprintf(port.name, "MIDI %d-%d-%u", card.number, device, p);
    else
    sprintf(port.name, "MIDI %d-%d", card.number, device);
    }
    }
    if ((info.flags & SNDRV_RAWMIDI_INFO_OUTPUT) && p < output_count)
    port.capability |= SNDRV_SEQ_PORT_CAP_WRITE | SNDRV_SEQ_PORT_CAP_SYNC_WRITE | SNDRV_SEQ_PORT_CAP_SUBS_WRITE;
    if ((info.flags & SNDRV_RAWMIDI_INFO_INPUT) && p < input_count)
    port.capability |= SNDRV_SEQ_PORT_CAP_READ | SNDRV_SEQ_PORT_CAP_SYNC_READ | SNDRV_SEQ_PORT_CAP_SUBS_READ;
    if ((port.capability & (SNDRV_SEQ_PORT_CAP_WRITE|SNDRV_SEQ_PORT_CAP_READ)) == (SNDRV_SEQ_PORT_CAP_WRITE|SNDRV_SEQ_PORT_CAP_READ) &&
    info.flags & SNDRV_RAWMIDI_INFO_DUPLEX)
    port.capability |= SNDRV_SEQ_PORT_CAP_DUPLEX;
    if (port.capability & SNDRV_SEQ_PORT_CAP_READ)
    port.direction |= SNDRV_SEQ_PORT_DIR_INPUT;
    if (port.capability & SNDRV_SEQ_PORT_CAP_WRITE)
    port.direction |= SNDRV_SEQ_PORT_DIR_OUTPUT;
    port.type = SNDRV_SEQ_PORT_TYPE_MIDI_GENERIC
    | SNDRV_SEQ_PORT_TYPE_HARDWARE
    | SNDRV_SEQ_PORT_TYPE_PORT;
    port.midi_channels = 16;
    memset(&pcallbacks, 0, sizeof(pcallbacks));
    pcallbacks.owner = THIS_MODULE;
    pcallbacks.private_data = ms;
    pcallbacks.subscribe = midisynth_subscribe;
    pcallbacks.unsubscribe = midisynth_unsubscribe;
    pcallbacks.use = midisynth_use;
    pcallbacks.unuse = midisynth_unuse;
    pcallbacks.event_input = event_process_midi;
    port.kernel = &pcallbacks;
    if (rmidi.ops && rmidi.ops.get_port_info)
    rmidi.ops.get_port_info(rmidi, p, port);
    if (snd_seq_kernel_client_ctl(client.seq_client, SNDRV_SEQ_IOCTL_CREATE_PORT, port)<0)
    goto __nomem;
    ms.seq_client = client.seq_client;
    ms.seq_port = port.addr.port;
    }
    client.ports_per_device[device] = ports;
    client.ports[device] = msynth;
    client.num_ports++;
    if (newclient)
    synths[card.number] = client;
    return 0;	/* success */
    __nomem:
    if (msynth != core::ptr::null_mut()) {
    for (p = 0; p < ports; p++)
    snd_seq_midisynth_delete(&msynth[p]);
    kfree(msynth);
    }
    if (newclient) {
    snd_seq_delete_kernel_client(client.seq_client);
    kfree(client);
    }
    return -ENOMEM;
    }
// release midi synth port
    static void
    snd_seq_midisynth_remove(struct snd_seq_device *dev)
    {
    struct seq_midisynth_client *client;
    struct seq_midisynth *msynth;
    struct snd_card *card = dev.card;
    let mut device: c_int = dev.device, p, ports;
    guard(mutex)(&register_mutex);
    client = synths[card.number];
    if (client == core::ptr::null_mut() || client.ports[device] == core::ptr::null_mut())
    return;
    ports = client.ports_per_device[device];
    client.ports_per_device[device] = 0;
    msynth = client.ports[device];
    client.ports[device] = core::ptr::null_mut();
    for (p = 0; p < ports; p++)
    snd_seq_midisynth_delete(&msynth[p]);
    kfree(msynth);
    client.num_ports--;
    if (client.num_ports <= 0) {
    snd_seq_delete_kernel_client(client.seq_client);
    synths[card.number] = core::ptr::null_mut();
    kfree(client);
    }
    }
    static struct snd_seq_driver seq_midisynth_driver = {
    .probe = snd_seq_midisynth_probe,
    .remove = snd_seq_midisynth_remove,
    .driver = {
    .name = KBUILD_MODNAME,
    },
    .id = SNDRV_SEQ_DEV_ID_MIDISYNTH,
    .argsize = 0,
    };
    module_snd_seq_driver(seq_midisynth_driver);
