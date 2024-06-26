use super::*;

use crate::bus::Bus;
use crate::cpu::{STACK_RESET, STATUS_RESET};
use crate::rom::Rom;

fn new_cpu() -> CPU {
    let rom = Rom::blank();
    let mut bus = Bus::new(rom);
    bus.allow_rom_writes = true;

    CPU::new(bus)
}

#[test]
fn test_0xa9_lda_immidiate_load_data() {
    let mut cpu = new_cpu();
    cpu.load(vec![0xa9, 0x05, 0x00]);
    cpu.reset();
    cpu.run();

    assert_eq!(cpu.register_a, 0x05);
    assert!(!cpu.status.contains(StatusFlags::ZERO));
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_lda_from_memory() {
    let mut cpu = new_cpu();
    cpu.mem_write(0x10, 0x55);
    cpu.load_and_run(vec![0xa5, 0x10, 0x00]);

    assert_eq!(cpu.register_a, 0x55);
}

#[test]
fn test_sta() {
    let mut cpu: CPU = new_cpu();
    cpu.load(vec![0x85, 0x10, 0x00]);
    cpu.reset();
    cpu.register_a = 4;
    cpu.run();

    assert_eq!(cpu.mem_read(0x10), 4);
}

#[test]
fn test_0xa9_lda_zero_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![0xa9, 0x00, 0x00]);

    assert!(cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_0xaa_tax_move_a_to_x() {
    let mut cpu = new_cpu();

    cpu.load(vec![0xaa, 0x00]);
    cpu.reset();
    cpu.register_a = 10;
    cpu.run();

    assert_eq!(cpu.register_x, 10);
}

#[test]
fn test_5_ops_working_together() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

    assert_eq!(cpu.register_x, 0xc1)
}

#[test]
fn test_reset() {
    let mut cpu = new_cpu();
    cpu.register_x = 0xff;
    cpu.register_a = 0xff;
    cpu.register_y = 0xff;
    cpu.stack_pointer = 0x00;
    cpu.status = StatusFlags::from_bits_truncate(0xFF);
    cpu.reset();

    assert_eq!(cpu.register_x, 0);
    assert_eq!(cpu.register_a, 0);
    assert_eq!(cpu.register_y, 0);
    assert_eq!(cpu.stack_pointer, STACK_RESET);
    assert_eq!(cpu.status.bits(), STATUS_RESET.bits());
}

#[test]
fn test_tya_move_y_to_a() {
    let mut cpu = new_cpu();
    cpu.load(vec![0x98, 0x00]);
    cpu.reset();
    cpu.register_y = 15;
    cpu.run();

    assert_eq!(cpu.register_a, 15);
}

#[test]
fn test_txs_move_x_to_s() {
    let mut cpu = new_cpu();
    cpu.load(vec![0x9A, 0x00]);
    cpu.reset();
    cpu.register_x = 69;
    cpu.run();

    assert_eq!(cpu.stack_pointer, 69);
}

#[test]
fn test_txa_move_x_to_a() {
    let mut cpu = new_cpu();
    cpu.load(vec![0x8A, 0x00]);
    cpu.reset();
    cpu.register_x = 37;
    cpu.run();

    assert_eq!(cpu.register_a, 37);
}

