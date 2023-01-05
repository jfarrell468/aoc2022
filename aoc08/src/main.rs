#[derive(Clone, Copy)]
enum Direction {
    Top = 0,
    Bottom = 1,
    Left = 2,
    Right = 3,
}

struct Tree {
    height: i8,
    max_heights: [Option<i8>; 4],
}

impl Tree {
    fn new(c: char) -> Tree {
        Tree {
            height: c as i8 - '0' as i8,
            max_heights: [None, None, None, None],
        }
    }
}

struct Forest {
    trees: Vec<Vec<Tree>>,
}

impl Forest {
    fn is_visible(&mut self, r: usize, c: usize) -> bool {
        self.is_visible_from(r, c, Direction::Top)
            || self.is_visible_from(r, c, Direction::Bottom)
            || self.is_visible_from(r, c, Direction::Left)
            || self.is_visible_from(r, c, Direction::Right)
    }
    fn is_visible_from(&mut self, r: usize, c: usize, d: Direction) -> bool {
        self.is_on_edge(r, c, d) || self.trees[r][c].height > self.max_height_in_dir(r, c, d)
    }
    fn is_on_edge(&self, r: usize, c: usize, d: Direction) -> bool {
        match d {
            Direction::Top => r == 0,
            Direction::Bottom => r == self.trees.len() - 1,
            Direction::Left => c == 0,
            Direction::Right => c == self.trees[r].len() - 1,
        }
    }
    fn max_height_in_dir(&mut self, r: usize, c: usize, d: Direction) -> i8 {
        if let Some(h) = self.trees[r][c].max_heights[d as usize] {
            return h;
        }
        let max = self.max_height_in_dir_uncached(r, c, d);
        self.trees[r][c].max_heights[d as usize].replace(max);
        max
    }
    fn max_height_in_dir_uncached(&mut self, r: usize, c: usize, d: Direction) -> i8 {
        let (nr, nc) = self.neighbor(r, c, d);
        let nh = self.trees[nr][nc].height;
        if self.is_on_edge(nr, nc, d) {
            return nh;
        }
        std::cmp::max(nh, self.max_height_in_dir(nr, nc, d))
    }
    fn neighbor(&self, r: usize, c: usize, d: Direction) -> (usize, usize) {
        match d {
            Direction::Top => (r - 1, c),
            Direction::Bottom => (r + 1, c),
            Direction::Left => (r, c - 1),
            Direction::Right => (r, c + 1),
        }
    }
    fn scenic_score(&self, r: usize, c: usize) -> i32 {
        self.scenic_score_in_direction(r, c, Direction::Top)
            * self.scenic_score_in_direction(r, c, Direction::Bottom)
            * self.scenic_score_in_direction(r, c, Direction::Left)
            * self.scenic_score_in_direction(r, c, Direction::Right)
    }
    fn scenic_score_in_direction(&self, r: usize, c: usize, d: Direction) -> i32 {
        let mut r1 = r;
        let mut c1 = c;
        let mut score = 0;
        while !self.is_on_edge(r1, c1, d) {
            score += 1;
            (r1, c1) = self.neighbor(r1, c1, d);
            if self.trees[r1][c1].height >= self.trees[r][c].height {
                break;
            }
        }
        score
    }
}

fn main() {
    let mut forest = Forest { trees: Vec::new() };
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        forest
            .trees
            .push(line.chars().map(|c| Tree::new(c)).collect());
    }
    let mut num_visible = 0;
    let mut max_scenic_score = 0;
    for r in 0..forest.trees.len() {
        for c in 0..forest.trees[r].len() {
            if forest.is_visible(r, c) {
                num_visible += 1;
            }
            max_scenic_score = std::cmp::max(max_scenic_score, forest.scenic_score(r, c));
        }
    }
    println!("Part 1: {}", num_visible);
    println!("Part 2: {}", max_scenic_score);
}
