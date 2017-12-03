fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_string = args.get(1).expect("Input missing!");
    let input: u64 = input_string.parse::<u64>().expect("Invalid input!");

    println!("Distance: {}", distance(input));
}

/// Calculates the Manhattan distance of a given index
fn distance(input: u64) -> u64 {
    if input <= 1 {
        return 0;
    }

    let ring = get_spiral_ring(input);
    let ring_min = get_ring_min(ring);
    triangle_wave(input as i64 - ring_min as i64 + 1, 2 * ring as i64, ring as i64) + ring
}

/// Generates sample from a triangle wave varying between 0 and 2 * amplitude.
/// The wave starts from the value 2 * amplitude.
fn triangle_wave(i: i64, period: i64, amplitude: i64) -> u64 {
    let a = ((i % period) - period / 2).abs();
    ((a - period / 4) * 2 * amplitude / period + amplitude / 2) as u64
}

/// Calculates the ring number for a given index.
fn get_spiral_ring(index: u64) -> u64 {
    let a = (index as f64).sqrt();
    ((a - 1.0) / 2.0).ceil() as u64
}

/// Calculates the maximum index for a given ring
fn get_ring_max(ring: u64) -> u64 {
    4 * ring * ring + 4 * ring + 1
}

/// Calculates the minimum index for a given ring
fn get_ring_min(ring: u64) -> u64 {
    match ring {
        0 => 1,
        _ => get_ring_max(ring - 1) + 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! parameter_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (function, input, expected) = $value;
                assert_eq!(expected, function(input));
            }
        )*
        }
    }

    parameter_tests! {
        spiral_ring_test_01: (get_spiral_ring, 1, 0),
        spiral_ring_test_02: (get_spiral_ring, 5, 1),
        spiral_ring_test_03: (get_spiral_ring, 9, 1),
        spiral_ring_test_04: (get_spiral_ring, 10, 2),
        spiral_ring_test_05: (get_spiral_ring, 20, 2),
        spiral_ring_test_06: (get_spiral_ring, 25, 2),
        spiral_ring_test_07: (get_spiral_ring, 26, 3),

        distance_test_01: (distance, 1, 0),
        distance_test_02: (distance, 12, 3),
        distance_test_03: (distance, 23, 2),
        distance_test_04: (distance, 1024, 31),
    }
}