#[test]
fn test_txa_zero_flag() {
    let mut cpu = new_cpu();
    cpu.load(vec![0x8A, 0x00]);
    cpu.reset();
    cpu.register_x = 0;
    cpu.run();

    assert!(cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_txa_negative_flag() {
    let mut cpu = new_cpu();
    cpu.load(vec![0x8A, 0x00]);
    cpu.reset();
    cpu.register_x = 0b1000_0010;
    cpu.run();

    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_tsx_move_s_to_x() {
    let mut cpu = new_cpu();
    cpu.load(vec![0xBA, 0x00]);
    cpu.reset();
    cpu.run();

    assert_eq!(cpu.register_x, STACK_RESET);
}

#[test]
fn test_tsx_zero_flag() {
    let mut cpu = new_cpu();
    cpu.load(vec![0xBA, 0x00]);
    cpu.reset();
    cpu.stack_pointer = 0;
    cpu.run();

    assert!(cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_tsx_negative_flag() {
    let mut cpu = new_cpu();
    cpu.load(vec![0xBA, 0x00]);
    cpu.reset();
    cpu.stack_pointer = 0b1000_0010;
    cpu.run();

    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_tay_move_a_to_y() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![0xA9, 0x13, 0xA8, 0x00]);

    assert_eq!(cpu.register_y, 0x13);
}

#[test]
fn test_tay_zero_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![0xA9, 0x00, 0xA8, 0x00]);

    assert!(cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_tay_negative_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![0xA9, 0b1000_0010, 0xA8, 0x00]);

    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_ldx_load_immediate() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![0xA2, 0xCC, 0x00]);

    assert_eq!(cpu.register_x, 0xCC);
}

#[test]
fn test_ldx_load_zero_page() {
    let mut cpu = new_cpu();
    cpu.load(vec![0xA6, 0x10, 0x00]);
    cpu.reset();
    cpu.mem_write(0x10, 0x33);
    cpu.run();

    assert_eq!(cpu.register_x, 0x33);
}

#[test]
fn test_ldx_load_zero_page_y() {
    let mut cpu = new_cpu();
    cpu.load(vec![0xB6, 0x11, 0x00]);
    cpu.reset();
    cpu.register_y = 1;
    cpu.mem_write(0x12, 0x44);
    cpu.run();

    assert_eq!(cpu.register_x, 0x44);
}

#[test]
fn test_ldx_load_absolute() {
    let mut cpu = new_cpu();
    cpu.load(vec![0xAE, 0x10, 0x02, 0x00]);
    cpu.reset();
    cpu.mem_write(0x0210, 0x99);
    cpu.run();

    assert_eq!(cpu.register_x, 0x99);

}

#[test]
fn test_ldx_load_absolute_y() {
    let mut cpu = new_cpu();
    cpu.load(vec![0xBE, 0x10, 0x02, 0x00]);
    cpu.reset();
    cpu.register_y = 4;
    cpu.mem_write(0x0214, 0x88);
    cpu.run();

    assert_eq!(cpu.register_x, 0x88);

}

#[test]
fn test_ldx_zero_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![0xa2, 0x00, 0x00]);

    assert!(cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_ldx_negative_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![0xA2, 0b1011_0010, 0x00]);

    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_ldy_load_immediate() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![0xA0, 0xDE, 0x00]);

    assert_eq!(cpu.register_y, 0xDE);
}

#[test]
fn test_ldy_load_zero_page() {
    let mut cpu = new_cpu();
    cpu.load(vec![0xA4, 0x69, 0x00]);
    cpu.reset();
    cpu.mem_write(0x69, 0xBB);
    cpu.run();

    assert_eq!(cpu.register_y, 0xBB);
}

#[test]
fn test_ldy_load_zero_page_x() {
    let mut cpu = new_cpu();
    cpu.load(vec![0xB4, 0x11, 0x00]);
    cpu.reset();
    cpu.register_x = 2;
    cpu.mem_write(0x13, 0x47);
    cpu.run();

    assert_eq!(cpu.register_y, 0x47);
}

#[test]
fn test_ldy_load_absolute() {
    let mut cpu = new_cpu();
    cpu.load(vec![0xAC, 0x9A, 0x04, 0x00]);
    cpu.reset();
    cpu.mem_write(0x049A, 0x12);
    cpu.run();

    assert_eq!(cpu.register_y, 0x12);

}

#[test]
fn test_ldy_load_absolute_x() {
    let mut cpu = new_cpu();
    cpu.load(vec![0xBC, 0x35, 0x02, 0x00]);
    cpu.reset();
    cpu.register_y = 0xA2;
    cpu.mem_write(0x0235, 0x87);
    cpu.run();

    assert_eq!(cpu.register_y, 0x87);

}

#[test]
fn test_ldy_zero_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![0xa0, 0x00, 0x00]);

    assert!(cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_ldy_negative_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![0xA2, 0b1001_0111, 0x00]);

    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_sty_zero_page() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![0xA0, 0x11, 0x84, 0xBD, 0x00]);

    assert_eq!(cpu.mem_read(0xBD), 0x11);
}

#[test]
fn test_sty_zero_page_x() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![0xA2, 0x0A, 0xA0, 0x77, 0x94, 0x30, 0x00]);

    assert_eq!(cpu.mem_read(0x3A), 0x77);
}

#[test]
fn test_sty_absolute() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![0xA0, 0xCD, 0x8C, 0x34, 0x12, 0x00]);

    assert_eq!(cpu.mem_read(0x1234), 0xCD);
}


#[test]
fn test_stx_zero_page() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![0xA2, 0xDD, 0x86, 0xB0, 0x00]);

    assert_eq!(cpu.mem_read(0xB0), 0xDD);
}

