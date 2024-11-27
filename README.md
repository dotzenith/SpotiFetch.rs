<h2 align="center"> ━━━━━━  ❖  ━━━━━━ </h2>

<!-- BADGES -->
<div align="center">
   <p></p>
   
   <img src="https://img.shields.io/github/stars/dotzenith/SpotiFetch.rs?color=F8BD96&labelColor=302D41&style=for-the-badge">   

   <img src="https://img.shields.io/github/forks/dotzenith/SpotiFetch.rs?color=DDB6F2&labelColor=302D41&style=for-the-badge">   

   <img src="https://img.shields.io/github/repo-size/dotzenith/SpotiFetch.rs?color=ABE9B3&labelColor=302D41&style=for-the-badge">
   
   <img src="https://img.shields.io/github/commit-activity/y/dotzenith/SpotiFetch.rs?color=96CDFB&labelColor=302D41&style=for-the-badge&label=COMMITS"/>
   <br>
</div>

<p/>

---

### ❖ Information 

  SpotiFetch.rs is a simple fetch tool to display info about your Spotify profile using the spotify API. It's a rewrite of the original [spotifetch](https://github.com/dotzenith/SpotiFetch)

  <img src="https://github.com/dotzenith/dotzenith/blob/main/assets/SpotiFetch/spotifetch.gif" alt="spotifetch gif">

---

### ❖ Requirements

Register an app on the Spotify developer dashboard [here](https://developer.spotify.com/dashboard/)

Edit the app settings and set `http://127.0.0.1:9090` as the redirect URI

Take a note of your Client ID and Client Secret

Put the following in your `.bashrc` or `.zshrc` or the equivalent for your shell
```sh
export RSPOTIFY_CLIENT_ID='insert-your-spotify-client-id-here'
export RSPOTIFY_CLIENT_SECRET='insert-your-spotify-client-secret-here'
export RSPOTIFY_REDIRECT_URI='http://127.0.0.1:9090'
```

---

### ❖ Installation

#### Shell
```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/dotzenith/SpotiFetch.rs/releases/latest/download/spotifetch-installer.sh | sh
```

#### Brew
```sh
brew install dotzenith/tap/spotifetch
```

#### Powershell
```sh
powershell -ExecutionPolicy ByPass -c "irm https://github.com/dotzenith/SpotiFetch.rs/releases/latest/download/spotifetch-installer.ps1 | iex"
```

#### Cargo
```sh
cargo install spotifetch
```

#### Binaries
Pre-Compiled binaries for linux, mac, and windows are available in [Releases](https://github.com/dotzenith/SpotiFetch.rs/releases)

#### Source
- First, install [rust](https://rustup.rs/)
```sh
git clone https://github.com/dotzenith/SpotiFetch.rs.git
cd SpotiFetch.rs
cargo build --release
./target/release/spotifetch
```

### ❖ Usage 

If the instructions in the Requirements section are followed properly, SpotiFetch will ask you to log in and give permissions to fetch stats the first time it's used. Login is not required after the first use. 


```
Usage: spotifetch [OPTIONS] <COMMAND>

Commands:
  profile      Fetch general information about user profile
  top-tracks   Fetch the top tracks for a given term
  top-artists  Fetch the top artists for a given term
  help         Print this message or the help of the given subcommand(s)

Options:
  -c, --colorscheme <STR>  See the readme for available colorschemes [default: "catppuccin mocha"]
  -t, --term <STR>         The timeline for the top tracks/artists; short, mid, long [default: mid]
  -a, --art                Use cover art for album or artist to generate a colorscheme
  -r, --random             Use a random color as the outline
  -h, --help               Print help
  -V, --version            Print version
```


#### SpotiFetch can be used like any other fetch tool

```sh
$ spotifetch profile      # fetches profile stats
$ spotifetch top-artists  # fetches your top five artists
$ spotifetch top-tracks   # fetches your top five songs
```

#### The top artists and tracks depends on the time-frame. By default, SpotiFetch fetches your top artists in the short term, but mid term, and long term are also available using the `--term`/`-t` option.

```sh
$ spotifetch top-artists -t short # fetches top artists in the short term
$ spotifetch top-artists -t mid   # fetches top artists in the mid term
$ spotifetch top-artists -t long  # fetches top artists in the long term
```

#### SpotiFetch supports `--random`/`-r` option to print the spotify ascii art with a random colored outline instead of the usual green

```sh
$ spotifetch profile                # prints green spotify art
$ spotifetch profile --random       # prints spotify art with random color
```

#### SpotiFetch can be used with a variety of different colorschemes.

> SpotiFetch uses [catppuccin mocha](https://github.com/catppuccin) as it's default color scheme, but a different one can be specified using the `--colorscheme`/`c` option 

For example:
```sh
$ spotifetch profile                # uses catppuccin mocha
$ spotifetch profile -c nord        # uses nord
```

The list of all available colorschemes can be found [here](https://github.com/dotzenith/kolorz.rs)

#### SpotiFetch also supports dynamically generated colorschemes using the `--art`/`a` option

```sh
$ spotifetch profile --art      # Generates colorscheme based on the cover art of the recently played song
$ spotifetch top-artists --art  # Generates colorscheme based on the profile image of the top artist
$ spotifetch top-tracks --art   # Generates colorscheme based on the cover art of the top track 
```
---

### ❖ About SpotiFetch.rs

SpotiFetch.rs is the direct result of browsing too many unix subreddits and general interest in cli tools. This is a rewrite of the original [spotifetch](https://github.com/dotzenith/SpotiFetch) because I don't feel like maintaining old python projects.

---

### ❖ What's New? 
0.1.4 - Move away from openssl

---

<div align="center">

   <img src="https://img.shields.io/static/v1.svg?label=License&message=MIT&color=F5E0DC&labelColor=302D41&style=for-the-badge">

</div>
