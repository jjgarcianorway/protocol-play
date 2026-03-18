// SPDX-License-Identifier: GPL-3.0-or-later

use super::types::*;

/// Check for Ronda (2 same-value) or Rondin (3 same-value) in a hand.
/// Returns: (is_ronda, is_rondin, matching_value)
pub fn check_ronda(hand: &[Orb]) -> (bool, bool, Option<u8>) {
    if hand.len() < 2 {
        return (false, false, None);
    }
    // Count values
    let mut counts = [0u8; 11]; // index 1-10
    for orb in hand {
        counts[orb.value as usize] += 1;
    }
    for val in 1..=10u8 {
        if counts[val as usize] >= 3 {
            return (false, true, Some(val)); // Rondin
        }
        if counts[val as usize] >= 2 {
            return (true, false, Some(val)); // Ronda
        }
    }
    (false, false, None)
}

/// Attempt to capture from the table by playing an orb with given value.
/// Returns indices of captured orbs from the table (sorted).
/// Captures the matching value + consecutive values above it.
pub fn find_captures(table: &[Orb], played_value: u8) -> Vec<usize> {
    // Find all orbs on table that match the played value
    let matching_indices: Vec<usize> = table.iter().enumerate()
        .filter(|(_, orb)| orb.value == played_value)
        .map(|(i, _)| i)
        .collect();

    if matching_indices.is_empty() {
        return Vec::new();
    }

    // Collect all values on the table for consecutive check
    let mut table_values: Vec<(u8, usize)> = table.iter().enumerate()
        .map(|(i, orb)| (orb.value, i))
        .collect();
    table_values.sort_by_key(|&(v, _)| v);

    // Start with the matching orbs, then add consecutive values above
    let mut captured: Vec<usize> = matching_indices.clone();
    let mut next_value = played_value + 1;

    loop {
        let consecutive: Vec<usize> = table.iter().enumerate()
            .filter(|(i, orb)| orb.value == next_value && !captured.contains(i))
            .map(|(i, _)| i)
            .collect();
        if consecutive.is_empty() {
            break;
        }
        captured.extend(consecutive);
        next_value += 1;
    }

    captured.sort();
    captured
}

/// Check if capturing would result in mesa limpia (empty table).
pub fn is_mesa_limpia(table_len: usize, captured_count: usize) -> bool {
    captured_count > 0 && captured_count == table_len
}

/// Check if player has a matching orb for "se cayo" reaction.
pub fn find_se_cayo_matches(hand: &[Orb], value: u8) -> Vec<usize> {
    hand.iter().enumerate()
        .filter(|(_, orb)| orb.value == value)
        .map(|(i, _)| i)
        .collect()
}

/// Determine game winner. Returns (player_wins, npc_wins, is_tie).
pub fn determine_winner(
    player_captured: i32,
    npc_captured: i32,
    player_treasure: i32,
    npc_treasure: i32,
) -> (bool, bool) {
    let player_total = player_captured + player_treasure;
    let npc_total = npc_captured + npc_treasure;
    (player_total > npc_total, npc_total > player_total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ronda_detection() {
        let hand = vec![
            Orb { value: 3, color: OrbColor::Orange },
            Orb { value: 3, color: OrbColor::Cyan },
            Orb { value: 7, color: OrbColor::Pink },
        ];
        let (ronda, rondin, val) = check_ronda(&hand);
        assert!(ronda);
        assert!(!rondin);
        assert_eq!(val, Some(3));
    }

    #[test]
    fn test_rondin_detection() {
        let hand = vec![
            Orb { value: 5, color: OrbColor::Orange },
            Orb { value: 5, color: OrbColor::Cyan },
            Orb { value: 5, color: OrbColor::Pink },
        ];
        let (ronda, rondin, val) = check_ronda(&hand);
        assert!(!ronda);
        assert!(rondin);
        assert_eq!(val, Some(5));
    }

    #[test]
    fn test_capture_with_consecutive() {
        let table = vec![
            Orb { value: 3, color: OrbColor::Orange },
            Orb { value: 4, color: OrbColor::Cyan },
            Orb { value: 6, color: OrbColor::Pink },
            Orb { value: 7, color: OrbColor::Purple },
        ];
        let captured = find_captures(&table, 3);
        assert_eq!(captured, vec![0, 1]); // captures 3 and 4 (consecutive)
    }

    #[test]
    fn test_no_capture() {
        let table = vec![
            Orb { value: 3, color: OrbColor::Orange },
            Orb { value: 5, color: OrbColor::Cyan },
        ];
        let captured = find_captures(&table, 7);
        assert!(captured.is_empty());
    }
}