#[test]
fn test_stx_zero_page_y() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![0xA0, 0x0B, 0xA2, 0x78, 0x96, 0x01, 0x00]);

    assert_eq!(cpu.mem_read(0x000C), 0x78);
}

#[test]
fn test_stx_absolute() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![0xA2, 0xCE, 0x8E, 0x35, 0x13, 0x00]);

    assert_eq!(cpu.mem_read(0x1335), 0xCE);
}

#[test]
fn test_sei_interrupt_disable() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![0x78, 0x00]);

    assert!(cpu.status.contains(StatusFlags::INTERRUPT_DISABLE));
}

#[test]
fn test_sed_decimal_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![0xF8, 0x00]);

    assert!(cpu.status.contains(StatusFlags::DECIMAL_MODE));
}

#[test]
fn test_sec_carry_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![0x38, 0x00]);

    assert!(cpu.status.contains(StatusFlags::CARRY));
}

#[test]
fn test_and_immediate() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![0xA9, 0b0111_1111, 0x29, 0b1010_0101, 0x00]);

    assert_eq!(cpu.register_a, 0b0010_0101);
}

#[test]
fn test_and_indirect_x() {
    let mut cpu = new_cpu();
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
    let mut cpu = new_cpu();
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
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b1010_0000,
        0x0A,
    ]);

    assert!(cpu.status.contains(StatusFlags::CARRY));
}

#[test]
fn test_asl_zero_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b1000_0000,
        0x0A,
    ]);

    assert!(cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_asl_negative_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b0100_0000,
        0x0A,
    ]);

    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_asl_implied() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b0000_1011,
        0x0A,
    ]);

    assert_eq!(cpu.register_a, 0b0001_0110);
}

#[test]
fn test_asl_zero_page() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b0101_1010,
        0x85, 0x10,
        0x06, 0x10,
    ]);

    assert_eq!(cpu.mem_read(0x10), 0b1011_0100);
}

#[test]
fn test_asl_zero_page_x() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b0010_1101,
        0x85, 0x06,
        0xA2, 0x04,
        0x16, 0x02,
    ]);

    assert_eq!(cpu.mem_read(0x06), 0b0101_1010);
}

#[test]
fn test_asl_absolute() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b0011_0001,
        0x8D, 0xAB, 0xCD,
        0x0E, 0xAB, 0xCD,
    ]);

    assert_eq!(cpu.mem_read(0xCDAB), 0b0110_0010);
}

#[test]
fn test_asl_absolute_x() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b0011_0001,
        0x8D, 0x34, 0x10,
        0xA2, 0x04,
        0x1E, 0x30, 0x10,
    ]);

    assert_eq!(cpu.mem_read(0x1034), 0b0110_0010);
}

#[test]
fn test_and_negative_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![0xA9, 0b1101_1010, 0x29, 0b1101_1010, 0x00]);

    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_and_zero_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![0xA9, 0b0101_1010, 0x29, 0b1010_0101, 0x00]);

    assert!(cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_bit_zero_flag() {
    let mut cpu = new_cpu();
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
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b1010_1010,
        0x85, 0x10,
        0xA9, 0b0101_0101,
        0x24, 0x10
    ]);

    let mem = cpu.mem_read(0x0010);
    let b6 = mem & 0b0100_0000;
    let b7 = mem & 0b1000_0000;

    assert_eq!(cpu.status.bits() & 0b0100_0000, b6);
    assert_eq!(cpu.status.bits() & 0b1000_0000, b7);
}

#[test]
fn test_clc_carry_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b1000_0000,  // LDA
        0x0A,               // ASL
        0x18,               // CLC
    ]);

    assert!(!cpu.status.contains(StatusFlags::CARRY));
}

#[test]
fn test_cld_decimal_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xF8,   // SED
        0xD8,   // CLD
    ]);

    assert!(!cpu.status.contains(StatusFlags::DECIMAL_MODE));
}

#[test]
fn test_cli_interrupt_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0x78,   // SEI
        0x58,   // CLI
    ]);

    assert!(!cpu.status.contains(StatusFlags::INTERRUPT_DISABLE));
}

#[test]
fn test_clv_overflow_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b1111_0000,
        0x85, 0x10,
        0xA9, 0b0000_0000,
        0x24, 0x10,
        0xB8,
    ]);

    assert!(!cpu.status.contains(StatusFlags::OVERFLOW));
}

#[test]
fn test_cmp_negative_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b1111_1111,
        0xC9, 0b0100_0000,
    ]);

    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cmp_carry_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b0101_0101,
        0x85, 0x09,
        0xA9, 0b0111_0101,
        0xC5, 0x09,
    ]);

    assert!(cpu.status.contains(StatusFlags::CARRY));
}

