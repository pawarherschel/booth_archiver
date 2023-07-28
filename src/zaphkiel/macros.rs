#[macro_export]
/// Time the execution of a statement and print the result to stdout.
/// The statement can be an expression that returns a value.
///
/// # Example
/// ```
/// use booth_archiver::time_it;
/// let result = time_it!("sleep 1 second" => std::thread::sleep(std::time::Duration::from_secs(1)));
/// ```
macro_rules! time_it {
    ($comment:literal => $stmt:stmt) => {{
        print!("{}", $comment);
        let start = std::time::Instant::now();
        let result = { $stmt };
        let duration = start.elapsed();
        println!(" => {:?}", duration);
        result
    }};
}
