use std::collections::HashSet;

/// repeated fraction representation of sqrt(d)
pub fn continued_fraction(d: i64) -> (i64, Vec<i64>) {
    let mut repeat = Vec::new();
    let sqrt = (d as f64).sqrt();
    if sqrt.floor() == sqrt {
        return (sqrt as i64, repeat);
    }

    let base = sqrt as i64;
    let mut top = 1;
    let mut bot_number = -base;
    let mut seen_states = HashSet::new();

    loop {
        let key = (bot_number, top);

        if seen_states.contains(&key) {
            break;
        }

        seen_states.insert(key);

        top = (d - bot_number*bot_number) / top;

        let mut to_subtract = 0;

        while bot_number + to_subtract + top <= base {
            to_subtract += top;
        }

        bot_number = -bot_number - to_subtract;

        repeat.push(to_subtract/top);
    }

    (base, repeat)
}
