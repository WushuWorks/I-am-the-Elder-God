# License

I am the Elder God. A 3 vs 1 board game made using Quicksilver

Copyright (C) 2019  WushuWorks

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.

# I am the Elder God Game

Many specific details are currently in flux, and will change during development to suite
 what is practical and possible however the following elements are fairly certain as of writing.

1. This will be a 3 versus 1 board game
2. This will be developed in 2D
3. This will use square tiles

Visit the project [website](https://www.wushuworks.com/projects/i-am-the-elder-god) for more details

# Supported Browsers

In order of *best to least beast*

Firefox, Opera, Chrome

The below browsers are **officially unsupported**

1) All Microsoft browsers

# Quick Play Instructions (For Players)

It is not required but **highly recommended to play in Firefox**, download it from the official
website [here](https://www.mozilla.org/)

**Play** the current web release [here](https://wushuworks.github.io/I-am-the-Elder-God/)

# Play Instructions (For Developers)

## Desktop
1. Clone to a repo of your choice
2. Run `cargo run --release`

## Web
1. Clone to a repo of your choice
2. **If** `cargo-web` is not installed, run `cargo install cargo-web`
3. Run `web start --release`
4. Copy `http://[::1]:8000` into a Firefox of your choice

# Build for Deployment to GitHub Pages (For Developers)

1. **Push** all commits to `master` branch
2. **Checkout** (or create) to `gh-pages` branch, *very important*
3. **Merge** from `master`
4. **If** `cargo-web` is not installed, run `cargo install cargo-web`
5. **Run** `cargo web deploy --release`
6. **Copy** everything in `target/deploy`to the project root directory
7. **Add** all the files to git `git add .` and commit them `git commit -m "Don't use Microsoft browsers"`
8. **Push** everything to your `gh-pages` branch, `git push origin gh-pages`

Play game in a Firefox browser of your choice at `https://{github-username}.github.io/{repo name}/`

## Sources and Inspirations Cited

This game makes heavy use of the state machine used in [Mehen's Portable Casino](https://github.com/OtherAesop/mehens_portable_casino)
and shares structure and code with it.

This game was heavily inspired by [Evolve](https://2k.com/en-US/game/evolve/) and [Arkham Horror](https://www.fantasyflightgames.com/en/products/arkham-horror-third-edition/)