//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/88pm860x_onkey.c
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
// 88pm860x_onkey.c - Marvell 88PM860x ONKEY driver
//
// Copyright (C) 2009-2010 Marvell International Ltd.
// Haojian Zhuang <haojian.zhuang@marvell.com>
//
// This file is subject to the terms and conditions of the GNU General
// Public License. See the file "COPYING" in the main directory of this
// archive for more details.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
//

pub const PM8607_WAKEUP: c_uint = 0x0b;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm860x_onkey_info {
    pub idev: *mut input_dev,
    pub chip: *mut pm860x_chip,
    pub i2c: *mut i2c_client,
    pub dev: *mut device,
    pub irq: c_int,
}

// 88PM860x gives us an interrupt when ONKEY is held
#[no_mangle]
unsafe extern "C" fn pm860x_onkey_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t pm860x_onkey_handler(int irq, void *data)
    {
    struct pm860x_onkey_info *info = data;
    int ret;
    ret = pm860x_reg_read(info.i2c, PM8607_STATUS_2);
    ret &= ONKEY_STATUS;
    input_report_key(info.idev, KEY_POWER, ret);
    input_sync(info.idev);
// Enable 8-second long onkey detection
    pm860x_set_bits(info.i2c, PM8607_WAKEUP, 3, LONG_ONKEY_EN);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn pm860x_onkey_probe(pdev: *mut platform_device) -> c_int {
    static int pm860x_onkey_probe(struct platform_device *pdev)
    {
    struct pm860x_chip *chip = dev_get_drvdata(pdev.dev.parent);
    struct pm860x_onkey_info *info;
    int irq, ret;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return -EINVAL;
    info = devm_kzalloc(&pdev.dev, sizeof(struct pm860x_onkey_info),
    GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    info.chip = chip;
    info.i2c = (chip.id == CHIP_PM8607) ? chip.client : chip.companion;
    info.dev = &pdev.dev;
    info.irq = irq;
    info.idev = devm_input_allocate_device(&pdev.dev);
    if (!info.idev) {
    dev_err(chip.dev, "Failed to allocate input dev\n");
    return -ENOMEM;
    }
    info.idev.name = "88pm860x_on";
    info.idev.phys = "88pm860x_on/input0";
    info.idev.id.bustype = BUS_I2C;
    info.idev.dev.parent = &pdev.dev;
    info.idev.evbit[0] = BIT_MASK(EV_KEY);
    info.idev.keybit[BIT_WORD(KEY_POWER)] = BIT_MASK(KEY_POWER);
    ret = input_register_device(info.idev);
    if (ret) {
    dev_err(chip.dev, "Can't register input device: %d\n", ret);
    return ret;
    }
    ret = devm_request_threaded_irq(&pdev.dev, info.irq, core::ptr::null_mut(),
    pm860x_onkey_handler, IRQF_ONESHOT,
    "onkey", info);
    if (ret < 0) {
    dev_err(chip.dev, "Failed to request IRQ: #%d: %d\n",
    info.irq, ret);
    return ret;
    }
    platform_set_drvdata(pdev, info);
    device_init_wakeup(&pdev.dev, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pm860x_onkey_suspend(dev: *mut device) -> c_int {
    static int pm860x_onkey_suspend(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct pm860x_chip *chip = dev_get_drvdata(pdev.dev.parent);
    if (device_may_wakeup(dev))
    chip.wakeup_flag |= 1 << PM8607_IRQ_ONKEY;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pm860x_onkey_resume(dev: *mut device) -> c_int {
    static int pm860x_onkey_resume(struct device *dev)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct pm860x_chip *chip = dev_get_drvdata(pdev.dev.parent);
    if (device_may_wakeup(dev))
    chip.wakeup_flag &= ~(1 << PM8607_IRQ_ONKEY);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(pm860x_onkey_pm_ops,
    pm860x_onkey_suspend, pm860x_onkey_resume);
    static struct platform_driver pm860x_onkey_driver = {
    .driver		= {
    .name	= "88pm860x-onkey",
    .pm	= pm_sleep_ptr(&pm860x_onkey_pm_ops),
    },
    .probe		= pm860x_onkey_probe,
    };
    module_platform_driver(pm860x_onkey_driver);
    MODULE_DESCRIPTION("Marvell 88PM860x ONKEY driver");
    MODULE_AUTHOR("Haojian Zhuang <haojian.zhuang@marvell.com>");
    MODULE_LICENSE("GPL");
