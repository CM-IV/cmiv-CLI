# cmiv-CLI

A command line interface which auto generates a frontend framework.

![image](https://github.com/CM-IV/cmiv-CLI/assets/44551614/f639763e-71a3-48cf-baf7-32e8d8902bcf)


Pick from four options (so far):

- NextJS 13 App Router
- Astro v3.0
- Preact Vite Typescript
- SolidJS Vite Typescript

**1.2.0 Update**

I have recently added the wmr frontend framework tool for Preact development. So instead of using the Snowpack build tool in the Javascript/Typescript templates, you can opt for wmr which uses the rollup bundler.

**1.3.1 Update**

I have added the Vite frontend toolset into the cli as an option.  It makes use of the preact-ts preset.
A .gitignore file was also included in the wmr-ts directory.

**1.4.1 Update**

The Vitejs and WMR templates should now be Dockerized with updated dependencies.

**1.4.2 Update**

The Vitejs template was updated to include the Vitest unit testing framework!

**1.4.3 Update**

gitignore files should now be included in the project, npm does weird things with its file naming upon publishing packages.

**2.0.0 Update - Breaking Changes!!!**

Completely re-written in typescript and simplified, templates are still installed but if you want a git repo you'll have to initialize it yourself with `git init`.

**2.1.0 Update**

Using packages from [UnJS](https://unjs.io/packages) now.  The templates have been updated and the terminal UI was overhauled.

## Installation

---
Install globally with npm

> npm i -g @cm-iv/generate-project

or use (p)npx

> pnpx @cm-iv/generate-project generate-project
## Run the CLI

---

> generate-project
