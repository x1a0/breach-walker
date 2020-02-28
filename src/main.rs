use std::env;

const BREACH_WIDTH: usize = 67;
const BREACH_HEIGHT: usize = 33;

const BREACH_MAP: &str = "\
                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
                          ^^^^^^^^^.^^^^^^^.^^^^.^^^^^^^^^^^..^^.^^^^^^^^^^^^^^^.^^^^^^^^^^^^\
                          ^^^^^^^^^..^^^^^^^.^^^.^^^^^^^^...^^..^^^^^^^^^^^^^^^^^.^^^^^^^^^^^\
                          ^^^^^^..^^^.^^^^^^.^^...^^^^^^.^^^^^^^^^^^^^^^^^^^^^^^^^.^..^^^^^^^\
                          ^^^^^.^^...^^^^^^^^..^^^.^^^^^..^^^^^^^...^^^^^^^^^^^..^^.^.^^^^^^^\
                          ^.^...^^^.^^^^^^^^^^^^^^.^^^..^^..^^^...^^...^^^^^^..^^..^^^..^^^^^\
                          ^^.^^^.^^^^^^^^^^.^^^^^^^...^^^^^^.^..^^^^^^..^^^^..^^^^^^^^^^.^^^^\
                          ^^^^^^^.^^^^^^^^^^...^^^^^^.^^..^^..^^^^^^^^..^^^^.^^^^^^..^^^^.^^^\
                          ^^^^^^^^.^^^^^^^^^^^..^^^^^^..^^.^^^^c^^^^..^^^^^^^..^^^^.^^^^^..^^\
                          ^^^^^^^^...^^^^^^^^^^...^^^^^^^^^.^^ccc^^^.^^^^^^^^^^.^^.^^^^^^^^.^\
                          ^^^^^^^^^^.^^^^^^^^...^^..^^^^^..^^^^c^^^^.^^^^^^^^^^.^^^..^^^^^^.^\
                          ^^^^^...^.^^^^^^^^..^^^^^^.^^^.^^^^^^^.^^^.^^.^^^^.^..^^^^.^^^...^^\
                          ^^^..^^^.^^^^^^^^^.^^^^^^^.^^^..^^^^^..^^^^..^.^^.^.^^^^^^..^^.^^^^\
                          ^^^.^^^^^^^^^^^^^^^^^^^^^.^^^^^^.^^^^.^^^^^^^^.^^.^^^^^^^^^^..^^^^^\
                          ^^^^....^^^^...^^^^^^^^^^^..^^..^^^^^.^^^^^...^^.^^^^^^^^^^.^^^^^^^\
                          ^^^^^^^^.^^^^^..^^^^^..^^^^^...^^^^^^^..^^^^^^..^^^^^^^^^^^..^^^^^^\
                          ^^^^^^^^^.^^^^^^..^^^^.^^^^^^.^^^^^^^^.^^^^^^^^^^^^^^^^^^^^^^.^^^^^\
                          ^^^^..^^^^.^^^^..^^^^^.^^^^^^^.^^^^^..^.^^^^..^^^^..^^^^^^^^^^.^^^^\
                          ^^^.^^..^.^^^^^^.^^^..^^^^^^^^^...^^.^^^..^.^...^.^^.^^^^^^.^..^^^^\
                          ^^^^.^^^.^^^^^^^^...^^.^^^^^^^^^^...^^^^^^.^^^^^.^^^.^^^^..^.^^^^^^\
                          ^^^^^.^^^^^^^^^^^^^^^^^.^^^^^^^^^^^^^.^^^.^^^^^^^^^^.^^^..^^^^^^^^^\
                          ^^.^^.^^^^^^..^^^^^^^^^.^^^^^^^^^^^^.^...^^^^^^^^^^.^.^^.^^..^^^^^^\
                          ^.^..^^^^^^.^^.^^^^^^^^^..^^^^^^^^^.^^^^^^^^^^^^^^.^^.^^^...^.^^^^^\
                          ^.^^^^^^^^..^^...^^^^^^^^^.^^^^^^^^.^^^^^^^^^..^..^^^^..^^^^^^.^^^^\
                          ^^.^^^^^^^^.^^^^..^^^^^^^^^..^^^^^^^..^^^^^^.^^.^^^^^^^^..^^...^^^^\
                          ^^^.^^^^^^.^^^^^^.^^^^^^^^^.^..^^^^^^.^^^^..^^^^^.^^^^^^^^.^^^..^^^\
                          ^^^^.^^^^.^^^^^^^..^^^^.^..^^^^...^^^.^^^^.^^^^^...^^^^^^..^^^^^.^^\
                          ^^^^.^^^^^.^^^^^^^^.^^.^.^^^^^.^^^.^^^..^^^..^^.^^^..^^^.^^^^^^..^^\
                          ^^^^^.^..^^.^^^^^^^^..^^^^^^^^^.^^..^^.^^^^^..^.^^^^.^^^^^...^^.^^^\
                          ^^^^^^.^.^^^..^^^^^^^.^^..^^^^^^.^^^..^^^^^^^^.^^^^.^^^^^.^^^.^^..^\
                          ^^^^^^^^^..^^.^^^^^^^^..^^.^^^^^.^^^^^^^^^^^^^^^^^^^...^^.^^^^....^\
                          ^^^^^^^^^^^..^^^^^^^^^^^^^..^^^^^..^^^^^^^^^^^^^^^^^^^^..^^^^^^^^^^\
                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\
                          ";

