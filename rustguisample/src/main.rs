/**
 * アプリケーションの基盤となるメインファイル
 */

use iced::{ 
    button, executor, Align, Application, Button, Column, Command, Element, Font, Text
    HorizontalAlignment, Length, Row, Settings, Subscription,
};
use iced::Settings;

struct GUI {
    start_stop_button_state: button::State,
    reset_button_state: button::State,
}

// GUIにApplicationトレイトを実装する。
impl Application for GUI {  
    // ぞれぞれ変数を用意する。
    type Executor = executor::Null;
    type Message = ();
    type Flags = ();

    /**
     * 初期化用のメソッド
     */
    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (GUI, Command::none());
    }

    /**
     * ウィンドウのタイトルを設定するメソッド
     */
    fn title(&self) -> String {
        String::from("DEMO");
    }

    /**
     * アプリケーションの状態を表示するメソッド
     */
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        Command::none();
    }

    /**
     * ウィンドウに表示するウィジットを設定するメソッド
     */
    fn view(&mut self) -> Element<'_, Self::Message> {
        Text::new("Hello, World!").into();
    }
}

/**
 * メイン関数
 */
fn main() {
    // run関数を呼び出す。
    GUi::run(Settings::default());
}