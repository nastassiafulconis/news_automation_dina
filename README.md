## News Automation

This program summarizes news articles most recently published when given a keyword.

## Running this Program

1. First, get an API key from the [News API](https://newsapi.org/account). Save it somewhere you won't forget!

2. Next, install dependencies. You will need to [install Rust](https://www.rust-lang.org/tools/install).
3. Then, clone the repository using `git clone https://github.com/nastassiafulconis/news_automation_dina.git` or download the ZIP file.
4. Navigate to the repository, and run `cargo build --release`.
5. To run the program, run `./target/release/news_automation`.
6. Follow the prompts. The program will ask you for keywords and your API key!

## Example

With input `three body problem`, the output will look like this: 

```
Title: 3 Body Problem's VFX Designer on Creating a Sci-Fi World
Gizmodo.com | 2024-04-30T18:45:00Z
Description: Netflix’s 3 Body Problem won over audiences with its sci-fi mysteries, complex characters, and startling visuals. If you’re still haunted by that Panama Canal scene—in which a ship full of people is sliced into ribbons—or the unpredictable landscapes of the V…
URL: https://gizmodo.com/3-body-problem-vfx-reel-interview-panama-canal-vr-game-1851442710

Title: Apple iPad Pro (2024) review: the best kind of overkill
The Verge | 2024-05-13T21:00:00Z
Description: With an M4 chip, a new OLED screen, a thinner and lighter body, and new accessories like the Pencil Pro and the Magic Keyboard, Apple’s latest tablet is a winner. But iPadOS still holds it back.
URL: https://www.theverge.com/24155440/apple-ipad-pro-2024-review

Title: Australia tries to stop a violence against women 'epidemic', starting with schools
BBC News | 2024-05-11T05:04:00Z
Description: As the nation reels from the Bondi Junction stabbings, what can be done to shift behaviour?
URL: https://www.bbc.co.uk/news/world-australia-68989354
```
