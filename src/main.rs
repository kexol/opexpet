#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Duration;

use eframe::egui::{
    self, Color32, CornerRadius, CursorIcon, FontId, Mesh, Pos2, Rect, Sense, Shape,
    Stroke, StrokeKind, TextureHandle, TextureOptions, ViewportCommand, pos2, vec2,
};

const WIN_W: f32 = 340.0;
const WIN_H: f32 = 420.0;
/// Размер ореха на экране (в точках).
const NUT_W: f32 = 210.0;
const NUT_H: f32 = 225.0;
/// Точка опоры ореха (низ по центру) в координатах окна.
const PIVOT: Pos2 = pos2(WIN_W / 2.0, WIN_H - 36.0);
/// Центр ореха в координатах окна.
const NUT_CENTER: Pos2 = pos2(PIVOT.x, PIVOT.y - NUT_H / 2.0);
/// Полуоси «тела» ореха: по ним ловим мышь и считаем столкновения.
const BODY_RX: f32 = NUT_W * 0.47;
const BODY_RY: f32 = NUT_H * 0.48;
/// Радиус качения.
const ROLL_R: f32 = (BODY_RX + BODY_RY) / 2.0;
/// Ускорение свободного падения, точек/с².
const GRAVITY: f32 = 2600.0;
/// Зазор между низом окна и панелью задач, когда орех стоит на «полу».
const FLOOR_GAP: f32 = 8.0;

const TITLE: &str = "Орех";

const GREETING: &str = "Салют! Я Орех — стойкий ореховый друг и помощник.";

/// Реплики «сами по себе» и по пункту меню «Поговорить».
const PHRASES: &[&str] = &[
    "Меня не прогрызть — скорлупа проверена зомби!",
    "Я стою здесь, пока ты работаешь. Держу оборону!",
    "Попей воды и разомни спину — я присмотрю за экраном.",
    "Тащи меня куда хочешь, я не убегу. Я же орех!",
    "Хрум-хрум… нет-нет, это не я. Честно.",
    "Ни один баг не пройдёт мимо моей скорлупы!",
    "Может, пора сделать перерыв? Я подожду.",
    "Знаешь, почему я крутой? Потому что я твёрдый орешек.",
    "Если что — я всегда поверх всех окон.",
    "Посмотри вдаль секунд двадцать — глаза скажут спасибо.",
    "Сохранил файл? Ctrl+S — лучший друг человека. После ореха.",
    "Сделай коммит, пока не поздно. Я видел, что бывает с теми, кто не коммитит.",
    "Я не умею бегать, зато умею ждать. Это тоже талант.",
    "Подсолнухи дают солнце, горохострелы стреляют, а я… я просто надёжный.",
    "Спина прямая? Проверь. Я вот всегда ровно стою.",
    "Если код не работает — это не ты, это он. Наверное.",
    "Говорят, орехи полезны для мозга. Так что я тут по делу.",
    "Тишина… Подозрительно. Зомби что-то замышляют.",
    "Музыка играет, орех на посту. Всё по плану.",
    "Не забудь поесть. Только не орехи. Пожалуйста.",
    "Часов у меня нет, но ощущение, что ты давно не отдыхал.",
    "Маленький шаг — тоже шаг. Даже если ты орех и шагать не умеешь.",
    "Перетащи меня в угол, если мешаю. Я не обижусь. Почти.",
    "Дедлайн — это просто зомби с часами. Отобьёмся!",
    "Ты молодец. Просто напоминаю.",
    "Если долго смотреть на баг, баг начинает смотреть на тебя.",
    "Хорошая новость: я никогда не зависаю. Плохая: я вообще мало что делаю.",
    "Сделай глубокий вдох… и выдох. Вот так, по-ореховому.",
    "Я в отличной форме! В овальной, если быть точным.",
    "Иногда лучший фикс — это перезагрузка. И сон.",
    "Мой рекорд — простоять целую волну зомби. А твой?",
    "Не торопись. Хорошие вещи, как и орехи, зреют не сразу.",
    "Тут где-то крошки от печенья. Не я ел, честно.",
    "Работай спокойно, я прикрываю тыл. И фронт. И фланги.",
    "Помнишь, что собирался сделать сегодня? Вот и я не помню. Запиши!",
    "Сломалось? Выдохни, налей чаю и попробуй ещё раз.",
    "Я твёрдый снаружи и мягкий внутри. Как хороший пирожок.",
    "Я поверх окон, но никогда — поверх твоих планов.",
    "Сделай музыку погромче, а то зомби подкрадываются тихо.",
    "Если задача кажется неподъёмной — разбей её на кусочки. Как скорлупу.",
    "Сегодня отличный день, чтобы ничего не сломать.",
    "Я бы помог тебе печатать, но у меня лапки. Вернее, их нет.",
    "Сколько вкладок открыто? Не отвечай, я и так переживаю.",
    "Ещё немного — и будет готово. Я в тебя верю!",
    "Проветри комнату. Свежий воздух — враг сонных зомби.",
];

