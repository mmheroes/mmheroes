[![Build](https://github.com/mmheroes/mmheroes/actions/workflows/build.yml/badge.svg)](https://github.com/mmheroes/mmheroes/actions/workflows/build.yml)
# [WIP] Герои Мата и Меха

Попытка переписать известный в узких кругах текстовый квест «Герои Мата и Меха» на Rust. **Ещё не закончено!**

![Screenshot](https://github.com/mmheroes/mmheroes/blob/master/.github/screenshot.png?raw=true)

Оригинальная версия «Героев Мата и Меха» была написана в 1998-м году на Borland Pascal 7.0 под MS-DOS.

В 2012-м году появился [порт на JavaScript](https://sharpden.github.io/mmheroes), но у него [есть свои недостатки](http://sharpc.livejournal.com/75856.html).

Хотелось переписать игру с нуля, переосмыслив архитектуру, сделав её более портабельной и более async-friendly, ну и вообще чтобы красивенько было.
Так в 2020-м году началась работа над тем, что ты сейчас читаешь.

На данный момент предварительные версии доступны на:
- [x] Windows/macOS/Linux (через командную строку)
- [x] iOS
- [ ] Android
- [ ] Web (через WASM)

## А когда будет готово?
Не знаю. Возможно, никогда.

## Структура репозитория

- `mmheroes-core-rs` — движок квеста, написанный на Rust + сишный FFI к нему.
  Спроектирован таким образом, чтобы его можно было использовать независимо от способа рендеринга, будь то `ncurses` в терминале
  или `UIView` в iOS.
  Более того, поддерживает `no_std`, что теоретически позволяет использовать его в embedded-окружениях, например,
  на микроконтроллере или в ядре операционной системы. Не то чтобы в этом была цель, просто хотелось удостовериться в максимальной портабельности :)
- `mmheroes-rs` — приложение для терминала, использует `ncurses` для рендеринга на Linux/macOS и PDCurses для рендеринга на Windows.
- `mmheroes-ios` — приложение для iOS, состоит из графической части и обвязки FFI движка на Свифте.
  Намеренно сделано максимально тупым способом, без архитектурных излишеств.

## Как собирать
Здесь инструкции для программистов. Если ты просто хочешь поиграть, то пока рано — разработка ещё в процессе (и пока не близится к завершению).
Поиграй лучше в [JS-версию](https://sharpden.github.io/mmheroes).

Тулчейн для сборки программ на Rust можно взять [здесь](https://www.rust-lang.org/tools/install).

Приложение для терминала собирается с помощью Cargo (пакетного менеджера для Rust):
```
git clone https://github.com/mmheroes/mmheroes.git
cd mmheroes
cargo run
```

Приложения для iOS собирается с помощью Xcode. Но нужны некоторые дополнительные шаги:
1. Убеждаемся, что установлен Rust.
1. Устанавливаем тулчейны Rust с поддержкой сборки под iOS:
   ```
   rustup target add aarch64-apple-ios x86_64-apple-ios
   ```
1. Если будем собирать проект для устройства (а не для симулятора), нужно будет в каталоге `mmheroes-ios` создать файл
   с именем `DeveloperSettings.xcconfig` и следующим содержимым:
   ```
   DEVELOPMENT_TEAM=<your team ID>
   ORGANIZATION_IDENTIFIER=<your organization identifier>
   ```
   
   `DEVELOPMENT_TEAM` — это идентификатор, который можно узнать, залогинившись в
   [Apple Developer Center](https://developer.apple.com/account/) и открыв раздел "Membership". Team ID — это оно.
   
   `ORGANIZATION_IDENTIFIER` — это любая строка в reverse-DNS-нотации. Используется для формирования bundle identifier'а.
1. Открываем проект Xcode и собираем, проблем быть не должно. Проблемы всё же есть? Смело [открывай issue](https://github.com/mmheroes/mmheroes/issues/new).
    
