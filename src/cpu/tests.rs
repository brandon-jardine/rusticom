use super::*;

#[test]
fn test_0xa9_lda_immidiate_load_data() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0x05, 0x00]);

    assert_eq!(cpu.register_a, 0x05);
    assert!(cpu.status & 0b0000_0010 == 0b00);
    assert!(cpu.status & 0b1000_0000 == 0);
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

    assert!(cpu.status & 0b0000_0010 == 0b10);
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
    cpu.status = 0xff;
    cpu.reset();

    assert_eq!(cpu.register_x, 0);
    assert_eq!(cpu.register_a, 0);
    assert_eq!(cpu.register_y, 0);
    assert_eq!(cpu.register_s, 0xff);
    assert_eq!(cpu.status, 0);
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

    assert!(cpu.status & 0b0000_0010 == 0b10);
}

#[test]
fn test_txa_negative_flag() {
    let mut cpu = CPU::new();
    cpu.load(vec![0x8A, 0x00]);
    cpu.reset();
    cpu.register_x = 0b1000_0010;
    cpu.run();

    assert!(cpu.status & 0b1000_0000 == 0b1000_0000)
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

    assert!(cpu.status & 0b0000_0010 == 0b10);
}

#[test]
fn test_tsx_negative_flag() {
    let mut cpu = CPU::new();
    cpu.load(vec![0xBA, 0x00]);
    cpu.reset();
    cpu.register_s = 0b1000_0010;
    cpu.run();

    assert!(cpu.status & 0b1000_0000 == 0b1000_0000)
}
