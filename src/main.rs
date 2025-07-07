use crate::intensity_segments::IntensitySegments;

mod intensity_segments;

fn main() {
    let mut segments = IntensitySegments::new();
    assert_eq!(segments.to_string(), "[]");

    segments.add(10, 30, 1);
    assert_eq!(segments.to_string(), "[[10,1],[30,0]]");

    segments.add(20, 40, 1);
    assert_eq!(segments.to_string(), "[[10,1],[20,2],[30,1],[40,0]]");

    segments.add(10, 40, -2);
    assert_eq!(segments.to_string(), "[[10,-1],[20,0],[30,-1],[40,0]]");

    segments.set(15, 35, 5);
    assert_eq!(segments.to_string(), "[[10,-1],[15,5],[35,-1],[40,0]]");

    segments.set(10, 40, 0);
    assert_eq!(segments.to_string(), "[]");

    segments.add(5, 15, 3);
    segments.add(10, 20, 2);
    segments.set(8, 12, 0);
    assert_eq!(segments.to_string(), "[[5,3],[8,0],[12,5],[15,2],[20,0]]");

    segments.set(10, 10, 999);
    assert_eq!(segments.to_string(), "[[5,3],[8,0],[12,5],[15,2],[20,0]]"); // 不应变化
}
