fn main() {
    let mut current = House::new(0, 0);

    let moves : Vec<Dir> = INPUT.chars()
        .map(|c| From::from(c))
        .collect();

    let mut houses = vec![current];

    for d in moves {
        current = current.do_move(d);

        if !houses.contains(&current) {
            houses.push(current);
        }
    }

    println!("{:?}", houses.len());


    // Part 2
    let mut current = House::new(0, 0);
    let mut santa = House::new(0, 0);
    let mut robo_santa = House::new(0, 0);

    let moves : Vec<Dir> = INPUT.chars()
        .map(|c| From::from(c))
        .collect();

    let mut houses = vec![santa];
    let mut is_santa = true;

    for d in moves {
        if is_santa {
            santa = santa.do_move(d);
            current = santa;
        }
        else 
        {
            robo_santa = robo_santa.do_move(d);
            current = robo_santa;
        }

        is_santa = !is_santa;

        if !houses.contains(&current) {
            houses.push(current);
        }
    }

    println!("{:?}", houses.len());
}

#[derive(Debug)]
pub enum Dir { N, S, E, W }

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct House {
    x: i32,
    y: i32,
}

impl House {
    fn new(x: i32, y: i32) -> House {
        House{ x:x, y:y }
    }

    fn do_move(&self, dir: Dir) -> House {
        match dir {
            Dir::N => House::new(self.x, self.y + 1),
            Dir::S => House::new(self.x, self.y - 1),
            Dir::E => House::new(self.x + 1, self.y),
            Dir::W => House::new(self.x - 1, self.y),
        }
    }
}

impl From<char> for Dir {
    fn from(c: char) -> Dir {
        match c {
            '^' => Dir::N,
            '>' => Dir::E,
            'v' => Dir::S,
            '<' => Dir::W,
            _ => unreachable!()
        }
    }
}

