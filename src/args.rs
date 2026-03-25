use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(name = "ragnarok", about = "⚔  Ragnarök Web Fuzzer — pillage the web")]
pub struct Args {
    /// Цель: http://target.com/ или http://target.com/ODIN
    #[arg(short, long)]
    pub url: String,

    /// Рунный свиток (wordlist)
    #[arg(short, long)]
    pub wordlist: String,

    /// Число воинов (параллельные потоки)
    #[arg(short = 'W', long, default_value_t = 50)]
    pub warriors: usize,

    /// Таймаут запроса (секунды)
    #[arg(short = 'T', long, default_value_t = 7)]
    pub timeout: u64,
                     
    /// Максимум директорий для рекурсии на каждом уровне
    #[arg(long, default_value_t = 10)]
    pub max_dirs: usize,

    /// Коды Вальгаллы — статус-коды для вывода
    #[arg(short, long, default_value = "200,204,301,302,307,401,403,405")]
    pub valhalla: String,

    /// Расширения: php,html,txt
    #[arg(short, long, default_value = "")]
    pub runes: String,

    /// Глубина рекурсии (уровни Иггдрасиля)
    #[arg(short = 'd', long, default_value_t = 3)]
    pub depth: usize,

    /// Отключить рекурсию
    #[arg(long)]
    pub no_recurse: bool,

    /// Фильтр минимального размера ответа (байт)
    #[arg(long, default_value_t = 0)]
    pub min_size: u64,

    /// Фильтр максимального размера ответа (байт)
    #[arg(long, default_value_t = u64::MAX)]
    pub max_size: u64,

    /// Фильтр по числу слов в ответе (через запятую)
    #[arg(long, default_value = "")]
    pub filter_words: String,

    /// Сохранить добычу в файл
    #[arg(short, long)]
    pub loot: Option<String>,

    /// Сохранить добычу в JSON
    #[arg(long)]
    pub json: Option<String>,

    /// User-Agent
    #[arg(long, default_value = "Ragnarok/0.3 (Viking Fuzzer)")]
    pub shield: String,

    /// Прокси: http://127.0.0.1:8080 или socks5://127.0.0.1:1080
    #[arg(short = 'x', long)]
    pub proxy: Option<String>,

    /// Replay-прокси для найденных URL
    #[arg(long)]
    pub replay_proxy: Option<String>,

    /// Кастомные заголовки: "Authorization: Bearer token"
    #[arg(short = 'H', long)]
    pub headers: Vec<String>,

    /// Cookie строка: "session=abc123; token=xyz"
    #[arg(short = 'b', long)]
    pub cookies: Option<String>,

    /// HTTP метод: GET, POST, PUT, DELETE
    #[arg(short = 'X', long, default_value = "GET")]
    pub method: String,

    /// Тело POST-запроса: "username=admin&password=ODIN"
    #[arg(long)]
    pub body: Option<String>,

    /// Rate limit — макс запросов в секунду
    #[arg(long, default_value_t = 0)]
    pub rate_limit: u64,

    /// Включить wildcard-детектор
    #[arg(long)]
    pub wildcard: bool,
}
