use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Systick",
        extends: None,
        description: Some("Systick registers, 64-bit downcounter for CH32V103."),
        items: &[
            BlockItem {
                name: "ctlr",
                description: Some("SysTick Control register."),
                array: None,
                byte_offset: 0x0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ctlr"),
                }),
            },
            BlockItem {
                name: "cnt",
                description: Some("System counter register."),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 64,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "cntl",
                description: Some("SysTick counter low bits."),
                array: None,
                byte_offset: 0x4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "cnth",
                description: Some("SysTick counter high bits."),
                array: None,
                byte_offset: 0x8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "cmp",
                description: Some("System count compare register."),
                array: None,
                byte_offset: 0xc,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 64,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "cmpl",
                description: Some("SysTick compare low bits."),
                array: None,
                byte_offset: 0xc,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
            BlockItem {
                name: "cmph",
                description: Some("SysTick compare high bits."),
                array: None,
                byte_offset: 0x10,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: None,
                }),
            },
        ],
    }],
    fieldsets: &[FieldSet {
        name: "Ctlr",
        extends: None,
        description: Some("SysTick Control register."),
        bit_size: 32,
        fields: &[Field {
            name: "ste",
            description: Some("SysTick enable state. HCLK/8"),
            bit_offset: BitOffset::Regular(RegularBitOffset { offset: 0 }),
            bit_size: 1,
            array: None,
            enumm: None,
        }],
    }],
    enums: &[],
};
