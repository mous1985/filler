use std::io;

fn main() {
    // Read the player number from the game engine.
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let player_number = input.chars().nth(10).unwrap();
    println!("{}",player_number);
    let mut chars = Vec::new();
    if player_number=='1' 
    {
        chars=vec!['@','a']
    } else {
        chars=vec!['$','s']
    }
    //println!("{}", player_number);
    loop {
    
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        //let rows = input[8..10].parse::<usize>().unwrap();
        let lines = input[11..13].parse::<i32>().unwrap();

        // Define the Anfield grid as a 2D vector of characters.
        let mut grid = Vec::new();

        for i in 0..lines+1 {//mettre i en usize 
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read input");
            if i<1 {
                continue;
            } else {
                let row: Vec<char> = input[4..input.len()-1].chars().collect(); 
                grid.push(row);
            }
        }

        // reading the piece
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let piece_lines = input[8..9].parse::<i32>().unwrap();
        let mut piece = Vec::new();
        for _ in 0..piece_lines {
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read input");
            let row: Vec<char> = input[..input.len()-1].chars().collect();
            piece.push(row);
        }

        
        // Implement your game logic here to decide where to place the piece.
        let (piece_x, piece_y) = place_piece(&grid, &piece, &chars);

        // Print the coordinates of the piece placement to standard output.
        println!("{} {}", piece_x, piece_y);
    }
}