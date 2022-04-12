pub struct Tile {
    id: (u32, u32)
}
impl Tile {
    pub fn new(id: (u32, u32)) -> Tile {
        Tile {id}
    }
}

pub fn floor(x: u32, y: u32) -> Vec<Vec<Tile>> {
    let mut a: Vec<Vec<Tile>> = Vec::new();
    for i in 0..x {
        let mut b: Vec<Tile> = Vec::new();
        for j in 0..y {
            let t = Tile::new((i, j));
            b.push(t);
        }
        a.push(b);
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn make_floor() {
        let f = floor(5, 5);
        assert_eq!(f[0][0].id, (0, 0));
        assert_eq!(f[1][0].id, (1, 0));
        assert_eq!(f[4][3].id, (4, 3));
    }
}