fn main() {
    // Store map data in a 2 dimensional array with boolean values.
    // 0: obstacle
    // 1: passageway
    let mut map_data: [[bool; BREACH_WIDTH]; BREACH_HEIGHT] =
        [[false; BREACH_WIDTH]; BREACH_HEIGHT];

    for (i, c) in BREACH_MAP.chars().enumerate() {
        map_data[i / BREACH_WIDTH][i % BREACH_WIDTH] = c == '.';
    }

    let args: Vec<String> = env::args().collect();

    let (from_x, from_y) = parse_coord(&args[1]);
    if !map_data[from_y][from_x] {
        panic!("Where ARE you?");
    }

    let (dest_x, dest_y) = parse_coord(&args[2]);
    if !map_data[dest_y][dest_x] {
        panic!("Let me know when you find a way there.");
    }

    if from_x == dest_x && from_y == dest_y {
        panic!("Yeah that's funny.");
    }

    let mut steps: Vec<&str> = vec![];
    let mut visited: Vec<(usize, usize)> = vec![];

    let _ = dfs(
        from_x as i32,
        from_y as i32,
        dest_x,
        dest_y,
        &mut steps,
        &mut visited,
        &map_data,
    );

    print!("{}", steps.join(";"));
}

fn dfs(
    from_x: i32,
    from_y: i32,
    dest_x: usize,
    dest_y: usize,
    steps: &mut Vec<&str>,
    visited: &mut Vec<(usize, usize)>,
    map_data: &[[bool; BREACH_WIDTH]; BREACH_HEIGHT],
) -> bool {
    if from_x as usize == dest_x && from_y as usize == dest_y {
        return true;
    }

    if from_x < 0 || from_x >= BREACH_WIDTH as i32 || from_y < 0 || from_y >= BREACH_HEIGHT as i32 {
        // out of bounds
        return false;
    } else if !map_data[from_y as usize][from_x as usize] {
        // hit obstale
        return false;
    } else if visited.contains(&(from_x as usize, from_y as usize)) {
        // been here
        return false;
    }

    visited.push((from_x as usize, from_y as usize));

    let directions = [
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
    ];

    for (x, y) in &directions {
        let dir = xy_to_dir(*x, *y);
        steps.push(dir);

        if dfs(
            from_x + x,
            from_y + y,
            dest_x,
            dest_y,
            steps,
            visited,
            map_data,
        ) {
            return true;
        }

        steps.pop();
    }

    false
}

fn parse_coord(coordinate: &String) -> (usize, usize) {
    let split: Vec<&str> = coordinate.split(":").collect();
    let x: usize = split[0].parse().unwrap();
    let y: usize = split[1].parse().unwrap();
    (x, y)
}

fn xy_to_dir(x: i32, y: i32) -> &'static str {
    match (x, y) {
        (-1, -1) => "nw",
        (-1, 0) => "w",
        (-1, 1) => "sw",
        (0, -1) => "n",
        (0, 1) => "s",
        (1, -1) => "ne",
        (1, 0) => "e",
        (1, 1) => "se",
        _ => panic!("Illegal x,y input {}:{}", x, y),
    }
}
