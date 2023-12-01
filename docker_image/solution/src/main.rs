
use std::io;
// use std::time::Instant;

fn main() {
    // Read the player number from the game engine.
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let player_number = input.chars().nth(10).unwrap();
    let mut pchars = Vec::new(); // player chars
    let mut echars = Vec::new(); // ennemy chars
    let mut pcoords = Vec::new();
    let mut ecoords = Vec::new();
    if player_number=='1' 
    {
        pchars=vec!['@','a'];
        echars=vec!['$','s']
    } else {
        pchars=vec!['$','s'];
        echars=vec!['@','a']
    }
    // let start = Instant::now();
    loop {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let grid_details = input.split_whitespace().collect::<Vec<&str>>()[2];
        let lines = &grid_details[..grid_details.len()-1];
        let grid_lines = lines.parse::<usize>().unwrap();

        // Define the Anfield grid as a 2D vector of characters.
        let mut grid = Vec::new();

        // reading the grid
        for i in 0..grid_lines+1 {
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read input");
            if i<1 {
                continue;
            } else {
                let row: Vec<char> = input[4..input.len()-1].chars().collect();
                for j in 0..row.len(){
                    if pchars.contains(&row[j]) {
                        pcoords.push((j,i))
                    }
                    if echars.contains(&row[j]) {
                        ecoords.push((j,i))
                    }
                }
                grid.push(row);
            }
        }
        
        // Defining the piece
        let mut piece = Vec::new();

        // reading the piece
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let piece_details = input.split_whitespace().collect::<Vec<&str>>();
        let lines = piece_details[2];
        let piece_lines = lines[..lines.len()-1].parse::<i32>().unwrap();

        for _ in 0..piece_lines {
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read input");
            let row: Vec<char> = input[..input.len()-1].chars().collect();
            piece.push(row);
        }

        // finds the piece ideal position
        let (piece_x, piece_y) = place_piece(&grid, &piece, &pcoords, &ecoords, &pchars);

        // Print the coordinates of the piece placement to standard output.
        println!("{} {}", piece_x, piece_y);
        // let duration = start.elapsed();
        // println!("Time elapsed in expensive_function() is: {:?}", duration);
   }
}

fn place_piece(grid: &Vec<Vec<char>>, piece: &Vec<Vec<char>>, pcoords: &Vec<(usize,usize)>, ecoords: &Vec<(usize,usize)>, pchars: &Vec<char>) -> (usize, usize) {
    let grows = grid[0].len(); // number of grid rows
    let prows = piece[0].len(); // number of piece rows
    let  mut distance = ((grows as f32).powf(2.) + (grid.len() as f32).powf(2.)).sqrt();
    //let mut solutions = Vec::new(); // vector that will contain all solutions
    let mut sol = (0,0);
    // pour optimiser le test de placement de pièces on trouve le carré dans lequel se trouvent nos points
    let (mut xmin,mut xmax,mut ymin,mut ymax) = (grid.len(),0,grows,0);//les coordonnées du carré
    for (xg,yg) in pcoords {
        if xg < &xmin {xmin=*xg}
        if xg > &xmax {xmax=*xg}
        if yg < &ymin {ymin=*yg}
        if yg > &ymax {ymax=*yg}
    }

    let (mut xi,mut xf,mut yi,mut yf) = (0,grows-prows+1,0,grid.len()-piece.len()+1);
    let temp = xmin as i32 - prows as i32- 1;
    if (temp) > 0 {xi = xmin - prows + 1}
    if (xmax + prows - 1) < grows {xf = xmax + 1}
    let temp = ymin as i32 - piece.len() as i32 + 1;
    if (temp) > 0 {yi = ymin - piece.len() }
    if (ymax + piece.len() - 1) < grid.len() {yf = ymax }

    // l'idee est d'essayer de placer la piece partout dans la grille où se trouve le joueur
    // pour trouver toutes les solutions possibles
    for yg in yi..yf {
        for xg in xi..xf {
            if can_place_piece(grid, piece, pchars, xg, yg) {
                let min_dist = calcul_min_dist(piece, ecoords, (xg,yg), distance);
                if min_dist < distance {
                    distance = min_dist;
                    sol = (xg,yg);
                }
            }
        }
    }

    return sol
}

fn calcul_min_dist(piece: &Vec<Vec<char>>, ecoords: &Vec<(usize,usize)>,(xg,yg):(usize,usize), distance:f32) -> f32 {
    let prows = piece[0].len(); // number of piece rows
    let mut min_dist=distance;

    for yp in 0..piece.len(){
        for xp in 0..prows {
            if piece[yp][xp] != '.' {
                for (xe,ye) in ecoords {
                    let  dist=(((*ye as f32)-((yp + yg) as f32) ).powf(2.) + ((*xe as f32)-((xp + xg) as f32)).powf(2.)).sqrt();
                    if dist < min_dist{
                        min_dist = dist;
                    }
                }
            }
        }
    }
     
    return min_dist
}

fn can_place_piece(grid: &Vec<Vec<char>>, piece: &Vec<Vec<char>>, pchars: &Vec<char>, xg: usize, yg: usize) -> bool {
    let mut cross = 0; // nbre de croisements de la pièces avec mes points
    let mut stop = false;
    let prows = piece[0].len(); // number of piece rows

    for yp in 0..piece.len(){
        for xp in 0..prows {
            if piece[yp][xp] != '.' {
                if pchars.contains(&grid[yg+yp][xg+xp]) {
                    cross +=1;
                    if cross>1 {
                        stop = true;
                        break;
                    }
                } else if grid[yg+yp][xg+xp] != '.' {
                    stop = true;
                    break;
                }
            }
        }
        if stop {
            break
        }
    }
    if cross==1 && !stop {
        return  true;
    }
    false
}