/// Реакции на клик по ореху.
const POKES: &[&str] = &[
    "Ай! Полегче, у меня хоть и броня, но я живой!",
    "Эй, щекотно!",
    "Тук-тук! Кто там? Это я, орех.",
    "Не-не-не, я не кнопка!",
    "Ещё раз — и я начну трескаться. Шучу, не начну.",
    "Ух, вот это прыжок!",
    "Хи-хи, ещё!",
    "Проверяешь, твёрдый ли я? Твёрдый.",
    "Ой! Я думал, это зомби.",
    "Кликнешь сто раз — получишь… ничего. Но мне приятно.",
    "Опа! Я снова в воздухе!",
    "Осторожно, скорлупа полированная!",
    "Я тоже рад тебя видеть!",
    "Прыг-скок! Почти как горохострел. Только без гороха.",
    "Это было неожиданно. Мне понравилось.",
    "Бум! Вот это я понимаю — внимание.",
    "Привет-привет! Чем займёмся?",
    "Ай-ай! Шишка будет. Ореховая.",
    "Ещё чуть-чуть, и я научусь летать.",
    "Мягче! Я тебе не орех для колки. Хотя… ладно, орех.",
];

/// Старт режима «боулинг».
const ROLLS: &[&str] = &[
    "Боулинг-режим включён! Кегли, берегитесь!",
    "Покатился-а-а!",
    "Я колесо! Я свобода!",
    "Ореховый страйк начинается!",
    "Разойдись, зомби, орех на подходе!",
];

/// Когда орех бросили.
const THROWS: &[&str] = &[
    "Я лечу-у-у!",
    "А-а-а! Я не птица!",
    "Уи-и-и!",
    "Это был мой первый полёт. Предупреждать надо!",
    "Ну всё, я реактивный орех.",
];

/// Сильный удар о стену или пол.
const BONKS: &[&str] = &[
    "Бум! Страйк!",
    "Ой, стенка!",
    "Я в порядке! Скорлупа цела.",
    "Голова кружится… а где у меня голова?",
    "Ещё раз! Ещё!",
    "Вот это я понимаю — приземление!",
];

struct Particle {
    pos: Pos2,
    vel: egui::Vec2,
    born: f64,
    life: f64,
    size: f32,
    color: [u8; 3],
    gravity: f32,
}

/// Полёт и качение окна с орехом по экрану.
struct Motion {
    /// Левый верхний угол окна в экранных точках.
    pos: Pos2,
    vel: egui::Vec2,
    airborne: bool,
    /// Сопротивление качению, рад/с².
    friction: f32,
    /// Допустимые положения окна: по x — стены, по y — потолок и пол.
    min: Pos2,
    max: Pos2,
}

/// Поза ореха на текущий кадр.
struct Pose {
    jump: f32,
    sx: f32,
    sy: f32,
    /// Наклон вокруг нижней точки опоры.
    angle: f32,
    /// Вращение вокруг центра (качение).
    spin: f32,
    /// На сколько поднять центр, чтобы повёрнутый овал касался пола.
    lift: f32,
    /// Сплющивание от удара: масштаб по осям экрана и точка, к которой прижимаемся.
    squash: egui::Vec2,
    squash_at: Pos2,
}

fn wrap_angle(a: f32) -> f32 {
    let tau = std::f32::consts::TAU;
    a - tau * (a / tau).round()
}

struct NutApp {
    tex: TextureHandle,
    placed: bool,
    passthrough: bool,

    // речевой пузырь
    bubble_text: String,
    bubble_born: f64,
    bubble_until: f64,
    next_auto: f64,

    // анимации
    hop_at: f64,
    tilt: f32,
    tilt_vel: f32,
    last_win_pos: Option<Pos2>,
    /// Смещение курсора от левого верхнего угла окна, пока орех тащат.
    drag_grab: Option<egui::Vec2>,
    last_time: f64,
    particles: Vec<Particle>,
    rng: u64,

    // физика: бросок и качение
    motion: Option<Motion>,
    spin: f32,
    spin_vel: f32,
    impact_at: f64,
    impact_amp: f32,
    /// 0 — удар о пол, ±1 — о левую/правую стену.
    impact_side: f32,
    /// Последние положения курсора при перетаскивании — для скорости броска.
    drag_trail: Vec<(f64, Pos2)>,

    // «колоды» ещё не сказанных фраз (индексы в PHRASES / POKES)
    chat_bag: Vec<usize>,
    poke_bag: Vec<usize>,
}

