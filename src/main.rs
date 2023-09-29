use std::{
    fmt::{Debug, Formatter},
    num::NonZeroU8,
};
const WIDTH: usize = 9;

#[derive(Default, Clone)]
struct Board([[Option<NonZeroU8>; WIDTH]; WIDTH]);

impl Debug for Board {
    #[rustfmt::skip]
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let c: [[char; WIDTH]; WIDTH] = self
            .0
            .map(|r| r.map(|u| char::from(u.map_or(b' ', |b| b.get() + b'0'))));

        formatter.write_fmt(format_args!(
            "\n\
            {} {} {} | {} {} {} | {} {} {}\n\
            {} {} {} | {} {} {} | {} {} {}\n\
            {} {} {} | {} {} {} | {} {} {}\n\
            ------+-------+------\n\
            {} {} {} | {} {} {} | {} {} {}\n\
            {} {} {} | {} {} {} | {} {} {}\n\
            {} {} {} | {} {} {} | {} {} {}\n\
            ------+-------+------\n\
            {} {} {} | {} {} {} | {} {} {}\n\
            {} {} {} | {} {} {} | {} {} {}\n\
            {} {} {} | {} {} {} | {} {} {}\n\
            ",
            c[0][0],c[0][1],c[0][2], c[0][3],c[0][4],c[0][5], c[0][6],c[0][7],c[0][8],
            c[1][0],c[1][1],c[1][2], c[1][3],c[1][4],c[1][5], c[1][6],c[1][7],c[1][8],
            c[2][0],c[2][1],c[2][2], c[2][3],c[2][4],c[2][5], c[2][6],c[2][7],c[2][8],

            c[3][0],c[3][1],c[3][2], c[3][3],c[3][4],c[3][5], c[3][6],c[3][7],c[3][8],
            c[4][0],c[4][1],c[4][2], c[4][3],c[4][4],c[4][5], c[4][6],c[4][7],c[4][8],
            c[5][0],c[5][1],c[5][2], c[5][3],c[5][4],c[5][5], c[5][6],c[5][7],c[5][8],

            c[6][0],c[6][1],c[6][2], c[6][3],c[6][4],c[6][5], c[6][6],c[6][7],c[6][8],
            c[7][0],c[7][1],c[7][2], c[7][3],c[7][4],c[7][5], c[7][6],c[7][7],c[7][8],
            c[8][0],c[8][1],c[8][2], c[8][3],c[8][4],c[8][5], c[8][6],c[8][7],c[8][8],
        ))
    }
}

impl Board {
    fn has_error(&self) -> bool {
        let mut rows = [[false; WIDTH]; WIDTH];
        let mut cols = [[false; WIDTH]; WIDTH];
        let mut boxs = [[false; WIDTH]; WIDTH];

        for (y, &row) in self.0.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if let Some(digit) = cell {
                    let index = usize::from(digit.get() - 1);
                    let box_index = ((y / 3) * 3) + (x / 3);
                    if rows[x][index] || cols[y][index] || boxs[box_index][index] {
                        return true;
                    }
                    rows[x][index] = true;
                    cols[y][index] = true;
                    boxs[box_index][index] = true;
                }
            }
        }

        false
    }

    fn solve(&mut self) -> anyhow::Result<()> {
        if self.has_error() {
            anyhow::bail!("unsolvable");
        }

        // find first empty cell
        for y in 0..WIDTH {
            for x in 0..WIDTH {
                if self.0[y][x].is_some() {
                    continue;
                }

                for n in 1..=WIDTH {
                    self.0[y][x] = NonZeroU8::new(n as u8);

                    if let Ok(()) = self.solve() {
                        return Ok(());
                    }
                }

                self.0[y][x] = None;
                anyhow::bail!("unsolvable");
            }
        }

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let mut board = Board::default();
    for (i, byte) in std::fs::read("sudoku.txt")?
        .into_iter()
        .filter(|&b| b >= b'0' && b <= b'9')
        .take(WIDTH * WIDTH)
        .enumerate()
    {
        board.0[i / WIDTH][i % WIDTH] = NonZeroU8::new(byte - b'0');
    }

    println!("{board:?}");

    board.solve()?;
    println!("{board:?}");

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::Board;
    use std::num::NonZeroU8;

    const BOARD: Board = Board(unsafe {
        std::mem::transmute::<[[u8; 9]; 9], _>([
            [8, 0, 9, 3, 0, 0, 0, 0, 0],
            [7, 0, 0, 0, 5, 0, 0, 0, 0],
            [0, 1, 3, 7, 0, 8, 0, 9, 0],
            [2, 0, 0, 0, 0, 0, 0, 3, 0],
            [0, 0, 1, 0, 0, 0, 2, 0, 0],
            [0, 4, 0, 0, 0, 0, 0, 0, 8],
            [0, 5, 0, 8, 0, 9, 7, 6, 0],
            [0, 0, 0, 0, 1, 0, 0, 0, 3],
            [0, 0, 0, 0, 0, 6, 8, 0, 1],
        ])
    });

    const SOLUTION: [[Option<NonZeroU8>; 9]; 9] = unsafe {
        std::mem::transmute::<[[u8; 9]; 9], _>([
            [8, 6, 9, 3, 4, 2, 5, 1, 7],
            [7, 2, 4, 9, 5, 1, 3, 8, 6],
            [5, 1, 3, 7, 6, 8, 4, 9, 2],
            [2, 9, 8, 1, 7, 4, 6, 3, 5],
            [3, 7, 1, 6, 8, 5, 2, 4, 9],
            [6, 4, 5, 2, 9, 3, 1, 7, 8],
            [1, 5, 2, 8, 3, 9, 7, 6, 4],
            [4, 8, 6, 5, 1, 7, 9, 2, 3],
            [9, 3, 7, 4, 2, 6, 8, 5, 1],
        ])
    };

    #[test]
    fn test_solve() {
        let mut board = std::hint::black_box(BOARD.to_owned());
        board.solve().unwrap();
        assert_eq!(board.0, SOLUTION);
    }
}