#[test]
fn test_cmp_zero_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b0101_0101,
        0xC9, 0b0101_0101,
    ]);

    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert!(cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_cpx_zero_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA2, 0x45,
        0xE0, 0x45,
    ]);

    assert!(cpu.status.contains(StatusFlags::ZERO));
    assert!(cpu.status.contains(StatusFlags::CARRY));
}

#[test]
fn test_cpx_carry_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0x68,
        0x85, 0x04,
        0xA2, 0x69,
        0xE4, 0x04,
    ]);

    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_cpx_negative_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b0100_0000,
        0x8D, 0x12, 0x03,
        0xA2, 0b1100_0000,
        0xEC, 0x12, 0x03,
    ]);

    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_cpy_zero_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA0, 0x45,
        0xC0, 0x45,
    ]);

    assert!(cpu.status.contains(StatusFlags::ZERO));
    assert!(cpu.status.contains(StatusFlags::CARRY));
}

#[test]
fn test_cpy_carry_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0x68,
        0x85, 0x04,
        0xA0, 0x69,
        0xC4, 0x04,
    ]);

    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert!(!cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_cpy_negative_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b0100_0000,
        0x8D, 0x12, 0x04,
        0xA0, 0b1100_0000,
        0xCC, 0x12, 0x04,
    ]);

    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_lsr_carry_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b1010_0001,
        0x4A,
    ]);

    assert!(cpu.status.contains(StatusFlags::CARRY));
}

#[test]
fn test_lsr_zero_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b0000_0001,
        0x4A,
    ]);

    assert!(cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_lsr_negative_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b0100_0000,
        0x4A,
    ]);

    // Bit 7 should always be zero when doing LSR
    assert!(!cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_lsr_implied() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b0000_1011,
        0x4A,
    ]);

    assert_eq!(cpu.register_a, 0b0000_0101);
}

#[test]
fn test_lsr_zero_page() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b0101_1010,
        0x85, 0x10,
        0x46, 0x10,
    ]);

    assert_eq!(cpu.mem_read(0x10), 0b0010_1101);
}

#[test]
fn test_lsr_zero_page_x() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b0010_1101,
        0x85, 0x06,
        0xA2, 0x04,
        0x56, 0x02,
    ]);

    assert_eq!(cpu.mem_read(0x06), 0b0001_0110);
}

#[test]
fn test_lsr_absolute() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b0011_0001,
        0x8D, 0xAB, 0xCD,
        0x4E, 0xAB, 0xCD,
    ]);

    assert_eq!(cpu.mem_read(0xCDAB), 0b0001_1000);
}

#[test]
fn test_lsr_absolute_x() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b0011_0001,
        0x8D, 0x34, 0x03,
        0xA2, 0x04,
        0x5E, 0x30, 0x03,
    ]);

    assert_eq!(cpu.mem_read(0x0334), 0b0001_1000);
}

#[test]
fn test_dec_zero_page() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0x43,
        0x85, 0x22,
        0xC6, 0x22,
    ]);

    assert_eq!(cpu.mem_read(0x0022), 0x42);
}

#[test]
fn test_dec_zero_page_x() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0x43,
        0x85, 0x35,
        0xA2, 0x02,
        0xD6, 0x33,
    ]);

    assert_eq!(cpu.mem_read(0x0035), 0x42);
}

#[test]
fn test_dec_absolute() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0x43,
        0x8D, 0x00, 0x03,
        0xCE, 0x00, 0x03,
    ]);

    assert_eq!(cpu.mem_read(0x0300), 0x42);
}

#[test]
fn test_dec_absolute_x() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0x43,
        0x8D, 0x45, 0x03,
        0xA2, 0x05,
        0xDE, 0x40, 0x03,
    ]);

    assert_eq!(cpu.mem_read(0x0345), 0x42);
}

#[test]
fn test_dec_negative_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0x00,
        0x85, 0x69,
        0xC6, 0x69,
    ]);

    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_dec_zero_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0x01,
        0x85, 0x69,
        0xC6, 0x69,
    ]);

    assert!(cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_dex_implied() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA2, 0x70,
        0xCA,
    ]);

    assert_eq!(cpu.register_x, 0x6F);
}

#[test]
fn test_dex_zero_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA2, 0x01,
        0xCA,
    ]);

    assert!(cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_dex_negative_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA2, 0x01,
        0xCA,
        0xCA,
    ]);

    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_dey_implied() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA0, 0x70,
        0x88,
    ]);

    assert_eq!(cpu.register_y, 0x6F);
}

