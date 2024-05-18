use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Extend",
        extends: None,
        description: Some("EXTEND configuration. (EXTEN)"),
        items: &[BlockItem {
            name: "ctr",
            description: Some("EXTEN control register."),
            array: None,
            byte_offset: 0x0,
            inner: BlockItemInner::Register(Register {
                access: Access::ReadWrite,
                bit_size: 32,
                fieldset: Some("Ctr"),
            }),
        }],
    }],
    fieldsets: &[FieldSet {
        name: "Ctr",
        extends: None,
        description: Some("EXTEN control register."),
        bit_size: 32,
        fields: &[
            Field {
                name: "hsipre",
                description: Some("Whether HSI is divided."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 4 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "lkupen",
                description: Some("LOCKUP_Eable."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 6 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "lkuprst",
                description: Some("LOCKUP RESET."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 7 }),
                bit_size: 1,
                array: None,
                enumm: None,
            },
            Field {
                name: "ulldotrim",
                description: Some("ULLDOTRIM."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 8 }),
                bit_size: 3,
                array: None,
                enumm: None,
            },
            Field {
                name: "ldotrim",
                description: Some("LDOTRIM."),
                bit_offset: BitOffset::Regular(RegularBitOffset { offset: 12 }),
                bit_size: 2,
                array: None,
                enumm: None,
            },
        ],
    }],
    enums: &[],
};
