//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/maps/cfi_flagadm.c
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


//
// Copyright © 2001 Flaga hf. Medical Devices, Kári Davíðsson <kd@flaga.is>
//
// This program is free software; you can redistribute  it and/or modify it
// under  the terms of  the GNU General  Public License as published by the
// Free Software Foundation;  either version 2 of the  License, or (at your
// option) any later version.
//
// THIS  SOFTWARE  IS PROVIDED   ``AS  IS'' AND   ANY  EXPRESS OR IMPLIED
// WARRANTIES,   INCLUDING, BUT NOT  LIMITED  TO, THE IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.  IN
// NO  EVENT  SHALL   THE AUTHOR  BE    LIABLE FOR ANY   DIRECT, INDIRECT,
// INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
// NOT LIMITED   TO, PROCUREMENT OF  SUBSTITUTE GOODS  OR SERVICES; LOSS OF
// USE, DATA,  OR PROFITS; OR  BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON
// ANY THEORY OF LIABILITY, WHETHER IN  CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
// THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// You should have received a copy of the  GNU General Public License along
// with this program; if not, write  to the Free Software Foundation, Inc.,
// 675 Mass Ave, Cambridge, MA 02139, USA.
//

// We split the flash chip up into four parts.
// 1: bootloader first 128k			(0x00000000 - 0x0001FFFF) size 0x020000
// 2: kernel 640k					(0x00020000 - 0x000BFFFF) size 0x0A0000
// 3: compressed 1536k root ramdisk	(0x000C0000 - 0x0023FFFF) size 0x180000
// 4: writeable diskpartition (jffs)(0x00240000 - 0x003FFFFF) size 0x1C0000
//
pub const FLASH_PHYS_ADDR: c_uint = 0x40000000;
pub const FLASH_SIZE: c_uint = 0x400000;
pub const FLASH_PARTITION0_ADDR: c_uint = 0x00000000;
pub const FLASH_PARTITION0_SIZE: c_uint = 0x00020000;
pub const FLASH_PARTITION1_ADDR: c_uint = 0x00020000;
pub const FLASH_PARTITION1_SIZE: c_uint = 0x000A0000;
pub const FLASH_PARTITION2_ADDR: c_uint = 0x000C0000;
pub const FLASH_PARTITION2_SIZE: c_uint = 0x00180000;
pub const FLASH_PARTITION3_ADDR: c_uint = 0x00240000;
pub const FLASH_PARTITION3_SIZE: c_uint = 0x001C0000;
    static struct map_info flagadm_map = {
    .name =		"FlagaDM flash device",
    .size =		FLASH_SIZE,
    .bankwidth =	2,
    };
    static const struct mtd_partition flagadm_parts[] = {
    {
    .name =		"Bootloader",
    .offset	=	FLASH_PARTITION0_ADDR,
    .size =		FLASH_PARTITION0_SIZE
    },
    {
    .name =		"Kernel image",
    .offset =	FLASH_PARTITION1_ADDR,
    .size =		FLASH_PARTITION1_SIZE
    },
    {
    .name =		"Initial ramdisk image",
    .offset =	FLASH_PARTITION2_ADDR,
    .size =		FLASH_PARTITION2_SIZE
    },
    {
    .name =		"Persistent storage",
    .offset =	FLASH_PARTITION3_ADDR,
    .size =		FLASH_PARTITION3_SIZE
    }
    };

    static struct mtd_info *mymtd;
#[no_mangle]
unsafe extern "C" fn init_flagadm() -> int __init {
    static int __init init_flagadm(void)
    {
    printk(KERN_NOTICE "FlagaDM flash device: %x at %x\n",
    FLASH_SIZE, FLASH_PHYS_ADDR);
    flagadm_map.phys = FLASH_PHYS_ADDR;
    flagadm_map.virt = ioremap(FLASH_PHYS_ADDR,
    FLASH_SIZE);
    if (!flagadm_map.virt) {
    printk("Failed to ioremap\n");
    return -EIO;
    }
    simple_map_init(&flagadm_map);
    mymtd = do_map_probe("cfi_probe", &flagadm_map);
    if (mymtd) {
    mymtd.owner = THIS_MODULE;
    mtd_device_register(mymtd, flagadm_parts, PARTITION_COUNT);
    printk(KERN_NOTICE "FlagaDM flash device initialized\n");
    return 0;
    }
    iounmap((void __iomem *)flagadm_map.virt);
    return -ENXIO;
    }
#[no_mangle]
unsafe extern "C" fn cleanup_flagadm() -> void __exit {
    static void __exit cleanup_flagadm(void)
    {
    if (mymtd) {
    mtd_device_unregister(mymtd);
    map_destroy(mymtd);
    }
    if (flagadm_map.virt) {
    iounmap((void __iomem *)flagadm_map.virt);
    flagadm_map.virt = core::ptr::null_mut();
    }
    }
    module_init(init_flagadm);
    module_exit(cleanup_flagadm);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Kári Davíðsson <kd@flaga.is>");
    MODULE_DESCRIPTION("MTD map driver for Flaga digital module");
