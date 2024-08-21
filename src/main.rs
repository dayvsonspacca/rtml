use std::time::Instant;

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .init();

    let now = Instant::now();

    let mut view = rtml::View::from_file("views/component.html").unwrap();
    view.add_variable("phone", "+55 81 999051134");

    let html = view.build().unwrap();
    println!("{}", html);

    println!("Elapsed time: {:?}", now.elapsed());
}
