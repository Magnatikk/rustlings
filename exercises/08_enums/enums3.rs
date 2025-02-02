// enums3.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint enums3` or use the `hint` watch subcommand for a
// hint.

///Le but de cet exercice est de compléter l'enum Message à la manière des exercices précédent
/// Ainsi que de compléter la fonction permettant de process ces messages
/// Pour les enums, il nous suffit de lire les paramètres attendus en entrée des différentes fonctions
/// La fonction process attend que l'on utilise les 4 autres fonctions plus hautes
/// On a juste a créer un match message {} afin de traiter chaque message.
/// Egalement un test a corriger, car passer un tuple en paramètre requiere une paire de parenthèses en plus :
/// state.process(Message::ChangeColor(255, 0, 255)); devient state.process(Message::ChangeColor((255, 0, 255)));
enum Message {
    ChangeColor((u8,u8,u8)),
    Echo(String),
    Move(Point),
    Quit
}

struct Point {
    x: u8,
    y: u8,
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
    message: String,
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&mut self, s: String) {
        self.message = s
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        match message {
            Message::Quit=>self.quit(),
            Message::ChangeColor((x,y,z))=>self.change_color((x,y,z)),
            Message::Move(x)=>self.move_position(x),
            Message::Echo(x)=>self.echo(x)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            quit: false,
            position: Point { x: 0, y: 0 },
            color: (0, 0, 0),
            message: "hello world".to_string(),
        };
        state.process(Message::ChangeColor((255, 0, 255)));
        state.process(Message::Echo(String::from("Hello world!")));
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
        assert_eq!(state.message, "Hello world!");
    }
}
