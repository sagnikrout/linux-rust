//! Automatically rewritten from C to Rust
//! Source: sound/pci/ctxfi/xfi.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// xfi linux driver.
//
// Copyright (C) 2008, Creative Technology Ltd. All Rights Reserved.
//

    MODULE_AUTHOR("Creative Technology Ltd");
    MODULE_DESCRIPTION("X-Fi driver version 1.03");
    MODULE_LICENSE("GPL v2");
    let mut reference_rate: static unsigned int = 48000;
    let mut multiple: static unsigned int = 2;
    MODULE_PARM_DESC(reference_rate, "Reference rate (default=48000)");
    module_param(reference_rate, uint, 0444);
    MODULE_PARM_DESC(multiple, "Rate multiplier (default=2)");
    module_param(multiple, uint, 0444);
    static int index[SNDRV_CARDS] = SNDRV_DEFAULT_IDX;
    static char *id[SNDRV_CARDS] = SNDRV_DEFAULT_STR;
    static bool enable[SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP;
    static unsigned int subsystem[SNDRV_CARDS];
    module_param_array(index, int, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(index, "Index value for Creative X-Fi driver");
    module_param_array(id, charp, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(id, "ID string for Creative X-Fi driver");
    module_param_array(enable, bool, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(enable, "Enable Creative X-Fi driver");
    module_param_array(subsystem, int, core::ptr::null_mut(), 0444);
    MODULE_PARM_DESC(subsystem, "Override subsystem ID for Creative X-Fi driver");
    static const struct pci_device_id ct_pci_dev_ids[] = {
// only X-Fi is supported, so...
    { PCI_DEVICE(PCI_VENDOR_ID_CREATIVE, PCI_DEVICE_ID_CREATIVE_20K1),
    .driver_data = ATC20K1,
    },
    { PCI_DEVICE(PCI_VENDOR_ID_CREATIVE, PCI_DEVICE_ID_CREATIVE_20K2),
    .driver_data = ATC20K2,
    },
    { 0, }
    };
    MODULE_DEVICE_TABLE(pci, ct_pci_dev_ids);
    static int
    ct_card_probe(struct pci_dev *pci, const struct pci_device_id *pci_id)
    {
    static int dev;
    struct snd_card *card;
    struct ct_atc *atc;
    int err;
    if (dev >= SNDRV_CARDS)
    return -ENODEV;
    if (!enable[dev]) {
    dev++;
    return -ENOENT;
    }
    err = snd_card_new(&pci.dev, index[dev], id[dev], THIS_MODULE,
    0, &card);
    if (err)
    return err;
    if ((reference_rate != 48000) && (reference_rate != 44100)) {
    dev_err(card.dev,
    "Invalid reference_rate value %u!!!\n",
    reference_rate);
    dev_err(card.dev,
    "The valid values for reference_rate are 48000 and 44100, Value 48000 is assumed.\n");
    reference_rate = 48000;
    }
    if ((multiple != 1) && (multiple != 2) && (multiple != 4)) {
    dev_err(card.dev, "Invalid multiple value %u!!!\n",
    multiple);
    dev_err(card.dev,
    "The valid values for multiple are 1, 2 and 4, Value 2 is assumed.\n");
    multiple = 2;
    }
    err = ct_atc_create(card, pci, reference_rate, multiple,
    pci_id.driver_data, subsystem[dev], &atc);
    if (err < 0)
    goto error;
    card.private_data = atc;
// Create alsa devices supported by this card
    err = ct_atc_create_alsa_devs(atc);
    if (err < 0)
    goto error;
    strscpy(card.driver, "SB-XFi");
    strscpy(card.shortname, "Creative X-Fi");
    snprintf(card.longname, sizeof(card.longname), "%s %s %s",
    card.shortname, atc.chip_name, atc.model_name);
    err = snd_card_register(card);
    if (err < 0)
    goto error;
    pci_set_drvdata(pci, card);
    dev++;
    return 0;
    error:
    snd_card_free(card);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ct_card_remove(pci: *mut pci_dev) {
    static void ct_card_remove(struct pci_dev *pci)
    {
    snd_card_free(pci_get_drvdata(pci));
    }

#[no_mangle]
unsafe extern "C" fn ct_card_suspend(dev: *mut device) -> c_int {
    static int ct_card_suspend(struct device *dev)
    {
    struct snd_card *card = dev_get_drvdata(dev);
    struct ct_atc *atc = card.private_data;
    return atc.suspend(atc);
    }
#[no_mangle]
unsafe extern "C" fn ct_card_resume(dev: *mut device) -> c_int {
    static int ct_card_resume(struct device *dev)
    {
    struct snd_card *card = dev_get_drvdata(dev);
    struct ct_atc *atc = card.private_data;
    return atc.resume(atc);
    }
    static SIMPLE_DEV_PM_OPS(ct_card_pm, ct_card_suspend, ct_card_resume);

    static struct pci_driver ct_driver = {
    .name = KBUILD_MODNAME,
    .id_table = ct_pci_dev_ids,
    .probe = ct_card_probe,
    .remove = ct_card_remove,
    .driver = {
    .pm = CT_CARD_PM_OPS,
    },
    };
    module_pci_driver(ct_driver);
