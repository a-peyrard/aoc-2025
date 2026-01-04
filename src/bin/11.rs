use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let devices: HashMap<&str, Vec<&str>> = input.lines()
        .map(|line| line.split_once(':').unwrap())
        .map(|(device, raw_output)| (device, raw_output.trim().split_whitespace().collect::<Vec<_>>()))
        .collect();

    let mut queue: Vec<&str> = vec!["you"];
    let mut paths = 0;
    while let Some(device) = queue.pop() {
        if device == "out" {
            paths += 1;
            continue;
        }
        if let Some(values) = devices.get(device) {
            for value in values {
                queue.push(value);
            }
        }
    }

    Some(paths)
}

pub fn part_two(input: &str) -> Option<u64> {
    let devices: HashMap<&str, Vec<&str>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.split_once(':').unwrap())
        .map(|(device, raw_output)| {
            (
                device,
                raw_output.trim().split_whitespace().collect::<Vec<_>>(),
            )
        })
        .collect();

    let mut cache: HashMap<(&str, bool, bool), u64> = HashMap::new();
    let count = count_path("svr", false, false, &devices, &mut cache);

    Some(count)
}

fn count_path<'a>(
    device: &'a str,
    dac_seen: bool,
    fft_seen: bool,
    devices: &HashMap<&'a str, Vec<&'a str>>,
    cache: &mut HashMap<(&'a str, bool, bool), u64>,
) -> u64 {
    if let Some(&cached_count) = cache.get(&(device, dac_seen, fft_seen)) {
        return cached_count;
    }
    if device == "out" {
        if dac_seen && fft_seen {
            return 1;
        }
        return 0;
    }

    let new_dac_seen = dac_seen || device == "dac";
    let new_fft_seen = fft_seen || device == "fft";
    let mut count = 0;
    if let Some(next_devices) = devices.get(device) {
        for next_device in next_devices {
            count += count_path(next_device, new_dac_seen, new_fft_seen, devices, cache);
        }
    }

    cache.insert((device, new_dac_seen, new_fft_seen), count);

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two_example_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
