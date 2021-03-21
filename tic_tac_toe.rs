use std::io;
use io::stdin;

fn output_board(board:&[String;9])
{
    for i in 0..board.len() {

        if board[i].trim() == ""
        {
            print!("[{}]", i);
        }
        else{
            print!("[{}]", board[i]);
        }
        
        
        if i == 2 || i == 5 || i == 8 {
            println!("");
        }
    }

    println!("");
}

fn is_winning_board(board:&[String;9])->bool{

    let mut win:bool = false;

    // check horizontals
    let mut i: usize = 0;

    while i < board.len()
    {
        if board[i]==board[i+1] && board[i]==board[i+2]
        {
                if board[i] != "".to_string()
                {
                    win = true;
                    break;
                }  
        }
        i = i + 3;
    }

    
    // check vertical
    for i in 0..3{
        if board[i]==board[i + 3] && board[i]==board[(i+6)]{
            if board[i] != "".to_string()
            {   
                        win = true;
                        break;
            }
        }
           
    }

    // check diagonals
    if board[0]==board[4] && board[0]==board[8]
    {
        if board[0] != "".to_string()
        {
            win = true;
        }
    }
    
    if board[2]==board[4] && board[2]==board[6]
    {
        if board[2] != "".to_string()
            {
                win = true;
            }
    }
       

    return win
}

fn main() -> io::Result<()>
{
    let players:[&str; 2] = ["X", "O"];

    let mut board: [String; 9] = [
                                    "".to_string(),"".to_string(),"".to_string(),
                                    "".to_string(),"".to_string(),"".to_string(),
                                    "".to_string(),"".to_string(),"".to_string()
                                ];
    let mut turn : usize = 0;

    let mut is_winner: bool = false;
    
    // Game Loop
    loop{

        println!("Turn #{}:", &turn);
        output_board(&board);
        println!("");

        if turn >= board.len() && !is_winner
        {
            println!("No Winner Possible!\n\n");
            println!("...Game Over...");
            break;
        }
        
        

        //here we catch input from user
        let mut input: String =  String::new();
        stdin().read_line(&mut input).unwrap();

        // now we parse the string
        let parsed_index: usize = input.trim().parse::<usize>().unwrap();
        let player_idx: usize = turn % 2 ;
        
        board[parsed_index] = players[player_idx].to_string();

        println!("Player {} Input position was: {}\n", (turn % 2 ) + 1, &input);

        is_winner = is_winning_board(&board);

        if is_winner
        {
            println!("Player {} wins!",  (turn % 2 ) + 1);
            break;
        }

        

        // End of turn
        turn = turn + 1;
        
    }

    Ok(())

}