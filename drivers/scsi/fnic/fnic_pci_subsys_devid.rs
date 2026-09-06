//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/fnic/fnic_pci_subsys_devid.c
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
// Copyright 2008 Cisco Systems, Inc.  All rights reserved.
// Copyright 2007 Nuova Systems, Inc.  All rights reserved.
//

    static struct fnic_pcie_device fnic_pcie_device_table[] = {
    {PCI_DEVICE_ID_CISCO_SERENO, "Sereno", PCI_SUBDEVICE_ID_CISCO_VASONA,
    "VIC 1280"},
    {PCI_DEVICE_ID_CISCO_SERENO, "Sereno", PCI_SUBDEVICE_ID_CISCO_COTATI,
    "VIC 1240"},
    {PCI_DEVICE_ID_CISCO_SERENO, "Sereno",
    PCI_SUBDEVICE_ID_CISCO_LEXINGTON, "VIC 1225"},
    {PCI_DEVICE_ID_CISCO_SERENO, "Sereno", PCI_SUBDEVICE_ID_CISCO_ICEHOUSE,
    "VIC 1285"},
    {PCI_DEVICE_ID_CISCO_SERENO, "Sereno",
    PCI_SUBDEVICE_ID_CISCO_KIRKWOODLAKE, "VIC 1225T"},
    {PCI_DEVICE_ID_CISCO_SERENO, "Sereno",
    PCI_SUBDEVICE_ID_CISCO_SUSANVILLE, "VIC 1227"},
    {PCI_DEVICE_ID_CISCO_SERENO, "Sereno", PCI_SUBDEVICE_ID_CISCO_TORRANCE,
    "VIC 1227T"},
    {PCI_DEVICE_ID_CISCO_CRUZ, "Cruz", PCI_SUBDEVICE_ID_CISCO_CALISTOGA,
    "VIC 1340"},
    {PCI_DEVICE_ID_CISCO_CRUZ, "Cruz", PCI_SUBDEVICE_ID_CISCO_MOUNTAINVIEW,
    "VIC 1380"},
    {PCI_DEVICE_ID_CISCO_CRUZ, "Cruz", PCI_SUBDEVICE_ID_CISCO_MOUNTTIAN,
    "C3260-SIOC"},
    {PCI_DEVICE_ID_CISCO_CRUZ, "Cruz", PCI_SUBDEVICE_ID_CISCO_CLEARLAKE,
    "VIC 1385"},
    {PCI_DEVICE_ID_CISCO_CRUZ, "Cruz", PCI_SUBDEVICE_ID_CISCO_MOUNTTIAN2,
    "C3260-SIOC"},
    {PCI_DEVICE_ID_CISCO_CRUZ, "Cruz", PCI_SUBDEVICE_ID_CISCO_CLAREMONT,
    "VIC 1387"},
    {PCI_DEVICE_ID_CISCO_BODEGA, "Bodega", PCI_SUBDEVICE_ID_CISCO_BRADBURY,
    "VIC 1457"},
    {PCI_DEVICE_ID_CISCO_BODEGA, "Bodega",
    PCI_SUBDEVICE_ID_CISCO_BRENTWOOD, "VIC 1455"},
    {PCI_DEVICE_ID_CISCO_BODEGA, "Bodega",
    PCI_SUBDEVICE_ID_CISCO_BURLINGAME, "VIC 1487"},
    {PCI_DEVICE_ID_CISCO_BODEGA, "Bodega", PCI_SUBDEVICE_ID_CISCO_BAYSIDE,
    "VIC 1485"},
    {PCI_DEVICE_ID_CISCO_BODEGA, "Bodega",
    PCI_SUBDEVICE_ID_CISCO_BAKERSFIELD, "VIC 1440"},
    {PCI_DEVICE_ID_CISCO_BODEGA, "Bodega",
    PCI_SUBDEVICE_ID_CISCO_BOONVILLE, "VIC 1480"},
    {PCI_DEVICE_ID_CISCO_BODEGA, "Bodega", PCI_SUBDEVICE_ID_CISCO_BENICIA,
    "VIC 1495"},
    {PCI_DEVICE_ID_CISCO_BODEGA, "Bodega", PCI_SUBDEVICE_ID_CISCO_BEAUMONT,
    "VIC 1497"},
    {PCI_DEVICE_ID_CISCO_BODEGA, "Bodega", PCI_SUBDEVICE_ID_CISCO_BRISBANE,
    "VIC 1467"},
    {PCI_DEVICE_ID_CISCO_BODEGA, "Bodega", PCI_SUBDEVICE_ID_CISCO_BENTON,
    "VIC 1477"},
    {PCI_DEVICE_ID_CISCO_BODEGA, "Bodega",
    PCI_SUBDEVICE_ID_CISCO_TWIN_RIVER, "VIC 14425"},
    {PCI_DEVICE_ID_CISCO_BODEGA, "Bodega",
    PCI_SUBDEVICE_ID_CISCO_TWIN_PEAK, "VIC 14825"},
    {PCI_DEVICE_ID_CISCO_BEVERLY, "Beverly", PCI_SUBDEVICE_ID_CISCO_BERN,
    "VIC 15420"},
    {PCI_DEVICE_ID_CISCO_BEVERLY, "Beverly",
    PCI_SUBDEVICE_ID_CISCO_STOCKHOLM, "VIC 15428"},
    {PCI_DEVICE_ID_CISCO_BEVERLY, "Beverly", PCI_SUBDEVICE_ID_CISCO_KRAKOW,
    "VIC 15411"},
    {PCI_DEVICE_ID_CISCO_BEVERLY, "Beverly",
    PCI_SUBDEVICE_ID_CISCO_LUCERNE, "VIC 15231"},
    {PCI_DEVICE_ID_CISCO_BEVERLY, "Beverly", PCI_SUBDEVICE_ID_CISCO_TURKU,
    "VIC 15238"},
    {PCI_DEVICE_ID_CISCO_BEVERLY, "Beverly", PCI_SUBDEVICE_ID_CISCO_GENEVA,
    "VIC 15422"},
    {PCI_DEVICE_ID_CISCO_BEVERLY, "Beverly",
    PCI_SUBDEVICE_ID_CISCO_HELSINKI, "VIC 15235"},
    {PCI_DEVICE_ID_CISCO_BEVERLY, "Beverly",
    PCI_SUBDEVICE_ID_CISCO_GOTHENBURG, "VIC 15425"},
    {PCI_DEVICE_ID_CISCO_BEVERLY, "Beverly",
    PCI_SUBDEVICE_ID_CISCO_TURKU_PLUS, "VIC 15237"},
    {PCI_DEVICE_ID_CISCO_BEVERLY, "Beverly", PCI_SUBDEVICE_ID_CISCO_ZURICH,
    "VIC 15230"},
    {PCI_DEVICE_ID_CISCO_BEVERLY, "Beverly", PCI_SUBDEVICE_ID_CISCO_RIGA,
    "VIC 15427"},
    {0,}
    };
    int fnic_get_desc_by_devid(struct pci_dev *pdev, char **desc,
    char **subsys_desc)
    {
    let mut device: c_ushort = PCI_DEVICE_ID_CISCO_VIC_FC;
    let mut max: c_int = ARRAY_SIZE(fnic_pcie_device_table);
    struct fnic_pcie_device *t = fnic_pcie_device_table;
    let mut index: c_int = 0;
    if (pdev.device != device)
    return 1;
    while (t.device != 0) {
    if (memcmp
    ((char *) &pdev.subsystem_device,
    (char *) &t.subsystem_device, sizeof(short)) == 0)
    break;
    t++;
    index++;
    }
    if (index >= max - 1) {
// desc = NULL;
// subsys_desc = NULL;
    return 1;
    }
// desc = fnic_pcie_device_table[index].desc;
// subsys_desc = fnic_pcie_device_table[index].subsys_desc;
    return 0;
    }
