// raii.rs
fn create_box() {
    // 整数をヒープ上に確保
    let _box1 = Box::new(3i32);

    // `_box1`はここで破棄され、メモリは解放される。
}

fn main() {
    // 整数をヒープ上に確保
    let _box2 = Box::new(5i32);

    // ネストしたスコープ
    {
        // 整数をヒープ上に確保
        let _box3 = Box::new(4i32);

        // `_box3`はここで破棄され、メモリは解放される。
    }

    // お遊びで大量のボックスを作る。
    // もちろん手動で開放する必要はないよ！
    for _ in 0u32..1_000 {
        create_box();
    }

    // `_box2`はここで破棄され、メモリは解放される。
}