use super::*;

#[test]
fn test_0xa9_lda_immidiate_load_data() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x05, 0x00]);

    assert_eq!(cpu.register_a, 0x05);
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_lda_from_memory() {
    let mut cpu = CPU::new();
    cpu.mem_write(0x10, 0x55);
    cpu.load_and_run(vec![0xa5, 0x10, 0x00]);

    assert_eq!(cpu.register_a, 0x55);
}

#[test]
fn test_sta() {
    let mut cpu: CPU = CPU::new();
    cpu.load(vec![0x85, 0x10, 0x00]);
    cpu.reset();
    cpu.register_a = 4;
    cpu.run();

    assert_eq!(cpu.memory[0x10], 4);
}

#[test]
fn test_0xa9_lda_zero_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x00, 0x00]);

    assert!(cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_0xaa_tax_move_a_to_x() {
    let mut cpu = CPU::new();

    cpu.load(vec![0xaa, 0x00]);
    cpu.reset();
    cpu.register_a = 10;
    cpu.run();

    assert_eq!(cpu.register_x, 10);
}

#[test]
fn test_5_ops_working_together() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

    assert_eq!(cpu.register_x, 0xc1)
}

#[test]
fn test_inx_overflow() {
    let mut cpu = CPU::new();

    cpu.load(vec![0xe8, 0xe8, 0x00]);
    cpu.reset();
    cpu.register_x = 0xff;
    cpu.run();

    assert_eq!(cpu.register_x, 1)
}

#[test]
fn test_reset() {
    let mut cpu = CPU::new();
    cpu.register_x = 0xff;
    cpu.register_a = 0xff;
    cpu.register_y = 0xff;
    cpu.register_s = 0x00;
    cpu.status = StatusFlags::from_bits_truncate(0xFF);
    cpu.reset();

    assert_eq!(cpu.register_x, 0);
    assert_eq!(cpu.register_a, 0);
    assert_eq!(cpu.register_y, 0);
    assert_eq!(cpu.register_s, 0xff);
    assert_eq!(cpu.status.bits(), 0);
}

#[test]
fn test_tya_move_y_to_a() {
    let mut cpu = CPU::new();
    cpu.load(vec![0x98, 0x00]);
    cpu.reset();
    cpu.register_y = 15;
    cpu.run();

    assert_eq!(cpu.register_a, 15);
}

#[test]
fn test_txs_move_x_to_s() {
    let mut cpu = CPU::new();
    cpu.load(vec![0x9A, 0x00]);
    cpu.reset();
    cpu.register_x = 69;
    cpu.run();

    assert_eq!(cpu.register_s, 69);
}

#[test]
fn test_txa_move_x_to_a() {
    let mut cpu = CPU::new();
    cpu.load(vec![0x8A, 0x00]);
    cpu.reset();
    cpu.register_x = 37;
    cpu.run();

    assert_eq!(cpu.register_a, 37);
}

#[test]
fn test_txa_zero_flag() {
    let mut cpu = CPU::new();
    cpu.load(vec![0x8A, 0x00]);
    cpu.reset();
    cpu.register_x = 0;
    cpu.run();

    assert!(cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_txa_negative_flag() {
    let mut cpu = CPU::new();
    cpu.load(vec![0x8A, 0x00]);
    cpu.reset();
    cpu.register_x = 0b1000_0010;
    cpu.run();

    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_tsx_move_s_to_x() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xBA, 0x00]);

    assert_eq!(cpu.register_x, 0xff);
}

#[test]
fn test_tsx_zero_flag() {
    let mut cpu = CPU::new();
    cpu.load(vec![0xBA, 0x00]);
    cpu.reset();
    cpu.register_s = 0;
    cpu.run();

    assert!(cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_tsx_negative_flag() {
    let mut cpu = CPU::new();
    cpu.load(vec![0xBA, 0x00]);
    cpu.reset();
    cpu.register_s = 0b1000_0010;
    cpu.run();

    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_tay_move_a_to_y() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA9, 0x13, 0xA8, 0x00]);

    assert_eq!(cpu.register_y, 0x13);
}

#[test]
fn test_tay_zero_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA9, 0x00, 0xA8, 0x00]);

    assert!(cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_tay_negative_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA9, 0b1000_0010, 0xA8, 0x00]);

    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_ldx_load_immediate() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA2, 0xCC, 0x00]);

    assert_eq!(cpu.register_x, 0xCC);
}

#[test]
fn test_ldx_load_zero_page() {
    let mut cpu = CPU::new();
    cpu.load(vec![0xA6, 0x10, 0x00]);
    cpu.reset();
    cpu.memory[0x10] = 0x33;
    cpu.run();

    assert_eq!(cpu.register_x, 0x33);
}

