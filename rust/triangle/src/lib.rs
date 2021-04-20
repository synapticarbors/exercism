pub struct Triangle(u64, u64, u64);

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let has_zeros = sides.iter().any(|&x| x == 0);
        let satisfies_inequality = (sides[0] < sides[1] + sides[2])
            && (sides[1] < sides[0] + sides[2])
            && (sides[2] < sides[0] + sides[1]);

        if !has_zeros && satisfies_inequality {
            Some(Triangle(sides[0], sides[1], sides[2]))
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        (self.0 == self.1) && (self.1 == self.2)
    }

    pub fn is_scalene(&self) -> bool {
        (self.0 != self.1) && (self.0 != self.2) && (self.1 != self.2)
    }

    pub fn is_isosceles(&self) -> bool {
        (self.0 == self.1) || (self.0 == self.2) || (self.1 == self.2)
    }
}
