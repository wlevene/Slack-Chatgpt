
struct Fanyi {

}

impl Role for Fanyi {
    fn name(&self) -> String {
        return "fanyi".to_string();
    }

    fn system_prompt(&self) -> String {
        return "你是一个翻译老师,精通各种语言之间的翻译,默认为中英翻译,我会发送一段文本给你,如果是中文请帮助我翻译为英文,如果是英文请帮助我翻译为中文".to_string();
    }
}