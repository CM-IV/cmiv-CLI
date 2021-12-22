# cmiv-CLI

(GNU General Public License)

A command line interface which auto generates a frontend framework.

The framework can either be JavaScript or TypeScript depending on the choice made, Snowpack bundler and Preact are also included.

### Install the CLI

---

Install globally

> npm i -g @cm-iv/generate-project

### Run the CLI

---

Run the CLI

> generate-project --install

The --install option automatically installs the package.json dependencies for you.

Follow the prompts, it's that simple!

**1.2.0 Update**

I have recently added the wmr frontend framework tool for Preact development. So instead of using the Snowpack build tool in the Javascript/Typescript templates, you can opt for wmr which uses the rollup bundler.
