
use iui::prelude::*;
use iui::controls::{Label, Button, VerticalBox};
use std::rc::Rc;
use std::cell::RefCell;

use std::io;
use io::stdin;

// #[derive(Debug)]
// enum GameError{

//     InvalidBoardPositionError
// }

struct State {
    label:String,
}

#[derive( Clone)]
struct ButtonGrid{
    buttons:[Button;9],
}

#[derive(Debug, Clone)]
struct Game{
    board:[String;9],
    players: [String;2],
    turn:usize,
    win:bool,
}

impl Game{

    fn set_player_board_idx(&mut self, index: i8) -> bool
    {
        //we need to check if valid index
        if index < self.board.len() as i8 && index >= 0 && self.board[index as usize] == ""{
            let player:usize = self.turn %2;
            self.board[index as usize] = self.players[player].clone() ;

            true
        }else{
            println!("Invalid index [{}]!", &index);
            false
        }
        
    }


    fn update_turn(&mut self)
    {
        self.turn = self.turn + 1;
    }

    fn current_turn(&self)->usize
    {
        self.turn
    }

    fn current_player(&self)->usize
    {
        (self.turn % 2) + 1
    }

    fn current_player_token(&self)->String
    {
        self.players[self.current_turn() % 2].clone()
    }

    fn current_board_state(&self)->[String;9]
    {
        self.board.clone()
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

fn check_button_states(grid:&ButtonGrid, ui:&UI)->bool
{
    // check horizontals
    let mut i: usize = 0;

    while i < grid.buttons.len()
    {
        if grid.buttons[i].text(ui) == grid.buttons[i+1].text(ui) && 
           grid.buttons[i].text(ui) == grid.buttons[i+2].text(ui) &&
           grid.buttons[i].text(ui) != "".to_string()
        {
                    return true
        }
        i = i + 3;
    }


    // check vertical
    for i in 0..3
    {
        if grid.buttons[i].text(ui)== grid.buttons[i+3].text(ui) &&
           grid.buttons[i].text(ui)==grid.buttons[i+6].text(ui) &&
           grid.buttons[i].text(ui) != "".to_string()
        {  
                        return true
        }    
    }

    // check diagonals
    if grid.buttons[0].text(ui) == grid.buttons[4].text(ui) &&
       grid.buttons[0].text(ui) == grid.buttons[8].text(ui) &&
       grid.buttons[0].text(ui) != "".to_string()
    {
            return true
    }

    if grid.buttons[2].text(ui) == grid.buttons[4].text(ui) &&
       grid.buttons[2].text(ui) == grid.buttons[6].text(ui) &&
       grid.buttons[2].text(ui) != "".to_string()
    {
                return true
    }


        return false
}

fn build_button_grid(ui:&UI)->ButtonGrid
{

    let idx_0 = format!("{}", 0);
    let idx_1 = format!("{}", 1);
    let idx_2 = format!("{}", 2);
    let idx_3 = format!("{}", 3);

    let idx_4 = format!("{}", 4);
    let idx_5 = format!("{}", 5);
    let idx_6 = format!("{}", 6);
    let idx_7 = format!("{}", 7);

    let idx_8 = format!("{}", 8);

    ButtonGrid{
        buttons:[
            Button::new(&ui, idx_0.as_str()),
            Button::new(&ui, idx_1.as_str()),
            Button::new(&ui, idx_2.as_str()),
            Button::new(&ui, idx_3.as_str()),
            Button::new(&ui, idx_4.as_str()),
            Button::new(&ui, idx_5.as_str()),
            Button::new(&ui, idx_6.as_str()),
            Button::new(&ui, idx_7.as_str()),
            Button::new(&ui, idx_8.as_str()),
        ]
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


fn cli_main() -> io::Result<()>
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


fn main()
{

    //init game board
    let players:[String; 2] = [String::from("X"),String::from("O")];
    let mut game = Box::new(build_game(players));

    // Initialize the UI and State
    let ui = UI::init().expect("Couldn't initialize UI library");
    let state = Rc::new(RefCell::new(State { label: "".into() }));

    //Buttons
    let mut button_grid = build_button_grid(&ui);

    // Create a window into which controls can be placed
    let mut win = Window::new(&ui, "Tic Tac Toe", 200, 100, WindowType::NoMenubar);

    // Create a vertical layout to hold the controls
    let mut vbox = VerticalBox::new(&ui);
    vbox.set_padded(&ui, true);

    // Create a new label.
    let label = Label::new(&ui, "");

    // Create a button and its callback
    let mut i:i8 = 0;

    // let indices = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    while i < button_grid.buttons.len() as i8
    {
        button_grid.buttons[i as usize].on_clicked(&ui, {
            | button: &mut Button  | {
                if button.text(&ui) != "X" && button.text(&ui) != "O"
                {
                    button.set_text( &ui, String::from(game.current_player_token()).as_str());
                    game.update_turn();
                }
            }
        });

        i+=1;
    }
    

    // Adding controls to the box, and box to window
    vbox.append(&ui, label.clone(), LayoutStrategy::Stretchy);

    for button in button_grid.buttons.iter()
    {
        vbox.append(&ui, button.clone(), LayoutStrategy::Compact);
    }
    

    win.set_child(&ui, vbox);

    // Show the window
    win.show(&ui);

    // Run the application
    let mut event_loop = ui.event_loop();

    event_loop.on_tick(&ui, {

        let ui = ui.clone();
        let mut ui_label = label.clone();

        println!("Event Loop Turn: #{}", game.current_turn());
        
        if game.turn >= game.board.len() && check_button_states(&button_grid, &ui)
        {
            println!("No Winner Possible!\n\n");
            println!("...Game Over...");
        }
    
    
        // End of turn
        move || {
            // Update all the labels
            ui_label.set_text(&ui, &format!("Turn #{}", game.current_turn()));

            if check_button_states(&button_grid, &ui)
            {
                ui_label.set_text(&ui, format!("Player {} Won!",
                                  game.current_player()).as_str());
                // ui.quit();
            }
        }
    
    });

    event_loop.run(&ui);

}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    // use super::*;

    // #[test]
    // fn test_is_winning() {
    //     assert_eq!(is_winning([]]), false);
    // }

}