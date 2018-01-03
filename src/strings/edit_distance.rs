pub fn levenshtein_distance(a: &String, b: &String) -> usize {
    let str_a: Vec<char> = a.chars().collect();
    let str_b: Vec<char> = b.chars().collect();
    let mut result: Vec<usize> = (0..str_a.len() + 1).collect();

    for row in 1..str_b.len()+1 {
        let last_row = result.clone();
        result[0] = row;

        for col in 1..str_a.len()+1 {
            let cost = if str_a[col-1] == str_b[row-1] {
                0
            } else {
                1
            };

            result[col] = *[
                last_row[col] + 1,
                result[col-1] + 1,
                last_row[col-1] + cost
            ].iter().min().unwrap();
        }
    }

    result[str_a.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levenshtein_distance_simple() {
        let cases = [
            (1, "foo", "boo"),
            (3, "foo", "bar"),
            (4, "foo", "booboo"),
            (3, "foo", "boofoo"),
            (3, "tfoo", "boofoo"),
            (4, "tfoon", "boofoo"),
            (3, "boofoo", "foo"),
        ];
        for &(v, a, b) in cases.iter() {
            assert_eq!(
                v,
                levenshtein_distance(&a.to_string(), &b.to_string()),
                "strings: {}, {}", a, b
            );
        }
    }

    #[test]
    fn test_levenshtein_distance_advanced() {
        let cases = [
            (7, "appropriate meaning", "approximate matching")
        ];
        for &(v, a, b) in cases.iter() {
            assert_eq!(
                v,
                levenshtein_distance(&a.to_string(), &b.to_string()),
                "strings: {}, {}", a, b
            );
        }
    }
}