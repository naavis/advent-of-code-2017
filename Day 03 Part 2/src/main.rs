const GRID_SIDE_LENGTH: usize = 999;

fn main() {
    // Read input number
    let args: Vec<String> = std::env::args().collect();
    let input_string = args.get(1).expect("Input missing!");
    let input: u64 = input_string.parse::<u64>().expect("Invalid input!");

    // Use fixed grid even if it's ugly. :(
    let mut grid = vec![vec![0u64; GRID_SIDE_LENGTH + 2]; GRID_SIDE_LENGTH + 2];
    grid[GRID_SIDE_LENGTH / 2 + 1][GRID_SIDE_LENGTH / 2 + 1] = 1;
    let mut i = 1;
    loop {
        let (x_orig, y_orig) = spiral_index_to_xy(i);
        let x: usize = (1 + x_orig + (GRID_SIDE_LENGTH / 2) as i64) as usize;
        let y: usize = (1 + y_orig + (GRID_SIDE_LENGTH / 2) as i64) as usize;
        grid[x][y] += grid[x + 1][y];
        grid[x][y] += grid[x + 1][y + 1];
        grid[x][y] += grid[x][y + 1];
        grid[x][y] += grid[x - 1][y + 1];
        grid[x][y] += grid[x - 1][y];
        grid[x][y] += grid[x - 1][y - 1];
        grid[x][y] += grid[x][y - 1];
        grid[x][y] += grid[x + 1][y - 1];
        if grid[x][y] > input {
            println!("Result: {}", grid[x][y]);
            break;
        }
        i += 1;
    }
}

/// Convert spiral index to x, y coordinates.
/// The basic idea is from https://math.stackexchange.com/a/163101
/// which seems more sane than my solution to Part 1 of Day 03.
fn spiral_index_to_xy(index: usize) -> (i64, i64) {
    let n = index as i64;
    let k = (((n as f64).sqrt() - 1.0) / 2.0).ceil() as i64;
    let t = 2 * k;
    let mut m = (t + 1) * (t + 1);
    if n >= m - t {
        return (k - m + n, -k);
    } else {
        m -= t;
    }

    if n >= m - t {
        return (-k , -k + m - n);
    } else {
        m -= t;
    }

    if n >= m - t {
        return (-k + m - n , k );
    }
    (k, k - m + n + t)
}
