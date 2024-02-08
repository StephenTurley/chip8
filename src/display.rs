pub fn render(frame_buffer: &[[bool; 64]; 32]) {
    print!("\x1B[2J"); // clear terminal
    for row in *frame_buffer {
        println!();
        for px in row {
            let symbol = if px { "⚫" } else { "⚪" };

            print!("{}", symbol);
        }
    }
    println!();
}