const INPUT : &'static str = "^^<<v<<v><v^^<><>^^<v<v^>>^^^><^>v^>v><><><<vv^^<^>^^<v^>v>v^v>>>^<>v<^<v^><^>>>>><<v>>^>>^>v^>><<^>v>v<>^v^v^vvv><>^^>v><v<><>^><^^<vv^v<v>^v>>^v^>v><>v^<vv>^><<v^>vv^<<>v>>><<<>>^<vv<^<>^^vv>>>^><<<<vv^v^>>><><^>v<>^>v<v^v<^vv><^v^><<<<>^<>v>^v>v<v<v<<>v<^<<<v>>>>>^^v>vv^^<>^<>^^^^<^^^v<v^^>v<^^v^^>v>^v^^^^>><<v<>v<>^v^<v<>><>^^><<^^<^^>vv<>v^<^v<vv<<<>^>^^>^<>v^^vv<>>v><<<>vvv<>v<>><^<^v<>^vv>^^v<v<v><^<>>vv<^>>^>>vv^v<vv^vv<^<<>>^v^<>^>>>>vv>^^>v>vv>v><^vv^<<v>^<<^^<v<v>vv<v^^<>^^v>^>>v><^<<vv<<v^vv^^^v>>v<<v^><vv^><vv<^vv<<vv^v<<^v<^^v>><<v^>>^^<>v>><<v<>>^^<v>>^^>>vvv^><<<<<^<^vv<^<><v<<>^^^<<<^>^^^<v<<vv>vv<>^<>v<^v>^<<<v<v<v>>^v<>>v<<^<<v<<>^<<<><><>^>>>>^>v^v<<v<v<<>>vv<^vvv^^^^<vv>vv>^v^^v^<v^v><^vv<^vv>v<^>vv<>>^>^><vv<><^>v>^v>vvv<>^>^v<><>vv>><^v^<><><v>>v^v^><^<^>vv>v<^>vvv>v<<<<<^<v<<vv<^^^<<>>^v<vv<^<>v>^<v<>><><>^<<v>v^>^<vv>><><>>^>^>><^<v>^^>^^>^^v^^<^v^^>v^^>>><<><v<v<<v^vv<><><>^<v>^<<^^v^>v>><>^^^><^vvv<^^^^^v><<><v<^^v><><>>^>vv<vvvv<<>>><v<^^^^v<<^><v>^vv<v^^v^vv<^^>^^<v>><<v^>v<^^>^<^<v<^^v>^<<v>^>>>^v<>v<^^^>vvv^v<<^><>>><vvv^<^^^<^>>v>>><v>^^vvv^vvv<^^^^v^v^<vv^<v>^<<^>v^v^<<><>><^v><v<><<>><<<>^v>v<>^<v^v>^vv>>^<>v^^<<v><^v>>v<>>^v^^>><^>v^<^v^^>><>v^>^v^v<<<v^<v^^v<^>v<><>vv>>>>^>v<>v<<<>^^>vv^v<><v^<>^<<<<>>^^>^v<v^v<<><>^v<>>^v^<<^<^>>>^vv<><v<^^<>v^>>v<^^v<v>>>^>><<><<<>><vv<v>>^v>><^<v><vv>^vv<v<>>><>v^><>vv<^^v^^^v<>><^vvv<<^<>v>>>v>><v><>>><>><v^><v^v<v>^v>v<v>>^^<^>^>v><>vv>^v><<>>>>>>>^<<^vv^^vvvv<^^><<<v<<>vvv<>^><<v<v^v^<<v>v<>>^<vv^<v<v>^<<^^vv>v>^<vv<<>v<v^<>v>>^v^^vvvv>^^>>v^v^^><<^>v>>^^>^<^^<>v<v>vv^vv>v<v>>^v<><^vv^<vv<v^^^v<^v^>>^v>>>^^<^<^>^v^>^>>>^v>^>^^^>>^<>v^^<>^v<<^^>^^<vv<>v<^v^>><^v^>^<>>^vv^vv^>v^<vvvvvv^>><^^<^v<^<v^<<^^<<v^<^>><>v><^v^v^^^v>v^<>^<<v<^^vvv<v>^^>^v^^<><vv^v^>v^<<>>vv<>>>>v>v<>^>>>v<>^^><v<v^^^<>^<^><>^><<v>><>^<<>>><<^<vvv<^><v>>^vv^v>><v<>vv^<<^^<<><v><<^<v<vv<<^v^vv>v^>>>v<<<<v<<>v>^vv<^v><v<v>v<^>^^vv>v><v>><<v<<v^v>>><>^<>><><<^<<^v^v<<v>v>v<v<^^>vv<^v^^^<v^<<<v<>v^><^v>^<^<v>>^<<<v>>v^<><>>^v<>vvv<vvvvv<^^><^>><^^>^>^v^vv<^><<^v>><^^v>^v<>^>vvvv><^>^<<v^^vv<v^^<><>v>^>>^<^<<<^v^^^>^>>^>><><<^>v^^<v>>v<<<<vvv<vvvv^<^<v^^<>^>vvv^<vv^v^v>^<<><v><^v^v^^^>^^>^vv<>v>>v^>vv^vv>v<^v^^>>^v^v<>>^^><<v<<>><>>>^>^<>^^v^^><^<>><<^<vv^^^^^>>vv^<v^<^>>>>v<<><<^>vv>vvv>^<><>>>>vv><<v^v<^^^<<^^^vv^<v<><><<<<>><<v^<>v>v^><>v^v^^><>v>v>^^v<^v<>>^^^^^<v>><v^>^^<v>><v^^>v<^<^>>>^><^^>><<>>^><>^^^>v^^^>^^v^<>^^><^>>><><^>>v<v^>v<^><v<v^<>v<^v>v^<^vv^^><<<><><^v^<v<^^>v>v^>>^^vv^<v>^v>^<^v<>^>^><^<v>^v><^<^<>v^^>^><>>><<v><<><>v<<^v^^<^><>^<><><v>v<^^<v<v>>^^<<>>^<v>><^><^<^>^^v<>v>>><><<>^>v><><<<<v^^^^v<>>^^^v>><<^v>^>>><vv^>>^vv<^<>>^<^^<^v>v<v<<<<<>^<<^<<<<<^<^>>^><<>><>v^v>^<^>v^<><vvv^>^v^v^v><^<v<>vv<<^<>^^^<>^v>^<v^^<v^v>v<>>^>v<<>v<>v^v>v<<<>>v>vv>>v<<>v<>v<^>^>^<v>>v>^>^^^<vv>v<<>>><v>^vvv^^>^^<^vv^^^^>v>^v^>v^^v^>>^v>^vv>^^v^<<<<>^<><^<^<<^^>v^^^v<>>vvv<v>>vv><v<v>^<^v>>^v<vv^<<v<vv><^^v^v>v<>^v<<<^^v^^^<^v>v^v^v>><vvv<<>v<>^v>vv^v>vv<<^v<v>^v>v>><^v<v<>v>>>><<<><vv><>^v^<^vvv>v<>><^v>^>><v>vv<><><>v><>>><^>vv>>^<>v^>>^><<<^><<>^v^>>><><>vv>^<>^>^v^^><^>>><<>v^<^vv>^<^vv>><v<>vv<v><><<^><>v<^^<^>vv^^^^vv<<v><>vv<><v>v<>>>>^><v><>^<><>v<>><<>^^vvv>^^^<><>>vvv^v>><>vv<vv>^^^v^<<>^^v<><<^^v<>^^>^<^^v>>v^v^^>>v>>>^<<^<>^>^^v>>>><vv<<>^v<<vv><<^^vv><^>vv<>>v<v>v^>v>>v^<vv<<<v><v^>vvv^^>vv^<<v>v^>>v^<>>><><<^^<^v>^>>>v>v>^v<>vv><vv<vvv<<v>v>^v<<<>><<><><>v^>>>v^>v^>>vv^^<v>^<>>><^>v^<>^^><v>v<><<<><v^v<<<v<v^>v^v>^>v<^<>v>v^^>>v>vv^v<>>^^^^<>v^>>>>>>>><v<^<<vvv<^v^>^v<^<<>>><<<^<<^>^>v^<>^<<<>v>><^vv^>^>^>>>^<vv><v^^^<v^<v<><v^vvv<>v<vvv^vv<<<v^<^<^vvvv^<<vv<^v><<>^>^<v^v^<^>v^><>>v^>v^>^>>v<>vv^v<<>^^>>vv<>vv>>^v<^vv>^v>v<v^vvv^<<^><>v^<><vv><>v^^><<<><>^>^v^<>><vv<^>v^v>v<>><v<<^>^<vv<^v>^<<v><^<^^vv^<>><v^>^vv^<>>^^^^v>v><^^^v^<<<>^<^<<>><>>v<<^v^>><><v^>>^vv^v>vv>>>>>>^^<<>v^>v^v>^^>>><vv^^^v>^v>>^^^<>><>v^<<<v<vv^^<v^<<<>v>v^^^<vv<>>^v>^v<^<<><>vv>^^^<^^vv<v<<vv>^^>vv>v<<^>^vv><^><v>^^^^v<<vv>v^<<^^>>^^vvvv^v^>vv>>v^<v>vvv<>>^><>>v^^>>^<>>vvvv^>><v^v<^^<^vv>>v<<^<<^><v^^><v^>v^>><<<v>v>v^>^v<v^vv<^^^v<^<vvvvv<<vvv>><>v<v<v<<^v<><<>vv>><v>><^>>^^v>^>><>vv^><<>>vv<<<^<^^>^<<^>>>><v<^v<<<>>v>vv<^>^v><>>v<v^v<>v^vvvv>v^>>v><<^<v>^^v>>vv^^>v>^v>^v^^>^<^vv<v<<^>vv<<^>>^<<^^>>^<^>v^><^vv>^^v><v^>>><>v^v>^v<^><<<>vv><v>v<><>>v^<>^^>^<>^<<^>>vv^><^<v<^^vvv>>v^>>v^>v>vv><>>v<^>><<<v<<vv><v<v<v>v<v>vv^vvv^vv^>^>v><vv<v^^<>>>>vv^>^<>v<^>^<^v>vv<^<<>>^<^<vv><^^<>^<<v^v^>v<<><v>v>><^v<<^vvv>v>v<<^^<^^>v<vv<v<v^v>^^^>^>vv<v<<^^v^<v<^>^^^vv>v<>>>vv>><><^><><<<vvv<<^^v^<v^<<^>>vv>vv^v^>>><v><<v^v>>v>>vv>^^vvv^>^^>^>^>^v<<^vv^>vvv^^vv><^>^v^>^><>v<^^vv<v><v^<><^<>><v>^^v^v>v^vv<>><^v>^<^v>^<>^v>>>><<vv^^^vv^>>><vv^v>>v><^v^vv><<^v<<>^^<v><^v>vvv<><^^><<^v><>^<^v<^^<^vvvv^^>>>>vv>v>>>v<v^><<<<v>>v^><v>>vv^v<vv<>vv<>vvv>>>><>>><>^v<v^v><vvv<<v^^v^v<>>><>>^vv<<v<><<vv<v^>^^vv><^v^v<v^vvv^v>v^^^vv>^><^vvv<<>^vvv^<v<v^v>>>>^<<<><<<<<^v<^^>>>>^>^<v^^^v<vvv<vv^<>v<<<^<^>>v^<v><<><<^^vvv^>v<>>^^>v>^v>>v<v><v>>>>^<^<^>v^v<vv<>^>><>^<<^vvv^^<>^<vvv<>v^>^^<<^>^vv><vvv>>v^v^>v><v>^<^^<>^>^>>>^^vvv^<<>v^<<>><>v<^<^>v^>^vv><v<^<<<^v>^>>^<^v^<<<<^v^><v^v>v^><<v<><<v^<<^<<v<<v><v><><^^^^>v>^^<v>>v<vvv<<<>><>>^><<><^<>>^^>vv<^><^v^><vvv>>>vvv<<vv^<^^^<^>^<>>^>>^v^<^^v>^<v<<>^^v<^vv^><vvv>>^v><<^<v^<><><>>^>vv<<>^^^v^^<v<>><>>vv>v^>vvv^^v<vv<^<^>>^>>^>>v^<<<v^>v^<^v^vv^><^<^v<<v<<>v>^v^<<<v^vv<v<<>^^<v>>>^<v<^>^^v<v>>>><vv<^^<<>><<v<v>^^v^>>^^>>^v^<^v>v^v^v^v^>v^vv<><>^^<>^><^^^<<<^<v>v<<>^<^^^^^v^<^<<^^>^vv<>v^>><>>^>v>v<>^>v<v^>>><>^<><v>>>^>^>>v^><v<>v><^vv^>v<<v>v<><<vv<<v>^><^<v^>v<<v^v<<><v><>v<v><>^^<v<>><<>v>vv<<v>^v<v>vv><><>vv^<<>^>^<^>>>^v>v<^v^^^vv<>>>^<<^>>><<^^v^>v^<^v>vvv>v^^vv>^^>>v<>^<<>^<><^^v^>><>^>v>>^^^<<^^v<>^^>^<>^>><^>^vvv><^>^<^>^>>vv<^>>^v>>^<>>^^>>>v^<v>>v<<v<^>>v^^vv>v><^v^^><vv^v<^>v<<>v^^<><>^>vvv><^^^>^v^>v>>^vvv<^vv>^^>^>>v<>><<^v<<v^>^><>vv^<<^^vv><v>>^<^><^<v>^v<v>^<<>^v^^>v^>>^^^<^vv>v^>>>vv<<>v>>>^>v^^<v^v^^v^>>v<v<<v>^<<>>vv<<^v>v<<vv<<^<^v<^<><^^>v>>v>v^>><vv<^v<^>^>>v>^><<^<<>^v<v>>><^^<^<<<v^^>^>vv<<>^<>^<v^<<^v>vv>^^^v<^v><v<<<<<vv>vv>^^^^>v>v><<^<<<^vv><^<<<><v>><v^v>v<<v^^<v^>v>^v^v^<^<^vv>vvv<^^v<>v<<<<>v<v^<vvv^^^<<^<^<<>^<<><<<>v<^>^^v<^^v^>vv>vvv>v><v^^<<>>^><^>>v<<vv>v<<^^^v<<^v^^><><<<><<>v>^<<>v<<<^v>><v^v<^v<v^vv>v>><<^<><^v^^v<v>^>^>vvvv<<><<>>^<vv>^^><v<>v>v<v^^>^><>>><^><<><<<^<>v^><vv^^^^>>^v^>v^<>>v>^^><^<^v^<v^>>v>^vvv<>>v<v^v><>^vvvv<v^<<v^<<^^vv>><<<<<<v><<<v<v^v^^<v^^<>v<<<<^v<<><<v^<^><v<vv<v^v^<v^^vv<v^v<<<>^<<>vv<v<^>^<<><vv<<vv<v<^<^<>><^^<<>>>vv>>>>>>^v<v<>>v^v^^<v^<<<<>><<^v^^^<>^<vv>>>><>v^v^vvv^>>v>><v^v<<<^v>>^^<<^^vv><<<^^^<<<v><^^>>>>vvv^v<^>^^>v<^<><vv<v<>v>>>^vv<<^<v>^v^>^>^v>v>v^v^>v<<v>><>><v^^<<^>>>><<^v^<>^v<vv><>vvv^>v>v<v<v^>^<><><>^>>><v<<<v^vv><>^>^^<<v^>>v^^>^<v>><>><>v^v^^v>>>>vv>>^v<<^v^<>^>v^^>^^<<vvvvvvv>^<v^<<^<<>><<<^^^v^^^^v<^<>v<^^<>vv^^v^<>^<<^>>v>v<<<^^^^vvv^<^<><>v<<v^<^<>>><<><<<v<v<v><vv>^^<vv<<vv<<<v<^>^^vv<v<>><<>>>^v<<>^>>>v^>v>^^<>^<vv<><^>v>^>>>><>^^>v^^v>^vv^^v^><<<>>v<>v<vv<vv^v^v<^v^<^^><<<><vv^^>^<^<<>v>>>>^<<v>v<v>vv<^><^<v><<^>v>>v><<v<<^v^<>>^>>>^v^v>v^^vv^>^<^^>>^><^vv^^vv^<>>^^^^<^^><><v<>>^>>^><vv^>^vvv<^<<v^^<<<>^><>>>^^<><v<v<><<v^^^^^<^<^<<>><<>>>>^<<>>>^<^v^>><<^>>>^<<v>^>><>^<v>^<><v>^v^^vv<><^>vv^^v^<^^^v^vvv^>><>>v<<vv<>>^<^vvv<<^^><vvv^^<v<>vv^^<<>><v>><^^vvv<<<^>^<><^>vv^><^<<>vv<<v>>vv>v>v^<vv><vv><<>^^^^v^^^^<v>^<<^><><^^v^>v>^>><^><<>v^<v>>>^vvv>>^<^<>^^v^vv^^v><<vv^<>>>v<<<>v>^<>v<<>v^>^<<><<><v<v<v<>v^>v<><^^>^<^v^^><^>vv>^>vv<v<^v>vv>^^><<>vv^>^v<<^<<^<<>v<v<^<v>v>>^><v^^v^v>>>><v^v^<<<vv<<^^<>>v^v<^v>v>^^^v<v><v^^^vv<>v^v<^<>v><><v^<>>vv>v><>v>^v<><<<<<<v<>>v^vv<<<<v<<v><^<>^>><>^^vv>^<^<<>vv>>vv<vvv>><><v<>><^<v>^><^<<v>><v><v>^<v>><>v^^^^v<v^^v<>^^vv<>v<>v>^vv^><v^<<^<>^<>^^^>v^>>>v><<^>>v<^v<>^^<v<><v^v<v>v<><v<vv><<>v<^<^>v<>v^>v>^^<<<^^vv^<><<<>>v>^^<>v>>>><v<v<^^^v<v<v^><<>v^v<>v>><<<<v^<><^<<^>^<vvv<v^^v>>v^vv^><^v^^<>^^><<v^>>vv>^<v^vv<^^v<>>vvv<^v^>>^<v<v>>^>^^<<^>^>^v><>>^<^^v>^>>^^<><>>>^^>^^vvv>v<^^<>v^v^^<v<<^<v^v^<<>v^v<v<<v<>>><<^^^>>v>^vv>^>^^v<>^^<>v^^<><v<v<vvv^<vv<<>v^><<><v<>vv<<^vvvv><<<v>v>v^>v^<>v^>^<v<vvv^>^<>^>^^v<>><<<><v<^^>^v<v>^^v^v<<<^v^<>^<>v>^^>v<v<v>v>^^<<<><<^>v<v<^vv^v><^^<<vv>^<<v><>^>>>>><v^v<<<^>^v^v<<v<>vvv<<>v>v>>^v^v^>><<<<>v^<v<><<>>>^>>^>><<v>";