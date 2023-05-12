# README.md

# MiniGrep

# 🇺🇸

# README.md

# MiniGrep

MiniGrep is a simplified version of the classic UNIX command-line tool called `grep` (global regular expression print). It searches files for a given string or regular expression and prints lines that contain matches. This is a Rust implementation.

## Compiling the Project

Make sure you have Rust installed on your machine. If not, refer to the [official documentation](https://www.rust-lang.org/tools/install) for installation.

To compile the project, use the following command:

```bash
cargo build --release
```

## Running the Application

To run the application, use the following command:

```bash
cargo run <query> <filename>
```

where:
- `<query>` is the string you want to search for,
- `<filename>` is the name of the file you want to search in.

## Running Tests

To run the tests, use the following command:

```bash
cargo test
```

## Switching Between Case-Sensitive and Case-Insensitive Search

By default, the search is case-sensitive. If you want to switch to a case-insensitive search, set the environment variable `CASE_INSENSITIVE` to `1` as shown below:

```bash
CASE_INSENSITIVE=1 cargo run <query> <filename>
```

where:
- `<query>` is the string you want to search for,
- `<filename>` is the name of the file you want to search in.

# 🇷🇺

MiniGrep - это упрощенная версия классического инструмента командной строки UNIX под названием `grep` (global regular expression print). Он ищет файлы для заданного строки или регулярного выражения и выводит строки, которые содержат совпадения. Это реализация на Rust.

## Компиляция проекта

Убедитесь, что у вас установлен Rust на вашем компьютере. Если нет, смотрите [официальную документацию](https://www.rust-lang.org/tools/install) для установки.

Для компиляции проекта используйте следующую команду:

```bash
cargo build --release
```

## Запуск приложения

Чтобы запустить приложение, используйте следующую команду:

```bash
cargo run <query> <filename>
```

где:
- `<query>` - это строка, которую вы хотите найти,
- `<filename>` - имя файла, в котором вы хотите искать.

## Запуск тестов

Чтобы запустить тесты, используйте следующую команду:

```bash
cargo test
```

## Переключение между чувствительным и нечувствительным к регистру поиском

По умолчанию, поиск чувствителен к регистру. Если вы хотите переключиться на поиск, нечувствительный к регистру, установите переменную окружения `CASE_INSENSITIVE` в `1`, как показано ниже:

```bash
CASE_INSENSITIVE=1 cargo run <query> <filename>
```

где:
- `<query>` - это строка, которую вы хотите найти,
- `<filename>` - имя файла, в котором вы хотите искать.
