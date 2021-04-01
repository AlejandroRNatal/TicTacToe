use iui::prelude::*;
use iui::controls::{Label, Button, VerticalBox,LayoutGrid,GridExpand,GridAlignment};
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

#[derive(Clone)]
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
        if index < self.board.len() as i8 &&
           index >= 0 &&
           self.board[index as usize] == ""
        {
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
        self.turn += 1;
    }

    fn current_turn(&self)-> i8
    {
        self.turn as i8
    }

    fn current_player(&self)->usize
    {
        (self.turn % 2) + 1
    }

    fn current_player_token(&self)->String
    {
        self.players[self.current_turn() as usize % 2].clone()
    }

    fn current_board_state(&self)->[String;9]
    {
        self.board.clone()
    }

    fn check_verticals_win_condition(&self)-> bool
    {
        for i in 0..3
        {
            if self.board[i] == self.board[i + 3] &&
               self.board[i] == self.board[i + 6] &&
               self.board[i] != "".to_string()
            { 
                            return true
            }

        }

        return  false
    }

    fn check_diagonals_win_condition(&self) -> bool {
        // check diagonals
        if self.board[0] == self.board[4] &&
           self.board[0] == self.board[8] &&
           self.board[0] != "".to_string()
        {
                return true
            
        }

        if self.board[2] == self.board[4] &&
           self.board[2] == self.board[6] &&
           self.board[2] != "".to_string()
        {
                   return true  
        }

        return false
    }

    fn check_horizontals_win_condition(&self) -> bool
    {
        let mut i: usize = 0;

        while i < self.board.len()
        {
            if self.board[i] == self.board[i+1] &&
               self.board[i] == self.board[i+2] &&
               self.board[i] != "".to_string()
            {
                        return true
            }

            i = i + 3;
        }

        return false
    }

    fn is_winning(&self)->bool
    {
        return self.check_diagonals_win_condition() & 
               self.check_horizontals_win_condition() &
               self.check_verticals_win_condition()
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

    fn max_turns(&self) -> i8
    {
        self.board.len() as i8
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
        if grid.buttons[i].text(ui) == grid.buttons[i+3].text(ui) &&
           grid.buttons[i].text(ui) == grid.buttons[i+6].text(ui) &&
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

    let idx_0 = format!("{}", 0).as_str();
    let idx_1 = format!("{}", 1).as_str();
    let idx_2 = format!("{}", 2).as_str();
    let idx_3 = format!("{}", 3).as_str();

    let idx_4 = format!("{}", 4).as_str();
    let idx_5 = format!("{}", 5).as_str();
    let idx_6 = format!("{}", 6).as_str();
    let idx_7 = format!("{}", 7).as_str();

    let idx_8 = format!("{}", 8).as_str();

    ButtonGrid{
        buttons:[
            Button::new(&ui, idx_0),
            Button::new(&ui, idx_1),
            Button::new(&ui, idx_2),
            Button::new(&ui, idx_3),
            Button::new(&ui, idx_4),
            Button::new(&ui, idx_5),
            Button::new(&ui, idx_6),
            Button::new(&ui, idx_7),
            Button::new(&ui, idx_8),
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
    loop
    {
        println!("Turn #{}:", &game.turn);
        game.output_board();
        println!("");

        if game.current_turn() >= game.max_turns() && !game.is_winning()
        {
            println!("No Winner Possible!\n\n");
            println!("...Game Over...");
            break;
        }

        //Should use while let, just dont know how to write it
        let mut input: String =  String::new();
        loop 
        {
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

        println!("Player {} Input position was: {}\n", game.current_player(), &input);

        if game.is_winning()
        {
            println!("Player {} wins!",  game.current_player());
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
    let mut layout_grid = LayoutGrid::new(&ui);
    let mut vbox = VerticalBox::new(&ui);

    layout_grid.set_padded(&ui, true);
    vbox.set_padded(&ui, true);

    // Create a new label.
    let label = Label::new(&ui, "");

    // Create a button and its callback
    let mut i:i8 = 0;

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
    // vbox.append(&ui, label.clone(), LayoutStrategy::Stretchy);
    layout_grid.append(&ui, label.clone(),0 ,10, 5,5,GridExpand::Both,GridAlignment::Start,GridAlignment::Center);
    
    let mut i:i8 = 0;

    while i < button_grid.buttons.len() as i8
    {
        vbox.append(&ui,  button_grid.buttons[i as usize].clone(), LayoutStrategy::Stretchy);
        i+=1;
    }

    let mut i:i32 = 0;
    
    for button in button_grid.buttons.iter()
    {
        // vbox.append(&ui, button.clone(), LayoutStrategy::Compact);
        layout_grid.append(&ui, button.clone(),
                          1 + i ,10 + 10 * (i + 1),
                          5,5,
                          GridExpand::Both,
                          GridAlignment::Center,
                          GridAlignment::Center);
        i+=1;
    }
    

    // win.set_child(&ui, vbox);
    win.set_child(&ui, layout_grid);
    // Show the window
    win.show(&ui);

    // Run the application
    let mut event_loop = ui.event_loop();

    event_loop.on_tick(&ui, {

        let ui = ui.clone();
        let mut ui_label = label.clone();

        println!("Event Loop Turn: #{}", game.current_turn());
        
        if game.current_turn()  >= game.max_turns() && check_button_states(&button_grid, &ui)
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
    use super::*;

    #[test]
    fn test_is_winning() {
        // assert_eq!(is_winning([]]), false);
    }

}