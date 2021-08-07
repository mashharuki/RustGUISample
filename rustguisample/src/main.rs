/**
 * アプリケーションの基盤となるメインファイル
 */

use iced::{ 
    button, executor, Align, Application, Button, Column, Command, Element, Font, Text
    HorizontalAlignment, Length, Row, Settings, Subscription,
};
use iced::Settings;
use iced_native::Widget;

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
        (GUI {
            start_stop_button_state: button::State::new(),
            reset_button_state: button::State::new(),
        }, Command::none());
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
        // ウィジットを初期化する。
        let tick_text = Text::new("00:00:00.00").font(FONT).size(60);
        // 開始ボタンの定義
        let start_stop_button = Button::new(
            &mut self.start_stop_button_state,
            Text::new("Start")
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .font(FONT),
        )
        .min_width(80);
        // リセットボタンの定義
        let reset_button = Button::new(
            &mut self.reset_button_state,
            Text::new("Reset")
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .font(FONT),
        )
        .min_width(80);
        // カラムを用意する
        Column::new()
            .push(tick_text)
            .push(
                Row::new()
                        .push(start_stop_button)
                        .push(reset_button)
                        .spacing(10),
            )
            .spacing(10)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Align::Center)
            .into()
    }
}

/**
 * メイン関数
 */
fn main() {
    // run関数を呼び出す。
    GUi::run(Settings::default());
}