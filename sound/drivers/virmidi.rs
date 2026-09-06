//! Automatically rewritten from C to Rust
//! Source: sound/drivers/virmidi.c
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
// Dummy soundcard for virtual rawmidi devices
//
// Copyright (c) 2000 by Takashi Iwai <tiwai@suse.de>
//
// VIRTUAL RAW MIDI DEVICE CARDS
//
// This dummy card contains up to 4 virtual rawmidi devices.
// They are not real rawmidi devices but just associated with sequencer
// clients, so that any input/output sources can be connected as a raw
// MIDI device arbitrary.
// Also, multiple access is allowed to a single rawmidi device.
//
// Typical usage is like following:
// - Load snd-virmidi module.
// # modprobe snd-virmidi index=2
// Then, sequencer clients 72:0 to 75:0 will be created, which are
// mapped from /dev/snd/midiC1D0 to /dev/snd/midiC1D3, respectively.
//
// - Connect input/output via aconnect.
// % aconnect 64:0 72:0	# keyboard input redirection 64:0 -> 72:0
// % aconnect 72:0 65:0	# output device redirection 72:0 -> 65:0
//
// - Run application using a midi device (eg. /dev/snd/midiC1D0)
//

// hack: OSS defines midi_devs, so undefine it (versioned symbols)

    MODULE_AUTHOR("Takashi Iwai <tiwai@suse.de>");
    MODULE_DESCRIPTION("Dummy soundcard for virtual rawmidi devices");
    MODULE_LICENSE("GPL");
pub const MAX_MIDI_DEVICES: c_int = 4;
    static int index[SNDRV_CARDS] = SNDRV_DEFAULT_IDX;	/* Index 0-MAX */
    static char *id[SNDRV_CARDS] = SNDRV_DEFAULT_STR;	/* ID for this card */
    static bool enable[SNDRV_CARDS] = {1, [1 ... (SNDRV_CARDS - 1)] = 0};
    static int midi_devs[SNDRV_CARDS] = {[0 ... (SNDRV_CARDS - 1)] = 4};
    module_param_array(index, int, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(index, "Index value for virmidi soundcard.");
    module_param_array(id, charp, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(id, "ID string for virmidi soundcard.");
    module_param_array(enable, bool, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(enable, "Enable this soundcard.");
    module_param_array(midi_devs, int, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(midi_devs, "MIDI devices # (1-4)");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_card_virmidi {
    pub card: *mut snd_card,
    pub midi: [*mut snd_rawmidi; MAX_MIDI_DEVICES],
}

    static struct platform_device *devices[SNDRV_CARDS];
#[no_mangle]
unsafe extern "C" fn snd_virmidi_probe(devptr: *mut platform_device) -> c_int {
    static int snd_virmidi_probe(struct platform_device *devptr)
    {
    struct snd_card *card;
    struct snd_card_virmidi *vmidi;
    int idx, err;
    let mut dev: c_int = devptr.id;
    if (dev < 0 || dev >= SNDRV_CARDS) {
    dev_warn(&devptr.dev,
    "Invalid card index %d, using default 0\n", dev);
    dev = 0;
    }
    err = snd_devm_card_new(&devptr.dev, index[dev], id[dev], THIS_MODULE,
    sizeof(struct snd_card_virmidi), &card);
    if (err < 0)
    return err;
    vmidi = card.private_data;
    vmidi.card = card;
    if (midi_devs[dev] > MAX_MIDI_DEVICES) {
    dev_warn(&devptr.dev,
    "too much midi devices for virmidi %d: force to use %d\n",
    dev, MAX_MIDI_DEVICES);
    midi_devs[dev] = MAX_MIDI_DEVICES;
    }
    for (idx = 0; idx < midi_devs[dev]; idx++) {
    struct snd_rawmidi *rmidi;
    err = snd_virmidi_new(card, idx, &rmidi);
    if (err < 0)
    return err;
    vmidi.midi[idx] = rmidi;
    strscpy(rmidi.name, "Virtual Raw MIDI");
    }
    strscpy(card.driver, "VirMIDI");
    strscpy(card.shortname, "VirMIDI");
    sprintf(card.longname, "Virtual MIDI Card %i", dev + 1);
    err = snd_card_register(card);
    if (err)
    return err;
    platform_set_drvdata(devptr, card);
    return 0;
    }

    static struct platform_driver snd_virmidi_driver = {
    .probe		= snd_virmidi_probe,
    .driver		= {
    .name	= SND_VIRMIDI_DRIVER,
    },
    };
#[no_mangle]
unsafe extern "C" fn snd_virmidi_unregister_all() {
    static void snd_virmidi_unregister_all(void)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(devices); ++i)
    platform_device_unregister(devices[i]);
    platform_driver_unregister(&snd_virmidi_driver);
    }
#[no_mangle]
unsafe extern "C" fn alsa_card_virmidi_init() -> int __init {
    static int __init alsa_card_virmidi_init(void)
    {
    int i, cards, err;
    err = platform_driver_register(&snd_virmidi_driver);
    if (err < 0)
    return err;
    cards = 0;
    for (i = 0; i < SNDRV_CARDS; i++) {
    struct platform_device *device;
    if (!enable[i])
    continue;
    device = platform_device_register_simple(SND_VIRMIDI_DRIVER,
    i, core::ptr::null_mut(), 0);
    if (IS_ERR(device))
    continue;
    if (!platform_get_drvdata(device)) {
    platform_device_unregister(device);
    continue;
    }
    devices[i] = device;
    cards++;
    }
    if (!cards) {

    pr_err("Card-VirMIDI soundcard not found or device busy\n");

    snd_virmidi_unregister_all();
    return -ENODEV;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn alsa_card_virmidi_exit() -> void __exit {
    static void __exit alsa_card_virmidi_exit(void)
    {
    snd_virmidi_unregister_all();
    }
    module_init(alsa_card_virmidi_init)
    module_exit(alsa_card_virmidi_exit)
