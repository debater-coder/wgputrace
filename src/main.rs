use wgputrace::run;

fn main() {
    pollster::block_on(run());
}
