use std::collections::HashMap;

#[macro_use]
mod hash_macro {
    macro_rules! hashmap {
        ($( $key: expr => $val: expr ),*) => {{
            let mut map = ::std::collections::HashMap::new();
            $( map.insert($key.to_string(), $val); )*
            map
        }}
    }
}



pub struct SymbolTable {
    labels: HashMap<String, usize>,
    next_address: usize
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        let predefined_labels = hashmap![
            "SP" => 0,
            "LCL" => 1,
            "ARG" => 2,
            "THIS" => 3,
            "THAT" => 4,
            "SCREEN" => 16384,
            "KBD" => 24576,
            "R0" => 0,
            "R1" => 1,
            "R2" => 2,
            "R3" => 3,
            "R4" => 4,
            "R5" => 5,
            "R6" => 6,
            "R7" => 7,
            "R8" => 8,
            "R9" => 9,
            "R10" => 10,
            "R11" => 11,
            "R12" => 12,
            "R13" => 13,
            "R14" => 14,
            "R15" => 15
        ];

        SymbolTable {
            labels: predefined_labels,
            next_address: 16
        }
    }

    pub fn register_label(self: &mut Self, name: &str, value: usize) -> &usize {
        
        self.labels.insert(name.to_string(), value);
        
        self.labels.get(name).unwrap()
    }

    pub fn get_value(self: &mut Self, name: &str) -> Option<&usize> {
        if !self.labels.contains_key(name) {
            self.next_address += 1;
            return Some(self.register_label(name, self.next_address - 1));
        } else {
            return Some(&self.labels[name]);
        }
        
    }

    pub fn print_table(self: &Self) {

        for (key, value) in self.labels.clone().into_iter() {
            println!("{} / {}", key, value);
        }
    }

   
}