/// Test program to validate the new coast detection logic
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
enum TerrainType {
    Plains,
    Hills,
    Ocean,
    Coast,
}

fn determine_coast_tile_index(
    has_north_ocean: bool,
    has_east_ocean: bool,
    has_south_ocean: bool,
    has_west_ocean: bool,
) -> (u32, bool) {
    // Determine tile index based on ocean neighbor pattern
    let (tile_index, should_convert_to_coast) = match (
        has_north_ocean,
        has_east_ocean,
        has_south_ocean,
        has_west_ocean,
    ) {
        // Ocean on all sides: tile index 2
        (true, true, true, true) => (2, true),
        // North, east, and south ocean: tile index 1
        (true, true, true, false) => (1, true),
        // East and south ocean: tile index 9
        (false, true, true, false) => (9, true),
        // Only south ocean: tile index 8
        (false, false, true, false) => (8, true),
        // Any other ocean adjacency - use default coast index (8)
        (n, e, s, w) if n || e || s || w => (8, true),
        // No ocean neighbors - keep as land
        _ => (0, false), // Use 0 for land tiles
    };

    (tile_index, should_convert_to_coast)
}

fn test_coast_patterns() {
    let test_cases = vec![
        // (north, east, south, west, expected_index, expected_convert, description)
        (false, false, true, false, 8, true, "Only south ocean"),
        (false, true, true, false, 9, true, "East and south ocean"),
        (
            true,
            true,
            true,
            false,
            1,
            true,
            "North, east, and south ocean",
        ),
        (true, true, true, true, 2, true, "Ocean on all sides"),
        (true, false, false, false, 8, true, "Only north ocean"),
        (false, true, false, false, 8, true, "Only east ocean"),
        (false, false, false, true, 8, true, "Only west ocean"),
        (true, true, false, false, 8, true, "North and east ocean"),
        (true, false, true, false, 8, true, "North and south ocean"),
        (false, false, false, false, 0, false, "No ocean neighbors"),
    ];

    println!("Testing coast detection patterns:");
    println!("=================================");

    for (north, east, south, west, expected_index, expected_convert, description) in test_cases {
        let (actual_index, actual_convert) = determine_coast_tile_index(north, east, south, west);

        let status = if actual_index == expected_index && actual_convert == expected_convert {
            "✅ PASS"
        } else {
            "❌ FAIL"
        };

        println!(
            "{} {}: N:{} E:{} S:{} W:{} -> Index:{} Convert:{}",
            status, description, north, east, south, west, actual_index, actual_convert
        );

        if actual_index != expected_index || actual_convert != expected_convert {
            println!(
                "     Expected: Index:{} Convert:{}",
                expected_index, expected_convert
            );
        }
    }
}

fn main() {
    test_coast_patterns();
}