#[test]
fn test_ldx_load_zero_page_y() {
    let mut cpu = CPU::new();
    cpu.load(vec![0xB6, 0x11, 0x00]);
    cpu.reset();
    cpu.register_y = 1;
    cpu.memory[0x12] = 0x44;
    cpu.run();

    assert_eq!(cpu.register_x, 0x44);
}

#[test]
fn test_ldx_load_absolute() {
    let mut cpu = CPU::new();
    cpu.load(vec![0xAE, 0x10, 0x22, 0x00]);
    cpu.reset();
    cpu.memory[0x2210] = 0x99;
    cpu.run();

    assert_eq!(cpu.register_x, 0x99);

}

#[test]
fn test_ldx_load_absolute_y() {
    let mut cpu = CPU::new();
    cpu.load(vec![0xBE, 0x10, 0x22, 0x00]);
    cpu.reset();
    cpu.register_y = 4;
    cpu.memory[0x2214] = 0x88;
    cpu.run();

    assert_eq!(cpu.register_x, 0x88);

}

#[test]
fn test_ldx_zero_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa2, 0x00, 0x00]);

    assert!(cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_ldx_negative_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA2, 0b1011_0010, 0x00]);

    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_ldy_load_immediate() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA0, 0xDE, 0x00]);

    assert_eq!(cpu.register_y, 0xDE);
}

#[test]
fn test_ldy_load_zero_page() {
    let mut cpu = CPU::new();
    cpu.load(vec![0xA4, 0x69, 0x00]);
    cpu.reset();
    cpu.memory[0x69] = 0xBB;
    cpu.run();

    assert_eq!(cpu.register_y, 0xBB);
}

#[test]
fn test_ldy_load_zero_page_x() {
    let mut cpu = CPU::new();
    cpu.load(vec![0xB4, 0x11, 0x00]);
    cpu.reset();
    cpu.register_x = 2;
    cpu.memory[0x13] = 0x47;
    cpu.run();

    assert_eq!(cpu.register_y, 0x47);
}

#[test]
fn test_ldy_load_absolute() {
    let mut cpu = CPU::new();
    cpu.load(vec![0xAC, 0x9A, 0x44, 0x00]);
    cpu.reset();
    cpu.memory[0x449A] = 0x12;
    cpu.run();

    assert_eq!(cpu.register_y, 0x12);

}

#[test]
fn test_ldy_load_absolute_x() {
    let mut cpu = CPU::new();
    cpu.load(vec![0xBC, 0x35, 0x20, 0x00]);
    cpu.reset();
    cpu.register_y = 0xA2;
    cpu.memory[0x2035] = 0x87;
    cpu.run();

    assert_eq!(cpu.register_y, 0x87);

}

#[test]
fn test_ldy_zero_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa0, 0x00, 0x00]);

    assert!(cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_ldy_negative_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA2, 0b1001_0111, 0x00]);

    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_sty_zero_page() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA0, 0x11, 0x84, 0xBD, 0x00]);

    assert_eq!(cpu.memory[0xBD], 0x11);
}

#[test]
fn test_sty_zero_page_x() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA2, 0x0A, 0xA0, 0x77, 0x94, 0x30, 0x00]);

    assert_eq!(cpu.memory[0x3A], 0x77);
}

#[test]
fn test_sty_absolute() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA0, 0xCD, 0x8C, 0x34, 0x12, 0x00]);

    assert_eq!(cpu.memory[0x1234], 0xCD);
}


#[test]
fn test_stx_zero_page() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA2, 0xDD, 0x86, 0xB0, 0x00]);

    assert_eq!(cpu.memory[0xB0], 0xDD);
}

#[test]
fn test_stx_zero_page_y() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA0, 0x0B, 0xA2, 0x78, 0x96, 0x30, 0x00]);

    assert_eq!(cpu.memory[0x3B], 0x78);
}

#[test]
fn test_stx_absolute() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA2, 0xCE, 0x8E, 0x35, 0x13, 0x00]);

    assert_eq!(cpu.memory[0x1335], 0xCE);
}

#[test]
fn test_sei_interrupt_disable() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0x78, 0x00]);

    assert!(cpu.status.contains(StatusFlags::INTERRUPT_DISABLE));
}

#[test]
fn test_sed_decimal_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xF8, 0x00]);

    assert!(cpu.status.contains(StatusFlags::DECIMAL_MODE));
}

