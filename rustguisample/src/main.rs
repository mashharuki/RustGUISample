/**
 * アプリケーションの基盤となるメインファイル
 */

use iced::{ 
    button, executor, Align, Application, Button, Column, Command, Element, Font, Text
    HorizontalAlignment, Length, Row, Settings, Subscription,
};
use iced::Settings;
use iced_native::Widget;

// フォント用の変数を定義
const FONT: Font = Font::External {
    name: "PixelMplus12-Regular",
    bytes: include_bytes!("../rsc/PixelMplus12-Regular.ttf"),
};

// メッセージを定義する
#[derive(Debug, Clone)]
pub enum Message {
    Start,
    Stop,
    Reset,
} 

// TickStateを定義する
pub enum TickState {
    Stopped,
    Ticking,
}

// 構造体GUIを定義
struct GUI {
    start_stop_button_state: button::State,
    reset_button_state: button::State,
    tick_state: TickState,
}

// GUIにApplicationトレイトを実装する。
impl Application for GUI {  
    // ぞれぞれ変数を用意する。
    type Executor = executor::Null;
    type Message = Message;
    type Flags = ();

    /**
     * 初期化用のメソッド
     */
    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            GUI {
                start_stop_button_state: button::State::new(),
                reset_button_state: button::State::new(),
                tick_state: TickState::Stopped,
            }, 
            Command::none()
        );
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
        // メッセージを受信した時の処理
        match message {
            Message::Start => {
                self.tick_state = TickState::Ticking;
            }
            Message::Stop => {
                self.tick_state = TickState::Stopped;
            }
            Message::Reset => {}
        }
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
                        .font(Font),
        )
        .min_width(80);
        // リセットボタンの定義
        let reset_button = Button::new(
            &mut self.reset_button_state,
            Text::new("Reset")
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .font(Font),
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
            .into();

        // 経過時刻を表示するための変数を用意する。
        let duration_text = "00:00:00.00";
        // 開始＆停止時のテキストを用意する。
        let start_stop_text = match self.tick_state {
            // 停止時の定義
            TickState::Stopped => Text::new("Start")
                                        .horizontal_alignment(HorizontalAlignment::Center)
                                        .font(Font),
            // 経過中の定義
            TickState::Ticking => Text::new("Stop")
                                        .horizontal_alignment(HorizontalAlignment::Center)
                                        .font(Font),
        };

        // ボタンに表示するためのテキストを用意する。
        let start_stop_message = match self.tick_state {
            TickState::Stopped => Message::Start,
            TickState::Ticking => Message::Stop,
        };

        // ウィジットを初期化する。
        let tick_text = Text::new(duration_text).font(Font).size(60);
        // 開始ボタン用のウィジットを用意する。
        let start_stop_button = Button::new(&mut self.start_stop_button_state, start_stop_text)
                                                            .min_width(80)
                                                            .on_press(start_stop_message);
        // リセットボタン用のウィジットを用意する。
        let reset_button = Button::new(&mut self.reset_button_state, 
                                                        Text::new("Reset")
                                                                    .horizontal_alignment(HorizontalAlignment::Center)
                                                                    .font(Font)
                                                            )
                                                            .min_width(80)
                                                            .on_press(Message::Reset);
        
    }
}

/**
 * メイン関数
 */
fn main() {
    // 設定用の変数を定義する。
    let mut settings = Settings::default();
    // ウィンドウの大きさを調整する。
    settings.window.size = (400u32, 120u32);
    // run関数を呼び出す。
    GUI::run(Settings::default());
}