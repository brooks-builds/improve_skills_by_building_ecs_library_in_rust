#[test]
pub fn filter_map() {
    let mut numbers = vec![];
    for number in 0..100 {
        numbers.push(number);
    }
    let wrapped_numbers = numbers
        .iter()
        // .map(|number| WrappedNumber {
        //     value: *number,
        //     is_even: number % 2 == 0,
        // })
        // .filter(|wrapped_number| wrapped_number.is_even)
        .filter_map(|number| {
            let is_even = number % 2 == 0;
            if is_even {
                Some(WrappedNumber {
                    value: *number,
                    is_even,
                })
            } else {
                None
            }
        })
        .collect::<Vec<WrappedNumber>>();

    dbg!(wrapped_numbers);
}

#[derive(Debug)]
struct WrappedNumber {
    value: i32,
    is_even: bool,
}
