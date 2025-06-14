#[test]
fn option_combinators() {
    let value: Option<i32> = Some(5);
    assert_eq!(value.map(|v| v * 2), Some(10));
    assert_eq!(value.and_then(|v| Some(v + 1)), Some(6));
}

#[test]
fn result_propagation() -> Result<(), String> {
    fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
        s.parse::<i32>()
    }
    let value = parse_number("42").map_err(|e| e.to_string())?;
    assert_eq!(value, 42);
    Ok(())
}

#[test]
fn pattern_matching() {
    let value = Some(3);
    let result = match value {
        Some(v) if v > 1 => true,
        _ => false,
    };
    assert!(result);
}

#[test]
fn trait_and_generics() {
    trait Double {
        fn double(&self) -> Self;
    }
    impl Double for i32 {
        fn double(&self) -> Self { *self * 2 }
    }
    fn generic_double<T: Double>(v: T) -> T { v.double() }
    assert_eq!(generic_double(4i32), 8);
}

#[test]
fn iterator_closure() {
    let values = vec![1, 2, 3];
    let doubled: Vec<_> = values.iter().map(|v| v * 2).collect();
    assert_eq!(doubled, vec![2, 4, 6]);
}

#[test]
fn simple_thread() {
    use std::thread;
    use std::sync::{Arc, Mutex};

    let counter = Arc::new(Mutex::new(0));
    let handles: Vec<_> = (0..5).map(|_| {
        let counter = Arc::clone(&counter);
        thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        })
    }).collect();

    for handle in handles { handle.join().unwrap(); }

    assert_eq!(*counter.lock().unwrap(), 5);
}
