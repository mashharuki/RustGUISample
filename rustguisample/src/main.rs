/** 
 * アプリケーションの基盤となるメインファイル
 */

use iced::{ 
    button, executor, Align, Application, Button, Column, Command, Element, Font, Text
    HorizontalAlignment, Length, Row, Settings, Subscription,
};
use iced_native::Widget;
use std::time::{Duration, Instant};

// 各種定数を宣言する。
const FPS: u64 = 30;
const MILLISEC: u64 = 1000;
const MINUTE: u64 = 60; 
const HOUR: u64 = 60 * MINUTE;

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
    Update,
} 

// TickStateを定義する
pub enum TickState {
    Stopped,
    Ticking,
}

// 構造体Timerを用意する。
pub struct Timer {
    duration: Duration,
}

// Timerトレイトを適用する。
impl Timer {
    // 初期化関数
    fn new(duration: Duration) -> Timer {
        Timer { duration: duration }
    }
}

// Timerにicedのトレイトを適用させる。
impl<H, E> iced::subscription::Recipe<H, E> for Timer
    where
        H: std::hash::Hasher,
        {
            type Output = Instant;
            // ハッシュ化するための関数
            fn hash(&self, state: &mut H) {
                // クレートを読み込む
                use std::hash::Hash;
                std::any::TypeId::of::<Self>().hash(state);
                // ハッシュ値を生成する。
                self.duration.hash(state);
            }
            // 一定の間隔で現在の時刻を返す関数
            fn stream(
                self: Box<Self>, 
                _input: futures::stream::BoxStream<'static, E>,
            ) -> futures::stream::BoxStream<'static, Self::Output> {
                // クレートを読み込む
                use futures::stream::StreamExt;
                // 時刻を送信する。(Instantという型で時刻を返すようにする。)
                async_std::stream::interval(self.duration).map(|_| Instant::mow()).boxed();
            }
        }

// 構造体GUIを定義
struct GUI {
    start_stop_button_state: button::State,
    reset_button_state: button::State,
    tick_state: TickState,
    last_update: Instant,
    total_duration: Duration,
}

// GUIにApplicationトレイトを実装する。
impl Application for GUI {  
    // ぞれぞれ変数を用意する。
    type Executor = executor::Default;
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
                last_update: Instant::now(),
                total_duration: Duration::default(),
            }, 
            Command::none()
        );
    }

    /**
     * ウィンドウのタイトルを設定するメソッド
     */
    fn title(&self) -> String {
        String::from("DEMO")
    }

    /**
     * アプリケーションの状態を表示するメソッド
     */
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        // メッセージを受信した時の処理
        match message {
            Message::Start => {
                self.tick_state = TickState::Ticking;
                self.last_update = Instant::now();
            }
            Message::Stop => {
                self.tick_state = TickState::Stopped;
                self.total_duration +=  Instant::now() - self.last_update;
            }
            Message::Reset => {
                self.last_update = Instant::now();
                // 時間の経過を0にする。
                self.total_duration = Duration::default();
            }
            Message::Update => match self.tick_state {
                TickState::Ticking => {
                    let now_update = Instant::now();
                    self.total_duration += now_update - self.last_update;
                    self.last_update = now_update;
                }
                _ => {}
            },
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

        // 計測中の時間を表示するためのコード
        let seconds = self.total_duration.as_secs();
        // フォーマットを指定して表示する。
        let duration_text = format!(
            "{:0>2}:{:0>2}:{:0>2}:{:0>2}",
            seconds / HOUR,
            (seconds % HOUR) / MINUTE,
            seconds % MINUTE,
            self.total_duration.subsec_millis() / 10,
        );

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

    /**
     * サブスクリプション関数
     */
    fn subscription(&self) -> Subscription<Message> {
        // 時間を更新する間隔を定義する。
        let timer = Timer::new(Duration::from_millis(MILLISEC / FPS));
        // 指定した間隔で、メッセージを更新していく
        iced::Subscription::from_recipe(timer).map(|_| Message::Update)
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