#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Programmable Fast Interrupt Controller."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pfic {
    ptr: *mut u8,
}
unsafe impl Send for Pfic {}
unsafe impl Sync for Pfic {}
impl Pfic {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Interrupt Status Register."]
    #[inline(always)]
    pub const fn isr1(self) -> crate::common::Reg<regs::Isr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Interrupt Status Register."]
    #[inline(always)]
    pub const fn isr2(self) -> crate::common::Reg<regs::Isr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Interrupt Pending Register."]
    #[inline(always)]
    pub const fn ipr1(self) -> crate::common::Reg<regs::Ipr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Interrupt Pending Register."]
    #[inline(always)]
    pub const fn ipr2(self) -> crate::common::Reg<regs::Ipr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Interrupt Priority Register."]
    #[inline(always)]
    pub const fn ithresdr(self) -> crate::common::Reg<regs::Ithresdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "Interrupt Fast Address Register."]
    #[inline(always)]
    pub const fn fibaddrr(self) -> crate::common::Reg<regs::Fibaddrr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "Interrupt Config Register."]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "Interrupt Global Register."]
    #[inline(always)]
    pub const fn gisr(self) -> crate::common::Reg<regs::Gisr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "Interrupt 0 address Register."]
    #[inline(always)]
    pub const fn fifoaddrr0(self) -> crate::common::Reg<regs::Fifoaddrr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "Interrupt 1 address Register."]
    #[inline(always)]
    pub const fn fifoaddrr1(self) -> crate::common::Reg<regs::Fifoaddrr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "Interrupt 2 address Register."]
    #[inline(always)]
    pub const fn fifoaddrr2(self) -> crate::common::Reg<regs::Fifoaddrr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "Interrupt 3 address Register."]
    #[inline(always)]
    pub const fn fifoaddrr3(self) -> crate::common::Reg<regs::Fifoaddrr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "Interrupt Setting Register."]
    #[inline(always)]
    pub const fn ienr1(self) -> crate::common::Reg<regs::Ienr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "Interrupt Setting Register."]
    #[inline(always)]
    pub const fn ienr2(self) -> crate::common::Reg<regs::Ienr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "Interrupt Clear Register."]
    #[inline(always)]
    pub const fn irer1(self) -> crate::common::Reg<regs::Irer1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[doc = "Interrupt Clear Register."]
    #[inline(always)]
    pub const fn irer2(self) -> crate::common::Reg<regs::Irer2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
    }
    #[doc = "Interrupt Pending Register."]
    #[inline(always)]
    pub const fn ipsr1(self) -> crate::common::Reg<regs::Ipsr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[doc = "Interrupt Pending Register."]
    #[inline(always)]
    pub const fn ipsr2(self) -> crate::common::Reg<regs::Ipsr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0204usize) as _) }
    }
    #[doc = "Interrupt Pending Clear Register."]
    #[inline(always)]
    pub const fn iprr1(self) -> crate::common::Reg<regs::Iprr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0280usize) as _) }
    }
    #[doc = "Interrupt Pending Clear Register."]
    #[inline(always)]
    pub const fn iprr2(self) -> crate::common::Reg<regs::Iprr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0284usize) as _) }
    }
    #[doc = "Interrupt ACTIVE Register."]
    #[inline(always)]
    pub const fn iactr1(self) -> crate::common::Reg<regs::Iactr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
    }
    #[doc = "Interrupt ACTIVE Register."]
    #[inline(always)]
    pub const fn iactr2(self) -> crate::common::Reg<regs::Iactr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
    }
    #[doc = "System Control Register."]
    #[inline(always)]
    pub const fn sctlr(self) -> crate::common::Reg<regs::Sctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d10usize) as _) }
    }
}
pub mod regs {
    #[doc = "Interrupt Config Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "HWSTKCTRL."]
        #[inline(always)]
        pub const fn hwstkctrl(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "HWSTKCTRL."]
        #[inline(always)]
        pub fn set_hwstkctrl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "NESTCTRL."]
        #[inline(always)]
        pub const fn nestctrl(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "NESTCTRL."]
        #[inline(always)]
        pub fn set_nestctrl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "NMISET."]
        #[inline(always)]
        pub const fn nmiset(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "NMISET."]
        #[inline(always)]
        pub fn set_nmiset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "NMIRESET."]
        #[inline(always)]
        pub const fn nmireset(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "NMIRESET."]
        #[inline(always)]
        pub fn set_nmireset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "EXCSET."]
        #[inline(always)]
        pub const fn excset(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "EXCSET."]
        #[inline(always)]
        pub fn set_excset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "EXCRESET."]
        #[inline(always)]
        pub const fn excreset(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "EXCRESET."]
        #[inline(always)]
        pub fn set_excreset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PFICRSET."]
        #[inline(always)]
        pub const fn pficrset(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PFICRSET."]
        #[inline(always)]
        pub fn set_pficrset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "SYSRESET."]
        #[inline(always)]
        pub const fn sysreset(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "SYSRESET."]
        #[inline(always)]
        pub fn set_sysreset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "KEYCODE."]
        #[inline(always)]
        pub const fn keycode(&self) -> super::vals::Keycode {
            let val = (self.0 >> 16usize) & 0xffff;
            super::vals::Keycode::from_bits(val as u16)
        }
        #[doc = "KEYCODE."]
        #[inline(always)]
        pub fn set_keycode(&mut self, val: super::vals::Keycode) {
            self.0 =
                (self.0 & !(0xffff << 16usize)) | (((val.to_bits() as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Cfgr {
        #[inline(always)]
        fn default() -> Cfgr {
            Cfgr(0)
        }
    }
    #[doc = "Interrupt Fast Address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fibaddrr(pub u32);
    impl Fibaddrr {
        #[doc = "BASEADDR."]
        #[inline(always)]
        pub const fn baseaddr(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "BASEADDR."]
        #[inline(always)]
        pub fn set_baseaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Fibaddrr {
        #[inline(always)]
        fn default() -> Fibaddrr {
            Fibaddrr(0)
        }
    }
    #[doc = "Interrupt 0 address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fifoaddrr0(pub u32);
    impl Fifoaddrr0 {
        #[doc = "OFFADDR0."]
        #[inline(always)]
        pub const fn offaddr0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "OFFADDR0."]
        #[inline(always)]
        pub fn set_offaddr0(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
        #[doc = "IRQID0."]
        #[inline(always)]
        pub const fn irqid0(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "IRQID0."]
        #[inline(always)]
        pub fn set_irqid0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Fifoaddrr0 {
        #[inline(always)]
        fn default() -> Fifoaddrr0 {
            Fifoaddrr0(0)
        }
    }
    #[doc = "Interrupt 1 address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fifoaddrr1(pub u32);
    impl Fifoaddrr1 {
        #[doc = "OFFADDR1."]
        #[inline(always)]
        pub const fn offaddr1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "OFFADDR1."]
        #[inline(always)]
        pub fn set_offaddr1(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
        #[doc = "IRQID1."]
        #[inline(always)]
        pub const fn irqid1(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "IRQID1."]
        #[inline(always)]
        pub fn set_irqid1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Fifoaddrr1 {
        #[inline(always)]
        fn default() -> Fifoaddrr1 {
            Fifoaddrr1(0)
        }
    }
    #[doc = "Interrupt 2 address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fifoaddrr2(pub u32);
    impl Fifoaddrr2 {
        #[doc = "OFFADDR2."]
        #[inline(always)]
        pub const fn offaddr2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "OFFADDR2."]
        #[inline(always)]
        pub fn set_offaddr2(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
        #[doc = "IRQID2."]
        #[inline(always)]
        pub const fn irqid2(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "IRQID2."]
        #[inline(always)]
        pub fn set_irqid2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Fifoaddrr2 {
        #[inline(always)]
        fn default() -> Fifoaddrr2 {
            Fifoaddrr2(0)
        }
    }
    #[doc = "Interrupt 3 address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fifoaddrr3(pub u32);
    impl Fifoaddrr3 {
        #[doc = "OFFADDR3."]
        #[inline(always)]
        pub const fn offaddr3(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "OFFADDR3."]
        #[inline(always)]
        pub fn set_offaddr3(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
        #[doc = "IRQID3."]
        #[inline(always)]
        pub const fn irqid3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "IRQID3."]
        #[inline(always)]
        pub fn set_irqid3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Fifoaddrr3 {
        #[inline(always)]
        fn default() -> Fifoaddrr3 {
            Fifoaddrr3(0)
        }
    }
    #[doc = "Interrupt Global Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gisr(pub u32);
    impl Gisr {
        #[doc = "NESTSTA."]
        #[inline(always)]
        pub const fn neststa(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "NESTSTA."]
        #[inline(always)]
        pub fn set_neststa(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "GACTSTA."]
        #[inline(always)]
        pub const fn gactsta(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "GACTSTA."]
        #[inline(always)]
        pub fn set_gactsta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "GPENDSTA."]
        #[inline(always)]
        pub const fn gpendsta(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "GPENDSTA."]
        #[inline(always)]
        pub fn set_gpendsta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Gisr {
        #[inline(always)]
        fn default() -> Gisr {
            Gisr(0)
        }
    }
    #[doc = "Interrupt ACTIVE Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iactr1(pub u32);
    impl Iactr1 {
        #[doc = "IACTS."]
        #[inline(always)]
        pub const fn iacts(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "IACTS."]
        #[inline(always)]
        pub fn set_iacts(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for Iactr1 {
        #[inline(always)]
        fn default() -> Iactr1 {
            Iactr1(0)
        }
    }
    #[doc = "Interrupt ACTIVE Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iactr2(pub u32);
    impl Iactr2 {
        #[doc = "IACTS."]
        #[inline(always)]
        pub const fn iacts(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0fff_ffff;
            val as u32
        }
        #[doc = "IACTS."]
        #[inline(always)]
        pub fn set_iacts(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
        }
    }
    impl Default for Iactr2 {
        #[inline(always)]
        fn default() -> Iactr2 {
            Iactr2(0)
        }
    }
    #[doc = "Interrupt Setting Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ienr1(pub u32);
    impl Ienr1 {
        #[doc = "INTEN."]
        #[inline(always)]
        pub const fn inten(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "INTEN."]
        #[inline(always)]
        pub fn set_inten(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for Ienr1 {
        #[inline(always)]
        fn default() -> Ienr1 {
            Ienr1(0)
        }
    }
    #[doc = "Interrupt Setting Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ienr2(pub u32);
    impl Ienr2 {
        #[doc = "INTEN."]
        #[inline(always)]
        pub const fn inten(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0fff_ffff;
            val as u32
        }
        #[doc = "INTEN."]
        #[inline(always)]
        pub fn set_inten(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
        }
    }
    impl Default for Ienr2 {
        #[inline(always)]
        fn default() -> Ienr2 {
            Ienr2(0)
        }
    }
    #[doc = "Interrupt Pending Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipr1(pub u32);
    impl Ipr1 {
        #[doc = "PENDSTA."]
        #[inline(always)]
        pub const fn pendsta2_3(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "PENDSTA."]
        #[inline(always)]
        pub fn set_pendsta2_3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "PENDSTA."]
        #[inline(always)]
        pub const fn pendsta12_31(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "PENDSTA."]
        #[inline(always)]
        pub fn set_pendsta12_31(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for Ipr1 {
        #[inline(always)]
        fn default() -> Ipr1 {
            Ipr1(0)
        }
    }
    #[doc = "Interrupt Pending Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipr2(pub u32);
    impl Ipr2 {
        #[doc = "PENDSTA."]
        #[inline(always)]
        pub const fn pendsta(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0fff_ffff;
            val as u32
        }
        #[doc = "PENDSTA."]
        #[inline(always)]
        pub fn set_pendsta(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
        }
    }
    impl Default for Ipr2 {
        #[inline(always)]
        fn default() -> Ipr2 {
            Ipr2(0)
        }
    }
    #[doc = "Interrupt Pending Clear Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprr1(pub u32);
    impl Iprr1 {
        #[doc = "PENDRESET."]
        #[inline(always)]
        pub const fn pendreset2_3(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "PENDRESET."]
        #[inline(always)]
        pub fn set_pendreset2_3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "PENDRESET."]
        #[inline(always)]
        pub const fn pendreset12_31(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "PENDRESET."]
        #[inline(always)]
        pub fn set_pendreset12_31(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for Iprr1 {
        #[inline(always)]
        fn default() -> Iprr1 {
            Iprr1(0)
        }
    }
    #[doc = "Interrupt Pending Clear Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprr2(pub u32);
    impl Iprr2 {
        #[doc = "PENDRESET."]
        #[inline(always)]
        pub const fn pendreset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0fff_ffff;
            val as u32
        }
        #[doc = "PENDRESET."]
        #[inline(always)]
        pub fn set_pendreset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
        }
    }
    impl Default for Iprr2 {
        #[inline(always)]
        fn default() -> Iprr2 {
            Iprr2(0)
        }
    }
    #[doc = "Interrupt Pending Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipsr1(pub u32);
    impl Ipsr1 {
        #[doc = "PENDSET."]
        #[inline(always)]
        pub const fn pendset2_3(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "PENDSET."]
        #[inline(always)]
        pub fn set_pendset2_3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "PENDSET."]
        #[inline(always)]
        pub const fn pendset12_31(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "PENDSET."]
        #[inline(always)]
        pub fn set_pendset12_31(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for Ipsr1 {
        #[inline(always)]
        fn default() -> Ipsr1 {
            Ipsr1(0)
        }
    }
    #[doc = "Interrupt Pending Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipsr2(pub u32);
    impl Ipsr2 {
        #[doc = "PENDSET."]
        #[inline(always)]
        pub const fn pendset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0fff_ffff;
            val as u32
        }
        #[doc = "PENDSET."]
        #[inline(always)]
        pub fn set_pendset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
        }
    }
    impl Default for Ipsr2 {
        #[inline(always)]
        fn default() -> Ipsr2 {
            Ipsr2(0)
        }
    }
    #[doc = "Interrupt Clear Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Irer1(pub u32);
    impl Irer1 {
        #[doc = "INTRSET."]
        #[inline(always)]
        pub const fn intrset(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "INTRSET."]
        #[inline(always)]
        pub fn set_intrset(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for Irer1 {
        #[inline(always)]
        fn default() -> Irer1 {
            Irer1(0)
        }
    }
    #[doc = "Interrupt Clear Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Irer2(pub u32);
    impl Irer2 {
        #[doc = "INTRSET."]
        #[inline(always)]
        pub const fn intrset(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0fff_ffff;
            val as u32
        }
        #[doc = "INTRSET."]
        #[inline(always)]
        pub fn set_intrset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
        }
    }
    impl Default for Irer2 {
        #[inline(always)]
        fn default() -> Irer2 {
            Irer2(0)
        }
    }
    #[doc = "Interrupt Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr1(pub u32);
    impl Isr1 {
        #[doc = "Interrupt ID Status."]
        #[inline(always)]
        pub const fn intensta2_3(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Interrupt ID Status."]
        #[inline(always)]
        pub fn set_intensta2_3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Interrupt ID Status."]
        #[inline(always)]
        pub const fn intensta12_31(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Interrupt ID Status."]
        #[inline(always)]
        pub fn set_intensta12_31(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for Isr1 {
        #[inline(always)]
        fn default() -> Isr1 {
            Isr1(0)
        }
    }
    #[doc = "Interrupt Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr2(pub u32);
    impl Isr2 {
        #[doc = "Interrupt ID Status."]
        #[inline(always)]
        pub const fn intensta(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0fff_ffff;
            val as u32
        }
        #[doc = "Interrupt ID Status."]
        #[inline(always)]
        pub fn set_intensta(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
        }
    }
    impl Default for Isr2 {
        #[inline(always)]
        fn default() -> Isr2 {
            Isr2(0)
        }
    }
    #[doc = "Interrupt Priority Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ithresdr(pub u32);
    impl Ithresdr {
        #[doc = "THRESHOLD."]
        #[inline(always)]
        pub const fn threshold(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "THRESHOLD."]
        #[inline(always)]
        pub fn set_threshold(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Ithresdr {
        #[inline(always)]
        fn default() -> Ithresdr {
            Ithresdr(0)
        }
    }
    #[doc = "System Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sctlr(pub u32);
    impl Sctlr {
        #[doc = "SLEEPONEXIT."]
        #[inline(always)]
        pub const fn sleeponexit(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SLEEPONEXIT."]
        #[inline(always)]
        pub fn set_sleeponexit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SLEEPDEEP."]
        #[inline(always)]
        pub const fn sleepdeep(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SLEEPDEEP."]
        #[inline(always)]
        pub fn set_sleepdeep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "WFITOWFE."]
        #[inline(always)]
        pub const fn wfitowfe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "WFITOWFE."]
        #[inline(always)]
        pub fn set_wfitowfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SEVONPEND."]
        #[inline(always)]
        pub const fn sevonpend(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "SEVONPEND."]
        #[inline(always)]
        pub fn set_sevonpend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SETEVENT."]
        #[inline(always)]
        pub const fn setevent(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SETEVENT."]
        #[inline(always)]
        pub fn set_setevent(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Sctlr {
        #[inline(always)]
        fn default() -> Sctlr {
            Sctlr(0)
        }
    }
}
pub mod vals {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Keycode(pub u16);
    impl Keycode {
        #[doc = "NMI and EXC key."]
        pub const KEY2: Self = Self(0xbcaf);
        #[doc = "System Reset key."]
        pub const KEY3: Self = Self(0xbeef);
        #[doc = "HWSTK and NEST key."]
        pub const KEY1: Self = Self(0xfa05);
    }
    impl Keycode {
        pub const fn from_bits(val: u16) -> Keycode {
            Self(val & 0xffff)
        }
        pub const fn to_bits(self) -> u16 {
            self.0
        }
    }
    impl From<u16> for Keycode {
        #[inline(always)]
        fn from(val: u16) -> Keycode {
            Keycode::from_bits(val)
        }
    }
    impl From<Keycode> for u16 {
        #[inline(always)]
        fn from(val: Keycode) -> u16 {
            Keycode::to_bits(val)
        }
    }
}