#[test]
fn test_sec_carry_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0x38, 0x00]);

    assert!(cpu.status.contains(StatusFlags::CARRY));
}

#[test]
fn test_and_immediate() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA9, 0b0111_1111, 0x29, 0b1010_0101, 0x00]);

    assert_eq!(cpu.register_a, 0b0010_0101);
}

#[test]
fn test_and_indirect_x() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0b0000_0010, // LDA #$02
        0x85, 0xFF, // STA $FF
        0xA9, 0xFF, // LDA #$FF
        0x85, 0x04, // STA $04
        0xA2, 0x04, // LDX $04
        0xA9, 0xFF, // LDA #$FF
        0x21, 0x00, // AND ($00, x)
    ]);

    assert_eq!(cpu.register_a, 0b0000_0010);
}

#[test]
fn test_and_indirect_y() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0b0000_0100, // LDA #$04
        0x85, 0xFF, // STA $FF
        0xA9, 0xF0, // LDA #$F0
        0x85, 0x00, // STA $00
        0xA9, 0x00, // LDA #$00
        0x85, 0x01, // STA $01
        0xA0, 0x0F, // LDY #$0F
        0xA9, 0xFF, // LDA #$FF
        0x31, 0x00, // AND ($00), y
    ]);

    assert_eq!(cpu.register_a, 0b0000_0100);
}

#[test]
fn test_asl_carry_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0b1010_0000,
        0x0A,
    ]);

    assert!(cpu.status.contains(StatusFlags::CARRY));
}

#[test]
fn test_asl_zero_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0b1000_0000,
        0x0A,
    ]);

    assert!(cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_asl_negative_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0b0100_0000,
        0x0A,
    ]);

    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_asl_implied() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0b0000_1011,
        0x0A,
    ]);

    assert_eq!(cpu.register_a, 0b0001_0110);
}

#[test]
fn test_asl_zero_page() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0b0101_1010,
        0x85, 0x10,
        0x06, 0x10,
    ]);

    assert_eq!(cpu.memory[0x10], 0b1011_0100);
}

#[test]
fn test_asl_zero_page_x() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0b0010_1101,
        0x85, 0x06,
        0xA2, 0x04,
        0x16, 0x02,
    ]);

    assert_eq!(cpu.memory[0x06], 0b0101_1010);
}

#[test]
fn test_asl_absolute() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0b0011_0001,
        0x8D, 0xAB, 0xCD,
        0x0E, 0xAB, 0xCD,
    ]);

    assert_eq!(cpu.memory[0xCDAB], 0b0110_0010);
}

#[test]
fn test_asl_absolute_x() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0b0011_0001,
        0x8D, 0x34, 0x30,
        0xA2, 0x04,
        0x1E, 0x30, 0x30,
    ]);

    assert_eq!(cpu.memory[0x3034], 0b0110_0010);
}

#[test]
fn test_and_negative_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA9, 0b1101_1010, 0x29, 0b1101_1010, 0x00]);

    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_and_zero_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xA9, 0b0101_1010, 0x29, 0b1010_0101, 0x00]);

    assert!(cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_bit_zero_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0b1010_1010,
        0x85, 0x10,
        0xA9, 0b0101_0101,
        0x24, 0x10
    ]);

    assert!(cpu.status.contains(StatusFlags::ZERO));

    cpu.load_and_run(vec![
        0xA9, 0b1011_1010,
        0x85, 0x10,
        0xA9, 0b0101_0101,
        0x24, 0x10
    ]);

    assert!(!cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_bit_overflow_negative_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0b1010_1010,
        0x85, 0x10,
        0xA9, 0b0101_0101,
        0x24, 0x10
    ]);

    let mem = cpu.mem_read(0x0010);
    let b6 = mem & 0b0100_0000;
    let b7 = mem & 0b1000_0000;

    assert_eq!(cpu.status.bits & 0b0100_0000, b6);
    assert_eq!(cpu.status.bits & 0b1000_0000, b7);
}

#[test]
fn test_clc_carry_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xA9, 0b1000_0000,  // LDA
        0x0A,               // ASL
        0x18,               // CLC
    ]);

    assert!(!cpu.status.contains(StatusFlags::CARRY));
}

#[test]
fn test_cld_decimal_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0xF8,   // SED
        0xD8,   // CLD
    ]);

    assert!(!cpu.status.contains(StatusFlags::DECIMAL_MODE));
}

#[test]
fn test_cli_interrupt_flag() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![
        0x78,   // SEI
        0x58,   // CLI
    ]);

    assert!(!cpu.status.contains(StatusFlags::INTERRUPT_DISABLE));
}
