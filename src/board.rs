
const BLACK: [f32; 4] = [209.0 / 255.0, 139.0 / 255.0, 71.0 / 255.0, 1.0];
const WHITE: [f32; 4] = [255.0 / 255.0, 206.0 / 255.0, 158.0 / 255.0, 1.0];

#[derive(Copy, Clone)]
pub struct Square {
    pub color : [f32; 4],
    pub index: u8
}

pub struct Board {
    pub squares : Vec<Square>
}

impl Square {
    pub fn new(color: [f32; 4], index: u8) -> Self {
        Square { color , index }
    }
    pub fn index_to_file_rank(self: &Self) -> (u8, u8) {
        (self.index / 8, 7 - self.index % 8)
    }
}

impl Board {
    pub fn new() -> Self {
        let mut squares: Vec<Square> = Vec::new();
        for i in 0..64 {
            if (i + i / 8) % 2 == 0 {
                squares.push(Square::new(WHITE, i))
            } else {
                squares.push(Square::new(BLACK, i))
            }
        }
        Board { squares }
    }
    pub fn get_square_at_file_rank(&self, file: u8, rank:u8) -> &Square {
        &self.squares[(rank + file * 8) as usize]
    }

}
