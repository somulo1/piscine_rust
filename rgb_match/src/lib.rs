// use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(self, first: u8, second: u8) -> Color {
        match (first, second) {
            (f, s) if f == self.r && s == self.g => Color { r: s, g: f, ..self },
            (f, s) if f == self.r && s == self.b => Color { r: s, b: f, ..self },
            (f, s) if f == self.r && s == self.a => Color { r: s, a: f, ..self },
            (f, s) if f == self.g && s == self.r => Color { g: s, r: f, ..self },
            (f, s) if f == self.g && s == self.b => Color { g: s, b: f, ..self },
            (f, s) if f == self.g && s == self.a => Color { g: s, a: f, ..self },
            (f, s) if f == self.b && s == self.r => Color { b: s, r: f, ..self },
            (f, s) if f == self.b && s == self.g => Color { b: s, g: f, ..self },
            (f, s) if f == self.b && s == self.a => Color { b: s, a: f, ..self },
            (f, s) if f == self.a && s == self.r => Color { a: s, r: f, ..self },
            (f, s) if f == self.a && s == self.g => Color { a: s, g: f, ..self },
            (f, s) if f == self.a && s == self.b => Color { a: s, b: f, ..self },
            _ => self,
        }
    }
}

// impl Color {
//     pub fn swap(mut self, first: u8, second: u8) -> Color {
//         let mut components = HashMap::new();
//         components.insert('r', self.r);
//         components.insert('g', self.g);
//         components.insert('b', self.b);
//         components.insert('a', self.a);

//         let mut keys_to_swap = Vec::new();
//         for (&k, &v) in &components {
//             if v == first || v == second {
//                 keys_to_swap.push(k);
//             }
//         }

//         if keys_to_swap.len() == 2 {
//             let k1 = keys_to_swap[0];
//             let k2 = keys_to_swap[1];
//             let v1 = components[&k1];
//             let v2 = components[&k2];
//             components.insert(k1, v2);
//             components.insert(k2, v1);
//         }

//         Color {
//             r: components[&'r'],
//             g: components[&'g'],
//             b: components[&'b'],
//             a: components[&'a'],
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let c = Color {
            r: 255,
            g: 200,
            b: 10,
            a: 30,
        };
        let result = c.swap(c.r, c.b);
        assert_eq!(result, Color {r: 10, g: 200, b: 255, a: 30});
    }
}