impl NutApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let img = image::load_from_memory_with_format(
            include_bytes!("../assets/wallnut.webp"),
            image::ImageFormat::WebP,
        )
        .expect("не удалось декодировать wallnut.webp")
        .to_rgba8();
        let size = [img.width() as usize, img.height() as usize];
        let color = egui::ColorImage::from_rgba_unmultiplied(size, img.as_raw());
        let tex = cc
            .egui_ctx
            .load_texture("wallnut", color, TextureOptions::LINEAR);

        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0x9E37_79B9_7F4A_7C15);

        Self {
            tex,
            placed: false,
            passthrough: false,
            bubble_text: GREETING.to_owned(),
            bubble_born: 0.6,
            bubble_until: 7.0,
            next_auto: 25.0,
            hop_at: 0.0,
            tilt: 0.0,
            tilt_vel: 0.0,
            last_win_pos: None,
            drag_grab: None,
            last_time: 0.0,
            particles: Vec::new(),
            rng: seed | 1,
            motion: None,
            spin: 0.0,
            spin_vel: 0.0,
            impact_at: -10.0,
            impact_amp: 0.0,
            impact_side: 0.0,
            drag_trail: Vec::new(),
            chat_bag: Vec::new(),
            poke_bag: Vec::new(),
        }
    }

    fn rand(&mut self) -> f32 {
        // xorshift64*
        self.rng ^= self.rng >> 12;
        self.rng ^= self.rng << 25;
        self.rng ^= self.rng >> 27;
        let v = self.rng.wrapping_mul(0x2545_F491_4F6C_DD1D);
        (v >> 40) as f32 / (1u64 << 24) as f32
    }

    /// Следующая фраза из «колоды»: все фразы идут в случайном порядке
    /// и повторяются только после того, как прозвучали остальные.
    fn next_phrase(&mut self, poke: bool) -> &'static str {
        let list = if poke { POKES } else { PHRASES };
        let slot = if poke { &mut self.poke_bag } else { &mut self.chat_bag };
        let mut bag = std::mem::take(slot);
        if bag.is_empty() {
            bag.extend(0..list.len());
            for i in (1..bag.len()).rev() {
                let j = (self.rand() * (i + 1) as f32) as usize % (i + 1);
                bag.swap(i, j);
            }
            // на стыке колод не повторяем только что сказанное
            let last = bag.len() - 1;
            if last > 0 && list[bag[last]] == self.bubble_text {
                bag.swap(0, last);
            }
        }
        let phrase = list[bag.pop().unwrap_or(0)];
        *(if poke { &mut self.poke_bag } else { &mut self.chat_bag }) = bag;
        phrase
    }

    fn say(&mut self, now: f64, text: &str) {
        let text = text.to_owned();
        let reading = 3.5 + text.chars().count() as f64 * 0.06;
        self.bubble_born = now;
        self.bubble_until = now + reading + 1.0;
        self.bubble_text = text;
        self.next_auto = self.bubble_until + 18.0 + self.rand() as f64 * 25.0;
    }

    /// Прыжок с крошками и репликой: реакция на клик или обычная болтовня.
    fn poke(&mut self, now: f64, reaction: bool) {
        self.hop_at = now;
        let phrase = self.next_phrase(reaction);
        self.say(now, phrase);
        for _ in 0..14 {
            let a = -std::f32::consts::FRAC_PI_2 + (self.rand() - 0.5) * 2.4;
            let speed = 110.0 + self.rand() * 170.0;
            let size = 2.5 + self.rand() * 3.5;
            let off = (self.rand() - 0.5) * 100.0;
            let life = 0.7 + self.rand() as f64 * 0.4;
            self.particles.push(Particle {
                pos: pos2(PIVOT.x + off, PIVOT.y - NUT_H * 0.7),
                vel: vec2(a.cos(), a.sin()) * speed,
                born: now,
                life,
                size,
                color: [196, 150, 40],
                gravity: 520.0,
            });
        }
    }

    fn pick(&mut self, list: &[&'static str]) -> &'static str {
        list[(self.rand() * list.len() as f32) as usize % list.len()]
    }

    /// Облачко пыли в координатах окна.
    fn dust(&mut self, now: f64, at: Pos2, dir: egui::Vec2, count: usize) {
        for _ in 0..count {
            let side = vec2(-dir.y, dir.x) * (self.rand() - 0.5) * 2.0;
            let vel = (dir * (0.3 + self.rand()) + side) * (60.0 + self.rand() * 160.0);
            let life = 0.4 + self.rand() as f64 * 0.4;
            let size = 3.0 + self.rand() * 5.0;
            let shade = 120 + (self.rand() * 50.0) as u8;
            let jitter = vec2(self.rand() - 0.5, self.rand() - 0.5) * 30.0;
            self.particles.push(Particle {
                pos: at + jitter,
                vel,
                born: now,
                life,
                size,
                color: [shade + 40, shade + 20, shade],
                gravity: -40.0,
            });
        }
    }

    /// Удар о пол (side = 0) или стену (side = ±1): сплющивание, пыль, иногда реплика.
    fn impact(&mut self, now: f64, strength: f32, side: f32) {
        let strength = strength.clamp(0.0, 1.0);
        if strength < 0.06 {
            return;
        }
        self.impact_at = now;
        self.impact_amp = 0.22 * strength;
        self.impact_side = side;
        let (at, dir) = if side == 0.0 {
            (PIVOT, vec2(0.0, -1.0))
        } else {
            (NUT_CENTER + vec2(side * BODY_RX, 0.0), vec2(-side, 0.0))
        };
        self.dust(now, at, dir, (strength * 18.0) as usize);
        if strength > 0.45 && now > self.bubble_until {
            let phrase = self.pick(BONKS);
            self.say(now, phrase);
        }
    }

    /// Отпускаем окно в свободный полёт в пределах рабочей области его монитора.
    fn start_motion(&mut self, ctx: &egui::Context, win: Rect, vel: egui::Vec2, friction: f32) {
        let ppp = ctx.pixels_per_point();
        let monitor = ctx.input(|i| i.viewport().monitor_size);
        let Some(area) = work_area(win.center() * ppp)
            .map(|r| Rect::from_min_max(r.min / ppp, r.max / ppp))
            .or_else(|| monitor.map(|m| Rect::from_min_size(Pos2::ZERO, m)))
        else {
            return;
        };
        let min = pos2(
            area.min.x - (NUT_CENTER.x - BODY_RX),
            area.min.y - (NUT_CENTER.y - BODY_RY),
        );
        let max = pos2(
            area.max.x - (NUT_CENTER.x + BODY_RX),
            area.max.y - WIN_H - FLOOR_GAP,
        );
        if min.x > max.x || min.y > max.y {
            return;
        }
        let pos = win.min.clamp(min, max);
        self.motion = Some(Motion {
            pos,
            vel,
            airborne: pos.y < max.y - 0.5 || vel.y < 0.0,
            friction,
            min,
            max,
        });
    }

    /// Шаг физики. Возвращает, куда поставить окно.
    fn step_motion(&mut self, now: f64, dt: f32) -> Option<Pos2> {
        let mut m = self.motion.take()?;
        const STEPS: usize = 4;
        let h = dt / STEPS as f32;
        for _ in 0..STEPS {
            if m.airborne {
                m.vel.y += GRAVITY * h;
                m.pos += m.vel * h;
                self.spin += self.spin_vel * h;
                if m.pos.y < m.min.y {
                    m.pos.y = m.min.y;
                    m.vel.y = m.vel.y.abs() * 0.5;
                }
                if m.pos.y >= m.max.y {
                    m.pos.y = m.max.y;
                    let vy = m.vel.y;
                    if vy > 260.0 {
                        m.vel.y = -vy * 0.42;
                    } else {
                        m.vel.y = 0.0;
                        m.airborne = false;
                    }
                    // трение о пол уравнивает скорость и вращение
                    let w = (self.spin_vel + m.vel.x / ROLL_R) / 2.0;
                    self.spin_vel = w;
                    m.vel.x = w * ROLL_R;
                    self.impact(now, vy / 2400.0, 0.0);
                }
            } else {
                // Орех — «неваляшка»: тяжёлый низ тянет его встать ровно,
                // поэтому катится он с раскачкой, а в конце покачивается.
                let w = self.spin_vel;
                let mut a = -12.0 * self.spin.sin() - if w.abs() < 2.0 { 2.5 } else { 0.15 } * w;
                if w.abs() > 0.6 {
                    a -= m.friction * w.signum();
                }
                self.spin_vel += a * h;
                self.spin += self.spin_vel * h;
                m.vel = vec2(self.spin_vel * ROLL_R, 0.0);
                m.pos.x += m.vel.x * h;
            }
            if m.pos.x < m.min.x || m.pos.x > m.max.x {
                let side = if m.pos.x < m.min.x { -1.0 } else { 1.0 };
                m.pos.x = m.pos.x.clamp(m.min.x, m.max.x);
                let vx = m.vel.x;
                m.vel.x = -vx * 0.7;
                self.spin_vel = if m.airborne {
                    -self.spin_vel * 0.6
                } else {
                    m.vel.x / ROLL_R
                };
                self.impact(now, vx.abs() / 1600.0, side);
            }
        }
        self.spin = wrap_angle(self.spin);

        // пыль из-под ореха на быстром качении
        let vx = m.vel.x;
        if !m.airborne && vx.abs() > 400.0 && self.rand() < vx.abs() / 2500.0 {
            let back = -vx.signum();
            self.dust(now, PIVOT + vec2(back * 25.0, -4.0), vec2(back, -0.6), 1);
        }

        let pos = m.pos;
        let settled = !m.airborne && self.spin_vel.abs() < 0.05 && self.spin.abs() < 0.01;
        if settled {
            self.spin = 0.0;
            self.spin_vel = 0.0;
        } else {
            self.motion = Some(m);
        }
        Some(pos)
    }

    fn nut_pose(&self, now: f64, hover: f32) -> Pose {
        let t = now as f32;
        // дыхание
        let breath = (t * 2.3).sin();
        let mut sx = 1.0 - 0.014 * breath + 0.03 * hover;
        let mut sy = 1.0 + 0.022 * breath + 0.03 * hover;
        let mut angle = 0.025 * (t * 1.15).sin() + self.tilt;
        let mut jump = 0.0;

        // прыжок и приземление после клика
        let e = (now - self.hop_at) as f32;
        if (0.0..2.0).contains(&e) {
            let p = e / 0.55;
            if p < 1.0 {
                jump = 62.0 * 4.0 * p * (1.0 - p);
                let s = (std::f32::consts::PI * p).sin();
                sy += 0.10 * s;
                sx -= 0.06 * s;
            } else if p < 1.45 {
                let q = (p - 1.0) / 0.45;
                let s = (std::f32::consts::PI * q).sin();
                sy -= 0.17 * s;
                sx += 0.10 * s;
            }
            angle += 0.16 * (-3.2 * e).exp() * (24.0 * e).sin();
        }

        // повёрнутый овал выше или ниже — центр поднимаем так, чтобы он касался пола
        let (ss, sc) = self.spin.sin_cos();
        let lift = ((BODY_RX * ss).powi(2) + (BODY_RY * sc).powi(2)).sqrt() - BODY_RY;

        // сплющивание при ударе, с пружинистой отдачей
        let mut squash = vec2(1.0, 1.0);
        let mut squash_at = PIVOT;
        let e = (now - self.impact_at) as f32;
        if (0.0..0.7).contains(&e) {
            let s = self.impact_amp * (-7.0 * e).exp() * (20.0 * e).cos();
            if self.impact_side == 0.0 {
                squash = vec2(1.0 + 0.7 * s, 1.0 - s);
            } else {
                squash = vec2(1.0 - s, 1.0 + 0.7 * s);
                squash_at = NUT_CENTER + vec2(self.impact_side * BODY_RX, 0.0);
            }
        }

        Pose {
            jump,
            sx,
            sy,
            angle,
            spin: self.spin,
            lift,
            squash,
            squash_at,
        }
    }

    fn draw_nut(&self, painter: &egui::Painter, pose: &Pose, shift: egui::Vec2, alpha: f32) {
        let (sin, cos) = pose.angle.sin_cos();
        let (ssin, scos) = pose.spin.sin_cos();
        let pivot = pos2(PIVOT.x, PIVOT.y - pose.jump) + shift;
        let center = vec2(0.0, -NUT_H * pose.sy / 2.0);
        let color = Color32::WHITE.gamma_multiply(alpha);
        let mut mesh = Mesh::with_texture(self.tex.id());
        let corners = [
            (-0.5, -1.0, 0.0, 0.0),
            (0.5, -1.0, 1.0, 0.0),
            (0.5, 0.0, 1.0, 1.0),
            (-0.5, 0.0, 0.0, 1.0),
        ];
        for (cx, cy, u, v) in corners {
            // вращение вокруг центра, затем наклон вокруг точки опоры
            let p = vec2(cx * NUT_W * pose.sx, cy * NUT_H * pose.sy) - center;
            let p = center + vec2(p.x * scos - p.y * ssin, p.x * ssin + p.y * scos);
            let p = p - vec2(0.0, pose.lift);
            let pos = pivot + vec2(p.x * cos - p.y * sin, p.x * sin + p.y * cos);
            let pos = pose.squash_at + (pos - pose.squash_at) * pose.squash;
            mesh.vertices.push(egui::epaint::Vertex {
                pos,
                uv: pos2(u, v),
                color,
            });
        }
        mesh.add_triangle(0, 1, 2);
        mesh.add_triangle(0, 2, 3);
        painter.add(Shape::mesh(mesh));
    }

    /// Тень лежит на полу: `air` — высота окна над полом.
    fn draw_shadow(&self, painter: &egui::Painter, jump: f32, air: f32) {
        let k = 1.0 - ((jump + air) / 140.0).clamp(0.0, 0.6);
        let center = pos2(PIVOT.x, PIVOT.y + 2.0 + air);
        for i in 0..5 {
            let f = 1.0 - i as f32 * 0.16;
            let r = vec2(NUT_W * 0.46 * k * f, 12.0 * k * f);
            painter.add(Shape::ellipse_filled(
                center,
                r,
                Color32::from_black_alpha(20),
            ));
        }
    }

    fn draw_particles(&mut self, painter: &egui::Painter, now: f64, dt: f32) {
        self.particles.retain(|p| now - p.born < p.life);
        for p in &mut self.particles {
            p.vel.y += p.gravity * dt;
            p.pos += p.vel * dt;
            let k = 1.0 - ((now - p.born) / p.life) as f32;
            let [r, g, b] = p.color;
            let c = Color32::from_rgba_unmultiplied(r, g, b, (255.0 * k) as u8);
            painter.circle_filled(p.pos, p.size * (0.4 + 0.6 * k), c);
        }
    }

    fn draw_bubble(&self, ui: &egui::Ui, now: f64) {
        if now < self.bubble_born || now > self.bubble_until {
            return;
        }
        let painter = ui.painter();
        let age = (now - self.bubble_born) as f32;
        let left = (self.bubble_until - now) as f32;

        // появление «с пружинкой» и плавное исчезновение
        let appear = (age / 0.28).clamp(0.0, 1.0);
        let pop = 1.0 + 1.70158 * (appear - 1.0).powi(3) + 0.70158 * (appear - 1.0).powi(2);
        let scale = pop.max(0.0);
        let alpha = ((left / 0.35).clamp(0.0, 1.0) * 255.0) as u8;
        if alpha == 0 {
            return;
        }

        let font = FontId::proportional(15.0);
        let wrap = 250.0;
        let dark = Color32::from_rgba_unmultiplied(32, 36, 44, alpha);

        let full = painter.layout(self.bubble_text.clone(), font.clone(), dark, wrap);
        let shown = ((age - 0.15).max(0.0) * 38.0) as usize;
        let partial: String = self.bubble_text.chars().take(shown).collect();
        let partial = painter.layout(partial, font, dark, wrap);

        let pad = vec2(14.0, 10.0);
        let size = full.size() + pad * 2.0;
        // хвостик указывает на орех (чуть выше его макушки)
        let tail_tip = pos2(WIN_W / 2.0, PIVOT.y - NUT_H - 12.0);
        let bottom = tail_tip.y - 12.0;
        let rect = Rect::from_min_size(pos2((WIN_W - size.x) / 2.0, bottom - size.y), size);

        // масштабируем относительно кончика хвостика
        let tf = |p: Pos2| tail_tip + (p - tail_tip) * scale;
        let r = Rect::from_min_max(tf(rect.min), tf(rect.max));

        let fill = Color32::from_rgba_unmultiplied(255, 255, 255, alpha);
        let edge = Color32::from_rgba_unmultiplied(205, 208, 215, alpha);
        let shadow = Color32::from_black_alpha((alpha as f32 * 0.16) as u8);
        let cr = CornerRadius::same(12);

        painter.rect_filled(r.translate(vec2(0.0, 3.0)).expand(1.0), cr, shadow);
        painter.rect_filled(r, cr, fill);
        painter.rect_stroke(r, cr, Stroke::new(1.0, edge), StrokeKind::Inside);

        let tail = vec![
            tf(pos2(tail_tip.x - 9.0, bottom - 1.0)),
            tf(pos2(tail_tip.x + 9.0, bottom - 1.0)),
            tf(tail_tip),
        ];
        painter.add(Shape::convex_polygon(tail.clone(), fill, Stroke::NONE));
        painter.line_segment([tail[0], tail[2]], Stroke::new(1.0, edge));
        painter.line_segment([tail[1], tail[2]], Stroke::new(1.0, edge));

        if scale > 0.9 {
            painter.galley(r.min + pad * scale, partial, dark);
        }
    }
}

#[cfg(windows)]
fn cursor_screen_pos() -> Option<Pos2> {
    use windows_sys::Win32::Foundation::POINT;
    use windows_sys::Win32::UI::WindowsAndMessaging::GetCursorPos;
    let mut p = POINT { x: 0, y: 0 };
    // SAFETY: p — валидный указатель на POINT.
    if unsafe { GetCursorPos(&mut p) } != 0 {
        Some(pos2(p.x as f32, p.y as f32))
    } else {
        None
    }
}

/// Рабочая область (без панели задач) монитора, ближайшего к точке `at`;
/// всё в физических пикселях.
#[cfg(windows)]
fn work_area(at: Pos2) -> Option<Rect> {
    use windows_sys::Win32::Foundation::{POINT, RECT};
    use windows_sys::Win32::Graphics::Gdi::{
        GetMonitorInfoW, MONITOR_DEFAULTTONEAREST, MONITORINFO, MonitorFromPoint,
    };
    let empty = RECT { left: 0, top: 0, right: 0, bottom: 0 };
    let mut info = MONITORINFO {
        cbSize: std::mem::size_of::<MONITORINFO>() as u32,
        rcMonitor: empty,
        rcWork: empty,
        dwFlags: 0,
    };
    let pt = POINT { x: at.x as i32, y: at.y as i32 };
    // SAFETY: MONITOR_DEFAULTTONEAREST всегда возвращает монитор;
    // info — валидный MONITORINFO с заполненным cbSize.
    if unsafe { GetMonitorInfoW(MonitorFromPoint(pt, MONITOR_DEFAULTTONEAREST), &mut info) } != 0 {
        let r = info.rcWork;
        Some(Rect::from_min_max(
            pos2(r.left as f32, r.top as f32),
            pos2(r.right as f32, r.bottom as f32),
        ))
    } else {
        None
    }
}

/// Убираем системную рамку и скругление Windows 11 у окна без декораций,
/// иначе вокруг прозрачного окна видна тонкая линия.
#[cfg(windows)]
fn strip_window_chrome(title: &str) {
    use windows_sys::Win32::Graphics::Dwm::DwmSetWindowAttribute;
    use windows_sys::Win32::UI::WindowsAndMessaging::FindWindowW;
    let wide: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();
    // SAFETY: wide завершена нулём; HWND проверяем на null перед использованием.
    unsafe {
        let hwnd = FindWindowW(std::ptr::null(), wide.as_ptr());
        if hwnd.is_null() {
            return;
        }
        let border_none: u32 = 0xFFFF_FFFE; // DWMWA_COLOR_NONE
        let no_round: u32 = 1; // DWMWCP_DONOTROUND
        DwmSetWindowAttribute(hwnd, 34, &border_none as *const _ as *const _, 4);
        let ncr_disabled: u32 = 2; // DWMNCRP_DISABLED — без тени окна
        DwmSetWindowAttribute(hwnd, 2, &ncr_disabled as *const _ as *const _, 4);
        DwmSetWindowAttribute(hwnd, 33, &no_round as *const _ as *const _, 4);
        disable_undecorated_shadow(hwnd);
    }
}

#[cfg(windows)]
static PREV_WND_PROC: std::sync::atomic::AtomicIsize = std::sync::atomic::AtomicIsize::new(0);

/// egui-winit всегда включает у окон без рамки «undecorated shadow»: winit оставляет
/// 1px неклиентской области в WM_NCCALCSIZE, и DWM рисует по ней тень.
/// Подменяем оконную процедуру и отдаём всё окно под клиентскую область.
#[cfg(windows)]
unsafe fn disable_undecorated_shadow(hwnd: windows_sys::Win32::Foundation::HWND) {
    use std::sync::atomic::Ordering;
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        GWLP_WNDPROC, SWP_FRAMECHANGED, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER,
        SetWindowLongPtrW, SetWindowPos,
    };
    if PREV_WND_PROC.load(Ordering::Relaxed) != 0 {
        return;
    }
    // SAFETY: hwnd — живое окно этого потока; прежнюю процедуру сохраняем и вызываем дальше.
    unsafe {
        let prev = SetWindowLongPtrW(hwnd, GWLP_WNDPROC, no_shadow_wnd_proc as *const () as isize);
        PREV_WND_PROC.store(prev, Ordering::Relaxed);
        // пересчитать неклиентскую область уже с новой процедурой
        SetWindowPos(
            hwnd,
            std::ptr::null_mut(),
            0,
            0,
            0,
            0,
            SWP_FRAMECHANGED | SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE,
        );
    }
}