#[test]
fn test_dey_zero_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA0, 0x01,
        0x88,
    ]);

    assert!(cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_dey_negative_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA0, 0x01,
        0x88,
        0x88,
    ]);

    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_eor_immediate() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b0110_1001,
        0x49, 0b1111_0000,
    ]);

    assert_eq!(cpu.register_a, 0b1001_1001);
}

#[test]
fn test_eor_zero_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b1111_1111,
        0x49, 0b1111_1111,
    ]);

    assert!(cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_eor_negative_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b1111_1111,
        0x49, 0b0101_1111,
    ]);

    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_inc_zero_page() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0x41,
        0x85, 0x22,
        0xE6, 0x22,
    ]);

    assert_eq!(cpu.mem_read(0x0022), 0x42);
}

#[test]
fn test_inc_zero_page_x() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0x41,
        0x85, 0x35,
        0xA2, 0x02,
        0xF6, 0x33,
    ]);

    assert_eq!(cpu.mem_read(0x0035), 0x42);
}

#[test]
fn test_inc_absolute() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0x41,
        0x8D, 0x69, 0x03,
        0xEE, 0x69, 0x03,
    ]);

    assert_eq!(cpu.mem_read(0x0369), 0x42);
}

#[test]
fn test_inc_absolute_x() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0x41,
        0x8D, 0x45, 0x03,
        0xA2, 0x05,
        0xFE, 0x40, 0x03,
    ]);

    assert_eq!(cpu.mem_read(0x0345), 0x42);
}

#[test]
fn test_inc_negative_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0x7F,
        0x85, 0x69,
        0xE6, 0x69,
    ]);

    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_inc_zero_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0xFF,
        0x85, 0x69,
        0xE6, 0x69,
    ]);

    assert!(cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_inx_implied() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA2, 0x70,
        0xE8,
    ]);

    assert_eq!(cpu.register_x, 0x71);
}

#[test]
fn test_inx_zero_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA2, 0xFF,
        0xE8,
    ]);

    assert!(cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_inx_negative_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA2, 0x7F,
        0xE8,
    ]);

    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_inx_overflow() {
    let mut cpu = new_cpu();

    cpu.load(vec![0xe8, 0xe8, 0x00]);
    cpu.reset();
    cpu.register_x = 0xff;
    cpu.run();

    assert_eq!(cpu.register_x, 1)
}

#[test]
fn test_iny_implied() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA0, 0x70,
        0xC8,
    ]);

    assert_eq!(cpu.register_y, 0x71);
}

#[test]
fn test_iny_zero_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA0, 0xFF,
        0xC8,
    ]);

    assert!(cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_iny_negative_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA0, 0x7F,
        0xC8,
    ]);

    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_ora_immediate() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b0101_1010,
        0x09, 0b0010_0101,
    ]);

    assert_eq!(cpu.register_a, 0b0111_1111);
}

#[test]
fn test_ora_zero_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b0000_0000,
        0x09, 0b0000_0000,
    ]);

    assert_eq!(cpu.register_a, 0);
    assert!(cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_ora_negative_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b0000_0000,
        0x09, 0b1000_1010,
    ]);

    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_rol_accumulator() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b0101_0111,
        0x2A,
    ]);

    assert_eq!(cpu.register_a, 0b1010_1110);
}

#[test]
fn test_rol_absolute() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b0101_0111,
        0x8D, 0x38, 0x0D,
        0xA9, 0b0101_0111,
        0x2E, 0x38, 0x0D,
    ]);

    assert_eq!(cpu.mem_read(0x0D38), 0b1010_1110);
}

#[test]
fn test_rol_carry_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b1100_1100,
        0x2A,
        0x2A,
    ]);

    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert!(cpu.register_a & 0b0000_0001 == 1);
}

#[test]
fn test_rol_zero_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b1000_0000,
        0x2A,
    ]);

    assert!(cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_rol_negative_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![0xA9, 0b0100_0000, 0x2A]);

    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_pha_after_reset() {
    let mut cpu = new_cpu();
    cpu.load(vec![0x48, 0x00]);
    cpu.reset();
    cpu.register_a = 0xDE;
    cpu.run();

    assert_eq!(cpu.stack_pointer, STACK_RESET - 1);

    let addr = u16::from_le_bytes([cpu.stack_pointer + 1, 0x01]);
    assert_eq!(cpu.mem_read(addr), 0xDE);
}

