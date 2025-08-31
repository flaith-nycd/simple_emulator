/*
SimpleEmulator.rs d'après "D:\PROJECTS\APPLE\SimpleEmulator.pb"

1. Structure SimpleEmulator : Contient la mémoire, le compteur de programme et la table d'instructions
2. Table d'instructions : Utilise un tableau d'options de fonctions pour mapper les opcodes
3. Méthodes d'instructions : add(), sub(), mul(), div(), nop() qui reproduisent le comportement original

/!\ 4. Gestion sécurisée : Utilisation de wrapping_* pour les opérations arithmétiques pour éviter les paniques
    https://www.dotnetperls.com/wrapping-add-rust
    https://www.slingacademy.com/article/exploring-rusts-overflow-behavior-wrapping-saturating-and-panicking/#wrapping-arithmetic

5. Chargement de programme : La méthode load_program() charge les données en mémoire
6. Boucle d'exécution : La méthode run() exécute le programme jusqu'à rencontrer l'opcode 0xFF
*/

// Structure pour représenter l'émulateur
struct SimpleEmulator {
    memory: [u8; 65536],
    pc: usize,
    instructions: [Option<fn(&mut SimpleEmulator) -> u8>; 256],
}

impl SimpleEmulator {
    fn new() -> Self {
        let mut emu = SimpleEmulator {
            memory: [0; 65536],
            pc: 0,
            instructions: [None; 256],
        };

        // Initialiser la table d'instructions
        emu.instructions[0x00] = Some(Self::add);
        emu.instructions[0x01] = Some(Self::sub);
        emu.instructions[0x02] = Some(Self::mul);
        emu.instructions[0x03] = Some(Self::div);
        emu.instructions[0xEA] = Some(Self::nop);

        emu
    }

    /// Formater un octet en hexadécimal
    fn hexa(value: u8) -> String {
        format!("{:02X}", value)
    }

    /// Récupérer un octet de la mémoire et incrémenter le PC
    fn get_memory(&mut self) -> u8 {
        self.pc += 1;
        self.memory[self.pc]
    }

    // Instructions
    fn add(&mut self) -> u8 {
        let a = self.get_memory();
        let b = self.get_memory();
        println!("----Add ${} to ${}", Self::hexa(a), Self::hexa(b));
        a.wrapping_add(b)
    }

    fn sub(&mut self) -> u8 {
        let a = self.get_memory();
        let b = self.get_memory();
        println!("----Sub ${} to ${}", Self::hexa(a), Self::hexa(b));
        a.wrapping_sub(b)
    }

    fn mul(&mut self) -> u8 {
        let a = self.get_memory();
        let b = self.get_memory();
        println!("----Mul ${} to ${}", Self::hexa(a), Self::hexa(b));
        a.wrapping_mul(b)
    }

    fn div(&mut self) -> u8 {
        let a = self.get_memory();
        let b = self.get_memory();
        println!("----Div ${} to ${}", Self::hexa(a), Self::hexa(b));
        if b != 0 { a / b } else { 0 }
    }

    fn nop(&mut self) -> u8 {
        println!("----Nop");
        0xEA
    }

    /// Charger le programme en mémoire
    fn load_program(&mut self, program: &[u8]) {
        for (i, &byte) in program.iter().enumerate() {
            if i < self.memory.len() {
                self.memory[i] = byte;
            }
        }
    }

    /// Exécuter le programme
    fn run(&mut self) {
        let mut opcode = self.memory[self.pc];

        while opcode != 0xFF {
            if let Some(instruction) = self.instructions[opcode as usize] {
                let result = instruction(self);
                println!("${}", Self::hexa(result));
            } else {
                println!("Unknown opcode: ${:02X}", opcode);
                break;
            }

            self.pc += 1;
            if self.pc >= self.memory.len() {
                break;
            }
            opcode = self.memory[self.pc];
        }

        println!("----End");
    }
}

fn main() {
    // Programme à exécuter (équivalent au DataSection)
    let program_data = [
        0x00, 0x21, 0x30, // Addition de $21 et $30
        0x00, 0xF0, 0x50, // Additionne $F0 à $50
        0x01, 0x33, 0x45, // Soustraction
        0x01, 0xF0, 0x53, // Soustraction
        0x02, 0x33, 0x03, // Multiplication
        0x03, 0xDE, 0x45, // Division
        0xEA, // NOP
        0xEA, // NOP
        0xEA, // NOP
        0x00, 0x01, 0x01, // Addition
        0x01, 0x01, 0x01, // Soustraction
        0x02, 0x01, 0x01, // Multiplication
        0x03, 0x01, 0x01, // Division
        0xFF, // Fin du programme
    ];

    let mut emulator = SimpleEmulator::new();
    emulator.load_program(&program_data);
    emulator.run();
}
