use itertools::Itertools;

#[derive(Debug)]
struct SecureArea {
    width: usize,
    height: usize,
    robots: Vec<Robot>,
}

type Vector = (isize, isize);

#[derive(Debug)]
struct Robot {
    position: Vector,
    velocity: Vector,
}

impl SecureArea {
    fn from_input(input: &str, width: usize, height: usize) -> Self {
        SecureArea {
            width,
            height,
            robots: input.trim().lines().map(Robot::from_input).collect(),
        }
    }

    fn tick(&mut self) {
        for robot in &mut self.robots {
            robot.tick(self.width, self.height);
        }
    }

    fn might_be_tree(&self) -> bool {
        let mut consecutive = 0;

        let mut positions: Vec<Vector> = Vec::from_iter(self.robots.iter().map(|r| r.position));
        positions.sort_unstable();

        for (r1, r2) in positions.iter().tuple_windows::<(_, _)>() {
            if r1.0 == r2.0 && r1.1.abs_diff(r2.1) == 1 || r1.1 == r2.1 && r1.0.abs_diff(r2.0) == 1
            {
                consecutive += 1
            }
            if consecutive > 100 {
                return true;
            }
        }

        false
    }

    fn render(&self) {
        for y in 0..(self.height as isize) {
            for x in 0..(self.width as isize) {
                if self
                    .robots
                    .iter()
                    .any(|rob| rob.position.0 == x && rob.position.1 == y)
                {
                    print!("A");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }

    fn calculate_safety_factor(&self) -> usize {
        let middle_x = (self.width / 2) as isize;
        let middle_y = (self.height / 2) as isize;

        let quadrant1 = self
            .robots
            .iter()
            .filter(|robot| robot.position.0 < middle_x && robot.position.1 < middle_y)
            .count();
        let quadrant2 = self
            .robots
            .iter()
            .filter(|robot| robot.position.0 > middle_x && robot.position.1 < middle_y)
            .count();
        let quadrant3 = self
            .robots
            .iter()
            .filter(|robot| robot.position.0 < middle_x && robot.position.1 > middle_y)
            .count();
        let quadrant4 = self
            .robots
            .iter()
            .filter(|robot| robot.position.0 > middle_x && robot.position.1 > middle_y)
            .count();

        quadrant1 * quadrant2 * quadrant3 * quadrant4
    }
}

impl Robot {
    fn from_input(input: &str) -> Self {
        let parts: Vec<&str> = input.split(" ").collect();
        let position = parts[0].split("=").nth(1).unwrap();
        let velocity = parts[1].split("=").nth(1).unwrap();

        Self {
            position: parse_vector(position),
            velocity: parse_vector(velocity),
        }
    }

    fn tick(&mut self, width: usize, height: usize) {
        self.position.0 = (self.position.0 + self.velocity.0 + width as isize) % width as isize;
        self.position.1 = (self.position.1 + self.velocity.1 + height as isize) % height as isize;
    }
}

fn parse_vector(input: &str) -> Vector {
    let parts: Vec<isize> = input.split(",").map(|n| n.parse().unwrap()).collect();
    (parts[0], parts[1])
}

pub fn day14_part1(input: String) -> usize {
    day14_part1_with_size(input, 101, 103)
}

fn day14_part1_with_size(input: String, width: usize, height: usize) -> usize {
    let mut secure_area = SecureArea::from_input(&input, width, height);

    for _ in 0..100 {
        secure_area.tick();
    }

    secure_area.calculate_safety_factor()
}

pub fn day14_part2(input: String) -> usize {
    let mut secure_area = SecureArea::from_input(&input, 101, 103);

    let mut i = 0;
    loop {
        i += 1;

        secure_area.tick();

        if secure_area.might_be_tree() {
            secure_area.render();

            break;
        }
    }

    i
}

#[cfg(test)]
mod test {
    use crate::day14::day14_part1_with_size;

    #[test]
    fn test_part1() {
        assert_eq!(
            12,
            day14_part1_with_size(
                r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#
                    .to_string(),
                11,
                7
            )
        )
    }
}
