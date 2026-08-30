#[cfg(test)]
mod tests {
    use pipe_macro::pipe;

    fn add(a: usize, b: usize) -> usize {
        a + b
    }

    fn double(num: usize) -> usize {
        num * 2
    }

    #[test]
    fn it_works() {
        let num = 4;
        let result = pipe! {
            num
                |> add(2, _)
                |> double(_)
        };
        assert_eq!(result, 12);
    }

    #[test]
    fn multiple_substitutions_work() {
        let result = pipe! { 4 |> add(_, _) };
        assert_eq!(result, 8);
    }
}
