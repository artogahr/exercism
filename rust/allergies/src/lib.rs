pub struct Allergies {
    allergies: Vec<Allergen>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies {
            allergies: (0..8)
                .filter(|i| score & (1 << i) != 0)
                .filter_map(|i| match i {
                    0 => Some(Allergen::Eggs),
                    1 => Some(Allergen::Peanuts),
                    2 => Some(Allergen::Shellfish),
                    3 => Some(Allergen::Strawberries),
                    4 => Some(Allergen::Tomatoes),
                    5 => Some(Allergen::Chocolate),
                    6 => Some(Allergen::Pollen),
                    7 => Some(Allergen::Cats),
                    _ => None,
                })
                .collect::<Vec<Allergen>>(),
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergies.to_vec()
    }
}