#[cfg(windows)]
unsafe extern "system" fn no_shadow_wnd_proc(
    hwnd: windows_sys::Win32::Foundation::HWND,
    msg: u32,
    wparam: windows_sys::Win32::Foundation::WPARAM,
    lparam: windows_sys::Win32::Foundation::LPARAM,
) -> windows_sys::Win32::Foundation::LRESULT {
    use std::sync::atomic::Ordering;
    use windows_sys::Win32::UI::WindowsAndMessaging::{CallWindowProcW, WM_NCCALCSIZE, WNDPROC};
    if msg == WM_NCCALCSIZE && wparam != 0 {
        return 0; // клиентская область = всё окно, без рамки и тени
    }
    // SAFETY: PREV_WND_PROC хранит процедуру, которую вернул SetWindowLongPtrW.
    unsafe {
        let prev: WNDPROC = std::mem::transmute(PREV_WND_PROC.load(Ordering::Relaxed));
        CallWindowProcW(prev, hwnd, msg, wparam, lparam)
    }
}

#[cfg(not(windows))]
fn strip_window_chrome(_title: &str) {}

#[cfg(not(windows))]
fn work_area(_at: Pos2) -> Option<Rect> {
    None
}

#[cfg(not(windows))]
fn cursor_screen_pos() -> Option<Pos2> {
    None
}