#[test]
fn test_pha_stack_overflow() {
    let mut cpu = new_cpu();
    cpu.load(vec![0x48, 0x00]);
    cpu.reset();
    cpu.register_a = 0xAD;
    cpu.stack_pointer = 0;
    cpu.run();

    assert_eq!(cpu.stack_pointer, 0xFF);
    assert_eq!(cpu.mem_read(0x0100), 0xAD);
}

#[test]
fn test_php_push_stack() {
    let mut cpu = new_cpu();
    cpu.load(vec![0x08, 0x00]);
    cpu.reset();
    cpu.status = StatusFlags::from_bits_truncate(0b0101_1010);
    cpu.run();

    assert_eq!(cpu.mem_read(0x01fd), 0b0111_1010);
}

#[test]
fn test_pla_negative_flag() {
    let mut cpu = new_cpu();
    cpu.load(vec![0x68, 0x00]);
    cpu.reset();
    cpu.mem_write(0x1ff, 0b1010_1111);
    cpu.stack_pointer = 0xfe;
    cpu.run();

    assert_eq!(cpu.register_a, 0b1010_1111);
    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_pla_zero_flag() {
    let mut cpu = new_cpu();
    cpu.load(vec![0x68, 0x00]);
    cpu.reset();
    cpu.register_a = 0b1111_1111;
    cpu.mem_write(0x1ff, 0b0000_0000);
    cpu.run();

    assert_eq!(cpu.register_a, 0);
    assert!(cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_plp_pull_flags() {
    let mut cpu = new_cpu();
    cpu.load(vec![0x28, 0x00]);
    cpu.reset();
    cpu.status = StatusFlags::empty();
    cpu.mem_write(0x1ff, 0b1111_1111);
    cpu.stack_pointer = 0xfe;
    cpu.run();

    assert_eq!(cpu.status.bits(), 0b1110_1111);
}

#[test]
fn test_ror_accumulator() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b0101_0111,
        0x6A,
    ]);

    assert_eq!(cpu.register_a, 0b0010_1011);
}

#[test]
fn test_ror_absolute() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b0101_0111,
        0x8D, 0x38, 0x0D,
        // 0xA9, 0b0101_0111,
        0x6E, 0x38, 0x0D,
    ]);

    assert_eq!(cpu.mem_read(0x0D38), 0b0010_1011);
}

#[test]
fn test_ror_carry_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b0000_0011,
        0x6A,
        0x6A,
    ]);

    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert!(cpu.register_a & 0b1000_0000 == 0b1000_0000);
}

#[test]
fn test_ror_carry_absolute() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b0101_0111,
        0x8D, 0x38, 0x0D,
        0x6E, 0x38, 0x0D,
        0x6E, 0x38, 0x0D,
    ]);

    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert_eq!(cpu.mem_read(0x0D38), 0b1001_0101);
}

#[test]
fn test_ror_zero_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b0000_0001,
        0x6A,
    ]);

    assert!(cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_ror_negative_flag() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![0xA9, 0b0000_0001, 0x6A, 0x6A, 0x00]);

    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_bcc_branch() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0x18,       // CLC
        0x90, 0x06, // BCC 0x10
    ]);

    assert_eq!(cpu.program_counter, 0x800A);
}

#[test]
fn test_bcc_dont_branch() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0x38,       // SEC
        0x90, 0x06, // BCC 0x10
    ]);

    assert_eq!(cpu.program_counter, 0x8004);
}

#[test]
fn test_bcs_branch() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0x38,       // SEC
        0xB0, 0x06, // BCC 0x10
    ]);

    assert_eq!(cpu.program_counter, 0x800A);
}

#[test]
fn test_bcs_dont_branch() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0x18,       // CLC
        0xB0, 0x06, // BCC 0x10
    ]);

    assert_eq!(cpu.program_counter, 0x8004);
}

#[test]
fn test_bmi_branch() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b1100_0000,
        0x30, 0x06,
    ]);

    assert_eq!(cpu.program_counter, 0x800B);
}

#[test]
fn test_bmi_dont_branch() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b0111_1111,
        0x30, 0x06,
    ]);

    assert_eq!(cpu.program_counter, 0x8005);
}

#[test]
fn test_beq_branch() {
    let mut cpu: CPU = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0x00,
        0xF0, 0x06,
    ]);

    assert_eq!(cpu.program_counter, 0x800B);
}

#[test]
fn test_beq_dont_branch() {
    let mut cpu: CPU = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0x69,
        0xF0, 0x06,
    ]);

    assert_eq!(cpu.program_counter, 0x8005);
}

