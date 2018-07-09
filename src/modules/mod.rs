use Segment;

mod dir;
mod os;
mod status;

pub fn handle(module: &str, segment: &mut Segment, args: &[&str]) {
    match module {
        "os" => os::segment(segment, args),
        "status" => status::segment(segment, args),
        "dir" => dir::segment(segment, args),
        _ => panic!("unknown module, {}", module),
    }
}