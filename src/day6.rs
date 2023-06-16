use std::{collections::HashSet, fs::read_to_string, path::Path};

fn read_signals(path: &Path) -> Result<Vec<String>, &str> {
    let lines = if let Ok(lines) = read_to_string(path) {
        lines
    } else {
        return Err("Error reading file.");
    };

    let mut codes: Vec<String> = Vec::new();
    lines.lines().into_iter().for_each(|line| {
        codes.push(line.to_string());
    });

    Ok(codes)
}

fn index_after_marker(signal: &String) -> usize {
    signal
        .char_indices()
        .into_iter()
        .try_fold(Vec::new(), |mut marker, (index, char)| {
            marker.insert(0, char);

            if marker.len() > 4 {
                marker.pop();
            }

            let mut unique: HashSet<char> = HashSet::new();

            marker.iter().for_each(|char| {
                unique.insert(*char);
            });

            if unique.len() >= 4 {
                return Err(index + 1);
            }

            Ok(marker)
        })
        .unwrap_err()
}

pub fn day6() {
    let signals = read_signals(Path::new("signals.txt")).unwrap();

    signals.iter().for_each(|signal| {
        let index = index_after_marker(signal);
        println!("{index}");
    });
}