#[test]
fn test_bne_branch() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0x69,
        0xD0, 0x06,
    ]);

    assert_eq!(cpu.program_counter, 0x800B);
}

#[test]
fn test_bne_dont_branch() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0,
        0xD0, 0x06,
    ]);

    assert_eq!(cpu.program_counter, 0x8005);
}

#[test]
fn test_bpl_branch() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b0101_1010,
        0x10, 0x06,
    ]);

    assert_eq!(cpu.program_counter, 0x800B);
}

#[test]
fn test_bpl_dont_branch() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0xA9, 0b1101_1010,
        0x10, 0x06,
    ]);

    assert_eq!(cpu.program_counter, 0x8005);
}

#[test]
fn test_bvc_branch() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0x50, 0x06,
    ]);
    cpu.reset();
    cpu.status.set(StatusFlags::OVERFLOW, false);
    cpu.run();

    assert_eq!(cpu.program_counter, 0x8009);
}

#[test]
fn test_bvc_dont_branch() {
    let mut cpu = new_cpu();
    cpu.load(vec![
        0x50, 0x06,
    ]);
    cpu.reset();
    cpu.status.set(StatusFlags::OVERFLOW, true);
    cpu.run();

    assert_eq!(cpu.program_counter, 0x8003);
}

#[test]
fn test_bvs_branch() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0x70, 0x06,
    ]);
    cpu.reset();
    cpu.status.set(StatusFlags::OVERFLOW, true);
    cpu.run();

    assert_eq!(cpu.program_counter, 0x8009);
}

#[test]
fn test_bvs_dont_branch() {
    let mut cpu = new_cpu();
    cpu.load(vec![
        0x70, 0x06,
    ]);
    cpu.reset();
    cpu.status.set(StatusFlags::OVERFLOW, false);
    cpu.run();

    assert_eq!(cpu.program_counter, 0x8003);
}

#[test]
fn test_jmp_absolute() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0x4C, 0x69, 0x80,
        0x00,
    ]);

    // PC is incremented once when 0x00 is read, so check
    // is for 1 higher than expected from JMP
    assert_eq!(cpu.program_counter, 0x806A);
}

#[test]
fn test_jmp_indirect() {
    let mut cpu = new_cpu();
    cpu.load(vec![
        0x6C, 0x00, 0x81,
        0x00,
    ]);
    cpu.reset();
    cpu.mem_write_u16(0x8100, 0x1234);
    cpu.run();

    assert_eq!(cpu.program_counter, 0x1235);
}

#[test]
fn test_jsr_pc() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0x20, 0x02, 0x40,
        0x00,
    ]);

    assert_eq!(cpu.program_counter, 0x4003);
}

#[test]
fn test_jsr_stack() {
    let mut cpu = new_cpu();
    cpu.load_and_run(vec![
        0x20, 0x02, 0x40,
        0x00,
    ]);

    assert_eq!(cpu.mem_read(0x01FD), 0x80);
    assert_eq!(cpu.mem_read(0x01FC), 0x02);
}

#[test]
fn test_rts_pc() {
    let mut cpu = new_cpu();
    cpu.load(vec![
        0x20, 0x02, 0x81,
    ]);
    cpu.reset();
    cpu.mem_write(0x8102, 0x60);
    cpu.run();

    assert_eq!(cpu.program_counter, 0x8004);
}

#[test]
fn test_adc_carry() {
    let mut cpu = new_cpu();
    cpu.load(vec![
        0x18, // CLC
        0x69, 0xFF, // ADC $FF
    ]);
    cpu.reset();
    cpu.register_a = 0x01;
    cpu.run();

    assert!(cpu.status.contains(StatusFlags::CARRY));
}

