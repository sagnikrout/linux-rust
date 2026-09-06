//! Automatically rewritten from C to Rust
//! Source: sound/ppc/powermac.c
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
// Driver for PowerMac AWACS
// Copyright (c) 2001 by Takashi Iwai <tiwai@suse.de>
// based on dmasound.c.
//

    MODULE_DESCRIPTION("PowerMac");
    MODULE_LICENSE("GPL");
    static int index = SNDRV_DEFAULT_IDX1;		/* Index 0-MAX */
    static char *id = SNDRV_DEFAULT_STR1;		/* ID for this card */
    let mut enable_beep: static bool = 1;
    module_param(index, int, 0444);
    MODULE_PARM_DESC(index, "Index value for " CHIP_NAME " soundchip.");
    module_param(id, charp, 0444);
    MODULE_PARM_DESC(id, "ID string for " CHIP_NAME " soundchip.");
    module_param(enable_beep, bool, 0444);
    MODULE_PARM_DESC(enable_beep, "Enable beep using PCM.");
    static struct platform_device *device;
//
#[no_mangle]
unsafe extern "C" fn snd_pmac_probe(devptr: *mut platform_device) -> c_int {
    static int snd_pmac_probe(struct platform_device *devptr)
    {
    struct snd_card *card;
    struct snd_pmac *chip;
    char *name_ext;
    int err;
    err = snd_card_new(&devptr.dev, index, id, THIS_MODULE, 0, &card);
    if (err < 0)
    return err;
    err = snd_pmac_new(card, &chip);
    if (err < 0)
    goto __error;
    card.private_data = chip;
    switch (chip.model) {
    case PMAC_BURGUNDY:
    strscpy(card.driver, "PMac Burgundy");
    strscpy(card.shortname, "PowerMac Burgundy");
    sprintf(card.longname, "%s (Dev %d) Sub-frame %d",
    card.shortname, chip.device_id, chip.subframe);
    err = snd_pmac_burgundy_init(chip);
    if (err < 0)
    goto __error;
    break;
    case PMAC_DACA:
    strscpy(card.driver, "PMac DACA");
    strscpy(card.shortname, "PowerMac DACA");
    sprintf(card.longname, "%s (Dev %d) Sub-frame %d",
    card.shortname, chip.device_id, chip.subframe);
    err = snd_pmac_daca_init(chip);
    if (err < 0)
    goto __error;
    break;
    case PMAC_TUMBLER:
    case PMAC_SNAPPER:
    name_ext = chip.model == PMAC_TUMBLER ? "Tumbler" : "Snapper";
    sprintf(card.driver, "PMac %s", name_ext);
    sprintf(card.shortname, "PowerMac %s", name_ext);
    sprintf(card.longname, "%s (Dev %d) Sub-frame %d",
    card.shortname, chip.device_id, chip.subframe);
    err = snd_pmac_tumbler_init(chip);
    if (err < 0)
    goto __error;
    err = snd_pmac_tumbler_post_init();
    if (err < 0)
    goto __error;
    break;
    case PMAC_AWACS:
    case PMAC_SCREAMER:
    name_ext = chip.model == PMAC_SCREAMER ? "Screamer" : "AWACS";
    sprintf(card.driver, "PMac %s", name_ext);
    sprintf(card.shortname, "PowerMac %s", name_ext);
    if (chip.is_pbook_3400)
    name_ext = " [PB3400]";
#[no_mangle]
pub unsafe extern "C" fn if(_arg: chip->is_pbook_G3) -> else {
    else if (chip.is_pbook_G3)
    name_ext = " [PBG3]";
    else
    name_ext = "";
    sprintf(card.longname, "%s%s Rev %d",
    card.shortname, name_ext, chip.revision);
    err = snd_pmac_awacs_init(chip);
    if (err < 0)
    goto __error;
    break;
    default:
    dev_err(&devptr.dev, "unsupported hardware %d\n", chip.model);
    err = -EINVAL;
    goto __error;
    }
    err = snd_pmac_pcm_new(chip);
    if (err < 0)
    goto __error;
    chip.initialized = 1;
    if (enable_beep)
    snd_pmac_attach_beep(chip);
    err = snd_card_register(card);
    if (err < 0)
    goto __error;
    platform_set_drvdata(devptr, card);
    return 0;
    __error:
    snd_card_free(card);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn snd_pmac_remove(devptr: *mut platform_device) {
    static void snd_pmac_remove(struct platform_device *devptr)
    {
    snd_card_free(platform_get_drvdata(devptr));
    }

#[no_mangle]
unsafe extern "C" fn snd_pmac_driver_suspend(dev: *mut device) -> c_int {
    static int snd_pmac_driver_suspend(struct device *dev)
    {
    struct snd_card *card = dev_get_drvdata(dev);
    snd_pmac_suspend(card.private_data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_pmac_driver_resume(dev: *mut device) -> c_int {
    static int snd_pmac_driver_resume(struct device *dev)
    {
    struct snd_card *card = dev_get_drvdata(dev);
    snd_pmac_resume(card.private_data);
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(snd_pmac_pm, snd_pmac_driver_suspend, snd_pmac_driver_resume);

    static struct platform_driver snd_pmac_driver = {
    .probe		= snd_pmac_probe,
    .remove		= snd_pmac_remove,
    .driver		= {
    .name	= SND_PMAC_DRIVER,
    .pm	= SND_PMAC_PM_OPS,
    },
    };
#[no_mangle]
unsafe extern "C" fn alsa_card_pmac_init() -> int __init {
    static int __init alsa_card_pmac_init(void)
    {
    int err;
    err = platform_driver_register(&snd_pmac_driver);
    if (err < 0)
    return err;
    device = platform_device_register_simple(SND_PMAC_DRIVER, -1, core::ptr::null_mut(), 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn alsa_card_pmac_exit() -> void __exit {
    static void __exit alsa_card_pmac_exit(void)
    {
    if (!IS_ERR(device))
    platform_device_unregister(device);
    platform_driver_unregister(&snd_pmac_driver);
    }
    module_init(alsa_card_pmac_init)
    module_exit(alsa_card_pmac_exit)
