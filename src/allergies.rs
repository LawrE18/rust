/*
Given a person's allergy score, determine whether or not they're allergic to a given item,
and their full list of allergies.
An allergy test produces a single numeric score which contains the information about all
the allergies the person has (that they were tested for).
The list of items (and their value) that were tested are:
    eggs (1)
    peanuts (2)
    shellfish (4)
    strawberries (8)
    tomatoes (16)
    chocolate (32)
    pollen (64)
    cats (128)

So if Tom is allergic to peanuts and chocolate, he gets a score of 34.
Now, given just that score of 34, your program should be able to say:
    Whether Tom is allergic to any one of those allergens listed above.
    All the allergens Tom is allergic to.
Note: a given score may include allergens not listed above (i.e. allergens that score 256,
512, 1024, etc.). Your program should ignore those components of the score. For example,
if the allergy score is 257, your program should only report the eggs (1) allergy.
 */

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Allergen {
    Eggs = 0,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}
// My solution
pub struct Allergies {
    score_: u32,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let score = if score >= 256 {
            score % 256
        } else {score};
        Allergies {
            score_: score,
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        for item_ in self.allergies() {
            if item_ == *allergen {
                return true;
            }
        }
        false
    }
    pub fn allergies(&self) -> Vec<Allergen> {
        let mut allergens: Vec<Allergen> = Vec::new();
        let score_str = format!("{:b}", self.score_);
        let mut score_chars = score_str.chars().rev();

        for (i, ch) in score_chars.enumerate() {
            if ch == '1' {
                let al = match i {
                    0 => Allergen::Eggs,
                    1 => Allergen::Peanuts,
                    2 => Allergen::Shellfish,
                    3 => Allergen::Strawberries,
                    4 => Allergen::Tomatoes,
                    5 => Allergen::Chocolate,
                    6 => Allergen::Pollen,
                    7 => Allergen::Cats,
                    _ => panic!("score not in range")
                };
                allergens.push(al);
            }
        }
        allergens
    }
}

// Community solution
/*
    Для определение есть ли конкретная аллергия просто используется побитовое и.
    Для определения всех аллергенов итерируемся по константному листу
    и фильтруем все аллергены которые есть у человека, затем клонируем их, потому
    что они неизменяемы и заменить ссылку не можем, и затем собираем их в коллекцию,
    котрая Vec по умолчанию.
 */
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Allergen_ {
    Eggs = 1 << 0,
    Peanuts = 1 << 1,
    Shellfish = 1 << 2,
    Strawberries = 1 << 3,
    Tomatoes = 1 << 4,
    Chocolate = 1 << 5,
    Pollen = 1 << 6,
    Cats = 1 << 7,
}

const ALLERGENS: [Allergen_; 8] =
    [Eggs, Peanuts, Shellfish, Strawberries, Tomatoes, Chocolate, Pollen, Cats];

pub struct Allergies_ {
    allergens: u32,
}

impl Allergies_ {
    pub fn new(n: u32) -> Allergies_ {
        Allergies_ { allergens: n }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen_) -> bool {
        let allergen = *allergen as u32;
        self.allergens & allergen == allergen
    }

    pub fn allergies(&self) -> Vec<Allergen_> {
        ALLERGENS.iter().filter(|a| self.is_allergic_to(a)).cloned().collect()
    }
}