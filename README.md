# Dioxus v0.6 + Tailwindcss

Easily development cross-platform app with this Dioxus custom template.

___

## Development Setup
1. Install Tailwind CSS

```bash
   npm install -D tailwindcss@3
```
```bash
   npx tailwindcss -i ./input.css -o ./public/tailwind.css
```

2. **Install the Dioxus CLI**

To get started with Dioxus, install the CLI using the following command:

```bash
   cargo install dioxus-cli
```

Start the Dioxus development server with live reloading \
by default, the Dioxus development server runs for the web platform
```bash
   dx serve
```

run the development server for a different platform, specify the platform explicitly:
```bash
   dx serve --platform desktop
```

## Build and Clean
1. Build the project for production:
```bash
   dx bundle --platform web
```

2. **Clean the project** Remove all `target` dir:
```bash
   dx clean
```


## Resources Dependencies

Before starting, ensure you have the following installed on your system:

- Dioxus: [https://dioxuslabs.com](https://dioxuslabs.com)
- Rust: [https://www.rust-lang.org](https://www.rust-lang.org)
- Nodejs: [https://nodejs.org/en/download](https://nodejs.org/en/download)
- TailwindCSS: [https://v3.tailwindcss.com](https://v3.tailwindcss.com/docs/installation)
- Tauri: [https://tauri.app/start/prerequisites](https://tauri.app/start/prerequisites/#linux)