#[test]
fn test_adc_zero() {
    let mut cpu = new_cpu();
    cpu.load(vec![
        0x69, 0x00,
    ]);
    cpu.reset();
    cpu.run();

    assert!(cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_adc_negative() {
    let mut cpu = new_cpu();
    cpu.load(vec![
        0x69, 0x80,
    ]);
    cpu.reset();
    cpu.run();

    assert!(cpu.status.contains(StatusFlags::NEGATIVE));
}

#[test]
fn test_adc_overflow() {
    let mut cpu = new_cpu();
    cpu.load(vec![
        0x69, 0b0100_0000,
    ]);
    cpu.reset();
    cpu.register_a = 0b0100_0000;
    cpu.run();

    assert!(cpu.status.contains(StatusFlags::OVERFLOW));

    cpu.load(vec![
        0x69, 0b0111_1111,
    ]);
    cpu.reset();
    cpu.status.set(StatusFlags::CARRY, true);
    cpu.run();

    assert!(cpu.status.contains(StatusFlags::OVERFLOW));
}

#[test]
fn test_adc_decimal_mode() {
    let mut cpu = new_cpu();
    cpu.enable_decimal = true;
    cpu.load(vec![
        0xF8, // SED
        0x69, 0x19,
    ]);
    cpu.reset();
    cpu.register_a = 0x28;
    cpu.run();

    assert_eq!(cpu.register_a, 0x47);
}

#[test]
fn test_adc_decimal_carry() {
    let mut cpu = new_cpu();
    cpu.enable_decimal = true;
    // 0x58 + 0x46 + 0x01 (carry) = 105 (0x05 + carry)
    cpu.load(vec![
        0xF8, // SED
        0x38, // SEC
        0x69, 0x58,
    ]);
    cpu.reset();
    cpu.register_a = 0x46;
    cpu.run();

    assert_eq!(cpu.register_a, 0x05);
    assert!(cpu.status.contains(StatusFlags::CARRY));
}

#[test]
fn test_adc_decimal_add_81_and_92() {
    let mut cpu = new_cpu();
    cpu.enable_decimal = true;
    cpu.load(vec![
        0xF8,
        0x69, 0x92,
    ]);
    cpu.reset();
    cpu.register_a = 0x81;
    cpu.run();

    println!("reg a: {}", cpu.register_a);
    assert_eq!(cpu.register_a, 0x73);
    assert!(cpu.status.contains(StatusFlags::CARRY));
    assert!(cpu.status.contains(StatusFlags::OVERFLOW));
}

#[test]
fn test_adc_decimal_add_zero() {
    let mut cpu = new_cpu();
    cpu.enable_decimal = true;
    cpu.load(vec![
        0xF8,
        0x69, 0x00,
    ]);
    cpu.reset();
    cpu.register_a = 0x10;
    cpu.run();

    assert_eq!(cpu.register_a, 0x10);
}

#[test]
fn test_adc_decimal_9_plus_11() {
    let mut cpu = new_cpu();
    cpu.enable_decimal = true;
    cpu.load(vec![
        0xF8,
        0x69, 0x11,
    ]);
    cpu.reset();
    cpu.register_a = 0x09;
    cpu.run();

    assert_eq!(cpu.register_a, 0x20);
}

#[test]
fn test_sbc_zero() {
    let mut cpu = new_cpu();
    cpu.load(vec![
        0x38, 0xB8,
        0xE9, 0x0A,
    ]);
    cpu.reset();
    cpu.register_a = 0x0A;
    cpu.run();

    assert!(cpu.status.contains(StatusFlags::ZERO));
}

#[test]
fn test_sbc() {
    let mut cpu = new_cpu();
    cpu.load(vec![
        0x38, 0xB8,
        0xE9, 0x27,
    ]);
    cpu.reset();
    cpu.register_a = 0x29;
    cpu.run();

    assert_eq!(cpu.register_a, 0x02);
}

#[test]
fn test_sbc_decimal_mode() {
    let mut cpu = new_cpu();
    cpu.enable_decimal = true;
    cpu.load(vec![
        0x38, 0xB8,
        0xF8,
        0xE9, 0x03,
    ]);
    cpu.reset();
    cpu.register_a = 0x99;
    cpu.run();

    assert_eq!(cpu.register_a, 0x96);
}

#[test]
fn test_sbc_decimal_mode_wrap() {
    let mut cpu = new_cpu();
    cpu.enable_decimal = true;
    cpu.load(vec![
        0x38, 0xB8,
        0xF8,
        0xE9, 0x24,
    ]);
    cpu.reset();
    cpu.register_a = 0x50;
    cpu.run();

    assert_eq!(cpu.register_a, 0x26);
}

#[test]
fn test_rti() {
    let mut cpu = new_cpu();
    cpu.load(vec![
        0xA9, 0xCC, // LDA #$FF
        0x48,       // PHA
        0xA9, 0xBB, // LDA #$BB
        0x48,       // PHA
        0xA9, 0xFF, // LDA #$CC
        0x48,       // PHA
        0x40,       // RTI
    ]);
    cpu.reset();
    cpu.run();

    // TODO: I'm not sure about the behavior of bits
    // 4 & 5 when pulled off the stack here.
    assert_eq!(cpu.status.bits(), 0b1110_1111);
    assert_eq!(cpu.program_counter, u16::from_le_bytes([0xBB, 0xCC]) + 1);
}

