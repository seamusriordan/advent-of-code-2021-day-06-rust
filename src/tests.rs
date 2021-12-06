#[cfg(test)]
mod tests {
    use crate::advance_n_days;

    #[test]
    fn example_18_days_gives_26() {
        let  state = vec![3,4,3,1,2];

        assert_eq!(26, advance_n_days(&state, 18));
    }

    #[test]
    fn example_80_days_gives_26() {
        let  state = vec![3,4,3,1,2];

        assert_eq!(5934, advance_n_days(&state, 80));
    }

    #[test]
    fn example_256_days_gives_26984457539() {
        let  state = vec![3,4,3,1,2];

        assert_eq!(26984457539, advance_n_days(&state, 256));
    }
}