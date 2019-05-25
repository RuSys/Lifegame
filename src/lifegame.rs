
#[derive(Debug)]
#[allow(dead_code)]
pub struct Lifegame {
    width: usize,
    height: usize,
    field: Vec<Vec<bool>>,
    prefield: Vec<Vec<bool>>,
}

impl Lifegame {
    #![allow(dead_code)]
    pub fn new(x: usize, y: usize) -> Lifegame {
        Lifegame{width: x, height: y, field: vec![vec![false; y]; x], prefield: vec![vec![false; y]; x]}
    }

    #[allow(dead_code)]
    pub fn nextgen(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let mut c = 0;
                for xx in (x as i32 -1)..(x as i32 +2) {
                    for yy in (y as i32 -1)..(y as i32 +2) {
                        //print!("{}:{} \n",xx,yy);
                        if yy < 0 || yy >= self.height as i32 || xx < 0 || xx >= self.width as i32 || (x as i32 == xx && y as i32 == yy) {continue;}
                        if self.prefield[xx as usize][yy as usize] {c += 1;}
                    }
                }
                if (c == 2 && self.prefield[x][y]) || c == 3 {
                    self.field[x][y] = true;
                }
                else {
                    self.field[x][y] = false;
                }
            }
        }
        self.prefield = self.field.clone();
    }

    #[allow(dead_code)]
    pub fn print(&mut self) {
        for x in 0..self.width{
            for y in 0..self.height {
                if self.field[x][y] {
                    print!("o");
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
    }

    #[allow(dead_code)]
    pub fn set(&mut self, x: usize, y: usize) {
        self.field[x][y] = true;
        self.prefield[x][y] = true;
    }

    #[allow(dead_code)]
    pub fn get(&mut self, x: usize, y: usize) -> bool {
        self.field[x][y].clone()
    }
}