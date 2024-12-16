# Dioxus v0.6 + Tailwindcss + daisyui

Your new bare-bones project includes minimal organization with a single `main.rs` file and a few assets.

### Tailwind
1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the Tailwind CSS CLI: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
npm i -D daisyui@latest
```

```bash
npx tailwindcss -i ./input.css -o ./public/tailwind.css
```

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
npm run web
```

### Check Output Size

To check the size of the generated output, use the following command:
```bash
du -sh target/dx/rust-webassembly/release/web/public
```
