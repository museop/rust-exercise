use std::collections::HashMap;

macro_rules! map {
    ( $( $x:expr => $y:expr ),* ) => {
        {
            let mut temp_map = HashMap::new();
            // 2. 코드 확장: 캡처한 $x에 대해 반복 생성
            $(
                temp_map.insert($x, $y);
            )*
            temp_map
        }
    };
}

fn main() {
    let scores = map!(
        "Alice" => 10,
        "Bob" => 20,
        "Charlie" => 30
    );

    println!("{:?}", scores);
}
