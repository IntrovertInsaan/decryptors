# Decryptors

A CLI puzzle game where you decrypt hidden information through
unconventional thinking. Each level presents a unique cipher —
no linear thinking allowed.

## Puzzles
- **Level 0** — The Fruit Cipher
- **Level 1** — The Chess Cipher
- **Level 2** — The Ludo Cipher
- **Level 3** — The Three Realms
- **Level 4** — The Shape Speaks
- **Level 5** — The Cosmos Cipher
- **Level 6** — The Fable Cipher

## Requirements
A terminal with image support for the best experience:
- WezTerm ✦ recommended
- Ghostty ✦ recommended
- Kitty
- iTerm2
- Windows Terminal (supported, reduced quality)

Not supported: VSCode terminal, Mac Terminal.app

## Install & Run
```bash
git clone https://github.com/IntrovertInsaan/decryptors
cd decryptors
cargo run --release
```

macOS:
```bash
cargo run --release --no-default-features
```

Optional: install mpv for audio (used in levels 3 and 5)
```bash
sudo apt install mpv
```

## License
Code is [MIT](LICENSE).
Puzzle content is proprietary — see [LICENSE](LICENSE) for details.
