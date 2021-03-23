use std::io;
use io::stdin;

// #[derive(Debug)]
// enum GameError{

//     InvalidBoardPositionError
// }

struct Game{
    board:[String;9],
    players: [String;2],
    turn:usize,
    win:bool,
}

impl Game{

    fn set_player_board_idx(&mut self, index:i8) -> bool
    {
        //we need to check if valid index
        if index < self.board.len() as i8 && index >= 0{
            let player:usize = self.turn %2;
            self.board[index as usize] = self.players[player].clone() ;

            true
        }else{
            println!("Invalid index [{}]!", &index);
            false
        }
        
    }

    fn is_winning(&mut self)->bool
    {
        // check horizontals
        let mut i: usize = 0;

        while i < self.board.len()
        {
            if self.board[i]==self.board[i+1] && self.board[i]==self.board[i+2]
            {
                    if self.board[i] != "".to_string()
                    {
                        self.win = true;
                        break;
                    }  
            }
            i = i + 3;
        }

        
        // check vertical
        for i in 0..3{
            if self.board[i]==self.board[i + 3] && self.board[i]==self.board[(i+6)]{
                if self.board[i] != "".to_string()
                {   
                            self.win = true;
                            break;
                }
            }
            
        }

        // check diagonals
        if self.board[0]==self.board[4] && self.board[0]==self.board[8]
        {
            if self.board[0] != "".to_string()
            {
                self.win = true;
            }
        }
        
        if self.board[2]== self.board[4] && self.board[2]==self.board[6]
        {
            if self.board[2] != "".to_string()
                {
                    self.win = true;
                }
        }
        

            return self.win
    }

    fn update_turn(&mut self)
    {
        self.turn = self.turn + 1;
    }


    fn output_board(&self)
    {
        for i in 0..self.board.len() {

            if self.board[i].trim() == ""
            {
                print!("[{}]", i);
            }
            else{
                print!("[{}]", self.board[i]);
            }
            
            
            if i == 2 || i == 5 || i == 8 {
                println!("");
            }
        }

        println!("");
    }
}

fn build_game(players:[String; 2])->Game{

    Game{
        board: [
            "".to_string(),"".to_string(),"".to_string(),
            "".to_string(),"".to_string(),"".to_string(),
            "".to_string(),"".to_string(),"".to_string()
        ],
        players,
        turn: 0,
        win: false,
    }

}

fn main() -> io::Result<()>
{
    let players:[String; 2] = [String::from("X"), String::from("O")];

    let mut game:Game = build_game(players);
    
    // Game Loop
    loop{

        println!("Turn #{}:", &game.turn);
        game.output_board();
        println!("");

        if game.turn >= game.board.len() && !game.win
        {
            println!("No Winner Possible!\n\n");
            println!("...Game Over...");
            break;
        }

        //Should use while let, just dont know how to write it
        let mut input: String =  String::new();
        loop {
            //here we catch input from user
            let parsed_index = match stdin().read_line(&mut input)
            {
                Ok(_ ) => match input.trim().parse::<i8>(){
                    Ok(n) => n,
                    Err(_) => 11//invalid board indexes
                },
                Err(_) => 11//invalid board indexes
            };
            
            if game.set_player_board_idx(parsed_index)
            {
                break;
            }
        }

        println!("Player {} Input position was: {}\n", (game.turn % 2 ) + 1, &input);

        if game.is_winning()
        {
            println!("Player {} wins!",  (game.turn % 2 ) + 1);
            break;
        }

        // End of turn
        game.update_turn();
        
    }

    Ok(())

}


// #[cfg(test)]
// mod tests {
//     // Note this useful idiom: importing names from outer (for mod tests) scope.
//     use super::*;

//     // #[test]
//     // fn test_is_winning() {
//     //     assert_eq!(is_winning([]]), false);
//     // }

// }