impl eframe::App for NutApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        let now = ctx.input(|i| i.time);
        let dt = ((now - self.last_time) as f32).clamp(0.0, 0.05);
        self.last_time = now;

        let (win_rect, monitor) = ctx.input(|i| {
            let v = i.viewport();
            (v.inner_rect, v.monitor_size)
        });

        // стартовая позиция — правый нижний угол над панелью задач
        if !self.placed && win_rect.is_some() {
            let ppp = ctx.pixels_per_point();
            // (0, 0) всегда на основном мониторе
            let area = work_area(Pos2::ZERO)
                .map(|r| Rect::from_min_max(r.min / ppp, r.max / ppp))
                .or_else(|| monitor.map(|m| Rect::from_min_size(Pos2::ZERO, m)));
            if let Some(a) = area {
                ctx.send_viewport_cmd(ViewportCommand::OuterPosition(pos2(
                    a.max.x - WIN_W - 24.0,
                    a.max.y - WIN_H - FLOOR_GAP,
                )));
                self.placed = true;
                self.last_win_pos = None;
                strip_window_chrome(TITLE);
            }
        }

        if let Some(r) = win_rect {
            if let Some(prev) = self.last_win_pos {
                let d = r.min - prev;
                if d.length() < 400.0 {
                    // частицы живут в координатах экрана, а не окна
                    for p in &mut self.particles {
                        p.pos -= d;
                    }
                    // инерция: орех «отстаёт» от окна, которое тащат, и покачивается
                    if self.motion.is_none() && d.x.abs() < 150.0 {
                        self.tilt_vel -= d.x * 0.16;
                    }
                }
            }
            self.last_win_pos = Some(r.min);
        }
        self.tilt_vel += (-130.0 * self.tilt - 7.5 * self.tilt_vel) * dt;
        self.tilt = (self.tilt + self.tilt_vel * dt).clamp(-0.5, 0.5);

        // автоматические реплики
        if now > self.next_auto {
            let phrase = self.next_phrase(false);
            self.say(now, phrase);
        }

        // --- геометрия и ввод ---
        let nut_radii = vec2(BODY_RX, BODY_RY);
        let hit_rect = Rect::from_center_size(NUT_CENTER, nut_radii * 2.0);

        let resp = ui.interact(hit_rect, egui::Id::new("nut"), Sense::click_and_drag());
        let mut want_quit = false;
        let mut want_talk = false;
        let mut want_roll = false;
        resp.context_menu(|ui| {
            if ui.button("Поговорить").clicked() {
                want_talk = true;
                ui.close();
            }
            if ui.button("Покатиться").clicked() {
                want_roll = true;
                ui.close();
            }
            if ui.button("Выход").clicked() {
                want_quit = true;
                ui.close();
            }
        });
        if want_quit {
            ctx.send_viewport_cmd(ViewportCommand::Close);
        }
        if resp.clicked() {
            self.poke(now, true);
        } else if want_talk {
            self.poke(now, false);
        }
        // Таскаем окно сами: системный StartDrag не пускает верх окна выше
        // экрана, а над орехом в окне ещё ~160px места под пузырь.
        let cursor = cursor_screen_pos().map(|c| (c.to_vec2() / ctx.pixels_per_point()).to_pos2());
        if resp.drag_started_by(egui::PointerButton::Primary) {
            // поймали на лету — полёт прерывается
            self.motion = None;
            self.drag_trail.clear();
            match (cursor, win_rect) {
                (Some(c), Some(r)) => self.drag_grab = Some(c - r.min),
                _ => ctx.send_viewport_cmd(ViewportCommand::StartDrag),
            }
        }
        if let (Some(_), Some(c)) = (self.drag_grab, cursor) {
            self.drag_trail.push((now, c));
            self.drag_trail.retain(|&(t, _)| now - t < 0.1);
        }
        // Бросок: если отпустили на ходу, орех улетает с той же скоростью.
        if self.drag_grab.is_some() && !resp.dragged() {
            if let (Some(&(t0, p0)), Some(&(t1, p1)), Some(r)) =
                (self.drag_trail.first(), self.drag_trail.last(), win_rect)
            {
                let span = (t1 - t0) as f32;
                let vel = if span > 0.02 { (p1 - p0) / span } else { egui::Vec2::ZERO };
                if vel.length() > 500.0 {
                    let vel = vel.normalized() * vel.length().min(4000.0);
                    self.start_motion(&ctx, r, vel, 2.5);
                    self.spin_vel += vel.x / ROLL_R * 0.6;
                    if now > self.bubble_until {
                        let phrase = self.pick(THROWS);
                        self.say(now, phrase);
                    }
                }
            }
            self.drag_trail.clear();
        }
        if !resp.dragged() {
            self.drag_grab = None;
        }
        if let (Some(grab), Some(c), Some(r)) = (self.drag_grab, cursor, win_rect) {
            let target = c - grab;
            if (target - r.min).length_sq() > 0.25 {
                ctx.send_viewport_cmd(ViewportCommand::OuterPosition(target));
            }
        }

        // Боулинг: подпрыгнуть и покатиться к дальнему краю экрана.
        if want_roll && let Some(r) = win_rect {
            self.start_motion(&ctx, r, egui::Vec2::ZERO, 1.0);
            if let Some(m) = &mut self.motion {
                let dir = if r.min.x - m.min.x > m.max.x - r.min.x { -1.0 } else { 1.0 };
                m.vel = vec2(dir * 1150.0, -650.0);
                m.airborne = true;
                self.spin_vel = m.vel.x / ROLL_R;
                let phrase = self.pick(ROLLS);
                self.say(now, phrase);
            }
        }

        if let Some(target) = self.step_motion(now, dt) {
            ctx.send_viewport_cmd(ViewportCommand::OuterPosition(target));
        } else {
            // без движения орех сам выпрямляется после качения или броска
            self.spin = wrap_angle(self.spin);
            self.spin_vel += (-60.0 * self.spin - 9.0 * self.spin_vel) * dt;
            self.spin += self.spin_vel * dt;
            if self.spin.abs() < 0.002 && self.spin_vel.abs() < 0.02 {
                self.spin = 0.0;
                self.spin_vel = 0.0;
            }
        }
        if resp.hovered() || resp.dragged() {
            ctx.set_cursor_icon(if resp.dragged() {
                CursorIcon::Grabbing
            } else {
                CursorIcon::Grab
            });
        }

        // Прозрачные области окна пропускают клики сквозь себя:
        // ловим мышь только над самим орехом (или пока открыто меню).
        let over_nut = match (cursor, win_rect) {
            (Some(c), Some(r)) => {
                let local = c - r.min.to_vec2();
                let d = (local - NUT_CENTER) / nut_radii;
                d.x * d.x + d.y * d.y <= 1.0
            }
            _ => true,
        };
        let interactive = over_nut || resp.dragged() || resp.context_menu_opened();
        if self.passthrough == interactive {
            self.passthrough = !interactive;
            ctx.send_viewport_cmd(ViewportCommand::MousePassthrough(self.passthrough));
        }

        // --- отрисовка ---
        let hover = ctx.animate_bool_with_time(egui::Id::new("hover"), resp.hovered(), 0.15);
        let pose = self.nut_pose(now, hover);
        let painter = ui.painter().clone();
        let air = self.motion.as_ref().map_or(0.0, |m| m.max.y - m.pos.y);
        self.draw_shadow(&painter, pose.jump, air);
        // на большой скорости за орехом тянется след
        if let Some(m) = &self.motion {
            let speed = m.vel.length();
            if speed > 650.0 {
                let fade = ((speed - 650.0) / 600.0).min(1.0);
                for i in (1..=3).rev() {
                    let lag = 0.022 * i as f32;
                    let ghost = Pose {
                        spin: pose.spin - self.spin_vel * lag,
                        squash: vec2(1.0, 1.0),
                        ..pose
                    };
                    let alpha = fade * 0.3 / i as f32;
                    self.draw_nut(&painter, &ghost, -m.vel * lag, alpha);
                }
            }
        }
        self.draw_nut(&painter, &pose, egui::Vec2::ZERO, 1.0);
        self.draw_particles(&painter, now, dt);
        self.draw_bubble(ui, now);

        // 60 к/с нужны только для быстрых движений; в покое орех лишь
        // медленно «дышит», и ему хватает ~25 к/с — так GPU почти не нагружается.
        let bubble_age = now - self.bubble_born;
        let typing_done = 0.15 + self.bubble_text.chars().count() as f64 / 38.0;
        let bubble_busy = now >= self.bubble_born
            && now <= self.bubble_until
            && (bubble_age < typing_done.max(0.28) || self.bubble_until - now < 0.35);
        let busy = now - self.hop_at < 2.0
            || !self.particles.is_empty()
            || bubble_busy
            || self.tilt.abs() > 0.002
            || self.tilt_vel.abs() > 0.02
            || self.motion.is_some()
            || self.spin != 0.0
            || now - self.impact_at < 0.7
            || resp.dragged();
        let frame_ms = if busy { 16 } else { 40 };
        ctx.request_repaint_after(Duration::from_millis(frame_ms));
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(TITLE)
            .with_inner_size([WIN_W, WIN_H])
            .with_position([1200.0, 450.0])
            .with_decorations(false)
            .with_transparent(true)
            .with_always_on_top()
            .with_resizable(false)
            .with_taskbar(false),
        ..Default::default()
    };
    eframe::run_native(
        TITLE,
        options,
        Box::new(|cc| Ok(Box::new(NutApp::new(cc)))),
    )
}
