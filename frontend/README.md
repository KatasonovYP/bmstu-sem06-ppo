# ProjectName

node: v21.2.0

yarn: 1.22.19

Description...

## Start the app

### start the development server

```bash
yarn nx serve <app_name>
```

Open your browser and navigate to http://localhost:4200/.

### build and run the app

```bash
yarn nx build <app_name>
```

### build and preview the app

```bash
yarn nx preview <app_name>
```

Open your browser and navigate to http://localhost:4300/.

### start the development Storybook

```bash
yarn nx serve storybook
```

Open your browser and navigate to http://localhost:4400/.

### build and run the storybook

```bash
yarn nx build storybook
live-server .\dist\apps\storybook --port=4500
```

Open your browser and navigate to http://localhost:4500/.

## Executors

### CI

```bash
yarn nx run-many -t ci --fix --parallel=8 --output-style=stream
```

### Lint

```bash
yarn nx lint <app_name> --fix
```

### Lint many

```bash
yarn nx run-many -t lint --fix --parallel=8
```

### Prettier many

```bash
yarn nx run-many -t prettier --fix --parallel=8
```

## Generators

### Generate slice

```bash
yarn nx g @my/plugin:slice [name] [layer]
```

### Generate component (deprecated)

```bash
yarn nx g @my/plugin:component <component_name> --directory=libs/<lib_name>/src/<your_layer>/<component_name>
```
